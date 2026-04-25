//! Chat-mode session state: loading, saving, sliding-window compaction, and
//! relevance / summarization helpers for the --session feature.
//!
//! Session files are markdown so the user can read and edit them. Sections are:
//!   - `# Session: …` header
//!   - `## Baseline SQL Query` (optional)
//!   - `## Conversation History` (one `### Message N` per turn with Role + Content)
//!   - `## SQL Results (Last Successful)` (optional)
//!   - `## SQL Errors` (bulleted list)
//!   - `## Summary` (produced by sliding-window compaction)

use std::{fs, io::Write as _, path::Path};

use reqwest::blocking::Client;
use serde_json::json;

use super::{
    Args, CliError, CliResult, PromptType, check_model, duckdb_sql::extract_sql_sample,
    get_completion,
};

/// One turn of the chat-mode conversation. `timestamp` is written back to the
/// session file but is advisory — it's re-stamped to `now` on load, so it's
/// primarily useful within a single run. Marked dead_code because only `role`
/// and `content` are functionally meaningful.
#[derive(Debug, Clone)]
pub(super) struct SessionMessage {
    pub(super) role:      String,
    pub(super) content:   String,
    #[allow(dead_code)]
    pub(super) timestamp: String,
}

/// In-memory representation of a session markdown file.
#[derive(Debug, Clone)]
pub(super) struct SessionState {
    pub(super) baseline_sql: Option<String>,
    pub(super) messages:     Vec<SessionMessage>,
    pub(super) sql_results:  Option<String>,
    pub(super) sql_errors:   Vec<String>,
    pub(super) summary:      Option<String>,
}

/// Normalize session path to always have a `.md` extension.
pub(super) fn normalize_session_path(session_path: &str) -> String {
    let path = Path::new(session_path);
    if let Some(ext) = path.extension()
        && ext == "md"
    {
        return session_path.to_string();
    }
    path.with_extension("md").to_string_lossy().to_string()
}

/// Load session state from a markdown file, or return an empty state if the
/// file doesn't exist yet.
pub(super) fn load_session(session_path: &Path) -> CliResult<SessionState> {
    if !session_path.exists() {
        return Ok(SessionState {
            baseline_sql: None,
            messages:     Vec::new(),
            sql_results:  None,
            sql_errors:   Vec::new(),
            summary:      None,
        });
    }

    let content = fs::read_to_string(session_path)?;
    let mut state = SessionState {
        baseline_sql: None,
        messages:     Vec::new(),
        sql_results:  None,
        sql_errors:   Vec::new(),
        summary:      None,
    };

    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;
    let mut current_content = String::new();
    let mut current_role = String::new();

    while i < lines.len() {
        let line = lines[i].trim();

        if line.starts_with("# Session:") {
            // Skip header
        } else if line == "## Baseline SQL Query" {
            current_content.clear();
            i += 1;
            if i < lines.len() && lines[i].trim() == "```sql" {
                i += 1;
            }
            while i < lines.len() && !lines[i].trim().starts_with("```") {
                if !current_content.is_empty() {
                    current_content.push('\n');
                }
                current_content.push_str(lines[i]);
                i += 1;
            }
            state.baseline_sql = Some(current_content.trim().to_string());
            current_content.clear();
        } else if line == "## Conversation History" {
            i += 1;
            let mut in_content_section = false;
            let mut in_code_block = false;
            while i < lines.len() {
                let msg_line = lines[i];
                let msg_line_trimmed = msg_line.trim();

                // Break on next top-level section — but only when not inside a code fence.
                if !in_code_block
                    && msg_line_trimmed.starts_with("##")
                    && !msg_line_trimmed.starts_with("###")
                {
                    if !current_content.is_empty() && !current_role.is_empty() {
                        state.messages.push(SessionMessage {
                            role:      current_role.clone(),
                            content:   current_content.trim().to_string(),
                            timestamp: chrono::Utc::now().to_rfc3339(),
                        });
                    }
                    break;
                }

                // Track code fences so `##` inside one doesn't terminate the section.
                if msg_line_trimmed.starts_with("```") {
                    in_code_block = !in_code_block;
                }

                if msg_line_trimmed.starts_with("### Message") {
                    // New message — flush the previous one first.
                    if !current_content.is_empty() && !current_role.is_empty() {
                        state.messages.push(SessionMessage {
                            role:      current_role.clone(),
                            content:   current_content.trim().to_string(),
                            timestamp: chrono::Utc::now().to_rfc3339(),
                        });
                    }
                    current_content.clear();
                    current_role.clear();
                    in_content_section = false;
                    in_code_block = false;
                } else if msg_line_trimmed.starts_with("**Role:**") {
                    current_role = msg_line_trimmed.replace("**Role:**", "").trim().to_string();
                    in_content_section = false;
                } else if msg_line_trimmed.starts_with("**Content:**") {
                    current_content.clear();
                    in_content_section = true;
                } else if in_content_section {
                    // Preserve original formatting (blank lines, code blocks, etc.).
                    if !current_content.is_empty() {
                        current_content.push('\n');
                    }
                    current_content.push_str(msg_line);
                }
                i += 1;
            }
            // Flush last message if we hit EOF before the next section header.
            if !current_content.is_empty() && !current_role.is_empty() {
                state.messages.push(SessionMessage {
                    role:      current_role.clone(),
                    content:   current_content.trim().to_string(),
                    timestamp: chrono::Utc::now().to_rfc3339(),
                });
            }
            continue;
        } else if line == "## SQL Results (Last Successful)" {
            current_content.clear();
            i += 1;
            if i < lines.len() && lines[i].trim() == "```csv" {
                i += 1;
            }
            while i < lines.len() && !lines[i].trim().starts_with("```") {
                if !current_content.is_empty() {
                    current_content.push('\n');
                }
                current_content.push_str(lines[i]);
                i += 1;
            }
            state.sql_results = Some(current_content.trim().to_string());
            current_content.clear();
        } else if line == "## SQL Errors" {
            i += 1;
            while i < lines.len() && !lines[i].trim().starts_with("##") {
                let line = lines[i].trim();
                if line.starts_with("- ") {
                    let error = line.strip_prefix("- ").unwrap_or(line).to_string();
                    state.sql_errors.push(error);
                }
                i += 1;
            }
            continue;
        } else if line == "## Summary" {
            current_content.clear();
            i += 1;
            while i < lines.len() && !lines[i].trim().starts_with("##") {
                if !current_content.is_empty() {
                    current_content.push('\n');
                }
                current_content.push_str(lines[i]);
                i += 1;
            }
            state.summary = Some(current_content.trim().to_string());
            current_content.clear();
            continue;
        }
        i += 1;
    }

    Ok(state)
}

