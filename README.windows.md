# EOIE on Windows

Run the native executable from PowerShell:

```powershell
.\eoie.exe help
```

`eoie.exe` is the Windows build; Linux builds produce `eoie`. Both are ignored
build products. An original Linux binary may still exist in older local bundles.
`eoie.ps1` builds the native executable if it is missing, then forwards arguments
and the exit code.

## Build and test

Use Rust 1.88 or newer with the Windows MSVC toolchain and the Visual Studio C++
build tools. Cargo downloads the exact dependencies in `src/Cargo.lock` from
crates.io on the first build. `src/vendor` remains ignored and is not required.
Use `-Offline` only after fetching the locked dependencies into your Cargo cache.

The build/test driver is now authored in Spiral. From `src`, bootstrap it in a
separate target directory, then run it from that same directory:

```powershell
cargo build --locked -p eoie-dev --target-dir target/workflow-driver
./target/workflow-driver/debug/eoie-dev.exe --root .. --test --skip-release
```

Use `--offline` after caching dependencies. This runs tests without installing
an executable; omit `--skip-release` to build a release candidate too. Keeping
the driver outside the workspace build target avoids Windows executable locks.
PowerShell remains a transitional adapter for regeneration and validated installation:

```powershell
pwsh -NoProfile -File .\build.ps1 -Test
```

This builds the CLI, runs the native workspace tests, builds an optimized release,
and copies it to `eoie.exe`. It works from any current directory. Build output is
under `src/target`, replacing the old `/mnt/data/eoie-rust-target` setting.

Every workspace suite runs. Individual tests requiring optional compiler package
attestation and Plan IR interfaces are explicitly ignored by default. The patched
alpha418 single-flight host implements these interfaces. To include those tests,
set `EOIE_DOTNET` to its .NET host and supply its built `SpiralCompiler.dll`:

```powershell
pwsh -NoProfile -File .\build.ps1 -Test -CompilerContracts -SpiralCompiler C:\tools\SpiralCompiler.dll
```

`EOIE_SPIRAL_COMPILE` also selects a compiler. Discovery searches Windows `.exe`
and `.cmd` names on PATH and `%APPDATA%/eoie/spiral-compiler.path`, following
explicit `EOIE_CONFIG_HOME` and `XDG_CONFIG_HOME` overrides. The supplied Linux
compiler/toolchain payloads cannot run as Windows executables.

Managed compiler identities include neighboring assemblies, runtime configuration
and native runtime libraries, so changing `SpiralCompilerCore.dll` invalidates
cached attestations even when the launcher is unchanged. Paths are relative to
the compiler directory; relocating the same files preserves identity. Debug
symbols and snapshot metadata are excluded. Discovery rejects links and bounds
the dependency tree to eight levels, 512 entries and 128 MiB total artifact data.

For native Spiral code generation and compiler configuration, see [README.md](README.md).

## Runtime behavior

- Reads, staged writes, replacement, copies, directory creation, and removal use
  a native Windows backend. It rejects junction/reparse ancestors, path traversal,
  alternate data streams, device names, and trailing-dot/space aliases. Directory
  handles deny delete sharing while operations use their paths.
- Preserved-mode file copies stream into an exclusive temporary file instead of loading
  the entire source into memory. Read failures and rejected replacements remove
  the temporary file and preserve the existing destination.
- `fs-copy-tree` rejects overlapping paths and linked ancestors, copies into an
  exclusive sibling directory, then publishes without replacing a destination.
  Failed copies remove only their own stage. Traversal stops at 128 nested levels
  or 100,000 entries; names must be valid Unicode, and directory enumeration
  enforces the remaining entry budget before collecting more names.
- Bounded processes start suspended, enter a Windows Job Object, and then resume.
  Timeout and error cleanup terminate descendants as well as the immediate child.
  Processes run without opening console windows.
- ZIP creation, verification, and extraction run natively. Windows archives mark
  `eoie` and executable/script extensions executable in ZIP Unix metadata.
- POSIX file modes map to Windows's read-only attribute. Execute bits and Unix
  owner/group distinctions do not change Windows ACLs. Symlink commands require
  Windows Developer Mode or an account with the symlink privilege; failures are
  reported rather than replaced with a different kind of link.
- `fs-chmod` changes permissions through a validated handle and rejects linked
  roots, ancestors and leaves. Filesystem-action mode changes and rollback use
  the same boundary. `fs-symlink` treats both input paths as root-relative,
  stores a relocatable link-relative target and creates the link exclusively.
- Filesystem-action copies and links stage output before replacement. Changed
  destinations block publication, and rollback preserves detected later edits.
  Copy rollback handles read-only output and reports Windows sharing locks.
  Entry promotion replaces a read-only target through `FileRenameInfoEx` with a
  NUL-terminated name inside the passed size, then checks that the target is the
  staged entry (volume serial and file index) and the stage name is gone; a
  reported success that did not promote is returned as an error. The check
  relies on file IDs that survive a rename, as on NTFS and ReFS; on FAT/exFAT,
  where an ID can change with the directory entry, a completed promotion may be
  reported as an error.
  File hashing uses bounded reads and rejects linked ancestors.
- `self-upgrade-check` accepts exactly one `eoie` or `eoie.exe` in each bundle
  and runs all six strict compatibility checks; generic archive validation
  cannot substitute for those release checks.
- Use native programs in command plans, such as `cmd.exe` or `pwsh.exe`. Linux
  paths such as `/bin/sh` and Linux portable-toolchain receipts remain Linux inputs.

The saved workspace lease can expire independently of operating system support.
To begin a new work session in this bundle:

```powershell
.\eoie.exe agile begin . "Windows work session"
```

The port does not fabricate new attestations for the original Linux release.
Source changes can make its recorded hashes/evidence stale; resealing a release
requires the matching compiler and release verification workflow.

## Source and validation

Agile receipts bind package manifests and imported Spiral sources as well as
state files and compiler identity. An unsupported or oversized package graph
requires a fresh compiler check. A failed check or a concurrent input change
does not publish new receipts; ordinary Agile task updates remain available
without a compiler. Task edits and history appends only renew a file receipt when
the original file matches its previous attestation. Other edits remain visible
to the next compiler check.

Platform-specific Rust lives in `rust_std_fs_mutation/windows.rs`,
`rust_std_fs/windows.rs`, `rust_std_fs/portable.rs`, and
`eoie_process_streaming/windows.rs`. The matching embedded Rust in the `.spi`
sources is updated too. Retain these adapter files when regenerating Rust owners.

Windows regression tests cover Unicode/space paths, junction rejection, ancestor
locking, traversal/device-name rejection, staged replacement and rollback,
process output/exit status, descendant termination, stdout-consumer deadlines,
command receipts, and ZIP roundtrips/executable metadata. Linux-specific backends
remain behind their original platform conditions; Linux execution was not tested
in this Windows environment. From this directory, Linux compilation can be
checked without WSL:

```powershell
rustup target add x86_64-unknown-linux-gnu
Set-Location src
cargo check --locked --workspace --all-targets --target x86_64-unknown-linux-gnu --target-dir target/linux-check --jobs 2
```

This compiles Linux-specific code and tests but does not execute them. The
Windows/Linux CI matrix remains responsible for runtime validation on both hosts.
