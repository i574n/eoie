#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_EveryTask,
    US0_StatusBucket(Rc<str>),
    US0_LoadBucket(i32),
    US0_AssigneeBucket(i32),
    US0_EpicBucket(Rc<str>),
    US0_UnknownBucket,
}
#[derive(Clone)]
enum US3 {
    US3_LoadLight,
    US3_LoadMedium,
    US3_LoadHeavy,
}
#[derive(Clone)]
enum US2 {
    US2_LoadUnset,
    US2_PlannedLoad(US3),
}
#[derive(Clone)]
enum US1 {
    US1_Valid(US2),
    US1_Invalid,
}
#[derive(Clone)]
enum US4 {
    US4_Valid(US3),
    US4_Invalid,
}
#[derive(Clone)]
enum US7 {
    US7_AssigneeAgent,
    US7_AssigneePair,
    US7_AssigneeUser,
}
#[derive(Clone)]
enum US6 {
    US6_Unassigned,
    US6_PlannedAssignee(US7),
}
#[derive(Clone)]
enum US5 {
    US5_Valid(US6),
    US5_Invalid,
}
#[derive(Clone)]
enum US8 {
    US8_Valid(US7),
    US8_Invalid,
}
#[derive(Clone)]
enum US9 {
    US9_Unestimated,
    US9_EstimateMinutes(i32),
}
#[derive(Clone)]
enum US10 {
    US10_ScanPlain,
    US10_ScanQuoted,
    US10_ScanEscaped,
}
#[derive(Clone)]
enum US11 {
    US11_Valid(i32),
    US11_Invalid,
}
fn parse_planned_load_2(mut v0: Rc<str>) -> US1 {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.to_ascii_lowercase());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("none|-"); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v2.split("|").any(|item| item == &*v1);
    if v3 {
        let mut v4: US2 = US2::US2_LoadUnset;
        US1::US1_Valid(v4.clone())
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Light"); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = std::rc::Rc::<str>::from(v6.to_ascii_lowercase());
        let mut v8: bool = v1 == v7 ;
        let mut v12: US4 = if v8 {
            let mut v9: US3 = US3::US3_LoadLight;
            US4::US4_Valid(v9.clone())
        } else {
            US4::US4_Invalid
        };
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Medium"); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<str> = std::rc::Rc::<str>::from(v13.to_ascii_lowercase());
        let mut v15: bool = v1 == v14 ;
        let mut v18: US4 = if v15 {
            let mut v16: US3 = US3::US3_LoadMedium;
            US4::US4_Valid(v16.clone())
        } else {
            v12.clone()
        };
        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Heavy"); } LIT.with(|lit| lit.clone()) };
        let mut v20: Rc<str> = std::rc::Rc::<str>::from(v19.to_ascii_lowercase());
        let mut v21: bool = v1 == v20 ;
        let mut v24: US4 = if v21 {
            let mut v22: US3 = US3::US3_LoadHeavy;
            US4::US4_Valid(v22.clone())
        } else {
            v18.clone()
        };
        match &v24 {
            US4::US4_Invalid => {
                US1::US1_Invalid
            }
            US4::US4_Valid(v25) => {
                let mut v25: US3 = v25.clone();
                let mut v26: US2 = US2::US2_PlannedLoad(v25.clone());
                US1::US1_Valid(v26.clone())
            }
        }
    }
}
fn parse_planned_assignee_3(mut v0: Rc<str>) -> US5 {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.to_ascii_lowercase());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("none|-"); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v2.split("|").any(|item| item == &*v1);
    if v3 {
        let mut v4: US6 = US6::US6_Unassigned;
        US5::US5_Valid(v4.clone())
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Agent"); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = std::rc::Rc::<str>::from(v6.to_ascii_lowercase());
        let mut v8: bool = v1 == v7 ;
        let mut v12: US8 = if v8 {
            let mut v9: US7 = US7::US7_AssigneeAgent;
            US8::US8_Valid(v9.clone())
        } else {
            US8::US8_Invalid
        };
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Pair"); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<str> = std::rc::Rc::<str>::from(v13.to_ascii_lowercase());
        let mut v15: bool = v1 == v14 ;
        let mut v18: US8 = if v15 {
            let mut v16: US7 = US7::US7_AssigneePair;
            US8::US8_Valid(v16.clone())
        } else {
            v12.clone()
        };
        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("User"); } LIT.with(|lit| lit.clone()) };
        let mut v20: Rc<str> = std::rc::Rc::<str>::from(v19.to_ascii_lowercase());
        let mut v21: bool = v1 == v20 ;
        let mut v24: US8 = if v21 {
            let mut v22: US7 = US7::US7_AssigneeUser;
            US8::US8_Valid(v22.clone())
        } else {
            v18.clone()
        };
        match &v24 {
            US8::US8_Invalid => {
                US5::US5_Invalid
            }
            US8::US8_Valid(v25) => {
                let mut v25: US7 = v25.clone();
                let mut v26: US6 = US6::US6_PlannedAssignee(v25.clone());
                US5::US5_Valid(v26.clone())
            }
        }
    }
}
fn bucket_of_1(mut v0: Rc<str>, mut v1: Rc<str>) -> US0 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all|status|load|assignee|epic"); } LIT.with(|lit| lit.clone()) };
    let mut v3: u64 = v2.split("|").position(|item| item == &*v0).map(|index| index as u64).unwrap_or(u64::MAX);
    let mut v4: bool = 0u64 == v3;
    if v4 {
        US0::US0_EveryTask
    } else {
        let mut v6: bool = 1u64 == v3;
        if v6 {
            US0::US0_StatusBucket(v1.clone())
        } else {
            let mut v8: bool = 2u64 == v3;
            if v8 {
                let mut v9: US1 = parse_planned_load_2(v1.clone());
                match &v9 {
                    US1::US1_Invalid => {
                        US0::US0_UnknownBucket
                    }
                    US1::US1_Valid(v10) => {
                        let mut v10: US2 = v10.clone();
                        let mut v16: i32 = match &v10 {
                            US2::US2_LoadUnset => {
                                0i32
                            }
                            US2::US2_PlannedLoad(v11) => {
                                let mut v11: US3 = v11.clone();
                                match &v11 {
                                    US3::US3_LoadHeavy => {
                                        3i32
                                    }
                                    US3::US3_LoadLight => {
                                        1i32
                                    }
                                    US3::US3_LoadMedium => {
                                        2i32
                                    }
                                }
                            }
                        };
                        US0::US0_LoadBucket(v16)
                    }
                }
            } else {
                let mut v21: bool = 3u64 == v3;
                if v21 {
                    let mut v22: US5 = parse_planned_assignee_3(v1.clone());
                    match &v22 {
                        US5::US5_Invalid => {
                            US0::US0_UnknownBucket
                        }
                        US5::US5_Valid(v23) => {
                            let mut v23: US6 = v23.clone();
                            let mut v29: i32 = match &v23 {
                                US6::US6_PlannedAssignee(v24) => {
                                    let mut v24: US7 = v24.clone();
                                    match &v24 {
                                        US7::US7_AssigneeAgent => {
                                            1i32
                                        }
                                        US7::US7_AssigneePair => {
                                            2i32
                                        }
                                        US7::US7_AssigneeUser => {
                                            3i32
                                        }
                                    }
                                }
                                US6::US6_Unassigned => {
                                    0i32
                                }
                            };
                            US0::US0_AssigneeBucket(v29)
                        }
                    }
                } else {
                    let mut v34: bool = 4u64 == v3;
                    if v34 {
                        US0::US0_EpicBucket(v1.clone())
                    } else {
                        US0::US0_UnknownBucket
                    }
                }
            }
        }
    }
}
fn field_end_7(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US10) -> i32 {
    loop {
        let mut v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let mut v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = match &v3 {
                US10::US10_ScanPlain => {
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
                let mut v10: i32 = v1 + 1;
                let mut v25: US10 = match &v3 {
                    US10::US10_ScanEscaped => {
                        US10::US10_ScanQuoted
                    }
                    US10::US10_ScanPlain => {
                        let mut v19: bool = v5 == 34i32;
                        if v19 {
                            US10::US10_ScanQuoted
                        } else {
                            US10::US10_ScanPlain
                        }
                    }
                    US10::US10_ScanQuoted => {
                        let mut v12: bool = v5 == 92i32;
                        if v12 {
                            US10::US10_ScanEscaped
                        } else {
                            let mut v14: bool = v5 == 34i32;
                            if v14 {
                                US10::US10_ScanPlain
                            } else {
                                US10::US10_ScanQuoted
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
fn skip_spaces_8(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
        let mut v3: bool = v2 == 32i32;
        if v3 {
            let mut v4: i32 = v1 + 1;
            (v0, v1) = (v0.clone(), v4);
            continue;
        } else {
            return v1;
        }
    }
}
fn field_start_6(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let mut v5: US10 = US10::US10_ScanPlain;
            let mut v6: i32 = field_end_7(v0.clone(), v1, v2, v5.clone());
            let mut v7: i32 = v6 + 1;
            let mut v8: i32 = skip_spaces_8(v0.clone(), v7);
            let mut v9: i32 = v3 - 1;
            (v0, v1, v2, v3) = (v0.clone(), v8, v2, v9);
            continue;
        }
    }
}
fn declared_from_10(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        let mut v3: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v1)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
        let mut v4: bool = v3 < 0;
        if v4 {
            return v3;
        } else {
            let mut v5: i32 = v0.get(..v3 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let mut v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v3 as usize).unwrap_or(""));
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let mut v8: bool = v6.starts_with(&*v7);
            if v8 {
                return v3;
            } else {
                let mut v9: i32 = v3 + 1;
                (v0, v1, v2) = (v0.clone(), v1.clone(), v9);
                continue;
            }
        }
    }
}
fn decimal_digits_value_13(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: bool = v2 <= v1;
        if v4 {
            return v3;
        } else {
            let mut v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v6: bool = 48i32 <= v5;
            let mut v8: bool = if v6 {
                let mut v7: bool = v5 <= 57i32;
                v7
            } else {
                false
            };
            if v8 {
                let mut v9: i32 = v1 + 1;
                let mut v10: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let mut v11: i32 = v3 * 10 + v10 - 48;
                (v0, v1, v2, v3) = (v0.clone(), v9, v2, v11);
                continue;
            } else {
                return -1i32;
            }
        }
    }
}
fn decimal_value_12(mut v0: Rc<str>) -> US11 {
    let mut v1: i32 = v0.len() as i32;
    let mut v2: bool = v1 <= 0i32;
    if v2 {
        US11::US11_Invalid
    } else {
        let mut v4: bool = v1 <= 6i32;
        if v4 {
            let mut v5: i32 = 0i32;
            let mut v6: i32 = 0i32;
            let mut v7: i32 = decimal_digits_value_13(v0.clone(), v5, v1, v6);
            let mut v8: bool = v7 < 0;
            if v8 {
                US11::US11_Invalid
            } else {
                US11::US11_Valid(v7)
            }
        } else {
            US11::US11_Invalid
        }
    }
}
fn planning_at_11(mut v0: Rc<str>, mut v1: i32) -> (US9, US2, US6) {
    let mut v2: i32 = v1 + 10i32;
    let mut v3: i32 = v0.len() as i32;
    let mut v4: i32 = 1i32;
    let mut v5: i32 = field_start_6(v0.clone(), v2, v3, v4);
    let mut v6: US10 = US10::US10_ScanPlain;
    let mut v7: i32 = field_end_7(v0.clone(), v5, v3, v6.clone());
    let mut v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v7 as usize).unwrap_or(""));
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Minutes "); } LIT.with(|lit| lit.clone()) };
    let mut v10: bool = v8.starts_with(&*v9);
    let mut v23: US9 = if v10 {
        let mut v11: i32 = v8.len() as i32;
        let mut v12: Rc<str> = std::rc::Rc::<str>::from(v8.get(8i32 as usize..v11 as usize).unwrap_or(""));
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v15: Rc<str> = std::rc::Rc::<str>::from(v12.replace(&*v13, &v14));
        let mut v16: US11 = decimal_value_12(v15.clone());
        match &v16 {
            US11::US11_Invalid => {
                US9::US9_Unestimated
            }
            US11::US11_Valid(v17) => {
                let mut v17: i32 = *v17;
                US9::US9_EstimateMinutes(v17)
            }
        }
    } else {
        US9::US9_Unestimated
    };
    let mut v24: i32 = 2i32;
    let mut v25: i32 = field_start_6(v0.clone(), v2, v3, v24);
    let mut v26: US10 = US10::US10_ScanPlain;
    let mut v27: i32 = field_end_7(v0.clone(), v25, v3, v26.clone());
    let mut v28: Rc<str> = std::rc::Rc::<str>::from(v0.get(v25 as usize..v27 as usize).unwrap_or(""));
    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Load "); } LIT.with(|lit| lit.clone()) };
    let mut v30: bool = v28.starts_with(&*v29);
    let mut v39: US2 = if v30 {
        let mut v31: i32 = v28.len() as i32;
        let mut v32: Rc<str> = std::rc::Rc::<str>::from(v28.get(5i32 as usize..v31 as usize).unwrap_or(""));
        let mut v33: US1 = parse_planned_load_2(v32.clone());
        match &v33 {
            US1::US1_Invalid => {
                US2::US2_LoadUnset
            }
            US1::US1_Valid(v34) => {
                let mut v34: US2 = v34.clone();
                v34.clone()
            }
        }
    } else {
        US2::US2_LoadUnset
    };
    let mut v40: i32 = 3i32;
    let mut v41: i32 = field_start_6(v0.clone(), v2, v3, v40);
    let mut v42: US10 = US10::US10_ScanPlain;
    let mut v43: i32 = field_end_7(v0.clone(), v41, v3, v42.clone());
    let mut v44: Rc<str> = std::rc::Rc::<str>::from(v0.get(v41 as usize..v43 as usize).unwrap_or(""));
    let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Assigned "); } LIT.with(|lit| lit.clone()) };
    let mut v46: bool = v44.starts_with(&*v45);
    let mut v55: US6 = if v46 {
        let mut v47: i32 = v44.len() as i32;
        let mut v48: Rc<str> = std::rc::Rc::<str>::from(v44.get(9i32 as usize..v47 as usize).unwrap_or(""));
        let mut v49: US5 = parse_planned_assignee_3(v48.clone());
        match &v49 {
            US5::US5_Invalid => {
                US6::US6_Unassigned
            }
            US5::US5_Valid(v50) => {
                let mut v50: US6 = v50.clone();
                v50.clone()
            }
        }
    } else {
        US6::US6_Unassigned
    };
    (v23.clone(), v39.clone(), v55.clone())
}
fn planning_of_9(mut v0: Rc<str>, mut v1: Rc<str>) -> (US9, US2, US6) {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planning (\""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v1, &*v3, &*v4, &*v4].concat());
    let mut v6: i32 = 0i32;
    let mut v7: i32 = declared_from_10(v0.clone(), v5.clone(), v6);
    let mut v8: bool = v7 < 0;
    let (mut v15, mut v16, mut v17): (US9, US2, US6) = if v8 {
        let mut v9: US9 = US9::US9_Unestimated;
        let mut v10: US2 = US2::US2_LoadUnset;
        let mut v11: US6 = US6::US6_Unassigned;
        (v9.clone(), v10.clone(), v11.clone())
    } else {
        let (mut v12, mut v13, mut v14): (US9, US2, US6) = planning_at_11(v0.clone(), v7);
        (v12.clone(), v13.clone(), v14.clone())
    };
    (v15.clone(), v16.clone(), v17.clone())
}
fn rollup_task_at_5(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> (Rc<str>, Rc<str>, Rc<str>, US9, US2, US6) {
    let mut v3: i32 = 0i32;
    let mut v4: i32 = field_start_6(v0.clone(), v1, v2, v3);
    let mut v5: US10 = US10::US10_ScanPlain;
    let mut v6: i32 = field_end_7(v0.clone(), v4, v2, v5.clone());
    let mut v7: Rc<str> = std::rc::Rc::<str>::from(v0.get(v4 as usize..v6 as usize).unwrap_or(""));
    let mut v8: i32 = v7.len() as i32;
    let mut v9: i32 = v8 - 1;
    let mut v10: Rc<str> = std::rc::Rc::<str>::from(v7.get(1i32 as usize..v9 as usize).unwrap_or(""));
    let mut v11: i32 = 6i32;
    let mut v12: i32 = field_start_6(v0.clone(), v1, v2, v11);
    let mut v13: US10 = US10::US10_ScanPlain;
    let mut v14: i32 = field_end_7(v0.clone(), v12, v2, v13.clone());
    let mut v15: Rc<str> = std::rc::Rc::<str>::from(v0.get(v12 as usize..v14 as usize).unwrap_or(""));
    let mut v16: i32 = 7i32;
    let mut v17: i32 = field_start_6(v0.clone(), v1, v2, v16);
    let mut v18: US10 = US10::US10_ScanPlain;
    let mut v19: i32 = field_end_7(v0.clone(), v17, v2, v18.clone());
    let mut v20: Rc<str> = std::rc::Rc::<str>::from(v0.get(v17 as usize..v19 as usize).unwrap_or(""));
    let mut v21: i32 = v20.len() as i32;
    let mut v22: i32 = v21 - 1;
    let mut v23: Rc<str> = std::rc::Rc::<str>::from(v20.get(1i32 as usize..v22 as usize).unwrap_or(""));
    let (mut v24, mut v25, mut v26): (US9, US2, US6) = planning_of_9(v0.clone(), v10.clone());
    (v10.clone(), v15.clone(), v23.clone(), v24.clone(), v25.clone(), v26.clone())
}
fn bucket_holds_14(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: US9, mut v5: US2, mut v6: US6) -> bool {
    match &v0 {
        US0::US0_AssigneeBucket(v17) => {
            let mut v17: i32 = *v17;
            let mut v23: i32 = match &v6 {
                US6::US6_PlannedAssignee(v18) => {
                    let mut v18: US7 = v18.clone();
                    match &v18 {
                        US7::US7_AssigneeAgent => {
                            1i32
                        }
                        US7::US7_AssigneePair => {
                            2i32
                        }
                        US7::US7_AssigneeUser => {
                            3i32
                        }
                    }
                }
                US6::US6_Unassigned => {
                    0i32
                }
            };
            let mut v24: bool = v17 == v23;
            v24
        }
        US0::US0_EpicBucket(v25) => {
            let mut v25: Rc<str> = v25.clone();
            let mut v26: bool = v25 == v1 ;
            if v26 {
                true
            } else {
                let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
                let mut v29: Rc<str> = std::rc::Rc::<str>::from(v3.replace(&*v27, &v28));
                let mut v30: bool = v29.split("|").any(|item| item == &*v25);
                v30
            }
        }
        US0::US0_EveryTask => {
            true
        }
        US0::US0_LoadBucket(v9) => {
            let mut v9: i32 = *v9;
            let mut v15: i32 = match &v5 {
                US2::US2_LoadUnset => {
                    0i32
                }
                US2::US2_PlannedLoad(v10) => {
                    let mut v10: US3 = v10.clone();
                    match &v10 {
                        US3::US3_LoadHeavy => {
                            3i32
                        }
                        US3::US3_LoadLight => {
                            1i32
                        }
                        US3::US3_LoadMedium => {
                            2i32
                        }
                    }
                }
            };
            let mut v16: bool = v9 == v15;
            v16
        }
        US0::US0_StatusBucket(v7) => {
            let mut v7: Rc<str> = v7.clone();
            let mut v8: bool = v7 == v2 ;
            v8
        }
        US0::US0_UnknownBucket => {
            false
        }
    }
}
fn tally_with_15(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: US9, mut v9: US2, mut v10: US6) -> (i32, i32, i32, i32, i32) {
    let mut v13: i32 = match &v8 {
        US9::US9_EstimateMinutes(v11) => {
            let mut v11: i32 = *v11;
            v11
        }
        US9::US9_Unestimated => {
            0i32
        }
    };
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Done"); } LIT.with(|lit| lit.clone()) };
    let mut v15: bool = v6 == v14 ;
    let mut v16: bool = v15 == false;
    let mut v17: i32 = v0 + 1;
    let mut v18: i32 = if v16 {
        1i32
    } else {
        0i32
    };
    let mut v19: i32 = v1 + v18;
    let mut v22: bool = match &v8 {
        US9::US9_EstimateMinutes(v20) => {
            let mut v20: i32 = *v20;
            true
        }
        US9::US9_Unestimated => {
            false
        }
    };
    let mut v23: i32 = if v22 {
        1i32
    } else {
        0i32
    };
    let mut v24: i32 = v2 + v23;
    let mut v25: i32 = v3 + v13;
    let mut v26: i32 = if v16 {
        v13
    } else {
        0i32
    };
    let mut v27: i32 = v4 + v26;
    (v17, v19, v24, v25, v27)
}
fn tally_from_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US0, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32) -> (i32, i32, i32, i32, i32) {
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
    let mut v10: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v9)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
    let mut v11: bool = v10 < 0;
    let (mut v54, mut v55, mut v56, mut v57, mut v58): (i32, i32, i32, i32, i32) = if v11 {
        (v4, v5, v6, v7, v8)
    } else {
        let mut v12: i32 = v10 + 6i32;
        let mut v13: i32 = v0.get(..v10 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
        let mut v14: Rc<str> = std::rc::Rc::<str>::from(v0.get(v13 as usize..v10 as usize).unwrap_or(""));
        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
        let mut v16: bool = v14.starts_with(&*v15);
        let (mut v49, mut v50, mut v51, mut v52, mut v53): (i32, i32, i32, i32, i32) = if v16 {
            let (mut v17, mut v18, mut v19, mut v20, mut v21, mut v22): (Rc<str>, Rc<str>, Rc<str>, US9, US2, US6) = rollup_task_at_5(v0.clone(), v12, v2);
            let mut v23: bool = bucket_holds_14(v3.clone(), v17.clone(), v18.clone(), v19.clone(), v20.clone(), v21.clone(), v22.clone());
            let (mut v39, mut v40, mut v41, mut v42, mut v43): (i32, i32, i32, i32, i32) = if v23 {
                let (mut v24, mut v25, mut v26, mut v27, mut v28): (i32, i32, i32, i32, i32) = tally_with_15(v4, v5, v6, v7, v8, v17.clone(), v18.clone(), v19.clone(), v20.clone(), v21.clone(), v22.clone());
                let (mut v29, mut v30, mut v31, mut v32, mut v33): (i32, i32, i32, i32, i32) = tally_from_4(v0.clone(), v12, v2, v3.clone(), v24, v25, v26, v27, v28);
                (v29, v30, v31, v32, v33)
            } else {
                let (mut v34, mut v35, mut v36, mut v37, mut v38): (i32, i32, i32, i32, i32) = tally_from_4(v0.clone(), v12, v2, v3.clone(), v4, v5, v6, v7, v8);
                (v34, v35, v36, v37, v38)
            };
            (v39, v40, v41, v42, v43)
        } else {
            let (mut v44, mut v45, mut v46, mut v47, mut v48): (i32, i32, i32, i32, i32) = tally_from_4(v0.clone(), v12, v2, v3.clone(), v4, v5, v6, v7, v8);
            (v44, v45, v46, v47, v48)
        };
        (v49, v50, v51, v52, v53)
    };
    (v54, v55, v56, v57, v58)
}
fn planning_tally_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> (i32, i32, i32, i32, i32) {
    let mut v3: i32 = 0i32;
    let mut v4: i32 = v0.len() as i32;
    let mut v5: US0 = bucket_of_1(v1.clone(), v2.clone());
    let mut v6: i32 = 0i32;
    let mut v7: i32 = 0i32;
    let mut v8: i32 = 0i32;
    let mut v9: i32 = 0i32;
    let mut v10: i32 = 0i32;
    let (mut v11, mut v12, mut v13, mut v14, mut v15): (i32, i32, i32, i32, i32) = tally_from_4(v0.clone(), v3, v4, v5.clone(), v6, v7, v8, v9, v10);
    (v11, v12, v13, v14, v15)
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> (i32, i32, i32, i32, i32)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> (i32, i32, i32, i32, i32)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>| -> (i32, i32, i32, i32, i32) {
        planning_tally_0(v0.clone(), v1.clone(), v2.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_planning_tally(v0: &str, v1: &str, v2: &str) -> (i32, i32, i32, i32, i32) {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2))
}
