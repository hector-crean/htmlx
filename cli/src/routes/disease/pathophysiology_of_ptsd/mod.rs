use blocks::block::tabs::{Tab, TabsProps, TabsRepresentation};
use maud::{html, Render};

pub struct Page;

impl Page {
    fn videos_tab(&self) -> Tab {
        Tab {
            name: "Vidoes".to_string(),
            blocks: vec![],
        }
    }
    fn brain_model_tab(&self) -> Tab {
        Tab {
            name: "Brain model".to_string(),
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
                tabs: vec![self.brain_model_tab(), self.videos_tab()]
            })
        }
    }
}
