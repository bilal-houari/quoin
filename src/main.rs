use anyhow::Result;
use clap::{Parser, Subcommand};
use quoin::pandoc::PandocWrapper;
use quoin::styles::Profile;
use quoin::server::start_server;

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

        /// Use ultra-dense layout (8pt font, 2cm margins)
        #[arg(long, group = "density_level")]
        ultra_dense: bool,

        /// Use dense layout (10pt font, 2cm margins)
        #[arg(long, group = "density_level")]
        dense: bool,

        /// Use standard layout (10pt font, 2.5cm/3cm margins) [default]
        #[arg(long, group = "density_level")]
        standard: bool,

        /// Use comfort layout (12pt font, 2.5cm/3cm margins)
        #[arg(long, group = "density_level")]
        comfort: bool,

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

        /// Disable advanced code block styling (enabled by default)
        #[arg(long)]
        no_pretty_code: bool,

        /// Enable section numbering
        #[arg(long)]
        section_numbering: bool,

        /// Output Typst source instead of PDF (or in addition to it)
        #[arg(long)]
        typ: bool,

        /// Append a Table of Contents (Outline) at the end
        #[arg(long)]
        outline: bool,

        /// Override custom variables (e.g., -V cols=2)
        #[arg(short = 'V', long = "variable")]
        variables: Vec<String>,
    },
    /// Starts a local web server for live preview
    Server {
        /// Port to listen on
        #[arg(short, long, default_value = "3000")]
        port: u16,

        /// Only host the API (no webapp UI)
        #[arg(long)]
        api_only: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Convert { 
            input, 
            output, 
            ultra_dense, 
            dense, 
            standard: _standard, 
            comfort, 
            two_cols, 
            latex_font, 
            no_alt_table, 
            table_dims, 
            no_pretty_code, 
            section_numbering,
            typ,
            outline,
            variables 
        } => {
            let mut profile = Profile::new();

            // Set global defaults (grid, breakable blocks, etc.)
            profile.set_global_defaults();

            // Determine density (default to standard)
            let density = if *ultra_dense {
                "ultra-dense"
            } else if *dense {
                "dense"
            } else if *comfort {
                "comfort"
            } else {
                "standard"
            };

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

            // Apply pretty-code modifier unless disabled
            if !*no_pretty_code {
                profile.set_pretty_code();
            }

            // Apply section-numbering if requested
            if *section_numbering {
                profile.set_section_numbering(true);
            }

            // Apply outline if requested
            if *outline {
                profile.set_outline();
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
            if *typ {
                let typ_output = if output == "output.pdf" {
                    // Try to derive from input
                    if input == "-" {
                        "output.typ".to_string()
                    } else {
                        format!("{}.typ", input.split('.').next().unwrap_or(input))
                    }
                } else if output.ends_with(".pdf") {
                    output.replace(".pdf", ".typ")
                } else if output == "-" {
                    "-".to_string() // Stream to stdout
                } else {
                    format!("{}.typ", output)
                };
                
                PandocWrapper::convert(&profile, input, &typ_output)?;
            } else {
                PandocWrapper::convert(&profile, input, output)?;
            }
        }
        Commands::Server { port, api_only } => {
            start_server(*port, *api_only).await?;
        }
    }

    Ok(())
}
