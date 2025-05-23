#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

use maex::run;
use std::{collections::VecDeque, io::Result, path::PathBuf, time};

fn main() -> Result<()> {
    // Command line arguments
    let mut args: VecDeque<String> = std::env::args().collect();
    let arg0 = args.pop_front().unwrap();
    let Some(index_path) = args.pop_front() else {
        eprintln!("Usage: {arg0} <index> [<output>]");
        std::process::exit(1);
    };
    let index_path = PathBuf::from(index_path);
    let output_path = match args.pop_front() {
        Some(path) => PathBuf::from(path),
        None => default_output()?,
    };

    run(&index_path, &output_path)
}

/// Returns the default output path.
fn default_output() -> Result<PathBuf> {
    // Executable path
    let exe_path = std::env::current_exe()?;
    // Directory of the executable
    let exe_dir = exe_path.parent().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to get the directory of the executable",
        )
    })?;
    // Default output path
    let timestamp = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let default_path = exe_dir.join(timestamp.to_string());

    Ok(default_path)
}
