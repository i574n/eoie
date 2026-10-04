#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_patterns, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
use std::fs;
use std::path::Path;

fn legacy_copy_tree_inner(source: &Path, destination: &Path, stage: &Path, depth: usize, entries: &mut usize, modes: &mut Vec<(std::path::PathBuf, u32)>) -> Result<usize, String> {
    if eoie_tree_copy_entry_allowed_binding(i32::try_from(depth).unwrap_or(-1), i32::try_from(*entries).unwrap_or(-1)) != 1 { return Err("fs-copy-tree traversal limit exceeded".to_owned()); }
    *entries += 1;
    let metadata = fs::symlink_metadata(source).map_err(|error| format!("metadata {}: {error}", source.display()))?;
    #[cfg(windows)]
    let linked = { use std::os::windows::fs::MetadataExt as _; metadata.file_attributes() & 0x400 != 0 };
    #[cfg(not(windows))]
    let linked = metadata.file_type().is_symlink();
    if linked { return Err(format!("fs-copy-tree rejects linked entry: {}", source.display())); }
    if metadata.is_file() {
        eoie_rust_std_fs::copy_regular_atomic_preserve(source, destination)?;
        return Ok(1);
    }
    if !metadata.is_dir() { return Err(format!("unsupported entry: {}", source.display())); }
    if destination != stage { eoie_rust_std_fs_mutation::rooted_create_directory_within(stage, destination)?; }
    modes.push((destination.to_path_buf(), eoie_rust_std_fs::portable_mode(&metadata.permissions())));
    let names = eoie_rust_std_fs::rooted_list_names_limited(source, eoie_tree_copy_remaining_binding(*entries as i32, 0) as usize)?;
    let mut copied = 0usize;
    for name in names {
        copied += legacy_copy_tree_inner(&source.join(&name), &destination.join(name), stage, depth + 1, entries, modes)?;
    }
    Ok(copied)
}

fn legacy_copy_destination(path: &Path) -> Result<std::path::PathBuf, String> {
    if path.components().any(|part| matches!(part, std::path::Component::ParentDir)) {
        return Err("fs-copy-tree destination rejects parent components".to_owned());
    }
    let absolute = std::path::absolute(path).map_err(|error| error.to_string())?;
    let mut existing = absolute.as_path();
    let mut missing = Vec::new();
    loop {
        match fs::symlink_metadata(existing) {
            Ok(_) => break,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                missing.push(existing.file_name().ok_or("destination has no leaf")?.to_owned());
                existing = existing.parent().ok_or("destination has no existing ancestor")?;
            }
            Err(error) => return Err(format!("destination metadata: {error}")),
        }
    }
    if missing.is_empty() { return Err(format!("destination already exists: {}", path.display())); }
    let mut resolved = eoie_rust_std_fs::rooted_canonical_directory(existing)?;
    for part in missing.into_iter().rev() { resolved.push(part); }
    Ok(resolved)
}

fn legacy_copy_tree_with_hook(source: &Path, destination: &Path, before_publish: impl FnOnce(&Path) -> Result<(), String>) -> Result<usize, String> {
    let source = eoie_rust_std_fs::rooted_canonical_directory(source)?;
    let destination = legacy_copy_destination(destination)?;
    if destination.starts_with(&source) || source.starts_with(&destination) {
        return Err("fs-copy-tree source and destination must not overlap".to_owned());
    }
    let parent = destination.parent().ok_or("destination has no parent")?;
    eoie_rust_std_fs_mutation::rooted_ensure_directory(parent)?;
    #[cfg(windows)]
    let _source_guard = eoie_rust_std_fs::windows_guard(&source, true, false)?;
    #[cfg(windows)]
    let _parent_guard = eoie_rust_std_fs::windows_guard(parent, true, false)?;
    let stamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(|error| error.to_string())?.as_nanos();
    let stage = parent.join(format!(".eoie-copy-stage-{}-{stamp}", std::process::id()));
    eoie_rust_std_fs_mutation::rooted_create_directory_within(parent, &stage)?;
    let mut modes = Vec::new();
    let result: Result<usize, String> = (|| {
        let copied = legacy_copy_tree_inner(&source, &stage, &stage, 0, &mut 0, &mut modes)?;
        before_publish(&destination)?;
        for (directory, mode) in modes.iter().rev() {
            eoie_rust_std_fs_mutation::rooted_set_directory_mode_within(parent, directory, *mode)?;
        }
        eoie_rust_std_fs_mutation::rooted_promote_directory_within(parent, &stage, &destination)?;
        Ok(copied)
    })();
    match result {
        Ok(copied) => Ok(copied),
        Err(error) => {
            let cleanup = (|| {
                for (directory, mode) in &modes {
                    eoie_rust_std_fs_mutation::rooted_set_directory_mode_within(parent, directory, *mode | 0o700)?;
                }
                eoie_rust_std_fs::rooted_remove_tree_within(parent, &stage)?;
                Ok::<(), String>(())
            })();
            match cleanup {
                Ok(()) => Err(format!("fs-copy-tree rolled back owned stage: {error}")),
                Err(cleanup) => Err(format!("fs-copy-tree failed: {error}; stage cleanup failed: {cleanup}")),
            }
        }
    }
}

pub fn copy_tree_run(args: &[String]) -> Result<(), String> {
    if args.len() != 3 { return Err("fs-copy-tree expects source destination".to_owned()); }
    let source = Path::new(&args[1]);
    let destination = Path::new(&args[2]);
    let copied = legacy_copy_tree_with_hook(source, destination, |_| Ok(()))?;
    println!("eoie proxy fs-copy-tree ok source={} destination={} files={copied}", source.display(), destination.display());
    Ok(())
}

#[cfg(test)]
#[path = "tests/support/tree_copy.rs"]
mod tree_copy_tests;


fn method0(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 < 0i32;
    if v2 {
        0i32
    } else {
        let mut v3: bool = 128i32 < v0;
        if v3 {
            0i32
        } else {
            let mut v4: bool = v1 < 0i32;
            if v4 {
                0i32
            } else {
                let mut v5: bool = v1 < 100000i32;
                if v5 {
                    1i32
                } else {
                    0i32
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(i32, i32) -> i32> {
    Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        method0(v0, v1)
    })
}
fn method1(mut v0: i32) -> i32 {
    let mut v1: bool = v0 < 0i32;
    if v1 {
        -1i32
    } else {
        let mut v2: bool = 100000i32 < v0;
        if v2 {
            -1i32
        } else {
            let mut v3: i32 = 100000i32 - v0;
            v3
        }
    }
}
fn closure1() -> Rc<dyn Fn(i32, i32) -> i32> {
    Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        method1(v0)
    })
}
pub fn eoie_tree_copy_entry_allowed_binding(v0: i32, v1: i32) -> i32 {
    closure0()(v0, v1)
}
pub fn eoie_tree_copy_remaining_binding(v0: i32, v1: i32) -> i32 {
    closure1()(v0, v1)
}
