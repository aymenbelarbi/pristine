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
        #[arg(long, value_enum, default_value = "markdown")]
        format: OutputFormat,
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
        #[arg(long, value_enum, default_value = "markdown")]
        format: OutputFormat,
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
        #[arg(long, value_enum, default_value = "markdown")]
        format: OutputFormat,
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
        #[arg(long, value_enum, default_value = "json")]
        format: OutputFormat,
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
        #[arg(long, value_enum, default_value = "redact")]
        policy: PolicyMode,
        /// Output format
        #[arg(long, value_enum, default_value = "markdown")]
        format: OutputFormat,
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
        Commands::Overview { source, revision, subpath, format, output, explain } => {
            tracing::info!("Generating overview for: {}", source);
            // Placeholder implementation
            println!("Overview pack for: {}", source);
        }
        Commands::Pack { source, query, revision, subpath, max_tokens, format, output, explain } => {
            tracing::info!("Generating pack for: {}", source);
            // Placeholder implementation
            println!("Task pack for: {}", source);
        }
        Commands::ReviewDiff { source, base, head, format, output, explain } => {
            tracing::info!("Generating review diff for: {}", source);
            // Placeholder implementation
            println!("Review pack for: {}..{}", base, head);
        }
        Commands::Agent { source, revision, format, output } => {
            tracing::info!("Generating agent pack for: {}", source);
            // Placeholder implementation
            println!("Agent pack for: {}", source);
        }
        Commands::SafeShare { source, revision, policy, format, output } => {
            tracing::info!("Generating safe share for: {}", source);
            // Placeholder implementation
            println!("Safe pack for: {}", source);
        }
        Commands::Config { action } => {
            match action {
                ConfigAction::Init => {
                    println!("Initializing configuration file...");
                }
                ConfigAction::Validate => {
                    println!("Validating configuration file...");
                }
                ConfigAction::Show => {
                    println!("Showing current configuration...");
                }
            }
        }
    }
}
