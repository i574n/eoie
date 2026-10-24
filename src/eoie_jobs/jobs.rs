#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US1 {
    US1_JobPending,
    US1_JobRunning,
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
#[derive(Clone)]
enum US0 {
    US0_Live(US1),
    US0_Settled(US2),
}
#[derive(Clone)]
enum US3 {
    US3_Some(u64),
    US3_None,
}
#[derive(Clone)]
enum US4 {
    US4_JobStart,
    US4_JobRun,
    US4_JobSupervise,
    US4_JobHost,
    US4_JobStatus,
    US4_JobWait,
    US4_JobList,
    US4_JobTail,
    US4_JobStop,
    US4_JobUnknown,
}
fn joined_6(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2.clone()
}
fn jobs_root_8() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("EOIE_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("eoie").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("eoie").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".eoie-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn job_directory_7(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = jobs_root_8();
    joined_6(v1.clone(), v0.clone())
}
fn job_file_5(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = job_directory_7(v0.clone());
    joined_6(v2.clone(), v1.clone())
}
fn text_or_empty_9(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1.clone()
}
fn method11(mut v0: Option<u64>) -> Option<u64> {
    v0.clone()
}
fn closure1() -> Rc<dyn Fn((u64)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US3> = Rc::new(move |mut v0: (u64)| -> US3 {
        let mut v1: u64 = (v0);
        US3::US3_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_10(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method11(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US3> = closure1();
    let mut v4: Option<US3> = v2.map(|x| v3(x));
    let mut v5: US3 = US3::US3_None;
    let mut v6: US3 = v4.unwrap_or(v5);
    match &v6 {
        US3::US3_None => {
            0u64
        }
        US3::US3_Some(v7) => {
            let mut v7: u64 = *v7;
            v7
        }
    }
}
fn now_ms_12() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn job_record_of_4(mut v0: Rc<str>) -> (Rc<str>, u64, u64, u64) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = job_file_5(v0.clone(), v1.clone());
    let mut v3: Rc<str> = text_or_empty_9(v2.clone());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = job_file_5(v0.clone(), v4.clone());
    let mut v6: Rc<str> = text_or_empty_9(v5.clone());
    let mut v7: u64 = decimal_u64_10(v6.clone());
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("started_ms"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = job_file_5(v0.clone(), v8.clone());
    let mut v10: Rc<str> = text_or_empty_9(v9.clone());
    let mut v11: u64 = decimal_u64_10(v10.clone());
    let mut v12: u64 = now_ms_12();
    (v3.clone(), v7, v11, v12)
}
fn job_state_of_13(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: u64) -> US0 {
    let mut v4: bool = v0.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v4 {
        let mut v5: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(2).unwrap_or("").trim());
        let mut v6: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished"); } LIT.with(|lit| lit.clone()) };
        if v6 {
            let mut v7: i32 = v0.split(char::from(9u8)).next().and_then(|code| code.trim().parse::<i32>().ok()).unwrap_or(1);
            let mut v8: US2 = US2::US2_JobFinished(v7);
            US0::US0_Settled(v8.clone())
        } else {
            let mut v10: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed-out"); } LIT.with(|lit| lit.clone()) };
            if v10 {
                let mut v11: US2 = US2::US2_JobTimedOut;
                US0::US0_Settled(v11.clone())
            } else {
                let mut v13: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped"); } LIT.with(|lit| lit.clone()) };
                if v13 {
                    let mut v14: US2 = US2::US2_JobStopped;
                    US0::US0_Settled(v14.clone())
                } else {
                    let mut v16: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lost"); } LIT.with(|lit| lit.clone()) };
                    if v16 {
                        let mut v17: US2 = US2::US2_JobLost;
                        US0::US0_Settled(v17.clone())
                    } else {
                        let mut v19: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory-capped"); } LIT.with(|lit| lit.clone()) };
                        if v19 {
                            let mut v20: US2 = US2::US2_JobMemoryCapped;
                            US0::US0_Settled(v20.clone())
                        } else {
                            let mut v22: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expired-in-queue"); } LIT.with(|lit| lit.clone()) };
                            if v22 {
                                let mut v23: US2 = US2::US2_JobExpiredInQueue;
                                US0::US0_Settled(v23.clone())
                            } else {
                                let mut v25: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("skipped"); } LIT.with(|lit| lit.clone()) };
                                if v25 {
                                    let mut v26: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(3).unwrap_or("").trim());
                                    let mut v27: US2 = US2::US2_JobSkipped(v26.clone());
                                    US0::US0_Settled(v27.clone())
                                } else {
                                    let mut v29: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(3).unwrap_or("").trim());
                                    let mut v30: US2 = US2::US2_JobSpawnFailed(v29.clone());
                                    US0::US0_Settled(v30.clone())
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
                let mut v42: US2 = US2::US2_JobLost;
                US0::US0_Settled(v42.clone())
            } else {
                let mut v44: US1 = US1::US1_JobRunning;
                US0::US0_Live(v44.clone())
            }
        } else {
            let mut v47: u64 = v2.wrapping_add(15000u64);
            let mut v48: bool = v47 < v3;
            if v48 {
                let mut v49: US2 = US2::US2_JobLost;
                US0::US0_Settled(v49.clone())
            } else {
                let mut v51: US1 = US1::US1_JobPending;
                US0::US0_Live(v51.clone())
            }
        }
    }
}
fn process_alive_14(mut v0: u64) -> bool {
    let mut v1: bool = { #[cfg(windows)] fn alive(pid: u64) -> bool { unsafe extern "system" { fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void; fn WaitForSingleObject(handle: *mut std::ffi::c_void, ms: u32) -> u32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; fn GetLastError() -> u32; } if pid == 0 { return true; } unsafe { let handle = OpenProcess(0x0010_0000, 0, pid as u32); if handle.is_null() { return GetLastError() == 5; } let running = WaitForSingleObject(handle, 0) == 258; CloseHandle(handle); running } } #[cfg(not(windows))] fn alive(pid: u64) -> bool { pid == 0 || std::os::unix::process::parent_id() as u64 == pid } alive(v0) };
    v1
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
fn text_written_16(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2.clone()
}
fn job_state_now_3(mut v0: Rc<str>) -> US0 {
    let (mut v1, mut v2, mut v3, mut v4): (Rc<str>, u64, u64, u64) = job_record_of_4(v0.clone());
    let mut v5: US0 = job_state_of_13(v1.clone(), v2, v3, v4);
    match &v5 {
        US0::US0_Settled(v6) => {
            let mut v6: US2 = v6.clone();
            match &v6 {
                US2::US2_JobLost => {
                    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("supervisor"); } LIT.with(|lit| lit.clone()) };
                    let mut v8: Rc<str> = job_file_5(v0.clone(), v7.clone());
                    let mut v9: Rc<str> = text_or_empty_9(v8.clone());
                    let mut v10: u64 = decimal_u64_10(v9.clone());
                    let mut v11: bool = v10 != 0u64;
                    let mut v13: bool = if v11 {
                        process_alive_14(v10)
                    } else {
                        false
                    };
                    if v13 {
                        let mut v14: US1 = US1::US1_JobRunning;
                        US0::US0_Live(v14.clone())
                    } else {
                        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pid"); } LIT.with(|lit| lit.clone()) };
                        let mut v17: Rc<str> = job_file_5(v0.clone(), v16.clone());
                        let mut v18: Rc<str> = text_or_empty_9(v17.clone());
                        let mut v19: u64 = decimal_u64_10(v18.clone());
                        let mut v20: bool = v19 == 0u64;
                        let mut v23: bool = if v20 {
                            true
                        } else {
                            let mut v21: bool = process_alive_14(v19);
                            let mut v22: bool = v21 == false;
                            v22
                        };
                        let mut v24: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v25: bool = v24 && v23;
                        let mut v42: Rc<str> = if v25 {
                            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
                            let mut v27: Rc<str> = job_file_5(v0.clone(), v26.clone());
                            let mut v28: bool = v2 > 0u64;
                            let mut v29: u64 = if v28 {
                                v2
                            } else {
                                v3
                            };
                            let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the supervisor stopped beating: it was killed (with its caller, by the memory reaper) or crashed, and its job tree ended with it"); } LIT.with(|lit| lit.clone()) };
                            let mut v31: US2 = US2::US2_JobLost;
                            let mut v32: Rc<str> = settled_word_15(v31.clone());
                            let mut v33: Rc<str> = std::rc::Rc::<str>::from((125i32).to_string());
                            let mut v34: Rc<str> = std::rc::Rc::<str>::from((v29).to_string());
                            let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                            let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let mut v38: Rc<str> = std::rc::Rc::<str>::from([&*v32, &*v35, &*v30, &*v36, &*v37].concat());
                            let mut v39: Rc<str> = std::rc::Rc::<str>::from([&*v33, &*v35, &*v34, &*v35, &*v38].concat());
                            text_written_16(v27.clone(), v39.clone())
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
fn file_exists_18(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.is_file();
    v8
}
fn gate_queued_17(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = job_file_5(v0.clone(), v1.clone());
    let mut v3: bool = file_exists_18(v2.clone());
    let mut v8: bool = if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after-done"); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = job_file_5(v0.clone(), v4.clone());
        let mut v6: bool = file_exists_18(v5.clone());
        let mut v7: bool = v6 == false;
        v7
    } else {
        false
    };
    if v8 {
        true
    } else {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("window-class"); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = job_file_5(v0.clone(), v9.clone());
        let mut v11: bool = file_exists_18(v10.clone());
        if v11 {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("window"); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = job_file_5(v0.clone(), v12.clone());
            let mut v14: Rc<str> = text_or_empty_9(v13.clone());
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("granted"); } LIT.with(|lit| lit.clone()) };
            let mut v16: bool = v14.starts_with(&*v15);
            let mut v17: bool = v16 == false;
            v17
        } else {
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-wrapper.ps1"); } LIT.with(|lit| lit.clone()) };
            let mut v19: Rc<str> = job_file_5(v0.clone(), v18.clone());
            let mut v20: bool = file_exists_18(v19.clone());
            if v20 {
                let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stdout.log"); } LIT.with(|lit| lit.clone()) };
                let mut v22: Rc<str> = job_file_5(v0.clone(), v21.clone());
                let mut v23: Rc<str> = text_or_empty_9(v22.clone());
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
fn state_settled_19(mut v0: US0) -> bool {
    match &v0 {
        US0::US0_Live(v1) => {
            let mut v1: US1 = v1.clone();
            false
        }
        US0::US0_Settled(v2) => {
            let mut v2: US2 = v2.clone();
            true
        }
    }
}
fn live_word_21(mut v0: US1) -> Rc<str> {
    match &v0 {
        US1::US1_JobPending => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pending"); } LIT.with(|lit| lit.clone()) };
            v1.clone()
        }
        US1::US1_JobRunning => {
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("running"); } LIT.with(|lit| lit.clone()) };
            v2.clone()
        }
    }
}
fn state_word_20(mut v0: US0) -> Rc<str> {
    match &v0 {
        US0::US0_Live(v1) => {
            let mut v1: US1 = v1.clone();
            live_word_21(v1.clone())
        }
        US0::US0_Settled(v3) => {
            let mut v3: US2 = v3.clone();
            settled_word_15(v3.clone())
        }
    }
}
fn settled_exit_22(mut v0: US2) -> i32 {
    match &v0 {
        US2::US2_JobExpiredInQueue => {
            75i32
        }
        US2::US2_JobFinished(v1) => {
            let mut v1: i32 = *v1;
            v1
        }
        US2::US2_JobLost => {
            125i32
        }
        US2::US2_JobMemoryCapped => {
            137i32
        }
        US2::US2_JobSkipped(v3) => {
            let mut v3: Rc<str> = v3.clone();
            125i32
        }
        US2::US2_JobSpawnFailed(v2) => {
            let mut v2: Rc<str> = v2.clone();
            125i32
        }
        US2::US2_JobStopped => {
            143i32
        }
        US2::US2_JobTimedOut => {
            124i32
        }
    }
}
fn elapsed_of_23(mut v0: Rc<str>) -> u64 {
    let (mut v1, mut v2, mut v3, mut v4): (Rc<str>, u64, u64, u64) = job_record_of_4(v0.clone());
    let mut v5: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v8: u64 = if v5 {
        v4
    } else {
        let mut v6: Rc<str> = std::rc::Rc::<str>::from(v1.split(char::from(9u8)).nth(1).unwrap_or("0"));
        decimal_u64_10(v6.clone())
    };
    let mut v9: bool = v8 > v3;
    if v9 {
        let mut v10: u64 = v8.wrapping_sub(v3);
        v10
    } else {
        0u64
    }
}
fn decimal_text_24(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1.clone()
}
fn file_bytes_25(mut v0: Rc<str>) -> u64 {
    let mut v1: u64 = std::fs::metadata(&*v0).map(|metadata| metadata.len()).unwrap_or(0);
    v1
}
fn job_summary_2(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: US0 = job_state_now_3(v1.clone());
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie job "); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ok id="); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" state="); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v0, &*v4, &*v1, &*v5].concat());
    let mut v7: bool = gate_queued_17(v1.clone());
    let mut v10: bool = if v7 {
        let mut v8: bool = state_settled_19(v2.clone());
        let mut v9: bool = v8 == false;
        v9
    } else {
        false
    };
    let mut v13: Rc<str> = if v10 {
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("queued"); } LIT.with(|lit| lit.clone()) };
        v11.clone()
    } else {
        state_word_20(v2.clone())
    };
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" exit="); } LIT.with(|lit| lit.clone()) };
    let mut v21: Rc<str> = match &v2 {
        US0::US0_Live(v15) => {
            let mut v15: US1 = v15.clone();
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            v16.clone()
        }
        US0::US0_Settled(v17) => {
            let mut v17: US2 = v17.clone();
            let mut v18: i32 = settled_exit_22(v17.clone());
            let mut v19: Rc<str> = std::rc::Rc::<str>::from((v18).to_string());
            v19.clone()
        }
    };
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v23: u64 = elapsed_of_23(v1.clone());
    let mut v24: Rc<str> = decimal_text_24(v23);
    let mut v25: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v14, &*v21, &*v22, &*v24].concat());
    let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" stdout_bytes="); } LIT.with(|lit| lit.clone()) };
    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stdout.log"); } LIT.with(|lit| lit.clone()) };
    let mut v28: Rc<str> = job_file_5(v1.clone(), v27.clone());
    let mut v29: u64 = file_bytes_25(v28.clone());
    let mut v30: Rc<str> = decimal_text_24(v29);
    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" stderr_bytes="); } LIT.with(|lit| lit.clone()) };
    let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stderr.log"); } LIT.with(|lit| lit.clone()) };
    let mut v33: Rc<str> = job_file_5(v1.clone(), v32.clone());
    let mut v34: u64 = file_bytes_25(v33.clone());
    let mut v35: Rc<str> = decimal_text_24(v34);
    let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v37: Rc<str> = std::rc::Rc::<str>::from([&*v26, &*v30, &*v31, &*v35, &*v36].concat());
    let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
    let mut v39: Rc<str> = job_file_5(v1.clone(), v38.clone());
    let mut v40: Rc<str> = text_or_empty_9(v39.clone());
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
    let mut v49: Rc<str> = job_file_5(v1.clone(), v48.clone());
    let mut v50: Rc<str> = text_or_empty_9(v49.clone());
    let mut v51: Rc<str> = std::rc::Rc::<str>::from([&*v47, &*v50, &*v36, &*v36, &*v36].concat());
    let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory_cap_mb"); } LIT.with(|lit| lit.clone()) };
    let mut v53: Rc<str> = job_file_5(v1.clone(), v52.clone());
    let mut v54: Rc<str> = text_or_empty_9(v53.clone());
    let mut v55: bool = v54.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v66: Rc<str> = if v55 {
        v36.clone()
    } else {
        let mut v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" max_private_mb="); } LIT.with(|lit| lit.clone()) };
        let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" peak_private_mb="); } LIT.with(|lit| lit.clone()) };
        let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("peak_private_mb"); } LIT.with(|lit| lit.clone()) };
        let mut v59: Rc<str> = job_file_5(v1.clone(), v58.clone());
        let mut v60: bool = file_exists_18(v59.clone());
        let mut v64: Rc<str> = if v60 {
            let mut v61: Rc<str> = job_file_5(v1.clone(), v58.clone());
            text_or_empty_9(v61.clone())
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
fn stdout_line_26(mut v0: Rc<str>) -> () {
    println!("{}", &*v0);
    ()
}
fn started_code_1(mut v0: bool) -> i32 {
    let mut v3: Rc<str> = if v0 {
        let mut v1: Rc<str> = eoie_job_launch::eoie_job_start(true);
        v1.clone()
    } else {
        let mut v2: Rc<str> = eoie_job_launch::eoie_job_start(false);
        v2.clone()
    };
    let mut v4: i32 = (v3).split(char::from(9u8)).next().and_then(|code| code.trim().parse::<i32>().ok()).unwrap_or(2);
    let mut v5: Rc<str> = std::rc::Rc::<str>::from((v3).split(char::from(9u8)).nth(1).unwrap_or("").trim());
    let mut v7: bool = if v0 {
        let mut v6: bool = v5.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v6
    } else {
        false
    };
    if v7 {
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("run"); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = job_summary_2(v8.clone(), v5.clone());
        stdout_line_26(v9.clone())
    };
    v4
}
fn stderr_line_29(mut v0: Rc<str>) -> () {
    eprintln!("{}", &*v0);
    ()
}
fn rejected_28(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: job "); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v0, &*v2, &*v2, &*v2].concat());
    stderr_line_29(v3.clone());
    2i32
}
fn directory_exists_30(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.is_dir();
    v8
}
fn job_status_27() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v1.starts_with(&*v2);
    let mut v6: bool = if v3 {
        let mut v4: Rc<str> = job_directory_7(v1.clone());
        directory_exists_30(v4.clone())
    } else {
        false
    };
    if v6 {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = job_summary_2(v7.clone(), v1.clone());
        stdout_line_26(v8.clone());
        0i32
    } else {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status: unknown job id "); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v1, &*v10, &*v10, &*v10].concat());
        rejected_28(v11.clone())
    }
}
fn state_exit_32(mut v0: US0) -> i32 {
    match &v0 {
        US0::US0_Live(v1) => {
            let mut v1: US1 = v1.clone();
            3i32
        }
        US0::US0_Settled(v2) => {
            let mut v2: US2 = v2.clone();
            settled_exit_22(v2.clone())
        }
    }
}
fn command_line_argument_count_34() -> i32 {
    let mut v0: i32 = (std::env::args().count() as i32) - 1;
    v0
}
fn argument_count_33() -> i32 {
    let mut v0: i32 = command_line_argument_count_34();
    let mut v1: i32 = v0.wrapping_add(1i32);
    v1
}
fn parent_pid_36() -> u64 {
    let mut v0: u64 = { #[cfg(windows)] fn parent() -> u64 { #[repr(C)] struct Entry { size: u32, usage: u32, pid: u32, heap: usize, module: u32, threads: u32, parent: u32, base: i32, flags: u32, exe: [u16; 260] } unsafe extern "system" { fn CreateToolhelp32Snapshot(flags: u32, pid: u32) -> *mut std::ffi::c_void; fn Process32FirstW(snapshot: *mut std::ffi::c_void, entry: *mut Entry) -> i32; fn Process32NextW(snapshot: *mut std::ffi::c_void, entry: *mut Entry) -> i32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; } let me = std::process::id(); unsafe { let snapshot = CreateToolhelp32Snapshot(2, 0); if snapshot as isize == -1 { return 0; } let mut entry: Entry = std::mem::zeroed(); entry.size = std::mem::size_of::<Entry>() as u32; let mut found = 0u64; let mut more = Process32FirstW(snapshot, &mut entry) != 0; while more { if entry.pid == me { found = entry.parent as u64; break; } more = Process32NextW(snapshot, &mut entry) != 0; } CloseHandle(snapshot); found } } #[cfg(not(windows))] fn parent() -> u64 { std::os::unix::process::parent_id() as u64 } parent() };
    v0
}
fn watched_caller_35() -> u64 {
    let mut v0: u64 = std::env::var("EOIE_JOB_WATCH").ok().and_then(|value| value.parse::<u64>().ok()).unwrap_or(0);
    let mut v1: bool = v0 == 0u64;
    if v1 {
        parent_pid_36()
    } else {
        v0
    }
}
fn waited_37(mut v0: Rc<str>, mut v1: u64, mut v2: u64) -> US0 {
    loop {
        let mut v3: US0 = job_state_now_3(v0.clone());
        let mut v4: bool = state_settled_19(v3.clone());
        let mut v7: bool = if v4 {
            true
        } else {
            let mut v5: u64 = now_ms_12();
            let mut v6: bool = v5 > v1;
            v6
        };
        let mut v10: bool = if v7 {
            true
        } else {
            let mut v8: bool = process_alive_14(v2);
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
fn job_waited_31() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v1.starts_with(&*v2);
    let mut v6: bool = if v3 {
        let mut v4: Rc<str> = job_directory_7(v1.clone());
        directory_exists_30(v4.clone())
    } else {
        false
    };
    if v6 {
        let mut v7: u64 = now_ms_12();
        let mut v8: i32 = argument_count_33();
        let mut v9: bool = v8 > 4i32;
        let mut v13: u64 = if v9 {
            let mut v10: i32 = 4i32;
            let mut v11: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v10 as usize).unwrap_or_default());
            decimal_u64_10(v11.clone())
        } else {
            864000000u64
        };
        let mut v14: u64 = v7.wrapping_add(v13);
        let mut v15: u64 = watched_caller_35();
        let mut v16: US0 = waited_37(v1.clone(), v14, v15);
        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait"); } LIT.with(|lit| lit.clone()) };
        let mut v18: Rc<str> = job_summary_2(v17.clone(), v1.clone());
        stdout_line_26(v18.clone());
        state_exit_32(v16.clone())
    } else {
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait: unknown job id "); } LIT.with(|lit| lit.clone()) };
        let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v22: Rc<str> = std::rc::Rc::<str>::from([&*v20, &*v1, &*v21, &*v21, &*v21].concat());
        rejected_28(v22.clone())
    }
}
fn job_run_0() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("start"); } LIT.with(|lit| lit.clone()) };
    let mut v29: US4 = if v2 {
        US4::US4_JobStart
    } else {
        let mut v4: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("run"); } LIT.with(|lit| lit.clone()) };
        if v4 {
            US4::US4_JobRun
        } else {
            let mut v6: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("supervise"); } LIT.with(|lit| lit.clone()) };
            if v6 {
                US4::US4_JobSupervise
            } else {
                let mut v8: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host"); } LIT.with(|lit| lit.clone()) };
                if v8 {
                    US4::US4_JobHost
                } else {
                    let mut v10: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
                    if v10 {
                        US4::US4_JobStatus
                    } else {
                        let mut v12: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait"); } LIT.with(|lit| lit.clone()) };
                        if v12 {
                            US4::US4_JobWait
                        } else {
                            let mut v14: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list"); } LIT.with(|lit| lit.clone()) };
                            if v14 {
                                US4::US4_JobList
                            } else {
                                let mut v16: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tail"); } LIT.with(|lit| lit.clone()) };
                                if v16 {
                                    US4::US4_JobTail
                                } else {
                                    let mut v18: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop"); } LIT.with(|lit| lit.clone()) };
                                    if v18 {
                                        US4::US4_JobStop
                                    } else {
                                        US4::US4_JobUnknown
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };
    match &v29 {
        US4::US4_JobHost => {
            let mut v37: i32 = eoie_job_host::eoie_job_host_run();
            v37
        }
        US4::US4_JobList => {
            let mut v40: i32 = eoie_job_control::eoie_job_list_run();
            v40
        }
        US4::US4_JobRun => {
            let mut v32: bool = true;
            started_code_1(v32)
        }
        US4::US4_JobStart => {
            let mut v30: bool = false;
            started_code_1(v30)
        }
        US4::US4_JobStatus => {
            job_status_27()
        }
        US4::US4_JobStop => {
            let mut v42: i32 = eoie_job_control::eoie_job_stop_run();
            v42
        }
        US4::US4_JobSupervise => {
            let mut v34: i32 = 3i32;
            let mut v35: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v34 as usize).unwrap_or_default());
            let mut v36: i32 = eoie_job_supervise::eoie_job_supervise(&*v35);
            v36
        }
        US4::US4_JobTail => {
            let mut v41: i32 = eoie_job_control::eoie_job_tail_run();
            v41
        }
        US4::US4_JobUnknown => {
            let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job start [--after <id>|--after-ok <id>] [--heavy|--compile] [--max-private-mb <n>] [--slot-within <minutes>] [--done-within <minutes>] [--quiet [--quiet-hold <min>] [--quiet-wait <min>]] <label> <timeout-ms> <program> [args...] (the cap covers the whole job tree; reaching it stops the job as memory-capped, exit 137) | job run [--after <id>|--after-ok <id>] [--heavy|--compile] [--max-private-mb <n>] [--slot-within <minutes>] [--done-within <minutes>] [--quiet [--quiet-hold <min>] [--quiet-wait <min>]] <label> <timeout-ms> <program> [args...] (supervises in this process, so the job's processes stay in the caller's tree and end with it; exits with the job's code; for long runs from an agent shell that can be reaped, use job start plus a background job wait) | job status <id> | job wait <id> [max-ms, default until it settles] | job list [count] | job tail <id> [lines] | job stop <id>"); } LIT.with(|lit| lit.clone()) };
            rejected_28(v43.clone())
        }
        US4::US4_JobWait => {
            job_waited_31()
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        job_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_job_run() -> i32 {
    closure0()()
}
