use crate::state::file_tree::FileTreeNode;
use gpui_kit::{App, AppContext, Entity, SharedString};
use std::path::PathBuf;

pub struct Workspace {
    pub name: SharedString,
    pub root_path: PathBuf,
    pub file_tree: Entity<FileTreeNode>,
}

impl Workspace {
    pub fn new(root_path: PathBuf, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            // TODO: Handle file tree error instead of panicking
            let file_tree =
                FileTreeNode::build(root_path.clone()).expect("Failed to open workspace");
            let name = file_tree.name.clone().into();
            Self {
                root_path,
                name,
                file_tree: cx.new(|_| file_tree),
            }
        })
    }
}
