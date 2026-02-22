# QSV Stats Command: Comprehensive Technical Guide

## Table of Contents
1. [Introduction & Purpose](#introduction--purpose)
2. [Core Rust Concepts](#core-rust-concepts)
3. [Architecture Overview](#architecture-overview)
4. [Data Type System](#data-type-system)
5. [Statistics Computation](#statistics-computation)
6. [Processing Modes](#processing-modes)
7. [Performance Optimizations](#performance-optimizations)
8. [Caching System](#caching-system)
9. [Code Walkthrough](#code-walkthrough)
10. [Contributing Guide](#contributing-guide)

---

## Introduction & Purpose

The `stats` command is one of the most critical components of qsv. It computes comprehensive statistical summaries and infers data types for CSV columns. Unlike sampling-based approaches, stats performs **guaranteed** inference by scanning the entire file.

### Key Responsibilities
- **Type Inference**: Detects NULL, Integer, String, Float, Date, DateTime, and (optionally) Boolean types (when `--infer-boolean` is enabled)
- **Streaming Statistics**: Computes mean, sum, min/max, standard deviation, variance, etc. with constant memory
- **Non-Streaming Statistics**: Computes cardinality, modes, medians, quartiles (requires loading all data)
- **Date Handling**: Flexible date format inference with configurable patterns
- **Caching**: Stores computed statistics to avoid recalculation
- **Foundation for Other Commands**: Used by `schema`, `validate`, `describegpt`, `joinp`, `pivotp`, `sqlp`

### Performance Characteristics
- Uses unsafe Rust for performance-critical operations
- Supports parallel processing with multi-threading
- Implements intelligent caching based on file modification times
- Optimized memory access patterns with cache-line alignment

---

## Core Rust Concepts

Before diving into the stats implementation, here are the essential Rust concepts you need:

### 1. **Ownership & Borrowing**
```rust
// Ownership: Variables own data
let data = vec![1, 2, 3];
// data owns the vector

// Borrowing (References): Access without ownership
fn read_data(values: &Vec<i32>) {
    println!("{:?}", values); // Immutable borrow
}

// Mutable Borrowing: Exclusive access for modification
fn modify_data(values: &mut Vec<i32>) {
    values.push(4); // Can modify
}
```

**In stats.rs**: The `Stats` struct holds data about a column. Multiple threads might need to read it, so references are used.

### 2. **Result Type for Error Handling**
```rust
// Result is an enum with two variants: Ok(value) or Err(error)
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

// Using Result with ? operator (early return on error)
fn process() -> Result<i32, String> {
    let result = divide(10, 2)?; // Returns error if divide fails
    Ok(result * 2)
}
```

**In stats.rs**: Most functions return `CliResult<()>`, which is `Result<(), CliError>`. The `?` operator simplifies error propagation.

### 3. **Structs and Traits**
```rust
// Struct: Data container
struct Person {
    name: String,
    age: u32,
}

// Trait: Interface defining behavior
trait Drawable {
    fn draw(&self);
}

// Implementing a trait for a struct
impl Drawable for Person {
    fn draw(&self) {
        println!("Drawing {}", self.name);
    }
}
```

**In stats.rs**: `Stats` is a struct containing statistics data. It implements traits like `Serialize` for output and `Commute` for merging parallel results.

### 4. **Generics and Type Parameters**
```rust
// Generic function: Works with any type T
fn print_value<T: std::fmt::Display>(value: T) {
    println!("Value: {}", value);
}

// Generic struct: Can hold any type
struct Container<T> {
    value: T,
}

// Trait bounds: Restrict what types can be used
fn get_sum<T: std::ops::Add<Output = T> + Default + Copy>(values: &[T]) -> T {
    values.iter().fold(T::default(), |acc, &x| acc + x)
}
```

**In stats.rs**: The `compute()` function uses generics: `fn compute<I>(&self, sel: &Selection, it: I, weight_col_idx: Option<usize>) -> Vec<Stats>` where `I` is any iterator over CSV records.

### 5. **Iterators and the Iterator Trait**
```rust
// Iterator: Produces values one at a time
let vec = vec![1, 2, 3];
for item in vec.iter() {
    println!("{}", item);
}

// Higher-order iterator methods
let doubled: Vec<i32> = vec.iter()
    .map(|x| x * 2)
    .filter(|x| x > &2)
    .collect();
```

**In stats.rs**: CSV records are processed as iterators. This allows both sequential and parallel processing without code duplication.

### 6. **Unsafe Rust**
```rust
// Unsafe code: Bypasses compiler safety checks
// Use when you KNOW it's safe but compiler can't verify
unsafe {
    // Dereference raw pointer
    let ptr = data.as_ptr();
    let value = *ptr;
}

// Safety comments explain why it's safe
```

**In stats.rs**: Unsafe code is used in hot loops to skip bounds checking and null checks for performance. Every unsafe block has a "safety:" comment explaining why it's actually safe.

### 7. **Thread Safety and Synchronization**
```rust
// Channels: Thread-safe communication
use std::sync::mpsc;
let (sender, receiver) = mpsc::channel();
std::thread::spawn(move || {
    sender.send(42).unwrap();
});
let value = receiver.recv().unwrap();
```

**In stats.rs**: Uses `crossbeam_channel` for thread-safe communication between worker threads in parallel processing.

### 8. **Derive Macros and Attributes**
```rust
// #[derive(...)]: Auto-generate trait implementations
#[derive(Clone, Debug, Serialize, Deserialize)]
struct MyData {
    value: i32,
}

// Custom attributes
#[repr(C, align(64))] // Memory alignment
struct CacheAligned {
    data: u64,
}
```

**In stats.rs**: `#[repr(C, align(64))]` aligns `Stats` struct to CPU cache line size for performance.

---

## Architecture Overview

### High-Level Flow

```
User runs: qsv stats mydata.csv
    ↓
Entry Point: fn run(argv: &[&str])
    ↓
Parse Arguments
    ↓
Setup Configuration
    ↓
Check Cache → Cache Valid? → Output cached stats
    ↓ (Cache invalid or missing)
Check Index Exists → Has Index?
    ├─→ Yes: parallel_stats() (multi-threaded)
    └─→ No: sequential_stats() (single-threaded)
    ↓
Process Records:
    For each record:
        For each column:
            Infer type → Update statistics
    ↓
Convert Stats to CSV Records
    ↓
Save Cache Files
    ↓
Output Results (stdout or file)
```

### Key Components

```
src/cmd/stats.rs
├── Args (command-line arguments structure)
├── Stats (single column's statistics)
├── FieldType (type inference enum)
├── StatsData (serializable statistics output)
├── StatsArgs (cached configuration)
├── BooleanPattern (boolean inference configuration)
│
└── Functions:
    ├── run() → Main entry point
    ├── sequential_stats() → Single-threaded processing
    ├── parallel_stats() → Multi-threaded processing
    ├── compute() → Core computation loop
    ├── stats_to_records() → Output formatting
    └── ... (30+ helper functions)
```

### Module Dependencies

```
stats.rs depends on:
├── config → CSV reader/writer configuration
├── select → Column selection logic
├── util → Utility functions (memory checks, logging)
├── stats crate → Online statistics computation
├── csv → CSV parsing
├── serde → JSON serialization
└── crossbeam_channel → Thread communication
```

---

## Data Type System

### Type Inference Mechanism

The stats command infers one of six core data types for each column:

#### 1. **FieldType Enum**
```rust
#[derive(Clone, Copy, PartialEq, Eq)]
enum FieldType {
    TNull,      // All values are NULL/empty (default)
    TString,    // Contains text (fallback)
    TFloat,     // Contains decimal numbers
    TInteger,   // All values are integers
    TDate,      // Dates (e.g., "2024-01-15")
    TDateTime,  // Dates with times (e.g., "2024-01-15T10:30:00Z")
}
```

If `--infer-boolean` is enabled, stats also checks for boolean patterns (configurable with `--boolean-patterns` with the default `1:0,t*:f*,y*:n*`) and infers a boolean type if all values match those patterns and the cardinality is 2.

#### 2. **Type Inference Process**

For each cell value, the stats command tries to parse it in this order:

```
1. Is it empty/NULL? → TNull
2. Can it parse as integer? → TInteger
3. Can it parse as float? → TFloat
4. Should we try date parsing (check whitelist)?
   ├─→ Can parse as DateTime? → TDateTime
   └─→ Can parse as Date? → TDate
5. Default → TString
```

#### 3. **Implementation in `Stats` Struct**

```rust
struct Stats {
    typ: FieldType,           // Current inferred type
    // ... other fields
}

impl Stats {
    fn add(&mut self, 
           field: &[u8], 
           infer_date: bool, 
           infer_boolean: bool, 
           prefer_dmy: bool) {
        // Process one CSV cell value
        // Update typ based on parsing attempts
    }
}
```

#### 4. **Type Inference Example**

```csv
name,age,salary,joined_date
Alice,30,50000.50,2024-01-15
Bob,25,45000,2023-06-20
Charlie,,55000,2022-12-01
```

Inferred types:
- `name` → TString (contains text)
- `age` → TInteger (all integers, even with NULL)
- `salary` → TFloat (contains decimal)
- `joined_date` → TDate (if date inference enabled)

---

## Statistics Computation

### 1. **Streaming vs Non-Streaming Statistics**

**Streaming Statistics** (constant memory O(1)):
- Sum, min, max, range
- Sort order detection
- Mean, standard deviation, variance
- String length statistics (min, max, avg)
- Cardinality (when not counting unique values)
- Coefficient of variation

**Non-Streaming Statistics** (requires O(n) memory):
- Median (requires sorting all values)
- Quartiles (requires sorting)
- Modes and antimodes (requires frequency counting)
- Percentiles (requires sorting)
- Median Absolute Deviation (MAD)

### 2. **Online Statistics Using Welford's Algorithm**

For computing mean and standard deviation efficiently:

```rust
// Pseudocode of Welford's algorithm
mean = 0
M2 = 0  // Sum of squared differences

for each value x:
    count += 1
    delta = x - mean
    mean = mean + delta / count
    delta2 = x - mean
    M2 = M2 + delta * delta2

variance = M2 / (count - 1)
stddev = sqrt(variance)
```

**Why it's better**:
- Doesn't require storing all values
- Numerically stable (avoids precision issues)
- Used in the `OnlineStats` struct from the `stats` crate

### 3. **Stats Struct Fields (Cache-Aligned)**

```rust
#[repr(C, align(64))]  // Align to 64-byte cache line
struct Stats {
    // HOT DATA (frequently accessed)
    typ: FieldType,           // 1 byte
    is_ascii: bool,           // 1 byte
    max_precision: u16,       // 2 bytes
    nullcount: u64,           // 8 bytes
    sum_stotlen: u64,         // 8 bytes
    
    // CONFIGURATION
    which: WhichStats,        // Flags for what to compute
    
    // COMPUTATIONAL FIELDS
    sum: Option<TypedSum>,           // Numeric sum
    online: Option<OnlineStats>,     // Mean/variance
    online_len: Option<OnlineStats>, // String length stats
    modes: Option<Unsorted<Vec<u8>>>, // For mode computation
    unsorted_stats: Option<Unsorted<f64>>, // For median/quartiles
    minmax: Option<TypedMinMax>,     // Min/max values
}
```

**Cache-line alignment**: By aligning to 64 bytes (typical CPU cache line), multiple threads can access their own `Stats` objects without false sharing (cache coherency issues).

### 4. **Computation Example: Computing Mean**

```rust
// Creating a Stats object
let mut stats = Stats::new(WhichStats { sum: true, ... });

// Processing each value
for value in csv_row {
    if let Ok(num) = parse_number(value) {
        stats.online.mut_ref()
            .add(num as f64, 1.0);  // Welford's algorithm
    }
}

// Getting results
let mean = stats.online.ref_stat().mean();
let stddev = stats.online.ref_stat().std();
```

### 5. **TypedSum for Numeric Summation**

The stats command tracks sums using a struct with separate integer and float accumulators:

```rust
struct TypedSum {
    float:   Option<f64>,  // Float accumulator (None until a float is seen)
    integer: i64,          // Integer accumulator
    stotlen: u64,          // Sum of the total length of strings
}
```

---

## Processing Modes

### 1. **Sequential Processing** (No Index)

**When used**: CSV file has no index, or explicit single-thread requested

```rust
fn sequential_stats(&self, whitelist: &str) -> CliResult<(csv::ByteRecord, Vec<Stats>)> {
    let mut rdr = self.rconfig().reader()?;
    let (headers, sel) = self.sel_headers(&mut rdr)?;
    
    // Initialize date inference flags
    init_date_inference(self.flag_infer_dates, &headers, whitelist)?;
    
    // Single thread processes all records
    let stats = self.compute(&sel, rdr.byte_records());
    Ok((headers, stats))
}
```

**Flow**:
```
1. Open CSV file
2. Read headers
3. Apply column selection
4. Initialize type inference flags
5. For each record:
   - Parse fields
   - Update statistics
6. Return computed stats
```

**Performance**: O(n) time, O(m) space where n=records, m=columns

### 2. **Parallel Processing** (With Index)

**When used**: CSV file has an index, and num_jobs > 1

```rust
fn parallel_stats(&self, whitelist: &str, idx_count: u64) -> CliResult<...> {
    let mut rdr = self.rconfig().reader()?;
    let (headers, sel) = self.sel_headers(&mut rdr)?;
    init_date_inference(self.flag_infer_dates, &headers, whitelist)?;
    
    let njobs = util::njobs(self.flag_jobs);
    let chunk_size = util::chunk_size(idx_count as usize, njobs);
    let nchunks = util::num_of_chunks(idx_count as usize, chunk_size);
    
    // Create thread pool
    let pool = ThreadPool::new(njobs);
    let (send, recv) = crossbeam_channel::bounded(nchunks);
    
    // Each thread processes a chunk
    for i in 0..nchunks {
        pool.execute(move || {
            // Each thread:
            // 1. Opens its own file handle
            // 2. Seeks to its chunk's start (using index)
            // 3. Processes chunk_size records
            // 4. Sends results back via channel
            let stats = args.compute(&sel, it);
            send.send(stats).unwrap();
        });
    }
    
    // Merge results from all threads
    Ok((headers, merge_all(recv.iter()).unwrap_or_default()))
}
```

**Visualization**:
```
File: [AAAA|BBBB|CCCC|DDDD]  (4 chunks)
           ↓     ↓     ↓     ↓
      Thread0 Thread1 Thread2 Thread3
           ↓     ↓     ↓     ↓
        Stats Stats Stats Stats
           ↓_____|____|____|
               Merge
                 ↓
          Combined Stats
```

**Key Technique - Merge**: The `Commute` trait allows combining statistics from multiple threads:

```rust
trait Commute {
    fn merge(&mut self, other: Self);
}

// For statistics, merging means:
// - Combine online statistics (using Welford)
// - Merge min/max
// - Merge modes and unique values
// - Combine counts
```

---

## Performance Optimizations

### 1. **Unsafe Code for Hot Loop Optimization**

The `compute()` function is marked `#[inline]` and uses unsafe to avoid bounds checking:

```rust
#[inline]
fn compute<I>(&self, sel: &Selection, it: I, weight_col_idx: Option<usize>) -> Vec<Stats>
where
    I: Iterator<Item = csv::Result<csv::ByteRecord>>,
{
    // Pre-computation: cache flags in local variables (register allocation)
    let infer_date_flags = INFER_DATE_FLAGS.get().unwrap();
    let infer_boolean = self.flag_infer_boolean;
    let prefer_dmy = self.flag_prefer_dmy;
    
    for row in it {
        unsafe {
            // SAFETY: We know INFER_DATE_FLAGS has same size as stats vector
            // Compiler can't verify this, so we use unsafe to skip bounds check
            current_row = row.unwrap_unchecked();
            for field in sel.select(&current_row) {
                stats.get_unchecked_mut(i).add(
                    field,
                    *infer_date_flags.get_unchecked(i),
                    infer_boolean,
                    prefer_dmy,
                );
                i += 1;
            }
        }
    }
    stats
}
```

#### How stats decides if a file is “indexed”

Parallel processing only kicks in when the input is considered indexed. That decision is made by `Config::indexed()` and is used in stats like this:

```rust
match rconfig.indexed()? {
    Some(idx) => {
        // use idx.count() and go parallel
        args.parallel_stats(&args.flag_dates_whitelist, idx.count())
    }
    None => {
        // fall back to single-threaded
        args.sequential_stats(&args.flag_dates_whitelist)
    }
}
```

`Config::indexed()` returns Some when:
- A companion index file exists and is usable (typically the CSV path with a `.csv.idx` file computed via `util::idx_path(p)`, or an explicit `idx_path` set on the `Config`).
- If the index is stale (CSV is newer than the index), qsv transparently rebuilds it via `autoindex_file()` and then uses it.
- If no index exists, qsv may auto-create one when auto-indexing is enabled and the file size meets the threshold:
  - Global threshold: `QSV_AUTOINDEX_SIZE` env var (bytes).
  - Stats override: passing a negative `--cache-threshold` sets a per-run auto-index threshold to its absolute value (in bytes). If that negative value ends with `5` (e.g., `-5000005`), the created index (and stats cache) is deleted after the run.

`Config::indexed()` returns None (not indexed) when:
- Input is stdin (`-`): indexes aren’t supported for `<stdin>`.
- The input is Snappy-compressed (`.sz`): snappy files are not indexed.
- No index exists and auto-indexing isn’t triggered or the file is below the threshold.
- Auto-indexing is not configured and the file is large (≥ 100MB; `NO_INDEX_WARNING_FILESIZE`): qsv logs a warning but proceeds unindexed.

Useful flags and env vars:
- `--jobs <N>`: number of threads; `--jobs 1` forces sequential even if indexed.
- `QSV_AUTOINDEX_SIZE=<bytes>`: auto-create index for files ≥ this size.
- `--cache-threshold -<bytes>` (stats only): auto-index threshold for this run; append `5` to auto-delete index after.

**Safety**: The unsafe code is actually safe because:
- We initialize `INFER_DATE_FLAGS` with length == stats.len()
- We only access indices 0..stats.len()
- Thread-safe because each thread has its own iterator

### 2. **Cache-Line Alignment**

```rust
#[repr(C, align(64))]
struct Stats {
    // ...
}
```

**Why**: In parallel processing, multiple threads access different `Stats` objects. If they're on the same cache line, cache coherency overhead increases. Aligning to 64 bytes ensures each thread has its own cache line.

### 3. **OnceLock for Global Initialization**

```rust
static INFER_DATE_FLAGS: OnceLock<SmallVec<[bool; 50]>> = OnceLock::new();

// Initialize once, read many times
INFER_DATE_FLAGS.set(flags).ok();

// In hot loop:
let flags = INFER_DATE_FLAGS.get().unwrap(); // No locking, just reads
```

**Benefit**: Zero-cost initialization pattern. After first initialization, reads are just pointer dereferences with no overhead.

### 4. **SmallVec for Stack Allocation**

```rust
use smallvec::SmallVec;

// Allocates on stack for up to 50 bools, then heap
static INFER_DATE_FLAGS: OnceLock<SmallVec<[bool; 50]>> = OnceLock::new();
```

**Benefit**: Most CSVs have < 50 columns, so no heap allocation needed.

### 5. **Register Allocation Hints**

Frequently-used flags are cached in local variables:

```rust
let infer_boolean = self.flag_infer_boolean;  // Hint compiler: keep in register
let prefer_dmy = self.flag_prefer_dmy;        // Not in struct every iteration
```

**Result**: Compiler can allocate these to CPU registers instead of accessing memory each iteration.

---

## Caching System

### 1. **Cache Files**

For input `mydata.csv`, stats creates three files:

```
mydata.csv              (original input)
mydata.stats.csv        (computed statistics - CSV format)
mydata.stats.csv.json   (metadata about how stats were computed)
mydata.stats.csv.data.jsonl (optional: statistics in JSONL format)
```

### 2. **StatsArgs Structure**

Metadata stored in `.stats.csv.json`:

```rust
struct StatsArgs {
    arg_input: String,
    flag_select: String,
    flag_everything: bool,
    flag_infer_boolean: bool,
    // ... all other flags
    canonical_input_path: String,
    record_count: u64,
    date_generated: String,
    compute_duration_ms: u64,
    qsv_version: String,
}
```

### 3. **Cache Validation Logic**

```
Check if mydata.stats.csv exists:
  ├─→ No: Compute stats
  ├─→ Yes: Read mydata.stats.csv.json
  │
  Check if args match:
    ├─→ No: Recompute (flags changed)
    └─→ Yes: Check file modification time
    
  Check if stats_file newer than input_file:
    ├─→ Yes: Use cached stats
    └─→ No: Recompute (input changed)
```

### 4. **Cache Threshold Control**

The `--cache-threshold` flag controls caching behavior:

```
Default: 5000 (milliseconds)
  ├─→ If stats computation takes > 5000ms, cache results

--cache-threshold 0
  ├─→ Don't cache anything

--cache-threshold 1
  ├─→ Always cache

--cache-threshold -5000000
  ├─→ Create index if file > 5MB, keep cache and index after

--cache-threshold -5000005
  ├─→ Create temp index if file > 5MB, delete after run
```

---

## Code Walkthrough

### 1. **Main Entry: `fn run(argv: &[&str]) -> CliResult<()>`**

```rust
pub fn run(argv: &[&str]) -> CliResult<()> {
    // Step 1: Parse command-line arguments
    let mut args: Args = util::get_args(USAGE, argv)?;
    
    // Step 2: Handle typesonly mode (disable other stats)
    if args.flag_typesonly {
        args.flag_everything = false;
        args.flag_mode = false;
        // ... disable non-type stats
    }
    
    // Step 3: Setup boolean inference
    if args.flag_infer_boolean {
        let patterns = parse_boolean_patterns(&args.flag_boolean_patterns)?;
        BOOLEAN_PATTERNS.set(patterns)?;
    }
    
    // Step 4: Check environment variable overrides
    args.flag_prefer_dmy = args.flag_prefer_dmy 
        || util::get_envvar_flag("QSV_PREFER_DMY");
    
    // ... (continue with file I/O, caching, computation)
}
```

### 2. **Reading Input: Sequential vs Parallel Decision**

```rust
// After cache checks, decide processing strategy
let (headers, stats) = match rconfig.indexed()? {
    None => {
        // No index: use single thread
        record_count = util::count_rows(&rconfig)?;
        args.sequential_stats(&args.flag_dates_whitelist)?
    },
    Some(idx) => {
        // Index exists: use parallel processing
        record_count = idx.count();
        match args.flag_jobs {
            Some(1) => args.sequential_stats(...),
            _ => args.parallel_stats(..., record_count),
        }
    },
}?;
```

### 3. **Core Computation: `fn compute<I>()` **

This is the innermost loop, processing each record:

```rust
#[inline]
fn compute<I>(&self, sel: &Selection, it: I, weight_col_idx: Option<usize>) -> Vec<Stats>
where
    I: Iterator<Item = csv::Result<csv::ByteRecord>>,
{
    let sel_len = sel.len();
    let mut stats = self.new_stats(sel_len);
    
    // Cache flags for register allocation
    let infer_date_flags = INFER_DATE_FLAGS.get().unwrap();
    let infer_boolean = self.flag_infer_boolean;
    let prefer_dmy = self.flag_prefer_dmy;
    
    let mut i;
    for row in it {
        i = 0;
        unsafe {
            // Process each field in the row
            current_row = row.unwrap_unchecked();
            for field in sel.select(&current_row) {
                stats.get_unchecked_mut(i).add(
                    field,
                    *infer_date_flags.get_unchecked(i),
                    infer_boolean,
                    prefer_dmy,
                );
                i += 1;
            }
        }
    }
    stats
}
```

### 4. **Type Inference: `Stats::add()` Method**

```rust
impl Stats {
    fn add(&mut self, 
           field: &[u8], 
           infer_date: bool, 
           infer_boolean: bool, 
           prefer_dmy: bool) {
        
        // Empty field?
        if field.is_empty() {
            self.nullcount += 1;
            return;
        }
        
        let field_str = std::str::from_utf8(field).unwrap_or("");
        
        // Try to infer type in order
        // 1. Try integer
        if let Ok(int_val) = field_str.parse::<i64>() {
            self.typ = if self.typ == TNull { TInteger } else { TString };
            self.sum.as_mut().map(|s| s.add_integer(int_val));
            return;
        }
        
        // 2. Try float
        if let Ok(float_val) = field_str.parse::<f64>() {
            self.typ = TFloat;
            self.sum.as_mut().map(|s| s.add_float(float_val));
            return;
        }
        
        // 3. Try date if enabled
        if infer_date {
            if let Ok(date) = parse_date(field_str, prefer_dmy) {
                self.typ = TDate;
                // Update date statistics
                return;
            }
        }
        
        // 4. Default to string
        self.typ = TString;
    }
}
```

### 5. **Output Generation: `fn stats_to_records()`**

```rust
fn stats_to_records(&self, stats: Vec<Stats>, visualize_ws: bool) 
    -> Vec<csv::StringRecord> {
    
    let round_places = self.flag_round;
    let mut records = Vec::with_capacity(stats.len());
    
    // Create thread pool for parallel output generation
    let pool = ThreadPool::new(util::njobs(self.flag_jobs));
    let mut results = Vec::with_capacity(stats.len());
    
    // Each Stats object converted to a record in parallel
    for stat in stats {
        let (send, recv) = crossbeam_channel::bounded(0);
        results.push(recv);
        pool.execute(move || {
            send.send(stat.to_record(round_places, ...)).unwrap();
        });
    }
    
    // Collect results
    for recv in results.into_iter() {
        records.push(recv.recv().unwrap());
    }
    
    records
}
```

---

## Contributing Guide

### Getting Started

1. **Setup Development Environment**
   ```bash
   cd /path/to/qsv
   rustup update stable
   cargo build --release
   ```

2. **Run Existing Tests**
   ```bash
   cargo test --lib cmd::stats
   ```

3. **Create a Test File**
   ```bash
   echo "name,age,score
   Alice,30,95.5
   Bob,25,87.2
   Charlie,35," > test.csv
   ```

4. **Run Stats Command**
   ```bash
   ./target/release/qsv stats test.csv
   ```

### Common Contribution Areas

#### 1. **Adding a New Statistic**

Example: Add "min_fractional_width" (smallest number of decimal places)

**Steps**:
1. Add field to `StatsData` struct:
   ```rust
   pub struct StatsData {
       // ... existing fields
       pub min_fractional_width: Option<u32>,
   }
   ```

2. Add field to `Stats` struct:
   ```rust
   struct Stats {
       min_fractional_width: Option<u32>,
       // ...
   }
   ```

3. Update `Stats::new()` initialization:
   ```rust
   impl Stats {
       fn new(which: WhichStats) -> Self {
           Self {
               min_fractional_width: None,
               // ...
           }
       }
   }
   ```

4. Update `Stats::add()` to compute the value:
   ```rust
   fn add(&mut self, field: &[u8], ...) {
       if let Ok(float_val) = field_str.parse::<f64>() {
           // Compute fractional width
           let frac_part = format!("{}", float_val);
           if let Some(dot_pos) = frac_part.find('.') {
               let frac_digits = frac_part.len() - dot_pos - 1;
               self.min_fractional_width = Some(
                   self.min_fractional_width
                       .map_or(frac_digits as u32, |m| m.min(frac_digits as u32))
               );
           }
       }
   }
   ```

5. Update `Stats::to_record()` to include in output

6. Add to `stats_headers()` method

7. Write tests in `tests/test_stats.rs`

#### 2. **Optimizing Type Inference**

The current type inference is very strict. You could add options like:

- `--lenient-types`: Allow numeric columns with 5% null values to be treated as numeric
- `--type-hints`: Accept a JSON file specifying column types
- `--infer-uuid`: Detect UUID columns

**Implementation approach**:
1. Add new Args flags
2. Modify `FieldType` enum or add configuration
3. Update `Stats::add()` logic
4. Update type inference documentation

#### 3. **Adding Percentile Calculations**

The stats command already has percentile support but could be enhanced:

```rust
// Currently supported: --percentiles --percentile-list 5,10,25,50,75,90,95
// Enhancement: Add more percentile methods (interpolation types)
```

#### 4. **Improving Date Inference**

Current implementation:
- Matches 19 date formats
- Can whitelist columns by name
- Supports DMY/MDY preference

Possible improvements:
- Add support for more date formats
- Learn patterns from data (auto-detect format from samples)
- Support multiple date formats in same column

#### 5. **Memory Usage Optimization**

Areas to investigate:
- Profile memory usage with large files
- Use more efficient data structures for mode/cardinality
- Implement streaming quantiles (approximate quartiles)

### Testing Your Changes

```bash
# Run tests in test_stats.rs
cargo test --test test_stats -- --test-threads=1

# Run specific test
cargo test test_stats::integer_stats

# Run with logging
RUST_LOG=debug cargo test test_stats

# Build the release binary
cargo build --release

# Create test file and manually verify
echo "col1,col2
1,2
3,4" | ./target/release/qsv stats -
```

### Code Style and Standards

Per the project's `.github/copilot-instructions.md`:

1. **Use latest Rust features** (edition 2024, Rust 1.93+)
2. **Always include safety comments**:
   ```rust
   unsafe {
       // safety: We verified that the index is valid because...
       some_unsafe_operation();
   }
   ```

3. **Use meaningful variable names**
   ```rust
   // Good
   let max_precision = extract_precision(&value);
   
   // Bad
   let mp = extract_precision(&value);
   ```

4. **Document complex algorithms**:
   ```rust
   /// Computes online statistics using Welford's algorithm.
   /// This is numerically stable and requires O(1) memory.
   fn add_online(&mut self, value: f64) {
       // Implementation with comments explaining the math
   }
   ```

### Submitting a Contribution

1. **Create a feature branch**:
   ```bash
   git checkout -b feature/my-new-stat
   ```

2. **Make your changes** with meaningful commit messages

3. **Run all tests**:
   ```bash
   cargo test --lib cmd::stats
   cargo test --test test_stats
   ```

4. **Update documentation** if adding new flags or behaviors

5. **Open a pull request** with:
   - Clear description of changes
   - Motivation for the change
   - Test results
   - Any performance impact

### Debugging Tips

1. **Enable logging**:
   ```bash
   RUST_LOG=debug qsv stats myfile.csv
   ```

2. **Use conditional compilation**:
   ```rust
   #[cfg(debug_assertions)]
   eprintln!("Debug: {:?}", value);
   ```

3. **Profile with samply**:
   ```bash
   cargo install samply
   samply record ./target/release/qsv stats large-file.csv
   ```

4. **Use gdb/lldb**:
   ```bash
   rust-lldb ./target/debug/qsv -- stats test.csv
   ```

---

## Conclusion

The stats command is a masterclass in performant Rust:
- Leverages type system for safety
- Uses unsafe strategically for performance
- Implements clever caching to avoid redundant work
- Scales from single-thread to multi-threaded seamlessly
- Provides guaranteed type inference and statistics

By understanding these concepts and patterns, you'll be well-equipped to contribute to the stats command and improve qsv's data analysis capabilities!
