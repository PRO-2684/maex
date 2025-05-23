use std::{path::PathBuf, fs};
use maex::{load_index, run};

#[test]
fn test_load_index() {
    let json_data = r#"
    {
        "objects": {
            "path/to/asset": {
                "hash": "0123456789abcdef0123456789abcdef01234567",
                "size": 12345
            }
        }
    }
    "#;

    let reader = json_data.as_bytes();
    let result = load_index(reader).unwrap();
    assert_eq!(result.len(), 1);

    let key = PathBuf::from("01/0123456789abcdef0123456789abcdef01234567");
    let value = PathBuf::from("path/to/asset");
    assert_eq!(result.get(&key).unwrap(), &value);
}

#[test]
fn test_run() {
    let index_path = PathBuf::from("tests/assets/indexes/1.json");
    let output_path = PathBuf::from("tests/output");
    run(&index_path, &output_path).unwrap();

    // Check if expected files exist
    let should_exist = [
        "icons/icon_128x128.png",
        "icons/icon_16x16.png",
    ];
    for path in should_exist {
        let full_path = output_path.join(path);
        assert!(full_path.exists(), "File {} does not exist", full_path.display());
    }

    // Clean up
    fs::remove_dir_all(output_path).unwrap();
}
