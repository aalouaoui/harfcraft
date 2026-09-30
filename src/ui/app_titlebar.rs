use crate::{
    APP_NAME,
    actions::{CloseProject, OpenProject},
    state::AppState,
};
use gpui_kit::{
    Action, App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window,
    assets::IconName,
    base::h_flex,
    component::{
        ActiveTheme, Sizable, Theme, ThemeMode, TitleBar,
        button::{Button, ButtonVariants},
        menu::DropdownMenu,
    },
};

pub struct AppTitlebar;

impl AppTitlebar {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|_| AppTitlebar)
    }
}

impl Render for AppTitlebar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        TitleBar::new().child(
            h_flex()
                .justify_between()
                .items_center()
                .size_full()
                .child(
                    h_flex()
                        .gap_2()
                        .child(
                            Button::new("menu")
                                .small()
                                .ghost()
                                .icon(IconName::Menu)
                                .dropdown_menu(|menu, _, cx| {
                                    menu.menu("Open Project", OpenProject.boxed_clone())
                                        .menu_with_disabled(
                                            "Close Project",
                                            CloseProject.boxed_clone(),
                                            AppState::global(cx).workspace.is_none(),
                                        )
                                }),
                        )
                        .child(APP_NAME),
                )
                .child(
                    h_flex().child(
                        Button::new("menu")
                            .small()
                            .ghost()
                            .icon(IconName::Palette)
                            .on_click(|_, window, cx| {
                                Theme::change(
                                    if cx.theme().is_dark() {
                                        ThemeMode::Light
                                    } else {
                                        ThemeMode::Dark
                                    },
                                    Some(window),
                                    cx,
                                );
                                let theme = cx.theme().theme_name().to_string();
                                AppState::global_mut(cx).config.change_theme(theme);
                            }),
                    ),
                ),
        )
    }
}
