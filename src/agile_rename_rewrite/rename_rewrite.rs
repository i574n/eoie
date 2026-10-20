#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_ScanPlain,
    US0_ScanQuoted,
    US0_ScanEscaped,
}
fn field_end_5(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US0) -> i32 {
    loop {
        let mut v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let mut v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = match &v3 {
                US0::US0_ScanPlain => {
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
                let mut v25: US0 = match &v3 {
                    US0::US0_ScanEscaped => {
                        US0::US0_ScanQuoted
                    }
                    US0::US0_ScanPlain => {
                        let mut v19: bool = v5 == 34i32;
                        if v19 {
                            US0::US0_ScanQuoted
                        } else {
                            US0::US0_ScanPlain
                        }
                    }
                    US0::US0_ScanQuoted => {
                        let mut v12: bool = v5 == 92i32;
                        if v12 {
                            US0::US0_ScanEscaped
                        } else {
                            let mut v14: bool = v5 == 34i32;
                            if v14 {
                                US0::US0_ScanPlain
                            } else {
                                US0::US0_ScanQuoted
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
fn skip_spaces_6(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn field_start_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let mut v5: US0 = US0::US0_ScanPlain;
            let mut v6: i32 = field_end_5(v0.clone(), v1, v2, v5.clone());
            let mut v7: i32 = v6.wrapping_add(1i32);
            let mut v8: i32 = skip_spaces_6(v0.clone(), v7);
            let mut v9: i32 = v3.wrapping_sub(1i32);
            (v0, v1, v2, v3) = (v0.clone(), v8, v2, v9);
            continue;
        }
    }
}
fn unescaped_from_8(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let mut v5: bool = v3 <= v1;
        if v5 {
            let mut v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v3 as usize).unwrap_or(""));
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v6, &*v7, &*v7, &*v7].concat());
            return v8.clone();
        } else {
            let mut v9: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v10: bool = v9 == 92i32;
            if v10 {
                let mut v11: i32 = v1.wrapping_add(1i32);
                let mut v12: i32 = v11.wrapping_add(1i32);
                let mut v13: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v1 as usize).unwrap_or(""));
                let mut v14: i32 = v0.as_bytes().get(v11 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
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
                            let mut v21: Rc<str> = std::rc::Rc::<str>::from(v0.get(v11 as usize..v12 as usize).unwrap_or(""));
                            v21.clone()
                        }
                    }
                };
                let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v26: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v13, &*v24, &*v25, &*v25].concat());
                (v0, v1, v2, v3, v4) = (v0.clone(), v12, v12, v3, v26.clone());
                continue;
            } else {
                let mut v28: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2, v3, v4) = (v0.clone(), v28, v2, v3, v4.clone());
                continue;
            }
        }
    }
}
fn unquoted_7(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = v0.len() as i32;
    let mut v2: i32 = v1.wrapping_sub(1i32);
    let mut v3: Rc<str> = std::rc::Rc::<str>::from(v0.get(1i32 as usize..v2 as usize).unwrap_or(""));
    let mut v4: i32 = 0i32;
    let mut v5: i32 = 0i32;
    let mut v6: i32 = v3.len() as i32;
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    unescaped_from_8(v3.clone(), v4, v5, v6, v7.clone())
}
fn deps_renamed_9(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: bool) -> (Rc<str>, bool) {
    loop {
        let mut v7: bool = v2 <= v1;
        if v7 {
            return (v5.clone(), v6);
        } else {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let mut v9: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v8)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
            let mut v10: bool = v9 < 0i32;
            let mut v11: i32 = if v10 {
                v2
            } else {
                v9
            };
            let mut v13: i32 = if v10 {
                v2
            } else {
                let mut v12: i32 = v9.wrapping_add(1i32);
                v12
            };
            let mut v14: Rc<str> = std::rc::Rc::<str>::from(v0.get(v1 as usize..v11 as usize).unwrap_or(""));
            let mut v15: Rc<str> = std::rc::Rc::<str>::from(v14.trim());
            let mut v16: bool = v15 == v3 ;
            let mut v17: Rc<str> = if v16 {
                v4.clone()
            } else {
                v15.clone()
            };
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v19: bool = v5 == v18 ;
            let mut v21: Rc<str> = if v19 {
                v17.clone()
            } else {
                let mut v20: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v8, &*v17, &*v18, &*v18].concat());
                v20.clone()
            };
            let mut v22: bool = v16 || v6;
            (v0, v1, v2, v3, v4, v5, v6) = (v0.clone(), v13, v2, v3.clone(), v4.clone(), v21.clone(), v22);
            continue;
        }
    }
}
fn dependent_line_3(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: Rc<str>, mut v6: i32, mut v7: Rc<str>, mut v8: i32) -> (Rc<str>, i32, Rc<str>, i32, Rc<str>, i32) {
    let mut v9: i32 = v0.len() as i32;
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
    let mut v11: i32 = v10.len() as i32;
    let mut v12: i32 = v1 + v11;
    let mut v13: i32 = 0i32;
    let mut v14: i32 = field_start_4(v0.clone(), v12, v9, v13);
    let mut v15: US0 = US0::US0_ScanPlain;
    let mut v16: i32 = field_end_5(v0.clone(), v14, v9, v15.clone());
    let mut v17: Rc<str> = std::rc::Rc::<str>::from(v0.get(v14 as usize..v16 as usize).unwrap_or(""));
    let mut v18: Rc<str> = unquoted_7(v17.clone());
    let mut v19: i32 = v0.len() as i32;
    let mut v20: i32 = v10.len() as i32;
    let mut v21: i32 = v1 + v20;
    let mut v22: i32 = 1i32;
    let mut v23: i32 = field_start_4(v0.clone(), v21, v19, v22);
    let mut v24: US0 = US0::US0_ScanPlain;
    let mut v25: i32 = field_end_5(v0.clone(), v23, v19, v24.clone());
    let mut v26: Rc<str> = std::rc::Rc::<str>::from(v0.get(v23 as usize..v25 as usize).unwrap_or(""));
    let mut v27: i32 = v26.find(&*v2).map(|offset| offset as i32).unwrap_or(-1);
    let mut v28: bool = v27 < 0i32;
    let mut v29: bool = v28 == false;
    let (mut v36, mut v37, mut v38, mut v39, mut v40): (i32, Rc<str>, i32, Rc<str>, i32) = if v29 {
        let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v31: bool = v7 == v30 ;
        let mut v34: Rc<str> = if v31 {
            v18.clone()
        } else {
            let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let mut v33: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v32, &*v18, &*v30, &*v30].concat());
            v33.clone()
        };
        let mut v35: i32 = v8.wrapping_add(1i32);
        (v4, v5.clone(), v6, v34.clone(), v35)
    } else {
        (v4, v5.clone(), v6, v7.clone(), v8)
    };
    let mut v41: i32 = v0.len() as i32;
    let mut v42: i32 = v10.len() as i32;
    let mut v43: i32 = v1 + v42;
    let mut v44: i32 = 7i32;
    let mut v45: i32 = field_start_4(v0.clone(), v43, v41, v44);
    let mut v46: US0 = US0::US0_ScanPlain;
    let mut v47: i32 = field_end_5(v0.clone(), v45, v41, v46.clone());
    let mut v48: Rc<str> = std::rc::Rc::<str>::from(v0.get(v45 as usize..v47 as usize).unwrap_or(""));
    let mut v49: Rc<str> = unquoted_7(v48.clone());
    let mut v50: i32 = 0i32;
    let mut v51: i32 = v49.len() as i32;
    let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v53: bool = false;
    let (mut v54, mut v55): (Rc<str>, bool) = deps_renamed_9(v49.clone(), v50, v51, v2.clone(), v3.clone(), v52.clone(), v53);
    if v55 {
        let mut v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
        let mut v59: Rc<str> = std::rc::Rc::<str>::from(v54.replace(&*v57, &v58));
        let mut v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
        let mut v61: Rc<str> = std::rc::Rc::<str>::from(v59.replace(&*v56, &v60));
        let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
        let mut v64: Rc<str> = std::rc::Rc::<str>::from(v61.replace(&*v62, &v63));
        let mut v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
        let mut v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
        let mut v67: Rc<str> = std::rc::Rc::<str>::from(v64.replace(&*v65, &v66));
        let mut v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
        let mut v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
        let mut v70: Rc<str> = std::rc::Rc::<str>::from(v67.replace(&*v68, &v69));
        let mut v71: Rc<str> = std::rc::Rc::<str>::from([&*v56, &*v70, &*v56, &*v52, &*v52].concat());
        let mut v72: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v45 as usize).unwrap_or(""));
        let mut v73: i32 = v0.len() as i32;
        let mut v74: Rc<str> = std::rc::Rc::<str>::from(v0.get(v47 as usize..v73 as usize).unwrap_or(""));
        let mut v75: Rc<str> = std::rc::Rc::<str>::from([&*v72, &*v71, &*v74, &*v52, &*v52].concat());
        let mut v76: bool = v37 == v52 ;
        let mut v79: Rc<str> = if v76 {
            v18.clone()
        } else {
            let mut v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let mut v78: Rc<str> = std::rc::Rc::<str>::from([&*v37, &*v77, &*v18, &*v52, &*v52].concat());
            v78.clone()
        };
        let mut v80: i32 = v38.wrapping_add(1i32);
        (v75.clone(), v36, v79.clone(), v80, v39.clone(), v40)
    } else {
        (v0.clone(), v36, v37.clone(), v38, v39.clone(), v40)
    }
}
fn renamed_line_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>, mut v5: i32, mut v6: Rc<str>, mut v7: i32) -> (Rc<str>, i32, Rc<str>, i32, Rc<str>, i32) {
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let mut v9: bool = v0.starts_with(&*v8);
    if v9 {
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v11, &*v1, &*v12, &*v13].concat());
        let mut v15: i32 = v0.find(&*v14).map(|offset| offset as i32).unwrap_or(-1);
        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planning ("); } LIT.with(|lit| lit.clone()) };
        let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v11, &*v1, &*v12, &*v13].concat());
        let mut v18: i32 = v0.find(&*v17).map(|offset| offset as i32).unwrap_or(-1);
        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
        let mut v20: Rc<str> = std::rc::Rc::<str>::from([&*v19, &*v11, &*v1, &*v12, &*v13].concat());
        let mut v21: i32 = v0.find(&*v20).map(|offset| offset as i32).unwrap_or(-1);
        let mut v22: i32 = v3.wrapping_add(1i32);
        let mut v23: bool = v15 < 0i32;
        let mut v24: bool = v23 == false;
        if v24 {
            let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
            let mut v27: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v25, &v26));
            let mut v28: Rc<str> = std::rc::Rc::<str>::from(v27.to_ascii_lowercase());
            let mut v29: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v11, &*v1, &*v12, &*v13].concat());
            let mut v30: i32 = v29.len() as i32;
            let mut v31: i32 = v15 + v30;
            let mut v32: i32 = v0.len() as i32;
            let mut v33: Rc<str> = std::rc::Rc::<str>::from(v0.get(v31 as usize..v32 as usize).unwrap_or(""));
            let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = "); } LIT.with(|lit| lit.clone()) };
            let mut v35: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v11, &*v2, &*v12, &*v13].concat());
            let mut v36: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v28, &*v34, &*v35, &*v33].concat());
            (v36.clone(), v22, v4.clone(), v5, v6.clone(), v7)
        } else {
            let mut v37: bool = v18 < 0i32;
            let mut v38: bool = v37 == false;
            if v38 {
                let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("planning_of_"); } LIT.with(|lit| lit.clone()) };
                let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
                let mut v42: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v40, &v41));
                let mut v43: Rc<str> = std::rc::Rc::<str>::from(v42.to_ascii_lowercase());
                let mut v44: Rc<str> = std::rc::Rc::<str>::from([&*v39, &*v43, &*v13, &*v13, &*v13].concat());
                let mut v45: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v11, &*v1, &*v12, &*v13].concat());
                let mut v46: i32 = v45.len() as i32;
                let mut v47: i32 = v18 + v46;
                let mut v48: i32 = v0.len() as i32;
                let mut v49: Rc<str> = std::rc::Rc::<str>::from(v0.get(v47 as usize..v48 as usize).unwrap_or(""));
                let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = "); } LIT.with(|lit| lit.clone()) };
                let mut v51: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v11, &*v2, &*v12, &*v13].concat());
                let mut v52: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v44, &*v50, &*v51, &*v49].concat());
                (v52.clone(), v22, v4.clone(), v5, v6.clone(), v7)
            } else {
                let mut v53: bool = v21 < 0i32;
                let mut v54: bool = v53 == false;
                if v54 {
                    let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note_of_"); } LIT.with(|lit| lit.clone()) };
                    let mut v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                    let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
                    let mut v58: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v56, &v57));
                    let mut v59: Rc<str> = std::rc::Rc::<str>::from(v58.to_ascii_lowercase());
                    let mut v60: i32 = v19.len() as i32;
                    let mut v61: i32 = v21 + v60;
                    let mut v62: i32 = v0.len() as i32;
                    let mut v63: i32 = 1i32;
                    let mut v64: i32 = field_start_4(v0.clone(), v61, v62, v63);
                    let mut v65: US0 = US0::US0_ScanPlain;
                    let mut v66: i32 = field_end_5(v0.clone(), v64, v62, v65.clone());
                    let mut v67: Rc<str> = std::rc::Rc::<str>::from(v0.get(v64 as usize..v66 as usize).unwrap_or(""));
                    let mut v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
                    let mut v69: Rc<str> = std::rc::Rc::<str>::from(v67.replace(&*v68, &v13));
                    let mut v70: Rc<str> = std::rc::Rc::<str>::from([&*v55, &*v59, &*v57, &*v69, &*v13].concat());
                    let mut v71: Rc<str> = std::rc::Rc::<str>::from([&*v19, &*v11, &*v1, &*v12, &*v13].concat());
                    let mut v72: i32 = v71.len() as i32;
                    let mut v73: i32 = v21 + v72;
                    let mut v74: i32 = v0.len() as i32;
                    let mut v75: Rc<str> = std::rc::Rc::<str>::from(v0.get(v73 as usize..v74 as usize).unwrap_or(""));
                    let mut v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = "); } LIT.with(|lit| lit.clone()) };
                    let mut v77: Rc<str> = std::rc::Rc::<str>::from([&*v19, &*v11, &*v2, &*v12, &*v13].concat());
                    let mut v78: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v70, &*v76, &*v77, &*v75].concat());
                    (v78.clone(), v22, v4.clone(), v5, v6.clone(), v7)
                } else {
                    let mut v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
                    let mut v80: i32 = v0.find(&*v79).map(|offset| offset as i32).unwrap_or(-1);
                    let mut v81: bool = v80 < 0i32;
                    if v81 {
                        (v0.clone(), v3, v4.clone(), v5, v6.clone(), v7)
                    } else {
                        dependent_line_3(v0.clone(), v80, v1.clone(), v2.clone(), v3, v4.clone(), v5, v6.clone(), v7)
                    }
                }
            }
        }
    } else {
        (v0.clone(), v3, v4.clone(), v5, v6.clone(), v7)
    }
}
fn renamed_from_1(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: i32, mut v7: Rc<str>, mut v8: i32, mut v9: Rc<str>, mut v10: i32) -> (Rc<str>, i32, Rc<str>, i32, Rc<str>, i32) {
    loop {
        let mut v11: bool = v2 <= v1;
        if v11 {
            return (v5.clone(), v6, v7.clone(), v8, v9.clone(), v10);
        } else {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v13: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v12)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
            let mut v14: bool = v13 < 0i32;
            let mut v15: i32 = if v14 {
                v2
            } else {
                v13
            };
            let mut v16: Rc<str> = std::rc::Rc::<str>::from(v0.get(v1 as usize..v15 as usize).unwrap_or(""));
            let (mut v17, mut v18, mut v19, mut v20, mut v21, mut v22): (Rc<str>, i32, Rc<str>, i32, Rc<str>, i32) = renamed_line_2(v16.clone(), v3.clone(), v4.clone(), v6, v7.clone(), v8, v9.clone(), v10);
            let mut v24: Rc<str> = if v14 {
                let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v23.clone()
            } else {
                v12.clone()
            };
            let mut v25: i32 = v15.wrapping_add(1i32);
            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v27: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v17, &*v24, &*v26, &*v26].concat());
            (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10) = (v0.clone(), v25, v2, v3.clone(), v4.clone(), v27.clone(), v18, v19.clone(), v20, v21.clone(), v22);
            continue;
        }
    }
}
fn renamed_text_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> (Rc<str>, i32, i32, Rc<str>, i32, Rc<str>) {
    let mut v3: i32 = 0i32;
    let mut v4: i32 = v0.len() as i32;
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: i32 = 0i32;
    let mut v7: i32 = 0i32;
    let mut v8: i32 = 0i32;
    let (mut v9, mut v10, mut v11, mut v12, mut v13, mut v14): (Rc<str>, i32, Rc<str>, i32, Rc<str>, i32) = renamed_from_1(v0.clone(), v3, v4, v1.clone(), v2.clone(), v5.clone(), v6, v5.clone(), v7, v5.clone(), v8);
    (v9.clone(), v10, v12, v11.clone(), v14, v13.clone())
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> (Rc<str>, i32, i32, Rc<str>, i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> (Rc<str>, i32, i32, Rc<str>, i32, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>| -> (Rc<str>, i32, i32, Rc<str>, i32, Rc<str>) {
        renamed_text_0(v0.clone(), v1.clone(), v2.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_renamed_text(v0: &str, v1: &str, v2: &str) -> (Rc<str>, i32, i32, Rc<str>, i32, Rc<str>) {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2))
}
