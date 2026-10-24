#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_LiveGui,
    US0_TestSuite,
    US0_HeavyBuild,
    US0_Compile,
    US0_CompilerInstall,
    US0_TreeRead,
    US0_Bench,
    US0_CompileHold,
}
#[derive(Clone)]
enum US1 {
    US1_Admitted,
    US1_HeldBack(Rc<str>),
    US1_LowMemory(u64),
}
#[derive(Clone)]
enum US2 {
    US2_ClassKnown(US0),
    US2_ClassUnknown,
}
#[derive(Clone)]
enum US3 {
    US3_Some(u64),
    US3_None,
}
#[derive(Clone)]
enum US5 {
    US5_Held,
    US5_Waiting,
}
#[derive(Clone)]
enum US4 {
    US4_RowKnown(Rc<str>, US0, US5, Rc<str>, u64, u64),
    US4_RowUnknown,
}
fn number_text_2(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1.clone()
}
fn needed_mb_3(mut v0: US0) -> u64 {
    match &v0 {
        US0::US0_Bench => {
            2048u64
        }
        US0::US0_Compile => {
            1024u64
        }
        US0::US0_CompileHold => {
            0u64
        }
        US0::US0_CompilerInstall => {
            2048u64
        }
        US0::US0_HeavyBuild => {
            1024u64
        }
        US0::US0_LiveGui => {
            4096u64
        }
        US0::US0_TestSuite => {
            3072u64
        }
        US0::US0_TreeRead => {
            0u64
        }
    }
}
fn admission_terms_1(mut v0: US0, mut v1: US1) -> Rc<str> {
    match &v1 {
        US1::US1_Admitted => {
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v2.clone()
        }
        US1::US1_HeldBack(v3) => {
            let mut v3: Rc<str> = v3.clone();
            v3.clone()
        }
        US1::US1_LowMemory(v4) => {
            let mut v4: u64 = *v4;
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("reason=low-memory free_mb="); } LIT.with(|lit| lit.clone()) };
            let mut v6: Rc<str> = number_text_2(v4);
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" need_mb="); } LIT.with(|lit| lit.clone()) };
            let mut v8: u64 = needed_mb_3(v0.clone());
            let mut v9: Rc<str> = number_text_2(v8);
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v6, &*v7, &*v9, &*v10].concat());
            v11.clone()
        }
    }
}
fn class_word_5(mut v0: US0) -> Rc<str> {
    match &v0 {
        US0::US0_Bench => {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bench"); } LIT.with(|lit| lit.clone()) };
            v7.clone()
        }
        US0::US0_Compile => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US0::US0_CompileHold => {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile-hold"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US0::US0_CompilerInstall => {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compiler-install"); } LIT.with(|lit| lit.clone()) };
            v5.clone()
        }
        US0::US0_HeavyBuild => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-build"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
        US0::US0_LiveGui => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("live-gui"); } LIT.with(|lit| lit.clone()) };
            v1.clone()
        }
        US0::US0_TestSuite => {
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test-suite"); } LIT.with(|lit| lit.clone()) };
            v2.clone()
        }
        US0::US0_TreeRead => {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tree-read"); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        }
    }
}
fn class_of_4(mut v0: Rc<str>) -> US2 {
    let mut v1: US0 = US0::US0_LiveGui;
    let mut v2: Rc<str> = class_word_5(v1.clone());
    let mut v3: bool = v0.clone() == v2.clone();
    if v3 {
        let mut v4: US0 = US0::US0_LiveGui;
        US2::US2_ClassKnown(v4.clone())
    } else {
        let mut v6: US0 = US0::US0_TestSuite;
        let mut v7: Rc<str> = class_word_5(v6.clone());
        let mut v8: bool = v0.clone() == v7.clone();
        if v8 {
            let mut v9: US0 = US0::US0_TestSuite;
            US2::US2_ClassKnown(v9.clone())
        } else {
            let mut v11: US0 = US0::US0_HeavyBuild;
            let mut v12: Rc<str> = class_word_5(v11.clone());
            let mut v13: bool = v0.clone() == v12.clone();
            if v13 {
                let mut v14: US0 = US0::US0_HeavyBuild;
                US2::US2_ClassKnown(v14.clone())
            } else {
                let mut v16: US0 = US0::US0_Compile;
                let mut v17: Rc<str> = class_word_5(v16.clone());
                let mut v18: bool = v0.clone() == v17.clone();
                if v18 {
                    let mut v19: US0 = US0::US0_Compile;
                    US2::US2_ClassKnown(v19.clone())
                } else {
                    let mut v21: US0 = US0::US0_CompilerInstall;
                    let mut v22: Rc<str> = class_word_5(v21.clone());
                    let mut v23: bool = v0.clone() == v22.clone();
                    if v23 {
                        let mut v24: US0 = US0::US0_CompilerInstall;
                        US2::US2_ClassKnown(v24.clone())
                    } else {
                        let mut v26: US0 = US0::US0_TreeRead;
                        let mut v27: Rc<str> = class_word_5(v26.clone());
                        let mut v28: bool = v0.clone() == v27.clone();
                        if v28 {
                            let mut v29: US0 = US0::US0_TreeRead;
                            US2::US2_ClassKnown(v29.clone())
                        } else {
                            let mut v31: US0 = US0::US0_Bench;
                            let mut v32: Rc<str> = class_word_5(v31.clone());
                            let mut v33: bool = v0.clone() == v32.clone();
                            if v33 {
                                let mut v34: US0 = US0::US0_Bench;
                                US2::US2_ClassKnown(v34.clone())
                            } else {
                                let mut v36: US0 = US0::US0_CompileHold;
                                let mut v37: Rc<str> = class_word_5(v36.clone());
                                let mut v38: bool = v0.clone() == v37.clone();
                                if v38 {
                                    let mut v39: US0 = US0::US0_CompileHold;
                                    US2::US2_ClassKnown(v39.clone())
                                } else {
                                    US2::US2_ClassUnknown
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn first_row_7(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().next().unwrap_or(""));
    v1.clone()
}
fn tab_field_8(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(v1 as usize).unwrap_or(""));
    v2.clone()
}
fn method10(mut v0: Option<u64>) -> Option<u64> {
    v0.clone()
}
fn closure1() -> Rc<dyn Fn((u64)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US3> = Rc::new(move |mut v0: (u64)| -> US3 {
        let mut v1: u64 = (v0);
        US3::US3_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_of_9(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method10(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US3> = closure1();
    let mut v4: Option<US3> = v2.map(|x| v3(x));
    let mut v5: US3 = US3::US3_None;
    let mut v6: US3 = v4.unwrap_or(v5);
    match &v6 {
        US3::US3_None => {
            0u64
        }
        US3::US3_Some(v7) => {
            let mut v7: u64 = *v7;
            v7
        }
    }
}
fn other_rows_11(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().skip(1).collect::<Vec<_>>().join("
"));
    v1.clone()
}
fn held_until_latest_6(mut v0: Rc<str>, mut v1: u64) -> u64 {
    loop {
        let mut v2: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v2 {
            return v1;
        } else {
            let mut v3: Rc<str> = first_row_7(v0.clone());
            let mut v4: i32 = 2i32;
            let mut v5: Rc<str> = tab_field_8(v3.clone(), v4);
            let mut v6: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
            let mut v10: u64 = if v6 {
                let mut v7: i32 = 5i32;
                let mut v8: Rc<str> = tab_field_8(v3.clone(), v7);
                decimal_of_9(v8.clone())
            } else {
                0u64
            };
            let mut v11: Rc<str> = other_rows_11(v0.clone());
            let mut v12: bool = v10 > v1;
            let mut v13: u64 = if v12 {
                v10
            } else {
                v1
            };
            (v0, v1) = (v11.clone(), v13);
            continue;
        }
    }
}
fn compiles_15(mut v0: US0) -> bool {
    match &v0 {
        US0::US0_Compile => {
            true
        }
        _ => {
            false
        }
    }
}
fn lease_only_16(mut v0: US0) -> bool {
    match &v0 {
        US0::US0_TreeRead => {
            true
        }
        _ => {
            false
        }
    }
}
fn exclusive_17(mut v0: US0) -> bool {
    match &v0 {
        US0::US0_CompilerInstall => {
            true
        }
        US0::US0_LiveGui => {
            true
        }
        _ => {
            false
        }
    }
}
fn quiet_18(mut v0: US0) -> bool {
    match &v0 {
        US0::US0_Bench => {
            true
        }
        _ => {
            false
        }
    }
}
fn loud_19(mut v0: US0) -> bool {
    match &v0 {
        US0::US0_Bench => {
            true
        }
        US0::US0_HeavyBuild => {
            true
        }
        US0::US0_TestSuite => {
            true
        }
        _ => {
            false
        }
    }
}
fn holds_compiles_20(mut v0: US0) -> bool {
    match &v0 {
        US0::US0_CompileHold => {
            true
        }
        _ => {
            false
        }
    }
}
fn clashes_14(mut v0: US0, mut v1: US0) -> bool {
    let mut v2: bool = lease_only_16(v0.clone());
    let mut v4: bool = if v2 {
        true
    } else {
        lease_only_16(v1.clone())
    };
    if v4 {
        false
    } else {
        let mut v5: bool = exclusive_17(v0.clone());
        let mut v7: bool = if v5 {
            true
        } else {
            exclusive_17(v1.clone())
        };
        if v7 {
            true
        } else {
            let mut v8: bool = quiet_18(v0.clone());
            let mut v10: bool = if v8 {
                loud_19(v1.clone())
            } else {
                false
            };
            let mut v14: bool = if v10 {
                true
            } else {
                let mut v11: bool = quiet_18(v1.clone());
                if v11 {
                    loud_19(v0.clone())
                } else {
                    false
                }
            };
            let mut v18: bool = if v14 {
                true
            } else {
                let mut v15: bool = holds_compiles_20(v0.clone());
                if v15 {
                    compiles_15(v1.clone())
                } else {
                    false
                }
            };
            if v18 {
                true
            } else {
                let mut v19: bool = holds_compiles_20(v1.clone());
                if v19 {
                    compiles_15(v0.clone())
                } else {
                    false
                }
            }
        }
    }
}
fn row_of_21(mut v0: Rc<str>) -> US4 {
    let mut v1: i32 = 1i32;
    let mut v2: Rc<str> = tab_field_8(v0.clone(), v1);
    let mut v3: US2 = class_of_4(v2.clone());
    match &v3 {
        US2::US2_ClassKnown(v5) => {
            let mut v5: US0 = v5.clone();
            let mut v6: i32 = 2i32;
            let mut v7: Rc<str> = tab_field_8(v0.clone(), v6);
            let mut v8: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
            let mut v11: US5 = if v8 {
                US5::US5_Held
            } else {
                US5::US5_Waiting
            };
            let mut v12: i32 = 0i32;
            let mut v13: Rc<str> = tab_field_8(v0.clone(), v12);
            let mut v14: i32 = 3i32;
            let mut v15: Rc<str> = tab_field_8(v0.clone(), v14);
            let mut v16: i32 = 4i32;
            let mut v17: Rc<str> = tab_field_8(v0.clone(), v16);
            let mut v18: u64 = decimal_of_9(v17.clone());
            let mut v19: i32 = 5i32;
            let mut v20: Rc<str> = tab_field_8(v0.clone(), v19);
            let mut v21: u64 = decimal_of_9(v20.clone());
            US4::US4_RowKnown(v13.clone(), v5.clone(), v11.clone(), v15.clone(), v18, v21)
        }
        US2::US2_ClassUnknown => {
            US4::US4_RowUnknown
        }
    }
}
fn row_blocks_13(mut v0: US0, mut v1: Rc<str>, mut v2: u64, mut v3: bool, mut v4: Rc<str>) -> bool {
    let mut v5: US4 = row_of_21(v4.clone());
    match &v5 {
        US4::US4_RowKnown(v6, v7, v8, v9, v10, v11) => {
            let mut v6: Rc<str> = v6.clone();
            let mut v7: US0 = v7.clone();
            let mut v8: US5 = v8.clone();
            let mut v9: Rc<str> = v9.clone();
            let mut v10: u64 = *v10;
            let mut v11: u64 = *v11;
            let mut v12: bool = v6.clone() == v1.clone();
            if v12 {
                false
            } else {
                match &v8 {
                    US5::US5_Held => {
                        clashes_14(v7.clone(), v0.clone())
                    }
                    US5::US5_Waiting => {
                        let mut v14: bool = exclusive_17(v7.clone());
                        let mut v16: bool = if v14 {
                            true
                        } else {
                            quiet_18(v7.clone())
                        };
                        let mut v18: bool = if v16 {
                            true
                        } else {
                            holds_compiles_20(v7.clone())
                        };
                        let mut v20: bool = if v18 {
                            let mut v19: bool = v10 < v2;
                            v19
                        } else {
                            false
                        };
                        let mut v22: bool = if v20 {
                            clashes_14(v7.clone(), v0.clone())
                        } else {
                            false
                        };
                        if v22 {
                            let mut v23: bool = v3 == false;
                            v23
                        } else {
                            false
                        }
                    }
                }
            }
        }
        US4::US4_RowUnknown => {
            false
        }
    }
}
fn first_blocker_12(mut v0: US0, mut v1: Rc<str>, mut v2: u64, mut v3: bool, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let mut v5: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v5 {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v6.clone();
        } else {
            let mut v7: Rc<str> = first_row_7(v4.clone());
            let mut v8: bool = row_blocks_13(v0.clone(), v1.clone(), v2, v3, v7.clone());
            if v8 {
                return v7.clone();
            } else {
                let mut v9: Rc<str> = other_rows_11(v4.clone());
                (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v2, v3, v9.clone());
                continue;
            }
        }
    }
}
fn settle_ms_24(mut v0: US0) -> u64 {
    match &v0 {
        US0::US0_Bench => {
            90000u64
        }
        US0::US0_Compile => {
            20000u64
        }
        US0::US0_HeavyBuild => {
            90000u64
        }
        US0::US0_TestSuite => {
            90000u64
        }
        _ => {
            0u64
        }
    }
}
fn settling_mb_23(mut v0: Rc<str>, mut v1: u64) -> u64 {
    let mut v2: US4 = row_of_21(v0.clone());
    match &v2 {
        US4::US4_RowKnown(v3, v4, v5, v6, v7, v8) => {
            let mut v3: Rc<str> = v3.clone();
            let mut v4: US0 = v4.clone();
            let mut v5: US5 = v5.clone();
            let mut v6: Rc<str> = v6.clone();
            let mut v7: u64 = *v7;
            let mut v8: u64 = *v8;
            match &v5 {
                US5::US5_Held => {
                    let mut v9: u64 = settle_ms_24(v4.clone());
                    let mut v10: u64 = v7.wrapping_add(v9);
                    let mut v11: bool = v1 < v10;
                    if v11 {
                        needed_mb_3(v4.clone())
                    } else {
                        0u64
                    }
                }
                US5::US5_Waiting => {
                    0u64
                }
            }
        }
        US4::US4_RowUnknown => {
            0u64
        }
    }
}
fn settling_total_22(mut v0: Rc<str>, mut v1: u64, mut v2: u64) -> u64 {
    loop {
        let mut v3: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v3 {
            return v2;
        } else {
            let mut v4: Rc<str> = other_rows_11(v0.clone());
            let mut v5: Rc<str> = first_row_7(v0.clone());
            let mut v6: u64 = settling_mb_23(v5.clone(), v1);
            let mut v7: u64 = v2.wrapping_add(v6);
            (v0, v1, v2) = (v4.clone(), v1, v7);
            continue;
        }
    }
}
fn free_after_settling_25(mut v0: u64, mut v1: u64) -> u64 {
    let mut v2: bool = v0 > v1;
    if v2 {
        let mut v3: u64 = v0.wrapping_sub(v1);
        v3
    } else {
        0u64
    }
}
fn minutes_left_28(mut v0: u64, mut v1: u64) -> u64 {
    let mut v2: bool = v0 <= v1;
    if v2 {
        0u64
    } else {
        let mut v3: u64 = v0.wrapping_sub(v1);
        let mut v4: u64 = v3.wrapping_add(59999u64);
        let mut v5: u64 = v4.wrapping_div(60000u64);
        v5
    }
}
fn blocker_terms_27(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("by="); } LIT.with(|lit| lit.clone()) };
    let mut v3: i32 = 0i32;
    let mut v4: Rc<str> = tab_field_8(v0.clone(), v3);
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" holder="); } LIT.with(|lit| lit.clone()) };
    let mut v6: i32 = 1i32;
    let mut v7: Rc<str> = tab_field_8(v0.clone(), v6);
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" state="); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v4, &*v5, &*v7, &*v8].concat());
    let mut v10: i32 = 2i32;
    let mut v11: Rc<str> = tab_field_8(v0.clone(), v10);
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" left_min="); } LIT.with(|lit| lit.clone()) };
    let mut v13: i32 = 5i32;
    let mut v14: Rc<str> = tab_field_8(v0.clone(), v13);
    let mut v15: u64 = decimal_of_9(v14.clone());
    let mut v16: u64 = minutes_left_28(v15, v1);
    let mut v17: Rc<str> = number_text_2(v16);
    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" label='"); } LIT.with(|lit| lit.clone()) };
    let mut v19: i32 = 3i32;
    let mut v20: Rc<str> = tab_field_8(v0.clone(), v19);
    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v20, &*v21, &*v22, &*v22].concat());
    let mut v24: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v11, &*v12, &*v17, &*v23].concat());
    v24.clone()
}
fn admission_of_26(mut v0: US0, mut v1: Rc<str>, mut v2: u64, mut v3: u64) -> US1 {
    let mut v4: bool = v1.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v4 {
        let mut v5: Rc<str> = blocker_terms_27(v1.clone(), v3);
        US1::US1_HeldBack(v5.clone())
    } else {
        let mut v7: u64 = needed_mb_3(v0.clone());
        let mut v8: bool = v2 < v7;
        if v8 {
            US1::US1_LowMemory(v2)
        } else {
            US1::US1_Admitted
        }
    }
}
fn verdict_terms_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: u64, mut v4: Rc<str>, mut v5: u64, mut v6: u64) -> Rc<str> {
    let mut v7: US2 = class_of_4(v0.clone());
    match &v7 {
        US2::US2_ClassKnown(v9) => {
            let mut v9: US0 = v9.clone();
            let mut v10: bool = v3 > 0u64;
            let mut v16: bool = if v10 {
                let mut v11: u64 = v3.wrapping_mul(60000u64);
                let mut v12: u64 = v6.wrapping_add(v11);
                let mut v13: u64 = 0u64;
                let mut v14: u64 = held_until_latest_6(v4.clone(), v13);
                let mut v15: bool = v12 <= v14;
                v15
            } else {
                false
            };
            let mut v17: Rc<str> = first_blocker_12(v9.clone(), v1.clone(), v2, v16, v4.clone());
            let mut v18: u64 = 0u64;
            let mut v19: u64 = settling_total_22(v4.clone(), v6, v18);
            let mut v20: u64 = free_after_settling_25(v5, v19);
            let mut v21: US1 = admission_of_26(v9.clone(), v17.clone(), v20, v6);
            admission_terms_1(v9.clone(), v21.clone())
        }
        US2::US2_ClassUnknown => {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("reason=unknown-class"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, u64, u64, Rc<str>, u64, u64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, u64, u64, Rc<str>, u64, u64) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: u64, mut v4: Rc<str>, mut v5: u64, mut v6: u64| -> Rc<str> {
        verdict_terms_0(v0.clone(), v1.clone(), v2, v3, v4.clone(), v5, v6)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_window_verdict(v0: &str, v1: &str, v2: u64, v3: u64, v4: &str, v5: u64, v6: u64) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), v2, v3, Rc::<str>::from(v4), v5, v6)
}
