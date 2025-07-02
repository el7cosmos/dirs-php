# PHP Wrapper for dirs-rs

A PHP extension that provides access to platform-specific directory paths by wrapping the
Rust [dirs-rs](https://codeberg.org/dirs/dirs-rs) crate.

## Installation

You can install this extension using [Pie](https://github.com/php/pie):

```shell
pie install el7cosmos/dirs
```

## Building from Source

### Requirements

- PHP 8.0 or higher
- Rust toolchain

### Build

```shell
phpize
./configure
make
```

### Install

```shell
make install
```

## Functions

This extension provides the following functions to retrieve platform-specific directory paths:

### Base Directories

- `home_dir()`: Returns the path to the user's home directory
- `cache_dir()`: Returns the path to the user's cache directory
- `config_dir()`: Returns the path to the user's config directory
- `config_local_dir()`: Returns the path to the user's local config directory
- `data_dir()`: Returns the path to the user's data directory
- `data_local_dir()`: Returns the path to the user's local data directory
- `executable_dir()`: Returns the path to the user's executable directory
- `preference_dir()`: Returns the path to the user's preference directory
- `runtime_dir()`: Returns the path to the user's runtime directory
- `state_dir()`: Returns the path to the user's state directory

### User Directories

- `audio_dir()`: Returns the path to the user's audio directory
- `desktop_dir()`: Returns the path to the user's desktop directory
- `document_dir()`: Returns the path to the user's document directory
- `download_dir()`: Returns the path to the user's download directory
- `font_dir()`: Returns the path to the user's font directory
- `picture_dir()`: Returns the path to the user's picture directory
- `public_dir()`: Returns the path to the user's public directory
- `template_dir()`: Returns the path to the user's template directory
- `video_dir()`: Returns the path to the user's video directory

All functions return either a string containing the path or `null` if the directory could not be determined.

## Example Usage

```php
<?php
// Get user's home directory
$home = home_dir();
echo $home; // e.g., "/home/username"

// Get downloads directory
$downloads = download_dir();
echo $downloads; // e.g., "/home/username/Downloads"
```

## Platform Support

This extension supports the following platforms:

- Linux
- macOS

The behavior on each platform follows the [dirs-rs](https://codeberg.org/dirs/dirs-rs) implementation.
