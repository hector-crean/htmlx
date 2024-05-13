

use askama::Template;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct BarDatum {
    title: String,
    icon: String,
    content: String,
    percent: f32,
}

impl Default for BarDatum {
    fn default() -> Self {
       Self {
        title: "default-title".to_string(),
        icon: "icon.png".to_string(),
        content: "placeholder".to_string(),
        percent: 20.0,
       }
    }
}

#[derive(Template, Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
#[template(path = "bar-chart.html",)]
#[serde(rename_all = "camelCase")]
pub struct BarChartProps {
    pub title: String,
    pub bars: Vec<BarDatum>,
}



impl Default for BarChartProps {
    fn default() -> Self {
        BarChartProps {
            title: "default-title".to_string(),
            bars: vec![
                BarDatum::default()
            ],
        }
    }
}