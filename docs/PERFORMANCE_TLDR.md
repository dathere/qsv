# Performance Tuning Guide (TLDR version)

This guide will help you get the best performance out of qsv for your data analysis workflows.

## Key Performance Features

### 1. Indexing (Most Important!)

Think of indexing like creating a table of contents for your CSV files. It's the single most important thing you can do to improve performance. Here's why you want to use it:

- Makes slicing data nearly instant
- Gives you immediate row counts
- Enables parallel processing for commands like `stats`, `frequency`, and `sample`
- Adds random access capabilities for advanced features
- Takes very little time to create (example: a 520MB file with 1 million rows takes less than half a second)

**Quick Setup:**
```bash
# Automatically index files larger than 10MB
export QSV_AUTOINDEX_SIZE=10000000
```

### 2. Stats Cache

The stats cache is qsv's secret weapon for fast data analysis. When you run the `stats` command, qsv saves detailed information about your data that other commands can use to work smarter:

- Makes frequency tables faster by knowing which columns have unique values
- Helps create accurate JSON and SQL schemas without repeated analysis
- Enables smart pivoting by automatically choosing the right aggregation functions
- Speeds up data sampling and comparison operations

**Pro Tip:** Always run `stats` with the `--stats-jsonl` option on your frequently-used datasets to create this cache. Alternatively, you can also set QSV_STATSCACHE_MODE to "force" or "auto".

### 3. Memory Management

qsv is designed to handle large files efficiently, but some operations need to load entire files into memory. Here's what you need to know:

**Commands that need full memory loading (marked with 🤯):**
- `dedup` (unless using --sorted)
- `reverse`
- `sort`
- `stats` (for advanced statistics)
- `table`
- `transpose`

**Memory-intensive commands (marked with 😣):**
- `frequency`
- `schema`
- `tojsonl`

qsv will automatically prevent out-of-memory crashes by checking your system's resources before running these commands.

### 4. Multithreading

Many commands automatically use parallel processing when possible:
- With index: `count`, `stats`, `frequency`, `sample`, `schema`, `split`, `tojsonl`
- Without index: `apply`, `dedup`, `diff`, `sort`, `sqlp`, and others

qsv automatically detects your CPU cores and uses them appropriately.

## Quick Performance Tips

1. **Always index large files** - It's fast and makes everything else faster
2. **Use the stats cache** - Run `stats --stats-jsonl` on your important datasets
3. **For very large files:**
   - Use `extsort` instead of `sort`
   - Use `extdedup` instead of `dedup`
   - Consider using the `--memcheck` option for memory-intensive operations

## Advanced Tuning

If you need to fine-tune performance further:

1. **Buffer sizes** can be adjusted:
   ```bash
   # Adjust read/write buffer sizes (in bytes)
   export QSV_RDR_BUFFER_CAPACITY=131072  # Default 128KB
   export QSV_WTR_BUFFER_CAPACITY=262144  # Default 256KB
   ```

2. **Control parallel processing:**
   ```bash
   # Set maximum number of parallel jobs
   # if you need to use your system for other CPU-intensive tasks
   # otherwise, qsv will use ALL available CPU cores
   # If you're just doing casual computing tasks, this is OK
   export QSV_MAX_JOBS=4
   ```

3. **Memory safety limits:**
   ```bash
   # Adjust memory safety margin (10-90%, default 20)
   # Modern Operating Systems are very smart in dynamically allocating
   # memory so this is just a safeguard
   export QSV_FREEMEMORY_HEADROOM_PCT=10
   ```

For most users, the default settings will work well. These advanced options are here if you need to optimize for specific scenarios.
