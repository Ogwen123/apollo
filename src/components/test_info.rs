use cargo_ptest::parse::ParsedTest;
use gpui::{div, App, IntoElement, ParentElement, RenderOnce, Window};

pub struct TestInfo {
    pub test: ParsedTest
}

impl RenderOnce for TestInfo {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        div().child(self.test.module_path)
    }
}