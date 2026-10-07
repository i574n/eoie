use super::*;

struct Fixture(std::path::PathBuf);
impl Fixture {
    fn new() -> Self {
        let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let root = std::env::temp_dir().join(format!("eoie-fs-actions-{}-{nonce}", std::process::id()));
        std::fs::create_dir_all(root.join("nested/deeper")).unwrap();
        std::fs::write(root.join("target.txt"), "root target").unwrap();
        Self(root)
    }
}
impl Drop for Fixture {
    fn drop(&mut self) { let _ = std::fs::remove_dir_all(&self.0); }
}

#[test]
fn chmod_accepts_real_directories_and_preserves_verification() {
    let fixture = Fixture::new();
    let directory = fixture.0.join("nested");
    let mode = fs_action_mode(&directory).unwrap();
    let items = [FsActionItem { relative: "nested".into(), kind: FsActionKind::Chmod { expected_mode: mode, new_mode: mode } }];
    let prepared = fs_action_prepare(&fixture.0, &items).unwrap();
    let undo = fs_action_apply_one(&prepared[0]).unwrap();
    fs_action_verify_one(&prepared[0]).unwrap();
    fs_action_rollback(&[undo]).unwrap();
}

#[test]
fn nested_symlink_targets_remain_root_relative() {
    let fixture = Fixture::new();
    for (link, expected) in [("link", "target.txt"), ("nested/link", "../target.txt"), ("nested/deeper/link", "../../target.txt")] {
        let items = [FsActionItem { relative: link.into(), kind: FsActionKind::Symlink { target: "target.txt".into() } }];
        let prepared = fs_action_prepare(&fixture.0, &items).unwrap();
        let FsPreparedAction::Symlink { path, target, .. } = &prepared[0] else { panic!("expected symlink") };
        assert_eq!(target, &std::path::PathBuf::from(expected));
        assert_eq!(std::fs::read_to_string(path.parent().unwrap().join(target)).unwrap(), "root target");
        // Verify the actual filesystem link where symlink creation is available.
        match fs_action_apply_one(&prepared[0]) {
            Ok(undo) => {
                fs_action_verify_one(&prepared[0]).unwrap();
                assert_eq!(std::fs::read_to_string(path).unwrap(), "root target");
                fs_action_rollback(&[undo]).unwrap();
                assert!(!path.exists());
            }
            Err(error) if cfg!(windows) && error.contains("1314") => eprintln!("symlink privilege unavailable; target resolution was verified"),
            Err(error) => panic!("{error}"),
        }
    }
}

