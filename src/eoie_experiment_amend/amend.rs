#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_AmendAccepted,
    US1_AmendRefused(Rc<str>),
}
#[derive(Clone)]
enum US3 {
    US3_WelchTwoSided,
    US3_OneSampleT,
    US3_BinomialAtMost,
    US3_WilsonBand,
    US3_MedianBand,
    US3_BinomialBand,
}
#[derive(Clone)]
enum US2 {
    US2_TestKnown(US3),
    US2_TestUnknown,
}
#[derive(Clone)]
enum US4 {
    US4_PlanAccepted,
    US4_PlanRejected(Rc<str>),
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
fn command_line_argument_count_4() -> i32 {
    let mut v0: i32 = (std::env::args().count() as i32) - 1;
    v0
}
fn command_line_argument_5(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::args().nth((v0 + 1) as usize).unwrap_or_default());
    v1.clone()
}
fn argument_3(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_sub(1i32);
    let mut v2: bool = v1 >= 0i32;
    let mut v5: bool = if v2 {
        let mut v3: i32 = command_line_argument_count_4();
        let mut v4: bool = v1 < v3;
        v4
    } else {
        false
    };
    let mut v9: US0 = if v5 {
        let mut v6: Rc<str> = command_line_argument_5(v1);
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
fn joined_7(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2.clone()
}
fn file_text_8(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1.clone()
}
fn plan_text_6(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("registration"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v3, &*v2, &*v4, &*v4].concat());
    let mut v6: Rc<str> = joined_7(v0.clone(), v5.clone());
    let mut v7: Rc<str> = file_text_8(v6.clone());
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("amendments"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v3, &*v8, &*v4, &*v4].concat());
    let mut v10: Rc<str> = joined_7(v0.clone(), v9.clone());
    let mut v11: Rc<str> = file_text_8(v10.clone());
    let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v11, &*v4, &*v4, &*v4].concat());
    v12.clone()
}
fn option_value_9(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip_while(|arg| arg != &*v0).nth(1).unwrap_or_default());
    v1.clone()
}
fn path_exists_11(mut v0: Rc<str>) -> bool {
    let mut v1: bool = std::path::Path::new(&*v0).exists();
    v1
}
fn looked_at_10(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("disclosures"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v3, &*v2, &*v4, &*v4].concat());
    let mut v6: Rc<str> = joined_7(v0.clone(), v5.clone());
    let mut v7: bool = path_exists_11(v6.clone());
    if v7 {
        true
    } else {
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("verdict"); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v3, &*v8, &*v4, &*v4].concat());
        let mut v10: Rc<str> = joined_7(v0.clone(), v9.clone());
        path_exists_11(v10.clone())
    }
}
fn amendable_named_13(mut v0: Rc<str>) -> bool {
    let mut v1: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("question"); } LIT.with(|lit| lit.clone()) };
    if v1 {
        true
    } else {
        let mut v2: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test"); } LIT.with(|lit| lit.clone()) };
        if v2 {
            true
        } else {
            let mut v3: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("metric"); } LIT.with(|lit| lit.clone()) };
            if v3 {
                true
            } else {
                let mut v4: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("group"); } LIT.with(|lit| lit.clone()) };
                if v4 {
                    true
                } else {
                    let mut v5: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("arms"); } LIT.with(|lit| lit.clone()) };
                    if v5 {
                        true
                    } else {
                        let mut v6: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("runs"); } LIT.with(|lit| lit.clone()) };
                        if v6 {
                            true
                        } else {
                            let mut v7: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("prediction"); } LIT.with(|lit| lit.clone()) };
                            if v7 {
                                true
                            } else {
                                let mut v8: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("confidence"); } LIT.with(|lit| lit.clone()) };
                                if v8 {
                                    true
                                } else {
                                    let mut v9: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("decision"); } LIT.with(|lit| lit.clone()) };
                                    if v9 {
                                        true
                                    } else {
                                        let mut v10: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("time"); } LIT.with(|lit| lit.clone()) };
                                        v10
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn amend_checked_12(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: bool) -> US1 {
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the metric was already read (an interim look or the verdict); an amendment must precede the runs it changes, so record an addendum instead"); } LIT.with(|lit| lit.clone()) };
        US1::US1_AmendRefused(v3.clone())
    } else {
        let mut v5: bool = amendable_named_13(v0.clone());
        let mut v6: bool = v5 == false;
        if v6 {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--field must be one of "); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("question|test|metric|group|arms|runs|prediction|confidence|decision|time"); } LIT.with(|lit| lit.clone()) };
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", received '"); } LIT.with(|lit| lit.clone()) };
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
            let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v8, &*v9, &*v0, &*v10].concat());
            US1::US1_AmendRefused(v11.clone())
        } else {
            let mut v13: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v13 {
                let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--reason says why the plan changes"); } LIT.with(|lit| lit.clone()) };
                US1::US1_AmendRefused(v14.clone())
            } else {
                US1::US1_AmendAccepted
            }
        }
    }
}
fn now_ms_14() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn civil_of_days_16(mut v0: u64) -> (u64, u64, u64) {
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
fn utc_stamp_15(mut v0: u64) -> Rc<str> {
    let mut v1: u64 = v0.wrapping_div(1000u64);
    let mut v2: u64 = v1.wrapping_div(86400u64);
    let (mut v3, mut v4, mut v5): (u64, u64, u64) = civil_of_days_16(v2);
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
fn field_line_17(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(format!("{}={}
", &*v0, (v1).chars().map(|c| if c.is_control() { char::from(32u8) } else { c }).collect::<String>()).as_str());
    v2.clone()
}
fn local_stamp_18() -> Rc<str> {
    let mut v0: Rc<str> = { #[cfg(windows)] fn local() -> String { #[repr(C)] struct Clock { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(clock: *mut Clock); } let mut clock = Clock { year: 0, month: 0, weekday: 0, day: 0, hour: 0, minute: 0, second: 0, millis: 0 }; unsafe { GetLocalTime(&mut clock); } let _ = clock.weekday; let _ = clock.millis; format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}", clock.year, clock.month, clock.day, clock.hour, clock.minute, clock.second) } #[cfg(not(windows))] fn local() -> String { std::process::Command::new("date").arg("+%Y-%m-%dT%H:%M:%S").output().ok().map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned()).unwrap_or_default() } std::rc::Rc::<str>::from(local().as_str()) };
    v0.clone()
}
fn field_of_19(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from((v0).lines().rev().find_map(|line| line.split_once(char::from(61u8)).filter(|(name, _)| *name == &*v1).map(|(_, value)| value)).unwrap_or(""));
    v2.clone()
}
fn test_of_20(mut v0: Rc<str>) -> US2 {
    let mut v1: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
    if v1 {
        let mut v2: US3 = US3::US3_WelchTwoSided;
        US2::US2_TestKnown(v2.clone())
    } else {
        let mut v4: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
        if v4 {
            let mut v5: US3 = US3::US3_OneSampleT;
            US2::US2_TestKnown(v5.clone())
        } else {
            let mut v7: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
            if v7 {
                let mut v8: US3 = US3::US3_BinomialAtMost;
                US2::US2_TestKnown(v8.clone())
            } else {
                let mut v10: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                if v10 {
                    let mut v11: US3 = US3::US3_WilsonBand;
                    US2::US2_TestKnown(v11.clone())
                } else {
                    let mut v13: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                    if v13 {
                        let mut v14: US3 = US3::US3_MedianBand;
                        US2::US2_TestKnown(v14.clone())
                    } else {
                        let mut v16: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                        if v16 {
                            let mut v17: US3 = US3::US3_BinomialBand;
                            US2::US2_TestKnown(v17.clone())
                        } else {
                            US2::US2_TestUnknown
                        }
                    }
                }
            }
        }
    }
}
fn decimal_i32_21(mut v0: Rc<str>) -> i32 {
    let mut v1: i32 = (v0).trim().parse::<i32>().unwrap_or(-1);
    v1
}
fn arms_needed_23(mut v0: US3) -> i32 {
    match &v0 {
        US3::US3_WelchTwoSided => {
            2i32
        }
        _ => {
            1i32
        }
    }
}
fn comma_count_24(mut v0: Rc<str>) -> i32 {
    let mut v1: i32 = (v0).split(char::from(44u8)).filter(|item| !item.trim().is_empty()).count() as i32;
    v1
}
fn needs_prediction_25(mut v0: US3) -> bool {
    match &v0 {
        US3::US3_WelchTwoSided => {
            false
        }
        _ => {
            true
        }
    }
}
fn plan_checked_22(mut v0: US3, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>) -> US4 {
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
                    let mut v17: i32 = arms_needed_23(v0.clone());
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
                        US3::US3_BinomialAtMost => {
                            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
                            v24.clone()
                        }
                        US3::US3_BinomialBand => {
                            let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                            v27.clone()
                        }
                        US3::US3_MedianBand => {
                            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                            v26.clone()
                        }
                        US3::US3_OneSampleT => {
                            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
                            v23.clone()
                        }
                        US3::US3_WelchTwoSided => {
                            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
                            v22.clone()
                        }
                        US3::US3_WilsonBand => {
                            let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                            v25.clone()
                        }
                    };
                    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" with --group needs --arms naming the group value (without both it measures the whole table)"); } LIT.with(|lit| lit.clone()) };
                    let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v36: Rc<str> = std::rc::Rc::<str>::from([&*v33, &*v34, &*v35, &*v35, &*v35].concat());
                    US4::US4_PlanRejected(v36.clone())
                } else {
                    let mut v38: i32 = comma_count_24(v1.clone());
                    let mut v39: i32 = arms_needed_23(v0.clone());
                    let mut v40: bool = v38 != v39;
                    let mut v45: bool = if v40 {
                        let mut v41: bool = v1.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        if v41 {
                            true
                        } else {
                            let mut v42: i32 = arms_needed_23(v0.clone());
                            let mut v43: bool = v42 == 2i32;
                            v43
                        }
                    } else {
                        false
                    };
                    if v45 {
                        let mut v57: Rc<str> = match &v0 {
                            US3::US3_BinomialAtMost => {
                                let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
                                v48.clone()
                            }
                            US3::US3_BinomialBand => {
                                let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                                v51.clone()
                            }
                            US3::US3_MedianBand => {
                                let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                                v50.clone()
                            }
                            US3::US3_OneSampleT => {
                                let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
                                v47.clone()
                            }
                            US3::US3_WelchTwoSided => {
                                let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
                                v46.clone()
                            }
                            US3::US3_WilsonBand => {
                                let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                                v49.clone()
                            }
                        };
                        let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" needs --arms with "); } LIT.with(|lit| lit.clone()) };
                        let mut v59: i32 = arms_needed_23(v0.clone());
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
                            US3::US3_MedianBand => {
                                true
                            }
                            _ => {
                                false
                            }
                        };
                        let mut v70: bool = if v67 {
                            let mut v68: i32 = comma_count_24(v4.clone());
                            let mut v69: bool = v68 != 2i32;
                            v69
                        } else {
                            false
                        };
                        if v70 {
                            let mut v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median needs --prediction low,high (the band the median must fall in)"); } LIT.with(|lit| lit.clone()) };
                            US4::US4_PlanRejected(v71.clone())
                        } else {
                            let mut v73: bool = needs_prediction_25(v0.clone());
                            let mut v75: bool = if v73 {
                                let mut v74: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                v74
                            } else {
                                false
                            };
                            if v75 {
                                let mut v87: Rc<str> = match &v0 {
                                    US3::US3_BinomialAtMost => {
                                        let mut v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
                                        v78.clone()
                                    }
                                    US3::US3_BinomialBand => {
                                        let mut v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                                        v81.clone()
                                    }
                                    US3::US3_MedianBand => {
                                        let mut v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                                        v80.clone()
                                    }
                                    US3::US3_OneSampleT => {
                                        let mut v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
                                        v77.clone()
                                    }
                                    US3::US3_WelchTwoSided => {
                                        let mut v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
                                        v76.clone()
                                    }
                                    US3::US3_WilsonBand => {
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
fn text_appended_26(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::fs::OpenOptions::new().create(true).append(true).open(&*v0).and_then(|mut file| { use std::io::Write; file.write_all((v1).as_bytes()) }).err().map(|error| format!("append {}: {error}", &*v0)).unwrap_or_default().as_str());
    v2.clone()
}
fn stdout_line_27(mut v0: Rc<str>) -> () {
    println!("{}", &*v0);
    ()
}
fn amended_0() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: Rc<str> = argument_3(v0);
    let mut v2: i32 = 4i32;
    let mut v3: Rc<str> = argument_3(v2);
    let mut v4: Rc<str> = plan_text_6(v1.clone(), v3.clone());
    let mut v5: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v5 {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("amend: no registration for "); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" in "); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v3, &*v7, &*v1, &*v8].concat());
        failed_1(v9.clone())
    } else {
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--field"); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = option_value_9(v11.clone());
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--reason"); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<str> = option_value_9(v13.clone());
        let mut v15: bool = looked_at_10(v1.clone(), v3.clone());
        let mut v16: US1 = amend_checked_12(v12.clone(), v14.clone(), v15);
        match &v16 {
            US1::US1_AmendAccepted => {
                let mut v22: u64 = now_ms_14();
                let mut v23: Rc<str> = utc_stamp_15(v22);
                let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("amended"); } LIT.with(|lit| lit.clone()) };
                let mut v25: Rc<str> = field_line_17(v24.clone(), v23.clone());
                let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("reason"); } LIT.with(|lit| lit.clone()) };
                let mut v27: Rc<str> = option_value_9(v13.clone());
                let mut v28: Rc<str> = field_line_17(v26.clone(), v27.clone());
                let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--value"); } LIT.with(|lit| lit.clone()) };
                let mut v30: Rc<str> = option_value_9(v29.clone());
                let mut v31: Rc<str> = field_line_17(v12.clone(), v30.clone());
                let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("registered_local"); } LIT.with(|lit| lit.clone()) };
                let mut v33: Rc<str> = local_stamp_18();
                let mut v34: Rc<str> = field_line_17(v32.clone(), v33.clone());
                let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v36: Rc<str> = std::rc::Rc::<str>::from([&*v25, &*v28, &*v31, &*v34, &*v35].concat());
                let mut v37: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v36, &*v35, &*v35, &*v35].concat());
                let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test"); } LIT.with(|lit| lit.clone()) };
                let mut v39: Rc<str> = field_of_19(v37.clone(), v38.clone());
                let mut v40: US2 = test_of_20(v39.clone());
                match &v40 {
                    US2::US2_TestKnown(v48) => {
                        let mut v48: US3 = v48.clone();
                        let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("arms"); } LIT.with(|lit| lit.clone()) };
                        let mut v50: Rc<str> = field_of_19(v37.clone(), v49.clone());
                        let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("group"); } LIT.with(|lit| lit.clone()) };
                        let mut v52: Rc<str> = field_of_19(v37.clone(), v51.clone());
                        let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("runs"); } LIT.with(|lit| lit.clone()) };
                        let mut v54: Rc<str> = field_of_19(v37.clone(), v53.clone());
                        let mut v55: i32 = decimal_i32_21(v54.clone());
                        let mut v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("prediction"); } LIT.with(|lit| lit.clone()) };
                        let mut v57: Rc<str> = field_of_19(v37.clone(), v56.clone());
                        let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("metric"); } LIT.with(|lit| lit.clone()) };
                        let mut v59: Rc<str> = field_of_19(v37.clone(), v58.clone());
                        let mut v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("decision"); } LIT.with(|lit| lit.clone()) };
                        let mut v61: Rc<str> = field_of_19(v37.clone(), v60.clone());
                        let mut v62: US4 = plan_checked_22(v48.clone(), v50.clone(), v52.clone(), v55, v57.clone(), v59.clone(), v61.clone());
                        match &v62 {
                            US4::US4_PlanAccepted => {
                                let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("amendments"); } LIT.with(|lit| lit.clone()) };
                                let mut v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                                let mut v69: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v68, &*v67, &*v35, &*v35].concat());
                                let mut v70: Rc<str> = joined_7(v1.clone(), v69.clone());
                                let mut v71: Rc<str> = text_appended_26(v70.clone(), v36.clone());
                                let mut v72: bool = v71.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                if v72 {
                                    let mut v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("amend: "); } LIT.with(|lit| lit.clone()) };
                                    let mut v74: Rc<str> = std::rc::Rc::<str>::from([&*v73, &*v71, &*v35, &*v35, &*v35].concat());
                                    failed_1(v74.clone())
                                } else {
                                    let mut v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie experiment amend ok id="); } LIT.with(|lit| lit.clone()) };
                                    let mut v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" field="); } LIT.with(|lit| lit.clone()) };
                                    let mut v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" value="); } LIT.with(|lit| lit.clone()) };
                                    let mut v79: Rc<str> = option_value_9(v29.clone());
                                    let mut v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" amended="); } LIT.with(|lit| lit.clone()) };
                                    let mut v81: Rc<str> = std::rc::Rc::<str>::from([&*v78, &*v79, &*v80, &*v23, &*v35].concat());
                                    let mut v82: Rc<str> = std::rc::Rc::<str>::from([&*v76, &*v3, &*v77, &*v12, &*v81].concat());
                                    stdout_line_27(v82.clone());
                                    0i32
                                }
                            }
                            US4::US4_PlanRejected(v63) => {
                                let mut v63: Rc<str> = v63.clone();
                                let mut v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("amend: the amended plan is incomplete: "); } LIT.with(|lit| lit.clone()) };
                                let mut v65: Rc<str> = std::rc::Rc::<str>::from([&*v64, &*v63, &*v35, &*v35, &*v35].concat());
                                failed_1(v65.clone())
                            }
                        }
                    }
                    US2::US2_TestUnknown => {
                        let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("amend: --test must be "); } LIT.with(|lit| lit.clone()) };
                        let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch|one-sample|binomial|wilson|median|binomial-band"); } LIT.with(|lit| lit.clone()) };
                        let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", received '"); } LIT.with(|lit| lit.clone()) };
                        let mut v44: Rc<str> = field_of_19(v37.clone(), v38.clone());
                        let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
                        let mut v46: Rc<str> = std::rc::Rc::<str>::from([&*v41, &*v42, &*v43, &*v44, &*v45].concat());
                        failed_1(v46.clone())
                    }
                }
            }
            US1::US1_AmendRefused(v17) => {
                let mut v17: Rc<str> = v17.clone();
                let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("amend: "); } LIT.with(|lit| lit.clone()) };
                let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v20: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v17, &*v19, &*v19, &*v19].concat());
                failed_1(v20.clone())
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        amended_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn addended_28() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: Rc<str> = argument_3(v0);
    let mut v2: i32 = 4i32;
    let mut v3: Rc<str> = argument_3(v2);
    let mut v4: i32 = 5i32;
    let mut v5: Rc<str> = argument_3(v4);
    let mut v6: Rc<str> = plan_text_6(v1.clone(), v3.clone());
    let mut v7: bool = v6.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v7 {
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("addendum: no registration for "); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" in "); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v3, &*v9, &*v1, &*v10].concat());
        failed_1(v11.clone())
    } else {
        let mut v13: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v13 {
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("addendum: the text is the post-hoc note, kept apart from the verdict"); } LIT.with(|lit| lit.clone()) };
            failed_1(v14.clone())
        } else {
            let mut v16: u64 = now_ms_14();
            let mut v17: Rc<str> = utc_stamp_15(v16);
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("addenda"); } LIT.with(|lit| lit.clone()) };
            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v21: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v19, &*v18, &*v20, &*v20].concat());
            let mut v22: Rc<str> = joined_7(v1.clone(), v21.clone());
            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note"); } LIT.with(|lit| lit.clone()) };
            let mut v25: Rc<str> = field_line_17(v24.clone(), v5.clone());
            let mut v26: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v23, &*v25, &*v20, &*v20].concat());
            let mut v27: Rc<str> = text_appended_26(v22.clone(), v26.clone());
            let mut v28: bool = v27.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v28 {
                let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("addendum: "); } LIT.with(|lit| lit.clone()) };
                let mut v30: Rc<str> = std::rc::Rc::<str>::from([&*v29, &*v27, &*v20, &*v20, &*v20].concat());
                failed_1(v30.clone())
            } else {
                let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie experiment addendum ok id="); } LIT.with(|lit| lit.clone()) };
                let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" at="); } LIT.with(|lit| lit.clone()) };
                let mut v34: bool = looked_at_10(v1.clone(), v3.clone());
                let mut v37: Rc<str> = if v34 {
                    let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" after_look=yes"); } LIT.with(|lit| lit.clone()) };
                    v35.clone()
                } else {
                    let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" after_look=no"); } LIT.with(|lit| lit.clone()) };
                    v36.clone()
                };
                let mut v38: Rc<str> = std::rc::Rc::<str>::from([&*v32, &*v3, &*v33, &*v17, &*v37].concat());
                stdout_line_27(v38.clone());
                0i32
            }
        }
    }
}
fn closure1() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        addended_28()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_experiment_amend_run() -> i32 {
    closure0()()
}
pub fn eoie_experiment_addendum_run() -> i32 {
    closure1()()
}
