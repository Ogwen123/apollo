use crate::components::test_list_item::TestListItem;
use crate::state::{Project, StateProvider};
use cargo_ptest::parse::ParsedTest;
use gpui::Overflow::Scroll;
use gpui::prelude::FluentBuilder;
use gpui::{
    AppContext, Context, Element, InteractiveElement, IntoElement, ParentElement, Render,
    StatefulInteractiveElement, Styled, UniformListScrollHandle, Window, div, px, uniform_list,
};

pub struct TestList {}

impl Render for TestList {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let tests = cx
            .state()
            .get_active_project()
            .unwrap_or(Project::default())
            .tests_linear()
            .unwrap();

        div().w_full().h(px(30.0 * tests.len() as f32)).children({
            let mut elements = Vec::new();

            for (index, test) in tests.iter().enumerate() {
                elements.push(cx.new(|_| TestListItem {
                    index,
                    test_data: test.clone(),
                }))
            }

            elements
        })
        // .child(
        //     uniform_list(
        //         "uniform_test_list",
        //         tests.len(),
        //         cx.processor(move |_this, range, _window, _cx| {
        //             let mut items = Vec::new();
        //             for i in range {
        //                 let test_data = tests[i as usize].clone();
        //                 items.push(_cx.new(|_| TestItem {test_data}))
        //             }
        //             items
        //         })
        //
        //     )
        //         .id("test_uniform_list")
        //         .overflow_scroll()
        // )
        // .overflow_scroll()
    }
}
