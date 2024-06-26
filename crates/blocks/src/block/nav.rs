use crate::{
    node::{Node, RouteStatus, RouteTemplate},
    page::Page,
};
use maud::html;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
pub struct NavProps {
    pub routes: Vec<RouteTemplate>,
}

impl NavProps {
    pub fn new(routes: Vec<RouteTemplate>) -> Self {
        Self { routes }
    }

    fn render_nav_container(&self) -> maud::Markup {
        html! {
            nav class="relative w-full h-full p-4" {
                (self.render_background())
                (self.render_main_content())
                (self.render_bottom_decoration())
            }
        }
    }

    fn render_background(&self) -> maud::Markup {
        html! {
            div class="absolute bottom-0 left-11 right-0 top-8 bg-slate-900/[0.03]" {}
        }
    }

    fn render_main_content(&self) -> maud::Markup {
        html! {
            div class="pointer-events-auto relative z-10 rounded-lg bg-white text-[0.8125rem] leading-5 text-slate-700 shadow-xl shadow-black/5 ring-1 ring-slate-700/10 w-full" {
                div class="flex flex-col gap-3" {
                    div class="flex items-center px-3.5 py-2.5 text-slate-400" {
                        "Pages"
                    }
                    div class="border-t border-slate-400/20 px-3.5 py-3 flex flex-col gap-2" {
                        @for route in &self.routes {
                            (self.render_route(route))
                        }
                    }
                }
            }
        }
    }

    fn render_route(&self, route: &RouteTemplate) -> maud::Markup {
        let name = route
            .template
            .rsplit_once("/")
            .unwrap()
            .0
            .replace("/", " / ");

        let (opacity, emoji, pointer) = match &route.status {
            RouteStatus::Finished => (100, "✅", "pointer-events-auto"),
            RouteStatus::NotStarted => (20, "⛔", "pointer-events-none"),
            RouteStatus::UnderDevelopment => (100, "🚧", "pointer-events-auto"),
        };

        let container_class = format!("text-white hover:underline opacity-{} {}", opacity, pointer);

        html! {
            div id=(route.path.rsplit("/").collect::<Vec<&str>>()[1]) class=(container_class) {
                div class="flex items-center rounded-md p-1.5 bg-gray-600 hover:bg-[#B5D1DF]" {
                    svg class="mr-2.5 h-5 w-5 flex-none stroke-slate-400" fill="none" viewBox="0 0 24 24" stroke-width="2" {
                        path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" {}
                    }
                    (emoji)
                    (name)
                }
            }
        }
    }

    fn render_bottom_decoration(&self) -> maud::Markup {
        html! {
            div class="z-0" {
                div class="absolute -right-[95%] left-0 top-8 h-px bg-slate-900/[0.1] [mask-image:linear-gradient(to_right,transparent,white_4rem,white_calc(100%-4rem),transparent)]" {}
                div class="absolute -bottom-16 -top-2 right-0 w-px bg-slate-900/[0.1] [mask-image:linear-gradient(to_top,transparent,white_4rem,white_calc(100%-4rem),transparent)]" {}
                div class="absolute flex items-end h-8 -mb-px overflow-hidden -right-2/3 top-px" {
                    div class="flex -mb-px h-[2px] w-80 -scale-x-100" {
                        div class="w-full flex-none blur-sm [background-image:linear-gradient(90deg,rgba(56,189,248,0)_0%,#0EA5E9_32.29%,rgba(236,72,153,0.3)_67.19%,rgba(236,72,153,0)_100%)]" {}
                        div class="-ml-[100%] w-full flex-none blur-[1px] [background-image:linear-gradient(90deg,rgba(56,189,248,0)_0%,#0EA5E9_32.29%,rgba(236,72,153,0.3)_67.19%,rgba(236,72,153,0)_100%)]" {}
                    }
                }
            }
        }
    }
}

impl maud::Render for NavProps {
    fn render(&self) -> maud::Markup {
        self.render_nav_container()
    }
}