/// Save session state to a markdown file.
///
/// Writes atomically via a temp file in the same directory, then renames into place,
/// so a crash mid-write leaves either the previous good file or the new one — never
/// a truncated file.
pub(super) fn save_session(session_path: &Path, state: &SessionState) -> CliResult<()> {
    use std::fmt::Write as _;

    let parent_dir = session_path.parent().filter(|p| !p.as_os_str().is_empty());
    if let Some(parent) = parent_dir {
        fs::create_dir_all(parent)?;
    }

    let mut content = String::new();
    let _ = write!(
        content,
        "# Session: {}\n\n",
        session_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
    );

    if let Some(ref sql) = state.baseline_sql {
        content.push_str("## Baseline SQL Query\n\n");
        content.push_str("```sql\n");
        content.push_str(sql);
        content.push_str("\n```\n\n");
    }

    content.push_str("## Conversation History\n\n");
    for (idx, msg) in state.messages.iter().enumerate() {
        let _ = write!(content, "### Message {}\n\n", idx + 1);
        let _ = write!(content, "**Role:** {}\n\n", msg.role);
        let _ = write!(content, "**Content:**\n\n{}\n\n", msg.content);
    }

    if let Some(ref results) = state.sql_results {
        content.push_str("## SQL Results (Last Successful)\n\n");
        content.push_str("```csv\n");
        content.push_str(results);
        content.push_str("\n```\n\n");
    }

    if !state.sql_errors.is_empty() {
        content.push_str("## SQL Errors\n\n");
        for error in &state.sql_errors {
            let _ = writeln!(content, "- {error}");
        }
        content.push('\n');
    }

    if let Some(ref summary) = state.summary {
        content.push_str("## Summary\n\n");
        content.push_str(summary);
        content.push('\n');
    }

    // Atomic write via tempfile-then-persist in the destination directory.
    let tmp_dir = parent_dir.unwrap_or_else(|| Path::new("."));
    let mut tmp = tempfile::Builder::new()
        .prefix(".describegpt-session-")
        .suffix(".tmp")
        .tempfile_in(tmp_dir)?;
    tmp.as_file_mut().write_all(content.as_bytes())?;
    tmp.as_file_mut().sync_all()?;
    tmp.persist(session_path)
        .map_err(|e| CliError::from(e.error))?;
    Ok(())
}

/// Append a SQL error to session state (if one exists) and immediately persist.
pub(super) fn track_sql_error_in_session(
    session_state: Option<&mut SessionState>,
    normalized_session_path: Option<&String>,
    error_msg: String,
) {
    if let Some(state) = session_state {
        state.sql_errors.push(error_msg);
        if let Some(normalized_path) = normalized_session_path {
            let _ = save_session(Path::new(normalized_path), state);
        }
    }
}

