#![cfg(any(windows, target_os = "linux"))]
use eoie_rust_std_fs::{rooted_list_names, rooted_list_names_limited};
use std::fs;

fn scratch() -> std::path::PathBuf {
    let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let root = std::env::temp_dir().join(format!("eoie-list-bounds-{}-{nonce}", std::process::id()));
    fs::create_dir(&root).unwrap();
    root
}
#[test]
fn directory_limits_are_exact_and_preserve_sorted_unicode_names() {
    let root = scratch();
    assert!(rooted_list_names_limited(&root, 0).unwrap().is_empty());
    for name in ["z", "a", "ü"] { fs::write(root.join(name), b"data").unwrap(); }
    assert!(rooted_list_names_limited(&root, 0).is_err());
    assert!(rooted_list_names_limited(&root, 2).unwrap_err().contains("limit"));
    assert_eq!(rooted_list_names_limited(&root, 3).unwrap(), ["a", "z", "ü"]);
    assert_eq!(rooted_list_names(&root).unwrap(), ["a", "z", "ü"]);
    fs::remove_dir_all(root).unwrap();
}
#[test]
fn directory_listing_rejects_lossy_filename_aliases() {
    let root = scratch();
    #[cfg(windows)]
    let invalid = { use std::os::windows::ffi::OsStringExt as _; std::ffi::OsString::from_wide(&[0xd800]) };
    #[cfg(target_os = "linux")]
    let invalid = { use std::os::unix::ffi::OsStringExt as _; std::ffi::OsString::from_vec(vec![0xff]) };
    fs::write(root.join(invalid), b"must not disappear").unwrap();
    fs::write(root.join("�"), b"different file").unwrap();
    assert!(rooted_list_names_limited(&root, 10).unwrap_err().contains("Unicode"));
    assert!(rooted_list_names(&root).unwrap_err().contains("Unicode"));
    fs::remove_dir_all(root).unwrap();
}

