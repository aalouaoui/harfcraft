use gpui_kit::{
    App, AppContext, Entity, EventEmitter, FocusHandle, Focusable, ParentElement, Render,
    SharedString, Styled, Window,
    base::{dock::PanelEvent, h_flex, input::EditorState},
    component::{
        IconName, Sizable,
        button::{Button, ButtonVariants},
        input::Editor,
    },
    prelude::{Context, IntoElement},
};

pub struct EditorPanel {
    name: SharedString,
    editor_state: Entity<EditorState>,
    focus_handle: FocusHandle,
}

impl EditorPanel {
    pub fn new(name: impl Into<SharedString>, window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let editor_state =
                cx.new(|cx| EditorState::new(window, cx).placeholder("Enter your text here"));
            Self {
                name: name.into(),
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
    fn closable(&self, _cx: &App) -> bool {
        false
    }
}

impl gpui_kit::component::dock::Panel for EditorPanel {
    fn inner_padding(&self, _cx: &App) -> bool {
        false
    }
    fn title(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .mr_neg_2()
            .gap_2()
            .child(self.name.clone())
            .child(Button::new("close").small().ghost().icon(IconName::Close))
    }
}

impl Render for EditorPanel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Editor::new(&self.editor_state).size_full()
    }
}
