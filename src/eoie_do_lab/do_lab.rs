#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US1 {
    US1_LabAudit(Rc<str>, Rc<str>),
    US1_Blank,
}
#[derive(Clone)]
enum US0 {
    US0_LabParsed(US1),
    US0_LabRejected(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_Keep,
    US2_Orphan(Rc<str>),
    US2_DoneItem,
    US2_Stale(u64),
}
#[derive(Clone)]
enum US3 {
    US3_Succeeded,
    US3_Failed,
}
fn now_ms_1() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn records_of_2(mut v0: Rc<str>) -> std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> {
    let mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new((&*v0).split(char::from(30u8)).map(|record| record.split(char::from(31u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
    v1.clone()
}
fn field_at_5(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let mut v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).cloned().unwrap_or_default();
    v3.clone()
}
fn field_index_6(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: Rc<str>) -> i32 {
    let mut v3: i32 = v0.get(v1 as usize).and_then(|fields| fields.iter().position(|field| field.len() > 1 && field.starts_with(char::from(119u8)) && &field[1..] == &*v2)).map(|index| index as i32).unwrap_or(-1);
    v3
}
fn body_at_7(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let mut v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).filter(|field| !field.is_empty()).map(|field| std::rc::Rc::<str>::from(&field[1..])).unwrap_or_default();
    v3.clone()
}
fn lab_parsed_4(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let mut v2: i32 = 0i32;
    let mut v3: Rc<str> = field_at_5(v0.clone(), v1, v2);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("against"); } LIT.with(|lit| lit.clone()) };
    let mut v5: i32 = field_index_6(v0.clone(), v1, v4.clone());
    let mut v6: i32 = 3i32;
    let mut v7: Rc<str> = body_at_7(v0.clone(), v1, v6);
    let mut v8: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v8 {
        let mut v9: US1 = US1::US1_Blank;
        US0::US0_LabParsed(v9.clone())
    } else {
        let mut v11: bool = v3.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("query"); } LIT.with(|lit| lit.clone()) };
        let mut v15: bool = if v11 {
            true
        } else {
            let mut v12: i32 = 1i32;
            let mut v13: Rc<str> = field_at_5(v0.clone(), v1, v12);
            let mut v14: bool = v13.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lab"); } LIT.with(|lit| lit.clone()) };
            v14
        };
        if v15 {
            let mut v16: i32 = 1i32;
            let mut v17: Rc<str> = field_at_5(v0.clone(), v1, v16);
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cannot share one program with lab yet; run it as its own call"); } LIT.with(|lit| lit.clone()) };
            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v20: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v18, &*v19, &*v19, &*v19].concat());
            US0::US0_LabRejected(v20.clone())
        } else {
            let mut v22: i32 = 2i32;
            let mut v23: Rc<str> = body_at_7(v0.clone(), v1, v22);
            let mut v24: bool = v23.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("audit"); } LIT.with(|lit| lit.clone()) };
            if v24 {
                let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lab: expected lab audit <folder> against <backlog>"); } LIT.with(|lit| lit.clone()) };
                US0::US0_LabRejected(v25.clone())
            } else {
                let mut v27: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v29: bool = if v27 {
                    true
                } else {
                    let mut v28: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("against"); } LIT.with(|lit| lit.clone()) };
                    v28
                };
                if v29 {
                    let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lab audit: expected the lab folder"); } LIT.with(|lit| lit.clone()) };
                    US0::US0_LabRejected(v30.clone())
                } else {
                    let mut v32: bool = v5 < 0i32;
                    let mut v36: bool = if v32 {
                        true
                    } else {
                        let mut v33: i32 = v5.wrapping_add(1i32);
                        let mut v34: Rc<str> = body_at_7(v0.clone(), v1, v33);
                        let mut v35: bool = v34.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v35
                    };
                    if v36 {
                        let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lab audit: expected against <backlog root>"); } LIT.with(|lit| lit.clone()) };
                        US0::US0_LabRejected(v37.clone())
                    } else {
                        let mut v39: i32 = v5.wrapping_add(1i32);
                        let mut v40: Rc<str> = body_at_7(v0.clone(), v1, v39);
                        let mut v41: US1 = US1::US1_LabAudit(v7.clone(), v40.clone());
                        US0::US0_LabParsed(v41.clone())
                    }
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
            let mut v4: US0 = lab_parsed_4(v0.clone(), v1);
            match &v4 {
                US0::US0_LabParsed(v6) => {
                    let mut v6: US1 = v6.clone();
                    let mut v7: i32 = v1.wrapping_add(1i32);
                    (v0, v1) = (v0.clone(), v7);
                    continue;
                }
                US0::US0_LabRejected(v5) => {
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
fn empty_lab_tally_8() -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0.clone(), false, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64)
}
fn backlog_items_12(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { let listed = std::env::current_exe().and_then(|program| std::process::Command::new(program).current_dir(&*v0).args(["agile", "list", &*v1, "all", "--json"]).stdin(std::process::Stdio::null()).output()); std::rc::Rc::<str>::from(match listed { Ok(output) if output.status.success() => serde_json::from_slice::<serde_json::Value>(&output.stdout).ok().and_then(|value| value.get("tasks").and_then(|tasks| tasks.as_array()).map(|tasks| tasks.iter().map(|task| format!("{}{tab}{}{tab}{}{tab}{}", task["id"].as_str().unwrap_or(""), task["status"].as_str().unwrap_or(""), task["progress"].as_u64().unwrap_or(0), task["last_note"].as_str().unwrap_or(""), tab = char::from(9u8))).collect::<Vec<_>>().join("
"))).unwrap_or_else(|| String::from("error: agile list printed no tasks")), Ok(output) => format!("error: {}", String::from_utf8_lossy(&output.stderr).trim().trim_start_matches("eoie error: ")), Err(error) => format!("error: agile list: {error}") }) };
    v2.clone()
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
fn lab_directories_14(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { let mut names = std::fs::read_dir(std::path::Path::new(&*v0).join(&*v1)).into_iter().flatten().flatten().filter(|entry| entry.path().is_dir()).map(|entry| entry.file_name().to_string_lossy().into_owned()).collect::<Vec<_>>(); names.sort(); std::rc::Rc::<str>::from(names.join("
")) };
    v2.clone()
}
fn row_15(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(char::from(31u8).to_string());
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v2, &*v1, &*v3, &*v3].concat());
    v4.clone()
}
fn line_count_17(mut v0: Rc<str>) -> u64 {
    let mut v1: u64 = (v0).lines().filter(|line| !line.is_empty()).count() as u64;
    v1
}
fn line_at_18(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from((v0).lines().filter(|line| !line.is_empty()).nth(v1 as usize).unwrap_or(""));
    v2.clone()
}
fn item_id_of_19(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_uppercase().replace(char::from(95u8), "-"));
    v1.clone()
}
fn item_of_20(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from((v0).lines().find(|line| line.split(char::from(9u8)).next() == Some(&*v1)).unwrap_or(""));
    v2.clone()
}
fn tab_part_21(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(9u8)).nth(v1 as usize).unwrap_or(""));
    v2.clone()
}
fn note_age_days_22(mut v0: Rc<str>) -> u64 {
    let mut v1: u64 = { let stamp = (&*v0).trim(); let digits = |from: usize, to: usize| stamp.get(from..to).and_then(|part| part.parse::<i64>().ok()); match (digits(0, 4), digits(5, 7), digits(8, 10)) { (Some(year), Some(month), Some(day)) => { let shifted = if month <= 2 { year - 1 } else { year }; let era = shifted.div_euclid(400); let of_era = shifted - era * 400; let of_year = (153 * (if month > 2 { month - 3 } else { month + 9 }) + 2) / 5 + day - 1; let days = era * 146097 + of_era * 365 + of_era / 4 - of_era / 100 + of_year - 719468; let today = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs() as i64 / 86400).unwrap_or(0); (today - days).max(0) as u64 } _ => 0 } };
    v1
}
fn verdict_of_23(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64) -> US2 {
    let mut v3: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v3 {
        US2::US2_Orphan(v1.clone())
    } else {
        let mut v5: i32 = 1i32;
        let mut v6: Rc<str> = tab_part_21(v0.clone(), v5);
        let mut v7: bool = v6.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Done"); } LIT.with(|lit| lit.clone()) };
        if v7 {
            US2::US2_DoneItem
        } else {
            let mut v9: bool = v2 > 14u64;
            if v9 {
                US2::US2_Stale(v2)
            } else {
                US2::US2_Keep
            }
        }
    }
}
fn decimal_text_25(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1.clone()
}
fn verdict_text_24(mut v0: US2) -> Rc<str> {
    match &v0 {
        US2::US2_DoneItem => {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("done: promote or delete"); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        }
        US2::US2_Keep => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("keep"); } LIT.with(|lit| lit.clone()) };
            v1.clone()
        }
        US2::US2_Orphan(v2) => {
            let mut v2: Rc<str> = v2.clone();
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("orphan: no backlog item "); } LIT.with(|lit| lit.clone()) };
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v5: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v2, &*v4, &*v4, &*v4].concat());
            v5.clone()
        }
        US2::US2_Stale(v7) => {
            let mut v7: u64 = *v7;
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stale: no note for "); } LIT.with(|lit| lit.clone()) };
            let mut v9: Rc<str> = decimal_text_25(v7);
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" days"); } LIT.with(|lit| lit.clone()) };
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v9, &*v10, &*v11, &*v11].concat());
            v12.clone()
        }
    }
}
fn counted_26(mut v0: Rc<str>, mut v1: bool, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: US2) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) {
    let (mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = match &v9 {
        US2::US2_Keep => {
            let mut v10: u64 = v2.wrapping_add(1u64);
            let mut v11: u64 = v3.wrapping_add(1u64);
            (v0.clone(), v1, v10, v11, v4, v5, v6, v7, v8)
        }
        _ => {
            let (mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = match &v9 {
                US2::US2_Orphan(v12) => {
                    let mut v12: Rc<str> = v12.clone();
                    let mut v13: u64 = v2.wrapping_add(1u64);
                    let mut v14: u64 = v4.wrapping_add(1u64);
                    (v0.clone(), v1, v13, v3, v14, v5, v6, v7, v8)
                }
                _ => {
                    let (mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = match &v9 {
                        US2::US2_DoneItem => {
                            let mut v15: u64 = v2.wrapping_add(1u64);
                            let mut v16: u64 = v5.wrapping_add(1u64);
                            (v0.clone(), v1, v15, v3, v4, v16, v6, v7, v8)
                        }
                        _ => {
                            let (mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = match &v9 {
                                US2::US2_Stale(v17) => {
                                    let mut v17: u64 = *v17;
                                    let mut v18: u64 = v2.wrapping_add(1u64);
                                    let mut v19: u64 = v6.wrapping_add(1u64);
                                    (v0.clone(), v1, v18, v3, v4, v5, v19, v7, v8)
                                }
                                _ => unreachable!(),
                            };
                            (v20.clone(), v21, v22, v23, v24, v25, v26, v27, v28)
                        }
                    };
                    (v29.clone(), v30, v31, v32, v33, v34, v35, v36, v37)
                }
            };
            (v38.clone(), v39, v40, v41, v42, v43, v44, v45, v46)
        }
    };
    (v47.clone(), v48, v49, v50, v51, v52, v53, v54, v55)
}
fn aligned_27(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { let separator = char::from(31u8); let rows = (&*v0).lines().collect::<Vec<_>>(); let mut widths: Vec<usize> = Vec::new(); for row in &rows { if row.contains(separator) { let cells = row.split(separator).collect::<Vec<_>>(); for (index, cell) in cells[..cells.len() - 1].iter().enumerate() { let length = cell.chars().count(); if widths.len() <= index { widths.push(length); } else if widths[index] < length { widths[index] = length; } } } } let shown = rows.iter().map(|row| if row.contains(separator) { let cells = row.split(separator).collect::<Vec<_>>(); let last = cells.len() - 1; cells.iter().enumerate().map(|(index, cell)| if index == last { cell.to_string() } else { format!("{:<width$}  ", cell, width = widths[index]) }).collect::<String>() } else { row.to_string() }).collect::<Vec<_>>(); std::rc::Rc::<str>::from(shown.join("
")) };
    v1.clone()
}
fn labs_rows_16(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: bool, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) {
    let mut v15: u64 = line_count_17(v3.clone());
    let mut v16: bool = v4 < v15;
    let (mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73, mut v74): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = if v16 {
        let mut v17: Rc<str> = line_at_18(v3.clone(), v4);
        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v20: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v18, &*v17, &*v19, &*v19].concat());
        let mut v21: Rc<str> = item_id_of_19(v17.clone());
        let mut v22: Rc<str> = item_of_20(v2.clone(), v21.clone());
        let mut v23: i32 = 3i32;
        let mut v24: Rc<str> = tab_part_21(v22.clone(), v23);
        let mut v25: u64 = note_age_days_22(v24.clone());
        let mut v26: US2 = verdict_of_23(v22.clone(), v21.clone(), v25);
        let mut v27: bool = v22.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v43: Rc<str> = if v27 {
            let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let mut v29: Rc<str> = verdict_text_24(v26.clone());
            let mut v30: Rc<str> = row_15(v28.clone(), v29.clone());
            let mut v31: Rc<str> = row_15(v28.clone(), v30.clone());
            let mut v32: Rc<str> = row_15(v28.clone(), v31.clone());
            row_15(v20.clone(), v32.clone())
        } else {
            let mut v34: i32 = 1i32;
            let mut v35: Rc<str> = tab_part_21(v22.clone(), v34);
            let mut v36: i32 = 2i32;
            let mut v37: Rc<str> = tab_part_21(v22.clone(), v36);
            let mut v38: Rc<str> = verdict_text_24(v26.clone());
            let mut v39: Rc<str> = row_15(v37.clone(), v38.clone());
            let mut v40: Rc<str> = row_15(v35.clone(), v39.clone());
            let mut v41: Rc<str> = row_15(v21.clone(), v40.clone());
            row_15(v20.clone(), v41.clone())
        };
        let mut v44: u64 = v4.wrapping_add(1u64);
        let mut v45: Rc<str> = lines_joined_13(v5.clone(), v43.clone());
        let (mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = counted_26(v6.clone(), v7, v8, v9, v10, v11, v12, v13, v14, v26.clone());
        let (mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = labs_rows_16(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v44, v45.clone(), v46.clone(), v47, v48, v49, v50, v51, v52, v53, v54);
        (v55.clone(), v56, v57, v58, v59, v60, v61, v62, v63)
    } else {
        let mut v64: Rc<str> = aligned_27(v5.clone());
        let mut v65: Rc<str> = lines_joined_13(v6.clone(), v64.clone());
        (v65.clone(), v7, v8, v9, v10, v11, v12, v13, v14)
    };
    (v66.clone(), v67, v68, v69, v70, v71, v72, v73, v74)
}
fn non_portable_28(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { let (b, q, a) = (char::from(92u8), char::from(34u8), char::from(39u8)); let pattern = regex::Regex::new(&format!("[A-Za-z]:[{b}{b}/]Users[{b}{b}/][^{b}s{q}{a}<>|]+|(?:^|[{b}s{q}{a}=(])/home/[a-z_][a-z0-9_-]*/[^{b}s{q}{a}<>|]+")).unwrap(); fn files(path: &std::path::Path, found: &mut Vec<std::path::PathBuf>) { if path.is_dir() { let mut entries = std::fs::read_dir(path).into_iter().flatten().flatten().map(|entry| entry.path()).collect::<Vec<_>>(); entries.sort(); for entry in entries { files(&entry, found); } } else { found.push(path.to_path_buf()); } } let root = std::path::Path::new(&*v0); let mut found = Vec::new(); files(&root.join(&*v1), &mut found); let mut rows = Vec::new(); for path in found { let text = String::from_utf8_lossy(&std::fs::read(&path).unwrap_or_default()).into_owned(); let shown = path.strip_prefix(root).unwrap_or(&path).to_string_lossy().replace(b, "/"); for (index, line) in text.lines().enumerate() { for hit in pattern.find_iter(line) { let text = hit.as_str(); let text = if text.get(1..2) == Some(":") { text } else { text.trim_start_matches(|c: char| c != char::from(47u8)) }; rows.push(format!("{}:{}  non-portable path {}", shown, index + 1, text)); } } } std::rc::Rc::<str>::from(rows.join("
")) };
    v2.clone()
}
fn ignored_paths_29(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { let marker = format!("{0}{0} ", char::from(33u8)); let listed = std::process::Command::new("git").current_dir(&*v0).args(["status", "--porcelain", "--ignored=matching", "--untracked-files=all", "--"]).arg(&*v1).stdin(std::process::Stdio::null()).output(); std::rc::Rc::<str>::from(match listed { Ok(output) if output.status.success() => String::from_utf8_lossy(&output.stdout).lines().filter_map(|line| line.strip_prefix(marker.as_str())).map(|path| path.trim_matches(char::from(34u8)).trim_end_matches(char::from(47u8)).to_owned()).filter(|path| !path.split(char::from(47u8)).any(|part| part == ".runs")).map(|path| format!("{path}  ignored by .gitignore (run output outside .runs)")).collect::<Vec<_>>().join("
"), _ => String::new() }) };
    v2.clone()
}
fn audit_run_11(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) {
    let mut v12: Rc<str> = backlog_items_12(v0.clone(), v2.clone());
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: "); } LIT.with(|lit| lit.clone()) };
    let mut v14: bool = v12.starts_with(&*v13);
    let (mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = if v14 {
        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lab audit: "); } LIT.with(|lit| lit.clone()) };
        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v12, &*v16, &*v16, &*v16].concat());
        let mut v18: Rc<str> = lines_joined_13(v3.clone(), v17.clone());
        (v18.clone(), true, v5, v6, v7, v8, v9, v10, v11)
    } else {
        let mut v19: Rc<str> = lab_directories_14(v0.clone(), v1.clone());
        let mut v20: u64 = 0u64;
        let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lab"); } LIT.with(|lit| lit.clone()) };
        let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("item"); } LIT.with(|lit| lit.clone()) };
        let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
        let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("progress"); } LIT.with(|lit| lit.clone()) };
        let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("verdict"); } LIT.with(|lit| lit.clone()) };
        let mut v26: Rc<str> = row_15(v24.clone(), v25.clone());
        let mut v27: Rc<str> = row_15(v23.clone(), v26.clone());
        let mut v28: Rc<str> = row_15(v22.clone(), v27.clone());
        let mut v29: Rc<str> = row_15(v21.clone(), v28.clone());
        let (mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = labs_rows_16(v0.clone(), v1.clone(), v12.clone(), v19.clone(), v20, v29.clone(), v3.clone(), v4, v5, v6, v7, v8, v9, v10, v11);
        let mut v39: Rc<str> = non_portable_28(v0.clone(), v1.clone());
        let mut v40: Rc<str> = ignored_paths_29(v0.clone(), v1.clone());
        let mut v41: Rc<str> = lines_joined_13(v30.clone(), v39.clone());
        let mut v42: Rc<str> = lines_joined_13(v41.clone(), v40.clone());
        let mut v43: u64 = line_count_17(v39.clone());
        let mut v44: u64 = v37.wrapping_add(v43);
        let mut v45: u64 = line_count_17(v40.clone());
        let mut v46: u64 = v38.wrapping_add(v45);
        (v42.clone(), v31, v32, v33, v34, v35, v36, v44, v46)
    };
    (v47.clone(), v48, v49, v50, v51, v52, v53, v54, v55)
}
fn step_run_10(mut v0: Rc<str>, mut v1: US1, mut v2: Rc<str>, mut v3: bool, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) {
    let (mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = match &v1 {
        US1::US1_LabAudit(v11, v12) => {
            let mut v11: Rc<str> = v11.clone();
            let mut v12: Rc<str> = v12.clone();
            audit_run_11(v0.clone(), v11.clone(), v12.clone(), v2.clone(), v3, v4, v5, v6, v7, v8, v9, v10)
        }
        _ => {
            let (mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = match &v1 {
                US1::US1_Blank => {
                    (v2.clone(), v3, v4, v5, v6, v7, v8, v9, v10)
                }
                _ => unreachable!(),
            };
            (v22.clone(), v23, v24, v25, v26, v27, v28, v29, v30)
        }
    };
    (v31.clone(), v32, v33, v34, v35, v36, v37, v38, v39)
}
fn labs_run_9(mut v0: Rc<str>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: i32, mut v3: Rc<str>, mut v4: bool, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) {
    let mut v12: i32 = v1.len() as i32;
    let mut v13: bool = v2 < v12;
    let (mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = if v13 {
        let mut v14: US0 = lab_parsed_4(v1.clone(), v2);
        let (mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = match &v14 {
            US0::US0_LabParsed(v15) => {
                let mut v15: US1 = v15.clone();
                let mut v16: i32 = v2.wrapping_add(1i32);
                let (mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = step_run_10(v0.clone(), v15.clone(), v3.clone(), v4, v5, v6, v7, v8, v9, v10, v11);
                labs_run_9(v0.clone(), v1.clone(), v16, v17.clone(), v18, v19, v20, v21, v22, v23, v24, v25)
            }
            _ => {
                let (mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = match &v14 {
                    US0::US0_LabRejected(v35) => {
                        let mut v35: Rc<str> = v35.clone();
                        let mut v36: i32 = v2.wrapping_add(1i32);
                        labs_run_9(v0.clone(), v1.clone(), v36, v3.clone(), v4, v5, v6, v7, v8, v9, v10, v11)
                    }
                    _ => unreachable!(),
                };
                (v46.clone(), v47, v48, v49, v50, v51, v52, v53, v54)
            }
        };
        (v55.clone(), v56, v57, v58, v59, v60, v61, v62, v63)
    } else {
        (v3.clone(), v4, v5, v6, v7, v8, v9, v10, v11)
    };
    (v64.clone(), v65, v66, v67, v68, v69, v70, v71, v72)
}
fn receipt_counts_30(mut v0: Rc<str>, mut v1: bool, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64) -> Rc<str> {
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("labs "); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = decimal_text_25(v2);
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  keep "); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = decimal_text_25(v3);
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  orphan "); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = decimal_text_25(v4);
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  done "); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = decimal_text_25(v5);
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v14, &*v15, &*v16, &*v17].concat());
    let mut v19: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v10, &*v11, &*v12, &*v18].concat());
    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  stale "); } LIT.with(|lit| lit.clone()) };
    let mut v21: Rc<str> = decimal_text_25(v6);
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  non-portable "); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = decimal_text_25(v7);
    let mut v24: Rc<str> = std::rc::Rc::<str>::from([&*v19, &*v20, &*v21, &*v22, &*v23].concat());
    let mut v25: bool = v8 == 0u64;
    if v25 {
        v24.clone()
    } else {
        let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  ignored "); } LIT.with(|lit| lit.clone()) };
        let mut v27: Rc<str> = decimal_text_25(v8);
        let mut v28: Rc<str> = std::rc::Rc::<str>::from([&*v24, &*v26, &*v27, &*v17, &*v17].concat());
        v28.clone()
    }
}
fn elapsed_text_32(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1.clone()
}
fn local_offset_minutes_34() -> i64 {
    let mut v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn clock_text_33() -> Rc<str> {
    let mut v0: i64 = local_offset_minutes_34();
    let mut v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1.clone()
}
fn receipt_line_31(mut v0: US3, mut v1: Rc<str>, mut v2: u64) -> () {
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
    let mut v8: Rc<str> = elapsed_text_32(v2);
    let mut v9: Rc<str> = clock_text_33();
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v7, &*v9, &*v10, &*v10].concat());
    let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v7, &*v1, &*v7, &*v11].concat());
    println!("{}", &*v12);
    ()
}
fn lab_program_run_0(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: u64 = now_ms_1();
    let mut v3: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = records_of_2(v1.clone());
    let mut v4: i32 = 1i32;
    let mut v5: Rc<str> = first_rejection_3(v3.clone(), v4);
    let mut v6: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v6 {
        let mut v7: i32 = 1i32;
        let (mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = empty_lab_tally_8();
        let (mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = labs_run_9(v0.clone(), v3.clone(), v7, v8.clone(), v9, v10, v11, v12, v13, v14, v15, v16);
        let mut v26: bool = v17.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v26 {
            ()
        } else {
            println!("{}", &*v17);
            ()
        };
        let mut v32: bool = if v18 {
            true
        } else {
            let mut v27: u64 = v21.wrapping_add(v22);
            let mut v28: u64 = v27.wrapping_add(v23);
            let mut v29: u64 = v28.wrapping_add(v24);
            let mut v30: u64 = v29.wrapping_add(v25);
            let mut v31: bool = v30 > 0u64;
            v31
        };
        if v32 {
            let mut v33: US3 = US3::US3_Failed;
            let mut v34: Rc<str> = receipt_counts_30(v17.clone(), v18, v19, v20, v21, v22, v23, v24, v25);
            receipt_line_31(v33.clone(), v34.clone(), v2);
            1i32
        } else {
            let mut v35: US3 = US3::US3_Succeeded;
            let mut v36: Rc<str> = receipt_counts_30(v17.clone(), v18, v19, v20, v21, v22, v23, v24, v25);
            receipt_line_31(v35.clone(), v36.clone(), v2);
            0i32
        }
    } else {
        let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("program rejected: "); } LIT.with(|lit| lit.clone()) };
        let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v40: Rc<str> = std::rc::Rc::<str>::from([&*v38, &*v5, &*v39, &*v39, &*v39].concat());
        println!("{}", &*v40);
        let mut v41: US3 = US3::US3_Failed;
        let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("labs 0"); } LIT.with(|lit| lit.clone()) };
        receipt_line_31(v41.clone(), v42.clone(), v2);
        2i32
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        lab_program_run_0(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_do_lab_run(v0: &str, v1: &str) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
