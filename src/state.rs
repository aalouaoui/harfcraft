use gpui_kit::{App, BorrowAppContext, Global, PathPromptOptions, SharedString};
use std::path::PathBuf;

pub struct Workspace {
    pub name: SharedString,
    pub root_path: PathBuf,
}

impl Workspace {
    pub fn new(root_path: PathBuf) -> Self {
        let name = root_path.file_name().unwrap().to_string_lossy().into();
        Self { root_path, name }
    }
}

pub struct AppState {
    pub workspace: Option<Workspace>,
}

impl Global for AppState {}

impl AppState {
    pub fn new() -> Self {
        Self { workspace: None }
    }

    pub fn global(cx: &App) -> &Self {
        cx.global()
    }

    pub fn global_mut(cx: &mut App) -> &mut Self {
        cx.global_mut()
    }

    pub fn open_workspace(cx: &mut App) {
        let prompt = cx.prompt_for_paths(PathPromptOptions {
            files: false,
            directories: true,
            multiple: false,
            prompt: Some("Open UFO".into()),
        });
        cx.spawn(async move |app| {
            let root_path = prompt.await.ok()?.ok()??.first()?.clone();
            let workspace = Workspace::new(root_path);
            app.update_global::<AppState, _>(|state, _| {
                state.workspace = Some(workspace);
            });
            Some(())
        })
        .detach();
    }

    pub fn close_workspace(cx: &mut App) {
        cx.update_global::<AppState, _>(|state, _| {
            state.workspace = None;
        });
    }
}
