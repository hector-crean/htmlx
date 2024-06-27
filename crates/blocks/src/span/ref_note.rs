use maud::html;

pub struct RefNote(pub i32);
impl RefNote {
    pub fn new(idx: i32) -> Self {
        Self(idx)
    }
}
impl maud::Render for RefNote {
    fn render(&self) -> maud::Markup {
        html!(a aria-describedby="ref-marker" { (self.0)})
    }
}

pub struct FootNote(pub String);
impl FootNote {
    pub fn new(mark: &str) -> Self {
        Self(mark.into())
    }
}
impl maud::Render for FootNote {
    fn render(&self) -> maud::Markup {
        html!(a aria-describedby="ref-marker" { (self.0)})
    }
}
