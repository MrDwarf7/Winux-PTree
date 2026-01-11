mod cache;
mod traversal;
mod error;
mod cli;

#[cfg(windows)]
mod usn_journal;

use anyhow::Result;
use cli::{OutputFormat, ColorMode};
use std::time::Instant;

fn main() -> Result<()> {
    let program_start = Instant::now();

    // ========================================================================
    // Parse Command-Line Arguments
    // ========================================================================

    let args = cli::parse_args();

    // ========================================================================
    // Determine Color Output Settings
    // ========================================================================

    let use_colors = match args.color {
        ColorMode::Auto => atty::is(atty::Stream::Stdout),
        ColorMode::Always => true,
        ColorMode::Never => false,
    };

    // ========================================================================
    // Load or Create Cache
    // ========================================================================

    let cache_path = cache::get_cache_path()?;
    let mut cache = cache::DiskCache::open(&cache_path)?;

    // ========================================================================
    // Traverse Disk & Update Cache
    // ========================================================================

    let debug_info = traversal::traverse_disk(&args.drive, &mut cache, &args)?;

    // ========================================================================
    // Output Results
    // ========================================================================

    let output_start = Instant::now();
    if !args.quiet {
        let output = match args.format {
            OutputFormat::Tree => {
                if use_colors {
                    cache.build_colored_tree_output()?
                } else {
                    cache.build_tree_output()?
                }
            }
            OutputFormat::Json => cache.build_json_output()?,
        };
        println!("{}", output);
    }
    let output_elapsed = output_start.elapsed();

    // ========================================================================
    // Debug Output (Final Summary)
    // ========================================================================

    if args.debug {
        let total_elapsed = program_start.elapsed();
        print_debug_summary(&debug_info, output_elapsed, &cache_path, total_elapsed);
    }

    Ok(())
}

/// Print formatted debug summary
fn print_debug_summary(
    debug_info: &traversal::DebugInfo,
    output_time: std::time::Duration,
    cache_path: &std::path::Path,
    total_time: std::time::Duration,
) {
    eprintln!("\n{}", "=".repeat(70));
    eprintln!("{:^70}", "PERFORMANCE DEBUG INFO");
    eprintln!("{}", "=".repeat(70));

    eprintln!("\n{:<40} {}", "Execution Mode:", if debug_info.is_first_run { "FULL DISK SCAN (First Run)" } else if debug_info.cache_used { "CACHED (< 1 hour)" } else { "PARTIAL SCAN (Current Dir)" });
    eprintln!("{:<40} {}", "Scan Root:", debug_info.scan_root.display());

    eprintln!("\n{:<40} {}", "Directories Scanned:", format_number(debug_info.total_dirs));
    eprintln!("{:<40} {}", "Threads Used:", debug_info.threads_used);

    if !debug_info.cache_used {
        eprintln!("\n{:<40} {:.3}s", "Traversal Time:", debug_info.traversal_time.as_secs_f64());
        eprintln!("{:<40} {:.3}s", "Cache Save Time:", debug_info.save_time.as_secs_f64());
    }
    eprintln!("{:<40} {:.3}s", "Output Formatting Time:", output_time.as_secs_f64());
    eprintln!("{:<40} {:.3}s", "Total Time:", total_time.as_secs_f64());

    eprintln!("\n{:<40} {}", "Cache Location:", cache_path.display());
    eprintln!("{}", "=".repeat(70));
    eprintln!();
}

/// Format large numbers with thousands separator
fn format_number(n: usize) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.insert(0, ',');
        }
        result.insert(0, c);
    }
    result
}
