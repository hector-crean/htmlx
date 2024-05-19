use std::borrow::Cow;

use maud::{html, Render};

use crate::block::Block;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
pub struct Page {
    pub title: String,
    pub subtitle: Option<String>,
    pub blocks: Vec<Block>,
}

impl Page {
    pub fn new(title: &str, subtitle: Option<&str>, blocks: Vec<Block>) -> Self {
        Self {
            blocks,
            title: title.into(),
            subtitle: subtitle.map(|s| s.into()),
        }
    }
}

impl maud::Render for Page {
    fn render(&self) -> maud::Markup {
        html!(
            div class="content gradient-lighter" {
                div class="content-header" {
                    div class="container" {
                        button id="button-back" {}
                        div class="title" {
                            (self.title)
                        }
                    }
                    div class="subtitle" {
                        (self.subtitle.clone().unwrap_or("".into()))
                    }
                }
                div class="text-container" {
                    @for block in &self.blocks {
                        (block)
                    }
                }

            }


        )
    }
}
