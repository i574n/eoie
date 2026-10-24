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
enum US1 {
    US1_Some(Rc<str>),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_RowsKept(Rc<str>),
    US2_RowsRefused(Rc<str>),
}
#[derive(Clone)]
enum US3 {
    US3_TestKnown(US0),
    US3_TestUnknown,
}
#[derive(Clone)]
enum US4 {
    US4_PlanAccepted,
    US4_PlanRejected(Rc<str>),
}
#[derive(Clone)]
enum US5 {
    US5_TableLoaded(std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>),
    US5_TableUnreadable(Rc<str>),
}
fn stderr_line_3(mut v0: Rc<str>) -> () {
    eprintln!("{}", &*v0);
    ()
}
fn refused_2(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: table test "); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v0));
    stderr_line_3(v2.clone());
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
    let mut v9: US1 = if v5 {
        let mut v6: Rc<str> = command_line_argument_7(v1);
        US1::US1_Some(v6.clone())
    } else {
        US1::US1_None
    };
    match &v9 {
        US1::US1_None => {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US1::US1_Some(v10) => {
            let mut v10: Rc<str> = v10.clone();
            v10.clone()
        }
    }
}
fn option_value_8(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip_while(|arg| arg != &*v0).nth(1).unwrap_or_default());
    v1.clone()
}
fn optional_column_9(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>) -> i32 {
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v2 {
        -1i32
    } else {
        let mut v3: i32 = (v0).first().and_then(|header| header.iter().position(|item| &**item == &*v1)).map(|index| index as i32).unwrap_or(-1);
        v3
    }
}
fn arms_needed_10(mut v0: US0) -> i32 {
    match &v0 {
        US0::US0_WelchTwoSided => {
            2i32
        }
        _ => {
            1i32
        }
    }
}
fn in_arm_13(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: i32) -> bool {
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
fn text_before_14(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: bool = (v0) < (v1);
    v2
}
fn in_scope_12(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: i32) -> bool {
    let mut v7: bool = in_arm_13(v0.clone(), v1.clone(), v2, v3, v4.clone(), v5.clone(), v6);
    if v7 {
        let mut v8: bool = v3 < 0i32;
        if v8 {
            true
        } else {
            let mut v9: Rc<str> = std::rc::Rc::<str>::from((v0).get(v6 as usize).and_then(|cells| cells.get(v3 as usize)).map(|cell| &**cell).unwrap_or(""));
            let mut v10: bool = text_before_14(v9.clone(), v4.clone());
            let mut v11: bool = v10 == false;
            v11
        }
    } else {
        false
    }
}
fn number_parsed_15(mut v0: Rc<str>) -> f64 {
    let mut v1: f64 = (v0).trim().parse::<f64>().unwrap_or(f64::NAN);
    v1
}
fn success_cell_16(mut v0: Rc<str>) -> bool {
    let mut v1: bool = { let cell = (v0).trim(); !cell.is_empty() && !matches!(cell, "0" | "false" | "no" | "-") };
    v1
}
fn tallied_11(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: f64, mut v11: f64, mut v12: f64, mut v13: i32) -> (i32, f64, f64, f64, i32) {
    let mut v14: bool = v7 > v8;
    let (mut v49, mut v50, mut v51, mut v52, mut v53): (i32, f64, f64, f64, i32) = if v14 {
        (v9, v10, v11, v12, v13)
    } else {
        let mut v15: bool = in_scope_12(v0.clone(), v1.clone(), v2, v3, v4.clone(), v5.clone(), v7);
        let (mut v44, mut v45, mut v46, mut v47, mut v48): (i32, f64, f64, f64, i32) = if v15 {
            let mut v16: Rc<str> = std::rc::Rc::<str>::from((v0).get(v7 as usize).and_then(|cells| cells.get(v6 as usize)).map(|cell| &**cell).unwrap_or(""));
            let mut v17: f64 = number_parsed_15(v16.clone());
            let mut v18: bool = v17 == v17;
            let mut v19: i32 = v9.wrapping_add(1i32);
            let mut v20: bool = success_cell_16(v16.clone());
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
            let (mut v33, mut v34, mut v35, mut v36, mut v37): (i32, f64, f64, f64, i32) = tallied_11(v0.clone(), v1.clone(), v2, v3, v4.clone(), v5.clone(), v6, v32, v8, v27, v28, v29, v30, v31);
            (v33, v34, v35, v36, v37)
        } else {
            let mut v38: i32 = v7.wrapping_add(1i32);
            let (mut v39, mut v40, mut v41, mut v42, mut v43): (i32, f64, f64, f64, i32) = tallied_11(v0.clone(), v1.clone(), v2, v3, v4.clone(), v5.clone(), v6, v38, v8, v9, v10, v11, v12, v13);
            (v39, v40, v41, v42, v43)
        };
        (v44, v45, v46, v47, v48)
    };
    (v49, v50, v51, v52, v53)
}
fn confidence_given_17() -> f64 {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--confidence"); } LIT.with(|lit| lit.clone()) };
    let mut v1: Rc<str> = option_value_8(v0.clone());
    let mut v2: f64 = number_parsed_15(v1.clone());
    let mut v3: bool = v2 == v2;
    if v3 {
        v2
    } else {
        0.95f64
    }
}
fn values_of_18(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: Rc<str>) -> Rc<str> {
    loop {
        let mut v10: bool = v7 > v8;
        if v10 {
            return v9.clone();
        } else {
            let mut v11: bool = in_scope_12(v0.clone(), v1.clone(), v2, v3, v4.clone(), v5.clone(), v7);
            if v11 {
                let mut v12: Rc<str> = std::rc::Rc::<str>::from((v0).get(v7 as usize).and_then(|cells| cells.get(v6 as usize)).map(|cell| &**cell).unwrap_or(""));
                let mut v13: f64 = number_parsed_15(v12.clone());
                let mut v14: f64 = number_parsed_15(v12.clone());
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
fn median_terms_between_20(mut v0: Rc<str>, mut v1: f64, mut v2: f64) -> Rc<str> {
    let mut v3: Rc<str> = eoie_experiment_stats::eoie_median_terms(&*v0, v1, v2);
    v3.clone()
}
fn median_terms_19(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = std::rc::Rc::<str>::from((v1).split(&*v2).next().unwrap_or(""));
    let mut v4: f64 = number_parsed_15(v3.clone());
    let mut v5: Rc<str> = std::rc::Rc::<str>::from((v1).split_once(&*v2).map(|(_, rest)| rest).unwrap_or(""));
    let mut v6: f64 = number_parsed_15(v5.clone());
    median_terms_between_20(v0.clone(), v4, v6)
}
fn i32_text_21(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string().as_str());
    v1.clone()
}
fn numeric_note_22(mut v0: US0, mut v1: i32, mut v2: f64, mut v3: f64, mut v4: f64, mut v5: i32) -> Rc<str> {
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
        let mut v13: Rc<str> = i32_text_21(v6);
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" numeric)"); } LIT.with(|lit| lit.clone()) };
        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v16: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v13, &*v14, &*v15, &*v15].concat());
        v16.clone()
    } else {
        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v17.clone()
    }
}
fn stdout_line_23(mut v0: Rc<str>) -> () {
    println!("{}", &*v0);
    ()
}
fn tested_run_1(mut v0: US0, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>) -> i32 {
    let mut v2: Rc<str> = where_clauses_4();
    let mut v3: i32 = 3i32;
    let mut v4: Rc<str> = argument_5(v3);
    let (mut v5, mut v6): (bool, Rc<str>) = eoie_table_query::eoie_table_rows_kept(&*v4, &*v2);
    let mut v9: US2 = if v5 {
        US2::US2_RowsKept(v6.clone())
    } else {
        US2::US2_RowsRefused(v6.clone())
    };
    let (mut v16, mut v17): (Rc<str>, Rc<str>) = match &v9 {
        US2::US2_RowsKept(v10) => {
            let mut v10: Rc<str> = v10.clone();
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (v10.clone(), v11.clone())
        }
        US2::US2_RowsRefused(v12) => {
            let mut v12: Rc<str> = v12.clone();
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (v13.clone(), v12.clone())
        }
    };
    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--group"); } LIT.with(|lit| lit.clone()) };
    let mut v19: Rc<str> = option_value_8(v18.clone());
    let mut v20: i32 = optional_column_9(v1.clone(), v19.clone());
    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--metric"); } LIT.with(|lit| lit.clone()) };
    let mut v22: Rc<str> = option_value_8(v21.clone());
    let mut v23: i32 = (v1).first().and_then(|header| header.iter().position(|item| &**item == &*v22)).map(|index| index as i32).unwrap_or(-1);
    let mut v24: bool = v23 < 0i32;
    let mut v26: bool = if v24 {
        true
    } else {
        let mut v25: bool = v17.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v25
    };
    let mut v31: bool = if v26 {
        true
    } else {
        let mut v27: Rc<str> = option_value_8(v18.clone());
        let mut v28: bool = v27.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v28 {
            let mut v29: bool = v20 < 0i32;
            v29
        } else {
            false
        }
    };
    if v31 {
        let mut v32: i32 = 3i32;
        let mut v33: Rc<str> = argument_5(v32);
        let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown columns in "); } LIT.with(|lit| lit.clone()) };
        let mut v35: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v33));
        let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (header: "); } LIT.with(|lit| lit.clone()) };
        let mut v37: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v36));
        let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let mut v39: Rc<str> = std::rc::Rc::<str>::from((v1).first().map(|header| header.iter().map(|item| &**item).collect::<Vec<_>>().join(&*v38)).unwrap_or_default().as_str());
        let mut v40: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v39));
        let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
        let mut v42: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v41));
        refused_2(v42.clone())
    } else {
        let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--arms"); } LIT.with(|lit| lit.clone()) };
        let mut v45: Rc<str> = option_value_8(v44.clone());
        let mut v46: bool = v45.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v50: Rc<str> = if v46 {
            let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
            v47.clone()
        } else {
            let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let mut v49: Rc<str> = std::rc::Rc::<str>::from((v45).split(&*v48).next().unwrap_or(""));
            v49.clone()
        };
        let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let mut v52: Rc<str> = std::rc::Rc::<str>::from((v45).split_once(&*v51).map(|(_, rest)| rest).unwrap_or(""));
        let mut v53: i32 = arms_needed_10(v0.clone());
        let mut v54: bool = v53 == 2i32;
        let mut v55: i32 = -1i32;
        let mut v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v57: i32 = 1i32;
        let mut v58: i32 = ((v1).len().max(1) - 1) as i32;
        let mut v59: i32 = 0i32;
        let mut v60: f64 = 0.0f64;
        let mut v61: f64 = 0.0f64;
        let mut v62: f64 = 0.0f64;
        let mut v63: i32 = 0i32;
        let (mut v64, mut v65, mut v66, mut v67, mut v68): (i32, f64, f64, f64, i32) = tallied_11(v1.clone(), v16.clone(), v20, v55, v56.clone(), v50.clone(), v23, v57, v58, v59, v60, v61, v62, v63);
        let (mut v82, mut v83, mut v84, mut v85, mut v86): (i32, f64, f64, f64, i32) = if v54 {
            let mut v69: i32 = -1i32;
            let mut v70: i32 = 1i32;
            let mut v71: i32 = ((v1).len().max(1) - 1) as i32;
            let mut v72: i32 = 0i32;
            let mut v73: f64 = 0.0f64;
            let mut v74: f64 = 0.0f64;
            let mut v75: f64 = 0.0f64;
            let mut v76: i32 = 0i32;
            let (mut v77, mut v78, mut v79, mut v80, mut v81): (i32, f64, f64, f64, i32) = tallied_11(v1.clone(), v16.clone(), v20, v69, v56.clone(), v52.clone(), v23, v70, v71, v72, v73, v74, v75, v76);
            (v77, v78, v79, v80, v81)
        } else {
            (v64, v65, v66, v67, v68)
        };
        let mut v98: Rc<str> = match &v0 {
            US0::US0_BinomialAtMost => {
                let mut v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
                v89.clone()
            }
            US0::US0_BinomialBand => {
                let mut v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                v92.clone()
            }
            US0::US0_MedianBand => {
                let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                v91.clone()
            }
            US0::US0_OneSampleT => {
                let mut v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
                v88.clone()
            }
            US0::US0_WelchTwoSided => {
                let mut v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
                v87.clone()
            }
            US0::US0_WilsonBand => {
                let mut v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                v90.clone()
            }
        };
        let mut v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--prediction"); } LIT.with(|lit| lit.clone()) };
        let mut v100: Rc<str> = option_value_8(v99.clone());
        let mut v101: f64 = number_parsed_15(v100.clone());
        let mut v102: f64 = confidence_given_17();
        let mut v103: bool = match &v0 {
            US0::US0_MedianBand => {
                true
            }
            _ => {
                false
            }
        };
        let mut v111: Rc<str> = if v103 {
            let mut v104: i32 = -1i32;
            let mut v105: i32 = 1i32;
            let mut v106: i32 = ((v1).len().max(1) - 1) as i32;
            let mut v107: Rc<str> = values_of_18(v1.clone(), v16.clone(), v20, v104, v56.clone(), v50.clone(), v23, v105, v106, v56.clone());
            let mut v108: Rc<str> = option_value_8(v99.clone());
            median_terms_19(v107.clone(), v108.clone())
        } else {
            let mut v110: Rc<str> = eoie_experiment_stats::eoie_experiment_terms(&*v98, v64, v65, v66, v67, v68, v82, v83, v84, v85, v86, v101, v102);
            v110.clone()
        };
        let mut v132: Rc<str> = if v54 {
            let mut v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
            let mut v113: Rc<str> = Rc::<str>::from(format!("{}{}", v50, v112));
            let mut v114: Rc<str> = i32_text_21(v64);
            let mut v115: Rc<str> = Rc::<str>::from(format!("{}{}", v113, v114));
            let mut v116: Rc<str> = numeric_note_22(v0.clone(), v64, v65, v66, v67, v68);
            let mut v117: Rc<str> = Rc::<str>::from(format!("{}{}", v115, v116));
            let mut v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
            let mut v119: Rc<str> = Rc::<str>::from(format!("{}{}", v118, v52));
            let mut v120: Rc<str> = Rc::<str>::from(format!("{}{}", v119, v112));
            let mut v121: Rc<str> = i32_text_21(v82);
            let mut v122: Rc<str> = Rc::<str>::from(format!("{}{}", v120, v121));
            let mut v123: Rc<str> = numeric_note_22(v0.clone(), v82, v83, v84, v85, v86);
            let mut v124: Rc<str> = Rc::<str>::from(format!("{}{}", v122, v123));
            let mut v125: Rc<str> = Rc::<str>::from(format!("{}{}", v117, v124));
            v125.clone()
        } else {
            let mut v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
            let mut v127: Rc<str> = Rc::<str>::from(format!("{}{}", v50, v126));
            let mut v128: Rc<str> = i32_text_21(v64);
            let mut v129: Rc<str> = Rc::<str>::from(format!("{}{}", v127, v128));
            let mut v130: Rc<str> = numeric_note_22(v0.clone(), v64, v65, v66, v67, v68);
            let mut v131: Rc<str> = Rc::<str>::from(format!("{}{}", v129, v130));
            v131.clone()
        };
        let mut v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie table test ok test="); } LIT.with(|lit| lit.clone()) };
        let mut v134: Rc<str> = Rc::<str>::from(format!("{}{}", v133, v98));
        let mut v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" n="); } LIT.with(|lit| lit.clone()) };
        let mut v136: Rc<str> = Rc::<str>::from(format!("{}{}", v135, v132));
        let mut v137: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let mut v138: Rc<str> = Rc::<str>::from(format!("{}{}", v136, v137));
        let mut v139: Rc<str> = Rc::<str>::from(format!("{}{}", v138, v111));
        let mut v140: Rc<str> = Rc::<str>::from(format!("{}{}", v134, v139));
        let mut v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" post_hoc=yes"); } LIT.with(|lit| lit.clone()) };
        let mut v142: Rc<str> = Rc::<str>::from(format!("{}{}", v140, v141));
        stdout_line_23(v142.clone());
        0i32
    }
}
fn test_of_24(mut v0: Rc<str>) -> US3 {
    let mut v1: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
    if v1 {
        let mut v2: US0 = US0::US0_WelchTwoSided;
        US3::US3_TestKnown(v2.clone())
    } else {
        let mut v4: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
        if v4 {
            let mut v5: US0 = US0::US0_OneSampleT;
            US3::US3_TestKnown(v5.clone())
        } else {
            let mut v7: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
            if v7 {
                let mut v8: US0 = US0::US0_BinomialAtMost;
                US3::US3_TestKnown(v8.clone())
            } else {
                let mut v10: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                if v10 {
                    let mut v11: US0 = US0::US0_WilsonBand;
                    US3::US3_TestKnown(v11.clone())
                } else {
                    let mut v13: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                    if v13 {
                        let mut v14: US0 = US0::US0_MedianBand;
                        US3::US3_TestKnown(v14.clone())
                    } else {
                        let mut v16: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                        if v16 {
                            let mut v17: US0 = US0::US0_BinomialBand;
                            US3::US3_TestKnown(v17.clone())
                        } else {
                            US3::US3_TestUnknown
                        }
                    }
                }
            }
        }
    }
}
fn comma_count_26(mut v0: Rc<str>) -> i32 {
    let mut v1: i32 = (v0).split(char::from(44u8)).filter(|item| !item.trim().is_empty()).count() as i32;
    v1
}
fn needs_prediction_27(mut v0: US0) -> bool {
    match &v0 {
        US0::US0_WelchTwoSided => {
            false
        }
        _ => {
            true
        }
    }
}
fn plan_checked_25(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>) -> US4 {
    let mut v7: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v7 {
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--metric names the primary metric column"); } LIT.with(|lit| lit.clone()) };
        US4::US4_PlanRejected(v8.clone())
    } else {
        let mut v10: bool = v6.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v10 {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--decision states the decision rule before any data"); } LIT.with(|lit| lit.clone()) };
            US4::US4_PlanRejected(v11.clone())
        } else {
            let mut v13: bool = v3 < 2i32;
            if v13 {
                let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--runs is the planned count per arm, at least 2"); } LIT.with(|lit| lit.clone()) };
                US4::US4_PlanRejected(v14.clone())
            } else {
                let mut v16: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v19: bool = if v16 {
                    let mut v17: i32 = arms_needed_10(v0.clone());
                    let mut v18: bool = v17 == 1i32;
                    v18
                } else {
                    false
                };
                let mut v21: bool = if v19 {
                    let mut v20: bool = v2.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v20
                } else {
                    false
                };
                if v21 {
                    let mut v33: Rc<str> = match &v0 {
                        US0::US0_BinomialAtMost => {
                            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
                            v24.clone()
                        }
                        US0::US0_BinomialBand => {
                            let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                            v27.clone()
                        }
                        US0::US0_MedianBand => {
                            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                            v26.clone()
                        }
                        US0::US0_OneSampleT => {
                            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
                            v23.clone()
                        }
                        US0::US0_WelchTwoSided => {
                            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
                            v22.clone()
                        }
                        US0::US0_WilsonBand => {
                            let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                            v25.clone()
                        }
                    };
                    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" with --group needs --arms naming the group value (without both it measures the whole table)"); } LIT.with(|lit| lit.clone()) };
                    let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v36: Rc<str> = std::rc::Rc::<str>::from([&*v33, &*v34, &*v35, &*v35, &*v35].concat());
                    US4::US4_PlanRejected(v36.clone())
                } else {
                    let mut v38: i32 = comma_count_26(v1.clone());
                    let mut v39: i32 = arms_needed_10(v0.clone());
                    let mut v40: bool = v38 != v39;
                    let mut v45: bool = if v40 {
                        let mut v41: bool = v1.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        if v41 {
                            true
                        } else {
                            let mut v42: i32 = arms_needed_10(v0.clone());
                            let mut v43: bool = v42 == 2i32;
                            v43
                        }
                    } else {
                        false
                    };
                    if v45 {
                        let mut v57: Rc<str> = match &v0 {
                            US0::US0_BinomialAtMost => {
                                let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
                                v48.clone()
                            }
                            US0::US0_BinomialBand => {
                                let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                                v51.clone()
                            }
                            US0::US0_MedianBand => {
                                let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                                v50.clone()
                            }
                            US0::US0_OneSampleT => {
                                let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
                                v47.clone()
                            }
                            US0::US0_WelchTwoSided => {
                                let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
                                v46.clone()
                            }
                            US0::US0_WilsonBand => {
                                let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                                v49.clone()
                            }
                        };
                        let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" needs --arms with "); } LIT.with(|lit| lit.clone()) };
                        let mut v59: i32 = arms_needed_10(v0.clone());
                        let mut v60: bool = v59 == 2i32;
                        let mut v63: Rc<str> = if v60 {
                            let mut v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("two group values (a,b)"); } LIT.with(|lit| lit.clone()) };
                            v61.clone()
                        } else {
                            let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one group value"); } LIT.with(|lit| lit.clone()) };
                            v62.clone()
                        };
                        let mut v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v65: Rc<str> = std::rc::Rc::<str>::from([&*v57, &*v58, &*v63, &*v64, &*v64].concat());
                        US4::US4_PlanRejected(v65.clone())
                    } else {
                        let mut v67: bool = match &v0 {
                            US0::US0_MedianBand => {
                                true
                            }
                            _ => {
                                false
                            }
                        };
                        let mut v70: bool = if v67 {
                            let mut v68: i32 = comma_count_26(v4.clone());
                            let mut v69: bool = v68 != 2i32;
                            v69
                        } else {
                            false
                        };
                        if v70 {
                            let mut v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median needs --prediction low,high (the band the median must fall in)"); } LIT.with(|lit| lit.clone()) };
                            US4::US4_PlanRejected(v71.clone())
                        } else {
                            let mut v73: bool = needs_prediction_27(v0.clone());
                            let mut v75: bool = if v73 {
                                let mut v74: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                v74
                            } else {
                                false
                            };
                            if v75 {
                                let mut v87: Rc<str> = match &v0 {
                                    US0::US0_BinomialAtMost => {
                                        let mut v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
                                        v78.clone()
                                    }
                                    US0::US0_BinomialBand => {
                                        let mut v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                                        v81.clone()
                                    }
                                    US0::US0_MedianBand => {
                                        let mut v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                                        v80.clone()
                                    }
                                    US0::US0_OneSampleT => {
                                        let mut v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
                                        v77.clone()
                                    }
                                    US0::US0_WelchTwoSided => {
                                        let mut v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
                                        v76.clone()
                                    }
                                    US0::US0_WilsonBand => {
                                        let mut v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                                        v79.clone()
                                    }
                                };
                                let mut v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" needs --prediction (the model value or rate it is tested against)"); } LIT.with(|lit| lit.clone()) };
                                let mut v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v90: Rc<str> = std::rc::Rc::<str>::from([&*v87, &*v88, &*v89, &*v89, &*v89].concat());
                                US4::US4_PlanRejected(v90.clone())
                            } else {
                                US4::US4_PlanAccepted
                            }
                        }
                    }
                }
            }
        }
    }
}
fn table_tested_0() -> i32 {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--test"); } LIT.with(|lit| lit.clone()) };
    let mut v1: Rc<str> = option_value_8(v0.clone());
    let mut v2: US3 = test_of_24(v1.clone());
    match &v2 {
        US3::US3_TestKnown(v7) => {
            let mut v7: US0 = v7.clone();
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--arms"); } LIT.with(|lit| lit.clone()) };
            let mut v9: Rc<str> = option_value_8(v8.clone());
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--group"); } LIT.with(|lit| lit.clone()) };
            let mut v11: Rc<str> = option_value_8(v10.clone());
            let mut v12: i32 = 2i32;
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--prediction"); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<str> = option_value_8(v13.clone());
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--metric"); } LIT.with(|lit| lit.clone()) };
            let mut v16: Rc<str> = option_value_8(v15.clone());
            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("post hoc"); } LIT.with(|lit| lit.clone()) };
            let mut v18: US4 = plan_checked_25(v7.clone(), v9.clone(), v11.clone(), v12, v14.clone(), v16.clone(), v17.clone());
            match &v18 {
                US4::US4_PlanAccepted => {
                    let mut v25: i32 = 3i32;
                    let mut v26: Rc<str> = argument_5(v25);
                    let (mut v27, mut v28, mut v29): (bool, std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, Rc<str>) = { let path = &*v26; match std::fs::read_to_string(path) { Ok(text) => { let separator = if path.ends_with(".csv") { char::from(44u8) } else { char::from(9u8) }; (true, std::rc::Rc::new(text.lines().map(|line| line.split(separator).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>()), { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) } Err(error) => (false, std::rc::Rc::new(Vec::new()), std::rc::Rc::<str>::from(format!("{path}: {error}").as_str())) } };
                    let mut v32: US5 = if v27 {
                        US5::US5_TableLoaded(v28.clone())
                    } else {
                        US5::US5_TableUnreadable(v29.clone())
                    };
                    match &v32 {
                        US5::US5_TableLoaded(v35) => {
                            let mut v35: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = v35.clone();
                            tested_run_1(v7.clone(), v35.clone())
                        }
                        US5::US5_TableUnreadable(v33) => {
                            let mut v33: Rc<str> = v33.clone();
                            refused_2(v33.clone())
                        }
                    }
                }
                US4::US4_PlanRejected(v19) => {
                    let mut v19: Rc<str> = v19.clone();
                    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
                    let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v20));
                    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: table test <file.tsv> --test welch|one-sample|binomial|wilson|median|binomial-band --metric <column> [--arms a[,b] [--group <column>]] (a one-population test without both measures the whole table) [--prediction x] [--confidence 0.95] [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]..."); } LIT.with(|lit| lit.clone()) };
                    let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v22));
                    refused_2(v23.clone())
                }
            }
        }
        US3::US3_TestUnknown => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("needs --test with a known test; "); } LIT.with(|lit| lit.clone()) };
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: table test <file.tsv> --test welch|one-sample|binomial|wilson|median|binomial-band --metric <column> [--arms a[,b] [--group <column>]] (a one-population test without both measures the whole table) [--prediction x] [--confidence 0.95] [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]..."); } LIT.with(|lit| lit.clone()) };
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("needs --test with a known test; usage: table test <file.tsv> --test welch|one-sample|binomial|wilson|median|binomial-band --metric <column> [--arms a[,b] [--group <column>]] (a one-population test without both measures the whole table) [--prediction x] [--confidence 0.95] [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]..."); } LIT.with(|lit| lit.clone()) };
            refused_2(v5.clone())
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        table_tested_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_table_test_run() -> i32 {
    closure0()()
}
