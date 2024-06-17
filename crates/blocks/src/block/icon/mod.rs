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
            div class="w-full h-full @container flex items-center justify-center gap-2"{
                div class="text-white aspect-square w-full max-w-[min(100cqmin,72px)]" {
                    (PreEscaped(self.name.svg()))
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
            div class="flex items-center justify-center w-full gap-4" {
                        (PreEscaped(self.name.svg()))
            }

        }
    }
}
