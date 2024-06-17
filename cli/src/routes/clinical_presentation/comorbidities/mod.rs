use crate::rich_text_block;
use blocks::block::{
    bar_chart::BarChartDatum, html, placeholder_container::PlaceholderContainerProps,
    references::ReferencesProps, Block,
};
use maud::{html, Render};

pub struct Page;

impl Render for Page {
    fn render(&self) -> maud::Markup {
        html! {
        // rich_text_block!("../../../input/clinical_presentation_comorbidities/fc9c5804-9b7a-420c-8e82-a28be4076d56.html"),
        // Block::BarChartBlock(BarChartProps {
        //     title: String::from("Psychiatric Comorbidities"),
        //     bars: psychiatric_comborbidities_bars()
        // }),
        h2 {
            "Comborbidities associated with PTSD"
        }
        div class="panel" {
            p {
                span class="text-bold"{"Filter"} " the graph by medical or psychiatric comorbidities. "
                span class="text-bold"{"Select"}" a comorbidity bar to see more info."
            }

        }

        (Block::PlaceholderContainerBlock(PlaceholderContainerProps {
            id: String::from("ptsd-comorbidities-bar-chart"),
            full_bleed: true,
            class: String::from("round-lg")
        }))

        // Block::BarChartBlock(BarChartProps {
        //     title: String::from("Medical Comorbidities"),
        //     bars: medical_comorbidities_bars()
        // }),

        // rich_text_block!("../../../input/clinical_presentation_comorbidities/ef31ae84-3c15-4fe3-a4c2-fc5fc3bbba00.html"),
        // rich_text_block!("../../../input/clinical_presentation_comorbidities/a9c28646-8a85-4451-b325-84f66b4bd3b0.html"),
        // rich_text_block!("../../../input/clinical_presentation_comorbidities/205c7abf-3558-47df-8802-24b65d66e573.html"),
        // rich_text_block!("../../../input/clinical_presentation_comorbidities/a635eebc-678b-49bd-b3c6-7f784c0b0b1a.html"),
        // rich_text_block!("../../../input/clinical_presentation_comorbidities/a38fe64f-0afb-4b78-9c63-c3af002238ac.html"),
        // rich_text_block!("../../../input/clinical_presentation_comorbidities/ec93bd2c-999e-4377-9a9f-ba2ff44e0479.html"),
        // rich_text_block!("../../../input/clinical_presentation_comorbidities/78ec8944-fa18-455e-babe-c5fa6cd7fb69.html"),
        // rich_text_block!("../../../input/clinical_presentation_comorbidities/e2a3348c-921d-4aee-ba5d-0d9d5ea0d12d.html"),
        // rich_text_block!("../../../input/clinical_presentation_comorbidities/f49aea70-5faa-4d15-9f1f-c461a474f83c.html"),
        // rich_text_block!("../../../input/clinical_presentation_comorbidities/89a0070f-52df-4323-926e-cf76f526ced9.html"),
        // rich_text_block!("../../../input/clinical_presentation_comorbidities/f9f9d6ac-a315-469a-b56c-d61606380c5b.html"),

        (Block::ReferencesBlock(Box::new(ReferencesProps {
            references: rich_text_block!("../../../input/clinical_presentation_comorbidities/e0c2fb34-6776-4a5e-bc6f-cbaacfc675b2.html")
        })))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
