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
                            div class="flex flex-wrap gap-1 px-1 pt-1 -mb-px text-sm font-medium text-center text-gray-500 dark:text-gray-400" {
                                @for (index, tab) in self.tabs.iter().enumerate() {
                                    button data-group=(self.id) data-slide=(index) data-toggable-button="true" class="inline-block p-4 border-b-0 border-transparent rounded-t-lg hover:text-gray-600 hover:border-gray-300 dark:hover:text-gray-300 active:font-bold active:bg-gradient-to-b from-[#3283cf] to-[#4c9be6] hover:bg-gradient-to-b data-[active=true]:bg-gradient-to-b data-[active=true]:border-gray-300 data-[active=true]:text-white hover:border-gray-300" {
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
                            div class="flex flex-row flex-wrap items-center justify-center gap-1 px-1 pt-1 -mb-px text-sm font-medium text-center text-gray-500 dark:text-gray-400" {
                                @for (index, tab) in self.tabs.iter().enumerate() {
                                    button data-group=(self.id) data-slide=(index) data-toggable-button="true" class="uppercase drop-shadow-md inline-block p-4 border-b-2 border-transparent rounded-lg hover:text-gray-600 hover:border-gray-300 dark:hover:text-gray-300 active:font-bold active:bg-gradient-to-b from-[#3283cf] to-[#4c9be6] hover:bg-gradient-to-b data-[active=true]:bg-gradient-to-b data-[active=true]:border-gray-300 data-[active=true]:text-white hover:border-gray-300" {
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
