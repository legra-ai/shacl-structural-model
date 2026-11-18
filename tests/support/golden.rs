//! Golden JSON and Postcard vectors, plus plain two-format round trips.
//!
//! Regenerate deliberately with `UPDATE_GOLDEN=1`.

use std::fmt::Debug;
use std::path::PathBuf;

use serde::Serialize;
use serde::de::DeserializeOwned;

fn golden_path(name: &str, extension: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/golden")
        .join(format!("{name}.{extension}"))
}

/// Assert that `value` survives JSON and Postcard.
pub fn round_trip<T>(value: &T)
where
    T: Serialize + DeserializeOwned + PartialEq + Debug,
{
    let json = serde_json::to_vec(value).unwrap();
    assert_eq!(&serde_json::from_slice::<T>(&json).unwrap(), value);
    let binary = postcard::to_allocvec(value).unwrap();
    assert_eq!(&postcard::from_bytes::<T>(&binary).unwrap(), value);
}

/// Assert that `value` encodes to the golden vectors and decodes back from them.
pub fn assert_golden<T>(name: &str, value: &T)
where
    T: Serialize + DeserializeOwned + PartialEq + Debug,
{
    let json = serde_json::to_string_pretty(value).unwrap() + "\n";
    let binary = postcard::to_allocvec(value).unwrap();
    let (json_path, binary_path) = (golden_path(name, "json"), golden_path(name, "postcard"));
    if std::env::var_os("UPDATE_GOLDEN").is_some() {
        std::fs::write(&json_path, &json).unwrap();
        std::fs::write(&binary_path, &binary).unwrap();
    }
    let expected_json = std::fs::read_to_string(&json_path).unwrap();
    let expected_binary = std::fs::read(&binary_path).unwrap();
    assert_eq!(json, expected_json, "JSON vector {name} changed");
    assert_eq!(binary, expected_binary, "Postcard vector {name} changed");
    assert_eq!(&serde_json::from_str::<T>(&expected_json).unwrap(), value);
    assert_eq!(&postcard::from_bytes::<T>(&expected_binary).unwrap(), value);
}
