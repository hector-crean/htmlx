include!(concat!(env!("OUT_DIR"), "/project_root.rs"));

use maud::Render;

pub mod macros;
pub mod routes;
pub mod svg_ops;
pub mod typegen;
