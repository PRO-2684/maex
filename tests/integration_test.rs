use std::path::PathBuf;

use maex::load_index;

#[test]
fn test_load_index_success() {
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
