#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US2 {
    US2_WelchTwoSided,
    US2_OneSampleT,
    US2_BinomialAtMost,
    US2_WilsonBand,
    US2_MedianBand,
    US2_BinomialBand,
}
#[derive(Clone)]
enum US1 {
    US1_TestKnown(US2),
    US1_TestUnknown,
}
#[derive(Clone)]
enum US3 {
    US3_PlanAccepted,
    US3_PlanRejected(Rc<str>),
}
fn stderr_line_3(mut v0: Rc<str>) -> () {
    eprintln!("{}", &*v0);
    ()
}
fn failed_2(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: experiment "); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v0, &*v2, &*v2, &*v2].concat());
    stderr_line_3(v3.clone());
    2i32
}
fn command_line_argument_count_5() -> i32 {
    let mut v0: i32 = (std::env::args().count() as i32) - 1;
    v0
}
fn command_line_argument_6(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::args().nth((v0 + 1) as usize).unwrap_or_default());
    v1.clone()
}
fn argument_4(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_sub(1i32);
    let mut v2: bool = v1 >= 0i32;
    let mut v5: bool = if v2 {
        let mut v3: i32 = command_line_argument_count_5();
        let mut v4: bool = v1 < v3;
        v4
    } else {
        false
    };
    let mut v9: US0 = if v5 {
        let mut v6: Rc<str> = command_line_argument_6(v1);
        US0::US0_Some(v6.clone())
    } else {
        US0::US0_None
    };
    match &v9 {
        US0::US0_None => {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US0::US0_Some(v10) => {
            let mut v10: Rc<str> = v10.clone();
            v10.clone()
        }
    }
}
fn identifier_ok_7(mut v0: Rc<str>) -> bool {
    let mut v1: bool = !(v0).is_empty() && (v0).chars().all(|c| c.is_ascii_alphanumeric() || c == char::from(45u8) || c == char::from(95u8));
    v1
}
fn option_value_8(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip_while(|arg| arg != &*v0).nth(1).unwrap_or_default());
    v1.clone()
}
fn test_of_9(mut v0: Rc<str>) -> US1 {
    let mut v1: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
    if v1 {
        let mut v2: US2 = US2::US2_WelchTwoSided;
        US1::US1_TestKnown(v2.clone())
    } else {
        let mut v4: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
        if v4 {
            let mut v5: US2 = US2::US2_OneSampleT;
            US1::US1_TestKnown(v5.clone())
        } else {
            let mut v7: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
            if v7 {
                let mut v8: US2 = US2::US2_BinomialAtMost;
                US1::US1_TestKnown(v8.clone())
            } else {
                let mut v10: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                if v10 {
                    let mut v11: US2 = US2::US2_WilsonBand;
                    US1::US1_TestKnown(v11.clone())
                } else {
                    let mut v13: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                    if v13 {
                        let mut v14: US2 = US2::US2_MedianBand;
                        US1::US1_TestKnown(v14.clone())
                    } else {
                        let mut v16: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                        if v16 {
                            let mut v17: US2 = US2::US2_BinomialBand;
                            US1::US1_TestKnown(v17.clone())
                        } else {
                            US1::US1_TestUnknown
                        }
                    }
                }
            }
        }
    }
}
fn decimal_i32_10(mut v0: Rc<str>) -> i32 {
    let mut v1: i32 = (v0).trim().parse::<i32>().unwrap_or(-1);
    v1
}
fn arms_needed_12(mut v0: US2) -> i32 {
    match &v0 {
        US2::US2_WelchTwoSided => {
            2i32
        }
        _ => {
            1i32
        }
    }
}
fn comma_count_13(mut v0: Rc<str>) -> i32 {
    let mut v1: i32 = (v0).split(char::from(44u8)).filter(|item| !item.trim().is_empty()).count() as i32;
    v1
}
fn needs_prediction_14(mut v0: US2) -> bool {
    match &v0 {
        US2::US2_WelchTwoSided => {
            false
        }
        _ => {
            true
        }
    }
}
fn plan_checked_11(mut v0: US2, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>) -> US3 {
    let mut v7: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v7 {
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--metric names the primary metric column"); } LIT.with(|lit| lit.clone()) };
        US3::US3_PlanRejected(v8.clone())
    } else {
        let mut v10: bool = v6.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v10 {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--decision states the decision rule before any data"); } LIT.with(|lit| lit.clone()) };
            US3::US3_PlanRejected(v11.clone())
        } else {
            let mut v13: bool = v3 < 2i32;
            if v13 {
                let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--runs is the planned count per arm, at least 2"); } LIT.with(|lit| lit.clone()) };
                US3::US3_PlanRejected(v14.clone())
            } else {
                let mut v16: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v19: bool = if v16 {
                    let mut v17: i32 = arms_needed_12(v0.clone());
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
                        US2::US2_BinomialAtMost => {
                            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
                            v24.clone()
                        }
                        US2::US2_BinomialBand => {
                            let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                            v27.clone()
                        }
                        US2::US2_MedianBand => {
                            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                            v26.clone()
                        }
                        US2::US2_OneSampleT => {
                            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
                            v23.clone()
                        }
                        US2::US2_WelchTwoSided => {
                            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
                            v22.clone()
                        }
                        US2::US2_WilsonBand => {
                            let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                            v25.clone()
                        }
                    };
                    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" with --group needs --arms naming the group value (without both it measures the whole table)"); } LIT.with(|lit| lit.clone()) };
                    let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v36: Rc<str> = std::rc::Rc::<str>::from([&*v33, &*v34, &*v35, &*v35, &*v35].concat());
                    US3::US3_PlanRejected(v36.clone())
                } else {
                    let mut v38: i32 = comma_count_13(v1.clone());
                    let mut v39: i32 = arms_needed_12(v0.clone());
                    let mut v40: bool = v38 != v39;
                    let mut v45: bool = if v40 {
                        let mut v41: bool = v1.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        if v41 {
                            true
                        } else {
                            let mut v42: i32 = arms_needed_12(v0.clone());
                            let mut v43: bool = v42 == 2i32;
                            v43
                        }
                    } else {
                        false
                    };
                    if v45 {
                        let mut v57: Rc<str> = match &v0 {
                            US2::US2_BinomialAtMost => {
                                let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
                                v48.clone()
                            }
                            US2::US2_BinomialBand => {
                                let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                                v51.clone()
                            }
                            US2::US2_MedianBand => {
                                let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                                v50.clone()
                            }
                            US2::US2_OneSampleT => {
                                let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
                                v47.clone()
                            }
                            US2::US2_WelchTwoSided => {
                                let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
                                v46.clone()
                            }
                            US2::US2_WilsonBand => {
                                let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                                v49.clone()
                            }
                        };
                        let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" needs --arms with "); } LIT.with(|lit| lit.clone()) };
                        let mut v59: i32 = arms_needed_12(v0.clone());
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
                        US3::US3_PlanRejected(v65.clone())
                    } else {
                        let mut v67: bool = match &v0 {
                            US2::US2_MedianBand => {
                                true
                            }
                            _ => {
                                false
                            }
                        };
                        let mut v70: bool = if v67 {
                            let mut v68: i32 = comma_count_13(v4.clone());
                            let mut v69: bool = v68 != 2i32;
                            v69
                        } else {
                            false
                        };
                        if v70 {
                            let mut v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median needs --prediction low,high (the band the median must fall in)"); } LIT.with(|lit| lit.clone()) };
                            US3::US3_PlanRejected(v71.clone())
                        } else {
                            let mut v73: bool = needs_prediction_14(v0.clone());
                            let mut v75: bool = if v73 {
                                let mut v74: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                v74
                            } else {
                                false
                            };
                            if v75 {
                                let mut v87: Rc<str> = match &v0 {
                                    US2::US2_BinomialAtMost => {
                                        let mut v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
                                        v78.clone()
                                    }
                                    US2::US2_BinomialBand => {
                                        let mut v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                                        v81.clone()
                                    }
                                    US2::US2_MedianBand => {
                                        let mut v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                                        v80.clone()
                                    }
                                    US2::US2_OneSampleT => {
                                        let mut v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
                                        v77.clone()
                                    }
                                    US2::US2_WelchTwoSided => {
                                        let mut v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
                                        v76.clone()
                                    }
                                    US2::US2_WilsonBand => {
                                        let mut v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                                        v79.clone()
                                    }
                                };
                                let mut v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" needs --prediction (the model value or rate it is tested against)"); } LIT.with(|lit| lit.clone()) };
                                let mut v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v90: Rc<str> = std::rc::Rc::<str>::from([&*v87, &*v88, &*v89, &*v89, &*v89].concat());
                                US3::US3_PlanRejected(v90.clone())
                            } else {
                                US3::US3_PlanAccepted
                            }
                        }
                    }
                }
            }
        }
    }
}
fn now_ms_15() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn civil_of_days_17(mut v0: u64) -> (u64, u64, u64) {
    let mut v1: u64 = v0.wrapping_add(719468u64);
    let mut v2: u64 = v1.wrapping_div(146097u64);
    let mut v3: u64 = v2.wrapping_mul(146097u64);
    let mut v4: u64 = v1.wrapping_sub(v3);
    let mut v5: u64 = v4.wrapping_div(1460u64);
    let mut v6: u64 = v4.wrapping_sub(v5);
    let mut v7: u64 = v4.wrapping_div(36524u64);
    let mut v8: u64 = v6.wrapping_add(v7);
    let mut v9: u64 = v4.wrapping_div(146096u64);
    let mut v10: u64 = v8.wrapping_sub(v9);
    let mut v11: u64 = v10.wrapping_div(365u64);
    let mut v12: u64 = 365u64.wrapping_mul(v11);
    let mut v13: u64 = v11.wrapping_div(4u64);
    let mut v14: u64 = v12.wrapping_add(v13);
    let mut v15: u64 = v11.wrapping_div(100u64);
    let mut v16: u64 = v14.wrapping_sub(v15);
    let mut v17: u64 = v4.wrapping_sub(v16);
    let mut v18: u64 = 5u64.wrapping_mul(v17);
    let mut v19: u64 = v18.wrapping_add(2u64);
    let mut v20: u64 = v19.wrapping_div(153u64);
    let mut v21: u64 = 153u64.wrapping_mul(v20);
    let mut v22: u64 = v21.wrapping_add(2u64);
    let mut v23: u64 = v22.wrapping_div(5u64);
    let mut v24: u64 = v17.wrapping_sub(v23);
    let mut v25: u64 = v24.wrapping_add(1u64);
    let mut v26: bool = v20 < 10u64;
    let mut v29: u64 = if v26 {
        let mut v27: u64 = v20.wrapping_add(3u64);
        v27
    } else {
        let mut v28: u64 = v20.wrapping_sub(9u64);
        v28
    };
    let mut v30: u64 = v2.wrapping_mul(400u64);
    let mut v31: u64 = v11.wrapping_add(v30);
    let mut v32: bool = v29 < 3u64;
    let mut v33: u64 = if v32 {
        1u64
    } else {
        0u64
    };
    let mut v34: u64 = v31.wrapping_add(v33);
    (v34, v29, v25)
}
fn utc_stamp_16(mut v0: u64) -> Rc<str> {
    let mut v1: u64 = v0.wrapping_div(1000u64);
    let mut v2: u64 = v1.wrapping_div(86400u64);
    let (mut v3, mut v4, mut v5): (u64, u64, u64) = civil_of_days_17(v2);
    let mut v6: u64 = v1.wrapping_rem(86400u64);
    let mut v7: u64 = v6.wrapping_div(60u64);
    let mut v8: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let mut v10: bool = v4 < 10u64;
    let mut v16: Rc<str> = if v10 {
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v12, &*v13, &*v13, &*v13].concat());
        v14.clone()
    } else {
        let mut v15: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
        v15.clone()
    };
    let mut v17: bool = v5 < 10u64;
    let mut v23: Rc<str> = if v17 {
        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let mut v19: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v21: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v19, &*v20, &*v20, &*v20].concat());
        v21.clone()
    } else {
        let mut v22: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
        v22.clone()
    };
    let mut v24: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v9, &*v16, &*v9, &*v23].concat());
    let mut v25: u64 = v7.wrapping_div(60u64);
    let mut v26: bool = v25 < 10u64;
    let mut v32: Rc<str> = if v26 {
        let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let mut v28: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
        let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v30: Rc<str> = std::rc::Rc::<str>::from([&*v27, &*v28, &*v29, &*v29, &*v29].concat());
        v30.clone()
    } else {
        let mut v31: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
        v31.clone()
    };
    let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let mut v34: u64 = v7.wrapping_rem(60u64);
    let mut v35: bool = v34 < 10u64;
    let mut v41: Rc<str> = if v35 {
        let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let mut v37: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
        let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v39: Rc<str> = std::rc::Rc::<str>::from([&*v36, &*v37, &*v38, &*v38, &*v38].concat());
        v39.clone()
    } else {
        let mut v40: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
        v40.clone()
    };
    let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Z"); } LIT.with(|lit| lit.clone()) };
    let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v44: Rc<str> = std::rc::Rc::<str>::from([&*v32, &*v33, &*v41, &*v42, &*v43].concat());
    let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("T"); } LIT.with(|lit| lit.clone()) };
    let mut v46: Rc<str> = std::rc::Rc::<str>::from([&*v24, &*v45, &*v44, &*v43, &*v43].concat());
    v46.clone()
}
fn field_line_18(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(format!("{}={}
", &*v0, (v1).chars().map(|c| if c.is_control() { char::from(32u8) } else { c }).collect::<String>()).as_str());
    v2.clone()
}
fn joined_19(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2.clone()
}
fn local_stamp_20() -> Rc<str> {
    let mut v0: Rc<str> = { #[cfg(windows)] fn local() -> String { #[repr(C)] struct Clock { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(clock: *mut Clock); } let mut clock = Clock { year: 0, month: 0, weekday: 0, day: 0, hour: 0, minute: 0, second: 0, millis: 0 }; unsafe { GetLocalTime(&mut clock); } let _ = clock.weekday; let _ = clock.millis; format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}", clock.year, clock.month, clock.day, clock.hour, clock.minute, clock.second) } #[cfg(not(windows))] fn local() -> String { std::process::Command::new("date").arg("+%Y-%m-%dT%H:%M:%S").output().ok().map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned()).unwrap_or_default() } std::rc::Rc::<str>::from(local().as_str()) };
    v0.clone()
}
fn text_created_21(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from({ let path = std::path::Path::new(&*v0); let created = path.parent().map(std::fs::create_dir_all).unwrap_or(Ok(())).and_then(|_| std::fs::OpenOptions::new().write(true).create_new(true).open(path)).and_then(|mut file| { use std::io::Write; file.write_all((v1).as_bytes()) }); created.err().map(|error| format!("write {}: {error}", path.display())).unwrap_or_default() }.as_str());
    v2.clone()
}
fn stdout_line_22(mut v0: Rc<str>) -> () {
    println!("{}", &*v0);
    ()
}
fn registered_1() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: Rc<str> = argument_4(v0);
    let mut v2: i32 = 4i32;
    let mut v3: Rc<str> = argument_4(v2);
    let mut v4: bool = identifier_ok_7(v3.clone());
    let mut v5: bool = v4 == false;
    if v5 {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("register: the ID must be letters, digits, '-' or '_', received '"); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'; "); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: experiment register <ledger> <ID> --test "); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch|one-sample|binomial|wilson|median|binomial-band"); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" --metric <col> --runs <n per arm> --decision <rule> [--group <col> --arms a[,b]] [--prediction <x>] [--confidence 0.95] [--question <text>] [--time <col>] | experiment analyze <ledger> <ID> <table.tsv> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--interim <reason>] | experiment amend <ledger> <ID> --field <name> --value <v> --reason <text> | experiment addendum <ledger> <ID> <text> | experiment show <ledger> <ID> | experiment import <ledger> <ID> --registered-at <iso> --evidence <commit|file@line> <register flags> | experiment record-verdict <ledger> <ID> --verdict <text> --at <iso> [--evidence <ref>]"); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v9, &*v10, &*v11, &*v11].concat());
        let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v3, &*v7, &*v12, &*v11].concat());
        failed_2(v13.clone())
    } else {
        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--test"); } LIT.with(|lit| lit.clone()) };
        let mut v16: Rc<str> = option_value_8(v15.clone());
        let mut v17: US1 = test_of_9(v16.clone());
        match &v17 {
            US1::US1_TestKnown(v25) => {
                let mut v25: US2 = v25.clone();
                let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--runs"); } LIT.with(|lit| lit.clone()) };
                let mut v27: Rc<str> = option_value_8(v26.clone());
                let mut v28: i32 = decimal_i32_10(v27.clone());
                let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--arms"); } LIT.with(|lit| lit.clone()) };
                let mut v30: Rc<str> = option_value_8(v29.clone());
                let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--group"); } LIT.with(|lit| lit.clone()) };
                let mut v32: Rc<str> = option_value_8(v31.clone());
                let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--prediction"); } LIT.with(|lit| lit.clone()) };
                let mut v34: Rc<str> = option_value_8(v33.clone());
                let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--metric"); } LIT.with(|lit| lit.clone()) };
                let mut v36: Rc<str> = option_value_8(v35.clone());
                let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--decision"); } LIT.with(|lit| lit.clone()) };
                let mut v38: Rc<str> = option_value_8(v37.clone());
                let mut v39: US3 = plan_checked_11(v25.clone(), v30.clone(), v32.clone(), v28, v34.clone(), v36.clone(), v38.clone());
                match &v39 {
                    US3::US3_PlanAccepted => {
                        let mut v45: u64 = now_ms_15();
                        let mut v46: Rc<str> = utc_stamp_16(v45);
                        let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--confidence"); } LIT.with(|lit| lit.clone()) };
                        let mut v48: Rc<str> = option_value_8(v47.clone());
                        let mut v49: bool = v48.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v52: Rc<str> = if v49 {
                            let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0.95"); } LIT.with(|lit| lit.clone()) };
                            v50.clone()
                        } else {
                            option_value_8(v47.clone())
                        };
                        let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("id"); } LIT.with(|lit| lit.clone()) };
                        let mut v54: Rc<str> = field_line_18(v53.clone(), v3.clone());
                        let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("registered"); } LIT.with(|lit| lit.clone()) };
                        let mut v56: Rc<str> = field_line_18(v55.clone(), v46.clone());
                        let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("question"); } LIT.with(|lit| lit.clone()) };
                        let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--question"); } LIT.with(|lit| lit.clone()) };
                        let mut v59: Rc<str> = option_value_8(v58.clone());
                        let mut v60: Rc<str> = field_line_18(v57.clone(), v59.clone());
                        let mut v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test"); } LIT.with(|lit| lit.clone()) };
                        let mut v73: Rc<str> = match &v25 {
                            US2::US2_BinomialAtMost => {
                                let mut v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
                                v64.clone()
                            }
                            US2::US2_BinomialBand => {
                                let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                                v67.clone()
                            }
                            US2::US2_MedianBand => {
                                let mut v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                                v66.clone()
                            }
                            US2::US2_OneSampleT => {
                                let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
                                v63.clone()
                            }
                            US2::US2_WelchTwoSided => {
                                let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
                                v62.clone()
                            }
                            US2::US2_WilsonBand => {
                                let mut v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                                v65.clone()
                            }
                        };
                        let mut v74: Rc<str> = field_line_18(v61.clone(), v73.clone());
                        let mut v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("metric"); } LIT.with(|lit| lit.clone()) };
                        let mut v76: Rc<str> = option_value_8(v35.clone());
                        let mut v77: Rc<str> = field_line_18(v75.clone(), v76.clone());
                        let mut v78: Rc<str> = std::rc::Rc::<str>::from([&*v54, &*v56, &*v60, &*v74, &*v77].concat());
                        let mut v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("group"); } LIT.with(|lit| lit.clone()) };
                        let mut v80: Rc<str> = option_value_8(v31.clone());
                        let mut v81: Rc<str> = field_line_18(v79.clone(), v80.clone());
                        let mut v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("arms"); } LIT.with(|lit| lit.clone()) };
                        let mut v83: Rc<str> = option_value_8(v29.clone());
                        let mut v84: Rc<str> = field_line_18(v82.clone(), v83.clone());
                        let mut v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("runs"); } LIT.with(|lit| lit.clone()) };
                        let mut v86: Rc<str> = option_value_8(v26.clone());
                        let mut v87: Rc<str> = field_line_18(v85.clone(), v86.clone());
                        let mut v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("prediction"); } LIT.with(|lit| lit.clone()) };
                        let mut v89: Rc<str> = option_value_8(v33.clone());
                        let mut v90: Rc<str> = field_line_18(v88.clone(), v89.clone());
                        let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("confidence"); } LIT.with(|lit| lit.clone()) };
                        let mut v92: Rc<str> = field_line_18(v91.clone(), v52.clone());
                        let mut v93: Rc<str> = std::rc::Rc::<str>::from([&*v81, &*v84, &*v87, &*v90, &*v92].concat());
                        let mut v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("registration"); } LIT.with(|lit| lit.clone()) };
                        let mut v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                        let mut v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v97: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v95, &*v94, &*v96, &*v96].concat());
                        let mut v98: Rc<str> = joined_19(v1.clone(), v97.clone());
                        let mut v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("decision"); } LIT.with(|lit| lit.clone()) };
                        let mut v100: Rc<str> = option_value_8(v37.clone());
                        let mut v101: Rc<str> = field_line_18(v99.clone(), v100.clone());
                        let mut v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("time"); } LIT.with(|lit| lit.clone()) };
                        let mut v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--time"); } LIT.with(|lit| lit.clone()) };
                        let mut v104: Rc<str> = option_value_8(v103.clone());
                        let mut v105: Rc<str> = field_line_18(v102.clone(), v104.clone());
                        let mut v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("registered_local"); } LIT.with(|lit| lit.clone()) };
                        let mut v107: Rc<str> = local_stamp_20();
                        let mut v108: Rc<str> = field_line_18(v106.clone(), v107.clone());
                        let mut v109: Rc<str> = std::rc::Rc::<str>::from([&*v78, &*v93, &*v101, &*v105, &*v108].concat());
                        let mut v110: Rc<str> = text_created_21(v98.clone(), v109.clone());
                        let mut v111: bool = v110.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        if v111 {
                            let mut v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("register: "); } LIT.with(|lit| lit.clone()) };
                            let mut v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is already registered or the ledger is unwritable ("); } LIT.with(|lit| lit.clone()) };
                            let mut v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
                            let mut v115: Rc<str> = std::rc::Rc::<str>::from([&*v112, &*v3, &*v113, &*v110, &*v114].concat());
                            failed_2(v115.clone())
                        } else {
                            let mut v117: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie experiment register ok id="); } LIT.with(|lit| lit.clone()) };
                            let mut v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" test="); } LIT.with(|lit| lit.clone()) };
                            let mut v130: Rc<str> = match &v25 {
                                US2::US2_BinomialAtMost => {
                                    let mut v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
                                    v121.clone()
                                }
                                US2::US2_BinomialBand => {
                                    let mut v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                                    v124.clone()
                                }
                                US2::US2_MedianBand => {
                                    let mut v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                                    v123.clone()
                                }
                                US2::US2_OneSampleT => {
                                    let mut v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
                                    v120.clone()
                                }
                                US2::US2_WelchTwoSided => {
                                    let mut v119: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
                                    v119.clone()
                                }
                                US2::US2_WilsonBand => {
                                    let mut v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                                    v122.clone()
                                }
                            };
                            let mut v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" runs="); } LIT.with(|lit| lit.clone()) };
                            let mut v132: Rc<str> = option_value_8(v26.clone());
                            let mut v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" registered="); } LIT.with(|lit| lit.clone()) };
                            let mut v134: Rc<str> = std::rc::Rc::<str>::from([&*v131, &*v132, &*v133, &*v46, &*v96].concat());
                            let mut v135: Rc<str> = std::rc::Rc::<str>::from([&*v117, &*v3, &*v118, &*v130, &*v134].concat());
                            stdout_line_22(v135.clone());
                            0i32
                        }
                    }
                    US3::US3_PlanRejected(v40) => {
                        let mut v40: Rc<str> = v40.clone();
                        let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("register: "); } LIT.with(|lit| lit.clone()) };
                        let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v43: Rc<str> = std::rc::Rc::<str>::from([&*v41, &*v40, &*v42, &*v42, &*v42].concat());
                        failed_2(v43.clone())
                    }
                }
            }
            US1::US1_TestUnknown => {
                let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("register: --test must be "); } LIT.with(|lit| lit.clone()) };
                let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch|one-sample|binomial|wilson|median|binomial-band"); } LIT.with(|lit| lit.clone()) };
                let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", received '"); } LIT.with(|lit| lit.clone()) };
                let mut v21: Rc<str> = option_value_8(v15.clone());
                let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
                let mut v23: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v19, &*v20, &*v21, &*v22].concat());
                failed_2(v23.clone())
            }
        }
    }
}
fn file_text_24(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1.clone()
}
fn path_exists_25(mut v0: Rc<str>) -> bool {
    let mut v1: bool = std::path::Path::new(&*v0).exists();
    v1
}
fn shown_23() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: Rc<str> = argument_4(v0);
    let mut v2: i32 = 4i32;
    let mut v3: Rc<str> = argument_4(v2);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("registration"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v5, &*v4, &*v6, &*v6].concat());
    let mut v8: Rc<str> = joined_19(v1.clone(), v7.clone());
    let mut v9: Rc<str> = file_text_24(v8.clone());
    let mut v10: bool = v9.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v10 {
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show: no registration for "); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" in "); } LIT.with(|lit| lit.clone()) };
        let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v3, &*v12, &*v1, &*v6].concat());
        failed_2(v13.clone())
    } else {
        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("registration\n"); } LIT.with(|lit| lit.clone()) };
        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("amendments\n"); } LIT.with(|lit| lit.clone()) };
        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("amendments"); } LIT.with(|lit| lit.clone()) };
        let mut v18: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v5, &*v17, &*v6, &*v6].concat());
        let mut v19: Rc<str> = joined_19(v1.clone(), v18.clone());
        let mut v20: Rc<str> = file_text_24(v19.clone());
        let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("disclosures\n"); } LIT.with(|lit| lit.clone()) };
        let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("disclosures"); } LIT.with(|lit| lit.clone()) };
        let mut v23: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v5, &*v22, &*v6, &*v6].concat());
        let mut v24: Rc<str> = joined_19(v1.clone(), v23.clone());
        let mut v25: Rc<str> = file_text_24(v24.clone());
        let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("verdict\n"); } LIT.with(|lit| lit.clone()) };
        let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("verdict"); } LIT.with(|lit| lit.clone()) };
        let mut v28: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v5, &*v27, &*v6, &*v6].concat());
        let mut v29: Rc<str> = joined_19(v1.clone(), v28.clone());
        let mut v30: Rc<str> = file_text_24(v29.clone());
        let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("addenda\n"); } LIT.with(|lit| lit.clone()) };
        let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("addenda"); } LIT.with(|lit| lit.clone()) };
        let mut v33: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v5, &*v32, &*v6, &*v6].concat());
        let mut v34: Rc<str> = joined_19(v1.clone(), v33.clone());
        let mut v35: Rc<str> = file_text_24(v34.clone());
        let mut v36: Rc<str> = std::rc::Rc::<str>::from([&*v31, &*v35, &*v6, &*v6, &*v6].concat());
        let mut v37: Rc<str> = std::rc::Rc::<str>::from([&*v21, &*v25, &*v26, &*v30, &*v36].concat());
        let mut v38: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v9, &*v16, &*v20, &*v37].concat());
        stdout_line_22(v38.clone());
        let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie experiment show ok id="); } LIT.with(|lit| lit.clone()) };
        let mut v40: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v5, &*v27, &*v6, &*v6].concat());
        let mut v41: Rc<str> = joined_19(v1.clone(), v40.clone());
        let mut v42: bool = path_exists_25(v41.clone());
        let mut v45: Rc<str> = if v42 {
            let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" verdict=recorded"); } LIT.with(|lit| lit.clone()) };
            v43.clone()
        } else {
            let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" verdict=pending"); } LIT.with(|lit| lit.clone()) };
            v44.clone()
        };
        let mut v46: Rc<str> = std::rc::Rc::<str>::from([&*v39, &*v3, &*v45, &*v6, &*v6].concat());
        stdout_line_22(v46.clone());
        0i32
    }
}
fn experiment_run_0() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: Rc<str> = argument_4(v0);
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("register"); } LIT.with(|lit| lit.clone()) };
    if v2 {
        registered_1()
    } else {
        let mut v4: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("analyze"); } LIT.with(|lit| lit.clone()) };
        if v4 {
            let mut v5: i32 = eoie_experiment_analysis::eoie_experiment_analyze_run();
            v5
        } else {
            let mut v6: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show"); } LIT.with(|lit| lit.clone()) };
            if v6 {
                shown_23()
            } else {
                let mut v8: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("amend"); } LIT.with(|lit| lit.clone()) };
                if v8 {
                    let mut v9: i32 = eoie_experiment_amend::eoie_experiment_amend_run();
                    v9
                } else {
                    let mut v10: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("addendum"); } LIT.with(|lit| lit.clone()) };
                    if v10 {
                        let mut v11: i32 = eoie_experiment_amend::eoie_experiment_addendum_run();
                        v11
                    } else {
                        let mut v12: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("import"); } LIT.with(|lit| lit.clone()) };
                        if v12 {
                            let mut v13: i32 = eoie_experiment_import::eoie_experiment_import_run();
                            v13
                        } else {
                            let mut v14: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("record-verdict"); } LIT.with(|lit| lit.clone()) };
                            if v14 {
                                let mut v15: i32 = eoie_experiment_import::eoie_experiment_verdict_run();
                                v15
                            } else {
                                let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown verb '"); } LIT.with(|lit| lit.clone()) };
                                let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'; "); } LIT.with(|lit| lit.clone()) };
                                let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: experiment register <ledger> <ID> --test "); } LIT.with(|lit| lit.clone()) };
                                let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch|one-sample|binomial|wilson|median|binomial-band"); } LIT.with(|lit| lit.clone()) };
                                let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" --metric <col> --runs <n per arm> --decision <rule> [--group <col> --arms a[,b]] [--prediction <x>] [--confidence 0.95] [--question <text>] [--time <col>] | experiment analyze <ledger> <ID> <table.tsv> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--interim <reason>] | experiment amend <ledger> <ID> --field <name> --value <v> --reason <text> | experiment addendum <ledger> <ID> <text> | experiment show <ledger> <ID> | experiment import <ledger> <ID> --registered-at <iso> --evidence <commit|file@line> <register flags> | experiment record-verdict <ledger> <ID> --verdict <text> --at <iso> [--evidence <ref>]"); } LIT.with(|lit| lit.clone()) };
                                let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v22: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v19, &*v20, &*v21, &*v21].concat());
                                let mut v23: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v1, &*v17, &*v22, &*v21].concat());
                                failed_2(v23.clone())
                            }
                        }
                    }
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        experiment_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_experiment_run() -> i32 {
    closure0()()
}
