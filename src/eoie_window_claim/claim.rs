#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_StoredRead(Rc<str>, Rc<str>, u64),
    US0_StoredShape(Rc<str>, Rc<str>, u64),
}
#[derive(Clone)]
enum UH0 {
    UH0_NoLeases,
    UH0_LeaseThen(US0, Rc<UH0>),
}
#[derive(Clone)]
enum US2 {
    US2_LiveGui,
    US2_TestSuite,
    US2_HeavyBuild,
    US2_Compile,
    US2_CompilerInstall,
    US2_TreeRead,
    US2_Bench,
    US2_CompileHold,
}
#[derive(Clone)]
enum US3 {
    US3_Held,
    US3_Waiting,
}
#[derive(Clone)]
enum US1 {
    US1_RowKnown(Rc<str>, US2, US3, Rc<str>, u64, u64),
    US1_RowUnknown,
}
#[derive(Clone)]
enum US4 {
    US4_ClassKnown(US2),
    US4_ClassUnknown,
}
#[derive(Clone)]
enum US5 {
    US5_Some(u64),
    US5_None,
}
#[derive(Clone)]
enum US6 {
    US6_TreeFree,
    US6_TreeHeld(Rc<str>, Rc<str>, u64),
}
#[derive(Clone)]
enum US7 {
    US7_Some(Rc<str>, Rc<str>, u64),
    US7_None,
}
#[derive(Clone)]
enum US8 {
    US8_EditAllowed,
    US8_EditRefused(Rc<str>, Rc<str>, u64),
}
struct Mut0 { l0: Rc<str> }
fn stderr_line_1(mut v0: Rc<str>) -> () {
    eprintln!("{}", &*v0);
    ()
}
fn now_ms_2() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn windows_root_3() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("EOIE_WINDOWS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("eoie").join("windows"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("eoie").join("windows"))).unwrap_or_else(|| std::path::PathBuf::from(".eoie-windows")).to_string_lossy().as_ref());
    v0.clone()
}
fn stored_rows_4(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { let mut rows = std::fs::read_dir(&*v0).into_iter().flatten().flatten().map(|entry| entry.path()).filter(|path| path.extension().map(|ext| ext == "window").unwrap_or(false)).filter_map(|path| std::fs::read_to_string(&path).ok()).map(|text| text.trim().to_owned()).filter(|text| !text.is_empty()).collect::<Vec<_>>(); rows.sort_by_key(|row| row.split(char::from(9u8)).nth(4).and_then(|start| start.parse::<u64>().ok()).unwrap_or(0)); std::rc::Rc::<str>::from(rows.join("
")) };
    v1.clone()
}
fn other_rows_6(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().skip(1).collect::<Vec<_>>().join("
"));
    v1.clone()
}
fn first_row_7(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().next().unwrap_or(""));
    v1.clone()
}
fn tab_field_9(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(v1 as usize).unwrap_or(""));
    v2.clone()
}
fn class_word_11(mut v0: US2) -> Rc<str> {
    match &v0 {
        US2::US2_Bench => {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bench"); } LIT.with(|lit| lit.clone()) };
            v7.clone()
        }
        US2::US2_Compile => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US2::US2_CompileHold => {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile-hold"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US2::US2_CompilerInstall => {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compiler-install"); } LIT.with(|lit| lit.clone()) };
            v5.clone()
        }
        US2::US2_HeavyBuild => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-build"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
        US2::US2_LiveGui => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("live-gui"); } LIT.with(|lit| lit.clone()) };
            v1.clone()
        }
        US2::US2_TestSuite => {
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test-suite"); } LIT.with(|lit| lit.clone()) };
            v2.clone()
        }
        US2::US2_TreeRead => {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tree-read"); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        }
    }
}
fn class_of_10(mut v0: Rc<str>) -> US4 {
    let mut v1: US2 = US2::US2_LiveGui;
    let mut v2: Rc<str> = class_word_11(v1.clone());
    let mut v3: bool = v0.clone() == v2.clone();
    if v3 {
        let mut v4: US2 = US2::US2_LiveGui;
        US4::US4_ClassKnown(v4.clone())
    } else {
        let mut v6: US2 = US2::US2_TestSuite;
        let mut v7: Rc<str> = class_word_11(v6.clone());
        let mut v8: bool = v0.clone() == v7.clone();
        if v8 {
            let mut v9: US2 = US2::US2_TestSuite;
            US4::US4_ClassKnown(v9.clone())
        } else {
            let mut v11: US2 = US2::US2_HeavyBuild;
            let mut v12: Rc<str> = class_word_11(v11.clone());
            let mut v13: bool = v0.clone() == v12.clone();
            if v13 {
                let mut v14: US2 = US2::US2_HeavyBuild;
                US4::US4_ClassKnown(v14.clone())
            } else {
                let mut v16: US2 = US2::US2_Compile;
                let mut v17: Rc<str> = class_word_11(v16.clone());
                let mut v18: bool = v0.clone() == v17.clone();
                if v18 {
                    let mut v19: US2 = US2::US2_Compile;
                    US4::US4_ClassKnown(v19.clone())
                } else {
                    let mut v21: US2 = US2::US2_CompilerInstall;
                    let mut v22: Rc<str> = class_word_11(v21.clone());
                    let mut v23: bool = v0.clone() == v22.clone();
                    if v23 {
                        let mut v24: US2 = US2::US2_CompilerInstall;
                        US4::US4_ClassKnown(v24.clone())
                    } else {
                        let mut v26: US2 = US2::US2_TreeRead;
                        let mut v27: Rc<str> = class_word_11(v26.clone());
                        let mut v28: bool = v0.clone() == v27.clone();
                        if v28 {
                            let mut v29: US2 = US2::US2_TreeRead;
                            US4::US4_ClassKnown(v29.clone())
                        } else {
                            let mut v31: US2 = US2::US2_Bench;
                            let mut v32: Rc<str> = class_word_11(v31.clone());
                            let mut v33: bool = v0.clone() == v32.clone();
                            if v33 {
                                let mut v34: US2 = US2::US2_Bench;
                                US4::US4_ClassKnown(v34.clone())
                            } else {
                                let mut v36: US2 = US2::US2_CompileHold;
                                let mut v37: Rc<str> = class_word_11(v36.clone());
                                let mut v38: bool = v0.clone() == v37.clone();
                                if v38 {
                                    let mut v39: US2 = US2::US2_CompileHold;
                                    US4::US4_ClassKnown(v39.clone())
                                } else {
                                    US4::US4_ClassUnknown
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn method13(mut v0: Option<u64>) -> Option<u64> {
    v0.clone()
}
fn closure1() -> Rc<dyn Fn((u64)) -> US5> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US5> = Rc::new(move |mut v0: (u64)| -> US5 {
        let mut v1: u64 = (v0);
        US5::US5_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_of_12(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method13(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US5> = closure1();
    let mut v4: Option<US5> = v2.map(|x| v3(x));
    let mut v5: US5 = US5::US5_None;
    let mut v6: US5 = v4.unwrap_or(v5);
    match &v6 {
        US5::US5_None => {
            0u64
        }
        US5::US5_Some(v7) => {
            let mut v7: u64 = *v7;
            v7
        }
    }
}
fn row_of_8(mut v0: Rc<str>) -> US1 {
    let mut v1: i32 = 1i32;
    let mut v2: Rc<str> = tab_field_9(v0.clone(), v1);
    let mut v3: US4 = class_of_10(v2.clone());
    match &v3 {
        US4::US4_ClassKnown(v5) => {
            let mut v5: US2 = v5.clone();
            let mut v6: i32 = 2i32;
            let mut v7: Rc<str> = tab_field_9(v0.clone(), v6);
            let mut v8: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
            let mut v11: US3 = if v8 {
                US3::US3_Held
            } else {
                US3::US3_Waiting
            };
            let mut v12: i32 = 0i32;
            let mut v13: Rc<str> = tab_field_9(v0.clone(), v12);
            let mut v14: i32 = 3i32;
            let mut v15: Rc<str> = tab_field_9(v0.clone(), v14);
            let mut v16: i32 = 4i32;
            let mut v17: Rc<str> = tab_field_9(v0.clone(), v16);
            let mut v18: u64 = decimal_of_12(v17.clone());
            let mut v19: i32 = 5i32;
            let mut v20: Rc<str> = tab_field_9(v0.clone(), v19);
            let mut v21: u64 = decimal_of_12(v20.clone());
            US1::US1_RowKnown(v13.clone(), v5.clone(), v11.clone(), v15.clone(), v18, v21)
        }
        US4::US4_ClassUnknown => {
            US1::US1_RowUnknown
        }
    }
}
fn leases_of_rows_5(mut v0: Rc<str>, mut v1: Rc<UH0>) -> Rc<UH0> {
    let mut v2: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v2 {
        v1.clone()
    } else {
        let mut v3: Rc<str> = other_rows_6(v0.clone());
        let mut v4: Rc<UH0> = leases_of_rows_5(v3.clone(), v1.clone());
        let mut v5: Rc<str> = first_row_7(v0.clone());
        let mut v6: US1 = row_of_8(v5.clone());
        match &v6 {
            US1::US1_RowKnown(v7, v8, v9, v10, v11, v12) => {
                let mut v7: Rc<str> = v7.clone();
                let mut v8: US2 = v8.clone();
                let mut v9: US3 = v9.clone();
                let mut v10: Rc<str> = v10.clone();
                let mut v11: u64 = *v11;
                let mut v12: u64 = *v12;
                let mut v13: bool = match &v8 {
                    US2::US2_TreeRead => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                if v13 {
                    let mut v14: US0 = US0::US0_StoredRead(v10.clone(), v7.clone(), v12);
                    Rc::new(UH0::UH0_LeaseThen(v14.clone(), v4.clone()))
                } else {
                    v4.clone()
                }
            }
            US1::US1_RowUnknown => {
                v4.clone()
            }
        }
    }
}
fn claim_of_14(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: u64) -> US6 {
    loop {
        match &*v0 {
            UH0::UH0_LeaseThen(v4, v5) => {
                let mut v4: US0 = v4.clone();
                let mut v5: Rc<UH0> = v5.clone();
                let mut v15: US7 = match &v4 {
                    US0::US0_StoredRead(v6, v7, v8) => {
                        let mut v6: Rc<str> = v6.clone();
                        let mut v7: Rc<str> = v7.clone();
                        let mut v8: u64 = *v8;
                        US7::US7_Some(v6.clone(), v7.clone(), v8)
                    }
                    US0::US0_StoredShape(v10, v11, v12) => {
                        let mut v10: Rc<str> = v10.clone();
                        let mut v11: Rc<str> = v11.clone();
                        let mut v12: u64 = *v12;
                        US7::US7_None
                    }
                };
                match &v15 {
                    US7::US7_None => {
                        (v0, v1, v2) = (v5.clone(), v1.clone(), v2);
                        continue;
                    }
                    US7::US7_Some(v16, v17, v18) => {
                        let mut v16: Rc<str> = v16.clone();
                        let mut v17: Rc<str> = v17.clone();
                        let mut v18: u64 = *v18;
                        let mut v19: bool = v2 < v18;
                        let mut v38: bool = if v19 {
                            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                            let mut v22: Rc<str> = Rc::<str>::from(v1.replace(&*v20, &*v21));
                            let mut v23: Rc<str> = Rc::<str>::from(v22.to_lowercase());
                            let mut v24: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v21));
                            let mut v25: Rc<str> = Rc::<str>::from(v16.replace(&*v20, &*v21));
                            let mut v26: Rc<str> = Rc::<str>::from(v25.to_lowercase());
                            let mut v27: bool = v26.ends_with(&*v21);
                            let mut v29: Rc<str> = if v27 {
                                v26.clone()
                            } else {
                                let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v21));
                                v28.clone()
                            };
                            let mut v30: bool = v24.clone() == v29.clone();
                            if v30 {
                                true
                            } else {
                                let mut v31: Rc<str> = Rc::<str>::from(v16.replace(&*v20, &*v21));
                                let mut v32: Rc<str> = Rc::<str>::from(v31.to_lowercase());
                                let mut v33: bool = v32.ends_with(&*v21);
                                let mut v35: Rc<str> = if v33 {
                                    v32.clone()
                                } else {
                                    let mut v34: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v21));
                                    v34.clone()
                                };
                                let mut v36: bool = v23.starts_with(&*v35);
                                v36
                            }
                        } else {
                            false
                        };
                        if v38 {
                            return US6::US6_TreeHeld(v16.clone(), v17.clone(), v18);
                        } else {
                            (v0, v1, v2) = (v5.clone(), v1.clone(), v2);
                            continue;
                        }
                    }
                }
            }
            UH0::UH0_NoLeases => {
                return US6::US6_TreeFree;
            }
        }
    }
}
fn stdout_line_15(mut v0: Rc<str>) -> () {
    println!("{}", &*v0);
    ()
}
fn method17(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3.clone();
    ()
}
fn format_real_16(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method17(v2.clone(), v3.clone());
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn window_claim_run_0() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: window claim <path> (refused while a tree-read window covers the path)"); } LIT.with(|lit| lit.clone()) };
        stderr_line_1(v3.clone());
        2i32
    } else {
        let mut v4: u64 = now_ms_2();
        let mut v5: Rc<str> = windows_root_3();
        let mut v6: Rc<str> = stored_rows_4(v5.clone());
        let mut v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoLeases); } CASE.with(|case| case.clone()) };
        let mut v8: Rc<UH0> = leases_of_rows_5(v6.clone(), v7.clone());
        let mut v9: US6 = claim_of_14(v8.clone(), v1.clone(), v4);
        let mut v16: US8 = match &v9 {
            US6::US6_TreeFree => {
                US8::US8_EditAllowed
            }
            US6::US6_TreeHeld(v11, v12, v13) => {
                let mut v11: Rc<str> = v11.clone();
                let mut v12: Rc<str> = v12.clone();
                let mut v13: u64 = *v13;
                US8::US8_EditRefused(v11.clone(), v12.clone(), v13)
            }
        };
        match &v16 {
            US8::US8_EditAllowed => {
                let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie window claim ok path="); } LIT.with(|lit| lit.clone()) };
                let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v1));
                let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v19));
                let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit allowed"); } LIT.with(|lit| lit.clone()) };
                let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
                stdout_line_15(v22.clone());
                0i32
            }
            US8::US8_EditRefused(v23, v24, v25) => {
                let mut v23: Rc<str> = v23.clone();
                let mut v24: Rc<str> = v24.clone();
                let mut v25: u64 = *v25;
                let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie window claim refused path="); } LIT.with(|lit| lit.clone()) };
                let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v1));
                let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let mut v29: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v28));
                let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit refused: "); } LIT.with(|lit| lit.clone()) };
                let mut v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v23));
                let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is read by "); } LIT.with(|lit| lit.clone()) };
                let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
                let mut v34: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v24));
                let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" for "); } LIT.with(|lit| lit.clone()) };
                let mut v36: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v35));
                let mut v37: u64 = v25.wrapping_sub(v4);
                let mut v38: u64 = v37.wrapping_add(59999u64);
                let mut v39: u64 = v38.wrapping_div(60000u64);
                let mut v40: Rc<str> = format_real_16(v39);
                let mut v41: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v40));
                let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" more min"); } LIT.with(|lit| lit.clone()) };
                let mut v43: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v42));
                let mut v44: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v43));
                stdout_line_15(v44.clone());
                1i32
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        window_claim_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_window_claim_run() -> i32 {
    closure0()()
}
