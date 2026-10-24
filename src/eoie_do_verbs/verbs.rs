#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Edits,
    US0_Reads,
    US0_Facts,
    US0_Runs,
    US0_Writes,
    US0_Git,
    US0_Jobs,
    US0_Labs,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(US0, Rc<UH0>),
}
#[derive(Clone)]
enum US1 {
    US1_Some(US0),
    US1_None,
}
#[derive(Clone)]
enum US4 {
    US4_Search,
    US4_ReadAround,
    US4_ListFiles,
    US4_Head,
    US4_Tail,
    US4_Wait,
}
#[derive(Clone)]
enum US5 {
    US5_HashVerb,
    US5_EnvVerb,
    US5_ExistsVerb,
    US5_NearestVerb,
    US5_JsonVerb,
    US5_LinesVerb,
    US5_TableVerb,
    US5_DirsVerb,
    US5_TagVerb,
    US5_CountVerb,
    US5_DiffVerb,
}
#[derive(Clone)]
enum US6 {
    US6_RunVerb,
    US6_ErrorsVerb,
    US6_ProcessesVerb,
    US6_StopVerb,
}
#[derive(Clone)]
enum US7 {
    US7_AppendVerb,
    US7_CopyVerb,
    US7_MoveVerb,
}
#[derive(Clone)]
enum US3 {
    US3_ReadVerb(US4),
    US3_FactVerb(US5),
    US3_RunStepVerb(US6),
    US3_WriteVerb(US7),
    US3_GitVerb,
    US3_JobVerb,
    US3_LabVerb,
}
#[derive(Clone)]
enum US2 {
    US2_Some(US3),
    US2_None,
}
#[derive(Clone)]
enum US8 {
    US8_Some(US6),
    US8_None,
}
#[derive(Clone)]
enum US9 {
    US9_Some(US7),
    US9_None,
}
#[derive(Clone)]
enum US10 {
    US10_Some(US4),
    US10_None,
}
#[derive(Clone)]
enum US11 {
    US11_Some(US5),
    US11_None,
}
fn records_of_0(mut v0: Rc<str>) -> std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> {
    let mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new((&*v0).split(char::from(30u8)).map(|record| record.split(char::from(31u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
    v1.clone()
}
fn first_seen_2(mut v0: Rc<UH0>, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: bool, mut v6: bool, mut v7: bool) -> US0 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v9, v10) => {
                let mut v9: US0 = v9.clone();
                let mut v10: Rc<UH0> = v10.clone();
                let mut v18: bool = match &v9 {
                    US0::US0_Edits => {
                        false
                    }
                    US0::US0_Facts => {
                        v6
                    }
                    US0::US0_Git => {
                        v4
                    }
                    US0::US0_Jobs => {
                        v2
                    }
                    US0::US0_Labs => {
                        v3
                    }
                    US0::US0_Reads => {
                        v7
                    }
                    US0::US0_Runs => {
                        v1
                    }
                    US0::US0_Writes => {
                        v5
                    }
                };
                if v18 {
                    return v9.clone();
                } else {
                    (v0, v1, v2, v3, v4, v5, v6, v7) = (v10.clone(), v1, v2, v3, v4, v5, v6, v7);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US0::US0_Edits;
            }
        }
    }
}
fn field_at_4(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let mut v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).cloned().unwrap_or_default();
    v3.clone()
}
fn field_index_6(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: Rc<str>) -> i32 {
    let mut v3: i32 = v0.get(v1 as usize).and_then(|fields| fields.iter().position(|field| field.len() > 1 && field.starts_with(char::from(119u8)) && &field[1..] == &*v2)).map(|index| index as i32).unwrap_or(-1);
    v3
}
fn run_named_9(mut v0: Rc<str>) -> US8 {
    let mut v1: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("run"); } LIT.with(|lit| lit.clone()) };
    if v1 {
        let mut v2: US6 = US6::US6_RunVerb;
        US8::US8_Some(v2.clone())
    } else {
        let mut v4: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("errors"); } LIT.with(|lit| lit.clone()) };
        if v4 {
            let mut v5: US6 = US6::US6_ErrorsVerb;
            US8::US8_Some(v5.clone())
        } else {
            let mut v7: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("processes"); } LIT.with(|lit| lit.clone()) };
            if v7 {
                let mut v8: US6 = US6::US6_ProcessesVerb;
                US8::US8_Some(v8.clone())
            } else {
                let mut v10: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop"); } LIT.with(|lit| lit.clone()) };
                if v10 {
                    let mut v11: US6 = US6::US6_StopVerb;
                    US8::US8_Some(v11.clone())
                } else {
                    US8::US8_None
                }
            }
        }
    }
}
fn write_named_10(mut v0: Rc<str>) -> US9 {
    let mut v1: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("append"); } LIT.with(|lit| lit.clone()) };
    if v1 {
        let mut v2: US7 = US7::US7_AppendVerb;
        US9::US9_Some(v2.clone())
    } else {
        let mut v4: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("copy"); } LIT.with(|lit| lit.clone()) };
        if v4 {
            let mut v5: US7 = US7::US7_CopyVerb;
            US9::US9_Some(v5.clone())
        } else {
            let mut v7: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("move"); } LIT.with(|lit| lit.clone()) };
            if v7 {
                let mut v8: US7 = US7::US7_MoveVerb;
                US9::US9_Some(v8.clone())
            } else {
                US9::US9_None
            }
        }
    }
}
fn program_verb_tail_8(mut v0: Rc<str>) -> US2 {
    let mut v1: US8 = run_named_9(v0.clone());
    match &v1 {
        US8::US8_None => {
            let mut v5: US9 = write_named_10(v0.clone());
            match &v5 {
                US9::US9_None => {
                    let mut v9: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git"); } LIT.with(|lit| lit.clone()) };
                    if v9 {
                        let mut v10: US3 = US3::US3_GitVerb;
                        US2::US2_Some(v10.clone())
                    } else {
                        let mut v12: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
                        if v12 {
                            let mut v13: US3 = US3::US3_JobVerb;
                            US2::US2_Some(v13.clone())
                        } else {
                            let mut v15: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lab"); } LIT.with(|lit| lit.clone()) };
                            if v15 {
                                let mut v16: US3 = US3::US3_LabVerb;
                                US2::US2_Some(v16.clone())
                            } else {
                                US2::US2_None
                            }
                        }
                    }
                }
                US9::US9_Some(v6) => {
                    let mut v6: US7 = v6.clone();
                    let mut v7: US3 = US3::US3_WriteVerb(v6.clone());
                    US2::US2_Some(v7.clone())
                }
            }
        }
        US8::US8_Some(v2) => {
            let mut v2: US6 = v2.clone();
            let mut v3: US3 = US3::US3_RunStepVerb(v2.clone());
            US2::US2_Some(v3.clone())
        }
    }
}
fn fact_named_11(mut v0: Rc<str>) -> US11 {
    let mut v1: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("hash"); } LIT.with(|lit| lit.clone()) };
    if v1 {
        let mut v2: US5 = US5::US5_HashVerb;
        US11::US11_Some(v2.clone())
    } else {
        let mut v4: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("env"); } LIT.with(|lit| lit.clone()) };
        if v4 {
            let mut v5: US5 = US5::US5_EnvVerb;
            US11::US11_Some(v5.clone())
        } else {
            let mut v7: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exists"); } LIT.with(|lit| lit.clone()) };
            if v7 {
                let mut v8: US5 = US5::US5_ExistsVerb;
                US11::US11_Some(v8.clone())
            } else {
                let mut v10: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("nearest"); } LIT.with(|lit| lit.clone()) };
                if v10 {
                    let mut v11: US5 = US5::US5_NearestVerb;
                    US11::US11_Some(v11.clone())
                } else {
                    let mut v13: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("json"); } LIT.with(|lit| lit.clone()) };
                    if v13 {
                        let mut v14: US5 = US5::US5_JsonVerb;
                        US11::US11_Some(v14.clone())
                    } else {
                        let mut v16: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lines"); } LIT.with(|lit| lit.clone()) };
                        if v16 {
                            let mut v17: US5 = US5::US5_LinesVerb;
                            US11::US11_Some(v17.clone())
                        } else {
                            let mut v19: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table"); } LIT.with(|lit| lit.clone()) };
                            if v19 {
                                let mut v20: US5 = US5::US5_TableVerb;
                                US11::US11_Some(v20.clone())
                            } else {
                                let mut v22: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dirs"); } LIT.with(|lit| lit.clone()) };
                                if v22 {
                                    let mut v23: US5 = US5::US5_DirsVerb;
                                    US11::US11_Some(v23.clone())
                                } else {
                                    let mut v25: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tag"); } LIT.with(|lit| lit.clone()) };
                                    if v25 {
                                        let mut v26: US5 = US5::US5_TagVerb;
                                        US11::US11_Some(v26.clone())
                                    } else {
                                        let mut v28: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("count"); } LIT.with(|lit| lit.clone()) };
                                        if v28 {
                                            let mut v29: US5 = US5::US5_CountVerb;
                                            US11::US11_Some(v29.clone())
                                        } else {
                                            let mut v31: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("diff"); } LIT.with(|lit| lit.clone()) };
                                            if v31 {
                                                let mut v32: US5 = US5::US5_DiffVerb;
                                                US11::US11_Some(v32.clone())
                                            } else {
                                                US11::US11_None
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn program_verb_named_7(mut v0: Rc<str>) -> US2 {
    let mut v1: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("search"); } LIT.with(|lit| lit.clone()) };
    let mut v25: US10 = if v1 {
        let mut v2: US4 = US4::US4_Search;
        US10::US10_Some(v2.clone())
    } else {
        let mut v4: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read"); } LIT.with(|lit| lit.clone()) };
        if v4 {
            let mut v5: US4 = US4::US4_ReadAround;
            US10::US10_Some(v5.clone())
        } else {
            let mut v7: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files"); } LIT.with(|lit| lit.clone()) };
            if v7 {
                let mut v8: US4 = US4::US4_ListFiles;
                US10::US10_Some(v8.clone())
            } else {
                let mut v10: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("head"); } LIT.with(|lit| lit.clone()) };
                if v10 {
                    let mut v11: US4 = US4::US4_Head;
                    US10::US10_Some(v11.clone())
                } else {
                    let mut v13: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tail"); } LIT.with(|lit| lit.clone()) };
                    if v13 {
                        let mut v14: US4 = US4::US4_Tail;
                        US10::US10_Some(v14.clone())
                    } else {
                        let mut v16: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait"); } LIT.with(|lit| lit.clone()) };
                        if v16 {
                            let mut v17: US4 = US4::US4_Wait;
                            US10::US10_Some(v17.clone())
                        } else {
                            US10::US10_None
                        }
                    }
                }
            }
        }
    };
    match &v25 {
        US10::US10_None => {
            let mut v29: US11 = fact_named_11(v0.clone());
            match &v29 {
                US11::US11_None => {
                    program_verb_tail_8(v0.clone())
                }
                US11::US11_Some(v30) => {
                    let mut v30: US5 = v30.clone();
                    let mut v31: US3 = US3::US3_FactVerb(v30.clone());
                    US2::US2_Some(v31.clone())
                }
            }
        }
        US10::US10_Some(v26) => {
            let mut v26: US4 = v26.clone();
            let mut v27: US3 = US3::US3_ReadVerb(v26.clone());
            US2::US2_Some(v27.clone())
        }
    }
}
fn query_family_5(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US1 {
    let mut v2: i32 = 1i32;
    let mut v3: Rc<str> = field_at_4(v0.clone(), v1, v2);
    let mut v4: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files"); } LIT.with(|lit| lit.clone()) };
    let mut v8: bool = if v4 {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("count"); } LIT.with(|lit| lit.clone()) };
        let mut v6: i32 = field_index_6(v0.clone(), v1, v5.clone());
        let mut v7: bool = v6 >= 0i32;
        v7
    } else {
        false
    };
    if v8 {
        let mut v9: US0 = US0::US0_Facts;
        US1::US1_Some(v9.clone())
    } else {
        let mut v11: US2 = program_verb_named_7(v3.clone());
        match &v11 {
            US2::US2_None => {
                US1::US1_None
            }
            US2::US2_Some(v12) => {
                let mut v12: US3 = v12.clone();
                let mut v30: US0 = match &v12 {
                    US3::US3_FactVerb(v15) => {
                        let mut v15: US5 = v15.clone();
                        US0::US0_Facts
                    }
                    US3::US3_GitVerb => {
                        US0::US0_Git
                    }
                    US3::US3_JobVerb => {
                        US0::US0_Jobs
                    }
                    US3::US3_LabVerb => {
                        US0::US0_Labs
                    }
                    US3::US3_ReadVerb(v13) => {
                        let mut v13: US4 = v13.clone();
                        US0::US0_Reads
                    }
                    US3::US3_RunStepVerb(v17) => {
                        let mut v17: US6 = v17.clone();
                        US0::US0_Runs
                    }
                    US3::US3_WriteVerb(v19) => {
                        let mut v19: US7 = v19.clone();
                        US0::US0_Writes
                    }
                };
                US1::US1_Some(v30.clone())
            }
        }
    }
}
fn families_of_3(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: bool, mut v6: bool, mut v7: bool, mut v8: bool) -> (bool, bool, bool, bool, bool, bool, bool) {
    let mut v9: i32 = v0.len() as i32;
    let mut v10: bool = v1 >= v9;
    let (mut v117, mut v118, mut v119, mut v120, mut v121, mut v122, mut v123): (bool, bool, bool, bool, bool, bool, bool) = if v10 {
        (v2, v3, v4, v5, v6, v7, v8)
    } else {
        let mut v11: i32 = 0i32;
        let mut v12: Rc<str> = field_at_4(v0.clone(), v1, v11);
        let mut v13: bool = v12.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("query"); } LIT.with(|lit| lit.clone()) };
        let (mut v110, mut v111, mut v112, mut v113, mut v114, mut v115, mut v116): (bool, bool, bool, bool, bool, bool, bool) = if v13 {
            let mut v14: i32 = v1.wrapping_add(1i32);
            let (mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21): (bool, bool, bool, bool, bool, bool, bool) = families_of_3(v0.clone(), v14, v2, v3, v4, v5, v6, v7, v8);
            (v15, v16, v17, v18, v19, v20, v21)
        } else {
            let mut v22: US1 = query_family_5(v0.clone(), v1);
            let (mut v103, mut v104, mut v105, mut v106, mut v107, mut v108, mut v109): (bool, bool, bool, bool, bool, bool, bool) = match &v22 {
                US1::US1_Some(v23) => {
                    let mut v23: US0 = v23.clone();
                    let mut v24: i32 = v1.wrapping_add(1i32);
                    let (mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80): (bool, bool, bool, bool, bool, bool, bool) = match &v23 {
                        US0::US0_Runs => {
                            (true, v3, v4, v5, v6, v7, v8)
                        }
                        _ => {
                            let (mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73): (bool, bool, bool, bool, bool, bool, bool) = match &v23 {
                                US0::US0_Jobs => {
                                    (v2, true, v4, v5, v6, v7, v8)
                                }
                                _ => {
                                    let (mut v60, mut v61, mut v62, mut v63, mut v64, mut v65, mut v66): (bool, bool, bool, bool, bool, bool, bool) = match &v23 {
                                        US0::US0_Labs => {
                                            (v2, v3, true, v5, v6, v7, v8)
                                        }
                                        _ => {
                                            let (mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59): (bool, bool, bool, bool, bool, bool, bool) = match &v23 {
                                                US0::US0_Git => {
                                                    (v2, v3, v4, true, v6, v7, v8)
                                                }
                                                _ => {
                                                    let (mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52): (bool, bool, bool, bool, bool, bool, bool) = match &v23 {
                                                        US0::US0_Writes => {
                                                            (v2, v3, v4, v5, true, v7, v8)
                                                        }
                                                        _ => {
                                                            let (mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45): (bool, bool, bool, bool, bool, bool, bool) = match &v23 {
                                                                US0::US0_Facts => {
                                                                    (v2, v3, v4, v5, v6, true, v8)
                                                                }
                                                                _ => {
                                                                    let (mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38): (bool, bool, bool, bool, bool, bool, bool) = match &v23 {
                                                                        US0::US0_Reads => {
                                                                            (v2, v3, v4, v5, v6, v7, true)
                                                                        }
                                                                        _ => {
                                                                            let (mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31): (bool, bool, bool, bool, bool, bool, bool) = match &v23 {
                                                                                US0::US0_Edits => {
                                                                                    (v2, v3, v4, v5, v6, v7, v8)
                                                                                }
                                                                                _ => unreachable!(),
                                                                            };
                                                                            (v25, v26, v27, v28, v29, v30, v31)
                                                                        }
                                                                    };
                                                                    (v32, v33, v34, v35, v36, v37, v38)
                                                                }
                                                            };
                                                            (v39, v40, v41, v42, v43, v44, v45)
                                                        }
                                                    };
                                                    (v46, v47, v48, v49, v50, v51, v52)
                                                }
                                            };
                                            (v53, v54, v55, v56, v57, v58, v59)
                                        }
                                    };
                                    (v60, v61, v62, v63, v64, v65, v66)
                                }
                            };
                            (v67, v68, v69, v70, v71, v72, v73)
                        }
                    };
                    families_of_3(v0.clone(), v24, v74, v75, v76, v77, v78, v79, v80)
                }
                _ => {
                    let (mut v96, mut v97, mut v98, mut v99, mut v100, mut v101, mut v102): (bool, bool, bool, bool, bool, bool, bool) = match &v22 {
                        US1::US1_None => {
                            let mut v88: i32 = v1.wrapping_add(1i32);
                            families_of_3(v0.clone(), v88, v2, v3, v4, v5, v6, v7, v8)
                        }
                        _ => unreachable!(),
                    };
                    (v96, v97, v98, v99, v100, v101, v102)
                }
            };
            (v103, v104, v105, v106, v107, v108, v109)
        };
        (v110, v111, v112, v113, v114, v115, v116)
    };
    (v117, v118, v119, v120, v121, v122, v123)
}
fn records_family_1(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>) -> US0 {
    let mut v1: US0 = US0::US0_Runs;
    let mut v2: US0 = US0::US0_Jobs;
    let mut v3: US0 = US0::US0_Labs;
    let mut v4: US0 = US0::US0_Git;
    let mut v5: US0 = US0::US0_Writes;
    let mut v6: US0 = US0::US0_Facts;
    let mut v7: US0 = US0::US0_Reads;
    let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(v7.clone(), v8.clone()));
    let mut v10: Rc<UH0> = Rc::new(UH0::UH0_Cons(v6.clone(), v9.clone()));
    let mut v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5.clone(), v10.clone()));
    let mut v12: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4.clone(), v11.clone()));
    let mut v13: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3.clone(), v12.clone()));
    let mut v14: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v13.clone()));
    let mut v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v1.clone(), v14.clone()));
    let mut v16: i32 = 1i32;
    let mut v17: bool = false;
    let mut v18: bool = false;
    let mut v19: bool = false;
    let mut v20: bool = false;
    let mut v21: bool = false;
    let mut v22: bool = false;
    let mut v23: bool = false;
    let (mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30): (bool, bool, bool, bool, bool, bool, bool) = families_of_3(v0.clone(), v16, v17, v18, v19, v20, v21, v22, v23);
    first_seen_2(v15.clone(), v24, v25, v26, v27, v28, v29, v30)
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        let mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = records_of_0(v0.clone());
        let mut v2: US0 = records_family_1(v1.clone());
        match &v2 {
            US0::US0_Edits => {
                let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edits"); } LIT.with(|lit| lit.clone()) };
                v3.clone()
            }
            US0::US0_Facts => {
                let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("facts"); } LIT.with(|lit| lit.clone()) };
                v5.clone()
            }
            US0::US0_Git => {
                let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git"); } LIT.with(|lit| lit.clone()) };
                v8.clone()
            }
            US0::US0_Jobs => {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("jobs"); } LIT.with(|lit| lit.clone()) };
                v9.clone()
            }
            US0::US0_Labs => {
                let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("labs"); } LIT.with(|lit| lit.clone()) };
                v10.clone()
            }
            US0::US0_Reads => {
                let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("reads"); } LIT.with(|lit| lit.clone()) };
                v4.clone()
            }
            US0::US0_Runs => {
                let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("runs"); } LIT.with(|lit| lit.clone()) };
                v6.clone()
            }
            US0::US0_Writes => {
                let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("writes"); } LIT.with(|lit| lit.clone()) };
                v7.clone()
            }
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
fn do_verb_family_12(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: US2 = program_verb_named_7(v0.clone());
    match &v1 {
        US2::US2_None => {
            let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v37.clone()
        }
        US2::US2_Some(v2) => {
            let mut v2: US3 = v2.clone();
            let mut v20: US0 = match &v2 {
                US3::US3_FactVerb(v5) => {
                    let mut v5: US5 = v5.clone();
                    US0::US0_Facts
                }
                US3::US3_GitVerb => {
                    US0::US0_Git
                }
                US3::US3_JobVerb => {
                    US0::US0_Jobs
                }
                US3::US3_LabVerb => {
                    US0::US0_Labs
                }
                US3::US3_ReadVerb(v3) => {
                    let mut v3: US4 = v3.clone();
                    US0::US0_Reads
                }
                US3::US3_RunStepVerb(v7) => {
                    let mut v7: US6 = v7.clone();
                    US0::US0_Runs
                }
                US3::US3_WriteVerb(v9) => {
                    let mut v9: US7 = v9.clone();
                    US0::US0_Writes
                }
            };
            match &v20 {
                US0::US0_Edits => {
                    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edits"); } LIT.with(|lit| lit.clone()) };
                    v21.clone()
                }
                US0::US0_Facts => {
                    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("facts"); } LIT.with(|lit| lit.clone()) };
                    v23.clone()
                }
                US0::US0_Git => {
                    let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git"); } LIT.with(|lit| lit.clone()) };
                    v26.clone()
                }
                US0::US0_Jobs => {
                    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("jobs"); } LIT.with(|lit| lit.clone()) };
                    v27.clone()
                }
                US0::US0_Labs => {
                    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("labs"); } LIT.with(|lit| lit.clone()) };
                    v28.clone()
                }
                US0::US0_Reads => {
                    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("reads"); } LIT.with(|lit| lit.clone()) };
                    v22.clone()
                }
                US0::US0_Runs => {
                    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("runs"); } LIT.with(|lit| lit.clone()) };
                    v24.clone()
                }
                US0::US0_Writes => {
                    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("writes"); } LIT.with(|lit| lit.clone()) };
                    v25.clone()
                }
            }
        }
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        do_verb_family_12(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_do_program_family(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
pub fn eoie_do_verb_family(v0: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0))
}
