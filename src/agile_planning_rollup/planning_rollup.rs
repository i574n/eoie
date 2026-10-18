#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_RollupText,
    US0_RollupJson,
}
#[derive(Clone)]
enum US1 {
    US1_ScanPlain,
    US1_ScanQuoted,
    US1_ScanEscaped,
}
#[derive(Clone)]
enum US2 {
    US2_FileReadable(Rc<str>),
    US2_FileMissing(Rc<str>),
}
#[derive(Clone)]
enum US3 {
    US3_Valid(US0),
    US3_Invalid,
}
fn tally_of_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> (i32, i32, i32, i32, i32) {
    let (mut v3, mut v4, mut v5, mut v6, mut v7): (i32, i32, i32, i32, i32) = agile_planning_tally::eoie_agile_planning_tally(&*v0, &*v1, &*v2);
    (v3, v4, v5, v6, v7)
}
fn tally_json_6(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32) -> Rc<str> {
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tasks"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\""); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\":"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v5, &*v7, &*v8, &*v9].concat());
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open_tasks"); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v11, &*v7, &*v12, &*v9].concat());
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("estimated"); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
    let mut v16: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v14, &*v7, &*v15, &*v9].concat());
    let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v13, &*v16, &*v9, &*v9].concat());
    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("minutes"); } LIT.with(|lit| lit.clone()) };
    let mut v19: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let mut v20: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v18, &*v7, &*v19, &*v9].concat());
    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open_minutes"); } LIT.with(|lit| lit.clone()) };
    let mut v22: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
    let mut v23: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v21, &*v7, &*v22, &*v9].concat());
    let mut v24: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v20, &*v23, &*v9, &*v9].concat());
    v24.clone()
}
fn tally_columns_7(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32) -> Rc<str> {
    let mut v5: Rc<str> = { let text = (v0).to_string(); let filler = (5i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(" ".repeat(filler) + &text) };
    let mut v6: Rc<str> = { let text = (v1).to_string(); let filler = (7i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(" ".repeat(filler) + &text) };
    let mut v7: Rc<str> = { let text = (v2).to_string(); let filler = (11i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(" ".repeat(filler) + &text) };
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v6, &*v7, &*v8, &*v8].concat());
    let mut v10: Rc<str> = { let text = (v3).to_string(); let filler = (9i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(" ".repeat(filler) + &text) };
    let mut v11: Rc<str> = { let text = (v4).to_string(); let filler = (14i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(" ".repeat(filler) + &text) };
    let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v10, &*v11, &*v8, &*v8].concat());
    v12.clone()
}
fn rollup_line_5(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: bool) -> Rc<str> {
    match &v0 {
        US0::US0_RollupJson => {
            let mut v16: Rc<str> = if v8 {
                let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v14.clone()
            } else {
                let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                v15.clone()
            };
            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"group\":\""); } LIT.with(|lit| lit.clone()) };
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\",\"value\":\""); } LIT.with(|lit| lit.clone()) };
            let mut v19: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v17, &*v1, &*v18, &*v2].concat());
            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let mut v21: Rc<str> = tally_json_6(v3, v4, v5, v6, v7);
            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v24: Rc<str> = std::rc::Rc::<str>::from([&*v19, &*v20, &*v21, &*v22, &*v23].concat());
            v24.clone()
        }
        US0::US0_RollupText => {
            let mut v9: Rc<str> = { let text = v1; let filler = (11i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
            let mut v10: Rc<str> = { let text = v2; let filler = (28i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
            let mut v11: Rc<str> = tally_columns_7(v3, v4, v5, v6, v7);
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v10, &*v11, &*v12, &*v12].concat());
            v13.clone()
        }
    }
}
fn print_group_rows_3(mut v0: Rc<str>, mut v1: US0, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: bool) -> i32 {
    loop {
        let mut v6: i32 = v3.len() as i32;
        let mut v7: bool = v6 <= v4;
        if v7 {
            return 0i32;
        } else {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let mut v9: i32 = v3.get(v4 as usize..).and_then(|rest| rest.find(&*v8)).map(|found| (found + v4 as usize) as i32).unwrap_or(-1);
            let mut v10: bool = v9 < 0i32;
            let mut v12: i32 = if v10 {
                let mut v11: i32 = v3.len() as i32;
                v11
            } else {
                v9
            };
            let mut v13: Rc<str> = std::rc::Rc::<str>::from(v3.get(v4 as usize..v12 as usize).unwrap_or(""));
            let (mut v14, mut v15, mut v16, mut v17, mut v18): (i32, i32, i32, i32, i32) = tally_of_4(v0.clone(), v2.clone(), v13.clone());
            let mut v19: Rc<str> = rollup_line_5(v1.clone(), v2.clone(), v13.clone(), v14, v15, v16, v17, v18, v5);
            let mut v20: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v19.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
            let mut v21: i32 = v12.wrapping_add(1i32);
            let mut v22: bool = false;
            (v0, v1, v2, v3, v4, v5) = (v0.clone(), v1.clone(), v2.clone(), v3.clone(), v21, v22);
            continue;
        }
    }
}
fn field_end_11(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US1) -> i32 {
    loop {
        let mut v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let mut v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = match &v3 {
                US1::US1_ScanPlain => {
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
                let mut v25: US1 = match &v3 {
                    US1::US1_ScanEscaped => {
                        US1::US1_ScanQuoted
                    }
                    US1::US1_ScanPlain => {
                        let mut v19: bool = v5 == 34i32;
                        if v19 {
                            US1::US1_ScanQuoted
                        } else {
                            US1::US1_ScanPlain
                        }
                    }
                    US1::US1_ScanQuoted => {
                        let mut v12: bool = v5 == 92i32;
                        if v12 {
                            US1::US1_ScanEscaped
                        } else {
                            let mut v14: bool = v5 == 34i32;
                            if v14 {
                                US1::US1_ScanPlain
                            } else {
                                US1::US1_ScanQuoted
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
fn skip_spaces_12(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn field_start_10(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let mut v5: US1 = US1::US1_ScanPlain;
            let mut v6: i32 = field_end_11(v0.clone(), v1, v2, v5.clone());
            let mut v7: i32 = v6.wrapping_add(1i32);
            let mut v8: i32 = skip_spaces_12(v0.clone(), v7);
            let mut v9: i32 = v3.wrapping_sub(1i32);
            (v0, v1, v2, v3) = (v0.clone(), v8, v2, v9);
            continue;
        }
    }
}
fn epic_ids_from_9(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
        let mut v5: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v4)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let mut v6: bool = v5 < 0i32;
        if v6 {
            return v3.clone();
        } else {
            let mut v7: i32 = v5 + 6i32;
            let mut v8: i32 = v0.get(..v5 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let mut v9: Rc<str> = std::rc::Rc::<str>::from(v0.get(v8 as usize..v5 as usize).unwrap_or(""));
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let mut v11: bool = v9.starts_with(&*v10);
            if v11 {
                let mut v12: i32 = 2i32;
                let mut v13: i32 = field_start_10(v0.clone(), v7, v2, v12);
                let mut v14: US1 = US1::US1_ScanPlain;
                let mut v15: i32 = field_end_11(v0.clone(), v13, v2, v14.clone());
                let mut v16: Rc<str> = std::rc::Rc::<str>::from(v0.get(v13 as usize..v15 as usize).unwrap_or(""));
                let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Epic"); } LIT.with(|lit| lit.clone()) };
                let mut v18: bool = v16 == v17 ;
                if v18 {
                    let mut v19: i32 = 0i32;
                    let mut v20: i32 = field_start_10(v0.clone(), v7, v2, v19);
                    let mut v21: US1 = US1::US1_ScanPlain;
                    let mut v22: i32 = field_end_11(v0.clone(), v20, v2, v21.clone());
                    let mut v23: Rc<str> = std::rc::Rc::<str>::from(v0.get(v20 as usize..v22 as usize).unwrap_or(""));
                    let mut v24: i32 = v23.len() as i32;
                    let mut v25: i32 = v24.wrapping_sub(1i32);
                    let mut v26: Rc<str> = std::rc::Rc::<str>::from(v23.get(1i32 as usize..v25 as usize).unwrap_or(""));
                    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v28: bool = v3 == v27 ;
                    let mut v31: Rc<str> = if v28 {
                        v26.clone()
                    } else {
                        let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                        let mut v30: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v29, &*v26, &*v27, &*v27].concat());
                        v30.clone()
                    };
                    (v0, v1, v2, v3) = (v0.clone(), v7, v2, v31.clone());
                    continue;
                } else {
                    (v0, v1, v2, v3) = (v0.clone(), v7, v2, v3.clone());
                    continue;
                }
            } else {
                (v0, v1, v2, v3) = (v0.clone(), v7, v2, v3.clone());
                continue;
            }
        }
    }
}
fn planning_epic_ids_8(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = 0i32;
    let mut v2: i32 = v0.len() as i32;
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    epic_ids_from_9(v0.clone(), v1, v2, v3.clone())
}
fn rollup_line_14(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: bool) -> Rc<str> {
    match &v0 {
        US0::US0_RollupJson => {
            let mut v17: Rc<str> = if v9 {
                let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v15.clone()
            } else {
                let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                v16.clone()
            };
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"group\":\""); } LIT.with(|lit| lit.clone()) };
            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\",\"value\":\""); } LIT.with(|lit| lit.clone()) };
            let mut v20: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v18, &*v2, &*v19, &*v3].concat());
            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let mut v22: Rc<str> = tally_json_6(v4, v5, v6, v7, v8);
            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v25: Rc<str> = std::rc::Rc::<str>::from([&*v20, &*v21, &*v22, &*v23, &*v24].concat());
            v25.clone()
        }
        US0::US0_RollupText => {
            let mut v10: Rc<str> = { let text = v2; let filler = (11i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
            let mut v11: Rc<str> = { let text = v3; let filler = (28i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
            let mut v12: Rc<str> = tally_columns_7(v4, v5, v6, v7, v8);
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v11, &*v12, &*v13, &*v13].concat());
            v14.clone()
        }
    }
}
fn print_group_rows_13(mut v0: Rc<str>, mut v1: US0, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: i32, mut v6: bool) -> i32 {
    loop {
        let mut v7: i32 = v4.len() as i32;
        let mut v8: bool = v7 <= v5;
        if v8 {
            return 0i32;
        } else {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let mut v10: i32 = v4.get(v5 as usize..).and_then(|rest| rest.find(&*v9)).map(|found| (found + v5 as usize) as i32).unwrap_or(-1);
            let mut v11: bool = v10 < 0i32;
            let mut v13: i32 = if v11 {
                let mut v12: i32 = v4.len() as i32;
                v12
            } else {
                v10
            };
            let mut v14: Rc<str> = std::rc::Rc::<str>::from(v4.get(v5 as usize..v13 as usize).unwrap_or(""));
            let (mut v15, mut v16, mut v17, mut v18, mut v19): (i32, i32, i32, i32, i32) = tally_of_4(v0.clone(), v2.clone(), v14.clone());
            let mut v20: Rc<str> = rollup_line_14(v1.clone(), v2.clone(), v3.clone(), v14.clone(), v15, v16, v17, v18, v19, v6);
            let mut v21: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v20.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
            let mut v22: i32 = v13.wrapping_add(1i32);
            let mut v23: bool = false;
            (v0, v1, v2, v3, v4, v5, v6) = (v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v22, v23);
            continue;
        }
    }
}
fn print_rollup_2(mut v0: Rc<str>, mut v1: US0, mut v2: u64) -> i32 {
    let mut v12: Rc<str> = match &v1 {
        US0::US0_RollupJson => {
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"groups\":["); } LIT.with(|lit| lit.clone()) };
            v10.clone()
        }
        US0::US0_RollupText => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("GROUP"); } LIT.with(|lit| lit.clone()) };
            let mut v4: Rc<str> = { let text = v3; let filler = (11i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("VALUE"); } LIT.with(|lit| lit.clone()) };
            let mut v6: Rc<str> = { let text = v5; let filler = (28i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TASKS   OPEN  ESTIMATED  MINUTES  OPEN_MINUTES"); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v9: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v6, &*v7, &*v8, &*v8].concat());
            v9.clone()
        }
    };
    let mut v13: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v12.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planned,Active,Blocked,Paused,Done"); } LIT.with(|lit| lit.clone()) };
    let mut v16: i32 = 0i32;
    let mut v17: bool = true;
    let mut v18: i32 = print_group_rows_3(v0.clone(), v1.clone(), v14.clone(), v15.clone(), v16, v17);
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("load"); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("light,medium,heavy,none"); } LIT.with(|lit| lit.clone()) };
    let mut v21: i32 = 0i32;
    let mut v22: bool = false;
    let mut v23: i32 = print_group_rows_3(v0.clone(), v1.clone(), v19.clone(), v20.clone(), v21, v22);
    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("assignee"); } LIT.with(|lit| lit.clone()) };
    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agent,pair,user,none"); } LIT.with(|lit| lit.clone()) };
    let mut v26: i32 = 0i32;
    let mut v27: bool = false;
    let mut v28: i32 = print_group_rows_3(v0.clone(), v1.clone(), v24.clone(), v25.clone(), v26, v27);
    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("epic"); } LIT.with(|lit| lit.clone()) };
    let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("epic+deps"); } LIT.with(|lit| lit.clone()) };
    let mut v31: Rc<str> = planning_epic_ids_8(v0.clone());
    let mut v32: i32 = 0i32;
    let mut v33: bool = false;
    let mut v34: i32 = print_group_rows_13(v0.clone(), v1.clone(), v29.clone(), v30.clone(), v31.clone(), v32, v33);
    let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
    let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let (mut v37, mut v38, mut v39, mut v40, mut v41): (i32, i32, i32, i32, i32) = tally_of_4(v0.clone(), v35.clone(), v36.clone());
    let mut v66: Rc<str> = match &v1 {
        US0::US0_RollupJson => {
            let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]"); } LIT.with(|lit| lit.clone()) };
            let mut v59: Rc<str> = tally_json_6(v37, v38, v39, v40, v41);
            let mut v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"elapsed_ms\":"); } LIT.with(|lit| lit.clone()) };
            let mut v61: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v2);
            let mut v62: Rc<str> = std::rc::Rc::<str>::from((v61).to_string());
            let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
            let mut v64: Rc<str> = std::rc::Rc::<str>::from([&*v58, &*v59, &*v60, &*v62, &*v63].concat());
            v64.clone()
        }
        US0::US0_RollupText => {
            let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie agile plan ok tasks="); } LIT.with(|lit| lit.clone()) };
            let mut v43: Rc<str> = std::rc::Rc::<str>::from((v37).to_string());
            let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" open="); } LIT.with(|lit| lit.clone()) };
            let mut v45: Rc<str> = std::rc::Rc::<str>::from((v38).to_string());
            let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" estimated="); } LIT.with(|lit| lit.clone()) };
            let mut v47: Rc<str> = std::rc::Rc::<str>::from([&*v42, &*v43, &*v44, &*v45, &*v46].concat());
            let mut v48: Rc<str> = std::rc::Rc::<str>::from((v39).to_string());
            let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" minutes="); } LIT.with(|lit| lit.clone()) };
            let mut v50: Rc<str> = std::rc::Rc::<str>::from((v40).to_string());
            let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" open_minutes="); } LIT.with(|lit| lit.clone()) };
            let mut v52: Rc<str> = std::rc::Rc::<str>::from((v41).to_string());
            let mut v53: Rc<str> = std::rc::Rc::<str>::from([&*v48, &*v49, &*v50, &*v51, &*v52].concat());
            let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
            let mut v55: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v2);
            let mut v56: Rc<str> = std::rc::Rc::<str>::from((v55).to_string());
            let mut v57: Rc<str> = std::rc::Rc::<str>::from([&*v47, &*v53, &*v54, &*v56, &*v36].concat());
            v57.clone()
        }
    };
    let mut v67: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v66.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v67
}
fn planning_rollup_1(mut v0: Rc<str>, mut v1: US0) -> i32 {
    let mut v2: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v3, &*v4, &*v4, &*v4].concat());
    let mut v6: bool = std::path::Path::new(&*v5).is_file();
    let mut v9: US2 = if v6 {
        US2::US2_FileReadable(v5.clone())
    } else {
        US2::US2_FileMissing(v5.clone())
    };
    match &v9 {
        US2::US2_FileMissing(v10) => {
            let mut v10: Rc<str> = v10.clone();
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile plan rejected: "); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v11, &*v4, &*v4, &*v4].concat());
            let mut v14: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v13 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v14
        }
        US2::US2_FileReadable(v15) => {
            let mut v15: Rc<str> = v15.clone();
            let mut v16: Rc<str> = std::fs::read_to_string(&*v15).unwrap_or_default().into();
            print_rollup_2(v16.clone(), v1.clone(), v2)
        }
    }
}
fn planning_rollup_entry_0() -> i32 {
    let mut v0: i32 = std::env::args().skip(2).count() as i32;
    let mut v1: bool = v0 == 2i32;
    if v1 {
        let mut v2: i32 = 1i32;
        let mut v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v4: US0 = US0::US0_RollupText;
        planning_rollup_1(v3.clone(), v4.clone())
    } else {
        let mut v6: bool = v0 == 3i32;
        if v6 {
            let mut v7: i32 = 2i32;
            let mut v8: Rc<str> = usize::try_from(v7).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--json"); } LIT.with(|lit| lit.clone()) };
            let mut v10: bool = v8 == v9 ;
            let mut v14: US3 = if v10 {
                let mut v11: US0 = US0::US0_RollupJson;
                US3::US3_Valid(v11.clone())
            } else {
                US3::US3_Invalid
            };
            match &v14 {
                US3::US3_Invalid => {
                    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("plan expects <root> [--json]"); } LIT.with(|lit| lit.clone()) };
                    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile plan rejected: "); } LIT.with(|lit| lit.clone()) };
                    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v22: Rc<str> = std::rc::Rc::<str>::from([&*v20, &*v19, &*v21, &*v21, &*v21].concat());
                    let mut v23: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v22 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v23
                }
                US3::US3_Valid(v15) => {
                    let mut v15: US0 = v15.clone();
                    let mut v16: i32 = 1i32;
                    let mut v17: Rc<str> = usize::try_from(v16).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    planning_rollup_1(v17.clone(), v15.clone())
                }
            }
        } else {
            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("plan expects <root> [--json]"); } LIT.with(|lit| lit.clone()) };
            let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile plan rejected: "); } LIT.with(|lit| lit.clone()) };
            let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v29: Rc<str> = std::rc::Rc::<str>::from([&*v27, &*v26, &*v28, &*v28, &*v28].concat());
            let mut v30: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v29 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v30
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        planning_rollup_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_planning_rollup_run() -> i32 {
    closure0()()
}
