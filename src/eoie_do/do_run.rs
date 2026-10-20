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
    US2_Succeeded,
    US2_Failed,
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
fn program_decoded_4(mut v0: Rc<str>) -> US1 {
    let mut v1: Rc<str> = compiled_text_5(v0.clone());
    let mut v2: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = records_of_6(v1.clone());
    let mut v3: i32 = 0i32;
    let mut v4: i32 = 0i32;
    let mut v5: Rc<str> = field_at_7(v2.clone(), v3, v4);
    let mut v6: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
    if v6 {
        let mut v7: bool = v2.iter().skip(1).any(|fields| fields.first().map(|kind| &**kind == "query").unwrap_or(false) && fields.get(1).map(|verb| ["run", "errors", "processes", "stop"].contains(&&**verb)).unwrap_or(false));
        if v7 {
            let mut v8: bool = v2.iter().skip(1).any(|fields| fields.first().map(|kind| &**kind == "edit").unwrap_or(false));
            let mut v11: US0 = if v8 {
                US0::US0_Edits
            } else {
                US0::US0_Runs
            };
            US1::US1_Delegated(v11.clone(), v1.clone())
        } else {
            let mut v13: bool = v2.iter().skip(1).any(|fields| fields.first().map(|kind| &**kind == "query").unwrap_or(false) && fields.get(1).map(|verb| &**verb == "job").unwrap_or(false));
            if v13 {
                let mut v14: US0 = US0::US0_Jobs;
                US1::US1_Delegated(v14.clone(), v1.clone())
            } else {
                let mut v16: bool = v2.iter().skip(1).any(|fields| fields.first().map(|kind| &**kind == "query").unwrap_or(false) && fields.get(1).map(|verb| &**verb == "lab").unwrap_or(false));
                if v16 {
                    let mut v17: US0 = US0::US0_Labs;
                    US1::US1_Delegated(v17.clone(), v1.clone())
                } else {
                    let mut v19: bool = v2.iter().skip(1).any(|fields| fields.first().map(|kind| &**kind == "query").unwrap_or(false) && fields.get(1).map(|verb| &**verb == "git").unwrap_or(false));
                    if v19 {
                        let mut v20: US0 = US0::US0_Git;
                        US1::US1_Delegated(v20.clone(), v1.clone())
                    } else {
                        let mut v22: bool = v2.iter().skip(1).any(|fields| fields.first().map(|kind| &**kind == "query").unwrap_or(false) && fields.get(1).map(|verb| &**verb == "append" || &**verb == "copy").unwrap_or(false));
                        if v22 {
                            let mut v23: US0 = US0::US0_Writes;
                            US1::US1_Delegated(v23.clone(), v1.clone())
                        } else {
                            let mut v25: bool = v2.iter().skip(1).any(|fields| fields.first().map(|kind| &**kind == "query").unwrap_or(false) && fields.get(1).map(|verb| ["hash", "env", "exists", "nearest", "json", "lines", "table", "dirs", "tag", "count", "diff"].contains(&&**verb) || (&**verb == "files" && fields.iter().any(|word| &**word == "wcount"))).unwrap_or(false));
                            if v25 {
                                let mut v26: US0 = US0::US0_Facts;
                                US1::US1_Delegated(v26.clone(), v1.clone())
                            } else {
                                let mut v28: bool = v2.iter().skip(1).any(|fields| fields.first().map(|kind| &**kind == "query").unwrap_or(false));
                                if v28 {
                                    let mut v29: US0 = US0::US0_Reads;
                                    US1::US1_Delegated(v29.clone(), v1.clone())
                                } else {
                                    let mut v31: US0 = US0::US0_Edits;
                                    US1::US1_Delegated(v31.clone(), v1.clone())
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        let mut v40: i32 = 1i32;
        let mut v41: i32 = 0i32;
        let mut v42: Rc<str> = field_at_7(v2.clone(), v40, v41);
        US1::US1_CompileRejected(v42.clone())
    }
}
fn elapsed_text_9(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1.clone()
}
fn local_offset_minutes_11() -> i64 {
    let mut v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn clock_text_10() -> Rc<str> {
    let mut v0: i64 = local_offset_minutes_11();
    let mut v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1.clone()
}
fn receipt_line_8(mut v0: US2, mut v1: Rc<str>, mut v2: u64) -> () {
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
    let mut v8: Rc<str> = elapsed_text_9(v2);
    let mut v9: Rc<str> = clock_text_10();
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
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v7: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v4, &*v6, &*v6, &*v6].concat());
            println!("{}", &*v7);
            let mut v8: US2 = US2::US2_Failed;
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files 0  replacements 0"); } LIT.with(|lit| lit.clone()) };
            receipt_line_8(v8.clone(), v9.clone(), v2);
            2i32
        }
        US1::US1_Delegated(v10, v11) => {
            let mut v10: US0 = v10.clone();
            let mut v11: Rc<str> = v11.clone();
            delegated_run_2(v10.clone(), v0.clone(), v11.clone())
        }
    }
}
fn lines_joined_13(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
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
fn program_from_12(mut v0: i32, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let mut v3: bool = v0 < v1;
        if v3 {
            let mut v4: i32 = v0.wrapping_add(1i32);
            let mut v5: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
            let mut v6: Rc<str> = lines_joined_13(v2.clone(), v5.clone());
            (v0, v1, v2) = (v4, v1, v6.clone());
            continue;
        } else {
            return v2.clone();
        }
    }
}
fn do_run_0() -> i32 {
    let mut v0: i32 = std::env::args().count() as i32;
    let mut v1: i32 = 2i32;
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v1 as usize).unwrap_or_default());
    let mut v3: bool = v0 < 4i32;
    if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: eoie "); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("do <root> <program>... | do <root> - (program on stdin); statements: edit <path|glob> [dry-run] [expect files <n> replacements <n>], replace once|all|<n> times \"<old>\" with \"<new>\", show <path>"); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v5, &*v6, &*v6, &*v6].concat());
        println!("{}", &*v7);
        2i32
    } else {
        let mut v8: bool = std::path::Path::new(&*v2).is_dir();
        let mut v9: bool = v8 == false;
        if v9 {
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: root is not a directory: "); } LIT.with(|lit| lit.clone()) };
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v2, &*v11, &*v11, &*v11].concat());
            println!("{}", &*v12);
            2i32
        } else {
            let mut v13: i32 = 3i32;
            let mut v14: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v13 as usize).unwrap_or_default());
            let mut v15: bool = v14.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            if v15 {
                let mut v16: Rc<str> = { let mut text = String::new(); let _ = std::io::Read::read_to_string(&mut std::io::stdin(), &mut text); std::rc::Rc::<str>::from(text) };
                program_run_1(v2.clone(), v16.clone())
            } else {
                let mut v18: i32 = 3i32;
                let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v20: Rc<str> = program_from_12(v18, v0, v19.clone());
                program_run_1(v2.clone(), v20.clone())
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
