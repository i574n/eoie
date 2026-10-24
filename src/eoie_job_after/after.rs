#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_AfterCleared,
    US0_AfterHalted(i32),
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
    US4_Some(u64),
    US4_None,
}
fn joined_2(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2.clone()
}
fn jobs_root_4() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("EOIE_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("eoie").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("eoie").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".eoie-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn job_directory_3(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = jobs_root_4();
    joined_2(v1.clone(), v0.clone())
}
fn job_file_1(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = job_directory_3(v0.clone());
    joined_2(v2.clone(), v1.clone())
}
fn text_or_empty_5(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1.clone()
}
fn now_ms_7() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn file_exists_9(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.is_file();
    v8
}
fn method13(mut v0: Option<u64>) -> Option<u64> {
    v0.clone()
}
fn closure1() -> Rc<dyn Fn((u64)) -> US4> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US4> = Rc::new(move |mut v0: (u64)| -> US4 {
        let mut v1: u64 = (v0);
        US4::US4_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_12(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method13(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US4> = closure1();
    let mut v4: Option<US4> = v2.map(|x| v3(x));
    let mut v5: US4 = US4::US4_None;
    let mut v6: US4 = v4.unwrap_or(v5);
    match &v6 {
        US4::US4_None => {
            0u64
        }
        US4::US4_Some(v7) => {
            let mut v7: u64 = *v7;
            v7
        }
    }
}
fn job_record_of_11(mut v0: Rc<str>) -> (Rc<str>, u64, u64, u64) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = job_file_1(v0.clone(), v1.clone());
    let mut v3: Rc<str> = text_or_empty_5(v2.clone());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = job_file_1(v0.clone(), v4.clone());
    let mut v6: Rc<str> = text_or_empty_5(v5.clone());
    let mut v7: u64 = decimal_u64_12(v6.clone());
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("started_ms"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = job_file_1(v0.clone(), v8.clone());
    let mut v10: Rc<str> = text_or_empty_5(v9.clone());
    let mut v11: u64 = decimal_u64_12(v10.clone());
    let mut v12: u64 = now_ms_7();
    (v3.clone(), v7, v11, v12)
}
fn job_state_of_14(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: u64) -> US1 {
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
fn process_alive_15(mut v0: u64) -> bool {
    let mut v1: bool = { #[cfg(windows)] fn alive(pid: u64) -> bool { unsafe extern "system" { fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void; fn WaitForSingleObject(handle: *mut std::ffi::c_void, ms: u32) -> u32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; fn GetLastError() -> u32; } if pid == 0 { return true; } unsafe { let handle = OpenProcess(0x0010_0000, 0, pid as u32); if handle.is_null() { return GetLastError() == 5; } let running = WaitForSingleObject(handle, 0) == 258; CloseHandle(handle); running } } #[cfg(not(windows))] fn alive(pid: u64) -> bool { pid == 0 || std::os::unix::process::parent_id() as u64 == pid } alive(v0) };
    v1
}
fn settled_word_16(mut v0: US3) -> Rc<str> {
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
fn text_written_17(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2.clone()
}
fn job_state_now_10(mut v0: Rc<str>) -> US1 {
    let (mut v1, mut v2, mut v3, mut v4): (Rc<str>, u64, u64, u64) = job_record_of_11(v0.clone());
    let mut v5: US1 = job_state_of_14(v1.clone(), v2, v3, v4);
    match &v5 {
        US1::US1_Settled(v6) => {
            let mut v6: US3 = v6.clone();
            match &v6 {
                US3::US3_JobLost => {
                    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("supervisor"); } LIT.with(|lit| lit.clone()) };
                    let mut v8: Rc<str> = job_file_1(v0.clone(), v7.clone());
                    let mut v9: Rc<str> = text_or_empty_5(v8.clone());
                    let mut v10: u64 = decimal_u64_12(v9.clone());
                    let mut v11: bool = v10 != 0u64;
                    let mut v13: bool = if v11 {
                        process_alive_15(v10)
                    } else {
                        false
                    };
                    if v13 {
                        let mut v14: US2 = US2::US2_JobRunning;
                        US1::US1_Live(v14.clone())
                    } else {
                        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pid"); } LIT.with(|lit| lit.clone()) };
                        let mut v17: Rc<str> = job_file_1(v0.clone(), v16.clone());
                        let mut v18: Rc<str> = text_or_empty_5(v17.clone());
                        let mut v19: u64 = decimal_u64_12(v18.clone());
                        let mut v20: bool = v19 == 0u64;
                        let mut v23: bool = if v20 {
                            true
                        } else {
                            let mut v21: bool = process_alive_15(v19);
                            let mut v22: bool = v21 == false;
                            v22
                        };
                        let mut v24: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v25: bool = v24 && v23;
                        let mut v42: Rc<str> = if v25 {
                            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
                            let mut v27: Rc<str> = job_file_1(v0.clone(), v26.clone());
                            let mut v28: bool = v2 > 0u64;
                            let mut v29: u64 = if v28 {
                                v2
                            } else {
                                v3
                            };
                            let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the supervisor stopped beating: it was killed (with its caller, by the memory reaper) or crashed, and its job tree ended with it"); } LIT.with(|lit| lit.clone()) };
                            let mut v31: US3 = US3::US3_JobLost;
                            let mut v32: Rc<str> = settled_word_16(v31.clone());
                            let mut v33: Rc<str> = std::rc::Rc::<str>::from((125i32).to_string());
                            let mut v34: Rc<str> = std::rc::Rc::<str>::from((v29).to_string());
                            let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                            let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let mut v38: Rc<str> = std::rc::Rc::<str>::from([&*v32, &*v35, &*v30, &*v36, &*v37].concat());
                            let mut v39: Rc<str> = std::rc::Rc::<str>::from([&*v33, &*v35, &*v34, &*v35, &*v38].concat());
                            text_written_17(v27.clone(), v39.clone())
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
fn state_settled_18(mut v0: US1) -> bool {
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
fn decimal_text_19(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1.clone()
}
fn awaited_8(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64) -> (US1, bool) {
    loop {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop"); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<str> = job_file_1(v0.clone(), v3.clone());
        let mut v5: bool = file_exists_9(v4.clone());
        if v5 {
            let mut v6: US3 = US3::US3_JobStopped;
            let mut v7: US1 = US1::US1_Settled(v6.clone());
            return (v7.clone(), true);
        } else {
            let mut v8: US1 = job_state_now_10(v1.clone());
            let mut v9: bool = state_settled_18(v8.clone());
            if v9 {
                return (v8.clone(), false);
            } else {
                let mut v10: u64 = now_ms_7();
                let mut v11: u64 = v2.wrapping_add(1000u64);
                let mut v12: bool = v10 > v11;
                let mut v17: u64 = if v12 {
                    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
                    let mut v14: Rc<str> = job_file_1(v0.clone(), v13.clone());
                    let mut v15: Rc<str> = decimal_text_19(v10);
                    let mut v16: Rc<str> = text_written_17(v14.clone(), v15.clone());
                    v10
                } else {
                    v2
                };
                let mut v18: u64 = 250u64;
                std::thread::sleep(std::time::Duration::from_millis(v18));
                (v0, v1, v2) = (v0.clone(), v1.clone(), v17);
                continue;
            }
        }
    }
}
fn settled_exit_21(mut v0: US3) -> i32 {
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
fn state_exit_20(mut v0: US1) -> i32 {
    match &v0 {
        US1::US1_Live(v1) => {
            let mut v1: US2 = v1.clone();
            3i32
        }
        US1::US1_Settled(v2) => {
            let mut v2: US3 = v2.clone();
            settled_exit_21(v2.clone())
        }
    }
}
fn live_word_23(mut v0: US2) -> Rc<str> {
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
fn state_word_22(mut v0: US1) -> Rc<str> {
    match &v0 {
        US1::US1_Live(v1) => {
            let mut v1: US2 = v1.clone();
            live_word_23(v1.clone())
        }
        US1::US1_Settled(v3) => {
            let mut v3: US3 = v3.clone();
            settled_word_16(v3.clone())
        }
    }
}
fn after_outcome_of_6(mut v0: Rc<str>, mut v1: Rc<str>) -> US0 {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from((v1).split(char::from(9u8)).nth(0i32 as usize).unwrap_or("").trim());
    let mut v3: u64 = now_ms_7();
    let mut v4: u64 = 0u64;
    let (mut v5, mut v6): (US1, bool) = awaited_8(v0.clone(), v2.clone(), v4);
    if v6 {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = job_file_1(v0.clone(), v7.clone());
        let mut v9: u64 = now_ms_7();
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped on request while waiting for "); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v2));
        let mut v12: US3 = US3::US3_JobStopped;
        let mut v13: Rc<str> = settled_word_16(v12.clone());
        let mut v14: Rc<str> = std::rc::Rc::<str>::from((143i32).to_string());
        let mut v15: Rc<str> = std::rc::Rc::<str>::from((v9).to_string());
        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v19: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v16, &*v11, &*v17, &*v18].concat());
        let mut v20: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v16, &*v15, &*v16, &*v19].concat());
        let mut v21: Rc<str> = text_written_17(v8.clone(), v20.clone());
        US0::US0_AfterHalted(143i32)
    } else {
        let mut v23: Rc<str> = std::rc::Rc::<str>::from((v1).split(char::from(9u8)).nth(1i32 as usize).unwrap_or("").trim());
        let mut v24: bool = v23.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
        let mut v27: bool = if v24 {
            let mut v25: i32 = state_exit_20(v5.clone());
            let mut v26: bool = v25 != 0i32;
            v26
        } else {
            false
        };
        if v27 {
            let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--after-ok: "); } LIT.with(|lit| lit.clone()) };
            let mut v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v2));
            let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ended "); } LIT.with(|lit| lit.clone()) };
            let mut v31: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v30));
            let mut v32: Rc<str> = state_word_22(v5.clone());
            let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
            let mut v34: i32 = state_exit_20(v5.clone());
            let mut v35: Rc<str> = std::rc::Rc::<str>::from((v34).to_string().as_str());
            let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" exit "); } LIT.with(|lit| lit.clone()) };
            let mut v37: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v35));
            let mut v38: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v37));
            let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
            let mut v40: Rc<str> = job_file_1(v0.clone(), v39.clone());
            let mut v41: u64 = now_ms_7();
            let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v43: US3 = US3::US3_JobSkipped(v42.clone());
            let mut v44: Rc<str> = settled_word_16(v43.clone());
            let mut v45: Rc<str> = std::rc::Rc::<str>::from((125i32).to_string());
            let mut v46: Rc<str> = std::rc::Rc::<str>::from((v41).to_string());
            let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v49: Rc<str> = std::rc::Rc::<str>::from([&*v44, &*v47, &*v38, &*v48, &*v42].concat());
            let mut v50: Rc<str> = std::rc::Rc::<str>::from([&*v45, &*v47, &*v46, &*v47, &*v49].concat());
            let mut v51: Rc<str> = text_written_17(v40.clone(), v50.clone());
            US0::US0_AfterHalted(125i32)
        } else {
            let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after-done"); } LIT.with(|lit| lit.clone()) };
            let mut v54: Rc<str> = job_file_1(v0.clone(), v53.clone());
            let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
            let mut v56: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v55));
            let mut v57: Rc<str> = state_word_22(v5.clone());
            let mut v58: Rc<str> = Rc::<str>::from(format!("{}{}", v56, v57));
            let mut v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" waited_ms="); } LIT.with(|lit| lit.clone()) };
            let mut v60: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v59));
            let mut v61: u64 = now_ms_7();
            let mut v62: u64 = v61.wrapping_sub(v3);
            let mut v63: Rc<str> = decimal_text_19(v62);
            let mut v64: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v63));
            let mut v65: Rc<str> = text_written_17(v54.clone(), v64.clone());
            US0::US0_AfterCleared
        }
    }
}
fn after_wait_0(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = job_file_1(v0.clone(), v1.clone());
    let mut v3: Rc<str> = text_or_empty_5(v2.clone());
    let mut v4: US0 = after_outcome_of_6(v0.clone(), v3.clone());
    match &v4 {
        US0::US0_AfterCleared => {
            0i32
        }
        US0::US0_AfterHalted(v5) => {
            let mut v5: i32 = *v5;
            v5
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        after_wait_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_job_after_wait(v0: &str) -> i32 {
    closure0()(Rc::<str>::from(v0))
}
