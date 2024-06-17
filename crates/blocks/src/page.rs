use std::borrow::Cow;

use maud::{html, Render};

use crate::block::Block;

#[derive(Debug, Clone)]
pub struct Page<R: maud::Render> {
    pub title: String,
    pub subtitle: Option<String>,
    pub page: R,
}

impl<R: maud::Render> Page<R> {
    pub fn new(title: &str, subtitle: Option<&str>, page: R) -> Self {
        Self {
            page,
            title: title.into(),
            subtitle: subtitle.map(|s| s.into()),
        }
    }
}

impl<R: maud::Render> maud::Render for Page<R> {
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
                div class="pt-2 presentation_wrapper" {
                    (self.page)
                }

            }


        )
    }
}
