//! Filesystem abstraction layer for virtual mounting
//!
//! Provides FUSE support on Linux/macOS and Dokan support on Windows
//! for mounting encrypted vaults as virtual filesystems.
//!
//! Note: FUSE/Dokan support is currently a placeholder and not yet implemented.
//! Enable the "fuse" feature to include FUSE dependencies.

use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FsError {
    #[error("Mount failed: {0}")]
    MountFailed(String),
    #[error("Unmount failed: {0}")]
    UnmountFailed(String),
    #[error("Not supported on this platform")]
    NotSupported,
}

pub struct MountOptions {
    pub mount_point: PathBuf,
    pub read_only: bool,
    pub allow_other: bool,
}

#[cfg(all(any(target_os = "linux", target_os = "macos"), feature = "fuse"))]
pub mod fuse {
    use super::*;

    pub fn mount(_options: MountOptions) -> Result<(), FsError> {
        // FUSE implementation would go here
        // This is a placeholder for the full implementation
        Err(FsError::NotSupported)
    }

    pub fn unmount(_mount_point: &PathBuf) -> Result<(), FsError> {
        Err(FsError::NotSupported)
    }
}

#[cfg(not(all(any(target_os = "linux", target_os = "macos"), feature = "fuse")))]
pub mod fuse {
    use super::*;

    pub fn mount(_options: MountOptions) -> Result<(), FsError> {
        Err(FsError::NotSupported)
    }

    pub fn unmount(_mount_point: &PathBuf) -> Result<(), FsError> {
        Err(FsError::NotSupported)
    }
}

#[cfg(target_os = "windows")]
pub mod dokan {
    use super::*;

    pub fn mount(_options: MountOptions) -> Result<(), FsError> {
        // Dokan implementation would go here
        Err(FsError::NotSupported)
    }

    pub fn unmount(_mount_point: &PathBuf) -> Result<(), FsError> {
        Err(FsError::NotSupported)
    }
}

// Re-export platform-specific modules
#[cfg(any(target_os = "linux", target_os = "macos"))]
pub use fuse as platform;

#[cfg(target_os = "windows")]
pub use dokan as platform;
