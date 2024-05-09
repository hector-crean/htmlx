use askama::Template;
use crate::block::{BlocksTemplate, RichTextTemplate, Block};





#[derive(Clone)] 
pub struct Tab {
    pub name: String,
    pub tab_content: BlocksTemplate
}

#[derive(Template, Clone)] 
#[template(path = "tabs.html", ext = "html", escape = "none")] 
pub struct TabsTemplate<'a> {
    pub id: &'a str,  // Changed to a borrowed string
    pub tabs: Vec<Tab>
}