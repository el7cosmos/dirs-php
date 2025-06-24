#![cfg_attr(windows, feature(abi_vectorcall))]
use ext_php_rs::prelude::*;

/// Returns the path to the user's home directory.
#[php_function]
pub fn home_dir() -> Option<String> {
    dirs::home_dir()?.to_str().map(|path| path.to_string())
}

/// Returns the path to the user's cache directory.
#[php_function]
pub fn cache_dir() -> Option<String> {
    dirs::cache_dir()?.to_str().map(|path| path.to_string())
}

/// Returns the path to the user's config directory.
#[php_function]
pub fn config_dir() -> Option<String> {
    dirs::config_dir()?.to_str().map(|path| path.to_string())
}

/// Returns the path to the user's local config directory.
#[php_function]
pub fn config_local_dir() -> Option<String> {
    dirs::config_local_dir()?
        .to_str()
        .map(|path| path.to_string())
}

/// Returns the path to the user's data directory.
#[php_function]
pub fn data_dir() -> Option<String> {
    dirs::data_dir()?.to_str().map(|path| path.to_string())
}

/// Returns the path to the user's local data directory.
#[php_function]
pub fn data_local_dir() -> Option<String> {
    dirs::data_local_dir()?
        .to_str()
        .map(|path| path.to_string())
}

/// Returns the path to the user's executable directory.
#[php_function]
pub fn executable_dir() -> Option<String> {
    dirs::executable_dir()?
        .to_str()
        .map(|path| path.to_string())
}

/// Returns the path to the user's preference directory.
#[php_function]
pub fn preference_dir() -> Option<String> {
    dirs::preference_dir()?
        .to_str()
        .map(|path| path.to_string())
}

/// Returns the path to the user's runtime directory.
#[php_function]
pub fn runtime_dir() -> Option<String> {
    dirs::runtime_dir()?.to_str().map(|path| path.to_string())
}

/// Returns the path to the user's state directory.
#[php_function]
pub fn state_dir() -> Option<String> {
    dirs::state_dir()?.to_str().map(|path| path.to_string())
}

/// Returns the path to the user's audio directory.
#[php_function]
pub fn audio_dir() -> Option<String> {
    dirs::audio_dir()?.to_str().map(|path| path.to_string())
}

/// Returns the path to the user's desktop directory.
#[php_function]
pub fn desktop_dir() -> Option<String> {
    dirs::desktop_dir()?.to_str().map(|path| path.to_string())
}

/// Returns the path to the user's document directory.
#[php_function]
pub fn document_dir() -> Option<String> {
    dirs::document_dir()?.to_str().map(|path| path.to_string())
}

/// Returns the path to the user's download directory.
#[php_function]
pub fn download_dir() -> Option<String> {
    dirs::download_dir()?.to_str().map(|path| path.to_string())
}

/// Returns the path to the user's font directory.
#[php_function]
pub fn font_dir() -> Option<String> {
    dirs::font_dir()?.to_str().map(|path| path.to_string())
}

/// Returns the path to the user's picture directory.
#[php_function]
pub fn picture_dir() -> Option<String> {
    dirs::picture_dir()?.to_str().map(|path| path.to_string())
}

/// Returns the path to the user's public directory.
#[php_function]
pub fn public_dir() -> Option<String> {
    dirs::public_dir()?.to_str().map(|path| path.to_string())
}

/// Returns the path to the user's template directory.
#[php_function]
pub fn template_dir() -> Option<String> {
    dirs::template_dir()?.to_str().map(|path| path.to_string())
}

/// Returns the path to the user's video directory.
#[php_function]
pub fn video_dir() -> Option<String> {
    dirs::video_dir()?.to_str().map(|path| path.to_string())
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
