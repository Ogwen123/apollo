use cargo_ptest::parse::ParsedTest;
use gpui::{App, Context, IntoElement, ParentElement, Render, RenderOnce, Window, div};

pub struct TestItem {
    pub test_data: ParsedTest,
}

impl Render for TestItem {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .child(self.test_data.module_path.clone())
            .child(self.test_data.status.to_string())
    }
}
