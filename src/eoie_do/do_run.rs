#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Edits,
    US0_Reads,
    US0_Facts,
    US0_Runs,
    US0_Writes,
    US0_Git,
    US0_Jobs,
    US0_Labs,
}
#[derive(Clone)]
enum US1 {
    US1_CompileRejected(Rc<str>),
    US1_Delegated(US0, Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_Some(US0),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_Succeeded,
    US3_Failed,
}
#[derive(Clone)]
enum US4 {
    US4_Some(Rc<str>),
    US4_None,
}
fn delegated_run_2(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>) -> i32 {
    match &v0 {
        US0::US0_Edits => {
            let mut v3: i32 = eoie_do_edit::eoie_do_edit_run(&*v1, &*v2);
            v3
        }
        US0::US0_Facts => {
            let mut v5: i32 = eoie_do_facts::eoie_do_facts_run(&*v1, &*v2);
            v5
        }
        US0::US0_Git => {
            let mut v8: i32 = eoie_do_git::eoie_do_git_run(&*v1, &*v2);
            v8
        }
        US0::US0_Jobs => {
            let mut v9: i32 = eoie_do_jobs::eoie_do_jobs_run(&*v1, &*v2);
            v9
        }
        US0::US0_Labs => {
            let mut v10: i32 = eoie_do_lab::eoie_do_lab_run(&*v1, &*v2);
            v10
        }
        US0::US0_Reads => {
            let mut v4: i32 = eoie_do_read::eoie_do_read_run(&*v1, &*v2);
            v4
        }
        US0::US0_Runs => {
            let mut v6: i32 = eoie_do_process::eoie_do_process_run(&*v1, &*v2);
            v6
        }
        US0::US0_Writes => {
            let mut v7: i32 = eoie_do_write::eoie_do_write_run(&*v1, &*v2);
            v7
        }
    }
}
fn now_ms_3() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn compiled_text_5(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = eoie_do_program::eoie_do_program_compiled(&*v0);
    v1.clone()
}
fn records_of_6(mut v0: Rc<str>) -> std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> {
    let mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new((&*v0).split(char::from(30u8)).map(|record| record.split(char::from(31u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
    v1.clone()
}
fn field_at_7(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let mut v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).cloned().unwrap_or_default();
    v3.clone()
}
fn program_family_named_9(mut v0: Rc<str>) -> US2 {
    let mut v1: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edits"); } LIT.with(|lit| lit.clone()) };
    if v1 {
        let mut v2: US0 = US0::US0_Edits;
        US2::US2_Some(v2.clone())
    } else {
        let mut v4: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("reads"); } LIT.with(|lit| lit.clone()) };
        if v4 {
            let mut v5: US0 = US0::US0_Reads;
            US2::US2_Some(v5.clone())
        } else {
            let mut v7: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("facts"); } LIT.with(|lit| lit.clone()) };
            if v7 {
                let mut v8: US0 = US0::US0_Facts;
                US2::US2_Some(v8.clone())
            } else {
                let mut v10: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("runs"); } LIT.with(|lit| lit.clone()) };
                if v10 {
                    let mut v11: US0 = US0::US0_Runs;
                    US2::US2_Some(v11.clone())
                } else {
                    let mut v13: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("writes"); } LIT.with(|lit| lit.clone()) };
                    if v13 {
                        let mut v14: US0 = US0::US0_Writes;
                        US2::US2_Some(v14.clone())
                    } else {
                        let mut v16: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git"); } LIT.with(|lit| lit.clone()) };
                        if v16 {
                            let mut v17: US0 = US0::US0_Git;
                            US2::US2_Some(v17.clone())
                        } else {
                            let mut v19: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("jobs"); } LIT.with(|lit| lit.clone()) };
                            if v19 {
                                let mut v20: US0 = US0::US0_Jobs;
                                US2::US2_Some(v20.clone())
                            } else {
                                let mut v22: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("labs"); } LIT.with(|lit| lit.clone()) };
                                if v22 {
                                    let mut v23: US0 = US0::US0_Labs;
                                    US2::US2_Some(v23.clone())
                                } else {
                                    US2::US2_None
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn program_family_of_8(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<str> = eoie_do_verbs::eoie_do_program_family(&*v0);
    let mut v2: US2 = program_family_named_9(v1.clone());
    match &v2 {
        US2::US2_None => {
            US0::US0_Edits
        }
        US2::US2_Some(v3) => {
            let mut v3: US0 = v3.clone();
            v3.clone()
        }
    }
}
fn program_decoded_4(mut v0: Rc<str>) -> US1 {
    let mut v1: Rc<str> = compiled_text_5(v0.clone());
    let mut v2: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = records_of_6(v1.clone());
    let mut v3: i32 = 0i32;
    let mut v4: i32 = 0i32;
    let mut v5: Rc<str> = field_at_7(v2.clone(), v3, v4);
    let mut v6: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
    if v6 {
        let mut v7: US0 = program_family_of_8(v1.clone());
        match &v7 {
            US0::US0_Runs => {
                let mut v8: bool = v2.iter().skip(1).any(|fields| fields.first().map(|kind| &**kind == "edit").unwrap_or(false));
                let mut v11: US0 = if v8 {
                    US0::US0_Edits
                } else {
                    US0::US0_Runs
                };
                US1::US1_Delegated(v11.clone(), v1.clone())
            }
            _ => {
                US1::US1_Delegated(v7.clone(), v1.clone())
            }
        }
    } else {
        let mut v15: i32 = 1i32;
        let mut v16: i32 = 0i32;
        let mut v17: Rc<str> = field_at_7(v2.clone(), v15, v16);
        US1::US1_CompileRejected(v17.clone())
    }
}
fn elapsed_text_11(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1.clone()
}
fn local_offset_minutes_13() -> i64 {
    let mut v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn clock_text_12() -> Rc<str> {
    let mut v0: i64 = local_offset_minutes_13();
    let mut v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1.clone()
}
fn receipt_line_10(mut v0: US3, mut v1: Rc<str>, mut v2: u64) -> () {
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
    let mut v8: Rc<str> = elapsed_text_11(v2);
    let mut v9: Rc<str> = clock_text_12();
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v7, &*v9, &*v10, &*v10].concat());
    let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v7, &*v1, &*v7, &*v11].concat());
    println!("{}", &*v12);
    ()
}
fn program_run_1(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: u64 = now_ms_3();
    let mut v3: US1 = program_decoded_4(v1.clone());
    match &v3 {
        US1::US1_CompileRejected(v4) => {
            let mut v4: Rc<str> = v4.clone();
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("program rejected: "); } LIT.with(|lit| lit.clone()) };
            let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v4));
            println!("{}", &*v6);
            let mut v7: US3 = US3::US3_Failed;
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files 0  replacements 0"); } LIT.with(|lit| lit.clone()) };
            receipt_line_10(v7.clone(), v8.clone(), v2);
            2i32
        }
        US1::US1_Delegated(v9, v10) => {
            let mut v9: US0 = v9.clone();
            let mut v10: Rc<str> = v10.clone();
            delegated_run_2(v9.clone(), v0.clone(), v10.clone())
        }
    }
}
fn command_line_argument_count_15() -> i32 {
    let mut v0: i32 = (std::env::args().count() as i32) - 1;
    v0
}
fn argument_count_14() -> i32 {
    let mut v0: i32 = command_line_argument_count_15();
    let mut v1: i32 = v0.wrapping_add(1i32);
    v1
}
fn command_line_argument_17(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::args().nth((v0 + 1) as usize).unwrap_or_default());
    v1.clone()
}
fn argument_16(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_sub(1i32);
    let mut v2: bool = v1 >= 0i32;
    let mut v5: bool = if v2 {
        let mut v3: i32 = command_line_argument_count_15();
        let mut v4: bool = v1 < v3;
        v4
    } else {
        false
    };
    let mut v9: US4 = if v5 {
        let mut v6: Rc<str> = command_line_argument_17(v1);
        US4::US4_Some(v6.clone())
    } else {
        US4::US4_None
    };
    match &v9 {
        US4::US4_None => {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US4::US4_Some(v10) => {
            let mut v10: Rc<str> = v10.clone();
            v10.clone()
        }
    }
}
fn directory_exists_18(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.is_dir();
    v8
}
fn lines_joined_20(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v2 {
        v0.clone()
    } else {
        let mut v3: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v3 {
            v1.clone()
        } else {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v4));
            let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v1));
            v6.clone()
        }
    }
}
fn program_from_19(mut v0: i32, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let mut v3: bool = v0 < v1;
        if v3 {
            let mut v4: i32 = v0.wrapping_add(1i32);
            let mut v5: Rc<str> = argument_16(v0);
            let mut v6: Rc<str> = lines_joined_20(v2.clone(), v5.clone());
            (v0, v1, v2) = (v4, v1, v6.clone());
            continue;
        } else {
            return v2.clone();
        }
    }
}
fn do_run_0() -> i32 {
    let mut v0: i32 = argument_count_14();
    let mut v1: i32 = 2i32;
    let mut v2: Rc<str> = argument_16(v1);
    let mut v3: bool = v0 < 4i32;
    if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: eoie "); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("do <root> <program>... | do <root> - (program on stdin); statements: edit <path|glob> [dry-run] [expect files <n> replacements <n>], replace [pattern] once|all|<n> times \"<old>\" with \"<new>\" (pattern: a regex, $1 groups in the replacement; either takes raw lines between <<< and a >>> line), patch <<< apply_patch hunks (@@, then lines starting with space, - or +) >>>, show <path>"); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: eoie do <root> <program>... | do <root> - (program on stdin); statements: edit <path|glob> [dry-run] [expect files <n> replacements <n>], replace [pattern] once|all|<n> times \"<old>\" with \"<new>\" (pattern: a regex, $1 groups in the replacement; either takes raw lines between <<< and a >>> line), patch <<< apply_patch hunks (@@, then lines starting with space, - or +) >>>, show <path>"); } LIT.with(|lit| lit.clone()) };
        println!("{}", &*v6);
        2i32
    } else {
        let mut v7: bool = directory_exists_18(v2.clone());
        let mut v8: bool = v7 == false;
        if v8 {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: root is not a directory: "); } LIT.with(|lit| lit.clone()) };
            let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v2));
            println!("{}", &*v10);
            2i32
        } else {
            let mut v11: i32 = 3i32;
            let mut v12: Rc<str> = argument_16(v11);
            let mut v13: bool = v12.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            if v13 {
                let mut v14: Rc<str> = { let mut text = String::new(); let _ = std::io::Read::read_to_string(&mut std::io::stdin(), &mut text); std::rc::Rc::<str>::from(text) };
                program_run_1(v2.clone(), v14.clone())
            } else {
                let mut v16: i32 = 3i32;
                let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v18: Rc<str> = program_from_19(v16, v0, v17.clone());
                program_run_1(v2.clone(), v18.clone())
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        do_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_do_run() -> i32 {
    closure0()()
}
