use maud::{PreEscaped, Render};

pub struct DefaultInitJs;

impl Render for DefaultInitJs {
    fn render(&self) -> maud::Markup {
        PreEscaped(
            r#"
        import tabs from '@/components/tabs';

        export default {
            init: () => {
                tabs.init()
            }
        }"#
            .to_string(),
        )
    }
}
