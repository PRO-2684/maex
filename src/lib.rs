//! # `maex` library crate
//!
//! If you are reading this, you are reading the documentation for the `maex` library crate. For the cli, kindly refer to the README file.
//!
//! This library provides two public functions, one low-level and one high-level:
//!
//! - [`load_index`]: Reads the index (as a reader), returns a map of hashed paths to meaningful paths.
//! - [`run`]: Extracts assets, given the index path and the output path.

#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

use serde::Deserialize;
use serde_json::{Result as SerdeResult, from_reader};
use std::{
    collections::HashMap,
    fs::{self, File},
    io,
    path::{Path, PathBuf},
    process,
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
pub fn load_index<R: io::Read>(reader: R) -> SerdeResult<HashMap<PathBuf, PathBuf>> {
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

/// Extracts assets, given the index path and the output path.
pub fn run(index_path: &Path, output_path: &Path) -> io::Result<()> {
    // Create the output path if it doesn't exist
    if !output_path.exists() {
        fs::create_dir_all(output_path)?;
    }
    // Resolve objects base path
    let base_path = index_path.parent().and_then(|p| p.parent());
    let Some(base_path) = base_path else {
        eprintln!("Failed to resolve base path");
        process::exit(1);
    };
    let base_path = base_path.join("objects");
    // Read the index
    eprintln!("Reading index from {}...", index_path.display());
    let index_file = File::open(index_path)?;
    let index = load_index(index_file)?;

    // Iterate over the hashmap
    for (hashed_path, meaningful_path) in index.into_iter() {
        eprintln!("{} -> {}", hashed_path.display(), meaningful_path.display());
        // Copy `base_path/hashed_path` to `output_path/meaningful_path`
        let src = base_path.join(&hashed_path);
        let dest = output_path.join(meaningful_path);
        // Create the file recursively
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)?;
        }
        // Copy the file
        fs::copy(src, dest)?;
    }

    Ok(())
}
