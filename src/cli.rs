use clap::Parser;

/// QuickSys - A fast system information collector
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Cli {
    /// Pretty-print JSON output
    #[clap(long)]
    pub pretty: bool,
    
    /// Select specific fields to include in output (comma-separated)
    /// Example: os,cpu,apps.tally
    #[clap(long)]
    pub select: Option<String>,
    
    /// Skip Tally software detection
    #[clap(long)]
    pub no_tally: bool,
    
    /// Probe Tally HTTP endpoint (default: 127.0.0.1:9000)
    #[clap(long)]
    pub tally_http: Option<String>,
    
    /// Global timeout in milliseconds
    #[clap(long, default_value = "500")]
    pub timeout_ms: u64,
    
    /// Start local HTTP server on specified port
    #[clap(long)]
    pub http: Option<u16>,
}