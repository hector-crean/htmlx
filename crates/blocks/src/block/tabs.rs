use crate::block::{Block, BlocksProps, RichTextProps};
use maud::html;
use std::borrow::Cow;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
pub struct Tab {
    pub name: String,
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
pub struct TabsProps {
    pub id: uuid::Uuid,
    pub tabs: Vec<Tab>,
}

impl maud::Render for TabsProps {
    fn render(&self) -> maud::Markup {
        html! {
            div data-full-bleed="true" class="flex flex-col items-center justify-center p-4 rounded-lg " {
                div class="w-full toggable-buttons-container tabbed" {
                    @for (index, tab) in self.tabs.iter().enumerate() {
                        button data-group=(self.id) data-slide=(index) data-toggable-button="true" class="toggable-button" {
                            (tab.name)
                        }
                    }
                }

                div class="w-full tabbed-top" {
                    @for (index, tab) in self.tabs.iter().enumerate() {
                        div class="toggable-container" data-group=(self.id) data-slide=(index) {
                            @for block in &tab.blocks {
                                (block)
                            }
                        }
                    }
                }
            }
        }
    }
}