#[test]
fn spiral_mode_policy_rejects_out_of_range_values() {
    for mode in -1..=4096 {
        assert_eq!(eoie_fs_actions_mode_valid_binding(mode, 0), i32::from((0..=4095).contains(&mode)));
    }
    assert_eq!(eoie_fs_actions_mode_valid_binding(493, 1), 0);
    assert!(fs_action_parse_mode("10000").is_err());
    assert_eq!(fs_action_parse_mode("0o755").unwrap(), 0o755);
}
#[test]
fn chmod_apply_and_rollback_reject_parent_substitution() {
    let fixture = Fixture::new();
    let path = fixture.0.join("nested/value.txt");
    std::fs::write(&path, b"inside").unwrap();
    let outside = fixture.0.join("outside");
    std::fs::create_dir(&outside).unwrap();
    std::fs::write(outside.join("value.txt"), b"outside").unwrap();
    let before = std::fs::metadata(outside.join("value.txt")).unwrap().permissions();
    let mode = fs_action_mode(&path).unwrap();
    let items = [FsActionItem { relative: "nested/value.txt".into(), kind: FsActionKind::Chmod { expected_mode: mode, new_mode: 0 } }];
    let prepared = fs_action_prepare(&fixture.0, &items).unwrap();
    std::fs::rename(fixture.0.join("nested"), fixture.0.join("moved")).unwrap();
    #[cfg(windows)] {
        use std::os::windows::process::CommandExt as _;
        let output = std::process::Command::new("pwsh").creation_flags(0x08000000)
            .args(["-NoProfile", "-NonInteractive", "-Command", "$ErrorActionPreference='Stop'; New-Item -ItemType Junction -Path $env:EOIE_TEST_LINK -Target $env:EOIE_TEST_TARGET | Out-Null"])
            .env("EOIE_TEST_LINK", fixture.0.join("nested")).env("EOIE_TEST_TARGET", &outside).output().unwrap();
        assert!(output.status.success(), "{output:?}");
    }
    #[cfg(unix)] std::os::unix::fs::symlink(&outside, fixture.0.join("nested")).unwrap();
    assert!(fs_action_apply_one(&prepared[0]).is_err());
    assert!(fs_action_rollback(&[FsActionUndo::RestoreMode { path, permissions: before.clone() }]).is_err());
    let directory = fixture.0.join("nested/new");
    assert!(fs_action_apply_one(&FsPreparedAction::Mkdir { path: directory, existed: false }).is_err());
    assert!(!outside.join("new").exists());
    std::fs::create_dir(outside.join("empty")).unwrap();
    assert!(fs_action_rollback(&[FsActionUndo::RemoveDir(fixture.0.join("nested/empty"))]).is_err());
    assert!(outside.join("empty").is_dir());
    assert_eq!(eoie_rust_std_fs::portable_mode(&std::fs::metadata(outside.join("value.txt")).unwrap().permissions()), eoie_rust_std_fs::portable_mode(&before));
    #[cfg(windows)] std::fs::remove_dir(fixture.0.join("nested")).unwrap();
    #[cfg(unix)] std::fs::remove_file(fixture.0.join("nested")).unwrap();
}

fn assert_no_action_stage(root: &std::path::Path) {
    assert!(!std::fs::read_dir(root).unwrap().any(|entry| entry.unwrap().file_name().to_string_lossy().starts_with(".eoie-fs-action-stage-")));
}

#[test]
fn failed_link_creation_preserves_the_original_file() {
    let fixture = Fixture::new();
    let path = fixture.0.join("target.txt");
    let original = fs_action_existing_link(&path).unwrap();
    let error = fs_action_publish_entry_with(&path, &original, |_| Err("injected creation failure".into())).unwrap_err();
    assert!(error.contains("injected creation failure"));
    assert_eq!(std::fs::read(&path).unwrap(), b"root target");
    assert_no_action_stage(&fixture.0);
}

#[test]
fn changed_destination_is_preserved_and_owned_stage_is_removed() {
    let fixture = Fixture::new();
    let path = fixture.0.join("target.txt");
    let original = fs_action_existing_link(&path).unwrap();
    let result = fs_action_publish_entry_with(&path, &original, |stage| {
        fs_action_stage_regular(stage, b"candidate", &std::fs::metadata(&path).unwrap().permissions())?;
        std::fs::write(&path, b"concurrent").unwrap();
        Ok(())
    });
    assert!(result.is_err());
    assert_eq!(std::fs::read(&path).unwrap(), b"concurrent");
    assert_no_action_stage(&fixture.0);
}

#[test]
fn link_replacement_restores_original_file_and_rejects_changed_rollback_target() {
    let fixture = Fixture::new();
    let path = fixture.0.join("nested/replaced");
    std::fs::write(&path, b"original").unwrap();
    let permissions = std::fs::metadata(&path).unwrap().permissions();
    let items = [FsActionItem { relative: "nested/replaced".into(), kind: FsActionKind::Symlink { target: "target.txt".into() } }];
    let prepared = fs_action_prepare(&fixture.0, &items).unwrap();
    let undo = match fs_action_apply_one(&prepared[0]) {
        Ok(undo) => undo,
        Err(error) if cfg!(windows) && error.contains("1314") => { assert_eq!(std::fs::read(&path).unwrap(), b"original"); return; }
        Err(error) => panic!("{error}"),
    };
    assert_eq!(std::fs::read(&path).unwrap(), b"root target");
    fs_action_rollback(&[undo]).unwrap();
    assert_eq!(std::fs::read(&path).unwrap(), b"original");
    assert!(!std::fs::symlink_metadata(&path).unwrap().file_type().is_symlink());
    assert_eq!(eoie_rust_std_fs::portable_mode(&std::fs::metadata(&path).unwrap().permissions()), eoie_rust_std_fs::portable_mode(&permissions));
    let undo = fs_action_apply_one(&prepared[0]).unwrap();
    std::fs::remove_file(&path).unwrap();
    std::fs::write(&path, b"concurrent").unwrap();
    assert!(fs_action_rollback(&[undo]).is_err());
    assert_eq!(std::fs::read(&path).unwrap(), b"concurrent");
    assert_no_action_stage(path.parent().unwrap());
}

