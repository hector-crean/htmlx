use maud::{PreEscaped, Render};
use serde_json::json;

use rand::Rng;
use std::collections::HashMap;
use std::convert::AsRef;
use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use strum::AsRefStr;

use crate::node_map::{self, to_sentence_case};

pub struct DefaultInitJs;

impl Render for DefaultInitJs {
    fn render(&self) -> maud::Markup {
        PreEscaped(
            r#"
        import tabs from '@/components/tabs';

        const page = {
            init: () => {
                tabs.init()
            }
        };
        
        export default page;
        "#
            .to_string(),
        )
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type, PartialEq)]
pub enum RouteStatus {
    Finished,
    UnderDevelopment,
    NotStarted,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type, PartialEq)]
pub struct RouteTemplate {
    pub path: String,
    pub template: String,
    pub status: RouteStatus,
}

#[derive(AsRefStr, Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
pub enum FileExtension {
    #[strum(serialize = "html")]
    Html,
    #[strum(serialize = "rs")]
    Rust,
}

#[derive(Debug, Clone)]
pub enum NodeType {
    File {
        extension: FileExtension,
        renderable: PreEscaped<String>,
        status: RouteStatus,
    },
    Folder,
}

#[derive(Debug)]
pub struct Node {
    pub path_segment: String,
    pub node_type: NodeType,
    pub children: Vec<Node>,
}

#[derive(Debug)]
pub struct NodeIterator<'a> {
    stack: Vec<(&'a Node, String)>,
}

impl<'a> Iterator for NodeIterator<'a> {
    type Item = (String, &'a Node);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((node, path)) = self.stack.pop() {
            let new_path = if path.is_empty() {
                node.path_segment.clone()
            } else {
                format!("{}/{}", path, node.path_segment)
            };

            for child in node.children.iter().rev() {
                self.stack.push((child, new_path.clone()));
            }

            Some((new_path, node))
        } else {
            None
        }
    }
}

impl Node {
    // Constructor for Node
    pub fn new(path_segment: &str, node_type: NodeType) -> Self {
        Node {
            path_segment: path_segment.into(),
            node_type,
            children: Vec::new(),
        }
    }

    pub fn preprend_path_segment(mut self, prepended: &str) -> Self {
        self.path_segment = format!("{}/{}", prepended, self.path_segment);
        self
    }

    // Method to add a child Node
    pub fn add_child(mut self, child: Node) -> Self {
        self.children.push(child);
        self
    }

    pub fn iter(&self) -> NodeIterator {
        NodeIterator {
            stack: vec![(self, String::new())],
        }
    }

    // Traverse method
    pub fn traverse(&self, operation: &mut impl FnMut(&Self)) {
        operation(self);
        for child in &self.children {
            child.traverse(operation);
        }
    }

    pub fn generate_routes(&self) -> Vec<RouteTemplate> {
        let mut routes = Vec::<RouteTemplate>::new();

        for (path, node) in self.iter() {
            if let NodeType::File {
                extension, status, ..
            } = &node.node_type
            {
                let template = format!("/{}.{}", path, extension.as_ref());
                let path = format!("/{}", path);

                routes.push(RouteTemplate {
                    path,
                    template,
                    status: status.clone(),
                });
            }
        }

        routes
    }

