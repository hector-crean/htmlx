use maud::{html, Render};

use super::Block;

#[derive(Debug, Clone)]
pub struct GridTableProps {
    pub rows: Vec<Vec<Block>>,
}

impl Render for GridTableProps {
    fn render(&self) -> maud::Markup {
        let COLS = self.rows[0].len();
        let row_class = format!(
            "grid grid-cols-1 gap-4 overflow-hidden rounded-md md:grid-cols-[200px_repeat({},1fr)]",
            COLS - 1
        );
        html! {

        div class="flex flex-col [&>*:nth-child(odd)]:bg-[#c1daed] text-[#6e777e] [&>*:nth-child(even)]:bg-[#ededed] gap-1" {
            @for row in &self.rows {
                div class=(row_class) {
                    @for (i, cell) in row.iter().enumerate() {
                        @match i {
                            0 => {
                                div class="col-span-1 p-2 bg-[#0b5079] text-[#eaeded]" { (cell)}
                            }
                            _ => {
                                div class="col-span-1 p-2 " { (cell)}
                            }
                        }

                    }
                }
            }
        }

        }
    }
}
