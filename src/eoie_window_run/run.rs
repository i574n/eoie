#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
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
enum US0 {
    US0_ClassKnown(US1),
    US0_ClassUnknown,
}
#[derive(Clone)]
enum US2 {
    US2_Some(u64),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_ProcessHolder,
    US3_LeaseHolder,
}
#[derive(Clone)]
enum US4 {
    US4_Held,
    US4_Waiting,
}
fn stderr_line_2(mut v0: Rc<str>) -> () {
    eprintln!("{}", &*v0);
    ()
}
fn refused_1(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: window "); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v0, &*v2, &*v2, &*v2].concat());
    stderr_line_2(v3.clone());
    2i32
}
fn class_word_4(mut v0: US1) -> Rc<str> {
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
fn class_of_3(mut v0: Rc<str>) -> US0 {
    let mut v1: US1 = US1::US1_LiveGui;
    let mut v2: Rc<str> = class_word_4(v1.clone());
    let mut v3: bool = v0.clone() == v2.clone();
    if v3 {
        let mut v4: US1 = US1::US1_LiveGui;
        US0::US0_ClassKnown(v4.clone())
    } else {
        let mut v6: US1 = US1::US1_TestSuite;
        let mut v7: Rc<str> = class_word_4(v6.clone());
        let mut v8: bool = v0.clone() == v7.clone();
        if v8 {
            let mut v9: US1 = US1::US1_TestSuite;
            US0::US0_ClassKnown(v9.clone())
        } else {
            let mut v11: US1 = US1::US1_HeavyBuild;
            let mut v12: Rc<str> = class_word_4(v11.clone());
            let mut v13: bool = v0.clone() == v12.clone();
            if v13 {
                let mut v14: US1 = US1::US1_HeavyBuild;
                US0::US0_ClassKnown(v14.clone())
            } else {
                let mut v16: US1 = US1::US1_Compile;
                let mut v17: Rc<str> = class_word_4(v16.clone());
                let mut v18: bool = v0.clone() == v17.clone();
                if v18 {
                    let mut v19: US1 = US1::US1_Compile;
                    US0::US0_ClassKnown(v19.clone())
                } else {
                    let mut v21: US1 = US1::US1_CompilerInstall;
                    let mut v22: Rc<str> = class_word_4(v21.clone());
                    let mut v23: bool = v0.clone() == v22.clone();
                    if v23 {
                        let mut v24: US1 = US1::US1_CompilerInstall;
                        US0::US0_ClassKnown(v24.clone())
                    } else {
                        let mut v26: US1 = US1::US1_TreeRead;
                        let mut v27: Rc<str> = class_word_4(v26.clone());
                        let mut v28: bool = v0.clone() == v27.clone();
                        if v28 {
                            let mut v29: US1 = US1::US1_TreeRead;
                            US0::US0_ClassKnown(v29.clone())
                        } else {
                            let mut v31: US1 = US1::US1_Bench;
                            let mut v32: Rc<str> = class_word_4(v31.clone());
                            let mut v33: bool = v0.clone() == v32.clone();
                            if v33 {
                                let mut v34: US1 = US1::US1_Bench;
                                US0::US0_ClassKnown(v34.clone())
                            } else {
                                let mut v36: US1 = US1::US1_CompileHold;
                                let mut v37: Rc<str> = class_word_4(v36.clone());
                                let mut v38: bool = v0.clone() == v37.clone();
                                if v38 {
                                    let mut v39: US1 = US1::US1_CompileHold;
                                    US0::US0_ClassKnown(v39.clone())
                                } else {
                                    US0::US0_ClassUnknown
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn method6(mut v0: Option<u64>) -> Option<u64> {
    v0.clone()
}
fn closure1() -> Rc<dyn Fn((u64)) -> US2> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US2> = Rc::new(move |mut v0: (u64)| -> US2 {
        let mut v1: u64 = (v0);
        US2::US2_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_of_5(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method6(v1.clone());
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
fn command_after_separator_7() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip_while(|arg| arg != "--").skip(1).collect::<Vec<_>>().join(&char::from(31u8).to_string()).as_str());
    v0.clone()
}
fn now_ms_8() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn number_text_9(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1.clone()
}
fn process_number_10() -> u64 {
    let mut v0: u64 = std::process::id() as u64;
    v0
}
fn own_option_11(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().take_while(|arg| arg != "--").skip_while(|arg| arg != &*v0).nth(1).unwrap_or_default().as_str());
    v1.clone()
}
fn command_head_12(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(31u8)).next().unwrap_or(""));
    v1.clone()
}
fn windows_root_14() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("EOIE_WINDOWS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("eoie").join("windows"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("eoie").join("windows"))).unwrap_or_else(|| std::path::PathBuf::from(".eoie-windows")).to_string_lossy().as_ref());
    v0.clone()
}
fn lock_taken_15(mut v0: Rc<str>) -> bool {
    let mut v1: bool = { let lock = std::path::Path::new(&*v0).join("lock"); let _ = std::fs::create_dir_all(&*v0); let mut taken = false; for _ in 0..400 { if std::fs::create_dir(&lock).is_ok() { taken = true; break; } let stale = std::fs::metadata(&lock).and_then(|meta| meta.modified()).ok().and_then(|time| time.elapsed().ok()).map(|age| age.as_secs() > 10).unwrap_or(false); if stale { let _ = std::fs::remove_dir(&lock); } else { std::thread::sleep(std::time::Duration::from_millis(50)); } } taken };
    v1
}
fn stored_rows_16(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { let mut rows = std::fs::read_dir(&*v0).into_iter().flatten().flatten().map(|entry| entry.path()).filter(|path| path.extension().map(|ext| ext == "window").unwrap_or(false)).filter_map(|path| std::fs::read_to_string(&path).ok()).map(|text| text.trim().to_owned()).filter(|text| !text.is_empty()).collect::<Vec<_>>(); rows.sort_by_key(|row| row.split(char::from(9u8)).nth(4).and_then(|start| start.parse::<u64>().ok()).unwrap_or(0)); std::rc::Rc::<str>::from(rows.join("
")) };
    v1.clone()
}
fn first_row_18(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().next().unwrap_or(""));
    v1.clone()
}
fn tab_field_20(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(v1 as usize).unwrap_or(""));
    v2.clone()
}
fn row_expired_19(mut v0: Rc<str>, mut v1: u64) -> bool {
    let mut v2: i32 = 5i32;
    let mut v3: Rc<str> = tab_field_20(v0.clone(), v2);
    let mut v4: u64 = decimal_of_5(v3.clone());
    let mut v5: bool = v4 < v1;
    v5
}
fn holder_of_22(mut v0: Rc<str>) -> US3 {
    let mut v1: i32 = 3i32;
    let mut v2: Rc<str> = tab_field_20(v0.clone(), v1);
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(eoie job "); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = v2.contains(&*v3);
    let mut v9: bool = if v4 {
        true
    } else {
        let mut v5: i32 = 0i32;
        let mut v6: Rc<str> = tab_field_20(v0.clone(), v5);
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-run"); } LIT.with(|lit| lit.clone()) };
        let mut v8: bool = v6.ends_with(&*v7);
        v8
    };
    if v9 {
        US3::US3_ProcessHolder
    } else {
        US3::US3_LeaseHolder
    }
}
fn window_pid_23(mut v0: Rc<str>) -> u64 {
    let mut v1: u64 = (v0).split(char::from(45u8)).nth(2).and_then(|pid| pid.parse::<u64>().ok()).unwrap_or(0);
    v1
}
fn process_alive_24(mut v0: u64) -> bool {
    let mut v1: bool = { #[cfg(windows)] fn alive(pid: u64) -> bool { unsafe extern "system" { fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void; fn WaitForSingleObject(handle: *mut std::ffi::c_void, ms: u32) -> u32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; fn GetLastError() -> u32; } if pid == 0 { return true; } unsafe { let handle = OpenProcess(0x0010_0000, 0, pid as u32); if handle.is_null() { return GetLastError() == 5; } let running = WaitForSingleObject(handle, 0) == 258; CloseHandle(handle); running } } #[cfg(not(windows))] fn alive(pid: u64) -> bool { pid == 0 || std::os::unix::process::parent_id() as u64 == pid } alive(v0) };
    v1
}
fn orphaned_window_21(mut v0: Rc<str>) -> bool {
    let mut v1: US3 = holder_of_22(v0.clone());
    match &v1 {
        US3::US3_LeaseHolder => {
            false
        }
        US3::US3_ProcessHolder => {
            let mut v2: i32 = 0i32;
            let mut v3: Rc<str> = tab_field_20(v0.clone(), v2);
            let mut v4: u64 = window_pid_23(v3.clone());
            let mut v5: bool = process_alive_24(v4);
            let mut v6: bool = v5 == false;
            v6
        }
    }
}
fn joined_26(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2.clone()
}
fn window_file_25(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".window"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v2, &*v3, &*v3, &*v3].concat());
    joined_26(v0.clone(), v4.clone())
}
fn file_removed_27(mut v0: Rc<str>) -> bool {
    let mut v1: bool = std::fs::remove_file(&*v0).is_ok();
    v1
}
fn other_rows_28(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().skip(1).collect::<Vec<_>>().join("
"));
    v1.clone()
}
fn live_rows_17(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let mut v4: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v4 {
            return v3.clone();
        } else {
            let mut v5: Rc<str> = first_row_18(v1.clone());
            let mut v6: bool = row_expired_19(v5.clone(), v2);
            let mut v8: bool = if v6 {
                true
            } else {
                orphaned_window_21(v5.clone())
            };
            if v8 {
                let mut v9: i32 = 0i32;
                let mut v10: Rc<str> = tab_field_20(v5.clone(), v9);
                let mut v11: Rc<str> = window_file_25(v0.clone(), v10.clone());
                let mut v12: bool = file_removed_27(v11.clone());
                let mut v13: Rc<str> = other_rows_28(v1.clone());
                (v0, v1, v2, v3) = (v0.clone(), v13.clone(), v2, v3.clone());
                continue;
            } else {
                let mut v15: Rc<str> = other_rows_28(v1.clone());
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
fn free_mb_29() -> u64 {
    let mut v0: u64 = { #[cfg(windows)] fn free() -> u64 { #[repr(C)] struct Status { length: u32, load: u32, total: u64, available: u64, total_page: u64, available_page: u64, total_virtual: u64, available_virtual: u64, extended: u64 } unsafe extern "system" { fn GlobalMemoryStatusEx(status: *mut Status) -> i32; } unsafe { let mut status: Status = std::mem::zeroed(); status.length = std::mem::size_of::<Status>() as u32; if GlobalMemoryStatusEx(&mut status) == 0 { u64::MAX } else { status.available / 1048576 } } } #[cfg(not(windows))] fn free() -> u64 { std::fs::read_to_string("/proc/meminfo").ok().and_then(|text| text.lines().find(|line| line.starts_with("MemAvailable:")).and_then(|line| line.split_whitespace().nth(1)).and_then(|kb| kb.parse::<u64>().ok())).map(|kb| kb / 1024).unwrap_or(u64::MAX) } std::env::var("EOIE_WINDOWS_FREE_MB").ok().and_then(|text| text.trim().parse::<u64>().ok()).unwrap_or_else(free) };
    v0
}
fn verdict_text_30(mut v0: US1, mut v1: Rc<str>, mut v2: u64, mut v3: u64, mut v4: Rc<str>, mut v5: u64, mut v6: u64) -> Rc<str> {
    let mut v7: Rc<str> = class_word_4(v0.clone());
    let mut v8: Rc<str> = eoie_window_admission::eoie_window_verdict(&*v7, &*v1, v2, v3, &*v4, v5, v6);
    v8.clone()
}
fn one_line_33(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.chars().map(|c| if c.is_control() { char::from(32u8) } else { c }).collect::<String>());
    v1.clone()
}
fn row_text_32(mut v0: Rc<str>, mut v1: US1, mut v2: US4, mut v3: Rc<str>, mut v4: u64, mut v5: u64) -> Rc<str> {
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = class_word_4(v1.clone());
    let mut v11: Rc<str> = match &v2 {
        US4::US4_Held => {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US4::US4_Waiting => {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("waiting"); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        }
    };
    let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v6, &*v7, &*v6, &*v11].concat());
    let mut v13: Rc<str> = one_line_33(v3.clone());
    let mut v14: Rc<str> = number_text_9(v4);
    let mut v15: Rc<str> = number_text_9(v5);
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v6, &*v15, &*v16, &*v16].concat());
    let mut v18: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v6, &*v13, &*v6, &*v17].concat());
    v18.clone()
}
fn request_row_31(mut v0: US1, mut v1: u64, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: u64, mut v6: US4, mut v7: u64) -> Rc<str> {
    row_text_32(v3.clone(), v0.clone(), v6.clone(), v2.clone(), v4, v7)
}
fn text_written_34(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2.clone()
}
fn lock_released_35(mut v0: Rc<str>) -> () {
    { let _ = std::fs::remove_dir(std::path::Path::new(&*v0).join("lock")); };
    ()
}
fn request_head_36(mut v0: US1, mut v1: u64, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: u64, mut v6: Rc<str>) -> Rc<str> {
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie window request "); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" id="); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" class="); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = class_word_4(v0.clone());
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v10, &*v11, &*v11, &*v11].concat());
    let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v6, &*v8, &*v3, &*v12].concat());
    v13.clone()
}
fn stdout_line_37(mut v0: Rc<str>) -> () {
    println!("{}", &*v0);
    ()
}
fn requested_13(mut v0: US1, mut v1: u64, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: u64, mut v6: bool) -> i32 {
    loop {
        let mut v7: Rc<str> = windows_root_14();
        let mut v8: bool = lock_taken_15(v7.clone());
        let mut v9: bool = v8 == false;
        if v9 {
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("request: the registry lock stayed taken for 20 s"); } LIT.with(|lit| lit.clone()) };
            return refused_1(v10.clone());
        } else {
            let mut v12: u64 = now_ms_8();
            let mut v13: Rc<str> = stored_rows_16(v7.clone());
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v15: Rc<str> = live_rows_17(v7.clone(), v13.clone(), v12, v14.clone());
            let mut v16: u64 = free_mb_29();
            let mut v17: Rc<str> = verdict_text_30(v0.clone(), v3.clone(), v4, v1, v15.clone(), v16, v12);
            let mut v18: bool = v17.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v18 {
                let mut v19: u64 = v1.wrapping_mul(60000u64);
                let mut v20: u64 = v12.wrapping_add(v19);
                let mut v21: Rc<str> = window_file_25(v7.clone(), v3.clone());
                let mut v22: US4 = US4::US4_Held;
                let mut v23: Rc<str> = request_row_31(v0.clone(), v1, v2.clone(), v3.clone(), v12, v5, v22.clone(), v20);
                let mut v24: Rc<str> = text_written_34(v21.clone(), v23.clone());
                lock_released_35(v7.clone());
                let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
                let mut v26: Rc<str> = request_head_36(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v25.clone());
                let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" minutes="); } LIT.with(|lit| lit.clone()) };
                let mut v28: Rc<str> = number_text_9(v1);
                let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" free_mb="); } LIT.with(|lit| lit.clone()) };
                let mut v30: Rc<str> = number_text_9(v16);
                let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" waited_ms="); } LIT.with(|lit| lit.clone()) };
                let mut v32: u64 = v12.wrapping_sub(v4);
                let mut v33: Rc<str> = number_text_9(v32);
                let mut v34: Rc<str> = std::rc::Rc::<str>::from([&*v30, &*v31, &*v33, &*v14, &*v14].concat());
                let mut v35: Rc<str> = std::rc::Rc::<str>::from([&*v26, &*v27, &*v28, &*v29, &*v34].concat());
                stdout_line_37(v35.clone());
                return 0i32;
            } else {
                let mut v36: bool = v12 >= v5;
                if v36 {
                    let mut v37: Rc<str> = window_file_25(v7.clone(), v3.clone());
                    let mut v38: bool = file_removed_27(v37.clone());
                    lock_released_35(v7.clone());
                    let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("blocked"); } LIT.with(|lit| lit.clone()) };
                    let mut v40: Rc<str> = request_head_36(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v39.clone());
                    let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                    let mut v42: Rc<str> = std::rc::Rc::<str>::from([&*v40, &*v41, &*v17, &*v14, &*v14].concat());
                    stdout_line_37(v42.clone());
                    return 1i32;
                } else {
                    let mut v43: Rc<str> = window_file_25(v7.clone(), v3.clone());
                    let mut v44: US4 = US4::US4_Waiting;
                    let mut v45: u64 = v12.wrapping_add(30000u64);
                    let mut v46: Rc<str> = request_row_31(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v44.clone(), v45);
                    let mut v47: Rc<str> = text_written_34(v43.clone(), v46.clone());
                    lock_released_35(v7.clone());
                    if v6 {
                        ()
                    } else {
                        let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("queued"); } LIT.with(|lit| lit.clone()) };
                        let mut v49: Rc<str> = request_head_36(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v48.clone());
                        let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                        let mut v51: Rc<str> = std::rc::Rc::<str>::from([&*v49, &*v50, &*v17, &*v14, &*v14].concat());
                        stdout_line_37(v51.clone())
                    };
                    let mut v52: u64 = 2000u64;
                    std::thread::sleep(std::time::Duration::from_millis(v52));
                    let mut v53: bool = true;
                    (v0, v1, v2, v3, v4, v5, v6) = (v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v53);
                    continue;
                }
            }
        }
    }
}
fn child_status_38(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: i32 = { let mut parts = (v0).split(char::from(31u8)); let program = parts.next().unwrap_or_default().to_owned(); match std::process::Command::new(&program).args(parts).env("EOIE_WINDOW_HELD", &*v1).status() { Ok(status) => status.code().unwrap_or(1), Err(error) => { eprintln!("eoie error: window run: cannot start {program}: {error}"); 127 } } };
    v2
}
fn exit_text_39(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string().as_str());
    v1.clone()
}
fn window_ran_0() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let mut v2: US0 = class_of_3(v1.clone());
    match &v2 {
        US0::US0_ClassKnown(v46) => {
            let mut v46: US1 = v46.clone();
            let mut v47: i32 = 4i32;
            let mut v48: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v47 as usize).unwrap_or_default());
            let mut v49: u64 = decimal_of_5(v48.clone());
            let mut v50: Rc<str> = command_after_separator_7();
            let mut v51: bool = v49 == 0u64;
            if v51 {
                let mut v52: i32 = 4i32;
                let mut v53: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v52 as usize).unwrap_or_default());
                let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("run: minutes must be a whole number above 0, received '"); } LIT.with(|lit| lit.clone()) };
                let mut v55: Rc<str> = Rc::<str>::from(format!("{}{}", v54, v53));
                let mut v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'; "); } LIT.with(|lit| lit.clone()) };
                let mut v57: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v56));
                let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: window request <class> <minutes> [--label <text>] [--wait <minutes>|until-granted] | window release <id> | window list | window check <class> | window claim <path> | window run <class> <minutes> [--label <text>] [--wait <minutes>] -- <command> [args...]; classes "); } LIT.with(|lit| lit.clone()) };
                let mut v59: US1 = US1::US1_LiveGui;
                let mut v60: Rc<str> = class_word_4(v59.clone());
                let mut v61: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v60.clone()));
                let mut v62: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v61.clone()));
                let mut v63: US1 = US1::US1_TestSuite;
                let mut v64: Rc<str> = class_word_4(v63.clone());
                let mut v65: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v64.clone()));
                let mut v66: Rc<str> = Rc::<str>::from(format!("{}{}", v62.clone(), v65.clone()));
                let mut v67: US1 = US1::US1_HeavyBuild;
                let mut v68: Rc<str> = class_word_4(v67.clone());
                let mut v69: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v68.clone()));
                let mut v70: Rc<str> = Rc::<str>::from(format!("{}{}", v66.clone(), v69.clone()));
                let mut v71: US1 = US1::US1_Compile;
                let mut v72: Rc<str> = class_word_4(v71.clone());
                let mut v73: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v72.clone()));
                let mut v74: Rc<str> = Rc::<str>::from(format!("{}{}", v70.clone(), v73.clone()));
                let mut v75: US1 = US1::US1_CompilerInstall;
                let mut v76: Rc<str> = class_word_4(v75.clone());
                let mut v77: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v76.clone()));
                let mut v78: Rc<str> = Rc::<str>::from(format!("{}{}", v74.clone(), v77.clone()));
                let mut v79: US1 = US1::US1_TreeRead;
                let mut v80: Rc<str> = class_word_4(v79.clone());
                let mut v81: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v80.clone()));
                let mut v82: Rc<str> = Rc::<str>::from(format!("{}{}", v78.clone(), v81.clone()));
                let mut v83: US1 = US1::US1_Bench;
                let mut v84: Rc<str> = class_word_4(v83.clone());
                let mut v85: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v84.clone()));
                let mut v86: Rc<str> = Rc::<str>::from(format!("{}{}", v82.clone(), v85.clone()));
                let mut v87: US1 = US1::US1_CompileHold;
                let mut v88: Rc<str> = class_word_4(v87.clone());
                let mut v89: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v88.clone()));
                let mut v90: Rc<str> = Rc::<str>::from(format!("{}{}", v86.clone(), v89.clone()));
                let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v92: Rc<str> = std::rc::Rc::<str>::from([&*v58, &*v90, &*v91, &*v91, &*v91].concat());
                let mut v93: Rc<str> = Rc::<str>::from(format!("{}{}", v57, v92));
                refused_1(v93.clone())
            } else {
                let mut v95: bool = v50.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                if v95 {
                    let mut v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("run: give the command after --, e.g. window run compile 5 --wait 10 -- pwsh -File build.ps1"); } LIT.with(|lit| lit.clone()) };
                    refused_1(v96.clone())
                } else {
                    let mut v98: u64 = now_ms_8();
                    let mut v99: Rc<str> = number_text_9(v98);
                    let mut v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("win-"); } LIT.with(|lit| lit.clone()) };
                    let mut v101: Rc<str> = Rc::<str>::from(format!("{}{}", v100, v99));
                    let mut v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                    let mut v103: Rc<str> = Rc::<str>::from(format!("{}{}", v101, v102));
                    let mut v104: u64 = process_number_10();
                    let mut v105: Rc<str> = number_text_9(v104);
                    let mut v106: Rc<str> = Rc::<str>::from(format!("{}{}", v103, v105));
                    let mut v107: Rc<str> = class_word_4(v46.clone());
                    let mut v108: Rc<str> = Rc::<str>::from(format!("{}{}", v102, v107));
                    let mut v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-run"); } LIT.with(|lit| lit.clone()) };
                    let mut v110: Rc<str> = Rc::<str>::from(format!("{}{}", v108, v109));
                    let mut v111: Rc<str> = Rc::<str>::from(format!("{}{}", v106, v110));
                    let mut v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--label"); } LIT.with(|lit| lit.clone()) };
                    let mut v113: Rc<str> = own_option_11(v112.clone());
                    let mut v114: bool = v113.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v117: Rc<str> = if v114 {
                        command_head_12(v50.clone())
                    } else {
                        own_option_11(v112.clone())
                    };
                    let mut v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--wait"); } LIT.with(|lit| lit.clone()) };
                    let mut v119: Rc<str> = own_option_11(v118.clone());
                    let mut v120: u64 = decimal_of_5(v119.clone());
                    let mut v121: u64 = v120.wrapping_mul(60000u64);
                    let mut v122: u64 = v98.wrapping_add(v121);
                    let mut v123: bool = false;
                    let mut v124: i32 = requested_13(v46.clone(), v49, v117.clone(), v111.clone(), v98, v122, v123);
                    let mut v125: bool = v124 != 0i32;
                    if v125 {
                        v124
                    } else {
                        let mut v126: Rc<str> = class_word_4(v46.clone());
                        let mut v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                        let mut v128: Rc<str> = Rc::<str>::from(format!("{}{}", v126, v127));
                        let mut v129: Rc<str> = Rc::<str>::from(format!("{}{}", v128, v111));
                        let mut v130: i32 = child_status_38(v50.clone(), v129.clone());
                        let mut v131: Rc<str> = windows_root_14();
                        let mut v132: Rc<str> = window_file_25(v131.clone(), v111.clone());
                        let mut v133: bool = file_removed_27(v132.clone());
                        let mut v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie window run ok id="); } LIT.with(|lit| lit.clone()) };
                        let mut v135: Rc<str> = Rc::<str>::from(format!("{}{}", v134, v111));
                        let mut v136: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" exit="); } LIT.with(|lit| lit.clone()) };
                        let mut v137: Rc<str> = Rc::<str>::from(format!("{}{}", v135, v136));
                        let mut v138: Rc<str> = exit_text_39(v130);
                        let mut v139: Rc<str> = Rc::<str>::from(format!("{}{}", v137, v138));
                        let mut v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" released=yes"); } LIT.with(|lit| lit.clone()) };
                        let mut v141: Rc<str> = Rc::<str>::from(format!("{}{}", v139, v140));
                        stdout_line_37(v141.clone());
                        v130
                    }
                }
            }
        }
        US0::US0_ClassUnknown => {
            let mut v3: i32 = 3i32;
            let mut v4: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v3 as usize).unwrap_or_default());
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("run: unknown class '"); } LIT.with(|lit| lit.clone()) };
            let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v4));
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'; "); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: window request <class> <minutes> [--label <text>] [--wait <minutes>|until-granted] | window release <id> | window list | window check <class> | window claim <path> | window run <class> <minutes> [--label <text>] [--wait <minutes>] -- <command> [args...]; classes "); } LIT.with(|lit| lit.clone()) };
            let mut v10: US1 = US1::US1_LiveGui;
            let mut v11: Rc<str> = class_word_4(v10.clone());
            let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v11.clone()));
            let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v12.clone()));
            let mut v14: US1 = US1::US1_TestSuite;
            let mut v15: Rc<str> = class_word_4(v14.clone());
            let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v15.clone()));
            let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v13.clone(), v16.clone()));
            let mut v18: US1 = US1::US1_HeavyBuild;
            let mut v19: Rc<str> = class_word_4(v18.clone());
            let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v19.clone()));
            let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", v17.clone(), v20.clone()));
            let mut v22: US1 = US1::US1_Compile;
            let mut v23: Rc<str> = class_word_4(v22.clone());
            let mut v24: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v23.clone()));
            let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", v21.clone(), v24.clone()));
            let mut v26: US1 = US1::US1_CompilerInstall;
            let mut v27: Rc<str> = class_word_4(v26.clone());
            let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v27.clone()));
            let mut v29: Rc<str> = Rc::<str>::from(format!("{}{}", v25.clone(), v28.clone()));
            let mut v30: US1 = US1::US1_TreeRead;
            let mut v31: Rc<str> = class_word_4(v30.clone());
            let mut v32: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v31.clone()));
            let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", v29.clone(), v32.clone()));
            let mut v34: US1 = US1::US1_Bench;
            let mut v35: Rc<str> = class_word_4(v34.clone());
            let mut v36: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v35.clone()));
            let mut v37: Rc<str> = Rc::<str>::from(format!("{}{}", v33.clone(), v36.clone()));
            let mut v38: US1 = US1::US1_CompileHold;
            let mut v39: Rc<str> = class_word_4(v38.clone());
            let mut v40: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v39.clone()));
            let mut v41: Rc<str> = Rc::<str>::from(format!("{}{}", v37.clone(), v40.clone()));
            let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v43: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v41, &*v42, &*v42, &*v42].concat());
            let mut v44: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v43));
            refused_1(v44.clone())
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        window_ran_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn text_or_empty_42(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1.clone()
}
fn decimal_u64_43(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method6(v1.clone());
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
fn file_minutes_44(mut v0: Rc<str>) -> u64 {
    let mut v1: u64 = std::fs::metadata(&*v0).and_then(|meta| meta.modified()).ok().and_then(|time| time.elapsed().ok()).map(|age| age.as_secs() / 60).unwrap_or(0);
    v1
}
fn field_rest_45(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from((v0).trim().splitn((v1 as usize) + 1, char::from(9u8)).nth(v1 as usize).unwrap_or("").replace(char::from(9u8), " ").as_str());
    v2.clone()
}
fn holder_lines_41(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        let mut v2: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v2 {
            return v1.clone();
        } else {
            let mut v3: Rc<str> = first_row_18(v0.clone());
            let mut v4: Rc<str> = text_or_empty_42(v3.clone());
            let mut v5: i32 = 1i32;
            let mut v6: Rc<str> = tab_field_20(v4.clone(), v5);
            let mut v7: u64 = decimal_u64_43(v6.clone());
            let mut v8: bool = process_alive_24(v7);
            let mut v9: bool = v8 == false;
            if v9 {
                let mut v10: bool = file_removed_27(v3.clone());
                let mut v11: Rc<str> = other_rows_28(v0.clone());
                (v0, v1) = (v11.clone(), v1.clone());
                continue;
            } else {
                let mut v13: i32 = 0i32;
                let mut v14: Rc<str> = tab_field_20(v4.clone(), v13);
                let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-slot-"); } LIT.with(|lit| lit.clone()) };
                let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v14));
                let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  pid "); } LIT.with(|lit| lit.clone()) };
                let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v17));
                let mut v19: i32 = 1i32;
                let mut v20: Rc<str> = tab_field_20(v4.clone(), v19);
                let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v20));
                let mut v22: u64 = file_minutes_44(v3.clone());
                let mut v23: Rc<str> = number_text_9(v22);
                let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  held "); } LIT.with(|lit| lit.clone()) };
                let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v23));
                let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m  "); } LIT.with(|lit| lit.clone()) };
                let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v26));
                let mut v28: i32 = 3i32;
                let mut v29: Rc<str> = field_rest_45(v4.clone(), v28);
                let mut v30: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v29));
                let mut v31: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v30));
                let mut v32: Rc<str> = other_rows_28(v0.clone());
                let mut v33: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v37: Rc<str> = if v33 {
                    v31.clone()
                } else {
                    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let mut v35: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v34));
                    let mut v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v31));
                    v36.clone()
                };
                (v0, v1) = (v32.clone(), v37.clone());
                continue;
            }
        }
    }
}
fn heavy_queue_dir_46() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("EOIE_HEAVY_QUEUE").or_else(|| std::env::var_os("LOCALAPPDATA").or_else(|| std::env::var_os("HOME")).map(|base| std::path::PathBuf::from(base).join("spiral-bin").join("heavy-queue").into_os_string())).map(|path| path.to_string_lossy().into_owned()).unwrap_or_default().as_str());
    v0.clone()
}
fn holder_paths_47(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from({ let mut paths = std::fs::read_dir(&*v0).into_iter().flatten().flatten().map(|entry| entry.path()).filter(|path| path.extension().is_some_and(|ext| ext == "h")).map(|path| path.to_string_lossy().into_owned()).collect::<Vec<_>>(); paths.sort(); paths.join("
") }.as_str());
    v1.clone()
}
fn heavy_holders_text_40() -> Rc<str> {
    let mut v0: Rc<str> = heavy_queue_dir_46();
    let mut v1: Rc<str> = holder_paths_47(v0.clone());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    holder_lines_41(v1.clone(), v2.clone())
}
fn closure2() -> Rc<dyn Fn() -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> Rc<str>> = Rc::new(move || -> Rc<str> {
        heavy_holders_text_40()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_window_run_run() -> i32 {
    closure0()()
}
pub fn eoie_heavy_holders_text() -> Rc<str> {
    closure2()()
}
