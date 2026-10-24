#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_RegularFile,
    US0_Directory,
    US0_LinkedEntry,
    US0_AbsentEntry,
}
#[derive(Clone)]
enum US2 {
    US2_CFamily,
    US2_HashFamily,
    US2_DashFamily,
    US2_HaskellFamily,
    US2_LeanFamily,
    US2_LispFamily,
    US2_ErlangFamily,
    US2_MarkupFamily,
    US2_MlFamily,
    US2_PascalFamily,
    US2_BatchFamily,
}
#[derive(Clone)]
enum US3 {
    US3_SkipNestedRepository,
    US3_SkipVendorDirectory,
    US3_SkipUnknownSyntax,
    US3_SkipGeneratedOutput,
    US3_SkipLinkedEntry,
}
#[derive(Clone)]
enum US1 {
    US1_ScanPath(US2),
    US1_SkipPath(US3),
}
#[derive(Clone)]
enum US4 {
    US4_ScanContent,
    US4_SkipBinary,
    US4_SkipLarge,
    US4_SkipUnreadable,
}
#[derive(Clone)]
enum US5 {
    US5_OpenTodo,
    US5_StaleTodo,
    US5_UnknownTodo,
}
fn command_line_argument_count_2() -> i32 {
    let mut v0: i32 = (std::env::args().count() as i32) - 1;
    v0
}
fn argument_count_1() -> i32 {
    let mut v0: i32 = command_line_argument_count_2();
    let mut v1: bool = v0 > 0i32;
    if v1 {
        let mut v2: i32 = v0.wrapping_sub(1i32);
        v2
    } else {
        0i32
    }
}
fn scan_start_3(mut v0: bool, mut v1: bool, mut v2: Rc<str>, mut v3: Rc<str>) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) {
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, v0, v1, false, v4.clone(), v2.clone(), v3.clone(), 0i32, 0i32, 0i32)
}
fn path_verdict_of_code_7(mut v0: i32) -> US1 {
    let mut v1: bool = 0i32 < v0;
    if v1 {
        let mut v2: bool = v0 == 1i32;
        let mut v35: US2 = if v2 {
            US2::US2_CFamily
        } else {
            let mut v4: bool = v0 == 2i32;
            if v4 {
                US2::US2_HashFamily
            } else {
                let mut v6: bool = v0 == 3i32;
                if v6 {
                    US2::US2_DashFamily
                } else {
                    let mut v8: bool = v0 == 4i32;
                    if v8 {
                        US2::US2_HaskellFamily
                    } else {
                        let mut v10: bool = v0 == 5i32;
                        if v10 {
                            US2::US2_LeanFamily
                        } else {
                            let mut v12: bool = v0 == 6i32;
                            if v12 {
                                US2::US2_LispFamily
                            } else {
                                let mut v14: bool = v0 == 7i32;
                                if v14 {
                                    US2::US2_ErlangFamily
                                } else {
                                    let mut v16: bool = v0 == 8i32;
                                    if v16 {
                                        US2::US2_MarkupFamily
                                    } else {
                                        let mut v18: bool = v0 == 9i32;
                                        if v18 {
                                            US2::US2_MlFamily
                                        } else {
                                            let mut v20: bool = v0 == 10i32;
                                            if v20 {
                                                US2::US2_PascalFamily
                                            } else {
                                                let mut v22: bool = v0 == 11i32;
                                                if v22 {
                                                    US2::US2_BatchFamily
                                                } else {
                                                    US2::US2_CFamily
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
        };
        US1::US1_ScanPath(v35.clone())
    } else {
        let mut v37: i32 = v0.wrapping_neg();
        let mut v38: bool = v37 == 1i32;
        let mut v53: US3 = if v38 {
            US3::US3_SkipNestedRepository
        } else {
            let mut v40: bool = v37 == 2i32;
            if v40 {
                US3::US3_SkipVendorDirectory
            } else {
                let mut v42: bool = v37 == 3i32;
                if v42 {
                    US3::US3_SkipUnknownSyntax
                } else {
                    let mut v44: bool = v37 == 4i32;
                    if v44 {
                        US3::US3_SkipGeneratedOutput
                    } else {
                        let mut v46: bool = v37 == 5i32;
                        if v46 {
                            US3::US3_SkipLinkedEntry
                        } else {
                            US3::US3_SkipUnknownSyntax
                        }
                    }
                }
            }
        };
        US1::US1_SkipPath(v53.clone())
    }
}
fn counted_path_8(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: US1) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) {
    let mut v12: i32 = v0.wrapping_add(1i32);
    let (mut v86, mut v87, mut v88, mut v89, mut v90, mut v91, mut v92, mut v93, mut v94, mut v95, mut v96): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = match &v11 {
        US1::US1_ScanPath(v13) => {
            let mut v13: US2 = v13.clone();
            (v12, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10)
        }
        _ => {
            let (mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83, mut v84, mut v85): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = match &v11 {
                US1::US1_SkipPath(v14) => {
                    let mut v14: US3 = v14.clone();
                    match &v14 {
                        US3::US3_SkipNestedRepository => {
                            let mut v15: i32 = v3.wrapping_add(1i32);
                            (v12, v1, v2, v15, v4, v5, v6, v7, v8, v9, v10)
                        }
                        _ => {
                            let (mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = match &v14 {
                                US3::US3_SkipVendorDirectory => {
                                    let mut v16: i32 = v4.wrapping_add(1i32);
                                    (v12, v1, v2, v3, v16, v5, v6, v7, v8, v9, v10)
                                }
                                _ => {
                                    let (mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = match &v14 {
                                        US3::US3_SkipUnknownSyntax => {
                                            let mut v17: i32 = v5.wrapping_add(1i32);
                                            (v12, v1, v2, v3, v4, v17, v6, v7, v8, v9, v10)
                                        }
                                        _ => {
                                            let (mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = match &v14 {
                                                US3::US3_SkipGeneratedOutput => {
                                                    let mut v18: i32 = v6.wrapping_add(1i32);
                                                    (v12, v1, v2, v3, v4, v5, v18, v7, v8, v9, v10)
                                                }
                                                _ => {
                                                    let (mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = match &v14 {
                                                        US3::US3_SkipLinkedEntry => {
                                                            let mut v19: i32 = v9.wrapping_add(1i32);
                                                            (v12, v1, v2, v3, v4, v5, v6, v7, v8, v19, v10)
                                                        }
                                                        _ => unreachable!(),
                                                    };
                                                    (v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30)
                                                }
                                            };
                                            (v31, v32, v33, v34, v35, v36, v37, v38, v39, v40, v41)
                                        }
                                    };
                                    (v42, v43, v44, v45, v46, v47, v48, v49, v50, v51, v52)
                                }
                            };
                            (v53, v54, v55, v56, v57, v58, v59, v60, v61, v62, v63)
                        }
                    }
                }
                _ => unreachable!(),
            };
            (v75, v76, v77, v78, v79, v80, v81, v82, v83, v84, v85)
        }
    };
    (v86, v87, v88, v89, v90, v91, v92, v93, v94, v95, v96)
}
fn counted_content_9(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: US4) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) {
    let (mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = match &v11 {
        US4::US4_ScanContent => {
            let mut v12: i32 = v1.wrapping_add(1i32);
            (v0, v12, v2, v3, v4, v5, v6, v7, v8, v9, v10)
        }
        _ => {
            let (mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = match &v11 {
                US4::US4_SkipBinary => {
                    let mut v13: i32 = v7.wrapping_add(1i32);
                    (v0, v1, v2, v3, v4, v5, v6, v13, v8, v9, v10)
                }
                _ => {
                    let (mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = match &v11 {
                        US4::US4_SkipLarge => {
                            let mut v14: i32 = v8.wrapping_add(1i32);
                            (v0, v1, v2, v3, v4, v5, v6, v7, v14, v9, v10)
                        }
                        _ => {
                            let (mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = match &v11 {
                                US4::US4_SkipUnreadable => {
                                    let mut v15: i32 = v10.wrapping_add(1i32);
                                    (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v15)
                                }
                                _ => unreachable!(),
                            };
                            (v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26)
                        }
                    };
                    (v27, v28, v29, v30, v31, v32, v33, v34, v35, v36, v37)
                }
            };
            (v38, v39, v40, v41, v42, v43, v44, v45, v46, v47, v48)
        }
    };
    (v49, v50, v51, v52, v53, v54, v55, v56, v57, v58, v59)
}
fn counted_todos_10(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: i32) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) {
    let mut v12: i32 = v2 + v11;
    (v0, v1, v12, v3, v4, v5, v6, v7, v8, v9, v10)
}
fn widened_11(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: bool, mut v12: bool, mut v13: bool, mut v14: Rc<str>, mut v15: Rc<str>, mut v16: Rc<str>, mut v17: i32, mut v18: i32, mut v19: i32, mut v20: Rc<str>, mut v21: i32) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) {
    let mut v22: i32 = (v20.clone().len() as i32);
    let mut v23: bool = v22 <= v21;
    let (mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73, mut v74, mut v75, mut v76): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = if v23 {
        (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19)
    } else {
        let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v25: i32 = v20.get(v21 as usize..).and_then(|rest| rest.find(&*v24)).map(|found| (found + v21 as usize) as i32).unwrap_or(-1);
        let mut v26: bool = v25 < 0i32;
        let mut v27: i32 = if v26 {
            v22
        } else {
            v25
        };
        let mut v28: bool = v11 || v12;
        let mut v34: i32 = if v28 {
            0i32
        } else {
            let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wide (over 120 columns, the Spiral formatter rewraps it): "); } LIT.with(|lit| lit.clone()) };
            let mut v30: Rc<str> = std::rc::Rc::<str>::from(v20.get(v21 as usize..v27 as usize).unwrap_or(""));
            let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v32: Rc<str> = std::rc::Rc::<str>::from([&*v29, &*v30, &*v31, &*v31, &*v31].concat());
            let mut v33: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v32.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
            v33
        };
        let mut v35: i32 = v19.wrapping_add(1i32);
        let mut v36: i32 = v27.wrapping_add(1i32);
        let (mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55, mut v56): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = widened_11(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v35, v20.clone(), v36);
        (v37, v38, v39, v40, v41, v42, v43, v44, v45, v46, v47, v48, v49, v50, v51.clone(), v52.clone(), v53.clone(), v54, v55, v56)
    };
    (v57, v58, v59, v60, v61, v62, v63, v64, v65, v66, v67, v68, v69, v70, v71.clone(), v72.clone(), v73.clone(), v74, v75, v76)
}
fn standings_from_12(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: bool, mut v12: bool, mut v13: bool, mut v14: Rc<str>, mut v15: Rc<str>, mut v16: Rc<str>, mut v17: i32, mut v18: i32, mut v19: i32, mut v20: Rc<str>, mut v21: i32) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) {
    let mut v22: i32 = (v20.clone().len() as i32);
    let mut v23: bool = v22 <= v21;
    let (mut v133, mut v134, mut v135, mut v136, mut v137, mut v138, mut v139, mut v140, mut v141, mut v142, mut v143, mut v144, mut v145, mut v146, mut v147, mut v148, mut v149, mut v150, mut v151, mut v152): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = if v23 {
        (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19)
    } else {
        let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v25: i32 = v20.get(v21 as usize..).and_then(|rest| rest.find(&*v24)).map(|found| (found + v21 as usize) as i32).unwrap_or(-1);
        let mut v26: bool = v25 < 0i32;
        let mut v27: i32 = if v26 {
            v22
        } else {
            v25
        };
        let mut v28: Rc<str> = std::rc::Rc::<str>::from(v20.get(v21 as usize..v27 as usize).unwrap_or(""));
        let mut v29: Rc<str> = agile_todo_lines::eoie_agile_todo_id(&*v15, &*v28);
        let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v31: bool = v29 == v30 ;
        let (mut v92, mut v93, mut v94, mut v95, mut v96, mut v97, mut v98, mut v99, mut v100, mut v101, mut v102, mut v103, mut v104, mut v105, mut v106, mut v107, mut v108, mut v109, mut v110, mut v111): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = if v31 {
            (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19)
        } else {
            let mut v32: Rc<str> = agile_task_census::eoie_agile_task_status(&*v15, &*v29);
            let mut v33: bool = v32 == v30 ;
            let mut v40: US5 = if v33 {
                US5::US5_UnknownTodo
            } else {
                let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Done"); } LIT.with(|lit| lit.clone()) };
                let mut v36: bool = v32 == v35 ;
                if v36 {
                    US5::US5_StaleTodo
                } else {
                    US5::US5_OpenTodo
                }
            };
            let (mut v72, mut v73, mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83, mut v84, mut v85, mut v86, mut v87, mut v88, mut v89, mut v90, mut v91): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = match &v40 {
                US5::US5_OpenTodo => {
                    (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19)
                }
                _ => {
                    let mut v41: bool = v11 || v12;
                    let mut v49: i32 = if v41 {
                        0i32
                    } else {
                        let mut v45: Rc<str> = match &v40 {
                            US5::US5_StaleTodo => {
                                let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stale (item is Done): "); } LIT.with(|lit| lit.clone()) };
                                v42.clone()
                            }
                            US5::US5_UnknownTodo => {
                                let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown (no backlog item): "); } LIT.with(|lit| lit.clone()) };
                                v43.clone()
                            }
                            _ => unreachable!(),
                        };
                        let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" at "); } LIT.with(|lit| lit.clone()) };
                        let mut v47: Rc<str> = std::rc::Rc::<str>::from([&*v45, &*v29, &*v46, &*v28, &*v30].concat());
                        let mut v48: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v47.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                        v48
                    };
                    let (mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = match &v40 {
                        US5::US5_StaleTodo => {
                            let mut v50: i32 = v17.wrapping_add(1i32);
                            (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v50, v18, v19)
                        }
                        _ => {
                            let mut v51: i32 = v18.wrapping_add(1i32);
                            (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v51, v19)
                        }
                    };
                    (v52, v53, v54, v55, v56, v57, v58, v59, v60, v61, v62, v63, v64, v65, v66.clone(), v67.clone(), v68.clone(), v69, v70, v71)
                }
            };
            (v72, v73, v74, v75, v76, v77, v78, v79, v80, v81, v82, v83, v84, v85, v86.clone(), v87.clone(), v88.clone(), v89, v90, v91)
        };
        let mut v112: i32 = v27.wrapping_add(1i32);
        let (mut v113, mut v114, mut v115, mut v116, mut v117, mut v118, mut v119, mut v120, mut v121, mut v122, mut v123, mut v124, mut v125, mut v126, mut v127, mut v128, mut v129, mut v130, mut v131, mut v132): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = standings_from_12(v92, v93, v94, v95, v96, v97, v98, v99, v100, v101, v102, v103, v104, v105, v106.clone(), v107.clone(), v108.clone(), v109, v110, v111, v20.clone(), v112);
        (v113, v114, v115, v116, v117, v118, v119, v120, v121, v122, v123, v124, v125, v126, v127.clone(), v128.clone(), v129.clone(), v130, v131, v132)
    };
    (v133, v134, v135, v136, v137, v138, v139, v140, v141, v142, v143, v144, v145, v146, v147.clone(), v148.clone(), v149.clone(), v150, v151, v152)
}
fn scanned_file_6(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: bool, mut v12: bool, mut v13: bool, mut v14: Rc<str>, mut v15: Rc<str>, mut v16: Rc<str>, mut v17: i32, mut v18: i32, mut v19: i32, mut v20: Rc<str>, mut v21: Rc<str>, mut v22: Rc<str>) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) {
    let mut v23: bool = std::fs::symlink_metadata(&*v21).map(|metadata| metadata.file_type().is_symlink()).unwrap_or(false);
    let mut v32: US0 = if v23 {
        US0::US0_LinkedEntry
    } else {
        let mut v25: bool = std::path::Path::new(&*v21).is_dir();
        if v25 {
            US0::US0_Directory
        } else {
            let mut v27: bool = std::path::Path::new(&*v21).is_file();
            if v27 {
                US0::US0_RegularFile
            } else {
                US0::US0_AbsentEntry
            }
        }
    };
    let (mut v339, mut v340, mut v341, mut v342, mut v343, mut v344, mut v345, mut v346, mut v347, mut v348, mut v349, mut v350, mut v351, mut v352, mut v353, mut v354, mut v355, mut v356, mut v357, mut v358): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = match &v32 {
        US0::US0_RegularFile => {
            let mut v33: bool = std::path::Path::new(&*v21).with_extension("spi").is_file();
            let mut v34: i32 = agile_todo_paths::eoie_agile_todo_path_verdict(&*v22, v33);
            let mut v35: US1 = path_verdict_of_code_7(v34);
            let (mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = counted_path_8(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v35.clone());
            match &v35 {
                US1::US1_ScanPath(v47) => {
                    let mut v47: US2 = v47.clone();
                    let mut v48: u64 = std::fs::metadata(&*v21).map(|metadata| metadata.len()).unwrap_or(0);
                    let mut v49: bool = v48 > 33554432u64;
                    let mut v52: US4 = if v49 {
                        US4::US4_SkipLarge
                    } else {
                        US4::US4_ScanContent
                    };
                    match &v52 {
                        US4::US4_SkipLarge => {
                            let mut v53: US4 = US4::US4_SkipLarge;
                            let (mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = counted_content_9(v36, v37, v38, v39, v40, v41, v42, v43, v44, v45, v46, v53.clone());
                            (v54, v55, v56, v57, v58, v59, v60, v61, v62, v63, v64, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19)
                        }
                        _ => {
                            let (mut v65, mut v66): (bool, Rc<str>) = match std::fs::read(&*v21) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
                            let mut v69: US4 = if v65 {
                                US4::US4_SkipBinary
                            } else {
                                US4::US4_ScanContent
                            };
                            let (mut v200, mut v201, mut v202, mut v203, mut v204, mut v205, mut v206, mut v207, mut v208, mut v209, mut v210, mut v211, mut v212, mut v213, mut v214, mut v215, mut v216, mut v217, mut v218, mut v219): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = match &v69 {
                                US4::US4_ScanContent => {
                                    let mut v70: i32 = match &v47 {
                                        US2::US2_BatchFamily => {
                                            11i32
                                        }
                                        US2::US2_CFamily => {
                                            1i32
                                        }
                                        US2::US2_DashFamily => {
                                            3i32
                                        }
                                        US2::US2_ErlangFamily => {
                                            7i32
                                        }
                                        US2::US2_HashFamily => {
                                            2i32
                                        }
                                        US2::US2_HaskellFamily => {
                                            4i32
                                        }
                                        US2::US2_LeanFamily => {
                                            5i32
                                        }
                                        US2::US2_LispFamily => {
                                            6i32
                                        }
                                        US2::US2_MarkupFamily => {
                                            8i32
                                        }
                                        US2::US2_MlFamily => {
                                            9i32
                                        }
                                        US2::US2_PascalFamily => {
                                            10i32
                                        }
                                    };
                                    let (mut v71, mut v72, mut v73): (Rc<str>, i32, Rc<str>) = agile_todo_lines::eoie_agile_todo_findings(v70, &*v16, &*v15, &*v66, &*v20, v11);
                                    let mut v74: US4 = US4::US4_ScanContent;
                                    let (mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83, mut v84, mut v85): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = counted_content_9(v36, v37, v38, v39, v40, v41, v42, v43, v44, v45, v46, v74.clone());
                                    let (mut v86, mut v87, mut v88, mut v89, mut v90, mut v91, mut v92, mut v93, mut v94, mut v95, mut v96): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = counted_todos_10(v75, v76, v77, v78, v79, v80, v81, v82, v83, v84, v85, v72);
                                    let mut v97: i32 = 0i32;
                                    let (mut v98, mut v99, mut v100, mut v101, mut v102, mut v103, mut v104, mut v105, mut v106, mut v107, mut v108, mut v109, mut v110, mut v111, mut v112, mut v113, mut v114, mut v115, mut v116, mut v117): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = widened_11(v86, v87, v88, v89, v90, v91, v92, v93, v94, v95, v96, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19, v73.clone(), v97);
                                    let mut v118: bool = v72 <= 0i32;
                                    if v118 {
                                        (v98, v99, v100, v101, v102, v103, v104, v105, v106, v107, v108, v109, v110, v111, v112.clone(), v113.clone(), v114.clone(), v115, v116, v117)
                                    } else {
                                        let (mut v128, mut v129, mut v130, mut v131, mut v132, mut v133, mut v134, mut v135, mut v136, mut v137, mut v138, mut v139, mut v140, mut v141, mut v142, mut v143, mut v144, mut v145, mut v146, mut v147): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = if v110 {
                                            let mut v119: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                            let mut v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                            let mut v121: Rc<str> = std::rc::Rc::<str>::from([&*v112, &*v71, &*v119, &*v120, &*v120].concat());
                                            (v98, v99, v100, v101, v102, v103, v104, v105, v106, v107, v108, v109, v110, true, v121.clone(), v113.clone(), v114.clone(), v115, v116, v117)
                                        } else {
                                            let mut v122: bool = v109 && v111;
                                            let mut v126: Rc<str> = if v122 {
                                                let mut v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                                                let mut v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                let mut v125: Rc<str> = std::rc::Rc::<str>::from([&*v123, &*v71, &*v124, &*v124, &*v124].concat());
                                                v125.clone()
                                            } else {
                                                v71.clone()
                                            };
                                            let mut v127: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v126.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                                            (v98, v99, v100, v101, v102, v103, v104, v105, v106, v107, v108, v109, v110, true, v112.clone(), v113.clone(), v114.clone(), v115, v116, v117)
                                        };
                                        let mut v148: i32 = 0i32;
                                        let (mut v149, mut v150, mut v151, mut v152, mut v153, mut v154, mut v155, mut v156, mut v157, mut v158, mut v159, mut v160, mut v161, mut v162, mut v163, mut v164, mut v165, mut v166, mut v167, mut v168): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = standings_from_12(v128, v129, v130, v131, v132, v133, v134, v135, v136, v137, v138, v139, v140, v141, v142.clone(), v143.clone(), v144.clone(), v145, v146, v147, v71.clone(), v148);
                                        (v149, v150, v151, v152, v153, v154, v155, v156, v157, v158, v159, v160, v161, v162, v163.clone(), v164.clone(), v165.clone(), v166, v167, v168)
                                    }
                                }
                                _ => {
                                    let (mut v189, mut v190, mut v191, mut v192, mut v193, mut v194, mut v195, mut v196, mut v197, mut v198, mut v199): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = counted_content_9(v36, v37, v38, v39, v40, v41, v42, v43, v44, v45, v46, v69.clone());
                                    (v189, v190, v191, v192, v193, v194, v195, v196, v197, v198, v199, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19)
                                }
                            };
                            (v200, v201, v202, v203, v204, v205, v206, v207, v208, v209, v210, v211, v212, v213, v214.clone(), v215.clone(), v216.clone(), v217, v218, v219)
                        }
                    }
                }
                _ => {
                    (v36, v37, v38, v39, v40, v41, v42, v43, v44, v45, v46, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19)
                }
            }
        }
        _ => {
            let (mut v319, mut v320, mut v321, mut v322, mut v323, mut v324, mut v325, mut v326, mut v327, mut v328, mut v329, mut v330, mut v331, mut v332, mut v333, mut v334, mut v335, mut v336, mut v337, mut v338): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = match &v32 {
                US0::US0_LinkedEntry => {
                    let mut v260: US3 = US3::US3_SkipLinkedEntry;
                    let mut v261: US1 = US1::US1_SkipPath(v260.clone());
                    let (mut v262, mut v263, mut v264, mut v265, mut v266, mut v267, mut v268, mut v269, mut v270, mut v271, mut v272): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = counted_path_8(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v261.clone());
                    (v262, v263, v264, v265, v266, v267, v268, v269, v270, v271, v272, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19)
                }
                _ => {
                    let mut v273: i32 = agile_todo_paths::eoie_agile_todo_path_verdict(&*v22, false);
                    let mut v274: US1 = path_verdict_of_code_7(v273);
                    let (mut v275, mut v276, mut v277, mut v278, mut v279, mut v280, mut v281, mut v282, mut v283, mut v284, mut v285): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = counted_path_8(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v274.clone());
                    let (mut v299, mut v300, mut v301, mut v302, mut v303, mut v304, mut v305, mut v306, mut v307, mut v308, mut v309, mut v310, mut v311, mut v312, mut v313, mut v314, mut v315, mut v316, mut v317, mut v318): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = match &v274 {
                        US1::US1_ScanPath(v286) => {
                            let mut v286: US2 = v286.clone();
                            let mut v287: US4 = US4::US4_SkipUnreadable;
                            let (mut v288, mut v289, mut v290, mut v291, mut v292, mut v293, mut v294, mut v295, mut v296, mut v297, mut v298): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = counted_content_9(v275, v276, v277, v278, v279, v280, v281, v282, v283, v284, v285, v287.clone());
                            (v288, v289, v290, v291, v292, v293, v294, v295, v296, v297, v298, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19)
                        }
                        _ => {
                            (v275, v276, v277, v278, v279, v280, v281, v282, v283, v284, v285, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19)
                        }
                    };
                    (v299, v300, v301, v302, v303, v304, v305, v306, v307, v308, v309, v310, v311, v312, v313.clone(), v314.clone(), v315.clone(), v316, v317, v318)
                }
            };
            (v319, v320, v321, v322, v323, v324, v325, v326, v327, v328, v329, v330, v331, v332, v333.clone(), v334.clone(), v335.clone(), v336, v337, v338)
        }
    };
    (v339, v340, v341, v342, v343, v344, v345, v346, v347, v348, v349, v350, v351, v352, v353.clone(), v354.clone(), v355.clone(), v356, v357, v358)
}
fn listed_files_13(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: bool, mut v12: bool, mut v13: bool, mut v14: Rc<str>, mut v15: Rc<str>, mut v16: Rc<str>, mut v17: i32, mut v18: i32, mut v19: i32, mut v20: Rc<str>, mut v21: Rc<str>, mut v22: Rc<str>, mut v23: i32) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) {
    let mut v24: i32 = (v22.clone().len() as i32);
    let mut v25: bool = v24 <= v23;
    let (mut v132, mut v133, mut v134, mut v135, mut v136, mut v137, mut v138, mut v139, mut v140, mut v141, mut v142, mut v143, mut v144, mut v145, mut v146, mut v147, mut v148, mut v149, mut v150, mut v151): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = if v25 {
        (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19)
    } else {
        let mut v26: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
        let mut v27: i32 = v22.get(v23 as usize..).and_then(|rest| rest.find(&*v26)).map(|found| (found + v23 as usize) as i32).unwrap_or(-1);
        let mut v28: bool = v27 < 0i32;
        let mut v29: i32 = if v28 {
            v24
        } else {
            v27
        };
        let mut v30: Rc<str> = std::rc::Rc::<str>::from(v22.get(v23 as usize..v29 as usize).unwrap_or(""));
        let mut v31: i32 = v29.wrapping_add(1i32);
        let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v33: bool = v30 == v32 ;
        let (mut v112, mut v113, mut v114, mut v115, mut v116, mut v117, mut v118, mut v119, mut v120, mut v121, mut v122, mut v123, mut v124, mut v125, mut v126, mut v127, mut v128, mut v129, mut v130, mut v131): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = if v33 {
            let (mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = listed_files_13(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19, v20.clone(), v21.clone(), v22.clone(), v31);
            (v34, v35, v36, v37, v38, v39, v40, v41, v42, v43, v44, v45, v46, v47, v48.clone(), v49.clone(), v50.clone(), v51, v52, v53)
        } else {
            let mut v54: bool = v30 == v32 ;
            let mut v62: Rc<str> = if v54 {
                v21.clone()
            } else {
                let mut v55: bool = v21 == v32 ;
                if v55 {
                    v30.clone()
                } else {
                    let mut v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                    let mut v57: bool = v21.ends_with(&*v56);
                    if v57 {
                        let mut v58: Rc<str> = std::rc::Rc::<str>::from([&*v21, &*v30, &*v32, &*v32, &*v32].concat());
                        v58.clone()
                    } else {
                        let mut v59: Rc<str> = std::rc::Rc::<str>::from([&*v21, &*v56, &*v30, &*v32, &*v32].concat());
                        v59.clone()
                    }
                }
            };
            let mut v63: bool = v30 == v32 ;
            let mut v71: Rc<str> = if v63 {
                v20.clone()
            } else {
                let mut v64: bool = v20 == v32 ;
                if v64 {
                    v30.clone()
                } else {
                    let mut v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                    let mut v66: bool = v20.ends_with(&*v65);
                    if v66 {
                        let mut v67: Rc<str> = std::rc::Rc::<str>::from([&*v20, &*v30, &*v32, &*v32, &*v32].concat());
                        v67.clone()
                    } else {
                        let mut v68: Rc<str> = std::rc::Rc::<str>::from([&*v20, &*v65, &*v30, &*v32, &*v32].concat());
                        v68.clone()
                    }
                }
            };
            let (mut v72, mut v73, mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83, mut v84, mut v85, mut v86, mut v87, mut v88, mut v89, mut v90, mut v91): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = scanned_file_6(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19, v62.clone(), v71.clone(), v30.clone());
            let (mut v92, mut v93, mut v94, mut v95, mut v96, mut v97, mut v98, mut v99, mut v100, mut v101, mut v102, mut v103, mut v104, mut v105, mut v106, mut v107, mut v108, mut v109, mut v110, mut v111): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = listed_files_13(v72, v73, v74, v75, v76, v77, v78, v79, v80, v81, v82, v83, v84, v85, v86.clone(), v87.clone(), v88.clone(), v89, v90, v91, v20.clone(), v21.clone(), v22.clone(), v31);
            (v92, v93, v94, v95, v96, v97, v98, v99, v100, v101, v102, v103, v104, v105, v106.clone(), v107.clone(), v108.clone(), v109, v110, v111)
        };
        (v112, v113, v114, v115, v116, v117, v118, v119, v120, v121, v122, v123, v124, v125, v126.clone(), v127.clone(), v128.clone(), v129, v130, v131)
    };
    (v132, v133, v134, v135, v136, v137, v138, v139, v140, v141, v142, v143, v144, v145, v146.clone(), v147.clone(), v148.clone(), v149, v150, v151)
}
fn scanned_root_5(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: bool, mut v12: bool, mut v13: bool, mut v14: Rc<str>, mut v15: Rc<str>, mut v16: Rc<str>, mut v17: i32, mut v18: i32, mut v19: i32, mut v20: Rc<str>) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) {
    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = std::rc::Rc::<str>::from(v20.replace(&*v21, &v22));
    let mut v24: bool = std::fs::symlink_metadata(&*v20).map(|metadata| metadata.file_type().is_symlink()).unwrap_or(false);
    let mut v33: US0 = if v24 {
        US0::US0_LinkedEntry
    } else {
        let mut v26: bool = std::path::Path::new(&*v20).is_dir();
        if v26 {
            US0::US0_Directory
        } else {
            let mut v28: bool = std::path::Path::new(&*v20).is_file();
            if v28 {
                US0::US0_RegularFile
            } else {
                US0::US0_AbsentEntry
            }
        }
    };
    let (mut v118, mut v119, mut v120, mut v121, mut v122, mut v123, mut v124, mut v125, mut v126, mut v127, mut v128, mut v129, mut v130, mut v131, mut v132, mut v133, mut v134, mut v135, mut v136, mut v137): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = match &v33 {
        US0::US0_RegularFile => {
            let mut v34: Rc<str> = std::rc::Rc::<str>::from(v20.replace(&*v21, &v22));
            scanned_file_6(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19, v23.clone(), v20.clone(), v34.clone())
        }
        _ => {
            let (mut v98, mut v99, mut v100, mut v101, mut v102, mut v103, mut v104, mut v105, mut v106, mut v107, mut v108, mut v109, mut v110, mut v111, mut v112, mut v113, mut v114, mut v115, mut v116, mut v117): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = match &v33 {
                US0::US0_Directory => {
                    let (mut v55, mut v56): (bool, Rc<str>) = agile_todo_sources::eoie_agile_todo_sources(&*v20);
                    if v55 {
                        let mut v57: i32 = 0i32;
                        let (mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73, mut v74, mut v75, mut v76, mut v77): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = listed_files_13(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19, v20.clone(), v23.clone(), v56.clone(), v57);
                        (v58, v59, v60, v61, v62, v63, v64, v65, v66, v67, v68, v69, v70, v71, v72.clone(), v73.clone(), v74.clone(), v75, v76, v77)
                    } else {
                        (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19)
                    }
                }
                _ => {
                    (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19)
                }
            };
            (v98, v99, v100, v101, v102, v103, v104, v105, v106, v107, v108, v109, v110, v111, v112.clone(), v113.clone(), v114.clone(), v115, v116, v117)
        }
    };
    (v118, v119, v120, v121, v122, v123, v124, v125, v126, v127, v128, v129, v130, v131, v132.clone(), v133.clone(), v134.clone(), v135, v136, v137)
}
fn scanned_arguments_4(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: bool, mut v12: bool, mut v13: bool, mut v14: Rc<str>, mut v15: Rc<str>, mut v16: Rc<str>, mut v17: i32, mut v18: i32, mut v19: i32, mut v20: i32, mut v21: i32, mut v22: i32) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32, i32) {
    loop {
        let mut v23: bool = v21 <= v20;
        if v23 {
            return (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19, v22);
        } else {
            let mut v24: Rc<str> = usize::try_from(v20).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let mut v25: bool = v24.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--id"); } LIT.with(|lit| lit.clone()) };
            let mut v30: i32 = if v25 {
                2i32
            } else {
                let mut v26: bool = v24.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--tagged"); } LIT.with(|lit| lit.clone()) };
                let mut v28: bool = if v26 {
                    true
                } else {
                    let mut v27: bool = v24.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--json"); } LIT.with(|lit| lit.clone()) };
                    v27
                };
                if v28 {
                    1i32
                } else {
                    0i32
                }
            };
            let mut v31: bool = 0i32 < v30;
            if v31 {
                let mut v32: i32 = v20.wrapping_add(v30);
                (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22) = (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19, v32, v21, v22);
                continue;
            } else {
                let (mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = scanned_root_5(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19, v24.clone());
                let mut v74: i32 = v20.wrapping_add(1i32);
                let mut v75: i32 = v22.wrapping_add(1i32);
                (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22) = (v54, v55, v56, v57, v58, v59, v60, v61, v62, v63, v64, v65, v66, v67, v68.clone(), v69.clone(), v70.clone(), v71, v72, v73, v74, v21, v75);
                continue;
            }
        }
    }
}
fn scanned_listed_roots_15(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: bool, mut v12: bool, mut v13: bool, mut v14: Rc<str>, mut v15: Rc<str>, mut v16: Rc<str>, mut v17: i32, mut v18: i32, mut v19: i32, mut v20: Rc<str>, mut v21: i32, mut v22: i32) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32, i32) {
    loop {
        let mut v23: i32 = (v20.clone().len() as i32);
        let mut v24: bool = v23 <= v21;
        if v24 {
            return (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19, v22);
        } else {
            let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v26: i32 = v20.get(v21 as usize..).and_then(|rest| rest.find(&*v25)).map(|found| (found + v21 as usize) as i32).unwrap_or(-1);
            let mut v27: bool = v26 < 0i32;
            let mut v28: i32 = if v27 {
                v23
            } else {
                v26
            };
            let mut v29: Rc<str> = std::rc::Rc::<str>::from(v20.get(v21 as usize..v28 as usize).unwrap_or(""));
            let (mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = scanned_root_5(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19, v29.clone());
            let mut v50: i32 = v28.wrapping_add(1i32);
            let mut v51: i32 = v22.wrapping_add(1i32);
            (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22) = (v30, v31, v32, v33, v34, v35, v36, v37, v38, v39, v40, v41, v42, v43, v44.clone(), v45.clone(), v46.clone(), v47, v48, v49, v20.clone(), v50, v51);
            continue;
        }
    }
}
fn harvest_roots_from_16(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = HarvestRoot \""); } LIT.with(|lit| lit.clone()) };
        let mut v4: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v3)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let mut v5: bool = v4 < 0i32;
        if v5 {
            return v2.clone();
        } else {
            let mut v6: i32 = v4.wrapping_add(19i32);
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let mut v8: i32 = v0.get(v6 as usize..).and_then(|rest| rest.find(&*v7)).map(|found| (found + v6 as usize) as i32).unwrap_or(-1);
            let mut v9: Rc<str> = std::rc::Rc::<str>::from(v0.get(v6 as usize..v8 as usize).unwrap_or(""));
            let mut v10: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<str> = if v10 {
                v9.clone()
            } else {
                let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v11, &*v9, &*v12, &*v12].concat());
                v13.clone()
            };
            let mut v15: i32 = v8.wrapping_add(1i32);
            (v0, v1, v2) = (v0.clone(), v15, v14.clone());
            continue;
        }
    }
}
fn scanned_default_roots_14(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: bool, mut v12: bool, mut v13: bool, mut v14: Rc<str>, mut v15: Rc<str>, mut v16: Rc<str>, mut v17: i32, mut v18: i32, mut v19: i32, mut v20: Rc<str>) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32, i32) {
    let mut v21: i32 = 0i32;
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = harvest_roots_from_16(v15.clone(), v21, v22.clone());
    let mut v24: bool = v23.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v24 {
        let (mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = scanned_root_5(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19, v20.clone());
        (v25, v26, v27, v28, v29, v30, v31, v32, v33, v34, v35, v36, v37, v38, v39.clone(), v40.clone(), v41.clone(), v42, v43, v44, 1i32)
    } else {
        let mut v45: i32 = 0i32;
        let mut v46: i32 = 0i32;
        scanned_listed_roots_15(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19, v23.clone(), v45, v46)
    }
}
fn todos_entry_0() -> i32 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v1: i32 = argument_count_1();
    let mut v2: bool = v1 <= 1i32;
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile todos <root> [<path>...] [--json] [--tagged | --id <ID>]"); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile todos rejected: "); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v3, &*v5, &*v5, &*v5].concat());
        let mut v7: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v6 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v7
    } else {
        let mut v8: i32 = 1i32;
        let mut v9: Rc<str> = usize::try_from(v8).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v10: bool = std::fs::symlink_metadata(&*v9).map(|metadata| metadata.file_type().is_symlink()).unwrap_or(false);
        let mut v19: US0 = if v10 {
            US0::US0_LinkedEntry
        } else {
            let mut v12: bool = std::path::Path::new(&*v9).is_dir();
            if v12 {
                US0::US0_Directory
            } else {
                let mut v14: bool = std::path::Path::new(&*v9).is_file();
                if v14 {
                    US0::US0_RegularFile
                } else {
                    US0::US0_AbsentEntry
                }
            }
        };
        match &v19 {
            US0::US0_Directory => {
                let mut v20: bool = std::env::args().skip(4).any(|argument| argument == "--json");
                let mut v23: i32 = if v20 {
                    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"todos\":["); } LIT.with(|lit| lit.clone()) };
                    let mut v22: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v21.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                    v22
                } else {
                    0i32
                };
                let mut v24: bool = false;
                let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
                let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v27: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v25, &*v26, &*v26, &*v26].concat());
                let (mut v28, mut v29): (bool, Rc<str>) = match std::fs::read(&*v27) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
                let mut v30: Rc<str> = std::rc::Rc::<str>::from({ let args = std::env::args().skip(4).collect::<Vec<_>>(); match args.iter().position(|argument| argument == "--id") { Some(at) => args.get(at + 1).cloned().unwrap_or_default(), None => if args.iter().any(|argument| argument == "--tagged") { "*".to_owned() } else { String::new() } } });
                let (mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = scan_start_3(v20, v24, v29.clone(), v30.clone());
                let mut v51: i32 = 2i32;
                let mut v52: i32 = 0i32;
                let (mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32, i32) = scanned_arguments_4(v31, v32, v33, v34, v35, v36, v37, v38, v39, v40, v41, v42, v43, v44, v45.clone(), v46.clone(), v47.clone(), v48, v49, v50, v51, v1, v52);
                let mut v74: bool = v73 == 0i32;
                let (mut v96, mut v97, mut v98, mut v99, mut v100, mut v101, mut v102, mut v103, mut v104, mut v105, mut v106, mut v107, mut v108, mut v109, mut v110, mut v111, mut v112, mut v113, mut v114, mut v115, mut v116): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32, i32) = if v74 {
                    scanned_default_roots_14(v53, v54, v55, v56, v57, v58, v59, v60, v61, v62, v63, v64, v65, v66, v67.clone(), v68.clone(), v69.clone(), v70, v71, v72, v9.clone())
                } else {
                    (v53, v54, v55, v56, v57, v58, v59, v60, v61, v62, v63, v64, v65, v66, v67.clone(), v68.clone(), v69.clone(), v70, v71, v72, v73)
                };
                if v20 {
                    let mut v117: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                    let mut v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("],\"roots\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v119: Rc<str> = std::rc::Rc::<str>::from((v116).to_string());
                    let mut v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"files\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v121: Rc<str> = std::rc::Rc::<str>::from((v96).to_string());
                    let mut v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"scanned\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v123: Rc<str> = std::rc::Rc::<str>::from([&*v118, &*v119, &*v120, &*v121, &*v122].concat());
                    let mut v124: Rc<str> = std::rc::Rc::<str>::from((v97).to_string());
                    let mut v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"todo_count\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v126: Rc<str> = std::rc::Rc::<str>::from((v98).to_string());
                    let mut v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"skipped\":{\"nested\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v128: Rc<str> = std::rc::Rc::<str>::from((v99).to_string());
                    let mut v129: Rc<str> = std::rc::Rc::<str>::from([&*v124, &*v125, &*v126, &*v127, &*v128].concat());
                    let mut v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"vendor\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v131: Rc<str> = std::rc::Rc::<str>::from((v100).to_string());
                    let mut v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"generated\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v133: Rc<str> = std::rc::Rc::<str>::from((v102).to_string());
                    let mut v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"syntax\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v135: Rc<str> = std::rc::Rc::<str>::from([&*v130, &*v131, &*v132, &*v133, &*v134].concat());
                    let mut v136: Rc<str> = std::rc::Rc::<str>::from((v101).to_string());
                    let mut v137: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"binary\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v138: Rc<str> = std::rc::Rc::<str>::from((v103).to_string());
                    let mut v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"large\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v140: Rc<str> = std::rc::Rc::<str>::from((v104).to_string());
                    let mut v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"links\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v142: Rc<str> = std::rc::Rc::<str>::from((v105).to_string());
                    let mut v143: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"unreadable\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v144: Rc<str> = std::rc::Rc::<str>::from((v106).to_string());
                    let mut v145: Rc<str> = std::rc::Rc::<str>::from([&*v140, &*v141, &*v142, &*v143, &*v144].concat());
                    let mut v146: Rc<str> = std::rc::Rc::<str>::from([&*v136, &*v137, &*v138, &*v139, &*v145].concat());
                    let mut v147: Rc<str> = std::rc::Rc::<str>::from([&*v123, &*v129, &*v135, &*v146, &*v26].concat());
                    let mut v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("},\"elapsed_ms\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v149: Rc<str> = std::rc::Rc::<str>::from((v117).to_string());
                    let mut v150: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
                    let mut v151: Rc<str> = std::rc::Rc::<str>::from([&*v147, &*v148, &*v149, &*v150, &*v26].concat());
                    let mut v152: i32 = (v151.clone().len() as i32);
                    let mut v153: i32 = v152.wrapping_sub(1i32);
                    let mut v154: Rc<str> = std::rc::Rc::<str>::from(v151.get(0i32 as usize..v153 as usize).unwrap_or(""));
                    let mut v155: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"stale\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v156: Rc<str> = std::rc::Rc::<str>::from((v113).to_string());
                    let mut v157: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"unknown\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v158: Rc<str> = std::rc::Rc::<str>::from((v114).to_string());
                    let mut v159: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"wide\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v160: Rc<str> = std::rc::Rc::<str>::from((v115).to_string());
                    let mut v161: Rc<str> = std::rc::Rc::<str>::from([&*v158, &*v159, &*v160, &*v150, &*v26].concat());
                    let mut v162: Rc<str> = std::rc::Rc::<str>::from([&*v154, &*v155, &*v156, &*v157, &*v161].concat());
                    let mut v163: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v162.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                    v163
                } else {
                    let mut v164: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                    let mut v165: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie agile todos ok roots="); } LIT.with(|lit| lit.clone()) };
                    let mut v166: Rc<str> = std::rc::Rc::<str>::from((v116).to_string());
                    let mut v167: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" files="); } LIT.with(|lit| lit.clone()) };
                    let mut v168: Rc<str> = std::rc::Rc::<str>::from((v96).to_string());
                    let mut v169: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" scanned="); } LIT.with(|lit| lit.clone()) };
                    let mut v170: Rc<str> = std::rc::Rc::<str>::from([&*v165, &*v166, &*v167, &*v168, &*v169].concat());
                    let mut v171: Rc<str> = std::rc::Rc::<str>::from((v97).to_string());
                    let mut v172: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" todos="); } LIT.with(|lit| lit.clone()) };
                    let mut v173: Rc<str> = std::rc::Rc::<str>::from((v98).to_string());
                    let mut v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" skipped="); } LIT.with(|lit| lit.clone()) };
                    let mut v175: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("nested:"); } LIT.with(|lit| lit.clone()) };
                    let mut v176: Rc<str> = std::rc::Rc::<str>::from((v99).to_string());
                    let mut v177: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",vendor:"); } LIT.with(|lit| lit.clone()) };
                    let mut v178: Rc<str> = std::rc::Rc::<str>::from((v100).to_string());
                    let mut v179: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",generated:"); } LIT.with(|lit| lit.clone()) };
                    let mut v180: Rc<str> = std::rc::Rc::<str>::from([&*v175, &*v176, &*v177, &*v178, &*v179].concat());
                    let mut v181: Rc<str> = std::rc::Rc::<str>::from((v102).to_string());
                    let mut v182: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",syntax:"); } LIT.with(|lit| lit.clone()) };
                    let mut v183: Rc<str> = std::rc::Rc::<str>::from((v101).to_string());
                    let mut v184: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",binary:"); } LIT.with(|lit| lit.clone()) };
                    let mut v185: Rc<str> = std::rc::Rc::<str>::from((v103).to_string());
                    let mut v186: Rc<str> = std::rc::Rc::<str>::from([&*v181, &*v182, &*v183, &*v184, &*v185].concat());
                    let mut v187: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",large:"); } LIT.with(|lit| lit.clone()) };
                    let mut v188: Rc<str> = std::rc::Rc::<str>::from((v104).to_string());
                    let mut v189: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",links:"); } LIT.with(|lit| lit.clone()) };
                    let mut v190: Rc<str> = std::rc::Rc::<str>::from((v105).to_string());
                    let mut v191: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",unreadable:"); } LIT.with(|lit| lit.clone()) };
                    let mut v192: Rc<str> = std::rc::Rc::<str>::from((v106).to_string());
                    let mut v193: Rc<str> = std::rc::Rc::<str>::from([&*v189, &*v190, &*v191, &*v192, &*v26].concat());
                    let mut v194: Rc<str> = std::rc::Rc::<str>::from([&*v180, &*v186, &*v187, &*v188, &*v193].concat());
                    let mut v195: Rc<str> = std::rc::Rc::<str>::from([&*v171, &*v172, &*v173, &*v174, &*v194].concat());
                    let mut v196: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                    let mut v197: Rc<str> = std::rc::Rc::<str>::from((v164).to_string());
                    let mut v198: Rc<str> = std::rc::Rc::<str>::from([&*v170, &*v195, &*v196, &*v197, &*v26].concat());
                    let mut v199: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" stale="); } LIT.with(|lit| lit.clone()) };
                    let mut v200: Rc<str> = std::rc::Rc::<str>::from((v113).to_string());
                    let mut v201: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" unknown="); } LIT.with(|lit| lit.clone()) };
                    let mut v202: Rc<str> = std::rc::Rc::<str>::from((v114).to_string());
                    let mut v203: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" wide="); } LIT.with(|lit| lit.clone()) };
                    let mut v204: Rc<str> = std::rc::Rc::<str>::from((v115).to_string());
                    let mut v205: Rc<str> = std::rc::Rc::<str>::from([&*v203, &*v204, &*v26, &*v26, &*v26].concat());
                    let mut v206: Rc<str> = std::rc::Rc::<str>::from([&*v199, &*v200, &*v201, &*v202, &*v205].concat());
                    let mut v207: Rc<str> = std::rc::Rc::<str>::from([&*v198, &*v206, &*v26, &*v26, &*v26].concat());
                    let mut v208: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v207.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                    v208
                }
            }
            _ => {
                let mut v210: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the backlog root "); } LIT.with(|lit| lit.clone()) };
                let mut v211: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is not a directory"); } LIT.with(|lit| lit.clone()) };
                let mut v212: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v213: Rc<str> = std::rc::Rc::<str>::from([&*v210, &*v9, &*v211, &*v212, &*v212].concat());
                let mut v214: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile todos rejected: "); } LIT.with(|lit| lit.clone()) };
                let mut v215: Rc<str> = std::rc::Rc::<str>::from([&*v214, &*v213, &*v212, &*v212, &*v212].concat());
                let mut v216: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v215 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v216
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        todos_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn todo_sites_17(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: bool = false;
    let mut v3: bool = true;
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v4, &*v5, &*v5, &*v5].concat());
    let (mut v7, mut v8): (bool, Rc<str>) = match std::fs::read(&*v6) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
    let (mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = scan_start_3(v2, v3, v8.clone(), v1.clone());
    let (mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32, i32) = scanned_default_roots_14(v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23.clone(), v24.clone(), v25.clone(), v26, v27, v28, v0.clone());
    v43.clone()
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        todo_sites_17(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_todos_run() -> i32 {
    closure0()()
}
pub fn eoie_agile_todo_sites(v0: &str, v1: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
