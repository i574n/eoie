#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Unestimated,
    US0_EstimateMinutes(i32),
}
#[derive(Clone)]
enum US2 {
    US2_LoadLight,
    US2_LoadMedium,
    US2_LoadHeavy,
}
#[derive(Clone)]
enum US1 {
    US1_LoadUnset,
    US1_PlannedLoad(US2),
}
#[derive(Clone)]
enum US4 {
    US4_AssigneeAgent,
    US4_AssigneePair,
    US4_AssigneeUser,
}
#[derive(Clone)]
enum US3 {
    US3_Unassigned,
    US3_PlannedAssignee(US4),
}
#[derive(Clone)]
enum US5 {
    US5_ScanPlain,
    US5_ScanQuoted,
    US5_ScanEscaped,
}
#[derive(Clone)]
enum US6 {
    US6_Valid(i32),
    US6_Invalid,
}
#[derive(Clone)]
enum US7 {
    US7_Valid(US1),
    US7_Invalid,
}
#[derive(Clone)]
enum US8 {
    US8_Valid(US2),
    US8_Invalid,
}
#[derive(Clone)]
enum US9 {
    US9_Valid(US3),
    US9_Invalid,
}
#[derive(Clone)]
enum US10 {
    US10_Valid(US4),
    US10_Invalid,
}
fn planning_header_0() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("EST"); } LIT.with(|lit| lit.clone()) };
    let mut v1: Rc<str> = { let text = v0; let filler = (7i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("LOAD"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { let text = v2; let filler = (7i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("WHO"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { let text = v4; let filler = (6i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v3, &*v5, &*v6, &*v6].concat());
    v7.clone()
}
fn closure0() -> Rc<dyn Fn() -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> Rc<str>> = Rc::new(move || -> Rc<str> {
        planning_header_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn declared_from_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
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
fn field_end_6(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US5) -> i32 {
    loop {
        let mut v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let mut v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = match &v3 {
                US5::US5_ScanPlain => {
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
                let mut v25: US5 = match &v3 {
                    US5::US5_ScanEscaped => {
                        US5::US5_ScanQuoted
                    }
                    US5::US5_ScanPlain => {
                        let mut v19: bool = v5 == 34i32;
                        if v19 {
                            US5::US5_ScanQuoted
                        } else {
                            US5::US5_ScanPlain
                        }
                    }
                    US5::US5_ScanQuoted => {
                        let mut v12: bool = v5 == 92i32;
                        if v12 {
                            US5::US5_ScanEscaped
                        } else {
                            let mut v14: bool = v5 == 34i32;
                            if v14 {
                                US5::US5_ScanPlain
                            } else {
                                US5::US5_ScanQuoted
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
fn skip_spaces_7(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn field_start_5(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let mut v5: US5 = US5::US5_ScanPlain;
            let mut v6: i32 = field_end_6(v0.clone(), v1, v2, v5.clone());
            let mut v7: i32 = v6.wrapping_add(1i32);
            let mut v8: i32 = skip_spaces_7(v0.clone(), v7);
            let mut v9: i32 = v3.wrapping_sub(1i32);
            (v0, v1, v2, v3) = (v0.clone(), v8, v2, v9);
            continue;
        }
    }
}
fn decimal_digits_value_9(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
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
fn decimal_value_8(mut v0: Rc<str>) -> US6 {
    let mut v1: i32 = v0.len() as i32;
    let mut v2: bool = v1 <= 0i32;
    if v2 {
        US6::US6_Invalid
    } else {
        let mut v4: bool = v1 <= 6i32;
        if v4 {
            let mut v5: i32 = 0i32;
            let mut v6: i32 = 0i32;
            let mut v7: i32 = decimal_digits_value_9(v0.clone(), v5, v1, v6);
            let mut v8: bool = v7 < 0i32;
            if v8 {
                US6::US6_Invalid
            } else {
                US6::US6_Valid(v7)
            }
        } else {
            US6::US6_Invalid
        }
    }
}
fn parse_planned_load_10(mut v0: Rc<str>) -> US7 {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.to_ascii_lowercase());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("none|-"); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v2.split("|").any(|item| item == &*v1);
    if v3 {
        let mut v4: US1 = US1::US1_LoadUnset;
        US7::US7_Valid(v4.clone())
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Light"); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = std::rc::Rc::<str>::from(v6.to_ascii_lowercase());
        let mut v8: bool = v1 == v7 ;
        let mut v12: US8 = if v8 {
            let mut v9: US2 = US2::US2_LoadLight;
            US8::US8_Valid(v9.clone())
        } else {
            US8::US8_Invalid
        };
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Medium"); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<str> = std::rc::Rc::<str>::from(v13.to_ascii_lowercase());
        let mut v15: bool = v1 == v14 ;
        let mut v18: US8 = if v15 {
            let mut v16: US2 = US2::US2_LoadMedium;
            US8::US8_Valid(v16.clone())
        } else {
            v12.clone()
        };
        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Heavy"); } LIT.with(|lit| lit.clone()) };
        let mut v20: Rc<str> = std::rc::Rc::<str>::from(v19.to_ascii_lowercase());
        let mut v21: bool = v1 == v20 ;
        let mut v24: US8 = if v21 {
            let mut v22: US2 = US2::US2_LoadHeavy;
            US8::US8_Valid(v22.clone())
        } else {
            v18.clone()
        };
        match &v24 {
            US8::US8_Invalid => {
                US7::US7_Invalid
            }
            US8::US8_Valid(v25) => {
                let mut v25: US2 = v25.clone();
                let mut v26: US1 = US1::US1_PlannedLoad(v25.clone());
                US7::US7_Valid(v26.clone())
            }
        }
    }
}
fn parse_planned_assignee_11(mut v0: Rc<str>) -> US9 {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.to_ascii_lowercase());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("none|-"); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v2.split("|").any(|item| item == &*v1);
    if v3 {
        let mut v4: US3 = US3::US3_Unassigned;
        US9::US9_Valid(v4.clone())
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Agent"); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = std::rc::Rc::<str>::from(v6.to_ascii_lowercase());
        let mut v8: bool = v1 == v7 ;
        let mut v12: US10 = if v8 {
            let mut v9: US4 = US4::US4_AssigneeAgent;
            US10::US10_Valid(v9.clone())
        } else {
            US10::US10_Invalid
        };
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Pair"); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<str> = std::rc::Rc::<str>::from(v13.to_ascii_lowercase());
        let mut v15: bool = v1 == v14 ;
        let mut v18: US10 = if v15 {
            let mut v16: US4 = US4::US4_AssigneePair;
            US10::US10_Valid(v16.clone())
        } else {
            v12.clone()
        };
        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("User"); } LIT.with(|lit| lit.clone()) };
        let mut v20: Rc<str> = std::rc::Rc::<str>::from(v19.to_ascii_lowercase());
        let mut v21: bool = v1 == v20 ;
        let mut v24: US10 = if v21 {
            let mut v22: US4 = US4::US4_AssigneeUser;
            US10::US10_Valid(v22.clone())
        } else {
            v18.clone()
        };
        match &v24 {
            US10::US10_Invalid => {
                US9::US9_Invalid
            }
            US10::US10_Valid(v25) => {
                let mut v25: US4 = v25.clone();
                let mut v26: US3 = US3::US3_PlannedAssignee(v25.clone());
                US9::US9_Valid(v26.clone())
            }
        }
    }
}
fn planning_at_4(mut v0: Rc<str>, mut v1: i32) -> (US0, US1, US3) {
    let mut v2: i32 = v1 + 10i32;
    let mut v3: i32 = v0.len() as i32;
    let mut v4: i32 = 1i32;
    let mut v5: i32 = field_start_5(v0.clone(), v2, v3, v4);
    let mut v6: US5 = US5::US5_ScanPlain;
    let mut v7: i32 = field_end_6(v0.clone(), v5, v3, v6.clone());
    let mut v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v7 as usize).unwrap_or(""));
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Minutes "); } LIT.with(|lit| lit.clone()) };
    let mut v10: bool = v8.starts_with(&*v9);
    let mut v23: US0 = if v10 {
        let mut v11: i32 = v8.len() as i32;
        let mut v12: Rc<str> = std::rc::Rc::<str>::from(v8.get(8i32 as usize..v11 as usize).unwrap_or(""));
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v15: Rc<str> = std::rc::Rc::<str>::from(v12.replace(&*v13, &v14));
        let mut v16: US6 = decimal_value_8(v15.clone());
        match &v16 {
            US6::US6_Invalid => {
                US0::US0_Unestimated
            }
            US6::US6_Valid(v17) => {
                let mut v17: i32 = *v17;
                US0::US0_EstimateMinutes(v17)
            }
        }
    } else {
        US0::US0_Unestimated
    };
    let mut v24: i32 = 2i32;
    let mut v25: i32 = field_start_5(v0.clone(), v2, v3, v24);
    let mut v26: US5 = US5::US5_ScanPlain;
    let mut v27: i32 = field_end_6(v0.clone(), v25, v3, v26.clone());
    let mut v28: Rc<str> = std::rc::Rc::<str>::from(v0.get(v25 as usize..v27 as usize).unwrap_or(""));
    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Load "); } LIT.with(|lit| lit.clone()) };
    let mut v30: bool = v28.starts_with(&*v29);
    let mut v39: US1 = if v30 {
        let mut v31: i32 = v28.len() as i32;
        let mut v32: Rc<str> = std::rc::Rc::<str>::from(v28.get(5i32 as usize..v31 as usize).unwrap_or(""));
        let mut v33: US7 = parse_planned_load_10(v32.clone());
        match &v33 {
            US7::US7_Invalid => {
                US1::US1_LoadUnset
            }
            US7::US7_Valid(v34) => {
                let mut v34: US1 = v34.clone();
                v34.clone()
            }
        }
    } else {
        US1::US1_LoadUnset
    };
    let mut v40: i32 = 3i32;
    let mut v41: i32 = field_start_5(v0.clone(), v2, v3, v40);
    let mut v42: US5 = US5::US5_ScanPlain;
    let mut v43: i32 = field_end_6(v0.clone(), v41, v3, v42.clone());
    let mut v44: Rc<str> = std::rc::Rc::<str>::from(v0.get(v41 as usize..v43 as usize).unwrap_or(""));
    let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Assigned "); } LIT.with(|lit| lit.clone()) };
    let mut v46: bool = v44.starts_with(&*v45);
    let mut v55: US3 = if v46 {
        let mut v47: i32 = v44.len() as i32;
        let mut v48: Rc<str> = std::rc::Rc::<str>::from(v44.get(9i32 as usize..v47 as usize).unwrap_or(""));
        let mut v49: US9 = parse_planned_assignee_11(v48.clone());
        match &v49 {
            US9::US9_Invalid => {
                US3::US3_Unassigned
            }
            US9::US9_Valid(v50) => {
                let mut v50: US3 = v50.clone();
                v50.clone()
            }
        }
    } else {
        US3::US3_Unassigned
    };
    (v23.clone(), v39.clone(), v55.clone())
}
fn planning_of_2(mut v0: Rc<str>, mut v1: Rc<str>) -> (US0, US1, US3) {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planning (\""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v1, &*v3, &*v4, &*v4].concat());
    let mut v6: i32 = 0i32;
    let mut v7: i32 = declared_from_3(v0.clone(), v5.clone(), v6);
    let mut v8: bool = v7 < 0i32;
    let (mut v15, mut v16, mut v17): (US0, US1, US3) = if v8 {
        let mut v9: US0 = US0::US0_Unestimated;
        let mut v10: US1 = US1::US1_LoadUnset;
        let mut v11: US3 = US3::US3_Unassigned;
        (v9.clone(), v10.clone(), v11.clone())
    } else {
        let (mut v12, mut v13, mut v14): (US0, US1, US3) = planning_at_4(v0.clone(), v7);
        (v12.clone(), v13.clone(), v14.clone())
    };
    (v15.clone(), v16.clone(), v17.clone())
}
fn planning_columns_1(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let (mut v2, mut v3, mut v4): (US0, US1, US3) = planning_of_2(v0.clone(), v1.clone());
    let mut v12: Rc<str> = match &v2 {
        US0::US0_EstimateMinutes(v6) => {
            let mut v6: i32 = *v6;
            let mut v7: Rc<str> = std::rc::Rc::<str>::from((v6).to_string());
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v10: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v8, &*v9, &*v9, &*v9].concat());
            v10.clone()
        }
        US0::US0_Unestimated => {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            v5.clone()
        }
    };
    let mut v13: Rc<str> = { let text = v12; let filler = (7i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
    let mut v22: Rc<str> = match &v3 {
        US1::US1_LoadUnset => {
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            v14.clone()
        }
        US1::US1_PlannedLoad(v15) => {
            let mut v15: US2 = v15.clone();
            let mut v19: Rc<str> = match &v15 {
                US2::US2_LoadHeavy => {
                    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Heavy"); } LIT.with(|lit| lit.clone()) };
                    v18.clone()
                }
                US2::US2_LoadLight => {
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Light"); } LIT.with(|lit| lit.clone()) };
                    v16.clone()
                }
                US2::US2_LoadMedium => {
                    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Medium"); } LIT.with(|lit| lit.clone()) };
                    v17.clone()
                }
            };
            let mut v20: Rc<str> = std::rc::Rc::<str>::from(v19.to_ascii_lowercase());
            v20.clone()
        }
    };
    let mut v23: Rc<str> = { let text = v22; let filler = (7i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
    let mut v32: Rc<str> = match &v4 {
        US3::US3_PlannedAssignee(v25) => {
            let mut v25: US4 = v25.clone();
            let mut v29: Rc<str> = match &v25 {
                US4::US4_AssigneeAgent => {
                    let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Agent"); } LIT.with(|lit| lit.clone()) };
                    v26.clone()
                }
                US4::US4_AssigneePair => {
                    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Pair"); } LIT.with(|lit| lit.clone()) };
                    v27.clone()
                }
                US4::US4_AssigneeUser => {
                    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("User"); } LIT.with(|lit| lit.clone()) };
                    v28.clone()
                }
            };
            let mut v30: Rc<str> = std::rc::Rc::<str>::from(v29.to_ascii_lowercase());
            v30.clone()
        }
        US3::US3_Unassigned => {
            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            v24.clone()
        }
    };
    let mut v33: Rc<str> = { let text = v32; let filler = (6i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v35: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v23, &*v33, &*v34, &*v34].concat());
    v35.clone()
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        planning_columns_1(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn planning_json_fields_13(mut v0: US0, mut v1: US1, mut v2: US3) -> Rc<str> {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"estimate_minutes\":"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = match &v0 {
        US0::US0_EstimateMinutes(v5) => {
            let mut v5: i32 = *v5;
            let mut v6: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
            v6.clone()
        }
        US0::US0_Unestimated => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("null"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
    };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"load\":"); } LIT.with(|lit| lit.clone()) };
    let mut v21: Rc<str> = match &v1 {
        US1::US1_LoadUnset => {
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("null"); } LIT.with(|lit| lit.clone()) };
            v10.clone()
        }
        US1::US1_PlannedLoad(v11) => {
            let mut v11: US2 = v11.clone();
            let mut v15: Rc<str> = match &v11 {
                US2::US2_LoadHeavy => {
                    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Heavy"); } LIT.with(|lit| lit.clone()) };
                    v14.clone()
                }
                US2::US2_LoadLight => {
                    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Light"); } LIT.with(|lit| lit.clone()) };
                    v12.clone()
                }
                US2::US2_LoadMedium => {
                    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Medium"); } LIT.with(|lit| lit.clone()) };
                    v13.clone()
                }
            };
            let mut v16: Rc<str> = std::rc::Rc::<str>::from(v15.to_ascii_lowercase());
            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v19: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v16, &*v17, &*v18, &*v18].concat());
            v19.clone()
        }
    };
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v8, &*v9, &*v21, &*v22].concat());
    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"assignee\":"); } LIT.with(|lit| lit.clone()) };
    let mut v35: Rc<str> = match &v2 {
        US3::US3_PlannedAssignee(v26) => {
            let mut v26: US4 = v26.clone();
            let mut v30: Rc<str> = match &v26 {
                US4::US4_AssigneeAgent => {
                    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Agent"); } LIT.with(|lit| lit.clone()) };
                    v27.clone()
                }
                US4::US4_AssigneePair => {
                    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Pair"); } LIT.with(|lit| lit.clone()) };
                    v28.clone()
                }
                US4::US4_AssigneeUser => {
                    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("User"); } LIT.with(|lit| lit.clone()) };
                    v29.clone()
                }
            };
            let mut v31: Rc<str> = std::rc::Rc::<str>::from(v30.to_ascii_lowercase());
            let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let mut v33: Rc<str> = std::rc::Rc::<str>::from([&*v32, &*v31, &*v32, &*v22, &*v22].concat());
            v33.clone()
        }
        US3::US3_Unassigned => {
            let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("null"); } LIT.with(|lit| lit.clone()) };
            v25.clone()
        }
    };
    let mut v36: Rc<str> = std::rc::Rc::<str>::from([&*v23, &*v24, &*v35, &*v22, &*v22].concat());
    v36.clone()
}
fn planning_json_12(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let (mut v2, mut v3, mut v4): (US0, US1, US3) = planning_of_2(v0.clone(), v1.clone());
    planning_json_fields_13(v2.clone(), v3.clone(), v4.clone())
}
fn closure2() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        planning_json_12(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn planning_summary_15(mut v0: US0, mut v1: US1, mut v2: US3) -> Rc<str> {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("estimate="); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = match &v0 {
        US0::US0_EstimateMinutes(v5) => {
            let mut v5: i32 = *v5;
            let mut v6: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v9: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v7, &*v8, &*v8, &*v8].concat());
            v9.clone()
        }
        US0::US0_Unestimated => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
    };
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" load="); } LIT.with(|lit| lit.clone()) };
    let mut v21: Rc<str> = match &v1 {
        US1::US1_LoadUnset => {
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            v13.clone()
        }
        US1::US1_PlannedLoad(v14) => {
            let mut v14: US2 = v14.clone();
            let mut v18: Rc<str> = match &v14 {
                US2::US2_LoadHeavy => {
                    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Heavy"); } LIT.with(|lit| lit.clone()) };
                    v17.clone()
                }
                US2::US2_LoadLight => {
                    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Light"); } LIT.with(|lit| lit.clone()) };
                    v15.clone()
                }
                US2::US2_LoadMedium => {
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Medium"); } LIT.with(|lit| lit.clone()) };
                    v16.clone()
                }
            };
            let mut v19: Rc<str> = std::rc::Rc::<str>::from(v18.to_ascii_lowercase());
            v19.clone()
        }
    };
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v11, &*v12, &*v21, &*v22].concat());
    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" assignee="); } LIT.with(|lit| lit.clone()) };
    let mut v33: Rc<str> = match &v2 {
        US3::US3_PlannedAssignee(v26) => {
            let mut v26: US4 = v26.clone();
            let mut v30: Rc<str> = match &v26 {
                US4::US4_AssigneeAgent => {
                    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Agent"); } LIT.with(|lit| lit.clone()) };
                    v27.clone()
                }
                US4::US4_AssigneePair => {
                    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Pair"); } LIT.with(|lit| lit.clone()) };
                    v28.clone()
                }
                US4::US4_AssigneeUser => {
                    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("User"); } LIT.with(|lit| lit.clone()) };
                    v29.clone()
                }
            };
            let mut v31: Rc<str> = std::rc::Rc::<str>::from(v30.to_ascii_lowercase());
            v31.clone()
        }
        US3::US3_Unassigned => {
            let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            v25.clone()
        }
    };
    let mut v34: Rc<str> = std::rc::Rc::<str>::from([&*v23, &*v24, &*v33, &*v22, &*v22].concat());
    v34.clone()
}
fn planning_summary_of_14(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let (mut v2, mut v3, mut v4): (US0, US1, US3) = planning_of_2(v0.clone(), v1.clone());
    planning_summary_15(v2.clone(), v3.clone(), v4.clone())
}
fn closure3() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        planning_summary_of_14(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_planning_header() -> Rc<str> {
    closure0()()
}
pub fn eoie_agile_planning_columns(v0: &str, v1: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn eoie_agile_planning_json(v0: &str, v1: &str) -> Rc<str> {
    closure2()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn eoie_agile_planning_summary(v0: &str, v1: &str) -> Rc<str> {
    closure3()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
