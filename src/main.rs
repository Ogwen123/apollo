mod components;
mod events;
mod state;
mod style;
mod utils;
mod widgets;

use crate::components::status_bar::StatusBar;
use crate::components::test_list::TestList;
use crate::components::toolbar::ToolBar;
use crate::components::workspace::Workspace;
use crate::state::{Alert, AlertSeverity, AlertType, State, StateProvider};
use crate::style::{GlobalStyle, Style, StyleProvider};
use crate::utils::file::{load_state, save_state};
use crate::widgets::core::modal::Modal;
use gpui::{
    App, Application, AsyncApp, Bounds, Context, SharedString, Task, TitlebarOptions, Window,
    WindowBounds, WindowOptions, div, prelude::*, px, size,
};
use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use crate::utils::assets::Assets;

trait ModalHelper {
    fn open_modal(&mut self, cx: &mut App, modal: Modal);
    fn close_modal(&mut self, cx: &mut App);
}

impl ModalHelper for Window {
    fn open_modal(&mut self, cx: &mut App, modal: Modal) {
        let root = self
            .root::<Base>()
            .flatten()
            .expect("Window root should be type Base");
        root.update(cx, |base, cx| {
            base.modals.push(modal);
        });
        self.refresh()
    }

    fn close_modal(&mut self, cx: &mut App) {
        let root = self
            .root::<Base>()
            .flatten()
            .expect("Window root should be type Base");
        root.update(cx, |base, cx| {
            base.modals.pop();
        });
        self.refresh()
    }
}

trait AlertHandler {
    fn alert_success<T: ToString>(&mut self, message: T, time: Option<f64>);

    fn alert_info<T: ToString>(&mut self, message: T, time: Option<f64>);

    fn alert_warning<T: ToString>(&mut self, message: T, time: Option<f64>);

    fn alert_error<T: ToString>(&mut self, message: T, time: Option<f64>);

    fn alert_clear(&mut self);
}

impl AlertHandler for App {
    fn alert_success<T: ToString>(&mut self, message: T, time: Option<f64>) {
        self.update_global::<State, ()>(|global, _| {
            global.alert = Some(Alert {
                string: message.to_string(),
                severity: AlertSeverity::SUCCESS,
                _type: match time {
                    Some(res) => AlertType::Timed(res),
                    None => AlertType::UserMustClose,
                },
            });
        })
    }

    fn alert_info<T: ToString>(&mut self, message: T, time: Option<f64>) {
        self.update_global::<State, ()>(|global, _| {
            global.alert = Some(Alert {
                string: message.to_string(),
                severity: AlertSeverity::INFO,
                _type: match time {
                    Some(res) => AlertType::Timed(res),
                    None => AlertType::UserMustClose,
                },
            });
        })
    }

    fn alert_warning<T: ToString>(&mut self, message: T, time: Option<f64>) {
        self.update_global::<State, ()>(|global, _| {
            global.alert = Some(Alert {
                string: message.to_string(),
                severity: AlertSeverity::WARNING,
                _type: match time {
                    Some(res) => AlertType::Timed(res),
                    None => AlertType::UserMustClose,
                },
            });
        })
    }

    fn alert_error<T: ToString>(&mut self, message: T, time: Option<f64>) {
        self.update_global::<State, ()>(|global, _| {
            global.alert = Some(Alert {
                string: message.to_string(),
                severity: AlertSeverity::ERROR,
                _type: match time {
                    Some(res) => AlertType::Timed(res),
                    None => AlertType::UserMustClose,
                },
            });
        })
    }

    fn alert_clear(&mut self) {
        self.update_global::<State, ()>(|global, _| {
            global.alert = None;
        })
    }
}

trait AsyncAlertHandler {
    fn alert_success<T: ToString>(&self, message: T, time: Option<f64>);

    fn alert_info<T: ToString>(&self, message: T, time: Option<f64>);

    fn alert_warning<T: ToString>(&self, message: T, time: Option<f64>);

    fn alert_error<T: ToString>(&self, message: T, time: Option<f64>);

    fn alert_clear(&self);
}

