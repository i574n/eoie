#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_EstimateField,
    US0_LoadField,
    US0_AssigneeField,
}
impl US0 {
    fn tag(&self) -> i32 {
        match self {
            US0::US0_EstimateField => 0,
            US0::US0_LoadField => 1,
            US0::US0_AssigneeField => 2,
        }
    }
}
impl Default for US0 {
    fn default() -> Self {
        US0::US0_EstimateField
    }
}
#[derive(Clone)]
enum US1 {
    US1_Valid(Rc<str>),
    US1_Invalid,
}
impl US1 {
    fn tag(&self) -> i32 {
        match self {
            US1::US1_Valid(..) => 0,
            US1::US1_Invalid => 1,
        }
    }
}
impl Default for US1 {
    fn default() -> Self {
        US1::US1_Valid({ thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) })
    }
}
#[derive(Clone)]
enum US4 {
    US4_AssigneeAgent,
    US4_AssigneePair,
    US4_AssigneeUser,
}
impl US4 {
    fn tag(&self) -> i32 {
        match self {
            US4::US4_AssigneeAgent => 0,
            US4::US4_AssigneePair => 1,
            US4::US4_AssigneeUser => 2,
        }
    }
}
impl Default for US4 {
    fn default() -> Self {
        US4::US4_AssigneeAgent
    }
}
#[derive(Clone)]
enum US3 {
    US3_Unassigned,
    US3_PlannedAssignee(US4),
}
impl US3 {
    fn tag(&self) -> i32 {
        match self {
            US3::US3_Unassigned => 0,
            US3::US3_PlannedAssignee(..) => 1,
        }
    }
}
impl Default for US3 {
    fn default() -> Self {
        US3::US3_Unassigned
    }
}
#[derive(Clone)]
enum US2 {
    US2_Valid(US3),
    US2_Invalid,
}
impl US2 {
    fn tag(&self) -> i32 {
        match self {
            US2::US2_Valid(..) => 0,
            US2::US2_Invalid => 1,
        }
    }
}
impl Default for US2 {
    fn default() -> Self {
        US2::US2_Valid(<US3>::default())
    }
}
#[derive(Clone)]
enum US6 {
    US6_Unestimated,
    US6_EstimateMinutes(i32),
}
impl US6 {
    fn tag(&self) -> i32 {
        match self {
            US6::US6_Unestimated => 0,
            US6::US6_EstimateMinutes(..) => 1,
        }
    }
}
impl Default for US6 {
    fn default() -> Self {
        US6::US6_Unestimated
    }
}
#[derive(Clone)]
enum US5 {
    US5_Valid(US6),
    US5_Invalid,
}
impl US5 {
    fn tag(&self) -> i32 {
        match self {
            US5::US5_Valid(..) => 0,
            US5::US5_Invalid => 1,
        }
    }
}
impl Default for US5 {
    fn default() -> Self {
        US5::US5_Valid(<US6>::default())
    }
}
#[derive(Clone)]
enum US7 {
    US7_Valid(i32),
    US7_Invalid,
}
impl US7 {
    fn tag(&self) -> i32 {
        match self {
            US7::US7_Valid(..) => 0,
            US7::US7_Invalid => 1,
        }
    }
}
impl Default for US7 {
    fn default() -> Self {
        US7::US7_Valid(<i32>::default())
    }
}
#[derive(Clone)]
enum US8 {
    US8_MinuteUnit,
    US8_HourUnit,
}
impl US8 {
    fn tag(&self) -> i32 {
        match self {
            US8::US8_MinuteUnit => 0,
            US8::US8_HourUnit => 1,
        }
    }
}
impl Default for US8 {
    fn default() -> Self {
        US8::US8_MinuteUnit
    }
}
#[derive(Clone)]
enum US11 {
    US11_LoadLight,
    US11_LoadMedium,
    US11_LoadHeavy,
}
impl US11 {
    fn tag(&self) -> i32 {
        match self {
            US11::US11_LoadLight => 0,
            US11::US11_LoadMedium => 1,
            US11::US11_LoadHeavy => 2,
        }
    }
}
impl Default for US11 {
    fn default() -> Self {
        US11::US11_LoadLight
    }
}
#[derive(Clone)]
enum US10 {
    US10_LoadUnset,
    US10_PlannedLoad(US11),
}
impl US10 {
    fn tag(&self) -> i32 {
        match self {
            US10::US10_LoadUnset => 0,
            US10::US10_PlannedLoad(..) => 1,
        }
    }
}
impl Default for US10 {
    fn default() -> Self {
        US10::US10_LoadUnset
    }
}
#[derive(Clone)]
enum US9 {
    US9_Valid(US10),
    US9_Invalid,
}
impl US9 {
    fn tag(&self) -> i32 {
        match self {
            US9::US9_Valid(..) => 0,
            US9::US9_Invalid => 1,
        }
    }
}
impl Default for US9 {
    fn default() -> Self {
        US9::US9_Valid(<US10>::default())
    }
}
#[derive(Clone)]
enum US12 {
    US12_Valid(US0),
    US12_Invalid,
}
impl US12 {
    fn tag(&self) -> i32 {
        match self {
            US12::US12_Valid(..) => 0,
            US12::US12_Invalid => 1,
        }
    }
}
impl Default for US12 {
    fn default() -> Self {
        US12::US12_Valid(<US0>::default())
    }
}
fn parse_planned_assignee_3(mut v0: Rc<str>) -> US2 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("none|-|agent|pair|user"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(v0.to_ascii_lowercase());
    let mut v3: u64 = v1.split("|").position(|item| item == &*v2).map(|index| index as u64).unwrap_or(u64::MAX);
    let mut v4: bool = 0u64 == v3;
    if v4 {
        let mut v5: US3 = US3::US3_Unassigned;
        US2::US2_Valid(v5.clone())
    } else {
        let mut v7: bool = 1u64 == v3;
        if v7 {
            let mut v8: US3 = US3::US3_Unassigned;
            US2::US2_Valid(v8.clone())
        } else {
            let mut v10: bool = 2u64 == v3;
            if v10 {
                let mut v11: US4 = US4::US4_AssigneeAgent;
                let mut v12: US3 = US3::US3_PlannedAssignee(v11.clone());
                US2::US2_Valid(v12.clone())
            } else {
                let mut v14: bool = 3u64 == v3;
                if v14 {
                    let mut v15: US4 = US4::US4_AssigneePair;
                    let mut v16: US3 = US3::US3_PlannedAssignee(v15.clone());
                    US2::US2_Valid(v16.clone())
                } else {
                    let mut v18: bool = 4u64 == v3;
                    if v18 {
                        let mut v19: US4 = US4::US4_AssigneeUser;
                        let mut v20: US3 = US3::US3_PlannedAssignee(v19.clone());
                        US2::US2_Valid(v20.clone())
                    } else {
                        US2::US2_Invalid
                    }
                }
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
fn decimal_value_5(mut v0: Rc<str>) -> US7 {
    let mut v1: i32 = v0.len() as i32;
    let mut v2: bool = v1 <= 0i32;
    if v2 {
        US7::US7_Invalid
    } else {
        let mut v4: bool = v1 <= 6i32;
        if v4 {
            let mut v5: i32 = 0i32;
            let mut v6: i32 = 0i32;
            let mut v7: i32 = decimal_digits_value_6(v0.clone(), v5, v1, v6);
            let mut v8: bool = v7 < 0;
            if v8 {
                US7::US7_Invalid
            } else {
                US7::US7_Valid(v7)
            }
        } else {
            US7::US7_Invalid
        }
    }
}
fn parse_planned_estimate_4(mut v0: Rc<str>) -> US5 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("none|-"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v1.split("|").any(|item| item == &*v0);
    if v2 {
        let mut v3: US6 = US6::US6_Unestimated;
        US5::US5_Valid(v3.clone())
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
        let mut v17: US7 = decimal_value_5(v16.clone());
        match &v17 {
            US7::US7_Invalid => {
                US5::US5_Invalid
            }
            US7::US7_Valid(v18) => {
                let mut v18: i32 = *v18;
                let mut v19: bool = v0.ends_with(&*v5);
                let mut v22: US8 = if v19 {
                    US8::US8_HourUnit
                } else {
                    US8::US8_MinuteUnit
                };
                let mut v24: i32 = match &v22 {
                    US8::US8_HourUnit => {
                        60i32
                    }
                    US8::US8_MinuteUnit => {
                        1i32
                    }
                };
                let mut v25: i32 = v18 * v24;
                let mut v26: bool = v25 <= 999999i32;
                if v26 {
                    let mut v27: US6 = US6::US6_EstimateMinutes(v25);
                    US5::US5_Valid(v27.clone())
                } else {
                    US5::US5_Invalid
                }
            }
        }
    }
}
fn parse_planned_load_7(mut v0: Rc<str>) -> US9 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("none|-|light|medium|heavy"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(v0.to_ascii_lowercase());
    let mut v3: u64 = v1.split("|").position(|item| item == &*v2).map(|index| index as u64).unwrap_or(u64::MAX);
    let mut v4: bool = 0u64 == v3;
    if v4 {
        let mut v5: US10 = US10::US10_LoadUnset;
        US9::US9_Valid(v5.clone())
    } else {
        let mut v7: bool = 1u64 == v3;
        if v7 {
            let mut v8: US10 = US10::US10_LoadUnset;
            US9::US9_Valid(v8.clone())
        } else {
            let mut v10: bool = 2u64 == v3;
            if v10 {
                let mut v11: US11 = US11::US11_LoadLight;
                let mut v12: US10 = US10::US10_PlannedLoad(v11.clone());
                US9::US9_Valid(v12.clone())
            } else {
                let mut v14: bool = 3u64 == v3;
                if v14 {
                    let mut v15: US11 = US11::US11_LoadMedium;
                    let mut v16: US10 = US10::US10_PlannedLoad(v15.clone());
                    US9::US9_Valid(v16.clone())
                } else {
                    let mut v18: bool = 4u64 == v3;
                    if v18 {
                        let mut v19: US11 = US11::US11_LoadHeavy;
                        let mut v20: US10 = US10::US10_PlannedLoad(v19.clone());
                        US9::US9_Valid(v20.clone())
                    } else {
                        US9::US9_Invalid
                    }
                }
            }
        }
    }
}
fn planned_value_token_2(mut v0: US0, mut v1: Rc<str>) -> US1 {
    match &v0 {
        US0::US0_AssigneeField => {
            let mut v36: US2 = parse_planned_assignee_3(v1.clone());
            match &v36 {
                US2::US2_Invalid => {
                    US1::US1_Invalid
                }
                US2::US2_Valid(v37) => {
                    let mut v37: US3 = v37.clone();
                    let mut v50: Rc<str> = match &v37 {
                        US3::US3_PlannedAssignee(v39) => {
                            let mut v39: US4 = v39.clone();
                            let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Assigned "); } LIT.with(|lit| lit.clone()) };
                            let mut v46: Rc<str> = match &v39 {
                                US4::US4_AssigneeAgent => {
                                    let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Agent"); } LIT.with(|lit| lit.clone()) };
                                    v41.clone()
                                }
                                US4::US4_AssigneePair => {
                                    let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Pair"); } LIT.with(|lit| lit.clone()) };
                                    v42.clone()
                                }
                                US4::US4_AssigneeUser => {
                                    let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("User"); } LIT.with(|lit| lit.clone()) };
                                    v43.clone()
                                }
                            };
                            let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let mut v48: Rc<str> = std::rc::Rc::<str>::from([&*v40, &*v46, &*v47, &*v47, &*v47].concat());
                            v48.clone()
                        }
                        US3::US3_Unassigned => {
                            let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Unassigned"); } LIT.with(|lit| lit.clone()) };
                            v38.clone()
                        }
                    };
                    US1::US1_Valid(v50.clone())
                }
            }
        }
        US0::US0_EstimateField => {
            let mut v2: US5 = parse_planned_estimate_4(v1.clone());
            match &v2 {
                US5::US5_Invalid => {
                    US1::US1_Invalid
                }
                US5::US5_Valid(v3) => {
                    let mut v3: US6 = v3.clone();
                    let mut v12: Rc<str> = match &v3 {
                        US6::US6_EstimateMinutes(v5) => {
                            let mut v5: i32 = *v5;
                            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Minutes "); } LIT.with(|lit| lit.clone()) };
                            let mut v7: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
                            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
                            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let mut v10: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v7, &*v8, &*v9, &*v9].concat());
                            v10.clone()
                        }
                        US6::US6_Unestimated => {
                            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Unestimated"); } LIT.with(|lit| lit.clone()) };
                            v4.clone()
                        }
                    };
                    US1::US1_Valid(v12.clone())
                }
            }
        }
        US0::US0_LoadField => {
            let mut v17: US9 = parse_planned_load_7(v1.clone());
            match &v17 {
                US9::US9_Invalid => {
                    US1::US1_Invalid
                }
                US9::US9_Valid(v18) => {
                    let mut v18: US10 = v18.clone();
                    let mut v31: Rc<str> = match &v18 {
                        US10::US10_LoadUnset => {
                            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("LoadUnset"); } LIT.with(|lit| lit.clone()) };
                            v19.clone()
                        }
                        US10::US10_PlannedLoad(v20) => {
                            let mut v20: US11 = v20.clone();
                            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Load "); } LIT.with(|lit| lit.clone()) };
                            let mut v27: Rc<str> = match &v20 {
                                US11::US11_LoadHeavy => {
                                    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Heavy"); } LIT.with(|lit| lit.clone()) };
                                    v24.clone()
                                }
                                US11::US11_LoadLight => {
                                    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Light"); } LIT.with(|lit| lit.clone()) };
                                    v22.clone()
                                }
                                US11::US11_LoadMedium => {
                                    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Medium"); } LIT.with(|lit| lit.clone()) };
                                    v23.clone()
                                }
                            };
                            let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let mut v29: Rc<str> = std::rc::Rc::<str>::from([&*v21, &*v27, &*v28, &*v28, &*v28].concat());
                            v29.clone()
                        }
                    };
                    US1::US1_Valid(v31.clone())
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
                US0::US0_AssigneeField => {
                    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("assignee must be agent, pair, user or none"); } LIT.with(|lit| lit.clone()) };
                    v6.clone()
                }
                US0::US0_EstimateField => {
                    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("estimate must be whole minutes (360, 360m) or hours (6h), at most 999999 minutes, or none"); } LIT.with(|lit| lit.clone()) };
                    v4.clone()
                }
                US0::US0_LoadField => {
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
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("estimate|load|assignee"); } LIT.with(|lit| lit.clone()) };
    let mut v3: u64 = v2.split("|").position(|item| item == &*v0).map(|index| index as u64).unwrap_or(u64::MAX);
    let mut v4: bool = 0u64 == v3;
    let mut v16: US12 = if v4 {
        let mut v5: US0 = US0::US0_EstimateField;
        US12::US12_Valid(v5.clone())
    } else {
        let mut v7: bool = 1u64 == v3;
        if v7 {
            let mut v8: US0 = US0::US0_LoadField;
            US12::US12_Valid(v8.clone())
        } else {
            let mut v10: bool = 2u64 == v3;
            if v10 {
                let mut v11: US0 = US0::US0_AssigneeField;
                US12::US12_Valid(v11.clone())
            } else {
                US12::US12_Invalid
            }
        }
    };
    match &v16 {
        US12::US12_Invalid => {
            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("field must be estimate, load or assignee"); } LIT.with(|lit| lit.clone()) };
            (1i32, v20.clone())
        }
        US12::US12_Valid(v17) => {
            let mut v17: US0 = v17.clone();
            field_value_token_1(v17.clone(), v1.clone())
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
