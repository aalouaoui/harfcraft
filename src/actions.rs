use crate::state::AppState;
use gpui_kit::{App, actions};

actions!(harfcraft, [OpenProject, CloseProject]);

pub fn init(cx: &mut App) {
    cx.on_action(|_: &OpenProject, cx| {
        AppState::open_workspace_picker(cx);
    });
    cx.on_action(|_: &CloseProject, cx| {
        AppState::close_workspace(cx);
    });
}
