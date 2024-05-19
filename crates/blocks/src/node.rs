use maud::Render;
use serde_json::json;

use std::convert::AsRef;
use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use strum::AsRefStr;

#[derive(AsRefStr, Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
pub enum FileExtension {
    #[strum(serialize = "html")]
    Html,
    #[strum(serialize = "rs")]
    Rust,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum NodeType<R: Render> {
    File {
        extension: FileExtension,
        renderable: R,
    },
    Folder,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Node<R: Render> {
    pub path_segment: String,
    pub node_type: NodeType<R>,
    pub children: Vec<Node<R>>,
}

#[derive(Debug)]
pub struct NodeIterator<'a, R: Render> {
    stack: Vec<(&'a Node<R>, String)>,
}

impl<'a, R: Render> Iterator for NodeIterator<'a, R> {
    type Item = (String, &'a Node<R>);

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

impl<R: Render> Node<R> {
    // Constructor for Node
    pub fn new(path_segment: &str, node_type: NodeType<R>) -> Self {
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
    pub fn add_child(mut self, child: Node<R>) -> Self {
        self.children.push(child);
        self
    }

    pub fn iter(&self) -> NodeIterator<R> {
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

    pub fn generate_routes_json(&self) -> Result<String, serde_json::Error> {
        let mut routes = vec![];

        for (path, node) in self.iter() {
            if let NodeType::File { extension, .. } = &node.node_type {
                let template = format!("/{}.{}", path, extension.as_ref());
                let path = format!("/{}", path);

                routes.push(json!({ "path": path, "template": template }));
            }
        }

        serde_json::to_string_pretty(&routes)
    }
}

pub struct Routes<R: Render> {
    root: Node<R>,
}

impl<R: Render> Routes<R> {
    pub fn new(base_path: &str, root: Node<R>) -> Self {
        let root = root.preprend_path_segment(base_path);
        Self { root }
    }
    pub fn build(&self) -> Result<(), std::io::Error> {
        for (path, node) in self.root.iter() {
            match &node.node_type {
                NodeType::File {
                    renderable,
                    extension,
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
}

#[cfg(test)]
mod tests {

    use crate::page::Page;

    use super::*;

    fn root() -> Node<Page> {
        Node::new("Root", NodeType::Folder).add_child(Node::new("Child 2", NodeType::Folder))
    }

    #[test]
    fn traverse() {
        root().traverse(&mut |node| {
            println!("{:?}", node);
        });
    }
}
