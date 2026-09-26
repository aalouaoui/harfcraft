use crate::{
    state::AppState,
    ui::{app_titlebar::AppTitlebar, app_welcome::AppWelcome},
};
use gpui_kit::{
    App, AppContext, Context, Entity, ParentElement, Render, Styled, Window,
    base::v_flex,
    component::button::Button,
    prelude::{FluentBuilder, IntoElement},
};

pub struct AppRoot {
    welcome_screen: Entity<AppWelcome>,
}

impl AppRoot {
    pub fn new(_window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self {
            welcome_screen: AppWelcome::new(cx),
        })
    }
}

impl Render for AppRoot {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = AppState::global(cx);
        v_flex()
            .size_full()
            .child(AppTitlebar)
            .when_none(&state.workspace, |this| {
                this.child(self.welcome_screen.clone())
            })
            .when_some(state.workspace.as_ref(), |this, ws| {
                let workspace = ws.read(cx);
                this.child(
                    v_flex()
                        .size_full()
                        .justify_center()
                        .items_center()
                        .gap_2()
                        .child(workspace.root_path.display().to_string())
                        .child(
                            Button::new("close-workspace")
                                .child("Close Workspace")
                                .on_click(|_, _, cx| {
                                    AppState::close_workspace(cx);
                                }),
                        ),
                )
            })
    }
}
