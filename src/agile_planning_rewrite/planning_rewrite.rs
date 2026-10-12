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
    US1_ScanPlain,
    US1_ScanQuoted,
    US1_ScanEscaped,
}
impl US1 {
    fn tag(&self) -> i32 {
        match self {
            US1::US1_ScanPlain => 0,
            US1::US1_ScanQuoted => 1,
            US1::US1_ScanEscaped => 2,
        }
    }
}
impl Default for US1 {
    fn default() -> Self {
        US1::US1_ScanPlain
    }
}
#[derive(Clone)]
enum US2 {
    US2_Valid(Rc<str>),
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
        US2::US2_Valid({ thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) })
    }
}
#[derive(Clone)]
enum US3 {
    US3_Valid(US0),
    US3_Invalid,
}
impl US3 {
    fn tag(&self) -> i32 {
        match self {
            US3::US3_Valid(..) => 0,
            US3::US3_Invalid => 1,
        }
    }
}
impl Default for US3 {
    fn default() -> Self {
        US3::US3_Valid(<US0>::default())
    }
}
fn planning_schema_ensured_2(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_planning ="); } LIT.with(|lit| lit.clone()) };
    let mut v2: i32 = v0.find(&*v1).map(|offset| offset as i32).unwrap_or(-1);
    let mut v3: bool = v2 < 0;
    if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-> agile_item"); } LIT.with(|lit| lit.clone()) };
        let mut v5: i32 = v0.find(&*v4).map(|offset| offset as i32).unwrap_or(-1);
        let mut v6: bool = v5 < 0;
        if v6 {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_load =\n    | Light :: task_load | Medium :: task_load | Heavy :: task_load\n\n"); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_assignee =\n    | Agent :: task_assignee | Pair :: task_assignee | User :: task_assignee\n\n"); } LIT.with(|lit| lit.clone()) };
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union planned_estimate =\n    | Unestimated :: planned_estimate | Minutes :: u32 -> planned_estimate\n\n"); } LIT.with(|lit| lit.clone()) };
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union planned_load =\n    | LoadUnset :: planned_load | Load :: task_load -> planned_load\n\nunion planned_assignee =\n    | Unassigned :: planned_assignee | Assigned :: task_assignee -> planned_assignee\n\n"); } LIT.with(|lit| lit.clone()) };
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_planning =\n    | Planning :: string * planned_estimate * planned_load * planned_assignee -> task_planning"); } LIT.with(|lit| lit.clone()) };
            let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v8, &*v9, &*v10, &*v11].concat());
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n\n"); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v15: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v13, &*v0, &*v14, &*v14].concat());
            v15.clone()
        } else {
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v17: i32 = v0.get(v5 as usize..).and_then(|rest| rest.find(&*v16)).map(|found| (found + v5 as usize) as i32).unwrap_or(-1);
            let mut v18: bool = v17 < 0;
            let mut v20: i32 = if v18 {
                let mut v19: i32 = v0.len() as i32;
                v19
            } else {
                v17
            };
            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n\n"); } LIT.with(|lit| lit.clone()) };
            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_load =\n    | Light :: task_load | Medium :: task_load | Heavy :: task_load\n\n"); } LIT.with(|lit| lit.clone()) };
            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_assignee =\n    | Agent :: task_assignee | Pair :: task_assignee | User :: task_assignee\n\n"); } LIT.with(|lit| lit.clone()) };
            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union planned_estimate =\n    | Unestimated :: planned_estimate | Minutes :: u32 -> planned_estimate\n\n"); } LIT.with(|lit| lit.clone()) };
            let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union planned_load =\n    | LoadUnset :: planned_load | Load :: task_load -> planned_load\n\nunion planned_assignee =\n    | Unassigned :: planned_assignee | Assigned :: task_assignee -> planned_assignee\n\n"); } LIT.with(|lit| lit.clone()) };
            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_planning =\n    | Planning :: string * planned_estimate * planned_load * planned_assignee -> task_planning"); } LIT.with(|lit| lit.clone()) };
            let mut v27: Rc<str> = std::rc::Rc::<str>::from([&*v22, &*v23, &*v24, &*v25, &*v26].concat());
            let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v29: Rc<str> = std::rc::Rc::<str>::from([&*v21, &*v27, &*v28, &*v28, &*v28].concat());
            let mut v30: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v20 as usize).unwrap_or(""));
            let mut v31: i32 = v0.len() as i32;
            let mut v32: Rc<str> = std::rc::Rc::<str>::from(v0.get(v20 as usize..v31 as usize).unwrap_or(""));
            let mut v33: Rc<str> = std::rc::Rc::<str>::from([&*v30, &*v29, &*v32, &*v28, &*v28].concat());
            v33.clone()
        }
    } else {
        v0.clone()
    }
}
fn declared_from_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
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
fn field_end_6(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US1) -> i32 {
    loop {
        let mut v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let mut v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = match &v3 {
                US1::US1_ScanPlain => {
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
                let mut v25: US1 = match &v3 {
                    US1::US1_ScanEscaped => {
                        US1::US1_ScanQuoted
                    }
                    US1::US1_ScanPlain => {
                        let mut v19: bool = v5 == 34i32;
                        if v19 {
                            US1::US1_ScanQuoted
                        } else {
                            US1::US1_ScanPlain
                        }
                    }
                    US1::US1_ScanQuoted => {
                        let mut v12: bool = v5 == 92i32;
                        if v12 {
                            US1::US1_ScanEscaped
                        } else {
                            let mut v14: bool = v5 == 34i32;
                            if v14 {
                                US1::US1_ScanPlain
                            } else {
                                US1::US1_ScanQuoted
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
            let mut v4: i32 = v1 + 1;
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
            let mut v5: US1 = US1::US1_ScanPlain;
            let mut v6: i32 = field_end_6(v0.clone(), v1, v2, v5.clone());
            let mut v7: i32 = v6 + 1;
            let mut v8: i32 = skip_spaces_7(v0.clone(), v7);
            let mut v9: i32 = v3 - 1;
            (v0, v1, v2, v3) = (v0.clone(), v8, v2, v9);
            continue;
        }
    }
}
fn planning_tokens_of_3(mut v0: Rc<str>, mut v1: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>) {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planning (\""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v1, &*v3, &*v4, &*v4].concat());
    let mut v6: i32 = 0i32;
    let mut v7: i32 = declared_from_4(v0.clone(), v5.clone(), v6);
    let mut v8: bool = v7 < 0;
    let (mut v29, mut v30, mut v31): (Rc<str>, Rc<str>, Rc<str>) = if v8 {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Unestimated"); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("LoadUnset"); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Unassigned"); } LIT.with(|lit| lit.clone()) };
        (v9.clone(), v10.clone(), v11.clone())
    } else {
        let mut v12: i32 = v7 + 10i32;
        let mut v13: i32 = v0.len() as i32;
        let mut v14: i32 = 1i32;
        let mut v15: i32 = field_start_5(v0.clone(), v12, v13, v14);
        let mut v16: US1 = US1::US1_ScanPlain;
        let mut v17: i32 = field_end_6(v0.clone(), v15, v13, v16.clone());
        let mut v18: Rc<str> = std::rc::Rc::<str>::from(v0.get(v15 as usize..v17 as usize).unwrap_or(""));
        let mut v19: i32 = 2i32;
        let mut v20: i32 = field_start_5(v0.clone(), v12, v13, v19);
        let mut v21: US1 = US1::US1_ScanPlain;
        let mut v22: i32 = field_end_6(v0.clone(), v20, v13, v21.clone());
        let mut v23: Rc<str> = std::rc::Rc::<str>::from(v0.get(v20 as usize..v22 as usize).unwrap_or(""));
        let mut v24: i32 = 3i32;
        let mut v25: i32 = field_start_5(v0.clone(), v12, v13, v24);
        let mut v26: US1 = US1::US1_ScanPlain;
        let mut v27: i32 = field_end_6(v0.clone(), v25, v13, v26.clone());
        let mut v28: Rc<str> = std::rc::Rc::<str>::from(v0.get(v25 as usize..v27 as usize).unwrap_or(""));
        (v18.clone(), v23.clone(), v28.clone())
    };
    (v29.clone(), v30.clone(), v31.clone())
}
fn planning_row_9(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("planning_of_"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = std::rc::Rc::<str>::from(v0.replace(&*v6, &v7));
    let mut v9: Rc<str> = std::rc::Rc::<str>::from(v8.to_ascii_lowercase());
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v9, &*v10, &*v10, &*v10].concat());
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = Planning ("); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = std::rc::Rc::<str>::from(v0.replace(&*v14, &v15));
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = std::rc::Rc::<str>::from(v16.replace(&*v13, &v17));
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let mut v21: Rc<str> = std::rc::Rc::<str>::from(v18.replace(&*v19, &v20));
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let mut v24: Rc<str> = std::rc::Rc::<str>::from(v21.replace(&*v22, &v23));
    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let mut v27: Rc<str> = std::rc::Rc::<str>::from(v24.replace(&*v25, &v26));
    let mut v28: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v27, &*v13, &*v10, &*v10].concat());
    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
    let mut v30: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v11, &*v12, &*v28, &*v29].concat());
    let mut v31: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v29, &*v2, &*v29, &*v3].concat());
    let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let mut v33: Rc<str> = std::rc::Rc::<str>::from([&*v30, &*v31, &*v32, &*v10, &*v10].concat());
    v33.clone()
}
fn planning_upserted_8(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> US2 {
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v1, &*v6, &*v7, &*v7].concat());
    let mut v9: i32 = v0.find(&*v8).map(|offset| offset as i32).unwrap_or(-1);
    let mut v10: bool = v9 < 0;
    if v10 {
        US2::US2_Invalid
    } else {
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planning (\""); } LIT.with(|lit| lit.clone()) };
        let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v1, &*v6, &*v7, &*v7].concat());
        let mut v14: i32 = 0i32;
        let mut v15: i32 = declared_from_4(v0.clone(), v13.clone(), v14);
        let mut v16: bool = v15 < 0;
        if v16 {
            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v18: i32 = v0.get(v9 as usize..).and_then(|rest| rest.find(&*v17)).map(|found| (found + v9 as usize) as i32).unwrap_or(-1);
            let mut v19: bool = v18 < 0;
            let mut v21: i32 = if v19 {
                let mut v20: i32 = v0.len() as i32;
                v20
            } else {
                v18
            };
            let mut v22: Rc<str> = planning_row_9(v1.clone(), v2.clone(), v3.clone(), v4.clone());
            let mut v23: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v22, &*v7, &*v7, &*v7].concat());
            let mut v24: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v21 as usize).unwrap_or(""));
            let mut v25: i32 = v0.len() as i32;
            let mut v26: Rc<str> = std::rc::Rc::<str>::from(v0.get(v21 as usize..v25 as usize).unwrap_or(""));
            let mut v27: Rc<str> = std::rc::Rc::<str>::from([&*v24, &*v23, &*v26, &*v7, &*v7].concat());
            US2::US2_Valid(v27.clone())
        } else {
            let mut v29: i32 = v0.get(..v15 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v31: i32 = v0.get(v15 as usize..).and_then(|rest| rest.find(&*v30)).map(|found| (found + v15 as usize) as i32).unwrap_or(-1);
            let mut v32: bool = v31 < 0;
            let mut v34: i32 = if v32 {
                let mut v33: i32 = v0.len() as i32;
                v33
            } else {
                v31
            };
            let mut v35: Rc<str> = planning_row_9(v1.clone(), v2.clone(), v3.clone(), v4.clone());
            let mut v36: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v29 as usize).unwrap_or(""));
            let mut v37: i32 = v0.len() as i32;
            let mut v38: Rc<str> = std::rc::Rc::<str>::from(v0.get(v34 as usize..v37 as usize).unwrap_or(""));
            let mut v39: Rc<str> = std::rc::Rc::<str>::from([&*v36, &*v35, &*v38, &*v7, &*v7].concat());
            US2::US2_Valid(v39.clone())
        }
    }
}
fn field_token_rewrite_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: US0, mut v3: Rc<str>) -> (i32, Rc<str>) {
    let mut v4: Rc<str> = planning_schema_ensured_2(v0.clone());
    let (mut v5, mut v6, mut v7): (Rc<str>, Rc<str>, Rc<str>) = planning_tokens_of_3(v0.clone(), v1.clone());
    let (mut v14, mut v15, mut v16): (Rc<str>, Rc<str>, Rc<str>) = match &v2 {
        US0::US0_EstimateField => {
            (v3.clone(), v6.clone(), v7.clone())
        }
        _ => {
            let (mut v11, mut v12, mut v13): (Rc<str>, Rc<str>, Rc<str>) = match &v2 {
                US0::US0_LoadField => {
                    (v5.clone(), v3.clone(), v7.clone())
                }
                _ => {
                    let (mut v8, mut v9, mut v10): (Rc<str>, Rc<str>, Rc<str>) = match &v2 {
                        US0::US0_AssigneeField => {
                            (v5.clone(), v6.clone(), v3.clone())
                        }
                        _ => unreachable!(),
                    };
                    (v8.clone(), v9.clone(), v10.clone())
                }
            };
            (v11.clone(), v12.clone(), v13.clone())
        }
    };
    let mut v17: US2 = planning_upserted_8(v4.clone(), v1.clone(), v14.clone(), v15.clone(), v16.clone());
    match &v17 {
        US2::US2_Invalid => {
            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no task with this id"); } LIT.with(|lit| lit.clone()) };
            (2i32, v19.clone())
        }
        US2::US2_Valid(v18) => {
            let mut v18: Rc<str> = v18.clone();
            (0i32, v18.clone())
        }
    }
}
fn planning_rewrite_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> (i32, Rc<str>) {
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("estimate|load|assignee"); } LIT.with(|lit| lit.clone()) };
    let mut v5: u64 = v4.split("|").position(|item| item == &*v2).map(|index| index as u64).unwrap_or(u64::MAX);
    let mut v6: bool = 0u64 == v5;
    let mut v18: US3 = if v6 {
        let mut v7: US0 = US0::US0_EstimateField;
        US3::US3_Valid(v7.clone())
    } else {
        let mut v9: bool = 1u64 == v5;
        if v9 {
            let mut v10: US0 = US0::US0_LoadField;
            US3::US3_Valid(v10.clone())
        } else {
            let mut v12: bool = 2u64 == v5;
            if v12 {
                let mut v13: US0 = US0::US0_AssigneeField;
                US3::US3_Valid(v13.clone())
            } else {
                US3::US3_Invalid
            }
        }
    };
    match &v18 {
        US3::US3_Invalid => {
            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("field must be estimate, load or assignee"); } LIT.with(|lit| lit.clone()) };
            (1i32, v22.clone())
        }
        US3::US3_Valid(v19) => {
            let mut v19: US0 = v19.clone();
            field_token_rewrite_1(v0.clone(), v1.clone(), v19.clone(), v3.clone())
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> (i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> (i32, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>| -> (i32, Rc<str>) {
        planning_rewrite_0(v0.clone(), v1.clone(), v2.clone(), v3.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_planning_rewrite(v0: &str, v1: &str, v2: &str, v3: &str) -> (i32, Rc<str>) {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3))
}
