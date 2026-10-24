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
#[derive(Clone)]
enum US0 {
    US0_Valid(US1),
    US0_Invalid,
}
#[derive(Clone)]
enum US2 {
    US2_Valid(Rc<str>),
    US2_Invalid,
}
#[derive(Clone)]
enum US3 {
    US3_OpeningSentence,
    US3_FollowingSentence,
    US3_CallerPunctuated,
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
#[derive(Clone)]
enum US4 {
    US4_Valid(US5),
    US4_Invalid,
}
#[derive(Clone)]
enum US7 {
    US7_AgileEpic,
    US7_AgileFeature,
    US7_AgileStory,
    US7_AgileTaskKind,
}
#[derive(Clone)]
enum US6 {
    US6_Valid(US7),
    US6_Invalid,
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
#[derive(Clone)]
enum US8 {
    US8_Valid(US9),
    US8_Invalid,
}
fn parse_edit_field_1(mut v0: Rc<str>) -> US0 {
    let mut v1: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("title"); } LIT.with(|lit| lit.clone()) };
    let mut v5: US0 = if v1 {
        let mut v2: US1 = US1::US1_EditTitle;
        US0::US0_Valid(v2.clone())
    } else {
        US0::US0_Invalid
    };
    let mut v6: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("title-append"); } LIT.with(|lit| lit.clone()) };
    let mut v9: US0 = if v6 {
        let mut v7: US1 = US1::US1_AppendTitle;
        US0::US0_Valid(v7.clone())
    } else {
        v5.clone()
    };
    let mut v10: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kind"); } LIT.with(|lit| lit.clone()) };
    let mut v13: US0 = if v10 {
        let mut v11: US1 = US1::US1_EditKind;
        US0::US0_Valid(v11.clone())
    } else {
        v9.clone()
    };
    let mut v14: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("priority"); } LIT.with(|lit| lit.clone()) };
    let mut v17: US0 = if v14 {
        let mut v15: US1 = US1::US1_EditPriority;
        US0::US0_Valid(v15.clone())
    } else {
        v13.clone()
    };
    let mut v18: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("difficulty"); } LIT.with(|lit| lit.clone()) };
    let mut v21: US0 = if v18 {
        let mut v19: US1 = US1::US1_EditDifficulty;
        US0::US0_Valid(v19.clone())
    } else {
        v17.clone()
    };
    let mut v22: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("deps"); } LIT.with(|lit| lit.clone()) };
    let mut v25: US0 = if v22 {
        let mut v23: US1 = US1::US1_EditDeps;
        US0::US0_Valid(v23.clone())
    } else {
        v21.clone()
    };
    let mut v26: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tags"); } LIT.with(|lit| lit.clone()) };
    if v26 {
        let mut v27: US1 = US1::US1_EditTags;
        US0::US0_Valid(v27.clone())
    } else {
        v25.clone()
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
            let mut v174: US2 = match &v5 {
                US1::US1_AppendTitle => {
                    let mut v9: i32 = (v1.clone().len() as i32);
                    let mut v10: i32 = v9.wrapping_sub(1i32);
                    let mut v11: Rc<str> = std::rc::Rc::<str>::from(v1.get(0i32 as usize..v10 as usize).unwrap_or(""));
                    let mut v12: i32 = (v11.clone().len() as i32);
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
                    let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v32));
                    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                    let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
                    let mut v36: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v34, &v35));
                    let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                    let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
                    let mut v39: Rc<str> = std::rc::Rc::<str>::from(v36.replace(&*v37, &v38));
                    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
                    let mut v42: Rc<str> = std::rc::Rc::<str>::from(v39.replace(&*v40, &v41));
                    let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                    let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
                    let mut v45: Rc<str> = std::rc::Rc::<str>::from(v42.replace(&*v43, &v44));
                    let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                    let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
                    let mut v48: Rc<str> = std::rc::Rc::<str>::from(v45.replace(&*v46, &v47));
                    let mut v49: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v48));
                    let mut v50: Rc<str> = Rc::<str>::from(format!("{}{}", v49, v37));
                    US2::US2_Valid(v50.clone())
                }
                US1::US1_EditDeps => {
                    quoted_field_value_3(v2.clone())
                }
                US1::US1_EditDifficulty => {
                    let mut v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D0"); } LIT.with(|lit| lit.clone()) };
                    let mut v117: bool = v2 == v116 ;
                    let mut v121: US4 = if v117 {
                        let mut v118: US5 = US5::US5_AgileD0;
                        US4::US4_Valid(v118.clone())
                    } else {
                        US4::US4_Invalid
                    };
                    let mut v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D1"); } LIT.with(|lit| lit.clone()) };
                    let mut v123: bool = v2 == v122 ;
                    let mut v126: US4 = if v123 {
                        let mut v124: US5 = US5::US5_AgileD1;
                        US4::US4_Valid(v124.clone())
                    } else {
                        v121.clone()
                    };
                    let mut v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D2"); } LIT.with(|lit| lit.clone()) };
                    let mut v128: bool = v2 == v127 ;
                    let mut v131: US4 = if v128 {
                        let mut v129: US5 = US5::US5_AgileD2;
                        US4::US4_Valid(v129.clone())
                    } else {
                        v126.clone()
                    };
                    let mut v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D3"); } LIT.with(|lit| lit.clone()) };
                    let mut v133: bool = v2 == v132 ;
                    let mut v136: US4 = if v133 {
                        let mut v134: US5 = US5::US5_AgileD3;
                        US4::US4_Valid(v134.clone())
                    } else {
                        v131.clone()
                    };
                    let mut v137: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D4"); } LIT.with(|lit| lit.clone()) };
                    let mut v138: bool = v2 == v137 ;
                    let mut v141: US4 = if v138 {
                        let mut v139: US5 = US5::US5_AgileD4;
                        US4::US4_Valid(v139.clone())
                    } else {
                        v136.clone()
                    };
                    let mut v142: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D5"); } LIT.with(|lit| lit.clone()) };
                    let mut v143: bool = v2 == v142 ;
                    let mut v146: US4 = if v143 {
                        let mut v144: US5 = US5::US5_AgileD5;
                        US4::US4_Valid(v144.clone())
                    } else {
                        v141.clone()
                    };
                    let mut v147: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D6"); } LIT.with(|lit| lit.clone()) };
                    let mut v148: bool = v2 == v147 ;
                    let mut v151: US4 = if v148 {
                        let mut v149: US5 = US5::US5_AgileD6;
                        US4::US4_Valid(v149.clone())
                    } else {
                        v146.clone()
                    };
                    let mut v152: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D7"); } LIT.with(|lit| lit.clone()) };
                    let mut v153: bool = v2 == v152 ;
                    let mut v156: US4 = if v153 {
                        let mut v154: US5 = US5::US5_AgileD7;
                        US4::US4_Valid(v154.clone())
                    } else {
                        v151.clone()
                    };
                    let mut v157: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D8"); } LIT.with(|lit| lit.clone()) };
                    let mut v158: bool = v2 == v157 ;
                    let mut v161: US4 = if v158 {
                        let mut v159: US5 = US5::US5_AgileD8;
                        US4::US4_Valid(v159.clone())
                    } else {
                        v156.clone()
                    };
                    match &v161 {
                        US4::US4_Invalid => {
                            US2::US2_Invalid
                        }
                        US4::US4_Valid(v162) => {
                            let mut v162: US5 = v162.clone();
                            let mut v163: Rc<str> = match &v162 {
                                US5::US5_AgileD0 => {
                                    v116.clone()
                                }
                                US5::US5_AgileD1 => {
                                    v122.clone()
                                }
                                US5::US5_AgileD2 => {
                                    v127.clone()
                                }
                                US5::US5_AgileD3 => {
                                    v132.clone()
                                }
                                US5::US5_AgileD4 => {
                                    v137.clone()
                                }
                                US5::US5_AgileD5 => {
                                    v142.clone()
                                }
                                US5::US5_AgileD6 => {
                                    v147.clone()
                                }
                                US5::US5_AgileD7 => {
                                    v152.clone()
                                }
                                US5::US5_AgileD8 => {
                                    v157.clone()
                                }
                            };
                            US2::US2_Valid(v163.clone())
                        }
                    }
                }
                US1::US1_EditKind => {
                    let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Epic"); } LIT.with(|lit| lit.clone()) };
                    let mut v53: bool = v2 == v52 ;
                    let mut v57: US6 = if v53 {
                        let mut v54: US7 = US7::US7_AgileEpic;
                        US6::US6_Valid(v54.clone())
                    } else {
                        US6::US6_Invalid
                    };
                    let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Feature"); } LIT.with(|lit| lit.clone()) };
                    let mut v59: bool = v2 == v58 ;
                    let mut v62: US6 = if v59 {
                        let mut v60: US7 = US7::US7_AgileFeature;
                        US6::US6_Valid(v60.clone())
                    } else {
                        v57.clone()
                    };
                    let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Story"); } LIT.with(|lit| lit.clone()) };
                    let mut v64: bool = v2 == v63 ;
                    let mut v67: US6 = if v64 {
                        let mut v65: US7 = US7::US7_AgileStory;
                        US6::US6_Valid(v65.clone())
                    } else {
                        v62.clone()
                    };
                    let mut v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TaskKind"); } LIT.with(|lit| lit.clone()) };
                    let mut v69: bool = v2 == v68 ;
                    let mut v72: US6 = if v69 {
                        let mut v70: US7 = US7::US7_AgileTaskKind;
                        US6::US6_Valid(v70.clone())
                    } else {
                        v67.clone()
                    };
                    match &v72 {
                        US6::US6_Invalid => {
                            US2::US2_Invalid
                        }
                        US6::US6_Valid(v73) => {
                            let mut v73: US7 = v73.clone();
                            let mut v74: Rc<str> = match &v73 {
                                US7::US7_AgileEpic => {
                                    v52.clone()
                                }
                                US7::US7_AgileFeature => {
                                    v58.clone()
                                }
                                US7::US7_AgileStory => {
                                    v63.clone()
                                }
                                US7::US7_AgileTaskKind => {
                                    v68.clone()
                                }
                            };
                            US2::US2_Valid(v74.clone())
                        }
                    }
                }
                US1::US1_EditPriority => {
                    let mut v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P0"); } LIT.with(|lit| lit.clone()) };
                    let mut v80: bool = v2 == v79 ;
                    let mut v84: US8 = if v80 {
                        let mut v81: US9 = US9::US9_AgileP0;
                        US8::US8_Valid(v81.clone())
                    } else {
                        US8::US8_Invalid
                    };
                    let mut v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P1"); } LIT.with(|lit| lit.clone()) };
                    let mut v86: bool = v2 == v85 ;
                    let mut v89: US8 = if v86 {
                        let mut v87: US9 = US9::US9_AgileP1;
                        US8::US8_Valid(v87.clone())
                    } else {
                        v84.clone()
                    };
                    let mut v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P2"); } LIT.with(|lit| lit.clone()) };
                    let mut v91: bool = v2 == v90 ;
                    let mut v94: US8 = if v91 {
                        let mut v92: US9 = US9::US9_AgileP2;
                        US8::US8_Valid(v92.clone())
                    } else {
                        v89.clone()
                    };
                    let mut v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P3"); } LIT.with(|lit| lit.clone()) };
                    let mut v96: bool = v2 == v95 ;
                    let mut v99: US8 = if v96 {
                        let mut v97: US9 = US9::US9_AgileP3;
                        US8::US8_Valid(v97.clone())
                    } else {
                        v94.clone()
                    };
                    let mut v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P4"); } LIT.with(|lit| lit.clone()) };
                    let mut v101: bool = v2 == v100 ;
                    let mut v104: US8 = if v101 {
                        let mut v102: US9 = US9::US9_AgileP4;
                        US8::US8_Valid(v102.clone())
                    } else {
                        v99.clone()
                    };
                    let mut v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P5"); } LIT.with(|lit| lit.clone()) };
                    let mut v106: bool = v2 == v105 ;
                    let mut v109: US8 = if v106 {
                        let mut v107: US9 = US9::US9_AgileP5;
                        US8::US8_Valid(v107.clone())
                    } else {
                        v104.clone()
                    };
                    match &v109 {
                        US8::US8_Invalid => {
                            US2::US2_Invalid
                        }
                        US8::US8_Valid(v110) => {
                            let mut v110: US9 = v110.clone();
                            let mut v111: Rc<str> = match &v110 {
                                US9::US9_AgileP0 => {
                                    v79.clone()
                                }
                                US9::US9_AgileP1 => {
                                    v85.clone()
                                }
                                US9::US9_AgileP2 => {
                                    v90.clone()
                                }
                                US9::US9_AgileP3 => {
                                    v95.clone()
                                }
                                US9::US9_AgileP4 => {
                                    v100.clone()
                                }
                                US9::US9_AgileP5 => {
                                    v105.clone()
                                }
                            };
                            US2::US2_Valid(v111.clone())
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
            match &v174 {
                US2::US2_Invalid => {
                    let mut v176: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v176.clone()
                }
                US2::US2_Valid(v175) => {
                    let mut v175: Rc<str> = v175.clone();
                    v175.clone()
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
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("field must be "); } LIT.with(|lit| lit.clone()) };
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("title, title-append, kind, priority, difficulty, deps, tags"); } LIT.with(|lit| lit.clone()) };
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("field must be title, title-append, kind, priority, difficulty, deps, tags"); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", estimate, load or assignee"); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("field must be title, title-append, kind, priority, difficulty, deps, tags, estimate, load or assignee"); } LIT.with(|lit| lit.clone()) };
            v14.clone()
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
