use gpui_kit::{
    App, IntoElement, ParentElement, RenderOnce, Styled, Window,
    assets::IconName,
    base::h_flex,
    component::{
        ActiveTheme, Sizable, Theme, ThemeMode, TitleBar,
        button::{Button, ButtonVariants},
    },
};

use crate::{APP_NAME, state::AppState};

#[derive(IntoElement)]
pub struct AppTitlebar;

impl RenderOnce for AppTitlebar {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        TitleBar::new().child(
            h_flex()
                .justify_between()
                .items_center()
                .size_full()
                .child(
                    h_flex()
                        .gap_2()
                        .child(Button::new("menu").small().ghost().icon(IconName::Menu))
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
