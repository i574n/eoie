#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_PlainStart,
    US0_HeavyStart,
    US0_CompileStart,
}
#[derive(Clone)]
enum US1 {
    US1_Some(u64),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_Launched(Rc<str>, Rc<str>),
    US2_LaunchRefused(Rc<str>),
}
fn method3(mut v0: Option<u64>) -> Option<u64> {
    v0.clone()
}
fn closure1() -> Rc<dyn Fn((u64)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US1> = Rc::new(move |mut v0: (u64)| -> US1 {
        let mut v1: u64 = (v0);
        US1::US1_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_2(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method3(v1.clone());
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
fn joined_5(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2.clone()
}
fn jobs_root_6() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("EOIE_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("eoie").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("eoie").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".eoie-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn job_directory_4(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = jobs_root_6();
    joined_5(v1.clone(), v0.clone())
}
fn directory_exists_7(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.is_dir();
    v8
}
fn command_line_argument_count_9() -> i32 {
    let mut v0: i32 = (std::env::args().count() as i32) - 1;
    v0
}
fn argument_count_8() -> i32 {
    let mut v0: i32 = command_line_argument_count_9();
    let mut v1: i32 = v0.wrapping_add(1i32);
    v1
}
fn stderr_line_11(mut v0: Rc<str>) -> () {
    eprintln!("{}", &*v0);
    ()
}
fn rejected_10(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: job "); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v0, &*v2, &*v2, &*v2].concat());
    stderr_line_11(v3.clone());
    2i32
}
fn now_ms_13() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn decimal_text_14(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1.clone()
}
fn job_file_16(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = job_directory_4(v0.clone());
    joined_5(v2.clone(), v1.clone())
}
fn text_written_17(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2.clone()
}
fn heavy_gate_script_18() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("EOIE_HEAVY_GATE").map(std::path::PathBuf::from).filter(|path| path.is_file()).or_else(|| std::env::current_dir().ok().and_then(|cwd| cwd.ancestors().map(|base| base.join("spiral").join("apps").join("compiler").join("tmp").join("scripts").join("heavy.ps1")).find(|candidate| candidate.is_file()))).map(|path| path.to_string_lossy().into_owned()).unwrap_or_default());
    v0.clone()
}
fn job_argv_15(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64) -> Rc<str> {
    match &v0 {
        US0::US0_CompileStart => {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("window-class"); } LIT.with(|lit| lit.clone()) };
            let mut v6: Rc<str> = job_file_16(v1.clone(), v5.clone());
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = text_written_17(v6.clone(), v7.clone());
            v3.clone()
        }
        US0::US0_HeavyStart => {
            let mut v9: Rc<str> = heavy_gate_script_18();
            let mut v10: bool = v9.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v10 {
                let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v11.clone()
            } else {
                let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("argv.inner"); } LIT.with(|lit| lit.clone()) };
                let mut v13: Rc<str> = job_file_16(v1.clone(), v12.clone());
                let mut v14: Rc<str> = text_written_17(v13.clone(), v3.clone());
                let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-wrapper.ps1"); } LIT.with(|lit| lit.clone()) };
                let mut v16: Rc<str> = job_file_16(v1.clone(), v15.clone());
                let mut v17: Rc<str> = job_file_16(v1.clone(), v12.clone());
                let mut v18: Rc<str> = eoie_machine_windows::eoie_heavy_wrapper(&*v9, &*v17, &*v2, &*v1, v4);
                let mut v19: Rc<str> = text_written_17(v16.clone(), v18.clone());
                let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pwsh\n-NoProfile\n-File\n"); } LIT.with(|lit| lit.clone()) };
                let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v16));
                v21.clone()
            }
        }
        US0::US0_PlainStart => {
            v3.clone()
        }
    }
}
fn environment_snapshot_19() -> Rc<str> {
    let mut v0: Rc<str> = Rc::<str>::from(std::env::vars_os().map(|(key, value)| format!("{}={}", key.to_string_lossy(), value.to_string_lossy())).collect::<Vec<_>>().join(&char::from(0u8).to_string()).as_str());
    v0.clone()
}
fn job_launched_12(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: bool, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: Rc<str>, mut v10: u64, mut v11: Rc<str>, mut v12: Rc<str>) -> US2 {
    let mut v13: u64 = now_ms_13();
    let mut v14: Rc<str> = decimal_text_14(v13);
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v14));
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v17));
    let mut v19: Rc<str> = std::rc::Rc::<str>::from(v9.chars().map(|c| if c.is_ascii_alphanumeric() || c == char::from(45u8) { c } else { char::from(95u8) }).take(40).collect::<String>());
    let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v19));
    let mut v21: Rc<str> = job_directory_4(v20.clone());
    let mut v22: Rc<str> = std::rc::Rc::<str>::from(std::fs::create_dir_all(&*v21).err().map(|error| format!("create {}: {error}", &*v21)).unwrap_or_default());
    let mut v23: bool = v22.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v23 {
        US2::US2_LaunchRefused(v22.clone())
    } else {
        let mut v25: Rc<str> = job_argv_15(v0.clone(), v20.clone(), v9.clone(), v11.clone(), v10);
        let mut v26: bool = v25.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v26 {
            let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--heavy needs spiral's scripts/heavy.ps1: set EOIE_HEAVY_GATE or run from inside a tree that has a spiral checkout"); } LIT.with(|lit| lit.clone()) };
            US2::US2_LaunchRefused(v27.clone())
        } else {
            let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("label"); } LIT.with(|lit| lit.clone()) };
            let mut v30: Rc<str> = job_file_16(v20.clone(), v29.clone());
            let mut v31: Rc<str> = text_written_17(v30.clone(), v9.clone());
            let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout_ms"); } LIT.with(|lit| lit.clone()) };
            let mut v33: Rc<str> = job_file_16(v20.clone(), v32.clone());
            let mut v34: Rc<str> = decimal_text_14(v10);
            let mut v35: Rc<str> = text_written_17(v33.clone(), v34.clone());
            let mut v36: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v35));
            let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cwd"); } LIT.with(|lit| lit.clone()) };
            let mut v38: Rc<str> = job_file_16(v20.clone(), v37.clone());
            let mut v39: Rc<str> = text_written_17(v38.clone(), v12.clone());
            let mut v40: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v39));
            let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("argv"); } LIT.with(|lit| lit.clone()) };
            let mut v42: Rc<str> = job_file_16(v20.clone(), v41.clone());
            let mut v43: Rc<str> = text_written_17(v42.clone(), v25.clone());
            let mut v44: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v43));
            let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("started_ms"); } LIT.with(|lit| lit.clone()) };
            let mut v46: Rc<str> = job_file_16(v20.clone(), v45.clone());
            let mut v47: u64 = now_ms_13();
            let mut v48: Rc<str> = decimal_text_14(v47);
            let mut v49: Rc<str> = text_written_17(v46.clone(), v48.clone());
            let mut v50: Rc<str> = Rc::<str>::from(format!("{}{}", v44, v49));
            let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("env"); } LIT.with(|lit| lit.clone()) };
            let mut v52: Rc<str> = job_file_16(v20.clone(), v51.clone());
            let mut v53: Rc<str> = environment_snapshot_19();
            let mut v54: Rc<str> = text_written_17(v52.clone(), v53.clone());
            let mut v55: Rc<str> = Rc::<str>::from(format!("{}{}", v50, v54));
            let mut v56: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v64: Rc<str> = if v56 {
                let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v57.clone()
            } else {
                let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after"); } LIT.with(|lit| lit.clone()) };
                let mut v59: Rc<str> = job_file_16(v20.clone(), v58.clone());
                let mut v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let mut v61: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v60));
                let mut v62: Rc<str> = Rc::<str>::from(format!("{}{}", v61, v1));
                text_written_17(v59.clone(), v62.clone())
            };
            let mut v65: bool = v4 == 0u64;
            let mut v71: Rc<str> = if v65 {
                let mut v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v66.clone()
            } else {
                let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory_cap_mb"); } LIT.with(|lit| lit.clone()) };
                let mut v68: Rc<str> = job_file_16(v20.clone(), v67.clone());
                let mut v69: Rc<str> = decimal_text_14(v4);
                text_written_17(v68.clone(), v69.clone())
            };
            let mut v72: bool = v5 == 0u64;
            let mut v77: u64 = if v72 {
                v6
            } else {
                let mut v73: bool = v6 == 0u64;
                let mut v75: bool = if v73 {
                    true
                } else {
                    let mut v74: bool = v5 < v6;
                    v74
                };
                if v75 {
                    v5
                } else {
                    v6
                }
            };
            let mut v78: bool = v77 == 0u64;
            let mut v87: Rc<str> = if v78 {
                let mut v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v79.clone()
            } else {
                let mut v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot_by_ms"); } LIT.with(|lit| lit.clone()) };
                let mut v81: Rc<str> = job_file_16(v20.clone(), v80.clone());
                let mut v82: u64 = now_ms_13();
                let mut v83: u64 = v77.wrapping_mul(60000u64);
                let mut v84: u64 = v82.wrapping_add(v83);
                let mut v85: Rc<str> = decimal_text_14(v84);
                text_written_17(v81.clone(), v85.clone())
            };
            let mut v88: bool = v6 == 0u64;
            let mut v97: Rc<str> = if v88 {
                let mut v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v89.clone()
            } else {
                let mut v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("done_by_ms"); } LIT.with(|lit| lit.clone()) };
                let mut v91: Rc<str> = job_file_16(v20.clone(), v90.clone());
                let mut v92: u64 = now_ms_13();
                let mut v93: u64 = v6.wrapping_mul(60000u64);
                let mut v94: u64 = v92.wrapping_add(v93);
                let mut v95: Rc<str> = decimal_text_14(v94);
                text_written_17(v91.clone(), v95.clone())
            };
            let mut v98: bool = v7 == 0u64;
            let mut v109: Rc<str> = if v98 {
                let mut v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v99.clone()
            } else {
                let mut v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("quiet_hold_min"); } LIT.with(|lit| lit.clone()) };
                let mut v101: Rc<str> = job_file_16(v20.clone(), v100.clone());
                let mut v102: Rc<str> = decimal_text_14(v7);
                let mut v103: Rc<str> = text_written_17(v101.clone(), v102.clone());
                let mut v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("quiet_wait_min"); } LIT.with(|lit| lit.clone()) };
                let mut v105: Rc<str> = job_file_16(v20.clone(), v104.clone());
                let mut v106: Rc<str> = decimal_text_14(v8);
                let mut v107: Rc<str> = text_written_17(v105.clone(), v106.clone());
                let mut v108: Rc<str> = Rc::<str>::from(format!("{}{}", v103, v107));
                v108.clone()
            };
            let mut v110: bool = v55.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v112: bool = if v110 {
                true
            } else {
                let mut v111: bool = v64.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v111
            };
            let mut v114: bool = if v112 {
                true
            } else {
                let mut v113: bool = v71.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v113
            };
            let mut v116: bool = if v114 {
                true
            } else {
                let mut v115: bool = v87.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v115
            };
            let mut v118: bool = if v116 {
                true
            } else {
                let mut v117: bool = v97.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v117
            };
            let mut v120: bool = if v118 {
                true
            } else {
                let mut v119: bool = v109.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v119
            };
            if v120 {
                let mut v121: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v64));
                let mut v122: Rc<str> = Rc::<str>::from(format!("{}{}", v121, v71));
                let mut v123: Rc<str> = Rc::<str>::from(format!("{}{}", v122, v87));
                let mut v124: Rc<str> = Rc::<str>::from(format!("{}{}", v97, v109));
                let mut v125: Rc<str> = Rc::<str>::from(format!("{}{}", v123, v124));
                US2::US2_LaunchRefused(v125.clone())
            } else {
                let mut v129: Rc<str> = if v3 {
                    let mut v127: Rc<str> = std::rc::Rc::<str>::from(std::process::id().to_string().as_str());
                    v127.clone()
                } else {
                    let mut v128: Rc<str> = eoie_job_host::eoie_job_host_request(&*v20);
                    v128.clone()
                };
                let mut v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error:"); } LIT.with(|lit| lit.clone()) };
                let mut v131: bool = v129.starts_with(&*v130);
                if v131 {
                    US2::US2_LaunchRefused(v129.clone())
                } else {
                    US2::US2_Launched(v20.clone(), v129.clone())
                }
            }
        }
    }
}
fn stdout_line_20(mut v0: Rc<str>) -> () {
    println!("{}", &*v0);
    ()
}
fn job_started_1(mut v0: bool) -> (i32, Rc<str>) {
    let mut v1: i32 = 3i32;
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v1 as usize).unwrap_or_default());
    let mut v3: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--after"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("any"); } LIT.with(|lit| lit.clone()) };
        v4.clone()
    } else {
        let mut v5: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--after-ok"); } LIT.with(|lit| lit.clone()) };
        if v5 {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        } else {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v7.clone()
        }
    };
    let mut v10: bool = v9.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = if v10 {
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v11.clone()
    } else {
        let mut v12: i32 = 4i32;
        let mut v13: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v12 as usize).unwrap_or_default());
        v13.clone()
    };
    let mut v15: i32 = if v10 {
        3i32
    } else {
        5i32
    };
    let mut v16: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v15 as usize).unwrap_or_default());
    let mut v17: bool = v16.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--heavy"); } LIT.with(|lit| lit.clone()) };
    let mut v23: US0 = if v17 {
        US0::US0_HeavyStart
    } else {
        let mut v19: bool = v16.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--compile"); } LIT.with(|lit| lit.clone()) };
        if v19 {
            US0::US0_CompileStart
        } else {
            US0::US0_PlainStart
        }
    };
    let mut v26: i32 = match &v23 {
        US0::US0_CompileStart => {
            1i32
        }
        US0::US0_HeavyStart => {
            1i32
        }
        US0::US0_PlainStart => {
            0i32
        }
    };
    let mut v27: i32 = v15.wrapping_add(v26);
    let mut v28: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v27 as usize).unwrap_or_default());
    let mut v29: bool = v28.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--max-private-mb"); } LIT.with(|lit| lit.clone()) };
    let mut v33: u64 = if v29 {
        let mut v30: i32 = v27.wrapping_add(1i32);
        let mut v31: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v30 as usize).unwrap_or_default());
        decimal_u64_2(v31.clone())
    } else {
        0u64
    };
    let mut v35: i32 = if v29 {
        let mut v34: i32 = v27.wrapping_add(2i32);
        v34
    } else {
        v27
    };
    let mut v36: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v35 as usize).unwrap_or_default());
    let mut v37: bool = v36.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--slot-within"); } LIT.with(|lit| lit.clone()) };
    let mut v41: u64 = if v37 {
        let mut v38: i32 = v35.wrapping_add(1i32);
        let mut v39: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v38 as usize).unwrap_or_default());
        decimal_u64_2(v39.clone())
    } else {
        0u64
    };
    let mut v43: i32 = if v37 {
        let mut v42: i32 = v35.wrapping_add(2i32);
        v42
    } else {
        v35
    };
    let mut v44: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v43 as usize).unwrap_or_default());
    let mut v45: bool = v44.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--done-within"); } LIT.with(|lit| lit.clone()) };
    let mut v49: u64 = if v45 {
        let mut v46: i32 = v43.wrapping_add(1i32);
        let mut v47: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v46 as usize).unwrap_or_default());
        decimal_u64_2(v47.clone())
    } else {
        0u64
    };
    let mut v51: i32 = if v45 {
        let mut v50: i32 = v43.wrapping_add(2i32);
        v50
    } else {
        v43
    };
    let mut v52: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v51 as usize).unwrap_or_default());
    let mut v53: bool = v52.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--quiet"); } LIT.with(|lit| lit.clone()) };
    let mut v55: i32 = if v53 {
        let mut v54: i32 = v51.wrapping_add(1i32);
        v54
    } else {
        v51
    };
    let mut v56: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v55 as usize).unwrap_or_default());
    let mut v57: bool = v56.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--quiet-hold"); } LIT.with(|lit| lit.clone()) };
    let mut v62: u64 = if v57 {
        let mut v58: i32 = v55.wrapping_add(1i32);
        let mut v59: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v58 as usize).unwrap_or_default());
        decimal_u64_2(v59.clone())
    } else {
        if v53 {
            30u64
        } else {
            0u64
        }
    };
    let mut v64: i32 = if v57 {
        let mut v63: i32 = v55.wrapping_add(2i32);
        v63
    } else {
        v55
    };
    let mut v65: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v64 as usize).unwrap_or_default());
    let mut v66: bool = v65.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--quiet-wait"); } LIT.with(|lit| lit.clone()) };
    let mut v71: u64 = if v66 {
        let mut v67: i32 = v64.wrapping_add(1i32);
        let mut v68: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v67 as usize).unwrap_or_default());
        decimal_u64_2(v68.clone())
    } else {
        if v53 {
            10u64
        } else {
            0u64
        }
    };
    let mut v73: i32 = if v66 {
        let mut v72: i32 = v64.wrapping_add(2i32);
        v72
    } else {
        v64
    };
    let mut v79: bool = if v10 {
        true
    } else {
        let mut v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
        let mut v75: bool = v14.starts_with(&*v74);
        if v75 {
            let mut v76: Rc<str> = job_directory_4(v14.clone());
            directory_exists_7(v76.clone())
        } else {
            false
        }
    };
    let mut v81: Rc<str> = if v79 {
        let mut v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v80.clone()
    } else {
        v14.clone()
    };
    let mut v82: i32 = argument_count_8();
    let mut v83: i32 = v73.wrapping_add(3i32);
    let mut v84: bool = v82 < v83;
    if v84 {
        let mut v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job start [--after <id>|--after-ok <id>] [--heavy|--compile] [--max-private-mb <n>] [--slot-within <minutes>] [--done-within <minutes>] [--quiet [--quiet-hold <min>] [--quiet-wait <min>]] <label> <timeout-ms> <program> [args...] (the cap covers the whole job tree; reaching it stops the job as memory-capped, exit 137) | job run [--after <id>|--after-ok <id>] [--heavy|--compile] [--max-private-mb <n>] [--slot-within <minutes>] [--done-within <minutes>] [--quiet [--quiet-hold <min>] [--quiet-wait <min>]] <label> <timeout-ms> <program> [args...] (supervises in this process, so the job's processes stay in the caller's tree and end with it; exits with the job's code; for long runs from an agent shell that can be reaped, use job start plus a background job wait) | job status <id> | job wait <id> [max-ms, default until it settles] | job list [count] | job tail <id> [lines] | job stop <id>"); } LIT.with(|lit| lit.clone()) };
        let mut v86: i32 = rejected_10(v85.clone());
        let mut v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (v86, v87.clone())
    } else {
        let mut v88: bool = v81.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v88 {
            let mut v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--after names no job: "); } LIT.with(|lit| lit.clone()) };
            let mut v90: Rc<str> = Rc::<str>::from(format!("{}{}", v89, v81));
            let mut v91: i32 = rejected_10(v90.clone());
            let mut v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (v91, v92.clone())
        } else {
            let mut v94: bool = if v29 {
                let mut v93: bool = v33 == 0u64;
                v93
            } else {
                false
            };
            if v94 {
                let mut v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--max-private-mb needs a positive number of MB"); } LIT.with(|lit| lit.clone()) };
                let mut v96: i32 = rejected_10(v95.clone());
                let mut v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                (v96, v97.clone())
            } else {
                let mut v99: bool = if v37 {
                    let mut v98: bool = v41 == 0u64;
                    v98
                } else {
                    false
                };
                if v99 {
                    let mut v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--slot-within needs a positive number of minutes"); } LIT.with(|lit| lit.clone()) };
                    let mut v101: i32 = rejected_10(v100.clone());
                    let mut v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    (v101, v102.clone())
                } else {
                    let mut v104: bool = if v45 {
                        let mut v103: bool = v49 == 0u64;
                        v103
                    } else {
                        false
                    };
                    if v104 {
                        let mut v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--done-within needs a positive number of minutes"); } LIT.with(|lit| lit.clone()) };
                        let mut v106: i32 = rejected_10(v105.clone());
                        let mut v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        (v106, v107.clone())
                    } else {
                        let mut v108: bool = v57 || v66;
                        let mut v110: bool = if v108 {
                            let mut v109: bool = v53 == false;
                            v109
                        } else {
                            false
                        };
                        if v110 {
                            let mut v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--quiet-hold and --quiet-wait follow --quiet"); } LIT.with(|lit| lit.clone()) };
                            let mut v112: i32 = rejected_10(v111.clone());
                            let mut v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            (v112, v113.clone())
                        } else {
                            let mut v116: bool = if v53 {
                                let mut v114: bool = match &v23 {
                                    US0::US0_HeavyStart => {
                                        true
                                    }
                                    _ => {
                                        false
                                    }
                                };
                                let mut v115: bool = v114 != true;
                                v115
                            } else {
                                false
                            };
                            if v116 {
                                let mut v117: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--quiet applies to --heavy jobs: it holds compiles off while the heavy work starts"); } LIT.with(|lit| lit.clone()) };
                                let mut v118: i32 = rejected_10(v117.clone());
                                let mut v119: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                (v118, v119.clone())
                            } else {
                                let mut v123: bool = if v53 {
                                    let mut v120: bool = v62 == 0u64;
                                    if v120 {
                                        true
                                    } else {
                                        let mut v121: bool = v62 > 60u64;
                                        v121
                                    }
                                } else {
                                    false
                                };
                                if v123 {
                                    let mut v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--quiet-hold takes 1 to 60 minutes (compiles wait at most that long)"); } LIT.with(|lit| lit.clone()) };
                                    let mut v125: i32 = rejected_10(v124.clone());
                                    let mut v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    (v125, v126.clone())
                                } else {
                                    let mut v128: bool = if v37 {
                                        match &v23 {
                                            US0::US0_PlainStart => {
                                                true
                                            }
                                            _ => {
                                                false
                                            }
                                        }
                                    } else {
                                        false
                                    };
                                    if v128 {
                                        let mut v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--slot-within applies to --heavy or --compile jobs, which wait for a slot or window"); } LIT.with(|lit| lit.clone()) };
                                        let mut v130: i32 = rejected_10(v129.clone());
                                        let mut v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        (v130, v131.clone())
                                    } else {
                                        let mut v132: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v73 as usize).unwrap_or_default());
                                        let mut v133: i32 = v73.wrapping_add(1i32);
                                        let mut v134: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v133 as usize).unwrap_or_default());
                                        let mut v135: u64 = decimal_u64_2(v134.clone());
                                        let mut v136: bool = v135 == 0u64;
                                        if v136 {
                                            let mut v137: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout-ms must be a positive integer"); } LIT.with(|lit| lit.clone()) };
                                            let mut v138: i32 = rejected_10(v137.clone());
                                            let mut v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                            (v138, v139.clone())
                                        } else {
                                            let mut v140: i32 = v73.wrapping_add(2i32);
                                            let mut v141: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(v140 as usize).collect::<Vec<_>>().join("
"));
                                            let mut v142: Rc<str> = std::rc::Rc::<str>::from(std::env::current_dir().map(|path| path.to_string_lossy().into_owned()).unwrap_or_default());
                                            let mut v143: US2 = job_launched_12(v23.clone(), v9.clone(), v14.clone(), v0, v33, v41, v49, v62, v71, v132.clone(), v135, v141.clone(), v142.clone());
                                            match &v143 {
                                                US2::US2_LaunchRefused(v144) => {
                                                    let mut v144: Rc<str> = v144.clone();
                                                    let mut v145: i32 = rejected_10(v144.clone());
                                                    let mut v146: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                    (v145, v146.clone())
                                                }
                                                US2::US2_Launched(v147, v148) => {
                                                    let mut v147: Rc<str> = v147.clone();
                                                    let mut v148: Rc<str> = v148.clone();
                                                    let mut v151: Rc<str> = if v0 {
                                                        let mut v149: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie job run started id="); } LIT.with(|lit| lit.clone()) };
                                                        v149.clone()
                                                    } else {
                                                        let mut v150: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie job start ok id="); } LIT.with(|lit| lit.clone()) };
                                                        v150.clone()
                                                    };
                                                    let mut v152: Rc<str> = Rc::<str>::from(format!("{}{}", v151, v147));
                                                    let mut v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" supervisor_pid="); } LIT.with(|lit| lit.clone()) };
                                                    let mut v154: Rc<str> = Rc::<str>::from(format!("{}{}", v152, v153));
                                                    let mut v155: Rc<str> = Rc::<str>::from(format!("{}{}", v154, v148));
                                                    let mut v156: Rc<str> = decimal_text_14(v135);
                                                    let mut v157: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" timeout_ms="); } LIT.with(|lit| lit.clone()) };
                                                    let mut v158: Rc<str> = Rc::<str>::from(format!("{}{}", v157, v156));
                                                    let mut v159: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" gate="); } LIT.with(|lit| lit.clone()) };
                                                    let mut v160: Rc<str> = Rc::<str>::from(format!("{}{}", v158, v159));
                                                    let mut v166: Rc<str> = match &v23 {
                                                        US0::US0_CompileStart => {
                                                            let mut v163: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
                                                            v163.clone()
                                                        }
                                                        US0::US0_HeavyStart => {
                                                            let mut v162: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy"); } LIT.with(|lit| lit.clone()) };
                                                            v162.clone()
                                                        }
                                                        US0::US0_PlainStart => {
                                                            let mut v161: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("plain"); } LIT.with(|lit| lit.clone()) };
                                                            v161.clone()
                                                        }
                                                    };
                                                    let mut v167: Rc<str> = Rc::<str>::from(format!("{}{}", v160, v166));
                                                    let mut v168: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" label="); } LIT.with(|lit| lit.clone()) };
                                                    let mut v169: Rc<str> = Rc::<str>::from(format!("{}{}", v168, v132));
                                                    let mut v176: Rc<str> = if v10 {
                                                        let mut v170: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                        v170.clone()
                                                    } else {
                                                        let mut v171: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" after="); } LIT.with(|lit| lit.clone()) };
                                                        let mut v172: Rc<str> = Rc::<str>::from(format!("{}{}", v171, v14));
                                                        let mut v173: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" after_mode="); } LIT.with(|lit| lit.clone()) };
                                                        let mut v174: Rc<str> = Rc::<str>::from(format!("{}{}", v172, v173));
                                                        let mut v175: Rc<str> = Rc::<str>::from(format!("{}{}", v174, v9));
                                                        v175.clone()
                                                    };
                                                    let mut v177: Rc<str> = Rc::<str>::from(format!("{}{}", v169, v176));
                                                    let mut v178: bool = v33 == 0u64;
                                                    let mut v183: Rc<str> = if v178 {
                                                        let mut v179: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                        v179.clone()
                                                    } else {
                                                        let mut v180: Rc<str> = decimal_text_14(v33);
                                                        let mut v181: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" max_private_mb="); } LIT.with(|lit| lit.clone()) };
                                                        let mut v182: Rc<str> = Rc::<str>::from(format!("{}{}", v181, v180));
                                                        v182.clone()
                                                    };
                                                    let mut v184: Rc<str> = Rc::<str>::from(format!("{}{}", v177, v183));
                                                    let mut v185: bool = v41 == 0u64;
                                                    let mut v190: Rc<str> = if v185 {
                                                        let mut v186: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                        v186.clone()
                                                    } else {
                                                        let mut v187: Rc<str> = decimal_text_14(v41);
                                                        let mut v188: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" slot_within_min="); } LIT.with(|lit| lit.clone()) };
                                                        let mut v189: Rc<str> = Rc::<str>::from(format!("{}{}", v188, v187));
                                                        v189.clone()
                                                    };
                                                    let mut v191: bool = v49 == 0u64;
                                                    let mut v196: Rc<str> = if v191 {
                                                        let mut v192: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                        v192.clone()
                                                    } else {
                                                        let mut v193: Rc<str> = decimal_text_14(v49);
                                                        let mut v194: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" done_within_min="); } LIT.with(|lit| lit.clone()) };
                                                        let mut v195: Rc<str> = Rc::<str>::from(format!("{}{}", v194, v193));
                                                        v195.clone()
                                                    };
                                                    let mut v197: Rc<str> = Rc::<str>::from(format!("{}{}", v190, v196));
                                                    let mut v198: bool = v62 == 0u64;
                                                    let mut v207: Rc<str> = if v198 {
                                                        let mut v199: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                        v199.clone()
                                                    } else {
                                                        let mut v200: Rc<str> = decimal_text_14(v62);
                                                        let mut v201: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" quiet_hold_min="); } LIT.with(|lit| lit.clone()) };
                                                        let mut v202: Rc<str> = Rc::<str>::from(format!("{}{}", v201, v200));
                                                        let mut v203: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" quiet_wait_min="); } LIT.with(|lit| lit.clone()) };
                                                        let mut v204: Rc<str> = Rc::<str>::from(format!("{}{}", v202, v203));
                                                        let mut v205: Rc<str> = decimal_text_14(v71);
                                                        let mut v206: Rc<str> = Rc::<str>::from(format!("{}{}", v204, v205));
                                                        v206.clone()
                                                    };
                                                    let mut v208: Rc<str> = Rc::<str>::from(format!("{}{}", v197, v207));
                                                    let mut v209: Rc<str> = Rc::<str>::from(format!("{}{}", v184, v208));
                                                    let mut v210: Rc<str> = Rc::<str>::from(format!("{}{}", v167, v209));
                                                    let mut v211: Rc<str> = Rc::<str>::from(format!("{}{}", v155, v210));
                                                    stdout_line_20(v211.clone());
                                                    if v0 {
                                                        let mut v212: i32 = eoie_job_supervise::eoie_job_supervise(&*v147);
                                                        (v212, v147.clone())
                                                    } else {
                                                        (0i32, v147.clone())
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn started_text_0(mut v0: bool) -> Rc<str> {
    let (mut v1, mut v2): (i32, Rc<str>) = job_started_1(v0);
    let mut v3: Rc<str> = std::rc::Rc::<str>::from((v1).to_string().as_str());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v4));
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
    v6.clone()
}
fn closure0() -> Rc<dyn Fn(bool) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(bool) -> Rc<str>> = Rc::new(move |mut v0: bool| -> Rc<str> {
        started_text_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn launched_text_21(mut v0: Rc<str>, mut v1: u64, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let mut v4: US0 = US0::US0_PlainStart;
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: bool = false;
    let mut v7: u64 = 0u64;
    let mut v8: u64 = 0u64;
    let mut v9: u64 = 0u64;
    let mut v10: u64 = 0u64;
    let mut v11: u64 = 0u64;
    let mut v12: US2 = job_launched_12(v4.clone(), v5.clone(), v5.clone(), v6, v7, v8, v9, v10, v11, v0.clone(), v1, v2.clone(), v3.clone());
    match &v12 {
        US2::US2_LaunchRefused(v18) => {
            let mut v18: Rc<str> = v18.clone();
            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: "); } LIT.with(|lit| lit.clone()) };
            let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v18));
            v20.clone()
        }
        US2::US2_Launched(v13, v14) => {
            let mut v13: Rc<str> = v13.clone();
            let mut v14: Rc<str> = v14.clone();
            let mut v15: Rc<str> = std::rc::Rc::<str>::from(char::from(30u8).to_string());
            let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v15));
            let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v14));
            v17.clone()
        }
    }
}
fn closure2() -> Rc<dyn Fn(Rc<str>, u64, Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, u64, Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: u64, mut v2: Rc<str>, mut v3: Rc<str>| -> Rc<str> {
        launched_text_21(v0.clone(), v1, v2.clone(), v3.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_job_start(v0: bool) -> Rc<str> {
    closure0()(v0)
}
pub fn eoie_job_launched_text(v0: &str, v1: u64, v2: &str, v3: &str) -> Rc<str> {
    closure2()(Rc::<str>::from(v0), v1, Rc::<str>::from(v2), Rc::<str>::from(v3))
}
