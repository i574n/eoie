#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_WelchTwoSided,
    US0_OneSampleT,
    US0_BinomialAtMost,
    US0_WilsonBand,
    US0_MedianBand,
    US0_BinomialBand,
}
#[derive(Clone)]
enum US2 {
    US2_WelchPlan(Rc<str>),
    US2_OneSamplePlan(f64),
}
#[derive(Clone)]
enum US3 {
    US3_BinomialPlan(f64),
    US3_WilsonPlan(f64),
    US3_BinomialBandPlan(f64),
}
#[derive(Clone)]
enum US4 {
    US4_MedianPlan(f64, f64),
}
#[derive(Clone)]
enum US1 {
    US1_TestedMean(US2),
    US1_TestedRate(US3),
    US1_TestedMedian(US4),
}
#[derive(Clone)]
enum US5 {
    US5_Some(Rc<str>),
    US5_None,
}
#[derive(Clone)]
enum US6 {
    US6_RowsKept(Rc<str>),
    US6_RowsRefused(Rc<str>),
}
#[derive(Clone)]
enum US7 {
    US7_OneArm,
    US7_AgainstArm(Rc<str>),
}
#[derive(Clone)]
enum US8 {
    US8_FullLook,
    US8_InterimLook(Rc<str>),
    US8_LookRefused(Rc<str>),
}
#[derive(Clone)]
enum US9 {
    US9_NoBand,
    US9_Band(f64, f64),
}
#[derive(Clone)]
enum US10 {
    US10_PlanRead(Rc<str>, Rc<str>, Rc<str>, US0, Rc<str>, i32, f64, Rc<str>, Rc<str>, Rc<str>, US1),
    US10_PlanBroken(Rc<str>),
}
#[derive(Clone)]
enum US11 {
    US11_TestKnown(US0),
    US11_TestUnknown,
}
#[derive(Clone)]
enum US12 {
    US12_TableLoaded(std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>),
    US12_TableUnreadable(Rc<str>),
}
fn stderr_line_2(mut v0: Rc<str>) -> () {
    eprintln!("{}", &*v0);
    ()
}
fn failed_1(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: experiment "); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v0, &*v2, &*v2, &*v2].concat());
    stderr_line_2(v3.clone());
    2i32
}
fn where_clauses_4() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from({ let args = std::env::args().collect::<Vec<_>>(); args.windows(2).filter(|pair| pair[0] == "--where").map(|pair| pair[1].clone()).collect::<Vec<_>>().join("
") }.as_str());
    v0.clone()
}
fn command_line_argument_count_6() -> i32 {
    let mut v0: i32 = (std::env::args().count() as i32) - 1;
    v0
}
fn command_line_argument_7(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::args().nth((v0 + 1) as usize).unwrap_or_default());
    v1.clone()
}
fn argument_5(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_sub(1i32);
    let mut v2: bool = v1 >= 0i32;
    let mut v5: bool = if v2 {
        let mut v3: i32 = command_line_argument_count_6();
        let mut v4: bool = v1 < v3;
        v4
    } else {
        false
    };
    let mut v9: US5 = if v5 {
        let mut v6: Rc<str> = command_line_argument_7(v1);
        US5::US5_Some(v6.clone())
    } else {
        US5::US5_None
    };
    match &v9 {
        US5::US5_None => {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US5::US5_Some(v10) => {
            let mut v10: Rc<str> = v10.clone();
            v10.clone()
        }
    }
}
fn optional_column_8(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>) -> i32 {
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v2 {
        -1i32
    } else {
        let mut v3: i32 = (v0).first().and_then(|header| header.iter().position(|item| &**item == &*v1)).map(|index| index as i32).unwrap_or(-1);
        v3
    }
}
fn in_arm_11(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: i32) -> bool {
    let mut v7: bool = (v1).as_bytes().get(v6 as usize) == Some(&49u8);
    if v7 {
        let mut v8: bool = v2 < 0i32;
        if v8 {
            true
        } else {
            let mut v9: Rc<str> = std::rc::Rc::<str>::from((v0).get(v6 as usize).and_then(|cells| cells.get(v2 as usize)).map(|cell| &**cell).unwrap_or(""));
            let mut v10: bool = v9.clone() == v5.clone();
            v10
        }
    } else {
        false
    }
}
fn text_before_12(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: bool = (v0) < (v1);
    v2
}
fn in_scope_10(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: i32) -> bool {
    let mut v7: bool = in_arm_11(v0.clone(), v1.clone(), v2, v3, v4.clone(), v5.clone(), v6);
    if v7 {
        let mut v8: bool = v3 < 0i32;
        if v8 {
            true
        } else {
            let mut v9: Rc<str> = std::rc::Rc::<str>::from((v0).get(v6 as usize).and_then(|cells| cells.get(v3 as usize)).map(|cell| &**cell).unwrap_or(""));
            let mut v10: bool = text_before_12(v9.clone(), v4.clone());
            let mut v11: bool = v10 == false;
            v11
        }
    } else {
        false
    }
}
fn number_parsed_13(mut v0: Rc<str>) -> f64 {
    let mut v1: f64 = (v0).trim().parse::<f64>().unwrap_or(f64::NAN);
    v1
}
fn success_cell_14(mut v0: Rc<str>) -> bool {
    let mut v1: bool = { let cell = (v0).trim(); !cell.is_empty() && !matches!(cell, "0" | "false" | "no" | "-") };
    v1
}
fn tallied_9(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: f64, mut v11: f64, mut v12: f64, mut v13: i32) -> (i32, f64, f64, f64, i32) {
    let mut v14: bool = v7 > v8;
    let (mut v49, mut v50, mut v51, mut v52, mut v53): (i32, f64, f64, f64, i32) = if v14 {
        (v9, v10, v11, v12, v13)
    } else {
        let mut v15: bool = in_scope_10(v0.clone(), v1.clone(), v2, v3, v4.clone(), v5.clone(), v7);
        let (mut v44, mut v45, mut v46, mut v47, mut v48): (i32, f64, f64, f64, i32) = if v15 {
            let mut v16: Rc<str> = std::rc::Rc::<str>::from((v0).get(v7 as usize).and_then(|cells| cells.get(v6 as usize)).map(|cell| &**cell).unwrap_or(""));
            let mut v17: f64 = number_parsed_13(v16.clone());
            let mut v18: bool = v17 == v17;
            let mut v19: i32 = v9.wrapping_add(1i32);
            let mut v20: bool = success_cell_14(v16.clone());
            let mut v21: i32 = if v20 {
                1i32
            } else {
                0i32
            };
            let mut v22: i32 = v13.wrapping_add(v21);
            let (mut v27, mut v28, mut v29, mut v30, mut v31): (i32, f64, f64, f64, i32) = if v18 {
                let mut v23: f64 = v10 + 1.0f64;
                let mut v24: f64 = v11 + v17;
                let mut v25: f64 = v17 * v17;
                let mut v26: f64 = v12 + v25;
                (v19, v23, v24, v26, v22)
            } else {
                (v19, v10, v11, v12, v22)
            };
            let mut v32: i32 = v7.wrapping_add(1i32);
            let (mut v33, mut v34, mut v35, mut v36, mut v37): (i32, f64, f64, f64, i32) = tallied_9(v0.clone(), v1.clone(), v2, v3, v4.clone(), v5.clone(), v6, v32, v8, v27, v28, v29, v30, v31);
            (v33, v34, v35, v36, v37)
        } else {
            let mut v38: i32 = v7.wrapping_add(1i32);
            let (mut v39, mut v40, mut v41, mut v42, mut v43): (i32, f64, f64, f64, i32) = tallied_9(v0.clone(), v1.clone(), v2, v3, v4.clone(), v5.clone(), v6, v38, v8, v9, v10, v11, v12, v13);
            (v39, v40, v41, v42, v43)
        };
        (v44, v45, v46, v47, v48)
    };
    (v49, v50, v51, v52, v53)
}
fn second_arm_of_15(mut v0: US1) -> US7 {
    match &v0 {
        US1::US1_TestedMean(v1) => {
            let mut v1: US2 = v1.clone();
            match &v1 {
                US2::US2_WelchPlan(v2) => {
                    let mut v2: Rc<str> = v2.clone();
                    US7::US7_AgainstArm(v2.clone())
                }
                _ => {
                    US7::US7_OneArm
                }
            }
        }
        _ => {
            US7::US7_OneArm
        }
    }
}
fn early_rows_16(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: i32, mut v7: i32, mut v8: i32) -> i32 {
    loop {
        let mut v9: bool = v6 > v7;
        let mut v11: bool = if v9 {
            true
        } else {
            let mut v10: bool = v3 < 0i32;
            v10
        };
        if v11 {
            return v8;
        } else {
            let mut v12: bool = in_arm_11(v0.clone(), v1.clone(), v2, v3, v4.clone(), v5.clone(), v6);
            let mut v15: bool = if v12 {
                let mut v13: bool = in_scope_10(v0.clone(), v1.clone(), v2, v3, v4.clone(), v5.clone(), v6);
                let mut v14: bool = v13 == false;
                v14
            } else {
                false
            };
            if v15 {
                let mut v16: i32 = v6.wrapping_add(1i32);
                let mut v17: i32 = v8.wrapping_add(1i32);
                (v0, v1, v2, v3, v4, v5, v6, v7, v8) = (v0.clone(), v1.clone(), v2, v3, v4.clone(), v5.clone(), v16, v7, v17);
                continue;
            } else {
                let mut v19: i32 = v6.wrapping_add(1i32);
                (v0, v1, v2, v3, v4, v5, v6, v7, v8) = (v0.clone(), v1.clone(), v2, v3, v4.clone(), v5.clone(), v19, v7, v8);
                continue;
            }
        }
    }
}
fn i32_text_18(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string().as_str());
    v1.clone()
}
fn numeric_note_19(mut v0: US0, mut v1: i32, mut v2: f64, mut v3: f64, mut v4: f64, mut v5: i32) -> Rc<str> {
    let mut v6: i32 = v2 as i32;
    let mut v9: bool = match &v0 {
        US0::US0_MedianBand => {
            true
        }
        US0::US0_OneSampleT => {
            true
        }
        US0::US0_WelchTwoSided => {
            true
        }
        _ => {
            false
        }
    };
    let mut v11: bool = if v9 {
        let mut v10: bool = v6 < v1;
        v10
    } else {
        false
    };
    if v11 {
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ("); } LIT.with(|lit| lit.clone()) };
        let mut v13: Rc<str> = i32_text_18(v6);
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" numeric)"); } LIT.with(|lit| lit.clone()) };
        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v16: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v13, &*v14, &*v15, &*v15].concat());
        v16.clone()
    } else {
        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v17.clone()
    }
}
fn arm_counts_17(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: US0, mut v4: Rc<str>, mut v5: i32, mut v6: f64, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: US1, mut v11: i32, mut v12: f64, mut v13: f64, mut v14: f64, mut v15: i32, mut v16: i32, mut v17: f64, mut v18: f64, mut v19: f64, mut v20: i32) -> Rc<str> {
    let mut v21: Rc<str> = i32_text_18(v5);
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = i32_text_18(v11);
    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v25: Rc<str> = numeric_note_19(v3.clone(), v11, v12, v13, v14, v15);
    let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v27: Rc<str> = std::rc::Rc::<str>::from([&*v21, &*v25, &*v26, &*v26, &*v26].concat());
    let mut v28: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v22, &*v23, &*v24, &*v27].concat());
    let mut v29: US7 = second_arm_of_15(v10.clone());
    match &v29 {
        US7::US7_AgainstArm(v30) => {
            let mut v30: Rc<str> = v30.clone();
            let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
            let mut v32: Rc<str> = i32_text_18(v16);
            let mut v33: Rc<str> = numeric_note_19(v3.clone(), v16, v17, v18, v19, v20);
            let mut v34: Rc<str> = std::rc::Rc::<str>::from([&*v32, &*v24, &*v21, &*v33, &*v26].concat());
            let mut v35: Rc<str> = std::rc::Rc::<str>::from([&*v28, &*v31, &*v30, &*v22, &*v34].concat());
            v35.clone()
        }
        US7::US7_OneArm => {
            v28.clone()
        }
    }
}
fn joined_20(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2.clone()
}
fn file_text_21(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1.clone()
}
fn field_of_22(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from((v0).lines().rev().find_map(|line| line.split_once(char::from(61u8)).filter(|(name, _)| *name == &*v1).map(|(_, value)| value)).unwrap_or(""));
    v2.clone()
}
fn option_value_23(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip_while(|arg| arg != &*v0).nth(1).unwrap_or_default());
    v1.clone()
}
fn look_of_24(mut v0: Rc<str>, mut v1: bool, mut v2: Rc<str>, mut v3: Rc<str>) -> US8 {
    let mut v4: bool = v3.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v4 {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the verdict was already recorded at "); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; an experiment is analyzed once (experiment show prints it)"); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v3, &*v6, &*v7, &*v7].concat());
        US8::US8_LookRefused(v8.clone())
    } else {
        let mut v10: bool = v1 == false;
        if v10 {
            US8::US8_FullLook
        } else {
            let mut v12: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v12 {
                let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("interim look refused: "); } LIT.with(|lit| lit.clone()) };
                let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rows of the planned count; pass --interim '<reason>' to look anyway and record a disclosure"); } LIT.with(|lit| lit.clone()) };
                let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v16: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v0, &*v14, &*v15, &*v15].concat());
                US8::US8_LookRefused(v16.clone())
            } else {
                US8::US8_InterimLook(v2.clone())
            }
        }
    }
}
fn stdout_line_25(mut v0: Rc<str>) -> () {
    println!("{}", &*v0);
    ()
}
fn now_ms_26() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn median_terms_between_28(mut v0: Rc<str>, mut v1: f64, mut v2: f64) -> Rc<str> {
    let mut v3: Rc<str> = eoie_experiment_stats::eoie_median_terms(&*v0, v1, v2);
    v3.clone()
}
fn band_of_29(mut v0: US1) -> US9 {
    match &v0 {
        US1::US1_TestedMedian(v1) => {
            let mut v1: US4 = v1.clone();
            match &v1 {
                US4::US4_MedianPlan(v2, v3) => {
                    let mut v2: f64 = *v2;
                    let mut v3: f64 = *v3;
                    US9::US9_Band(v2, v3)
                }
            }
        }
        _ => {
            US9::US9_NoBand
        }
    }
}
fn values_of_30(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: Rc<str>) -> Rc<str> {
    loop {
        let mut v10: bool = v7 > v8;
        if v10 {
            return v9.clone();
        } else {
            let mut v11: bool = in_scope_10(v0.clone(), v1.clone(), v2, v3, v4.clone(), v5.clone(), v7);
            if v11 {
                let mut v12: Rc<str> = std::rc::Rc::<str>::from((v0).get(v7 as usize).and_then(|cells| cells.get(v6 as usize)).map(|cell| &**cell).unwrap_or(""));
                let mut v13: f64 = number_parsed_13(v12.clone());
                let mut v14: f64 = number_parsed_13(v12.clone());
                let mut v15: bool = v13 == v14;
                let mut v21: Rc<str> = if v15 {
                    let mut v16: bool = v9.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    if v16 {
                        v12.clone()
                    } else {
                        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v19: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v17, &*v12, &*v18, &*v18].concat());
                        v19.clone()
                    }
                } else {
                    v9.clone()
                };
                let mut v22: i32 = v7.wrapping_add(1i32);
                (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9) = (v0.clone(), v1.clone(), v2, v3, v4.clone(), v5.clone(), v6, v22, v8, v21.clone());
                continue;
            } else {
                let mut v24: i32 = v7.wrapping_add(1i32);
                (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9) = (v0.clone(), v1.clone(), v2, v3, v4.clone(), v5.clone(), v6, v24, v8, v9.clone());
                continue;
            }
        }
    }
}
fn prediction_of_31(mut v0: US1) -> f64 {
    match &v0 {
        US1::US1_TestedMean(v1) => {
            let mut v1: US2 = v1.clone();
            match &v1 {
                US2::US2_OneSamplePlan(v2) => {
                    let mut v2: f64 = *v2;
                    v2
                }
                _ => {
                    0.0f64
                }
            }
        }
        US1::US1_TestedRate(v4) => {
            let mut v4: US3 = v4.clone();
            match &v4 {
                US3::US3_BinomialBandPlan(v7) => {
                    let mut v7: f64 = *v7;
                    v7
                }
                US3::US3_BinomialPlan(v5) => {
                    let mut v5: f64 = *v5;
                    v5
                }
                US3::US3_WilsonPlan(v6) => {
                    let mut v6: f64 = *v6;
                    v6
                }
            }
        }
        _ => {
            0.0f64
        }
    }
}
fn terms_of_27(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: Rc<str>, mut v5: i32, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: US0, mut v10: Rc<str>, mut v11: i32, mut v12: f64, mut v13: Rc<str>, mut v14: Rc<str>, mut v15: Rc<str>, mut v16: US1, mut v17: i32, mut v18: f64, mut v19: f64, mut v20: f64, mut v21: i32, mut v22: i32, mut v23: f64, mut v24: f64, mut v25: f64, mut v26: i32) -> Rc<str> {
    let mut v27: US9 = band_of_29(v16.clone());
    match &v27 {
        US9::US9_Band(v28, v29) => {
            let mut v28: f64 = *v28;
            let mut v29: f64 = *v29;
            let mut v30: i32 = 1i32;
            let mut v31: i32 = ((v0).len().max(1) - 1) as i32;
            let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v33: Rc<str> = values_of_30(v0.clone(), v1.clone(), v2, v3, v4.clone(), v8.clone(), v5, v30, v31, v32.clone());
            median_terms_between_28(v33.clone(), v28, v29)
        }
        US9::US9_NoBand => {
            let mut v46: Rc<str> = match &v9 {
                US0::US0_BinomialAtMost => {
                    let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
                    v37.clone()
                }
                US0::US0_BinomialBand => {
                    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                    v40.clone()
                }
                US0::US0_MedianBand => {
                    let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                    v39.clone()
                }
                US0::US0_OneSampleT => {
                    let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
                    v36.clone()
                }
                US0::US0_WelchTwoSided => {
                    let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
                    v35.clone()
                }
                US0::US0_WilsonBand => {
                    let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                    v38.clone()
                }
            };
            let mut v47: f64 = prediction_of_31(v16.clone());
            let mut v48: Rc<str> = eoie_experiment_stats::eoie_experiment_terms(&*v46, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v47, v12);
            v48.clone()
        }
    }
}
fn field_line_32(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(format!("{}={}
", &*v0, (v1).chars().map(|c| if c.is_control() { char::from(32u8) } else { c }).collect::<String>()).as_str());
    v2.clone()
}
fn text_appended_33(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::fs::OpenOptions::new().create(true).append(true).open(&*v0).and_then(|mut file| { use std::io::Write; file.write_all((v1).as_bytes()) }).err().map(|error| format!("append {}: {error}", &*v0)).unwrap_or_default().as_str());
    v2.clone()
}
fn text_created_34(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from({ let path = std::path::Path::new(&*v0); let created = path.parent().map(std::fs::create_dir_all).unwrap_or(Ok(())).and_then(|_| std::fs::OpenOptions::new().write(true).create_new(true).open(path)).and_then(|mut file| { use std::io::Write; file.write_all((v1).as_bytes()) }); created.err().map(|error| format!("write {}: {error}", path.display())).unwrap_or_default() }.as_str());
    v2.clone()
}
fn analyzed_plan_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: US0, mut v5: Rc<str>, mut v6: i32, mut v7: f64, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: US1, mut v12: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>) -> i32 {
    let mut v13: Rc<str> = where_clauses_4();
    let mut v14: i32 = 5i32;
    let mut v15: Rc<str> = argument_5(v14);
    let (mut v16, mut v17): (bool, Rc<str>) = eoie_table_query::eoie_table_rows_kept(&*v15, &*v13);
    let mut v20: US6 = if v16 {
        US6::US6_RowsKept(v17.clone())
    } else {
        US6::US6_RowsRefused(v17.clone())
    };
    let (mut v27, mut v28): (Rc<str>, Rc<str>) = match &v20 {
        US6::US6_RowsKept(v21) => {
            let mut v21: Rc<str> = v21.clone();
            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (v21.clone(), v22.clone())
        }
        US6::US6_RowsRefused(v23) => {
            let mut v23: Rc<str> = v23.clone();
            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (v24.clone(), v23.clone())
        }
    };
    let mut v29: i32 = optional_column_8(v12.clone(), v5.clone());
    let mut v30: i32 = optional_column_8(v12.clone(), v9.clone());
    let mut v31: i32 = (v12).first().and_then(|header| header.iter().position(|item| &**item == &*v2)).map(|index| index as i32).unwrap_or(-1);
    let mut v32: bool = v31 < 0i32;
    let mut v34: bool = if v32 {
        true
    } else {
        let mut v33: bool = v28.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v33
    };
    let mut v38: bool = if v34 {
        true
    } else {
        let mut v35: bool = v5.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v35 {
            let mut v36: bool = v29 < 0i32;
            v36
        } else {
            false
        }
    };
    let mut v42: bool = if v38 {
        true
    } else {
        let mut v39: bool = v9.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v39 {
            let mut v40: bool = v30 < 0i32;
            v40
        } else {
            false
        }
    };
    if v42 {
        let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("analyze: unknown columns in "); } LIT.with(|lit| lit.clone()) };
        let mut v44: i32 = 5i32;
        let mut v45: Rc<str> = argument_5(v44);
        let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (header: "); } LIT.with(|lit| lit.clone()) };
        let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let mut v48: Rc<str> = std::rc::Rc::<str>::from((v12).first().map(|header| header.iter().map(|item| &**item).collect::<Vec<_>>().join(&*v47)).unwrap_or_default().as_str());
        let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
        let mut v50: Rc<str> = std::rc::Rc::<str>::from([&*v43, &*v45, &*v46, &*v48, &*v49].concat());
        failed_1(v50.clone())
    } else {
        let mut v52: i32 = 1i32;
        let mut v53: i32 = ((v12).len().max(1) - 1) as i32;
        let mut v54: i32 = 0i32;
        let mut v55: f64 = 0.0f64;
        let mut v56: f64 = 0.0f64;
        let mut v57: f64 = 0.0f64;
        let mut v58: i32 = 0i32;
        let (mut v59, mut v60, mut v61, mut v62, mut v63): (i32, f64, f64, f64, i32) = tallied_9(v12.clone(), v27.clone(), v29, v30, v10.clone(), v3.clone(), v31, v52, v53, v54, v55, v56, v57, v58);
        let mut v64: US7 = second_arm_of_15(v11.clone());
        let (mut v88, mut v89, mut v90, mut v91, mut v92, mut v93): (i32, f64, f64, f64, i32, i32) = match &v64 {
            US7::US7_AgainstArm(v65) => {
                let mut v65: Rc<str> = v65.clone();
                let mut v66: i32 = 1i32;
                let mut v67: i32 = ((v12).len().max(1) - 1) as i32;
                let mut v68: i32 = 0i32;
                let mut v69: f64 = 0.0f64;
                let mut v70: f64 = 0.0f64;
                let mut v71: f64 = 0.0f64;
                let mut v72: i32 = 0i32;
                let (mut v73, mut v74, mut v75, mut v76, mut v77): (i32, f64, f64, f64, i32) = tallied_9(v12.clone(), v27.clone(), v29, v30, v10.clone(), v65.clone(), v31, v66, v67, v68, v69, v70, v71, v72);
                let mut v78: i32 = 1i32;
                let mut v79: i32 = ((v12).len().max(1) - 1) as i32;
                let mut v80: i32 = 0i32;
                let mut v81: i32 = early_rows_16(v12.clone(), v27.clone(), v29, v30, v10.clone(), v65.clone(), v78, v79, v80);
                (v73, v74, v75, v76, v77, v81)
            }
            US7::US7_OneArm => {
                (v59, v60, v61, v62, v63, 0i32)
            }
        };
        let mut v94: i32 = 1i32;
        let mut v95: i32 = ((v12).len().max(1) - 1) as i32;
        let mut v96: i32 = 0i32;
        let mut v97: i32 = early_rows_16(v12.clone(), v27.clone(), v29, v30, v10.clone(), v3.clone(), v94, v95, v96);
        let mut v98: i32 = v97.wrapping_add(v93);
        let mut v99: Rc<str> = arm_counts_17(v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6, v7, v8.clone(), v9.clone(), v10.clone(), v11.clone(), v59, v60, v61, v62, v63, v88, v89, v90, v91, v92);
        let mut v100: bool = v59 < v6;
        let mut v102: bool = if v100 {
            true
        } else {
            let mut v101: bool = v88 < v6;
            v101
        };
        let mut v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("verdict"); } LIT.with(|lit| lit.clone()) };
        let mut v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
        let mut v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v106: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v104, &*v103, &*v105, &*v105].concat());
        let mut v107: Rc<str> = joined_20(v0.clone(), v106.clone());
        let mut v108: Rc<str> = file_text_21(v107.clone());
        let mut v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("analyzed"); } LIT.with(|lit| lit.clone()) };
        let mut v110: Rc<str> = field_of_22(v108.clone(), v109.clone());
        let mut v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--interim"); } LIT.with(|lit| lit.clone()) };
        let mut v112: Rc<str> = option_value_23(v111.clone());
        let mut v113: US8 = look_of_24(v99.clone(), v102, v112.clone(), v110.clone());
        match &v113 {
            US8::US8_LookRefused(v114) => {
                let mut v114: Rc<str> = v114.clone();
                let mut v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie experiment analyze refused id="); } LIT.with(|lit| lit.clone()) };
                let mut v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let mut v117: Rc<str> = std::rc::Rc::<str>::from([&*v115, &*v1, &*v116, &*v114, &*v105].concat());
                stdout_line_25(v117.clone());
                1i32
            }
            _ => {
                let mut v118: u64 = now_ms_26();
                let mut v119: Rc<str> = agile_task_notes::eoie_utc_stamp(v118);
                let mut v120: bool = v30 < 0i32;
                let mut v124: Rc<str> = if v120 {
                    v105.clone()
                } else {
                    let mut v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" before_registration="); } LIT.with(|lit| lit.clone()) };
                    let mut v122: Rc<str> = i32_text_18(v98);
                    let mut v123: Rc<str> = std::rc::Rc::<str>::from([&*v121, &*v122, &*v105, &*v105, &*v105].concat());
                    v123.clone()
                };
                let mut v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("id="); } LIT.with(|lit| lit.clone()) };
                let mut v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" test="); } LIT.with(|lit| lit.clone()) };
                let mut v138: Rc<str> = match &v4 {
                    US0::US0_BinomialAtMost => {
                        let mut v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
                        v129.clone()
                    }
                    US0::US0_BinomialBand => {
                        let mut v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                        v132.clone()
                    }
                    US0::US0_MedianBand => {
                        let mut v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                        v131.clone()
                    }
                    US0::US0_OneSampleT => {
                        let mut v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
                        v128.clone()
                    }
                    US0::US0_WelchTwoSided => {
                        let mut v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
                        v127.clone()
                    }
                    US0::US0_WilsonBand => {
                        let mut v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                        v130.clone()
                    }
                };
                let mut v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" n="); } LIT.with(|lit| lit.clone()) };
                let mut v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let mut v141: Rc<str> = terms_of_27(v12.clone(), v27.clone(), v29, v30, v10.clone(), v31, v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6, v7, v8.clone(), v9.clone(), v10.clone(), v11.clone(), v59, v60, v61, v62, v63, v88, v89, v90, v91, v92);
                let mut v142: Rc<str> = std::rc::Rc::<str>::from([&*v139, &*v99, &*v140, &*v141, &*v124].concat());
                let mut v143: Rc<str> = std::rc::Rc::<str>::from([&*v125, &*v1, &*v126, &*v138, &*v142].concat());
                let mut v165: Rc<str> = match &v113 {
                    US8::US8_InterimLook(v144) => {
                        let mut v144: Rc<str> = v144.clone();
                        let mut v145: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("disclosures"); } LIT.with(|lit| lit.clone()) };
                        let mut v146: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v104, &*v145, &*v105, &*v105].concat());
                        let mut v147: Rc<str> = joined_20(v0.clone(), v146.clone());
                        let mut v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("disclosed"); } LIT.with(|lit| lit.clone()) };
                        let mut v149: Rc<str> = field_line_32(v148.clone(), v119.clone());
                        let mut v150: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("reason"); } LIT.with(|lit| lit.clone()) };
                        let mut v151: Rc<str> = field_line_32(v150.clone(), v144.clone());
                        let mut v152: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("result"); } LIT.with(|lit| lit.clone()) };
                        let mut v153: Rc<str> = field_line_32(v152.clone(), v143.clone());
                        let mut v154: Rc<str> = std::rc::Rc::<str>::from([&*v149, &*v151, &*v153, &*v105, &*v105].concat());
                        text_appended_33(v147.clone(), v154.clone())
                    }
                    _ => {
                        let mut v156: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v104, &*v103, &*v105, &*v105].concat());
                        let mut v157: Rc<str> = joined_20(v0.clone(), v156.clone());
                        let mut v158: Rc<str> = field_line_32(v109.clone(), v119.clone());
                        let mut v159: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("result"); } LIT.with(|lit| lit.clone()) };
                        let mut v160: Rc<str> = field_line_32(v159.clone(), v143.clone());
                        let mut v161: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("decision"); } LIT.with(|lit| lit.clone()) };
                        let mut v162: Rc<str> = field_line_32(v161.clone(), v8.clone());
                        let mut v163: Rc<str> = std::rc::Rc::<str>::from([&*v158, &*v160, &*v162, &*v105, &*v105].concat());
                        text_created_34(v157.clone(), v163.clone())
                    }
                };
                let mut v166: bool = v165.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                if v166 {
                    let mut v167: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("analyze: "); } LIT.with(|lit| lit.clone()) };
                    let mut v168: Rc<str> = std::rc::Rc::<str>::from([&*v167, &*v165, &*v105, &*v105, &*v105].concat());
                    failed_1(v168.clone())
                } else {
                    let mut v170: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie experiment analyze ok "); } LIT.with(|lit| lit.clone()) };
                    let mut v174: Rc<str> = match &v113 {
                        US8::US8_InterimLook(v171) => {
                            let mut v171: Rc<str> = v171.clone();
                            let mut v172: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" look=interim disclosed=yes"); } LIT.with(|lit| lit.clone()) };
                            v172.clone()
                        }
                        _ => {
                            let mut v173: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" look=full verdict=recorded"); } LIT.with(|lit| lit.clone()) };
                            v173.clone()
                        }
                    };
                    let mut v175: Rc<str> = std::rc::Rc::<str>::from([&*v170, &*v143, &*v174, &*v105, &*v105].concat());
                    stdout_line_25(v175.clone());
                    0i32
                }
            }
        }
    }
}
fn plan_text_35(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("registration"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v3, &*v2, &*v4, &*v4].concat());
    let mut v6: Rc<str> = joined_20(v0.clone(), v5.clone());
    let mut v7: Rc<str> = file_text_21(v6.clone());
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("amendments"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v3, &*v8, &*v4, &*v4].concat());
    let mut v10: Rc<str> = joined_20(v0.clone(), v9.clone());
    let mut v11: Rc<str> = file_text_21(v10.clone());
    let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v11, &*v4, &*v4, &*v4].concat());
    v12.clone()
}
fn test_of_37(mut v0: Rc<str>) -> US11 {
    let mut v1: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
    if v1 {
        let mut v2: US0 = US0::US0_WelchTwoSided;
        US11::US11_TestKnown(v2.clone())
    } else {
        let mut v4: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
        if v4 {
            let mut v5: US0 = US0::US0_OneSampleT;
            US11::US11_TestKnown(v5.clone())
        } else {
            let mut v7: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
            if v7 {
                let mut v8: US0 = US0::US0_BinomialAtMost;
                US11::US11_TestKnown(v8.clone())
            } else {
                let mut v10: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                if v10 {
                    let mut v11: US0 = US0::US0_WilsonBand;
                    US11::US11_TestKnown(v11.clone())
                } else {
                    let mut v13: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                    if v13 {
                        let mut v14: US0 = US0::US0_MedianBand;
                        US11::US11_TestKnown(v14.clone())
                    } else {
                        let mut v16: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                        if v16 {
                            let mut v17: US0 = US0::US0_BinomialBand;
                            US11::US11_TestKnown(v17.clone())
                        } else {
                            US11::US11_TestUnknown
                        }
                    }
                }
            }
        }
    }
}
fn whole_of_38(mut v0: Rc<str>) -> i32 {
    let mut v1: i32 = (v0).trim().parse::<i32>().unwrap_or(-1);
    v1
}
fn number_of_39(mut v0: Rc<str>) -> f64 {
    let mut v1: f64 = (v0).trim().parse::<f64>().unwrap_or(f64::NAN);
    v1
}
fn first_item_40(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(44u8)).next().unwrap_or("").trim());
    v1.clone()
}
fn second_item_42(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(44u8)).nth(1).unwrap_or("").trim());
    v1.clone()
}
fn test_plan_of_41(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>) -> US1 {
    match &v0 {
        US0::US0_BinomialAtMost => {
            let mut v9: f64 = number_of_39(v2.clone());
            let mut v10: US3 = US3::US3_BinomialPlan(v9);
            US1::US1_TestedRate(v10.clone())
        }
        US0::US0_BinomialBand => {
            let mut v15: f64 = number_of_39(v2.clone());
            let mut v16: US3 = US3::US3_BinomialBandPlan(v15);
            US1::US1_TestedRate(v16.clone())
        }
        US0::US0_MedianBand => {
            let mut v18: Rc<str> = first_item_40(v2.clone());
            let mut v19: f64 = number_of_39(v18.clone());
            let mut v20: Rc<str> = second_item_42(v2.clone());
            let mut v21: f64 = number_of_39(v20.clone());
            let mut v22: US4 = US4::US4_MedianPlan(v19, v21);
            US1::US1_TestedMedian(v22.clone())
        }
        US0::US0_OneSampleT => {
            let mut v6: f64 = number_of_39(v2.clone());
            let mut v7: US2 = US2::US2_OneSamplePlan(v6);
            US1::US1_TestedMean(v7.clone())
        }
        US0::US0_WelchTwoSided => {
            let mut v3: Rc<str> = second_item_42(v1.clone());
            let mut v4: US2 = US2::US2_WelchPlan(v3.clone());
            US1::US1_TestedMean(v4.clone())
        }
        US0::US0_WilsonBand => {
            let mut v12: f64 = number_of_39(v2.clone());
            let mut v13: US3 = US3::US3_WilsonPlan(v12);
            US1::US1_TestedRate(v13.clone())
        }
    }
}
fn plan_trusted_36(mut v0: Rc<str>) -> US10 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = field_of_22(v0.clone(), v1.clone());
    let mut v3: US11 = test_of_37(v2.clone());
    match &v3 {
        US11::US11_TestKnown(v6) => {
            let mut v6: US0 = v6.clone();
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("runs"); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = field_of_22(v0.clone(), v7.clone());
            let mut v9: i32 = whole_of_38(v8.clone());
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("confidence"); } LIT.with(|lit| lit.clone()) };
            let mut v11: Rc<str> = field_of_22(v0.clone(), v10.clone());
            let mut v12: f64 = number_of_39(v11.clone());
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("id"); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<str> = field_of_22(v0.clone(), v13.clone());
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("metric"); } LIT.with(|lit| lit.clone()) };
            let mut v16: Rc<str> = field_of_22(v0.clone(), v15.clone());
            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("arms"); } LIT.with(|lit| lit.clone()) };
            let mut v18: Rc<str> = field_of_22(v0.clone(), v17.clone());
            let mut v19: bool = v18.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v23: Rc<str> = if v19 {
                let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
                v20.clone()
            } else {
                let mut v21: Rc<str> = field_of_22(v0.clone(), v17.clone());
                first_item_40(v21.clone())
            };
            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("group"); } LIT.with(|lit| lit.clone()) };
            let mut v25: Rc<str> = field_of_22(v0.clone(), v24.clone());
            let mut v26: bool = v12 == v12;
            let mut v27: f64 = if v26 {
                v12
            } else {
                0.95f64
            };
            let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("decision"); } LIT.with(|lit| lit.clone()) };
            let mut v29: Rc<str> = field_of_22(v0.clone(), v28.clone());
            let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("time"); } LIT.with(|lit| lit.clone()) };
            let mut v31: Rc<str> = field_of_22(v0.clone(), v30.clone());
            let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("registered_local"); } LIT.with(|lit| lit.clone()) };
            let mut v33: Rc<str> = field_of_22(v0.clone(), v32.clone());
            let mut v34: Rc<str> = field_of_22(v0.clone(), v17.clone());
            let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("prediction"); } LIT.with(|lit| lit.clone()) };
            let mut v36: Rc<str> = field_of_22(v0.clone(), v35.clone());
            let mut v37: US1 = test_plan_of_41(v6.clone(), v34.clone(), v36.clone());
            US10::US10_PlanRead(v14.clone(), v16.clone(), v23.clone(), v6.clone(), v25.clone(), v9, v27, v29.clone(), v31.clone(), v33.clone(), v37.clone())
        }
        US11::US11_TestUnknown => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the registration names an unknown test"); } LIT.with(|lit| lit.clone()) };
            US10::US10_PlanBroken(v4.clone())
        }
    }
}
fn analyzed_0() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: Rc<str> = argument_5(v0);
    let mut v2: i32 = 4i32;
    let mut v3: Rc<str> = argument_5(v2);
    let mut v4: Rc<str> = plan_text_35(v1.clone(), v3.clone());
    let mut v5: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v5 {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("analyze: no registration for "); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" in "); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v3, &*v7, &*v1, &*v8].concat());
        failed_1(v9.clone())
    } else {
        let mut v11: US10 = plan_trusted_36(v4.clone());
        match &v11 {
            US10::US10_PlanBroken(v12) => {
                let mut v12: Rc<str> = v12.clone();
                let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("analyze: the registration of "); } LIT.with(|lit| lit.clone()) };
                let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is unusable: "); } LIT.with(|lit| lit.clone()) };
                let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v16: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v3, &*v14, &*v12, &*v15].concat());
                failed_1(v16.clone())
            }
            US10::US10_PlanRead(v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28) => {
                let mut v18: Rc<str> = v18.clone();
                let mut v19: Rc<str> = v19.clone();
                let mut v20: Rc<str> = v20.clone();
                let mut v21: US0 = v21.clone();
                let mut v22: Rc<str> = v22.clone();
                let mut v23: i32 = *v23;
                let mut v24: f64 = *v24;
                let mut v25: Rc<str> = v25.clone();
                let mut v26: Rc<str> = v26.clone();
                let mut v27: Rc<str> = v27.clone();
                let mut v28: US1 = v28.clone();
                let mut v29: i32 = 5i32;
                let mut v30: Rc<str> = argument_5(v29);
                let (mut v31, mut v32, mut v33): (bool, std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, Rc<str>) = { let path = &*v30; match std::fs::read_to_string(path) { Ok(text) => { let separator = if path.ends_with(".csv") { char::from(44u8) } else { char::from(9u8) }; (true, std::rc::Rc::new(text.lines().map(|line| line.split(separator).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>()), { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) } Err(error) => (false, std::rc::Rc::new(Vec::new()), std::rc::Rc::<str>::from(format!("{path}: {error}").as_str())) } };
                let mut v36: US12 = if v31 {
                    US12::US12_TableLoaded(v32.clone())
                } else {
                    US12::US12_TableUnreadable(v33.clone())
                };
                match &v36 {
                    US12::US12_TableLoaded(v42) => {
                        let mut v42: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = v42.clone();
                        analyzed_plan_3(v1.clone(), v18.clone(), v19.clone(), v20.clone(), v21.clone(), v22.clone(), v23, v24, v25.clone(), v26.clone(), v27.clone(), v28.clone(), v42.clone())
                    }
                    US12::US12_TableUnreadable(v37) => {
                        let mut v37: Rc<str> = v37.clone();
                        let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("analyze: "); } LIT.with(|lit| lit.clone()) };
                        let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v40: Rc<str> = std::rc::Rc::<str>::from([&*v38, &*v37, &*v39, &*v39, &*v39].concat());
                        failed_1(v40.clone())
                    }
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        analyzed_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_experiment_analyze_run() -> i32 {
    closure0()()
}
