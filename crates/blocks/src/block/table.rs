use maud::{html, Markup, Render};

use super::Block;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
pub struct TableProps {
    pub dimension: [u32; 2],
    pub headers: Vec<Block>,
    pub rows: Vec<Vec<Block>>,
}

impl Render for TableProps {
    fn render(&self) -> Markup {
        let cl = format!(
            "grid grid-cols-1 gap-4 md:grid-cols-{} grid-container",
            self.dimension[1]
        );
        html! {
            div class="container p-4 mx-auto rounded-sm" {
                div class=(cl) {
                    thead {
                        tr class="bg-gray-200" {
                            @for header in &self.headers {
                                div class="p-4 font-bold bg-gray-200" {
                                    (header)
                                }
                            }
                        }
                    }
                    tbody {
                        @for row in &self.rows {
                            tr {
                                @for cell in row {
                                    div class="p-4 bg-[#c1daed]" {
                                        (cell)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
