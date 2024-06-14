use maud::html;

pub struct RefNote {
    idx: i32,
}
impl RefNote {
    pub fn new(idx: i32) -> Self {
        Self { idx }
    }
}
impl maud::Render for RefNote {
    fn render(&self) -> maud::Markup {
        html!(a aria-describedby="ref-marker" { (self.idx)})
    }
}
