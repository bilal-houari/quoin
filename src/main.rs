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

        /// Density preset to use (ultra-dense, dense, standard, comfort)
        #[arg(short, long, default_value = "standard")]
        density: String,

        /// Enable 2-column layout
        #[arg(long)]
        two_cols: bool,

        /// Disable alternative table styling (enabled by default)
        #[arg(long)]
        no_alt_table: bool,

        /// Restore default Pandoc table dimensions (overrides custom filter)
        #[arg(long)]
        table_dims: bool,

        /// Enable "New Computer Modern" LaTeX-style font
        #[arg(long)]
        latex_font: bool,

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
        Commands::Convert { input, output, density, two_cols, latex_font, no_alt_table, table_dims, pretty_code, variables } => {
            let mut profile = Profile::new();

            // Set density settings
            profile.set_density(density);

            // Set column count
            profile.set_two_cols(*two_cols);

            // Warning 2-column layouts
            if *two_cols {
                eprintln!("Warning: Using 2 columns may cause text collisions.");
            }

            // Apply LaTeX-style font if requested
            if *latex_font {
                profile.set_latex_font();
            }

            // Apply alternative table styling unless disabled
            if !*no_alt_table {
                profile.set_alt_table();
            }

            // Disable Lua table filter if table-dims is requested
            if *table_dims {
                profile.use_lua_table_filter = false;
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
