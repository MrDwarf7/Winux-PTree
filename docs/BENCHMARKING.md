# Benchmarking with Criterion

## Overview

ptree uses [Criterion.rs](https://bheisler.github.io/criterion.rs/book/) for statistical benchmarking. Criterion provides detailed performance analysis with confidence intervals, regression detection, and HTML reports.

## Running Benchmarks

### Run All Benchmarks
```bash
cargo bench --release
```

### Run Specific Benchmark
```bash
# Run only tree traversal benchmarks
cargo bench --release -- tree_traversal

# Run only sorting benchmarks
cargo bench --release -- directory_sorting
```

### Run With Verbose Output
```bash
cargo bench --release -- --verbose
```

### Generate HTML Report
Criterion automatically generates HTML reports in `target/criterion/`:
```
target/criterion/tree_traversal/report/index.html
target/criterion/directory_sorting/report/index.html
```

## Benchmarks Included

### 1. Tree Traversal (`tree_traversal`)
Measures directory tree traversal performance with varying directory counts.

**Test cases:**
- 64 directories (depth 3, breadth 4)
- 192 directories (depth 4, breadth 3)
- 320 directories (depth 5, breadth 2)

**What it measures:**
- Raw directory enumeration speed
- I/O throughput
- Effect of directory depth on performance

**Typical output:**
```
tree_traversal/64 dirs           time:   [45.2 ms 46.1 ms 47.2 ms]
tree_traversal/192 dirs          time:   [125.3 ms 127.8 ms 130.5 ms]
tree_traversal/320 dirs          time:   [198.2 ms 201.5 ms 205.3 ms]
```

### 2. Directory Sorting (`directory_sorting`)
Measures performance of sequential sorting on directory names.

**Test cases:**
- 10 items
- 50 items
- 100 items
- 500 items
- 1000 items

**What it measures:**
- Single-threaded sorting performance
- Effect of data size on sort time
- Baseline for parallel sorting comparison

**Typical output:**
```
directory_sorting/10 items        time:   [234 ns 236 ns 239 ns]
directory_sorting/100 items       time:   [2.34 µs 2.38 µs 2.42 µs]
directory_sorting/1000 items      time:   [28.4 µs 29.1 µs 29.8 µs]
```

### 3. Parallel Sorting (`parallel_sorting`)
Compares sequential vs parallel sorting at various thresholds.

**Test cases:**
- Sequential: 50, 100, 500, 1000, 5000 items
- Parallel: 50, 100, 500, 1000, 5000 items

**What it measures:**
- When parallel sorting becomes beneficial
- Thread overhead vs computation cost
- Optimal threshold for switching

**Typical output:**
```
parallel_sorting/sequential_50    time:   [456 ns 461 ns 467 ns]
parallel_sorting/parallel_50      time:   [2.34 µs 2.39 µs 2.44 µs]   (overhead!)
...
parallel_sorting/sequential_5000  time:   [284 µs 291 µs 299 µs]
parallel_sorting/parallel_5000    time:   [85.4 µs 87.2 µs 89.3 µs]    (much faster!)
```

### 4. Cache Operations (`cache_operations`)
Measures serialization and deserialization performance.

**Test cases:**
- Serialize: 100, 1000, 10000 entries
- Deserialize: 100, 1000, 10000 entries

**What it measures:**
- Cache I/O overhead
- Bincode serialization speed
- Effect of cache size on save/load time

**Typical output:**
```
cache_operations/serialize_100    time:   [12.3 µs 12.8 µs 13.4 µs]
cache_operations/deserialize_100  time:   [8.45 µs 8.72 µs 9.02 µs]
cache_operations/serialize_10000  time:   [1.23 ms 1.28 ms 1.34 ms]
```

### 5. File Enumeration (`file_enumeration`)
Measures directory listing performance with various file counts.

**Test cases:**
- 10 files per directory
- 50 files per directory
- 100 files per directory
- 500 files per directory

**What it measures:**
- File system API overhead
- Directory size impact
- Typical case performance

**Typical output:**
```
file_enumeration/10 files         time:   [234 µs 241 µs 249 µs]
file_enumeration/100 files        time:   [1.23 ms 1.28 ms 1.34 ms]
file_enumeration/500 files        time:   [5.67 ms 5.89 ms 6.14 ms]
```

## Understanding Criterion Output

### Basic Format
```
benchmark_name/test_case          time:   [lower 95% upper 95%]
                                   change: [+X.X% +Y.Y% +Z.Z%]
```

### Example Interpretation
```
tree_traversal/64 dirs            time:   [45.2 ms 46.1 ms 47.2 ms]
                                   change: [+0.5% +1.2% +2.1%]
```

- **Time**: Average is 46.1 ms, confidence interval is 45.2-47.2 ms (95%)
- **Change**: Compared to previous run, 0.5-2.1% slower (regression detection)
- **[FASTER] / [SLOWER]**: Significant change from baseline

## Running Benchmarks Effectively

### 1. Baseline Comparison
```bash
# First run: establish baseline
cargo bench --release > baseline.txt

# After code changes
cargo bench --release > after_changes.txt

# Criterion will show regression/improvement automatically
```

### 2. Benchmark Single Component
```bash
# Only cache operations
cargo bench --release -- cache_operations

# Only sorting
cargo bench --release -- sorting
```

### 3. With More Samples (More Accurate)
```bash
# Takes longer but more precise
CRITERION_OVERRIDES='sample_size=100' cargo bench --release
```

### 4. Disable Regression Detection
```bash
# Just get current numbers, ignore baselines
cargo bench --release -- --save-baseline "my_baseline"
cargo bench --release -- --baseline "my_baseline"
```

## Expected Performance

### On Modern Hardware (SSD, 8+ cores)

| Benchmark | Expected Time |
|-----------|---------------|
| Tree traversal (64 dirs) | 40-60 ms |
| Tree traversal (192 dirs) | 120-160 ms |
| Tree traversal (320 dirs) | 190-250 ms |
| Directory sort (1000 items) | 25-35 µs |
| Parallel sort (5000 items) | 80-120 µs |
| Cache serialize (10K entries) | 1.0-1.5 ms |
| File enumeration (100 files) | 1.0-1.5 ms |

### Slower Hardware (HDD, 4 cores)

Expect 2-3x slower traversal times due to I/O latency.

## Optimizing Based on Benchmarks

### If Traversal is Slow
1. Check for I/O bottlenecks (use `--debug` flag)
2. Consider `--skip` to exclude large directories
3. Benchmark shows baseline for your hardware

### If Sorting is Slow
1. Threshold (100 items) is already optimized
2. Parallel sorting shows speedup at 1000+ items
3. Most directories have < 100 children (fast path)

### If Cache Operations are Slow
1. Bincode is very efficient
2. Slow save = slow disk write speed
3. Consider SSD for better performance

### If File Enumeration is Slow
1. This is filesystem dependent
2. Use `-j` to adjust thread count
3. NTFS can be slower than ext4

## Criterion Features

### Regression Detection
Criterion automatically compares against previous baselines:
```
[SLOWER]  tree_traversal/64 dirs   time: [+5.2% +8.1% +11.2%]
```

### Outlier Detection
Identifies and handles outliers:
```
   Outliers: 1 (4.6%) low severe
             1 (4.6%) high mild
```

### Statistical Analysis
Provides confidence intervals and standard deviation:
```
Mean                        46.1 ms
Std. Dev.                   2.1 ms
Median                      45.8 ms
MAD                         1.8 ms
```

## HTML Reports

After each benchmark run, open:
```
target/criterion/report/index.html
```

Features:
- Time series graphs (performance over time)
- Distribution histograms
- Comparison plots
- Outlier analysis
- Regression detection

## Continuous Benchmarking

### Setup
```bash
# Create baseline
cargo bench --release -- --save-baseline "main"

# After making changes
cargo bench --release -- --baseline "main"
```

### Compare Multiple Baselines
```bash
cargo bench --release -- --baseline "main" --verbose
```

## Troubleshooting

### Benchmarks Taking Too Long
```bash
# Reduce sample size
CRITERION_OVERRIDES='sample_size=5' cargo bench --release
```

### Results are Noisy
```bash
# Run on system at rest, close other applications
# Or increase sample size:
CRITERION_OVERRIDES='sample_size=50' cargo bench --release
```

### Can't Find HTML Report
Look in: `target/criterion/report/index.html`

## Summary

Criterion provides:
✅ Statistical rigor (confidence intervals)  
✅ Regression detection (catch slowdowns)  
✅ HTML reports (visualize performance)  
✅ Multiple test cases (comprehensive coverage)  
✅ Customizable thresholds (flexible analysis)  

Use benchmarks to:
1. **Baseline**: Understand current performance
2. **Optimize**: Identify bottlenecks
3. **Validate**: Verify improvements
4. **Regress test**: Catch slowdowns early
5. **Document**: Show performance claims
