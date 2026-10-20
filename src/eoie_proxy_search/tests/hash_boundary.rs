#![cfg(target_os = "linux")]
use std::{env, fs, time::{SystemTime, UNIX_EPOCH}};

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
