# Auto-Update System for QSV MCP Server

The QSV MCP Server includes a comprehensive auto-update system that keeps your skills in sync with qsv releases.

## How It Works

### Three-Tier Update Strategy

1. **qsv Binary Updates**
   - Detected automatically on MCP server startup
   - Users update via: `qsv --update` or package managers (brew, cargo, etc.)
   - Server detects version changes and notifies you

2. **Skill Definitions Updates**
   - Auto-detected when qsv version changes
   - Can be auto-regenerated (if configured)
   - Manual regeneration: `qsv --update-mcp-skills`

3. **MCP Server Code Updates**
   - Checked against GitHub releases
   - Updated via: `npm update` or `git pull && npm install && npm run build`

## Configuration

Add these environment variables to your Claude Desktop config:

```json
{
  "mcpServers": {
    "qsv": {
      "command": "node",
      "args": ["/path/to/qsv/.claude/skills/dist/mcp-server.js"],
      "env": {
        "QSV_MCP_BIN_PATH": "/usr/local/bin/qsv",
        "QSV_MCP_WORKING_DIR": "/Users/your-username/Downloads",

        "QSV_MCP_AUTO_REGENERATE_SKILLS": "false",
        "QSV_MCP_CHECK_UPDATES_ON_STARTUP": "true",
        "QSV_MCP_NOTIFY_UPDATES": "true",
        "QSV_MCP_GITHUB_REPO": "dathere/qsv"
      }
    }
  }
}
```

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `QSV_MCP_AUTO_REGENERATE_SKILLS` | `false` | Automatically regenerate skills when qsv version changes |
| `QSV_MCP_CHECK_UPDATES_ON_STARTUP` | `true` | Check for updates when MCP server starts |
| `QSV_MCP_NOTIFY_UPDATES` | `true` | Show update notifications in logs |
| `QSV_MCP_GITHUB_REPO` | `dathere/qsv` | GitHub repository to check for releases |

## Update Workflows

### Workflow 1: Conservative (Recommended)

**Configuration**:
```json
"QSV_MCP_AUTO_REGENERATE_SKILLS": "false"
```

**Process**:
1. Update qsv: `qsv --update`
2. MCP server detects version mismatch on next startup
3. Server logs show instructions to regenerate skills
4. Run: `qsv --update-mcp-skills`
5. Restart Claude Desktop

**Pros**:
- Full control over when updates happen
- Can review changes before applying
- No unexpected behavior

**Cons**:
- Requires manual steps
- Can forget to regenerate skills

### Workflow 2: Automatic

**Configuration**:
```json
"QSV_MCP_AUTO_REGENERATE_SKILLS": "true"
```

**Process**:
1. Update qsv: `qsv --update`
2. MCP server detects version mismatch on next startup
3. Server automatically runs `qsv --update-mcp-skills`
4. Server logs show success message
5. Restart Claude Desktop to load new skills

**Pros**:
- Fully automated
- Always in sync
- No separate toolchain required (uses qsv binary)
- Less maintenance

**Cons**:
- Regeneration adds startup time (~5-10 seconds)
- Must restart MCP server after auto-regeneration
- Requires qsv binary with "mcp" feature enabled

### Workflow 3: CI/CD Pipeline (Advanced)

For teams or advanced users, set up a GitHub Actions workflow:

```yaml
# .github/workflows/update-mcp-skills.yml
name: Update MCP Skills

on:
  # Trigger when qsv releases
  repository_dispatch:
    types: [qsv-release]

  # Or run daily
  schedule:
    - cron: '0 0 * * *'

  # Or manual trigger
  workflow_dispatch:

jobs:
  regenerate-skills:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install qsv
        run: |
          curl -LO https://github.com/dathere/qsv/releases/latest/download/qsv-linux-x86_64.zip
          unzip qsv-linux-x86_64.zip
          sudo mv qsv /usr/local/bin/

      - name: Regenerate skills
        run: qsv --update-mcp-skills

      - name: Build MCP server
        run: |
          cd .claude/skills
          npm install
          npm run build

      - name: Commit and push
        run: |
          git config user.name "GitHub Actions"
          git config user.email "actions@github.com"
          git add .claude/skills/qsv/*.json
          git commit -m "chore: regenerate skills for qsv $(qsv --version | cut -d' ' -f2)"
          git push
```

## Update Scenarios

### Scenario 1: qsv Updated, Skills Outdated

**What you'll see**:
```
⚠️  VERSION MISMATCH DETECTED ⚠️
   qsv binary: 0.133.0
   Skills generated with: 0.132.0

ℹ️  To update skills manually, run:
   qsv --update-mcp-skills
   Then restart the MCP server
```

**Action**: Run `qsv --update-mcp-skills` then restart Claude Desktop

### Scenario 2: New qsv Release Available

**What you'll see**:
```
📦 UPDATE CHECK RESULTS:
🆕 New qsv release available: 0.133.0 (you have 0.132.0)
   Update with: qsv --update
```

**Action**: Run `qsv --update`, then regenerate skills

### Scenario 3: Skills Auto-Regenerated

