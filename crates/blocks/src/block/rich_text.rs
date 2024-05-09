use askama::Template; 


#[derive(Clone)] 
pub enum RichText {
    Html(String)
}
impl std::fmt::Display for RichText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RichText::Html(content) => {
                write!(f, "{}", content)

            }
        }
    }
}

#[derive(Template, Clone)] 
#[template(path = "rich_text.html")] 
                                 
pub struct RichTextTemplate { 
    pub text: RichText,             
}