/// Clear SQL errors and stash a small sample of the successful results in
/// session state. Sets `baseline_sql` on first success.
pub(super) fn update_session_after_sql_success(
    session_state: Option<&mut SessionState>,
    sql_results: &str,
    sql_query: &str,
) {
    if let Some(state) = session_state {
        // SQL execution succeeded, so any previously-recorded errors are stale —
        // clear them unconditionally. Sampling the results is best-effort: if it
        // fails (missing file, permission, parse error), we still clear errors
        // and set the baseline; we just skip updating sql_results.
        state.sql_errors.clear();

        let results_path = Path::new(sql_results).with_extension("csv");
        if results_path.exists()
            && let Ok(sample) = extract_sql_sample(&results_path)
        {
            state.sql_results = Some(sample);
        }

        // Baseline SQL is only set after a successful execution, so we don't
        // pin a broken query as the baseline.
        if state.baseline_sql.is_none() {
            state.baseline_sql = Some(sql_query.to_string());
        }
    }
}

/// Generate an LLM summary of the oldest messages before they are dropped by
/// the sliding window.
pub(super) fn generate_summary(
    old_messages: &[SessionMessage],
    args: &Args,
    client: &Client,
    api_key: &str,
) -> CliResult<String> {
    use std::fmt::Write as _;

    let mut summary_prompt = String::from(
        "Please provide a concise summary of the following conversation history. Focus on the key \
         SQL query refinements, user requests, and assistant responses:\n\n",
    );

    for msg in old_messages {
        let _ = write!(summary_prompt, "{}: {}\n\n", msg.role, msg.content);
    }

    summary_prompt
        .push_str("\nProvide a brief summary that captures the essence of this conversation:");

    let system_prompt = "You are a helpful assistant that summarizes conversation history for SQL \
                         query refinement sessions.";
    let messages = json!([
        {"role": "system", "content": system_prompt},
        {"role": "user", "content": summary_prompt}
    ]);

    let model = check_model(client, Some(api_key), args)?;
    let completion = get_completion(args, client, &model, api_key, &messages, PromptType::Prompt)?;
    Ok(completion.response)
}

/// Cap the message history at `max_len` by summarizing the oldest overflow.
/// The summary is appended to any existing summary already in state.
pub(super) fn apply_sliding_window(
    state: &mut SessionState,
    max_len: usize,
    args: &Args,
    client: &Client,
    api_key: &str,
) -> CliResult<()> {
    if state.messages.len() <= max_len {
        return Ok(());
    }

    let num_to_summarize = state.messages.len() - max_len;
    let old_messages: Vec<SessionMessage> = state.messages.drain(..num_to_summarize).collect();

    let summary = generate_summary(&old_messages, args, client, api_key)?;

    if let Some(ref existing_summary) = state.summary {
        state.summary = Some(format!("{existing_summary}\n\n{summary}"));
    } else {
        state.summary = Some(summary);
    }

    Ok(())
}

/// Decide whether a new user prompt is a refinement of the baseline SQL query.
///
/// Cheap heuristic first (SQL keywords / references to prior turns); if that's
/// inconclusive, ask the LLM. Used to decide whether to continue the current
/// session or start a fresh one.
pub(super) fn check_message_relevance(
    prompt: &str,
    baseline_sql: &str,
    args: &Args,
    client: &Client,
    api_key: &str,
) -> CliResult<bool> {
    let sql_keywords = [
        "sql",
        "query",
        "select",
        "where",
        "join",
        "group",
        "order",
        "filter",
        "refine",
        "modify",
        "change",
        "update",
        "fix",
        "correct",
        "improve",
        "add",
        "remove",
        "include",
        "exclude",
        "sort",
        "aggregate",
        "count",
    ];

    let prompt_lower = prompt.to_lowercase();
    let has_sql_keywords = sql_keywords.iter().any(|kw| prompt_lower.contains(kw));

    let has_references = prompt_lower.contains("previous")
        || prompt_lower.contains("last")
        || prompt_lower.contains("above")
        || prompt_lower.contains("before");

    if has_sql_keywords || has_references {
        return Ok(true);
    }

    // Fallback: ask the LLM.
    let relevance_prompt = format!(
        "The user has been working on refining a SQL query. The baseline SQL query \
         is:\n\n```sql\n{baseline_sql}\n```\n\nUser's new message: \"{prompt}\"\n\nIs this \
         message related to refining, modifying, or improving the SQL query above? Answer with \
         only 'yes' or 'no'."
    );

    let system_prompt = "You are a helpful assistant that determines if user messages are related \
                         to SQL query refinement.";
    let messages = json!([
        {"role": "system", "content": system_prompt},
        {"role": "user", "content": relevance_prompt}
    ]);

    let model = check_model(client, Some(api_key), args)?;
    let completion = get_completion(args, client, &model, api_key, &messages, PromptType::Prompt)?;

    // The system prompt asks for "yes" / "no". Be strict so tokens that merely
    // contain "yes" (e.g. "yesterday") don't get read as affirmative. Accept
    // "yes" / "y" exactly, optionally followed by punctuation, plus "yes, …"
    // or "yes." leading responses.
    let response_lower = completion.response.to_lowercase();
    let trimmed = response_lower
        .trim()
        .trim_end_matches(['.', ',', '!', '?', ' ']);

    Ok(trimmed == "yes" || trimmed == "y" || trimmed.starts_with("yes "))
}
