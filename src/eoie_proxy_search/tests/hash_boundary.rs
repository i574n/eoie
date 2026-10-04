use std::{env, fs, time::{SystemTime, UNIX_EPOCH}};

#[test]
fn hashes_regular_bytes_with_a_bound_and_rejects_linked_ancestors() {
    if env::var_os("EOIE_HASH_BOUNDARY_CHILD").is_none() {
        let output = std::process::Command::new(env::current_exe().unwrap())
            .args(["--exact", "hashes_regular_bytes_with_a_bound_and_rejects_linked_ancestors"])
            .env("EOIE_HASH_LIMIT_BYTES", "3").env("EOIE_HASH_BOUNDARY_CHILD", "1").output().unwrap();
        assert!(output.status.success(), "{output:?}");
        return;
    }
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let root = env::temp_dir().join(format!("eoie hash ü {} {stamp}", std::process::id()));
    fs::create_dir_all(root.join("real")).unwrap();
    let file = root.join("real/input");
    fs::write(&file, b"abc").unwrap();
    assert_eq!(eoie_proxy_search::inspection_file_sha256(&file).unwrap(), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    fs::write(&file, b"abcd").unwrap();
    assert!(eoie_proxy_search::inspection_file_sha256(&file).is_err());
    fs::write(&file, b"abc").unwrap();
    let linked = root.join("linked");
    #[cfg(windows)] {
        use std::os::windows::process::CommandExt as _;
        let output = std::process::Command::new("pwsh").creation_flags(0x08000000)
            .args(["-NoProfile", "-NonInteractive", "-Command", "$ErrorActionPreference='Stop'; New-Item -ItemType Junction -Path $env:EOIE_TEST_LINK -Target $env:EOIE_TEST_TARGET | Out-Null"])
            .env("EOIE_TEST_LINK", &linked).env("EOIE_TEST_TARGET", root.join("real")).output().unwrap();
        assert!(output.status.success(), "{output:?}");
    }
    #[cfg(unix)] std::os::unix::fs::symlink(root.join("real"), &linked).unwrap();
    assert!(eoie_proxy_search::inspection_file_sha256(&linked.join("input")).is_err());
    assert!(eoie_proxy_search::inspection_file_sha256(&root).is_err());
    assert_eq!(fs::read(&file).unwrap(), b"abc");
    #[cfg(windows)] fs::remove_dir(&linked).unwrap();
    #[cfg(unix)] fs::remove_file(&linked).unwrap();
    fs::remove_dir_all(root).unwrap();
}

#[cfg(target_os = "linux")]
#[test]
fn linux_fifo_is_rejected_without_waiting_for_a_writer() {
    use std::{ffi::CString, os::unix::{ffi::OsStrExt as _, fs::FileTypeExt as _}, process::{Command, Stdio}, time::{Duration, Instant}};
    if let Some(root) = env::var_os("EOIE_FIFO_CHILD") {
        let root = std::path::PathBuf::from(root);
        let fifo = root.join("fifo");
        assert!(eoie_proxy_search::inspection_file_sha256(&fifo).is_err());
        assert!(eoie_rust_std_fs::rooted_open_regular_read(&fifo).is_err());
        assert!(eoie_rust_std_fs::copy_regular_atomic_preserve(&root.join("source"), &fifo).is_err());
        assert!(eoie_rust_std_fs::copy_regular_atomic_preserve(&fifo, &root.join("output")).is_err());
        assert!(fs::symlink_metadata(fifo).unwrap().file_type().is_fifo());
        assert!(!root.join("output").exists());
        return;
    }
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let root = env::temp_dir().join(format!("eoie-fifo-{}-{stamp}", std::process::id()));
    fs::create_dir(&root).unwrap();
    fs::write(root.join("source"), b"source").unwrap();
    let path = CString::new(root.join("fifo").as_os_str().as_bytes()).unwrap();
    unsafe extern "C" { fn mkfifo(path: *const std::os::raw::c_char, mode: u32) -> i32; }
    assert_eq!(unsafe { mkfifo(path.as_ptr(), 0o600) }, 0);
    let mut child = Command::new(env::current_exe().unwrap())
        .args(["--exact", "linux_fifo_is_rejected_without_waiting_for_a_writer", "--nocapture"])
        .env("EOIE_FIFO_CHILD", &root).stdout(Stdio::null()).spawn().unwrap();
    let deadline = Instant::now() + Duration::from_secs(5);
    let status = loop {
        if let Some(status) = child.try_wait().unwrap() { break Some(status); }
        if Instant::now() >= deadline { child.kill().unwrap(); child.wait().unwrap(); break None; }
        std::thread::sleep(Duration::from_millis(10));
    };
    fs::remove_dir_all(root).unwrap();
    assert!(status.is_some_and(|status| status.success()), "regular-file operations blocked or failed on a FIFO");
}
