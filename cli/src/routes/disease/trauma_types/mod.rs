use blocks::block::tabs::{Tab, TabsProps, TabsRepresentation};
use maud::{html, Render};

pub struct Page;

impl Page {
    fn traumatic_events_drivers_tab(&self) -> Tab {
        Tab {
            name: "Information surrounding different traumatic events as key drivers".to_string(),
            blocks: vec![],
        }
    }
    fn risk_factors_tab(&self) -> Tab {
        Tab {
            name: "Risk Factors".to_string(),
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
                tabs: vec![self.traumatic_events_drivers_tab(), self.risk_factors_tab()]
            })
        }
    }
}
