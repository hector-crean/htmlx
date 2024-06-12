pub mod clinical_course;
pub mod clinical_presentation;
pub mod diagnosis;
pub mod disease;
pub mod disease_burden;
pub mod recovery;
pub mod specific_populations;
pub mod treatment;

use crate::routes;


use blocks::node::{FileExtension, Node, NodeType};
use blocks::page::Page;

pub struct App {
    pub root_node: Node<Page>,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        let node = App::pages();
        // .add_child(Node::new(
        //     "home",
        //     NodeType::File {
        //         extension: FileExtension::Html,
        //         renderable: Page::new(vec![Block::NavBlock(NavProps::new(Self::nav()))]),
        //     },
        // ));

        Self { root_node: node }
    }

    pub fn nav() -> Vec<(String, String)> {
        App::pages()
            .iter()
            // .filter(|(_, node)| match &node.node_type {
            //     NodeType::File {
            //         extension,
            //         renderable,
            //     } => true,
            //     _ => false,
            // })
            .map(|(path, node)| (path, node.path_segment.clone()))
            .collect::<Vec<(String, String)>>()
    }
    pub fn pages() -> Node<Page> {
        Node::new("ptsd", NodeType::Folder)
            .add_child(
                Node::new("clinical_presentation", NodeType::Folder)
                    .add_child(Node::new("symptoms", NodeType::Folder).add_child(Node::new(
                        "page",
                        NodeType::File {
                            extension: FileExtension::Html,
                            renderable: Page::new(
                                "Symptoms",
                                None,
                                routes::clinical_presentation::symptoms::blocks(),
                            ),
                        },
                    )))
                    .add_child(
                        Node::new("comorbidities", NodeType::Folder).add_child(Node::new(
                            "page",
                            NodeType::File {
                                extension: FileExtension::Html,
                                renderable: Page::new(
                                    "Comorbidities",
                                    None,
                                    routes::clinical_presentation::comorbidities::blocks(),
                                ),
                            },
                        )),
                    ),
            )
            // .add_child(
            //     Node::new("disease", NodeType::Folder)
            //         .add_child(
            //             Node::new("trauma_types", NodeType::Folder).add_child(Node::new(
            //                 "page",
            //                 NodeType::File {
            //                     extension: FileExtension::Html,
            //                     renderable: Page::new(vec![]),
            //                 },
            //             )),
            //         )
            //         .add_child(
            //             Node::new("pathophysiology_of_ptsd", NodeType::Folder).add_child(
            //                 Node::new(
            //                     "page",
            //                     NodeType::File {
            //                         extension: FileExtension::Html,
            //                         renderable: Page::new(vec![]),
            //                     },
            //                 ),
            //             ),
            //         ),
            // )
            .add_child(
                Node::new("specific_populations", NodeType::Folder).add_child(Node::new(
                    "page",
                    NodeType::File {
                        extension: FileExtension::Html,
                        renderable: Page::new(
                            "Specific Populations",
                            None,
                            routes::specific_populations::blocks(),
                        ),
                    },
                )),
            )
            .add_child(
                Node::new("disease_burden", NodeType::Folder).add_child(Node::new(
                    "page",
                    NodeType::File {
                        extension: FileExtension::Html,
                        renderable: Page::new("Disease Burden", None, disease_burden::blocks()),
                    },
                )),
            )
            .add_child(
                Node::new("diagnosis", NodeType::Folder)
                    // .add_child(
                    //     Node::new("assessment_and_diagnosis", NodeType::Folder).add_child(
                    //         Node::new(
                    //             "page",
                    //             NodeType::File {
                    //                 extension: FileExtension::Html,
                    //                 renderable: Page::new(vec![]),
                    //             },
                    //         ),
                    //     ),
                    // )
                    .add_child(Node::new("stigma", NodeType::Folder).add_child(Node::new(
                        "page",
                        NodeType::File {
                            extension: FileExtension::Html,
                            renderable: Page::new("Stigma", None, diagnosis::stigma::blocks()),
                        },
                    ))), // .add_child(
                         //     Node::new("interviews_with_clinicians", NodeType::Folder).add_child(
                         //         Node::new(
                         //             "page",
                         //             NodeType::File {
                         //                 extension: FileExtension::Html,
                         //                 renderable: Page::new(vec![]),
                         //             },
                         //         ),
                         //     ),
                         // ),
            )
        // .add_child(
        //     Node::new("clinical_course", NodeType::Folder)
        //         .add_child(Node::new("delayed_onset_ptsd", NodeType::Folder).add_child(
        //             Node::new(
        //                 "page",
        //                 NodeType::File {
        //                     extension: FileExtension::Html,
        //                     renderable: Page::new(vec![]),
        //                 },
        //             ),
        //         ))
        //         .add_child(
        //             Node::new("chronic_ptsd", NodeType::Folder).add_child(Node::new(
        //                 "page",
        //                 NodeType::File {
        //                     extension: FileExtension::Html,
        //                     renderable: Page::new(vec![]),
        //                 },
        //             )),
        //         )
        //         .add_child(
        //             Node::new("underdiagnosis", NodeType::Folder).add_child(Node::new(
        //                 "page",
        //                 NodeType::File {
        //                     extension: FileExtension::Html,
        //                     renderable: Page::new(vec![]),
        //                 },
        //             )),
        //         ),
        // )
        // .add_child(
        //     Node::new("treatment", NodeType::Folder)
        //         .add_child(
        //             Node::new("guidelines", NodeType::Folder).add_child(Node::new(
        //                 "page",
        //                 NodeType::File {
        //                     extension: FileExtension::Html,
        //                     renderable: Page::new(vec![]),
        //                 },
        //             )),
        //         )
        //         .add_child(
        //             Node::new("unmet_needs_and_barriers", NodeType::Folder).add_child(
        //                 Node::new(
        //                     "page",
        //                     NodeType::File {
        //                         extension: FileExtension::Html,
        //                         renderable: Page::new(vec![]),
        //                     },
        //                 ),
        //             ),
        //         )
        //         .add_child(
        //             Node::new("trauma_informed_care", NodeType::Folder).add_child(Node::new(
        //                 "page",
        //                 NodeType::File {
        //                     extension: FileExtension::Html,
        //                     renderable: Page::new(vec![]),
        //                 },
        //             )),
        //         ),
        // )
        // .add_child(
        //     Node::new("recovery", NodeType::Folder)
        //         .add_child(
        //             Node::new("intermediate_recovery", NodeType::Folder).add_child(Node::new(
        //                 "page",
        //                 NodeType::File {
        //                     extension: FileExtension::Html,
        //                     renderable: Page::new(vec![]),
        //                 },
        //             )),
        //         )
        //         .add_child(
        //             Node::new("long_term_reconstruction", NodeType::Folder).add_child(
        //                 Node::new(
        //                     "page",
        //                     NodeType::File {
        //                         extension: FileExtension::Html,
        //                         renderable: Page::new(vec![]),
        //                     },
        //                 ),
        //             ),
        //         ),
        // )
    }
}
