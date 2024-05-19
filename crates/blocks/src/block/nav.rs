use maud::html;

use crate::{node::Node, page::Page};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
pub struct NavProps {
    routes: Vec<(String, String)>,
}

impl NavProps {
    pub fn new(routes: Vec<(String, String)>) -> Self {
        Self { routes }
    }
}

impl maud::Render for NavProps {
    fn render(&self) -> maud::Markup {
        {
            html! {
                nav class="flex flex-col items-start justify-start" {
                    @for (path, name) in self.routes.iter() {
                        a href=(format!("/{}", path)) {(path)}
                     }
                }
            }
        }
    }
}
