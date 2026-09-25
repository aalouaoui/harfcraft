use gpui_kit::{
    App, AppContext, Context, Entity, ParentElement, Render, Styled, Window, base::v_flex,
    component::button::Button, prelude::IntoElement,
};

use crate::{
    state::AppState,
    ui::{app_statusbar::AppStatusbar, app_titlebar::AppTitlebar},
};

pub struct AppRoot;

impl AppRoot {
    pub fn new(_window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|_| Self)
    }
}

impl Render for AppRoot {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = AppState::global(cx);
        v_flex()
            .size_full()
            .child(AppTitlebar)
            .child(if let Some(ws) = &state.workspace {
                v_flex()
                    .size_full()
                    .justify_center()
                    .items_center()
                    .gap_2()
                    .child(ws.root_path.display().to_string())
                    .child(
                        Button::new("close-workspace")
                            .child("Close Workspace")
                            .on_click(|_, _, cx| {
                                AppState::close_workspace(cx);
                            }),
                    )
            } else {
                v_flex()
                    .size_full()
                    .justify_center()
                    .items_center()
                    .gap_2()
                    .child("No open workspace")
                    .child(
                        Button::new("open-workspace")
                            .child("Open Workspace")
                            .on_click(|_, _, cx| {
                                AppState::open_workspace(cx);
                            }),
                    )
            })
            .child(AppStatusbar)
    }
}
