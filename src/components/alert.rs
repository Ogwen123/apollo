use crate::AlertHandler;
use crate::state::{AlertSeverity, AlertType, State, StateProvider};
use crate::style::{Colour, Size, StyleProvider};
use crate::widgets::core::button::button::ContentPosition;
use crate::widgets::core::button::icon_button::IconButton;
use crate::widgets::core::icon::Icons;
use gpui::prelude::FluentBuilder;
use gpui::{
    App, BorrowAppContext, InteractiveElement, IntoElement, ParentElement, RenderOnce, Styled,
    Window, div, px, rgb,
};
use std::cmp::max;

pub struct AlertDisplay {}

impl RenderOnce for AlertDisplay {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let alert_option = cx.state().alert.clone();

        let window_w = window.viewport_size().width.to_f64().floor() as i32;

        let top_margin = 50.0;
        let left_margin = 40.0;

        let w = max(window_w - left_margin as i32 * 2, 0) as f32;

        div().when_some(alert_option, |_self, alert| {
            _self
                .absolute()
                .top(px(top_margin))
                .left(px(left_margin))
                .w(px(w))
                .h(px(60.0))
                .text_color(rgb(0x202020))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .bg(match alert.severity {
                            AlertSeverity::SUCCESS => &cx.style().alert.success,
                            AlertSeverity::INFO => &cx.style().alert.info,
                            AlertSeverity::WARNING => &cx.style().alert.warning,
                            AlertSeverity::ERROR => &cx.style().alert.error,
                        })
                        .p(px(5.0))
                        .rounded(cx.style().rounding.abs())
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .justify_between()
                                .when_else(
                                    alert.title.is_some(),
                                    |_self| _self.child(alert.title.unwrap()),
                                    |_self| {
                                        _self.child(match alert.severity {
                                            AlertSeverity::SUCCESS => "Success",
                                            AlertSeverity::INFO => "Info",
                                            AlertSeverity::WARNING => "Warning",
                                            AlertSeverity::ERROR => "Error",
                                        })
                                    },
                                )
                                .when(alert._type == AlertType::UserMustClose, |_self| {
                                    _self.child(
                                        IconButton::new("alert-close-button")
                                            .icon(Icons::Close)
                                            .icon_colour(Colour::Rgb(0x202020))
                                            .justify_content(ContentPosition::Centre)
                                            .align_text(ContentPosition::Centre)
                                            .w(Size::Px(22.0))
                                            .h(Size::Px(22.0))
                                            .icon_size(Size::Px(14.0))
                                            .mx(cx.style().margin)
                                            .colour(Colour::Rgba(0x00000000))
                                            .hover_colour(Colour::Rgba(0xffffff22))
                                            .rounding_all(Size::Px(100.0))
                                            .on_click(move |e, window, _cx| {
                                                _cx.alert_clear();
                                            })
                                            .render(window, cx),
                                    )
                                }),
                        )
                        .child(alert.message),
                )
        })
    }
}
