use askama::Template;
use strum::IntoStaticStr;

use super::{rich_text::{RichText, RichTextProps}, Block};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type, Default)]
#[serde(rename_all = "camelCase")]
struct Vec2 {
    x: f32,
    y: f32
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
#[serde(rename_all = "camelCase")]
struct BrainRegionLabel {
    position:Vec2,
    alt_draw_mode: bool
}

impl Default for BrainRegionLabel {
    fn default() -> Self {
        BrainRegionLabel {
            position: Vec2::default(),
            alt_draw_mode: false, // typically you might want labels to draw in a standard mode by default
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct BrainRegion {
    name: BrainRegionName,
    fill_color: String,
    label: BrainRegionLabel,
    centroid: Vec2,
    points: Vec<Vec2>
}

impl Default for BrainRegion {
    fn default() -> Self {
        BrainRegion {
            name: BrainRegionName::default(),
            fill_color: String::from("#FFFFFF"), // Default fill color as white
            label: BrainRegionLabel::default(),
            centroid: Vec2::default(),
            points: vec![],
        }
    }
}


#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type, IntoStaticStr)]
#[serde(rename_all = "camelCase")]
pub enum BrainRegionName {
    #[default]
    Frontosubcortical,
    Orbitofrontal,
    AnteriorCingulatedGyrus,
    BilateralTemporalCortex,
    ParietalLobe,
    Thalamus,
    Hippocampus,
    Amygdala,
    Hypothalamus,
    AnteriorCingulateCortex,
    PosteriorCingulateCortex,
    Striatum,
    PrefrontalCortex,
    VentralFrontalCortex,
    FrontalLobe,
    Dlpfc,
    Vlpfc,
    NucleusAccumbens,
    BasalForebrain,
    AnteriorCaudate,
    GreyMatter,
    LateralVentricle,
    OccipitalLobe,
    AuditoryCortex,
    SubstantiaNigra,
    NucleusAccumbensArea,
    AmyloidStage1MildRegion1,
    AmyloidStage2ModerateRegion1,
    AmyloidStage2MildRegion1,
    AmyloidStage3SevereRegion1,
    AmyloidStage3ModerateRegion1,
    AmyloidStage3ModerateRegion2,
    LocusCoeruleus,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct BrainComment {
    pub icon: String,
    pub symptom: RichTextProps,
    pub highlighted_regions: Vec<BrainRegionName>,
    pub description: Vec<Block>
}

impl BrainComment {
    pub fn new<S: Into<String>>(icon: S,
        symptom: RichTextProps,
        highlighted_regions: Vec<BrainRegionName>,
        description: Vec<Block>) -> Self {
        Self {
            icon: icon.into(), symptom, highlighted_regions, description
        }
    }
}

impl Default for BrainComment {
    fn default() -> Self {
        BrainComment {
            icon: String::from("🧠"),
            symptom: RichTextProps::default(),
            highlighted_regions: vec![],
            description: vec![]
        }
    }
}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct CommentGroup {
    pub name: String,
    pub comments: Vec<BrainComment>
}

impl CommentGroup {
    pub fn new<S: Into<String>>(name: S, comments: Vec<BrainComment>) -> Self { 
        Self {
        name: name.into(),
        comments
    }}
}

impl Default for CommentGroup {
    fn default() -> Self {
        CommentGroup {
            name: String::from("Default Group"),
            comments: vec![BrainComment::default()],
        }
    }
}



#[derive(Template, Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
#[template(path = "interactive-brain/tabs.html",)]
pub struct InteractiveBrainProps {
    pub groups: Vec<CommentGroup>,
}

impl InteractiveBrainProps {
    fn highlighted_regions_str(highlighted_regions: &Vec<BrainRegionName>) -> String {
        
        let regions = highlighted_regions
        .iter()
        .map(|region| { 
            let region_name: &'static str = region.clone().into(); 
            return region_name.to_string()
        })
        .collect::<Vec<String>>().join(" ");
        println!("{:?}", &regions);
        regions
    }
}

impl Default for InteractiveBrainProps {
    fn default() -> Self {
        InteractiveBrainProps {
            groups: vec![
                CommentGroup::default()
            ],
        }
    }
}