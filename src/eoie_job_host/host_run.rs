#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn string_slice(value: &str, from: i64, to: i64) -> Rc<str> {
    let bytes = value.as_bytes();
    let length = bytes.len() as i64;
    if from < 0 || from > length || to < from - 1 || to >= length { std::process::abort(); }
    if to < from { return { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }; }
    if (bytes[from as usize] & 0xC0) == 0x80 || (to + 1 < length && (bytes[(to + 1) as usize] & 0xC0) == 0x80) { std::process::exit(3); }
    let slice = &bytes[from as usize..(to + 1) as usize];
    match std::str::from_utf8(slice) { Ok(text) => Rc::<str>::from(text), Err(error) => Rc::<str>::from(std::str::from_utf8(&slice[..error.valid_up_to()]).unwrap_or("")) }
}
#[derive(Clone)]
enum US0 {
    US0_HomePresent,
    US0_HomeGone,
}
#[derive(Clone)]
enum US1 {
    US1_KeepServing,
    US1_HostIdleExit,
}
#[derive(Clone)]
enum US2 {
    US2_Some(u64),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_HostOwns,
    US3_OtherHostServes(u64),
}
#[derive(Clone)]
enum US4 {
    US4_HostMissing,
    US4_HostRunning(u64),
}
#[derive(Clone)]
enum US5 {
    US5_SupervisorUp(u64),
    US5_SupervisorLate,
}
#[derive(Clone)]
enum US6 {
    US6_Some(US5),
    US6_None,
}
fn joined_3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2.clone()
}
fn jobs_root_5() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("EOIE_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("eoie").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("eoie").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".eoie-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn host_home_directory_4() -> Rc<str> {
    let mut v0: Rc<str> = jobs_root_5();
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("shell-host"); } LIT.with(|lit| lit.clone()) };
    joined_3(v0.clone(), v1.clone())
}
fn requests_directory_2() -> Rc<str> {
    let mut v0: Rc<str> = host_home_directory_4();
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("requests"); } LIT.with(|lit| lit.clone()) };
    joined_3(v0.clone(), v1.clone())
}
fn next_request_6(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_dir(&*v0).into_iter().flatten().flatten().map(|entry| entry.file_name().to_string_lossy().into_owned()).filter(|name| name.ends_with(".start")).min().unwrap_or_default());
    v1.clone()
}
fn text_or_empty_7(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1.clone()
}
fn request_removed_8(mut v0: Rc<str>) -> bool {
    let mut v1: bool = std::fs::remove_file(&*v0).is_ok();
    v1
}
fn loop_10(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".start"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(6i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0.clone(), 0i32 as i64, v5 as i64);
            v0 = v6.clone();
            continue;
        } else {
            return v0.clone();
        }
    }
}
fn request_id_9(mut v0: Rc<str>) -> Rc<str> {
    loop_10(v0.clone())
}
fn supervisor_child_started_11(mut v0: std::rc::Rc<std::cell::RefCell<Vec<std::process::Child>>>, mut v1: Rc<str>, mut v2: Rc<str>) -> bool {
    let mut v3: bool = { let mut command = std::process::Command::new(&*v1); command.arg("job").arg("supervise").arg(&*v2).stdin(std::process::Stdio::null()).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()); #[cfg(windows)] { use std::os::windows::process::CommandExt; command.creation_flags(0x0800_0000); } match command.spawn() { Ok(child) => { (v0).borrow_mut().push(child); true } Err(_) => false } };
    v3
}
fn now_ms_12() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn live_children_13(mut v0: std::rc::Rc<std::cell::RefCell<Vec<std::process::Child>>>) -> u64 {
    let mut v1: u64 = { let mut kids = (v0).borrow_mut(); kids.retain_mut(|child| matches!(child.try_wait(), Ok(None))); kids.len() as u64 };
    v1
}
fn directory_exists_15(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.is_dir();
    v8
}
fn home_of_14() -> US0 {
    let mut v0: Rc<str> = requests_directory_2();
    let mut v1: bool = directory_exists_15(v0.clone());
    if v1 {
        US0::US0_HomePresent
    } else {
        US0::US0_HomeGone
    }
}
fn host_step_of_16(mut v0: u64, mut v1: u64, mut v2: u64, mut v3: US0) -> US1 {
    let mut v4: bool = v0 > 0u64;
    if v4 {
        US1::US1_KeepServing
    } else {
        let mut v6: bool = match &v3 {
            US0::US0_HomeGone => {
                true
            }
            _ => {
                false
            }
        };
        if v6 {
            US1::US1_HostIdleExit
        } else {
            let mut v8: u64 = v2.wrapping_sub(v1);
            let mut v9: bool = v8 >= 600000u64;
            if v9 {
                US1::US1_HostIdleExit
            } else {
                US1::US1_KeepServing
            }
        }
    }
}
fn host_pid_file_17() -> Rc<str> {
    let mut v0: Rc<str> = host_home_directory_4();
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pid"); } LIT.with(|lit| lit.clone()) };
    joined_3(v0.clone(), v1.clone())
}
fn method19(mut v0: Option<u64>) -> Option<u64> {
    v0.clone()
}
fn closure1() -> Rc<dyn Fn((u64)) -> US2> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US2> = Rc::new(move |mut v0: (u64)| -> US2 {
        let mut v1: u64 = (v0);
        US2::US2_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_18(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method19(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US2> = closure1();
    let mut v4: Option<US2> = v2.map(|x| v3(x));
    let mut v5: US2 = US2::US2_None;
    let mut v6: US2 = v4.unwrap_or(v5);
    match &v6 {
        US2::US2_None => {
            0u64
        }
        US2::US2_Some(v7) => {
            let mut v7: u64 = *v7;
            v7
        }
    }
}
fn own_pid_20() -> u64 {
    let mut v0: u64 = std::process::id() as u64;
    v0
}
fn host_served_1(mut v0: std::rc::Rc<std::cell::RefCell<Vec<std::process::Child>>>, mut v1: u64) -> i32 {
    loop {
        let mut v2: Rc<str> = requests_directory_2();
        let mut v3: Rc<str> = next_request_6(v2.clone());
        let mut v4: bool = v3.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v4 {
            let mut v5: Rc<str> = joined_3(v2.clone(), v3.clone());
            let mut v6: Rc<str> = text_or_empty_7(v5.clone());
            let mut v7: bool = request_removed_8(v5.clone());
            let mut v8: Rc<str> = request_id_9(v3.clone());
            let mut v9: bool = supervisor_child_started_11(v0.clone(), v6.clone(), v8.clone());
            let mut v10: u64 = now_ms_12();
            (v0, v1) = (v0.clone(), v10);
            continue;
        } else {
            let mut v12: u64 = live_children_13(v0.clone());
            let mut v13: u64 = now_ms_12();
            let mut v14: US0 = home_of_14();
            let mut v15: US1 = host_step_of_16(v12, v1, v13, v14.clone());
            match &v15 {
                US1::US1_HostIdleExit => {
                    let mut v16: Rc<str> = host_pid_file_17();
                    let mut v17: Rc<str> = text_or_empty_7(v16.clone());
                    let mut v18: u64 = decimal_u64_18(v17.clone());
                    let mut v19: u64 = own_pid_20();
                    let mut v20: bool = v18 == v19;
                    let mut v23: bool = if v20 {
                        let mut v21: Rc<str> = host_pid_file_17();
                        request_removed_8(v21.clone())
                    } else {
                        false
                    };
                    return 0i32;
                }
                US1::US1_KeepServing => {
                    let mut v24: u64 = 500u64;
                    std::thread::sleep(std::time::Duration::from_millis(v24));
                    let mut v25: bool = v12 > 0u64;
                    let mut v26: u64 = if v25 {
                        v13
                    } else {
                        v1
                    };
                    (v0, v1) = (v0.clone(), v26);
                    continue;
                }
            }
        }
    }
}
fn process_alive_21(mut v0: u64) -> bool {
    let mut v1: bool = { #[cfg(windows)] fn alive(pid: u64) -> bool { unsafe extern "system" { fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void; fn WaitForSingleObject(handle: *mut std::ffi::c_void, ms: u32) -> u32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; fn GetLastError() -> u32; } if pid == 0 { return true; } unsafe { let handle = OpenProcess(0x0010_0000, 0, pid as u32); if handle.is_null() { return GetLastError() == 5; } let running = WaitForSingleObject(handle, 0) == 258; CloseHandle(handle); running } } #[cfg(not(windows))] fn alive(pid: u64) -> bool { pid == 0 || std::os::unix::process::parent_id() as u64 == pid } alive(v0) };
    v1
}
fn host_claim_of_22(mut v0: u64, mut v1: u64, mut v2: bool) -> US3 {
    let mut v3: bool = v0 != 0u64;
    let mut v5: bool = if v3 {
        let mut v4: bool = v0 != v1;
        v4
    } else {
        false
    };
    let mut v6: bool = v5 && v2;
    if v6 {
        US3::US3_OtherHostServes(v0)
    } else {
        US3::US3_HostOwns
    }
}
fn decimal_text_23(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1.clone()
}
fn text_written_24(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2.clone()
}
fn no_children_25() -> std::rc::Rc<std::cell::RefCell<Vec<std::process::Child>>> {
    let mut v0: std::rc::Rc<std::cell::RefCell<Vec<std::process::Child>>> = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    v0.clone()
}
fn host_run_0() -> i32 {
    let mut v0: Rc<str> = host_pid_file_17();
    let mut v1: Rc<str> = text_or_empty_7(v0.clone());
    let mut v2: u64 = decimal_u64_18(v1.clone());
    let mut v3: u64 = own_pid_20();
    let mut v4: bool = process_alive_21(v2);
    let mut v5: US3 = host_claim_of_22(v2, v3, v4);
    match &v5 {
        US3::US3_HostOwns => {
            let mut v7: Rc<str> = requests_directory_2();
            let mut v8: Rc<str> = std::rc::Rc::<str>::from(std::fs::create_dir_all(&*v7).err().map(|error| format!("create {}: {error}", &*v7)).unwrap_or_default());
            let mut v9: Rc<str> = host_pid_file_17();
            let mut v10: u64 = own_pid_20();
            let mut v11: Rc<str> = decimal_text_23(v10);
            let mut v12: Rc<str> = text_written_24(v9.clone(), v11.clone());
            let mut v13: std::rc::Rc<std::cell::RefCell<Vec<std::process::Child>>> = no_children_25();
            let mut v14: u64 = now_ms_12();
            host_served_1(v13.clone(), v14)
        }
        US3::US3_OtherHostServes(v6) => {
            let mut v6: u64 = *v6;
            0i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        host_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn supervisor_executable_27() -> Rc<str> {
    let mut v0: Rc<str> = { let copy = || -> std::io::Result<std::path::PathBuf> { let source = std::env::current_exe()?; let metadata = std::fs::metadata(&source)?; let stamp = metadata.modified().ok().and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok()).map(|elapsed| elapsed.as_secs()).unwrap_or(0); let folder = std::env::var_os("LOCALAPPDATA").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir).join("eoie").join("supervisor"); std::fs::create_dir_all(&folder)?; let stem = source.file_stem().map(|stem| stem.to_string_lossy().into_owned()).unwrap_or_default(); let target = folder.join(format!("{}-{}-{}{}", stem, metadata.len(), stamp, std::env::consts::EXE_SUFFIX)); if !target.is_file() { let temporary = target.with_extension("tmp"); std::fs::copy(&source, &temporary)?; std::fs::rename(&temporary, &target)?; let day = std::time::Duration::from_secs(86400); for entry in std::fs::read_dir(&folder).into_iter().flatten().flatten() { let path = entry.path(); let same_stem = path.file_name().map(|name| name.to_string_lossy().starts_with(&format!("{stem}-"))).unwrap_or(false); let stale = entry.metadata().and_then(|metadata| metadata.modified()).ok().and_then(|time| time.elapsed().ok()).map(|age| age > day).unwrap_or(false); if same_stem && stale && path != target { let _ = std::fs::remove_file(&path); } } } Ok(target) }; std::rc::Rc::<str>::from(copy().map(|path| path.to_string_lossy().into_owned()).unwrap_or_default()) };
    v0.clone()
}
fn request_name_28(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".start"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    v2.clone()
}
fn host_state_of_29(mut v0: u64, mut v1: bool) -> US4 {
    let mut v2: bool = v0 != 0u64;
    let mut v3: bool = v2 && v1;
    if v3 {
        US4::US4_HostRunning(v0)
    } else {
        US4::US4_HostMissing
    }
}
fn args_mark_30() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(char::from(31u8).to_string());
    v0.clone()
}
fn detached_started_31(mut v0: Rc<str>, mut v1: Rc<str>) -> u64 {
    let mut v2: u64 = { #[cfg(windows)] let pid = { use std::os::windows::ffi::OsStrExt; #[repr(C)] struct StartupInfo { cb: u32, reserved: *mut u16, desktop: *mut u16, title: *mut u16, x: u32, y: u32, x_size: u32, y_size: u32, x_chars: u32, y_chars: u32, fill: u32, flags: u32, show: u16, reserved2_len: u16, reserved2: *mut u8, input: *mut std::ffi::c_void, output: *mut std::ffi::c_void, error: *mut std::ffi::c_void } #[repr(C)] struct StartupInfoEx { startup: StartupInfo, attributes: *mut std::ffi::c_void } #[repr(C)] struct ProcessInfo { process: *mut std::ffi::c_void, thread: *mut std::ffi::c_void, pid: u32, tid: u32 } unsafe extern "system" { fn CreateProcessW(app: *const u16, line: *mut u16, process_attributes: *mut std::ffi::c_void, thread_attributes: *mut std::ffi::c_void, inherit: i32, flags: u32, environment: *mut std::ffi::c_void, cwd: *const u16, startup: *mut StartupInfo, info: *mut ProcessInfo) -> i32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void; fn InitializeProcThreadAttributeList(list: *mut std::ffi::c_void, count: u32, flags: u32, size: *mut usize) -> i32; fn UpdateProcThreadAttribute(list: *mut std::ffi::c_void, flags: u32, attribute: usize, value: *mut std::ffi::c_void, size: usize, previous: *mut std::ffi::c_void, returned: *mut usize) -> i32; fn DeleteProcThreadAttributeList(list: *mut std::ffi::c_void); } #[link(name = "user32")] unsafe extern "system" { fn GetShellWindow() -> *mut std::ffi::c_void; fn GetWindowThreadProcessId(window: *mut std::ffi::c_void, pid: *mut u32) -> u32; } let quote_char = char::from(34u8); let quoted = |arg: &str| if arg.is_empty() || arg.contains(char::from(32u8)) || arg.contains(quote_char) { format!("{quote_char}{}{quote_char}", arg.replace(quote_char, &format!("{}{quote_char}", char::from(92u8)))) } else { arg.to_owned() }; let line = std::iter::once(quoted(&*v0)).chain((v1).split(char::from(31u8)).map(quoted)).collect::<Vec<_>>().join(" ").encode_utf16().chain(std::iter::once(0u16)).collect::<Vec<u16>>(); let cwd = std::env::temp_dir().as_os_str().encode_wide().chain(std::iter::once(0u16)).collect::<Vec<u16>>(); let shell_window = unsafe { GetShellWindow() }; let mut shell_pid = 0u32; if !shell_window.is_null() { unsafe { GetWindowThreadProcessId(shell_window, &mut shell_pid) }; } let parent = if shell_pid == 0 { std::ptr::null_mut() } else { unsafe { OpenProcess(0x0080, 0, shell_pid) } }; let mut list_size = 0usize; unsafe { InitializeProcThreadAttributeList(std::ptr::null_mut(), 1, 0, &mut list_size) }; let mut list = vec![0u8; list_size.max(64)]; let list_ptr = list.as_mut_ptr() as *mut std::ffi::c_void; let mut parent_value = parent; let attributed = !parent.is_null() && unsafe { InitializeProcThreadAttributeList(list_ptr, 1, 0, &mut list_size) } != 0 && unsafe { UpdateProcThreadAttribute(list_ptr, 0, 0x0002_0000, &mut parent_value as *mut *mut std::ffi::c_void as *mut std::ffi::c_void, std::mem::size_of::<*mut std::ffi::c_void>(), std::ptr::null_mut(), std::ptr::null_mut()) } != 0; let start = |flags: u32, with_parent: bool| -> u64 { let mut line = line.clone(); let mut startup: StartupInfoEx = unsafe { std::mem::zeroed() }; startup.startup.cb = (if with_parent { std::mem::size_of::<StartupInfoEx>() } else { std::mem::size_of::<StartupInfo>() }) as u32; if with_parent { startup.attributes = list_ptr; } let mut info: ProcessInfo = unsafe { std::mem::zeroed() }; let flags = if with_parent { flags | 0x0008_0000 } else { flags }; if unsafe { CreateProcessW(std::ptr::null(), line.as_mut_ptr(), std::ptr::null_mut(), std::ptr::null_mut(), 0, flags, std::ptr::null_mut(), cwd.as_ptr(), &mut startup as *mut StartupInfoEx as *mut StartupInfo, &mut info) } == 0 { 0 } else { unsafe { CloseHandle(info.thread); CloseHandle(info.process); } info.pid as u64 } }; let tries = [(0x0100_0228u32, true), (0x0000_0228u32, true), (0x0100_0228u32, false), (0x0000_0228u32, false)]; let pid = tries.iter().filter(|(_, with_parent)| attributed || !*with_parent).map(|(flags, with_parent)| start(*flags, *with_parent)).find(|pid| *pid != 0).unwrap_or(0); if attributed { unsafe { DeleteProcThreadAttributeList(list_ptr) }; } if !parent.is_null() { unsafe { CloseHandle(parent) }; } pid }; #[cfg(unix)] let pid = { use std::os::unix::process::CommandExt; let mut command = std::process::Command::new(&*v0); command.args((v1).split(char::from(31u8))).current_dir(std::env::temp_dir()).stdin(std::process::Stdio::null()).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()).process_group(0); command.spawn().map(|child| child.id() as u64).unwrap_or(0) }; pid };
    v2
}
fn job_directory_34(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = jobs_root_5();
    joined_3(v1.clone(), v0.clone())
}
fn job_file_33(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = job_directory_34(v0.clone());
    joined_3(v2.clone(), v1.clone())
}
fn supervisor_wait_of_35(mut v0: u64, mut v1: u64, mut v2: u64) -> US6 {
    let mut v3: bool = v0 != 0u64;
    if v3 {
        let mut v4: US5 = US5::US5_SupervisorUp(v0);
        US6::US6_Some(v4.clone())
    } else {
        let mut v6: bool = v1 >= v2;
        if v6 {
            let mut v7: US5 = US5::US5_SupervisorLate;
            US6::US6_Some(v7.clone())
        } else {
            US6::US6_None
        }
    }
}
fn supervisor_awaited_32(mut v0: Rc<str>, mut v1: u64) -> US5 {
    loop {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("supervisor"); } LIT.with(|lit| lit.clone()) };
        let mut v3: Rc<str> = job_file_33(v0.clone(), v2.clone());
        let mut v4: Rc<str> = text_or_empty_7(v3.clone());
        let mut v5: u64 = decimal_u64_18(v4.clone());
        let mut v6: u64 = now_ms_12();
        let mut v7: US6 = supervisor_wait_of_35(v5, v6, v1);
        match &v7 {
            US6::US6_None => {
                let mut v9: u64 = 50u64;
                std::thread::sleep(std::time::Duration::from_millis(v9));
                (v0, v1) = (v0.clone(), v1);
                continue;
            }
            US6::US6_Some(v8) => {
                let mut v8: US5 = v8.clone();
                return v8.clone();
            }
        }
    }
}
fn supervisor_requested_26(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = jobs_root_5();
    let mut v2: Rc<str> = supervisor_executable_27();
    let mut v3: Rc<str> = requests_directory_2();
    let mut v4: Rc<str> = std::rc::Rc::<str>::from(std::fs::create_dir_all(&*v3).err().map(|error| format!("create {}: {error}", &*v3)).unwrap_or_default());
    let mut v5: Rc<str> = request_name_28(v0.clone());
    let mut v6: Rc<str> = joined_3(v3.clone(), v5.clone());
    let mut v7: Rc<str> = text_written_24(v6.clone(), v2.clone());
    let mut v8: Rc<str> = host_pid_file_17();
    let mut v9: Rc<str> = text_or_empty_7(v8.clone());
    let mut v10: u64 = decimal_u64_18(v9.clone());
    let mut v11: bool = process_alive_21(v10);
    let mut v12: US4 = host_state_of_29(v10, v11);
    let mut v21: u64 = match &v12 {
        US4::US4_HostMissing => {
            let mut v14: Rc<str> = args_mark_30();
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
            let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v14));
            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host"); } LIT.with(|lit| lit.clone()) };
            let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v17));
            detached_started_31(v2.clone(), v18.clone())
        }
        US4::US4_HostRunning(v13) => {
            let mut v13: u64 = *v13;
            0u64
        }
    };
    let mut v22: u64 = now_ms_12();
    let mut v23: u64 = v22.wrapping_add(5000u64);
    let mut v24: US5 = supervisor_awaited_32(v0.clone(), v23);
    match &v24 {
        US5::US5_SupervisorLate => {
            let mut v27: bool = request_removed_8(v6.clone());
            if v27 {
                let mut v28: Rc<str> = args_mark_30();
                let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
                let mut v30: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v28));
                let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("supervise"); } LIT.with(|lit| lit.clone()) };
                let mut v32: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v31));
                let mut v33: Rc<str> = args_mark_30();
                let mut v34: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v33));
                let mut v35: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v0));
                let mut v36: u64 = detached_started_31(v2.clone(), v35.clone());
                let mut v37: bool = v36 == 0u64;
                if v37 {
                    let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: neither the job host nor a detached start launched the supervisor"); } LIT.with(|lit| lit.clone()) };
                    v38.clone()
                } else {
                    decimal_text_23(v36)
                }
            } else {
                let mut v41: u64 = now_ms_12();
                let mut v42: u64 = v41.wrapping_add(60000u64);
                let mut v43: US5 = supervisor_awaited_32(v0.clone(), v42);
                match &v43 {
                    US5::US5_SupervisorLate => {
                        let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: the job host took the request but the supervisor never recorded its pid"); } LIT.with(|lit| lit.clone()) };
                        v46.clone()
                    }
                    US5::US5_SupervisorUp(v44) => {
                        let mut v44: u64 = *v44;
                        decimal_text_23(v44)
                    }
                }
            }
        }
        US5::US5_SupervisorUp(v25) => {
            let mut v25: u64 = *v25;
            decimal_text_23(v25)
        }
    }
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        supervisor_requested_26(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_job_host_run() -> i32 {
    closure0()()
}
pub fn eoie_job_host_request(v0: &str) -> Rc<str> {
    closure2()(Rc::<str>::from(v0))
}
