#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_patterns, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone, Debug)]
pub struct SpiralCompilerDiscovery {
    pub path: std::path::PathBuf,
    pub source: &'static str,
    pub fingerprint: u64,
    pub entry_sha256: String,
    pub facade_sha256: Option<String>,
    pub sha256: String,
}

fn spiral_hash_bytes(hash: &mut u64, bytes: &[u8]) {
    for byte in bytes { *hash ^= u64::from(*byte); *hash = hash.wrapping_mul(1099511628211); }
}

fn spiral_compiler_facade(path: &std::path::Path) -> Result<Option<std::path::PathBuf>, String> {
    let Some(bin) = path.parent() else { return Ok(None); };
    let Some(root) = bin.parent() else { return Ok(None); };
    let candidate = root.join("source/facade/bin/linux/amd64/Release/net11.0/SpiralCompilerFacadeCli.dll");
    let metadata = match std::fs::symlink_metadata(&candidate) {
        Ok(value) => value,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(format!("inspect Spiral compiler facade {}: {error}", candidate.display())),
    };
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(format!("Spiral compiler facade must be a regular non-symlink file: {}", candidate.display()));
    }
    std::fs::canonicalize(&candidate).map(Some).map_err(|error| format!("canonicalize Spiral compiler facade {}: {error}", candidate.display()))
}

fn spiral_dependency_name(name: &str) -> bool {
    let name = name.to_ascii_lowercase();
    name.ends_with(".dll") || name.ends_with(".deps.json")
        || name.ends_with(".runtimeconfig.json") || name.ends_with(".runtimeconfig.dev.json")
        || name.ends_with(".so") || name.contains(".so.") || name.ends_with(".dylib")
}

fn spiral_artifact_metadata(path: &std::path::Path) -> Result<std::fs::Metadata, String> {
    let metadata = std::fs::symlink_metadata(path).map_err(|error| format!("inspect compiler artifact {}: {error}", path.display()))?;
    let mut linked = metadata.file_type().is_symlink();
    #[cfg(windows)] {
        use std::os::windows::fs::MetadataExt;
        linked |= metadata.file_attributes() & 0x400 != 0;
    }
    if linked || !(metadata.is_file() || metadata.is_dir()) { return Err(format!("compiler artifact must not be a link or special entry: {}", path.display())); }
    Ok(metadata)
}

fn spiral_dependency_paths(root: &std::path::Path, directory: &std::path::Path, depth: usize, remaining: &mut usize, output: &mut Vec<(String, std::path::PathBuf)>) -> Result<(), String> {
    if depth > 8 { return Err("compiler dependency tree exceeds depth limit".to_owned()); }
    if !spiral_artifact_metadata(directory)?.is_dir() { return Err("compiler dependency root is not a directory".to_owned()); }
    for name in eoie_rust_std_fs::rooted_list_names(directory)? {
        let path = directory.join(&name);
        *remaining = remaining.checked_sub(1).ok_or_else(|| "compiler dependency inventory exceeds 512 entries".to_owned())?;
        let metadata = spiral_artifact_metadata(&path)?;
        if metadata.is_dir() { spiral_dependency_paths(root, &path, depth + 1, remaining, output)?; continue; }
        if !spiral_dependency_name(&name) { continue; }
        if !spiral_artifact_metadata(&path)?.is_file() { return Err(format!("compiler dependency is not a file: {}", path.display())); }
        let relative = path.strip_prefix(root).map_err(|error| error.to_string())?;
        let label = relative.to_str().ok_or_else(|| "compiler dependency path is not UTF-8".to_owned())?.replace('\\', "/");
        output.push((label, path));
    }
    Ok(())
}

fn spiral_managed_dependencies(entry: &std::path::Path) -> Result<Vec<(String, std::path::PathBuf)>, String> {
    let dll = entry.extension().and_then(|value| value.to_str()).is_some_and(|value| value.eq_ignore_ascii_case("dll"));
    let apphost = entry.with_extension("dll");
    if !dll && !apphost.is_file() { return Ok(Vec::new()); }
    let root = entry.parent().ok_or_else(|| "compiler has no parent directory".to_owned())?;
    let mut paths = Vec::new();
    spiral_dependency_paths(root, root, 0, &mut 512, &mut paths)?;
    paths.retain(|(_, path)| path != entry);
    paths.sort_by(|first, second| first.0.cmp(&second.0));
    Ok(paths)
}

fn spiral_artifact_sha(path: &std::path::Path, remaining: &mut usize) -> Result<String, String> {
    let metadata = spiral_artifact_metadata(path)?;
    if !metadata.is_file() || metadata.len() > *remaining as u64 { return Err(format!("compiler identity exceeds 128 MiB artifact budget: {}", path.display())); }
    let bytes = eoie_rust_std_fs::read_regular_limited(path, *remaining)?;
    *remaining -= bytes.len();
    Ok(eoie_proxy_search::inspection_sha256(bytes))
}

