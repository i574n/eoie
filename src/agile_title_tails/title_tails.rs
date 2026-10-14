#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn digits_in_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> bool {
    loop {
        let mut v3: bool = v2 <= v1;
        if v3 {
            return true;
        } else {
            let mut v4: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v5: bool = 48i32 <= v4;
            let mut v7: bool = if v5 {
                let mut v6: bool = v4 <= 57i32;
                v6
            } else {
                false
            };
            if v7 {
                let mut v8: i32 = v1 + 1;
                (v0, v1, v2) = (v0.clone(), v8, v2);
                continue;
            } else {
                return false;
            }
        }
    }
}
fn dated_clock_3(mut v0: Rc<str>) -> bool {
    let mut v1: i32 = v0.len() as i32;
    let mut v2: bool = 16i32 <= v1;
    if v2 {
        let mut v3: i32 = 0i32;
        let mut v4: i32 = 4i32;
        let mut v5: bool = digits_in_4(v0.clone(), v3, v4);
        if v5 {
            let mut v6: i32 = v0.as_bytes().get(4i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v7: bool = v6 == 45i32;
            if v7 {
                let mut v8: i32 = 5i32;
                let mut v9: i32 = 7i32;
                let mut v10: bool = digits_in_4(v0.clone(), v8, v9);
                if v10 {
                    let mut v11: i32 = v0.as_bytes().get(7i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                    let mut v12: bool = v11 == 45i32;
                    if v12 {
                        let mut v13: i32 = 8i32;
                        let mut v14: i32 = 10i32;
                        let mut v15: bool = digits_in_4(v0.clone(), v13, v14);
                        if v15 {
                            let mut v16: i32 = v0.as_bytes().get(10i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                            let mut v17: bool = v16 == 32i32;
                            if v17 {
                                let mut v18: i32 = 11i32;
                                let mut v19: i32 = 13i32;
                                let mut v20: bool = digits_in_4(v0.clone(), v18, v19);
                                if v20 {
                                    let mut v21: i32 = v0.as_bytes().get(13i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                                    let mut v22: bool = v21 == 58i32;
                                    if v22 {
                                        let mut v23: i32 = 14i32;
                                        let mut v24: i32 = 16i32;
                                        let mut v25: bool = digits_in_4(v0.clone(), v23, v24);
                                        if v25 {
                                            let mut v26: i32 = v0.len() as i32;
                                            let mut v27: bool = v26 == 16i32;
                                            if v27 {
                                                true
                                            } else {
                                                let mut v28: i32 = v0.as_bytes().get(16i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                                                let mut v29: bool = v28 == 32i32;
                                                v29
                                            }
                                        } else {
                                            false
                                        }
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}
fn bare_clock_5(mut v0: Rc<str>) -> bool {
    let mut v1: i32 = v0.len() as i32;
    let mut v2: bool = 5i32 <= v1;
    if v2 {
        let mut v3: i32 = 0i32;
        let mut v4: i32 = 2i32;
        let mut v5: bool = digits_in_4(v0.clone(), v3, v4);
        if v5 {
            let mut v6: i32 = v0.as_bytes().get(2i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v7: bool = v6 == 58i32;
            if v7 {
                let mut v8: i32 = 3i32;
                let mut v9: i32 = 5i32;
                let mut v10: bool = digits_in_4(v0.clone(), v8, v9);
                if v10 {
                    let mut v11: i32 = v0.len() as i32;
                    let mut v12: bool = v11 == 5i32;
                    if v12 {
                        true
                    } else {
                        let mut v13: i32 = v0.as_bytes().get(5i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                        let mut v14: bool = v13 == 32i32;
                        v14
                    }
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}
fn stamped_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>) {
    let mut v3: bool = dated_clock_3(v0.clone());
    let (mut v25, mut v26, mut v27): (Rc<str>, Rc<str>, Rc<str>) = if v3 {
        let mut v4: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..16i32 as usize).unwrap_or(""));
        let mut v5: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..10i32 as usize).unwrap_or(""));
        let mut v6: i32 = v0.len() as i32;
        let mut v7: Rc<str> = std::rc::Rc::<str>::from(v0.get(16i32 as usize..v6 as usize).unwrap_or(""));
        let mut v8: Rc<str> = std::rc::Rc::<str>::from(v7.trim());
        (v4.clone(), v5.clone(), v8.clone())
    } else {
        let mut v9: bool = bare_clock_5(v0.clone());
        let (mut v22, mut v23, mut v24): (Rc<str>, Rc<str>, Rc<str>) = if v9 {
            let mut v10: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..5i32 as usize).unwrap_or(""));
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v12: bool = v1 == v11 ;
            let mut v15: Rc<str> = if v12 {
                v10.clone()
            } else {
                let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let mut v14: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v13, &*v10, &*v11, &*v11].concat());
                v14.clone()
            };
            let mut v16: i32 = v0.len() as i32;
            let mut v17: Rc<str> = std::rc::Rc::<str>::from(v0.get(5i32 as usize..v16 as usize).unwrap_or(""));
            let mut v18: Rc<str> = std::rc::Rc::<str>::from(v17.trim());
            (v15.clone(), v1.clone(), v18.clone())
        } else {
            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v20: bool = v1 == v19 ;
            let mut v21: Rc<str> = if v20 {
                v2.clone()
            } else {
                v1.clone()
            };
            (v21.clone(), v1.clone(), v0.clone())
        };
        (v22.clone(), v23.clone(), v24.clone())
    };
    (v25.clone(), v26.clone(), v27.clone())
}
fn note_row_6(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note_of_"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = std::rc::Rc::<str>::from(v0.replace(&*v7, &v8));
    let mut v10: Rc<str> = std::rc::Rc::<str>::from(v9.to_ascii_lowercase());
    let mut v11: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v10, &*v8, &*v11, &*v12].concat());
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = Note ("); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = std::rc::Rc::<str>::from(v0.replace(&*v16, &v17));
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = std::rc::Rc::<str>::from(v18.replace(&*v15, &v19));
    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = std::rc::Rc::<str>::from(v20.replace(&*v21, &v22));
    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let mut v26: Rc<str> = std::rc::Rc::<str>::from(v23.replace(&*v24, &v25));
    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let mut v29: Rc<str> = std::rc::Rc::<str>::from(v26.replace(&*v27, &v28));
    let mut v30: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v29, &*v15, &*v12, &*v12].concat());
    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
    let mut v32: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v13, &*v14, &*v30, &*v31].concat());
    let mut v33: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32, "); } LIT.with(|lit| lit.clone()) };
    let mut v35: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v16, &v17));
    let mut v36: Rc<str> = std::rc::Rc::<str>::from(v35.replace(&*v15, &v19));
    let mut v37: Rc<str> = std::rc::Rc::<str>::from(v36.replace(&*v21, &v22));
    let mut v38: Rc<str> = std::rc::Rc::<str>::from(v37.replace(&*v24, &v25));
    let mut v39: Rc<str> = std::rc::Rc::<str>::from(v38.replace(&*v27, &v28));
    let mut v40: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v39, &*v15, &*v12, &*v12].concat());
    let mut v41: Rc<str> = std::rc::Rc::<str>::from(v3.replace(&*v16, &v17));
    let mut v42: Rc<str> = std::rc::Rc::<str>::from(v41.replace(&*v15, &v19));
    let mut v43: Rc<str> = std::rc::Rc::<str>::from(v42.replace(&*v21, &v22));
    let mut v44: Rc<str> = std::rc::Rc::<str>::from(v43.replace(&*v24, &v25));
    let mut v45: Rc<str> = std::rc::Rc::<str>::from(v44.replace(&*v27, &v28));
    let mut v46: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v45, &*v15, &*v12, &*v12].concat());
    let mut v47: Rc<str> = std::rc::Rc::<str>::from([&*v33, &*v34, &*v40, &*v31, &*v46].concat());
    let mut v48: Rc<str> = std::rc::Rc::<str>::from(v4.replace(&*v16, &v17));
    let mut v49: Rc<str> = std::rc::Rc::<str>::from(v48.replace(&*v15, &v19));
    let mut v50: Rc<str> = std::rc::Rc::<str>::from(v49.replace(&*v21, &v22));
    let mut v51: Rc<str> = std::rc::Rc::<str>::from(v50.replace(&*v24, &v25));
    let mut v52: Rc<str> = std::rc::Rc::<str>::from(v51.replace(&*v27, &v28));
    let mut v53: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v52, &*v15, &*v12, &*v12].concat());
    let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let mut v55: Rc<str> = std::rc::Rc::<str>::from([&*v32, &*v47, &*v31, &*v53, &*v54].concat());
    v55.clone()
}
fn stamped_8(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>) {
    let mut v4: bool = dated_clock_3(v0.clone());
    let (mut v26, mut v27, mut v28): (Rc<str>, Rc<str>, Rc<str>) = if v4 {
        let mut v5: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..16i32 as usize).unwrap_or(""));
        let mut v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..10i32 as usize).unwrap_or(""));
        let mut v7: i32 = v0.len() as i32;
        let mut v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(16i32 as usize..v7 as usize).unwrap_or(""));
        let mut v9: Rc<str> = std::rc::Rc::<str>::from(v8.trim());
        (v5.clone(), v6.clone(), v9.clone())
    } else {
        let mut v10: bool = bare_clock_5(v0.clone());
        let (mut v23, mut v24, mut v25): (Rc<str>, Rc<str>, Rc<str>) = if v10 {
            let mut v11: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..5i32 as usize).unwrap_or(""));
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v13: bool = v1 == v12 ;
            let mut v16: Rc<str> = if v13 {
                v11.clone()
            } else {
                let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let mut v15: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v14, &*v11, &*v12, &*v12].concat());
                v15.clone()
            };
            let mut v17: i32 = v0.len() as i32;
            let mut v18: Rc<str> = std::rc::Rc::<str>::from(v0.get(5i32 as usize..v17 as usize).unwrap_or(""));
            let mut v19: Rc<str> = std::rc::Rc::<str>::from(v18.trim());
            (v16.clone(), v1.clone(), v19.clone())
        } else {
            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v21: bool = v2 == v20 ;
            let mut v22: Rc<str> = if v21 {
                v3.clone()
            } else {
                v2.clone()
            };
            (v22.clone(), v1.clone(), v0.clone())
        };
        (v23.clone(), v24.clone(), v25.clone())
    };
    (v26.clone(), v27.clone(), v28.clone())
}
fn moved_from_7(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>, mut v4: i32, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: i32) -> (Rc<str>, i32) {
    let mut v10: bool = v2 <= v1;
    let (mut v41, mut v42): (Rc<str>, i32) = if v10 {
        (v8.clone(), v9)
    } else {
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
        let mut v12: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v11)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let mut v13: bool = v12 < 0;
        let mut v14: i32 = if v13 {
            v2
        } else {
            v12
        };
        let mut v15: bool = v12 < 0;
        let mut v17: i32 = if v15 {
            v2
        } else {
            let mut v16: i32 = v12 + 2;
            v16
        };
        let mut v18: Rc<str> = std::rc::Rc::<str>::from(v0.get(v1 as usize..v14 as usize).unwrap_or(""));
        let mut v19: Rc<str> = std::rc::Rc::<str>::from(v18.trim());
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v21: bool = v19 == v20 ;
        let (mut v39, mut v40): (Rc<str>, i32) = if v21 {
            let (mut v22, mut v23): (Rc<str>, i32) = moved_from_7(v0.clone(), v17, v2, v3.clone(), v4, v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9);
            (v22.clone(), v23)
        } else {
            let (mut v24, mut v25, mut v26): (Rc<str>, Rc<str>, Rc<str>) = stamped_8(v19.clone(), v5.clone(), v6.clone(), v7.clone());
            let mut v27: bool = v26 == v20 ;
            let mut v28: Rc<str> = if v27 {
                v19.clone()
            } else {
                v26.clone()
            };
            let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("migrated"); } LIT.with(|lit| lit.clone()) };
            let mut v30: Rc<str> = note_row_6(v3.clone(), v4, v24.clone(), v29.clone(), v28.clone());
            let mut v31: bool = v8 == v20 ;
            let mut v33: Rc<str> = if v31 {
                v20.clone()
            } else {
                let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                v32.clone()
            };
            let mut v34: i32 = v4 + 1;
            let mut v35: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v33, &*v30, &*v20, &*v20].concat());
            let mut v36: i32 = v9 + 1;
            let (mut v37, mut v38): (Rc<str>, i32) = moved_from_7(v0.clone(), v17, v2, v3.clone(), v34, v25.clone(), v24.clone(), v7.clone(), v35.clone(), v36);
            (v37.clone(), v38)
        };
        (v39.clone(), v40)
    };
    (v41.clone(), v42)
}
fn moved_from_1(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>, mut v4: i32, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: i32) -> (Rc<str>, i32) {
    let mut v8: bool = v2 <= v1;
    let (mut v39, mut v40): (Rc<str>, i32) = if v8 {
        (v5.clone(), v7)
    } else {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
        let mut v10: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v9)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let mut v11: bool = v10 < 0;
        let mut v12: i32 = if v11 {
            v2
        } else {
            v10
        };
        let mut v13: bool = v10 < 0;
        let mut v15: i32 = if v13 {
            v2
        } else {
            let mut v14: i32 = v10 + 2;
            v14
        };
        let mut v16: Rc<str> = std::rc::Rc::<str>::from(v0.get(v1 as usize..v12 as usize).unwrap_or(""));
        let mut v17: Rc<str> = std::rc::Rc::<str>::from(v16.trim());
        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v19: bool = v17 == v18 ;
        let (mut v37, mut v38): (Rc<str>, i32) = if v19 {
            let (mut v20, mut v21): (Rc<str>, i32) = moved_from_1(v0.clone(), v15, v2, v3.clone(), v4, v5.clone(), v6.clone(), v7);
            (v20.clone(), v21)
        } else {
            let (mut v22, mut v23, mut v24): (Rc<str>, Rc<str>, Rc<str>) = stamped_2(v17.clone(), v5.clone(), v6.clone());
            let mut v25: bool = v24 == v18 ;
            let mut v26: Rc<str> = if v25 {
                v17.clone()
            } else {
                v24.clone()
            };
            let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("migrated"); } LIT.with(|lit| lit.clone()) };
            let mut v28: Rc<str> = note_row_6(v3.clone(), v4, v22.clone(), v27.clone(), v26.clone());
            let mut v29: bool = v5 == v18 ;
            let mut v31: Rc<str> = if v29 {
                v18.clone()
            } else {
                let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                v30.clone()
            };
            let mut v32: i32 = v4 + 1;
            let mut v33: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v31, &*v28, &*v18, &*v18].concat());
            let mut v34: i32 = v7 + 1;
            let (mut v35, mut v36): (Rc<str>, i32) = moved_from_7(v0.clone(), v15, v2, v3.clone(), v32, v23.clone(), v22.clone(), v6.clone(), v33.clone(), v34);
            (v35.clone(), v36)
        };
        (v37.clone(), v38)
    };
    (v39.clone(), v40)
}
fn tail_rows_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>) -> (i32, Rc<str>) {
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
    let mut v5: i32 = v0.find(&*v4).map(|offset| offset as i32).unwrap_or(-1);
    let mut v6: bool = v5 < 0;
    if v6 {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (0i32, v7.clone())
    } else {
        let mut v8: i32 = v5 + 2;
        let mut v9: i32 = v0.len() as i32;
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v11: i32 = 0i32;
        let (mut v12, mut v13): (Rc<str>, i32) = moved_from_1(v0.clone(), v8, v9, v1.clone(), v2, v10.clone(), v3.clone(), v11);
        (v13, v12.clone())
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, i32, Rc<str>) -> (i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, i32, Rc<str>) -> (i32, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>| -> (i32, Rc<str>) {
        tail_rows_0(v0.clone(), v1.clone(), v2, v3.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_title_tail_rows(v0: &str, v1: &str, v2: i32, v3: &str) -> (i32, Rc<str>) {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), v2, Rc::<str>::from(v3))
}
