#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_EveryProcess,
    US1_SessionTree(u64),
    US1_NoSession,
}
#[derive(Clone)]
enum UH0 {
    UH0_NoPsRows,
    UH0_PsRowThen(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US2 {
    US2_Succeeded,
    US2_Failed,
}
#[derive(Clone)]
enum US3 {
    US3_Allowed(u64),
    US3_OwnProcess,
    US3_OwnAncestor,
    US3_NotAProcessId,
}
#[derive(Clone)]
enum US4 {
    US4_Some(u64),
    US4_None,
}
#[derive(Clone)]
enum US5 {
    US5_NoSuchProcess,
    US5_StoppedOf(u64, u64),
}
#[derive(Clone)]
enum US6 {
    US6_ListProcesses,
    US6_StopProcesses,
    US6_ProcessUnknown,
}
fn command_line_argument_count_3() -> i32 {
    let mut v0: i32 = (std::env::args().count() as i32) - 1;
    v0
}
fn argument_count_2() -> i32 {
    let mut v0: i32 = command_line_argument_count_3();
    let mut v1: i32 = v0.wrapping_add(1i32);
    v1
}
fn command_line_argument_6(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::args().nth((v0 + 1) as usize).unwrap_or_default());
    v1.clone()
}
fn argument_5(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_sub(1i32);
    let mut v2: bool = v1 >= 0i32;
    let mut v5: bool = if v2 {
        let mut v3: i32 = command_line_argument_count_3();
        let mut v4: bool = v1 < v3;
        v4
    } else {
        false
    };
    let mut v9: US0 = if v5 {
        let mut v6: Rc<str> = command_line_argument_6(v1);
        US0::US0_Some(v6.clone())
    } else {
        US0::US0_None
    };
    match &v9 {
        US0::US0_None => {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US0::US0_Some(v10) => {
            let mut v10: Rc<str> = v10.clone();
            v10.clone()
        }
    }
}
fn pattern_from_4(mut v0: i32, mut v1: i32) -> Rc<str> {
    loop {
        let mut v2: bool = v0 < v1;
        if v2 {
            let mut v3: Rc<str> = argument_5(v0);
            let mut v4: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--tree"); } LIT.with(|lit| lit.clone()) };
            let mut v7: bool = if v4 {
                true
            } else {
                let mut v5: Rc<str> = argument_5(v0);
                let mut v6: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--mine"); } LIT.with(|lit| lit.clone()) };
                v6
            };
            if v7 {
                let mut v8: i32 = v0.wrapping_add(1i32);
                (v0, v1) = (v8, v1);
                continue;
            } else {
                return argument_5(v0);
            }
        } else {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v12.clone();
        }
    }
}
fn pattern_problem_7(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(regex::RegexBuilder::new(&*v0).case_insensitive(true).build().err().map(|error| error.to_string()).unwrap_or_default().as_str());
    v1.clone()
}
fn process_rows_sampled_8(mut v0: u64) -> std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>> {
    let mut v1: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>> = { let kind = sysinfo::ProcessRefreshKind::nothing().with_cpu().with_memory().with_cmd(sysinfo::UpdateKind::Always); let mut system = sysinfo::System::new_with_specifics(sysinfo::RefreshKind::nothing().with_processes(kind)); std::thread::sleep(std::time::Duration::from_millis(v0)); system.refresh_processes_specifics(sysinfo::ProcessesToUpdate::All, true, kind); std::rc::Rc::new(system.processes().iter().map(|(pid, process)| { let command = process.cmd().iter().map(|part| part.to_string_lossy().into_owned()).collect::<Vec<_>>().join(" "); (pid.as_u32() as u64, process.parent().map(|parent| parent.as_u32() as u64).unwrap_or(0), std::rc::Rc::<str>::from(process.name().to_string_lossy().trim_end_matches(".exe")), std::rc::Rc::<str>::from(command.as_str()), process.cpu_usage() as f64 / 100.0, process.run_time() / 60, process.accumulated_cpu_time() / 60000, process.memory() / 1048576) }).collect::<Vec<_>>()) };
    v1.clone()
}
fn flag_present_9(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> bool {
    loop {
        let mut v3: bool = v1 < v2;
        if v3 {
            let mut v4: Rc<str> = argument_5(v1);
            let mut v5: bool = v4.clone() == v0.clone();
            if v5 {
                return true;
            } else {
                let mut v6: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v6, v2);
                continue;
            }
        } else {
            return false;
        }
    }
}
fn session_root_10(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v1: u64, mut v2: i32) -> u64 {
    loop {
        let mut v3: i32 = v0.iter().position(|row| row.0 == v1).map(|at| at as i32).unwrap_or(-1);
        let mut v4: bool = v3 < 0i32;
        let mut v6: bool = if v4 {
            true
        } else {
            let mut v5: bool = v2 > 32i32;
            v5
        };
        if v6 {
            return 0u64;
        } else {
            let mut v7: Rc<str> = v0[v3 as usize].2.clone();
            let mut v8: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("claude"); } LIT.with(|lit| lit.clone()) };
            let mut v10: bool = if v8 {
                true
            } else {
                let mut v9: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("codex"); } LIT.with(|lit| lit.clone()) };
                v9
            };
            if v10 {
                return v1;
            } else {
                let mut v11: u64 = v0[v3 as usize].1;
                let mut v12: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v11, v12);
                continue;
            }
        }
    }
}
fn local_offset_minutes_11() -> i64 {
    let mut v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn descends_from_13(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v1: u64, mut v2: u64, mut v3: i32) -> bool {
    loop {
        let mut v4: bool = v1 == v2;
        if v4 {
            return true;
        } else {
            let mut v5: bool = v1 == 0u64;
            let mut v7: bool = if v5 {
                true
            } else {
                let mut v6: bool = v3 > 64i32;
                v6
            };
            if v7 {
                return false;
            } else {
                let mut v8: i32 = v0.iter().position(|row| row.0 == v1).map(|at| at as i32).unwrap_or(-1);
                let mut v9: bool = v8 < 0i32;
                if v9 {
                    return false;
                } else {
                    let mut v10: u64 = v0[v8 as usize].1;
                    let mut v11: i32 = v3.wrapping_add(1i32);
                    (v0, v1, v2, v3) = (v0.clone(), v10, v2, v11);
                    continue;
                }
            }
        }
    }
}
fn pattern_matches_14(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: bool = { static PATTERN: std::sync::OnceLock<Option<regex::Regex>> = std::sync::OnceLock::new(); PATTERN.get_or_init(|| regex::RegexBuilder::new(&*v0).case_insensitive(true).build().ok()).as_ref().map(|found| found.is_match(&*v1)).unwrap_or(false) };
    v2
}
fn children_listed_15(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v1: u64, mut v2: i32, mut v3: i64, mut v4: i32, mut v5: Rc<UH0>, mut v6: Rc<str>, mut v7: u64) -> (Rc<UH0>, Rc<str>, u64) {
    let mut v8: i32 = v0.len() as i32;
    let mut v9: bool = v4 >= v8;
    let (mut v53, mut v54, mut v55): (Rc<UH0>, Rc<str>, u64) = if v9 {
        (v5.clone(), v6.clone(), v7)
    } else {
        let mut v10: u64 = v0[v4 as usize].0;
        let mut v11: u64 = v0[v4 as usize].1;
        let mut v12: bool = v11 == v1;
        let mut v14: bool = if v12 {
            let mut v13: bool = v10 != v1;
            v13
        } else {
            false
        };
        let mut v21: bool = if v14 {
            let mut v15: Rc<str> = std::rc::Rc::<str>::from((v10).to_string());
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v15));
            let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v16));
            let mut v19: bool = v6.contains(&*v18);
            let mut v20: bool = v19 == false;
            v20
        } else {
            false
        };
        let (mut v46, mut v47, mut v48): (Rc<UH0>, Rc<str>, u64) = if v21 {
            let mut v22: i32 = v2.wrapping_add(1i32);
            let mut v23: i32 = 0i32;
            let mut v24: u64 = v0[v4 as usize].0;
            let mut v25: Rc<str> = std::rc::Rc::<str>::from((v24).to_string());
            let mut v26: u64 = v0[v4 as usize].1;
            let mut v27: Rc<str> = std::rc::Rc::<str>::from((v26).to_string());
            let mut v28: u64 = v0[v4 as usize].5;
            let mut v29: Rc<str> = std::rc::Rc::<str>::from({ let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64; let local = (now - (v28 as i64) * 60 + v3 * 60).rem_euclid(86400); format!("{:02}:{:02}", local / 3600, local / 60 % 60) }.as_str());
            let mut v30: u64 = v0[v4 as usize].7;
            let mut v31: Rc<str> = std::rc::Rc::<str>::from((v30).to_string());
            let mut v32: f64 = v0[v4 as usize].4;
            let mut v33: Rc<str> = std::rc::Rc::<str>::from(format!("{:.1}", v32).as_str());
            let mut v34: Rc<str> = std::rc::Rc::<str>::from("  ".repeat(v2 as usize).as_str());
            let mut v35: Rc<str> = v0[v4 as usize].3.clone();
            let mut v36: Rc<str> = std::rc::Rc::<str>::from({ let text = &*v35; if text.chars().count() > 200 { format!("{}...", text.chars().take(200).collect::<String>()) } else { text.to_owned() } }.as_str());
            let mut v37: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v36));
            let mut v38: Rc<str> = std::rc::Rc::<str>::from((v24).to_string());
            let mut v39: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v38));
            let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let mut v41: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v40));
            let mut v42: Rc<UH0> = Rc::new(UH0::UH0_PsRowThen(v25.clone(), v27.clone(), v29.clone(), v31.clone(), v33.clone(), v37.clone(), v5.clone()));
            let (mut v43, mut v44, mut v45): (Rc<UH0>, Rc<str>, u64) = children_listed_15(v0.clone(), v10, v22, v3, v23, v42.clone(), v41.clone(), v7);
            (v43.clone(), v44.clone(), v45)
        } else {
            (v5.clone(), v6.clone(), v7)
        };
        let mut v49: i32 = v4.wrapping_add(1i32);
        let (mut v50, mut v51, mut v52): (Rc<UH0>, Rc<str>, u64) = children_listed_15(v0.clone(), v1, v2, v3, v49, v46.clone(), v47.clone(), v48);
        (v50.clone(), v51.clone(), v52)
    };
    (v53.clone(), v54.clone(), v55)
}
fn listed_12(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v1: Rc<str>, mut v2: US1, mut v3: bool, mut v4: u64, mut v5: i64, mut v6: i32, mut v7: Rc<UH0>, mut v8: Rc<str>, mut v9: u64) -> (Rc<UH0>, Rc<str>, u64) {
    let mut v10: i32 = v0.len() as i32;
    let mut v11: bool = v6 >= v10;
    let (mut v71, mut v72, mut v73): (Rc<UH0>, Rc<str>, u64) = if v11 {
        (v7.clone(), v8.clone(), v9)
    } else {
        let mut v12: u64 = v0[v6 as usize].0;
        let mut v13: bool = v12 != v4;
        let mut v20: bool = if v13 {
            let mut v14: Rc<str> = std::rc::Rc::<str>::from((v12).to_string());
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v14));
            let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v15));
            let mut v18: bool = v8.contains(&*v17);
            let mut v19: bool = v18 == false;
            v19
        } else {
            false
        };
        let mut v27: bool = if v20 {
            match &v2 {
                US1::US1_EveryProcess => {
                    true
                }
                US1::US1_NoSession => {
                    false
                }
                US1::US1_SessionTree(v21) => {
                    let mut v21: u64 = *v21;
                    let mut v22: i32 = 0i32;
                    descends_from_13(v0.clone(), v12, v21, v22)
                }
            }
        } else {
            false
        };
        let mut v34: bool = if v27 {
            let mut v28: Rc<str> = v0[v6 as usize].2.clone();
            let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
            let mut v30: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v29));
            let mut v31: Rc<str> = v0[v6 as usize].3.clone();
            let mut v32: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v31));
            pattern_matches_14(v1.clone(), v32.clone())
        } else {
            false
        };
        let (mut v64, mut v65, mut v66): (Rc<UH0>, Rc<str>, u64) = if v34 {
            let mut v35: u64 = v9.wrapping_add(1u64);
            let mut v36: u64 = v0[v6 as usize].0;
            let mut v37: Rc<str> = std::rc::Rc::<str>::from((v36).to_string());
            let mut v38: u64 = v0[v6 as usize].1;
            let mut v39: Rc<str> = std::rc::Rc::<str>::from((v38).to_string());
            let mut v40: u64 = v0[v6 as usize].5;
            let mut v41: Rc<str> = std::rc::Rc::<str>::from({ let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64; let local = (now - (v40 as i64) * 60 + v5 * 60).rem_euclid(86400); format!("{:02}:{:02}", local / 3600, local / 60 % 60) }.as_str());
            let mut v42: u64 = v0[v6 as usize].7;
            let mut v43: Rc<str> = std::rc::Rc::<str>::from((v42).to_string());
            let mut v44: f64 = v0[v6 as usize].4;
            let mut v45: Rc<str> = std::rc::Rc::<str>::from(format!("{:.1}", v44).as_str());
            let mut v46: Rc<str> = std::rc::Rc::<str>::from("  ".repeat(0i32 as usize).as_str());
            let mut v47: Rc<str> = v0[v6 as usize].3.clone();
            let mut v48: Rc<str> = std::rc::Rc::<str>::from({ let text = &*v47; if text.chars().count() > 200 { format!("{}...", text.chars().take(200).collect::<String>()) } else { text.to_owned() } }.as_str());
            let mut v49: Rc<str> = Rc::<str>::from(format!("{}{}", v46, v48));
            let mut v50: Rc<str> = std::rc::Rc::<str>::from((v36).to_string());
            let mut v51: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v50));
            let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let mut v53: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v52));
            let (mut v61, mut v62, mut v63): (Rc<UH0>, Rc<str>, u64) = if v3 {
                let mut v54: i32 = 1i32;
                let mut v55: i32 = 0i32;
                let mut v56: Rc<UH0> = Rc::new(UH0::UH0_PsRowThen(v37.clone(), v39.clone(), v41.clone(), v43.clone(), v45.clone(), v49.clone(), v7.clone()));
                let (mut v57, mut v58, mut v59): (Rc<UH0>, Rc<str>, u64) = children_listed_15(v0.clone(), v12, v54, v5, v55, v56.clone(), v53.clone(), v35);
                (v57.clone(), v58.clone(), v59)
            } else {
                let mut v60: Rc<UH0> = Rc::new(UH0::UH0_PsRowThen(v37.clone(), v39.clone(), v41.clone(), v43.clone(), v45.clone(), v49.clone(), v7.clone()));
                (v60.clone(), v53.clone(), v35)
            };
            (v61.clone(), v62.clone(), v63)
        } else {
            (v7.clone(), v8.clone(), v9)
        };
        let mut v67: i32 = v6.wrapping_add(1i32);
        let (mut v68, mut v69, mut v70): (Rc<UH0>, Rc<str>, u64) = listed_12(v0.clone(), v1.clone(), v2.clone(), v3, v4, v5, v67, v64.clone(), v65.clone(), v66);
        (v68.clone(), v69.clone(), v70)
    };
    (v71.clone(), v72.clone(), v73)
}
fn widths_of_17(mut v0: Rc<UH0>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32) -> (i32, i32, i32, i32, i32) {
    let (mut v38, mut v39, mut v40, mut v41, mut v42): (i32, i32, i32, i32, i32) = match &*v0 {
        UH0::UH0_NoPsRows => {
            (v1, v2, v3, v4, v5)
        }
        _ => {
            let (mut v33, mut v34, mut v35, mut v36, mut v37): (i32, i32, i32, i32, i32) = match &*v0 {
                UH0::UH0_PsRowThen(v6, v7, v8, v9, v10, v11, v12) => {
                    let mut v6: Rc<str> = v6.clone();
                    let mut v7: Rc<str> = v7.clone();
                    let mut v8: Rc<str> = v8.clone();
                    let mut v9: Rc<str> = v9.clone();
                    let mut v10: Rc<str> = v10.clone();
                    let mut v11: Rc<str> = v11.clone();
                    let mut v12: Rc<UH0> = v12.clone();
                    let mut v13: i32 = (&*v6).chars().count() as i32;
                    let mut v14: bool = v1 < v13;
                    let mut v15: i32 = if v14 {
                        v13
                    } else {
                        v1
                    };
                    let mut v16: i32 = (&*v7).chars().count() as i32;
                    let mut v17: bool = v2 < v16;
                    let mut v18: i32 = if v17 {
                        v16
                    } else {
                        v2
                    };
                    let mut v19: i32 = (&*v8).chars().count() as i32;
                    let mut v20: bool = v3 < v19;
                    let mut v21: i32 = if v20 {
                        v19
                    } else {
                        v3
                    };
                    let mut v22: i32 = (&*v9).chars().count() as i32;
                    let mut v23: bool = v4 < v22;
                    let mut v24: i32 = if v23 {
                        v22
                    } else {
                        v4
                    };
                    let mut v25: i32 = (&*v10).chars().count() as i32;
                    let mut v26: bool = v5 < v25;
                    let mut v27: i32 = if v26 {
                        v25
                    } else {
                        v5
                    };
                    widths_of_17(v12.clone(), v15, v18, v21, v24, v27)
                }
                _ => unreachable!(),
            };
            (v33, v34, v35, v36, v37)
        }
    };
    (v38, v39, v40, v41, v42)
}
fn rows_text_18(mut v0: Rc<UH0>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: Rc<str>) -> Rc<str> {
    loop {
        match &*v0 {
            UH0::UH0_NoPsRows => {
                return v6.clone();
            }
            UH0::UH0_PsRowThen(v7, v8, v9, v10, v11, v12, v13) => {
                let mut v7: Rc<str> = v7.clone();
                let mut v8: Rc<str> = v8.clone();
                let mut v9: Rc<str> = v9.clone();
                let mut v10: Rc<str> = v10.clone();
                let mut v11: Rc<str> = v11.clone();
                let mut v12: Rc<str> = v12.clone();
                let mut v13: Rc<UH0> = v13.clone();
                let mut v14: bool = v6.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v50: Rc<str> = if v14 {
                    let mut v15: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v7, width = v1 as usize).as_str());
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                    let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
                    let mut v18: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v8, width = v2 as usize).as_str());
                    let mut v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
                    let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v16));
                    let mut v21: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v9, width = v3 as usize).as_str());
                    let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
                    let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v16));
                    let mut v24: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v10, width = v4 as usize).as_str());
                    let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v24));
                    let mut v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v16));
                    let mut v27: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v11, width = v5 as usize).as_str());
                    let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v16));
                    let mut v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v12));
                    let mut v30: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v29));
                    v30.clone()
                } else {
                    let mut v31: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v7, width = v1 as usize).as_str());
                    let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                    let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
                    let mut v34: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v8, width = v2 as usize).as_str());
                    let mut v35: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v34));
                    let mut v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v32));
                    let mut v37: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v9, width = v3 as usize).as_str());
                    let mut v38: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v37));
                    let mut v39: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v32));
                    let mut v40: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v10, width = v4 as usize).as_str());
                    let mut v41: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v40));
                    let mut v42: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v32));
                    let mut v43: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v11, width = v5 as usize).as_str());
                    let mut v44: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v32));
                    let mut v45: Rc<str> = Rc::<str>::from(format!("{}{}", v44, v12));
                    let mut v46: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v45));
                    let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let mut v48: Rc<str> = Rc::<str>::from(format!("{}{}", v46, v47));
                    let mut v49: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v6));
                    v49.clone()
                };
                (v0, v1, v2, v3, v4, v5, v6) = (v13.clone(), v1, v2, v3, v4, v5, v50.clone());
                continue;
            }
        }
    }
}
fn listing_text_16(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: u64) -> Rc<str> {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pid"); } LIT.with(|lit| lit.clone()) };
    let mut v4: i32 = (&*v3).chars().count() as i32;
    let mut v5: bool = 0i32 < v4;
    let mut v6: i32 = if v5 {
        v4
    } else {
        0i32
    };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ppid"); } LIT.with(|lit| lit.clone()) };
    let mut v8: i32 = (&*v7).chars().count() as i32;
    let mut v9: bool = 0i32 < v8;
    let mut v10: i32 = if v9 {
        v8
    } else {
        0i32
    };
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("started"); } LIT.with(|lit| lit.clone()) };
    let mut v12: i32 = (&*v11).chars().count() as i32;
    let mut v13: bool = 0i32 < v12;
    let mut v14: i32 = if v13 {
        v12
    } else {
        0i32
    };
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("mem_mb"); } LIT.with(|lit| lit.clone()) };
    let mut v16: i32 = (&*v15).chars().count() as i32;
    let mut v17: bool = 0i32 < v16;
    let mut v18: i32 = if v17 {
        v16
    } else {
        0i32
    };
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cores"); } LIT.with(|lit| lit.clone()) };
    let mut v20: i32 = (&*v19).chars().count() as i32;
    let mut v21: bool = 0i32 < v20;
    let mut v22: i32 = if v21 {
        v20
    } else {
        0i32
    };
    let (mut v23, mut v24, mut v25, mut v26, mut v27): (i32, i32, i32, i32, i32) = widths_of_17(v0.clone(), v6, v10, v14, v18, v22);
    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v29: Rc<str> = rows_text_18(v0.clone(), v23, v24, v25, v26, v27, v28.clone());
    let mut v30: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v3, width = v23 as usize).as_str());
    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let mut v32: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v31));
    let mut v33: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v7, width = v24 as usize).as_str());
    let mut v34: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v33));
    let mut v35: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v31));
    let mut v36: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v11, width = v25 as usize).as_str());
    let mut v37: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v36));
    let mut v38: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v31));
    let mut v39: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v15, width = v26 as usize).as_str());
    let mut v40: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v39));
    let mut v41: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v31));
    let mut v42: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v19, width = v27 as usize).as_str());
    let mut v43: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v31));
    let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("command"); } LIT.with(|lit| lit.clone()) };
    let mut v45: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v44));
    let mut v46: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v45));
    let mut v47: bool = v29.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v47 {
        v46.clone()
    } else {
        let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v49: Rc<str> = Rc::<str>::from(format!("{}{}", v46, v48));
        let mut v50: Rc<str> = Rc::<str>::from(format!("{}{}", v49, v29));
        v50.clone()
    }
}
fn decimal_text_19(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1.clone()
}
fn elapsed_text_21(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1.clone()
}
fn clock_text_22() -> Rc<str> {
    let mut v0: i64 = local_offset_minutes_11();
    let mut v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1.clone()
}
fn receipt_line_20(mut v0: US2, mut v1: Rc<str>, mut v2: u64) -> () {
    let mut v6: Rc<str> = match &v0 {
        US2::US2_Failed => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US2::US2_Succeeded => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
    };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = elapsed_text_21(v2);
    let mut v9: Rc<str> = clock_text_22();
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v7, &*v9, &*v10, &*v10].concat());
    let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v7, &*v1, &*v7, &*v11].concat());
    println!("{}", &*v12);
    ()
}
fn list_run_1(mut v0: u64) -> i32 {
    let mut v1: i32 = argument_count_2();
    let mut v2: i32 = 2i32;
    let mut v3: Rc<str> = pattern_from_4(v2, v1);
    let mut v4: Rc<str> = pattern_problem_7(v3.clone());
    let mut v5: bool = v4.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v5 {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ps: invalid pattern: "); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v4));
        println!("{}", &*v7);
        2i32
    } else {
        let mut v8: u64 = 500u64;
        let mut v9: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>> = process_rows_sampled_8(v8);
        let mut v10: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>> = { let mut sorted = (*v9).clone(); sorted.sort_by(|left, right| right.5.cmp(&left.5).then(left.0.cmp(&right.0))); std::rc::Rc::new(sorted) };
        let mut v11: u64 = std::process::id() as u64;
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--mine"); } LIT.with(|lit| lit.clone()) };
        let mut v13: i32 = 2i32;
        let mut v14: bool = flag_present_9(v12.clone(), v13, v1);
        let mut v22: US1 = if v14 {
            let mut v15: i32 = 0i32;
            let mut v16: u64 = session_root_10(v10.clone(), v11, v15);
            let mut v17: bool = v16 == 0u64;
            if v17 {
                US1::US1_NoSession
            } else {
                US1::US1_SessionTree(v16)
            }
        } else {
            US1::US1_EveryProcess
        };
        match &v22 {
            US1::US1_NoSession => {
                let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ps --mine: no claude or codex process above this eoie"); } LIT.with(|lit| lit.clone()) };
                println!("{}", &*v23);
                2i32
            }
            _ => {
                let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--tree"); } LIT.with(|lit| lit.clone()) };
                let mut v25: i32 = 2i32;
                let mut v26: bool = flag_present_9(v24.clone(), v25, v1);
                let mut v27: i64 = local_offset_minutes_11();
                let mut v28: i32 = 0i32;
                let mut v29: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoPsRows); } CASE.with(|case| case.clone()) };
                let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                let mut v31: u64 = 0u64;
                let (mut v32, mut v33, mut v34): (Rc<UH0>, Rc<str>, u64) = listed_12(v10.clone(), v3.clone(), v22.clone(), v26, v11, v27, v28, v29.clone(), v30.clone(), v31);
                let mut v35: Rc<str> = listing_text_16(v32.clone(), v33.clone(), v34);
                println!("{}", &*v35);
                let mut v36: US2 = US2::US2_Succeeded;
                let mut v37: Rc<str> = decimal_text_19(v34);
                let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("processes "); } LIT.with(|lit| lit.clone()) };
                let mut v39: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v37));
                let mut v46: Rc<str> = match &v22 {
                    US1::US1_EveryProcess => {
                        let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v40.clone()
                    }
                    US1::US1_SessionTree(v41) => {
                        let mut v41: u64 = *v41;
                        let mut v42: Rc<str> = std::rc::Rc::<str>::from((v41).to_string());
                        let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  session "); } LIT.with(|lit| lit.clone()) };
                        let mut v44: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v42));
                        v44.clone()
                    }
                    _ => unreachable!(),
                };
                let mut v47: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v46));
                receipt_line_20(v36.clone(), v47.clone(), v0);
                0i32
            }
        }
    }
}
fn ancestor_pids_24() -> Rc<str> {
    let mut v0: Rc<str> = { let system = sysinfo::System::new_with_specifics(sysinfo::RefreshKind::nothing().with_processes(sysinfo::ProcessRefreshKind::nothing())); let mut chain: Vec<u32> = Vec::new(); let mut current = sysinfo::Pid::from_u32(std::process::id()); while let Some(parent) = system.process(current).and_then(|process| process.parent()) { if chain.contains(&parent.as_u32()) { break; } chain.push(parent.as_u32()); current = parent; } std::rc::Rc::<str>::from(chain.iter().map(|pid| pid.to_string()).collect::<Vec<_>>().join(",")) };
    v0.clone()
}
fn method28(mut v0: Option<u64>) -> Option<u64> {
    v0.clone()
}
fn closure1() -> Rc<dyn Fn((u64)) -> US4> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US4> = Rc::new(move |mut v0: (u64)| -> US4 {
        let mut v1: u64 = (v0);
        US4::US4_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_of_27(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method28(v1.clone());
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
fn stop_refusal_of_26(mut v0: Rc<str>, mut v1: u64, mut v2: Rc<str>) -> US3 {
    let mut v3: bool = !v0.is_empty() && v0.bytes().all(|byte| byte.is_ascii_digit());
    if v3 {
        let mut v4: u64 = decimal_of_27(v0.clone());
        let mut v5: bool = v4 == v1;
        if v5 {
            US3::US3_OwnProcess
        } else {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v0));
            let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v7));
            let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v2));
            let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v7));
            let mut v12: bool = v11.contains(&*v9);
            if v12 {
                US3::US3_OwnAncestor
            } else {
                US3::US3_Allowed(v4)
            }
        }
    } else {
        US3::US3_NotAProcessId
    }
}
fn process_stopped_30(mut v0: u64, mut v1: bool) -> (u64, u64) {
    let mut v2: (u64, u64) = { let system = sysinfo::System::new_with_specifics(sysinfo::RefreshKind::nothing().with_processes(sysinfo::ProcessRefreshKind::nothing())); let root = sysinfo::Pid::from_u32(v0 as u32); match system.process(root) { None => (0u64, 0u64), Some(_) => { let mut order = vec![root]; if v1 { let mut index = 0; while index < order.len() { let parent = order[index]; for (pid, process) in system.processes() { if process.parent() == Some(parent) && !order.contains(pid) { order.push(*pid); } } index += 1; } } for pid in order.iter().rev() { #[cfg(windows)] { unsafe extern "system" { fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void; fn TerminateProcess(process: *mut std::ffi::c_void, code: u32) -> i32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; } unsafe { let handle = OpenProcess(1, 0, pid.as_u32()); if !handle.is_null() { TerminateProcess(handle, 1); CloseHandle(handle); } } } #[cfg(not(windows))] { if let Some(process) = system.process(*pid) { process.kill(); } } } std::thread::sleep(std::time::Duration::from_millis(200)); let after = sysinfo::System::new_with_specifics(sysinfo::RefreshKind::nothing().with_processes(sysinfo::ProcessRefreshKind::nothing())); let survivors = order.iter().filter(|pid| { #[cfg(windows)] { let _ = &after; unsafe extern "system" { fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void; fn WaitForSingleObject(handle: *mut std::ffi::c_void, ms: u32) -> u32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; } unsafe { let handle = OpenProcess(0x0010_0000, 0, pid.as_u32()); if handle.is_null() { false } else { let running = WaitForSingleObject(handle, 2000) == 258; CloseHandle(handle); running } } } #[cfg(not(windows))] { after.process(**pid).is_some() } }).count(); ((order.len() - survivors) as u64, order.len() as u64) } } };
    v2.clone()
}
fn stop_once_29(mut v0: Rc<str>, mut v1: u64, mut v2: bool) -> bool {
    let mut v3: (u64, u64) = process_stopped_30(v1, v2);
    let mut v4: u64 = v3.clone().0;
    let mut v5: u64 = v3.1;
    let mut v6: bool = v5 == 0u64;
    let mut v9: US5 = if v6 {
        US5::US5_NoSuchProcess
    } else {
        US5::US5_StoppedOf(v4, v5)
    };
    let mut v45: Rc<str> = match &v9 {
        US5::US5_NoSuchProcess => {
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop "); } LIT.with(|lit| lit.clone()) };
            let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v0));
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  not found"); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
            v13.clone()
        }
        US5::US5_StoppedOf(v14, v15) => {
            let mut v14: u64 = *v14;
            let mut v15: u64 = *v15;
            let mut v16: bool = v14 != v15;
            if v16 {
                let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop "); } LIT.with(|lit| lit.clone()) };
                let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v0));
                let mut v19: Rc<str> = std::rc::Rc::<str>::from((v14).to_string());
                let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  stopped "); } LIT.with(|lit| lit.clone()) };
                let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v19));
                let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" of "); } LIT.with(|lit| lit.clone()) };
                let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v22));
                let mut v24: Rc<str> = std::rc::Rc::<str>::from((v15).to_string());
                let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v24));
                let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" processes"); } LIT.with(|lit| lit.clone()) };
                let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v26));
                let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v27));
                v28.clone()
            } else {
                let mut v29: bool = v15 == 1u64;
                if v29 {
                    let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop "); } LIT.with(|lit| lit.clone()) };
                    let mut v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v0));
                    let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  stopped"); } LIT.with(|lit| lit.clone()) };
                    let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
                    v33.clone()
                } else {
                    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop "); } LIT.with(|lit| lit.clone()) };
                    let mut v35: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v0));
                    let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  stopped (tree: "); } LIT.with(|lit| lit.clone()) };
                    let mut v37: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v36));
                    let mut v38: Rc<str> = std::rc::Rc::<str>::from((v15).to_string());
                    let mut v39: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v38));
                    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" processes)"); } LIT.with(|lit| lit.clone()) };
                    let mut v41: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v40));
                    v41.clone()
                }
            }
        }
    };
    println!("{}", &*v45);
    match &v9 {
        US5::US5_NoSuchProcess => {
            false
        }
        US5::US5_StoppedOf(v46, v47) => {
            let mut v46: u64 = *v46;
            let mut v47: u64 = *v47;
            let mut v48: bool = v46 == v47;
            v48
        }
    }
}
fn stops_from_25(mut v0: i32, mut v1: i32, mut v2: bool, mut v3: u64, mut v4: Rc<str>, mut v5: u64, mut v6: u64) -> (u64, u64) {
    loop {
        let mut v7: bool = v0 < v1;
        if v7 {
            let mut v8: Rc<str> = argument_5(v0);
            let mut v9: bool = v8.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--tree"); } LIT.with(|lit| lit.clone()) };
            if v9 {
                let mut v10: i32 = v0.wrapping_add(1i32);
                (v0, v1, v2, v3, v4, v5, v6) = (v10, v1, v2, v3, v4.clone(), v5, v6);
                continue;
            } else {
                let mut v13: US3 = stop_refusal_of_26(v8.clone(), v3, v4.clone());
                match &v13 {
                    US3::US3_Allowed(v14) => {
                        let mut v14: u64 = *v14;
                        let mut v15: bool = stop_once_29(v8.clone(), v14, v2);
                        if v15 {
                            let mut v16: i32 = v0.wrapping_add(1i32);
                            let mut v17: u64 = v5.wrapping_add(1u64);
                            (v0, v1, v2, v3, v4, v5, v6) = (v16, v1, v2, v3, v4.clone(), v17, v6);
                            continue;
                        } else {
                            let mut v20: i32 = v0.wrapping_add(1i32);
                            let mut v21: u64 = v6.wrapping_add(1u64);
                            (v0, v1, v2, v3, v4, v5, v6) = (v20, v1, v2, v3, v4.clone(), v5, v21);
                            continue;
                        }
                    }
                    _ => {
                        let mut v40: Rc<str> = match &v13 {
                            US3::US3_NotAProcessId => {
                                let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop "); } LIT.with(|lit| lit.clone()) };
                                let mut v35: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v8));
                                let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  refused: not a process id"); } LIT.with(|lit| lit.clone()) };
                                let mut v37: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v36));
                                v37.clone()
                            }
                            US3::US3_OwnAncestor => {
                                let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop "); } LIT.with(|lit| lit.clone()) };
                                let mut v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v8));
                                let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  refused: that is a parent of this eoie (the calling shell)"); } LIT.with(|lit| lit.clone()) };
                                let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
                                v33.clone()
                            }
                            US3::US3_OwnProcess => {
                                let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop "); } LIT.with(|lit| lit.clone()) };
                                let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v8));
                                let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  refused: that is this eoie process"); } LIT.with(|lit| lit.clone()) };
                                let mut v29: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v28));
                                v29.clone()
                            }
                            _ => unreachable!(),
                        };
                        println!("{}", &*v40);
                        let mut v41: i32 = v0.wrapping_add(1i32);
                        let mut v42: u64 = v6.wrapping_add(1u64);
                        (v0, v1, v2, v3, v4, v5, v6) = (v41, v1, v2, v3, v4.clone(), v5, v42);
                        continue;
                    }
                }
            }
        } else {
            return (v5, v6);
        }
    }
}
fn stop_run_23(mut v0: u64) -> i32 {
    let mut v1: i32 = argument_count_2();
    let mut v2: bool = v1 < 3i32;
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: eoie "); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ps [regex] [--tree] [--mine] | kill <pid>... [--tree]"); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: eoie ps [regex] [--tree] [--mine] | kill <pid>... [--tree]"); } LIT.with(|lit| lit.clone()) };
        println!("{}", &*v5);
        2i32
    } else {
        let mut v6: i32 = 2i32;
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--tree"); } LIT.with(|lit| lit.clone()) };
        let mut v8: i32 = 2i32;
        let mut v9: bool = flag_present_9(v7.clone(), v8, v1);
        let mut v10: u64 = std::process::id() as u64;
        let mut v11: Rc<str> = ancestor_pids_24();
        let mut v12: u64 = 0u64;
        let mut v13: u64 = 0u64;
        let (mut v14, mut v15): (u64, u64) = stops_from_25(v6, v1, v9, v10, v11.clone(), v12, v13);
        let mut v16: Rc<str> = decimal_text_19(v14);
        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped "); } LIT.with(|lit| lit.clone()) };
        let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v16));
        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  failed "); } LIT.with(|lit| lit.clone()) };
        let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v19));
        let mut v21: Rc<str> = decimal_text_19(v15);
        let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
        let mut v23: bool = v15 == 0u64;
        if v23 {
            let mut v24: US2 = US2::US2_Succeeded;
            receipt_line_20(v24.clone(), v22.clone(), v0);
            0i32
        } else {
            let mut v25: US2 = US2::US2_Failed;
            receipt_line_20(v25.clone(), v22.clone(), v0);
            1i32
        }
    }
}
fn now_ms_31() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn process_run_0() -> i32 {
    let mut v0: u64 = now_ms_31();
    let mut v1: i32 = 1i32;
    let mut v2: Rc<str> = argument_5(v1);
    let mut v3: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ps"); } LIT.with(|lit| lit.clone()) };
    let mut v9: US6 = if v3 {
        US6::US6_ListProcesses
    } else {
        let mut v5: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kill"); } LIT.with(|lit| lit.clone()) };
        if v5 {
            US6::US6_StopProcesses
        } else {
            US6::US6_ProcessUnknown
        }
    };
    match &v9 {
        US6::US6_ListProcesses => {
            list_run_1(v0)
        }
        US6::US6_ProcessUnknown => {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: eoie "); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ps [regex] [--tree] [--mine] | kill <pid>... [--tree]"); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: eoie ps [regex] [--tree] [--mine] | kill <pid>... [--tree]"); } LIT.with(|lit| lit.clone()) };
            println!("{}", &*v14);
            2i32
        }
        US6::US6_StopProcesses => {
            stop_run_23(v0)
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        process_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_process_run() -> i32 {
    closure0()()
}
