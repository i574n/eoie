#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_FieldEstimate,
    US0_FieldLoad,
    US0_FieldAssignee,
}
#[derive(Clone)]
enum US1 {
    US1_Valid(Rc<str>),
    US1_Invalid,
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
enum US2 {
    US2_Valid(US3),
    US2_Invalid,
}
#[derive(Clone)]
enum US5 {
    US5_Valid(US4),
    US5_Invalid,
}
#[derive(Clone)]
enum US7 {
    US7_Unestimated,
    US7_EstimateMinutes(i32),
}
#[derive(Clone)]
enum US6 {
    US6_Valid(US7),
    US6_Invalid,
}
#[derive(Clone)]
enum US8 {
    US8_Valid(i32),
    US8_Invalid,
}
#[derive(Clone)]
enum US9 {
    US9_MinuteUnit,
    US9_HourUnit,
}
#[derive(Clone)]
enum US12 {
    US12_LoadLight,
    US12_LoadMedium,
    US12_LoadHeavy,
}
#[derive(Clone)]
enum US11 {
    US11_LoadUnset,
    US11_PlannedLoad(US12),
}
#[derive(Clone)]
enum US10 {
    US10_Valid(US11),
    US10_Invalid,
}
#[derive(Clone)]
enum US13 {
    US13_Valid(US12),
    US13_Invalid,
}
#[derive(Clone)]
enum US14 {
    US14_Valid(US0),
    US14_Invalid,
}
fn parse_planned_assignee_3(mut v0: Rc<str>) -> US2 {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.to_ascii_lowercase());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("none|-"); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v2.split("|").any(|item| item == &*v1);
    if v3 {
        let mut v4: US3 = US3::US3_Unassigned;
        US2::US2_Valid(v4.clone())
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Agent"); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = std::rc::Rc::<str>::from(v6.to_ascii_lowercase());
        let mut v8: bool = v1 == v7 ;
        let mut v12: US5 = if v8 {
            let mut v9: US4 = US4::US4_AssigneeAgent;
            US5::US5_Valid(v9.clone())
        } else {
            US5::US5_Invalid
        };
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Pair"); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<str> = std::rc::Rc::<str>::from(v13.to_ascii_lowercase());
        let mut v15: bool = v1 == v14 ;
        let mut v18: US5 = if v15 {
            let mut v16: US4 = US4::US4_AssigneePair;
            US5::US5_Valid(v16.clone())
        } else {
            v12.clone()
        };
        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("User"); } LIT.with(|lit| lit.clone()) };
        let mut v20: Rc<str> = std::rc::Rc::<str>::from(v19.to_ascii_lowercase());
        let mut v21: bool = v1 == v20 ;
        let mut v24: US5 = if v21 {
            let mut v22: US4 = US4::US4_AssigneeUser;
            US5::US5_Valid(v22.clone())
        } else {
            v18.clone()
        };
        match &v24 {
            US5::US5_Invalid => {
                US2::US2_Invalid
            }
            US5::US5_Valid(v25) => {
                let mut v25: US4 = v25.clone();
                let mut v26: US3 = US3::US3_PlannedAssignee(v25.clone());
                US2::US2_Valid(v26.clone())
            }
        }
    }
}
fn decimal_digits_value_6(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
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
fn decimal_value_5(mut v0: Rc<str>) -> US8 {
    let mut v1: i32 = v0.len() as i32;
    let mut v2: bool = v1 <= 0i32;
    if v2 {
        US8::US8_Invalid
    } else {
        let mut v4: bool = v1 <= 6i32;
        if v4 {
            let mut v5: i32 = 0i32;
            let mut v6: i32 = 0i32;
            let mut v7: i32 = decimal_digits_value_6(v0.clone(), v5, v1, v6);
            let mut v8: bool = v7 < 0;
            if v8 {
                US8::US8_Invalid
            } else {
                US8::US8_Valid(v7)
            }
        } else {
            US8::US8_Invalid
        }
    }
}
fn parse_planned_estimate_4(mut v0: Rc<str>) -> US6 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("none|-"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v1.split("|").any(|item| item == &*v0);
    if v2 {
        let mut v3: US7 = US7::US7_Unestimated;
        US6::US6_Valid(v3.clone())
    } else {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) };
        let mut v6: bool = v0.ends_with(&*v5);
        let mut v16: Rc<str> = if v6 {
            let mut v7: i32 = v0.len() as i32;
            let mut v8: i32 = v7 - 1;
            let mut v9: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v8 as usize).unwrap_or(""));
            v9.clone()
        } else {
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
            let mut v11: bool = v0.ends_with(&*v10);
            if v11 {
                let mut v12: i32 = v0.len() as i32;
                let mut v13: i32 = v12 - 1;
                let mut v14: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v13 as usize).unwrap_or(""));
                v14.clone()
            } else {
                v0.clone()
            }
        };
        let mut v17: US8 = decimal_value_5(v16.clone());
        match &v17 {
            US8::US8_Invalid => {
                US6::US6_Invalid
            }
            US8::US8_Valid(v18) => {
                let mut v18: i32 = *v18;
                let mut v19: bool = v0.ends_with(&*v5);
                let mut v22: US9 = if v19 {
                    US9::US9_HourUnit
                } else {
                    US9::US9_MinuteUnit
                };
                let mut v24: i32 = match &v22 {
                    US9::US9_HourUnit => {
                        60i32
                    }
                    US9::US9_MinuteUnit => {
                        1i32
                    }
                };
                let mut v25: i32 = v18 * v24;
                let mut v26: bool = v25 <= 999999i32;
                if v26 {
                    let mut v27: US7 = US7::US7_EstimateMinutes(v25);
                    US6::US6_Valid(v27.clone())
                } else {
                    US6::US6_Invalid
                }
            }
        }
    }
}
fn parse_planned_load_7(mut v0: Rc<str>) -> US10 {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.to_ascii_lowercase());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("none|-"); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v2.split("|").any(|item| item == &*v1);
    if v3 {
        let mut v4: US11 = US11::US11_LoadUnset;
        US10::US10_Valid(v4.clone())
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Light"); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = std::rc::Rc::<str>::from(v6.to_ascii_lowercase());
        let mut v8: bool = v1 == v7 ;
        let mut v12: US13 = if v8 {
            let mut v9: US12 = US12::US12_LoadLight;
            US13::US13_Valid(v9.clone())
        } else {
            US13::US13_Invalid
        };
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Medium"); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<str> = std::rc::Rc::<str>::from(v13.to_ascii_lowercase());
        let mut v15: bool = v1 == v14 ;
        let mut v18: US13 = if v15 {
            let mut v16: US12 = US12::US12_LoadMedium;
            US13::US13_Valid(v16.clone())
        } else {
            v12.clone()
        };
        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Heavy"); } LIT.with(|lit| lit.clone()) };
        let mut v20: Rc<str> = std::rc::Rc::<str>::from(v19.to_ascii_lowercase());
        let mut v21: bool = v1 == v20 ;
        let mut v24: US13 = if v21 {
            let mut v22: US12 = US12::US12_LoadHeavy;
            US13::US13_Valid(v22.clone())
        } else {
            v18.clone()
        };
        match &v24 {
            US13::US13_Invalid => {
                US10::US10_Invalid
            }
            US13::US13_Valid(v25) => {
                let mut v25: US12 = v25.clone();
                let mut v26: US11 = US11::US11_PlannedLoad(v25.clone());
                US10::US10_Valid(v26.clone())
            }
        }
    }
}
fn planned_value_token_2(mut v0: US0, mut v1: Rc<str>) -> US1 {
    match &v0 {
        US0::US0_FieldAssignee => {
            let mut v34: US2 = parse_planned_assignee_3(v1.clone());
            match &v34 {
                US2::US2_Invalid => {
                    US1::US1_Invalid
                }
                US2::US2_Valid(v35) => {
                    let mut v35: US3 = v35.clone();
                    let mut v46: Rc<str> = match &v35 {
                        US3::US3_PlannedAssignee(v37) => {
                            let mut v37: US4 = v37.clone();
                            let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Assigned "); } LIT.with(|lit| lit.clone()) };
                            let mut v42: Rc<str> = match &v37 {
                                US4::US4_AssigneeAgent => {
                                    let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Agent"); } LIT.with(|lit| lit.clone()) };
                                    v39.clone()
                                }
                                US4::US4_AssigneePair => {
                                    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Pair"); } LIT.with(|lit| lit.clone()) };
                                    v40.clone()
                                }
                                US4::US4_AssigneeUser => {
                                    let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("User"); } LIT.with(|lit| lit.clone()) };
                                    v41.clone()
                                }
                            };
                            let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let mut v44: Rc<str> = std::rc::Rc::<str>::from([&*v38, &*v42, &*v43, &*v43, &*v43].concat());
                            v44.clone()
                        }
                        US3::US3_Unassigned => {
                            let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Unassigned"); } LIT.with(|lit| lit.clone()) };
                            v36.clone()
                        }
                    };
                    US1::US1_Valid(v46.clone())
                }
            }
        }
        US0::US0_FieldEstimate => {
            let mut v2: US6 = parse_planned_estimate_4(v1.clone());
            match &v2 {
                US6::US6_Invalid => {
                    US1::US1_Invalid
                }
                US6::US6_Valid(v3) => {
                    let mut v3: US7 = v3.clone();
                    let mut v12: Rc<str> = match &v3 {
                        US7::US7_EstimateMinutes(v5) => {
                            let mut v5: i32 = *v5;
                            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Minutes "); } LIT.with(|lit| lit.clone()) };
                            let mut v7: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
                            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
                            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let mut v10: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v7, &*v8, &*v9, &*v9].concat());
                            v10.clone()
                        }
                        US7::US7_Unestimated => {
                            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Unestimated"); } LIT.with(|lit| lit.clone()) };
                            v4.clone()
                        }
                    };
                    US1::US1_Valid(v12.clone())
                }
            }
        }
        US0::US0_FieldLoad => {
            let mut v17: US10 = parse_planned_load_7(v1.clone());
            match &v17 {
                US10::US10_Invalid => {
                    US1::US1_Invalid
                }
                US10::US10_Valid(v18) => {
                    let mut v18: US11 = v18.clone();
                    let mut v29: Rc<str> = match &v18 {
                        US11::US11_LoadUnset => {
                            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("LoadUnset"); } LIT.with(|lit| lit.clone()) };
                            v19.clone()
                        }
                        US11::US11_PlannedLoad(v20) => {
                            let mut v20: US12 = v20.clone();
                            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Load "); } LIT.with(|lit| lit.clone()) };
                            let mut v25: Rc<str> = match &v20 {
                                US12::US12_LoadHeavy => {
                                    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Heavy"); } LIT.with(|lit| lit.clone()) };
                                    v24.clone()
                                }
                                US12::US12_LoadLight => {
                                    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Light"); } LIT.with(|lit| lit.clone()) };
                                    v22.clone()
                                }
                                US12::US12_LoadMedium => {
                                    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Medium"); } LIT.with(|lit| lit.clone()) };
                                    v23.clone()
                                }
                            };
                            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let mut v27: Rc<str> = std::rc::Rc::<str>::from([&*v21, &*v25, &*v26, &*v26, &*v26].concat());
                            v27.clone()
                        }
                    };
                    US1::US1_Valid(v29.clone())
                }
            }
        }
    }
}
fn field_value_token_1(mut v0: US0, mut v1: Rc<str>) -> (i32, Rc<str>) {
    let mut v2: US1 = planned_value_token_2(v0.clone(), v1.clone());
    match &v2 {
        US1::US1_Invalid => {
            let mut v9: Rc<str> = match &v0 {
                US0::US0_FieldAssignee => {
                    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("assignee must be agent, pair, user or none"); } LIT.with(|lit| lit.clone()) };
                    v6.clone()
                }
                US0::US0_FieldEstimate => {
                    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("estimate must be whole minutes (360, 360m) or hours (6h), at most 999999 minutes, or none"); } LIT.with(|lit| lit.clone()) };
                    v4.clone()
                }
                US0::US0_FieldLoad => {
                    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("load must be light, medium, heavy or none"); } LIT.with(|lit| lit.clone()) };
                    v5.clone()
                }
            };
            (1i32, v9.clone())
        }
        US1::US1_Valid(v3) => {
            let mut v3: Rc<str> = v3.clone();
            (0i32, v3.clone())
        }
    }
}
fn planning_value_token_0(mut v0: Rc<str>, mut v1: Rc<str>) -> (i32, Rc<str>) {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Estimate"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = std::rc::Rc::<str>::from(v2.to_ascii_lowercase());
    let mut v4: bool = v0 == v3 ;
    let mut v8: US14 = if v4 {
        let mut v5: US0 = US0::US0_FieldEstimate;
        US14::US14_Valid(v5.clone())
    } else {
        US14::US14_Invalid
    };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Load"); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = std::rc::Rc::<str>::from(v9.to_ascii_lowercase());
    let mut v11: bool = v0 == v10 ;
    let mut v14: US14 = if v11 {
        let mut v12: US0 = US0::US0_FieldLoad;
        US14::US14_Valid(v12.clone())
    } else {
        v8.clone()
    };
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Assignee"); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = std::rc::Rc::<str>::from(v15.to_ascii_lowercase());
    let mut v17: bool = v0 == v16 ;
    let mut v20: US14 = if v17 {
        let mut v18: US0 = US0::US0_FieldAssignee;
        US14::US14_Valid(v18.clone())
    } else {
        v14.clone()
    };
    match &v20 {
        US14::US14_Invalid => {
            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("field must be estimate, load or assignee"); } LIT.with(|lit| lit.clone()) };
            (1i32, v24.clone())
        }
        US14::US14_Valid(v21) => {
            let mut v21: US0 = v21.clone();
            field_value_token_1(v21.clone(), v1.clone())
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> (i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> (i32, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> (i32, Rc<str>) {
        planning_value_token_0(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn legacy_agent_value_8(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("yes|pair|no"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agent\npair\nuser"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = v1.split("|").zip(v2.split(char::from(10u8))).find_map(|(key,item)| if key == &*v0 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
    v3.clone()
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        legacy_agent_value_8(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_planning_value_token(v0: &str, v1: &str) -> (i32, Rc<str>) {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn eoie_agile_planning_legacy_agent_value(v0: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0))
}
