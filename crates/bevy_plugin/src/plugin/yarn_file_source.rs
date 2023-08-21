use crate::prelude::*;
use bevy::{prelude::*, reflect::TypePath};
use std::path::PathBuf;

/// Possible sources to load a [`YarnFile`] from.
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypePath)]
pub enum YarnFileSource {
    /// A [`YarnFile`] that is already present in the asset server, addressed by its [`Handle`].
    Handle(Handle<YarnFile>),
    /// A [`YarnFile`] that is already present in memory, created with [`YarnFile::new`].
    InMemory(YarnFile),
    /// A [`YarnFile`] inside the `assets` folder. This will be loaded into the [`AssetServer`].
    /// Use [`YarnFileSource::file`] for convenience.
    File(PathBuf),
    /// A folder inside the `assets` folder which is searched for [`YarnFile`]s recursively, loading all files with the `.yarn` extension into the [`AssetServer`].
    /// Use [`YarnFileSource::folder`] for convenience.
    ///
    /// Not supported on Wasm and Android because Bevy cannot load folders on these platforms.
    Folder(PathBuf),
}

impl From<Handle<YarnFile>> for YarnFileSource {
    fn from(handle: Handle<YarnFile>) -> Self {
        Self::Handle(handle)
    }
}

impl From<YarnFile> for YarnFileSource {
    fn from(yarn_file: YarnFile) -> Self {
        Self::InMemory(yarn_file)
    }
}

impl YarnFileSource {
    /// Convenience function to create a [`YarnFileSource::File`] from a path.
    pub fn file(path: impl Into<PathBuf>) -> Self {
        Self::File(path.into())
    }

    /// Convenience function to create a [`YarnFileSource::folder`] from a path.
    /// Panics on Wasm and Android because Bevy cannot load folders on these platforms.
    pub fn folder(path: impl Into<PathBuf>) -> Self {
        #[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
        {
            Self::Folder(path.into())
        }
        #[cfg(any(target_arch = "wasm32", target_os = "android"))]
        {
            let _ = path;
            panic!("YarnFileSource::folder is not supported on this platform. Help: Use YarnFileSource::file instead and specify all Yarn files you want to load.")
        }
    }

    pub(crate) fn load(
        &self,
        asset_server: &AssetServer,
        assets: &mut ResMut<Assets<YarnFile>>,
    ) -> Vec<Handle<YarnFile>> {
        match self {
            Self::Handle(handle) => vec![handle.clone()],
            Self::InMemory(yarn_file) => vec![assets.add(yarn_file.clone())],
            Self::File(path) => vec![asset_server.load(path.as_path())],
            Self::Folder(path) => {
                #[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
                {
                    Self::load_folder(asset_server, path)
                }
                #[cfg(any(target_arch = "wasm32", target_os = "android"))]
                {
                    let _ = path;
                    panic!("YarnFileSource::Folder is not supported on this platform. Help: Use YarnFileSource::File instead and specify all Yarn files you want to load.")
                }
            }
        }
    }

    #[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
    fn load_folder(asset_server: &AssetServer, path: &std::path::Path) -> Vec<Handle<YarnFile>> {
        let handles: Vec<_> = asset_server
            .load_folder(path)
            .unwrap_or_else(|e| {
                panic!(
                    "Failed to load Yarn file folder {path}: {e}.\nHelp: Does the folder exist under the assets directory?",
                    path = path.display()
                )
            })
            .into_iter()
            .filter_map(|handle| {
                (asset_server
                    .get_handle_path(&handle)?
                    .path()
                    .extension()?
                    .to_str()?
                    == "yarn")
                    .then(|| handle.typed())
            })
            .collect();
        if handles.is_empty() {
            warn!("No Yarn files found in the assets subdirectory {path}, so Yarn Slinger won't be able to do anything this run. \
                        Help: Add some Yarn files to get started.", path = path.display());
        }
        handles
    }
}
