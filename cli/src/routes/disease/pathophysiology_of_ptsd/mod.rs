use blocks::block::{
    brain::{brain_glossary::BrainGlossaryProps, brain_region::BrainRegionName},
    tabs::{Tab, TabsProps, TabsRepresentation},
    Block,
};
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
        let brain_regions = vec![
            BrainRegionName::Frontosubcortical,
            BrainRegionName::Orbitofrontal,
        ];
        Tab {
            name: "Brain model".to_string(),
            blocks: vec![Block::BrainGlossaryBlock(BrainGlossaryProps {
                region_names: brain_regions,
            })],
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
