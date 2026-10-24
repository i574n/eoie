#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Some(u64),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_LiveGui,
    US1_TestSuite,
    US1_HeavyBuild,
    US1_Compile,
    US1_CompilerInstall,
    US1_TreeRead,
    US1_Bench,
    US1_CompileHold,
}
#[derive(Clone)]
enum US2 {
    US2_ProcessHolder,
    US2_LeaseHolder,
}
#[derive(Clone)]
enum US3 {
    US3_Held,
    US3_Waiting,
}
#[derive(Clone)]
enum US4 {
    US4_TreeStopped,
    US4_SurvivorsLeft(u64),
}
#[derive(Clone)]
enum US5 {
    US5_JobFinished(i32),
    US5_JobTimedOut,
    US5_JobStopped,
    US5_JobSpawnFailed(Rc<str>),
    US5_JobSkipped(Rc<str>),
    US5_JobLost,
    US5_JobMemoryCapped,
    US5_JobExpiredInQueue,
}
fn joined_3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2.clone()
}
fn jobs_root_5() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("EOIE_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("eoie").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("eoie").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".eoie-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn job_directory_4(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = jobs_root_5();
    joined_3(v1.clone(), v0.clone())
}
fn job_file_2(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = job_directory_4(v0.clone());
    joined_3(v2.clone(), v1.clone())
}
fn text_or_empty_6(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1.clone()
}
fn method8(mut v0: Option<u64>) -> Option<u64> {
    v0.clone()
}
fn closure1() -> Rc<dyn Fn((u64)) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US0> = Rc::new(move |mut v0: (u64)| -> US0 {
        let mut v1: u64 = (v0);
        US0::US0_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_7(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method8(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US0> = closure1();
    let mut v4: Option<US0> = v2.map(|x| v3(x));
    let mut v5: US0 = US0::US0_None;
    let mut v6: US0 = v4.unwrap_or(v5);
    match &v6 {
        US0::US0_None => {
            0u64
        }
        US0::US0_Some(v7) => {
            let mut v7: u64 = *v7;
            v7
        }
    }
}
fn now_ms_9() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn number_text_10(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1.clone()
}
fn process_number_11() -> u64 {
    let mut v0: u64 = std::process::id() as u64;
    v0
}
fn file_exists_13(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.is_file();
    v8
}
fn windows_root_14() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("EOIE_WINDOWS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("eoie").join("windows"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("eoie").join("windows"))).unwrap_or_else(|| std::path::PathBuf::from(".eoie-windows")).to_string_lossy().as_ref());
    v0.clone()
}
fn window_file_15(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".window"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v2, &*v3, &*v3, &*v3].concat());
    joined_3(v0.clone(), v4.clone())
}
fn file_removed_16(mut v0: Rc<str>) -> bool {
    let mut v1: bool = std::fs::remove_file(&*v0).is_ok();
    v1
}
fn decimal_text_17(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1.clone()
}
fn text_written_18(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2.clone()
}
fn lock_taken_20(mut v0: Rc<str>) -> bool {
    let mut v1: bool = { let lock = std::path::Path::new(&*v0).join("lock"); let _ = std::fs::create_dir_all(&*v0); let mut taken = false; for _ in 0..400 { if std::fs::create_dir(&lock).is_ok() { taken = true; break; } let stale = std::fs::metadata(&lock).and_then(|meta| meta.modified()).ok().and_then(|time| time.elapsed().ok()).map(|age| age.as_secs() > 10).unwrap_or(false); if stale { let _ = std::fs::remove_dir(&lock); } else { std::thread::sleep(std::time::Duration::from_millis(50)); } } taken };
    v1
}
fn stored_rows_21(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { let mut rows = std::fs::read_dir(&*v0).into_iter().flatten().flatten().map(|entry| entry.path()).filter(|path| path.extension().map(|ext| ext == "window").unwrap_or(false)).filter_map(|path| std::fs::read_to_string(&path).ok()).map(|text| text.trim().to_owned()).filter(|text| !text.is_empty()).collect::<Vec<_>>(); rows.sort_by_key(|row| row.split(char::from(9u8)).nth(4).and_then(|start| start.parse::<u64>().ok()).unwrap_or(0)); std::rc::Rc::<str>::from(rows.join("
")) };
    v1.clone()
}
fn first_row_23(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().next().unwrap_or(""));
    v1.clone()
}
fn tab_field_25(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(v1 as usize).unwrap_or(""));
    v2.clone()
}
fn decimal_of_26(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method8(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US0> = closure1();
    let mut v4: Option<US0> = v2.map(|x| v3(x));
    let mut v5: US0 = US0::US0_None;
    let mut v6: US0 = v4.unwrap_or(v5);
    match &v6 {
        US0::US0_None => {
            0u64
        }
        US0::US0_Some(v7) => {
            let mut v7: u64 = *v7;
            v7
        }
    }
}
fn row_expired_24(mut v0: Rc<str>, mut v1: u64) -> bool {
    let mut v2: i32 = 5i32;
    let mut v3: Rc<str> = tab_field_25(v0.clone(), v2);
    let mut v4: u64 = decimal_of_26(v3.clone());
    let mut v5: bool = v4 < v1;
    v5
}
fn holder_of_28(mut v0: Rc<str>) -> US2 {
    let mut v1: i32 = 3i32;
    let mut v2: Rc<str> = tab_field_25(v0.clone(), v1);
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(eoie job "); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = v2.contains(&*v3);
    let mut v9: bool = if v4 {
        true
    } else {
        let mut v5: i32 = 0i32;
        let mut v6: Rc<str> = tab_field_25(v0.clone(), v5);
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-run"); } LIT.with(|lit| lit.clone()) };
        let mut v8: bool = v6.ends_with(&*v7);
        v8
    };
    if v9 {
        US2::US2_ProcessHolder
    } else {
        US2::US2_LeaseHolder
    }
}
fn window_pid_29(mut v0: Rc<str>) -> u64 {
    let mut v1: u64 = (v0).split(char::from(45u8)).nth(2).and_then(|pid| pid.parse::<u64>().ok()).unwrap_or(0);
    v1
}
fn process_alive_30(mut v0: u64) -> bool {
    let mut v1: bool = { #[cfg(windows)] fn alive(pid: u64) -> bool { unsafe extern "system" { fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void; fn WaitForSingleObject(handle: *mut std::ffi::c_void, ms: u32) -> u32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; fn GetLastError() -> u32; } if pid == 0 { return true; } unsafe { let handle = OpenProcess(0x0010_0000, 0, pid as u32); if handle.is_null() { return GetLastError() == 5; } let running = WaitForSingleObject(handle, 0) == 258; CloseHandle(handle); running } } #[cfg(not(windows))] fn alive(pid: u64) -> bool { pid == 0 || std::os::unix::process::parent_id() as u64 == pid } alive(v0) };
    v1
}
fn orphaned_window_27(mut v0: Rc<str>) -> bool {
    let mut v1: US2 = holder_of_28(v0.clone());
    match &v1 {
        US2::US2_LeaseHolder => {
            false
        }
        US2::US2_ProcessHolder => {
            let mut v2: i32 = 0i32;
            let mut v3: Rc<str> = tab_field_25(v0.clone(), v2);
            let mut v4: u64 = window_pid_29(v3.clone());
            let mut v5: bool = process_alive_30(v4);
            let mut v6: bool = v5 == false;
            v6
        }
    }
}
fn other_rows_31(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().skip(1).collect::<Vec<_>>().join("
"));
    v1.clone()
}
fn live_rows_22(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let mut v4: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v4 {
            return v3.clone();
        } else {
            let mut v5: Rc<str> = first_row_23(v1.clone());
            let mut v6: bool = row_expired_24(v5.clone(), v2);
            let mut v8: bool = if v6 {
                true
            } else {
                orphaned_window_27(v5.clone())
            };
            if v8 {
                let mut v9: i32 = 0i32;
                let mut v10: Rc<str> = tab_field_25(v5.clone(), v9);
                let mut v11: Rc<str> = window_file_15(v0.clone(), v10.clone());
                let mut v12: bool = file_removed_16(v11.clone());
                let mut v13: Rc<str> = other_rows_31(v1.clone());
                (v0, v1, v2, v3) = (v0.clone(), v13.clone(), v2, v3.clone());
                continue;
            } else {
                let mut v15: Rc<str> = other_rows_31(v1.clone());
                let mut v16: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v20: Rc<str> = if v16 {
                    v5.clone()
                } else {
                    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v19: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v17, &*v5, &*v18, &*v18].concat());
                    v19.clone()
                };
                (v0, v1, v2, v3) = (v0.clone(), v15.clone(), v2, v20.clone());
                continue;
            }
        }
    }
}
fn free_mb_32() -> u64 {
    let mut v0: u64 = { #[cfg(windows)] fn free() -> u64 { #[repr(C)] struct Status { length: u32, load: u32, total: u64, available: u64, total_page: u64, available_page: u64, total_virtual: u64, available_virtual: u64, extended: u64 } unsafe extern "system" { fn GlobalMemoryStatusEx(status: *mut Status) -> i32; } unsafe { let mut status: Status = std::mem::zeroed(); status.length = std::mem::size_of::<Status>() as u32; if GlobalMemoryStatusEx(&mut status) == 0 { u64::MAX } else { status.available / 1048576 } } } #[cfg(not(windows))] fn free() -> u64 { std::fs::read_to_string("/proc/meminfo").ok().and_then(|text| text.lines().find(|line| line.starts_with("MemAvailable:")).and_then(|line| line.split_whitespace().nth(1)).and_then(|kb| kb.parse::<u64>().ok())).map(|kb| kb / 1024).unwrap_or(u64::MAX) } std::env::var("EOIE_WINDOWS_FREE_MB").ok().and_then(|text| text.trim().parse::<u64>().ok()).unwrap_or_else(free) };
    v0
}
fn class_word_34(mut v0: US1) -> Rc<str> {
    match &v0 {
        US1::US1_Bench => {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bench"); } LIT.with(|lit| lit.clone()) };
            v7.clone()
        }
        US1::US1_Compile => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US1::US1_CompileHold => {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile-hold"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US1::US1_CompilerInstall => {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compiler-install"); } LIT.with(|lit| lit.clone()) };
            v5.clone()
        }
        US1::US1_HeavyBuild => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-build"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
        US1::US1_LiveGui => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("live-gui"); } LIT.with(|lit| lit.clone()) };
            v1.clone()
        }
        US1::US1_TestSuite => {
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test-suite"); } LIT.with(|lit| lit.clone()) };
            v2.clone()
        }
        US1::US1_TreeRead => {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tree-read"); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        }
    }
}
fn verdict_text_33(mut v0: US1, mut v1: Rc<str>, mut v2: u64, mut v3: u64, mut v4: Rc<str>, mut v5: u64, mut v6: u64) -> Rc<str> {
    let mut v7: Rc<str> = class_word_34(v0.clone());
    let mut v8: Rc<str> = eoie_window_admission::eoie_window_verdict(&*v7, &*v1, v2, v3, &*v4, v5, v6);
    v8.clone()
}
fn one_line_37(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.chars().map(|c| if c.is_control() { char::from(32u8) } else { c }).collect::<String>());
    v1.clone()
}
fn row_text_36(mut v0: Rc<str>, mut v1: US1, mut v2: US3, mut v3: Rc<str>, mut v4: u64, mut v5: u64) -> Rc<str> {
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = class_word_34(v1.clone());
    let mut v11: Rc<str> = match &v2 {
        US3::US3_Held => {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US3::US3_Waiting => {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("waiting"); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        }
    };
    let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v6, &*v7, &*v6, &*v11].concat());
    let mut v13: Rc<str> = one_line_37(v3.clone());
    let mut v14: Rc<str> = number_text_10(v4);
    let mut v15: Rc<str> = number_text_10(v5);
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v6, &*v15, &*v16, &*v16].concat());
    let mut v18: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v6, &*v13, &*v6, &*v17].concat());
    v18.clone()
}
fn request_row_35(mut v0: US1, mut v1: u64, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: u64, mut v6: US3, mut v7: u64) -> Rc<str> {
    row_text_36(v3.clone(), v0.clone(), v6.clone(), v2.clone(), v4, v7)
}
fn lock_released_38(mut v0: Rc<str>) -> () {
    { let _ = std::fs::remove_dir(std::path::Path::new(&*v0).join("lock")); };
    ()
}
fn window_blocker_19(mut v0: US1, mut v1: u64, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: u64) -> Rc<str> {
    let mut v6: Rc<str> = windows_root_14();
    let mut v7: bool = lock_taken_20(v6.clone());
    let mut v8: bool = v7 == false;
    if v8 {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("reason=registry-lock-busy"); } LIT.with(|lit| lit.clone()) };
        v9.clone()
    } else {
        let mut v10: u64 = now_ms_9();
        let mut v11: Rc<str> = stored_rows_21(v6.clone());
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v13: Rc<str> = live_rows_22(v6.clone(), v11.clone(), v10, v12.clone());
        let mut v14: u64 = free_mb_32();
        let mut v15: Rc<str> = verdict_text_33(v0.clone(), v3.clone(), v4, v1, v13.clone(), v14, v10);
        let mut v16: bool = v15.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v16 {
            let mut v17: Rc<str> = window_file_15(v6.clone(), v3.clone());
            let mut v18: US3 = US3::US3_Held;
            let mut v19: u64 = v1.wrapping_mul(60000u64);
            let mut v20: u64 = v10.wrapping_add(v19);
            let mut v21: Rc<str> = request_row_35(v0.clone(), v1, v2.clone(), v3.clone(), v10, v5, v18.clone(), v20);
            let mut v22: Rc<str> = text_written_18(v17.clone(), v21.clone());
            lock_released_38(v6.clone());
            v12.clone()
        } else {
            let mut v23: Rc<str> = window_file_15(v6.clone(), v3.clone());
            let mut v24: US3 = US3::US3_Waiting;
            let mut v25: u64 = v10.wrapping_add(30000u64);
            let mut v26: Rc<str> = request_row_35(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v24.clone(), v25);
            let mut v27: Rc<str> = text_written_18(v23.clone(), v26.clone());
            lock_released_38(v6.clone());
            v15.clone()
        }
    }
}
fn compile_hold_loop_12(mut v0: Rc<str>, mut v1: US1, mut v2: u64, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: u64, mut v6: u64, mut v7: u64) -> () {
    loop {
        let mut v8: u64 = now_ms_9();
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop"); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = job_file_2(v0.clone(), v9.clone());
        let mut v11: bool = file_exists_13(v10.clone());
        if v11 {
            return ();
        } else {
            let mut v12: bool = v8 > v7;
            if v12 {
                let mut v13: Rc<str> = windows_root_14();
                let mut v14: Rc<str> = window_file_15(v13.clone(), v4.clone());
                let mut v15: bool = file_removed_16(v14.clone());
                let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("quiet"); } LIT.with(|lit| lit.clone()) };
                let mut v17: Rc<str> = job_file_2(v0.clone(), v16.clone());
                let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("started beside compile windows after "); } LIT.with(|lit| lit.clone()) };
                let mut v19: u64 = v8.wrapping_sub(v5);
                let mut v20: u64 = v19.wrapping_div(60000u64);
                let mut v21: Rc<str> = decimal_text_17(v20);
                let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" min of --quiet-wait"); } LIT.with(|lit| lit.clone()) };
                let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v24: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v21, &*v22, &*v23, &*v23].concat());
                let mut v25: Rc<str> = text_written_18(v17.clone(), v24.clone());
                return ();
            } else {
                let mut v26: Rc<str> = window_blocker_19(v1.clone(), v2, v3.clone(), v4.clone(), v5, v6);
                let mut v27: bool = v26.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                if v27 {
                    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("quiet"); } LIT.with(|lit| lit.clone()) };
                    let mut v29: Rc<str> = job_file_2(v0.clone(), v28.clone());
                    let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("holding compiles id="); } LIT.with(|lit| lit.clone()) };
                    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" waited_ms="); } LIT.with(|lit| lit.clone()) };
                    let mut v32: u64 = v8.wrapping_sub(v5);
                    let mut v33: Rc<str> = decimal_text_17(v32);
                    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v35: Rc<str> = std::rc::Rc::<str>::from([&*v30, &*v4, &*v31, &*v33, &*v34].concat());
                    let mut v36: Rc<str> = text_written_18(v29.clone(), v35.clone());
                    return ();
                } else {
                    let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("quiet"); } LIT.with(|lit| lit.clone()) };
                    let mut v38: Rc<str> = job_file_2(v0.clone(), v37.clone());
                    let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("waiting for compiles to end id="); } LIT.with(|lit| lit.clone()) };
                    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                    let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v42: Rc<str> = std::rc::Rc::<str>::from([&*v39, &*v4, &*v40, &*v26, &*v41].concat());
                    let mut v43: Rc<str> = text_written_18(v38.clone(), v42.clone());
                    let mut v44: u64 = 5000u64;
                    std::thread::sleep(std::time::Duration::from_millis(v44));
                    (v0, v1, v2, v3, v4, v5, v6, v7) = (v0.clone(), v1.clone(), v2, v3.clone(), v4.clone(), v5, v6, v7);
                    continue;
                }
            }
        }
        return ();
    }
}
fn compile_hold_taken_1(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("quiet_hold_min"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = job_file_2(v0.clone(), v1.clone());
    let mut v3: Rc<str> = text_or_empty_6(v2.clone());
    let mut v4: u64 = decimal_u64_7(v3.clone());
    let mut v5: bool = v4 == 0u64;
    if v5 {
        0i32
    } else {
        let mut v6: u64 = now_ms_9();
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("quiet_wait_min"); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = job_file_2(v0.clone(), v7.clone());
        let mut v9: Rc<str> = text_or_empty_6(v8.clone());
        let mut v10: u64 = decimal_u64_7(v9.clone());
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("label"); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = job_file_2(v0.clone(), v11.clone());
        let mut v13: Rc<str> = text_or_empty_6(v12.clone());
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (eoie job "); } LIT.with(|lit| lit.clone()) };
        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v14, &*v0, &*v15, &*v16].concat());
        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("win-"); } LIT.with(|lit| lit.clone()) };
        let mut v19: Rc<str> = number_text_10(v6);
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        let mut v21: u64 = process_number_11();
        let mut v22: Rc<str> = number_text_10(v21);
        let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-compile-hold"); } LIT.with(|lit| lit.clone()) };
        let mut v24: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v19, &*v20, &*v22, &*v23].concat());
        let mut v25: u64 = v10.wrapping_mul(60000u64);
        let mut v26: u64 = v6.wrapping_add(v25);
        let mut v27: US1 = US1::US1_CompileHold;
        compile_hold_loop_12(v0.clone(), v27.clone(), v4, v17.clone(), v24.clone(), v6, v26, v26);
        0i32
    }
}
fn settled_word_41(mut v0: US5) -> Rc<str> {
    match &v0 {
        US5::US5_JobExpiredInQueue => {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expired-in-queue"); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US5::US5_JobFinished(v1) => {
            let mut v1: i32 = *v1;
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished"); } LIT.with(|lit| lit.clone()) };
            v2.clone()
        }
        US5::US5_JobLost => {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lost"); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        }
        US5::US5_JobMemoryCapped => {
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory-capped"); } LIT.with(|lit| lit.clone()) };
            v10.clone()
        }
        US5::US5_JobSkipped(v7) => {
            let mut v7: Rc<str> = v7.clone();
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("skipped"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US5::US5_JobSpawnFailed(v5) => {
            let mut v5: Rc<str> = v5.clone();
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spawn-failed"); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        }
        US5::US5_JobStopped => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US5::US5_JobTimedOut => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed-out"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
    }
}
fn gate_loop_40(mut v0: Rc<str>, mut v1: US1, mut v2: u64, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: u64, mut v6: u64, mut v7: u64) -> i32 {
    loop {
        let mut v8: u64 = now_ms_9();
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = job_file_2(v0.clone(), v9.clone());
        let mut v11: Rc<str> = decimal_text_17(v8);
        let mut v12: Rc<str> = text_written_18(v10.clone(), v11.clone());
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop"); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<str> = job_file_2(v0.clone(), v13.clone());
        let mut v15: bool = file_exists_13(v14.clone());
        if v15 {
            let mut v16: Rc<str> = windows_root_14();
            let mut v17: Rc<str> = window_file_15(v16.clone(), v4.clone());
            let mut v18: bool = file_removed_16(v17.clone());
            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
            let mut v20: Rc<str> = job_file_2(v0.clone(), v19.clone());
            let mut v21: u64 = 0u64;
            let mut v22: bool = v21 == 0u64;
            let mut v25: US4 = if v22 {
                US4::US4_TreeStopped
            } else {
                US4::US4_SurvivorsLeft(v21)
            };
            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped"); } LIT.with(|lit| lit.clone()) };
            let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped on request"); } LIT.with(|lit| lit.clone()) };
            let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
            let mut v36: Rc<str> = match &v25 {
                US4::US4_SurvivorsLeft(v30) => {
                    let mut v30: u64 = *v30;
                    let mut v31: Rc<str> = std::rc::Rc::<str>::from((v30).to_string());
                    let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" processes left alive"); } LIT.with(|lit| lit.clone()) };
                    let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v34: Rc<str> = std::rc::Rc::<str>::from([&*v31, &*v32, &*v33, &*v33, &*v33].concat());
                    v34.clone()
                }
                US4::US4_TreeStopped => {
                    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("process tree stopped"); } LIT.with(|lit| lit.clone()) };
                    v29.clone()
                }
            };
            let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v38: Rc<str> = std::rc::Rc::<str>::from([&*v27, &*v28, &*v36, &*v37, &*v37].concat());
            let mut v39: Rc<str> = std::rc::Rc::<str>::from((143i32).to_string());
            let mut v40: Rc<str> = std::rc::Rc::<str>::from((v8).to_string());
            let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v43: Rc<str> = std::rc::Rc::<str>::from([&*v26, &*v41, &*v38, &*v42, &*v37].concat());
            let mut v44: Rc<str> = std::rc::Rc::<str>::from([&*v39, &*v41, &*v40, &*v41, &*v43].concat());
            let mut v45: Rc<str> = text_written_18(v20.clone(), v44.clone());
            return 143i32;
        } else {
            let mut v46: bool = v8 > v7;
            if v46 {
                let mut v47: Rc<str> = windows_root_14();
                let mut v48: Rc<str> = window_file_15(v47.clone(), v4.clone());
                let mut v49: bool = file_removed_16(v48.clone());
                let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
                let mut v51: Rc<str> = job_file_2(v0.clone(), v50.clone());
                let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no "); } LIT.with(|lit| lit.clone()) };
                let mut v53: Rc<str> = class_word_34(v1.clone());
                let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" window before the deadline (24 hours, or --slot-within); the job never started"); } LIT.with(|lit| lit.clone()) };
                let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v56: Rc<str> = std::rc::Rc::<str>::from([&*v52, &*v53, &*v54, &*v55, &*v55].concat());
                let mut v57: US5 = US5::US5_JobExpiredInQueue;
                let mut v58: Rc<str> = settled_word_41(v57.clone());
                let mut v59: Rc<str> = std::rc::Rc::<str>::from((75i32).to_string());
                let mut v60: Rc<str> = std::rc::Rc::<str>::from((v8).to_string());
                let mut v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let mut v63: Rc<str> = std::rc::Rc::<str>::from([&*v58, &*v61, &*v56, &*v62, &*v55].concat());
                let mut v64: Rc<str> = std::rc::Rc::<str>::from([&*v59, &*v61, &*v60, &*v61, &*v63].concat());
                let mut v65: Rc<str> = text_written_18(v51.clone(), v64.clone());
                return 75i32;
            } else {
                let mut v66: Rc<str> = window_blocker_19(v1.clone(), v2, v3.clone(), v4.clone(), v5, v6);
                let mut v67: bool = v66.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                if v67 {
                    let mut v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("window"); } LIT.with(|lit| lit.clone()) };
                    let mut v69: Rc<str> = job_file_2(v0.clone(), v68.clone());
                    let mut v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("granted id="); } LIT.with(|lit| lit.clone()) };
                    let mut v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" waited_ms="); } LIT.with(|lit| lit.clone()) };
                    let mut v72: u64 = v8.wrapping_sub(v5);
                    let mut v73: Rc<str> = decimal_text_17(v72);
                    let mut v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v75: Rc<str> = std::rc::Rc::<str>::from([&*v70, &*v4, &*v71, &*v73, &*v74].concat());
                    let mut v76: Rc<str> = text_written_18(v69.clone(), v75.clone());
                    return 0i32;
                } else {
                    let mut v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("window"); } LIT.with(|lit| lit.clone()) };
                    let mut v78: Rc<str> = job_file_2(v0.clone(), v77.clone());
                    let mut v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("waiting id="); } LIT.with(|lit| lit.clone()) };
                    let mut v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                    let mut v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v82: Rc<str> = std::rc::Rc::<str>::from([&*v79, &*v4, &*v80, &*v66, &*v81].concat());
                    let mut v83: Rc<str> = text_written_18(v78.clone(), v82.clone());
                    let mut v84: u64 = 5000u64;
                    std::thread::sleep(std::time::Duration::from_millis(v84));
                    (v0, v1, v2, v3, v4, v5, v6, v7) = (v0.clone(), v1.clone(), v2, v3.clone(), v4.clone(), v5, v6, v7);
                    continue;
                }
            }
        }
    }
}
fn windowed_gate_39(mut v0: Rc<str>, mut v1: US1) -> i32 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout_ms"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = job_file_2(v0.clone(), v2.clone());
    let mut v4: Rc<str> = text_or_empty_6(v3.clone());
    let mut v5: u64 = decimal_u64_7(v4.clone());
    let mut v6: u64 = now_ms_9();
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("label"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = job_file_2(v0.clone(), v7.clone());
    let mut v9: Rc<str> = text_or_empty_6(v8.clone());
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (eoie job "); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v10, &*v0, &*v11, &*v12].concat());
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("win-"); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = number_text_10(v6);
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let mut v17: u64 = process_number_11();
    let mut v18: Rc<str> = number_text_10(v17);
    let mut v19: Rc<str> = class_word_34(v1.clone());
    let mut v20: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v19, &*v12, &*v12, &*v12].concat());
    let mut v21: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v15, &*v16, &*v18, &*v20].concat());
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot_by_ms"); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = job_file_2(v0.clone(), v22.clone());
    let mut v24: Rc<str> = text_or_empty_6(v23.clone());
    let mut v25: u64 = decimal_u64_7(v24.clone());
    let mut v26: bool = v25 != 0u64;
    let mut v29: bool = if v26 {
        let mut v27: u64 = v6.wrapping_add(86400000u64);
        let mut v28: bool = v25 < v27;
        v28
    } else {
        false
    };
    let mut v31: u64 = if v29 {
        v25
    } else {
        let mut v30: u64 = v6.wrapping_add(86400000u64);
        v30
    };
    let mut v32: u64 = v5.wrapping_div(60000u64);
    let mut v33: u64 = v32.wrapping_add(1u64);
    gate_loop_40(v0.clone(), v1.clone(), v33, v13.clone(), v21.clone(), v6, v31, v31)
}
fn job_window_gate_0(mut v0: Rc<str>) -> i32 {
    let mut v1: i32 = eoie_job_slot::eoie_job_slot_gate(&*v0);
    let mut v2: bool = v1 != 0i32;
    if v2 {
        v1
    } else {
        let mut v3: US1 = US1::US1_HeavyBuild;
        let mut v4: i32 = windowed_gate_39(v0.clone(), v3.clone());
        let mut v5: bool = v4 != 0i32;
        if v5 {
            v4
        } else {
            compile_hold_taken_1(v0.clone())
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        job_window_gate_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn job_compile_gate_42(mut v0: Rc<str>) -> i32 {
    let mut v1: US1 = US1::US1_Compile;
    windowed_gate_39(v0.clone(), v1.clone())
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        job_compile_gate_42(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_job_window_gate(v0: &str) -> i32 {
    closure0()(Rc::<str>::from(v0))
}
pub fn eoie_job_compile_gate(v0: &str) -> i32 {
    closure2()(Rc::<str>::from(v0))
}
