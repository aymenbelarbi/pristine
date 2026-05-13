//! Pristine CLI
//!
//! Command-line interface for Pristine.

use clap::{Parser, Subcommand};
use pristine_core::*;

#[derive(Parser)]
#[command(name = "pristine", version, about = "Code Context Compiler")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Configuration file path
    #[arg(long, global = true)]
    config: Option<String>,

    /// Enable verbose output
    #[arg(long, short, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate an overview artifact
    Overview {
        /// Source repository (URL or local path)
        source: String,
        /// Git revision (branch, tag, commit)
        #[arg(long)]
        revision: Option<String>,
        /// Subdirectory path
        #[arg(long)]
        subpath: Option<String>,
        /// Output format
        #[arg(long, default_value = "markdown")]
        format: String,
        /// Output file (default: stdout)
        #[arg(long, short)]
        output: Option<String>,
        /// Show selection reasons
        #[arg(long)]
        explain: bool,
    },
    /// Generate a task-focused context pack
    Pack {
        /// Source repository (URL or local path)
        source: String,
        /// Task or query describing needed context
        #[arg(long)]
        query: Option<String>,
        /// Git revision
        #[arg(long)]
        revision: Option<String>,
        /// Subdirectory path
        #[arg(long)]
        subpath: Option<String>,
        /// Maximum token budget
        #[arg(long)]
        max_tokens: Option<u32>,
        /// Output format
        #[arg(long, default_value = "markdown")]
        format: String,
        /// Output file
        #[arg(long, short)]
        output: Option<String>,
        /// Show selection reasons
        #[arg(long)]
        explain: bool,
    },
    /// Generate a review pack for a diff
    ReviewDiff {
        /// Source repository
        source: String,
        /// Base revision
        #[arg(long)]
        base: String,
        /// Head revision
        #[arg(long)]
        head: String,
        /// Output format
        #[arg(long, default_value = "markdown")]
        format: String,
        /// Output file
        #[arg(long, short)]
        output: Option<String>,
        /// Show selection reasons
        #[arg(long)]
        explain: bool,
    },
    /// Generate an agent pack
    Agent {
        /// Source repository
        source: String,
        /// Git revision
        #[arg(long)]
        revision: Option<String>,
        /// Output format
        #[arg(long, default_value = "json")]
        format: String,
        /// Output file
        #[arg(long, short)]
        output: Option<String>,
    },
    /// Generate a safe-to-share artifact
    SafeShare {
        /// Source repository
        source: String,
        /// Git revision
        #[arg(long)]
        revision: Option<String>,
        /// Policy mode
        #[arg(long, default_value = "redact")]
        policy: String,
        /// Output format
        #[arg(long, default_value = "markdown")]
        format: String,
        /// Output file
        #[arg(long, short)]
        output: Option<String>,
    },
    /// Initialize configuration file
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Initialize a new configuration file
    Init,
    /// Validate configuration file
    Validate,
    /// Show current configuration
    Show,
}

/// Parse output format from string
fn parse_output_format(s: &str) -> Result<OutputFormat> {
    match s.to_lowercase().as_str() {
        "json" => Ok(OutputFormat::Json),
        "markdown" | "md" => Ok(OutputFormat::Markdown),
        "xml" => Ok(OutputFormat::Xml),
        "text" | "txt" => Ok(OutputFormat::Text),
        _ => Err(PristineError::InvalidSource(format!(
            "Unknown output format: {}",
            s
        ))),
    }
}

/// Parse policy mode from string
fn parse_policy_mode(s: &str) -> Result<PolicyMode> {
    match s.to_lowercase().as_str() {
        "allow" => Ok(PolicyMode::Allow),
        "redact" => Ok(PolicyMode::Redact),
        "fail" => Ok(PolicyMode::Fail),
        _ => Err(PristineError::InvalidSource(format!(
            "Unknown policy mode: {}",
            s
        ))),
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Initialize tracing
    let filter = if cli.verbose {
        "pristine=debug"
    } else {
        "pristine=info"
    };

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .init();

    match cli.command {
        Commands::Overview {
            source,
            revision: _,
            subpath: _,
            format,
            output: _,
            explain: _,
        } => {
            let _format = match parse_output_format(&format) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            };
            tracing::info!("Generating overview for: {}", source);
            println!("Overview pack for: {}", source);
        }
        Commands::Pack {
            source,
            query: _,
            revision: _,
            subpath: _,
            max_tokens: _,
            format,
            output: _,
            explain: _,
        } => {
            let _format = match parse_output_format(&format) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            };
            tracing::info!("Generating pack for: {}", source);
            println!("Task pack for: {}", source);
        }
        Commands::ReviewDiff {
            source,
            base,
            head,
            format,
            output: _,
            explain: _,
        } => {
            let _format = match parse_output_format(&format) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            };
            tracing::info!("Generating review diff for: {}", source);
            println!("Review pack for: {}..{}", base, head);
        }
        Commands::Agent {
            source,
            revision: _,
            format,
            output: _,
        } => {
            let _format = match parse_output_format(&format) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            };
            tracing::info!("Generating agent pack for: {}", source);
            println!("Agent pack for: {}", source);
        }
        Commands::SafeShare {
            source,
            revision: _,
            policy,
            format,
            output: _,
        } => {
            let _policy = match parse_policy_mode(&policy) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            };
            let _format = match parse_output_format(&format) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            };
            tracing::info!("Generating safe share for: {}", source);
            println!("Safe pack for: {}", source);
        }
        Commands::Config { action } => match action {
            ConfigAction::Init => {
                println!("Initializing configuration file...");
            }
            ConfigAction::Validate => {
                println!("Validating configuration file...");
            }
            ConfigAction::Show => {
                println!("Showing current configuration...");
            }
        },
    }
}
