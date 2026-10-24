#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn string_slice(value: &str, from: i64, to: i64) -> Rc<str> {
    let bytes = value.as_bytes();
    let length = bytes.len() as i64;
    if from < 0 || from > length || to < from - 1 || to >= length { std::process::abort(); }
    if to < from { return { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }; }
    if (bytes[from as usize] & 0xC0) == 0x80 || (to + 1 < length && (bytes[(to + 1) as usize] & 0xC0) == 0x80) { std::process::exit(3); }
    let slice = &bytes[from as usize..(to + 1) as usize];
    match std::str::from_utf8(slice) { Ok(text) => Rc::<str>::from(text), Err(error) => Rc::<str>::from(std::str::from_utf8(&slice[..error.valid_up_to()]).unwrap_or("")) }
}
#[derive(Clone)]
enum UH0 {
    UH0_NoPairs,
    UH0_PairThen(Rc<str>, i32, i32, Rc<UH0>),
}
#[derive(Clone)]
enum UH1 {
    UH1_NoDifferences,
    UH1_DifferenceThen(Rc<str>, i32, Rc<UH1>),
}
#[derive(Clone)]
enum US0 {
    US0_TableLoaded(std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>),
    US0_TableUnreadable(Rc<str>),
}
fn method5(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let mut v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let mut v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                let mut v12: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1, v12);
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method6(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let mut v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let mut v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                (v0, v1) = (v0.clone(), v3);
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn loop_4(mut v0: Rc<str>, mut v1: Rc<RefCell<Vec<Rc<str>>>>, mut v2: i32, mut v3: i32) -> bool {
    loop {
        let mut v4: bool = v3 >= v2;
        if v4 {
            return false;
        } else {
            let mut v5: Rc<str> = v1.clone().borrow()[v3 as usize].clone();
            let mut v6: i32 = (v5.clone().len() as i32);
            let mut v7: i32 = 0i32;
            let mut v8: i32 = method5(v5.clone(), v6, v7);
            let mut v9: i32 = v6.wrapping_sub(1i32);
            let mut v10: Rc<str> = string_slice(&v5.clone(), v8 as i64, v9 as i64);
            let mut v11: i32 = (v10.clone().len() as i32);
            let mut v12: i32 = method6(v10.clone(), v11);
            let mut v13: Rc<str> = string_slice(&v10.clone(), 0i32 as i64, v12 as i64);
            let mut v14: bool = v13.clone() == v0.clone();
            if v14 {
                return true;
            } else {
                let mut v15: i32 = v3.wrapping_add(1i32);
                (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v2, v15);
                continue;
            }
        }
    }
}
fn shared_columns_3(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v4.clone()
    } else {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = std::rc::Rc::<str>::from((v1).split(&*v5).next().unwrap_or(""));
        let mut v7: Rc<str> = std::rc::Rc::<str>::from((v1).split_once(&*v5).map(|(_, rest)| rest).unwrap_or(""));
        let mut v8: Rc<str> = shared_columns_3(v0.clone(), v7.clone(), v2.clone());
        let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v2.split(&*v5).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v10: i32 = (v9.clone().borrow().len() as i32);
        let mut v11: i32 = 0i32;
        let mut v12: bool = loop_4(v6.clone(), v9.clone(), v10, v11);
        let mut v15: bool = if v12 {
            true
        } else {
            let mut v13: i32 = (v0).first().and_then(|header| header.iter().position(|item| &**item == &*v6)).map(|index| index as i32).unwrap_or(-1);
            let mut v14: bool = v13 < 0i32;
            v14
        };
        if v15 {
            v8.clone()
        } else {
            let mut v16: bool = v6.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v16 {
                v8.clone()
            } else {
                let mut v17: bool = v8.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                if v17 {
                    v6.clone()
                } else {
                    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v19: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v5, &*v8, &*v18, &*v18].concat());
                    v19.clone()
                }
            }
        }
    }
}
fn absent_from_7(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v4.clone()
    } else {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = std::rc::Rc::<str>::from((v2).split(&*v5).next().unwrap_or(""));
        let mut v7: Rc<str> = std::rc::Rc::<str>::from((v2).split_once(&*v5).map(|(_, rest)| rest).unwrap_or(""));
        let mut v8: Rc<str> = absent_from_7(v0.clone(), v1.clone(), v7.clone());
        let mut v9: i32 = (v0).first().and_then(|header| header.iter().position(|item| &**item == &*v6)).map(|index| index as i32).unwrap_or(-1);
        let mut v10: bool = v9 < 0i32;
        let mut v13: bool = if v10 {
            true
        } else {
            let mut v11: i32 = (v1).first().and_then(|header| header.iter().position(|item| &**item == &*v6)).map(|index| index as i32).unwrap_or(-1);
            let mut v12: bool = v11 < 0i32;
            v12
        };
        if v13 {
            let mut v14: bool = v6.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v14 {
                v8.clone()
            } else {
                let mut v15: bool = v8.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                if v15 {
                    v6.clone()
                } else {
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v5, &*v8, &*v16, &*v16].concat());
                    v17.clone()
                }
            }
        } else {
            v8.clone()
        }
    }
}
fn indices_from_8(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v3.clone()
    } else {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = std::rc::Rc::<str>::from((v1).split(&*v4).next().unwrap_or(""));
        let mut v6: i32 = (v0).first().and_then(|header| header.iter().position(|item| &**item == &*v5)).map(|index| index as i32).unwrap_or(-1);
        let mut v7: Rc<str> = std::rc::Rc::<str>::from((v6).to_string().as_str());
        let mut v8: Rc<str> = std::rc::Rc::<str>::from((v1).split_once(&*v4).map(|(_, rest)| rest).unwrap_or(""));
        let mut v9: Rc<str> = indices_from_8(v0.clone(), v8.clone());
        let mut v10: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v10 {
            v9.clone()
        } else {
            let mut v11: bool = v9.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v11 {
                v7.clone()
            } else {
                let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v4, &*v9, &*v12, &*v12].concat());
                v13.clone()
            }
        }
    }
}
fn winners_from_9(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>>) -> std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>> {
    loop {
        let mut v6: bool = v4 < v3;
        if v6 {
            return v5.clone();
        } else {
            let mut v7: Rc<str> = std::rc::Rc::<str>::from((v1).split(char::from(44u8)).map(|index| index.parse::<usize>().ok().and_then(|index| (v0).get(v3 as usize).and_then(|cells| cells.get(index))).map(|cell| cell.to_string()).unwrap_or_default()).collect::<Vec<_>>().join("|").as_str());
            let mut v8: i32 = (v5).borrow().get(&*v7).copied().unwrap_or(-1);
            let mut v9: bool = v8 < 0i32;
            let mut v11: bool = if v9 {
                true
            } else {
                let mut v10: bool = v2 < 0i32;
                v10
            };
            let mut v23: bool = if v11 {
                true
            } else {
                let mut v12: Rc<str> = std::rc::Rc::<str>::from((v0).get(v3 as usize).and_then(|cells| cells.get(v2 as usize)).map(|cell| &**cell).unwrap_or(""));
                let mut v13: Rc<str> = std::rc::Rc::<str>::from((v0).get(v8 as usize).and_then(|cells| cells.get(v2 as usize)).map(|cell| &**cell).unwrap_or(""));
                let mut v14: bool = (v12).trim().parse::<f64>().map(|value| value.is_finite()).unwrap_or(false);
                let mut v16: bool = if v14 {
                    let mut v15: bool = (v13).trim().parse::<f64>().map(|value| value.is_finite()).unwrap_or(false);
                    v15
                } else {
                    false
                };
                if v16 {
                    let mut v17: f64 = (v13).trim().parse::<f64>().unwrap_or(0.0);
                    let mut v18: f64 = (v12).trim().parse::<f64>().unwrap_or(0.0);
                    let mut v19: bool = v17 <= v18;
                    v19
                } else {
                    let mut v20: bool = (v12) < (v13);
                    let mut v21: bool = v20 == false;
                    v21
                }
            };
            if v23 {
                { (v5).borrow_mut().insert(v7.clone(), v3); };
                ()
            };
            let mut v24: i32 = v3.wrapping_add(1i32);
            (v0, v1, v2, v3, v4, v5) = (v0.clone(), v1.clone(), v2, v24, v4, v5.clone());
            continue;
        }
    }
}
fn pairs_from_10(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: Rc<str>) -> Rc<UH0> {
    let mut v3: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v3 {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoPairs); } CASE.with(|case| case.clone()) }
    } else {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = std::rc::Rc::<str>::from((v2).split(&*v5).next().unwrap_or(""));
        let mut v7: i32 = (v0).first().and_then(|header| header.iter().position(|item| &**item == &*v6)).map(|index| index as i32).unwrap_or(-1);
        let mut v8: i32 = (v1).first().and_then(|header| header.iter().position(|item| &**item == &*v6)).map(|index| index as i32).unwrap_or(-1);
        let mut v9: Rc<str> = std::rc::Rc::<str>::from((v2).split_once(&*v5).map(|(_, rest)| rest).unwrap_or(""));
        let mut v10: Rc<UH0> = pairs_from_10(v0.clone(), v1.clone(), v9.clone());
        Rc::new(UH0::UH0_PairThen(v6.clone(), v7, v8, v10.clone()))
    }
}
fn difference_counted_13(mut v0: Rc<UH1>, mut v1: Rc<str>) -> Rc<UH1> {
    match &*v0 {
        UH1::UH1_DifferenceThen(v4, v5, v6) => {
            let mut v4: Rc<str> = v4.clone();
            let mut v5: i32 = *v5;
            let mut v6: Rc<UH1> = v6.clone();
            let mut v7: bool = v4.clone() == v1.clone();
            if v7 {
                let mut v8: i32 = v5.wrapping_add(1i32);
                Rc::new(UH1::UH1_DifferenceThen(v4.clone(), v8, v6.clone()))
            } else {
                let mut v10: bool = (v1.clone()) < (v4.clone());
                if v10 {
                    Rc::new(UH1::UH1_DifferenceThen(v1.clone(), 1i32, v0.clone()))
                } else {
                    let mut v12: Rc<UH1> = difference_counted_13(v6.clone(), v1.clone());
                    Rc::new(UH1::UH1_DifferenceThen(v4.clone(), v5, v12.clone()))
                }
            }
        }
        UH1::UH1_NoDifferences => {
            let mut v2: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_NoDifferences); } CASE.with(|case| case.clone()) };
            Rc::new(UH1::UH1_DifferenceThen(v1.clone(), 1i32, v2.clone()))
        }
    }
}
fn pairs_compared_12(mut v0: Rc<UH0>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v3: i32, mut v4: i32, mut v5: Rc<str>, mut v6: u64, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: Rc<str>, mut v11: Rc<UH1>) -> (i32, i32, i32, Rc<str>, Rc<UH1>) {
    let (mut v56, mut v57, mut v58, mut v59, mut v60): (i32, i32, i32, Rc<str>, Rc<UH1>) = match &*v0 {
        UH0::UH0_NoPairs => {
            (v7, v8, v9, v10.clone(), v11.clone())
        }
        _ => {
            let (mut v51, mut v52, mut v53, mut v54, mut v55): (i32, i32, i32, Rc<str>, Rc<UH1>) = match &*v0 {
                UH0::UH0_PairThen(v12, v13, v14, v15) => {
                    let mut v12: Rc<str> = v12.clone();
                    let mut v13: i32 = *v13;
                    let mut v14: i32 = *v14;
                    let mut v15: Rc<UH0> = v15.clone();
                    let mut v16: Rc<str> = std::rc::Rc::<str>::from((v1).get(v3 as usize).and_then(|cells| cells.get(v13 as usize)).map(|cell| &**cell).unwrap_or(""));
                    let mut v17: Rc<str> = std::rc::Rc::<str>::from((v2).get(v4 as usize).and_then(|cells| cells.get(v14 as usize)).map(|cell| &**cell).unwrap_or(""));
                    let mut v18: bool = v16.clone() == v17.clone();
                    if v18 {
                        let (mut v19, mut v20, mut v21, mut v22, mut v23): (i32, i32, i32, Rc<str>, Rc<UH1>) = pairs_compared_12(v15.clone(), v1.clone(), v2.clone(), v3, v4, v5.clone(), v6, v7, v8, v9, v10.clone(), v11.clone());
                        (v19, v20, v21, v22.clone(), v23.clone())
                    } else {
                        let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                        let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v24));
                        let mut v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v12));
                        let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v24));
                        let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v24));
                        let mut v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v17));
                        let mut v30: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v29));
                        let mut v31: bool = (v9 as u64) < v6;
                        let mut v32: Rc<UH1> = difference_counted_13(v11.clone(), v12.clone());
                        let mut v34: i32 = if v31 {
                            let mut v33: i32 = v9.wrapping_add(1i32);
                            v33
                        } else {
                            v9
                        };
                        let mut v40: Rc<str> = if v31 {
                            let mut v35: bool = v10.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            if v35 {
                                v30.clone()
                            } else {
                                let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v38: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v36, &*v30, &*v37, &*v37].concat());
                                v38.clone()
                            }
                        } else {
                            v10.clone()
                        };
                        let (mut v41, mut v42, mut v43, mut v44, mut v45): (i32, i32, i32, Rc<str>, Rc<UH1>) = pairs_compared_12(v15.clone(), v1.clone(), v2.clone(), v3, v4, v5.clone(), v6, v7, v8, v34, v40.clone(), v32.clone());
                        (v41, v42, v43, v44.clone(), v45.clone())
                    }
                }
                _ => unreachable!(),
            };
            (v51, v52, v53, v54.clone(), v55.clone())
        }
    };
    (v56, v57, v58, v59.clone(), v60.clone())
}
fn left_compared_11(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: Rc<str>, mut v3: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>>, mut v4: bool, mut v5: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>>, mut v6: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>>, mut v7: Rc<UH0>, mut v8: u64, mut v9: i32, mut v10: i32, mut v11: i32, mut v12: i32, mut v13: Rc<str>, mut v14: Rc<UH1>) -> (i32, i32, i32, Rc<str>, Rc<UH1>) {
    let mut v15: i32 = ((v0).len().max(1) - 1) as i32;
    let mut v16: bool = v15 < v9;
    let (mut v52, mut v53, mut v54, mut v55, mut v56): (i32, i32, i32, Rc<str>, Rc<UH1>) = if v16 {
        (v10, v11, v12, v13.clone(), v14.clone())
    } else {
        let mut v17: Rc<str> = std::rc::Rc::<str>::from((v2).split(char::from(44u8)).map(|index| index.parse::<usize>().ok().and_then(|index| (v0).get(v9 as usize).and_then(|cells| cells.get(index))).map(|cell| cell.to_string()).unwrap_or_default()).collect::<Vec<_>>().join("|").as_str());
        let mut v20: bool = if v4 {
            let mut v18: i32 = (v3).borrow().get(&*v17).copied().unwrap_or(-1);
            let mut v19: bool = v18 != v9;
            v19
        } else {
            false
        };
        let (mut v47, mut v48, mut v49, mut v50, mut v51): (i32, i32, i32, Rc<str>, Rc<UH1>) = if v20 {
            let mut v21: i32 = v9.wrapping_add(1i32);
            let (mut v22, mut v23, mut v24, mut v25, mut v26): (i32, i32, i32, Rc<str>, Rc<UH1>) = left_compared_11(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4, v5.clone(), v6.clone(), v7.clone(), v8, v21, v10, v11, v12, v13.clone(), v14.clone());
            (v22, v23, v24, v25.clone(), v26.clone())
        } else {
            { (v6).borrow_mut().insert(v17.clone(), v9); };
            let mut v27: i32 = (v5).borrow().get(&*v17).copied().unwrap_or(-1);
            let mut v28: i32 = v10.wrapping_add(1i32);
            let mut v29: bool = v27 < 0i32;
            let (mut v36, mut v37, mut v38, mut v39, mut v40): (i32, i32, i32, Rc<str>, Rc<UH1>) = if v29 {
                let mut v30: i32 = v11.wrapping_add(1i32);
                (v28, v30, v12, v13.clone(), v14.clone())
            } else {
                let (mut v31, mut v32, mut v33, mut v34, mut v35): (i32, i32, i32, Rc<str>, Rc<UH1>) = pairs_compared_12(v7.clone(), v0.clone(), v1.clone(), v9, v27, v17.clone(), v8, v28, v11, v12, v13.clone(), v14.clone());
                (v31, v32, v33, v34.clone(), v35.clone())
            };
            let mut v41: i32 = v9.wrapping_add(1i32);
            let (mut v42, mut v43, mut v44, mut v45, mut v46): (i32, i32, i32, Rc<str>, Rc<UH1>) = left_compared_11(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4, v5.clone(), v6.clone(), v7.clone(), v8, v41, v36, v37, v38, v39.clone(), v40.clone());
            (v42, v43, v44, v45.clone(), v46.clone())
        };
        (v47, v48, v49, v50.clone(), v51.clone())
    };
    (v52, v53, v54, v55.clone(), v56.clone())
}
fn right_unseen_14(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>, mut v2: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>>, mut v3: bool, mut v4: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>>, mut v5: i32, mut v6: i32, mut v7: i32) -> (i32, i32) {
    loop {
        let mut v8: i32 = ((v0).len().max(1) - 1) as i32;
        let mut v9: bool = v8 < v5;
        if v9 {
            return (v6, v7);
        } else {
            let mut v10: Rc<str> = std::rc::Rc::<str>::from((v1).split(char::from(44u8)).map(|index| index.parse::<usize>().ok().and_then(|index| (v0).get(v5 as usize).and_then(|cells| cells.get(index))).map(|cell| cell.to_string()).unwrap_or_default()).collect::<Vec<_>>().join("|").as_str());
            let mut v13: bool = if v3 {
                let mut v11: i32 = (v2).borrow().get(&*v10).copied().unwrap_or(-1);
                let mut v12: bool = v11 != v5;
                v12
            } else {
                false
            };
            if v13 {
                let mut v14: i32 = v5.wrapping_add(1i32);
                (v0, v1, v2, v3, v4, v5, v6, v7) = (v0.clone(), v1.clone(), v2.clone(), v3, v4.clone(), v14, v6, v7);
                continue;
            } else {
                let mut v17: i32 = (v4).borrow().get(&*v10).copied().unwrap_or(-1);
                let mut v18: bool = v17 < 0i32;
                let mut v19: i32 = if v18 {
                    1i32
                } else {
                    0i32
                };
                let mut v20: i32 = v5.wrapping_add(1i32);
                let mut v21: i32 = v6.wrapping_add(1i32);
                let mut v22: i32 = v7.wrapping_add(v19);
                (v0, v1, v2, v3, v4, v5, v6, v7) = (v0.clone(), v1.clone(), v2.clone(), v3, v4.clone(), v20, v21, v22);
                continue;
            }
        }
    }
}
fn differences_printed_15(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH1::UH1_DifferenceThen(v2, v3, v4) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: i32 = *v3;
                let mut v4: Rc<UH1> = v4.clone();
                let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("column "); } LIT.with(|lit| lit.clone()) };
                let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
                let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" differs="); } LIT.with(|lit| lit.clone()) };
                let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
                let mut v9: Rc<str> = std::rc::Rc::<str>::from((v3).to_string().as_str());
                let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
                println!("{}", &*v10);
                let mut v11: i32 = v1.wrapping_add(v3);
                (v0, v1) = (v4.clone(), v11);
                continue;
            }
            UH1::UH1_NoDifferences => {
                return v1;
            }
        }
    }
}
fn diff_tables_2(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: u64, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: bool) -> i32 {
    let mut v10: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = if v10 {
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = std::rc::Rc::<str>::from((v0).first().map(|header| header.iter().map(|item| &**item).collect::<Vec<_>>().join(&*v11)).unwrap_or_default().as_str());
        shared_columns_3(v1.clone(), v12.clone(), v5.clone())
    } else {
        v3.clone()
    };
    let mut v15: Rc<str> = absent_from_7(v0.clone(), v1.clone(), v5.clone());
    let mut v16: Rc<str> = absent_from_7(v0.clone(), v1.clone(), v14.clone());
    let mut v17: bool = v15.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = if v17 {
        v16.clone()
    } else {
        let mut v18: bool = v16.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v18 {
            v15.clone()
        } else {
            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v21: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v19, &*v16, &*v20, &*v20].concat());
            v21.clone()
        }
    };
    let mut v24: bool = v8.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v30: bool = if v24 {
        let mut v25: i32 = (v0).first().and_then(|header| header.iter().position(|item| &**item == &*v8)).map(|index| index as i32).unwrap_or(-1);
        let mut v26: bool = v25 < 0i32;
        if v26 {
            true
        } else {
            let mut v27: i32 = (v1).first().and_then(|header| header.iter().position(|item| &**item == &*v8)).map(|index| index as i32).unwrap_or(-1);
            let mut v28: bool = v27 < 0i32;
            v28
        }
    } else {
        false
    };
    let mut v32: Rc<str> = if v30 {
        v8.clone()
    } else {
        let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v31.clone()
    };
    let mut v33: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v33 {
        let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("diff needs --key a,b"); } LIT.with(|lit| lit.clone()) };
        let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: table "); } LIT.with(|lit| lit.clone()) };
        let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v37: Rc<str> = std::rc::Rc::<str>::from([&*v35, &*v34, &*v36, &*v36, &*v36].concat());
        eprintln!("{}", &*v37);
        2i32
    } else {
        let mut v38: bool = v23.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v44: Rc<str> = if v38 {
            v32.clone()
        } else {
            let mut v39: bool = v32.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v39 {
                v23.clone()
            } else {
                let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v42: Rc<str> = std::rc::Rc::<str>::from([&*v23, &*v40, &*v32, &*v41, &*v41].concat());
                v42.clone()
            }
        };
        let mut v45: bool = v44.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v45 {
            let mut v51: Rc<str> = if v38 {
                v32.clone()
            } else {
                let mut v46: bool = v32.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                if v46 {
                    v23.clone()
                } else {
                    let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                    let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v49: Rc<str> = std::rc::Rc::<str>::from([&*v23, &*v47, &*v32, &*v48, &*v48].concat());
                    v49.clone()
                }
            };
            let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("diff unknown columns: "); } LIT.with(|lit| lit.clone()) };
            let mut v53: Rc<str> = Rc::<str>::from(format!("{}{}", v52, v51));
            let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: table "); } LIT.with(|lit| lit.clone()) };
            let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v56: Rc<str> = std::rc::Rc::<str>::from([&*v54, &*v53, &*v55, &*v55, &*v55].concat());
            eprintln!("{}", &*v56);
            2i32
        } else {
            let mut v57: Rc<str> = indices_from_8(v0.clone(), v5.clone());
            let mut v58: Rc<str> = indices_from_8(v1.clone(), v5.clone());
            let mut v60: i32 = if v24 {
                let mut v59: i32 = (v0).first().and_then(|header| header.iter().position(|item| &**item == &*v8)).map(|index| index as i32).unwrap_or(-1);
                v59
            } else {
                -1i32
            };
            let mut v61: i32 = 1i32;
            let mut v62: i32 = ((v0).len().max(1) - 1) as i32;
            let mut v63: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>> = std::rc::Rc::new(std::cell::RefCell::new(std::collections::HashMap::new()));
            let mut v64: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>> = winners_from_9(v0.clone(), v57.clone(), v60, v61, v62, v63.clone());
            let mut v66: i32 = if v24 {
                let mut v65: i32 = (v1).first().and_then(|header| header.iter().position(|item| &**item == &*v8)).map(|index| index as i32).unwrap_or(-1);
                v65
            } else {
                -1i32
            };
            let mut v67: i32 = 1i32;
            let mut v68: i32 = ((v1).len().max(1) - 1) as i32;
            let mut v69: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>> = std::rc::Rc::new(std::cell::RefCell::new(std::collections::HashMap::new()));
            let mut v70: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>> = winners_from_9(v1.clone(), v58.clone(), v66, v67, v68, v69.clone());
            let mut v71: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>> = std::rc::Rc::new(std::cell::RefCell::new(std::collections::HashMap::new()));
            let mut v72: Rc<UH0> = pairs_from_10(v0.clone(), v1.clone(), v14.clone());
            let mut v73: i32 = 1i32;
            let mut v74: i32 = 0i32;
            let mut v75: i32 = 0i32;
            let mut v76: i32 = 0i32;
            let mut v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v78: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_NoDifferences); } CASE.with(|case| case.clone()) };
            let (mut v79, mut v80, mut v81, mut v82, mut v83): (i32, i32, i32, Rc<str>, Rc<UH1>) = left_compared_11(v0.clone(), v1.clone(), v57.clone(), v64.clone(), v24, v70.clone(), v71.clone(), v72.clone(), v6, v73, v74, v75, v76, v77.clone(), v78.clone());
            let mut v84: i32 = 1i32;
            let mut v85: i32 = 0i32;
            let mut v86: i32 = 0i32;
            let (mut v87, mut v88): (i32, i32) = right_unseen_14(v1.clone(), v58.clone(), v70.clone(), v24, v71.clone(), v84, v85, v86);
            let mut v89: i32 = 0i32;
            let mut v90: i32 = differences_printed_15(v83.clone(), v89);
            let mut v91: bool = v82.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v91 {
                let mut v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("key\tcolumn\tleft\tright\n"); } LIT.with(|lit| lit.clone()) };
                let mut v93: Rc<str> = Rc::<str>::from(format!("{}{}", v92, v82));
                println!("{}", &*v93);
                ()
            };
            let mut v94: Rc<str> = std::rc::Rc::<str>::from((v79).to_string().as_str());
            let mut v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie table diff ok left_rows="); } LIT.with(|lit| lit.clone()) };
            let mut v96: Rc<str> = Rc::<str>::from(format!("{}{}", v95, v94));
            let mut v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" right_rows="); } LIT.with(|lit| lit.clone()) };
            let mut v98: Rc<str> = Rc::<str>::from(format!("{}{}", v96, v97));
            let mut v99: Rc<str> = std::rc::Rc::<str>::from((v87).to_string().as_str());
            let mut v100: Rc<str> = Rc::<str>::from(format!("{}{}", v98, v99));
            let mut v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" only_left="); } LIT.with(|lit| lit.clone()) };
            let mut v102: Rc<str> = Rc::<str>::from(format!("{}{}", v100, v101));
            let mut v103: Rc<str> = std::rc::Rc::<str>::from((v80).to_string().as_str());
            let mut v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" only_right="); } LIT.with(|lit| lit.clone()) };
            let mut v105: Rc<str> = Rc::<str>::from(format!("{}{}", v103, v104));
            let mut v106: Rc<str> = std::rc::Rc::<str>::from((v88).to_string().as_str());
            let mut v107: Rc<str> = Rc::<str>::from(format!("{}{}", v105, v106));
            let mut v108: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" differing_cells="); } LIT.with(|lit| lit.clone()) };
            let mut v109: Rc<str> = Rc::<str>::from(format!("{}{}", v107, v108));
            let mut v110: Rc<str> = std::rc::Rc::<str>::from((v90).to_string().as_str());
            let mut v111: Rc<str> = Rc::<str>::from(format!("{}{}", v109, v110));
            let mut v112: Rc<str> = Rc::<str>::from(format!("{}{}", v102, v111));
            let mut v115: Rc<str> = if v24 {
                let mut v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" latest_by="); } LIT.with(|lit| lit.clone()) };
                let mut v114: Rc<str> = Rc::<str>::from(format!("{}{}", v113, v8));
                v114.clone()
            } else {
                v77.clone()
            };
            let mut v116: Rc<str> = Rc::<str>::from(format!("{}{}", v112, v115));
            println!("{}", &*v116);
            0i32
        }
    }
}
fn diff_files_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: u64, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: bool) -> i32 {
    let (mut v10, mut v11, mut v12): (bool, std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, Rc<str>) = { let path = &*v0; match std::fs::read_to_string(path) { Ok(text) => { let separator = if path.ends_with(".csv") { char::from(44u8) } else { char::from(9u8) }; (true, std::rc::Rc::new(text.lines().map(|line| line.split(separator).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>()), { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) } Err(error) => (false, std::rc::Rc::new(Vec::new()), std::rc::Rc::<str>::from(format!("{path}: {error}").as_str())) } };
    let mut v15: US0 = if v10 {
        US0::US0_TableLoaded(v11.clone())
    } else {
        US0::US0_TableUnreadable(v12.clone())
    };
    let (mut v16, mut v17, mut v18): (bool, std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, Rc<str>) = { let path = &*v1; match std::fs::read_to_string(path) { Ok(text) => { let separator = if path.ends_with(".csv") { char::from(44u8) } else { char::from(9u8) }; (true, std::rc::Rc::new(text.lines().map(|line| line.split(separator).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>()), { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) } Err(error) => (false, std::rc::Rc::new(Vec::new()), std::rc::Rc::<str>::from(format!("{path}: {error}").as_str())) } };
    let mut v21: US0 = if v16 {
        US0::US0_TableLoaded(v17.clone())
    } else {
        US0::US0_TableUnreadable(v18.clone())
    };
    match &v15 {
        US0::US0_TableUnreadable(v22) => {
            let mut v22: Rc<str> = v22.clone();
            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: table "); } LIT.with(|lit| lit.clone()) };
            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v25: Rc<str> = std::rc::Rc::<str>::from([&*v23, &*v22, &*v24, &*v24, &*v24].concat());
            eprintln!("{}", &*v25);
            2i32
        }
        _ => {
            match &v21 {
                US0::US0_TableUnreadable(v26) => {
                    let mut v26: Rc<str> = v26.clone();
                    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: table "); } LIT.with(|lit| lit.clone()) };
                    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v29: Rc<str> = std::rc::Rc::<str>::from([&*v27, &*v26, &*v28, &*v28, &*v28].concat());
                    eprintln!("{}", &*v29);
                    2i32
                }
                _ => {
                    match &v15 {
                        US0::US0_TableLoaded(v30) => {
                            let mut v30: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = v30.clone();
                            match &v21 {
                                US0::US0_TableLoaded(v31) => {
                                    let mut v31: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = v31.clone();
                                    diff_tables_2(v30.clone(), v31.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6, v7.clone(), v8.clone(), v9)
                                }
                                _ => unreachable!(),
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
}
fn no_request_16() -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>, bool) {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0.clone(), v0.clone(), v0.clone(), v0.clone(), 20u64, v0.clone(), v0.clone(), false)
}
fn table_diff_run_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: u64) -> i32 {
    let (mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>, bool) = no_request_16();
    diff_files_1(v0.clone(), v1.clone(), v6.clone(), v3.clone(), v8.clone(), v2.clone(), v5, v11.clone(), v4.clone(), v13)
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: u64| -> i32 {
        table_diff_run_0(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_table_diff_run(v0: &str, v1: &str, v2: &str, v3: &str, v4: &str, v5: u64) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), Rc::<str>::from(v4), v5)
}
