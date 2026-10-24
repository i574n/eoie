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
    let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v3));
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("WHO"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { let text = v5; let filler = (6i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
    let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v6));
    v7.clone()
}
fn closure0() -> Rc<dyn Fn() -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> Rc<str>> = Rc::new(move || -> Rc<str> {
        planning_header_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn planning_columns_of_2(mut v0: US0, mut v1: US1, mut v2: US3) -> Rc<str> {
    let mut v9: Rc<str> = match &v0 {
        US0::US0_EstimateMinutes(v4) => {
            let mut v4: i32 = *v4;
            let mut v5: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
            let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v6));
            v7.clone()
        }
        US0::US0_Unestimated => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
    };
    let mut v10: Rc<str> = { let text = v9; let filler = (7i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
    let mut v19: Rc<str> = match &v1 {
        US1::US1_LoadUnset => {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US1::US1_PlannedLoad(v12) => {
            let mut v12: US2 = v12.clone();
            let mut v16: Rc<str> = match &v12 {
                US2::US2_LoadHeavy => {
                    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Heavy"); } LIT.with(|lit| lit.clone()) };
                    v15.clone()
                }
                US2::US2_LoadLight => {
                    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Light"); } LIT.with(|lit| lit.clone()) };
                    v13.clone()
                }
                US2::US2_LoadMedium => {
                    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Medium"); } LIT.with(|lit| lit.clone()) };
                    v14.clone()
                }
            };
            let mut v17: Rc<str> = std::rc::Rc::<str>::from(v16.to_ascii_lowercase());
            v17.clone()
        }
    };
    let mut v20: Rc<str> = { let text = v19; let filler = (7i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
    let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v20));
    let mut v30: Rc<str> = match &v2 {
        US3::US3_PlannedAssignee(v23) => {
            let mut v23: US4 = v23.clone();
            let mut v27: Rc<str> = match &v23 {
                US4::US4_AssigneeAgent => {
                    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Agent"); } LIT.with(|lit| lit.clone()) };
                    v24.clone()
                }
                US4::US4_AssigneePair => {
                    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Pair"); } LIT.with(|lit| lit.clone()) };
                    v25.clone()
                }
                US4::US4_AssigneeUser => {
                    let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("User"); } LIT.with(|lit| lit.clone()) };
                    v26.clone()
                }
            };
            let mut v28: Rc<str> = std::rc::Rc::<str>::from(v27.to_ascii_lowercase());
            v28.clone()
        }
        US3::US3_Unassigned => {
            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            v22.clone()
        }
    };
    let mut v31: Rc<str> = { let text = v30; let filler = (6i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
    let mut v32: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v31));
    v32.clone()
}
fn declared_from_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
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
fn field_end_7(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US5) -> i32 {
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
fn skip_spaces_8(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn field_start_6(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let mut v5: US5 = US5::US5_ScanPlain;
            let mut v6: i32 = field_end_7(v0.clone(), v1, v2, v5.clone());
            let mut v7: i32 = v6.wrapping_add(1i32);
            let mut v8: i32 = skip_spaces_8(v0.clone(), v7);
            let mut v9: i32 = v3.wrapping_sub(1i32);
            (v0, v1, v2, v3) = (v0.clone(), v8, v2, v9);
            continue;
        }
    }
}
fn decimal_digits_value_10(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
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
fn decimal_value_9(mut v0: Rc<str>) -> US6 {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: bool = v1 <= 0i32;
    if v2 {
        US6::US6_Invalid
    } else {
        let mut v4: bool = v1 <= 6i32;
        if v4 {
            let mut v5: i32 = 0i32;
            let mut v6: i32 = 0i32;
            let mut v7: i32 = decimal_digits_value_10(v0.clone(), v5, v1, v6);
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
fn parse_planned_load_11(mut v0: Rc<str>) -> US7 {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.to_ascii_lowercase());
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("none"); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = if v2 {
        true
    } else {
        let mut v3: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        v3
    };
    if v4 {
        let mut v5: US1 = US1::US1_LoadUnset;
        US7::US7_Valid(v5.clone())
    } else {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Light"); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = std::rc::Rc::<str>::from(v7.to_ascii_lowercase());
        let mut v9: bool = v1.clone() == v8.clone();
        let mut v13: US8 = if v9 {
            let mut v10: US2 = US2::US2_LoadLight;
            US8::US8_Valid(v10.clone())
        } else {
            US8::US8_Invalid
        };
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Medium"); } LIT.with(|lit| lit.clone()) };
        let mut v15: Rc<str> = std::rc::Rc::<str>::from(v14.to_ascii_lowercase());
        let mut v16: bool = v1.clone() == v15.clone();
        let mut v19: US8 = if v16 {
            let mut v17: US2 = US2::US2_LoadMedium;
            US8::US8_Valid(v17.clone())
        } else {
            v13.clone()
        };
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Heavy"); } LIT.with(|lit| lit.clone()) };
        let mut v21: Rc<str> = std::rc::Rc::<str>::from(v20.to_ascii_lowercase());
        let mut v22: bool = v1.clone() == v21.clone();
        let mut v25: US8 = if v22 {
            let mut v23: US2 = US2::US2_LoadHeavy;
            US8::US8_Valid(v23.clone())
        } else {
            v19.clone()
        };
        match &v25 {
            US8::US8_Invalid => {
                US7::US7_Invalid
            }
            US8::US8_Valid(v26) => {
                let mut v26: US2 = v26.clone();
                let mut v27: US1 = US1::US1_PlannedLoad(v26.clone());
                US7::US7_Valid(v27.clone())
            }
        }
    }
}
fn parse_planned_assignee_12(mut v0: Rc<str>) -> US9 {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.to_ascii_lowercase());
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("none"); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = if v2 {
        true
    } else {
        let mut v3: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        v3
    };
    if v4 {
        let mut v5: US3 = US3::US3_Unassigned;
        US9::US9_Valid(v5.clone())
    } else {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Agent"); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = std::rc::Rc::<str>::from(v7.to_ascii_lowercase());
        let mut v9: bool = v1.clone() == v8.clone();
        let mut v13: US10 = if v9 {
            let mut v10: US4 = US4::US4_AssigneeAgent;
            US10::US10_Valid(v10.clone())
        } else {
            US10::US10_Invalid
        };
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Pair"); } LIT.with(|lit| lit.clone()) };
        let mut v15: Rc<str> = std::rc::Rc::<str>::from(v14.to_ascii_lowercase());
        let mut v16: bool = v1.clone() == v15.clone();
        let mut v19: US10 = if v16 {
            let mut v17: US4 = US4::US4_AssigneePair;
            US10::US10_Valid(v17.clone())
        } else {
            v13.clone()
        };
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("User"); } LIT.with(|lit| lit.clone()) };
        let mut v21: Rc<str> = std::rc::Rc::<str>::from(v20.to_ascii_lowercase());
        let mut v22: bool = v1.clone() == v21.clone();
        let mut v25: US10 = if v22 {
            let mut v23: US4 = US4::US4_AssigneeUser;
            US10::US10_Valid(v23.clone())
        } else {
            v19.clone()
        };
        match &v25 {
            US10::US10_Invalid => {
                US9::US9_Invalid
            }
            US10::US10_Valid(v26) => {
                let mut v26: US4 = v26.clone();
                let mut v27: US3 = US3::US3_PlannedAssignee(v26.clone());
                US9::US9_Valid(v27.clone())
            }
        }
    }
}
fn planning_at_5(mut v0: Rc<str>, mut v1: i32) -> (US0, US1, US3) {
    let mut v2: i32 = v1 + 10i32;
    let mut v3: i32 = (v0.clone().len() as i32);
    let mut v4: i32 = 1i32;
    let mut v5: i32 = field_start_6(v0.clone(), v2, v3, v4);
    let mut v6: US5 = US5::US5_ScanPlain;
    let mut v7: i32 = field_end_7(v0.clone(), v5, v3, v6.clone());
    let mut v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v7 as usize).unwrap_or(""));
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Minutes "); } LIT.with(|lit| lit.clone()) };
    let mut v10: bool = v8.starts_with(&*v9);
    let mut v23: US0 = if v10 {
        let mut v11: i32 = (v8.clone().len() as i32);
        let mut v12: Rc<str> = std::rc::Rc::<str>::from(v8.get(8i32 as usize..v11 as usize).unwrap_or(""));
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v15: Rc<str> = Rc::<str>::from(v12.replace(&*v13, &*v14));
        let mut v16: US6 = decimal_value_9(v15.clone());
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
    let mut v25: i32 = field_start_6(v0.clone(), v2, v3, v24);
    let mut v26: US5 = US5::US5_ScanPlain;
    let mut v27: i32 = field_end_7(v0.clone(), v25, v3, v26.clone());
    let mut v28: Rc<str> = std::rc::Rc::<str>::from(v0.get(v25 as usize..v27 as usize).unwrap_or(""));
    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Load "); } LIT.with(|lit| lit.clone()) };
    let mut v30: bool = v28.starts_with(&*v29);
    let mut v39: US1 = if v30 {
        let mut v31: i32 = (v28.clone().len() as i32);
        let mut v32: Rc<str> = std::rc::Rc::<str>::from(v28.get(5i32 as usize..v31 as usize).unwrap_or(""));
        let mut v33: US7 = parse_planned_load_11(v32.clone());
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
    let mut v41: i32 = field_start_6(v0.clone(), v2, v3, v40);
    let mut v42: US5 = US5::US5_ScanPlain;
    let mut v43: i32 = field_end_7(v0.clone(), v41, v3, v42.clone());
    let mut v44: Rc<str> = std::rc::Rc::<str>::from(v0.get(v41 as usize..v43 as usize).unwrap_or(""));
    let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Assigned "); } LIT.with(|lit| lit.clone()) };
    let mut v46: bool = v44.starts_with(&*v45);
    let mut v55: US3 = if v46 {
        let mut v47: i32 = (v44.clone().len() as i32);
        let mut v48: Rc<str> = std::rc::Rc::<str>::from(v44.get(9i32 as usize..v47 as usize).unwrap_or(""));
        let mut v49: US9 = parse_planned_assignee_12(v48.clone());
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
fn planning_of_3(mut v0: Rc<str>, mut v1: Rc<str>) -> (US0, US1, US3) {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planning (\""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v4));
    let mut v6: i32 = 0i32;
    let mut v7: i32 = declared_from_4(v0.clone(), v5.clone(), v6);
    let mut v8: bool = v7 < 0i32;
    let (mut v15, mut v16, mut v17): (US0, US1, US3) = if v8 {
        let mut v9: US0 = US0::US0_Unestimated;
        let mut v10: US1 = US1::US1_LoadUnset;
        let mut v11: US3 = US3::US3_Unassigned;
        (v9.clone(), v10.clone(), v11.clone())
    } else {
        let (mut v12, mut v13, mut v14): (US0, US1, US3) = planning_at_5(v0.clone(), v7);
        (v12.clone(), v13.clone(), v14.clone())
    };
    (v15.clone(), v16.clone(), v17.clone())
}
fn planning_columns_1(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let (mut v2, mut v3, mut v4): (US0, US1, US3) = planning_of_3(v0.clone(), v1.clone());
    planning_columns_of_2(v2.clone(), v3.clone(), v4.clone())
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        planning_columns_1(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn planning_json_fields_14(mut v0: US0, mut v1: US1, mut v2: US3) -> Rc<str> {
    let mut v7: Rc<str> = match &v0 {
        US0::US0_EstimateMinutes(v4) => {
            let mut v4: i32 = *v4;
            let mut v5: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
            v5.clone()
        }
        US0::US0_Unestimated => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("null"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
    };
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"estimate_minutes\":"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v7));
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"load\":"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
    let mut v23: Rc<str> = match &v1 {
        US1::US1_LoadUnset => {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("null"); } LIT.with(|lit| lit.clone()) };
            v12.clone()
        }
        US1::US1_PlannedLoad(v13) => {
            let mut v13: US2 = v13.clone();
            let mut v17: Rc<str> = match &v13 {
                US2::US2_LoadHeavy => {
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Heavy"); } LIT.with(|lit| lit.clone()) };
                    v16.clone()
                }
                US2::US2_LoadLight => {
                    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Light"); } LIT.with(|lit| lit.clone()) };
                    v14.clone()
                }
                US2::US2_LoadMedium => {
                    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Medium"); } LIT.with(|lit| lit.clone()) };
                    v15.clone()
                }
            };
            let mut v18: Rc<str> = std::rc::Rc::<str>::from(v17.to_ascii_lowercase());
            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v18));
            let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v19));
            v21.clone()
        }
    };
    let mut v24: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v23));
    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"assignee\":"); } LIT.with(|lit| lit.clone()) };
    let mut v26: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v25));
    let mut v38: Rc<str> = match &v2 {
        US3::US3_PlannedAssignee(v28) => {
            let mut v28: US4 = v28.clone();
            let mut v32: Rc<str> = match &v28 {
                US4::US4_AssigneeAgent => {
                    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Agent"); } LIT.with(|lit| lit.clone()) };
                    v29.clone()
                }
                US4::US4_AssigneePair => {
                    let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Pair"); } LIT.with(|lit| lit.clone()) };
                    v30.clone()
                }
                US4::US4_AssigneeUser => {
                    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("User"); } LIT.with(|lit| lit.clone()) };
                    v31.clone()
                }
            };
            let mut v33: Rc<str> = std::rc::Rc::<str>::from(v32.to_ascii_lowercase());
            let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let mut v35: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v33));
            let mut v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v34));
            v36.clone()
        }
        US3::US3_Unassigned => {
            let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("null"); } LIT.with(|lit| lit.clone()) };
            v27.clone()
        }
    };
    let mut v39: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v38));
    v39.clone()
}
fn planning_json_13(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let (mut v2, mut v3, mut v4): (US0, US1, US3) = planning_of_3(v0.clone(), v1.clone());
    planning_json_fields_14(v2.clone(), v3.clone(), v4.clone())
}
fn closure2() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        planning_json_13(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn planning_summary_16(mut v0: US0, mut v1: US1, mut v2: US3) -> Rc<str> {
    let mut v9: Rc<str> = match &v0 {
        US0::US0_EstimateMinutes(v4) => {
            let mut v4: i32 = *v4;
            let mut v5: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
            let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v6));
            v7.clone()
        }
        US0::US0_Unestimated => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
    };
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("estimate="); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v9));
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" load="); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
    let mut v22: Rc<str> = match &v1 {
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
    let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v22));
    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" assignee="); } LIT.with(|lit| lit.clone()) };
    let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v24));
    let mut v34: Rc<str> = match &v2 {
        US3::US3_PlannedAssignee(v27) => {
            let mut v27: US4 = v27.clone();
            let mut v31: Rc<str> = match &v27 {
                US4::US4_AssigneeAgent => {
                    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Agent"); } LIT.with(|lit| lit.clone()) };
                    v28.clone()
                }
                US4::US4_AssigneePair => {
                    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Pair"); } LIT.with(|lit| lit.clone()) };
                    v29.clone()
                }
                US4::US4_AssigneeUser => {
                    let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("User"); } LIT.with(|lit| lit.clone()) };
                    v30.clone()
                }
            };
            let mut v32: Rc<str> = std::rc::Rc::<str>::from(v31.to_ascii_lowercase());
            v32.clone()
        }
        US3::US3_Unassigned => {
            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            v26.clone()
        }
    };
    let mut v35: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v34));
    v35.clone()
}
fn planning_summary_of_15(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let (mut v2, mut v3, mut v4): (US0, US1, US3) = planning_of_3(v0.clone(), v1.clone());
    planning_summary_16(v2.clone(), v3.clone(), v4.clone())
}
fn closure3() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        planning_summary_of_15(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn planning_minutes_of_17(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let (mut v2, mut v3, mut v4): (US0, US1, US3) = planning_of_3(v0.clone(), v1.clone());
    match &v2 {
        US0::US0_EstimateMinutes(v5) => {
            let mut v5: i32 = *v5;
            v5
        }
        US0::US0_Unestimated => {
            0i32
        }
    }
}
fn closure4() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        planning_minutes_of_17(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn byte_find_between_21(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
    let mut v4: i32 = (v0).get(v2 as usize..v3 as usize).and_then(|slice| slice.find(&*v1)).map(|at| at as i32 + v2).unwrap_or(-1);
    v4
}
fn planning_offset_near_20(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    let mut v3: i32 = v2.wrapping_add(1i32);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
    let mut v5: i32 = v0.get(v3 as usize..).and_then(|rest| rest.find(&*v4)).map(|found| (found + v3 as usize) as i32).unwrap_or(-1);
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planning (\""); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v1));
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
    let mut v10: bool = v5 < 0i32;
    let mut v12: i32 = if v10 {
        let mut v11: i32 = (v0.clone().len() as i32);
        v11
    } else {
        v5
    };
    let mut v13: i32 = byte_find_between_21(v0.clone(), v9.clone(), v2, v12);
    let mut v14: bool = v13 >= 0i32;
    let mut v19: bool = if v14 {
        let mut v15: i32 = v0.get(..v13 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
        let mut v16: Rc<str> = std::rc::Rc::<str>::from(v0.get(v15 as usize..v13 as usize).unwrap_or(""));
        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
        let mut v18: bool = v16.starts_with(&*v17);
        v18
    } else {
        false
    };
    if v19 {
        v13
    } else {
        -1i32
    }
}
fn planning_near_19(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> (US0, US1, US3) {
    let mut v3: i32 = planning_offset_near_20(v0.clone(), v1.clone(), v2);
    let mut v4: bool = v3 < 0i32;
    let (mut v11, mut v12, mut v13): (US0, US1, US3) = if v4 {
        let mut v5: US0 = US0::US0_Unestimated;
        let mut v6: US1 = US1::US1_LoadUnset;
        let mut v7: US3 = US3::US3_Unassigned;
        (v5.clone(), v6.clone(), v7.clone())
    } else {
        let (mut v8, mut v9, mut v10): (US0, US1, US3) = planning_at_5(v0.clone(), v3);
        (v8.clone(), v9.clone(), v10.clone())
    };
    (v11.clone(), v12.clone(), v13.clone())
}
fn planning_columns_near_18(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> Rc<str> {
    let (mut v3, mut v4, mut v5): (US0, US1, US3) = planning_near_19(v0.clone(), v1.clone(), v2);
    planning_columns_of_2(v3.clone(), v4.clone(), v5.clone())
}
fn closure5() -> Rc<dyn Fn(Rc<str>, Rc<str>, i32) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, i32) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32| -> Rc<str> {
        planning_columns_near_18(v0.clone(), v1.clone(), v2)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn planning_json_near_22(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> Rc<str> {
    let (mut v3, mut v4, mut v5): (US0, US1, US3) = planning_near_19(v0.clone(), v1.clone(), v2);
    planning_json_fields_14(v3.clone(), v4.clone(), v5.clone())
}
fn closure6() -> Rc<dyn Fn(Rc<str>, Rc<str>, i32) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, i32) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32| -> Rc<str> {
        planning_json_near_22(v0.clone(), v1.clone(), v2)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn planning_minutes_near_23(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    let (mut v3, mut v4, mut v5): (US0, US1, US3) = planning_near_19(v0.clone(), v1.clone(), v2);
    match &v3 {
        US0::US0_EstimateMinutes(v6) => {
            let mut v6: i32 = *v6;
            v6
        }
        US0::US0_Unestimated => {
            0i32
        }
    }
}
fn closure7() -> Rc<dyn Fn(Rc<str>, Rc<str>, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, i32) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32| -> i32 {
        planning_minutes_near_23(v0.clone(), v1.clone(), v2)
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
pub fn eoie_agile_planning_minutes(v0: &str, v1: &str) -> i32 {
    closure4()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn eoie_agile_planning_columns_near(v0: &str, v1: &str, v2: i32) -> Rc<str> {
    closure5()(Rc::<str>::from(v0), Rc::<str>::from(v1), v2)
}
pub fn eoie_agile_planning_json_near(v0: &str, v1: &str, v2: i32) -> Rc<str> {
    closure6()(Rc::<str>::from(v0), Rc::<str>::from(v1), v2)
}
pub fn eoie_agile_planning_minutes_near(v0: &str, v1: &str, v2: i32) -> i32 {
    closure7()(Rc::<str>::from(v0), Rc::<str>::from(v1), v2)
}
