use blocks::block::{
    bar_chart::{BarChartProps, BarDatum},
    rich_text::{RichText, RichTextProps},
    Block,
};

use crate::rich_text_block;

fn psychiatric_comborbidities_bars() -> Vec<BarDatum> {
    vec![
                BarDatum {
                    id: uuid::Uuid::new_v4(),
                    short_title: String::from("Substance use"),
                    full_title: String::from("Substance use"),
                    icon: String::from(""),
                    content: rich_text_block!("../../../input/clinical_presentation_comorbidities/ef31ae84-3c15-4fe3-a4c2-fc5fc3bbba00.html"),
                    percent: 46.0
                },
                BarDatum {
                    id: uuid::Uuid::new_v4(),
                    short_title: String::from("Alcohol use"),
                    full_title: String::from("Alcohol use"),
                    icon: String::from(""),
                    content: rich_text_block!("../../../input/clinical_presentation_comorbidities/a9c28646-8a85-4451-b325-84f66b4bd3b0.html"),

                    percent: 10.
                },
                BarDatum {
                    id: uuid::Uuid::new_v4(),
                    short_title: String::from("Major Depressive Disorder"),
                    full_title: String::from("Major Depressive Disorder"),
                    icon: String::from(""),
                    content: rich_text_block!("../../../input/clinical_presentation_comorbidities/205c7abf-3558-47df-8802-24b65d66e573.html"),
                    percent: 50.
                },
                BarDatum {
                    id: uuid::Uuid::new_v4(),
                    short_title: String::from("Anxiety"),
                    full_title: String::from("Anxiety"),
                    icon: String::from(""),
                    content: rich_text_block!("../../../input/clinical_presentation_comorbidities/a635eebc-678b-49bd-b3c6-7f784c0b0b1a.html"),
                    percent: 0.
                },
                BarDatum {
                    id: uuid::Uuid::new_v4(),
                    short_title: String::from("Self-harm"),
                    full_title: String::from("Self-harm"),
                    icon: String::from(""),
                    content: rich_text_block!("../../../input/clinical_presentation_comorbidities/a38fe64f-0afb-4b78-9c63-c3af002238ac.html"),
                    percent: 19.
                }
            ]
}

fn medical_comorbidities_bars() -> Vec<BarDatum> {
    vec![
        BarDatum {
            id: uuid::Uuid::new_v4(),
            short_title: String::from("Sleep dysfunction"),
            full_title:String::from("Sleep dysfunction"),
            icon: String::from(""),
            content: rich_text_block!("../../../input/clinical_presentation_comorbidities/ec93bd2c-999e-4377-9a9f-ba2ff44e0479.html"),
            percent: 87.
        },
        BarDatum {
            id: uuid::Uuid::new_v4(),
            short_title: String::from("Chronic Pain"),
            full_title: String::from("Chronic Pain"),
            icon: String::from(""),
            content: rich_text_block!("../../../input/clinical_presentation_comorbidities/78ec8944-fa18-455e-babe-c5fa6cd7fb69.html"),
            percent: 20.
        },
        BarDatum {
            id: uuid::Uuid::new_v4(),
            short_title: String::from("Inflammation"),
            full_title: String::from("Inflammation"),
            icon: String::from(""),
            content: rich_text_block!("../../../input/clinical_presentation_comorbidities/e2a3348c-921d-4aee-ba5d-0d9d5ea0d12d.html"),

            percent: 50.
        },
        BarDatum {
            id: uuid::Uuid::new_v4(),
            short_title: String::from("Cardiometabolic disorders"),
            full_title: String::from("Cardiometabolic disorders"),
            icon: String::from(""),
            content: rich_text_block!("../../../input/clinical_presentation_comorbidities/f49aea70-5faa-4d15-9f1f-c461a474f83c.html"),

            percent: 0.
        },
        BarDatum {
            id: uuid::Uuid::new_v4(),
            short_title: String::from("Dementia"),
            full_title: String::from("Dementia"),
            icon: String::from(""),
            content: rich_text_block!("../../../input/clinical_presentation_comorbidities/89a0070f-52df-4323-926e-cf76f526ced9.html"),
            percent: 19.
        }
    ]
}
pub fn blocks() -> Vec<Block> {
    vec![
        rich_text_block!("../../../input/clinical_presentation_comorbidities/fc9c5804-9b7a-420c-8e82-a28be4076d56.html"),
        Block::BarChartBlock(BarChartProps {
            title: String::from("Psychiatric Comorbidities"),
            bars: psychiatric_comborbidities_bars()
        }),

        Block::BarChartBlock(BarChartProps {
            title: String::from("Medical Comorbidities"),
            bars: medical_comorbidities_bars()
        }),

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
        rich_text_block!("../../../input/clinical_presentation_comorbidities/e0c2fb34-6776-4a5e-bc6f-cbaacfc675b2.html")


    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn print_psychiatric_comborbidities_bar() -> Result<(), serde_json::Error> {
        let psychiatric_comborbidities_bars = psychiatric_comborbidities_bars();
        let json = serde_json::to_string_pretty(&psychiatric_comborbidities_bars)?;

        println!("{}", json);

        Ok(())
    }

    #[test]
    fn print_medical_comorbidities_bars() -> Result<(), serde_json::Error> {
        let medical_comorbidities_bars = medical_comorbidities_bars();
        let json = serde_json::to_string_pretty(&medical_comorbidities_bars)?;

        println!("{}", json);

        Ok(())
    }
}