**What you'll see**:
```
⚠️  VERSION MISMATCH DETECTED ⚠️
   qsv binary: 0.133.0
   Skills generated with: 0.132.0

[UpdateChecker] Auto-regenerating skills...
✅ Skills regenerated successfully
   Please restart the MCP server to load updated skills
```

**Action**: Restart Claude Desktop

## Version Tracking

The update checker stores version information in `.qsv-mcp-versions.json`:

```json
{
  "qsvBinaryVersion": "0.133.0",
  "skillsGeneratedWithVersion": "0.133.0",
  "mcpServerVersion": "13.0.0",
  "lastChecked": "2026-01-07T14:30:00.000Z"
}
```

This file is automatically managed and helps track update history.

## Troubleshooting

### Auto-regeneration fails

**Problem**: `QSV_MCP_AUTO_REGENERATE_SKILLS=true` but skills don't regenerate

**Possible causes**:
1. qsv binary not in PATH or `QSV_MCP_BIN_PATH` not set correctly
2. qsv binary wasn't built with "mcp" feature
3. Insufficient permissions

**Solution**:
- Verify qsv binary location: `which qsv`
- Ensure you're in the qsv repository: `pwd` (should show path ending in `/qsv`)
- Test manual regeneration: `cd /path/to/qsv && qsv --update-mcp-skills`
- Check if binary has mcp feature: `qsv --version` (shows installed features)
- Check MCP server logs: `~/Library/Logs/Claude/mcp*.log`
- If flag not available, rebuild qsv: `cargo build --release --features all_features`

### "Could not find qsv repository root" error

**Problem**: Running `qsv --update-mcp-skills` fails with "Could not find qsv repository root"

**Cause**: The command must be run from within the qsv repository directory structure (where `Cargo.toml` and `src/cmd` exist). This is because the skill generation writes to `.claude/skills/qsv/` relative to the repository root.

**Solution**:
1. **If you installed qsv via package manager** (Homebrew, cargo install, etc.):
   - Clone the qsv repository: `git clone https://github.com/dathere/qsv.git`
   - Navigate into it: `cd qsv`
   - Run: `qsv --update-mcp-skills`

2. **If you built qsv from source**:
   - Navigate to your qsv repository directory: `cd /path/to/qsv`
   - Verify you're in the right place: `ls Cargo.toml src/cmd` (both should exist)
   - Run: `qsv --update-mcp-skills`

3. **For auto-regeneration**:
   - The MCP server must have access to the repository directory
   - Auto-regeneration will fail if the qsv binary can't find the repository
   - Consider using manual regeneration workflow if binary is installed system-wide

### Version check fails

**Problem**: Update checker reports errors

**Possible causes**:
1. qsv not in PATH
2. Network issues (GitHub API)
3. Permissions issues reading skill files

**Solution**:
- Verify qsv works: `qsv --version`
- Check network: `curl https://api.github.com/repos/dathere/qsv/releases/latest`
- Set `QSV_MCP_CHECK_UPDATES_ON_STARTUP=false` to disable

### Skills still outdated after regeneration

**Problem**: Regenerated skills but still showing old version

**Solution**:
1. Verify regeneration completed: Check `.claude/skills/qsv/*.json` files
2. Restart Claude Desktop completely (not just close window)
3. Check Claude Desktop logs for MCP server initialization
4. Verify MCP server is loading skills from correct directory

## Performance Impact

- **Quick check**: < 50ms (version comparison only)
- **Full check**: < 500ms (includes GitHub API call)
- **Auto-regeneration**: ~5-10 seconds (only when needed)

The update check runs asynchronously and doesn't block MCP server startup.

## Requirements

- **qsv binary** with "mcp" feature enabled
  - Included in prebuilt binaries from GitHub releases
  - When building from source: Use `cargo build --release --features all_features`
- **qsv repository** cloned locally
  - The `--update-mcp-skills` command must be run from within the repository
  - Outputs to `.claude/skills/qsv/` relative to repository root
  - Clone with: `git clone https://github.com/dathere/qsv.git`
- **Node.js** >= 18.0.0 (for MCP server)

## Best Practices

1. **Development**: Use `QSV_MCP_AUTO_REGENERATE_SKILLS=true` for convenience
2. **Production**: Use `QSV_MCP_AUTO_REGENERATE_SKILLS=false` for stability
3. **Teams**: Set up CI/CD pipeline for coordinated updates
4. **Check logs regularly**: Monitor for update notifications
5. **Test after updates**: Verify skills work correctly after regeneration
6. **Use prebuilt binaries**: Easiest way to ensure "mcp" feature is enabled

## Related Documentation

- [README-MCP.md](../../README-MCP.md) - MCP server documentation
- [FILESYSTEM_USAGE.md](../guides/FILESYSTEM_USAGE.md) - Filesystem features
- [qsv --update documentation](https://github.com/dathere/qsv#self-update)

## Support

For issues with auto-updates:
1. Check Claude Desktop logs: `~/Library/Logs/Claude/mcp*.log`
2. Enable debug logging: Add `NODE_DEBUG=*` to env vars
3. Report issues: https://github.com/dathere/qsv/issues

---

**Updated**: 2026-02-22
**Version**: 16.1.2
**Status**: ✅ Production Ready
