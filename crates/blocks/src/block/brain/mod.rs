use brain_region::BrainRegionName;

pub mod brain_glossary;
pub mod brain_region;
pub mod interactive_brain;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type, Default)]
#[serde(rename_all = "camelCase")]
pub struct Vec2 {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type, Default)]
#[serde(rename_all = "camelCase")]
pub struct BrainRegionLabel {
    position: Vec2,
    alt_draw_mode: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type, Default)]
#[serde(rename_all = "camelCase")]
pub struct BrainRegion {
    name: BrainRegionName,
    fill_color: String,
    label: BrainRegionLabel,
    centroid: Vec2,
    points: Vec<Vec2>,
}