    pub fn generate_routes_json(&self) -> Result<String, serde_json::Error> {
        let routes = self.generate_routes();

        serde_json::to_string_pretty(&routes)
    }
    pub fn node_map(&self) -> Result<String, serde_json::Error> {
        let mut nodes = HashMap::new();

        for (path, node) in self.iter() {
            match &node.node_type {
                NodeType::File {
                    extension,
                    renderable,
                    ..
                } => {
                    let mut content = HashMap::new();

                    let base = path.split("/").collect::<Vec<&str>>();
                    let base = &base[0..base.len() - 1].join("/");
                    content.insert(
                        "default".to_string(),
                        node_map::Content::new(
                            Some(path.clone()),
                            vec![path.clone()],
                            vec![format!("{}/js/index.js", base)],
                        ),
                    );

                    // Create a random number generator
                    let mut rng = rand::thread_rng();

                    // Generate a random number between -100 and 100
                    let x = rng.gen_range(-20..=20);
                    let z = rng.gen_range(-20..=20);

                    nodes.insert(
                        String::from(&path),
                        node_map::GraphNode {
                            color: String::from("#260038"),
                            position: node_map::XYPosition { x, z },
                            node_type: node_map::GraphNodeType::Secondary,
                            category: None,
                            connections: HashMap::new(),
                            label: String::from(to_sentence_case(&path)),
                            content: Some(content),
                            distance: Some(32),
                        },
                    );
                }
                NodeType::Folder => {
                    nodes.insert(
                        String::from(&path),
                        node_map::GraphNode {
                            color: String::from("#260038"),
                            position: node_map::XYPosition { x: -48, z: 0 },
                            node_type: node_map::GraphNodeType::Category,
                            category: Some(true),
                            connections: HashMap::new(),
                            label: String::from(to_sentence_case(&node.path_segment)),
                            content: Some(HashMap::new()),
                            distance: Some(32),
                        },
                    );
                }
            }
        }

        print!("{:?}", &nodes);

        let data = node_map::NodeMapData {
            curve: node_map::Curve::new(String::from("#8f51f5")),
            nodes,
            map: node_map::Map {
                settings: node_map::MapSettings {
                    node_color: String::from("#ffffff"),
                    selected_node_color: String::from("#ffffff"),
                    connection_color: String::from("#ffffff"),
                },
                size: node_map::Size {
                    min: 400,
                    preferred: node_map::PreferredSize { x: 0.5, y: 0.15 },
                    max: 720,
                    ratio: 0.184,
                },
                nodes: HashMap::new(),
            },
            camera: node_map::Camera {
                reference_width: 1920,
                reference_height: 980,
                initial_position: node_map::InitialXYZPosition {
                    x: -2,
                    y: 85,
                    z: 47,
                },
                initial_target: node_map::InitialTarget { x: -2, y: 0, z: 5 },
            },
        };

        serde_json::to_string_pretty(&data)
    }
}
pub struct Routes {
    root: Node,
}

impl Routes {
    pub fn new(base_path: &str, root: Node) -> Self {
        let root = root.preprend_path_segment(base_path);
        Self { root }
    }
    pub fn traverse_and_write(&self) -> Result<(), std::io::Error> {
        for (path, node) in self.root.iter() {
            match &node.node_type {
                NodeType::File {
                    renderable,
                    extension,
                    ..
                } => {
                    let mut file = File::create(format!("{}.{}", path, extension.as_ref()))?;

                    let rendered = renderable.render();

                    file.write(rendered.0.as_bytes())?;
                }
                NodeType::Folder => {
                    std::fs::create_dir_all(path)?;
                }
            }
        }

        Ok(())
    }
    pub fn traverse_and_generate_js_default(&self) -> Result<(), std::io::Error> {
        for (path, node) in self.root.iter() {
            match &node.node_type {
                NodeType::File {
                    renderable,
                    extension,
                    ..
                } => match extension.as_ref() {
                    "html" => {
                        let mut file = File::create(format!("{}.js", path))?;

                        file.write(DefaultInitJs.render().0.as_bytes())?;
                    }
                    _ => {}
                },
                NodeType::Folder => {
                    std::fs::create_dir_all(path)?;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use crate::page::Page;

    use super::*;

    // fn root() -> Node<Page> {
    //     Node::new("Root", NodeType::Folder).add_child(Node::new("Child 2", NodeType::Folder))
    // }

    #[test]
    fn traverse() {
        // root().traverse(&mut |node| {
        //     println!("{:?}", node);
        // });
    }
}
