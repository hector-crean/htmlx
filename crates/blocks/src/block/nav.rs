use maud::html;

use crate::{
    node::{Node, RouteTemplate},
    page::Page,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
pub struct NavProps {
    pub routes: Vec<RouteTemplate>,
}

impl NavProps {
    pub fn new(routes: Vec<RouteTemplate>) -> Self {
        Self { routes }
    }
}

impl maud::Render for NavProps {
    fn render(&self) -> maud::Markup {
        {
            html! {
                nav class="flex flex-col items-start justify-start" {
                    @for RouteTemplate { path, template } in self.routes.iter() {
                        a href=(format!("{}", path)) {(path)}
                     }
                }
            }
        }
    }
}
