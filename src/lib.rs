//! # `maex` library crate
//!
//! If you are reading this, you are reading the documentation for the `maex` library crate. For the cli, kindly refer to the README file.
//!
//! ## Assets Folder
//!
//! The `assets` folder, usually located directly under `.minecraft`, consists of 3 folders:
//!
//! - `index`: This folder contains several JSON files for indexing the assets, each for a specific game version. Each JSON file contains a single key `objects` at the top level, which is a map of asset paths to their corresponding hash and size.
//! - `objects`: This folder contains the actual assets, which are stored in a hashed format. Specifically, denote the hash as `hash`, the asset will be stored in `objects/hash[0..2]/hash`.
//! - `skins`: Structured like `objects`, containing skin images, but I don't know where their indices are stored.

#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

use serde::Deserialize;
use serde_json::{Result, from_reader};
use std::{
    collections::HashMap,
    path::PathBuf,
};

/// An entry in the `objects` field.
#[derive(Debug, Deserialize)]
struct Entry {
    hash: String,
    #[allow(dead_code, reason = "we don't need it")]
    size: u64,
}

/// The JSON data.
#[derive(Debug, Deserialize)]
struct Data {
    objects: HashMap<String, Entry>,
}

/// Reads the index (as a reader), returns a map of hashed paths to meaningful paths. Note that the hashed paths are relative to base path `.minecraft/assets/objects/`.
///
/// ## Errors
///
/// This function will error if the JSON is malformed or if the reader fails to read the data.
///
/// ## Panics
///
/// This function will panic if the hash is not valid UTF-8.
///
/// ## Unexpected Behavior
///
/// This function may not behave as expected if the hash is less than 3 bytes long.
pub fn load_index<R: std::io::Read>(reader: R) -> Result<HashMap<PathBuf, PathBuf>> {
    let data: Data = from_reader(reader)?;
    let mut map = HashMap::new();

    for (path, entry) in data.objects {
        let meaningful_path = PathBuf::from(path);
        let hash = entry.hash;
        let parts = [str::from_utf8(&hash.as_bytes()[0..2]).unwrap(), &hash];
        let hashed_path = PathBuf::from_iter(parts);
        map.insert(hashed_path, meaningful_path);
    }

    Ok(map)
}
