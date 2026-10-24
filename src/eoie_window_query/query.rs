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
    US1_ProcessHolder,
    US1_LeaseHolder,
}
#[derive(Clone)]
enum US3 {
    US3_LiveGui,
    US3_TestSuite,
    US3_HeavyBuild,
    US3_Compile,
    US3_CompilerInstall,
    US3_TreeRead,
    US3_Bench,
    US3_CompileHold,
}
#[derive(Clone)]
enum US2 {
    US2_ClassKnown(US3),
    US2_ClassUnknown,
}
fn windows_root_1() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("EOIE_WINDOWS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("eoie").join("windows"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("eoie").join("windows"))).unwrap_or_else(|| std::path::PathBuf::from(".eoie-windows")).to_string_lossy().as_ref());
    v0.clone()
}
fn now_ms_2() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn stored_rows_3(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { let mut rows = std::fs::read_dir(&*v0).into_iter().flatten().flatten().map(|entry| entry.path()).filter(|path| path.extension().map(|ext| ext == "window").unwrap_or(false)).filter_map(|path| std::fs::read_to_string(&path).ok()).map(|text| text.trim().to_owned()).filter(|text| !text.is_empty()).collect::<Vec<_>>(); rows.sort_by_key(|row| row.split(char::from(9u8)).nth(4).and_then(|start| start.parse::<u64>().ok()).unwrap_or(0)); std::rc::Rc::<str>::from(rows.join("
")) };
    v1.clone()
}
fn first_row_5(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().next().unwrap_or(""));
    v1.clone()
}
fn tab_field_7(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(v1 as usize).unwrap_or(""));
    v2.clone()
}
fn method9(mut v0: Option<u64>) -> Option<u64> {
    v0.clone()
}
fn closure1() -> Rc<dyn Fn((u64)) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US0> = Rc::new(move |mut v0: (u64)| -> US0 {
        let mut v1: u64 = (v0);
        US0::US0_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_of_8(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method9(v1.clone());
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
fn row_expired_6(mut v0: Rc<str>, mut v1: u64) -> bool {
    let mut v2: i32 = 5i32;
    let mut v3: Rc<str> = tab_field_7(v0.clone(), v2);
    let mut v4: u64 = decimal_of_8(v3.clone());
    let mut v5: bool = v4 < v1;
    v5
}
fn holder_of_11(mut v0: Rc<str>) -> US1 {
    let mut v1: i32 = 3i32;
    let mut v2: Rc<str> = tab_field_7(v0.clone(), v1);
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(eoie job "); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = v2.contains(&*v3);
    let mut v9: bool = if v4 {
        true
    } else {
        let mut v5: i32 = 0i32;
        let mut v6: Rc<str> = tab_field_7(v0.clone(), v5);
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-run"); } LIT.with(|lit| lit.clone()) };
        let mut v8: bool = v6.ends_with(&*v7);
        v8
    };
    if v9 {
        US1::US1_ProcessHolder
    } else {
        US1::US1_LeaseHolder
    }
}
fn window_pid_12(mut v0: Rc<str>) -> u64 {
    let mut v1: u64 = (v0).split(char::from(45u8)).nth(2).and_then(|pid| pid.parse::<u64>().ok()).unwrap_or(0);
    v1
}
fn process_alive_13(mut v0: u64) -> bool {
    let mut v1: bool = { #[cfg(windows)] fn alive(pid: u64) -> bool { unsafe extern "system" { fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void; fn WaitForSingleObject(handle: *mut std::ffi::c_void, ms: u32) -> u32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; fn GetLastError() -> u32; } if pid == 0 { return true; } unsafe { let handle = OpenProcess(0x0010_0000, 0, pid as u32); if handle.is_null() { return GetLastError() == 5; } let running = WaitForSingleObject(handle, 0) == 258; CloseHandle(handle); running } } #[cfg(not(windows))] fn alive(pid: u64) -> bool { pid == 0 || std::os::unix::process::parent_id() as u64 == pid } alive(v0) };
    v1
}
fn orphaned_window_10(mut v0: Rc<str>) -> bool {
    let mut v1: US1 = holder_of_11(v0.clone());
    match &v1 {
        US1::US1_LeaseHolder => {
            false
        }
        US1::US1_ProcessHolder => {
            let mut v2: i32 = 0i32;
            let mut v3: Rc<str> = tab_field_7(v0.clone(), v2);
            let mut v4: u64 = window_pid_12(v3.clone());
            let mut v5: bool = process_alive_13(v4);
            let mut v6: bool = v5 == false;
            v6
        }
    }
}
fn joined_15(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2.clone()
}
fn window_file_14(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".window"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v2, &*v3, &*v3, &*v3].concat());
    joined_15(v0.clone(), v4.clone())
}
fn file_removed_16(mut v0: Rc<str>) -> bool {
    let mut v1: bool = std::fs::remove_file(&*v0).is_ok();
    v1
}
fn other_rows_17(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().skip(1).collect::<Vec<_>>().join("
"));
    v1.clone()
}
fn live_rows_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let mut v4: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v4 {
            return v3.clone();
        } else {
            let mut v5: Rc<str> = first_row_5(v1.clone());
            let mut v6: bool = row_expired_6(v5.clone(), v2);
            let mut v8: bool = if v6 {
                true
            } else {
                orphaned_window_10(v5.clone())
            };
            if v8 {
                let mut v9: i32 = 0i32;
                let mut v10: Rc<str> = tab_field_7(v5.clone(), v9);
                let mut v11: Rc<str> = window_file_14(v0.clone(), v10.clone());
                let mut v12: bool = file_removed_16(v11.clone());
                let mut v13: Rc<str> = other_rows_17(v1.clone());
                (v0, v1, v2, v3) = (v0.clone(), v13.clone(), v2, v3.clone());
                continue;
            } else {
                let mut v15: Rc<str> = other_rows_17(v1.clone());
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
fn number_text_19(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1.clone()
}
fn minutes_left_20(mut v0: u64, mut v1: u64) -> u64 {
    let mut v2: bool = v0 <= v1;
    if v2 {
        0u64
    } else {
        let mut v3: u64 = v0.wrapping_sub(v1);
        let mut v4: u64 = v3.wrapping_add(59999u64);
        let mut v5: u64 = v4.wrapping_div(60000u64);
        v5
    }
}
fn stdout_line_21(mut v0: Rc<str>) -> () {
    println!("{}", &*v0);
    ()
}
fn shown_rows_18(mut v0: Rc<str>, mut v1: u64, mut v2: u64) -> u64 {
    loop {
        let mut v3: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v3 {
            return v2;
        } else {
            let mut v4: Rc<str> = first_row_5(v0.clone());
            let mut v5: i32 = 2i32;
            let mut v6: Rc<str> = tab_field_7(v4.clone(), v5);
            let mut v7: bool = v6.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("waiting"); } LIT.with(|lit| lit.clone()) };
            let mut v25: Rc<str> = if v7 {
                let mut v8: i32 = 4i32;
                let mut v9: Rc<str> = tab_field_7(v4.clone(), v8);
                let mut v10: u64 = decimal_of_8(v9.clone());
                let mut v11: u64 = v1.wrapping_sub(v10);
                let mut v12: u64 = v11.wrapping_div(60000u64);
                let mut v13: Rc<str> = number_text_19(v12);
                let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("waited "); } LIT.with(|lit| lit.clone()) };
                let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v13));
                let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
                let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
                v17.clone()
            } else {
                let mut v18: i32 = 5i32;
                let mut v19: Rc<str> = tab_field_7(v4.clone(), v18);
                let mut v20: u64 = decimal_of_8(v19.clone());
                let mut v21: u64 = minutes_left_20(v20, v1);
                let mut v22: Rc<str> = number_text_19(v21);
                let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m left"); } LIT.with(|lit| lit.clone()) };
                let mut v24: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v23));
                v24.clone()
            };
            let mut v26: i32 = 0i32;
            let mut v27: Rc<str> = tab_field_7(v4.clone(), v26);
            let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
            let mut v29: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v28));
            let mut v30: i32 = 1i32;
            let mut v31: Rc<str> = tab_field_7(v4.clone(), v30);
            let mut v32: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v31));
            let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v28));
            let mut v34: i32 = 2i32;
            let mut v35: Rc<str> = tab_field_7(v4.clone(), v34);
            let mut v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v28));
            let mut v37: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v25));
            let mut v38: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v28));
            let mut v39: i32 = 3i32;
            let mut v40: Rc<str> = tab_field_7(v4.clone(), v39);
            let mut v41: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v40));
            let mut v42: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v41));
            stdout_line_21(v42.clone());
            let mut v43: Rc<str> = other_rows_17(v0.clone());
            let mut v44: u64 = v2.wrapping_add(1u64);
            (v0, v1, v2) = (v43.clone(), v1, v44);
            continue;
        }
    }
}
fn free_mb_22() -> u64 {
    let mut v0: u64 = { #[cfg(windows)] fn free() -> u64 { #[repr(C)] struct Status { length: u32, load: u32, total: u64, available: u64, total_page: u64, available_page: u64, total_virtual: u64, available_virtual: u64, extended: u64 } unsafe extern "system" { fn GlobalMemoryStatusEx(status: *mut Status) -> i32; } unsafe { let mut status: Status = std::mem::zeroed(); status.length = std::mem::size_of::<Status>() as u32; if GlobalMemoryStatusEx(&mut status) == 0 { u64::MAX } else { status.available / 1048576 } } } #[cfg(not(windows))] fn free() -> u64 { std::fs::read_to_string("/proc/meminfo").ok().and_then(|text| text.lines().find(|line| line.starts_with("MemAvailable:")).and_then(|line| line.split_whitespace().nth(1)).and_then(|kb| kb.parse::<u64>().ok())).map(|kb| kb / 1024).unwrap_or(u64::MAX) } std::env::var("EOIE_WINDOWS_FREE_MB").ok().and_then(|text| text.trim().parse::<u64>().ok()).unwrap_or_else(free) };
    v0
}
fn window_listed_0() -> i32 {
    let mut v0: Rc<str> = windows_root_1();
    let mut v1: u64 = now_ms_2();
    let mut v2: Rc<str> = stored_rows_3(v0.clone());
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = live_rows_4(v0.clone(), v2.clone(), v1, v3.clone());
    let mut v5: u64 = 0u64;
    let mut v6: u64 = shown_rows_18(v4.clone(), v1, v5);
    let mut v7: Rc<str> = eoie_window_run::eoie_heavy_holders_text();
    let mut v8: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v8 {
        ()
    } else {
        stdout_line_21(v7.clone())
    };
    let mut v9: u64 = (v7).lines().count() as u64;
    let mut v10: Rc<str> = number_text_19(v6);
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie window list ok windows="); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v10));
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" free_mb="); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
    let mut v15: u64 = free_mb_22();
    let mut v16: Rc<str> = number_text_19(v15);
    let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v16));
    let mut v18: Rc<str> = number_text_19(v9);
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" heavy_slots="); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v18));
    let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v20));
    stdout_line_21(v21.clone());
    0i32
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        window_listed_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn stderr_line_25(mut v0: Rc<str>) -> () {
    eprintln!("{}", &*v0);
    ()
}
fn refused_24(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: window "); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v0, &*v2, &*v2, &*v2].concat());
    stderr_line_25(v3.clone());
    2i32
}
fn class_word_27(mut v0: US3) -> Rc<str> {
    match &v0 {
        US3::US3_Bench => {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bench"); } LIT.with(|lit| lit.clone()) };
            v7.clone()
        }
        US3::US3_Compile => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US3::US3_CompileHold => {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile-hold"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US3::US3_CompilerInstall => {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compiler-install"); } LIT.with(|lit| lit.clone()) };
            v5.clone()
        }
        US3::US3_HeavyBuild => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-build"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
        US3::US3_LiveGui => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("live-gui"); } LIT.with(|lit| lit.clone()) };
            v1.clone()
        }
        US3::US3_TestSuite => {
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test-suite"); } LIT.with(|lit| lit.clone()) };
            v2.clone()
        }
        US3::US3_TreeRead => {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tree-read"); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        }
    }
}
fn class_of_26(mut v0: Rc<str>) -> US2 {
    let mut v1: US3 = US3::US3_LiveGui;
    let mut v2: Rc<str> = class_word_27(v1.clone());
    let mut v3: bool = v0.clone() == v2.clone();
    if v3 {
        let mut v4: US3 = US3::US3_LiveGui;
        US2::US2_ClassKnown(v4.clone())
    } else {
        let mut v6: US3 = US3::US3_TestSuite;
        let mut v7: Rc<str> = class_word_27(v6.clone());
        let mut v8: bool = v0.clone() == v7.clone();
        if v8 {
            let mut v9: US3 = US3::US3_TestSuite;
            US2::US2_ClassKnown(v9.clone())
        } else {
            let mut v11: US3 = US3::US3_HeavyBuild;
            let mut v12: Rc<str> = class_word_27(v11.clone());
            let mut v13: bool = v0.clone() == v12.clone();
            if v13 {
                let mut v14: US3 = US3::US3_HeavyBuild;
                US2::US2_ClassKnown(v14.clone())
            } else {
                let mut v16: US3 = US3::US3_Compile;
                let mut v17: Rc<str> = class_word_27(v16.clone());
                let mut v18: bool = v0.clone() == v17.clone();
                if v18 {
                    let mut v19: US3 = US3::US3_Compile;
                    US2::US2_ClassKnown(v19.clone())
                } else {
                    let mut v21: US3 = US3::US3_CompilerInstall;
                    let mut v22: Rc<str> = class_word_27(v21.clone());
                    let mut v23: bool = v0.clone() == v22.clone();
                    if v23 {
                        let mut v24: US3 = US3::US3_CompilerInstall;
                        US2::US2_ClassKnown(v24.clone())
                    } else {
                        let mut v26: US3 = US3::US3_TreeRead;
                        let mut v27: Rc<str> = class_word_27(v26.clone());
                        let mut v28: bool = v0.clone() == v27.clone();
                        if v28 {
                            let mut v29: US3 = US3::US3_TreeRead;
                            US2::US2_ClassKnown(v29.clone())
                        } else {
                            let mut v31: US3 = US3::US3_Bench;
                            let mut v32: Rc<str> = class_word_27(v31.clone());
                            let mut v33: bool = v0.clone() == v32.clone();
                            if v33 {
                                let mut v34: US3 = US3::US3_Bench;
                                US2::US2_ClassKnown(v34.clone())
                            } else {
                                let mut v36: US3 = US3::US3_CompileHold;
                                let mut v37: Rc<str> = class_word_27(v36.clone());
                                let mut v38: bool = v0.clone() == v37.clone();
                                if v38 {
                                    let mut v39: US3 = US3::US3_CompileHold;
                                    US2::US2_ClassKnown(v39.clone())
                                } else {
                                    US2::US2_ClassUnknown
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn verdict_text_28(mut v0: US3, mut v1: Rc<str>, mut v2: u64, mut v3: u64, mut v4: Rc<str>, mut v5: u64, mut v6: u64) -> Rc<str> {
    let mut v7: Rc<str> = class_word_27(v0.clone());
    let mut v8: Rc<str> = eoie_window_admission::eoie_window_verdict(&*v7, &*v1, v2, v3, &*v4, v5, v6);
    v8.clone()
}
fn window_checked_23() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let mut v2: US2 = class_of_26(v1.clone());
    match &v2 {
        US2::US2_ClassKnown(v46) => {
            let mut v46: US3 = v46.clone();
            let mut v47: Rc<str> = windows_root_1();
            let mut v48: u64 = now_ms_2();
            let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v50: u64 = 0u64;
            let mut v51: Rc<str> = stored_rows_3(v47.clone());
            let mut v52: Rc<str> = live_rows_4(v47.clone(), v51.clone(), v48, v49.clone());
            let mut v53: u64 = free_mb_22();
            let mut v54: Rc<str> = verdict_text_28(v46.clone(), v49.clone(), v48, v50, v52.clone(), v53, v48);
            let mut v55: bool = v54.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v55 {
                let mut v56: Rc<str> = class_word_27(v46.clone());
                let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie window check ok class="); } LIT.with(|lit| lit.clone()) };
                let mut v58: Rc<str> = Rc::<str>::from(format!("{}{}", v57, v56));
                let mut v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" free"); } LIT.with(|lit| lit.clone()) };
                let mut v60: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v59));
                stdout_line_21(v60.clone());
                0i32
            } else {
                let mut v61: Rc<str> = class_word_27(v46.clone());
                let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie window check blocked class="); } LIT.with(|lit| lit.clone()) };
                let mut v63: Rc<str> = Rc::<str>::from(format!("{}{}", v62, v61));
                let mut v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let mut v65: Rc<str> = Rc::<str>::from(format!("{}{}", v63, v64));
                let mut v66: Rc<str> = Rc::<str>::from(format!("{}{}", v65, v54));
                stdout_line_21(v66.clone());
                1i32
            }
        }
        US2::US2_ClassUnknown => {
            let mut v3: i32 = 3i32;
            let mut v4: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v3 as usize).unwrap_or_default());
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("check: unknown class '"); } LIT.with(|lit| lit.clone()) };
            let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v4));
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'; "); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: window request <class> <minutes> [--label <text>] [--wait <minutes>|until-granted] | window release <id> | window list | window check <class> | window claim <path> | window run <class> <minutes> [--label <text>] [--wait <minutes>] -- <command> [args...]; classes "); } LIT.with(|lit| lit.clone()) };
            let mut v10: US3 = US3::US3_LiveGui;
            let mut v11: Rc<str> = class_word_27(v10.clone());
            let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v11.clone()));
            let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v12.clone()));
            let mut v14: US3 = US3::US3_TestSuite;
            let mut v15: Rc<str> = class_word_27(v14.clone());
            let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v15.clone()));
            let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v13.clone(), v16.clone()));
            let mut v18: US3 = US3::US3_HeavyBuild;
            let mut v19: Rc<str> = class_word_27(v18.clone());
            let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v19.clone()));
            let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", v17.clone(), v20.clone()));
            let mut v22: US3 = US3::US3_Compile;
            let mut v23: Rc<str> = class_word_27(v22.clone());
            let mut v24: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v23.clone()));
            let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", v21.clone(), v24.clone()));
            let mut v26: US3 = US3::US3_CompilerInstall;
            let mut v27: Rc<str> = class_word_27(v26.clone());
            let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v27.clone()));
            let mut v29: Rc<str> = Rc::<str>::from(format!("{}{}", v25.clone(), v28.clone()));
            let mut v30: US3 = US3::US3_TreeRead;
            let mut v31: Rc<str> = class_word_27(v30.clone());
            let mut v32: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v31.clone()));
            let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", v29.clone(), v32.clone()));
            let mut v34: US3 = US3::US3_Bench;
            let mut v35: Rc<str> = class_word_27(v34.clone());
            let mut v36: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v35.clone()));
            let mut v37: Rc<str> = Rc::<str>::from(format!("{}{}", v33.clone(), v36.clone()));
            let mut v38: US3 = US3::US3_CompileHold;
            let mut v39: Rc<str> = class_word_27(v38.clone());
            let mut v40: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v39.clone()));
            let mut v41: Rc<str> = Rc::<str>::from(format!("{}{}", v37.clone(), v40.clone()));
            let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v43: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v41, &*v42, &*v42, &*v42].concat());
            let mut v44: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v43));
            refused_24(v44.clone())
        }
    }
}
fn closure2() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        window_checked_23()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_window_list_run() -> i32 {
    closure0()()
}
pub fn eoie_window_check_run() -> i32 {
    closure2()()
}
