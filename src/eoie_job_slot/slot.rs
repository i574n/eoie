#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_TreeStopped,
    US0_SurvivorsLeft(u64),
}
#[derive(Clone)]
enum US1 {
    US1_Some(u64),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_JobFinished(i32),
    US2_JobTimedOut,
    US2_JobStopped,
    US2_JobSpawnFailed(Rc<str>),
    US2_JobSkipped(Rc<str>),
    US2_JobLost,
    US2_JobMemoryCapped,
    US2_JobExpiredInQueue,
}
fn now_ms_2() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn joined_4(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2.clone()
}
fn jobs_root_6() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("EOIE_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("eoie").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("eoie").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".eoie-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn job_directory_5(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = jobs_root_6();
    joined_4(v1.clone(), v0.clone())
}
fn job_file_3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = job_directory_5(v0.clone());
    joined_4(v2.clone(), v1.clone())
}
fn decimal_text_7(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1.clone()
}
fn text_written_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2.clone()
}
fn file_exists_9(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.is_file();
    v8
}
fn file_removed_10(mut v0: Rc<str>) -> bool {
    let mut v1: bool = std::fs::remove_file(&*v0).is_ok();
    v1
}
fn text_or_empty_12(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1.clone()
}
fn method14(mut v0: Option<u64>) -> Option<u64> {
    v0.clone()
}
fn closure1() -> Rc<dyn Fn((u64)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US1> = Rc::new(move |mut v0: (u64)| -> US1 {
        let mut v1: u64 = (v0);
        US1::US1_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_13(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method14(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US1> = closure1();
    let mut v4: Option<US1> = v2.map(|x| v3(x));
    let mut v5: US1 = US1::US1_None;
    let mut v6: US1 = v4.unwrap_or(v5);
    match &v6 {
        US1::US1_None => {
            0u64
        }
        US1::US1_Some(v7) => {
            let mut v7: u64 = *v7;
            v7
        }
    }
}
fn slot_deadline_passed_11(mut v0: Rc<str>, mut v1: u64) -> bool {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot_by_ms"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = job_file_3(v0.clone(), v2.clone());
    let mut v4: Rc<str> = text_or_empty_12(v3.clone());
    let mut v5: u64 = decimal_u64_13(v4.clone());
    let mut v6: bool = v5 != 0u64;
    if v6 {
        let mut v7: bool = v1 > v5;
        v7
    } else {
        false
    }
}
fn settled_word_15(mut v0: US2) -> Rc<str> {
    match &v0 {
        US2::US2_JobExpiredInQueue => {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expired-in-queue"); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US2::US2_JobFinished(v1) => {
            let mut v1: i32 = *v1;
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished"); } LIT.with(|lit| lit.clone()) };
            v2.clone()
        }
        US2::US2_JobLost => {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lost"); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        }
        US2::US2_JobMemoryCapped => {
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory-capped"); } LIT.with(|lit| lit.clone()) };
            v10.clone()
        }
        US2::US2_JobSkipped(v7) => {
            let mut v7: Rc<str> = v7.clone();
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("skipped"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US2::US2_JobSpawnFailed(v5) => {
            let mut v5: Rc<str> = v5.clone();
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spawn-failed"); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        }
        US2::US2_JobStopped => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US2::US2_JobTimedOut => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed-out"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
    }
}
fn tickets_sorted_16(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from({ let mut names = std::fs::read_dir(&*v0).into_iter().flatten().flatten().map(|entry| entry.file_name().to_string_lossy().into_owned()).filter(|name| name.ends_with(".t")).collect::<Vec<_>>(); names.sort(); names.join("
") }.as_str());
    v1.clone()
}
fn first_row_18(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().next().unwrap_or(""));
    v1.clone()
}
fn ticket_pid_19(mut v0: Rc<str>) -> u64 {
    let mut v1: u64 = (v0).trim_end_matches(".t").split(char::from(45u8)).nth(1).and_then(|pid| pid.parse::<u64>().ok()).unwrap_or(0);
    v1
}
fn process_alive_20(mut v0: u64) -> bool {
    let mut v1: bool = { #[cfg(windows)] fn alive(pid: u64) -> bool { unsafe extern "system" { fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void; fn WaitForSingleObject(handle: *mut std::ffi::c_void, ms: u32) -> u32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; fn GetLastError() -> u32; } if pid == 0 { return true; } unsafe { let handle = OpenProcess(0x0010_0000, 0, pid as u32); if handle.is_null() { return GetLastError() == 5; } let running = WaitForSingleObject(handle, 0) == 258; CloseHandle(handle); running } } #[cfg(not(windows))] fn alive(pid: u64) -> bool { pid == 0 || std::os::unix::process::parent_id() as u64 == pid } alive(v0) };
    v1
}
fn other_rows_21(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().skip(1).collect::<Vec<_>>().join("
"));
    v1.clone()
}
fn first_live_ticket_17(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v1 {
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v2.clone();
        } else {
            let mut v3: Rc<str> = first_row_18(v0.clone());
            let mut v4: u64 = ticket_pid_19(v3.clone());
            let mut v5: bool = process_alive_20(v4);
            if v5 {
                return v3.clone();
            } else {
                let mut v6: Rc<str> = other_rows_21(v0.clone());
                v0 = v6.clone();
                continue;
            }
        }
    }
}
fn free_mb_22() -> u64 {
    let mut v0: u64 = { #[cfg(windows)] fn free() -> u64 { #[repr(C)] struct Status { length: u32, load: u32, total: u64, available: u64, total_page: u64, available_page: u64, total_virtual: u64, available_virtual: u64, extended: u64 } unsafe extern "system" { fn GlobalMemoryStatusEx(status: *mut Status) -> i32; } unsafe { let mut status: Status = std::mem::zeroed(); status.length = std::mem::size_of::<Status>() as u32; if GlobalMemoryStatusEx(&mut status) == 0 { u64::MAX } else { status.available / 1048576 } } } #[cfg(not(windows))] fn free() -> u64 { std::fs::read_to_string("/proc/meminfo").ok().and_then(|text| text.lines().find(|line| line.starts_with("MemAvailable:")).and_then(|line| line.split_whitespace().nth(1)).and_then(|kb| kb.parse::<u64>().ok())).map(|kb| kb / 1024).unwrap_or(u64::MAX) } std::env::var("EOIE_WINDOWS_FREE_MB").ok().and_then(|text| text.trim().parse::<u64>().ok()).unwrap_or_else(free) };
    v0
}
fn slot_free_23(mut v0: u64) -> bool {
    let mut v1: bool = { #[cfg(windows)] fn probe(name: &str) -> bool { unsafe extern "system" { fn OpenMutexW(access: u32, inherit: i32, name: *const u16) -> *mut std::ffi::c_void; fn WaitForSingleObject(handle: *mut std::ffi::c_void, ms: u32) -> u32; fn ReleaseMutex(handle: *mut std::ffi::c_void) -> i32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; } let name = name.encode_utf16().chain(std::iter::once(0u16)).collect::<Vec<u16>>(); unsafe { let handle = OpenMutexW(0x0010_0001, 0, name.as_ptr()); if handle.is_null() { return true; } let state = WaitForSingleObject(handle, 0); let taken = state == 0 || state == 0x80; if taken { ReleaseMutex(handle); } CloseHandle(handle); taken } } #[cfg(not(windows))] fn probe(_name: &str) -> bool { true } let base = std::env::var("EOIE_HEAVY_MUTEX").unwrap_or_else(|_| format!("Global{}spiral-heavy-job", char::from(92u8))); probe(&base) || (v0 > 3072 && probe(&format!("{base}-2"))) };
    v1
}
fn ticket_adopted_24(mut v0: Rc<str>) -> () {
    unsafe { std::env::set_var("SPIRAL_HEAVY_TICKET", &*v0); };
    ()
}
fn slot_loop_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64) -> i32 {
    loop {
        let mut v4: u64 = now_ms_2();
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = job_file_3(v0.clone(), v5.clone());
        let mut v7: Rc<str> = decimal_text_7(v4);
        let mut v8: Rc<str> = text_written_8(v6.clone(), v7.clone());
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop"); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = job_file_3(v0.clone(), v9.clone());
        let mut v11: bool = file_exists_9(v10.clone());
        if v11 {
            let mut v12: Rc<str> = joined_4(v1.clone(), v2.clone());
            let mut v13: bool = file_removed_10(v12.clone());
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
            let mut v15: Rc<str> = job_file_3(v0.clone(), v14.clone());
            let mut v16: u64 = 0u64;
            let mut v17: bool = v16 == 0u64;
            let mut v20: US0 = if v17 {
                US0::US0_TreeStopped
            } else {
                US0::US0_SurvivorsLeft(v16)
            };
            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped"); } LIT.with(|lit| lit.clone()) };
            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped on request"); } LIT.with(|lit| lit.clone()) };
            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
            let mut v31: Rc<str> = match &v20 {
                US0::US0_SurvivorsLeft(v25) => {
                    let mut v25: u64 = *v25;
                    let mut v26: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
                    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" processes left alive"); } LIT.with(|lit| lit.clone()) };
                    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v29: Rc<str> = std::rc::Rc::<str>::from([&*v26, &*v27, &*v28, &*v28, &*v28].concat());
                    v29.clone()
                }
                US0::US0_TreeStopped => {
                    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("process tree stopped"); } LIT.with(|lit| lit.clone()) };
                    v24.clone()
                }
            };
            let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v33: Rc<str> = std::rc::Rc::<str>::from([&*v22, &*v23, &*v31, &*v32, &*v32].concat());
            let mut v34: Rc<str> = std::rc::Rc::<str>::from((143i32).to_string());
            let mut v35: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
            let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v38: Rc<str> = std::rc::Rc::<str>::from([&*v21, &*v36, &*v33, &*v37, &*v32].concat());
            let mut v39: Rc<str> = std::rc::Rc::<str>::from([&*v34, &*v36, &*v35, &*v36, &*v38].concat());
            let mut v40: Rc<str> = text_written_8(v15.clone(), v39.clone());
            return 143i32;
        } else {
            let mut v41: bool = slot_deadline_passed_11(v0.clone(), v4);
            if v41 {
                let mut v42: Rc<str> = joined_4(v1.clone(), v2.clone());
                let mut v43: bool = file_removed_10(v42.clone());
                let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
                let mut v45: Rc<str> = job_file_3(v0.clone(), v44.clone());
                let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no heavy slot within --slot-within; the job never started"); } LIT.with(|lit| lit.clone()) };
                let mut v47: US2 = US2::US2_JobExpiredInQueue;
                let mut v48: Rc<str> = settled_word_15(v47.clone());
                let mut v49: Rc<str> = std::rc::Rc::<str>::from((75i32).to_string());
                let mut v50: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
                let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v54: Rc<str> = std::rc::Rc::<str>::from([&*v48, &*v51, &*v46, &*v52, &*v53].concat());
                let mut v55: Rc<str> = std::rc::Rc::<str>::from([&*v49, &*v51, &*v50, &*v51, &*v54].concat());
                let mut v56: Rc<str> = text_written_8(v45.clone(), v55.clone());
                return 75i32;
            } else {
                let mut v57: Rc<str> = tickets_sorted_16(v1.clone());
                let mut v58: Rc<str> = first_live_ticket_17(v57.clone());
                let mut v59: bool = v58.clone() == v2.clone();
                let mut v62: bool = if v59 {
                    let mut v60: u64 = free_mb_22();
                    slot_free_23(v60)
                } else {
                    false
                };
                if v62 {
                    ticket_adopted_24(v2.clone());
                    let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot"); } LIT.with(|lit| lit.clone()) };
                    let mut v64: Rc<str> = job_file_3(v0.clone(), v63.clone());
                    let mut v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ready ticket="); } LIT.with(|lit| lit.clone()) };
                    let mut v66: Rc<str> = Rc::<str>::from(format!("{}{}", v65, v2));
                    let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" waited_ms="); } LIT.with(|lit| lit.clone()) };
                    let mut v68: Rc<str> = Rc::<str>::from(format!("{}{}", v66, v67));
                    let mut v69: u64 = v4.wrapping_sub(v3);
                    let mut v70: Rc<str> = decimal_text_7(v69);
                    let mut v71: Rc<str> = Rc::<str>::from(format!("{}{}", v68, v70));
                    let mut v72: Rc<str> = text_written_8(v64.clone(), v71.clone());
                    return 0i32;
                } else {
                    let mut v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot"); } LIT.with(|lit| lit.clone()) };
                    let mut v74: Rc<str> = job_file_3(v0.clone(), v73.clone());
                    let mut v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("waiting ticket="); } LIT.with(|lit| lit.clone()) };
                    let mut v76: Rc<str> = Rc::<str>::from(format!("{}{}", v75, v2));
                    let mut v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" first="); } LIT.with(|lit| lit.clone()) };
                    let mut v78: Rc<str> = Rc::<str>::from(format!("{}{}", v76, v77));
                    let mut v79: Rc<str> = tickets_sorted_16(v1.clone());
                    let mut v80: Rc<str> = first_live_ticket_17(v79.clone());
                    let mut v81: Rc<str> = Rc::<str>::from(format!("{}{}", v78, v80));
                    let mut v82: Rc<str> = text_written_8(v74.clone(), v81.clone());
                    let mut v83: u64 = 5000u64;
                    std::thread::sleep(std::time::Duration::from_millis(v83));
                    (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v2.clone(), v3);
                    continue;
                }
            }
        }
    }
}
fn heavy_queue_25() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("EOIE_HEAVY_QUEUE").map(|queue| std::path::PathBuf::from(queue).to_string_lossy().into_owned()).or_else(|| std::env::var_os("LOCALAPPDATA").or_else(|| std::env::var_os("HOME")).map(|base| std::path::PathBuf::from(base).join("spiral-bin").join("heavy-queue").to_string_lossy().into_owned())).unwrap_or_default().as_str());
    v0.clone()
}
fn ticket_created_26(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from({ let ticks = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_nanos() as u64 / 100).unwrap_or(0) + 621355968000000000u64; let name = format!("{:020}-{}.t", ticks, std::process::id()); let _ = std::fs::create_dir_all(&*v0); match std::fs::write(std::path::Path::new(&*v0).join(&name), b"") { Ok(()) => name, Err(_) => String::new() } }.as_str());
    v1.clone()
}
fn slot_gate_0(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = heavy_queue_25();
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v3.clone()
    } else {
        ticket_created_26(v1.clone())
    };
    let mut v6: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v6 {
        0i32
    } else {
        let mut v7: u64 = now_ms_2();
        slot_loop_1(v0.clone(), v1.clone(), v5.clone(), v7)
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        slot_gate_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_job_slot_gate(v0: &str) -> i32 {
    closure0()(Rc::<str>::from(v0))
}
