use gpui_kit::base::TreeItem;
use std::{fs, path::PathBuf};

#[derive(Clone)]
pub struct FileTreeNode {
    pub path: PathBuf,
    pub name: String,
    pub extension: Option<String>,
    pub children: Vec<FileTreeNode>,
    pub is_dir: bool,
    pub is_root: bool,
}

impl FileTreeNode {
    pub fn build(path: PathBuf) -> Option<Self> {
        Self::build_internal(path, true)
    }
    fn build_internal(path: PathBuf, is_root: bool) -> Option<Self> {
        let metadata = fs::metadata(&path).ok()?;
        let name = path.file_name()?.to_string_lossy().to_string();
        let is_dir = metadata.is_dir();
        let extension = if is_dir {
            None
        } else {
            path.extension()
                .and_then(|s| s.to_str())
                .map(|ext| ext.to_lowercase())
        };
        let mut children = Vec::new();
        if is_dir {
            if let Ok(entries) = fs::read_dir(&path) {
                let mut entries: Vec<_> = entries.filter_map(Result::ok).collect();

                entries.sort_by_key(|e| (!e.path().is_dir(), e.file_name()));

                children.extend(
                    entries
                        .into_iter()
                        .filter_map(|e| Self::build_internal(e.path(), false)),
                );
            }
        }
        Some(Self {
            path,
            name,
            extension,
            is_dir,
            children,
            is_root,
        })
    }
}

impl From<&FileTreeNode> for TreeItem {
    fn from(node: &FileTreeNode) -> Self {
        TreeItem::new(node.path.to_string_lossy(), &node.name)
            .expanded(node.is_root)
            .children(node.children.iter().map(|n| TreeItem::from(n)))
    }
}
