//! Portable vault path handling and relative-path enforcement.

use std::ffi::OsStr;
use std::fmt::{self, Display, Formatter};
use std::path::{Component, Path, PathBuf};

use crate::error::{Result, VaultError};

const SYSTEM_DIR: &str = ".mediavault";
const INBOX_DIR: &str = "Inbox";
const REVIEW_QUEUE_DIR: &str = "_review_queue";
const COVERS_DIR: &str = "covers";
const THUMBNAILS_DIR: &str = "thumbnails";
const ASSETS_DIR: &str = "assets";
const PROGRESS_DIR: &str = "progress";

/// A normalized path that is guaranteed to stay inside the vault.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RelativePath {
    inner: PathBuf,
}

impl RelativePath {
    /// Creates a new normalized relative path.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        if path.as_os_str().is_empty() {
            return Err(VaultError::InvalidRelativePath("path is empty".to_string()));
        }

        let mut normalized = PathBuf::new();

        for component in path.components() {
            match component {
                Component::Normal(part) => normalized.push(part),
                Component::CurDir => {}
                Component::ParentDir => {
                    return Err(VaultError::InvalidRelativePath(
                        "parent directory segments are not allowed".to_string(),
                    ));
                }
                Component::RootDir | Component::Prefix(_) => {
                    return Err(VaultError::InvalidRelativePath(
                        "absolute paths are not allowed".to_string(),
                    ));
                }
            }
        }

        if normalized.as_os_str().is_empty() {
            return Err(VaultError::InvalidRelativePath(
                "path normalizes to an empty value".to_string(),
            ));
        }

        Ok(Self { inner: normalized })
    }

    /// Returns the underlying relative path.
    pub fn as_path(&self) -> &Path {
        &self.inner
    }

    /// Returns the relative path as an owned `PathBuf`.
    pub fn to_path_buf(&self) -> PathBuf {
        self.inner.clone()
    }

    /// Joins a child component and keeps the result relative.
    pub fn join<P: AsRef<Path>>(&self, child: P) -> Result<Self> {
        Self::new(self.inner.join(child))
    }

    /// Returns the file name if the path points to a file.
    pub fn file_name(&self) -> Option<&OsStr> {
        self.inner.file_name()
    }

    /// Returns the file stem if the path points to a file.
    pub fn file_stem(&self) -> Option<&OsStr> {
        self.inner.file_stem()
    }

    /// Returns the extension if the path points to a file.
    pub fn extension(&self) -> Option<&OsStr> {
        self.inner.extension()
    }
}

impl Display for RelativePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner.display())
    }
}

impl AsRef<Path> for RelativePath {
    fn as_ref(&self) -> &Path {
        self.as_path()
    }
}

/// Portable vault root and path resolution helpers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vault {
    root: PathBuf,
}

impl Vault {
    /// Creates a new vault wrapper for the given root directory.
    pub fn new<P: Into<PathBuf>>(root: P) -> Result<Self> {
        let root = root.into();

        if root.as_os_str().is_empty() {
            return Err(VaultError::InvalidVaultPath(
                "vault root is empty".to_string(),
            ));
        }

        Ok(Self { root })
    }

    /// Returns the configured vault root path.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Returns the system directory used by MediaVault.
    pub fn system_dir(&self) -> PathBuf {
        self.root.join(SYSTEM_DIR)
    }

    /// Returns the inbox directory inside the vault.
    pub fn inbox_dir(&self) -> PathBuf {
        self.root.join(INBOX_DIR)
    }

    /// Returns the review queue directory inside the vault.
    pub fn review_queue_dir(&self) -> PathBuf {
        self.root.join(REVIEW_QUEUE_DIR)
    }

    /// Returns the cover storage directory inside the vault.
    pub fn covers_dir(&self) -> PathBuf {
        self.system_dir().join(COVERS_DIR)
    }

    /// Returns the thumbnail cache directory (app-internal, 256 px grid cache).
    pub fn thumbnails_dir(&self) -> PathBuf {
        self.system_dir().join(THUMBNAILS_DIR)
    }

    /// Returns the per-media assets directory root (backdrops, crew photos, banners).
    pub fn assets_dir(&self) -> PathBuf {
        self.system_dir().join(ASSETS_DIR)
    }

    /// Returns the assets directory for a specific media item.
    pub fn asset_dir_for(&self, media_id: &str) -> PathBuf {
        self.assets_dir().join(media_id)
    }

    /// Returns the progress store directory for resume data.
    pub fn progress_dir(&self) -> PathBuf {
        self.system_dir().join(PROGRESS_DIR)
    }

    /// Resolves an absolute path into a vault-relative path.
    pub fn relative_from_absolute<P: AsRef<Path>>(&self, absolute: P) -> Result<RelativePath> {
        let absolute = absolute.as_ref();

        if !absolute.starts_with(&self.root) {
            return Err(VaultError::InvalidRelativePath(
                "path does not live inside this vault".to_string(),
            ));
        }

        let relative = absolute.strip_prefix(&self.root).map_err(|error| {
            VaultError::InvalidRelativePath(format!("could not strip vault root: {error}"))
        })?;

        RelativePath::new(relative)
    }

    /// Resolves a vault-relative path back to an absolute path.
    pub fn resolve<P: AsRef<Path>>(&self, relative: P) -> Result<PathBuf> {
        let relative = RelativePath::new(relative)?;
        Ok(self.root.join(relative.as_path()))
    }
}

/// Returns the reserved MediaVault system directory name.
pub fn system_dir_name() -> &'static str {
    SYSTEM_DIR
}

/// Returns the reserved inbox directory name.
pub fn inbox_dir_name() -> &'static str {
    INBOX_DIR
}

/// Returns the reserved review queue directory name.
pub fn review_queue_dir_name() -> &'static str {
    REVIEW_QUEUE_DIR
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_parent_directory_segments() {
        let error = RelativePath::new("../escape").expect_err("parent segments must be rejected");
        assert!(matches!(error, VaultError::InvalidRelativePath(_)));
    }

    #[test]
    fn normalizes_relative_paths() {
        let relative =
            RelativePath::new("Inbox/./movie.mkv").expect("relative path should be valid");
        assert_eq!(relative.to_string(), "Inbox/movie.mkv");
    }

    #[test]
    fn resolves_absolute_paths_inside_vault() {
        let vault = Vault::new("/vault").expect("vault root should be valid");
        let relative = vault
            .relative_from_absolute("/vault/Anime/Violet Evergarden.mkv")
            .expect("path should be inside vault");
        assert_eq!(relative.to_string(), "Anime/Violet Evergarden.mkv");
    }
}
