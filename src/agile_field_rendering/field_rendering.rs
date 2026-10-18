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
            let mut v173: US2 = match &v5 {
                US1::US1_AppendTitle => {
                    let mut v9: i32 = v1.len() as i32;
                    let mut v10: i32 = v9.wrapping_sub(1i32);
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
                    let mut v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D0"); } LIT.with(|lit| lit.clone()) };
                    let mut v116: bool = v2 == v115 ;
                    let mut v120: US4 = if v116 {
                        let mut v117: US5 = US5::US5_AgileD0;
                        US4::US4_Valid(v117.clone())
                    } else {
                        US4::US4_Invalid
                    };
                    let mut v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D1"); } LIT.with(|lit| lit.clone()) };
                    let mut v122: bool = v2 == v121 ;
                    let mut v125: US4 = if v122 {
                        let mut v123: US5 = US5::US5_AgileD1;
                        US4::US4_Valid(v123.clone())
                    } else {
                        v120.clone()
                    };
                    let mut v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D2"); } LIT.with(|lit| lit.clone()) };
                    let mut v127: bool = v2 == v126 ;
                    let mut v130: US4 = if v127 {
                        let mut v128: US5 = US5::US5_AgileD2;
                        US4::US4_Valid(v128.clone())
                    } else {
                        v125.clone()
                    };
                    let mut v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D3"); } LIT.with(|lit| lit.clone()) };
                    let mut v132: bool = v2 == v131 ;
                    let mut v135: US4 = if v132 {
                        let mut v133: US5 = US5::US5_AgileD3;
                        US4::US4_Valid(v133.clone())
                    } else {
                        v130.clone()
                    };
                    let mut v136: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D4"); } LIT.with(|lit| lit.clone()) };
                    let mut v137: bool = v2 == v136 ;
                    let mut v140: US4 = if v137 {
                        let mut v138: US5 = US5::US5_AgileD4;
                        US4::US4_Valid(v138.clone())
                    } else {
                        v135.clone()
                    };
                    let mut v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D5"); } LIT.with(|lit| lit.clone()) };
                    let mut v142: bool = v2 == v141 ;
                    let mut v145: US4 = if v142 {
                        let mut v143: US5 = US5::US5_AgileD5;
                        US4::US4_Valid(v143.clone())
                    } else {
                        v140.clone()
                    };
                    let mut v146: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D6"); } LIT.with(|lit| lit.clone()) };
                    let mut v147: bool = v2 == v146 ;
                    let mut v150: US4 = if v147 {
                        let mut v148: US5 = US5::US5_AgileD6;
                        US4::US4_Valid(v148.clone())
                    } else {
                        v145.clone()
                    };
                    let mut v151: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D7"); } LIT.with(|lit| lit.clone()) };
                    let mut v152: bool = v2 == v151 ;
                    let mut v155: US4 = if v152 {
                        let mut v153: US5 = US5::US5_AgileD7;
                        US4::US4_Valid(v153.clone())
                    } else {
                        v150.clone()
                    };
                    let mut v156: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D8"); } LIT.with(|lit| lit.clone()) };
                    let mut v157: bool = v2 == v156 ;
                    let mut v160: US4 = if v157 {
                        let mut v158: US5 = US5::US5_AgileD8;
                        US4::US4_Valid(v158.clone())
                    } else {
                        v155.clone()
                    };
                    match &v160 {
                        US4::US4_Invalid => {
                            US2::US2_Invalid
                        }
                        US4::US4_Valid(v161) => {
                            let mut v161: US5 = v161.clone();
                            let mut v162: Rc<str> = match &v161 {
                                US5::US5_AgileD0 => {
                                    v115.clone()
                                }
                                US5::US5_AgileD1 => {
                                    v121.clone()
                                }
                                US5::US5_AgileD2 => {
                                    v126.clone()
                                }
                                US5::US5_AgileD3 => {
                                    v131.clone()
                                }
                                US5::US5_AgileD4 => {
                                    v136.clone()
                                }
                                US5::US5_AgileD5 => {
                                    v141.clone()
                                }
                                US5::US5_AgileD6 => {
                                    v146.clone()
                                }
                                US5::US5_AgileD7 => {
                                    v151.clone()
                                }
                                US5::US5_AgileD8 => {
                                    v156.clone()
                                }
                            };
                            US2::US2_Valid(v162.clone())
                        }
                    }
                }
                US1::US1_EditKind => {
                    let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Epic"); } LIT.with(|lit| lit.clone()) };
                    let mut v52: bool = v2 == v51 ;
                    let mut v56: US6 = if v52 {
                        let mut v53: US7 = US7::US7_AgileEpic;
                        US6::US6_Valid(v53.clone())
                    } else {
                        US6::US6_Invalid
                    };
                    let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Feature"); } LIT.with(|lit| lit.clone()) };
                    let mut v58: bool = v2 == v57 ;
                    let mut v61: US6 = if v58 {
                        let mut v59: US7 = US7::US7_AgileFeature;
                        US6::US6_Valid(v59.clone())
                    } else {
                        v56.clone()
                    };
                    let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Story"); } LIT.with(|lit| lit.clone()) };
                    let mut v63: bool = v2 == v62 ;
                    let mut v66: US6 = if v63 {
                        let mut v64: US7 = US7::US7_AgileStory;
                        US6::US6_Valid(v64.clone())
                    } else {
                        v61.clone()
                    };
                    let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TaskKind"); } LIT.with(|lit| lit.clone()) };
                    let mut v68: bool = v2 == v67 ;
                    let mut v71: US6 = if v68 {
                        let mut v69: US7 = US7::US7_AgileTaskKind;
                        US6::US6_Valid(v69.clone())
                    } else {
                        v66.clone()
                    };
                    match &v71 {
                        US6::US6_Invalid => {
                            US2::US2_Invalid
                        }
                        US6::US6_Valid(v72) => {
                            let mut v72: US7 = v72.clone();
                            let mut v73: Rc<str> = match &v72 {
                                US7::US7_AgileEpic => {
                                    v51.clone()
                                }
                                US7::US7_AgileFeature => {
                                    v57.clone()
                                }
                                US7::US7_AgileStory => {
                                    v62.clone()
                                }
                                US7::US7_AgileTaskKind => {
                                    v67.clone()
                                }
                            };
                            US2::US2_Valid(v73.clone())
                        }
                    }
                }
                US1::US1_EditPriority => {
                    let mut v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P0"); } LIT.with(|lit| lit.clone()) };
                    let mut v79: bool = v2 == v78 ;
                    let mut v83: US8 = if v79 {
                        let mut v80: US9 = US9::US9_AgileP0;
                        US8::US8_Valid(v80.clone())
                    } else {
                        US8::US8_Invalid
                    };
                    let mut v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P1"); } LIT.with(|lit| lit.clone()) };
                    let mut v85: bool = v2 == v84 ;
                    let mut v88: US8 = if v85 {
                        let mut v86: US9 = US9::US9_AgileP1;
                        US8::US8_Valid(v86.clone())
                    } else {
                        v83.clone()
                    };
                    let mut v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P2"); } LIT.with(|lit| lit.clone()) };
                    let mut v90: bool = v2 == v89 ;
                    let mut v93: US8 = if v90 {
                        let mut v91: US9 = US9::US9_AgileP2;
                        US8::US8_Valid(v91.clone())
                    } else {
                        v88.clone()
                    };
                    let mut v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P3"); } LIT.with(|lit| lit.clone()) };
                    let mut v95: bool = v2 == v94 ;
                    let mut v98: US8 = if v95 {
                        let mut v96: US9 = US9::US9_AgileP3;
                        US8::US8_Valid(v96.clone())
                    } else {
                        v93.clone()
                    };
                    let mut v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P4"); } LIT.with(|lit| lit.clone()) };
                    let mut v100: bool = v2 == v99 ;
                    let mut v103: US8 = if v100 {
                        let mut v101: US9 = US9::US9_AgileP4;
                        US8::US8_Valid(v101.clone())
                    } else {
                        v98.clone()
                    };
                    let mut v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P5"); } LIT.with(|lit| lit.clone()) };
                    let mut v105: bool = v2 == v104 ;
                    let mut v108: US8 = if v105 {
                        let mut v106: US9 = US9::US9_AgileP5;
                        US8::US8_Valid(v106.clone())
                    } else {
                        v103.clone()
                    };
                    match &v108 {
                        US8::US8_Invalid => {
                            US2::US2_Invalid
                        }
                        US8::US8_Valid(v109) => {
                            let mut v109: US9 = v109.clone();
                            let mut v110: Rc<str> = match &v109 {
                                US9::US9_AgileP0 => {
                                    v78.clone()
                                }
                                US9::US9_AgileP1 => {
                                    v84.clone()
                                }
                                US9::US9_AgileP2 => {
                                    v89.clone()
                                }
                                US9::US9_AgileP3 => {
                                    v94.clone()
                                }
                                US9::US9_AgileP4 => {
                                    v99.clone()
                                }
                                US9::US9_AgileP5 => {
                                    v104.clone()
                                }
                            };
                            US2::US2_Valid(v110.clone())
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
            match &v173 {
                US2::US2_Invalid => {
                    let mut v175: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v175.clone()
                }
                US2::US2_Valid(v174) => {
                    let mut v174: Rc<str> = v174.clone();
                    v174.clone()
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
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("field must be title, title-append, kind, priority, difficulty, deps, tags, estimate, load or assignee"); } LIT.with(|lit| lit.clone()) };
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
