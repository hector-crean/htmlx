use std::path::PathBuf;

use clap::{Parser, Subcommand};
use cli::PROJECT_ROOT;
use cli::{
    pages::{ptsd_symptoms_node::PTSDSymptomsNode, Pagelike},
    typegen::typegen,
};
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
            let output_path: PathBuf = [PROJECT_ROOT, "src", "outputs", "ptsd-symptoms-node.html"]
                .iter()
                .collect();

            PTSDSymptomsNode::new(output_path).render_html()?;

            Ok(())
        }
    }
}
