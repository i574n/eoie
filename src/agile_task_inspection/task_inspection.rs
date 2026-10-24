#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_ShowText,
    US0_ShowJson,
}
#[derive(Clone)]
enum US1 {
    US1_FileReadable(Rc<str>),
    US1_FileMissing(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_ScanPlain,
    US2_ScanQuoted,
    US2_ScanEscaped,
}
#[derive(Clone)]
enum US3 {
    US3_Valid(US0),
    US3_Invalid,
}
fn task_summary_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>) -> Rc<str> {
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v9));
    let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v2));
    let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v9));
    let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v3));
    let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v4));
    let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v9));
    let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v5));
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("%  "); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v17));
    let mut v19: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v18));
    let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v6));
    v20.clone()
}
fn show_receipt_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: u64) -> Rc<str> {
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie agile show ok id="); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v0));
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" title_bytes="); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
    let mut v14: i32 = (v1.clone().len() as i32);
    let mut v15: Rc<str> = std::rc::Rc::<str>::from((v14).to_string());
    let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v15));
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v17));
    let mut v19: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v9);
    let mut v20: Rc<str> = std::rc::Rc::<str>::from((v19).to_string());
    let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v20));
    v21.clone()
}
fn show_task_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: u64) -> i32 {
    let mut v12: Rc<str> = task_summary_3(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone());
    let mut v13: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v12.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    let mut v14: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = if v14 {
        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        v15.clone()
    } else {
        v7.clone()
    };
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("deps: "); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v16));
    let mut v19: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v18.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    let mut v20: bool = v8.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v22: Rc<str> = if v20 {
        let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        v21.clone()
    } else {
        v8.clone()
    };
    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tags: "); } LIT.with(|lit| lit.clone()) };
    let mut v24: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v22));
    let mut v25: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v24.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("planning: "); } LIT.with(|lit| lit.clone()) };
    let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v9));
    let mut v28: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v27.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    let mut v29: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v1.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    let mut v30: bool = v10.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v32: i32 = if v30 {
        0i32
    } else {
        let mut v31: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v10.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
        v31
    };
    let mut v33: Rc<str> = show_receipt_4(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v11);
    let mut v34: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v33.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v34
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
fn task_json_record_at_5(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let mut v3: i32 = 0i32;
    let mut v4: i32 = field_start_6(v0.clone(), v1, v2, v3);
    let mut v5: US2 = US2::US2_ScanPlain;
    let mut v6: i32 = field_end_7(v0.clone(), v4, v2, v5.clone());
    let mut v7: Rc<str> = std::rc::Rc::<str>::from(v0.get(v4 as usize..v6 as usize).unwrap_or(""));
    let mut v8: i32 = (v7.clone().len() as i32);
    let mut v9: i32 = v8.wrapping_sub(1i32);
    let mut v10: Rc<str> = std::rc::Rc::<str>::from(v7.get(1i32 as usize..v9 as usize).unwrap_or(""));
    let mut v11: i32 = 1i32;
    let mut v12: i32 = field_start_6(v0.clone(), v1, v2, v11);
    let mut v13: US2 = US2::US2_ScanPlain;
    let mut v14: i32 = field_end_7(v0.clone(), v12, v2, v13.clone());
    let mut v15: Rc<str> = std::rc::Rc::<str>::from(v0.get(v12 as usize..v14 as usize).unwrap_or(""));
    let mut v16: i32 = (v15.clone().len() as i32);
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
    let mut v41: Rc<str> = Rc::<str>::from(v38.replace(&*v39, &*v40));
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
    let mut v52: i32 = (v51.clone().len() as i32);
    let mut v53: i32 = v52.wrapping_sub(1i32);
    let mut v54: Rc<str> = std::rc::Rc::<str>::from(v51.get(1i32 as usize..v53 as usize).unwrap_or(""));
    let mut v55: i32 = 8i32;
    let mut v56: i32 = field_start_6(v0.clone(), v1, v2, v55);
    let mut v57: US2 = US2::US2_ScanPlain;
    let mut v58: i32 = field_end_7(v0.clone(), v56, v2, v57.clone());
    let mut v59: Rc<str> = std::rc::Rc::<str>::from(v0.get(v56 as usize..v58 as usize).unwrap_or(""));
    let mut v60: i32 = (v59.clone().len() as i32);
    let mut v61: i32 = v60.wrapping_sub(1i32);
    let mut v62: Rc<str> = std::rc::Rc::<str>::from(v59.get(1i32 as usize..v61 as usize).unwrap_or(""));
    (v10.clone(), v18.clone(), v23.clone(), v28.clone(), v33.clone(), v41.clone(), v46.clone(), v54.clone(), v62.clone())
}
fn task_json_9(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: u64) -> Rc<str> {
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("id"); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v0));
    let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v12));
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"id"); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\":"); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"id\":"); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v14));
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{"); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v18));
    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("title"); } LIT.with(|lit| lit.clone()) };
    let mut v24: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v1));
    let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v12));
    let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"title"); } LIT.with(|lit| lit.clone()) };
    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"title\":"); } LIT.with(|lit| lit.clone()) };
    let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v25));
    let mut v29: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v28));
    let mut v30: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v21));
    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kind"); } LIT.with(|lit| lit.clone()) };
    let mut v32: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v2));
    let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v12));
    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"kind"); } LIT.with(|lit| lit.clone()) };
    let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"kind\":"); } LIT.with(|lit| lit.clone()) };
    let mut v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v33));
    let mut v37: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v21));
    let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("priority"); } LIT.with(|lit| lit.clone()) };
    let mut v39: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v3));
    let mut v40: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v12));
    let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"priority"); } LIT.with(|lit| lit.clone()) };
    let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"priority\":"); } LIT.with(|lit| lit.clone()) };
    let mut v43: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v40));
    let mut v44: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v43));
    let mut v45: Rc<str> = Rc::<str>::from(format!("{}{}", v44, v21));
    let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("difficulty"); } LIT.with(|lit| lit.clone()) };
    let mut v47: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v4));
    let mut v48: Rc<str> = Rc::<str>::from(format!("{}{}", v47, v12));
    let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"difficulty"); } LIT.with(|lit| lit.clone()) };
    let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"difficulty\":"); } LIT.with(|lit| lit.clone()) };
    let mut v51: Rc<str> = Rc::<str>::from(format!("{}{}", v50, v48));
    let mut v52: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v51));
    let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("progress"); } LIT.with(|lit| lit.clone()) };
    let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"progress"); } LIT.with(|lit| lit.clone()) };
    let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"progress\":"); } LIT.with(|lit| lit.clone()) };
    let mut v56: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v5));
    let mut v57: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v56));
    let mut v58: Rc<str> = Rc::<str>::from(format!("{}{}", v57, v21));
    let mut v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
    let mut v60: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v6));
    let mut v61: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v12));
    let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"status"); } LIT.with(|lit| lit.clone()) };
    let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"status\":"); } LIT.with(|lit| lit.clone()) };
    let mut v64: Rc<str> = Rc::<str>::from(format!("{}{}", v63, v61));
    let mut v65: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v64));
    let mut v66: Rc<str> = Rc::<str>::from(format!("{}{}", v65, v21));
    let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("deps"); } LIT.with(|lit| lit.clone()) };
    let mut v68: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v7));
    let mut v69: Rc<str> = Rc::<str>::from(format!("{}{}", v68, v12));
    let mut v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"deps"); } LIT.with(|lit| lit.clone()) };
    let mut v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"deps\":"); } LIT.with(|lit| lit.clone()) };
    let mut v72: Rc<str> = Rc::<str>::from(format!("{}{}", v71, v69));
    let mut v73: Rc<str> = Rc::<str>::from(format!("{}{}", v72, v21));
    let mut v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tags"); } LIT.with(|lit| lit.clone()) };
    let mut v75: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v8));
    let mut v76: Rc<str> = Rc::<str>::from(format!("{}{}", v75, v12));
    let mut v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"tags"); } LIT.with(|lit| lit.clone()) };
    let mut v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"tags\":"); } LIT.with(|lit| lit.clone()) };
    let mut v79: Rc<str> = Rc::<str>::from(format!("{}{}", v78, v76));
    let mut v80: Rc<str> = Rc::<str>::from(format!("{}{}", v73, v79));
    let mut v81: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v9));
    let mut v82: Rc<str> = Rc::<str>::from(format!("{}{}", v81, v21));
    let mut v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("elapsed_ms"); } LIT.with(|lit| lit.clone()) };
    let mut v84: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v10);
    let mut v85: Rc<str> = std::rc::Rc::<str>::from((v84).to_string());
    let mut v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"elapsed_ms"); } LIT.with(|lit| lit.clone()) };
    let mut v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"elapsed_ms\":"); } LIT.with(|lit| lit.clone()) };
    let mut v88: Rc<str> = Rc::<str>::from(format!("{}{}", v87, v85));
    let mut v89: Rc<str> = Rc::<str>::from(format!("{}{}", v82, v88));
    let mut v90: Rc<str> = Rc::<str>::from(format!("{}{}", v80, v89));
    let mut v91: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v52));
    let mut v92: Rc<str> = Rc::<str>::from(format!("{}{}", v91, v66));
    let mut v93: Rc<str> = Rc::<str>::from(format!("{}{}", v92, v90));
    let mut v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
    let mut v95: Rc<str> = Rc::<str>::from(format!("{}{}", v93, v94));
    v95.clone()
}
fn unescaped_from_12(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let mut v5: bool = v3 <= v1;
        if v5 {
            let mut v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v3 as usize).unwrap_or(""));
            let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v6));
            return v7.clone();
        } else {
            let mut v8: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = v8 == 92i32;
            if v9 {
                let mut v10: i32 = v1.wrapping_add(1i32);
                let mut v11: i32 = v10.wrapping_add(1i32);
                let mut v12: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v1 as usize).unwrap_or(""));
                let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v12));
                let mut v14: i32 = v0.as_bytes().get(v10 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let mut v15: bool = v14 == 110i32;
                let mut v24: Rc<str> = if v15 {
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    v16.clone()
                } else {
                    let mut v17: bool = v14 == 114i32;
                    if v17 {
                        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                        v18.clone()
                    } else {
                        let mut v19: bool = v14 == 116i32;
                        if v19 {
                            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            v20.clone()
                        } else {
                            let mut v21: Rc<str> = std::rc::Rc::<str>::from(v0.get(v10 as usize..v11 as usize).unwrap_or(""));
                            v21.clone()
                        }
                    }
                };
                let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v24));
                (v0, v1, v2, v3, v4) = (v0.clone(), v11, v11, v3, v25.clone());
                continue;
            } else {
                let mut v27: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2, v3, v4) = (v0.clone(), v27, v2, v3, v4.clone());
                continue;
            }
        }
    }
}
fn unquoted_11(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    let mut v3: Rc<str> = std::rc::Rc::<str>::from(v0.get(1i32 as usize..v2 as usize).unwrap_or(""));
    let mut v4: i32 = 0i32;
    let mut v5: i32 = 0i32;
    let mut v6: i32 = (v3.clone().len() as i32);
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    unescaped_from_12(v3.clone(), v4, v5, v6, v7.clone())
}
fn task_record_at_10(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let mut v3: i32 = 0i32;
    let mut v4: i32 = field_start_6(v0.clone(), v1, v2, v3);
    let mut v5: US2 = US2::US2_ScanPlain;
    let mut v6: i32 = field_end_7(v0.clone(), v4, v2, v5.clone());
    let mut v7: Rc<str> = std::rc::Rc::<str>::from(v0.get(v4 as usize..v6 as usize).unwrap_or(""));
    let mut v8: Rc<str> = unquoted_11(v7.clone());
    let mut v9: i32 = 1i32;
    let mut v10: i32 = field_start_6(v0.clone(), v1, v2, v9);
    let mut v11: US2 = US2::US2_ScanPlain;
    let mut v12: i32 = field_end_7(v0.clone(), v10, v2, v11.clone());
    let mut v13: Rc<str> = std::rc::Rc::<str>::from(v0.get(v10 as usize..v12 as usize).unwrap_or(""));
    let mut v14: Rc<str> = unquoted_11(v13.clone());
    let mut v15: i32 = 2i32;
    let mut v16: i32 = field_start_6(v0.clone(), v1, v2, v15);
    let mut v17: US2 = US2::US2_ScanPlain;
    let mut v18: i32 = field_end_7(v0.clone(), v16, v2, v17.clone());
    let mut v19: Rc<str> = std::rc::Rc::<str>::from(v0.get(v16 as usize..v18 as usize).unwrap_or(""));
    let mut v20: i32 = 3i32;
    let mut v21: i32 = field_start_6(v0.clone(), v1, v2, v20);
    let mut v22: US2 = US2::US2_ScanPlain;
    let mut v23: i32 = field_end_7(v0.clone(), v21, v2, v22.clone());
    let mut v24: Rc<str> = std::rc::Rc::<str>::from(v0.get(v21 as usize..v23 as usize).unwrap_or(""));
    let mut v25: i32 = 4i32;
    let mut v26: i32 = field_start_6(v0.clone(), v1, v2, v25);
    let mut v27: US2 = US2::US2_ScanPlain;
    let mut v28: i32 = field_end_7(v0.clone(), v26, v2, v27.clone());
    let mut v29: Rc<str> = std::rc::Rc::<str>::from(v0.get(v26 as usize..v28 as usize).unwrap_or(""));
    let mut v30: i32 = 5i32;
    let mut v31: i32 = field_start_6(v0.clone(), v1, v2, v30);
    let mut v32: US2 = US2::US2_ScanPlain;
    let mut v33: i32 = field_end_7(v0.clone(), v31, v2, v32.clone());
    let mut v34: Rc<str> = std::rc::Rc::<str>::from(v0.get(v31 as usize..v33 as usize).unwrap_or(""));
    let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
    let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v37: Rc<str> = Rc::<str>::from(v34.replace(&*v35, &*v36));
    let mut v38: i32 = 6i32;
    let mut v39: i32 = field_start_6(v0.clone(), v1, v2, v38);
    let mut v40: US2 = US2::US2_ScanPlain;
    let mut v41: i32 = field_end_7(v0.clone(), v39, v2, v40.clone());
    let mut v42: Rc<str> = std::rc::Rc::<str>::from(v0.get(v39 as usize..v41 as usize).unwrap_or(""));
    let mut v43: i32 = 7i32;
    let mut v44: i32 = field_start_6(v0.clone(), v1, v2, v43);
    let mut v45: US2 = US2::US2_ScanPlain;
    let mut v46: i32 = field_end_7(v0.clone(), v44, v2, v45.clone());
    let mut v47: Rc<str> = std::rc::Rc::<str>::from(v0.get(v44 as usize..v46 as usize).unwrap_or(""));
    let mut v48: Rc<str> = unquoted_11(v47.clone());
    let mut v49: i32 = 8i32;
    let mut v50: i32 = field_start_6(v0.clone(), v1, v2, v49);
    let mut v51: US2 = US2::US2_ScanPlain;
    let mut v52: i32 = field_end_7(v0.clone(), v50, v2, v51.clone());
    let mut v53: Rc<str> = std::rc::Rc::<str>::from(v0.get(v50 as usize..v52 as usize).unwrap_or(""));
    let mut v54: Rc<str> = unquoted_11(v53.clone());
    (v8.clone(), v14.clone(), v19.clone(), v24.clone(), v29.clone(), v37.clone(), v42.clone(), v48.clone(), v54.clone())
}
fn agile_show_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: US0) -> i32 {
    let mut v3: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v4, &*v5, &*v5, &*v5].concat());
    let mut v7: bool = std::path::Path::new(&*v6).is_file();
    let mut v10: US1 = if v7 {
        US1::US1_FileReadable(v6.clone())
    } else {
        US1::US1_FileMissing(v6.clone())
    };
    match &v10 {
        US1::US1_FileMissing(v11) => {
            let mut v11: Rc<str> = v11.clone();
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile show "); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v1));
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
            let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v15));
            let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v12));
            let mut v18: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v17 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v18
        }
        US1::US1_FileReadable(v19) => {
            let mut v19: Rc<str> = v19.clone();
            let mut v20: Rc<str> = std::fs::read_to_string(&*v19).unwrap_or_default().into();
            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
            let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v1));
            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
            let mut v24: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v23));
            let mut v25: i32 = v20.find(&*v24).map(|offset| offset as i32).unwrap_or(-1);
            let mut v26: bool = v25 < 0i32;
            if v26 {
                let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no task with this id"); } LIT.with(|lit| lit.clone()) };
                let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile show "); } LIT.with(|lit| lit.clone()) };
                let mut v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v1));
                let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let mut v31: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v30));
                let mut v32: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v27));
                let mut v33: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v32 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v33
            } else {
                let mut v34: i32 = v25 + 6i32;
                match &v2 {
                    US0::US0_ShowJson => {
                        let mut v48: i32 = (v20.clone().len() as i32);
                        let (mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = task_json_record_at_5(v20.clone(), v34, v48);
                        let mut v58: Rc<str> = agile_planning_fields::eoie_agile_planning_json(&*v20, &*v1);
                        let mut v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                        let mut v60: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v59));
                        let mut v61: Rc<str> = agile_note_listing::eoie_agile_notes_json(&*v20, &*v1);
                        let mut v62: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v61));
                        let mut v63: Rc<str> = task_json_9(v49.clone(), v50.clone(), v51.clone(), v52.clone(), v53.clone(), v54.clone(), v55.clone(), v56.clone(), v57.clone(), v62.clone(), v3);
                        let mut v64: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v63.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                        v64
                    }
                    US0::US0_ShowText => {
                        let mut v35: i32 = (v20.clone().len() as i32);
                        let (mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = task_record_at_10(v20.clone(), v34, v35);
                        let mut v45: Rc<str> = agile_planning_fields::eoie_agile_planning_summary(&*v20, &*v1);
                        let mut v46: Rc<str> = agile_note_listing::eoie_agile_notes_text(&*v20, &*v1);
                        show_task_2(v36.clone(), v37.clone(), v38.clone(), v39.clone(), v40.clone(), v41.clone(), v42.clone(), v43.clone(), v44.clone(), v45.clone(), v46.clone(), v3)
                    }
                }
            }
        }
    }
}
fn command_line_argument_count_14() -> i32 {
    let mut v0: i32 = (std::env::args().count() as i32) - 1;
    v0
}
fn argument_count_13() -> i32 {
    let mut v0: i32 = command_line_argument_count_14();
    let mut v1: bool = v0 > 0i32;
    if v1 {
        let mut v2: i32 = v0.wrapping_sub(1i32);
        v2
    } else {
        0i32
    }
}
fn agile_show_entry_0() -> i32 {
    let mut v0: i32 = argument_count_13();
    let mut v1: bool = v0 == 3i32;
    if v1 {
        let mut v2: i32 = 1i32;
        let mut v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v4: i32 = 2i32;
        let mut v5: Rc<str> = usize::try_from(v4).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v6: US0 = US0::US0_ShowText;
        agile_show_1(v3.clone(), v5.clone(), v6.clone())
    } else {
        let mut v8: i32 = 3i32;
        let mut v9: Rc<str> = usize::try_from(v8).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v10: bool = v9.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--json"); } LIT.with(|lit| lit.clone()) };
        let mut v14: US3 = if v10 {
            let mut v11: US0 = US0::US0_ShowJson;
            US3::US3_Valid(v11.clone())
        } else {
            US3::US3_Invalid
        };
        match &v14 {
            US3::US3_Invalid => {
                let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the only option is --json"); } LIT.with(|lit| lit.clone()) };
                let mut v22: i32 = 2i32;
                let mut v23: Rc<str> = usize::try_from(v22).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile show "); } LIT.with(|lit| lit.clone()) };
                let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v23));
                let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v26));
                let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v21));
                let mut v29: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v28 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v29
            }
            US3::US3_Valid(v15) => {
                let mut v15: US0 = v15.clone();
                let mut v16: i32 = 1i32;
                let mut v17: Rc<str> = usize::try_from(v16).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let mut v18: i32 = 2i32;
                let mut v19: Rc<str> = usize::try_from(v18).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                agile_show_1(v17.clone(), v19.clone(), v15.clone())
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        agile_show_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_show_run() -> i32 {
    closure0()()
}
