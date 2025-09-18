use std::{fs, io, path::{Path, PathBuf}, time::{Duration, SystemTime}};

pub struct WatchedFile {
    path: PathBuf,
    modified: SystemTime,
}

impl WatchedFile {
    pub fn new(path: PathBuf) -> Option<WatchedFile> {
        let poll = Self::poll(&path)?;

        Some(WatchedFile {
            path,
            modified: poll,
        })
    }

    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    pub fn poll(path: &Path) -> Option<SystemTime> {
        path.metadata().ok()?.modified().ok()
    }

    pub fn is_modified(&self) -> Option<bool> {
        let poll = Self::poll(&self.path)?;

        Some(poll != self.modified)
    }

    pub fn accept_changes(&mut self) -> Option<()> {
        let poll = Self::poll(&self.path)?;
        self.modified = poll;

        Some(())
    }

    pub fn read(&self) -> Option<String> {
        fs::read_to_string(&self.path).ok()
    }
}
