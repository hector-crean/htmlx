use blocks::block::tabs::{Tab, TabsProps, TabsRepresentation};
use maud::{html, Render};

pub struct Page;

impl Page {
    fn infographic_overview_tab(&self) -> Tab {
        Tab {
            name: "Infographic Overview".to_string(),
            blocks: vec![],
        }
    }
    fn psychotherapy_tab(&self) -> Tab {
        Tab {
            name: "Psychotherapy".to_string(),
            blocks: vec![],
        }
    }
    fn pharmacotherapy_tab(&self) -> Tab {
        Tab {
            name: "Pharmacotherapy".to_string(),
            blocks: vec![],
        }
    }
}

impl Render for Page {
    fn render(&self) -> maud::Markup {
        html! {
            (TabsProps {
                id: uuid::Uuid::new_v4(),
                representation: TabsRepresentation::TopLevel,
                tabs: vec![self.infographic_overview_tab(), self.psychotherapy_tab(), self.pharmacotherapy_tab()]
            })
        }
    }
}
