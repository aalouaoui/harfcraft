use gpui_kit::{
    App, AppContext, Entity, EventEmitter, FocusHandle, Focusable, Render, Styled, Window,
    base::{dock::PanelEvent, input::EditorState},
    component::input::Editor,
    prelude::{Context, IntoElement},
};

pub struct EditorPanel {
    editor_state: Entity<EditorState>,
    focus_handle: FocusHandle,
}

impl EditorPanel {
    pub fn new(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let editor_state =
                cx.new(|cx| EditorState::new(window, cx).placeholder("Enter your text here"));
            Self {
                editor_state,
                focus_handle: cx.focus_handle(),
            }
        })
    }
}

impl EventEmitter<PanelEvent> for EditorPanel {}

impl Focusable for EditorPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl gpui_kit::base::dock::Panel for EditorPanel {
    fn panel_name(&self) -> &'static str {
        "Editor"
    }
}

impl gpui_kit::component::dock::Panel for EditorPanel {}

impl Render for EditorPanel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Editor::new(&self.editor_state).size_full()
    }
}
