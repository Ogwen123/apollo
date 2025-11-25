use crate::components::test_list_item::TestListItem;
use crate::state::{Project, ScrollHandles, StateProvider};
use gpui::{App, AppContext, Context, Element, InteractiveElement, IntoElement, ParentElement, Render, RenderOnce, StatefulInteractiveElement, Styled, UniformListScrollHandle, Window, div, px, uniform_list, BorrowAppContext};

pub struct TestList {
    pub test_list_viewport: f32
}

impl RenderOnce for TestList {
    fn render(mut self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let tests = cx
            .state()
            .get_active_project()
            .unwrap_or(Project::default())
            .tests_linear()
            .unwrap_or(Vec::new());

        let line_height = 30.0;
        let raw_height = line_height * tests.len() as f32;
        let height = px(raw_height);

        div()
            .absolute()
            .top(px(cx.global::<ScrollHandles>().test_list))
            .left(px(0.0))
            .h(height)
            .on_scroll_wheel(move |e, _, cx| {
                let current = cx.global::<ScrollHandles>().test_list;
                let delta = e.delta.pixel_delta(px(1.0)).y.to_f64() as f32;

                cx.update_global::<ScrollHandles, ()>(|global, _| {
                    if current + delta < -(raw_height - self.test_list_viewport) {
                        global.test_list = -(raw_height - self.test_list_viewport);
                    } else if current + delta > 0.0 {
                        global.test_list = 0.0;
                    } else {
                        global.test_list += delta;
                    }
                })
            })
            .w_full()
            .child(
                div()
                    .id("test-list")
                    .flex()
                    .flex_col()
                    .w_full()
                    .h(height)
                    .overflow_scroll()
                    .children({
                        let mut elements = Vec::new();

                        for (index, test) in tests.iter().enumerate() {
                            elements.push(cx.new(|_| TestListItem {
                                index,
                                test_data: test.clone(),
                            }))
                        }

                        elements
                    }),
            )
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
