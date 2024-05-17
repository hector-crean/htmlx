use pages::Pagelike;

include!(concat!(env!("OUT_DIR"), "/project_root.rs"));

pub mod macros;
pub mod pages;
pub mod typegen;



#[derive(Debug)]
enum NodeType<F: Pagelike> {
    File(F),
    Directory,
}

#[derive(Debug)]
struct Node<F: Pagelike> {
    name: String,
    node_type: NodeType<F>,
    children: Vec<Node<F>>
}

impl<F: Pagelike> Node<F> {
    // Constructor for Node
    fn new(name: String, node_type: NodeType<F>) -> Self {
        Node {
            name,
            node_type,
            children: Vec::new(),
        }
    }

    // Method to add a child Node
    fn add_child(mut self, child: Node<F>) -> Self {
        self.children.push(child);
        self
    }
    // Method to print the tree structure
    fn print(&self, indent: usize) {
        let indent_str = " ".repeat(indent);
        match &self.node_type {
            NodeType::File(_) => println!("{}File: {}", indent_str, self.name),
            NodeType::Directory => {
                println!("{}Directory: {}", indent_str, self.name);
                for child in &self.children {
                    child.print(indent + 2);
                }
            }
        }
    }
}