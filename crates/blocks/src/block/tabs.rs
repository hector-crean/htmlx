use crate::block::{Block, BlocksProps, RichTextProps};
use maud::html;
use std::borrow::Cow;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
pub enum TabsRepresentation {
    Standard,
    Swiper,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
pub struct Tab {
    pub name: String,
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
pub struct TabsProps {
    pub id: uuid::Uuid,
    pub tabs: Vec<Tab>,
    pub representation: TabsRepresentation,
}

impl maud::Render for TabsProps {
    fn render(&self) -> maud::Markup {
        html! {
            @match self.representation {
                TabsRepresentation::Standard => {
                    div data-full-bleed="true" class="flex flex-col items-center justify-center bg-[#313231]  gap-2 text-white" {

                        div class="w-full border-b border-[#5C96CA] " {
                            div class="flex flex-wrap gap-1 px-1 pt-1 -mb-px text-sm font-medium text-center text-gray-500 dark:text-gray-400" {
                                @for (index, tab) in self.tabs.iter().enumerate() {
                                    button data-group=(self.id) data-slide=(index) data-toggable-button="true" class="inline-block p-4 border-b-2 border-transparent rounded-t-lg hover:text-gray-600 hover:border-gray-300 dark:hover:text-gray-300 active:font-bold active:bg-[#5C96CA] hover:bg-[#5C96CA] data-[active=true]:bg-[#5C96CA] data-[active=true]:border-gray-300 data-[active=true]:text-white hover:border-gray-300" {
                                        (tab.name)
                                    }
                                }
                            }
                        }

                        div class="w-full" {
                            @for (index, tab) in self.tabs.iter().enumerate() {
                                div id="toggable-container" class="w-full p-4" data-group=(self.id) data-slide=(index) {
                                        @for block in &tab.blocks {
                                            (block)
                                        }

                                }
                            }
                        }
                    }
                }
                TabsRepresentation::Swiper => {
                    div class="swiper" {
                        // Additional required wrapper
                        div class="swiper-wrapper" {
                            // Slides
                            @for tab in self.tabs.iter() {
                                div class="swiper-slide" {
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
                        div class="swiper-scrollbar" {}
                    }
                }
            }
        }
    }
}
