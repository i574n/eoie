#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US1 {
    US1_JobStart(Rc<str>, Rc<str>, Rc<str>, bool),
    US1_JobWait(Rc<str>, u64),
    US1_Blank,
}
#[derive(Clone)]
enum US0 {
    US0_JobParsed(US1),
    US0_JobRejected(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_Succeeded,
    US2_Failed,
}
fn now_ms_1() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn records_of_2(mut v0: Rc<str>) -> std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> {
    let mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new((&*v0).split(char::from(30u8)).map(|record| record.split(char::from(31u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
    v1.clone()
}
fn field_index_6(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: Rc<str>) -> i32 {
    let mut v3: i32 = v0.get(v1 as usize).and_then(|fields| fields.iter().position(|field| field.len() > 1 && field.starts_with(char::from(119u8)) && &field[1..] == &*v2)).map(|index| index as i32).unwrap_or(-1);
    v3
}
fn words_between_7(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32, mut v3: i32) -> Rc<str> {
    let mut v4: Rc<str> = std::rc::Rc::<str>::from(v0.get(v1 as usize).map(|fields| fields.iter().take(if v3 < 0 { fields.len() } else { v3 as usize }).skip(v2 as usize).filter(|raw| raw.len() > 1).map(|raw| if raw.starts_with(char::from(113u8)) { format!("{}{}{}", char::from(34u8), &raw[1..], char::from(34u8)) } else { raw[1..].to_owned() }).collect::<Vec<_>>().join(" ")).unwrap_or_default());
    v4.clone()
}
fn body_at_8(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let mut v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).filter(|field| !field.is_empty()).map(|field| std::rc::Rc::<str>::from(&field[1..])).unwrap_or_default();
    v3.clone()
}
fn duration_ms_9(mut v0: Rc<str>) -> u64 {
    let mut v1: u64 = { let text = (&*v0).trim(); let (digits, scale) = if let Some(value) = text.strip_suffix("ms") { (value, 1) } else if let Some(value) = text.strip_suffix(char::from(115u8)) { (value, 1000) } else if let Some(value) = text.strip_suffix(char::from(109u8)) { (value, 60000) } else { (text, 1000) }; digits.trim().parse::<f64>().map(|value| (value * scale as f64) as u64).unwrap_or(0) };
    v1
}
fn start_parsed_5(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("as"); } LIT.with(|lit| lit.clone()) };
    let mut v3: i32 = field_index_6(v0.clone(), v1, v2.clone());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout"); } LIT.with(|lit| lit.clone()) };
    let mut v5: i32 = field_index_6(v0.clone(), v1, v4.clone());
    let mut v6: i32 = 3i32;
    let mut v7: Rc<str> = words_between_7(v0.clone(), v1, v6, v3);
    let mut v8: bool = v3 < 0i32;
    let mut v10: bool = if v8 {
        true
    } else {
        let mut v9: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v9
    };
    if v10 {
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job start: expected job start <command> as <name> timeout <duration> [notify]"); } LIT.with(|lit| lit.clone()) };
        US0::US0_JobRejected(v11.clone())
    } else {
        let mut v13: bool = v5 < 0i32;
        let mut v18: bool = if v13 {
            true
        } else {
            let mut v14: i32 = v5.wrapping_add(1i32);
            let mut v15: Rc<str> = body_at_8(v0.clone(), v1, v14);
            let mut v16: u64 = duration_ms_9(v15.clone());
            let mut v17: bool = v16 == 0u64;
            v17
        };
        if v18 {
            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job start: expected timeout <duration> such as 30s or 5m"); } LIT.with(|lit| lit.clone()) };
            US0::US0_JobRejected(v19.clone())
        } else {
            let mut v21: i32 = v3.wrapping_add(1i32);
            let mut v22: Rc<str> = body_at_8(v0.clone(), v1, v21);
            let mut v23: bool = v22.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v25: bool = if v23 {
                true
            } else {
                let mut v24: bool = v22.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout"); } LIT.with(|lit| lit.clone()) };
                v24
            };
            if v25 {
                let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("as: expected a job name after it"); } LIT.with(|lit| lit.clone()) };
                US0::US0_JobRejected(v26.clone())
            } else {
                let mut v28: i32 = v5.wrapping_add(1i32);
                let mut v29: Rc<str> = body_at_8(v0.clone(), v1, v28);
                let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("notify"); } LIT.with(|lit| lit.clone()) };
                let mut v31: i32 = field_index_6(v0.clone(), v1, v30.clone());
                let mut v32: bool = v31 > 0i32;
                let mut v33: US1 = US1::US1_JobStart(v22.clone(), v7.clone(), v29.clone(), v32);
                US0::US0_JobParsed(v33.clone())
            }
        }
    }
}
fn wait_parsed_10(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let mut v2: i32 = 3i32;
    let mut v3: Rc<str> = body_at_8(v0.clone(), v1, v2);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout"); } LIT.with(|lit| lit.clone()) };
    let mut v5: i32 = field_index_6(v0.clone(), v1, v4.clone());
    let mut v6: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v8: bool = if v6 {
        true
    } else {
        let mut v7: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout"); } LIT.with(|lit| lit.clone()) };
        v7
    };
    if v8 {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job wait: expected job wait <name> [timeout <duration>]"); } LIT.with(|lit| lit.clone()) };
        US0::US0_JobRejected(v9.clone())
    } else {
        let mut v11: bool = v5 < 0i32;
        let mut v15: u64 = if v11 {
            0u64
        } else {
            let mut v12: i32 = v5.wrapping_add(1i32);
            let mut v13: Rc<str> = body_at_8(v0.clone(), v1, v12);
            duration_ms_9(v13.clone())
        };
        let mut v16: US1 = US1::US1_JobWait(v3.clone(), v15);
        US0::US0_JobParsed(v16.clone())
    }
}
fn field_at_11(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let mut v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).cloned().unwrap_or_default();
    v3.clone()
}
fn job_parsed_4(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let mut v2: i32 = 0i32;
    let mut v3: Rc<str> = field_at_11(v0.clone(), v1, v2);
    let mut v4: i32 = 2i32;
    let mut v5: Rc<str> = body_at_8(v0.clone(), v1, v4);
    let mut v6: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v6 {
        let mut v7: US1 = US1::US1_Blank;
        US0::US0_JobParsed(v7.clone())
    } else {
        let mut v9: bool = v3.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("query"); } LIT.with(|lit| lit.clone()) };
        let mut v13: bool = if v9 {
            true
        } else {
            let mut v10: i32 = 1i32;
            let mut v11: Rc<str> = field_at_11(v0.clone(), v1, v10);
            let mut v12: bool = v11.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
            v12
        };
        if v13 {
            let mut v14: i32 = 1i32;
            let mut v15: Rc<str> = field_at_11(v0.clone(), v1, v14);
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cannot share one program with job yet; run it as its own call"); } LIT.with(|lit| lit.clone()) };
            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v18: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v16, &*v17, &*v17, &*v17].concat());
            US0::US0_JobRejected(v18.clone())
        } else {
            let mut v20: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("start"); } LIT.with(|lit| lit.clone()) };
            if v20 {
                start_parsed_5(v0.clone(), v1)
            } else {
                let mut v22: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait"); } LIT.with(|lit| lit.clone()) };
                if v22 {
                    wait_parsed_10(v0.clone(), v1)
                } else {
                    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job: unknown action "); } LIT.with(|lit| lit.clone()) };
                    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; expected start or wait"); } LIT.with(|lit| lit.clone()) };
                    let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v27: Rc<str> = std::rc::Rc::<str>::from([&*v24, &*v5, &*v25, &*v26, &*v26].concat());
                    US0::US0_JobRejected(v27.clone())
                }
            }
        }
    }
}
fn first_rejection_3(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> Rc<str> {
    loop {
        let mut v2: i32 = v0.len() as i32;
        let mut v3: bool = v1 < v2;
        if v3 {
            let mut v4: US0 = job_parsed_4(v0.clone(), v1);
            match &v4 {
                US0::US0_JobParsed(v6) => {
                    let mut v6: US1 = v6.clone();
                    let mut v7: i32 = v1.wrapping_add(1i32);
                    (v0, v1) = (v0.clone(), v7);
                    continue;
                }
                US0::US0_JobRejected(v5) => {
                    let mut v5: Rc<str> = v5.clone();
                    return v5.clone();
                }
            }
        } else {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v11.clone();
        }
    }
}
fn empty_job_tally_12() -> (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0.clone(), false, 0u64, 0u64, 0u64, 0u64, v0.clone(), v0.clone(), 0i32)
}
fn shell_argv_16(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(if cfg!(windows) { format!("cmd.exe
/c
{}", &*v0) } else { format!("sh
-c
{}", &*v0) });
    v1.clone()
}
fn current_directory_of_17(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::canonicalize(&*v0).map(|path| path.to_string_lossy().trim_start_matches(&format!("{0}{0}?{0}", char::from(92u8))).to_owned()).unwrap_or_else(|_| (&*v0).to_owned()));
    v1.clone()
}
fn launched_18(mut v0: Rc<str>, mut v1: u64, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let mut v4: Rc<str> = eoie_jobs::eoie_job_launched(&*v0, v1, &*v2, &*v3);
    v4.clone()
}
fn lines_joined_19(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
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
fn part_at_20(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(30u8)).nth(v1 as usize).unwrap_or(""));
    v2.clone()
}
fn started_pid_21(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { let mut pid = String::new(); for _ in 0..40 { let summary = eoie_jobs::eoie_job_settled(&*v0, 0); if let Some(at) = summary.find(" pid=") { pid = summary[at + 5..].split(char::from(32u8)).next().unwrap_or("").to_owned(); } if !pid.is_empty() { break; } std::thread::sleep(std::time::Duration::from_millis(50)); } std::rc::Rc::<str>::from(pid) };
    v1.clone()
}
fn start_run_15(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool, mut v5: Rc<str>, mut v6: bool, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: Rc<str>, mut v12: Rc<str>, mut v13: i32) -> (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) {
    let mut v14: u64 = duration_ms_9(v3.clone());
    let mut v15: Rc<str> = shell_argv_16(v2.clone());
    let mut v16: Rc<str> = current_directory_of_17(v0.clone());
    let mut v17: Rc<str> = launched_18(v1.clone(), v14, v15.clone(), v16.clone());
    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: "); } LIT.with(|lit| lit.clone()) };
    let mut v19: bool = v17.starts_with(&*v18);
    let (mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = if v19 {
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job "); } LIT.with(|lit| lit.clone()) };
        let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
        let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v23: Rc<str> = std::rc::Rc::<str>::from([&*v20, &*v1, &*v21, &*v17, &*v22].concat());
        let mut v24: Rc<str> = lines_joined_19(v5.clone(), v23.clone());
        (v24.clone(), true, v7, v8, v9, v10, v11.clone(), v12.clone(), 1i32)
    } else {
        let mut v25: i32 = 0i32;
        let mut v26: Rc<str> = part_at_20(v17.clone(), v25);
        let mut v27: Rc<str> = started_pid_21(v26.clone());
        let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job "); } LIT.with(|lit| lit.clone()) };
        let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  started "); } LIT.with(|lit| lit.clone()) };
        let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  pid "); } LIT.with(|lit| lit.clone()) };
        let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  timeout "); } LIT.with(|lit| lit.clone()) };
        let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v33: Rc<str> = std::rc::Rc::<str>::from([&*v30, &*v27, &*v31, &*v3, &*v32].concat());
        let mut v34: Rc<str> = std::rc::Rc::<str>::from([&*v28, &*v1, &*v29, &*v2, &*v33].concat());
        let mut v35: Rc<str> = lines_joined_19(v5.clone(), v34.clone());
        let mut v36: u64 = v7.wrapping_add(1u64);
        let mut v37: Rc<str> = lines_joined_19(v11.clone(), v1.clone());
        let mut v39: Rc<str> = if v4 {
            lines_joined_19(v12.clone(), v1.clone())
        } else {
            v12.clone()
        };
        (v35.clone(), v6, v36, v8, v9, v10, v37.clone(), v39.clone(), v13)
    };
    (v40.clone(), v41, v42, v43, v44, v45, v46.clone(), v47.clone(), v48)
}
fn labeled_23(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = eoie_jobs::eoie_job_labeled(&*v0);
    v1.clone()
}
fn name_listed_24(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: bool = (v0).lines().any(|item| item == &*v1);
    v2
}
fn settled_25(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let mut v2: Rc<str> = eoie_jobs::eoie_job_settled(&*v0, v1);
    v2.clone()
}
fn summary_field_26(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from({ let marker = format!(" {}=", &*v1); let summary = &*v0; match summary.find(&marker) { Some(at) => { let rest = &summary[at + marker.len()..]; if let Some(quoted) = rest.strip_prefix(char::from(34u8)) { quoted.split(char::from(34u8)).next().unwrap_or("").to_owned() } else { rest.split(char::from(32u8)).next().unwrap_or("").to_owned() } } None => String::new() } });
    v2.clone()
}
fn notice_of_27(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished (exit "); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v2, &*v5, &*v6, &*v6].concat());
        v7.clone()
    } else {
        let mut v8: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed-out"); } LIT.with(|lit| lit.clone()) };
        if v8 {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed out"); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        } else {
            v1.clone()
        }
    };
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("notify  "); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v0, &*v13, &*v11, &*v14].concat());
    v15.clone()
}
fn decimal_of_29(mut v0: Rc<str>) -> u64 {
    let mut v1: u64 = (v0).trim().parse::<u64>().unwrap_or(0);
    v1
}
fn duration_text_30(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from({ let ms = v0; if ms % 1000 == 0 && ms > 0 { format!("{}s", ms / 1000) } else if ms >= 1000 { format!("{:.1}s", ms as f64 / 1000.0) } else { format!("{ms}ms") } });
    v1.clone()
}
fn settled_line_28(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64) -> Rc<str> {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = summary_field_26(v1.clone(), v3.clone());
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("detail"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = summary_field_26(v1.clone(), v5.clone());
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job "); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v0, &*v8, &*v9, &*v9].concat());
    let mut v11: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished"); } LIT.with(|lit| lit.clone()) };
    if v11 {
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished after "); } LIT.with(|lit| lit.clone()) };
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("elapsed_ms"); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<str> = summary_field_26(v1.clone(), v13.clone());
        let mut v15: u64 = decimal_of_29(v14.clone());
        let mut v16: Rc<str> = duration_text_30(v15);
        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  exit "); } LIT.with(|lit| lit.clone()) };
        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
        let mut v19: Rc<str> = summary_field_26(v1.clone(), v18.clone());
        let mut v20: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v12, &*v16, &*v17, &*v19].concat());
        v20.clone()
    } else {
        let mut v21: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed-out"); } LIT.with(|lit| lit.clone()) };
        if v21 {
            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed out after "); } LIT.with(|lit| lit.clone()) };
            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout_ms"); } LIT.with(|lit| lit.clone()) };
            let mut v24: Rc<str> = summary_field_26(v1.clone(), v23.clone());
            let mut v25: u64 = decimal_of_29(v24.clone());
            let mut v26: Rc<str> = duration_text_30(v25);
            let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("process tree stopped"); } LIT.with(|lit| lit.clone()) };
            let mut v28: bool = v6.contains(&*v27);
            let mut v33: Rc<str> = if v28 {
                let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  stopped (tree)"); } LIT.with(|lit| lit.clone()) };
                v29.clone()
            } else {
                let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wall-clock timeout; "); } LIT.with(|lit| lit.clone()) };
                let mut v31: Rc<str> = std::rc::Rc::<str>::from(v6.replace(&*v30, &v9));
                let mut v32: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v31, &*v9, &*v9, &*v9].concat());
                v32.clone()
            };
            let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  exit killed"); } LIT.with(|lit| lit.clone()) };
            let mut v35: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v22, &*v26, &*v33, &*v34].concat());
            v35.clone()
        } else {
            let mut v36: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("running"); } LIT.with(|lit| lit.clone()) };
            let mut v38: bool = if v36 {
                true
            } else {
                let mut v37: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pending"); } LIT.with(|lit| lit.clone()) };
                v37
            };
            if v38 {
                let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("still running after "); } LIT.with(|lit| lit.clone()) };
                let mut v40: Rc<str> = duration_text_30(v2);
                let mut v41: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v39, &*v40, &*v9, &*v9].concat());
                v41.clone()
            } else {
                let mut v42: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v4, &*v8, &*v6, &*v9].concat());
                v42.clone()
            }
        }
    }
}
fn output_tail_32(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let mut v2: Rc<str> = eoie_jobs::eoie_job_output(&*v0, v1);
    v2.clone()
}
fn line_count_33(mut v0: Rc<str>) -> u64 {
    let mut v1: u64 = (v0).lines().filter(|line| !line.is_empty()).count() as u64;
    v1
}
fn decimal_text_34(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1.clone()
}
fn output_block_31(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: u64 = 10u64;
    let mut v3: Rc<str> = output_tail_32(v1.clone(), v2);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = std::rc::Rc::<str>::from(v3.trim_end_matches(&*v4));
    let mut v6: u64 = line_count_33(v5.clone());
    let mut v7: bool = v6 == 0u64;
    if v7 {
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" output (no lines)"); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v8, &*v9, &*v9, &*v9].concat());
        v10.clone()
    } else {
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" output (last "); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = decimal_text_34(v6);
        let mut v13: bool = v6 == 1u64;
        let mut v16: Rc<str> = if v13 {
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" line)"); } LIT.with(|lit| lit.clone()) };
            v14.clone()
        } else {
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" lines)"); } LIT.with(|lit| lit.clone()) };
            v15.clone()
        };
        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v18: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v11, &*v12, &*v16, &*v17].concat());
        lines_joined_19(v18.clone(), v5.clone())
    }
}
fn wait_run_22(mut v0: Rc<str>, mut v1: u64, mut v2: Rc<str>, mut v3: bool, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: i32) -> (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) {
    let mut v11: Rc<str> = labeled_23(v0.clone());
    let mut v12: bool = name_listed_24(v8.clone(), v0.clone());
    let mut v14: u64 = if v12 {
        v4
    } else {
        let mut v13: u64 = v4.wrapping_add(1u64);
        v13
    };
    let mut v15: bool = v11.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let (mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64, mut v65, mut v66): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = if v15 {
        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job wait: no job named "); } LIT.with(|lit| lit.clone()) };
        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v18: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v0, &*v17, &*v17, &*v17].concat());
        let mut v19: Rc<str> = lines_joined_19(v2.clone(), v18.clone());
        (v19.clone(), true, v14, v5, v6, v7, v8.clone(), v9.clone(), 1i32)
    } else {
        let mut v20: bool = v1 == 0u64;
        let mut v21: u64 = if v20 {
            86400000u64
        } else {
            v1
        };
        let mut v22: Rc<str> = settled_25(v11.clone(), v21);
        let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state"); } LIT.with(|lit| lit.clone()) };
        let mut v24: Rc<str> = summary_field_26(v22.clone(), v23.clone());
        let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
        let mut v26: Rc<str> = summary_field_26(v22.clone(), v25.clone());
        let mut v27: bool = name_listed_24(v9.clone(), v0.clone());
        let mut v30: Rc<str> = if v27 {
            notice_of_27(v0.clone(), v24.clone(), v26.clone())
        } else {
            let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v29.clone()
        };
        let mut v31: bool = v24.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed-out"); } LIT.with(|lit| lit.clone()) };
        let mut v33: bool = if v31 {
            true
        } else {
            let mut v32: bool = v24.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("running"); } LIT.with(|lit| lit.clone()) };
            v32
        };
        let mut v35: bool = if v33 {
            true
        } else {
            let mut v34: bool = v24.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pending"); } LIT.with(|lit| lit.clone()) };
            v34
        };
        let mut v40: i32 = if v35 {
            3i32
        } else {
            let mut v36: bool = v24.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished"); } LIT.with(|lit| lit.clone()) };
            let mut v38: bool = if v36 {
                let mut v37: bool = v26.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                v37
            } else {
                false
            };
            if v38 {
                0i32
            } else {
                1i32
            }
        };
        let mut v41: Rc<str> = settled_line_28(v0.clone(), v22.clone(), v1);
        let mut v42: Rc<str> = lines_joined_19(v2.clone(), v41.clone());
        let mut v43: Rc<str> = lines_joined_19(v42.clone(), v30.clone());
        let mut v44: Rc<str> = output_block_31(v0.clone(), v11.clone());
        let mut v45: Rc<str> = lines_joined_19(v43.clone(), v44.clone());
        let mut v46: bool = v24.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished"); } LIT.with(|lit| lit.clone()) };
        let mut v47: u64 = if v46 {
            1u64
        } else {
            0u64
        };
        let mut v48: u64 = if v31 {
            1u64
        } else {
            0u64
        };
        let mut v49: bool = v30.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v50: u64 = if v49 {
            0u64
        } else {
            1u64
        };
        let mut v52: bool = if v3 {
            true
        } else {
            let mut v51: bool = v40 != 0i32;
            v51
        };
        let mut v53: u64 = v5.wrapping_add(v47);
        let mut v54: u64 = v6.wrapping_add(v48);
        let mut v55: u64 = v7.wrapping_add(v50);
        let mut v56: bool = v40 > v10;
        let mut v57: i32 = if v56 {
            v40
        } else {
            v10
        };
        (v45.clone(), v52, v14, v53, v54, v55, v8.clone(), v9.clone(), v57)
    };
    (v58.clone(), v59, v60, v61, v62, v63, v64.clone(), v65.clone(), v66)
}
fn step_run_14(mut v0: Rc<str>, mut v1: US1, mut v2: Rc<str>, mut v3: bool, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: i32) -> (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) {
    let (mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = match &v1 {
        US1::US1_JobStart(v11, v12, v13, v14) => {
            let mut v11: Rc<str> = v11.clone();
            let mut v12: Rc<str> = v12.clone();
            let mut v13: Rc<str> = v13.clone();
            let mut v14: bool = *v14;
            start_run_15(v0.clone(), v11.clone(), v12.clone(), v13.clone(), v14, v2.clone(), v3, v4, v5, v6, v7, v8.clone(), v9.clone(), v10)
        }
        _ => {
            let (mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = match &v1 {
                US1::US1_JobWait(v24, v25) => {
                    let mut v24: Rc<str> = v24.clone();
                    let mut v25: u64 = *v25;
                    wait_run_22(v24.clone(), v25, v2.clone(), v3, v4, v5, v6, v7, v8.clone(), v9.clone(), v10)
                }
                _ => {
                    let (mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = match &v1 {
                        US1::US1_Blank => {
                            (v2.clone(), v3, v4, v5, v6, v7, v8.clone(), v9.clone(), v10)
                        }
                        _ => unreachable!(),
                    };
                    (v35.clone(), v36, v37, v38, v39, v40, v41.clone(), v42.clone(), v43)
                }
            };
            (v44.clone(), v45, v46, v47, v48, v49, v50.clone(), v51.clone(), v52)
        }
    };
    (v53.clone(), v54, v55, v56, v57, v58, v59.clone(), v60.clone(), v61)
}
fn jobs_run_13(mut v0: Rc<str>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: i32, mut v3: Rc<str>, mut v4: bool, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: i32) -> (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) {
    let mut v12: i32 = v1.len() as i32;
    let mut v13: bool = v2 < v12;
    let (mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = if v13 {
        let mut v14: US0 = job_parsed_4(v1.clone(), v2);
        let (mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = match &v14 {
            US0::US0_JobParsed(v15) => {
                let mut v15: US1 = v15.clone();
                let mut v16: i32 = v2.wrapping_add(1i32);
                let (mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = step_run_14(v0.clone(), v15.clone(), v3.clone(), v4, v5, v6, v7, v8, v9.clone(), v10.clone(), v11);
                jobs_run_13(v0.clone(), v1.clone(), v16, v17.clone(), v18, v19, v20, v21, v22, v23.clone(), v24.clone(), v25)
            }
            _ => {
                let (mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = match &v14 {
                    US0::US0_JobRejected(v35) => {
                        let mut v35: Rc<str> = v35.clone();
                        let mut v36: i32 = v2.wrapping_add(1i32);
                        jobs_run_13(v0.clone(), v1.clone(), v36, v3.clone(), v4, v5, v6, v7, v8, v9.clone(), v10.clone(), v11)
                    }
                    _ => unreachable!(),
                };
                (v46.clone(), v47, v48, v49, v50, v51, v52.clone(), v53.clone(), v54)
            }
        };
        (v55.clone(), v56, v57, v58, v59, v60, v61.clone(), v62.clone(), v63)
    } else {
        (v3.clone(), v4, v5, v6, v7, v8, v9.clone(), v10.clone(), v11)
    };
    (v64.clone(), v65, v66, v67, v68, v69, v70.clone(), v71.clone(), v72)
}
fn code_text_36(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1.clone()
}
fn receipt_counts_35(mut v0: Rc<str>, mut v1: bool, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: i32) -> Rc<str> {
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("jobs "); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = decimal_text_34(v2);
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v10, &*v11, &*v11, &*v11].concat());
    let mut v13: bool = v3 > 0u64;
    let mut v17: Rc<str> = if v13 {
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  finished "); } LIT.with(|lit| lit.clone()) };
        let mut v15: Rc<str> = decimal_text_34(v3);
        let mut v16: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v14, &*v15, &*v11, &*v11].concat());
        v16.clone()
    } else {
        v12.clone()
    };
    let mut v18: bool = v4 > 0u64;
    let mut v22: Rc<str> = if v18 {
        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  timed out "); } LIT.with(|lit| lit.clone()) };
        let mut v20: Rc<str> = decimal_text_34(v4);
        let mut v21: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v19, &*v20, &*v11, &*v11].concat());
        v21.clone()
    } else {
        v17.clone()
    };
    let mut v23: bool = v7.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v27: Rc<str> = if v23 {
        let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  notifications "); } LIT.with(|lit| lit.clone()) };
        let mut v25: Rc<str> = decimal_text_34(v5);
        let mut v26: Rc<str> = std::rc::Rc::<str>::from([&*v22, &*v24, &*v25, &*v11, &*v11].concat());
        v26.clone()
    } else {
        v22.clone()
    };
    let mut v28: bool = v8 == 0i32;
    if v28 {
        v27.clone()
    } else {
        let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  exit "); } LIT.with(|lit| lit.clone()) };
        let mut v30: Rc<str> = code_text_36(v8);
        let mut v31: Rc<str> = std::rc::Rc::<str>::from([&*v27, &*v29, &*v30, &*v11, &*v11].concat());
        v31.clone()
    }
}
fn elapsed_text_38(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1.clone()
}
fn local_offset_minutes_40() -> i64 {
    let mut v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn clock_text_39() -> Rc<str> {
    let mut v0: i64 = local_offset_minutes_40();
    let mut v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1.clone()
}
fn receipt_line_37(mut v0: US2, mut v1: Rc<str>, mut v2: u64) -> () {
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
    let mut v8: Rc<str> = elapsed_text_38(v2);
    let mut v9: Rc<str> = clock_text_39();
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v7, &*v9, &*v10, &*v10].concat());
    let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v7, &*v1, &*v7, &*v11].concat());
    println!("{}", &*v12);
    ()
}
fn job_program_run_0(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: u64 = now_ms_1();
    let mut v3: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = records_of_2(v1.clone());
    let mut v4: i32 = 1i32;
    let mut v5: Rc<str> = first_rejection_3(v3.clone(), v4);
    let mut v6: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v6 {
        let mut v7: i32 = 1i32;
        let (mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = empty_job_tally_12();
        let (mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = jobs_run_13(v0.clone(), v3.clone(), v7, v8.clone(), v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16);
        let mut v26: bool = v17.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v26 {
            ()
        } else {
            println!("{}", &*v17);
            ()
        };
        if v18 {
            let mut v27: US2 = US2::US2_Failed;
            let mut v28: Rc<str> = receipt_counts_35(v17.clone(), v18, v19, v20, v21, v22, v23.clone(), v24.clone(), v25);
            receipt_line_37(v27.clone(), v28.clone(), v2);
            v25
        } else {
            let mut v29: US2 = US2::US2_Succeeded;
            let mut v30: Rc<str> = receipt_counts_35(v17.clone(), v18, v19, v20, v21, v22, v23.clone(), v24.clone(), v25);
            receipt_line_37(v29.clone(), v30.clone(), v2);
            0i32
        }
    } else {
        let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("program rejected: "); } LIT.with(|lit| lit.clone()) };
        let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v34: Rc<str> = std::rc::Rc::<str>::from([&*v32, &*v5, &*v33, &*v33, &*v33].concat());
        println!("{}", &*v34);
        let mut v35: US2 = US2::US2_Failed;
        let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("jobs 0"); } LIT.with(|lit| lit.clone()) };
        receipt_line_37(v35.clone(), v36.clone(), v2);
        2i32
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        job_program_run_0(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_do_jobs_run(v0: &str, v1: &str) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
