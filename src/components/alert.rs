use crate::AlertHandler;
use crate::state::{AlertSeverity, AlertType, State, StateProvider};
use crate::style::{Colour, Size, StyleProvider};
use crate::widgets::core::button::button::ContentPosition;
use crate::widgets::core::button::icon_button::IconButton;
use crate::widgets::core::icon::Icons;
use gpui::prelude::FluentBuilder;
use gpui::{
    App, BorrowAppContext, IntoElement, ParentElement, RenderOnce, Styled, Window, div, rgb,
};

pub struct AlertDisplay {}

impl RenderOnce for AlertDisplay {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let alert_option = cx.state().alert.clone();

        div().when_some(alert_option, |_self, alert| {
            _self
                .flex()
                .flex_row()
                .bg(match alert.severity {
                    AlertSeverity::SUCCESS => &cx.style().alert.success,
                    AlertSeverity::INFO => &cx.style().alert.info,
                    AlertSeverity::WARNING => &cx.style().alert.warning,
                    AlertSeverity::ERROR => &cx.style().alert.error,
                })
                .child(div()
                    .flex_col()
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
                    .child(alert.message)
                )
                .when(alert._type == AlertType::UserMustClose, |_self| {
                    _self.child(
                        IconButton::new("alert-close-button")
                            .icon(Icons::Close)
                            .icon_colour(&cx.style().text_colour)
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
                })
        })
    }
}
