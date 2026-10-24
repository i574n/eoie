#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Some(u64),
    US0_None,
}
#[derive(Clone)]
enum US2 {
    US2_JobPending,
    US2_JobRunning,
}
#[derive(Clone)]
enum US3 {
    US3_JobFinished(i32),
    US3_JobTimedOut,
    US3_JobStopped,
    US3_JobSpawnFailed(Rc<str>),
    US3_JobSkipped(Rc<str>),
    US3_JobLost,
    US3_JobMemoryCapped,
    US3_JobExpiredInQueue,
}
#[derive(Clone)]
enum US1 {
    US1_Live(US2),
    US1_Settled(US3),
}
#[derive(Clone)]
enum US4 {
    US4_StopRequested,
    US4_StopRefused(Rc<str>),
}
#[derive(Clone)]
enum US5 {
    US5_UnsettledPass,
    US5_SettledPass,
}
fn stderr_line_2(mut v0: Rc<str>) -> () {
    eprintln!("{}", &*v0);
    ()
}
fn rejected_1(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: job "); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v0, &*v2, &*v2, &*v2].concat());
    stderr_line_2(v3.clone());
    2i32
}
fn joined_4(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2.clone()
}
fn jobs_root_5() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("EOIE_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("eoie").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("eoie").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".eoie-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn job_directory_3(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = jobs_root_5();
    joined_4(v1.clone(), v0.clone())
}
fn directory_exists_6(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.is_dir();
    v8
}
fn command_line_argument_count_8() -> i32 {
    let mut v0: i32 = (std::env::args().count() as i32) - 1;
    v0
}
fn argument_count_7() -> i32 {
    let mut v0: i32 = command_line_argument_count_8();
    let mut v1: i32 = v0.wrapping_add(1i32);
    v1
}
fn method10(mut v0: Option<u64>) -> Option<u64> {
    v0.clone()
}
fn closure1() -> Rc<dyn Fn((u64)) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US0> = Rc::new(move |mut v0: (u64)| -> US0 {
        let mut v1: u64 = (v0);
        US0::US0_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_9(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method10(v1.clone());
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
fn job_file_14(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = job_directory_3(v0.clone());
    joined_4(v2.clone(), v1.clone())
}
fn text_or_empty_15(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1.clone()
}
fn now_ms_16() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn job_record_of_13(mut v0: Rc<str>) -> (Rc<str>, u64, u64, u64) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = job_file_14(v0.clone(), v1.clone());
    let mut v3: Rc<str> = text_or_empty_15(v2.clone());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = job_file_14(v0.clone(), v4.clone());
    let mut v6: Rc<str> = text_or_empty_15(v5.clone());
    let mut v7: u64 = decimal_u64_9(v6.clone());
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("started_ms"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = job_file_14(v0.clone(), v8.clone());
    let mut v10: Rc<str> = text_or_empty_15(v9.clone());
    let mut v11: u64 = decimal_u64_9(v10.clone());
    let mut v12: u64 = now_ms_16();
    (v3.clone(), v7, v11, v12)
}
fn job_state_of_17(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: u64) -> US1 {
    let mut v4: bool = v0.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v4 {
        let mut v5: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(2).unwrap_or("").trim());
        let mut v6: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished"); } LIT.with(|lit| lit.clone()) };
        if v6 {
            let mut v7: i32 = v0.split(char::from(9u8)).next().and_then(|code| code.trim().parse::<i32>().ok()).unwrap_or(1);
            let mut v8: US3 = US3::US3_JobFinished(v7);
            US1::US1_Settled(v8.clone())
        } else {
            let mut v10: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed-out"); } LIT.with(|lit| lit.clone()) };
            if v10 {
                let mut v11: US3 = US3::US3_JobTimedOut;
                US1::US1_Settled(v11.clone())
            } else {
                let mut v13: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped"); } LIT.with(|lit| lit.clone()) };
                if v13 {
                    let mut v14: US3 = US3::US3_JobStopped;
                    US1::US1_Settled(v14.clone())
                } else {
                    let mut v16: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lost"); } LIT.with(|lit| lit.clone()) };
                    if v16 {
                        let mut v17: US3 = US3::US3_JobLost;
                        US1::US1_Settled(v17.clone())
                    } else {
                        let mut v19: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory-capped"); } LIT.with(|lit| lit.clone()) };
                        if v19 {
                            let mut v20: US3 = US3::US3_JobMemoryCapped;
                            US1::US1_Settled(v20.clone())
                        } else {
                            let mut v22: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expired-in-queue"); } LIT.with(|lit| lit.clone()) };
                            if v22 {
                                let mut v23: US3 = US3::US3_JobExpiredInQueue;
                                US1::US1_Settled(v23.clone())
                            } else {
                                let mut v25: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("skipped"); } LIT.with(|lit| lit.clone()) };
                                if v25 {
                                    let mut v26: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(3).unwrap_or("").trim());
                                    let mut v27: US3 = US3::US3_JobSkipped(v26.clone());
                                    US1::US1_Settled(v27.clone())
                                } else {
                                    let mut v29: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(3).unwrap_or("").trim());
                                    let mut v30: US3 = US3::US3_JobSpawnFailed(v29.clone());
                                    US1::US1_Settled(v30.clone())
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        let mut v39: bool = v1 > 0u64;
        if v39 {
            let mut v40: u64 = v1.wrapping_add(15000u64);
            let mut v41: bool = v40 < v3;
            if v41 {
                let mut v42: US3 = US3::US3_JobLost;
                US1::US1_Settled(v42.clone())
            } else {
                let mut v44: US2 = US2::US2_JobRunning;
                US1::US1_Live(v44.clone())
            }
        } else {
            let mut v47: u64 = v2.wrapping_add(15000u64);
            let mut v48: bool = v47 < v3;
            if v48 {
                let mut v49: US3 = US3::US3_JobLost;
                US1::US1_Settled(v49.clone())
            } else {
                let mut v51: US2 = US2::US2_JobPending;
                US1::US1_Live(v51.clone())
            }
        }
    }
}
fn process_alive_18(mut v0: u64) -> bool {
    let mut v1: bool = { #[cfg(windows)] fn alive(pid: u64) -> bool { unsafe extern "system" { fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void; fn WaitForSingleObject(handle: *mut std::ffi::c_void, ms: u32) -> u32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; fn GetLastError() -> u32; } if pid == 0 { return true; } unsafe { let handle = OpenProcess(0x0010_0000, 0, pid as u32); if handle.is_null() { return GetLastError() == 5; } let running = WaitForSingleObject(handle, 0) == 258; CloseHandle(handle); running } } #[cfg(not(windows))] fn alive(pid: u64) -> bool { pid == 0 || std::os::unix::process::parent_id() as u64 == pid } alive(v0) };
    v1
}
fn settled_word_19(mut v0: US3) -> Rc<str> {
    match &v0 {
        US3::US3_JobExpiredInQueue => {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expired-in-queue"); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US3::US3_JobFinished(v1) => {
            let mut v1: i32 = *v1;
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished"); } LIT.with(|lit| lit.clone()) };
            v2.clone()
        }
        US3::US3_JobLost => {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lost"); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        }
        US3::US3_JobMemoryCapped => {
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory-capped"); } LIT.with(|lit| lit.clone()) };
            v10.clone()
        }
        US3::US3_JobSkipped(v7) => {
            let mut v7: Rc<str> = v7.clone();
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("skipped"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US3::US3_JobSpawnFailed(v5) => {
            let mut v5: Rc<str> = v5.clone();
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spawn-failed"); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        }
        US3::US3_JobStopped => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US3::US3_JobTimedOut => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed-out"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
    }
}
fn text_written_20(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2.clone()
}
fn job_state_now_12(mut v0: Rc<str>) -> US1 {
    let (mut v1, mut v2, mut v3, mut v4): (Rc<str>, u64, u64, u64) = job_record_of_13(v0.clone());
    let mut v5: US1 = job_state_of_17(v1.clone(), v2, v3, v4);
    match &v5 {
        US1::US1_Settled(v6) => {
            let mut v6: US3 = v6.clone();
            match &v6 {
                US3::US3_JobLost => {
                    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("supervisor"); } LIT.with(|lit| lit.clone()) };
                    let mut v8: Rc<str> = job_file_14(v0.clone(), v7.clone());
                    let mut v9: Rc<str> = text_or_empty_15(v8.clone());
                    let mut v10: u64 = decimal_u64_9(v9.clone());
                    let mut v11: bool = v10 != 0u64;
                    let mut v13: bool = if v11 {
                        process_alive_18(v10)
                    } else {
                        false
                    };
                    if v13 {
                        let mut v14: US2 = US2::US2_JobRunning;
                        US1::US1_Live(v14.clone())
                    } else {
                        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pid"); } LIT.with(|lit| lit.clone()) };
                        let mut v17: Rc<str> = job_file_14(v0.clone(), v16.clone());
                        let mut v18: Rc<str> = text_or_empty_15(v17.clone());
                        let mut v19: u64 = decimal_u64_9(v18.clone());
                        let mut v20: bool = v19 == 0u64;
                        let mut v23: bool = if v20 {
                            true
                        } else {
                            let mut v21: bool = process_alive_18(v19);
                            let mut v22: bool = v21 == false;
                            v22
                        };
                        let mut v24: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v25: bool = v24 && v23;
                        let mut v42: Rc<str> = if v25 {
                            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
                            let mut v27: Rc<str> = job_file_14(v0.clone(), v26.clone());
                            let mut v28: bool = v2 > 0u64;
                            let mut v29: u64 = if v28 {
                                v2
                            } else {
                                v3
                            };
                            let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the supervisor stopped beating: it was killed (with its caller, by the memory reaper) or crashed, and its job tree ended with it"); } LIT.with(|lit| lit.clone()) };
                            let mut v31: US3 = US3::US3_JobLost;
                            let mut v32: Rc<str> = settled_word_19(v31.clone());
                            let mut v33: Rc<str> = std::rc::Rc::<str>::from((125i32).to_string());
                            let mut v34: Rc<str> = std::rc::Rc::<str>::from((v29).to_string());
                            let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                            let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let mut v38: Rc<str> = std::rc::Rc::<str>::from([&*v32, &*v35, &*v30, &*v36, &*v37].concat());
                            let mut v39: Rc<str> = std::rc::Rc::<str>::from([&*v33, &*v35, &*v34, &*v35, &*v38].concat());
                            text_written_20(v27.clone(), v39.clone())
                        } else {
                            let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v41.clone()
                        };
                        v5.clone()
                    }
                }
                _ => {
                    v5.clone()
                }
            }
        }
        _ => {
            v5.clone()
        }
    }
}
fn file_exists_22(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.is_file();
    v8
}
fn gate_queued_21(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = job_file_14(v0.clone(), v1.clone());
    let mut v3: bool = file_exists_22(v2.clone());
    let mut v8: bool = if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after-done"); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = job_file_14(v0.clone(), v4.clone());
        let mut v6: bool = file_exists_22(v5.clone());
        let mut v7: bool = v6 == false;
        v7
    } else {
        false
    };
    if v8 {
        true
    } else {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("window-class"); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = job_file_14(v0.clone(), v9.clone());
        let mut v11: bool = file_exists_22(v10.clone());
        if v11 {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("window"); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = job_file_14(v0.clone(), v12.clone());
            let mut v14: Rc<str> = text_or_empty_15(v13.clone());
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("granted"); } LIT.with(|lit| lit.clone()) };
            let mut v16: bool = v14.starts_with(&*v15);
            let mut v17: bool = v16 == false;
            v17
        } else {
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-wrapper.ps1"); } LIT.with(|lit| lit.clone()) };
            let mut v19: Rc<str> = job_file_14(v0.clone(), v18.clone());
            let mut v20: bool = file_exists_22(v19.clone());
            if v20 {
                let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stdout.log"); } LIT.with(|lit| lit.clone()) };
                let mut v22: Rc<str> = job_file_14(v0.clone(), v21.clone());
                let mut v23: Rc<str> = text_or_empty_15(v22.clone());
                let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Invoke-Heavy: acquired slot"); } LIT.with(|lit| lit.clone()) };
                let mut v25: bool = v23.contains(&*v24);
                let mut v26: bool = v25 == false;
                v26
            } else {
                false
            }
        }
    }
}
fn state_settled_23(mut v0: US1) -> bool {
    match &v0 {
        US1::US1_Live(v1) => {
            let mut v1: US2 = v1.clone();
            false
        }
        US1::US1_Settled(v2) => {
            let mut v2: US3 = v2.clone();
            true
        }
    }
}
fn live_word_25(mut v0: US2) -> Rc<str> {
    match &v0 {
        US2::US2_JobPending => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pending"); } LIT.with(|lit| lit.clone()) };
            v1.clone()
        }
        US2::US2_JobRunning => {
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("running"); } LIT.with(|lit| lit.clone()) };
            v2.clone()
        }
    }
}
fn state_word_24(mut v0: US1) -> Rc<str> {
    match &v0 {
        US1::US1_Live(v1) => {
            let mut v1: US2 = v1.clone();
            live_word_25(v1.clone())
        }
        US1::US1_Settled(v3) => {
            let mut v3: US3 = v3.clone();
            settled_word_19(v3.clone())
        }
    }
}
fn settled_exit_26(mut v0: US3) -> i32 {
    match &v0 {
        US3::US3_JobExpiredInQueue => {
            75i32
        }
        US3::US3_JobFinished(v1) => {
            let mut v1: i32 = *v1;
            v1
        }
        US3::US3_JobLost => {
            125i32
        }
        US3::US3_JobMemoryCapped => {
            137i32
        }
        US3::US3_JobSkipped(v3) => {
            let mut v3: Rc<str> = v3.clone();
            125i32
        }
        US3::US3_JobSpawnFailed(v2) => {
            let mut v2: Rc<str> = v2.clone();
            125i32
        }
        US3::US3_JobStopped => {
            143i32
        }
        US3::US3_JobTimedOut => {
            124i32
        }
    }
}
fn elapsed_of_27(mut v0: Rc<str>) -> u64 {
    let (mut v1, mut v2, mut v3, mut v4): (Rc<str>, u64, u64, u64) = job_record_of_13(v0.clone());
    let mut v5: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v8: u64 = if v5 {
        v4
    } else {
        let mut v6: Rc<str> = std::rc::Rc::<str>::from(v1.split(char::from(9u8)).nth(1).unwrap_or("0"));
        decimal_u64_9(v6.clone())
    };
    let mut v9: bool = v8 > v3;
    if v9 {
        let mut v10: u64 = v8.wrapping_sub(v3);
        v10
    } else {
        0u64
    }
}
fn decimal_text_28(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1.clone()
}
fn file_bytes_29(mut v0: Rc<str>) -> u64 {
    let mut v1: u64 = std::fs::metadata(&*v0).map(|metadata| metadata.len()).unwrap_or(0);
    v1
}
fn job_summary_11(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: US1 = job_state_now_12(v1.clone());
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie job "); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ok id="); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" state="); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v0, &*v4, &*v1, &*v5].concat());
    let mut v7: bool = gate_queued_21(v1.clone());
    let mut v10: bool = if v7 {
        let mut v8: bool = state_settled_23(v2.clone());
        let mut v9: bool = v8 == false;
        v9
    } else {
        false
    };
    let mut v13: Rc<str> = if v10 {
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("queued"); } LIT.with(|lit| lit.clone()) };
        v11.clone()
    } else {
        state_word_24(v2.clone())
    };
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" exit="); } LIT.with(|lit| lit.clone()) };
    let mut v21: Rc<str> = match &v2 {
        US1::US1_Live(v15) => {
            let mut v15: US2 = v15.clone();
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            v16.clone()
        }
        US1::US1_Settled(v17) => {
            let mut v17: US3 = v17.clone();
            let mut v18: i32 = settled_exit_26(v17.clone());
            let mut v19: Rc<str> = std::rc::Rc::<str>::from((v18).to_string());
            v19.clone()
        }
    };
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v23: u64 = elapsed_of_27(v1.clone());
    let mut v24: Rc<str> = decimal_text_28(v23);
    let mut v25: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v14, &*v21, &*v22, &*v24].concat());
    let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" stdout_bytes="); } LIT.with(|lit| lit.clone()) };
    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stdout.log"); } LIT.with(|lit| lit.clone()) };
    let mut v28: Rc<str> = job_file_14(v1.clone(), v27.clone());
    let mut v29: u64 = file_bytes_29(v28.clone());
    let mut v30: Rc<str> = decimal_text_28(v29);
    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" stderr_bytes="); } LIT.with(|lit| lit.clone()) };
    let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stderr.log"); } LIT.with(|lit| lit.clone()) };
    let mut v33: Rc<str> = job_file_14(v1.clone(), v32.clone());
    let mut v34: u64 = file_bytes_29(v33.clone());
    let mut v35: Rc<str> = decimal_text_28(v34);
    let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v37: Rc<str> = std::rc::Rc::<str>::from([&*v26, &*v30, &*v31, &*v35, &*v36].concat());
    let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
    let mut v39: Rc<str> = job_file_14(v1.clone(), v38.clone());
    let mut v40: Rc<str> = text_or_empty_15(v39.clone());
    let mut v41: Rc<str> = std::rc::Rc::<str>::from(v40.split(char::from(9u8)).nth(3).unwrap_or("").trim());
    let mut v42: bool = v41.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v46: Rc<str> = if v42 {
        v36.clone()
    } else {
        let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" detail=\""); } LIT.with(|lit| lit.clone()) };
        let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let mut v45: Rc<str> = std::rc::Rc::<str>::from([&*v43, &*v41, &*v44, &*v36, &*v36].concat());
        v45.clone()
    };
    let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" label="); } LIT.with(|lit| lit.clone()) };
    let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("label"); } LIT.with(|lit| lit.clone()) };
    let mut v49: Rc<str> = job_file_14(v1.clone(), v48.clone());
    let mut v50: Rc<str> = text_or_empty_15(v49.clone());
    let mut v51: Rc<str> = std::rc::Rc::<str>::from([&*v47, &*v50, &*v36, &*v36, &*v36].concat());
    let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory_cap_mb"); } LIT.with(|lit| lit.clone()) };
    let mut v53: Rc<str> = job_file_14(v1.clone(), v52.clone());
    let mut v54: Rc<str> = text_or_empty_15(v53.clone());
    let mut v55: bool = v54.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v66: Rc<str> = if v55 {
        v36.clone()
    } else {
        let mut v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" max_private_mb="); } LIT.with(|lit| lit.clone()) };
        let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" peak_private_mb="); } LIT.with(|lit| lit.clone()) };
        let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("peak_private_mb"); } LIT.with(|lit| lit.clone()) };
        let mut v59: Rc<str> = job_file_14(v1.clone(), v58.clone());
        let mut v60: bool = file_exists_22(v59.clone());
        let mut v64: Rc<str> = if v60 {
            let mut v61: Rc<str> = job_file_14(v1.clone(), v58.clone());
            text_or_empty_15(v61.clone())
        } else {
            let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
            v63.clone()
        };
        let mut v65: Rc<str> = std::rc::Rc::<str>::from([&*v56, &*v54, &*v57, &*v64, &*v36].concat());
        v65.clone()
    };
    let mut v67: Rc<str> = std::rc::Rc::<str>::from([&*v66, &*v51, &*v36, &*v36, &*v36].concat());
    let mut v68: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v25, &*v37, &*v46, &*v67].concat());
    v68.clone()
}
fn stdout_line_30(mut v0: Rc<str>) -> () {
    println!("{}", &*v0);
    ()
}
fn last_lines_31(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let mut v2: Rc<str> = { let text = std::fs::read_to_string(&*v0).unwrap_or_default(); let lines = text.lines().collect::<Vec<_>>(); let from = lines.len().saturating_sub(v1 as usize); std::rc::Rc::<str>::from(lines[from..].join("
")) };
    v2.clone()
}
fn job_tailed_0() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v1.starts_with(&*v2);
    let mut v6: bool = if v3 {
        let mut v4: Rc<str> = job_directory_3(v1.clone());
        directory_exists_6(v4.clone())
    } else {
        false
    };
    if v6 {
        let mut v7: i32 = argument_count_7();
        let mut v8: bool = v7 > 4i32;
        let mut v12: u64 = if v8 {
            let mut v9: i32 = 4i32;
            let mut v10: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v9 as usize).unwrap_or_default());
            decimal_u64_9(v10.clone())
        } else {
            20u64
        };
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tail"); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<str> = job_summary_11(v13.clone(), v1.clone());
        stdout_line_30(v14.clone());
        let mut v15: Rc<str> = decimal_text_28(v12);
        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--- stdout (last "); } LIT.with(|lit| lit.clone()) };
        let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v15));
        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" lines)\n"); } LIT.with(|lit| lit.clone()) };
        let mut v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stdout.log"); } LIT.with(|lit| lit.clone()) };
        let mut v21: Rc<str> = job_file_14(v1.clone(), v20.clone());
        let mut v22: Rc<str> = last_lines_31(v21.clone(), v12);
        let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v22));
        stdout_line_30(v23.clone());
        let mut v24: Rc<str> = decimal_text_28(v12);
        let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--- stderr (last "); } LIT.with(|lit| lit.clone()) };
        let mut v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v24));
        let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v18));
        let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stderr.log"); } LIT.with(|lit| lit.clone()) };
        let mut v29: Rc<str> = job_file_14(v1.clone(), v28.clone());
        let mut v30: Rc<str> = last_lines_31(v29.clone(), v12);
        let mut v31: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v30));
        stdout_line_30(v31.clone());
        0i32
    } else {
        let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tail: unknown job id "); } LIT.with(|lit| lit.clone()) };
        let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v1));
        rejected_1(v33.clone())
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        job_tailed_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn waited_34(mut v0: Rc<str>, mut v1: u64, mut v2: u64) -> US1 {
    loop {
        let mut v3: US1 = job_state_now_12(v0.clone());
        let mut v4: bool = state_settled_23(v3.clone());
        let mut v7: bool = if v4 {
            true
        } else {
            let mut v5: u64 = now_ms_16();
            let mut v6: bool = v5 > v1;
            v6
        };
        let mut v10: bool = if v7 {
            true
        } else {
            let mut v8: bool = process_alive_18(v2);
            let mut v9: bool = v8 == false;
            v9
        };
        if v10 {
            return v3.clone();
        } else {
            let mut v11: u64 = 250u64;
            std::thread::sleep(std::time::Duration::from_millis(v11));
            (v0, v1, v2) = (v0.clone(), v1, v2);
            continue;
        }
    }
}
fn stop_text_33(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0.starts_with(&*v1);
    let mut v5: bool = if v2 {
        let mut v3: Rc<str> = job_directory_3(v0.clone());
        directory_exists_6(v3.clone())
    } else {
        false
    };
    let mut v6: bool = v5 == false;
    if v6 {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: stop: unknown job id "); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v0));
        v8.clone()
    } else {
        let mut v9: US1 = job_state_now_12(v0.clone());
        let mut v20: US4 = match &v9 {
            US1::US1_Live(v10) => {
                let mut v10: US2 = v10.clone();
                US4::US4_StopRequested
            }
            US1::US1_Settled(v12) => {
                let mut v12: US3 = v12.clone();
                let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("is already "); } LIT.with(|lit| lit.clone()) };
                let mut v14: Rc<str> = settled_word_19(v12.clone());
                let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; nothing to stop"); } LIT.with(|lit| lit.clone()) };
                let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v14, &*v15, &*v16, &*v16].concat());
                US4::US4_StopRefused(v17.clone())
            }
        };
        match &v20 {
            US4::US4_StopRefused(v21) => {
                let mut v21: Rc<str> = v21.clone();
                let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: stop: job "); } LIT.with(|lit| lit.clone()) };
                let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v0));
                let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v24));
                let mut v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v21));
                v26.clone()
            }
            US4::US4_StopRequested => {
                let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop"); } LIT.with(|lit| lit.clone()) };
                let mut v28: Rc<str> = job_file_14(v0.clone(), v27.clone());
                let mut v29: u64 = now_ms_16();
                let mut v30: Rc<str> = decimal_text_28(v29);
                let mut v31: Rc<str> = text_written_20(v28.clone(), v30.clone());
                let mut v32: u64 = now_ms_16();
                let mut v33: u64 = v32.wrapping_add(15000u64);
                let mut v34: u64 = 0u64;
                let mut v35: US1 = waited_34(v0.clone(), v33, v34);
                let mut v36: bool = state_settled_23(v35.clone());
                if v36 {
                    job_summary_11(v27.clone(), v0.clone())
                } else {
                    let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: stop: job "); } LIT.with(|lit| lit.clone()) };
                    let mut v39: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v0));
                    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" did not settle within 15 s; its supervisor may predate job stop"); } LIT.with(|lit| lit.clone()) };
                    let mut v41: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v40));
                    v41.clone()
                }
            }
        }
    }
}
fn job_stopped_32() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let mut v2: Rc<str> = stop_text_33(v1.clone());
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: "); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = v2.starts_with(&*v3);
    if v4 {
        let mut v5: Rc<str> = std::rc::Rc::<str>::from(&(v2)[7..]);
        rejected_1(v5.clone())
    } else {
        stdout_line_30(v2.clone());
        0i32
    }
}
fn closure2() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        job_stopped_32()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn listed_36(mut v0: Rc<str>, mut v1: US5, mut v2: u64) -> u64 {
    loop {
        let mut v3: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v5: bool = if v3 {
            true
        } else {
            let mut v4: bool = v2 == 0u64;
            v4
        };
        if v5 {
            return v2;
        } else {
            let mut v6: Rc<str> = std::rc::Rc::<str>::from(v0.lines().next().unwrap_or(""));
            let mut v7: Rc<str> = std::rc::Rc::<str>::from(v0.lines().skip(1).collect::<Vec<_>>().join("
"));
            let mut v8: US1 = job_state_now_12(v6.clone());
            let mut v13: bool = match &v1 {
                US5::US5_SettledPass => {
                    state_settled_23(v8.clone())
                }
                US5::US5_UnsettledPass => {
                    let mut v9: bool = state_settled_23(v8.clone());
                    let mut v10: bool = v9 == false;
                    v10
                }
            };
            if v13 {
                let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list"); } LIT.with(|lit| lit.clone()) };
                let mut v15: Rc<str> = job_summary_11(v14.clone(), v6.clone());
                stdout_line_30(v15.clone());
                let mut v16: u64 = v2.wrapping_sub(1u64);
                (v0, v1, v2) = (v7.clone(), v1.clone(), v16);
                continue;
            } else {
                (v0, v1, v2) = (v7.clone(), v1.clone(), v2);
                continue;
            }
        }
    }
}
fn job_listed_35() -> i32 {
    let mut v0: i32 = argument_count_7();
    let mut v1: bool = v0 > 3i32;
    let mut v5: u64 = if v1 {
        let mut v2: i32 = 3i32;
        let mut v3: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v2 as usize).unwrap_or_default());
        decimal_u64_9(v3.clone())
    } else {
        20u64
    };
    let mut v6: Rc<str> = jobs_root_5();
    let mut v7: bool = v5 > 120u64;
    let mut v8: u64 = if v7 {
        v5
    } else {
        120u64
    };
    let mut v9: Rc<str> = { let mut names = std::fs::read_dir(&*v6).map(|entries| entries.filter_map(|entry| entry.ok()).filter(|entry| entry.path().is_dir()).map(|entry| entry.file_name().to_string_lossy().into_owned()).filter(|name| name.starts_with("job-")).collect::<Vec<_>>()).unwrap_or_default(); names.sort(); names.reverse(); names.truncate(v8 as usize); std::rc::Rc::<str>::from(names.join("
")) };
    let mut v10: US5 = US5::US5_UnsettledPass;
    let mut v11: u64 = 120u64;
    let mut v12: u64 = listed_36(v9.clone(), v10.clone(), v11);
    let mut v13: u64 = 120u64.wrapping_sub(v12);
    let mut v14: bool = v13 < v5;
    let mut v18: u64 = if v14 {
        let mut v15: US5 = US5::US5_SettledPass;
        let mut v16: u64 = v5.wrapping_sub(v13);
        listed_36(v9.clone(), v15.clone(), v16)
    } else {
        0u64
    };
    0i32
}
fn closure3() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        job_listed_35()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure4() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        stop_text_33(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn launched_text_37(mut v0: Rc<str>, mut v1: u64, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let mut v4: Rc<str> = eoie_job_launch::eoie_job_launched_text(&*v0, v1, &*v2, &*v3);
    v4.clone()
}
fn closure5() -> Rc<dyn Fn(Rc<str>, u64, Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, u64, Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: u64, mut v2: Rc<str>, mut v3: Rc<str>| -> Rc<str> {
        launched_text_37(v0.clone(), v1, v2.clone(), v3.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn parent_pid_40() -> u64 {
    let mut v0: u64 = { #[cfg(windows)] fn parent() -> u64 { #[repr(C)] struct Entry { size: u32, usage: u32, pid: u32, heap: usize, module: u32, threads: u32, parent: u32, base: i32, flags: u32, exe: [u16; 260] } unsafe extern "system" { fn CreateToolhelp32Snapshot(flags: u32, pid: u32) -> *mut std::ffi::c_void; fn Process32FirstW(snapshot: *mut std::ffi::c_void, entry: *mut Entry) -> i32; fn Process32NextW(snapshot: *mut std::ffi::c_void, entry: *mut Entry) -> i32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; } let me = std::process::id(); unsafe { let snapshot = CreateToolhelp32Snapshot(2, 0); if snapshot as isize == -1 { return 0; } let mut entry: Entry = std::mem::zeroed(); entry.size = std::mem::size_of::<Entry>() as u32; let mut found = 0u64; let mut more = Process32FirstW(snapshot, &mut entry) != 0; while more { if entry.pid == me { found = entry.parent as u64; break; } more = Process32NextW(snapshot, &mut entry) != 0; } CloseHandle(snapshot); found } } #[cfg(not(windows))] fn parent() -> u64 { std::os::unix::process::parent_id() as u64 } parent() };
    v0
}
fn watched_caller_39() -> u64 {
    let mut v0: u64 = std::env::var("EOIE_JOB_WATCH").ok().and_then(|value| value.parse::<u64>().ok()).unwrap_or(0);
    let mut v1: bool = v0 == 0u64;
    if v1 {
        parent_pid_40()
    } else {
        v0
    }
}
fn settled_text_38(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let mut v2: u64 = now_ms_16();
    let mut v3: u64 = v2.wrapping_add(v1);
    let mut v4: u64 = watched_caller_39();
    let mut v5: US1 = waited_34(v0.clone(), v3, v4);
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = job_summary_11(v6.clone(), v0.clone());
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" pid="); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pid"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = job_file_14(v0.clone(), v10.clone());
    let mut v12: Rc<str> = text_or_empty_15(v11.clone());
    let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v12));
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" timeout_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout_ms"); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = job_file_14(v0.clone(), v16.clone());
    let mut v18: Rc<str> = text_or_empty_15(v17.clone());
    let mut v19: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v18));
    v19.clone()
}
fn closure6() -> Rc<dyn Fn(Rc<str>, u64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, u64) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: u64| -> Rc<str> {
        settled_text_38(v0.clone(), v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure7() -> Rc<dyn Fn(Rc<str>, u64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, u64) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: u64| -> Rc<str> {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stdout.log"); } LIT.with(|lit| lit.clone()) };
        let mut v3: Rc<str> = job_file_14(v0.clone(), v2.clone());
        last_lines_31(v3.clone(), v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn newest_labeled_41(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = jobs_root_5();
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_dir(&*v1).into_iter().flatten().flatten().filter(|entry| std::fs::read_to_string(entry.path().join("label")).map(|text| text.trim() == &*v0).unwrap_or(false)).map(|entry| entry.file_name().to_string_lossy().into_owned()).max_by_key(|name| name.split(char::from(45u8)).nth(1).and_then(|stamp| stamp.parse::<u64>().ok()).unwrap_or(0)).unwrap_or_default());
    v2.clone()
}
fn closure8() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        newest_labeled_41(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure9() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        let mut v1: US1 = job_state_now_12(v0.clone());
        state_word_24(v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_job_tail_run() -> i32 {
    closure0()()
}
pub fn eoie_job_stop_run() -> i32 {
    closure2()()
}
pub fn eoie_job_list_run() -> i32 {
    closure3()()
}
pub fn eoie_job_stop_text(v0: &str) -> Rc<str> {
    closure4()(Rc::<str>::from(v0))
}
pub fn eoie_job_launched(v0: &str, v1: u64, v2: &str, v3: &str) -> Rc<str> {
    closure5()(Rc::<str>::from(v0), v1, Rc::<str>::from(v2), Rc::<str>::from(v3))
}
pub fn eoie_job_settled(v0: &str, v1: u64) -> Rc<str> {
    closure6()(Rc::<str>::from(v0), v1)
}
pub fn eoie_job_output(v0: &str, v1: u64) -> Rc<str> {
    closure7()(Rc::<str>::from(v0), v1)
}
pub fn eoie_job_labeled(v0: &str) -> Rc<str> {
    closure8()(Rc::<str>::from(v0))
}
pub fn eoie_job_state_word(v0: &str) -> Rc<str> {
    closure9()(Rc::<str>::from(v0))
}
