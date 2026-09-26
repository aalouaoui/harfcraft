use gpui_kit::{
    AppContext, SharedString, WindowOptions,
    component::{Root, Theme, ThemeRegistry, TitleBar},
};
use harfcraft::{APP_ID, APP_NAME, state::AppState, ui::app_root::AppRoot};

fn main() {
    gpui_kit::application()
        .with_assets(gpui_kit::assets::AllAssets)
        .run(move |cx| {
            gpui_kit::init(cx);

            let state = AppState::new();
            let theme_name = SharedString::from(&state.config.active_theme);
            if let Some(theme) = ThemeRegistry::global(cx).themes().get(&theme_name).cloned() {
                Theme::global_mut(cx).apply_config(&theme);
            }
            cx.set_global(state);

            cx.set_app_identity(APP_ID, APP_NAME);
            cx.activate(true);

            let window_options = WindowOptions {
                app_id: Some(APP_ID.into()),
                window_decorations: Some(gpui_kit::WindowDecorations::Client),
                ..TitleBar::window_options()
            };
            cx.spawn(async move |cx| {
                cx.open_window(window_options, |window, cx| {
                    let view = AppRoot::new(window, cx);
                    cx.new(|cx| Root::new(view, window, cx))
                })
                .expect("Failed to open window");
            })
            .detach();
        });
}
