use std::collections::HashMap;

use crate::{
    block,
    node::{Node, NodeType},
    page::Page,
};
use strum::AsRefStr;

#[derive(Debug, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct XYPosition {
    pub x: i32,
    pub z: i32,
}

impl XYPosition {
    pub fn new(x: i32, z: i32) -> Self {
        XYPosition { x, z }
    }
}

impl Default for XYPosition {
    fn default() -> Self {
        XYPosition { x: 0, z: 0 }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct Content {
    pub label: Option<String>,
    pub html: Vec<String>,
    pub requires: Vec<String>,
}

impl Content {
    pub fn new(label: Option<String>, html: Vec<String>, requires: Vec<String>) -> Self {
        Content {
            label,
            html,
            requires,
        }
    }
}

impl Default for Content {
    fn default() -> Self {
        Content {
            label: None,
            html: vec![],
            requires: vec![],
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type, Default)]
pub enum GraphNodeType {
    #[serde(rename = "primary")]
    Primary,
    #[serde(rename = "menu")]
    Menu,
    #[serde(rename = "menu__disabled")]
    MenuDisabled,
    #[serde(rename = "category")]
    #[default]
    Category,
    #[serde(rename = "secondary")]
    Secondary,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct GraphNode {
    pub color: String,
    pub position: XYPosition,
    #[serde(rename = "type")]
    pub node_type: GraphNodeType,
    pub category: Option<bool>,
    pub connections: HashMap<String, ()>,
    pub label: String,
    pub content: Option<HashMap<String, Content>>,
    pub distance: Option<i32>,
}

impl GraphNode {
    pub fn new(
        color: String,
        position: XYPosition,
        node_type: GraphNodeType,
        category: Option<bool>,
        label: String,
        distance: Option<i32>,
    ) -> Self {
        GraphNode {
            color,
            position,
            node_type,
            category,
            connections: HashMap::new(),
            label,
            content: None,
            distance,
        }
    }
}

impl Default for GraphNode {
    fn default() -> Self {
        GraphNode {
            color: String::from("#000000"),
            position: XYPosition::default(),
            node_type: GraphNodeType::default(),
            category: None,
            connections: HashMap::new(),
            label: String::from("Default Label"),
            content: None,
            distance: None,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct Curve {
    pub color: String,
}

impl Curve {
    pub fn new(color: String) -> Self {
        Curve { color }
    }
}

impl Default for Curve {
    fn default() -> Self {
        Curve {
            color: String::from("#000000"),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct MapSettings {
    pub node_color: String,
    pub selected_node_color: String,
    pub connection_color: String,
}

impl MapSettings {
    pub fn new(node_color: String, selected_node_color: String, connection_color: String) -> Self {
        MapSettings {
            node_color,
            selected_node_color,
            connection_color,
        }
    }
}

impl Default for MapSettings {
    fn default() -> Self {
        MapSettings {
            node_color: String::from("#ffffff"),
            selected_node_color: String::from("#ffffff"),
            connection_color: String::from("#ffffff"),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct PreferredSize {
    pub x: f64,
    pub y: f64,
}

impl PreferredSize {
    pub fn new(x: f64, y: f64) -> Self {
        PreferredSize { x, y }
    }
}

impl Default for PreferredSize {
    fn default() -> Self {
        PreferredSize { x: 0.0, y: 0.0 }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct Size {
    pub min: i32,
    pub preferred: PreferredSize,
    pub max: i32,
    pub ratio: f64,
}

impl Size {
    pub fn new(min: i32, preferred: PreferredSize, max: i32, ratio: f64) -> Self {
        Size {
            min,
            preferred,
            max,
            ratio,
        }
    }
}

impl Default for Size {
    fn default() -> Self {
        Size {
            min: 0,
            preferred: PreferredSize::default(),
            max: 0,
            ratio: 0.0,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct Map {
    pub settings: MapSettings,
    pub size: Size,
    pub nodes: HashMap<String, ()>,
}

impl Map {
    pub fn new(settings: MapSettings, size: Size) -> Self {
        Map {
            settings,
            size,
            nodes: HashMap::new(),
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        Map {
            settings: MapSettings::default(),
            size: Size::default(),
            nodes: HashMap::new(),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct InitialXYZPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl InitialXYZPosition {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        InitialXYZPosition { x, y, z }
    }
}

impl Default for InitialXYZPosition {
    fn default() -> Self {
        InitialXYZPosition { x: 0, y: 0, z: 0 }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct InitialTarget {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl InitialTarget {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        InitialTarget { x, y, z }
    }
}

impl Default for InitialTarget {
    fn default() -> Self {
        InitialTarget { x: 0, y: 0, z: 0 }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct Camera {
    pub reference_width: i32,
    pub reference_height: i32,
    pub initial_position: InitialXYZPosition,
    pub initial_target: InitialTarget,
}

impl Camera {
    pub fn new(
        reference_width: i32,
        reference_height: i32,
        initial_position: InitialXYZPosition,
        initial_target: InitialTarget,
    ) -> Self {
        Camera {
            reference_width,
            reference_height,
            initial_position,
            initial_target,
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            reference_width: 1920,
            reference_height: 1080,
            initial_position: InitialXYZPosition::default(),
            initial_target: InitialTarget::default(),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct NodeMapData {
    pub curve: Curve,
    pub nodes: HashMap<String, GraphNode>,
    pub map: Map,
    pub camera: Camera,
}

impl NodeMapData {
    pub fn new(curve: Curve, nodes: HashMap<String, GraphNode>, map: Map, camera: Camera) -> Self {
        NodeMapData {
            curve,
            nodes,
            map,
            camera,
        }
    }
}

impl Default for NodeMapData {
    fn default() -> Self {
        NodeMapData {
            curve: Curve::default(),
            nodes: HashMap::new(),
            map: Map::default(),
            camera: Camera::default(),
        }
    }
}

pub fn to_sentence_case(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            first.to_uppercase().collect::<String>() + chars.as_str().to_lowercase().as_str()
        }
    }
}
