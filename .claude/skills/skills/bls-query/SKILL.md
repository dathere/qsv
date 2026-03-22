---
name: bls-query
description: Queries U.S. Bureau of Labor Statistics data using the BLS MCP server. Use when user asks about CPI, inflation, unemployment, employment, wages, jobs, labor statistics, producer prices, or any economic indicator tracked by BLS. Maps natural language to correct series IDs and tools. Copied from https://github.com/larasrinath/bls_mcp
metadata:
  version: 1.0.0
  mcp-server: bls
---

# BLS Data Query

## Instructions

When a user asks about U.S. economic/labor data, follow this workflow to resolve their query into the correct BLS series ID(s) and return accurate results.

### Step 1: Identify the topic

Map the user's natural language to a BLS survey and series ID. Consult `references/series-catalog.md` for the full mapping of common topics to series IDs.

Common mappings:
- "CPI", "inflation", "consumer prices" --> CU survey, series `CUUR0000SA0` (not seasonally adjusted) or `CUSR0000SA0` (seasonally adjusted)
- "unemployment rate" --> LN survey, series `LNS14000000`
- "jobs", "employment", "nonfarm payrolls" --> CE survey, series `CES0000000001`
- "wages", "average hourly earnings" --> CE survey, series `CES0500000003`
- "PPI", "producer prices" --> WP/PC survey
- "JOLTS", "job openings" --> JT survey, series `JTS000000000000000JOL`

If the topic is ambiguous or you cannot confidently map it, use `get_all_surveys` to find the survey, then `get_popular_series` with the survey abbreviation to discover series IDs.

### Step 2: Choose the right tool

| User intent | Tool | When to use |
|---|---|---|
| Latest value / current number | `get_latest_series` | "What is the current...", "latest...", "most recent..." |
| Recent trend / time series | `get_single_series` | "Show me CPI for the last few years", "trend in..." |
| Compare multiple indicators | `get_multiple_series` | "Compare X and Y", multiple series needed |
| Explore what's available | `get_all_surveys` then `get_popular_series` | "What data do you have on...", discovery questions |
| Details about a survey | `get_survey` | "Tell me about the [survey name]" |

### Step 3: Interpret and present results

- Always state the series ID used so the user can verify.
- Include the period (month/year) and value clearly.
- For CPI data, note whether it is seasonally adjusted or not.
- If the user asks about percent change or inflation rate, calculate it from the data if the `calculations` field is not available (requires API key).
- For unemployment, the value is already a percentage. For CPI, the value is an index number (base period 1982-84=100).

### Step 4: Handle unknowns

If you cannot confidently identify the series ID:
1. Call `get_all_surveys` to find the relevant survey abbreviation.
2. Call `get_popular_series` with that survey abbreviation.
3. Present the popular series to the user and ask which one they need.

Do NOT guess a series ID. An incorrect series ID will return wrong data silently.

## Common Issues

### Rate limit errors
If you see a rate limit error, inform the user they can set `BLS_API_KEY` for higher limits (500 queries/day vs 25).

### Empty results
Some series are discontinued or seasonal. Try the non-seasonally-adjusted variant, or check the survey metadata with `get_survey`.

### Series ID not found
Double-check the ID against `references/series-catalog.md`. BLS series IDs are case-sensitive and must be uppercase.
