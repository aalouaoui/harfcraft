use crate::state::workspace::Workspace;
use gpui_kit::{
    App, AppContext, Entity, EventEmitter, FocusHandle, Focusable, ParentElement, Render, Styled,
    Window,
    base::{TreeEntry, TreeItem, TreeState, dock::PanelEvent, h_flex},
    component::{ActiveTheme, Icon, IconName, list::ListItem, tree::tree},
    div,
    prelude::{Context, IntoElement},
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
                let tree_item = TreeItem::from(workspace.file_tree.read(cx));
                TreeState::new(cx).items(vec![tree_item])
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
        Icon::new(IconName::Folder)
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
    cx: &mut App,
) -> ListItem {
    let item = entry.item();
    let icon = if !entry.is_folder() {
        Icon::new(IconName::File)
    } else if entry.is_expanded() {
        Icon::new(IconName::FolderOpen)
    } else {
        Icon::new(IconName::Folder)
    };
    ListItem::new(ix).w_full().p_0().h_8().child(
        h_flex()
            .child(h_flex().children((0..entry.depth()).into_iter().map(|_| {
                div()
                    .h_8()
                    .w_4()
                    .border_color(cx.theme().secondary)
                    .border_r_1()
                    .child("")
            })))
            .child(
                h_flex()
                    .pl_2()
                    .gap_2()
                    .child(icon)
                    .child(item.label.clone()),
            ),
    )
}
