pub mod clinical_course;
pub mod clinical_presentation;
pub mod diagnosis;
pub mod disease;
pub mod disease_burden;
pub mod recovery;
pub mod specific_populations;
pub mod treatment;

use crate::routes;

use blocks::node::{FileExtension, Node, NodeType, RouteStatus};
use blocks::page::Page;
use maud::Render;

pub struct App {
    pub root_node: Node,
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
        // status: RouteStatus::NotStarted,
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
    pub fn pages() -> Node {
        Node::new("ptsd", NodeType::Folder)
            .add_child(
                Node::new("clinical_presentation", NodeType::Folder)
                    .add_child(
                        Node::new("symptoms", NodeType::Folder).add_child(Node::new(
                            "page",
                            NodeType::File {
                                extension: FileExtension::Html,
                                status: RouteStatus::UnderDevelopment,
                                renderable: Page::new(
                                    "Symptoms",
                                    Some("PTSD Symptom Domains"),
                                    routes::clinical_presentation::symptoms::Page,
                                )
                                .render(),
                            },
                        )),
                    )
                    .add_child(
                        Node::new("comorbidities", NodeType::Folder).add_child(Node::new(
                            "page",
                            NodeType::File {
                                extension: FileExtension::Html,
                                status: RouteStatus::UnderDevelopment,
                                renderable: Page::new(
                                    "Comorbidities",
                                    None,
                                    routes::clinical_presentation::comorbidities::Page,
                                )
                                .render(),
                            },
                        )),
                    ),
            )
            .add_child(
                Node::new("specific_populations", NodeType::Folder).add_child(Node::new(
                    "page",
                    NodeType::File {
                        extension: FileExtension::Html,
                        status: RouteStatus::UnderDevelopment,
                        renderable: Page::new(
                            "Specific Populations",
                            None,
                            routes::specific_populations::Page,
                        )
                        .render(),
                    },
                )),
            )
            .add_child(
                Node::new("disease_burden", NodeType::Folder).add_child(Node::new(
                    "page",
                    NodeType::File {
                        extension: FileExtension::Html,
                        status: RouteStatus::UnderDevelopment,
                        renderable: Page::new("Disease Burden", None, disease_burden::Page)
                            .render(),
                    },
                )),
            )
            .add_child(
                Node::new("diagnosis", NodeType::Folder)
                    .add_child(
                        Node::new("assessment_and_diagnosis", NodeType::Folder).add_child(
                            Node::new(
                                "page",
                                NodeType::File {
                                    extension: FileExtension::Html,
                                    status: RouteStatus::UnderDevelopment,
                                    renderable: Page::new(
                                        "Assessment and Diagnosis",
                                        None,
                                        diagnosis::assessment_and_diagnosis::Page,
                                    )
                                    .render(),
                                },
                            ),
                        ),
                    )
                    .add_child(Node::new("stigma", NodeType::Folder).add_child(Node::new(
                        "page",
                        NodeType::File {
                            extension: FileExtension::Html,
                            status: RouteStatus::UnderDevelopment,
                            renderable: Page::new("Stigma", None, diagnosis::stigma::Page).render(),
                        },
                    )))
                    .add_child(
                        Node::new("interviews_with_clinicians", NodeType::Folder).add_child(
                            Node::new(
                                "page",
                                NodeType::File {
                                    extension: FileExtension::Html,
                                    status: RouteStatus::NotStarted,
                                    renderable: Page::new(
                                        "Interviews with Clinicians",
                                        None,
                                        routes::diagnosis::interviews_with_clinicians::Page,
                                    )
                                    .render(),
                                },
                            ),
                        ),
                    ),
            )
            .add_child(
                Node::new("clinical_course", NodeType::Folder).add_child(Node::new(
                    "page",
                    NodeType::File {
                        extension: FileExtension::Html,
                        status: RouteStatus::UnderDevelopment,
                        renderable: Page::new(
                            "Clinical Course",
                            None,
                            routes::clinical_course::Page,
                        )
                        .render(),
                    },
                )),
            )
            .add_child(
                Node::new("treatment", NodeType::Folder)
                    .add_child(
                        Node::new("guidelines", NodeType::Folder).add_child(Node::new(
                            "page",
                            NodeType::File {
                                extension: FileExtension::Html,
                                status: RouteStatus::UnderDevelopment,
                                renderable: Page::new(
                                    "Guidelines",
                                    None,
                                    routes::treatment::guidelines::Page,
                                )
                                .render(),
                            },
                        )),
                    )
                    .add_child(
                        Node::new("unmet_needs_and_barriers", NodeType::Folder).add_child(
                            Node::new(
                                "page",
                                NodeType::File {
                                    extension: FileExtension::Html,
                                    status: RouteStatus::NotStarted,
                                    renderable: Page::new(
                                        "Unmet needs and barriers",
                                        None,
                                        routes::treatment::unmet_needs_and_barriers::Page,
                                    )
                                    .render(),
                                },
                            ),
                        ),
                    )
                    .add_child(
                        Node::new("trauma_informed_care", NodeType::Folder).add_child(Node::new(
                            "page",
                            NodeType::File {
                                extension: FileExtension::Html,
                                status: RouteStatus::NotStarted,
                                renderable: Page::new(
                                    "Trauma informed care",
                                    None,
                                    routes::treatment::trauma_informed_care::Page,
                                )
                                .render(),
                            },
                        )),
                    ),
            )
            .add_child(
                Node::new("recovery", NodeType::Folder)
                    .add_child(
                        Node::new("intermediate_recovery", NodeType::Folder).add_child(Node::new(
                            "page",
                            NodeType::File {
                                extension: FileExtension::Html,
                                status: RouteStatus::NotStarted,
                                renderable: Page::new(
                                    "Intermedate Recovery",
                                    None,
                                    routes::recovery::intermediate_recovery::Page,
                                )
                                .render(),
                            },
                        )),
                    )
                    .add_child(
                        Node::new("long_term_reconstruction", NodeType::Folder).add_child(
                            Node::new(
                                "page",
                                NodeType::File {
                                    extension: FileExtension::Html,
                                    status: RouteStatus::NotStarted,
                                    renderable: Page::new(
                                        "Long term reconstruction",
                                        None,
                                        routes::recovery::long_term_reconstruction::Page,
                                    )
                                    .render(),
                                },
                            ),
                        ),
                    ),
            )
            .add_child(
                Node::new("disease", NodeType::Folder)
                    .add_child(
                        Node::new("trauma_types", NodeType::Folder).add_child(Node::new(
                            "page",
                            NodeType::File {
                                extension: FileExtension::Html,
                                status: RouteStatus::UnderDevelopment,
                                renderable: Page::new(
                                    "Trauma Types",
                                    None,
                                    routes::disease::trauma_types::Page,
                                )
                                .render(),
                            },
                        )),
                    )
                    .add_child(
                        Node::new("pathophysiology_of_ptsd", NodeType::Folder).add_child(
                            Node::new(
                                "page",
                                NodeType::File {
                                    extension: FileExtension::Html,
                                    status: RouteStatus::UnderDevelopment,
                                    renderable: Page::new(
                                        "Pathophysiology of PTSD",
                                        None,
                                        routes::disease::pathophysiology_of_ptsd::Page,
                                    )
                                    .render(),
                                },
                            ),
                        ),
                    ),
            )
    }
}