impl AsyncAlertHandler for AsyncApp {
    fn alert_success<T: ToString>(&self, message: T, time: Option<f64>) {
        let _ = self.update_global::<State, ()>(|global, _| {
            global.alert = Some(Alert {
                string: message.to_string(),
                severity: AlertSeverity::SUCCESS,
                _type: match time {
                    Some(res) => AlertType::Timed(res),
                    None => AlertType::UserMustClose,
                },
            });
        });
    }

    fn alert_info<T: ToString>(&self, message: T, time: Option<f64>) {
        let _ = self.update_global::<State, ()>(|global, _| {
            global.alert = Some(Alert {
                string: message.to_string(),
                severity: AlertSeverity::INFO,
                _type: match time {
                    Some(res) => AlertType::Timed(res),
                    None => AlertType::UserMustClose,
                },
            });
        });
    }

    fn alert_warning<T: ToString>(&self, message: T, time: Option<f64>) {
        let _ = self.update_global::<State, ()>(|global, _| {
            global.alert = Some(Alert {
                string: message.to_string(),
                severity: AlertSeverity::WARNING,
                _type: match time {
                    Some(res) => AlertType::Timed(res),
                    None => AlertType::UserMustClose,
                },
            });
        });
    }

    fn alert_error<T: ToString>(&self, message: T, time: Option<f64>) {
        let _ = self.update_global::<State, ()>(|global, _| {
            global.alert = Some(Alert {
                string: message.to_string(),
                severity: AlertSeverity::ERROR,
                _type: match time {
                    Some(res) => AlertType::Timed(res),
                    None => AlertType::UserMustClose,
                },
            });
        });
    }

    fn alert_clear(&self) {
        let _ = self.update_global::<State, ()>(|global, _| {
            global.alert = None;
        });
    }
}

struct Base {
    modals: Vec<Modal>,
}

impl Render for Base {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(&cx.style().bg_colour)
            .items_center()
            .text_color(&cx.style().text_colour)
            .child(cx.new(|_| ToolBar {}))
            .child(cx.new(|_| Workspace {}))
            .child(cx.new(|_| StatusBar {}))
            // Modals
            .children(self.modals.iter().map(|x| {
                cx.new(|_| Modal {
                    title: x.title.clone(),
                    body: x.body.clone(),
                    width: x.width,
                    height: x.height,
                    padding: x.padding,
                    rounding: x.rounding,
                    bg_colour: x.bg_colour.clone(),
                    accept_button: x.accept_button.clone(),
                    cancel_button: x.cancel_button.clone(),
                    top_offset: x.top_offset,
                    on_close: if x.on_close.is_none() {
                        None
                    } else {
                        Some(x.on_close.clone().unwrap())
                    },
                    backdrop_close: x.backdrop_close,
                })
            }))
    }
}

fn main() {
    Application::new().with_assets(Assets {
        base: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets"),
    }).run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1000.), px(800.0)), cx);

        let session_type = env::var("XDG_SESSION_TYPE").unwrap_or_default();
        let wayland_display = env::var("WAYLAND_DISPLAY").ok().is_some();
        let x_display = env::var("DISPLAY").ok().is_some();
        let os = env::consts::OS;

        let mut csd: bool = false;

        if os == "linux" {
            if session_type == "wayland" || wayland_display {
                // wayland doesn't handle SSD
                csd = true
            } else if session_type == "x11" || x_display {
                // SSD handled by x11
                csd = true
            } else {
                // Unknown environment, defaulting to no CSD
                csd = false
            }
        }

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: Some(TitlebarOptions {
                title: Some(SharedString::new("Apollo")),
                appears_transparent: csd,
                traffic_light_position: None,
            }),
            ..Default::default()
        };

        // load previous state from file
        let mut state = load_state();
        state.csd = csd;
        cx.set_global(state);
        cx.set_global(GlobalStyle(Arc::new(Style::default())));

        let _ = cx
            .on_app_quit(|_cx| {
                // save state to a file
                save_state(_cx.state().clone());
                return Task::ready(());
            })
            .detach();

        cx.open_window(window_options, |_, cx| {
            cx.new(|_cx| Base { modals: Vec::new() })
        })
        .unwrap();

        cx.activate(true);
    });
}