fn spiral_compiler_identity(path: &std::path::Path, facade: Option<&std::path::Path>) -> Result<(u64, String, Option<String>, String), String> {
    let mut budget = 128 * 1024 * 1024;
    let entry_sha256 = spiral_artifact_sha(path, &mut budget)?;
    let facade_sha256 = match facade { Some(path) => Some(spiral_artifact_sha(path, &mut budget)?), None => None };
    let entry_name = path.file_name().and_then(|value| value.to_str()).ok_or_else(|| "compiler entry filename is not UTF-8".to_owned())?;
    let mut identity = format!("schema=3\nentry_name={}:{}\nentry_sha256={entry_sha256}\nfacade_sha256={}\n", entry_name.len(), entry_name, facade_sha256.as_deref().unwrap_or("absent"));
    for (scope, entry) in [("entry", Some(path)), ("facade", facade)] {
        if let Some(entry) = entry {
            for (relative, artifact) in spiral_managed_dependencies(entry)? {
                let sha = spiral_artifact_sha(&artifact, &mut budget)?;
                identity.push_str(&format!("{scope}/{}:{relative}={sha}\n", relative.len()));
            }
        }
    }
    let mut fingerprint = 14695981039346656037u64;
    spiral_hash_bytes(&mut fingerprint, identity.as_bytes());
    let sha256 = eoie_proxy_search::inspection_sha256(identity.into_bytes());
    Ok((fingerprint, entry_sha256, facade_sha256, sha256))
}

fn validate_spiral_compiler(candidate: &std::path::Path, source: &'static str) -> Result<SpiralCompilerDiscovery, String> {
    if !candidate.is_absolute() { return Err(format!("Spiral compiler from {source} must be absolute: {}", candidate.display())); }
    let metadata = std::fs::symlink_metadata(candidate).map_err(|error| format!("inspect Spiral compiler from {source} {}: {error}", candidate.display()))?;
    if metadata.file_type().is_symlink() || !metadata.is_file() { return Err(format!("Spiral compiler from {source} must be a regular non-symlink file: {}", candidate.display())); }
    let path = std::fs::canonicalize(candidate).map_err(|error| format!("canonicalize Spiral compiler {}: {error}", candidate.display()))?;
    let facade = spiral_compiler_facade(&path)?;
    let (fingerprint, entry_sha256, facade_sha256, sha256) = spiral_compiler_identity(&path, facade.as_deref())?;
    Ok(SpiralCompilerDiscovery { path, source, fingerprint, entry_sha256, facade_sha256, sha256 })
}

fn spiral_compiler_config_path() -> Option<std::path::PathBuf> {
    if let Some(value) = std::env::var_os("EOIE_CONFIG_HOME").filter(|value| !value.is_empty()) { return Some(std::path::PathBuf::from(value).join("spiral-compiler.path")); }
    if let Some(value) = std::env::var_os("XDG_CONFIG_HOME").filter(|value| !value.is_empty()) { return Some(std::path::PathBuf::from(value).join("eoie/spiral-compiler.path")); }
    #[cfg(windows)] if let Some(value) = std::env::var_os("APPDATA").filter(|v| !v.is_empty()) { return Some(std::path::PathBuf::from(value).join("eoie/spiral-compiler.path")); }
    std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")).filter(|value| !value.is_empty()).map(|value| std::path::PathBuf::from(value).join(".config/eoie/spiral-compiler.path"))
}

fn configured_spiral_compiler() -> Result<Option<SpiralCompilerDiscovery>, String> {
    let Some(config) = spiral_compiler_config_path() else { return Ok(None); };
    let metadata = match std::fs::symlink_metadata(&config) {
        Ok(value) => value,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(format!("inspect compiler config {}: {error}", config.display())),
    };
    if metadata.file_type().is_symlink() || !metadata.is_file() || metadata.len() > 4096 { return Err(format!("compiler config must be a regular non-symlink file <=4096 bytes: {}", config.display())); }
    let value = std::fs::read_to_string(&config).map_err(|error| format!("read compiler config {}: {error}", config.display()))?;
    let value = value.trim();
    if value.is_empty() || value.lines().count() != 1 { return Err(format!("compiler config must contain exactly one absolute path: {}", config.display())); }
    validate_spiral_compiler(std::path::Path::new(value), "config").map(Some)
}

fn path_spiral_compiler() -> Result<Option<SpiralCompilerDiscovery>, String> {
    let Some(path) = std::env::var_os("PATH") else { return Ok(None); };
    for directory in std::env::split_paths(&path) {
        for name in if cfg!(windows) { &["spiral-compile-session.exe", "spiral-compile.exe", "spiral-compile-session.cmd", "spiral-compile.cmd"][..] } else { &["spiral-compile-session", "spiral-compile"][..] } {
            let candidate = directory.join(name);
            if candidate.is_file() { return validate_spiral_compiler(&candidate, "PATH").map(Some); }
        }
    }
    Ok(None)
}

pub fn resolve_spiral_compiler(explicit: Option<&str>) -> Result<Option<SpiralCompilerDiscovery>, String> {
    if let Some(value) = explicit.filter(|value| !value.trim().is_empty()) { return validate_spiral_compiler(std::path::Path::new(value), "argument").map(Some); }
    if let Some(value) = std::env::var_os("EOIE_SPIRAL_COMPILE").filter(|value| !value.is_empty()) {
        let value = value.to_string_lossy();
        return validate_spiral_compiler(std::path::Path::new(value.as_ref()), "environment").map(Some);
    }
    if let Some(value) = configured_spiral_compiler()? { return Ok(Some(value)); }
    path_spiral_compiler()
}

fn spiral_main() -> i32 {
    0i32
}
fn main() {
    let main = std::thread::Builder::new().stack_size(1 << 30).spawn(spiral_main).unwrap();
    std::process::exit(main.join().unwrap());
}
