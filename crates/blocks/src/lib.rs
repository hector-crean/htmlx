include!(concat!(env!("OUT_DIR"), "/project_root.rs"));
include!(concat!(env!("OUT_DIR"), "/generated_icons.rs"));

pub mod block;
pub mod layout;
pub mod node;
pub mod node_map;
pub mod page;
pub mod span;
