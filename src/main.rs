use std::time::Instant;
use clap::Parser;
use serde_json::Value;

mod models;
mod cli;
mod utils;

#[cfg(windows)]
mod collector;

#[cfg(not(windows))]
mod mock_collector;

use cli::Cli;

#[cfg(windows)]
use collector::Collector;

#[cfg(not(windows))]
use mock_collector::Collector;

fn main() {
    let start_time = Instant::now();
    
    // Parse command line arguments
    let cli = Cli::parse();
    
    #[cfg(not(windows))]
    println!("Note: Running in cross-platform compatibility mode. Full functionality only available on Windows.");
    
    // Initialize collector with CLI options
    let mut collector = Collector::new(cli.timeout_ms);
    
    // Configure collector based on CLI flags
    if cli.no_tally {
        collector.disable_tally_detection();
    }
    
    if let Some(host_port) = cli.tally_http {
        collector.set_tally_http_endpoint(host_port);
    }
    
    // Collect system information
    let mut result = collector.collect();
    
    // Filter fields if --select is specified
    if let Some(fields) = cli.select {
        result = collector.filter_fields(result, fields);
    }
    
    // Add collector metadata including duration
    let duration_ms = start_time.elapsed().as_millis() as u64;
    collector.add_metadata(&mut result, duration_ms);
    
    // Output the result
    if cli.pretty {
        println!("{}", serde_json::to_string_pretty(&result).unwrap());
    } else {
        println!("{}", serde_json::to_string(&result).unwrap());
    }
    
    // Start HTTP server if requested
    if let Some(http_port) = cli.http {
        #[cfg(feature = "http")]
        {
            #[cfg(windows)]
            {
                println!("Starting HTTP server on port {}...", http_port);
                collector::http::start_server(http_port, result);
            }
            
            #[cfg(not(windows))]
            {
                println!("HTTP server not available in cross-platform mode.");
            }
        }
        
        #[cfg(not(feature = "http"))]
        {
            eprintln!("HTTP server feature not enabled. Recompile with --features http");
        }
    }
}
