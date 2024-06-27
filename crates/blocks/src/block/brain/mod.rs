pub use brain_region::BrainRegionName;

pub mod brain_region;
pub mod interactive_brain;
pub mod interactive_image;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type, Default)]
#[serde(rename_all = "camelCase")]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type, Default)]
#[serde(rename_all = "camelCase")]
pub struct BrainRegionLabel {
    pub position: Vec2,
    pub alt_draw_mode: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type, Default)]
#[serde(rename_all = "camelCase")]
pub struct BrainRegion {
    pub id: BrainRegionName,
    pub name: String,
    pub fill_color: String,
    pub label: BrainRegionLabel,
    pub centroid: Vec2,
    pub points: Vec<Vec2>,
}
