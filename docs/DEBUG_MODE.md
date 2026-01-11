# Debug Mode - Performance Timing

## Overview

The `--debug` flag enables detailed timing and performance metrics, useful for understanding where time is spent during execution and comparing first-run vs subsequent-run performance.

## Usage

```bash
# Enable debug output
ptree.exe --debug

# Combine with other flags
ptree.exe --debug --force
ptree.exe --debug --color always
ptree.exe --debug --format json
```

## Output Example

### First Run (Full Scan)
```
[DEBUG] First run detected (cache empty) - scanning full drive: C:\
[DEBUG] Using 16 threads for traversal
[DEBUG] === Performance Summary ===
[DEBUG] Total directories scanned: 125847
[DEBUG] Traversal time: 8.234s
[DEBUG] Cache save time: 0.142s
[DEBUG] Total time: 8.376s
[DEBUG] Scan root: C:\
[DEBUG] Cache location: C:\Users\YourName\AppData\Roaming\ptree\cache\ptree.dat
[DEBUG] Output formatting time: 1.823s
```

### Subsequent Run (Current Directory Only)
```
[DEBUG] Subsequent run detected (cache exists) - scanning current directory: C:\Users\YourName\Projects
[DEBUG] Using 16 threads for traversal
[DEBUG] === Performance Summary ===
[DEBUG] Total directories scanned: 487
[DEBUG] Traversal time: 0.082s
[DEBUG] Cache save time: 0.015s
[DEBUG] Total time: 0.097s
[DEBUG] Scan root: C:\Users\YourName\Projects
[DEBUG] Cache location: C:\Users\YourName\AppData\Roaming\ptree\cache\ptree.dat
[DEBUG] Output formatting time: 0.003s
```

## Understanding the Output

### Detection Logging

| Message | Meaning |
|---------|---------|
| `First run detected (cache empty)` | Cache doesn't exist - performing full drive scan |
| `Subsequent run detected (cache exists)` | Cache found - scanning only current directory |
| `Cache is fresh (age: N seconds, < 3600)` | Cache is < 1 hour old - using cached data, no rescan |

### Performance Metrics

- **Total directories scanned**: Number of directories found in scan
- **Traversal time**: Time spent walking the directory tree (multi-threaded)
- **Cache save time**: Time to serialize and write cache to disk
- **Total time**: Overall traversal + save time (not including output)
- **Output formatting time**: Time to generate and format output
- **Threads**: Number of threads used for parallel traversal

## Performance Comparison

Typical performance differences:

### First Run (Full Drive Scan)
```
Example: 125K directories on C: drive
Traversal time: 8-15 seconds (I/O bound)
Cache save time: 0.1-0.2 seconds
Total: 8-15 seconds
```

### Subsequent Run (Current Directory Only)
```
Example: 500 directories in current folder
Traversal time: 0.05-0.1 seconds
Cache save time: 0.01-0.02 seconds
Total: 0.05-0.1 seconds
```

### Cached Run (Within 1 Hour)
```
Example: Using cached data
Traversal time: 0 seconds (skipped)
Output formatting time: 1-2 seconds
Total: 1-2 seconds
```

**Performance improvement**: 100-200x faster on subsequent runs!

## Interpreting Results

### High Traversal Time
- **Cause**: Slow disk (HDD), large directory tree, or many files
- **Solution**: Use `-j 2` for slower I/O devices, or `--skip` to exclude large directories

### High Cache Save Time
- **Cause**: Slow disk or very large cache (millions of entries)
- **Solution**: Normal for large directory trees, data is compressed

### Slow Output Formatting
- **Cause**: Very large directory tree (100K+ directories)
- **Solution**: Normal, tree rendering is O(n), use `--format json` for faster output

## Use Cases

### 1. Verify First-Run vs Subsequent-Run Performance
```bash
# First run: shows full scan timing
ptree.exe --debug --force

# Subsequent run: shows current-directory-only scanning
cd "C:\Some\Other\Directory"
ptree.exe --debug
```

### 2. Benchmark Thread Count
```bash
# Single thread
ptree.exe --debug -j 1 --force

# Multiple threads
ptree.exe --debug -j 8 --force
```

### 3. Identify Performance Bottlenecks
```bash
# Analyze timing breakdown
ptree.exe --debug --force

# If traversal time is high: disk is slow
# If save time is high: drive is congested
# If output time is high: directory tree is very large
```

### 4. Verify Cache Freshness
```bash
# First run
ptree.exe --debug

# Within 1 hour (should use cache)
ptree.exe --debug

# After 1 hour (should rescan)
ptree.exe --debug
```

## Hardware Comparison

### Fast NVMe SSD
```
Traversal time: 2-4 seconds (even for 200K dirs)
Cache save time: <100ms
Output formatting: <1 second
```

### Standard SATA SSD
```
Traversal time: 5-10 seconds (for 150K dirs)
Cache save time: 100-300ms
Output formatting: 1-2 seconds
```

### Mechanical Hard Drive
```
Traversal time: 15-30 seconds (for 100K dirs)
Cache save time: 200-500ms
Output formatting: 1-2 seconds
```

### USB Drive / Network Share
```
Traversal time: 30+ seconds (highly variable)
Cache save time: 500ms+
Recommendation: Use `-j 2` or `-j 1` to avoid overwhelming device
```

## Combining With Other Flags

### Check Threading Performance
```bash
ptree.exe --debug -j 1 --force    # Single thread
ptree.exe --debug -j 8 --force    # Multi-threaded
# Compare times to see parallelism benefit
```

### Measure Output Format Overhead
```bash
ptree.exe --debug --format tree    # ASCII output
ptree.exe --debug --format json    # JSON output
# Compare "Output formatting time"
```

### Verify Skip Filter Impact
```bash
ptree.exe --debug --force                    # All directories
ptree.exe --debug --skip "Windows,Program Files" --force
# Compare directory counts and traversal times
```

## Performance Tips from Debug Output

### If Traversal Time is High
1. Check if scanning full drive (first run) vs current dir
2. Hardware: Use `-j 2` for slow I/O
3. Filtering: Use `--skip` to exclude large system directories

### If Output Time is High
1. Normal for very large trees (100K+ directories)
2. Try `--format json` for ~10% faster output
3. Consider using `--max-depth` to limit output depth

### If Cache Save Time is High
1. Normal on slow drives
2. Data is compressed for storage efficiency
3. Run `--quiet --force` to skip output and see pure traversal time

## Troubleshooting

### Debug Output Not Showing
- Ensure you use `--debug` flag
- Debug output goes to stderr, not stdout
- When piping, use: `ptree.exe --debug 2>&1 | tee log.txt`

### Timing Seems Wrong
- First run timing includes full disk scan (unavoidable)
- Subsequent run timing is current directory only (much faster)
- Times are wall-clock, not CPU time (includes I/O wait)

### Too Much Debug Output
- Combine with `--quiet` to suppress tree output
- Debug info still printed: `ptree.exe --debug --quiet --force`

## Summary

Use `--debug` to:
- ✅ Understand performance characteristics
- ✅ Verify first-run vs subsequent-run behavior
- ✅ Identify bottlenecks
- ✅ Compare different thread counts
- ✅ Benchmark against other tools
- ✅ Troubleshoot slow performance

The debug output clearly shows the dramatic performance improvement between first runs (full scan) and subsequent runs (current directory only).
