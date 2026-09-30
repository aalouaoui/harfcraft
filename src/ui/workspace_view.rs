use crate::{
    state::AppState,
    ui::{editor_panel::EditorPanel, explorer_panel::ExplorerPanel},
};
use gpui_kit::{
    App, AppContext, Entity, ParentElement, Render, Styled, Window,
    base::dock::{DockArea, DockLayout, DockPlacement},
    component::dock::{DockSkin, PanelStyle},
    div,
    prelude::{Context, IntoElement},
};
use std::rc::Rc;

pub struct WorkspaceView {
    dock_area: Entity<DockArea>,
    _skin: Rc<DockSkin>,
}

impl WorkspaceView {
    pub fn new(window: &mut Window, cx: &mut App) -> Option<Entity<Self>> {
        AppState::global(cx).workspace.clone().map(|workspace| {
            let explorer = ExplorerPanel::new(workspace.clone(), cx);
            let editor = EditorPanel::new(window, cx);
            cx.new(|cx| {
                let (dock_area, skin) = DockSkin::dock_area("workspace-view", None, window, cx);
                dock_area.update(cx, |area, cx| {
                    area.set_center(DockLayout::tabs().panel(editor), window, cx);
                    area.set_dock(
                        DockPlacement::Left,
                        DockLayout::tabs().panel(explorer),
                        window,
                        cx,
                    );
                });
                skin.set_toggle_button_visible(true, cx);
                skin.set_panel_style(PanelStyle::TabBar, cx);
                Self {
                    dock_area,
                    _skin: skin,
                }
            })
        })
    }
}

impl Render for WorkspaceView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(self.dock_area.clone())
    }
}
