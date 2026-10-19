#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Succeeded,
    US0_Failed,
}
#[derive(Clone)]
enum US3 {
    US3_Apply,
    US3_Preview,
}
#[derive(Clone)]
enum US4 {
    US4_Unchecked,
    US4_Expected(u64, u64),
}
#[derive(Clone)]
enum US5 {
    US5_Once,
    US5_Every,
    US5_Exactly(u64),
}
#[derive(Clone)]
enum UH1 {
    UH1_NoRules,
    UH1_Rule(Rc<str>, Rc<str>, US5, Rc<UH1>),
}
#[derive(Clone)]
enum US2 {
    US2_EditStep(Rc<str>, US3, US4, Rc<UH1>),
    US2_ShowStep(Rc<str>),
    US2_QueryStep(Rc<str>, Rc<str>),
}
#[derive(Clone)]
enum UH0 {
    UH0_Done,
    UH0_Step(US2, Rc<UH0>),
}
#[derive(Clone)]
enum US1 {
    US1_Compiled(Rc<UH0>),
    US1_CompileRejected(Rc<str>),
    US1_QueryProgram(Rc<str>),
    US1_FactProgram(Rc<str>),
    US1_RunProgram(Rc<str>),
    US1_EditsThenRun(Rc<UH0>, Rc<str>),
}
#[derive(Clone)]
enum US6 {
    US6_Untouched,
    US6_Refused(Rc<str>),
    US6_Changed(Rc<str>, Rc<str>, u64, Rc<str>),
}
fn elapsed_text_4(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1.clone()
}
fn local_offset_minutes_6() -> i64 {
    let mut v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn clock_text_5() -> Rc<str> {
    let mut v0: i64 = local_offset_minutes_6();
    let mut v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1.clone()
}
fn receipt_line_3(mut v0: US0, mut v1: Rc<str>, mut v2: u64) -> () {
    let mut v6: Rc<str> = match &v0 {
        US0::US0_Failed => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US0::US0_Succeeded => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
    };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = elapsed_text_4(v2);
    let mut v9: Rc<str> = clock_text_5();
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v7, &*v9, &*v10, &*v10].concat());
    let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v7, &*v1, &*v7, &*v11].concat());
    println!("{}", &*v12);
    ()
}
fn refused_2(mut v0: Rc<str>, mut v1: i32, mut v2: u64) -> i32 {
    println!("{}", &*v0);
    let mut v3: US0 = US0::US0_Failed;
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files 0  replacements 0"); } LIT.with(|lit| lit.clone()) };
    receipt_line_3(v3.clone(), v4.clone(), v2);
    v1
}
fn aligned_8(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { let separator = char::from(31u8); let rows = (&*v0).lines().collect::<Vec<_>>(); let mut widths: Vec<usize> = Vec::new(); for row in &rows { if row.contains(separator) { let cells = row.split(separator).collect::<Vec<_>>(); for (index, cell) in cells[..cells.len() - 1].iter().enumerate() { let length = cell.chars().count(); if widths.len() <= index { widths.push(length); } else if widths[index] < length { widths[index] = length; } } } } let shown = rows.iter().map(|row| if row.contains(separator) { let cells = row.split(separator).collect::<Vec<_>>(); let last = cells.len() - 1; cells.iter().enumerate().map(|(index, cell)| if index == last { cell.to_string() } else { format!("{:<width$}  ", cell, width = widths[index]) }).collect::<String>() } else { row.to_string() }).collect::<Vec<_>>(); std::rc::Rc::<str>::from(shown.join("
")) };
    v1.clone()
}
fn printed_9(mut v0: Rc<str>) -> () {
    let mut v1: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v1 {
        ()
    } else {
        println!("{}", &*v0);
        ()
    }
}
fn decimal_text_10(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1.clone()
}
fn edits_refused_7(mut v0: u64, mut v1: u64, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: bool, mut v7: bool, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: u64) -> i32 {
    let mut v11: Rc<str> = aligned_8(v9.clone());
    printed_9(v11.clone());
    let mut v12: bool = v1 == 0u64;
    let mut v20: Rc<str> = if v12 {
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("nothing written: the counts differ"); } LIT.with(|lit| lit.clone()) };
        v13.clone()
    } else {
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("nothing written: "); } LIT.with(|lit| lit.clone()) };
        let mut v15: Rc<str> = decimal_text_10(v1);
        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" of "); } LIT.with(|lit| lit.clone()) };
        let mut v17: Rc<str> = decimal_text_10(v0);
        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" edits failed validation"); } LIT.with(|lit| lit.clone()) };
        let mut v19: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v15, &*v16, &*v17, &*v18].concat());
        v19.clone()
    };
    let mut v21: i32 = 1i32;
    refused_2(v20.clone(), v21, v10)
}
fn fact_run_11(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: i32 = eoie_do_facts::eoie_do_facts_run(&*v0, &*v1);
    v2
}
fn query_run_12(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: i32 = eoie_do_read::eoie_do_read_run(&*v0, &*v1);
    v2
}
fn process_run_13(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: i32 = eoie_do_process::eoie_do_process_run(&*v0, &*v1);
    v2
}
fn now_ms_14() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn compiled_text_16(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = eoie_do_program::eoie_do_program_compiled(&*v0);
    v1.clone()
}
fn records_of_17(mut v0: Rc<str>) -> std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> {
    let mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new((&*v0).split(char::from(30u8)).map(|record| record.split(char::from(31u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
    v1.clone()
}
fn field_at_18(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let mut v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).cloned().unwrap_or_default();
    v3.clone()
}
fn count_of_21(mut v0: Rc<str>, mut v1: Rc<str>) -> US5 {
    let mut v2: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("once"); } LIT.with(|lit| lit.clone()) };
    if v2 {
        US5::US5_Once
    } else {
        let mut v4: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
        if v4 {
            US5::US5_Every
        } else {
            let mut v6: u64 = v1.trim().parse::<u64>().unwrap_or(0);
            US5::US5_Exactly(v6)
        }
    }
}
fn rules_decoded_20(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> (Rc<UH1>, i32) {
    let mut v2: i32 = v0.len() as i32;
    let mut v3: bool = v1 < v2;
    let mut v7: bool = if v3 {
        let mut v4: i32 = 0i32;
        let mut v5: Rc<str> = field_at_18(v0.clone(), v1, v4);
        let mut v6: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace"); } LIT.with(|lit| lit.clone()) };
        v6
    } else {
        false
    };
    if v7 {
        let mut v8: i32 = 3i32;
        let mut v9: Rc<str> = field_at_18(v0.clone(), v1, v8);
        let mut v10: i32 = 4i32;
        let mut v11: Rc<str> = field_at_18(v0.clone(), v1, v10);
        let mut v12: i32 = 1i32;
        let mut v13: Rc<str> = field_at_18(v0.clone(), v1, v12);
        let mut v14: i32 = 2i32;
        let mut v15: Rc<str> = field_at_18(v0.clone(), v1, v14);
        let mut v16: US5 = count_of_21(v13.clone(), v15.clone());
        let mut v17: i32 = v1.wrapping_add(1i32);
        let (mut v18, mut v19): (Rc<UH1>, i32) = rules_decoded_20(v0.clone(), v17);
        let mut v20: Rc<UH1> = Rc::new(UH1::UH1_Rule(v9.clone(), v11.clone(), v16.clone(), v18.clone()));
        (v20.clone(), v19)
    } else {
        let mut v21: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_NoRules); } CASE.with(|case| case.clone()) };
        (v21.clone(), v1)
    }
}
fn steps_decoded_19(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> Rc<UH0> {
    loop {
        let mut v2: i32 = v0.len() as i32;
        let mut v3: bool = v1 < v2;
        if v3 {
            let mut v4: i32 = 0i32;
            let mut v5: Rc<str> = field_at_18(v0.clone(), v1, v4);
            let mut v6: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit"); } LIT.with(|lit| lit.clone()) };
            if v6 {
                let mut v7: i32 = 3i32;
                let mut v8: Rc<str> = field_at_18(v0.clone(), v1, v7);
                let mut v9: bool = v8.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                let mut v16: US4 = if v9 {
                    US4::US4_Unchecked
                } else {
                    let mut v11: u64 = v8.trim().parse::<u64>().unwrap_or(0);
                    let mut v12: i32 = 4i32;
                    let mut v13: Rc<str> = field_at_18(v0.clone(), v1, v12);
                    let mut v14: u64 = v13.trim().parse::<u64>().unwrap_or(0);
                    US4::US4_Expected(v11, v14)
                };
                let mut v17: i32 = 2i32;
                let mut v18: Rc<str> = field_at_18(v0.clone(), v1, v17);
                let mut v19: bool = v18.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("preview"); } LIT.with(|lit| lit.clone()) };
                let mut v22: US3 = if v19 {
                    US3::US3_Preview
                } else {
                    US3::US3_Apply
                };
                let mut v23: i32 = v1.wrapping_add(1i32);
                let (mut v24, mut v25): (Rc<UH1>, i32) = rules_decoded_20(v0.clone(), v23);
                let mut v26: i32 = 1i32;
                let mut v27: Rc<str> = field_at_18(v0.clone(), v1, v26);
                let mut v28: Rc<UH0> = steps_decoded_19(v0.clone(), v25);
                let mut v29: US2 = US2::US2_EditStep(v27.clone(), v22.clone(), v16.clone(), v24.clone());
                return Rc::new(UH0::UH0_Step(v29.clone(), v28.clone()));
            } else {
                let mut v31: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show"); } LIT.with(|lit| lit.clone()) };
                if v31 {
                    let mut v32: i32 = 1i32;
                    let mut v33: Rc<str> = field_at_18(v0.clone(), v1, v32);
                    let mut v34: i32 = v1.wrapping_add(1i32);
                    let mut v35: Rc<UH0> = steps_decoded_19(v0.clone(), v34);
                    let mut v36: US2 = US2::US2_ShowStep(v33.clone());
                    return Rc::new(UH0::UH0_Step(v36.clone(), v35.clone()));
                } else {
                    let mut v38: i32 = v1.wrapping_add(1i32);
                    (v0, v1) = (v0.clone(), v38);
                    continue;
                }
            }
        } else {
            return { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Done); } CASE.with(|case| case.clone()) };
        }
    }
}
fn program_decoded_15(mut v0: Rc<str>) -> US1 {
    let mut v1: Rc<str> = compiled_text_16(v0.clone());
    let mut v2: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = records_of_17(v1.clone());
    let mut v3: i32 = 0i32;
    let mut v4: i32 = 0i32;
    let mut v5: Rc<str> = field_at_18(v2.clone(), v3, v4);
    let mut v6: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
    if v6 {
        let mut v7: bool = v2.iter().skip(1).any(|fields| fields.first().map(|kind| &**kind == "query").unwrap_or(false) && fields.get(1).map(|verb| ["run", "errors", "processes", "stop"].contains(&&**verb)).unwrap_or(false));
        if v7 {
            let mut v8: bool = v2.iter().skip(1).any(|fields| fields.first().map(|kind| &**kind == "edit").unwrap_or(false));
            if v8 {
                let mut v9: i32 = 1i32;
                let mut v10: Rc<UH0> = steps_decoded_19(v2.clone(), v9);
                US1::US1_EditsThenRun(v10.clone(), v1.clone())
            } else {
                US1::US1_RunProgram(v1.clone())
            }
        } else {
            let mut v14: bool = v2.iter().skip(1).any(|fields| fields.first().map(|kind| &**kind == "query").unwrap_or(false) && fields.get(1).map(|verb| ["hash", "env", "exists", "nearest", "json", "lines", "table", "dirs", "tag", "count", "diff"].contains(&&**verb) || (&**verb == "files" && fields.iter().any(|word| &**word == "wcount"))).unwrap_or(false));
            if v14 {
                US1::US1_FactProgram(v1.clone())
            } else {
                let mut v16: bool = v2.iter().skip(1).any(|fields| fields.first().map(|kind| &**kind == "query").unwrap_or(false));
                if v16 {
                    US1::US1_QueryProgram(v1.clone())
                } else {
                    let mut v18: i32 = 1i32;
                    let mut v19: Rc<UH0> = steps_decoded_19(v2.clone(), v18);
                    US1::US1_Compiled(v19.clone())
                }
            }
        }
    } else {
        let mut v24: i32 = 1i32;
        let mut v25: i32 = 0i32;
        let mut v26: Rc<str> = field_at_18(v2.clone(), v24, v25);
        US1::US1_CompileRejected(v26.clone())
    }
}
fn empty_tally_22() -> (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, false, false, v0.clone(), v0.clone())
}
fn target_paths_25(mut v0: Rc<str>, mut v1: Rc<str>) -> std::rc::Rc<Vec<std::rc::Rc<str>>> {
    let mut v2: std::rc::Rc<Vec<std::rc::Rc<str>>> = { fn wildcard(mask: &str, name: &str) -> bool { let parts = mask.split(char::from(42u8)).collect::<Vec<_>>(); if parts.len() == 1 { return mask == name; } let mut rest = name; if !rest.starts_with(parts[0]) { return false; } rest = &rest[parts[0].len()..]; for middle in &parts[1..parts.len() - 1] { match rest.find(middle) { Some(at) => rest = &rest[at + middle.len()..], None => return false } } rest.len() >= parts[parts.len() - 1].len() && rest.ends_with(parts[parts.len() - 1]) } fn joined(shown: &str, name: &str) -> String { if shown.is_empty() { name.to_owned() } else { format!("{shown}/{name}") } } fn walk(base: &std::path::Path, shown: &str, segments: &[String], found: &mut Vec<String>) { let Some((head, rest)) = segments.split_first() else { if base.is_file() { found.push(shown.to_owned()); } return; }; let children = || std::fs::read_dir(base).map(|entries| entries.filter_map(|entry| entry.ok()).filter(|entry| entry.file_type().map(|kind| !kind.is_symlink()).unwrap_or(false)).collect::<Vec<_>>()).unwrap_or_default(); if head == "**" { walk(base, shown, rest, found); for entry in children() { let name = entry.file_name().to_string_lossy().into_owned(); if entry.path().is_dir() && ![".git", "target", "node_modules"].contains(&name.as_str()) { walk(&entry.path(), &joined(shown, &name), segments, found); } } } else if head.contains(char::from(42u8)) { for entry in children() { let name = entry.file_name().to_string_lossy().into_owned(); if wildcard(head, &name) { walk(&entry.path(), &joined(shown, &name), rest, found); } } } else { walk(&base.join(head), &joined(shown, head), rest, found); } } let pattern = (&*v1).replace(char::from(92u8), "/"); let segments = pattern.split(char::from(47u8)).filter(|segment| !segment.is_empty() && *segment != ".").map(str::to_owned).collect::<Vec<_>>(); let mut found = Vec::new(); if pattern.contains(char::from(42u8)) { walk(std::path::Path::new(&*v0), "", &segments, &mut found); found.sort(); found.dedup(); } else { found.push(segments.join("/")); } std::rc::Rc::new(found.into_iter().map(std::rc::Rc::<str>::from).collect::<Vec<_>>()) };
    v2.clone()
}
fn opened_block_26(mut v0: u64, mut v1: u64, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: bool, mut v7: bool, mut v8: Rc<str>, mut v9: Rc<str>) -> (u64, u64, u64, Rc<str>, u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) {
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (0u64, 0u64, 0u64, v10.clone(), v0, v1, v2, v3, v4, v5, v6, v7, v8.clone(), v9.clone())
}
fn text_now_29(mut v0: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>)>>>, mut v1: Rc<str>, mut v2: Rc<str>) -> Result<std::rc::Rc<str>, std::rc::Rc<str>> {
    let mut v3: Result<std::rc::Rc<str>, std::rc::Rc<str>> = { let staged = v0.borrow().iter().find(|entry| &*entry.0 == &*v2).map(|entry| entry.1.clone()); match staged { Some(text) => Ok(text), None => std::fs::read(std::path::Path::new(&*v1).join(&*v2)).map_err(|error| std::rc::Rc::<str>::from(format!("unreadable: {error}"))).and_then(|bytes| String::from_utf8(bytes).map(std::rc::Rc::<str>::from).map_err(|_| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from("not UTF-8 text"); } LIT.with(|lit| lit.clone()) })) } };
    v3.clone()
}
fn opened_file_30(mut v0: Rc<str>) -> (Rc<str>, u64, Rc<str>, Rc<str>, Rc<str>) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0.clone(), 0u64, v1.clone(), v1.clone(), v1.clone())
}
fn replaced_in_32(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> std::rc::Rc<(std::rc::Rc<str>, u64, std::rc::Rc<str>)> {
    let mut v3: std::rc::Rc<(std::rc::Rc<str>, u64, std::rc::Rc<str>)> = { let text = &*v0; let crlf = text.contains("
"); let adapt = |value: &str| if crlf && value.contains(char::from(10u8)) && !value.contains("
") { value.replace("
", "
") } else { value.to_owned() }; let anchor = adapt(&*v1); let replacement = adapt(&*v2); if anchor.is_empty() { return std::rc::Rc::new((std::rc::Rc::<str>::from(text), 0, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) })); } let starts = text.match_indices(anchor.as_str()).map(|(at, _)| at).collect::<Vec<_>>(); let lines = starts.iter().map(|at| (text[..*at].matches(char::from(10u8)).count() + 1).to_string()).collect::<Vec<_>>().join(","); std::rc::Rc::new((std::rc::Rc::<str>::from(text.replace(anchor.as_str(), replacement.as_str())), starts.len() as u64, std::rc::Rc::<str>::from(lines))) };
    v3.clone()
}
fn joined_with_33(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v3 {
        v1.clone()
    } else {
        let mut v4: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v4 {
            v2.clone()
        } else {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v0, &*v2, &*v5, &*v5].concat());
            v6.clone()
        }
    }
}
fn quoted_36(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { let shown = format!("{:?}", &*v0); std::rc::Rc::<str>::from(if shown.chars().count() > 80 { format!("{}...{}", shown.chars().take(76).collect::<String>(), char::from(34u8)) } else { shown }) };
    v1.clone()
}
fn mismatch_35(mut v0: Rc<str>, mut v1: u64, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: bool = v1 == 0u64;
    if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("anchor not found: "); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = quoted_36(v0.clone());
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (0 matches, expected "); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v5, &*v6, &*v2, &*v7].concat());
        v8.clone()
    } else {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("anchor matched "); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = decimal_text_10(v1);
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" times: "); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = quoted_36(v0.clone());
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (expected "); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v16: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v2, &*v14, &*v15, &*v15].concat());
        let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v10, &*v11, &*v12, &*v16].concat());
        v17.clone()
    }
}
fn count_problem_34(mut v0: US5, mut v1: u64, mut v2: bool, mut v3: Rc<str>) -> Rc<str> {
    let mut v4: bool = v1 == 0u64;
    let mut v5: bool = v4 && v2;
    match &v0 {
        US5::US5_Every => {
            let mut v20: bool = if v4 {
                let mut v19: bool = v2 == false;
                v19
            } else {
                false
            };
            if v20 {
                let mut v21: u64 = 0u64;
                let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("at least 1"); } LIT.with(|lit| lit.clone()) };
                mismatch_35(v3.clone(), v21, v22.clone())
            } else {
                let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v24.clone()
            }
        }
        US5::US5_Exactly(v12) => {
            let mut v12: u64 = *v12;
            let mut v13: bool = v1 == v12;
            let mut v14: bool = v13 || v5;
            if v14 {
                let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v15.clone()
            } else {
                let mut v16: Rc<str> = decimal_text_10(v12);
                mismatch_35(v3.clone(), v1, v16.clone())
            }
        }
        US5::US5_Once => {
            let mut v6: bool = v1 == 1u64;
            let mut v7: bool = v6 || v5;
            if v7 {
                let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v8.clone()
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
                mismatch_35(v3.clone(), v1, v9.clone())
            }
        }
    }
}
fn sites_of_37(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: Rc<str> = { let text = &*v0; let anchor = &*v1; let first = anchor.lines().next().unwrap_or(anchor); let sites = if anchor.is_empty() { Vec::new() } else { text.lines().enumerate().filter(|(_, line)| line.contains(first)).map(|(index, line)| format!("{}:{}  {}", &*v2, index + 1, line)).collect::<Vec<_>>() }; std::rc::Rc::<str>::from(sites.join("
")) };
    v3.clone()
}
fn lines_joined_38(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    joined_with_33(v2.clone(), v0.clone(), v1.clone())
}
fn rules_over_31(mut v0: Rc<UH1>, mut v1: bool, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>) -> (Rc<str>, u64, Rc<str>, Rc<str>, Rc<str>) {
    let (mut v38, mut v39, mut v40, mut v41, mut v42): (Rc<str>, u64, Rc<str>, Rc<str>, Rc<str>) = match &*v0 {
        UH1::UH1_NoRules => {
            (v3.clone(), v4, v5.clone(), v6.clone(), v7.clone())
        }
        _ => {
            let (mut v33, mut v34, mut v35, mut v36, mut v37): (Rc<str>, u64, Rc<str>, Rc<str>, Rc<str>) = match &*v0 {
                UH1::UH1_Rule(v8, v9, v10, v11) => {
                    let mut v8: Rc<str> = v8.clone();
                    let mut v9: Rc<str> = v9.clone();
                    let mut v10: US5 = v10.clone();
                    let mut v11: Rc<UH1> = v11.clone();
                    let mut v12: bool = v6.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    if v12 {
                        let mut v13: std::rc::Rc<(std::rc::Rc<str>, u64, std::rc::Rc<str>)> = replaced_in_32(v3.clone(), v8.clone(), v9.clone());
                        let mut v14: u64 = v13.clone().1;
                        let mut v15: Rc<str> = v13.0.clone();
                        let mut v16: u64 = v4.wrapping_add(v14);
                        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                        let mut v18: Rc<str> = v13.2.clone();
                        let mut v19: Rc<str> = joined_with_33(v17.clone(), v5.clone(), v18.clone());
                        let mut v20: Rc<str> = count_problem_34(v10.clone(), v14, v1, v8.clone());
                        let mut v21: Rc<str> = sites_of_37(v3.clone(), v8.clone(), v2.clone());
                        let mut v22: Rc<str> = lines_joined_38(v7.clone(), v21.clone());
                        let (mut v23, mut v24, mut v25, mut v26, mut v27): (Rc<str>, u64, Rc<str>, Rc<str>, Rc<str>) = rules_over_31(v11.clone(), v1, v2.clone(), v15.clone(), v16, v19.clone(), v20.clone(), v22.clone());
                        (v23.clone(), v24, v25.clone(), v26.clone(), v27.clone())
                    } else {
                        (v3.clone(), v4, v5.clone(), v6.clone(), v7.clone())
                    }
                }
                _ => unreachable!(),
            };
            (v33.clone(), v34, v35.clone(), v36.clone(), v37.clone())
        }
    };
    (v38.clone(), v39, v40.clone(), v41.clone(), v42.clone())
}
fn file_result_of_28(mut v0: Rc<str>, mut v1: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>)>>>, mut v2: Rc<UH1>, mut v3: bool, mut v4: Rc<str>) -> (US6, Rc<str>) {
    let mut v5: Result<std::rc::Rc<str>, std::rc::Rc<str>> = text_now_29(v1.clone(), v0.clone(), v4.clone());
    let mut v6: bool = v5.is_ok();
    let (mut v28, mut v29): (US6, Rc<str>) = if v6 {
        let mut v7: Rc<str> = v5.clone().unwrap_or_default();
        let (mut v8, mut v9, mut v10, mut v11, mut v12): (Rc<str>, u64, Rc<str>, Rc<str>, Rc<str>) = opened_file_30(v7.clone());
        let (mut v13, mut v14, mut v15, mut v16, mut v17): (Rc<str>, u64, Rc<str>, Rc<str>, Rc<str>) = rules_over_31(v2.clone(), v3, v4.clone(), v8.clone(), v9, v10.clone(), v11.clone(), v12.clone());
        let mut v18: bool = v16.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v24: US6 = if v18 {
            let mut v19: bool = v14 == 0u64;
            if v19 {
                US6::US6_Untouched
            } else {
                US6::US6_Changed(v7.clone(), v13.clone(), v14, v15.clone())
            }
        } else {
            US6::US6_Refused(v16.clone())
        };
        (v24.clone(), v17.clone())
    } else {
        let mut v25: Rc<str> = v5.clone().err().unwrap_or_default();
        let mut v26: US6 = US6::US6_Refused(v25.clone());
        let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (v26.clone(), v27.clone())
    };
    (v28.clone(), v29.clone())
}
fn row2_40(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(char::from(31u8).to_string());
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v2, &*v1, &*v3, &*v3].concat());
    v4.clone()
}
fn staged_41(mut v0: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>)>>>, mut v1: Rc<str>, mut v2: Rc<str>) -> () {
    { let mut writes = v0.borrow_mut(); match writes.iter_mut().find(|entry| &*entry.0 == &*v1) { Some(entry) => entry.1 = v2.clone(), None => writes.push((v1.clone(), v2.clone())) } };
    ()
}
fn sha256_of_42(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { use sha2::Digest; std::rc::Rc::<str>::from(sha2::Sha256::digest(v0.as_bytes()).iter().map(|byte| format!("{byte:02x}")).collect::<String>()) };
    v1.clone()
}
fn counted_43(mut v0: u64, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: Rc<str> = decimal_text_10(v0);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v5: bool = v0 == 1u64;
    let mut v6: Rc<str> = if v5 {
        v1.clone()
    } else {
        v2.clone()
    };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v4, &*v6, &*v7, &*v7].concat());
    v8.clone()
}
fn lines_label_44(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { let mut lines = (&*v0).split(char::from(44u8)).filter_map(|value| value.parse::<u64>().ok()).collect::<Vec<_>>(); lines.sort(); lines.dedup(); let mut ranges: Vec<(u64, u64)> = Vec::new(); for line in lines.iter().copied() { match ranges.last_mut() { Some(range) if range.1 + 1 == line => range.1 = line, _ => ranges.push((line, line)) } } let text = ranges.iter().map(|(first, last)| if first == last { first.to_string() } else { format!("{first}-{last}") }).collect::<Vec<_>>().join(","); std::rc::Rc::<str>::from(if lines.len() == 1 { format!("line {text}") } else { format!("lines {text}") }) };
    v1.clone()
}
fn row4_45(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let mut v4: Rc<str> = std::rc::Rc::<str>::from(char::from(31u8).to_string());
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v4, &*v3, &*v5, &*v5].concat());
    let mut v7: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v4, &*v1, &*v4, &*v6].concat());
    v7.clone()
}
fn preview_of_46(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { let old = (&*v0).lines().collect::<Vec<_>>(); let new = (&*v1).lines().collect::<Vec<_>>(); let mut shown = Vec::new(); if old.len() == new.len() { for (index, (left, right)) in old.iter().zip(new.iter()).enumerate() { if left != right { shown.push(format!("-{}  {}", index + 1, left)); shown.push(format!("+{}  {}", index + 1, right)); } } } else { let prefix = old.iter().zip(new.iter()).take_while(|(left, right)| left == right).count(); let suffix = old[prefix..].iter().rev().zip(new[prefix..].iter().rev()).take_while(|(left, right)| left == right).count(); for index in prefix..old.len() - suffix { shown.push(format!("-{}  {}", index + 1, old[index])); } for index in prefix..new.len() - suffix { shown.push(format!("+{}  {}", index + 1, new[index])); } } std::rc::Rc::<str>::from(shown.join("
")) };
    v2.clone()
}
fn pass_with_39(mut v0: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>)>>>, mut v1: US3, mut v2: Rc<str>, mut v3: US6, mut v4: Rc<str>, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: Rc<str>, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: bool, mut v16: bool, mut v17: Rc<str>, mut v18: Rc<str>) -> (u64, u64, u64, Rc<str>, u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) {
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit "); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v21: Rc<str> = std::rc::Rc::<str>::from([&*v19, &*v2, &*v20, &*v20, &*v20].concat());
    let mut v22: Rc<str> = lines_joined_38(v8.clone(), v4.clone());
    let (mut v101, mut v102, mut v103, mut v104, mut v105, mut v106, mut v107, mut v108, mut v109, mut v110, mut v111, mut v112, mut v113, mut v114): (u64, u64, u64, Rc<str>, u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = match &v3 {
        US6::US6_Untouched => {
            (v5, v6, v7, v22.clone(), v9, v10, v11, v12, v13, v14, v15, v16, v17.clone(), v18.clone())
        }
        _ => {
            let (mut v87, mut v88, mut v89, mut v90, mut v91, mut v92, mut v93, mut v94, mut v95, mut v96, mut v97, mut v98, mut v99, mut v100): (u64, u64, u64, Rc<str>, u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = match &v3 {
                US6::US6_Refused(v23) => {
                    let mut v23: Rc<str> = v23.clone();
                    let mut v24: u64 = v7.wrapping_add(1u64);
                    let mut v25: Rc<str> = row2_40(v21.clone(), v23.clone());
                    let mut v26: Rc<str> = lines_joined_38(v18.clone(), v25.clone());
                    (v5, v6, v24, v22.clone(), v9, v10, v11, v12, v13, v14, v15, v16, v17.clone(), v26.clone())
                }
                _ => {
                    let (mut v73, mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83, mut v84, mut v85, mut v86): (u64, u64, u64, Rc<str>, u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = match &v3 {
                        US6::US6_Changed(v27, v28, v29, v30) => {
                            let mut v27: Rc<str> = v27.clone();
                            let mut v28: Rc<str> = v28.clone();
                            let mut v29: u64 = *v29;
                            let mut v30: Rc<str> = v30.clone();
                            let mut v38: Rc<str> = match &v1 {
                                US3::US3_Apply => {
                                    staged_41(v0.clone(), v2.clone(), v28.clone());
                                    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sha256 "); } LIT.with(|lit| lit.clone()) };
                                    let mut v32: Rc<str> = sha256_of_42(v27.clone());
                                    let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" -> "); } LIT.with(|lit| lit.clone()) };
                                    let mut v34: Rc<str> = sha256_of_42(v28.clone());
                                    let mut v35: Rc<str> = std::rc::Rc::<str>::from([&*v31, &*v32, &*v33, &*v34, &*v20].concat());
                                    v35.clone()
                                }
                                US3::US3_Preview => {
                                    let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dry run, nothing written"); } LIT.with(|lit| lit.clone()) };
                                    v36.clone()
                                }
                            };
                            let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replacement"); } LIT.with(|lit| lit.clone()) };
                            let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replacements"); } LIT.with(|lit| lit.clone()) };
                            let mut v41: Rc<str> = counted_43(v29, v39.clone(), v40.clone());
                            let mut v42: Rc<str> = lines_label_44(v30.clone());
                            let mut v43: Rc<str> = row4_45(v21.clone(), v41.clone(), v42.clone(), v38.clone());
                            let (mut v61, mut v62, mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = match &v1 {
                                US3::US3_Apply => {
                                    let mut v44: u64 = v11.wrapping_add(1u64);
                                    let mut v45: u64 = v13.wrapping_add(v29);
                                    let mut v46: Rc<str> = lines_joined_38(v17.clone(), v43.clone());
                                    (v9, v10, v44, v12, v45, v14, v15, v16, v46.clone(), v18.clone())
                                }
                                _ => {
                                    let (mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = match &v1 {
                                        US3::US3_Preview => {
                                            let mut v47: u64 = v14.wrapping_add(v29);
                                            let mut v48: Rc<str> = lines_joined_38(v17.clone(), v43.clone());
                                            let mut v49: Rc<str> = preview_of_46(v27.clone(), v28.clone());
                                            let mut v50: Rc<str> = lines_joined_38(v48.clone(), v49.clone());
                                            (v9, v10, v11, v12, v13, v47, v15, v16, v50.clone(), v18.clone())
                                        }
                                        _ => unreachable!(),
                                    };
                                    (v51, v52, v53, v54, v55, v56, v57, v58, v59.clone(), v60.clone())
                                }
                            };
                            let mut v71: u64 = v5.wrapping_add(1u64);
                            let mut v72: u64 = v6.wrapping_add(v29);
                            (v71, v72, v7, v22.clone(), v61, v62, v63, v64, v65, v66, v67, v68, v69.clone(), v70.clone())
                        }
                        _ => unreachable!(),
                    };
                    (v73, v74, v75, v76.clone(), v77, v78, v79, v80, v81, v82, v83, v84, v85.clone(), v86.clone())
                }
            };
            (v87, v88, v89, v90.clone(), v91, v92, v93, v94, v95, v96, v97, v98, v99.clone(), v100.clone())
        }
    };
    (v101, v102, v103, v104.clone(), v105, v106, v107, v108, v109, v110, v111, v112, v113.clone(), v114.clone())
}
fn files_run_27(mut v0: Rc<str>, mut v1: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>)>>>, mut v2: Rc<str>, mut v3: US3, mut v4: US4, mut v5: Rc<UH1>, mut v6: bool, mut v7: std::rc::Rc<Vec<std::rc::Rc<str>>>, mut v8: i32, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: Rc<str>, mut v13: u64, mut v14: u64, mut v15: u64, mut v16: u64, mut v17: u64, mut v18: u64, mut v19: bool, mut v20: bool, mut v21: Rc<str>, mut v22: Rc<str>) -> (u64, u64, u64, Rc<str>, u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) {
    let mut v23: i32 = v7.len() as i32;
    let mut v24: bool = v8 < v23;
    let (mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70): (u64, u64, u64, Rc<str>, u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = if v24 {
        let mut v25: Rc<str> = v7[v8 as usize].clone();
        let mut v26: i32 = v8.wrapping_add(1i32);
        let (mut v27, mut v28): (US6, Rc<str>) = file_result_of_28(v0.clone(), v1.clone(), v5.clone(), v6, v25.clone());
        let (mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42): (u64, u64, u64, Rc<str>, u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = pass_with_39(v1.clone(), v3.clone(), v25.clone(), v27.clone(), v28.clone(), v9, v10, v11, v12.clone(), v13, v14, v15, v16, v17, v18, v19, v20, v21.clone(), v22.clone());
        let (mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55, mut v56): (u64, u64, u64, Rc<str>, u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = files_run_27(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6, v7.clone(), v26, v29, v30, v31, v32.clone(), v33, v34, v35, v36, v37, v38, v39, v40, v41.clone(), v42.clone());
        (v43, v44, v45, v46.clone(), v47, v48, v49, v50, v51, v52, v53, v54, v55.clone(), v56.clone())
    } else {
        (v9, v10, v11, v12.clone(), v13, v14, v15, v16, v17, v18, v19, v20, v21.clone(), v22.clone())
    };
    (v57, v58, v59, v60.clone(), v61, v62, v63, v64, v65, v66, v67, v68, v69.clone(), v70.clone())
}
fn block_run_24(mut v0: Rc<str>, mut v1: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>)>>>, mut v2: Rc<str>, mut v3: US3, mut v4: US4, mut v5: Rc<UH1>, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: bool, mut v13: bool, mut v14: Rc<str>, mut v15: Rc<str>) -> (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) {
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*"); } LIT.with(|lit| lit.clone()) };
    let mut v17: bool = v2.contains(&*v16);
    let mut v18: std::rc::Rc<Vec<std::rc::Rc<str>>> = target_paths_25(v0.clone(), v2.clone());
    let mut v19: u64 = v18.len() as u64;
    let mut v20: u64 = v6.wrapping_add(1u64);
    let mut v21: u64 = v9.wrapping_add(v19);
    let mut v22: bool = v12 || v17;
    let mut v23: i32 = 0i32;
    let (mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37): (u64, u64, u64, Rc<str>, u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = opened_block_26(v20, v7, v8, v21, v10, v11, v22, v13, v14.clone(), v15.clone());
    let (mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51): (u64, u64, u64, Rc<str>, u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = files_run_27(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v17, v18.clone(), v23, v24, v25, v26, v27.clone(), v28, v29, v30, v31, v32, v33, v34, v35, v36.clone(), v37.clone());
    let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit "); } LIT.with(|lit| lit.clone()) };
    let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v54: Rc<str> = std::rc::Rc::<str>::from([&*v52, &*v2, &*v53, &*v53, &*v53].concat());
    let mut v55: bool = v40 == 0u64;
    let (mut v121, mut v122, mut v123, mut v124, mut v125, mut v126, mut v127, mut v128, mut v129, mut v130): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = if v55 {
        let (mut v110, mut v111, mut v112, mut v113, mut v114, mut v115, mut v116, mut v117, mut v118, mut v119): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = match &v4 {
            US4::US4_Expected(v56, v57) => {
                let mut v56: u64 = *v56;
                let mut v57: u64 = *v57;
                let mut v58: bool = v56 == v38;
                let mut v60: bool = if v58 {
                    let mut v59: bool = v57 == v39;
                    v59
                } else {
                    false
                };
                if v60 {
                    (v42, v43, v44, v45, v46, v47, v48, v49, v50.clone(), v51.clone())
                } else {
                    let mut v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected files "); } LIT.with(|lit| lit.clone()) };
                    let mut v62: Rc<str> = decimal_text_10(v56);
                    let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" replacements "); } LIT.with(|lit| lit.clone()) };
                    let mut v64: Rc<str> = decimal_text_10(v57);
                    let mut v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
                    let mut v66: Rc<str> = std::rc::Rc::<str>::from([&*v61, &*v62, &*v63, &*v64, &*v65].concat());
                    let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("found files "); } LIT.with(|lit| lit.clone()) };
                    let mut v68: Rc<str> = decimal_text_10(v38);
                    let mut v69: Rc<str> = decimal_text_10(v39);
                    let mut v70: Rc<str> = std::rc::Rc::<str>::from([&*v67, &*v68, &*v63, &*v69, &*v53].concat());
                    let mut v71: Rc<str> = std::rc::Rc::<str>::from([&*v66, &*v70, &*v53, &*v53, &*v53].concat());
                    let mut v72: Rc<str> = row2_40(v54.clone(), v71.clone());
                    let mut v73: Rc<str> = lines_joined_38(v51.clone(), v72.clone());
                    let mut v74: Rc<str> = lines_joined_38(v73.clone(), v41.clone());
                    (v42, v43, v44, v45, v46, v47, v48, true, v50.clone(), v74.clone())
                }
            }
            _ => {
                let (mut v100, mut v101, mut v102, mut v103, mut v104, mut v105, mut v106, mut v107, mut v108, mut v109): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = match &v4 {
                    US4::US4_Unchecked => {
                        let mut v85: bool = v38 == 0u64;
                        if v85 {
                            let mut v86: u64 = v43.wrapping_add(1u64);
                            let mut v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no file matched"); } LIT.with(|lit| lit.clone()) };
                            let mut v88: Rc<str> = row2_40(v54.clone(), v87.clone());
                            let mut v89: Rc<str> = lines_joined_38(v51.clone(), v88.clone());
                            (v42, v86, v44, v45, v46, v47, v48, v49, v50.clone(), v89.clone())
                        } else {
                            (v42, v43, v44, v45, v46, v47, v48, v49, v50.clone(), v51.clone())
                        }
                    }
                    _ => unreachable!(),
                };
                (v100, v101, v102, v103, v104, v105, v106, v107, v108.clone(), v109.clone())
            }
        };
        (v110, v111, v112, v113, v114, v115, v116, v117, v118.clone(), v119.clone())
    } else {
        let mut v120: u64 = v43.wrapping_add(1u64);
        (v42, v120, v44, v45, v46, v47, v48, v49, v50.clone(), v51.clone())
    };
    (v121, v122, v123, v124, v125, v126, v127, v128, v129.clone(), v130.clone())
}
fn shown_file_47(mut v0: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>)>>>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: Rc<str> = { let staged = v0.borrow().iter().find(|entry| &*entry.0 == &*v2).map(|entry| entry.1.to_string()); let text = staged.or_else(|| std::fs::read(std::path::Path::new(&*v1).join(&*v2)).ok().map(|bytes| String::from_utf8_lossy(&bytes).into_owned())); std::rc::Rc::<str>::from(match text { Some(text) => { let lines = text.lines().collect::<Vec<_>>(); let width = lines.len().to_string().len(); let mut shown = vec![(&*v2).to_owned()]; for (index, line) in lines.iter().enumerate() { shown.push(format!("{:<width$}  {}", index + 1, line)); } shown.join("
") } None => format!("{} (unreadable)", &*v2) }) };
    v3.clone()
}
fn steps_run_23(mut v0: Rc<str>, mut v1: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>)>>>, mut v2: Rc<UH0>, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: bool, mut v10: bool, mut v11: Rc<str>, mut v12: Rc<str>) -> (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) {
    let (mut v104, mut v105, mut v106, mut v107, mut v108, mut v109, mut v110, mut v111, mut v112, mut v113): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = match &*v2 {
        UH0::UH0_Done => {
            (v3, v4, v5, v6, v7, v8, v9, v10, v11.clone(), v12.clone())
        }
        _ => {
            let (mut v94, mut v95, mut v96, mut v97, mut v98, mut v99, mut v100, mut v101, mut v102, mut v103): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = match &*v2 {
                UH0::UH0_Step(v13, v14) => {
                    let mut v13: US2 = v13.clone();
                    let mut v14: Rc<UH0> = v14.clone();
                    match &v13 {
                        US2::US2_EditStep(v15, v16, v17, v18) => {
                            let mut v15: Rc<str> = v15.clone();
                            let mut v16: US3 = v16.clone();
                            let mut v17: US4 = v17.clone();
                            let mut v18: Rc<UH1> = v18.clone();
                            let (mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = block_run_24(v0.clone(), v1.clone(), v15.clone(), v16.clone(), v17.clone(), v18.clone(), v3, v4, v5, v6, v7, v8, v9, v10, v11.clone(), v12.clone());
                            steps_run_23(v0.clone(), v1.clone(), v14.clone(), v19, v20, v21, v22, v23, v24, v25, v26, v27.clone(), v28.clone())
                        }
                        _ => {
                            let (mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = match &v13 {
                                US2::US2_ShowStep(v39) => {
                                    let mut v39: Rc<str> = v39.clone();
                                    let mut v40: Rc<str> = shown_file_47(v1.clone(), v0.clone(), v39.clone());
                                    let mut v41: Rc<str> = lines_joined_38(v11.clone(), v40.clone());
                                    steps_run_23(v0.clone(), v1.clone(), v14.clone(), v3, v4, v5, v6, v7, v8, v9, v10, v41.clone(), v12.clone())
                                }
                                _ => {
                                    let (mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = match &v13 {
                                        US2::US2_QueryStep(v52, v53) => {
                                            let mut v52: Rc<str> = v52.clone();
                                            let mut v53: Rc<str> = v53.clone();
                                            steps_run_23(v0.clone(), v1.clone(), v14.clone(), v3, v4, v5, v6, v7, v8, v9, v10, v11.clone(), v12.clone())
                                        }
                                        _ => unreachable!(),
                                    };
                                    (v64, v65, v66, v67, v68, v69, v70, v71, v72.clone(), v73.clone())
                                }
                            };
                            (v74, v75, v76, v77, v78, v79, v80, v81, v82.clone(), v83.clone())
                        }
                    }
                }
                _ => unreachable!(),
            };
            (v94, v95, v96, v97, v98, v99, v100, v101, v102.clone(), v103.clone())
        }
    };
    (v104, v105, v106, v107, v108, v109, v110, v111, v112.clone(), v113.clone())
}
fn committed_48(mut v0: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>)>>>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { let writes = v0.borrow(); let root = std::path::Path::new(&*v1); let mut temporaries = Vec::new(); let mut problem = String::new(); for (path, text) in writes.iter() { let target = root.join(&**path); let temporary = target.with_file_name(format!(".{}.eoie-do.tmp", target.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default())); match std::fs::write(&temporary, text.as_bytes()) { Ok(()) => temporaries.push((temporary, target)), Err(error) => { problem = format!("write {path}: {error}"); break; } } } if problem.is_empty() { for (temporary, target) in &temporaries { if let Err(error) = std::fs::rename(temporary, target) { problem = format!("rename {}: {error}", target.display()); break; } } } else { for (temporary, _) in &temporaries { let _ = std::fs::remove_file(temporary); } } std::rc::Rc::<str>::from(problem) };
    v2.clone()
}
fn success_counts_49(mut v0: u64, mut v1: u64, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: bool, mut v7: bool, mut v8: Rc<str>, mut v9: Rc<str>) -> Rc<str> {
    let mut v10: Rc<str> = decimal_text_10(v2);
    let mut v11: Rc<str> = decimal_text_10(v4);
    let mut v12: Rc<str> = decimal_text_10(v5);
    let mut v13: bool = v5 == 0u64;
    if v13 {
        if v6 {
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files "); } LIT.with(|lit| lit.clone()) };
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" of "); } LIT.with(|lit| lit.clone()) };
            let mut v16: Rc<str> = decimal_text_10(v3);
            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" matched  replacements "); } LIT.with(|lit| lit.clone()) };
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v19: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v11, &*v18, &*v18, &*v18].concat());
            let mut v20: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v10, &*v15, &*v16, &*v19].concat());
            v20.clone()
        } else {
            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files "); } LIT.with(|lit| lit.clone()) };
            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  replacements "); } LIT.with(|lit| lit.clone()) };
            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v24: Rc<str> = std::rc::Rc::<str>::from([&*v21, &*v10, &*v22, &*v11, &*v23].concat());
            v24.clone()
        }
    } else {
        let mut v26: bool = v4 == 0u64;
        if v26 {
            let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files 0 written  replacements "); } LIT.with(|lit| lit.clone()) };
            let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" previewed"); } LIT.with(|lit| lit.clone()) };
            let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v30: Rc<str> = std::rc::Rc::<str>::from([&*v27, &*v12, &*v28, &*v29, &*v29].concat());
            v30.clone()
        } else {
            let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files "); } LIT.with(|lit| lit.clone()) };
            let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" written  replacements "); } LIT.with(|lit| lit.clone()) };
            let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  previewed "); } LIT.with(|lit| lit.clone()) };
            let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v35: Rc<str> = std::rc::Rc::<str>::from([&*v33, &*v12, &*v34, &*v34, &*v34].concat());
            let mut v36: Rc<str> = std::rc::Rc::<str>::from([&*v31, &*v10, &*v32, &*v11, &*v35].concat());
            v36.clone()
        }
    }
}
fn run_rejection_50(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = eoie_do_process::eoie_do_process_rejection(&*v0);
    v1.clone()
}
fn runs_after_edits_51(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = eoie_do_process::eoie_do_process_after_edits(&*v0, &*v1);
    v2.clone()
}
fn part_at_52(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(30u8)).nth(v1 as usize).unwrap_or(""));
    v2.clone()
}
fn program_run_1(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: u64 = now_ms_14();
    let mut v3: US1 = program_decoded_15(v1.clone());
    match &v3 {
        US1::US1_CompileRejected(v4) => {
            let mut v4: Rc<str> = v4.clone();
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("program rejected: "); } LIT.with(|lit| lit.clone()) };
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v7: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v4, &*v6, &*v6, &*v6].concat());
            let mut v8: i32 = 2i32;
            refused_2(v7.clone(), v8, v2)
        }
        US1::US1_Compiled(v16) => {
            let mut v16: Rc<UH0> = v16.clone();
            let mut v17: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>)>>> = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
            let (mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = empty_tally_22();
            let (mut v28, mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = steps_run_23(v0.clone(), v17.clone(), v16.clone(), v18, v19, v20, v21, v22, v23, v24, v25, v26.clone(), v27.clone());
            let mut v38: bool = v29 == 0u64;
            let mut v40: bool = if v38 {
                let mut v39: bool = v35 == false;
                v39
            } else {
                false
            };
            if v40 {
                let mut v41: Rc<str> = committed_48(v17.clone(), v0.clone());
                let mut v42: bool = v41.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                if v42 {
                    let mut v43: Rc<str> = aligned_8(v36.clone());
                    printed_9(v43.clone());
                    let mut v44: US0 = US0::US0_Succeeded;
                    let mut v45: Rc<str> = success_counts_49(v28, v29, v30, v31, v32, v33, v34, v35, v36.clone(), v37.clone());
                    receipt_line_3(v44.clone(), v45.clone(), v2);
                    0i32
                } else {
                    let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("nothing written: "); } LIT.with(|lit| lit.clone()) };
                    let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v48: Rc<str> = std::rc::Rc::<str>::from([&*v46, &*v41, &*v47, &*v47, &*v47].concat());
                    let mut v49: i32 = 1i32;
                    refused_2(v48.clone(), v49, v2)
                }
            } else {
                edits_refused_7(v28, v29, v30, v31, v32, v33, v34, v35, v36.clone(), v37.clone(), v2)
            }
        }
        US1::US1_EditsThenRun(v54, v55) => {
            let mut v54: Rc<UH0> = v54.clone();
            let mut v55: Rc<str> = v55.clone();
            let mut v56: Rc<str> = run_rejection_50(v55.clone());
            let mut v57: bool = v56.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v58: bool = v57 != true;
            if v58 {
                let mut v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("program rejected: "); } LIT.with(|lit| lit.clone()) };
                let mut v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v61: Rc<str> = std::rc::Rc::<str>::from([&*v59, &*v56, &*v60, &*v60, &*v60].concat());
                let mut v62: i32 = 2i32;
                refused_2(v61.clone(), v62, v2)
            } else {
                let mut v64: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>)>>> = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
                let (mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73, mut v74): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = empty_tally_22();
                let (mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83, mut v84): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = steps_run_23(v0.clone(), v64.clone(), v54.clone(), v65, v66, v67, v68, v69, v70, v71, v72, v73.clone(), v74.clone());
                let mut v85: bool = v76 == 0u64;
                let mut v87: bool = if v85 {
                    let mut v86: bool = v82 == false;
                    v86
                } else {
                    false
                };
                if v87 {
                    let mut v88: Rc<str> = committed_48(v64.clone(), v0.clone());
                    let mut v89: bool = v88.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    if v89 {
                        let mut v90: Rc<str> = aligned_8(v83.clone());
                        printed_9(v90.clone());
                        let mut v91: Rc<str> = runs_after_edits_51(v0.clone(), v55.clone());
                        let mut v92: i32 = 0i32;
                        let mut v93: Rc<str> = part_at_52(v91.clone(), v92);
                        printed_9(v93.clone());
                        let mut v94: Rc<str> = success_counts_49(v75, v76, v77, v78, v79, v80, v81, v82, v83.clone(), v84.clone());
                        let mut v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                        let mut v96: i32 = 1i32;
                        let mut v97: Rc<str> = part_at_52(v91.clone(), v96);
                        let mut v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v99: Rc<str> = std::rc::Rc::<str>::from([&*v94, &*v95, &*v97, &*v98, &*v98].concat());
                        let mut v100: i32 = 2i32;
                        let mut v101: Rc<str> = part_at_52(v91.clone(), v100);
                        let mut v102: bool = v101.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
                        if v102 {
                            let mut v103: US0 = US0::US0_Failed;
                            receipt_line_3(v103.clone(), v99.clone(), v2);
                            1i32
                        } else {
                            let mut v104: US0 = US0::US0_Succeeded;
                            receipt_line_3(v104.clone(), v99.clone(), v2);
                            0i32
                        }
                    } else {
                        let mut v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("nothing written: "); } LIT.with(|lit| lit.clone()) };
                        let mut v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v108: Rc<str> = std::rc::Rc::<str>::from([&*v106, &*v88, &*v107, &*v107, &*v107].concat());
                        let mut v109: i32 = 1i32;
                        refused_2(v108.clone(), v109, v2)
                    }
                } else {
                    edits_refused_7(v75, v76, v77, v78, v79, v80, v81, v82, v83.clone(), v84.clone(), v2)
                }
            }
        }
        US1::US1_FactProgram(v12) => {
            let mut v12: Rc<str> = v12.clone();
            fact_run_11(v0.clone(), v12.clone())
        }
        US1::US1_QueryProgram(v10) => {
            let mut v10: Rc<str> = v10.clone();
            query_run_12(v0.clone(), v10.clone())
        }
        US1::US1_RunProgram(v14) => {
            let mut v14: Rc<str> = v14.clone();
            process_run_13(v0.clone(), v14.clone())
        }
    }
}
fn program_from_53(mut v0: i32, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let mut v3: bool = v0 < v1;
        if v3 {
            let mut v4: i32 = v0.wrapping_add(1i32);
            let mut v5: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
            let mut v6: Rc<str> = lines_joined_38(v2.clone(), v5.clone());
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
                let mut v20: Rc<str> = program_from_53(v18, v0, v19.clone());
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
