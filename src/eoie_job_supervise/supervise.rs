#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_KeepRunning,
    US0_HaltOnTimeout,
    US0_HaltOnRequest,
    US0_HaltOnMemory,
}
#[derive(Clone)]
enum US1 {
    US1_TreeStopped,
    US1_SurvivorsLeft(u64),
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
enum US3 {
    US3_Some(u64),
    US3_None,
}
fn child_container_4() -> u64 {
    let mut v0: u64 = { #[cfg(windows)] fn container() -> u64 { static CONTAINER: std::sync::OnceLock<u64> = std::sync::OnceLock::new(); *CONTAINER.get_or_init(|| { unsafe extern "system" { fn CreateJobObjectW(attributes: *mut std::ffi::c_void, name: *const u16) -> *mut std::ffi::c_void; fn SetInformationJobObject(job: *mut std::ffi::c_void, class: i32, information: *mut std::ffi::c_void, length: u32) -> i32; } unsafe { let job = CreateJobObjectW(std::ptr::null_mut(), std::ptr::null()); if !job.is_null() { let mut limits = [0u64; 18]; limits[2] = 0x2800; SetInformationJobObject(job, 9, limits.as_mut_ptr().cast(), 144); } job as u64 } }) } #[cfg(not(windows))] fn container() -> u64 { 0 } container() };
    v0
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
fn now_ms_9() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn text_written_10(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2.clone()
}
fn halted_3(mut v0: std::rc::Rc<std::cell::RefCell<std::process::Child>>, mut v1: Rc<str>, mut v2: US0) -> i32 {
    let mut v3: u64 = child_container_4();
    let mut v4: u64 = { let mut child = v0.borrow_mut(); let root = child.id(); #[cfg(windows)] let survivors = { unsafe extern "system" { fn TerminateJobObject(job: *mut std::ffi::c_void, code: u32) -> i32; fn QueryInformationJobObject(job: *mut std::ffi::c_void, class: i32, information: *mut std::ffi::c_void, length: u32, returned: *mut u32) -> i32; } let _ = root; let job = v3 as *mut std::ffi::c_void; if !job.is_null() { unsafe { TerminateJobObject(job, 124); } } let _ = child.kill(); let _ = child.wait(); let active = || -> u64 { let mut accounting = [0u64; 6]; if job.is_null() || unsafe { QueryInformationJobObject(job, 1, accounting.as_mut_ptr().cast(), 48, std::ptr::null_mut()) } == 0 { 0 } else { accounting[5] & 0xffff_ffff } }; let mut waited = 0u64; while active() > 0 && waited < 3000 { std::thread::sleep(std::time::Duration::from_millis(100)); waited += 100; } active() }; #[cfg(unix)] let survivors = { let _ = v3; let _ = std::process::Command::new("kill").arg("-KILL").arg("--").arg(format!("-{root}")).status(); let _ = child.kill(); let _ = child.wait(); std::thread::sleep(std::time::Duration::from_millis(300)); std::fs::read_dir("/proc").map(|entries| entries.filter_map(|entry| entry.ok()).filter(|entry| std::fs::read_to_string(entry.path().join("stat")).ok().and_then(|stat| stat.rsplit_once(char::from(41u8)).map(|(_, rest)| rest.split_whitespace().nth(2) == Some(root.to_string().as_str()))).unwrap_or(false)).count() as u64).unwrap_or(0) }; survivors };
    let mut v5: bool = v4 == 0u64;
    let mut v8: US1 = if v5 {
        US1::US1_TreeStopped
    } else {
        US1::US1_SurvivorsLeft(v4)
    };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = job_file_5(v1.clone(), v9.clone());
    let mut v11: u64 = now_ms_9();
    let (mut v21, mut v22, mut v23): (i32, Rc<str>, Rc<str>) = match &v2 {
        US0::US0_HaltOnMemory => {
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory-capped"); } LIT.with(|lit| lit.clone()) };
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("private memory cap reached"); } LIT.with(|lit| lit.clone()) };
            (137i32, v14.clone(), v15.clone())
        }
        US0::US0_HaltOnRequest => {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped"); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped on request"); } LIT.with(|lit| lit.clone()) };
            (143i32, v12.clone(), v13.clone())
        }
        _ => {
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed-out"); } LIT.with(|lit| lit.clone()) };
            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wall-clock timeout"); } LIT.with(|lit| lit.clone()) };
            (124i32, v16.clone(), v17.clone())
        }
    };
    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
    let mut v32: Rc<str> = match &v8 {
        US1::US1_SurvivorsLeft(v26) => {
            let mut v26: u64 = *v26;
            let mut v27: Rc<str> = std::rc::Rc::<str>::from((v26).to_string());
            let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" processes left alive"); } LIT.with(|lit| lit.clone()) };
            let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v30: Rc<str> = std::rc::Rc::<str>::from([&*v27, &*v28, &*v29, &*v29, &*v29].concat());
            v30.clone()
        }
        US1::US1_TreeStopped => {
            let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("process tree stopped"); } LIT.with(|lit| lit.clone()) };
            v25.clone()
        }
    };
    let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v34: Rc<str> = std::rc::Rc::<str>::from([&*v23, &*v24, &*v32, &*v33, &*v33].concat());
    let mut v35: Rc<str> = std::rc::Rc::<str>::from((v21).to_string());
    let mut v36: Rc<str> = std::rc::Rc::<str>::from((v11).to_string());
    let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v39: Rc<str> = std::rc::Rc::<str>::from([&*v22, &*v37, &*v34, &*v38, &*v33].concat());
    let mut v40: Rc<str> = std::rc::Rc::<str>::from([&*v35, &*v37, &*v36, &*v37, &*v39].concat());
    let mut v41: Rc<str> = text_written_10(v10.clone(), v40.clone());
    let mut v42: u64 = eoie_machine_windows::eoie_job_windows_dropped(&*v1);
    let mut v52: i32 = match &v2 {
        US0::US0_HaltOnMemory => {
            137i32
        }
        US0::US0_HaltOnRequest => {
            143i32
        }
        _ => {
            124i32
        }
    };
    v52
}
fn container_peak_mb_11() -> u64 {
    let mut v0: u64 = child_container_4();
    let mut v1: u64 = { #[cfg(windows)] let peak = { unsafe extern "system" { fn QueryInformationJobObject(job: *mut std::ffi::c_void, class: i32, information: *mut std::ffi::c_void, length: u32, returned: *mut u32) -> i32; } let job = v0 as *mut std::ffi::c_void; let mut limits = [0u64; 18]; if job.is_null() || unsafe { QueryInformationJobObject(job, 9, limits.as_mut_ptr().cast(), 144, std::ptr::null_mut()) } == 0 { 0 } else { limits[17] / 1048576 } }; #[cfg(not(windows))] let peak = { let _ = v0; 0u64 }; peak };
    v1
}
fn decimal_text_12(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1.clone()
}
fn settled_word_13(mut v0: US2) -> Rc<str> {
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
fn file_exists_14(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.is_file();
    v8
}
fn text_or_empty_16(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1.clone()
}
fn gate_queued_15(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = job_file_5(v0.clone(), v1.clone());
    let mut v3: bool = file_exists_14(v2.clone());
    let mut v8: bool = if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after-done"); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = job_file_5(v0.clone(), v4.clone());
        let mut v6: bool = file_exists_14(v5.clone());
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
        let mut v11: bool = file_exists_14(v10.clone());
        if v11 {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("window"); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = job_file_5(v0.clone(), v12.clone());
            let mut v14: Rc<str> = text_or_empty_16(v13.clone());
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("granted"); } LIT.with(|lit| lit.clone()) };
            let mut v16: bool = v14.starts_with(&*v15);
            let mut v17: bool = v16 == false;
            v17
        } else {
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-wrapper.ps1"); } LIT.with(|lit| lit.clone()) };
            let mut v19: Rc<str> = job_file_5(v0.clone(), v18.clone());
            let mut v20: bool = file_exists_14(v19.clone());
            if v20 {
                let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stdout.log"); } LIT.with(|lit| lit.clone()) };
                let mut v22: Rc<str> = job_file_5(v0.clone(), v21.clone());
                let mut v23: Rc<str> = text_or_empty_16(v22.clone());
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
fn supervised_2(mut v0: std::rc::Rc<std::cell::RefCell<std::process::Child>>, mut v1: Rc<str>, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64) -> i32 {
    loop {
        let mut v6: i64 = match v0.clone().borrow_mut().try_wait() { Ok(Some(status)) => status.code().map(|code| code as i64).unwrap_or(-2), Ok(None) => -1, Err(_) => -2 };
        let mut v7: bool = v4 > 0u64;
        let mut v9: u64 = if v7 {
            container_peak_mb_11()
        } else {
            0u64
        };
        let mut v10: bool = v6 != -1i64;
        let mut v15: bool = if v10 {
            if v7 {
                let mut v11: u64 = v9.wrapping_mul(100u64);
                let mut v12: u64 = v4.wrapping_mul(98u64);
                let mut v13: bool = v11 >= v12;
                v13
            } else {
                false
            }
        } else {
            false
        };
        if v15 {
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("peak_private_mb"); } LIT.with(|lit| lit.clone()) };
            let mut v17: Rc<str> = job_file_5(v1.clone(), v16.clone());
            let mut v18: Rc<str> = decimal_text_12(v9);
            let mut v19: Rc<str> = text_written_10(v17.clone(), v18.clone());
            let mut v20: US0 = US0::US0_HaltOnMemory;
            return halted_3(v0.clone(), v1.clone(), v20.clone());
        } else {
            if v10 {
                let mut v22: i32 = v6 as i32;
                let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
                let mut v24: Rc<str> = job_file_5(v1.clone(), v23.clone());
                let mut v25: u64 = now_ms_9();
                let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v27: US2 = US2::US2_JobFinished(v22);
                let mut v28: Rc<str> = settled_word_13(v27.clone());
                let mut v29: Rc<str> = std::rc::Rc::<str>::from((v22).to_string());
                let mut v30: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
                let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let mut v33: Rc<str> = std::rc::Rc::<str>::from([&*v28, &*v31, &*v26, &*v32, &*v26].concat());
                let mut v34: Rc<str> = std::rc::Rc::<str>::from([&*v29, &*v31, &*v30, &*v31, &*v33].concat());
                let mut v35: Rc<str> = text_written_10(v24.clone(), v34.clone());
                let mut v36: u64 = eoie_machine_windows::eoie_job_windows_dropped(&*v1);
                return v22;
            } else {
                let mut v37: u64 = now_ms_9();
                let mut v38: bool = v37 > v2;
                let mut v43: bool = if v38 {
                    true
                } else {
                    let mut v39: bool = v5 != 0u64;
                    if v39 {
                        let mut v40: u64 = now_ms_9();
                        let mut v41: bool = v40 > v5;
                        v41
                    } else {
                        false
                    }
                };
                let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop"); } LIT.with(|lit| lit.clone()) };
                let mut v45: Rc<str> = job_file_5(v1.clone(), v44.clone());
                let mut v46: bool = file_exists_14(v45.clone());
                let mut v50: bool = if v7 {
                    let mut v47: u64 = v9.wrapping_mul(100u64);
                    let mut v48: u64 = v4.wrapping_mul(98u64);
                    let mut v49: bool = v47 >= v48;
                    v49
                } else {
                    false
                };
                let mut v57: US0 = if v46 {
                    US0::US0_HaltOnRequest
                } else {
                    if v50 {
                        US0::US0_HaltOnMemory
                    } else {
                        if v43 {
                            US0::US0_HaltOnTimeout
                        } else {
                            US0::US0_KeepRunning
                        }
                    }
                };
                match &v57 {
                    US0::US0_KeepRunning => {
                        let mut v58: u64 = now_ms_9();
                        let mut v59: u64 = v3.wrapping_add(1000u64);
                        let mut v60: bool = v58 > v59;
                        let mut v71: u64 = if v60 {
                            let mut v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
                            let mut v62: Rc<str> = job_file_5(v1.clone(), v61.clone());
                            let mut v63: Rc<str> = decimal_text_12(v58);
                            let mut v64: Rc<str> = text_written_10(v62.clone(), v63.clone());
                            let mut v70: Rc<str> = if v7 {
                                let mut v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("peak_private_mb"); } LIT.with(|lit| lit.clone()) };
                                let mut v66: Rc<str> = job_file_5(v1.clone(), v65.clone());
                                let mut v67: Rc<str> = decimal_text_12(v9);
                                text_written_10(v66.clone(), v67.clone())
                            } else {
                                let mut v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                v69.clone()
                            };
                            v58
                        } else {
                            v3
                        };
                        let mut v73: bool = if v60 {
                            gate_queued_15(v1.clone())
                        } else {
                            false
                        };
                        let mut v76: u64 = if v73 {
                            let mut v74: u64 = v58.wrapping_sub(v3);
                            let mut v75: u64 = v2.wrapping_add(v74);
                            v75
                        } else {
                            v2
                        };
                        let mut v77: u64 = 200u64;
                        std::thread::sleep(std::time::Duration::from_millis(v77));
                        (v0, v1, v2, v3, v4, v5) = (v0.clone(), v1.clone(), v76, v71, v4, v5);
                        continue;
                    }
                    _ => {
                        let mut v84: Rc<str> = if v7 {
                            let mut v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("peak_private_mb"); } LIT.with(|lit| lit.clone()) };
                            let mut v80: Rc<str> = job_file_5(v1.clone(), v79.clone());
                            let mut v81: Rc<str> = decimal_text_12(v9);
                            text_written_10(v80.clone(), v81.clone())
                        } else {
                            let mut v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v83.clone()
                        };
                        return halted_3(v0.clone(), v1.clone(), v57.clone());
                    }
                }
            }
        }
    }
}
fn method18(mut v0: Option<u64>) -> Option<u64> {
    v0.clone()
}
fn closure1() -> Rc<dyn Fn((u64)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US3> = Rc::new(move |mut v0: (u64)| -> US3 {
        let mut v1: u64 = (v0);
        US3::US3_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_17(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method18(v1.clone());
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
fn container_capped_19(mut v0: u64) -> bool {
    let mut v1: u64 = child_container_4();
    let mut v2: bool = { #[cfg(windows)] let capped = { unsafe extern "system" { fn QueryInformationJobObject(job: *mut std::ffi::c_void, class: i32, information: *mut std::ffi::c_void, length: u32, returned: *mut u32) -> i32; fn SetInformationJobObject(job: *mut std::ffi::c_void, class: i32, information: *mut std::ffi::c_void, length: u32) -> i32; } let job = v1 as *mut std::ffi::c_void; let mut limits = [0u64; 18]; !job.is_null() && unsafe { QueryInformationJobObject(job, 9, limits.as_mut_ptr().cast(), 144, std::ptr::null_mut()) } != 0 && { limits[2] |= 0x200; limits[15] = (v0).saturating_mul(1048576); (unsafe { SetInformationJobObject(job, 9, limits.as_mut_ptr().cast(), 144) } != 0) } }; #[cfg(not(windows))] let capped = { let _ = (v1, v0); false }; capped };
    v2
}
fn supervise_cleared_1(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-wrapper.ps1"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = job_file_5(v0.clone(), v1.clone());
    let mut v3: bool = file_exists_14(v2.clone());
    let mut v10: i32 = if v3 {
        let mut v4: i32 = eoie_job_gate::eoie_job_window_gate(&*v0);
        v4
    } else {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("window-class"); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = job_file_5(v0.clone(), v5.clone());
        let mut v7: bool = file_exists_14(v6.clone());
        if v7 {
            let mut v8: i32 = eoie_job_gate::eoie_job_compile_gate(&*v0);
            v8
        } else {
            0i32
        }
    };
    let mut v11: bool = v10 != 0i32;
    if v11 {
        v10
    } else {
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory_cap_mb"); } LIT.with(|lit| lit.clone()) };
        let mut v13: Rc<str> = job_file_5(v0.clone(), v12.clone());
        let mut v14: Rc<str> = text_or_empty_16(v13.clone());
        let mut v15: u64 = decimal_u64_17(v14.clone());
        let mut v16: bool = v15 > 0u64;
        let mut v19: bool = if v16 {
            let mut v17: bool = container_capped_19(v15);
            let mut v18: bool = v17 == false;
            v18
        } else {
            false
        };
        let mut v25: Rc<str> = if v19 {
            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory_cap_unenforced"); } LIT.with(|lit| lit.clone()) };
            let mut v21: Rc<str> = job_file_5(v0.clone(), v20.clone());
            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the job object refused the memory limit; the supervisor still stops the tree at the cap"); } LIT.with(|lit| lit.clone()) };
            text_written_10(v21.clone(), v22.clone())
        } else {
            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v24.clone()
        };
        let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("argv"); } LIT.with(|lit| lit.clone()) };
        let mut v27: Rc<str> = job_file_5(v0.clone(), v26.clone());
        let mut v28: Rc<str> = text_or_empty_16(v27.clone());
        let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cwd"); } LIT.with(|lit| lit.clone()) };
        let mut v30: Rc<str> = job_file_5(v0.clone(), v29.clone());
        let mut v31: Rc<str> = text_or_empty_16(v30.clone());
        let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stdout.log"); } LIT.with(|lit| lit.clone()) };
        let mut v33: Rc<str> = job_file_5(v0.clone(), v32.clone());
        let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stderr.log"); } LIT.with(|lit| lit.clone()) };
        let mut v35: Rc<str> = job_file_5(v0.clone(), v34.clone());
        let mut v36: u64 = child_container_4();
        let mut v37: Result<std::rc::Rc<std::cell::RefCell<std::process::Child>>, std::rc::Rc<str>> = { let mut parts = v28.lines(); let program = parts.next().unwrap_or_default().to_owned(); let arguments = parts.map(str::to_owned).collect::<Vec<_>>(); let outcome = (|| -> std::io::Result<std::process::Child> { let stdout = std::fs::File::create(&*v33)?; let stderr = std::fs::File::create(&*v35)?; let mut command = std::process::Command::new(&program); command.args(&arguments).current_dir(&*v31).stdin(std::process::Stdio::null()).stdout(stdout).stderr(stderr); #[cfg(windows)] { use std::os::windows::process::CommandExt; command.creation_flags(0x0800_0000).env_remove("NoDefaultCurrentDirectoryInExePath"); } #[cfg(unix)] { use std::os::unix::process::CommandExt; command.process_group(0); } let child = command.spawn()?; #[cfg(windows)] { unsafe extern "system" { fn AssignProcessToJobObject(job: *mut std::ffi::c_void, process: *mut std::ffi::c_void) -> i32; } use std::os::windows::io::AsRawHandle; if v36 != 0 { unsafe { AssignProcessToJobObject(v36 as *mut std::ffi::c_void, child.as_raw_handle()); } } } #[cfg(not(windows))] { let _ = v36; } Ok(child) })(); outcome.map(|child| std::rc::Rc::new(std::cell::RefCell::new(child))).map_err(|error| std::rc::Rc::<str>::from(format!("spawn {program}: {error}"))) };
        let mut v38: bool = v37.is_ok();
        if v38 {
            let mut v39: std::rc::Rc<std::cell::RefCell<std::process::Child>> = v37.clone().unwrap();
            let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pid"); } LIT.with(|lit| lit.clone()) };
            let mut v41: Rc<str> = job_file_5(v0.clone(), v40.clone());
            let mut v42: u64 = v39.borrow().id() as u64;
            let mut v43: Rc<str> = decimal_text_12(v42);
            let mut v44: Rc<str> = text_written_10(v41.clone(), v43.clone());
            let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
            let mut v46: Rc<str> = job_file_5(v0.clone(), v45.clone());
            let mut v47: u64 = now_ms_9();
            let mut v48: Rc<str> = decimal_text_12(v47);
            let mut v49: Rc<str> = text_written_10(v46.clone(), v48.clone());
            let mut v50: u64 = now_ms_9();
            let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout_ms"); } LIT.with(|lit| lit.clone()) };
            let mut v52: Rc<str> = job_file_5(v0.clone(), v51.clone());
            let mut v53: Rc<str> = text_or_empty_16(v52.clone());
            let mut v54: u64 = decimal_u64_17(v53.clone());
            let mut v55: u64 = v50.wrapping_add(v54);
            let mut v56: u64 = now_ms_9();
            let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("done_by_ms"); } LIT.with(|lit| lit.clone()) };
            let mut v58: Rc<str> = job_file_5(v0.clone(), v57.clone());
            let mut v59: Rc<str> = text_or_empty_16(v58.clone());
            let mut v60: u64 = decimal_u64_17(v59.clone());
            supervised_2(v39.clone(), v0.clone(), v55, v56, v15, v60)
        } else {
            let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
            let mut v63: Rc<str> = job_file_5(v0.clone(), v62.clone());
            let mut v64: u64 = now_ms_9();
            let mut v65: Rc<str> = v37.clone().err().unwrap_or_default();
            let mut v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v67: US2 = US2::US2_JobSpawnFailed(v66.clone());
            let mut v68: Rc<str> = settled_word_13(v67.clone());
            let mut v69: Rc<str> = std::rc::Rc::<str>::from((125i32).to_string());
            let mut v70: Rc<str> = std::rc::Rc::<str>::from((v64).to_string());
            let mut v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let mut v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v73: Rc<str> = std::rc::Rc::<str>::from([&*v68, &*v71, &*v65, &*v72, &*v66].concat());
            let mut v74: Rc<str> = std::rc::Rc::<str>::from([&*v69, &*v71, &*v70, &*v71, &*v73].concat());
            let mut v75: Rc<str> = text_written_10(v63.clone(), v74.clone());
            125i32
        }
    }
}
fn supervise_job_0(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("supervisor"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = job_file_5(v0.clone(), v1.clone());
    let mut v3: u64 = std::process::id() as u64;
    let mut v4: Rc<str> = decimal_text_12(v3);
    let mut v5: Rc<str> = text_written_10(v2.clone(), v4.clone());
    eoie_job_env::eoie_job_environment_adopted(&*v0);
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = job_file_5(v0.clone(), v6.clone());
    let mut v8: bool = file_exists_14(v7.clone());
    let mut v10: i32 = if v8 {
        let mut v9: i32 = eoie_job_after::eoie_job_after_wait(&*v0);
        v9
    } else {
        0i32
    };
    let mut v11: bool = v10 != 0i32;
    if v11 {
        v10
    } else {
        supervise_cleared_1(v0.clone())
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        supervise_job_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_job_supervise(v0: &str) -> i32 {
    closure0()(Rc::<str>::from(v0))
}
