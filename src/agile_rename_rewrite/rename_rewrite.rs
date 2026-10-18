#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_ScanPlain,
    US0_ScanQuoted,
    US0_ScanEscaped,
}
fn field_end_6(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US0) -> i32 {
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
fn skip_spaces_7(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn field_start_5(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let mut v5: US0 = US0::US0_ScanPlain;
            let mut v6: i32 = field_end_6(v0.clone(), v1, v2, v5.clone());
            let mut v7: i32 = v6.wrapping_add(1i32);
            let mut v8: i32 = skip_spaces_7(v0.clone(), v7);
            let mut v9: i32 = v3.wrapping_sub(1i32);
            (v0, v1, v2, v3) = (v0.clone(), v8, v2, v9);
            continue;
        }
    }
}
fn unescaped_from_10(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let mut v4: bool = v2 <= v1;
        if v4 {
            let mut v5: Rc<str> = std::rc::Rc::<str>::from(v0.get(v1 as usize..v2 as usize).unwrap_or(""));
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v7: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v5, &*v6, &*v6, &*v6].concat());
            return v7.clone();
        } else {
            let mut v8: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = v8 == 92i32;
            if v9 {
                let mut v10: i32 = v1.wrapping_add(1i32);
                let mut v11: i32 = v10.wrapping_add(1i32);
                let mut v12: Rc<str> = std::rc::Rc::<str>::from(v0.get(v1 as usize..v1 as usize).unwrap_or(""));
                let mut v13: i32 = v0.as_bytes().get(v10 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let mut v14: bool = v13 == 110i32;
                let mut v23: Rc<str> = if v14 {
                    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    v15.clone()
                } else {
                    let mut v16: bool = v13 == 114i32;
                    if v16 {
                        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                        v17.clone()
                    } else {
                        let mut v18: bool = v13 == 116i32;
                        if v18 {
                            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            v19.clone()
                        } else {
                            let mut v20: Rc<str> = std::rc::Rc::<str>::from(v0.get(v10 as usize..v11 as usize).unwrap_or(""));
                            v20.clone()
                        }
                    }
                };
                let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v25: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v12, &*v23, &*v24, &*v24].concat());
                (v0, v1, v2, v3) = (v0.clone(), v11, v2, v25.clone());
                continue;
            } else {
                let mut v27: i32 = v1.wrapping_add(1i32);
                return unescaped_from_9(v0.clone(), v27, v1, v2, v3.clone());
            }
        }
    }
}
fn unescaped_from_9(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
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
                return unescaped_from_10(v0.clone(), v12, v3, v26.clone());
            } else {
                let mut v28: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2, v3, v4) = (v0.clone(), v28, v2, v3, v4.clone());
                continue;
            }
        }
    }
}
fn unquoted_8(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = v0.len() as i32;
    let mut v2: i32 = v1.wrapping_sub(1i32);
    let mut v3: Rc<str> = std::rc::Rc::<str>::from(v0.get(1i32 as usize..v2 as usize).unwrap_or(""));
    let mut v4: i32 = 0i32;
    let mut v5: i32 = 0i32;
    let mut v6: i32 = v3.len() as i32;
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    unescaped_from_9(v3.clone(), v4, v5, v6, v7.clone())
}
fn deps_renamed_11(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: bool) -> (Rc<str>, bool) {
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
fn dependent_line_4(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: Rc<str>, mut v6: i32, mut v7: Rc<str>, mut v8: i32) -> (Rc<str>, i32, Rc<str>, i32, Rc<str>, i32) {
    let mut v9: i32 = v0.len() as i32;
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
    let mut v11: i32 = v10.len() as i32;
    let mut v12: i32 = v1 + v11;
    let mut v13: i32 = 0i32;
    let mut v14: i32 = field_start_5(v0.clone(), v12, v9, v13);
    let mut v15: US0 = US0::US0_ScanPlain;
    let mut v16: i32 = field_end_6(v0.clone(), v14, v9, v15.clone());
    let mut v17: Rc<str> = std::rc::Rc::<str>::from(v0.get(v14 as usize..v16 as usize).unwrap_or(""));
    let mut v18: Rc<str> = unquoted_8(v17.clone());
    let mut v19: i32 = v0.len() as i32;
    let mut v20: i32 = v10.len() as i32;
    let mut v21: i32 = v1 + v20;
    let mut v22: i32 = 1i32;
    let mut v23: i32 = field_start_5(v0.clone(), v21, v19, v22);
    let mut v24: US0 = US0::US0_ScanPlain;
    let mut v25: i32 = field_end_6(v0.clone(), v23, v19, v24.clone());
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
    let mut v45: i32 = field_start_5(v0.clone(), v43, v41, v44);
    let mut v46: US0 = US0::US0_ScanPlain;
    let mut v47: i32 = field_end_6(v0.clone(), v45, v41, v46.clone());
    let mut v48: Rc<str> = std::rc::Rc::<str>::from(v0.get(v45 as usize..v47 as usize).unwrap_or(""));
    let mut v49: Rc<str> = unquoted_8(v48.clone());
    let mut v50: i32 = 0i32;
    let mut v51: i32 = v49.len() as i32;
    let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v53: bool = false;
    let (mut v54, mut v55): (Rc<str>, bool) = deps_renamed_11(v49.clone(), v50, v51, v2.clone(), v3.clone(), v52.clone(), v53);
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
fn renamed_line_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>, mut v5: i32, mut v6: Rc<str>, mut v7: i32) -> (Rc<str>, i32, Rc<str>, i32, Rc<str>, i32) {
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
                    let mut v64: i32 = field_start_5(v0.clone(), v61, v62, v63);
                    let mut v65: US0 = US0::US0_ScanPlain;
                    let mut v66: i32 = field_end_6(v0.clone(), v64, v62, v65.clone());
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
                        dependent_line_4(v0.clone(), v80, v1.clone(), v2.clone(), v3, v4.clone(), v5, v6.clone(), v7)
                    }
                }
            }
        }
    } else {
        (v0.clone(), v3, v4.clone(), v5, v6.clone(), v7)
    }
}
fn renamed_from_2(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: i32, mut v7: Rc<str>, mut v8: i32, mut v9: Rc<str>, mut v10: i32) -> (Rc<str>, i32, Rc<str>, i32, Rc<str>, i32) {
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
            let (mut v17, mut v18, mut v19, mut v20, mut v21, mut v22): (Rc<str>, i32, Rc<str>, i32, Rc<str>, i32) = renamed_line_3(v16.clone(), v3.clone(), v4.clone(), v6, v7.clone(), v8, v9.clone(), v10);
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
fn dependent_line_13(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: Rc<str>, mut v6: i32, mut v7: i32) -> (Rc<str>, i32, Rc<str>, i32, Rc<str>, i32) {
    let mut v8: i32 = v0.len() as i32;
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
    let mut v10: i32 = v9.len() as i32;
    let mut v11: i32 = v1 + v10;
    let mut v12: i32 = 0i32;
    let mut v13: i32 = field_start_5(v0.clone(), v11, v8, v12);
    let mut v14: US0 = US0::US0_ScanPlain;
    let mut v15: i32 = field_end_6(v0.clone(), v13, v8, v14.clone());
    let mut v16: Rc<str> = std::rc::Rc::<str>::from(v0.get(v13 as usize..v15 as usize).unwrap_or(""));
    let mut v17: Rc<str> = unquoted_8(v16.clone());
    let mut v18: i32 = v0.len() as i32;
    let mut v19: i32 = v9.len() as i32;
    let mut v20: i32 = v1 + v19;
    let mut v21: i32 = 1i32;
    let mut v22: i32 = field_start_5(v0.clone(), v20, v18, v21);
    let mut v23: US0 = US0::US0_ScanPlain;
    let mut v24: i32 = field_end_6(v0.clone(), v22, v18, v23.clone());
    let mut v25: Rc<str> = std::rc::Rc::<str>::from(v0.get(v22 as usize..v24 as usize).unwrap_or(""));
    let mut v26: i32 = v25.find(&*v2).map(|offset| offset as i32).unwrap_or(-1);
    let mut v27: bool = v26 < 0i32;
    let mut v28: bool = v27 == false;
    let (mut v35, mut v36, mut v37, mut v38, mut v39): (i32, Rc<str>, i32, Rc<str>, i32) = if v28 {
        let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v30: bool = v5 == v29 ;
        let mut v33: Rc<str> = if v30 {
            v17.clone()
        } else {
            let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let mut v32: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v31, &*v17, &*v29, &*v29].concat());
            v32.clone()
        };
        let mut v34: i32 = v7.wrapping_add(1i32);
        (v4, v5.clone(), v6, v33.clone(), v34)
    } else {
        (v4, v5.clone(), v6, v5.clone(), v7)
    };
    let mut v40: i32 = v0.len() as i32;
    let mut v41: i32 = v9.len() as i32;
    let mut v42: i32 = v1 + v41;
    let mut v43: i32 = 7i32;
    let mut v44: i32 = field_start_5(v0.clone(), v42, v40, v43);
    let mut v45: US0 = US0::US0_ScanPlain;
    let mut v46: i32 = field_end_6(v0.clone(), v44, v40, v45.clone());
    let mut v47: Rc<str> = std::rc::Rc::<str>::from(v0.get(v44 as usize..v46 as usize).unwrap_or(""));
    let mut v48: Rc<str> = unquoted_8(v47.clone());
    let mut v49: i32 = 0i32;
    let mut v50: i32 = v48.len() as i32;
    let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v52: bool = false;
    let (mut v53, mut v54): (Rc<str>, bool) = deps_renamed_11(v48.clone(), v49, v50, v2.clone(), v3.clone(), v51.clone(), v52);
    if v54 {
        let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let mut v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
        let mut v58: Rc<str> = std::rc::Rc::<str>::from(v53.replace(&*v56, &v57));
        let mut v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
        let mut v60: Rc<str> = std::rc::Rc::<str>::from(v58.replace(&*v55, &v59));
        let mut v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
        let mut v63: Rc<str> = std::rc::Rc::<str>::from(v60.replace(&*v61, &v62));
        let mut v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
        let mut v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
        let mut v66: Rc<str> = std::rc::Rc::<str>::from(v63.replace(&*v64, &v65));
        let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
        let mut v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
        let mut v69: Rc<str> = std::rc::Rc::<str>::from(v66.replace(&*v67, &v68));
        let mut v70: Rc<str> = std::rc::Rc::<str>::from([&*v55, &*v69, &*v55, &*v51, &*v51].concat());
        let mut v71: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v44 as usize).unwrap_or(""));
        let mut v72: i32 = v0.len() as i32;
        let mut v73: Rc<str> = std::rc::Rc::<str>::from(v0.get(v46 as usize..v72 as usize).unwrap_or(""));
        let mut v74: Rc<str> = std::rc::Rc::<str>::from([&*v71, &*v70, &*v73, &*v51, &*v51].concat());
        let mut v75: bool = v36 == v51 ;
        let mut v78: Rc<str> = if v75 {
            v17.clone()
        } else {
            let mut v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let mut v77: Rc<str> = std::rc::Rc::<str>::from([&*v36, &*v76, &*v17, &*v51, &*v51].concat());
            v77.clone()
        };
        let mut v79: i32 = v37.wrapping_add(1i32);
        (v74.clone(), v35, v78.clone(), v79, v38.clone(), v39)
    } else {
        (v0.clone(), v35, v36.clone(), v37, v38.clone(), v39)
    }
}
fn renamed_line_12(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>, mut v5: i32, mut v6: i32) -> (Rc<str>, i32, Rc<str>, i32, Rc<str>, i32) {
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let mut v8: bool = v0.starts_with(&*v7);
    if v8 {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v10, &*v1, &*v11, &*v12].concat());
        let mut v14: i32 = v0.find(&*v13).map(|offset| offset as i32).unwrap_or(-1);
        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planning ("); } LIT.with(|lit| lit.clone()) };
        let mut v16: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v10, &*v1, &*v11, &*v12].concat());
        let mut v17: i32 = v0.find(&*v16).map(|offset| offset as i32).unwrap_or(-1);
        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
        let mut v19: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v10, &*v1, &*v11, &*v12].concat());
        let mut v20: i32 = v0.find(&*v19).map(|offset| offset as i32).unwrap_or(-1);
        let mut v21: i32 = v3.wrapping_add(1i32);
        let mut v22: bool = v14 < 0i32;
        let mut v23: bool = v22 == false;
        if v23 {
            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
            let mut v26: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v24, &v25));
            let mut v27: Rc<str> = std::rc::Rc::<str>::from(v26.to_ascii_lowercase());
            let mut v28: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v10, &*v1, &*v11, &*v12].concat());
            let mut v29: i32 = v28.len() as i32;
            let mut v30: i32 = v14 + v29;
            let mut v31: i32 = v0.len() as i32;
            let mut v32: Rc<str> = std::rc::Rc::<str>::from(v0.get(v30 as usize..v31 as usize).unwrap_or(""));
            let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = "); } LIT.with(|lit| lit.clone()) };
            let mut v34: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v10, &*v2, &*v11, &*v12].concat());
            let mut v35: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v27, &*v33, &*v34, &*v32].concat());
            (v35.clone(), v21, v4.clone(), v5, v4.clone(), v6)
        } else {
            let mut v36: bool = v17 < 0i32;
            let mut v37: bool = v36 == false;
            if v37 {
                let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("planning_of_"); } LIT.with(|lit| lit.clone()) };
                let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
                let mut v41: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v39, &v40));
                let mut v42: Rc<str> = std::rc::Rc::<str>::from(v41.to_ascii_lowercase());
                let mut v43: Rc<str> = std::rc::Rc::<str>::from([&*v38, &*v42, &*v12, &*v12, &*v12].concat());
                let mut v44: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v10, &*v1, &*v11, &*v12].concat());
                let mut v45: i32 = v44.len() as i32;
                let mut v46: i32 = v17 + v45;
                let mut v47: i32 = v0.len() as i32;
                let mut v48: Rc<str> = std::rc::Rc::<str>::from(v0.get(v46 as usize..v47 as usize).unwrap_or(""));
                let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = "); } LIT.with(|lit| lit.clone()) };
                let mut v50: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v10, &*v2, &*v11, &*v12].concat());
                let mut v51: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v43, &*v49, &*v50, &*v48].concat());
                (v51.clone(), v21, v4.clone(), v5, v4.clone(), v6)
            } else {
                let mut v52: bool = v20 < 0i32;
                let mut v53: bool = v52 == false;
                if v53 {
                    let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note_of_"); } LIT.with(|lit| lit.clone()) };
                    let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                    let mut v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
                    let mut v57: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v55, &v56));
                    let mut v58: Rc<str> = std::rc::Rc::<str>::from(v57.to_ascii_lowercase());
                    let mut v59: i32 = v18.len() as i32;
                    let mut v60: i32 = v20 + v59;
                    let mut v61: i32 = v0.len() as i32;
                    let mut v62: i32 = 1i32;
                    let mut v63: i32 = field_start_5(v0.clone(), v60, v61, v62);
                    let mut v64: US0 = US0::US0_ScanPlain;
                    let mut v65: i32 = field_end_6(v0.clone(), v63, v61, v64.clone());
                    let mut v66: Rc<str> = std::rc::Rc::<str>::from(v0.get(v63 as usize..v65 as usize).unwrap_or(""));
                    let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
                    let mut v68: Rc<str> = std::rc::Rc::<str>::from(v66.replace(&*v67, &v12));
                    let mut v69: Rc<str> = std::rc::Rc::<str>::from([&*v54, &*v58, &*v56, &*v68, &*v12].concat());
                    let mut v70: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v10, &*v1, &*v11, &*v12].concat());
                    let mut v71: i32 = v70.len() as i32;
                    let mut v72: i32 = v20 + v71;
                    let mut v73: i32 = v0.len() as i32;
                    let mut v74: Rc<str> = std::rc::Rc::<str>::from(v0.get(v72 as usize..v73 as usize).unwrap_or(""));
                    let mut v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = "); } LIT.with(|lit| lit.clone()) };
                    let mut v76: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v10, &*v2, &*v11, &*v12].concat());
                    let mut v77: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v69, &*v75, &*v76, &*v74].concat());
                    (v77.clone(), v21, v4.clone(), v5, v4.clone(), v6)
                } else {
                    let mut v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
                    let mut v79: i32 = v0.find(&*v78).map(|offset| offset as i32).unwrap_or(-1);
                    let mut v80: bool = v79 < 0i32;
                    if v80 {
                        (v0.clone(), v3, v4.clone(), v5, v4.clone(), v6)
                    } else {
                        dependent_line_13(v0.clone(), v79, v1.clone(), v2.clone(), v3, v4.clone(), v5, v6)
                    }
                }
            }
        }
    } else {
        (v0.clone(), v3, v4.clone(), v5, v4.clone(), v6)
    }
}
fn renamed_from_1(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: i32, mut v7: i32, mut v8: i32) -> (Rc<str>, i32, Rc<str>, i32, Rc<str>, i32) {
    let mut v9: bool = v2 <= v1;
    if v9 {
        (v5.clone(), v6, v5.clone(), v7, v5.clone(), v8)
    } else {
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v11: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v10)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let mut v12: bool = v11 < 0i32;
        let mut v13: i32 = if v12 {
            v2
        } else {
            v11
        };
        let mut v14: Rc<str> = std::rc::Rc::<str>::from(v0.get(v1 as usize..v13 as usize).unwrap_or(""));
        let (mut v15, mut v16, mut v17, mut v18, mut v19, mut v20): (Rc<str>, i32, Rc<str>, i32, Rc<str>, i32) = renamed_line_12(v14.clone(), v3.clone(), v4.clone(), v6, v5.clone(), v7, v8);
        let mut v22: Rc<str> = if v12 {
            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v21.clone()
        } else {
            v10.clone()
        };
        let mut v23: i32 = v13.wrapping_add(1i32);
        let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v25: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v15, &*v22, &*v24, &*v24].concat());
        renamed_from_2(v0.clone(), v23, v2, v3.clone(), v4.clone(), v25.clone(), v16, v17.clone(), v18, v19.clone(), v20)
    }
}
fn renamed_text_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> (Rc<str>, i32, i32, Rc<str>, i32, Rc<str>) {
    let mut v3: i32 = 0i32;
    let mut v4: i32 = v0.len() as i32;
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: i32 = 0i32;
    let mut v7: i32 = 0i32;
    let mut v8: i32 = 0i32;
    let (mut v9, mut v10, mut v11, mut v12, mut v13, mut v14): (Rc<str>, i32, Rc<str>, i32, Rc<str>, i32) = renamed_from_1(v0.clone(), v3, v4, v1.clone(), v2.clone(), v5.clone(), v6, v7, v8);
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
