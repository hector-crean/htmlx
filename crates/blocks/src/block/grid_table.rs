use maud::{html, Render};

use super::Block;

#[derive(Debug, Clone)]
enum GridTableRepresentation {
    WithIcon,
    WithoutIcon,
}

#[derive(Debug, Clone)]
pub struct GridTableProps {
    pub headers: Vec<Block>,
    pub rows: Vec<Vec<Block>>,
    // pub representation: GridTableRepresentation
}

impl Render for GridTableProps {
    fn render(&self) -> maud::Markup {
        let COLS = self.rows[0].len();

        let grid_cols_template = match &COLS {
            0 | 1 => "grid-cols-1".to_string(),
            2 => "grid-cols-[100px_1fr]".to_string(),
            cols @ _ => format!("grid-cols-[100px_100px_repeat({},1fr)]", cols - 2),
        };

        let row_class = format!("grid grid-cols-1 @md:{}", grid_cols_template);

        html! {

        div class="@container flex flex-col gap-y-1 text-[#6e777e] gap-x-1" {
            div class=(row_class) {
                @for (header_idx, header) in self.headers.iter().enumerate() {
                    div class="col-span-1 p-2 bg-[#0b5079] text-[#eaeded] flex flex-col items-center justify-center" { (header)}
                }
            }
            @for (row_idx, row) in self.rows.iter().enumerate() {
                div class=(row_class) {
                    @for (col_idx, cell) in row.iter().enumerate() {
                        @match (row_idx , col_idx) {
                            // (_, 0)  => {
                            //     div class="col-span-1 p-2 bg-[#0b5079] text-[#eaeded] flex flex-col items-center justify-center" { (cell)}
                            // }
                            (_,_) if row_idx % 2 == 0 => {
                                div class="col-span-1 p-2 bg-[#c1daed] flex flex-col items-center justify-center" { (cell)}
                            }
                            (_,_) if row_idx % 2 == 1 => {
                                div class="flex flex-col items-center justify-center col-span-1 p-2 bg-[#d5e5f0]" { (cell)}
                            }
                            _ => {
                                div class="col-span-1 p-2 bg-[#c1daed]" { (cell)}

                            }

                        }

                    }
                }
            }
        }

        }
    }
}
