use maud::{html, Markup, Render};

use super::Block;

#[derive(Debug, Clone)]
pub struct TableProps {
    pub dimension: [u32; 2],
    pub headers: Vec<Block>,
    pub rows: Vec<Vec<Block>>,
}

impl Render for TableProps {
    fn render(&self) -> Markup {
        let grid_cols_template = match self.dimension[1] {
            1 => "grid-cols-1".to_string(),
            2 => "grid-cols-[min-content_1fr]".to_string(),
            cols @ _ => format!(
                "grid-cols-[min-content_min-content_1fr_repeat({}, 1fr)]",
                cols
            ),
        };
        let cl = format!(
            "grid grid-cols-1 gap-4 md:{} grid-container",
            grid_cols_template
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
                                    div class="p-4 bg-[#c1daed] text-[#313231]" {
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