#[test]
fn entry_promotion_preserves_competing_targets_and_rejects_directories() {
    let fixture = Fixture::new();
    let stage = fixture.0.join("stage");
    let target = fixture.0.join("target.txt");
    std::fs::write(&stage, b"candidate").unwrap();
    assert!(eoie_rust_std_fs_mutation::rooted_promote_nondirectory_within(&fixture.0, &stage, &target, false).is_err());
    assert_eq!(std::fs::read(&target).unwrap(), b"root target");
    assert_eq!(std::fs::read(&stage).unwrap(), b"candidate");
    assert!(eoie_rust_std_fs_mutation::rooted_promote_nondirectory_within(&fixture.0, &stage, &fixture.0.join("nested"), true).is_err());
    assert!(fixture.0.join("nested/deeper").is_dir());
    eoie_rust_std_fs_mutation::rooted_promote_nondirectory_within(&fixture.0, &stage, &target, true).unwrap();
    assert_eq!(std::fs::read(target).unwrap(), b"candidate");
}

#[test]
fn directory_rollback_refuses_to_remove_new_contents() {
    let fixture = Fixture::new();
    let path = fixture.0.join("created");
    let undo = fs_action_apply_one(&FsPreparedAction::Mkdir { path: path.clone(), existed: false }).unwrap();
    std::fs::write(path.join("foreign"), b"keep").unwrap();
    assert!(fs_action_rollback(&[undo]).is_err());
    assert_eq!(std::fs::read(path.join("foreign")).unwrap(), b"keep");
}

fn prepared_copy(fixture: &Fixture, existing: bool) -> FsPreparedAction {
    if existing { std::fs::write(fixture.0.join("copied"), b"original").unwrap(); }
    let expected_sha = eoie_proxy_search::inspection_file_sha256(&fixture.0.join("target.txt")).unwrap();
    fs_action_prepare(&fixture.0, &[FsActionItem { relative: "copied".into(), kind: FsActionKind::Copy { source: "target.txt".into(), expected_sha } }]).unwrap().remove(0)
}

#[test]
fn copy_preserves_destination_changes_since_planning() {
    for existing in [false, true] {
        let fixture = Fixture::new();
        let action = prepared_copy(&fixture, existing);
        std::fs::write(fixture.0.join("copied"), b"concurrent").unwrap();
        assert!(fs_action_apply_one(&action).is_err());
        assert_eq!(std::fs::read(fixture.0.join("copied")).unwrap(), b"concurrent");
        assert_no_action_stage(&fixture.0);
    }
}

#[test]
fn copy_rejects_changed_source_before_publishing() {
    let fixture = Fixture::new();
    let action = prepared_copy(&fixture, true);
    std::fs::write(fixture.0.join("target.txt"), b"changed source").unwrap();
    assert!(fs_action_apply_one(&action).is_err());
    assert_eq!(std::fs::read(fixture.0.join("copied")).unwrap(), b"original");
    assert_no_action_stage(&fixture.0);
}

