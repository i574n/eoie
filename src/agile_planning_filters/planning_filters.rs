#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_AnyPlanning,
    US0_ExactPlanning(i32),
    US0_RejectedPlanningFilter,
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
fn load_filter_of_1(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0 == v1 ;
    if v2 {
        US0::US0_AnyPlanning
    } else {
        let mut v4: US1 = parse_planned_load_2(v0.clone());
        match &v4 {
            US1::US1_Invalid => {
                US0::US0_RejectedPlanningFilter
            }
            US1::US1_Valid(v5) => {
                let mut v5: US2 = v5.clone();
                let mut v11: i32 = match &v5 {
                    US2::US2_LoadUnset => {
                        0i32
                    }
                    US2::US2_PlannedLoad(v6) => {
                        let mut v6: US3 = v6.clone();
                        match &v6 {
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
                US0::US0_ExactPlanning(v11)
            }
        }
    }
}
fn parse_planned_assignee_4(mut v0: Rc<str>) -> US5 {
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
fn assignee_filter_of_3(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0 == v1 ;
    if v2 {
        US0::US0_AnyPlanning
    } else {
        let mut v4: US5 = parse_planned_assignee_4(v0.clone());
        match &v4 {
            US5::US5_Invalid => {
                US0::US0_RejectedPlanningFilter
            }
            US5::US5_Valid(v5) => {
                let mut v5: US6 = v5.clone();
                let mut v11: i32 = match &v5 {
                    US6::US6_PlannedAssignee(v6) => {
                        let mut v6: US7 = v6.clone();
                        match &v6 {
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
                US0::US0_ExactPlanning(v11)
            }
        }
    }
}
fn planning_filters_valid_0(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: US0 = load_filter_of_1(v0.clone());
    let mut v3: bool = match &v2 {
        US0::US0_RejectedPlanningFilter => {
            false
        }
        _ => {
            true
        }
    };
    if v3 {
        let mut v4: US0 = assignee_filter_of_3(v1.clone());
        match &v4 {
            US0::US0_RejectedPlanningFilter => {
                false
            }
            _ => {
                true
            }
        }
    } else {
        false
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> bool {
        planning_filters_valid_0(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn declared_from_7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        let mut v3: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v1)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
        let mut v4: bool = v3 < 0i32;
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
                let mut v9: i32 = v3.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1.clone(), v9);
                continue;
            }
        }
    }
}
fn field_end_10(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US10) -> i32 {
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
                let mut v10: i32 = v1.wrapping_add(1i32);
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
fn skip_spaces_11(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn field_start_9(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let mut v5: US10 = US10::US10_ScanPlain;
            let mut v6: i32 = field_end_10(v0.clone(), v1, v2, v5.clone());
            let mut v7: i32 = v6.wrapping_add(1i32);
            let mut v8: i32 = skip_spaces_11(v0.clone(), v7);
            let mut v9: i32 = v3.wrapping_sub(1i32);
            (v0, v1, v2, v3) = (v0.clone(), v8, v2, v9);
            continue;
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
                let mut v9: i32 = v1.wrapping_add(1i32);
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
            let mut v8: bool = v7 < 0i32;
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
fn planning_at_8(mut v0: Rc<str>, mut v1: i32) -> (US9, US2, US6) {
    let mut v2: i32 = v1 + 10i32;
    let mut v3: i32 = v0.len() as i32;
    let mut v4: i32 = 1i32;
    let mut v5: i32 = field_start_9(v0.clone(), v2, v3, v4);
    let mut v6: US10 = US10::US10_ScanPlain;
    let mut v7: i32 = field_end_10(v0.clone(), v5, v3, v6.clone());
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
    let mut v25: i32 = field_start_9(v0.clone(), v2, v3, v24);
    let mut v26: US10 = US10::US10_ScanPlain;
    let mut v27: i32 = field_end_10(v0.clone(), v25, v3, v26.clone());
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
    let mut v41: i32 = field_start_9(v0.clone(), v2, v3, v40);
    let mut v42: US10 = US10::US10_ScanPlain;
    let mut v43: i32 = field_end_10(v0.clone(), v41, v3, v42.clone());
    let mut v44: Rc<str> = std::rc::Rc::<str>::from(v0.get(v41 as usize..v43 as usize).unwrap_or(""));
    let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Assigned "); } LIT.with(|lit| lit.clone()) };
    let mut v46: bool = v44.starts_with(&*v45);
    let mut v55: US6 = if v46 {
        let mut v47: i32 = v44.len() as i32;
        let mut v48: Rc<str> = std::rc::Rc::<str>::from(v44.get(9i32 as usize..v47 as usize).unwrap_or(""));
        let mut v49: US5 = parse_planned_assignee_4(v48.clone());
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
fn planning_of_6(mut v0: Rc<str>, mut v1: Rc<str>) -> (US9, US2, US6) {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planning (\""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v1, &*v3, &*v4, &*v4].concat());
    let mut v6: i32 = 0i32;
    let mut v7: i32 = declared_from_7(v0.clone(), v5.clone(), v6);
    let mut v8: bool = v7 < 0i32;
    let (mut v15, mut v16, mut v17): (US9, US2, US6) = if v8 {
        let mut v9: US9 = US9::US9_Unestimated;
        let mut v10: US2 = US2::US2_LoadUnset;
        let mut v11: US6 = US6::US6_Unassigned;
        (v9.clone(), v10.clone(), v11.clone())
    } else {
        let (mut v12, mut v13, mut v14): (US9, US2, US6) = planning_at_8(v0.clone(), v7);
        (v12.clone(), v13.clone(), v14.clone())
    };
    (v15.clone(), v16.clone(), v17.clone())
}
fn planning_selected_5(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> bool {
    let (mut v4, mut v5, mut v6): (US9, US2, US6) = planning_of_6(v0.clone(), v1.clone());
    let mut v7: US0 = load_filter_of_1(v2.clone());
    let mut v13: i32 = match &v5 {
        US2::US2_LoadUnset => {
            0i32
        }
        US2::US2_PlannedLoad(v8) => {
            let mut v8: US3 = v8.clone();
            match &v8 {
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
    let mut v18: bool = match &v7 {
        US0::US0_AnyPlanning => {
            true
        }
        US0::US0_ExactPlanning(v14) => {
            let mut v14: i32 = *v14;
            let mut v15: bool = v14 == v13;
            v15
        }
        US0::US0_RejectedPlanningFilter => {
            false
        }
    };
    if v18 {
        let mut v19: US0 = assignee_filter_of_3(v3.clone());
        let mut v25: i32 = match &v6 {
            US6::US6_PlannedAssignee(v20) => {
                let mut v20: US7 = v20.clone();
                match &v20 {
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
        match &v19 {
            US0::US0_AnyPlanning => {
                true
            }
            US0::US0_ExactPlanning(v26) => {
                let mut v26: i32 = *v26;
                let mut v27: bool = v26 == v25;
                v27
            }
            US0::US0_RejectedPlanningFilter => {
                false
            }
        }
    } else {
        false
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>| -> bool {
        planning_selected_5(v0.clone(), v1.clone(), v2.clone(), v3.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_planning_filters_valid(v0: &str, v1: &str) -> bool {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn eoie_agile_planning_selected(v0: &str, v1: &str, v2: &str, v3: &str) -> bool {
    closure1()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3))
}
