#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US1 {
    US1_RunForeground(Rc<str>, u64),
    US1_RunUntil(Rc<str>, Rc<str>, u64),
    US1_RunBackground(Rc<str>, Rc<str>),
    US1_TailOutput(u64),
    US1_SearchOutput(Rc<str>),
    US1_ErrorsBy(Rc<str>),
    US1_ProcessesMatching(Rc<str>),
    US1_StopNamed(Rc<str>),
}
#[derive(Clone)]
enum US0 {
    US0_StepParsed(US1),
    US0_StepRejected(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_Exited(i64),
    US2_TimedOut,
    US2_Matched,
    US2_NoMatch,
    US2_ExitedBeforeMatch(i64),
    US2_BadPattern,
}
#[derive(Clone)]
enum US3 {
    US3_Succeeded,
    US3_Failed,
}
fn records_of_0(mut v0: Rc<str>) -> std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> {
    let mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new((&*v0).split(char::from(30u8)).map(|record| record.split(char::from(31u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
    v1.clone()
}
fn field_at_2(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let mut v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).cloned().unwrap_or_default();
    v3.clone()
}
fn command_words_5(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(v0.get(v1 as usize).map(|fields| fields.iter().skip(2).take_while(|field| ![("w", "timeout"), ("w", "until"), ("w", "background"), ("w", "then")].iter().any(|(kind, word)| field.starts_with(kind) && &field[1..] == *word)).filter(|field| !field.is_empty()).map(|field| if field.starts_with(char::from(113u8)) { format!("{}{}{}", char::from(34u8), &field[1..], char::from(34u8)) } else { field[1..].to_owned() }).collect::<Vec<_>>().join(" ")).unwrap_or_default());
    v2.clone()
}
fn field_index_6(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: Rc<str>) -> i32 {
    let mut v3: i32 = v0.get(v1 as usize).and_then(|fields| fields.iter().position(|field| field.len() > 1 && field.starts_with(char::from(119u8)) && &field[1..] == &*v2)).map(|index| index as i32).unwrap_or(-1);
    v3
}
fn body_at_7(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let mut v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).filter(|field| !field.is_empty()).map(|field| std::rc::Rc::<str>::from(&field[1..])).unwrap_or_default();
    v3.clone()
}
fn duration_ms_8(mut v0: Rc<str>) -> u64 {
    let mut v1: u64 = { let text = (&*v0).trim(); let (digits, scale) = if let Some(value) = text.strip_suffix("ms") { (value, 1) } else if let Some(value) = text.strip_suffix(char::from(115u8)) { (value, 1000) } else if let Some(value) = text.strip_suffix(char::from(109u8)) { (value, 60000) } else { (text, 1000) }; digits.trim().parse::<u64>().map(|value| value * scale).unwrap_or(0) };
    v1
}
fn first_of_kind_9(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: u8) -> Rc<str> {
    let mut v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.iter().skip(2).find(|field| field.as_bytes().first() == Some(&v2)).map(|field| std::rc::Rc::<str>::from(&field[1..]))).unwrap_or_default();
    v3.clone()
}
fn run_parsed_4(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let mut v2: Rc<str> = command_words_5(v0.clone(), v1);
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout"); } LIT.with(|lit| lit.clone()) };
    let mut v4: i32 = field_index_6(v0.clone(), v1, v3.clone());
    let mut v5: bool = v4 < 0i32;
    let mut v9: u64 = if v5 {
        600000u64
    } else {
        let mut v6: i32 = v4.wrapping_add(1i32);
        let mut v7: Rc<str> = body_at_7(v0.clone(), v1, v6);
        duration_ms_8(v7.clone())
    };
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("background"); } LIT.with(|lit| lit.clone()) };
    let mut v11: i32 = field_index_6(v0.clone(), v1, v10.clone());
    let mut v12: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v12 {
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("run: expected a command"); } LIT.with(|lit| lit.clone()) };
        US0::US0_StepRejected(v13.clone())
    } else {
        let mut v15: bool = v9 == 0u64;
        if v15 {
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout: expected a duration like 20s"); } LIT.with(|lit| lit.clone()) };
            US0::US0_StepRejected(v16.clone())
        } else {
            let mut v18: bool = v11 >= 0i32;
            if v18 {
                let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("as"); } LIT.with(|lit| lit.clone()) };
                let mut v20: i32 = field_index_6(v0.clone(), v1, v19.clone());
                let mut v21: i32 = v20.wrapping_add(1i32);
                let mut v22: Rc<str> = body_at_7(v0.clone(), v1, v21);
                let mut v23: bool = v22.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                if v23 {
                    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("background: expected as <name>"); } LIT.with(|lit| lit.clone()) };
                    US0::US0_StepRejected(v24.clone())
                } else {
                    let mut v26: US1 = US1::US1_RunBackground(v2.clone(), v22.clone());
                    US0::US0_StepParsed(v26.clone())
                }
            } else {
                let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("until"); } LIT.with(|lit| lit.clone()) };
                let mut v30: i32 = field_index_6(v0.clone(), v1, v29.clone());
                let mut v31: bool = v30 >= 0i32;
                if v31 {
                    let mut v32: u8 = 113u8;
                    let mut v33: Rc<str> = first_of_kind_9(v0.clone(), v1, v32);
                    let mut v34: bool = v33.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    if v34 {
                        let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("until: expected output ~ \"<regex>\""); } LIT.with(|lit| lit.clone()) };
                        US0::US0_StepRejected(v35.clone())
                    } else {
                        let mut v37: US1 = US1::US1_RunUntil(v2.clone(), v33.clone(), v9);
                        US0::US0_StepParsed(v37.clone())
                    }
                } else {
                    let mut v40: US1 = US1::US1_RunForeground(v2.clone(), v9);
                    US0::US0_StepParsed(v40.clone())
                }
            }
        }
    }
}
fn decimal_of_10(mut v0: Rc<str>) -> u64 {
    let mut v1: u64 = v0.trim().parse::<u64>().unwrap_or(0);
    v1
}
fn step_parsed_3(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let mut v2: i32 = 1i32;
    let mut v3: Rc<str> = field_at_2(v0.clone(), v1, v2);
    let mut v4: u8 = 113u8;
    let mut v5: Rc<str> = first_of_kind_9(v0.clone(), v1, v4);
    let mut v6: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("run"); } LIT.with(|lit| lit.clone()) };
    if v6 {
        run_parsed_4(v0.clone(), v1)
    } else {
        let mut v8: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tail"); } LIT.with(|lit| lit.clone()) };
        if v8 {
            let mut v9: u8 = 119u8;
            let mut v10: Rc<str> = first_of_kind_9(v0.clone(), v1, v9);
            let mut v11: bool = v10.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("output"); } LIT.with(|lit| lit.clone()) };
            let mut v12: bool = v11 != true;
            if v12 {
                let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tail: a run program reads its own output: tail output <n>"); } LIT.with(|lit| lit.clone()) };
                US0::US0_StepRejected(v13.clone())
            } else {
                let mut v15: i32 = 3i32;
                let mut v16: Rc<str> = body_at_7(v0.clone(), v1, v15);
                let mut v17: u64 = decimal_of_10(v16.clone());
                let mut v18: bool = v17 == 0u64;
                let mut v19: u64 = if v18 {
                    10u64
                } else {
                    v17
                };
                let mut v20: US1 = US1::US1_TailOutput(v19);
                US0::US0_StepParsed(v20.clone())
            }
        } else {
            let mut v23: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("search"); } LIT.with(|lit| lit.clone()) };
            if v23 {
                let mut v24: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                if v24 {
                    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("search: expected a quoted pattern"); } LIT.with(|lit| lit.clone()) };
                    US0::US0_StepRejected(v25.clone())
                } else {
                    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("in"); } LIT.with(|lit| lit.clone()) };
                    let mut v28: i32 = field_index_6(v0.clone(), v1, v27.clone());
                    let mut v29: i32 = v28.wrapping_add(1i32);
                    let mut v30: Rc<str> = body_at_7(v0.clone(), v1, v29);
                    let mut v31: bool = v30.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("output"); } LIT.with(|lit| lit.clone()) };
                    if v31 {
                        let mut v32: US1 = US1::US1_SearchOutput(v5.clone());
                        US0::US0_StepParsed(v32.clone())
                    } else {
                        let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("search: a run program searches its own output: search \"<regex>\" in output"); } LIT.with(|lit| lit.clone()) };
                        US0::US0_StepRejected(v34.clone())
                    }
                }
            } else {
                let mut v38: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("errors"); } LIT.with(|lit| lit.clone()) };
                if v38 {
                    let mut v39: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v43: bool = if v39 {
                        true
                    } else {
                        let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("by"); } LIT.with(|lit| lit.clone()) };
                        let mut v41: i32 = field_index_6(v0.clone(), v1, v40.clone());
                        let mut v42: bool = v41 < 0i32;
                        v42
                    };
                    if v43 {
                        let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("errors: expected errors in output by \"<regex>\""); } LIT.with(|lit| lit.clone()) };
                        US0::US0_StepRejected(v44.clone())
                    } else {
                        let mut v46: US1 = US1::US1_ErrorsBy(v5.clone());
                        US0::US0_StepParsed(v46.clone())
                    }
                } else {
                    let mut v49: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("processes"); } LIT.with(|lit| lit.clone()) };
                    if v49 {
                        let mut v50: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        if v50 {
                            let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("processes: expected where command ~ \"<regex>\""); } LIT.with(|lit| lit.clone()) };
                            US0::US0_StepRejected(v51.clone())
                        } else {
                            let mut v53: US1 = US1::US1_ProcessesMatching(v5.clone());
                            US0::US0_StepParsed(v53.clone())
                        }
                    } else {
                        let mut v56: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop"); } LIT.with(|lit| lit.clone()) };
                        if v56 {
                            let mut v57: u8 = 119u8;
                            let mut v58: Rc<str> = first_of_kind_9(v0.clone(), v1, v57);
                            let mut v59: bool = v58.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            if v59 {
                                let mut v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop: expected a run name"); } LIT.with(|lit| lit.clone()) };
                                US0::US0_StepRejected(v60.clone())
                            } else {
                                let mut v62: US1 = US1::US1_StopNamed(v58.clone());
                                US0::US0_StepParsed(v62.clone())
                            }
                        } else {
                            let mut v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cannot share one program with run yet; run it as its own call"); } LIT.with(|lit| lit.clone()) };
                            let mut v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let mut v67: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v65, &*v66, &*v66, &*v66].concat());
                            US0::US0_StepRejected(v67.clone())
                        }
                    }
                }
            }
        }
    }
}
fn query_rejection_1(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> Rc<str> {
    loop {
        let mut v2: i32 = v0.len() as i32;
        let mut v3: bool = v1 < v2;
        if v3 {
            let mut v4: i32 = 0i32;
            let mut v5: Rc<str> = field_at_2(v0.clone(), v1, v4);
            let mut v6: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("query"); } LIT.with(|lit| lit.clone()) };
            let mut v7: bool = v6 != true;
            if v7 {
                let mut v8: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v0.clone(), v8);
                continue;
            } else {
                let mut v10: US0 = step_parsed_3(v0.clone(), v1);
                match &v10 {
                    US0::US0_StepParsed(v12) => {
                        let mut v12: US1 = v12.clone();
                        let mut v13: i32 = v1.wrapping_add(1i32);
                        (v0, v1) = (v0.clone(), v13);
                        continue;
                    }
                    US0::US0_StepRejected(v11) => {
                        let mut v11: Rc<str> = v11.clone();
                        return v11.clone();
                    }
                }
            }
        } else {
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v18.clone();
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        let mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = records_of_0(v0.clone());
        let mut v2: i32 = 1i32;
        query_rejection_1(v1.clone(), v2)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn state_new_12() -> std::rc::Rc<std::cell::RefCell<(String, Vec<(String, std::process::Child, u64)>)>> {
    let mut v0: std::rc::Rc<std::cell::RefCell<(String, Vec<(String, std::process::Child, u64)>)>> = std::rc::Rc::new(std::cell::RefCell::new((String::new(), Vec::new())));
    v0.clone()
}
fn empty_run_tally_13() -> (Rc<str>, bool, u64, bool, i64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0.clone(), false, 0u64, false, 0i64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64)
}
fn run_spawned_16(mut v0: std::rc::Rc<std::cell::RefCell<(String, Vec<(String, std::process::Child, u64)>)>>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let mut v4: Rc<str> = std::rc::Rc::<str>::from((|| -> std::io::Result<()> { let path = std::env::temp_dir().join(format!("eoie-do-run-{}-{}.log", std::process::id(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_nanos()).unwrap_or(0))); let log = std::fs::File::create(&path)?; let log_errors = log.try_clone()?; #[cfg(windows)] let mut process = { use std::os::windows::process::CommandExt; let mut process = std::process::Command::new("cmd.exe"); process.arg("/c").raw_arg(&*v2).env_remove("NoDefaultCurrentDirectoryInExePath"); process }; #[cfg(not(windows))] let mut process = { let mut process = std::process::Command::new("sh"); process.arg("-c").arg(&*v2); process }; process.current_dir(&*v1).stdin(std::process::Stdio::null()).stdout(log).stderr(log_errors); #[cfg(unix)] { use std::os::unix::process::CommandExt; process.process_group(0); } let child = process.spawn()?; #[cfg(windows)] let job = { unsafe extern "system" { fn CreateJobObjectW(attributes: *mut std::ffi::c_void, name: *const u16) -> *mut std::ffi::c_void; fn SetInformationJobObject(job: *mut std::ffi::c_void, class: i32, information: *mut std::ffi::c_void, length: u32) -> i32; fn AssignProcessToJobObject(job: *mut std::ffi::c_void, process: *mut std::ffi::c_void) -> i32; } use std::os::windows::io::AsRawHandle; unsafe { let job = CreateJobObjectW(std::ptr::null_mut(), std::ptr::null()); if !job.is_null() { let mut limits = [0u64; 18]; limits[2] = 0x800; SetInformationJobObject(job, 9, limits.as_mut_ptr().cast(), 144); AssignProcessToJobObject(job, child.as_raw_handle()); } job as u64 } }; #[cfg(not(windows))] let job = 0u64; let mut state = v0.borrow_mut(); state.0 = path.to_string_lossy().into_owned(); state.1.push(((&*v3).to_owned(), child, job)); Ok(()) })().err().map(|error| format!("spawn failed: {error}")).unwrap_or_default());
    v4.clone()
}
fn run_ended_18(mut v0: std::rc::Rc<std::cell::RefCell<(String, Vec<(String, std::process::Child, u64)>)>>, mut v1: Rc<str>, mut v2: u64) -> (u8, i64, u64, u64) {
    let mut v3: (u8, i64, u64, u64) = { let mut state = v0.borrow_mut(); let path = state.0.clone(); let Some((_, mut child, job)) = state.1.pop() else { return (6, 0, 0, 0) }; let stop = |child: &mut std::process::Child| { #[cfg(windows)] { unsafe extern "system" { fn TerminateJobObject(job: *mut std::ffi::c_void, code: u32) -> i32; } if job != 0 { unsafe { TerminateJobObject(job as *mut std::ffi::c_void, 1); } } } #[cfg(unix)] { let _ = job; let _ = std::process::Command::new("kill").arg("-KILL").arg("--").arg(format!("-{}", child.id())).status(); } let _ = child.kill(); let _ = child.wait(); }; let pattern = if (&*v1).is_empty() { None } else { match regex::Regex::new(&*v1) { Ok(pattern) => Some(pattern), Err(_) => { stop(&mut child); return (6, 0, 0, 0); } } }; let started = std::time::Instant::now(); let limit = std::time::Duration::from_millis(v2); let lines = || std::fs::read(&path).map(|bytes| String::from_utf8_lossy(&bytes).lines().count() as u64).unwrap_or(0); loop { if let Some(pattern) = &pattern { let text = std::fs::read(&path).map(|bytes| String::from_utf8_lossy(&bytes).into_owned()).unwrap_or_default(); if text.lines().any(|line| pattern.is_match(line)) { stop(&mut child); return (3, 0, started.elapsed().as_millis() as u64, lines()); } } if let Ok(Some(status)) = child.try_wait() { let code = status.code().map(|code| code as i64).unwrap_or(-1); return (if pattern.is_some() { 5 } else { 1 }, code, started.elapsed().as_millis() as u64, lines()); } if started.elapsed() >= limit { stop(&mut child); return (if pattern.is_some() { 4 } else { 2 }, 0, started.elapsed().as_millis() as u64, lines()); } std::thread::sleep(std::time::Duration::from_millis(50)); } };
    v3.clone()
}
fn duration_text_20(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(if v0 < 10000 { format!("{}ms", v0) } else { format!("{:.1}s", v0 as f64 / 1000.0) });
    v1.clone()
}
fn timeout_text_21(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(if v0 % 1000 == 0 { format!("{}s", v0 / 1000) } else { format!("{}ms", v0) });
    v1.clone()
}
fn run_end_text_19(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: US2) -> Rc<str> {
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("run "); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v0, &*v7, &*v8, &*v8].concat());
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  lines "); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
    let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v11, &*v8, &*v8, &*v8].concat());
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v1, &*v13, &*v8, &*v8].concat());
    match &v5 {
        US2::US2_BadPattern => {
            let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("invalid pattern "); } LIT.with(|lit| lit.clone()) };
            let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  stopped (tree)"); } LIT.with(|lit| lit.clone()) };
            let mut v45: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v43, &*v14, &*v44, &*v8].concat());
            v45.clone()
        }
        US2::US2_Exited(v15) => {
            let mut v15: i64 = *v15;
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit "); } LIT.with(|lit| lit.clone()) };
            let mut v17: Rc<str> = std::rc::Rc::<str>::from((v15).to_string());
            let mut v18: Rc<str> = duration_text_20(v3);
            let mut v19: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v18, &*v8, &*v8, &*v8].concat());
            let mut v20: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v16, &*v17, &*v12, &*v19].concat());
            v20.clone()
        }
        US2::US2_ExitedBeforeMatch(v37) => {
            let mut v37: i64 = *v37;
            let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exited "); } LIT.with(|lit| lit.clone()) };
            let mut v39: Rc<str> = std::rc::Rc::<str>::from((v37).to_string());
            let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" before matching "); } LIT.with(|lit| lit.clone()) };
            let mut v41: Rc<str> = std::rc::Rc::<str>::from([&*v40, &*v14, &*v8, &*v8, &*v8].concat());
            let mut v42: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v38, &*v39, &*v41, &*v12].concat());
            v42.clone()
        }
        US2::US2_Matched => {
            let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("matched "); } LIT.with(|lit| lit.clone()) };
            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" after "); } LIT.with(|lit| lit.clone()) };
            let mut v27: Rc<str> = duration_text_20(v3);
            let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  stopped (tree)  exit killed"); } LIT.with(|lit| lit.clone()) };
            let mut v29: Rc<str> = std::rc::Rc::<str>::from([&*v27, &*v28, &*v8, &*v8, &*v8].concat());
            let mut v30: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v25, &*v14, &*v26, &*v29].concat());
            v30.clone()
        }
        US2::US2_NoMatch => {
            let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no match for "); } LIT.with(|lit| lit.clone()) };
            let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" in "); } LIT.with(|lit| lit.clone()) };
            let mut v33: Rc<str> = timeout_text_21(v2);
            let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  stopped (tree)"); } LIT.with(|lit| lit.clone()) };
            let mut v35: Rc<str> = std::rc::Rc::<str>::from([&*v32, &*v33, &*v34, &*v8, &*v8].concat());
            let mut v36: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v31, &*v14, &*v35, &*v12].concat());
            v36.clone()
        }
        US2::US2_TimedOut => {
            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed out after "); } LIT.with(|lit| lit.clone()) };
            let mut v22: Rc<str> = timeout_text_21(v2);
            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  stopped (tree)"); } LIT.with(|lit| lit.clone()) };
            let mut v24: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v21, &*v22, &*v23, &*v12].concat());
            v24.clone()
        }
    }
}
fn later_reads_output_22(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> bool {
    let mut v2: bool = v0.iter().skip(v1 as usize + 1).any(|fields| fields.get(1).map(|verb| [ "tail", "search", "errors" ].contains(&&**verb)).unwrap_or(false));
    v2
}
fn output_lines_23(mut v0: std::rc::Rc<std::cell::RefCell<(String, Vec<(String, std::process::Child, u64)>)>>, mut v1: u64, mut v2: bool) -> Rc<str> {
    let mut v3: Rc<str> = { let text = std::fs::read(&v0.borrow().0).map(|bytes| String::from_utf8_lossy(&bytes).into_owned()).unwrap_or_default(); let lines = text.lines().collect::<Vec<_>>(); let from = lines.len().saturating_sub(v1 as usize); let mut shown = Vec::new(); if v2 { shown.push(format!("output (last {} lines)", v1)); } else if from > 0 { shown.push(format!("... {} earlier lines", from)); } shown.extend(lines[from..].iter().map(|line| line.to_string())); std::rc::Rc::<str>::from(shown.join("
")) };
    v3.clone()
}
fn lines_joined_24(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v2 {
        v0.clone()
    } else {
        let mut v3: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v3 {
            v1.clone()
        } else {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v4, &*v1, &*v5, &*v5].concat());
            v6.clone()
        }
    }
}
fn reported_25(mut v0: Rc<str>, mut v1: bool, mut v2: u64, mut v3: bool, mut v4: i64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: Rc<str>, mut v14: bool) -> (Rc<str>, bool, u64, bool, i64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let mut v15: Rc<str> = lines_joined_24(v0.clone(), v13.clone());
    let mut v16: bool = v1 || v14;
    (v15.clone(), v16, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12)
}
fn foreground_run_17(mut v0: std::rc::Rc<std::cell::RefCell<(String, Vec<(String, std::process::Child, u64)>)>>, mut v1: Rc<str>, mut v2: bool, mut v3: u64, mut v4: bool, mut v5: i64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v15: i32, mut v16: Rc<str>, mut v17: Rc<str>, mut v18: u64) -> (Rc<str>, bool, u64, bool, i64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let mut v19: (u8, i64, u64, u64) = run_ended_18(v0.clone(), v17.clone(), v18);
    let mut v20: u8 = v19.clone().0;
    let mut v21: i64 = v19.clone().1;
    let mut v22: bool = v20 == 1u8;
    let mut v37: US2 = if v22 {
        US2::US2_Exited(v21)
    } else {
        let mut v24: bool = v20 == 2u8;
        if v24 {
            US2::US2_TimedOut
        } else {
            let mut v26: bool = v20 == 3u8;
            if v26 {
                US2::US2_Matched
            } else {
                let mut v28: bool = v20 == 4u8;
                if v28 {
                    US2::US2_NoMatch
                } else {
                    let mut v30: bool = v20 == 5u8;
                    if v30 {
                        US2::US2_ExitedBeforeMatch(v21)
                    } else {
                        US2::US2_BadPattern
                    }
                }
            }
        }
    };
    let mut v38: u64 = v19.clone().2;
    let mut v39: u64 = v19.3;
    let mut v40: Rc<str> = run_end_text_19(v16.clone(), v17.clone(), v18, v38, v39, v37.clone());
    let mut v48: Rc<str> = match &v37 {
        US2::US2_Exited(v41) => {
            let mut v41: i64 = *v41;
            let mut v42: bool = later_reads_output_22(v14.clone(), v15);
            if v42 {
                v40.clone()
            } else {
                let mut v43: u64 = 40u64;
                let mut v44: bool = false;
                let mut v45: Rc<str> = output_lines_23(v0.clone(), v43, v44);
                lines_joined_24(v40.clone(), v45.clone())
            }
        }
        _ => {
            v40.clone()
        }
    };
    let mut v53: bool = match &v37 {
        US2::US2_Exited(v49) => {
            let mut v49: i64 = *v49;
            let mut v50: bool = v49 == 0i64;
            let mut v51: bool = v50 != true;
            v51
        }
        US2::US2_Matched => {
            false
        }
        _ => {
            true
        }
    };
    let (mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64, mut v65, mut v66): (Rc<str>, bool, u64, bool, i64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_25(v1.clone(), v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v48.clone(), v53);
    let mut v67: u64 = v3.wrapping_add(1u64);
    let (mut v69, mut v70, mut v71, mut v72, mut v73, mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81): (Rc<str>, bool, u64, bool, i64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v37 {
        US2::US2_Exited(v68) => {
            let mut v68: i64 = *v68;
            (v54.clone(), v55, v67, true, v68, v59, v60, v61, v62, v63, v64, v65, v66)
        }
        _ => {
            (v54.clone(), v55, v67, v57, v58, v59, v60, v61, v62, v63, v64, v65, v66)
        }
    };
    (v69.clone(), v70, v71, v72, v73, v74, v75, v76, v77, v78, v79, v80, v81)
}
fn background_pid_26(mut v0: std::rc::Rc<std::cell::RefCell<(String, Vec<(String, std::process::Child, u64)>)>>) -> u64 {
    let mut v1: u64 = v0.borrow().1.last().map(|entry| entry.1.id() as u64).unwrap_or(0);
    v1
}
fn decimal_text_27(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1.clone()
}
fn output_searched_28(mut v0: std::rc::Rc<std::cell::RefCell<(String, Vec<(String, std::process::Child, u64)>)>>, mut v1: Rc<str>) -> std::rc::Rc<(std::rc::Rc<str>, u64, bool)> {
    let mut v2: std::rc::Rc<(std::rc::Rc<str>, u64, bool)> = { let text = std::fs::read(&v0.borrow().0).map(|bytes| String::from_utf8_lossy(&bytes).into_owned()).unwrap_or_default(); match regex::Regex::new(&*v1) { Err(error) => std::rc::Rc::new((std::rc::Rc::<str>::from(format!("search: invalid pattern: {error}")), 0, true)), Ok(pattern) => { let hits = text.lines().enumerate().filter(|(_, line)| pattern.is_match(line)).map(|(index, line)| format!("output:{}  {}", index + 1, line)).collect::<Vec<_>>(); std::rc::Rc::new((std::rc::Rc::<str>::from(hits.join("
")), hits.len() as u64, false)) } } };
    v2.clone()
}
fn errors_grouped_29(mut v0: std::rc::Rc<std::cell::RefCell<(String, Vec<(String, std::process::Child, u64)>)>>, mut v1: Rc<str>) -> std::rc::Rc<(std::rc::Rc<str>, u64, bool)> {
    let mut v2: std::rc::Rc<(std::rc::Rc<str>, u64, bool)> = { let text = std::fs::read(&v0.borrow().0).map(|bytes| String::from_utf8_lossy(&bytes).into_owned()).unwrap_or_default(); match regex::Regex::new(&*v1) { Err(error) => std::rc::Rc::new((std::rc::Rc::<str>::from(format!("errors: invalid pattern: {error}")), 0, true)), Ok(pattern) => { let mut groups: Vec<(String, u64, String)> = Vec::new(); let mut total = 0u64; for line in text.lines() { if let Some(captures) = pattern.captures(line) { total += 1; let key = captures.get(1).or_else(|| captures.get(0)).map(|found| found.as_str().to_owned()).unwrap_or_default(); match groups.iter_mut().find(|group| group.0 == key) { Some(group) => group.1 += 1, None => groups.push((key, 1, line.trim().to_owned())) } } } groups.sort_by(|left, right| right.1.cmp(&left.1)); let width = groups.iter().map(|group| group.0.len()).max().unwrap_or(0).max(4); let count_width = groups.iter().map(|group| group.1.to_string().len()).max().unwrap_or(0).max(5); let mut shown = vec![format!("{:<width$}  {:<count_width$}  {}", "code", "count", "first")]; for group in &groups { shown.push(format!("{:<width$}  {:<count_width$}  {}", group.0, group.1, group.2)); } std::rc::Rc::new((std::rc::Rc::<str>::from(shown.join("
")), total, false)) } } };
    v2.clone()
}
fn local_offset_minutes_30() -> i64 {
    let mut v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn processes_listed_31(mut v0: Rc<str>, mut v1: i64) -> std::rc::Rc<(std::rc::Rc<str>, u64, bool)> {
    let mut v2: std::rc::Rc<(std::rc::Rc<str>, u64, bool)> = { match regex::RegexBuilder::new(&*v0).case_insensitive(true).build() { Err(error) => std::rc::Rc::new((std::rc::Rc::<str>::from(format!("processes: invalid pattern: {error}")), 0, true)), Ok(pattern) => { let system = sysinfo::System::new_with_specifics(sysinfo::RefreshKind::nothing().with_processes(sysinfo::ProcessRefreshKind::nothing().with_cmd(sysinfo::UpdateKind::Always))); let own = std::process::id(); let offset = v1 * 60; let mut rows = system.processes().iter().filter(|(pid, _)| pid.as_u32() != own).filter_map(|(pid, process)| { let command = process.cmd().iter().map(|part| part.to_string_lossy().into_owned()).collect::<Vec<_>>().join(" "); if pattern.is_match(&command) { Some((process.start_time(), pid.as_u32(), command)) } else { None } }).collect::<Vec<_>>(); rows.sort(); let pid_width = rows.iter().map(|row| row.1.to_string().len()).max().unwrap_or(0).max(7); let mut shown = vec![format!("{:<pid_width$}  {:<7}  {}", "pid", "started", "command")]; for (started, pid, command) in &rows { let local = (*started as i64 + offset).rem_euclid(86400); shown.push(format!("{:<pid_width$}  {:<7}  {}", pid, format!("{:02}:{:02}", local / 3600, local / 60 % 60), command)); } std::rc::Rc::new((std::rc::Rc::<str>::from(shown.join("
")), rows.len() as u64, false)) } } };
    v2.clone()
}
fn run_stopped_32(mut v0: std::rc::Rc<std::cell::RefCell<(String, Vec<(String, std::process::Child, u64)>)>>, mut v1: Rc<str>) -> bool {
    let mut v2: bool = { let mut state = v0.borrow_mut(); match state.1.iter().position(|entry| entry.0 == &*v1) { None => false, Some(index) => { let (_, mut child, job) = state.1.remove(index); #[cfg(windows)] { unsafe extern "system" { fn TerminateJobObject(job: *mut std::ffi::c_void, code: u32) -> i32; } if job != 0 { unsafe { TerminateJobObject(job as *mut std::ffi::c_void, 1); } } } #[cfg(unix)] { let _ = job; let _ = std::process::Command::new("kill").arg("-KILL").arg("--").arg(format!("-{}", child.id())).status(); } let _ = child.kill(); let _ = child.wait(); true } } };
    v2
}
fn step_run_15(mut v0: Rc<str>, mut v1: std::rc::Rc<std::cell::RefCell<(String, Vec<(String, std::process::Child, u64)>)>>, mut v2: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v3: i32, mut v4: Rc<str>, mut v5: bool, mut v6: u64, mut v7: bool, mut v8: i64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: u64, mut v16: u64, mut v17: US1) -> (Rc<str>, bool, u64, bool, i64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let (mut v346, mut v347, mut v348, mut v349, mut v350, mut v351, mut v352, mut v353, mut v354, mut v355, mut v356, mut v357, mut v358): (Rc<str>, bool, u64, bool, i64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v17 {
        US1::US1_RunForeground(v18, v19) => {
            let mut v18: Rc<str> = v18.clone();
            let mut v19: u64 = *v19;
            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v21: Rc<str> = run_spawned_16(v1.clone(), v0.clone(), v18.clone(), v20.clone());
            let mut v22: bool = v21.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v22 {
                let (mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35): (Rc<str>, bool, u64, bool, i64, u64, u64, u64, u64, u64, u64, u64, u64) = foreground_run_17(v1.clone(), v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v2.clone(), v3, v18.clone(), v20.clone(), v19);
                (v23.clone(), v24, v25, v26, v27, v28, v29, v30, v31, v32, v33, v34, v35)
            } else {
                let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("run "); } LIT.with(|lit| lit.clone()) };
                let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                let mut v38: Rc<str> = std::rc::Rc::<str>::from([&*v36, &*v18, &*v37, &*v21, &*v20].concat());
                let mut v39: bool = true;
                let (mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52): (Rc<str>, bool, u64, bool, i64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_25(v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v38.clone(), v39);
                let mut v53: u64 = v6.wrapping_add(1u64);
                (v40.clone(), v41, v53, v43, v44, v45, v46, v47, v48, v49, v50, v51, v52)
            }
        }
        _ => {
            let (mut v333, mut v334, mut v335, mut v336, mut v337, mut v338, mut v339, mut v340, mut v341, mut v342, mut v343, mut v344, mut v345): (Rc<str>, bool, u64, bool, i64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v17 {
                US1::US1_RunUntil(v67, v68, v69) => {
                    let mut v67: Rc<str> = v67.clone();
                    let mut v68: Rc<str> = v68.clone();
                    let mut v69: u64 = *v69;
                    let mut v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v71: Rc<str> = run_spawned_16(v1.clone(), v0.clone(), v67.clone(), v70.clone());
                    let mut v72: bool = v71.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    if v72 {
                        let (mut v73, mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83, mut v84, mut v85): (Rc<str>, bool, u64, bool, i64, u64, u64, u64, u64, u64, u64, u64, u64) = foreground_run_17(v1.clone(), v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v2.clone(), v3, v67.clone(), v68.clone(), v69);
                        (v73.clone(), v74, v75, v76, v77, v78, v79, v80, v81, v82, v83, v84, v85)
                    } else {
                        let mut v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("run "); } LIT.with(|lit| lit.clone()) };
                        let mut v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                        let mut v88: Rc<str> = std::rc::Rc::<str>::from([&*v86, &*v67, &*v87, &*v71, &*v70].concat());
                        let mut v89: bool = true;
                        let (mut v90, mut v91, mut v92, mut v93, mut v94, mut v95, mut v96, mut v97, mut v98, mut v99, mut v100, mut v101, mut v102): (Rc<str>, bool, u64, bool, i64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_25(v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v88.clone(), v89);
                        let mut v103: u64 = v6.wrapping_add(1u64);
                        (v90.clone(), v91, v103, v93, v94, v95, v96, v97, v98, v99, v100, v101, v102)
                    }
                }
                _ => {
                    let (mut v320, mut v321, mut v322, mut v323, mut v324, mut v325, mut v326, mut v327, mut v328, mut v329, mut v330, mut v331, mut v332): (Rc<str>, bool, u64, bool, i64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v17 {
                        US1::US1_RunBackground(v117, v118) => {
                            let mut v117: Rc<str> = v117.clone();
                            let mut v118: Rc<str> = v118.clone();
                            let mut v119: Rc<str> = run_spawned_16(v1.clone(), v0.clone(), v117.clone(), v118.clone());
                            let mut v120: bool = v119.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let mut v133: Rc<str> = if v120 {
                                let mut v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("run "); } LIT.with(|lit| lit.clone()) };
                                let mut v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  background as "); } LIT.with(|lit| lit.clone()) };
                                let mut v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  pid "); } LIT.with(|lit| lit.clone()) };
                                let mut v124: u64 = background_pid_26(v1.clone());
                                let mut v125: Rc<str> = decimal_text_27(v124);
                                let mut v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v127: Rc<str> = std::rc::Rc::<str>::from([&*v123, &*v125, &*v126, &*v126, &*v126].concat());
                                let mut v128: Rc<str> = std::rc::Rc::<str>::from([&*v121, &*v117, &*v122, &*v118, &*v127].concat());
                                v128.clone()
                            } else {
                                let mut v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("run "); } LIT.with(|lit| lit.clone()) };
                                let mut v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                                let mut v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v132: Rc<str> = std::rc::Rc::<str>::from([&*v129, &*v117, &*v130, &*v119, &*v131].concat());
                                v132.clone()
                            };
                            let mut v134: bool = v120 != true;
                            let (mut v135, mut v136, mut v137, mut v138, mut v139, mut v140, mut v141, mut v142, mut v143, mut v144, mut v145, mut v146, mut v147): (Rc<str>, bool, u64, bool, i64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_25(v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v133.clone(), v134);
                            let mut v148: u64 = v6.wrapping_add(1u64);
                            (v135.clone(), v136, v148, v138, v139, v140, v141, v142, v143, v144, v145, v146, v147)
                        }
                        _ => {
                            let (mut v307, mut v308, mut v309, mut v310, mut v311, mut v312, mut v313, mut v314, mut v315, mut v316, mut v317, mut v318, mut v319): (Rc<str>, bool, u64, bool, i64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v17 {
                                US1::US1_TailOutput(v149) => {
                                    let mut v149: u64 = *v149;
                                    let mut v150: bool = true;
                                    let mut v151: Rc<str> = output_lines_23(v1.clone(), v149, v150);
                                    let mut v152: bool = false;
                                    reported_25(v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v151.clone(), v152)
                                }
                                _ => {
                                    let (mut v294, mut v295, mut v296, mut v297, mut v298, mut v299, mut v300, mut v301, mut v302, mut v303, mut v304, mut v305, mut v306): (Rc<str>, bool, u64, bool, i64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v17 {
                                        US1::US1_SearchOutput(v166) => {
                                            let mut v166: Rc<str> = v166.clone();
                                            let mut v167: std::rc::Rc<(std::rc::Rc<str>, u64, bool)> = output_searched_28(v1.clone(), v166.clone());
                                            let mut v168: Rc<str> = v167.0.clone();
                                            let mut v169: bool = v167.clone().2;
                                            let (mut v170, mut v171, mut v172, mut v173, mut v174, mut v175, mut v176, mut v177, mut v178, mut v179, mut v180, mut v181, mut v182): (Rc<str>, bool, u64, bool, i64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_25(v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v168.clone(), v169);
                                            let mut v183: u64 = v167.1;
                                            let mut v184: u64 = v15.wrapping_add(v183);
                                            let mut v185: u64 = v16.wrapping_add(1u64);
                                            (v170.clone(), v171, v172, v173, v174, v175, v176, v177, v178, v179, v180, v184, v185)
                                        }
                                        _ => {
                                            let (mut v281, mut v282, mut v283, mut v284, mut v285, mut v286, mut v287, mut v288, mut v289, mut v290, mut v291, mut v292, mut v293): (Rc<str>, bool, u64, bool, i64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v17 {
                                                US1::US1_ErrorsBy(v186) => {
                                                    let mut v186: Rc<str> = v186.clone();
                                                    let mut v187: std::rc::Rc<(std::rc::Rc<str>, u64, bool)> = errors_grouped_29(v1.clone(), v186.clone());
                                                    let mut v188: Rc<str> = v187.0.clone();
                                                    let mut v189: bool = v187.clone().2;
                                                    let (mut v190, mut v191, mut v192, mut v193, mut v194, mut v195, mut v196, mut v197, mut v198, mut v199, mut v200, mut v201, mut v202): (Rc<str>, bool, u64, bool, i64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_25(v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v188.clone(), v189);
                                                    let mut v203: u64 = v187.1;
                                                    let mut v204: u64 = v13.wrapping_add(v203);
                                                    let mut v205: u64 = v14.wrapping_add(1u64);
                                                    (v190.clone(), v191, v192, v193, v194, v195, v196, v197, v198, v204, v205, v201, v202)
                                                }
                                                _ => {
                                                    let (mut v268, mut v269, mut v270, mut v271, mut v272, mut v273, mut v274, mut v275, mut v276, mut v277, mut v278, mut v279, mut v280): (Rc<str>, bool, u64, bool, i64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v17 {
                                                        US1::US1_ProcessesMatching(v206) => {
                                                            let mut v206: Rc<str> = v206.clone();
                                                            let mut v207: i64 = local_offset_minutes_30();
                                                            let mut v208: std::rc::Rc<(std::rc::Rc<str>, u64, bool)> = processes_listed_31(v206.clone(), v207);
                                                            let mut v209: Rc<str> = v208.0.clone();
                                                            let mut v210: bool = v208.clone().2;
                                                            let (mut v211, mut v212, mut v213, mut v214, mut v215, mut v216, mut v217, mut v218, mut v219, mut v220, mut v221, mut v222, mut v223): (Rc<str>, bool, u64, bool, i64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_25(v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v209.clone(), v210);
                                                            let mut v224: u64 = v208.1;
                                                            let mut v225: u64 = v9.wrapping_add(v224);
                                                            let mut v226: u64 = v10.wrapping_add(1u64);
                                                            (v211.clone(), v212, v213, v214, v215, v225, v226, v218, v219, v220, v221, v222, v223)
                                                        }
                                                        _ => {
                                                            let (mut v255, mut v256, mut v257, mut v258, mut v259, mut v260, mut v261, mut v262, mut v263, mut v264, mut v265, mut v266, mut v267): (Rc<str>, bool, u64, bool, i64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v17 {
                                                                US1::US1_StopNamed(v227) => {
                                                                    let mut v227: Rc<str> = v227.clone();
                                                                    let mut v228: bool = run_stopped_32(v1.clone(), v227.clone());
                                                                    let mut v237: Rc<str> = if v228 {
                                                                        let mut v229: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop "); } LIT.with(|lit| lit.clone()) };
                                                                        let mut v230: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  stopped (tree)"); } LIT.with(|lit| lit.clone()) };
                                                                        let mut v231: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                                        let mut v232: Rc<str> = std::rc::Rc::<str>::from([&*v229, &*v227, &*v230, &*v231, &*v231].concat());
                                                                        v232.clone()
                                                                    } else {
                                                                        let mut v233: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop "); } LIT.with(|lit| lit.clone()) };
                                                                        let mut v234: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  no run by that name"); } LIT.with(|lit| lit.clone()) };
                                                                        let mut v235: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                                        let mut v236: Rc<str> = std::rc::Rc::<str>::from([&*v233, &*v227, &*v234, &*v235, &*v235].concat());
                                                                        v236.clone()
                                                                    };
                                                                    let mut v238: bool = v228 == false;
                                                                    let (mut v239, mut v240, mut v241, mut v242, mut v243, mut v244, mut v245, mut v246, mut v247, mut v248, mut v249, mut v250, mut v251): (Rc<str>, bool, u64, bool, i64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_25(v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v237.clone(), v238);
                                                                    let mut v252: u64 = if v228 {
                                                                        1u64
                                                                    } else {
                                                                        0u64
                                                                    };
                                                                    let mut v253: u64 = v11.wrapping_add(v252);
                                                                    let mut v254: u64 = v12.wrapping_add(1u64);
                                                                    (v239.clone(), v240, v241, v242, v243, v244, v245, v253, v254, v248, v249, v250, v251)
                                                                }
                                                                _ => unreachable!(),
                                                            };
                                                            (v255.clone(), v256, v257, v258, v259, v260, v261, v262, v263, v264, v265, v266, v267)
                                                        }
                                                    };
                                                    (v268.clone(), v269, v270, v271, v272, v273, v274, v275, v276, v277, v278, v279, v280)
                                                }
                                            };
                                            (v281.clone(), v282, v283, v284, v285, v286, v287, v288, v289, v290, v291, v292, v293)
                                        }
                                    };
                                    (v294.clone(), v295, v296, v297, v298, v299, v300, v301, v302, v303, v304, v305, v306)
                                }
                            };
                            (v307.clone(), v308, v309, v310, v311, v312, v313, v314, v315, v316, v317, v318, v319)
                        }
                    };
                    (v320.clone(), v321, v322, v323, v324, v325, v326, v327, v328, v329, v330, v331, v332)
                }
            };
            (v333.clone(), v334, v335, v336, v337, v338, v339, v340, v341, v342, v343, v344, v345)
        }
    };
    (v346.clone(), v347, v348, v349, v350, v351, v352, v353, v354, v355, v356, v357, v358)
}
fn steps_run_14(mut v0: Rc<str>, mut v1: std::rc::Rc<std::cell::RefCell<(String, Vec<(String, std::process::Child, u64)>)>>, mut v2: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v3: i32, mut v4: Rc<str>, mut v5: bool, mut v6: u64, mut v7: bool, mut v8: i64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: u64, mut v16: u64) -> (Rc<str>, bool, u64, bool, i64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let mut v17: i32 = v2.len() as i32;
    let mut v18: bool = v3 < v17;
    let (mut v89, mut v90, mut v91, mut v92, mut v93, mut v94, mut v95, mut v96, mut v97, mut v98, mut v99, mut v100, mut v101): (Rc<str>, bool, u64, bool, i64, u64, u64, u64, u64, u64, u64, u64, u64) = if v18 {
        let mut v19: US0 = step_parsed_3(v2.clone(), v3);
        let (mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83, mut v84, mut v85, mut v86, mut v87, mut v88): (Rc<str>, bool, u64, bool, i64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v19 {
            US0::US0_StepParsed(v20) => {
                let mut v20: US1 = v20.clone();
                let mut v21: i32 = v3.wrapping_add(1i32);
                let (mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33, mut v34): (Rc<str>, bool, u64, bool, i64, u64, u64, u64, u64, u64, u64, u64, u64) = step_run_15(v0.clone(), v1.clone(), v2.clone(), v3, v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v20.clone());
                steps_run_14(v0.clone(), v1.clone(), v2.clone(), v21, v22.clone(), v23, v24, v25, v26, v27, v28, v29, v30, v31, v32, v33, v34)
            }
            _ => {
                let (mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73, mut v74, mut v75): (Rc<str>, bool, u64, bool, i64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v19 {
                    US0::US0_StepRejected(v48) => {
                        let mut v48: Rc<str> = v48.clone();
                        let mut v49: i32 = v3.wrapping_add(1i32);
                        steps_run_14(v0.clone(), v1.clone(), v2.clone(), v49, v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16)
                    }
                    _ => unreachable!(),
                };
                (v63.clone(), v64, v65, v66, v67, v68, v69, v70, v71, v72, v73, v74, v75)
            }
        };
        (v76.clone(), v77, v78, v79, v80, v81, v82, v83, v84, v85, v86, v87, v88)
    } else {
        (v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16)
    };
    (v89.clone(), v90, v91, v92, v93, v94, v95, v96, v97, v98, v99, v100, v101)
}
fn segments_joined_35(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v2 {
        v1.clone()
    } else {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v3, &*v1, &*v4, &*v4].concat());
        v5.clone()
    }
}
fn counted_segment_34(mut v0: bool, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64) -> Rc<str> {
    if v0 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = decimal_text_27(v3);
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v4, &*v5, &*v6, &*v6].concat());
        segments_joined_35(v1.clone(), v7.clone())
    } else {
        v1.clone()
    }
}
fn receipt_counts_33(mut v0: Rc<str>, mut v1: bool, mut v2: u64, mut v3: bool, mut v4: i64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64) -> Rc<str> {
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("runs "); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = decimal_text_27(v2);
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v14, &*v15, &*v15, &*v15].concat());
    let mut v20: Rc<str> = if v3 {
        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  exit "); } LIT.with(|lit| lit.clone()) };
        let mut v18: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
        let mut v19: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v17, &*v18, &*v15, &*v15].concat());
        v19.clone()
    } else {
        v16.clone()
    };
    let mut v21: bool = v6 > 0u64;
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("processes"); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = counted_segment_34(v21, v20.clone(), v22.clone(), v5);
    let mut v24: bool = v8 > 0u64;
    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped"); } LIT.with(|lit| lit.clone()) };
    let mut v26: Rc<str> = counted_segment_34(v24, v23.clone(), v25.clone(), v7);
    let mut v27: bool = v10 > 0u64;
    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("errors"); } LIT.with(|lit| lit.clone()) };
    let mut v29: Rc<str> = counted_segment_34(v27, v26.clone(), v28.clone(), v9);
    let mut v30: bool = v12 > 0u64;
    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("matches"); } LIT.with(|lit| lit.clone()) };
    counted_segment_34(v30, v29.clone(), v31.clone(), v11)
}
fn after_edits_run_11(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: std::rc::Rc<std::cell::RefCell<(String, Vec<(String, std::process::Child, u64)>)>> = state_new_12();
    let mut v3: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = records_of_0(v1.clone());
    let mut v4: i32 = 1i32;
    let (mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16, mut v17): (Rc<str>, bool, u64, bool, i64, u64, u64, u64, u64, u64, u64, u64, u64) = empty_run_tally_13();
    let (mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30): (Rc<str>, bool, u64, bool, i64, u64, u64, u64, u64, u64, u64, u64, u64) = steps_run_14(v0.clone(), v2.clone(), v3.clone(), v4, v5.clone(), v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17);
    let mut v31: Rc<str> = std::rc::Rc::<str>::from(char::from(30u8).to_string());
    let mut v32: Rc<str> = receipt_counts_33(v18.clone(), v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30);
    let mut v33: Rc<str> = std::rc::Rc::<str>::from(char::from(30u8).to_string());
    let mut v36: Rc<str> = if v19 {
        let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
        v34.clone()
    } else {
        let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        v35.clone()
    };
    let mut v37: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v31, &*v32, &*v33, &*v36].concat());
    v37.clone()
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        after_edits_run_11(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn now_ms_37() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn first_rejection_38(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> Rc<str> {
    loop {
        let mut v2: i32 = v0.len() as i32;
        let mut v3: bool = v1 < v2;
        if v3 {
            let mut v4: i32 = 0i32;
            let mut v5: Rc<str> = field_at_2(v0.clone(), v1, v4);
            let mut v6: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("query"); } LIT.with(|lit| lit.clone()) };
            let mut v7: bool = v6 != true;
            if v7 {
                let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit and run statements cannot share one program yet; run them as two calls"); } LIT.with(|lit| lit.clone()) };
                return v8.clone();
            } else {
                let mut v9: US0 = step_parsed_3(v0.clone(), v1);
                match &v9 {
                    US0::US0_StepParsed(v11) => {
                        let mut v11: US1 = v11.clone();
                        let mut v12: i32 = v1.wrapping_add(1i32);
                        (v0, v1) = (v0.clone(), v12);
                        continue;
                    }
                    US0::US0_StepRejected(v10) => {
                        let mut v10: Rc<str> = v10.clone();
                        return v10.clone();
                    }
                }
            }
        } else {
            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v17.clone();
        }
    }
}
fn elapsed_text_40(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1.clone()
}
fn clock_text_41() -> Rc<str> {
    let mut v0: i64 = local_offset_minutes_30();
    let mut v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1.clone()
}
fn receipt_line_39(mut v0: US3, mut v1: Rc<str>, mut v2: u64) -> () {
    let mut v6: Rc<str> = match &v0 {
        US3::US3_Failed => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US3::US3_Succeeded => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
    };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = elapsed_text_40(v2);
    let mut v9: Rc<str> = clock_text_41();
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v7, &*v9, &*v10, &*v10].concat());
    let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v7, &*v1, &*v7, &*v11].concat());
    println!("{}", &*v12);
    ()
}
fn run_program_run_36(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: u64 = now_ms_37();
    let mut v3: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = records_of_0(v1.clone());
    let mut v4: i32 = 1i32;
    let mut v5: Rc<str> = first_rejection_38(v3.clone(), v4);
    let mut v6: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v6 {
        let mut v7: std::rc::Rc<std::cell::RefCell<(String, Vec<(String, std::process::Child, u64)>)>> = state_new_12();
        let mut v8: i32 = 1i32;
        let (mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21): (Rc<str>, bool, u64, bool, i64, u64, u64, u64, u64, u64, u64, u64, u64) = empty_run_tally_13();
        let (mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33, mut v34): (Rc<str>, bool, u64, bool, i64, u64, u64, u64, u64, u64, u64, u64, u64) = steps_run_14(v0.clone(), v7.clone(), v3.clone(), v8, v9.clone(), v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21);
        let mut v35: bool = v22.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v35 {
            ()
        } else {
            println!("{}", &*v22);
            ()
        };
        let mut v36: Rc<str> = receipt_counts_33(v22.clone(), v23, v24, v25, v26, v27, v28, v29, v30, v31, v32, v33, v34);
        if v23 {
            let mut v37: US3 = US3::US3_Failed;
            receipt_line_39(v37.clone(), v36.clone(), v2);
            1i32
        } else {
            let mut v38: US3 = US3::US3_Succeeded;
            receipt_line_39(v38.clone(), v36.clone(), v2);
            0i32
        }
    } else {
        let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("program rejected: "); } LIT.with(|lit| lit.clone()) };
        let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v42: Rc<str> = std::rc::Rc::<str>::from([&*v40, &*v5, &*v41, &*v41, &*v41].concat());
        println!("{}", &*v42);
        let mut v43: US3 = US3::US3_Failed;
        let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("runs 0"); } LIT.with(|lit| lit.clone()) };
        receipt_line_39(v43.clone(), v44.clone(), v2);
        2i32
    }
}
fn closure2() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        run_program_run_36(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_do_process_rejection(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
pub fn eoie_do_process_after_edits(v0: &str, v1: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn eoie_do_process_run(v0: &str, v1: &str) -> i32 {
    closure2()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
