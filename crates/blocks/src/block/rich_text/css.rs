use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use strum::{AsRefStr, Display, EnumString, EnumVariantNames};

#[derive(
    Hash,
    Eq,
    PartialEq,
    Debug,
    EnumString,
    EnumVariantNames,
    AsRefStr,
    Display,
    Serialize,
    Deserialize,
)]
#[strum(serialize_all = "camelCase")]
pub enum CssProperty {
    BackgroundColor,
    TextColor,
    Margin,
    MarginTop,
    MarginRight,
    MarginBottom,
    MarginLeft,
    Padding,
    PaddingTop,
    PaddingRight,
    PaddingBottom,
    PaddingLeft,
    Width,
    Height,
    FontSize,
    FontWeight,
    LineHeight,
    LetterSpacing,
    BorderWidth,
    BorderColor,
    BorderRadius,
    Display,
    FlexDirection,
    JustifyContent,
    AlignItems,
    TextAlign,
    // Add more properties as needed
}

impl CssProperty {
    fn from_str(s: &str) -> Option<CssProperty> {
        s.parse().ok()
    }
}

pub fn css_to_tailwind(css_properties: HashMap<String, Value>) -> String {
    let mut tailwind_classes = Vec::new();

    for (property, value) in css_properties.iter() {
        match (property, value) {
            (property, Value::String(value)) => {
                if let Some(css_property) = CssProperty::from_str(property) {
                    let tailwind_class = match css_property {
                        CssProperty::BackgroundColor => match value.as_str() {
                            "transparent" => "bg-transparent".to_string(),
                            "currentColor" => "bg-current".to_string(),
                            v if v.starts_with("#") => format!("bg-[#{}]", v.replace("#", "")),
                            _ => format!("bg-{}", value),
                        },
                        CssProperty::TextColor => match value.as_str() {
                            "transparent" => "text-transparent".to_string(),
                            "currentColor" => "text-current".to_string(),
                            v if v.starts_with("#") => format!("text-[#{}]", v.replace("#", "")),
                            _ => format!("text-{}", value),
                        },
                        CssProperty::Margin => format!("m-{}", value),
                        CssProperty::MarginTop => format!("mt-{}", value),
                        CssProperty::MarginRight => format!("mr-{}", value),
                        CssProperty::MarginBottom => format!("mb-{}", value),
                        CssProperty::MarginLeft => format!("ml-{}", value),
                        CssProperty::Padding => format!("p-{}", value),
                        CssProperty::PaddingTop => format!("pt-{}", value),
                        CssProperty::PaddingRight => format!("pr-{}", value),
                        CssProperty::PaddingBottom => format!("pb-{}", value),
                        CssProperty::PaddingLeft => format!("pl-{}", value),
                        CssProperty::Width => format!("w-{}", value),
                        CssProperty::Height => format!("h-{}", value),
                        CssProperty::FontSize => format!("text-{}", value),
                        CssProperty::FontWeight => format!("font-{}", value),
                        CssProperty::LineHeight => format!("leading-{}", value),
                        CssProperty::LetterSpacing => format!("tracking-{}", value),
                        CssProperty::BorderWidth => format!("border-{}", value),
                        CssProperty::BorderColor => match value.as_str() {
                            "transparent" => "border-transparent".to_string(),
                            "currentColor" => "border-current".to_string(),
                            v if v.starts_with("#") => format!("border-{}", v.replace("#", "")),
                            _ => format!("border-{}", value),
                        },
                        CssProperty::BorderRadius => format!("rounded-{}", value),
                        CssProperty::Display => match value.as_str() {
                            "block" => "block".to_string(),
                            "inlineBlock" => "inline-block".to_string(),
                            "inline" => "inline".to_string(),
                            "flex" => "flex".to_string(),
                            "inlineFlex" => "inline-flex".to_string(),
                            "grid" => "grid".to_string(),
                            "inlineGrid" => "inline-grid".to_string(),
                            "table" => "table".to_string(),
                            "inlineTable" => "inline-table".to_string(),
                            _ => value.to_string(),
                        },
                        CssProperty::FlexDirection => match value.as_str() {
                            "row" => "flex-row".to_string(),
                            "rowReverse" => "flex-row-reverse".to_string(),
                            "column" => "flex-col".to_string(),
                            "columnReverse" => "flex-col-reverse".to_string(),
                            _ => value.to_string(),
                        },
                        CssProperty::JustifyContent => match value.as_str() {
                            "flexStart" => "justify-start".to_string(),
                            "flexEnd" => "justify-end".to_string(),
                            "center" => "justify-center".to_string(),
                            "spaceBetween" => "justify-between".to_string(),
                            "spaceAround" => "justify-around".to_string(),
                            "spaceEvenly" => "justify-evenly".to_string(),
                            _ => value.to_string(),
                        },
                        CssProperty::AlignItems => match value.as_str() {
                            "flexStart" => "items-start".to_string(),
                            "flexEnd" => "items-end".to_string(),
                            "center" => "items-center".to_string(),
                            "baseline" => "items-baseline".to_string(),
                            "stretch" => "items-stretch".to_string(),
                            _ => value.to_string(),
                        },
                        CssProperty::TextAlign => match value.as_str() {
                            "left" => "text-left".to_string(),
                            "center" => "text-center".to_string(),
                            "right" => "text-right".to_string(),
                            "justify" => "text-justify".to_string(),
                            _ => value.to_string(),
                        },
                        // Handle other properties similarly
                    };
                    tailwind_classes.push(tailwind_class);
                }
            }
            _ => {}
        }
    }

    tailwind_classes.join(" ")
}
