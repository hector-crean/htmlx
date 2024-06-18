use blocks::block::tabs::{Tab, TabsProps, TabsRepresentation};
use maud::{html, Render};

pub struct Page;

impl Page {
    fn clinician_video_tab(&self) -> Tab {
        Tab {
            name: "Clinician video discussing importance of TIC".to_string(),
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
                tabs: vec![self.clinician_video_tab()]
            })
        }
    }
}
