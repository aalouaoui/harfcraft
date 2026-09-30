use crate::{
    state::AppState,
    ui::{app_titlebar::AppTitlebar, app_welcome::AppWelcome, workspace_view::WorkspaceView},
};
use gpui_kit::{
    App, AppContext, Context, Entity, ParentElement, Render, Styled, Subscription, Window,
    base::v_flex,
    prelude::{FluentBuilder, IntoElement},
};

pub struct AppRoot {
    title_bar: Entity<AppTitlebar>,
    welcome_screen: Entity<AppWelcome>,
    workspace_view: Option<Entity<WorkspaceView>>,
    _subscriptions: Vec<Subscription>,
}

impl AppRoot {
    pub fn new(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self {
            title_bar: AppTitlebar::new(cx),
            welcome_screen: AppWelcome::new(cx),
            workspace_view: WorkspaceView::new(window, cx),
            _subscriptions: vec![cx.observe_global_in::<AppState>(
                &window,
                |this: &mut Self, window, cx| {
                    this.workspace_view = WorkspaceView::new(window, cx);
                },
            )],
        })
    }
}

impl Render for AppRoot {
    fn render(&mut self, _window: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .child(self.title_bar.clone())
            .when_none(&self.workspace_view, |this| {
                this.child(self.welcome_screen.clone())
            })
            .when_some(self.workspace_view.clone(), |this, view| this.child(view))
    }
}
