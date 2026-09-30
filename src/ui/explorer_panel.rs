use crate::state::Workspace;
use gpui_kit::{
    App, AppContext, Entity, EventEmitter, FocusHandle, Focusable, ParentElement, Render, Styled,
    Window,
    base::{TreeEntry, TreeItem, TreeState, dock::PanelEvent, h_flex},
    component::{Icon, IconName, list::ListItem, tree::tree},
    div,
    prelude::{Context, IntoElement},
    px,
};

pub struct ExplorerPanel {
    tree_state: Entity<TreeState>,
    focus_handle: FocusHandle,
}

impl ExplorerPanel {
    pub fn new(workspace: Entity<Workspace>, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let tree_state = cx.new(|cx| {
                let workspace = workspace.read(cx);
                let name = workspace.name.clone();
                TreeState::new(cx).items(vec![TreeItem::new(name.clone(), name.clone())])
            });
            Self {
                tree_state,
                focus_handle: cx.focus_handle(),
            }
        })
    }
}

impl EventEmitter<PanelEvent> for ExplorerPanel {}

impl Focusable for ExplorerPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl gpui_kit::base::dock::Panel for ExplorerPanel {
    fn panel_name(&self) -> &'static str {
        "Explorer"
    }
    fn closable(&self, _: &App) -> bool {
        false
    }
    fn zoomable(&self, _: &App) -> bool {
        false
    }
}

impl gpui_kit::component::dock::Panel for ExplorerPanel {
    fn title(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .gap_2()
            .child(Icon::new(IconName::Folder))
            .child("Files")
    }
}

impl Render for ExplorerPanel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(tree(&self.tree_state, render_item))
    }
}

fn render_item(
    ix: usize,
    entry: &TreeEntry,
    _selected: bool,
    _window: &mut Window,
    _cx: &mut App,
) -> ListItem {
    let item = entry.item();
    let icon = if !entry.is_folder() {
        Icon::new(IconName::File)
    } else if entry.is_expanded() {
        Icon::new(IconName::FolderOpen)
    } else {
        Icon::new(IconName::Folder)
    };
    ListItem::new(ix)
        .w_full()
        .pl(px(16.) * entry.depth() + px(12.0))
        .child(h_flex().gap_2().child(icon).child(item.label.clone()))
}
