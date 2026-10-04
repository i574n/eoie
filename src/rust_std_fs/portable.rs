pub fn portable_mode(permissions: &std::fs::Permissions) -> u32 {
    #[cfg(unix)] { use std::os::unix::fs::PermissionsExt; permissions.mode() & 0o7777 }
    #[cfg(not(unix))] { if permissions.readonly() { 0o555 } else { 0o755 } }
}
pub fn portable_mode_matches(observed: u32, expected: u32) -> bool {
    #[cfg(unix)] { observed == expected }
    #[cfg(not(unix))] { (observed & 0o222 == 0) == (expected & 0o222 == 0) }
}
pub fn portable_set_mode(path: &Path, mode: u32) -> std::io::Result<()> {
    eoie_rust_std_fs_mutation::rooted_set_mode(path, mode).map_err(std::io::Error::other)
}
pub fn portable_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    #[cfg(unix)] { std::os::unix::fs::symlink(target, link) }
    #[cfg(windows)] { windows_symlink(target, link) }
    #[cfg(not(any(unix, windows)))] { Err(std::io::Error::new(std::io::ErrorKind::Unsupported, "symlink unsupported")) }
}
