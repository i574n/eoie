#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US1 {
    US1_AgilePlanned,
    US1_AgileActive,
    US1_AgileBlocked,
    US1_AgilePaused,
    US1_AgileDone,
}
#[derive(Clone)]
enum US0 {
    US0_AllStatuses,
    US0_OnlyStatus(US1),
}
#[derive(Clone)]
enum US2 {
    US2_TableFormat,
    US2_JsonFormat,
}
#[derive(Clone)]
enum US3 {
    US3_SortPriority,
    US3_SortDifficulty,
    US3_SortProgress,
}
#[derive(Clone)]
enum UH0 {
    UH0_NoRows,
    UH0_RowThen(i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, i32, Rc<UH0>),
}
#[derive(Clone)]
enum US4 {
    US4_ScanPlain,
    US4_ScanQuoted,
    US4_ScanEscaped,
}
#[derive(Clone)]
enum US5 {
    US5_FileReadable(Rc<str>),
    US5_FileMissing(Rc<str>),
}
#[derive(Clone)]
enum US6 {
    US6_ListParsed(US0, Rc<str>, Rc<str>, US2, std::rc::Rc<std::collections::HashMap<String, (u32, String)>>, Rc<str>, US3, Rc<str>),
    US6_ListRejected(Rc<str>),
}
#[derive(Clone)]
enum US7 {
    US7_SortKnown(US3),
    US7_SortUnknown,
}
#[derive(Clone)]
enum US8 {
    US8_Valid(US0),
    US8_Invalid,
}
fn listing_header_3(mut v0: US2) -> Rc<str> {
    match &v0 {
        US2::US2_JsonFormat => {
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"tasks\":["); } LIT.with(|lit| lit.clone()) };
            v13.clone()
        }
        US2::US2_TableFormat => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ID"); } LIT.with(|lit| lit.clone()) };
            let mut v2: Rc<str> = { let text = v1; let filler = (26i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" P  D     %  "); } LIT.with(|lit| lit.clone()) };
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("STATUS"); } LIT.with(|lit| lit.clone()) };
            let mut v5: Rc<str> = { let text = v4; let filler = (8i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIND"); } LIT.with(|lit| lit.clone()) };
            let mut v7: Rc<str> = { let text = v6; let filler = (9i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
            let mut v8: Rc<str> = agile_planning_fields::eoie_agile_planning_header();
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TITLE"); } LIT.with(|lit| lit.clone()) };
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v9, &*v10, &*v10, &*v10].concat());
            let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v3, &*v5, &*v7, &*v11].concat());
            v12.clone()
        }
    }
}
fn json_line_7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: i32, mut v10: Rc<str>, mut v11: std::rc::Rc<std::collections::HashMap<String, (u32, String)>>, mut v12: bool) -> Rc<str> {
    let mut v15: Rc<str> = if v12 {
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v13.clone()
    } else {
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        v14.clone()
    };
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{"); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("id"); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v0, &*v18, &*v19, &*v19].concat());
    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\":"); } LIT.with(|lit| lit.clone()) };
    let mut v22: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v17, &*v21, &*v20, &*v19].concat());
    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("title"); } LIT.with(|lit| lit.clone()) };
    let mut v25: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v1, &*v18, &*v19, &*v19].concat());
    let mut v26: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v24, &*v21, &*v25, &*v19].concat());
    let mut v27: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v16, &*v22, &*v23, &*v26].concat());
    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kind"); } LIT.with(|lit| lit.clone()) };
    let mut v29: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v2, &*v18, &*v19, &*v19].concat());
    let mut v30: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v28, &*v21, &*v29, &*v19].concat());
    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("priority"); } LIT.with(|lit| lit.clone()) };
    let mut v32: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v3, &*v18, &*v19, &*v19].concat());
    let mut v33: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v31, &*v21, &*v32, &*v19].concat());
    let mut v34: Rc<str> = std::rc::Rc::<str>::from([&*v23, &*v30, &*v23, &*v33, &*v23].concat());
    let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("difficulty"); } LIT.with(|lit| lit.clone()) };
    let mut v36: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v4, &*v18, &*v19, &*v19].concat());
    let mut v37: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v35, &*v21, &*v36, &*v19].concat());
    let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("progress"); } LIT.with(|lit| lit.clone()) };
    let mut v39: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v38, &*v21, &*v5, &*v19].concat());
    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
    let mut v41: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v6, &*v18, &*v19, &*v19].concat());
    let mut v42: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v40, &*v21, &*v41, &*v19].concat());
    let mut v43: Rc<str> = std::rc::Rc::<str>::from([&*v37, &*v23, &*v39, &*v23, &*v42].concat());
    let mut v44: u32 = v11.get(&*v0).map(|entry| entry.0).unwrap_or(0);
    let mut v45: Rc<str> = std::rc::Rc::<str>::from((v44).to_string());
    let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("deps"); } LIT.with(|lit| lit.clone()) };
    let mut v47: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v7, &*v18, &*v19, &*v19].concat());
    let mut v48: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v46, &*v21, &*v47, &*v19].concat());
    let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tags"); } LIT.with(|lit| lit.clone()) };
    let mut v50: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v8, &*v18, &*v19, &*v19].concat());
    let mut v51: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v49, &*v21, &*v50, &*v19].concat());
    let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("notes"); } LIT.with(|lit| lit.clone()) };
    let mut v53: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v52, &*v21, &*v45, &*v19].concat());
    let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("last_note"); } LIT.with(|lit| lit.clone()) };
    let mut v55: Rc<str> = std::rc::Rc::<str>::from(v11.get(&*v0).map(|entry| entry.1.as_str()).unwrap_or(""));
    let mut v56: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v55, &*v18, &*v19, &*v19].concat());
    let mut v57: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v54, &*v21, &*v56, &*v19].concat());
    let mut v58: Rc<str> = std::rc::Rc::<str>::from([&*v53, &*v23, &*v57, &*v23, &*v19].concat());
    let mut v59: Rc<str> = std::rc::Rc::<str>::from([&*v48, &*v23, &*v51, &*v23, &*v58].concat());
    let mut v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
    let mut v61: Rc<str> = std::rc::Rc::<str>::from([&*v59, &*v10, &*v60, &*v19, &*v19].concat());
    let mut v62: Rc<str> = std::rc::Rc::<str>::from([&*v27, &*v34, &*v43, &*v23, &*v61].concat());
    v62.clone()
}
fn table_line_8(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: i32, mut v10: Rc<str>) -> Rc<str> {
    let mut v11: Rc<str> = { let text = v0; let filler = (26i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v12, &*v3, &*v12, &*v4].concat());
    let mut v14: Rc<str> = { let text = v5; let filler = (4i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(" ".repeat(filler) + &*text) };
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = { let text = v6; let filler = (8i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
    let mut v17: Rc<str> = { let text = v2; let filler = (9i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
    let mut v18: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v14, &*v15, &*v16, &*v17].concat());
    let mut v19: Rc<str> = { let text = v1; if text.chars().count() <= 96i32 as usize { text } else { std::rc::Rc::<str>::from(text.chars().take((96i32 as usize).saturating_sub(3)).collect::<String>() + "...") } };
    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v21: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v18, &*v10, &*v19, &*v20].concat());
    v21.clone()
}
fn row_line_6(mut v0: Rc<str>, mut v1: US0, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: US2, mut v5: std::rc::Rc<std::collections::HashMap<String, (u32, String)>>, mut v6: Rc<str>, mut v7: US3, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: Rc<str>, mut v12: Rc<str>, mut v13: Rc<str>, mut v14: Rc<str>, mut v15: Rc<str>, mut v16: Rc<str>, mut v17: Rc<str>, mut v18: i32, mut v19: bool) -> Rc<str> {
    match &v4 {
        US2::US2_JsonFormat => {
            let mut v22: Rc<str> = agile_planning_fields::eoie_agile_planning_json_near(&*v0, &*v9, v18);
            json_line_7(v9.clone(), v10.clone(), v11.clone(), v12.clone(), v13.clone(), v14.clone(), v15.clone(), v16.clone(), v17.clone(), v18, v22.clone(), v5.clone(), v19)
        }
        US2::US2_TableFormat => {
            let mut v20: Rc<str> = agile_planning_fields::eoie_agile_planning_columns_near(&*v0, &*v9, v18);
            table_line_8(v9.clone(), v10.clone(), v11.clone(), v12.clone(), v13.clone(), v14.clone(), v15.clone(), v16.clone(), v17.clone(), v18, v20.clone())
        }
    }
}
fn rows_printed_5(mut v0: Rc<str>, mut v1: US0, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: US2, mut v5: std::rc::Rc<std::collections::HashMap<String, (u32, String)>>, mut v6: Rc<str>, mut v7: US3, mut v8: Rc<str>, mut v9: Rc<UH0>, mut v10: i32) -> i32 {
    loop {
        match &*v9 {
            UH0::UH0_NoRows => {
                return v10;
            }
            UH0::UH0_RowThen(v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22) => {
                let mut v11: i32 = *v11;
                let mut v12: Rc<str> = v12.clone();
                let mut v13: Rc<str> = v13.clone();
                let mut v14: Rc<str> = v14.clone();
                let mut v15: Rc<str> = v15.clone();
                let mut v16: Rc<str> = v16.clone();
                let mut v17: Rc<str> = v17.clone();
                let mut v18: Rc<str> = v18.clone();
                let mut v19: Rc<str> = v19.clone();
                let mut v20: Rc<str> = v20.clone();
                let mut v21: i32 = *v21;
                let mut v22: Rc<UH0> = v22.clone();
                let mut v23: bool = v10 == 0i32;
                let mut v24: Rc<str> = row_line_6(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v12.clone(), v13.clone(), v14.clone(), v15.clone(), v16.clone(), v17.clone(), v18.clone(), v19.clone(), v20.clone(), v21, v23);
                let mut v25: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v24.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                let mut v26: i32 = v10.wrapping_add(1i32);
                (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10) = (v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v22.clone(), v26);
                continue;
            }
        }
    }
}
fn field_end_12(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US4) -> i32 {
    loop {
        let mut v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let mut v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = match &v3 {
                US4::US4_ScanPlain => {
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
                let mut v25: US4 = match &v3 {
                    US4::US4_ScanEscaped => {
                        US4::US4_ScanQuoted
                    }
                    US4::US4_ScanPlain => {
                        let mut v19: bool = v5 == 34i32;
                        if v19 {
                            US4::US4_ScanQuoted
                        } else {
                            US4::US4_ScanPlain
                        }
                    }
                    US4::US4_ScanQuoted => {
                        let mut v12: bool = v5 == 92i32;
                        if v12 {
                            US4::US4_ScanEscaped
                        } else {
                            let mut v14: bool = v5 == 34i32;
                            if v14 {
                                US4::US4_ScanPlain
                            } else {
                                US4::US4_ScanQuoted
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
fn skip_spaces_13(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn field_start_11(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let mut v5: US4 = US4::US4_ScanPlain;
            let mut v6: i32 = field_end_12(v0.clone(), v1, v2, v5.clone());
            let mut v7: i32 = v6.wrapping_add(1i32);
            let mut v8: i32 = skip_spaces_13(v0.clone(), v7);
            let mut v9: i32 = v3.wrapping_sub(1i32);
            (v0, v1, v2, v3) = (v0.clone(), v8, v2, v9);
            continue;
        }
    }
}
fn task_row_at_10(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, i32) {
    let mut v3: i32 = 0i32;
    let mut v4: i32 = field_start_11(v0.clone(), v1, v2, v3);
    let mut v5: US4 = US4::US4_ScanPlain;
    let mut v6: i32 = field_end_12(v0.clone(), v4, v2, v5.clone());
    let mut v7: Rc<str> = std::rc::Rc::<str>::from(v0.get(v4 as usize..v6 as usize).unwrap_or(""));
    let mut v8: i32 = (v7.clone().len() as i32);
    let mut v9: i32 = v8.wrapping_sub(1i32);
    let mut v10: Rc<str> = std::rc::Rc::<str>::from(v7.get(1i32 as usize..v9 as usize).unwrap_or(""));
    let mut v11: i32 = 1i32;
    let mut v12: i32 = field_start_11(v0.clone(), v1, v2, v11);
    let mut v13: US4 = US4::US4_ScanPlain;
    let mut v14: i32 = field_end_12(v0.clone(), v12, v2, v13.clone());
    let mut v15: Rc<str> = std::rc::Rc::<str>::from(v0.get(v12 as usize..v14 as usize).unwrap_or(""));
    let mut v16: i32 = (v15.clone().len() as i32);
    let mut v17: i32 = v16.wrapping_sub(1i32);
    let mut v18: Rc<str> = std::rc::Rc::<str>::from(v15.get(1i32 as usize..v17 as usize).unwrap_or(""));
    let mut v19: i32 = 2i32;
    let mut v20: i32 = field_start_11(v0.clone(), v1, v2, v19);
    let mut v21: US4 = US4::US4_ScanPlain;
    let mut v22: i32 = field_end_12(v0.clone(), v20, v2, v21.clone());
    let mut v23: Rc<str> = std::rc::Rc::<str>::from(v0.get(v20 as usize..v22 as usize).unwrap_or(""));
    let mut v24: i32 = 3i32;
    let mut v25: i32 = field_start_11(v0.clone(), v1, v2, v24);
    let mut v26: US4 = US4::US4_ScanPlain;
    let mut v27: i32 = field_end_12(v0.clone(), v25, v2, v26.clone());
    let mut v28: Rc<str> = std::rc::Rc::<str>::from(v0.get(v25 as usize..v27 as usize).unwrap_or(""));
    let mut v29: i32 = 4i32;
    let mut v30: i32 = field_start_11(v0.clone(), v1, v2, v29);
    let mut v31: US4 = US4::US4_ScanPlain;
    let mut v32: i32 = field_end_12(v0.clone(), v30, v2, v31.clone());
    let mut v33: Rc<str> = std::rc::Rc::<str>::from(v0.get(v30 as usize..v32 as usize).unwrap_or(""));
    let mut v34: i32 = 5i32;
    let mut v35: i32 = field_start_11(v0.clone(), v1, v2, v34);
    let mut v36: US4 = US4::US4_ScanPlain;
    let mut v37: i32 = field_end_12(v0.clone(), v35, v2, v36.clone());
    let mut v38: Rc<str> = std::rc::Rc::<str>::from(v0.get(v35 as usize..v37 as usize).unwrap_or(""));
    let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v41: Rc<str> = std::rc::Rc::<str>::from(v38.replace(&*v39, &v40));
    let mut v42: i32 = 6i32;
    let mut v43: i32 = field_start_11(v0.clone(), v1, v2, v42);
    let mut v44: US4 = US4::US4_ScanPlain;
    let mut v45: i32 = field_end_12(v0.clone(), v43, v2, v44.clone());
    let mut v46: Rc<str> = std::rc::Rc::<str>::from(v0.get(v43 as usize..v45 as usize).unwrap_or(""));
    let mut v47: i32 = 7i32;
    let mut v48: i32 = field_start_11(v0.clone(), v1, v2, v47);
    let mut v49: US4 = US4::US4_ScanPlain;
    let mut v50: i32 = field_end_12(v0.clone(), v48, v2, v49.clone());
    let mut v51: Rc<str> = std::rc::Rc::<str>::from(v0.get(v48 as usize..v50 as usize).unwrap_or(""));
    let mut v52: i32 = (v51.clone().len() as i32);
    let mut v53: i32 = v52.wrapping_sub(1i32);
    let mut v54: Rc<str> = std::rc::Rc::<str>::from(v51.get(1i32 as usize..v53 as usize).unwrap_or(""));
    let mut v55: i32 = 8i32;
    let mut v56: i32 = field_start_11(v0.clone(), v1, v2, v55);
    let mut v57: US4 = US4::US4_ScanPlain;
    let mut v58: i32 = field_end_12(v0.clone(), v56, v2, v57.clone());
    let mut v59: Rc<str> = std::rc::Rc::<str>::from(v0.get(v56 as usize..v58 as usize).unwrap_or(""));
    let mut v60: i32 = (v59.clone().len() as i32);
    let mut v61: i32 = v60.wrapping_sub(1i32);
    let mut v62: Rc<str> = std::rc::Rc::<str>::from(v59.get(1i32 as usize..v61 as usize).unwrap_or(""));
    (v10.clone(), v18.clone(), v23.clone(), v28.clone(), v33.clone(), v41.clone(), v46.clone(), v54.clone(), v62.clone(), v1)
}
fn status_text_16(mut v0: US1) -> Rc<str> {
    match &v0 {
        US1::US1_AgileActive => {
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Active"); } LIT.with(|lit| lit.clone()) };
            v2.clone()
        }
        US1::US1_AgileBlocked => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Blocked"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
        US1::US1_AgileDone => {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Done"); } LIT.with(|lit| lit.clone()) };
            v5.clone()
        }
        US1::US1_AgilePaused => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Paused"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US1::US1_AgilePlanned => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planned"); } LIT.with(|lit| lit.clone()) };
            v1.clone()
        }
    }
}
fn status_selected_15(mut v0: US0, mut v1: Rc<str>) -> bool {
    match &v0 {
        US0::US0_AllStatuses => {
            true
        }
        US0::US0_OnlyStatus(v2) => {
            let mut v2: US1 = v2.clone();
            let mut v3: Rc<str> = status_text_16(v2.clone());
            let mut v4: bool = v3.clone() == v1.clone();
            v4
        }
    }
}
fn row_selected_14(mut v0: Rc<str>, mut v1: US0, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: US2, mut v5: std::rc::Rc<std::collections::HashMap<String, (u32, String)>>, mut v6: Rc<str>, mut v7: US3, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: Rc<str>, mut v12: Rc<str>, mut v13: Rc<str>, mut v14: Rc<str>, mut v15: Rc<str>, mut v16: Rc<str>, mut v17: Rc<str>, mut v18: i32) -> bool {
    let mut v19: bool = status_selected_15(v1.clone(), v15.clone());
    let mut v24: bool = if v19 {
        let mut v20: bool = (v16).split(char::from(44u8)).map(|dep| dep.trim()).filter(|dep| (v8).lines().any(|line| line == *dep)).next().is_none();
        let mut v21: bool = v6.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v21 {
            true
        } else {
            let mut v22: bool = agile_task_query::eoie_query_passes(&*v6, &*v9, &*v17, &*v15, v20);
            v22
        }
    } else {
        false
    };
    if v24 {
        let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
        let mut v26: bool = v2 == v25 ;
        if v26 {
            let mut v27: bool = v3 == v25 ;
            if v27 {
                true
            } else {
                let mut v28: bool = agile_planning_filters::eoie_agile_planning_selected_near(&*v0, &*v9, v18, &*v2, &*v3);
                v28
            }
        } else {
            let mut v30: bool = agile_planning_filters::eoie_agile_planning_selected_near(&*v0, &*v9, v18, &*v2, &*v3);
            v30
        }
    } else {
        false
    }
}
fn row_inserted_17(mut v0: i32, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: i32, mut v11: Rc<UH0>) -> Rc<UH0> {
    match &*v11 {
        UH0::UH0_NoRows => {
            let mut v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoRows); } CASE.with(|case| case.clone()) };
            Rc::new(UH0::UH0_RowThen(v0, v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v10, v12.clone()))
        }
        UH0::UH0_RowThen(v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25) => {
            let mut v14: i32 = *v14;
            let mut v15: Rc<str> = v15.clone();
            let mut v16: Rc<str> = v16.clone();
            let mut v17: Rc<str> = v17.clone();
            let mut v18: Rc<str> = v18.clone();
            let mut v19: Rc<str> = v19.clone();
            let mut v20: Rc<str> = v20.clone();
            let mut v21: Rc<str> = v21.clone();
            let mut v22: Rc<str> = v22.clone();
            let mut v23: Rc<str> = v23.clone();
            let mut v24: i32 = *v24;
            let mut v25: Rc<UH0> = v25.clone();
            let mut v26: bool = v0 < v14;
            if v26 {
                Rc::new(UH0::UH0_RowThen(v0, v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v10, v11.clone()))
            } else {
                let mut v28: Rc<UH0> = row_inserted_17(v0, v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v10, v25.clone());
                Rc::new(UH0::UH0_RowThen(v14, v15.clone(), v16.clone(), v17.clone(), v18.clone(), v19.clone(), v20.clone(), v21.clone(), v22.clone(), v23.clone(), v24, v28.clone()))
            }
        }
    }
}
fn rows_from_9(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US0, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: US2, mut v7: std::rc::Rc<std::collections::HashMap<String, (u32, String)>>, mut v8: Rc<str>, mut v9: US3, mut v10: Rc<str>, mut v11: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
        let mut v13: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v12)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let mut v14: bool = v13 < 0i32;
        if v14 {
            return v11.clone();
        } else {
            let mut v15: i32 = v13 + 6i32;
            let mut v16: i32 = v0.get(..v13 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let mut v17: Rc<str> = std::rc::Rc::<str>::from(v0.get(v16 as usize..v13 as usize).unwrap_or(""));
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let mut v19: bool = v17.starts_with(&*v18);
            if v19 {
                let (mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, i32) = task_row_at_10(v0.clone(), v15, v2);
                let mut v30: bool = row_selected_14(v0.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v10.clone(), v20.clone(), v21.clone(), v22.clone(), v23.clone(), v24.clone(), v25.clone(), v26.clone(), v27.clone(), v28.clone(), v29);
                if v30 {
                    let mut v31: i32 = (v23).get(1..).and_then(|digits| digits.parse::<i32>().ok()).unwrap_or(9);
                    let mut v41: i32 = match &v9 {
                        US3::US3_SortDifficulty => {
                            let mut v32: i32 = (v24).get(1..).and_then(|digits| digits.parse::<i32>().ok()).unwrap_or(9);
                            let mut v33: i32 = v32.wrapping_mul(10i32);
                            let mut v34: i32 = v33.wrapping_add(v31);
                            v34
                        }
                        US3::US3_SortPriority => {
                            v31
                        }
                        US3::US3_SortProgress => {
                            let mut v35: i32 = (v25).trim().parse::<i32>().unwrap_or(0).clamp(0, 100);
                            let mut v36: i32 = 100i32.wrapping_sub(v35);
                            let mut v37: i32 = v36.wrapping_mul(10i32);
                            let mut v38: i32 = v37.wrapping_add(v31);
                            v38
                        }
                    };
                    let mut v42: Rc<UH0> = row_inserted_17(v41, v20.clone(), v21.clone(), v22.clone(), v23.clone(), v24.clone(), v25.clone(), v26.clone(), v27.clone(), v28.clone(), v29, v11.clone());
                    (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11) = (v0.clone(), v15, v2, v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v10.clone(), v42.clone());
                    continue;
                } else {
                    (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11) = (v0.clone(), v15, v2, v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v10.clone(), v11.clone());
                    continue;
                }
            } else {
                (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11) = (v0.clone(), v15, v2, v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v10.clone(), v11.clone());
                continue;
            }
        }
    }
}
fn print_rows_4(mut v0: Rc<str>, mut v1: i32, mut v2: US0, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: US2, mut v6: std::rc::Rc<std::collections::HashMap<String, (u32, String)>>, mut v7: Rc<str>, mut v8: US3, mut v9: Rc<str>) -> i32 {
    let mut v10: i32 = 0i32;
    let mut v11: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoRows); } CASE.with(|case| case.clone()) };
    let mut v12: Rc<UH0> = rows_from_9(v0.clone(), v10, v1, v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v11.clone());
    let mut v13: i32 = 0i32;
    rows_printed_5(v0.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v12.clone(), v13)
}
fn filter_text_19(mut v0: US0) -> Rc<str> {
    match &v0 {
        US0::US0_AllStatuses => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
            v1.clone()
        }
        US0::US0_OnlyStatus(v2) => {
            let mut v2: US1 = v2.clone();
            status_text_16(v2.clone())
        }
    }
}
fn json_footer_18(mut v0: Rc<str>, mut v1: i32, mut v2: US0, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: US2, mut v6: std::rc::Rc<std::collections::HashMap<String, (u32, String)>>, mut v7: Rc<str>, mut v8: US3, mut v9: Rc<str>, mut v10: i32, mut v11: Rc<str>) -> Rc<str> {
    let mut v20: Rc<str> = match &v8 {
        US3::US3_SortPriority => {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v12.clone()
        }
        _ => {
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" sort="); } LIT.with(|lit| lit.clone()) };
            let mut v17: Rc<str> = match &v8 {
                US3::US3_SortDifficulty => {
                    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("difficulty"); } LIT.with(|lit| lit.clone()) };
                    v14.clone()
                }
                US3::US3_SortProgress => {
                    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("progress"); } LIT.with(|lit| lit.clone()) };
                    v15.clone()
                }
                _ => unreachable!(),
            };
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v19: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v17, &*v18, &*v18, &*v18].concat());
            v19.clone()
        }
    };
    let mut v21: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v24: Rc<str> = if v21 {
        let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v22.clone()
    } else {
        let mut v23: Rc<str> = agile_task_query::eoie_query_shown(&*v7);
        v23.clone()
    };
    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v26: Rc<str> = std::rc::Rc::<str>::from([&*v24, &*v20, &*v25, &*v25, &*v25].concat());
    let mut v27: bool = v26.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v33: Rc<str> = if v27 {
        v25.clone()
    } else {
        let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"query\":"); } LIT.with(|lit| lit.clone()) };
        let mut v29: Rc<str> = std::rc::Rc::<str>::from(v26.trim());
        let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let mut v31: Rc<str> = std::rc::Rc::<str>::from([&*v30, &*v29, &*v30, &*v25, &*v25].concat());
        let mut v32: Rc<str> = std::rc::Rc::<str>::from([&*v28, &*v31, &*v25, &*v25, &*v25].concat());
        v32.clone()
    };
    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("],\"shown\":"); } LIT.with(|lit| lit.clone()) };
    let mut v35: Rc<str> = std::rc::Rc::<str>::from((v10).to_string());
    let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"filter\":"); } LIT.with(|lit| lit.clone()) };
    let mut v37: Rc<str> = filter_text_19(v2.clone());
    let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v39: Rc<str> = std::rc::Rc::<str>::from([&*v38, &*v37, &*v38, &*v25, &*v25].concat());
    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
    let mut v41: bool = v3 == v40 ;
    let mut v52: Rc<str> = if v41 {
        let mut v42: bool = v4 == v40 ;
        if v42 {
            v25.clone()
        } else {
            let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"assignee\":"); } LIT.with(|lit| lit.clone()) };
            let mut v44: Rc<str> = std::rc::Rc::<str>::from([&*v38, &*v4, &*v38, &*v25, &*v25].concat());
            let mut v45: Rc<str> = std::rc::Rc::<str>::from([&*v43, &*v44, &*v25, &*v25, &*v25].concat());
            v45.clone()
        }
    } else {
        let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"load\":"); } LIT.with(|lit| lit.clone()) };
        let mut v48: Rc<str> = std::rc::Rc::<str>::from([&*v38, &*v3, &*v38, &*v25, &*v25].concat());
        let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"assignee\":"); } LIT.with(|lit| lit.clone()) };
        let mut v50: Rc<str> = std::rc::Rc::<str>::from([&*v38, &*v4, &*v38, &*v25, &*v25].concat());
        let mut v51: Rc<str> = std::rc::Rc::<str>::from([&*v47, &*v48, &*v49, &*v50, &*v25].concat());
        v51.clone()
    };
    let mut v53: Rc<str> = std::rc::Rc::<str>::from([&*v52, &*v33, &*v25, &*v25, &*v25].concat());
    let mut v54: Rc<str> = std::rc::Rc::<str>::from([&*v34, &*v35, &*v36, &*v39, &*v53].concat());
    let mut v55: Rc<str> = agile_task_census::eoie_agile_status_counts_json(&v0);
    let mut v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"elapsed_ms\":"); } LIT.with(|lit| lit.clone()) };
    let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
    let mut v58: Rc<str> = std::rc::Rc::<str>::from([&*v54, &*v55, &*v56, &*v11, &*v57].concat());
    v58.clone()
}
fn table_footer_20(mut v0: Rc<str>, mut v1: i32, mut v2: US0, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: US2, mut v6: std::rc::Rc<std::collections::HashMap<String, (u32, String)>>, mut v7: Rc<str>, mut v8: US3, mut v9: Rc<str>, mut v10: i32, mut v11: Rc<str>) -> Rc<str> {
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie agile list ok shown="); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = std::rc::Rc::<str>::from((v10).to_string());
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" filter="); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = filter_text_19(v2.clone());
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
    let mut v17: bool = v3 == v16 ;
    let mut v28: Rc<str> = if v17 {
        let mut v18: bool = v4 == v16 ;
        if v18 {
            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v19.clone()
        } else {
            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" assignee="); } LIT.with(|lit| lit.clone()) };
            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v22: Rc<str> = std::rc::Rc::<str>::from([&*v20, &*v4, &*v21, &*v21, &*v21].concat());
            v22.clone()
        }
    } else {
        let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" load="); } LIT.with(|lit| lit.clone()) };
        let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" assignee="); } LIT.with(|lit| lit.clone()) };
        let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v27: Rc<str> = std::rc::Rc::<str>::from([&*v24, &*v3, &*v25, &*v4, &*v26].concat());
        v27.clone()
    };
    let mut v37: Rc<str> = match &v8 {
        US3::US3_SortPriority => {
            let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v29.clone()
        }
        _ => {
            let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" sort="); } LIT.with(|lit| lit.clone()) };
            let mut v34: Rc<str> = match &v8 {
                US3::US3_SortDifficulty => {
                    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("difficulty"); } LIT.with(|lit| lit.clone()) };
                    v31.clone()
                }
                US3::US3_SortProgress => {
                    let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("progress"); } LIT.with(|lit| lit.clone()) };
                    v32.clone()
                }
                _ => unreachable!(),
            };
            let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v36: Rc<str> = std::rc::Rc::<str>::from([&*v30, &*v34, &*v35, &*v35, &*v35].concat());
            v36.clone()
        }
    };
    let mut v38: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v41: Rc<str> = if v38 {
        let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v39.clone()
    } else {
        let mut v40: Rc<str> = agile_task_query::eoie_query_shown(&*v7);
        v40.clone()
    };
    let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v43: Rc<str> = std::rc::Rc::<str>::from([&*v41, &*v37, &*v42, &*v42, &*v42].concat());
    let mut v44: Rc<str> = std::rc::Rc::<str>::from([&*v28, &*v43, &*v42, &*v42, &*v42].concat());
    let mut v45: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v13, &*v14, &*v15, &*v44].concat());
    let mut v46: Rc<str> = agile_task_census::eoie_agile_status_counts(&v0);
    let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v48: Rc<str> = std::rc::Rc::<str>::from([&*v45, &*v46, &*v47, &*v11, &*v42].concat());
    v48.clone()
}
fn print_listing_2(mut v0: Rc<str>, mut v1: i32, mut v2: US0, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: US2, mut v6: std::rc::Rc<std::collections::HashMap<String, (u32, String)>>, mut v7: Rc<str>, mut v8: US3, mut v9: Rc<str>, mut v10: u64) -> i32 {
    let mut v11: Rc<str> = listing_header_3(v5.clone());
    let mut v12: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v11.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    let mut v13: i32 = print_rows_4(v0.clone(), v1, v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone());
    let mut v14: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v10);
    let mut v15: Rc<str> = std::rc::Rc::<str>::from((v14).to_string());
    match &v5 {
        US2::US2_JsonFormat => {
            let mut v18: Rc<str> = json_footer_18(v0.clone(), v1, v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v13, v15.clone());
            let mut v19: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v18.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
            v19
        }
        US2::US2_TableFormat => {
            let mut v16: Rc<str> = table_footer_20(v0.clone(), v1, v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v13, v15.clone());
            let mut v17: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v16.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
            v17
        }
    }
}
fn note_index_of_21(mut v0: Rc<str>) -> std::rc::Rc<std::collections::HashMap<String, (u32, String)>> {
    let mut v1: std::rc::Rc<std::collections::HashMap<String, (u32, String)>> = std::rc::Rc::new(v0.lines().filter(|line| line.starts_with("inl note_of_")).filter_map(|line| { let body = line.split_once("= Note (")?.1; let mut fields = body.splitn(4, ", "); let id = fields.next()?.trim_matches(char::from(34u8)).to_owned(); let _seq = fields.next()?; let at = fields.next()?.trim_matches(char::from(34u8)).to_owned(); Some((id, at)) }).fold(std::collections::HashMap::new(), |mut index, (id, at)| { let entry = index.entry(id).or_insert((0u32, String::new())); entry.0 += 1; entry.1 = at; index }));
    v1.clone()
}
fn agile_list_1(mut v0: Rc<str>, mut v1: US0, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: US2, mut v5: std::rc::Rc<std::collections::HashMap<String, (u32, String)>>, mut v6: Rc<str>, mut v7: US3, mut v8: Rc<str>) -> i32 {
    let mut v9: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v10, &*v11, &*v11, &*v11].concat());
    let mut v13: bool = std::path::Path::new(&*v12).is_file();
    let mut v16: US5 = if v13 {
        US5::US5_FileReadable(v12.clone())
    } else {
        US5::US5_FileMissing(v12.clone())
    };
    match &v16 {
        US5::US5_FileMissing(v17) => {
            let mut v17: Rc<str> = v17.clone();
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile list rejected: "); } LIT.with(|lit| lit.clone()) };
            let mut v20: Rc<str> = std::rc::Rc::<str>::from([&*v19, &*v18, &*v11, &*v11, &*v11].concat());
            let mut v21: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v20 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v21
        }
        US5::US5_FileReadable(v22) => {
            let mut v22: Rc<str> = v22.clone();
            let mut v23: Rc<str> = std::fs::read_to_string(&*v22).unwrap_or_default().into();
            let mut v24: i32 = (v23.clone().len() as i32);
            let mut v25: i32 = agile_task_census::eoie_agile_first_malformed_task(&*v23);
            let mut v26: bool = v25 < 0i32;
            if v26 {
                let mut v27: bool = v6.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v29: bool = if v27 {
                    false
                } else {
                    let mut v28: bool = agile_task_query::eoie_query_reads_deps(&*v6);
                    v28
                };
                let mut v31: Rc<str> = if v29 {
                    let mut v30: Rc<str> = agile_task_census::eoie_agile_unsettled_ids(&v23);
                    v30.clone()
                } else {
                    v11.clone()
                };
                let mut v32: std::rc::Rc<std::collections::HashMap<String, (u32, String)>> = note_index_of_21(v23.clone());
                print_listing_2(v23.clone(), v24, v1.clone(), v2.clone(), v3.clone(), v4.clone(), v32.clone(), v6.clone(), v7.clone(), v31.clone(), v9)
            } else {
                let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("malformed Task fields at byte "); } LIT.with(|lit| lit.clone()) };
                let mut v35: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
                let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (expected "); } LIT.with(|lit| lit.clone()) };
                let mut v37: Rc<str> = std::rc::Rc::<str>::from((11i32).to_string());
                let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" fields)"); } LIT.with(|lit| lit.clone()) };
                let mut v39: Rc<str> = std::rc::Rc::<str>::from([&*v34, &*v35, &*v36, &*v37, &*v38].concat());
                let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile list rejected: "); } LIT.with(|lit| lit.clone()) };
                let mut v41: Rc<str> = std::rc::Rc::<str>::from([&*v40, &*v39, &*v11, &*v11, &*v11].concat());
                let mut v42: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v41 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v42
            }
        }
    }
}
fn default_list_request_22() -> (US0, Rc<str>, Rc<str>, US2, std::rc::Rc<std::collections::HashMap<String, (u32, String)>>, Rc<str>, US3, Rc<str>) {
    let mut v0: std::rc::Rc<std::collections::HashMap<String, (u32, String)>> = std::rc::Rc::new(std::collections::HashMap::new());
    let mut v1: US0 = US0::US0_AllStatuses;
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
    let mut v3: US2 = US2::US2_TableFormat;
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: US3 = US3::US3_SortPriority;
    (v1.clone(), v2.clone(), v2.clone(), v3.clone(), v0.clone(), v4.clone(), v5.clone(), v4.clone())
}
fn json_request_24(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: US2, mut v4: std::rc::Rc<std::collections::HashMap<String, (u32, String)>>, mut v5: Rc<str>, mut v6: US3, mut v7: Rc<str>) -> (US0, Rc<str>, Rc<str>, US2, std::rc::Rc<std::collections::HashMap<String, (u32, String)>>, Rc<str>, US3, Rc<str>) {
    let mut v8: US2 = US2::US2_JsonFormat;
    (v0.clone(), v1.clone(), v2.clone(), v8.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone())
}
fn parse_status_filter_25(mut v0: Rc<str>) -> US8 {
    let mut v1: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
    if v1 {
        let mut v2: US0 = US0::US0_AllStatuses;
        US8::US8_Valid(v2.clone())
    } else {
        let mut v4: US1 = US1::US1_AgilePlanned;
        let mut v5: Rc<str> = status_text_16(v4.clone());
        let mut v6: bool = v0.clone() == v5.clone();
        if v6 {
            let mut v7: US1 = US1::US1_AgilePlanned;
            let mut v8: US0 = US0::US0_OnlyStatus(v7.clone());
            US8::US8_Valid(v8.clone())
        } else {
            let mut v10: US1 = US1::US1_AgileActive;
            let mut v11: Rc<str> = status_text_16(v10.clone());
            let mut v12: bool = v0.clone() == v11.clone();
            if v12 {
                let mut v13: US1 = US1::US1_AgileActive;
                let mut v14: US0 = US0::US0_OnlyStatus(v13.clone());
                US8::US8_Valid(v14.clone())
            } else {
                let mut v16: US1 = US1::US1_AgileBlocked;
                let mut v17: Rc<str> = status_text_16(v16.clone());
                let mut v18: bool = v0.clone() == v17.clone();
                if v18 {
                    let mut v19: US1 = US1::US1_AgileBlocked;
                    let mut v20: US0 = US0::US0_OnlyStatus(v19.clone());
                    US8::US8_Valid(v20.clone())
                } else {
                    let mut v22: US1 = US1::US1_AgilePaused;
                    let mut v23: Rc<str> = status_text_16(v22.clone());
                    let mut v24: bool = v0.clone() == v23.clone();
                    if v24 {
                        let mut v25: US1 = US1::US1_AgilePaused;
                        let mut v26: US0 = US0::US0_OnlyStatus(v25.clone());
                        US8::US8_Valid(v26.clone())
                    } else {
                        let mut v28: US1 = US1::US1_AgileDone;
                        let mut v29: Rc<str> = status_text_16(v28.clone());
                        let mut v30: bool = v0.clone() == v29.clone();
                        if v30 {
                            let mut v31: US1 = US1::US1_AgileDone;
                            let mut v32: US0 = US0::US0_OnlyStatus(v31.clone());
                            US8::US8_Valid(v32.clone())
                        } else {
                            US8::US8_Invalid
                        }
                    }
                }
            }
        }
    }
}
fn list_request_from_23(mut v0: i32, mut v1: i32, mut v2: US0, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: US2, mut v6: std::rc::Rc<std::collections::HashMap<String, (u32, String)>>, mut v7: Rc<str>, mut v8: US3, mut v9: Rc<str>) -> US6 {
    loop {
        let mut v10: bool = v1 <= v0;
        if v10 {
            let mut v11: bool = agile_planning_filters::eoie_agile_planning_filters_valid(&*v3, &*v4);
            if v11 {
                return US6::US6_ListParsed(v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone());
            } else {
                let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("invalid planning filter (expected --load light|medium|heavy|none and --assignee agent|pair|user|none)"); } LIT.with(|lit| lit.clone()) };
                return US6::US6_ListRejected(v13.clone());
            }
        } else {
            let mut v16: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--json"); } LIT.with(|lit| lit.clone()) };
            let mut v18: bool = v16 == v17 ;
            if v18 {
                let mut v19: i32 = v0.wrapping_add(1i32);
                let (mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27): (US0, Rc<str>, Rc<str>, US2, std::rc::Rc<std::collections::HashMap<String, (u32, String)>>, Rc<str>, US3, Rc<str>) = json_request_24(v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone());
                (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9) = (v19, v1, v20.clone(), v21.clone(), v22.clone(), v23.clone(), v24.clone(), v25.clone(), v26.clone(), v27.clone());
                continue;
            } else {
                let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--load"); } LIT.with(|lit| lit.clone()) };
                let mut v30: bool = v16 == v29 ;
                if v30 {
                    let mut v31: i32 = v0.wrapping_add(2i32);
                    let mut v32: i32 = v0.wrapping_add(1i32);
                    let mut v33: Rc<str> = usize::try_from(v32).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9) = (v31, v1, v2.clone(), v33.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone());
                    continue;
                } else {
                    let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--assignee"); } LIT.with(|lit| lit.clone()) };
                    let mut v36: bool = v16 == v35 ;
                    if v36 {
                        let mut v37: i32 = v0.wrapping_add(2i32);
                        let mut v38: i32 = v0.wrapping_add(1i32);
                        let mut v39: Rc<str> = usize::try_from(v38).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9) = (v37, v1, v2.clone(), v3.clone(), v39.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone());
                        continue;
                    } else {
                        let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--sort"); } LIT.with(|lit| lit.clone()) };
                        let mut v42: bool = v16 == v41 ;
                        if v42 {
                            let mut v43: i32 = v0.wrapping_add(1i32);
                            let mut v44: Rc<str> = usize::try_from(v43).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                            let mut v45: bool = v44.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("priority"); } LIT.with(|lit| lit.clone()) };
                            let mut v57: US7 = if v45 {
                                let mut v46: US3 = US3::US3_SortPriority;
                                US7::US7_SortKnown(v46.clone())
                            } else {
                                let mut v48: bool = v44.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("difficulty"); } LIT.with(|lit| lit.clone()) };
                                if v48 {
                                    let mut v49: US3 = US3::US3_SortDifficulty;
                                    US7::US7_SortKnown(v49.clone())
                                } else {
                                    let mut v51: bool = v44.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("progress"); } LIT.with(|lit| lit.clone()) };
                                    if v51 {
                                        let mut v52: US3 = US3::US3_SortProgress;
                                        US7::US7_SortKnown(v52.clone())
                                    } else {
                                        US7::US7_SortUnknown
                                    }
                                }
                            };
                            match &v57 {
                                US7::US7_SortKnown(v58) => {
                                    let mut v58: US3 = v58.clone();
                                    let mut v59: i32 = v0.wrapping_add(2i32);
                                    (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9) = (v59, v1, v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v58.clone(), v9.clone());
                                    continue;
                                }
                                US7::US7_SortUnknown => {
                                    let mut v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--sort expects "); } LIT.with(|lit| lit.clone()) };
                                    let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("priority|difficulty|progress"); } LIT.with(|lit| lit.clone()) };
                                    let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", got "); } LIT.with(|lit| lit.clone()) };
                                    let mut v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let mut v65: Rc<str> = std::rc::Rc::<str>::from([&*v61, &*v62, &*v63, &*v44, &*v64].concat());
                                    return US6::US6_ListRejected(v65.clone());
                                }
                            }
                        } else {
                            let mut v69: i32 = v0.wrapping_add(1i32);
                            let mut v70: Rc<str> = usize::try_from(v69).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                            let mut v71: i32 = agile_task_query::eoie_query_flag_width(&*v16, &*v70);
                            let mut v72: bool = v71 > 0i32;
                            if v72 {
                                let mut v73: i32 = v0.wrapping_add(v71);
                                let mut v74: bool = v71 == 2i32;
                                let mut v76: Rc<str> = if v74 {
                                    v70.clone()
                                } else {
                                    let mut v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    v75.clone()
                                };
                                let mut v77: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v80: Rc<str> = if v77 {
                                    let mut v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    v78.clone()
                                } else {
                                    let mut v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                    v79.clone()
                                };
                                let mut v81: bool = v76.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v84: Rc<str> = if v81 {
                                    let mut v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    v82.clone()
                                } else {
                                    let mut v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                    v83.clone()
                                };
                                let mut v85: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v80, &*v16, &*v84, &*v76].concat());
                                (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9) = (v73, v1, v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v85.clone(), v8.clone(), v9.clone());
                                continue;
                            } else {
                                let mut v87: bool = v71 < 0i32;
                                if v87 {
                                    let mut v88: Rc<str> = agile_task_query::eoie_query_flag_refusal(&*v16, &*v70);
                                    return US6::US6_ListRejected(v88.clone());
                                } else {
                                    let mut v90: US8 = parse_status_filter_25(v16.clone());
                                    match &v90 {
                                        US8::US8_Invalid => {
                                            let mut v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("invalid agile status filter: "); } LIT.with(|lit| lit.clone()) };
                                            let mut v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (expected all, Planned, Active, Blocked, Paused or Done; or "); } LIT.with(|lit| lit.clone()) };
                                            let mut v95: Rc<str> = agile_task_query::eoie_query_usage();
                                            let mut v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", --sort priority|difficulty|progress)"); } LIT.with(|lit| lit.clone()) };
                                            let mut v97: Rc<str> = std::rc::Rc::<str>::from([&*v93, &*v16, &*v94, &*v95, &*v96].concat());
                                            return US6::US6_ListRejected(v97.clone());
                                        }
                                        US8::US8_Valid(v91) => {
                                            let mut v91: US0 = v91.clone();
                                            (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9) = (v69, v1, v91.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone());
                                            continue;
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
fn agile_list_entry_0() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: i32 = std::env::args().skip(2).count() as i32;
    let (mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9): (US0, Rc<str>, Rc<str>, US2, std::rc::Rc<std::collections::HashMap<String, (u32, String)>>, Rc<str>, US3, Rc<str>) = default_list_request_22();
    let mut v10: US6 = list_request_from_23(v0, v1, v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone());
    match &v10 {
        US6::US6_ListParsed(v11, v12, v13, v14, v15, v16, v17, v18) => {
            let mut v11: US0 = v11.clone();
            let mut v12: Rc<str> = v12.clone();
            let mut v13: Rc<str> = v13.clone();
            let mut v14: US2 = v14.clone();
            let mut v15: std::rc::Rc<std::collections::HashMap<String, (u32, String)>> = v15.clone();
            let mut v16: Rc<str> = v16.clone();
            let mut v17: US3 = v17.clone();
            let mut v18: Rc<str> = v18.clone();
            let mut v19: i32 = 1i32;
            let mut v20: Rc<str> = usize::try_from(v19).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            agile_list_1(v20.clone(), v11.clone(), v12.clone(), v13.clone(), v14.clone(), v15.clone(), v16.clone(), v17.clone(), v18.clone())
        }
        US6::US6_ListRejected(v22) => {
            let mut v22: Rc<str> = v22.clone();
            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile list rejected: "); } LIT.with(|lit| lit.clone()) };
            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v25: Rc<str> = std::rc::Rc::<str>::from([&*v23, &*v22, &*v24, &*v24, &*v24].concat());
            let mut v26: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v25 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v26
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
