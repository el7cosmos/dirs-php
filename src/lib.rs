#![cfg_attr(windows, feature(abi_vectorcall))]

use ext_php_rs::convert::IntoZval;
use ext_php_rs::flags::DataType;
use ext_php_rs::prelude::*;
use ext_php_rs::types::Zval;
use std::path::PathBuf;

#[derive(Debug)]
pub struct PhpPath(PathBuf);

impl From<PathBuf> for PhpPath {
    fn from(path: PathBuf) -> Self {
        PhpPath(path)
    }
}

impl IntoZval for PhpPath {
    const TYPE: DataType = DataType::String;

    fn set_zval(self, zv: &mut Zval, persistent: bool) -> ext_php_rs::error::Result<()> {
        self.0
            .to_str()
            .map_or(Err(ext_php_rs::error::Error::ZvalConversion(DataType::String)), |str| {
                zv.set_string(str, persistent)
            })
    }
}

/// Macro to generate directory functions
macro_rules! dirs_fn {
    ($fn_name:ident, $doc:expr) => {
        #[doc = $doc]
        #[php_function]
        pub fn $fn_name() -> Option<PhpPath> {
            dirs::$fn_name().map(PhpPath)
        }
    };
}

// Generate all directory functions
dirs_fn!(home_dir, "Returns the path to the user's home directory.");
dirs_fn!(cache_dir, "Returns the path to the user's cache directory.");
dirs_fn!(config_dir, "Returns the path to the user's config directory.");
dirs_fn!(config_local_dir, "Returns the path to the user's local config directory.");
dirs_fn!(data_dir, "Returns the path to the user's data directory.");
dirs_fn!(data_local_dir, "Returns the path to the user's local data directory.");
dirs_fn!(executable_dir, "Returns the path to the user's executable directory.");
dirs_fn!(preference_dir, "Returns the path to the user's preference directory.");
dirs_fn!(runtime_dir, "Returns the path to the user's runtime directory.");
dirs_fn!(state_dir, "Returns the path to the user's state directory.");
dirs_fn!(audio_dir, "Returns the path to the user's audio directory.");
dirs_fn!(desktop_dir, "Returns the path to the user's desktop directory.");
dirs_fn!(document_dir, "Returns the path to the user's document directory.");
dirs_fn!(download_dir, "Returns the path to the user's download directory.");
dirs_fn!(font_dir, "Returns the path to the user's font directory.");
dirs_fn!(picture_dir, "Returns the path to the user's picture directory.");
dirs_fn!(public_dir, "Returns the path to the user's public directory.");
dirs_fn!(template_dir, "Returns the path to the user's template directory.");
dirs_fn!(video_dir, "Returns the path to the user's video directory.");

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
