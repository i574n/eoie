use super::*;

#[test]
fn snapshot_preserves_sources_and_cleans_staging_after_failure() {
    let source = CoverageSmokeScratch::create("test-source").unwrap();
    fs::create_dir(source.0.join("src")).unwrap();
    fs::write(source.0.join("src/owner.spi"), b"original").unwrap();
    for ignored in [".git", ".cache", "vendor", "target"] {
        fs::create_dir(source.0.join(ignored)).unwrap();
        fs::write(source.0.join(ignored).join("keep"), b"ignored").unwrap();
    }
    let context = CoverageSmokeContext {
        root: &source.0, cargo: Path::new("cargo"), compiler: Path::new("compiler"),
        binary: Path::new("eoie"), target: Path::new("target"),
        profile_pattern: "profiles", timeout_ms: 1000, jobs: "2",
    };
    let mut staged = PathBuf::new();
    let result = coverage_smoke_in_snapshot(&context, |isolated| {
        staged = isolated.root.to_path_buf();
        assert_ne!(isolated.root, context.root);
        assert_eq!(isolated.target, context.target);
        for ignored in [".git", ".cache", "vendor", "target"] { assert!(!isolated.root.join(ignored).exists()); }
        fs::write(isolated.root.join("src/owner.spi"), b"mutated").unwrap();
        Err("deliberate phase failure".to_owned())
    });
    assert_eq!(result.unwrap_err(), "deliberate phase failure");
    assert_eq!(fs::read(source.0.join("src/owner.spi")).unwrap(), b"original");
    assert!(!staged.exists());
    assert!(!staged.parent().unwrap().exists());
}

#[test]
fn snapshot_bounds_file_bytes_entries_and_depth() {
    let source = CoverageSmokeScratch::create("test-bounds").unwrap();
    fs::write(source.0.join("file"), b"12345").unwrap();
    let output = CoverageSmokeScratch::create("test-output").unwrap();
    assert!(coverage_smoke_copy_tree(&source.0, &output.0.join("bytes"), &mut 4, &mut 10, 0).unwrap_err().contains("byte limit"));
    assert!(coverage_smoke_copy_tree(&source.0, &output.0.join("entries"), &mut 10, &mut 0, 0).unwrap_err().contains("entry limit"));
    assert!(coverage_smoke_copy_tree(&source.0, &output.0.join("depth"), &mut 10, &mut 10, 65).unwrap_err().contains("depth limit"));
    assert!(!output.0.join("bytes/file").exists());
}

#[cfg(unix)]
#[test]
fn snapshot_rejects_symlinks_without_following_them() {
    let source = CoverageSmokeScratch::create("test-link").unwrap();
    let outside = CoverageSmokeScratch::create("test-outside").unwrap();
    fs::write(outside.0.join("keep"), b"original").unwrap();
    std::os::unix::fs::symlink(&outside.0, source.0.join("linked")).unwrap();
    let output = CoverageSmokeScratch::create("test-output").unwrap();
    assert!(coverage_smoke_copy_tree(&source.0, &output.0.join("copy"), &mut 100, &mut 10, 0).unwrap_err().contains("links or special"));
    assert_eq!(fs::read(outside.0.join("keep")).unwrap(), b"original");
}

#[cfg(windows)]
#[test]
fn snapshot_rejects_windows_junctions_without_touching_targets() {
    let source = CoverageSmokeScratch::create("test-junction").unwrap();
    let outside = CoverageSmokeScratch::create("test-outside").unwrap();
    fs::write(outside.0.join("keep"), b"original").unwrap();
    let linked = source.0.join("linked");
    let created = Command::new("pwsh").args(["-NoProfile", "-NonInteractive", "-Command",
        "$ErrorActionPreference='Stop'; New-Item -ItemType Junction -Path $env:EOIE_TEST_JUNCTION -Target $env:EOIE_TEST_TARGET | Out-Null"])
        .env("EOIE_TEST_JUNCTION", &linked).env("EOIE_TEST_TARGET", &outside.0).output().unwrap();
    assert!(created.status.success(), "{created:?}");
    let output = CoverageSmokeScratch::create("test-output").unwrap();
    assert!(coverage_smoke_copy_tree(&source.0, &output.0.join("copy"), &mut 100, &mut 10, 0).unwrap_err().contains("links or special"));
    drop(source);
    assert_eq!(fs::read(outside.0.join("keep")).unwrap(), b"original");
}
