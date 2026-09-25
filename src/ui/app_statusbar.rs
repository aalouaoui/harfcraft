use gpui_kit::{
    App, IntoElement, ParentElement, RenderOnce, Window, component::status_bar::StatusBar,
};

#[derive(IntoElement)]
pub struct AppStatusbar;

impl RenderOnce for AppStatusbar {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        StatusBar::new().child("Status bar")
    }
}
