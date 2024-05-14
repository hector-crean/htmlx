use clap::{Parser, Subcommand};
use cli::{render_html::render_html, typegen::typegen};
use color_eyre::eyre::{self, Ok};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    Typegen {
        output_path: Option<String>,
    },
    RenderHtml {},
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Typegen { output_path } => {
            match output_path {
                Some(output_path) => {
                    typegen(output_path)?;
                }
                None => {}
            }
            Ok(())
        }
        Commands::RenderHtml {} => {
            render_html()?;

            Ok(())
        }
    }
}
