use maud::html;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
pub struct HtmlProps {
    pub inner: String,
}

impl maud::Render for HtmlProps {
    fn render(&self) -> maud::Markup {
        html! {
            (maud::PreEscaped(self.inner.clone()))
        }
    }
}
