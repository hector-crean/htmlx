use maud::{html, Render};

#[derive(Debug, Clone)]
pub struct SuggestedNodeProps {
    pub prompt: String,
    pub to_node_id: String,
}
impl SuggestedNodeProps {
    pub fn new(prompt: &str, to_node_id: &str) -> Self {
        Self {
            prompt: prompt.into(),
            to_node_id: to_node_id.into(),
        }
    }
}

impl Render for SuggestedNodeProps {
    fn render(&self) -> maud::Markup {
        html! {
            div class="suggested-node" {
                button class="dotted-button" data-node=(&self.to_node_id) {
                    "For more information, see"
                    p{(&self.prompt)}
                }
            }
        }
    }
}
