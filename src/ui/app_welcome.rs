use std::path;

use crate::{APP_DESCRIPTION, APP_NAME, state::AppState};
use gpui_kit::{
    App, AppContext, BorrowAppContext, Entity, FontWeight, ParentElement, Render, SharedString,
    Styled, Window,
    assets::IconName,
    base::{h_flex, v_flex},
    component::{
        ActiveTheme,
        button::{Button, ButtonVariants},
        label::Label,
    },
    prelude::{Context, IntoElement},
};

pub struct AppWelcome;

impl AppWelcome {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|_| Self)
    }
}

impl Render for AppWelcome {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let recent_projects = AppState::global(cx).config.recent_projects.clone();
        v_flex()
            .size_full()
            .justify_center()
            .items_center()
            .gap_4()
            .child(
                Label::new(APP_NAME)
                    .text_3xl()
                    .font_weight(FontWeight::BOLD),
            )
            .child(
                Label::new(APP_DESCRIPTION)
                    .text_sm()
                    .text_color(cx.theme().muted_foreground),
            )
            .child(
                v_flex().w_128().child(
                    Button::new("open-project")
                        .ghost()
                        .label("Open Project")
                        .icon(IconName::FolderOpen)
                        .on_click(|_, _, cx| {
                            AppState::open_workspace_picker(cx);
                        }),
                ),
            )
            .child(
                Label::new("RECENT PROJECTS")
                    .text_color(cx.theme().muted_foreground)
                    .text_xs(),
            )
            .child(
                v_flex()
                    .w_128()
                    .children(recent_projects.into_iter().map(|p| {
                        let parent = SharedString::from(p.parent().unwrap().to_string_lossy());
                        let name = SharedString::from(p.file_name().unwrap().to_string_lossy());
                        let path = SharedString::from(p.to_string_lossy());
                        Button::new(path.clone())
                            .ghost()
                            .child(
                                h_flex()
                                    .w_full()
                                    .justify_center()
                                    .child(
                                        Label::new(parent)
                                            .text_color(cx.theme().muted_foreground)
                                            .truncate(),
                                    )
                                    .child(
                                        Label::new(path::MAIN_SEPARATOR)
                                            .text_color(cx.theme().muted_foreground),
                                    )
                                    .child(Label::new(name)),
                            )
                            .on_click(move |_, _, cx| {
                                cx.update_global::<AppState, _>(|state, cx| {
                                    state.open_project(p.clone(), cx);
                                });
                            })
                    })),
            )
    }
}
