use maud::{html, PreEscaped};

use crate::SvgName;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct IconProps {
    pub name: SvgName,
    // pub max_width: Option<i32>
}

impl Default for IconProps {
    fn default() -> Self {
        IconProps {
            name: SvgName::default(),
            // max_width: None,
        }
    }
}

impl maud::Render for IconProps {
    fn render(&self) -> maud::Markup {
        html! {
            div class="container p-4 mx-auto" {
                div class="flex flex-wrap justify-center gap-4" {
                    div class="flex items-center justify-center w-16 h-16 text-white bg-blue-500 rounded-full shadow-lg" {
                        (PreEscaped(self.name.svg()))
                    }
                }

            }

        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SvgProps {
    pub name: SvgName,
    // pub max_width: Option<i32>
}

impl Default for SvgProps {
    fn default() -> Self {
        SvgProps {
            name: SvgName::default(),
        }
    }
}

impl maud::Render for SvgProps {
    fn render(&self) -> maud::Markup {
        html! {
            div class="flex items-center justify-center w-full h-full gap-4 shadow-lg" {
                        (PreEscaped(self.name.svg()))
            }

        }
    }
}
