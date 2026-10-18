#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_AllStatuses,
    US0_OnlyStatus(Rc<str>),
}
#[derive(Clone)]
enum US1 {
    US1_TableFormat,
    US1_JsonFormat,
}
#[derive(Clone)]
enum US2 {
    US2_ScanPlain,
    US2_ScanQuoted,
    US2_ScanEscaped,
}
#[derive(Clone)]
enum US3 {
    US3_FileReadable(Rc<str>),
    US3_FileMissing(Rc<str>),
}
#[derive(Clone)]
enum US4 {
    US4_ListParsed(US0, Rc<str>, Rc<str>, US1, std::rc::Rc<std::collections::HashMap<String, (u32, String)>>),
    US4_ListRejected(Rc<str>),
}
#[derive(Clone)]
enum US5 {
    US5_Valid(US0),
    US5_Invalid,
}
fn field_end_7(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US2) -> i32 {
    loop {
        let mut v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let mut v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = match &v3 {
                US2::US2_ScanPlain => {
                    let mut v6: bool = v5 == 44i32;
                    if v6 {
                        true
                    } else {
                        let mut v7: bool = v5 == 41i32;
                        v7
                    }
                }
                _ => {
                    false
                }
            };
            if v9 {
                return v1;
            } else {
                let mut v10: i32 = v1.wrapping_add(1i32);
                let mut v25: US2 = match &v3 {
                    US2::US2_ScanEscaped => {
                        US2::US2_ScanQuoted
                    }
                    US2::US2_ScanPlain => {
                        let mut v19: bool = v5 == 34i32;
                        if v19 {
                            US2::US2_ScanQuoted
                        } else {
                            US2::US2_ScanPlain
                        }
                    }
                    US2::US2_ScanQuoted => {
                        let mut v12: bool = v5 == 92i32;
                        if v12 {
                            US2::US2_ScanEscaped
                        } else {
                            let mut v14: bool = v5 == 34i32;
                            if v14 {
                                US2::US2_ScanPlain
                            } else {
                                US2::US2_ScanQuoted
                            }
                        }
                    }
                };
                (v0, v1, v2, v3) = (v0.clone(), v10, v2, v25.clone());
                continue;
            }
        }
    }
}
fn skip_spaces_8(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
        let mut v3: bool = v2 == 32i32;
        if v3 {
            let mut v4: i32 = v1.wrapping_add(1i32);
            (v0, v1) = (v0.clone(), v4);
            continue;
        } else {
            return v1;
        }
    }
}
fn field_start_6(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let mut v5: US2 = US2::US2_ScanPlain;
            let mut v6: i32 = field_end_7(v0.clone(), v1, v2, v5.clone());
            let mut v7: i32 = v6.wrapping_add(1i32);
            let mut v8: i32 = skip_spaces_8(v0.clone(), v7);
            let mut v9: i32 = v3.wrapping_sub(1i32);
            (v0, v1, v2, v3) = (v0.clone(), v8, v2, v9);
            continue;
        }
    }
}
fn task_row_at_5(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let mut v3: i32 = 0i32;
    let mut v4: i32 = field_start_6(v0.clone(), v1, v2, v3);
    let mut v5: US2 = US2::US2_ScanPlain;
    let mut v6: i32 = field_end_7(v0.clone(), v4, v2, v5.clone());
    let mut v7: Rc<str> = std::rc::Rc::<str>::from(v0.get(v4 as usize..v6 as usize).unwrap_or(""));
    let mut v8: i32 = v7.len() as i32;
    let mut v9: i32 = v8.wrapping_sub(1i32);
    let mut v10: Rc<str> = std::rc::Rc::<str>::from(v7.get(1i32 as usize..v9 as usize).unwrap_or(""));
    let mut v11: i32 = 1i32;
    let mut v12: i32 = field_start_6(v0.clone(), v1, v2, v11);
    let mut v13: US2 = US2::US2_ScanPlain;
    let mut v14: i32 = field_end_7(v0.clone(), v12, v2, v13.clone());
    let mut v15: Rc<str> = std::rc::Rc::<str>::from(v0.get(v12 as usize..v14 as usize).unwrap_or(""));
    let mut v16: i32 = v15.len() as i32;
    let mut v17: i32 = v16.wrapping_sub(1i32);
    let mut v18: Rc<str> = std::rc::Rc::<str>::from(v15.get(1i32 as usize..v17 as usize).unwrap_or(""));
    let mut v19: i32 = 2i32;
    let mut v20: i32 = field_start_6(v0.clone(), v1, v2, v19);
    let mut v21: US2 = US2::US2_ScanPlain;
    let mut v22: i32 = field_end_7(v0.clone(), v20, v2, v21.clone());
    let mut v23: Rc<str> = std::rc::Rc::<str>::from(v0.get(v20 as usize..v22 as usize).unwrap_or(""));
    let mut v24: i32 = 3i32;
    let mut v25: i32 = field_start_6(v0.clone(), v1, v2, v24);
    let mut v26: US2 = US2::US2_ScanPlain;
    let mut v27: i32 = field_end_7(v0.clone(), v25, v2, v26.clone());
    let mut v28: Rc<str> = std::rc::Rc::<str>::from(v0.get(v25 as usize..v27 as usize).unwrap_or(""));
    let mut v29: i32 = 4i32;
    let mut v30: i32 = field_start_6(v0.clone(), v1, v2, v29);
    let mut v31: US2 = US2::US2_ScanPlain;
    let mut v32: i32 = field_end_7(v0.clone(), v30, v2, v31.clone());
    let mut v33: Rc<str> = std::rc::Rc::<str>::from(v0.get(v30 as usize..v32 as usize).unwrap_or(""));
    let mut v34: i32 = 5i32;
    let mut v35: i32 = field_start_6(v0.clone(), v1, v2, v34);
    let mut v36: US2 = US2::US2_ScanPlain;
    let mut v37: i32 = field_end_7(v0.clone(), v35, v2, v36.clone());
    let mut v38: Rc<str> = std::rc::Rc::<str>::from(v0.get(v35 as usize..v37 as usize).unwrap_or(""));
    let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v41: Rc<str> = std::rc::Rc::<str>::from(v38.replace(&*v39, &v40));
    let mut v42: i32 = 6i32;
    let mut v43: i32 = field_start_6(v0.clone(), v1, v2, v42);
    let mut v44: US2 = US2::US2_ScanPlain;
    let mut v45: i32 = field_end_7(v0.clone(), v43, v2, v44.clone());
    let mut v46: Rc<str> = std::rc::Rc::<str>::from(v0.get(v43 as usize..v45 as usize).unwrap_or(""));
    let mut v47: i32 = 7i32;
    let mut v48: i32 = field_start_6(v0.clone(), v1, v2, v47);
    let mut v49: US2 = US2::US2_ScanPlain;
    let mut v50: i32 = field_end_7(v0.clone(), v48, v2, v49.clone());
    let mut v51: Rc<str> = std::rc::Rc::<str>::from(v0.get(v48 as usize..v50 as usize).unwrap_or(""));
    let mut v52: i32 = v51.len() as i32;
    let mut v53: i32 = v52.wrapping_sub(1i32);
    let mut v54: Rc<str> = std::rc::Rc::<str>::from(v51.get(1i32 as usize..v53 as usize).unwrap_or(""));
    let mut v55: i32 = 8i32;
    let mut v56: i32 = field_start_6(v0.clone(), v1, v2, v55);
    let mut v57: US2 = US2::US2_ScanPlain;
    let mut v58: i32 = field_end_7(v0.clone(), v56, v2, v57.clone());
    let mut v59: Rc<str> = std::rc::Rc::<str>::from(v0.get(v56 as usize..v58 as usize).unwrap_or(""));
    let mut v60: i32 = v59.len() as i32;
    let mut v61: i32 = v60.wrapping_sub(1i32);
    let mut v62: Rc<str> = std::rc::Rc::<str>::from(v59.get(1i32 as usize..v61 as usize).unwrap_or(""));
    (v10.clone(), v18.clone(), v23.clone(), v28.clone(), v33.clone(), v41.clone(), v46.clone(), v54.clone(), v62.clone())
}
fn row_selected_9(mut v0: Rc<str>, mut v1: US0, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: US1, mut v5: std::rc::Rc<std::collections::HashMap<String, (u32, String)>>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: Rc<str>, mut v12: Rc<str>, mut v13: Rc<str>, mut v14: Rc<str>) -> bool {
    let mut v18: bool = match &v1 {
        US0::US0_AllStatuses => {
            true
        }
        US0::US0_OnlyStatus(v15) => {
            let mut v15: Rc<str> = v15.clone();
            let mut v16: bool = v15 == v12 ;
            v16
        }
    };
    if v18 {
        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
        let mut v20: bool = v2 == v19 ;
        if v20 {
            let mut v21: bool = v3 == v19 ;
            if v21 {
                true
            } else {
                let mut v22: bool = agile_planning_filters::eoie_agile_planning_selected(&*v0, &*v6, &*v2, &*v3);
                v22
            }
        } else {
            let mut v24: bool = agile_planning_filters::eoie_agile_planning_selected(&*v0, &*v6, &*v2, &*v3);
            v24
        }
    } else {
        false
    }
}
fn json_line_11(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: std::rc::Rc<std::collections::HashMap<String, (u32, String)>>, mut v11: bool) -> Rc<str> {
    let mut v14: Rc<str> = if v11 {
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v12.clone()
    } else {
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        v13.clone()
    };
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{"); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("id"); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v19: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v0, &*v17, &*v18, &*v18].concat());
    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\":"); } LIT.with(|lit| lit.clone()) };
    let mut v21: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v16, &*v20, &*v19, &*v18].concat());
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("title"); } LIT.with(|lit| lit.clone()) };
    let mut v24: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v1, &*v17, &*v18, &*v18].concat());
    let mut v25: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v23, &*v20, &*v24, &*v18].concat());
    let mut v26: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v15, &*v21, &*v22, &*v25].concat());
    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kind"); } LIT.with(|lit| lit.clone()) };
    let mut v28: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v2, &*v17, &*v18, &*v18].concat());
    let mut v29: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v27, &*v20, &*v28, &*v18].concat());
    let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("priority"); } LIT.with(|lit| lit.clone()) };
    let mut v31: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v3, &*v17, &*v18, &*v18].concat());
    let mut v32: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v30, &*v20, &*v31, &*v18].concat());
    let mut v33: Rc<str> = std::rc::Rc::<str>::from([&*v22, &*v29, &*v22, &*v32, &*v22].concat());
    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("difficulty"); } LIT.with(|lit| lit.clone()) };
    let mut v35: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v4, &*v17, &*v18, &*v18].concat());
    let mut v36: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v34, &*v20, &*v35, &*v18].concat());
    let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("progress"); } LIT.with(|lit| lit.clone()) };
    let mut v38: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v37, &*v20, &*v5, &*v18].concat());
    let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
    let mut v40: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v6, &*v17, &*v18, &*v18].concat());
    let mut v41: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v39, &*v20, &*v40, &*v18].concat());
    let mut v42: Rc<str> = std::rc::Rc::<str>::from([&*v36, &*v22, &*v38, &*v22, &*v41].concat());
    let mut v43: u32 = v10.get(&*v0).map(|entry| entry.0).unwrap_or(0);
    let mut v44: Rc<str> = std::rc::Rc::<str>::from((v43).to_string());
    let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("deps"); } LIT.with(|lit| lit.clone()) };
    let mut v46: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v7, &*v17, &*v18, &*v18].concat());
    let mut v47: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v45, &*v20, &*v46, &*v18].concat());
    let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tags"); } LIT.with(|lit| lit.clone()) };
    let mut v49: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v8, &*v17, &*v18, &*v18].concat());
    let mut v50: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v48, &*v20, &*v49, &*v18].concat());
    let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("notes"); } LIT.with(|lit| lit.clone()) };
    let mut v52: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v51, &*v20, &*v44, &*v18].concat());
    let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("last_note"); } LIT.with(|lit| lit.clone()) };
    let mut v54: Rc<str> = std::rc::Rc::<str>::from(v10.get(&*v0).map(|entry| entry.1.as_str()).unwrap_or(""));
    let mut v55: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v54, &*v17, &*v18, &*v18].concat());
    let mut v56: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v53, &*v20, &*v55, &*v18].concat());
    let mut v57: Rc<str> = std::rc::Rc::<str>::from([&*v52, &*v22, &*v56, &*v22, &*v18].concat());
    let mut v58: Rc<str> = std::rc::Rc::<str>::from([&*v47, &*v22, &*v50, &*v22, &*v57].concat());
    let mut v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
    let mut v60: Rc<str> = std::rc::Rc::<str>::from([&*v58, &*v9, &*v59, &*v18, &*v18].concat());
    let mut v61: Rc<str> = std::rc::Rc::<str>::from([&*v26, &*v33, &*v42, &*v22, &*v60].concat());
    v61.clone()
}
fn table_line_12(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: Rc<str>) -> Rc<str> {
    let mut v10: Rc<str> = { let text = v0; let filler = (26i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v11, &*v3, &*v11, &*v4].concat());
    let mut v13: Rc<str> = { let text = v5; let filler = (4i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(" ".repeat(filler) + &*text) };
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = { let text = v6; let filler = (8i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
    let mut v16: Rc<str> = { let text = v2; let filler = (9i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
    let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v13, &*v14, &*v15, &*v16].concat());
    let mut v18: Rc<str> = { let text = v1; if text.chars().count() <= 96i32 as usize { text } else { std::rc::Rc::<str>::from(text.chars().take((96i32 as usize).saturating_sub(3)).collect::<String>() + "...") } };
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v17, &*v9, &*v18, &*v19].concat());
    v20.clone()
}
fn row_line_10(mut v0: Rc<str>, mut v1: US0, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: US1, mut v5: std::rc::Rc<std::collections::HashMap<String, (u32, String)>>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: Rc<str>, mut v12: Rc<str>, mut v13: Rc<str>, mut v14: Rc<str>, mut v15: bool) -> Rc<str> {
    match &v4 {
        US1::US1_JsonFormat => {
            let mut v18: Rc<str> = agile_planning_fields::eoie_agile_planning_json(&*v0, &*v6);
            json_line_11(v6.clone(), v7.clone(), v8.clone(), v9.clone(), v10.clone(), v11.clone(), v12.clone(), v13.clone(), v14.clone(), v18.clone(), v5.clone(), v15)
        }
        US1::US1_TableFormat => {
            let mut v16: Rc<str> = agile_planning_fields::eoie_agile_planning_columns(&*v0, &*v6);
            table_line_12(v6.clone(), v7.clone(), v8.clone(), v9.clone(), v10.clone(), v11.clone(), v12.clone(), v13.clone(), v14.clone(), v16.clone())
        }
    }
}
fn print_priority_rows_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>, mut v4: US0, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: US1, mut v8: std::rc::Rc<std::collections::HashMap<String, (u32, String)>>, mut v9: i32, mut v10: i32) -> i32 {
    loop {
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
        let mut v12: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v11)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let mut v13: bool = v12 < 0i32;
        if v13 {
            return v10;
        } else {
            let mut v14: i32 = v12 + 6i32;
            let (mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = task_row_at_5(v0.clone(), v14, v2);
            let mut v24: i32 = v0.get(..v12 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let mut v25: Rc<str> = std::rc::Rc::<str>::from(v0.get(v24 as usize..v12 as usize).unwrap_or(""));
            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let mut v27: bool = v25.starts_with(&*v26);
            if v27 {
                let mut v28: bool = v18 == v3 ;
                if v28 {
                    let mut v29: bool = row_selected_9(v0.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v15.clone(), v16.clone(), v17.clone(), v18.clone(), v19.clone(), v20.clone(), v21.clone(), v22.clone(), v23.clone());
                    if v29 {
                        let mut v30: i32 = v9 + v10;
                        let mut v31: bool = v30 == 0i32;
                        let mut v32: Rc<str> = row_line_10(v0.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v15.clone(), v16.clone(), v17.clone(), v18.clone(), v19.clone(), v20.clone(), v21.clone(), v22.clone(), v23.clone(), v31);
                        let mut v33: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v32.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                        let mut v34: i32 = v10.wrapping_add(1i32);
                        (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10) = (v0.clone(), v14, v2, v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9, v34);
                        continue;
                    } else {
                        (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10) = (v0.clone(), v14, v2, v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9, v10);
                        continue;
                    }
                } else {
                    (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10) = (v0.clone(), v14, v2, v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9, v10);
                    continue;
                }
            } else {
                (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10) = (v0.clone(), v14, v2, v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9, v10);
                continue;
            }
        }
    }
}
fn print_rows_from_rank_3(mut v0: Rc<str>, mut v1: i32, mut v2: US0, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: US1, mut v6: std::rc::Rc<std::collections::HashMap<String, (u32, String)>>, mut v7: i32, mut v8: i32) -> i32 {
    loop {
        let mut v9: bool = v7 <= 5i32;
        if v9 {
            let mut v10: i32 = 0i32;
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P"); } LIT.with(|lit| lit.clone()) };
            let mut v12: Rc<str> = std::rc::Rc::<str>::from((v7).to_string());
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v12, &*v13, &*v13, &*v13].concat());
            let mut v15: i32 = 0i32;
            let mut v16: i32 = print_priority_rows_4(v0.clone(), v10, v1, v14.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v8, v15);
            let mut v17: i32 = v7.wrapping_add(1i32);
            let mut v18: i32 = v8.wrapping_add(v16);
            (v0, v1, v2, v3, v4, v5, v6, v7, v8) = (v0.clone(), v1, v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v17, v18);
            continue;
        } else {
            return v8;
        }
    }
}
fn print_json_2(mut v0: Rc<str>, mut v1: i32, mut v2: US0, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: US1, mut v6: std::rc::Rc<std::collections::HashMap<String, (u32, String)>>, mut v7: u64) -> i32 {
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"tasks\":["); } LIT.with(|lit| lit.clone()) };
    let mut v9: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v8.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    let mut v10: i32 = 0i32;
    let mut v11: i32 = 0i32;
    let mut v12: i32 = print_rows_from_rank_3(v0.clone(), v1, v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v10, v11);
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("],\"shown\":"); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = std::rc::Rc::<str>::from((v12).to_string());
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"filter\":"); } LIT.with(|lit| lit.clone()) };
    let mut v19: Rc<str> = match &v2 {
        US0::US0_AllStatuses => {
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
            v16.clone()
        }
        US0::US0_OnlyStatus(v17) => {
            let mut v17: Rc<str> = v17.clone();
            v17.clone()
        }
    };
    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v22: Rc<str> = std::rc::Rc::<str>::from([&*v20, &*v19, &*v20, &*v21, &*v21].concat());
    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
    let mut v24: bool = v3 == v23 ;
    let mut v35: Rc<str> = if v24 {
        let mut v25: bool = v4 == v23 ;
        if v25 {
            v21.clone()
        } else {
            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"assignee\":"); } LIT.with(|lit| lit.clone()) };
            let mut v27: Rc<str> = std::rc::Rc::<str>::from([&*v20, &*v4, &*v20, &*v21, &*v21].concat());
            let mut v28: Rc<str> = std::rc::Rc::<str>::from([&*v26, &*v27, &*v21, &*v21, &*v21].concat());
            v28.clone()
        }
    } else {
        let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"load\":"); } LIT.with(|lit| lit.clone()) };
        let mut v31: Rc<str> = std::rc::Rc::<str>::from([&*v20, &*v3, &*v20, &*v21, &*v21].concat());
        let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"assignee\":"); } LIT.with(|lit| lit.clone()) };
        let mut v33: Rc<str> = std::rc::Rc::<str>::from([&*v20, &*v4, &*v20, &*v21, &*v21].concat());
        let mut v34: Rc<str> = std::rc::Rc::<str>::from([&*v30, &*v31, &*v32, &*v33, &*v21].concat());
        v34.clone()
    };
    let mut v36: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v14, &*v15, &*v22, &*v35].concat());
    let mut v37: Rc<str> = agile_task_census::eoie_agile_status_counts_json(&v0);
    let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"elapsed_ms\":"); } LIT.with(|lit| lit.clone()) };
    let mut v39: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v7);
    let mut v40: Rc<str> = std::rc::Rc::<str>::from((v39).to_string());
    let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
    let mut v42: Rc<str> = std::rc::Rc::<str>::from([&*v36, &*v37, &*v38, &*v40, &*v41].concat());
    let mut v43: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v42.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v43
}
fn print_table_13(mut v0: Rc<str>, mut v1: i32, mut v2: US0, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: US1, mut v6: std::rc::Rc<std::collections::HashMap<String, (u32, String)>>, mut v7: u64) -> i32 {
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ID"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = { let text = v8; let filler = (26i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" P  D     %  "); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("STATUS"); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = { let text = v11; let filler = (8i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIND"); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = { let text = v13; let filler = (9i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
    let mut v15: Rc<str> = agile_planning_fields::eoie_agile_planning_header();
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TITLE"); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v16, &*v17, &*v17, &*v17].concat());
    let mut v19: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v10, &*v12, &*v14, &*v18].concat());
    let mut v20: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v19.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    let mut v21: i32 = 0i32;
    let mut v22: i32 = 0i32;
    let mut v23: i32 = print_rows_from_rank_3(v0.clone(), v1, v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v21, v22);
    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie agile list ok shown="); } LIT.with(|lit| lit.clone()) };
    let mut v25: Rc<str> = std::rc::Rc::<str>::from((v23).to_string());
    let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" filter="); } LIT.with(|lit| lit.clone()) };
    let mut v30: Rc<str> = match &v2 {
        US0::US0_AllStatuses => {
            let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
            v27.clone()
        }
        US0::US0_OnlyStatus(v28) => {
            let mut v28: Rc<str> = v28.clone();
            v28.clone()
        }
    };
    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
    let mut v32: bool = v3 == v31 ;
    let mut v40: Rc<str> = if v32 {
        let mut v33: bool = v4 == v31 ;
        if v33 {
            v17.clone()
        } else {
            let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" assignee="); } LIT.with(|lit| lit.clone()) };
            let mut v35: Rc<str> = std::rc::Rc::<str>::from([&*v34, &*v4, &*v17, &*v17, &*v17].concat());
            v35.clone()
        }
    } else {
        let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" load="); } LIT.with(|lit| lit.clone()) };
        let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" assignee="); } LIT.with(|lit| lit.clone()) };
        let mut v39: Rc<str> = std::rc::Rc::<str>::from([&*v37, &*v3, &*v38, &*v4, &*v17].concat());
        v39.clone()
    };
    let mut v41: Rc<str> = std::rc::Rc::<str>::from([&*v24, &*v25, &*v26, &*v30, &*v40].concat());
    let mut v42: Rc<str> = agile_task_census::eoie_agile_status_counts(&v0);
    let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v44: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v7);
    let mut v45: Rc<str> = std::rc::Rc::<str>::from((v44).to_string());
    let mut v46: Rc<str> = std::rc::Rc::<str>::from([&*v41, &*v42, &*v43, &*v45, &*v17].concat());
    let mut v47: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v46.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v47
}
fn top_level_field_count_15(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: US2 = US2::US2_ScanPlain;
        let mut v5: i32 = field_end_7(v0.clone(), v1, v2, v4.clone());
        let mut v6: bool = v2 <= v5;
        if v6 {
            return -1i32;
        } else {
            let mut v7: i32 = v0.as_bytes().get(v5 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v8: bool = v7 == 41i32;
            if v8 {
                let mut v9: i32 = v3.wrapping_add(1i32);
                return v9;
            } else {
                let mut v10: i32 = v5.wrapping_add(1i32);
                let mut v11: i32 = skip_spaces_8(v0.clone(), v10);
                let mut v12: i32 = v3.wrapping_add(1i32);
                (v0, v1, v2, v3) = (v0.clone(), v11, v2, v12);
                continue;
            }
        }
    }
}
fn first_malformed_task_14(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
        let mut v4: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v3)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let mut v5: bool = v4 < 0i32;
        if v5 {
            return -1i32;
        } else {
            let mut v6: i32 = v4 + 6i32;
            let mut v7: i32 = v0.get(..v4 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let mut v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v7 as usize..v4 as usize).unwrap_or(""));
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let mut v10: bool = v8.starts_with(&*v9);
            if v10 {
                let mut v11: i32 = 0i32;
                let mut v12: i32 = top_level_field_count_15(v0.clone(), v6, v2, v11);
                let mut v13: bool = v12 == 11i32;
                if v13 {
                    (v0, v1, v2) = (v0.clone(), v6, v2);
                    continue;
                } else {
                    return v4;
                }
            } else {
                (v0, v1, v2) = (v0.clone(), v6, v2);
                continue;
            }
        }
    }
}
fn note_index_of_16(mut v0: Rc<str>) -> std::rc::Rc<std::collections::HashMap<String, (u32, String)>> {
    let mut v1: std::rc::Rc<std::collections::HashMap<String, (u32, String)>> = std::rc::Rc::new(v0.lines().filter(|line| line.starts_with("inl note_of_")).filter_map(|line| { let body = line.split_once("= Note (")?.1; let mut fields = body.splitn(4, ", "); let id = fields.next()?.trim_matches(char::from(34u8)).to_owned(); let _seq = fields.next()?; let at = fields.next()?.trim_matches(char::from(34u8)).to_owned(); Some((id, at)) }).fold(std::collections::HashMap::new(), |mut index, (id, at)| { let entry = index.entry(id).or_insert((0u32, String::new())); entry.0 += 1; entry.1 = at; index }));
    v1.clone()
}
fn agile_list_1(mut v0: Rc<str>, mut v1: US0, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: US1, mut v5: std::rc::Rc<std::collections::HashMap<String, (u32, String)>>) -> i32 {
    let mut v6: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v7, &*v8, &*v8, &*v8].concat());
    let mut v10: bool = std::path::Path::new(&*v9).is_file();
    let mut v13: US3 = if v10 {
        US3::US3_FileReadable(v9.clone())
    } else {
        US3::US3_FileMissing(v9.clone())
    };
    match &v13 {
        US3::US3_FileMissing(v14) => {
            let mut v14: Rc<str> = v14.clone();
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile list rejected: "); } LIT.with(|lit| lit.clone()) };
            let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v15, &*v8, &*v8, &*v8].concat());
            let mut v18: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v17 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v18
        }
        US3::US3_FileReadable(v19) => {
            let mut v19: Rc<str> = v19.clone();
            let mut v20: Rc<str> = std::fs::read_to_string(&*v19).unwrap_or_default().into();
            let mut v21: i32 = v20.len() as i32;
            let mut v22: i32 = 0i32;
            let mut v23: i32 = first_malformed_task_14(v20.clone(), v22, v21);
            let mut v24: bool = v23 < 0i32;
            if v24 {
                let mut v25: std::rc::Rc<std::collections::HashMap<String, (u32, String)>> = note_index_of_16(v20.clone());
                match &v4 {
                    US1::US1_JsonFormat => {
                        print_json_2(v20.clone(), v21, v1.clone(), v2.clone(), v3.clone(), v4.clone(), v25.clone(), v6)
                    }
                    US1::US1_TableFormat => {
                        print_table_13(v20.clone(), v21, v1.clone(), v2.clone(), v3.clone(), v4.clone(), v25.clone(), v6)
                    }
                }
            } else {
                let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("malformed Task fields at byte "); } LIT.with(|lit| lit.clone()) };
                let mut v31: Rc<str> = std::rc::Rc::<str>::from((v23).to_string());
                let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (expected "); } LIT.with(|lit| lit.clone()) };
                let mut v33: Rc<str> = std::rc::Rc::<str>::from((11i32).to_string());
                let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" fields)"); } LIT.with(|lit| lit.clone()) };
                let mut v35: Rc<str> = std::rc::Rc::<str>::from([&*v30, &*v31, &*v32, &*v33, &*v34].concat());
                let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile list rejected: "); } LIT.with(|lit| lit.clone()) };
                let mut v37: Rc<str> = std::rc::Rc::<str>::from([&*v36, &*v35, &*v8, &*v8, &*v8].concat());
                let mut v38: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v37 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v38
            }
        }
    }
}
fn default_list_request_17() -> (US0, Rc<str>, Rc<str>, US1, std::rc::Rc<std::collections::HashMap<String, (u32, String)>>) {
    let mut v0: std::rc::Rc<std::collections::HashMap<String, (u32, String)>> = std::rc::Rc::new(std::collections::HashMap::new());
    let mut v1: US0 = US0::US0_AllStatuses;
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
    let mut v3: US1 = US1::US1_TableFormat;
    (v1.clone(), v2.clone(), v2.clone(), v3.clone(), v0.clone())
}
fn json_request_19(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: US1, mut v4: std::rc::Rc<std::collections::HashMap<String, (u32, String)>>) -> (US0, Rc<str>, Rc<str>, US1, std::rc::Rc<std::collections::HashMap<String, (u32, String)>>) {
    let mut v5: US1 = US1::US1_JsonFormat;
    (v0.clone(), v1.clone(), v2.clone(), v5.clone(), v4.clone())
}
fn list_request_from_18(mut v0: i32, mut v1: i32, mut v2: US0, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: US1, mut v6: std::rc::Rc<std::collections::HashMap<String, (u32, String)>>) -> US4 {
    loop {
        let mut v7: bool = v1 <= v0;
        if v7 {
            let mut v8: bool = agile_planning_filters::eoie_agile_planning_filters_valid(&*v3, &*v4);
            if v8 {
                return US4::US4_ListParsed(v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone());
            } else {
                let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("invalid planning filter (expected --load light|medium|heavy|none and --assignee agent|pair|user|none)"); } LIT.with(|lit| lit.clone()) };
                return US4::US4_ListRejected(v10.clone());
            }
        } else {
            let mut v13: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--json"); } LIT.with(|lit| lit.clone()) };
            let mut v15: bool = v13 == v14 ;
            if v15 {
                let mut v16: i32 = v0.wrapping_add(1i32);
                let (mut v17, mut v18, mut v19, mut v20, mut v21): (US0, Rc<str>, Rc<str>, US1, std::rc::Rc<std::collections::HashMap<String, (u32, String)>>) = json_request_19(v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone());
                (v0, v1, v2, v3, v4, v5, v6) = (v16, v1, v17.clone(), v18.clone(), v19.clone(), v20.clone(), v21.clone());
                continue;
            } else {
                let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--load"); } LIT.with(|lit| lit.clone()) };
                let mut v24: bool = v13 == v23 ;
                if v24 {
                    let mut v25: i32 = v0.wrapping_add(2i32);
                    let mut v26: i32 = v0.wrapping_add(1i32);
                    let mut v27: Rc<str> = usize::try_from(v26).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    (v0, v1, v2, v3, v4, v5, v6) = (v25, v1, v2.clone(), v27.clone(), v4.clone(), v5.clone(), v6.clone());
                    continue;
                } else {
                    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--assignee"); } LIT.with(|lit| lit.clone()) };
                    let mut v30: bool = v13 == v29 ;
                    if v30 {
                        let mut v31: i32 = v0.wrapping_add(2i32);
                        let mut v32: i32 = v0.wrapping_add(1i32);
                        let mut v33: Rc<str> = usize::try_from(v32).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        (v0, v1, v2, v3, v4, v5, v6) = (v31, v1, v2.clone(), v3.clone(), v33.clone(), v5.clone(), v6.clone());
                        continue;
                    } else {
                        let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
                        let mut v36: bool = v13 == v35 ;
                        let mut v45: US5 = if v36 {
                            let mut v37: US0 = US0::US0_AllStatuses;
                            US5::US5_Valid(v37.clone())
                        } else {
                            let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planned|Active|Blocked|Paused|Done"); } LIT.with(|lit| lit.clone()) };
                            let mut v40: bool = v39.split("|").any(|item| item == &*v13);
                            if v40 {
                                let mut v41: US0 = US0::US0_OnlyStatus(v13.clone());
                                US5::US5_Valid(v41.clone())
                            } else {
                                US5::US5_Invalid
                            }
                        };
                        match &v45 {
                            US5::US5_Invalid => {
                                let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("invalid agile status filter: "); } LIT.with(|lit| lit.clone()) };
                                let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (expected all, Planned, Active, Blocked, Paused or Done)"); } LIT.with(|lit| lit.clone()) };
                                let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v52: Rc<str> = std::rc::Rc::<str>::from([&*v49, &*v13, &*v50, &*v51, &*v51].concat());
                                return US4::US4_ListRejected(v52.clone());
                            }
                            US5::US5_Valid(v46) => {
                                let mut v46: US0 = v46.clone();
                                let mut v47: i32 = v0.wrapping_add(1i32);
                                (v0, v1, v2, v3, v4, v5, v6) = (v47, v1, v46.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        }
    }
}
fn agile_list_entry_0() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: i32 = std::env::args().skip(2).count() as i32;
    let (mut v2, mut v3, mut v4, mut v5, mut v6): (US0, Rc<str>, Rc<str>, US1, std::rc::Rc<std::collections::HashMap<String, (u32, String)>>) = default_list_request_17();
    let mut v7: US4 = list_request_from_18(v0, v1, v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone());
    match &v7 {
        US4::US4_ListParsed(v8, v9, v10, v11, v12) => {
            let mut v8: US0 = v8.clone();
            let mut v9: Rc<str> = v9.clone();
            let mut v10: Rc<str> = v10.clone();
            let mut v11: US1 = v11.clone();
            let mut v12: std::rc::Rc<std::collections::HashMap<String, (u32, String)>> = v12.clone();
            let mut v13: i32 = 1i32;
            let mut v14: Rc<str> = usize::try_from(v13).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            agile_list_1(v14.clone(), v8.clone(), v9.clone(), v10.clone(), v11.clone(), v12.clone())
        }
        US4::US4_ListRejected(v16) => {
            let mut v16: Rc<str> = v16.clone();
            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile list rejected: "); } LIT.with(|lit| lit.clone()) };
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v19: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v16, &*v18, &*v18, &*v18].concat());
            let mut v20: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v19 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v20
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        agile_list_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_list_run() -> i32 {
    closure0()()
}
