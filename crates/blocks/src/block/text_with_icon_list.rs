use maud::{html, Render};

use super::icon::IconProps;

#[derive(Clone, Debug)]
pub struct TextWithIconList {
    pub list: Vec<(IconProps, String)>,
}

impl Render for TextWithIconList {
    fn render(&self) -> maud::Markup {
        html! {
            div class="grid w-full grid-cols-1 gap-2 md:grid-cols-2" {
                @for item in &self.list {
                div class="w-full grid grid-cols-[min-content_1fr] grid-rows-1"{

                        div class="w-24"  {
                           (item.0)
                        }
                        div class="flex flex-col items-start justify-center"{
                            span { (item.1)}
                        }
                    }
                }
            }
        }
    }
}
