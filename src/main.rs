use anyhow::Result;
use clap::{Parser, Subcommand};
use quoin::pandoc::PandocWrapper;
use quoin::styles::Profile;

#[derive(Parser)]
#[command(name = "quoin")]
#[command(about = "CLI engine for professional PDF generation via Pandoc & Typst", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Converts a document to PDF
    Convert {
        /// Input file path (use '-' for stdin)
        input: String,

        /// Output file path (use '-' for stdout)
        #[arg(short, long, default_value = "output.pdf")]
        output: String,

        /// Style preset to use (ultra-dense, dense, standard, comfort, etc.)
        #[arg(short, long)]
        styles: Vec<String>,

        /// Enable advanced code block styling
        #[arg(long)]
        pretty_code: bool,

        /// Override custom variables (e.g., -V cols=2)
        #[arg(short = 'V', long = "variable")]
        variables: Vec<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Convert { input, output, styles, pretty_code, variables } => {
            let mut profile = Profile::new();

            // Apply style presets if provided
            for style_name in styles {
                profile.apply_preset(style_name);
            }

            // Apply pretty-code modifier if flag is set
            if *pretty_code {
                profile.set_pretty_code();
            }

            // Apply custom variable overrides
            for var in variables {
                if let Some((key, value)) = var.split_once('=') {
                    profile.override_variable(key, value);
                } else {
                    // If no '=', treat as boolean true or just key
                    profile.override_variable(var, "true");
                }
            }

            // Execute conversion
            PandocWrapper::convert(&profile, input, output)?;
        }
    }

    Ok(())
}
