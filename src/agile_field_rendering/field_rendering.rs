#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US1 {
    US1_EditTitle,
    US1_AppendTitle,
    US1_EditKind,
    US1_EditPriority,
    US1_EditDifficulty,
    US1_EditDeps,
    US1_EditTags,
}
impl US1 {
    fn tag(&self) -> i32 {
        match self {
            US1::US1_EditTitle => 0,
            US1::US1_AppendTitle => 1,
            US1::US1_EditKind => 2,
            US1::US1_EditPriority => 3,
            US1::US1_EditDifficulty => 4,
            US1::US1_EditDeps => 5,
            US1::US1_EditTags => 6,
        }
    }
}
impl Default for US1 {
    fn default() -> Self {
        US1::US1_EditTitle
    }
}
#[derive(Clone)]
enum US0 {
    US0_Valid(US1),
    US0_Invalid,
}
impl US0 {
    fn tag(&self) -> i32 {
        match self {
            US0::US0_Valid(..) => 0,
            US0::US0_Invalid => 1,
        }
    }
}
impl Default for US0 {
    fn default() -> Self {
        US0::US0_Valid(<US1>::default())
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
    US3_OpeningSentence,
    US3_FollowingSentence,
    US3_CallerPunctuated,
}
impl US3 {
    fn tag(&self) -> i32 {
        match self {
            US3::US3_OpeningSentence => 0,
            US3::US3_FollowingSentence => 1,
            US3::US3_CallerPunctuated => 2,
        }
    }
}
impl Default for US3 {
    fn default() -> Self {
        US3::US3_OpeningSentence
    }
}
#[derive(Clone)]
enum US5 {
    US5_AgileD0,
    US5_AgileD1,
    US5_AgileD2,
    US5_AgileD3,
    US5_AgileD4,
    US5_AgileD5,
    US5_AgileD6,
    US5_AgileD7,
    US5_AgileD8,
}
impl US5 {
    fn tag(&self) -> i32 {
        match self {
            US5::US5_AgileD0 => 0,
            US5::US5_AgileD1 => 1,
            US5::US5_AgileD2 => 2,
            US5::US5_AgileD3 => 3,
            US5::US5_AgileD4 => 4,
            US5::US5_AgileD5 => 5,
            US5::US5_AgileD6 => 6,
            US5::US5_AgileD7 => 7,
            US5::US5_AgileD8 => 8,
        }
    }
}
impl Default for US5 {
    fn default() -> Self {
        US5::US5_AgileD0
    }
}
#[derive(Clone)]
enum US4 {
    US4_Valid(US5),
    US4_Invalid,
}
impl US4 {
    fn tag(&self) -> i32 {
        match self {
            US4::US4_Valid(..) => 0,
            US4::US4_Invalid => 1,
        }
    }
}
impl Default for US4 {
    fn default() -> Self {
        US4::US4_Valid(<US5>::default())
    }
}
#[derive(Clone)]
enum US7 {
    US7_AgileEpic,
    US7_AgileFeature,
    US7_AgileStory,
    US7_AgileTaskKind,
}
impl US7 {
    fn tag(&self) -> i32 {
        match self {
            US7::US7_AgileEpic => 0,
            US7::US7_AgileFeature => 1,
            US7::US7_AgileStory => 2,
            US7::US7_AgileTaskKind => 3,
        }
    }
}
impl Default for US7 {
    fn default() -> Self {
        US7::US7_AgileEpic
    }
}
#[derive(Clone)]
enum US6 {
    US6_Valid(US7),
    US6_Invalid,
}
impl US6 {
    fn tag(&self) -> i32 {
        match self {
            US6::US6_Valid(..) => 0,
            US6::US6_Invalid => 1,
        }
    }
}
impl Default for US6 {
    fn default() -> Self {
        US6::US6_Valid(<US7>::default())
    }
}
#[derive(Clone)]
enum US9 {
    US9_AgileP0,
    US9_AgileP1,
    US9_AgileP2,
    US9_AgileP3,
    US9_AgileP4,
    US9_AgileP5,
}
impl US9 {
    fn tag(&self) -> i32 {
        match self {
            US9::US9_AgileP0 => 0,
            US9::US9_AgileP1 => 1,
            US9::US9_AgileP2 => 2,
            US9::US9_AgileP3 => 3,
            US9::US9_AgileP4 => 4,
            US9::US9_AgileP5 => 5,
        }
    }
}
impl Default for US9 {
    fn default() -> Self {
        US9::US9_AgileP0
    }
}
#[derive(Clone)]
enum US8 {
    US8_Valid(US9),
    US8_Invalid,
}
impl US8 {
    fn tag(&self) -> i32 {
        match self {
            US8::US8_Valid(..) => 0,
            US8::US8_Invalid => 1,
        }
    }
}
impl Default for US8 {
    fn default() -> Self {
        US8::US8_Valid(<US9>::default())
    }
}
fn parse_edit_field_1(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("title|title-append|kind|priority|difficulty|deps|tags"); } LIT.with(|lit| lit.clone()) };
    let mut v2: u64 = v1.split("|").position(|item| item == &*v0).map(|index| index as u64).unwrap_or(u64::MAX);
    let mut v3: bool = 0u64 == v2;
    if v3 {
        let mut v4: US1 = US1::US1_EditTitle;
        US0::US0_Valid(v4.clone())
    } else {
        let mut v6: bool = 1u64 == v2;
        if v6 {
            let mut v7: US1 = US1::US1_AppendTitle;
            US0::US0_Valid(v7.clone())
        } else {
            let mut v9: bool = 2u64 == v2;
            if v9 {
                let mut v10: US1 = US1::US1_EditKind;
                US0::US0_Valid(v10.clone())
            } else {
                let mut v12: bool = 3u64 == v2;
                if v12 {
                    let mut v13: US1 = US1::US1_EditPriority;
                    US0::US0_Valid(v13.clone())
                } else {
                    let mut v15: bool = 4u64 == v2;
                    if v15 {
                        let mut v16: US1 = US1::US1_EditDifficulty;
                        US0::US0_Valid(v16.clone())
                    } else {
                        let mut v18: bool = 5u64 == v2;
                        if v18 {
                            let mut v19: US1 = US1::US1_EditDeps;
                            US0::US0_Valid(v19.clone())
                        } else {
                            let mut v21: bool = 6u64 == v2;
                            if v21 {
                                let mut v22: US1 = US1::US1_EditTags;
                                US0::US0_Valid(v22.clone())
                            } else {
                                US0::US0_Invalid
                            }
                        }
                    }
                }
            }
        }
    }
}
fn edit_field_position_0(mut v0: Rc<str>) -> i32 {
    let mut v1: US0 = parse_edit_field_1(v0.clone());
    match &v1 {
        US0::US0_Invalid => {
            -1i32
        }
        US0::US0_Valid(v2) => {
            let mut v2: US1 = v2.clone();
            match &v2 {
                US1::US1_AppendTitle => {
                    1i32
                }
                US1::US1_EditDeps => {
                    7i32
                }
                US1::US1_EditDifficulty => {
                    4i32
                }
                US1::US1_EditKind => {
                    2i32
                }
                US1::US1_EditPriority => {
                    3i32
                }
                US1::US1_EditTags => {
                    8i32
                }
                US1::US1_EditTitle => {
                    1i32
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        edit_field_position_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn quoted_field_value_3(mut v0: Rc<str>) -> US2 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = std::rc::Rc::<str>::from(v0.replace(&*v2, &v3));
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = std::rc::Rc::<str>::from(v4.replace(&*v1, &v5));
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = std::rc::Rc::<str>::from(v6.replace(&*v7, &v8));
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = std::rc::Rc::<str>::from(v9.replace(&*v10, &v11));
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = std::rc::Rc::<str>::from(v12.replace(&*v13, &v14));
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v15, &*v1, &*v16, &*v16].concat());
    US2::US2_Valid(v17.clone())
}
fn edit_field_token_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: US0 = parse_edit_field_1(v0.clone());
    match &v3 {
        US0::US0_Invalid => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US0::US0_Valid(v5) => {
            let mut v5: US1 = v5.clone();
            let mut v195: US2 = match &v5 {
                US1::US1_AppendTitle => {
                    let mut v9: i32 = v1.len() as i32;
                    let mut v10: i32 = v9 - 1;
                    let mut v11: Rc<str> = std::rc::Rc::<str>::from(v1.get(0i32 as usize..v10 as usize).unwrap_or(""));
                    let mut v12: i32 = v11.len() as i32;
                    let mut v13: bool = v12 <= 1i32;
                    let mut v26: US3 = if v13 {
                        US3::US3_OpeningSentence
                    } else {
                        let mut v15: i32 = v2.as_bytes().get(0i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                        let mut v16: bool = v15 == 59i32;
                        let mut v22: bool = if v16 {
                            true
                        } else {
                            let mut v17: bool = v15 == 44i32;
                            if v17 {
                                true
                            } else {
                                let mut v18: bool = v15 == 46i32;
                                if v18 {
                                    true
                                } else {
                                    let mut v19: bool = v15 == 32i32;
                                    v19
                                }
                            }
                        };
                        if v22 {
                            US3::US3_CallerPunctuated
                        } else {
                            US3::US3_FollowingSentence
                        }
                    };
                    let mut v32: Rc<str> = match &v26 {
                        US3::US3_CallerPunctuated => {
                            let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v28.clone()
                        }
                        US3::US3_FollowingSentence => {
                            let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
                            v29.clone()
                        }
                        US3::US3_OpeningSentence => {
                            let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v27.clone()
                        }
                    };
                    let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
                    let mut v35: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v33, &v34));
                    let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                    let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
                    let mut v38: Rc<str> = std::rc::Rc::<str>::from(v35.replace(&*v36, &v37));
                    let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
                    let mut v41: Rc<str> = std::rc::Rc::<str>::from(v38.replace(&*v39, &v40));
                    let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                    let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
                    let mut v44: Rc<str> = std::rc::Rc::<str>::from(v41.replace(&*v42, &v43));
                    let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                    let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
                    let mut v47: Rc<str> = std::rc::Rc::<str>::from(v44.replace(&*v45, &v46));
                    let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v49: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v32, &*v47, &*v36, &*v48].concat());
                    US2::US2_Valid(v49.clone())
                }
                US1::US1_EditDeps => {
                    quoted_field_value_3(v2.clone())
                }
                US1::US1_EditDifficulty => {
                    let mut v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D0|D1|D2|D3|D4|D5|D6|D7|D8"); } LIT.with(|lit| lit.clone()) };
                    let mut v128: u64 = v127.split("|").position(|item| item == &*v2).map(|index| index as u64).unwrap_or(u64::MAX);
                    let mut v129: bool = 0u64 == v128;
                    let mut v165: US4 = if v129 {
                        let mut v130: US5 = US5::US5_AgileD0;
                        US4::US4_Valid(v130.clone())
                    } else {
                        let mut v132: bool = 1u64 == v128;
                        if v132 {
                            let mut v133: US5 = US5::US5_AgileD1;
                            US4::US4_Valid(v133.clone())
                        } else {
                            let mut v135: bool = 2u64 == v128;
                            if v135 {
                                let mut v136: US5 = US5::US5_AgileD2;
                                US4::US4_Valid(v136.clone())
                            } else {
                                let mut v138: bool = 3u64 == v128;
                                if v138 {
                                    let mut v139: US5 = US5::US5_AgileD3;
                                    US4::US4_Valid(v139.clone())
                                } else {
                                    let mut v141: bool = 4u64 == v128;
                                    if v141 {
                                        let mut v142: US5 = US5::US5_AgileD4;
                                        US4::US4_Valid(v142.clone())
                                    } else {
                                        let mut v144: bool = 5u64 == v128;
                                        if v144 {
                                            let mut v145: US5 = US5::US5_AgileD5;
                                            US4::US4_Valid(v145.clone())
                                        } else {
                                            let mut v147: bool = 6u64 == v128;
                                            if v147 {
                                                let mut v148: US5 = US5::US5_AgileD6;
                                                US4::US4_Valid(v148.clone())
                                            } else {
                                                let mut v150: bool = 7u64 == v128;
                                                if v150 {
                                                    let mut v151: US5 = US5::US5_AgileD7;
                                                    US4::US4_Valid(v151.clone())
                                                } else {
                                                    let mut v153: bool = 8u64 == v128;
                                                    if v153 {
                                                        let mut v154: US5 = US5::US5_AgileD8;
                                                        US4::US4_Valid(v154.clone())
                                                    } else {
                                                        US4::US4_Invalid
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    };
                    match &v165 {
                        US4::US4_Invalid => {
                            US2::US2_Invalid
                        }
                        US4::US4_Valid(v166) => {
                            let mut v166: US5 = v166.clone();
                            let mut v184: Rc<str> = match &v166 {
                                US5::US5_AgileD0 => {
                                    let mut v167: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D0"); } LIT.with(|lit| lit.clone()) };
                                    v167.clone()
                                }
                                US5::US5_AgileD1 => {
                                    let mut v168: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D1"); } LIT.with(|lit| lit.clone()) };
                                    v168.clone()
                                }
                                US5::US5_AgileD2 => {
                                    let mut v169: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D2"); } LIT.with(|lit| lit.clone()) };
                                    v169.clone()
                                }
                                US5::US5_AgileD3 => {
                                    let mut v170: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D3"); } LIT.with(|lit| lit.clone()) };
                                    v170.clone()
                                }
                                US5::US5_AgileD4 => {
                                    let mut v171: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D4"); } LIT.with(|lit| lit.clone()) };
                                    v171.clone()
                                }
                                US5::US5_AgileD5 => {
                                    let mut v172: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D5"); } LIT.with(|lit| lit.clone()) };
                                    v172.clone()
                                }
                                US5::US5_AgileD6 => {
                                    let mut v173: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D6"); } LIT.with(|lit| lit.clone()) };
                                    v173.clone()
                                }
                                US5::US5_AgileD7 => {
                                    let mut v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D7"); } LIT.with(|lit| lit.clone()) };
                                    v174.clone()
                                }
                                US5::US5_AgileD8 => {
                                    let mut v175: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D8"); } LIT.with(|lit| lit.clone()) };
                                    v175.clone()
                                }
                            };
                            US2::US2_Valid(v184.clone())
                        }
                    }
                }
                US1::US1_EditKind => {
                    let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Epic|Feature|Story|TaskKind"); } LIT.with(|lit| lit.clone()) };
                    let mut v52: u64 = v51.split("|").position(|item| item == &*v2).map(|index| index as u64).unwrap_or(u64::MAX);
                    let mut v53: bool = 0u64 == v52;
                    let mut v69: US6 = if v53 {
                        let mut v54: US7 = US7::US7_AgileEpic;
                        US6::US6_Valid(v54.clone())
                    } else {
                        let mut v56: bool = 1u64 == v52;
                        if v56 {
                            let mut v57: US7 = US7::US7_AgileFeature;
                            US6::US6_Valid(v57.clone())
                        } else {
                            let mut v59: bool = 2u64 == v52;
                            if v59 {
                                let mut v60: US7 = US7::US7_AgileStory;
                                US6::US6_Valid(v60.clone())
                            } else {
                                let mut v62: bool = 3u64 == v52;
                                if v62 {
                                    let mut v63: US7 = US7::US7_AgileTaskKind;
                                    US6::US6_Valid(v63.clone())
                                } else {
                                    US6::US6_Invalid
                                }
                            }
                        }
                    };
                    match &v69 {
                        US6::US6_Invalid => {
                            US2::US2_Invalid
                        }
                        US6::US6_Valid(v70) => {
                            let mut v70: US7 = v70.clone();
                            let mut v78: Rc<str> = match &v70 {
                                US7::US7_AgileEpic => {
                                    let mut v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Epic"); } LIT.with(|lit| lit.clone()) };
                                    v71.clone()
                                }
                                US7::US7_AgileFeature => {
                                    let mut v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Feature"); } LIT.with(|lit| lit.clone()) };
                                    v72.clone()
                                }
                                US7::US7_AgileStory => {
                                    let mut v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Story"); } LIT.with(|lit| lit.clone()) };
                                    v73.clone()
                                }
                                US7::US7_AgileTaskKind => {
                                    let mut v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TaskKind"); } LIT.with(|lit| lit.clone()) };
                                    v74.clone()
                                }
                            };
                            US2::US2_Valid(v78.clone())
                        }
                    }
                }
                US1::US1_EditPriority => {
                    let mut v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P0|P1|P2|P3|P4|P5"); } LIT.with(|lit| lit.clone()) };
                    let mut v84: u64 = v83.split("|").position(|item| item == &*v2).map(|index| index as u64).unwrap_or(u64::MAX);
                    let mut v85: bool = 0u64 == v84;
                    let mut v109: US8 = if v85 {
                        let mut v86: US9 = US9::US9_AgileP0;
                        US8::US8_Valid(v86.clone())
                    } else {
                        let mut v88: bool = 1u64 == v84;
                        if v88 {
                            let mut v89: US9 = US9::US9_AgileP1;
                            US8::US8_Valid(v89.clone())
                        } else {
                            let mut v91: bool = 2u64 == v84;
                            if v91 {
                                let mut v92: US9 = US9::US9_AgileP2;
                                US8::US8_Valid(v92.clone())
                            } else {
                                let mut v94: bool = 3u64 == v84;
                                if v94 {
                                    let mut v95: US9 = US9::US9_AgileP3;
                                    US8::US8_Valid(v95.clone())
                                } else {
                                    let mut v97: bool = 4u64 == v84;
                                    if v97 {
                                        let mut v98: US9 = US9::US9_AgileP4;
                                        US8::US8_Valid(v98.clone())
                                    } else {
                                        let mut v100: bool = 5u64 == v84;
                                        if v100 {
                                            let mut v101: US9 = US9::US9_AgileP5;
                                            US8::US8_Valid(v101.clone())
                                        } else {
                                            US8::US8_Invalid
                                        }
                                    }
                                }
                            }
                        }
                    };
                    match &v109 {
                        US8::US8_Invalid => {
                            US2::US2_Invalid
                        }
                        US8::US8_Valid(v110) => {
                            let mut v110: US9 = v110.clone();
                            let mut v122: Rc<str> = match &v110 {
                                US9::US9_AgileP0 => {
                                    let mut v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P0"); } LIT.with(|lit| lit.clone()) };
                                    v111.clone()
                                }
                                US9::US9_AgileP1 => {
                                    let mut v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P1"); } LIT.with(|lit| lit.clone()) };
                                    v112.clone()
                                }
                                US9::US9_AgileP2 => {
                                    let mut v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P2"); } LIT.with(|lit| lit.clone()) };
                                    v113.clone()
                                }
                                US9::US9_AgileP3 => {
                                    let mut v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P3"); } LIT.with(|lit| lit.clone()) };
                                    v114.clone()
                                }
                                US9::US9_AgileP4 => {
                                    let mut v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P4"); } LIT.with(|lit| lit.clone()) };
                                    v115.clone()
                                }
                                US9::US9_AgileP5 => {
                                    let mut v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P5"); } LIT.with(|lit| lit.clone()) };
                                    v116.clone()
                                }
                            };
                            US2::US2_Valid(v122.clone())
                        }
                    }
                }
                US1::US1_EditTags => {
                    quoted_field_value_3(v2.clone())
                }
                US1::US1_EditTitle => {
                    quoted_field_value_3(v2.clone())
                }
            };
            match &v195 {
                US2::US2_Invalid => {
                    let mut v197: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v197.clone()
                }
                US2::US2_Valid(v196) => {
                    let mut v196: Rc<str> = v196.clone();
                    v196.clone()
                }
            }
        }
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>| -> Rc<str> {
        edit_field_token_2(v0.clone(), v1.clone(), v2.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn edit_field_rule_4(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: US0 = parse_edit_field_1(v0.clone());
    match &v1 {
        US0::US0_Invalid => {
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("field must be title, title-append, kind, priority, difficulty, deps or tags"); } LIT.with(|lit| lit.clone()) };
            v10.clone()
        }
        US0::US0_Valid(v2) => {
            let mut v2: US1 = v2.clone();
            match &v2 {
                US1::US1_EditDifficulty => {
                    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("difficulty must be D0..D8"); } LIT.with(|lit| lit.clone()) };
                    v5.clone()
                }
                US1::US1_EditKind => {
                    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kind must be Epic, Feature, Story or TaskKind"); } LIT.with(|lit| lit.clone()) };
                    v3.clone()
                }
                US1::US1_EditPriority => {
                    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("priority must be P0..P5"); } LIT.with(|lit| lit.clone()) };
                    v4.clone()
                }
                _ => {
                    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the value was rejected"); } LIT.with(|lit| lit.clone()) };
                    v6.clone()
                }
            }
        }
    }
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        edit_field_rule_4(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_edit_field_position(v0: &str) -> i32 {
    closure0()(Rc::<str>::from(v0))
}
pub fn eoie_agile_edit_field_token(v0: &str, v1: &str, v2: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2))
}
pub fn eoie_agile_edit_field_rule(v0: &str) -> Rc<str> {
    closure2()(Rc::<str>::from(v0))
}
