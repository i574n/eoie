use std::fs::{self as win_fs, File as WinFile, OpenOptions as WinOptions};
use std::os::windows::fs::{MetadataExt as _, OpenOptionsExt as _};

pub struct WindowsPathGuard { pub path: PathBuf, _ancestors: Vec<WinFile> }

pub fn windows_validate_component(value: &std::ffi::OsStr) -> Result<(), String> {
    let text = value.to_string_lossy();
    let stem = text.split('.').next().unwrap_or("").trim_end().to_ascii_uppercase();
    if text.is_empty() || text.ends_with([' ', '.']) || text.chars().any(|c| c < ' ' || "<>:\"|?*".contains(c))
        || matches!(stem.as_str(), "CON" | "PRN" | "AUX" | "NUL" | "CONIN$" | "CONOUT$")
        || ((stem.starts_with("COM") || stem.starts_with("LPT")) && matches!(&stem[3..], "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "¹" | "²" | "³")) {
        return Err(format!("unsafe Windows path component: {text}"));
    }
    Ok(())
}

fn windows_open_directory(path: &Path) -> Result<WinFile, String> {
    let file = WinOptions::new().read(true).share_mode(3)
        .custom_flags(0x02000000 | 0x00200000).open(path)
        .map_err(|e| format!("open directory {}: {e}", path.display()))?;
    let meta = file.metadata().map_err(|e| e.to_string())?;
    if !meta.is_dir() || meta.file_attributes() & 0x400 != 0 {
        return Err(format!("real non-reparse directory required: {}", path.display()));
    }
    Ok(file)
}

pub fn windows_guard(path: &Path, include_leaf: bool, create: bool) -> Result<WindowsPathGuard, String> {
    if matches!(path.components().next(), Some(std::path::Component::Prefix(_))) && !path.has_root() {
        return Err(format!("drive-relative path rejected: {}", path.display()));
    }
    for c in path.components() {
        match c {
            std::path::Component::ParentDir => return Err(format!("parent component rejected: {}", path.display())),
            std::path::Component::Normal(n) => windows_validate_component(n)?,
            std::path::Component::Prefix(p) => match p.kind() {
                std::path::Prefix::Disk(_) | std::path::Prefix::VerbatimDisk(_) |
                std::path::Prefix::UNC(_, _) | std::path::Prefix::VerbatimUNC(_, _) => (),
                _ => return Err("Windows device paths are not supported".to_owned()),
            },
            _ => (),
        }
    }
    let absolute = std::path::absolute(path).map_err(|e| e.to_string())?;
    let directory = if include_leaf { absolute.as_path() } else { absolute.parent().ok_or("path has no parent")? };
    let mut cursor = PathBuf::new();
    let mut handles = Vec::new();
    for c in directory.components() {
        cursor.push(c.as_os_str());
        if matches!(c, std::path::Component::Prefix(_)) { continue; }
        if create && matches!(c, std::path::Component::Normal(_)) {
            match win_fs::create_dir(&cursor) {
                Ok(()) => (),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => (),
                Err(e) => return Err(format!("mkdir {}: {e}", cursor.display())),
            }
        }
        handles.push(windows_open_directory(&cursor)?);
    }
    Ok(WindowsPathGuard { path: absolute, _ancestors: handles })
}

pub fn windows_open_regular(path: &Path) -> Result<WinFile, String> {
    let guard = windows_guard(path, false, false)?;
    let file = WinOptions::new().read(true).custom_flags(0x00200000).open(&guard.path)
        .map_err(|e| format!("open {}: {e}", path.display()))?;
    let meta = file.metadata().map_err(|e| e.to_string())?;
    if !meta.is_file() || meta.file_attributes() & 0x400 != 0 {
        return Err(format!("regular non-reparse file required: {}", path.display()));
    }
    Ok(file)
}

fn windows_existing(path: &Path) -> Result<Option<std::fs::Permissions>, String> {
    match win_fs::symlink_metadata(path) {
        Ok(m) if m.is_file() && m.file_attributes() & 0x400 == 0 => Ok(Some(m.permissions())),
        Ok(_) => Err(format!("regular non-reparse target required: {}", path.display())),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

fn windows_within(root: &Path, path: &Path) -> Result<(), String> {
    let relative = path.strip_prefix(root).map_err(|_| format!("path escapes root: {}", path.display()))?;
    if relative.as_os_str().is_empty() || relative.components().any(|c| !matches!(c, std::path::Component::Normal(_))) {
        return Err(format!("nonempty confined relative path required: {}", path.display()));
    }
    Ok(())
}

#[repr(C)] struct RenameInfo { flags: u32, root: *mut std::ffi::c_void, bytes: u32, name: [u16; 1] }
const FILE_RENAME_FLAG_REPLACE_IF_EXISTS: u32 = 0x1;
const FILE_RENAME_FLAG_IGNORE_READONLY_ATTRIBUTE: u32 = 0x40;
const FILE_RENAME_INFO_EX: i32 = 22;
const DIRECTORY_JUNCTION_ATTRIBUTES: u32 = 0x400 | 0x10;
fn windows_rename_info(target: &Path, flags: u32) -> Result<(Vec<usize>, u32), String> {
    let name = std::os::windows::ffi::OsStrExt::encode_wide(target.as_os_str()).chain(Some(0)).collect::<Vec<_>>();
    let bytes = u32::try_from((name.len() - 1) * 2).map_err(|_| "rename path too long")?;
    let size = u32::try_from(std::mem::offset_of!(RenameInfo, name) + name.len() * 2).map_err(|_| "rename buffer too large")?;
    let mut buffer = vec![0usize; (size as usize).div_ceil(std::mem::size_of::<usize>())];
    let info = buffer.as_mut_ptr().cast::<RenameInfo>();
    unsafe { std::ptr::addr_of_mut!((*info).flags).write(flags); std::ptr::addr_of_mut!((*info).root).write(std::ptr::null_mut()); }
    unsafe { std::ptr::addr_of_mut!((*info).bytes).write(bytes); }
    unsafe { name.as_ptr().copy_to_nonoverlapping(std::ptr::addr_of_mut!((*info).name).cast::<u16>(), name.len()); }
    Ok((buffer, size))
}

fn windows_replace_readonly(source: &Path, target: &Path) -> Result<(), String> {
    use std::os::windows::io::AsRawHandle as _;
    #[link(name = "kernel32")]
    unsafe extern "system" { fn SetFileInformationByHandle(file: *mut std::ffi::c_void, class: i32, info: *mut std::ffi::c_void, size: u32) -> i32; }
    let file = WinOptions::new().access_mode(0x10000 | 0x80).share_mode(3)
        .custom_flags(0x02000000 | 0x00200000).open(source).map_err(|error| error.to_string())?;
    let metadata = file.metadata().map_err(|error| error.to_string())?;
    if !((metadata.is_file() && metadata.file_attributes() & 0x400 == 0) || metadata.file_type().is_symlink()) {
        return Err("replacement source must be a regular file or link".to_owned());
    }
    let (mut buffer, size) = windows_rename_info(target, FILE_RENAME_FLAG_REPLACE_IF_EXISTS | FILE_RENAME_FLAG_IGNORE_READONLY_ATTRIBUTE)?;
    let result = unsafe { SetFileInformationByHandle(file.as_raw_handle(), FILE_RENAME_INFO_EX, buffer.as_mut_ptr().cast(), size) };
    if result == 0 { return Err(format!("readonly entry promotion: {}", std::io::Error::last_os_error())); }
    Ok(())
}

pub fn rooted_promote_nondirectory_within(root: &Path, temporary: &Path, target: &Path, replace: bool) -> Result<(), String> {
    use std::os::windows::ffi::OsStrExt as _;
    #[link(name = "kernel32")]
    unsafe extern "system" { fn MoveFileExW(source: *const u16, target: *const u16, flags: u32) -> i32; }
    windows_within(root, temporary)?; windows_within(root, target)?;
    if temporary == target || temporary.parent() != target.parent() { return Err("entry promotion requires distinct siblings".to_owned()); }
    let source = windows_guard(temporary, false, false)?;
    let destination = windows_guard(target, false, false)?;
    for (path, required) in [(&source.path, true), (&destination.path, false)] {
        match win_fs::symlink_metadata(path) {
            Ok(metadata) if (metadata.is_file() && metadata.file_attributes() & 0x400 == 0) || metadata.file_type().is_symlink() => (),
            Ok(_) => return Err("entry promotion requires regular files or links".to_owned()),
            Err(error) if !required && error.kind() == std::io::ErrorKind::NotFound => (),
            Err(error) => return Err(error.to_string()),
        }
    }
    let parent = win_fs::canonicalize(source.path.parent().ok_or("stage has no parent")?).map_err(|error| error.to_string())?;
    let from_path = parent.join(source.path.file_name().ok_or("stage has no leaf")?);
    let to_path = parent.join(destination.path.file_name().ok_or("target has no leaf")?);
    let from = from_path.as_os_str().encode_wide().chain(Some(0)).collect::<Vec<_>>();
    let to = to_path.as_os_str().encode_wide().chain(Some(0)).collect::<Vec<_>>();
    let staged = windows_identity(&from_path)?;
    if unsafe { MoveFileExW(from.as_ptr(), to.as_ptr(), u32::from(replace)) } == 0 {
        let error = std::io::Error::last_os_error();
        if !(replace && error.raw_os_error() == Some(5)) { return Err(format!("entry promotion: {error}")); }
        windows_replace_readonly(&from_path, &to_path)?;
    }
    if windows_identity(&to_path)? != staged || win_fs::symlink_metadata(&from_path).is_ok() {
        return Err(format!("entry promotion reported success but {} is not the staged entry", to_path.display()));
    }
    Ok(())
}
fn windows_identity(path: &Path) -> Result<(u32, u64), String> {
    #[repr(C)] struct Info { attributes: u32, times: [u32; 6], volume: u32, size: [u32; 2], links: u32, index: [u32; 2] }
    #[link(name = "kernel32")] unsafe extern "system" { fn GetFileInformationByHandle(file: *mut std::ffi::c_void, info: *mut Info) -> i32; }
    let file = WinOptions::new().access_mode(0x80).share_mode(7).custom_flags(0x02000000 | 0x00200000).open(path).map_err(|error| format!("entry identity {}: {error}", path.display()))?;
    let mut info = std::mem::MaybeUninit::<Info>::uninit();
    if unsafe { GetFileInformationByHandle(std::os::windows::io::AsRawHandle::as_raw_handle(&file), info.as_mut_ptr()) } == 0 {
        return Err(format!("entry identity {}: {}", path.display(), std::io::Error::last_os_error()));
    }
    let info = unsafe { info.assume_init() }; Ok((info.volume, u64::from(info.index[0]) << 32 | u64::from(info.index[1])))
}
pub fn rooted_remove_empty_directory_within(root: &Path, path: &Path) -> Result<(), String> {
    windows_within(root, path)?;
    let guard = windows_guard(path, false, false)?;
    { let _directory = windows_open_directory(&guard.path)?; }
    win_fs::remove_dir(&guard.path).map_err(|error| error.to_string())
}

pub fn rooted_set_mode(path: &Path, mode: u32) -> Result<(), String> {
    if mode > 0o7777 { return Err("mode exceeds 07777".to_owned()); }
    let guard = windows_guard(path, false, false)?;
    let file = WinOptions::new().access_mode(0x80 | 0x100).share_mode(3)
        .custom_flags(0x02000000 | 0x00200000).open(&guard.path).map_err(|error| format!("open mode target {}: {error}", path.display()))?;
    let metadata = file.metadata().map_err(|error| error.to_string())?;
    if metadata.file_attributes() & 0x400 != 0 || !(metadata.is_file() || metadata.is_dir()) {
        return Err("mode target must be a real file or directory".to_owned());
    }
    let mut permissions = metadata.permissions();
    permissions.set_readonly(mode & 0o222 == 0);
    file.set_permissions(permissions).map_err(|error| format!("chmod {}: {error}", path.display()))
}

pub fn rooted_ensure_directory(path: &Path) -> Result<(), String> { windows_guard(path, true, true).map(|_| ()) }
pub fn rooted_create_directory_within(root: &Path, path: &Path) -> Result<(), String> {
    windows_within(root, path)?;
    let g = windows_guard(path, false, false)?;
    win_fs::create_dir(&g.path).map_err(|e| e.to_string())
}
pub fn rooted_set_directory_mode_within(root: &Path, path: &Path, _mode: u32) -> Result<(), String> {
    windows_within(root, path)?;
    windows_guard(path, true, false).map(|_| ())
}
pub fn rooted_promote_directory_within(root: &Path, temporary: &Path, target: &Path) -> Result<(), String> {
    windows_within(root, temporary)?; windows_within(root, target)?;
    let a = windows_guard(temporary, false, false)?;
    let b = windows_guard(target, false, false)?;
    { let _validated = windows_open_directory(&a.path)?; }
    if win_fs::symlink_metadata(&b.path).is_ok() { return Err("directory destination already exists".to_owned()); }
    win_fs::rename(&a.path, &b.path).map_err(|e| e.to_string())
}
pub fn rooted_create_regular_within(root: &Path, path: &Path, _mode: u32) -> Result<WinFile, String> {
    windows_within(root, path)?;
    let g = windows_guard(path, false, false)?;
    WinOptions::new().write(true).create_new(true).custom_flags(0x00200000).open(&g.path).map_err(|e| e.to_string())
}
pub fn windows_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    let native_target = target.components().collect::<PathBuf>();
    let target = native_target.as_path();
    let resolved = if target.is_absolute() { target.to_path_buf() } else { link.parent().unwrap_or(Path::new(".")).join(target) };
    if resolved.is_dir() { std::os::windows::fs::symlink_dir(target, link) }
    else { std::os::windows::fs::symlink_file(target, link) }
}
pub fn rooted_create_symlink_within(root: &Path, path: &Path, target: &Path) -> Result<(), String> {
    windows_within(root, path)?;
    let g = windows_guard(path, false, false)?;
    windows_symlink(target, &g.path).map_err(|e| format!("create symlink (Windows Developer Mode or symlink privilege required): {e}"))
}
pub fn rooted_create_hardlink_within(root: &Path, existing: &Path, link: &Path) -> Result<(), String> {
    windows_within(root, existing)?; windows_within(root, link)?;
    let a = windows_guard(existing, false, false)?;
    let b = windows_guard(link, false, false)?;
    let _source = windows_open_regular(&a.path)?;
    win_fs::hard_link(&a.path, &b.path).map_err(|e| e.to_string())
}

fn windows_stage_reader(target: &Path, reader: &mut impl std::io::Read, permissions: Option<std::fs::Permissions>, create: bool) -> Result<(PathBuf, u64), String> {
    let g = windows_guard(target, false, create)?;
    let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(|e| e.to_string())?.as_nanos();
    for attempt in 0..32 {
        let stage = target.with_file_name(format!(".eoie-stage-{}-{nonce}-{attempt}", std::process::id()));
        let absolute_stage = g.path.with_file_name(stage.file_name().ok_or("stage has no filename")?);
        let mut file = match WinOptions::new().write(true).create_new(true).open(&absolute_stage) {
            Ok(f) => f, Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue, Err(e) => return Err(e.to_string()),
        };
        let result = (|| {
            let copied = std::io::copy(reader, &mut file).map_err(|e| e.to_string())?;
            if let Some(p) = permissions { file.set_permissions(p).map_err(|e| e.to_string())?; }
            file.sync_all().map_err(|e| e.to_string())?;
            Ok(copied)
        })();
        drop(file);
        return match result {
            Ok(copied) => Ok((stage, copied)),
            Err(error) => match win_fs::remove_file(&absolute_stage) {
                Ok(()) => Err(error),
                Err(cleanup) => Err(format!("{error}; stage cleanup failed {}: {cleanup}", absolute_stage.display())),
            },
        };
    }
    Err("unable to allocate a unique stage".to_owned())
}
fn windows_stage(target: &Path, bytes: &[u8], permissions: Option<std::fs::Permissions>, create: bool) -> Result<PathBuf, String> {
    windows_stage_reader(target, &mut &bytes[..], permissions, create).map(|(path, _)| path)
}
pub fn atomic_write(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let g = windows_guard(path, false, true)?;
    let stage = windows_stage(&g.path, bytes, windows_existing(&g.path)?, false)?;
    if let Err(e) = promote_stage(&stage, &g.path) {
        return match discard_stage(&stage) { Ok(()) => Err(e), Err(cleanup) => Err(format!("{e}; stage cleanup failed {}: {cleanup}", stage.display())) };
    }
    Ok(())
}
pub fn copy_regular_atomic_preserve(source: &Path, target: &Path) -> Result<u64, String> {
    let mut file = windows_open_regular(source)?;
    let permissions = file.metadata().map_err(|e| e.to_string())?.permissions();
    let g = windows_guard(target, false, true)?;
    windows_existing(&g.path)?;
    let (stage, copied) = windows_stage_reader(&g.path, &mut file, Some(permissions), false)?;
    if let Err(e) = promote_stage(&stage, &g.path) {
        return match discard_stage(&stage) { Ok(()) => Err(e), Err(cleanup) => Err(format!("{e}; stage cleanup failed {}: {cleanup}", stage.display())) };
    }
    Ok(copied)
}
pub fn stage_text_receipted_bound(target: &Path, text: &str, stage_index: i32, backup_slot: i32) -> Result<StagedWriteReceipt, String> {
    if stage_index < 0 || backup_slot < 0 { return Err("stage identity must be non-negative".to_owned()); }
    let g = windows_guard(target, false, false)?;
    let permissions = windows_existing(&g.path)?.ok_or("replace target is absent")?;
    let temporary = windows_stage(target, text.as_bytes(), Some(permissions), false)?;
    Ok(StagedWriteReceipt { target: target.to_path_buf(), temporary, bytes_written: text.len(), stage_index, backup_slot })
}
pub fn stage_new_text(target: &Path, text: &str) -> Result<PathBuf, String> {
    let g = windows_guard(target, false, true)?;
    if windows_existing(&g.path)?.is_some() { return Err("new target already exists".to_owned()); }
    windows_stage(target, text.as_bytes(), None, false)
}
pub fn promote_stage(temporary: &Path, target: &Path) -> Result<(), String> {
    if temporary.parent() != target.parent() { return Err("staged promotion requires a shared parent".to_owned()); }
    let g = windows_guard(target, false, false)?;
    windows_existing(&g.path)?;
    let stage = windows_guard(temporary, false, false)?;
    { let _validated = windows_open_regular(&stage.path)?; }
    win_fs::rename(&stage.path, &g.path).map_err(|e| format!("promote stage: {e}"))
}
pub fn discard_stage(temporary: &Path) -> Result<(), String> {
    let g = windows_guard(temporary, false, false)?;
    if windows_existing(&g.path)?.is_none() { return Ok(()); }
    win_fs::remove_file(&g.path).map_err(|e| e.to_string())
}
pub fn rooted_remove_regular(path: &Path) -> Result<(), String> {
    let g = windows_guard(path, false, false)?;
    windows_existing(&g.path)?.ok_or("remove target is absent")?;
    win_fs::remove_file(&g.path).map_err(|e| e.to_string())
}
pub fn rooted_unlink_nondirectory(path: &Path) -> Result<(), String> {
    let g = windows_guard(path, false, false)?;
    let m = win_fs::symlink_metadata(&g.path).map_err(|e| e.to_string())?;
    if m.file_attributes() & DIRECTORY_JUNCTION_ATTRIBUTES == DIRECTORY_JUNCTION_ATTRIBUTES { win_fs::remove_dir(&g.path).map_err(|e| e.to_string()) }
    else if m.is_dir() { Err("unlink target is a directory".to_owned()) }
    else { win_fs::remove_file(&g.path).map_err(|e| e.to_string()) }
}
pub fn rooted_remove_tree_within(root: &Path, path: &Path) -> Result<u64, String> {
    windows_within(root, path)?;
    let parent = windows_guard(path, false, false)?;
    let directory = windows_guard(&parent.path, true, false)?;
    let mut count = 0;
    for entry in win_fs::read_dir(&directory.path).map_err(|e| e.to_string())? {
        let p = entry.map_err(|e| e.to_string())?.path();
        let m = win_fs::symlink_metadata(&p).map_err(|e| e.to_string())?;
        if m.is_dir() && m.file_attributes() & 0x400 == 0 { count += rooted_remove_tree_within(&directory.path, &p)?; }
        else { rooted_unlink_nondirectory(&p)?; count += 1; }
    }
    drop(directory);
    win_fs::remove_dir(&parent.path).map_err(|e| e.to_string())?;
    Ok(count + 1)
}
#[cfg(test)]
mod windows_copy_tests {
    use super::*;
    use std::io::{Read, Write};
    struct Fixture(PathBuf);
    impl Fixture {
        fn new() -> Self {
            let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
            let root = std::env::temp_dir().join(format!("eoie copy ü {} {nonce}", std::process::id()));
            win_fs::create_dir(&root).unwrap();
            Self(root)
        }
        fn stages(&self) -> usize {
            win_fs::read_dir(&self.0).unwrap().filter(|entry| entry.as_ref().unwrap().file_name().to_string_lossy().starts_with(".eoie-stage-")).count()
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            for entry in win_fs::read_dir(&self.0).into_iter().flatten().flatten() {
                let Ok(metadata) = entry.metadata() else { continue };
                let mut permissions = metadata.permissions();
                permissions.set_readonly(false);
                let _ = win_fs::set_permissions(entry.path(), permissions);
            }
            let _ = win_fs::remove_dir_all(&self.0);
        }
    }
    #[test]
    fn large_copy_preserves_bytes_and_readonly_permissions() {
        let fixture = Fixture::new();
        let source = fixture.0.join("source.bin");
        let target = fixture.0.join("target.bin");
        let mut file = WinFile::create(&source).unwrap();
        let chunk = [0xa5u8; 16 * 1024];
        for _ in 0..256 { file.write_all(&chunk).unwrap(); }
        drop(file);
        let mut permissions = win_fs::metadata(&source).unwrap().permissions();
        permissions.set_readonly(true);
        win_fs::set_permissions(&source, permissions).unwrap();
        assert_eq!(copy_regular_atomic_preserve(&source, &target).unwrap(), 4 * 1024 * 1024);
        assert!(win_fs::metadata(&target).unwrap().permissions().readonly());
        let mut reader = WinFile::open(&target).unwrap();
        let mut buffer = [0u8; 16 * 1024];
        for _ in 0..256 { reader.read_exact(&mut buffer).unwrap(); assert_eq!(buffer, chunk); }
        assert_eq!(reader.read(&mut buffer).unwrap(), 0);
        assert_eq!(fixture.stages(), 0);
    }
    #[test]
    fn rejected_readonly_copy_preserves_target_and_removes_stage() {
        let fixture = Fixture::new();
        let source = fixture.0.join("source.bin");
        let target = fixture.0.join("target.bin");
        win_fs::write(&source, b"replacement").unwrap();
        win_fs::write(&target, b"original").unwrap();
        let mut permissions = win_fs::metadata(&source).unwrap().permissions();
        permissions.set_readonly(true);
        win_fs::set_permissions(&source, permissions).unwrap();
        let locked = WinOptions::new().read(true).share_mode(1).open(&target).unwrap();
        assert!(copy_regular_atomic_preserve(&source, &target).is_err());
        assert_eq!(win_fs::read(&target).unwrap(), b"original");
        assert_eq!(fixture.stages(), 0, "failed copy leaked its readonly temporary");
        drop(locked);
    }
    #[test]
    fn rename_info_name_is_nul_terminated_within_the_passed_size() {
        let offset = std::mem::offset_of!(RenameInfo, name);
        let leaves_covering_every_name_length_mod_4 = ["t", "t1", "t22", "t333"];
        for leaf in leaves_covering_every_name_length_mod_4 {
            let target = Path::new(r"\\?\C:\eoie").join(leaf);
            let (buffer, size) = windows_rename_info(&target, 1).unwrap();
            let length = unsafe { (*buffer.as_ptr().cast::<RenameInfo>()).bytes } as usize;
            assert_eq!(length, 2 * target.as_os_str().len(), "FileNameLength excludes the NUL");
            assert!(offset + length + 2 <= size as usize, "{leaf}: NUL outside the passed size");
            assert_eq!(unsafe { *buffer.as_ptr().cast::<u16>().add((offset + length) / 2) }, 0, "{leaf}");
        }
    }
    struct GeneratedReader { remaining: usize, fail_at_end: bool }
    impl Read for GeneratedReader {
        fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
            assert!(buffer.len() <= 64 * 1024, "copy attempted a source-sized buffer");
            if self.remaining == 0 {
                return if self.fail_at_end { Err(std::io::Error::other("injected read failure")) } else { Ok(0) };
            }
            let length = buffer.len().min(self.remaining).min(997);
            buffer[..length].fill(0x5a);
            self.remaining -= length;
            Ok(length)
        }
    }
    #[test]
    fn stage_stream_uses_bounded_chunks_and_cleans_up_read_errors() {
        let fixture = Fixture::new();
        let target = fixture.0.join("target.bin");
        win_fs::write(&target, b"original").unwrap();
        let mut reader = GeneratedReader { remaining: 2 * 1024 * 1024, fail_at_end: false };
        let (stage, copied) = windows_stage_reader(&target, &mut reader, None, false).unwrap();
        assert_eq!(copied, 2 * 1024 * 1024);
        assert_eq!(win_fs::metadata(&stage).unwrap().len(), copied);
        discard_stage(&stage).unwrap();
        let mut broken = GeneratedReader { remaining: 8192, fail_at_end: true };
        let error = windows_stage_reader(&target, &mut broken, None, false).unwrap_err();
        assert!(error.contains("injected read failure"));
        assert_eq!(win_fs::read(target).unwrap(), b"original");
        assert_eq!(fixture.stages(), 0);
    }
}
