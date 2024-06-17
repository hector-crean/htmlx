use crate::block::{Block, BlocksProps, RichTextProps};
use maud::html;
use std::borrow::Cow;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
pub struct TabsTheme {
    pub bg_color: String,
    pub text_color: String,
}
impl TabsTheme {
    pub fn new(bg_color: &str, text_color: &str) -> Self {
        Self {
            bg_color: bg_color.into(),
            text_color: text_color.into(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TabsRepresentation {
    TopLevel,
    Internal { theme: TabsTheme, full_bleed: bool },
    Swiper,
}

#[derive(Debug, Clone)]
pub struct Tab {
    pub name: String,
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone)]
pub struct TabsProps {
    pub id: uuid::Uuid,
    pub tabs: Vec<Tab>,
    pub representation: TabsRepresentation,
}

impl maud::Render for TabsProps {
    fn render(&self) -> maud::Markup {
        //text-white bg-[#313231]
        html! {
            @match &self.representation {
                TabsRepresentation::TopLevel => {
                    div data-full-bleed="true" class="flex flex-col items-center justify-center flex-1 gap-2" {

                        div class="w-full border-b border-b-4 border-[#4c9be6] " {
                            div class="toggable-buttons-container tabbed" {
                                @for (index, tab) in self.tabs.iter().enumerate() {
                                    button data-group=(self.id) data-slide=(index) data-toggable-button="true" class="toggable-button" {
                                        (tab.name)
                                    }
                                }
                            }
                        }

                        div class="w-full" {
                            @for (index, tab) in self.tabs.iter().enumerate() {
                                div id="toggable-container" class="presentation_wrapper" data-group=(self.id) data-slide=(index) {
                                        @for block in &tab.blocks {
                                            (block)
                                        }

                                }
                            }
                        }
                    }
                }
                TabsRepresentation::Internal { theme, full_bleed } => {
                    @let class_name = format!("flex flex-col pb-2 items-center justify-center overflow-hidden gap-2 rounded-lg text-[{}] bg-[{}]", &theme.text_color, &theme.bg_color);
                    div data-full-bleed=(full_bleed) class=(class_name) {

                        div class="w-full" {
                            div class="mt-8 mb-4 toggable-buttons-container" {
                                @for (index, tab) in self.tabs.iter().enumerate() {
                                    button data-group=(self.id) data-slide=(index) data-toggable-button="true" class="toggable-button padded" {
                                        (tab.name)
                                    }
                                }
                            }
                        }

                        div class="w-full" {
                            @for (index, tab) in self.tabs.iter().enumerate() {
                                div id="toggable-container" class="presentation_wrapper" data-group=(self.id) data-slide=(index) {
                                        @for block in &tab.blocks {
                                            (block)
                                        }

                                }
                            }
                        }
                    }
                }
                TabsRepresentation::Swiper => {
                    div class="swiper w-full max-h-[100vh]" {
                        // Additional required wrapper
                        div class="swiper-wrapper" {
                            // Slides
                            @for tab in self.tabs.iter() {
                                div class="flex flex-col items-start justify-center pb-12 swiper-slide" {
                                    @for block in &tab.blocks {
                                        (block)
                                    }
                                }
                            }

                        }
                        // If we need pagination
                        div class="swiper-pagination" {}

                        // If we need navigation buttons
                        div class="swiper-button-prev" {}
                        div class="swiper-button-next" {}

                        // If we need scrollbar
                        // div class="swiper-scrollbar" {}
                    }
                }
            }
        }
    }
}
