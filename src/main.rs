use anyhow::Result;
use clap::{Parser, Subcommand};
use quoin::pandoc::PandocWrapper;
use quoin::server::start_server;
use quoin::styles::Profile;

#[derive(Parser)]
#[command(name = "quoin")]
#[command(
    about = "A Markdown-to-PDF engine leveraging Pandoc & Typst for professional-grade typesetting.",
    long_about = "Quoin is a local-first document engine that combines the simplicity of Markdown with the typographic power of Typst. It features a built-in web server for live-reloading previews and a highly configurable CLI for automated workflows."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Increase verbosity level (can be used multiple times)
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    verbose: u8,
}

#[derive(Subcommand)]
enum Commands {
    /// Converts a document to PDF
    #[command(next_display_order = None)] // Allows us to manualy control order
    Convert {
        /// Input file path (use '-' for stdin)
        #[arg(display_order = 1)]
        input: String,

        /// Output file path (use '-' for stdout)
        #[arg(short, long, default_value = "output.pdf", display_order = 2)]
        output: String,

        // --- Layout Group ---
        /// Use ultra-dense layout (8pt font, 2cm margins). Ideal for cheat sheets.
        #[arg(long, group = "density_level", help_heading = "Layout Options", display_order = 10)]
        ultra_dense: bool,

        /// Use dense layout (10pt font, 2cm margins). Compact but readable.
        #[arg(long, group = "density_level", help_heading = "Layout Options", display_order = 11)]
        dense: bool,

        /// Use standard layout (10pt font, 2.5cm/3cm margins). [default]
        #[arg(long, group = "density_level", help_heading = "Layout Options", display_order = 12)]
        standard: bool,

        /// Use comfort layout (12pt font, 2.5cm/3cm margins). Maximum readability.
        #[arg(long, group = "density_level", help_heading = "Layout Options", display_order = 13)]
        comfort: bool,

        /// Enable 2-column layout (Note: may cause overlapping with large tables)
        #[arg(long, help_heading = "Layout Options", display_order = 14)]
        two_cols: bool,

        // --- Styling Group ---
        /// Enable "New Computer Modern" LaTeX-style font for that academic look
        #[arg(long, help_heading = "Formatting & Style", display_order = 20)]
        latex_font: bool,

        /// Disable advanced code block styling (syntax highlighting & background)
        #[arg(long, help_heading = "Formatting & Style", display_order = 21)]
        no_pretty_code: bool,

        /// Disable alternative table styling (enabled by default)
        #[arg(long, help_heading = "Formatting & Style", display_order = 22)]
        no_alt_table: bool,

        /// Restore default Pandoc table dimensions (overrides custom filter)
        #[arg(long, help_heading = "Formatting & Style", display_order = 23)]
        table_dims: bool,

        // --- Document Features ---
        /// Enable section numbering (e.g., 1.1, 1.2)
        #[arg(long, help_heading = "Document Features", display_order = 30)]
        section_numbering: bool,

        /// Append a Table of Contents (Outline) at the end of the document
        #[arg(long, help_heading = "Document Features", display_order = 31)]
        outline: bool,

        // --- Advanced ---
        /// Output Typst source instead of PDF (or in addition to it if output ends in .typ)
        #[arg(long, help_heading = "Advanced", display_order = 40)]
        typ: bool,

        /// Override custom variables or set Typst metadata (e.g., -V lang=fr -V cols=2)
        #[arg(short = 'V', long = "variable", help_heading = "Advanced", display_order = 41)]
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

    // Initialize logging
    // Default (0): INFO for quoin and general
    // -v (1): DEBUG for quoin, INFO for others
    // -vv (2+): TRACE for quoin, DEBUG for others
    let filter = match cli.verbose {
        0 => "info,quoin=info",
        1 => "info,quoin=debug",
        _ => "debug,quoin=trace",
    };

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();

    tracing::info!("Quoin starting...");

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
            variables,
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
            tracing::info!("Conversion completed successfully.");
        }
        Commands::Server { port, api_only } => {
            start_server(*port, *api_only).await?;
        }
    }

    Ok(())
}