#[test]
fn copy_rollback_restores_original_but_preserves_later_edits() {
    for existing in [false, true] {
        let fixture = Fixture::new();
        let action = prepared_copy(&fixture, existing);
        let undo = fs_action_apply_one(&action).unwrap();
        fs_action_verify_one(&action).unwrap();
        fs_action_rollback(&[undo]).unwrap();
        let target = fixture.0.join("copied");
        if existing { assert_eq!(std::fs::read(&target).unwrap(), b"original"); } else { assert!(!target.exists()); }
        let undo = fs_action_apply_one(&action).unwrap();
        std::fs::write(&target, b"concurrent").unwrap();
        assert!(fs_action_rollback(&[undo]).is_err());
        assert_eq!(std::fs::read(&target).unwrap(), b"concurrent");
        assert_no_action_stage(&fixture.0);
    }
}

#[cfg(windows)]
#[test]
fn entry_promotion_supports_paths_beyond_the_legacy_windows_limit() {
    let fixture = Fixture::new();
    let segment = "long-path-segment-".repeat(6);
    let parent = fixture.0.join(&segment).join(&segment).join(&segment);
    std::fs::create_dir_all(&parent).unwrap();
    assert!(parent.as_os_str().len() > 260);
    let stage = parent.join("stage");
    // Four leaf lengths cover every target-name length mod 4, i.e. every rename-buffer tail (CI run 37460036298).
    for leaf in ["target", "target1", "target22", "target333"] {
        let target = parent.join(leaf);
        std::fs::write(&stage, b"candidate").unwrap();
        eoie_rust_std_fs_mutation::rooted_promote_nondirectory_within(&parent, &stage, &target, false).unwrap();
        assert_eq!(std::fs::read(&target).unwrap(), b"candidate");
        assert!(!stage.exists());
        let mut readonly = std::fs::metadata(&target).unwrap().permissions();
        readonly.set_readonly(true);
        std::fs::set_permissions(&target, readonly).unwrap();
        std::fs::write(&stage, b"replacement").unwrap();
        eoie_rust_std_fs_mutation::rooted_promote_nondirectory_within(&parent, &stage, &target, true).unwrap();
        assert_eq!(std::fs::read(&target).unwrap(), b"replacement", "{leaf}");
        assert_eq!(std::fs::read_dir(&parent).unwrap().count(), 1 + ["target", "target1", "target22", "target333"].iter().position(|l| *l == leaf).unwrap(), "{leaf}");
    }
}

#[test]
fn readonly_copy_can_roll_back_and_failed_publication_cleans_its_stage() {
    for existing in [false, true] {
        let fixture = Fixture::new();
        let source = fixture.0.join("target.txt");
        let old_permissions = std::fs::metadata(&source).unwrap().permissions();
        let mut readonly = old_permissions.clone();
        readonly.set_readonly(true);
        std::fs::set_permissions(&source, readonly).unwrap();
        let action = prepared_copy(&fixture, existing);
        let undo = fs_action_apply_one(&action).unwrap();
        let target = fixture.0.join("copied");
        assert!(std::fs::metadata(&target).unwrap().permissions().readonly());
        #[cfg(windows)] {
            use std::os::windows::fs::OpenOptionsExt as _;
            let locked = std::fs::OpenOptions::new().read(true).share_mode(1).open(&target).unwrap();
            assert!(fs_action_rollback(std::slice::from_ref(&undo)).is_err());
            assert_eq!(std::fs::read(&target).unwrap(), b"root target");
            assert!(std::fs::metadata(&target).unwrap().permissions().readonly());
            assert_no_action_stage(&fixture.0);
            drop(locked);
        }
        fs_action_rollback(&[undo]).unwrap();
        if existing { assert_eq!(std::fs::read(&target).unwrap(), b"original"); } else { assert!(!target.exists()); }
        std::fs::write(&target, b"concurrent").unwrap();
        assert!(fs_action_apply_one(&action).is_err());
        assert_eq!(std::fs::read(&target).unwrap(), b"concurrent");
        assert_no_action_stage(&fixture.0);
        std::fs::set_permissions(&source, old_permissions).unwrap();
    }
}
