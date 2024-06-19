use std::ops::Range;

use maud::{html, Markup, Render};

use super::Block;

// #[derive(Debug)]
pub struct Table<'rows, const ROW: usize, const COL: usize> {
    headers: Option<[&'rows (dyn Render + 'rows); COL]>,
    rows: [[&'rows (dyn Render + 'rows); COL]; ROW],
}

impl<'rows, const ROW: usize, const COL: usize> Table<'rows, ROW, COL> {
    // Creates a new Table with given headers and rows
    pub fn new(
        headers: Option<[&'rows (dyn Render + 'rows); COL]>,
        rows: [[&'rows (dyn Render + 'rows); COL]; ROW],
    ) -> Self {
        Table { headers, rows }
    }
}

impl<'rows, const ROW: usize, const COL: usize> Render for Table<'rows, ROW, COL> {
    fn render(&self) -> maud::Markup {
        let COLS = self.rows[0].len();

        let grid_cols_template = match &COLS {
            0 | 1 => "grid-cols-1".to_string(),
            2 => "grid-cols-[min-content_1fr]".to_string(),
            cols @ _ => format!(
                "grid-cols-[min-content_min-content_repeat({},1fr)]",
                cols - 2
            ),
        };

        let grid_class = format!("grid grid-cols-1 @md:{}", grid_cols_template);

        html! {

        div class="@container flex flex-col gap-y-1 text-[#6e777e] gap-x-1" {
            div class=(grid_class) {
            @match self.headers {
                Some(headers) => {
                    @for (header_idx, header) in headers.iter().enumerate() {
                            div class="col-span-1 p-2 bg-[#0b5079] text-[#eaeded] flex flex-col items-center justify-center" { (header)}
                        }
                }
                None => {}
            }
            @for (row_idx, row) in self.rows.iter().enumerate() {

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
