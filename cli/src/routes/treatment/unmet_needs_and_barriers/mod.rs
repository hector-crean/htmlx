use blocks::block::tabs::{Tab, TabsProps, TabsRepresentation};
use maud::{html, Render};

pub struct Page;

impl Page {
    // fn infographic_overview_tab(&self) -> Tab {
    //     Tab {
    //         name: "Infographic Overview".to_string(),
    //         blocks: vec![],
    //     }
    // }
}

impl Render for Page {
    fn render(&self) -> maud::Markup {
        html! {
            (TabsProps {
                id: uuid::Uuid::new_v4(),
                representation: TabsRepresentation::TopLevel,
                tabs: vec![]
            })
        }
    }
}
