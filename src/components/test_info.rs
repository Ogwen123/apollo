use cargo_ptest::parse::ParsedTest;
use gpui::{App, IntoElement, ParentElement, RenderOnce, Window, div};

pub struct TestInfo {
    pub test: ParsedTest,
}

impl RenderOnce for TestInfo {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        div().child(self.test.module_path)
    }
}
