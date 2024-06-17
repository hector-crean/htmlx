use blocks::block::nav::NavProps;
use blocks::node::Routes;
use clap::{Parser, Subcommand};
use cli::routes::App;
use cli::typegen::typegen;
use cli::PROJECT_ROOT;
use color_eyre::eyre::{self, Ok};
use maud::Render;
use std::fs;
use std::io::Write;

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
    RenderHtml,
    GenerateNodeMap,
    JsToTs {
        root_directory_path: String,
    },
}

const TEST_PUBLIC_DIR: &'static str = r#"C:\Users\Hector.C\rust\htmx\view\src\assets\pages"#;
// const TEST_PUBLIC_DIR: &str = r#"/Users/hectorcrean/Rust/htmlx/view/src/assets/pages"#;

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
            let app = App::new();

            let routes_json = app.root_node.generate_routes_json()?;
            let node_data = app.root_node.node_map()?;
            let templates = app.root_node.generate_routes();

            let routes = Routes::new(TEST_PUBLIC_DIR, app.root_node);

            routes.build()?;

            let mut routes_file = fs::File::create(format!("{}/routes.json", TEST_PUBLIC_DIR))?;

            routes_file.write(routes_json.as_bytes())?;

            let mut data_file = fs::File::create(format!("{}/ptsd.json", TEST_PUBLIC_DIR))?;

            data_file.write(node_data.as_bytes())?;

            let nav = NavProps {
                routes: templates.clone(),
            };

            let mut nav_file = fs::File::create(format!("{}/nav.html", TEST_PUBLIC_DIR))?;

            nav_file.write(nav.render().0.as_bytes())?;

            Ok(())
        }
        Commands::GenerateNodeMap {} => {
            let app = App::new();

            let node_map = app.root_node.node_map()?;

            let mut data_file = fs::File::create(format!("{}/ptsd.json", PROJECT_ROOT))?;

            data_file.write(node_map.as_bytes())?;

            Ok(())
        }
        Commands::JsToTs {
            root_directory_path,
        } => {
            file_ops::js_to_ts(root_directory_path)?;
            Ok(())
        }
    }
}
