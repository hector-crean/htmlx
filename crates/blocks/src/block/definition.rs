use maud::{html, Markup};

use super::rich_text::RichText;

#[derive(Debug, Clone)]
pub struct DefinitionProps {
    pub color: Option<String>,
    pub id: String,
    pub term: Markup,
    pub definition: Markup,
}

impl Default for DefinitionProps {
    fn default() -> Self {
        Self {
            color: None,
            id: String::default(),
            term: Markup::default(),
            definition: Markup::default(),
        }
    }
}

impl maud::Render for DefinitionProps {
    fn render(&self) -> maud::Markup {
        html! {
            button data-definition-id=(self.id) class="justify-start w-full p-1 text-sm leading-5 bg-white rounded-md symptom hover:bg-gray-200" {

                div class="grid grid-cols-[min-content_1fr_1fr] grid-rows-1 items-center justify-center" {
                    @match &self.color {
                        Some(color) => {
                            div class="flex items-center justify-center w-full h-12 col-span-1 aspect-square" {
                                div class=(format!("w-8 rounded-full aspect-square bg-[radial-gradient(circle_at_50%_75%,{},white)]", color)) {}

                            }
                        }
                        None => {}
                    }
                    div class="flex-1 col-span-2 px-2 py-0 text-sm text-left" {
                        div class="text-sm font-medium text-gray-900" { span {(self.term)}}
                        div class="text-xs text-base text-gray-700" { span { (self.definition)}}
                    }
                }
            }


        }
    }
}
#[derive(Debug, Clone)]
pub struct DefinitionListProps {
    pub definitions: Vec<DefinitionProps>,
}

impl Default for DefinitionListProps {
    fn default() -> Self {
        Self {
            definitions: vec![],
        }
    }
}

impl maud::Render for DefinitionListProps {
    fn render(&self) -> maud::Markup {
        html!(
            div class="@container w-full" {
                div class="flex flex-col @5xl:flex-row w-full h-full gap-4" {
                    @for def in &self.definitions {
                        (def)
                    }
                }
            }
        )
    }
}
