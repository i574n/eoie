#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US1 {
    US1_TablesDiffed(Rc<str>, Rc<str>, Rc<str>, Rc<str>),
}
#[derive(Clone)]
enum US0 {
    US0_TableParsed(US1),
    US0_TableRejected(Rc<str>),
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
fn field_at_4(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let mut v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).cloned().unwrap_or_default();
    v3.clone()
}
fn body_at_6(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let mut v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).filter(|field| !field.is_empty()).map(|field| std::rc::Rc::<str>::from(&field[1..])).unwrap_or_default();
    v3.clone()
}
fn field_index_7(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: Rc<str>) -> i32 {
    let mut v3: i32 = v0.get(v1 as usize).and_then(|fields| fields.iter().position(|field| field.len() > 1 && field.starts_with(char::from(119u8)) && &field[1..] == &*v2)).map(|index| index as i32).unwrap_or(-1);
    v3
}
fn items_span_8(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32, mut v3: i32) -> Rc<str> {
    let mut v4: Rc<str> = { let fields = v0.get(v1 as usize).cloned().unwrap_or_default(); let until = if v3 < 0 { fields.len() } else { (v3 as usize).min(fields.len()) }; let mut items = Vec::new(); for raw in fields.iter().take(until).skip(v2 as usize) { if raw.is_empty() { continue; } let (kind, body) = raw.split_at(1); if kind == "q" { items.push(body.to_owned()); } else { items.extend(body.split(|c: char| c == char::from(44u8) || c.is_whitespace()).filter(|item| !item.is_empty()).map(str::to_owned)); } } std::rc::Rc::<str>::from(items.join("
")) };
    v4.clone()
}
fn table_parsed_5(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let mut v2: i32 = 1i32;
    let mut v3: Rc<str> = field_at_4(v0.clone(), v1, v2);
    let mut v4: i32 = 2i32;
    let mut v5: Rc<str> = body_at_6(v0.clone(), v1, v4);
    let mut v6: i32 = 3i32;
    let mut v7: Rc<str> = body_at_6(v0.clone(), v1, v6);
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("by"); } LIT.with(|lit| lit.clone()) };
    let mut v9: i32 = field_index_7(v0.clone(), v1, v8.clone());
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("on"); } LIT.with(|lit| lit.clone()) };
    let mut v11: i32 = field_index_7(v0.clone(), v1, v10.clone());
    let mut v12: bool = v3.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("diff"); } LIT.with(|lit| lit.clone()) };
    if v12 {
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cannot share one program with diff yet; run it as its own call"); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v15: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v13, &*v14, &*v14, &*v14].concat());
        US0::US0_TableRejected(v15.clone())
    } else {
        let mut v17: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v19: bool = if v17 {
            true
        } else {
            let mut v18: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v18
        };
        let mut v21: bool = if v19 {
            true
        } else {
            let mut v20: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("by"); } LIT.with(|lit| lit.clone()) };
            v20
        };
        if v21 {
            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("diff: expected two tables"); } LIT.with(|lit| lit.clone()) };
            US0::US0_TableRejected(v22.clone())
        } else {
            let mut v24: bool = v9 < 0i32;
            if v24 {
                let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("diff: expected by <key,...>"); } LIT.with(|lit| lit.clone()) };
                US0::US0_TableRejected(v25.clone())
            } else {
                let mut v27: i32 = v9.wrapping_add(1i32);
                let mut v28: bool = v11 < 0i32;
                let mut v29: i32 = if v28 {
                    -1i32
                } else {
                    v11
                };
                let mut v30: Rc<str> = items_span_8(v0.clone(), v1, v27, v29);
                let mut v35: Rc<str> = if v28 {
                    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v31.clone()
                } else {
                    let mut v32: i32 = v11.wrapping_add(1i32);
                    let mut v33: i32 = -1i32;
                    items_span_8(v0.clone(), v1, v32, v33)
                };
                let mut v36: bool = v30.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                if v36 {
                    let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("by: expected a value after it"); } LIT.with(|lit| lit.clone()) };
                    US0::US0_TableRejected(v37.clone())
                } else {
                    let mut v39: US1 = US1::US1_TablesDiffed(v5.clone(), v7.clone(), v30.clone(), v35.clone());
                    US0::US0_TableParsed(v39.clone())
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
            let mut v4: i32 = 0i32;
            let mut v5: Rc<str> = field_at_4(v0.clone(), v1, v4);
            let mut v6: bool = v5.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("query"); } LIT.with(|lit| lit.clone()) };
            if v6 {
                let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit and diff statements cannot share one program yet; run them as two calls"); } LIT.with(|lit| lit.clone()) };
                return v7.clone();
            } else {
                let mut v8: US0 = table_parsed_5(v0.clone(), v1);
                match &v8 {
                    US0::US0_TableParsed(v10) => {
                        let mut v10: US1 = v10.clone();
                        let mut v11: i32 = v1.wrapping_add(1i32);
                        (v0, v1) = (v0.clone(), v11);
                        continue;
                    }
                    US0::US0_TableRejected(v9) => {
                        let mut v9: Rc<str> = v9.clone();
                        return v9.clone();
                    }
                }
            }
        } else {
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v16.clone();
        }
    }
}
fn empty_diff_tally_9() -> (Rc<str>, bool, u64, u64, u64, u64) {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0.clone(), false, 0u64, 0u64, 0u64, 0u64)
}
fn tables_diffed_11(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> {
    let mut v5: std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> = (|| -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> { let read = |path: &str| -> Result<(Vec<String>, Vec<Vec<String>>), String> { let text = std::fs::read_to_string(std::path::Path::new(&*v0).join(path)).map_err(|error| format!("{path}  unreadable: {error}"))?; let separator = if path.ends_with(".csv") { char::from(44u8) } else { char::from(9u8) }; let mut lines = text.lines(); let header = lines.next().unwrap_or("").split(separator).map(|cell| cell.trim().to_owned()).collect::<Vec<_>>(); Ok((header, lines.filter(|line| !line.trim().is_empty()).map(|line| line.split(separator).map(|cell| cell.trim().to_owned()).collect()).collect())) }; let fail = |message: String| std::rc::Rc::new((std::rc::Rc::<str>::from(message), [0, 0, 0, 0], true)); let (left_header, left_rows) = match read(&*v1) { Ok(table) => table, Err(message) => return fail(message) }; let (right_header, right_rows) = match read(&*v2) { Ok(table) => table, Err(message) => return fail(message) }; let keys = (&*v3).lines().filter(|key| !key.is_empty()).map(str::to_owned).collect::<Vec<_>>(); let mut columns = (&*v4).lines().filter(|column| !column.is_empty()).map(str::to_owned).collect::<Vec<_>>(); if columns.is_empty() { columns = left_header.iter().filter(|name| !keys.contains(name) && right_header.contains(name)).cloned().collect(); } let position = |header: &Vec<String>, name: &String| header.iter().position(|title| title == name); for name in keys.iter().chain(&columns) { if position(&left_header, name).is_none() || position(&right_header, name).is_none() { return fail(format!("diff: column {name} is not in both tables")); } } let key_of = |header: &Vec<String>, row: &Vec<String>| keys.iter().map(|name| row.get(position(header, name).unwrap()).cloned().unwrap_or_default()).collect::<Vec<_>>().join("|"); let cell = |header: &Vec<String>, row: &Vec<String>, name: &String| row.get(position(header, name).unwrap()).cloned().unwrap_or_default(); let mut changes: Vec<(String, String, String, String)> = Vec::new(); let mut changed = 0u64; let mut both = 0u64; let mut removed = Vec::new(); for row in &left_rows { let key = key_of(&left_header, row); match right_rows.iter().find(|other| key_of(&right_header, other) == key) { None => removed.push(key), Some(other) => { both += 1; let mut differs = false; for name in &columns { let (before, after) = (cell(&left_header, row, name), cell(&right_header, other, name)); if before != after { differs = true; changes.push((key.clone(), name.clone(), before, after)); } } if differs { changed += 1; } } } } let added = right_rows.iter().map(|row| key_of(&right_header, row)).filter(|key| !left_rows.iter().any(|row| &key_of(&left_header, row) == key)).collect::<Vec<_>>(); let key_width = changes.iter().map(|change| change.0.len()).chain(if added.is_empty() { None } else { Some(5) }).chain(if removed.is_empty() { None } else { Some(7) }).max().unwrap_or(0).max(3); let column_width = changes.iter().map(|change| change.1.len()).max().unwrap_or(0).max(6); let before_width = changes.iter().map(|change| change.2.len()).max().unwrap_or(0).max(1); let mut shown = vec![format!("{:<key_width$}  {:<column_width$}  {:<before_width$}  {}", "key", "column", "a", "b")]; for change in &changes { shown.push(format!("{:<key_width$}  {:<column_width$}  {:<before_width$}  {}", change.0, change.1, change.2, change.3).trim_end().to_owned()); } for key in &added { shown.push(format!("{:<key_width$}  {}", "added", key)); } for key in &removed { shown.push(format!("{:<key_width$}  {}", "removed", key)); } std::rc::Rc::new((std::rc::Rc::<str>::from(shown.join("
")), [left_rows.len() as u64, right_rows.len() as u64, changed, both], false)) })();
    v5.clone()
}
fn lines_joined_12(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
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
fn tables_run_10(mut v0: Rc<str>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: i32, mut v3: Rc<str>, mut v4: bool, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64) -> (Rc<str>, bool, u64, u64, u64, u64) {
    let mut v9: i32 = v1.len() as i32;
    let mut v10: bool = v2 < v9;
    let (mut v67, mut v68, mut v69, mut v70, mut v71, mut v72): (Rc<str>, bool, u64, u64, u64, u64) = if v10 {
        let mut v11: US0 = table_parsed_5(v1.clone(), v2);
        let (mut v61, mut v62, mut v63, mut v64, mut v65, mut v66): (Rc<str>, bool, u64, u64, u64, u64) = match &v11 {
            US0::US0_TableParsed(v12) => {
                let mut v12: US1 = v12.clone();
                match &v12 {
                    US1::US1_TablesDiffed(v13, v14, v15, v16) => {
                        let mut v13: Rc<str> = v13.clone();
                        let mut v14: Rc<str> = v14.clone();
                        let mut v15: Rc<str> = v15.clone();
                        let mut v16: Rc<str> = v16.clone();
                        let mut v17: std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> = tables_diffed_11(v0.clone(), v13.clone(), v14.clone(), v15.clone(), v16.clone());
                        let mut v18: Rc<str> = v17.0.clone();
                        let mut v19: Rc<str> = lines_joined_12(v3.clone(), v18.clone());
                        let mut v21: bool = if v4 {
                            true
                        } else {
                            let mut v20: bool = v17.clone().2;
                            v20
                        };
                        let mut v22: u64 = 0u64;
                        let mut v23: u64 = v17.1[v22 as usize];
                        let mut v24: u64 = v5.wrapping_add(v23);
                        let mut v25: u64 = 1u64;
                        let mut v26: u64 = v17.1[v25 as usize];
                        let mut v27: u64 = v6.wrapping_add(v26);
                        let mut v28: u64 = 2u64;
                        let mut v29: u64 = v17.1[v28 as usize];
                        let mut v30: u64 = v7.wrapping_add(v29);
                        let mut v31: u64 = 3u64;
                        let mut v32: u64 = v17.1[v31 as usize];
                        let mut v33: u64 = v8.wrapping_add(v32);
                        let mut v34: i32 = v2.wrapping_add(1i32);
                        tables_run_10(v0.clone(), v1.clone(), v34, v19.clone(), v21, v24, v27, v30, v33)
                    }
                }
            }
            _ => {
                let (mut v55, mut v56, mut v57, mut v58, mut v59, mut v60): (Rc<str>, bool, u64, u64, u64, u64) = match &v11 {
                    US0::US0_TableRejected(v47) => {
                        let mut v47: Rc<str> = v47.clone();
                        let mut v48: i32 = v2.wrapping_add(1i32);
                        tables_run_10(v0.clone(), v1.clone(), v48, v3.clone(), v4, v5, v6, v7, v8)
                    }
                    _ => unreachable!(),
                };
                (v55.clone(), v56, v57, v58, v59, v60)
            }
        };
        (v61.clone(), v62, v63, v64, v65, v66)
    } else {
        (v3.clone(), v4, v5, v6, v7, v8)
    };
    (v67.clone(), v68, v69, v70, v71, v72)
}
fn decimal_text_14(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1.clone()
}
fn receipt_counts_13(mut v0: Rc<str>, mut v1: bool, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64) -> Rc<str> {
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rows a "); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = decimal_text_14(v2);
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  b "); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = decimal_text_14(v3);
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v7, &*v8, &*v9, &*v10].concat());
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  changed "); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = decimal_text_14(v4);
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  added "); } LIT.with(|lit| lit.clone()) };
    let mut v15: u64 = v3.wrapping_sub(v5);
    let mut v16: Rc<str> = decimal_text_14(v15);
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  removed "); } LIT.with(|lit| lit.clone()) };
    let mut v18: u64 = v2.wrapping_sub(v5);
    let mut v19: Rc<str> = decimal_text_14(v18);
    let mut v20: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v16, &*v17, &*v19, &*v10].concat());
    let mut v21: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v12, &*v13, &*v20, &*v10].concat());
    v21.clone()
}
fn elapsed_text_16(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1.clone()
}
fn local_offset_minutes_18() -> i64 {
    let mut v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn clock_text_17() -> Rc<str> {
    let mut v0: i64 = local_offset_minutes_18();
    let mut v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1.clone()
}
fn receipt_line_15(mut v0: US2, mut v1: Rc<str>, mut v2: u64) -> () {
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
    let mut v8: Rc<str> = elapsed_text_16(v2);
    let mut v9: Rc<str> = clock_text_17();
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v7, &*v9, &*v10, &*v10].concat());
    let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v7, &*v1, &*v7, &*v11].concat());
    println!("{}", &*v12);
    ()
}
fn tables_program_run_0(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: u64 = now_ms_1();
    let mut v3: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = records_of_2(v1.clone());
    let mut v4: i32 = 1i32;
    let mut v5: Rc<str> = first_rejection_3(v3.clone(), v4);
    let mut v6: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v6 {
        let mut v7: i32 = 1i32;
        let (mut v8, mut v9, mut v10, mut v11, mut v12, mut v13): (Rc<str>, bool, u64, u64, u64, u64) = empty_diff_tally_9();
        let (mut v14, mut v15, mut v16, mut v17, mut v18, mut v19): (Rc<str>, bool, u64, u64, u64, u64) = tables_run_10(v0.clone(), v3.clone(), v7, v8.clone(), v9, v10, v11, v12, v13);
        let mut v20: bool = v14.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v20 {
            ()
        } else {
            println!("{}", &*v14);
            ()
        };
        if v15 {
            let mut v21: US2 = US2::US2_Failed;
            let mut v22: Rc<str> = receipt_counts_13(v14.clone(), v15, v16, v17, v18, v19);
            receipt_line_15(v21.clone(), v22.clone(), v2);
            1i32
        } else {
            let mut v23: US2 = US2::US2_Succeeded;
            let mut v24: Rc<str> = receipt_counts_13(v14.clone(), v15, v16, v17, v18, v19);
            receipt_line_15(v23.clone(), v24.clone(), v2);
            0i32
        }
    } else {
        let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("program rejected: "); } LIT.with(|lit| lit.clone()) };
        let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v28: Rc<str> = std::rc::Rc::<str>::from([&*v26, &*v5, &*v27, &*v27, &*v27].concat());
        println!("{}", &*v28);
        let mut v29: US2 = US2::US2_Failed;
        let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rows a 0  b 0"); } LIT.with(|lit| lit.clone()) };
        receipt_line_15(v29.clone(), v30.clone(), v2);
        2i32
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        tables_program_run_0(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_do_tables_run(v0: &str, v1: &str) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
