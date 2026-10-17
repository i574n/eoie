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
    US4_ListParsed(US0, Rc<str>, Rc<str>, US1),
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
                let mut v10: i32 = v1 + 1;
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
            let mut v4: i32 = v1 + 1;
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
            let mut v7: i32 = v6 + 1;
            let mut v8: i32 = skip_spaces_8(v0.clone(), v7);
            let mut v9: i32 = v3 - 1;
            (v0, v1, v2, v3) = (v0.clone(), v8, v2, v9);
            continue;
        }
    }
}
fn task_row_at_5(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let mut v3: i32 = 0i32;
    let mut v4: i32 = field_start_6(v0.clone(), v1, v2, v3);
    let mut v5: US2 = US2::US2_ScanPlain;
    let mut v6: i32 = field_end_7(v0.clone(), v4, v2, v5.clone());
    let mut v7: Rc<str> = std::rc::Rc::<str>::from(v0.get(v4 as usize..v6 as usize).unwrap_or(""));
    let mut v8: i32 = v7.len() as i32;
    let mut v9: i32 = v8 - 1;
    let mut v10: Rc<str> = std::rc::Rc::<str>::from(v7.get(1i32 as usize..v9 as usize).unwrap_or(""));
    let mut v11: i32 = 1i32;
    let mut v12: i32 = field_start_6(v0.clone(), v1, v2, v11);
    let mut v13: US2 = US2::US2_ScanPlain;
    let mut v14: i32 = field_end_7(v0.clone(), v12, v2, v13.clone());
    let mut v15: Rc<str> = std::rc::Rc::<str>::from(v0.get(v12 as usize..v14 as usize).unwrap_or(""));
    let mut v16: i32 = v15.len() as i32;
    let mut v17: i32 = v16 - 1;
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
    (v10.clone(), v18.clone(), v23.clone(), v28.clone(), v33.clone(), v41.clone(), v46.clone())
}
fn row_selected_9(mut v0: Rc<str>, mut v1: US0, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: US1, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: Rc<str>) -> bool {
    let mut v15: bool = match &v1 {
        US0::US0_AllStatuses => {
            true
        }
        US0::US0_OnlyStatus(v12) => {
            let mut v12: Rc<str> = v12.clone();
            let mut v13: bool = v12 == v11 ;
            v13
        }
    };
    if v15 {
        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
        let mut v17: bool = v2 == v16 ;
        if v17 {
            let mut v18: bool = v3 == v16 ;
            if v18 {
                true
            } else {
                let mut v19: bool = agile_planning_filters::eoie_agile_planning_selected(&*v0, &*v5, &*v2, &*v3);
                v19
            }
        } else {
            let mut v21: bool = agile_planning_filters::eoie_agile_planning_selected(&*v0, &*v5, &*v2, &*v3);
            v21
        }
    } else {
        false
    }
}
fn json_line_11(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool) -> Rc<str> {
    let mut v11: Rc<str> = if v8 {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v9.clone()
    } else {
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        v10.clone()
    };
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{"); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("id"); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v0, &*v14, &*v15, &*v15].concat());
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\":"); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v13, &*v17, &*v16, &*v15].concat());
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("title"); } LIT.with(|lit| lit.clone()) };
    let mut v21: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v1, &*v14, &*v15, &*v15].concat());
    let mut v22: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v20, &*v17, &*v21, &*v15].concat());
    let mut v23: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v12, &*v18, &*v19, &*v22].concat());
    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kind"); } LIT.with(|lit| lit.clone()) };
    let mut v25: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v2, &*v14, &*v15, &*v15].concat());
    let mut v26: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v24, &*v17, &*v25, &*v15].concat());
    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("priority"); } LIT.with(|lit| lit.clone()) };
    let mut v28: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v3, &*v14, &*v15, &*v15].concat());
    let mut v29: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v27, &*v17, &*v28, &*v15].concat());
    let mut v30: Rc<str> = std::rc::Rc::<str>::from([&*v19, &*v26, &*v19, &*v29, &*v19].concat());
    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("difficulty"); } LIT.with(|lit| lit.clone()) };
    let mut v32: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v4, &*v14, &*v15, &*v15].concat());
    let mut v33: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v31, &*v17, &*v32, &*v15].concat());
    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("progress"); } LIT.with(|lit| lit.clone()) };
    let mut v35: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v34, &*v17, &*v5, &*v15].concat());
    let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
    let mut v37: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v6, &*v14, &*v15, &*v15].concat());
    let mut v38: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v36, &*v17, &*v37, &*v15].concat());
    let mut v39: Rc<str> = std::rc::Rc::<str>::from([&*v33, &*v19, &*v35, &*v19, &*v38].concat());
    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
    let mut v41: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v40, &*v15, &*v15, &*v15].concat());
    let mut v42: Rc<str> = std::rc::Rc::<str>::from([&*v23, &*v30, &*v39, &*v19, &*v41].concat());
    v42.clone()
}
fn table_line_12(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>) -> Rc<str> {
    let mut v8: Rc<str> = { let text = v0; let filler = (26i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v9, &*v3, &*v9, &*v4].concat());
    let mut v11: Rc<str> = { let text = v5; let filler = (4i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(" ".repeat(filler) + &*text) };
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = { let text = v6; let filler = (8i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
    let mut v14: Rc<str> = { let text = v2; let filler = (9i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
    let mut v15: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v11, &*v12, &*v13, &*v14].concat());
    let mut v16: Rc<str> = { let text = v1; if text.chars().count() <= 96i32 as usize { text } else { std::rc::Rc::<str>::from(text.chars().take((96i32 as usize).saturating_sub(3)).collect::<String>() + "...") } };
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v15, &*v7, &*v16, &*v17].concat());
    v18.clone()
}
fn row_line_10(mut v0: Rc<str>, mut v1: US0, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: US1, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: Rc<str>, mut v12: bool) -> Rc<str> {
    match &v4 {
        US1::US1_JsonFormat => {
            let mut v15: Rc<str> = agile_planning_fields::eoie_agile_planning_json(&*v0, &*v5);
            json_line_11(v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v10.clone(), v11.clone(), v15.clone(), v12)
        }
        US1::US1_TableFormat => {
            let mut v13: Rc<str> = agile_planning_fields::eoie_agile_planning_columns(&*v0, &*v5);
            table_line_12(v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v10.clone(), v11.clone(), v13.clone())
        }
    }
}
fn print_priority_rows_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>, mut v4: US0, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: US1, mut v8: i32, mut v9: i32) -> i32 {
    loop {
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
        let mut v11: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v10)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let mut v12: bool = v11 < 0;
        if v12 {
            return v9;
        } else {
            let mut v13: i32 = v11 + 6i32;
            let (mut v14, mut v15, mut v16, mut v17, mut v18, mut v19, mut v20): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = task_row_at_5(v0.clone(), v13, v2);
            let mut v21: i32 = v0.get(..v11 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let mut v22: Rc<str> = std::rc::Rc::<str>::from(v0.get(v21 as usize..v11 as usize).unwrap_or(""));
            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let mut v24: bool = v22.starts_with(&*v23);
            if v24 {
                let mut v25: bool = v17 == v3 ;
                if v25 {
                    let mut v26: bool = row_selected_9(v0.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v14.clone(), v15.clone(), v16.clone(), v17.clone(), v18.clone(), v19.clone(), v20.clone());
                    if v26 {
                        let mut v27: i32 = v8 + v9;
                        let mut v28: bool = v27 == 0i32;
                        let mut v29: Rc<str> = row_line_10(v0.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v14.clone(), v15.clone(), v16.clone(), v17.clone(), v18.clone(), v19.clone(), v20.clone(), v28);
                        let mut v30: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v29.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                        let mut v31: i32 = v9 + 1;
                        (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9) = (v0.clone(), v13, v2, v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8, v31);
                        continue;
                    } else {
                        (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9) = (v0.clone(), v13, v2, v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8, v9);
                        continue;
                    }
                } else {
                    (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9) = (v0.clone(), v13, v2, v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8, v9);
                    continue;
                }
            } else {
                (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9) = (v0.clone(), v13, v2, v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8, v9);
                continue;
            }
        }
    }
}
fn print_rows_from_rank_3(mut v0: Rc<str>, mut v1: i32, mut v2: US0, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: US1, mut v6: i32, mut v7: i32) -> i32 {
    loop {
        let mut v8: bool = v6 <= 5i32;
        if v8 {
            let mut v9: i32 = 0i32;
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P"); } LIT.with(|lit| lit.clone()) };
            let mut v11: Rc<str> = std::rc::Rc::<str>::from((v6).to_string());
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v11, &*v12, &*v12, &*v12].concat());
            let mut v14: i32 = 0i32;
            let mut v15: i32 = print_priority_rows_4(v0.clone(), v9, v1, v13.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v7, v14);
            let mut v16: i32 = v6 + 1;
            let mut v17: i32 = v7 + v15;
            (v0, v1, v2, v3, v4, v5, v6, v7) = (v0.clone(), v1, v2.clone(), v3.clone(), v4.clone(), v5.clone(), v16, v17);
            continue;
        } else {
            return v7;
        }
    }
}
fn print_json_2(mut v0: Rc<str>, mut v1: i32, mut v2: US0, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: US1, mut v6: u64) -> i32 {
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"tasks\":["); } LIT.with(|lit| lit.clone()) };
    let mut v8: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v7.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    let mut v9: i32 = 0i32;
    let mut v10: i32 = 0i32;
    let mut v11: i32 = print_rows_from_rank_3(v0.clone(), v1, v2.clone(), v3.clone(), v4.clone(), v5.clone(), v9, v10);
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("],\"shown\":"); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = std::rc::Rc::<str>::from((v11).to_string());
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"filter\":"); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = match &v2 {
        US0::US0_AllStatuses => {
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
            v15.clone()
        }
        US0::US0_OnlyStatus(v16) => {
            let mut v16: Rc<str> = v16.clone();
            v16.clone()
        }
    };
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v21: Rc<str> = std::rc::Rc::<str>::from([&*v19, &*v18, &*v19, &*v20, &*v20].concat());
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
    let mut v23: bool = v3 == v22 ;
    let mut v34: Rc<str> = if v23 {
        let mut v24: bool = v4 == v22 ;
        if v24 {
            v20.clone()
        } else {
            let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"assignee\":"); } LIT.with(|lit| lit.clone()) };
            let mut v26: Rc<str> = std::rc::Rc::<str>::from([&*v19, &*v4, &*v19, &*v20, &*v20].concat());
            let mut v27: Rc<str> = std::rc::Rc::<str>::from([&*v25, &*v26, &*v20, &*v20, &*v20].concat());
            v27.clone()
        }
    } else {
        let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"load\":"); } LIT.with(|lit| lit.clone()) };
        let mut v30: Rc<str> = std::rc::Rc::<str>::from([&*v19, &*v3, &*v19, &*v20, &*v20].concat());
        let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"assignee\":"); } LIT.with(|lit| lit.clone()) };
        let mut v32: Rc<str> = std::rc::Rc::<str>::from([&*v19, &*v4, &*v19, &*v20, &*v20].concat());
        let mut v33: Rc<str> = std::rc::Rc::<str>::from([&*v29, &*v30, &*v31, &*v32, &*v20].concat());
        v33.clone()
    };
    let mut v35: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v13, &*v14, &*v21, &*v34].concat());
    let mut v36: Rc<str> = agile_task_census::eoie_agile_status_counts_json(&v0);
    let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"elapsed_ms\":"); } LIT.with(|lit| lit.clone()) };
    let mut v38: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v6);
    let mut v39: Rc<str> = std::rc::Rc::<str>::from((v38).to_string());
    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
    let mut v41: Rc<str> = std::rc::Rc::<str>::from([&*v35, &*v36, &*v37, &*v39, &*v40].concat());
    let mut v42: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v41.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v42
}
fn print_table_13(mut v0: Rc<str>, mut v1: i32, mut v2: US0, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: US1, mut v6: u64) -> i32 {
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ID"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = { let text = v7; let filler = (26i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" P  D     %  "); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("STATUS"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = { let text = v10; let filler = (8i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIND"); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = { let text = v12; let filler = (9i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
    let mut v14: Rc<str> = agile_planning_fields::eoie_agile_planning_header();
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TITLE"); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v15, &*v16, &*v16, &*v16].concat());
    let mut v18: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v9, &*v11, &*v13, &*v17].concat());
    let mut v19: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v18.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    let mut v20: i32 = 0i32;
    let mut v21: i32 = 0i32;
    let mut v22: i32 = print_rows_from_rank_3(v0.clone(), v1, v2.clone(), v3.clone(), v4.clone(), v5.clone(), v20, v21);
    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie agile list ok shown="); } LIT.with(|lit| lit.clone()) };
    let mut v24: Rc<str> = std::rc::Rc::<str>::from((v22).to_string());
    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" filter="); } LIT.with(|lit| lit.clone()) };
    let mut v29: Rc<str> = match &v2 {
        US0::US0_AllStatuses => {
            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
            v26.clone()
        }
        US0::US0_OnlyStatus(v27) => {
            let mut v27: Rc<str> = v27.clone();
            v27.clone()
        }
    };
    let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
    let mut v31: bool = v3 == v30 ;
    let mut v39: Rc<str> = if v31 {
        let mut v32: bool = v4 == v30 ;
        if v32 {
            v16.clone()
        } else {
            let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" assignee="); } LIT.with(|lit| lit.clone()) };
            let mut v34: Rc<str> = std::rc::Rc::<str>::from([&*v33, &*v4, &*v16, &*v16, &*v16].concat());
            v34.clone()
        }
    } else {
        let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" load="); } LIT.with(|lit| lit.clone()) };
        let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" assignee="); } LIT.with(|lit| lit.clone()) };
        let mut v38: Rc<str> = std::rc::Rc::<str>::from([&*v36, &*v3, &*v37, &*v4, &*v16].concat());
        v38.clone()
    };
    let mut v40: Rc<str> = std::rc::Rc::<str>::from([&*v23, &*v24, &*v25, &*v29, &*v39].concat());
    let mut v41: Rc<str> = agile_task_census::eoie_agile_status_counts(&v0);
    let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v43: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v6);
    let mut v44: Rc<str> = std::rc::Rc::<str>::from((v43).to_string());
    let mut v45: Rc<str> = std::rc::Rc::<str>::from([&*v40, &*v41, &*v42, &*v44, &*v16].concat());
    let mut v46: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v45.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v46
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
                let mut v9: i32 = v3 + 1;
                return v9;
            } else {
                let mut v10: i32 = v5 + 1;
                let mut v11: i32 = skip_spaces_8(v0.clone(), v10);
                let mut v12: i32 = v3 + 1;
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
        let mut v5: bool = v4 < 0;
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
fn agile_list_1(mut v0: Rc<str>, mut v1: US0, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: US1) -> i32 {
    let mut v5: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v6, &*v7, &*v7, &*v7].concat());
    let mut v9: bool = std::path::Path::new(&*v8).is_file();
    let mut v12: US3 = if v9 {
        US3::US3_FileReadable(v8.clone())
    } else {
        US3::US3_FileMissing(v8.clone())
    };
    match &v12 {
        US3::US3_FileMissing(v13) => {
            let mut v13: Rc<str> = v13.clone();
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile list rejected: "); } LIT.with(|lit| lit.clone()) };
            let mut v16: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v14, &*v7, &*v7, &*v7].concat());
            let mut v17: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v16 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v17
        }
        US3::US3_FileReadable(v18) => {
            let mut v18: Rc<str> = v18.clone();
            let mut v19: Rc<str> = std::fs::read_to_string(&*v18).unwrap_or_default().into();
            let mut v20: i32 = v19.len() as i32;
            let mut v21: i32 = 0i32;
            let mut v22: i32 = first_malformed_task_14(v19.clone(), v21, v20);
            let mut v23: bool = v22 < 0;
            if v23 {
                match &v4 {
                    US1::US1_JsonFormat => {
                        print_json_2(v19.clone(), v20, v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5)
                    }
                    US1::US1_TableFormat => {
                        print_table_13(v19.clone(), v20, v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5)
                    }
                }
            } else {
                let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("malformed Task fields at byte "); } LIT.with(|lit| lit.clone()) };
                let mut v29: Rc<str> = std::rc::Rc::<str>::from((v22).to_string());
                let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (expected "); } LIT.with(|lit| lit.clone()) };
                let mut v31: Rc<str> = std::rc::Rc::<str>::from((11i32).to_string());
                let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" fields)"); } LIT.with(|lit| lit.clone()) };
                let mut v33: Rc<str> = std::rc::Rc::<str>::from([&*v28, &*v29, &*v30, &*v31, &*v32].concat());
                let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile list rejected: "); } LIT.with(|lit| lit.clone()) };
                let mut v35: Rc<str> = std::rc::Rc::<str>::from([&*v34, &*v33, &*v7, &*v7, &*v7].concat());
                let mut v36: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v35 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v36
            }
        }
    }
}
fn default_list_request_16() -> (US0, Rc<str>, Rc<str>, US1) {
    let mut v0: US0 = US0::US0_AllStatuses;
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
    let mut v2: US1 = US1::US1_TableFormat;
    (v0.clone(), v1.clone(), v1.clone(), v2.clone())
}
fn json_request_18(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: US1) -> (US0, Rc<str>, Rc<str>, US1) {
    let mut v4: US1 = US1::US1_JsonFormat;
    (v0.clone(), v1.clone(), v2.clone(), v4.clone())
}
fn list_request_from_17(mut v0: i32, mut v1: i32, mut v2: US0, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: US1) -> US4 {
    loop {
        let mut v6: bool = v1 <= v0;
        if v6 {
            let mut v7: bool = agile_planning_filters::eoie_agile_planning_filters_valid(&*v3, &*v4);
            if v7 {
                return US4::US4_ListParsed(v2.clone(), v3.clone(), v4.clone(), v5.clone());
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("invalid planning filter (expected --load light|medium|heavy|none and --assignee agent|pair|user|none)"); } LIT.with(|lit| lit.clone()) };
                return US4::US4_ListRejected(v9.clone());
            }
        } else {
            let mut v12: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--json"); } LIT.with(|lit| lit.clone()) };
            let mut v14: bool = v12 == v13 ;
            if v14 {
                let mut v15: i32 = v0 + 1;
                let (mut v16, mut v17, mut v18, mut v19): (US0, Rc<str>, Rc<str>, US1) = json_request_18(v2.clone(), v3.clone(), v4.clone(), v5.clone());
                (v0, v1, v2, v3, v4, v5) = (v15, v1, v16.clone(), v17.clone(), v18.clone(), v19.clone());
                continue;
            } else {
                let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--load"); } LIT.with(|lit| lit.clone()) };
                let mut v22: bool = v12 == v21 ;
                if v22 {
                    let mut v23: i32 = v0 + 2i32;
                    let mut v24: i32 = v0 + 1;
                    let mut v25: Rc<str> = usize::try_from(v24).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    (v0, v1, v2, v3, v4, v5) = (v23, v1, v2.clone(), v25.clone(), v4.clone(), v5.clone());
                    continue;
                } else {
                    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--assignee"); } LIT.with(|lit| lit.clone()) };
                    let mut v28: bool = v12 == v27 ;
                    if v28 {
                        let mut v29: i32 = v0 + 2i32;
                        let mut v30: i32 = v0 + 1;
                        let mut v31: Rc<str> = usize::try_from(v30).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        (v0, v1, v2, v3, v4, v5) = (v29, v1, v2.clone(), v3.clone(), v31.clone(), v5.clone());
                        continue;
                    } else {
                        let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
                        let mut v34: bool = v12 == v33 ;
                        let mut v43: US5 = if v34 {
                            let mut v35: US0 = US0::US0_AllStatuses;
                            US5::US5_Valid(v35.clone())
                        } else {
                            let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planned|Active|Blocked|Paused|Done"); } LIT.with(|lit| lit.clone()) };
                            let mut v38: bool = v37.split("|").any(|item| item == &*v12);
                            if v38 {
                                let mut v39: US0 = US0::US0_OnlyStatus(v12.clone());
                                US5::US5_Valid(v39.clone())
                            } else {
                                US5::US5_Invalid
                            }
                        };
                        match &v43 {
                            US5::US5_Invalid => {
                                let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("invalid agile status filter: "); } LIT.with(|lit| lit.clone()) };
                                let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (expected all, Planned, Active, Blocked, Paused or Done)"); } LIT.with(|lit| lit.clone()) };
                                let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v50: Rc<str> = std::rc::Rc::<str>::from([&*v47, &*v12, &*v48, &*v49, &*v49].concat());
                                return US4::US4_ListRejected(v50.clone());
                            }
                            US5::US5_Valid(v44) => {
                                let mut v44: US0 = v44.clone();
                                let mut v45: i32 = v0 + 1;
                                (v0, v1, v2, v3, v4, v5) = (v45, v1, v44.clone(), v3.clone(), v4.clone(), v5.clone());
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
    let (mut v2, mut v3, mut v4, mut v5): (US0, Rc<str>, Rc<str>, US1) = default_list_request_16();
    let mut v6: US4 = list_request_from_17(v0, v1, v2.clone(), v3.clone(), v4.clone(), v5.clone());
    match &v6 {
        US4::US4_ListParsed(v7, v8, v9, v10) => {
            let mut v7: US0 = v7.clone();
            let mut v8: Rc<str> = v8.clone();
            let mut v9: Rc<str> = v9.clone();
            let mut v10: US1 = v10.clone();
            let mut v11: i32 = 1i32;
            let mut v12: Rc<str> = usize::try_from(v11).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            agile_list_1(v12.clone(), v7.clone(), v8.clone(), v9.clone(), v10.clone())
        }
        US4::US4_ListRejected(v14) => {
            let mut v14: Rc<str> = v14.clone();
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile list rejected: "); } LIT.with(|lit| lit.clone()) };
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v14, &*v16, &*v16, &*v16].concat());
            let mut v18: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v17 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v18
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
