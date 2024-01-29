use std::path::{Path, PathBuf};
use std::{fs, io};

#[derive(Default, Clone, PartialEq, Eq)]
pub struct DirectoryEntry {
    path: PathBuf,
    is_directory: bool,
}

impl DirectoryEntry {
    pub fn from_path(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
            is_directory: path.is_dir(),
        }
    }

    pub fn is_dir(&self) -> bool {
        self.is_directory
    }

    pub fn is_file(&self) -> bool {
        !self.is_directory
    }

    pub fn to_path_buf(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn file_name(&self) -> &str {
        self.path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default()
    }
}

#[derive(Default)]
pub struct DirectoryContent {
    content: Vec<DirectoryEntry>,
}

impl DirectoryContent {
    pub fn new() -> Self {
        Self { content: vec![] }
    }

    pub fn from_path(path: &Path, include_files: bool) -> io::Result<Self> {
        Ok(Self {
            content: load_directory(path, include_files)?,
        })
    }

    pub fn iter(&self) -> std::slice::Iter<'_, DirectoryEntry> {
        self.content.iter()
    }

    pub fn push(&mut self, item: DirectoryEntry) {
        self.content.push(item);
    }
}

fn load_directory(path: &Path, include_files: bool) -> io::Result<Vec<DirectoryEntry>> {
    let paths = fs::read_dir(path)?;

    let mut result: Vec<DirectoryEntry> = Vec::new();
    for path in paths {
        match path {
            Ok(entry) => {
                if !include_files && entry.path().is_file() {
                    continue;
                }
                result.push(DirectoryEntry::from_path(entry.path().as_path()))
            }
            Err(_) => continue,
        };
    }

    // TODO: Sort content to display folders first
    // TODO: Implement "Show hidden files and folders" option

    Ok(result)
}
