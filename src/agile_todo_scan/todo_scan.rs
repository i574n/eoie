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
enum US1 {
    US1_ScanPath(US2),
    US1_SkipNestedRepository,
    US1_SkipVendorDirectory,
    US1_SkipUnknownSyntax,
    US1_SkipGeneratedOutput,
    US1_SkipLinkedEntry,
}
#[derive(Clone)]
enum US3 {
    US3_ScanContent,
    US3_SkipBinary,
    US3_SkipLarge,
    US3_SkipUnreadable,
}
#[derive(Clone)]
enum US4 {
    US4_OpenTodo,
    US4_StaleTodo,
    US4_UnknownTodo,
}
fn syntax_of_code_5(mut v0: i32) -> US2 {
    let mut v1: bool = 2i32 == v0;
    if v1 {
        US2::US2_HashFamily
    } else {
        let mut v3: bool = 3i32 == v0;
        if v3 {
            US2::US2_DashFamily
        } else {
            let mut v5: bool = 4i32 == v0;
            if v5 {
                US2::US2_HaskellFamily
            } else {
                let mut v7: bool = 5i32 == v0;
                if v7 {
                    US2::US2_LeanFamily
                } else {
                    let mut v9: bool = 6i32 == v0;
                    if v9 {
                        US2::US2_LispFamily
                    } else {
                        let mut v11: bool = 7i32 == v0;
                        if v11 {
                            US2::US2_ErlangFamily
                        } else {
                            let mut v13: bool = 8i32 == v0;
                            if v13 {
                                US2::US2_MarkupFamily
                            } else {
                                let mut v15: bool = 9i32 == v0;
                                if v15 {
                                    US2::US2_MlFamily
                                } else {
                                    let mut v17: bool = 10i32 == v0;
                                    if v17 {
                                        US2::US2_PascalFamily
                                    } else {
                                        let mut v19: bool = 11i32 == v0;
                                        if v19 {
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
}
fn path_verdict_of_code_4(mut v0: i32) -> US1 {
    let mut v1: bool = 100i32 <= v0;
    if v1 {
        let mut v2: i32 = v0 - 100i32;
        let mut v3: US2 = syntax_of_code_5(v2);
        US1::US1_ScanPath(v3.clone())
    } else {
        let mut v5: bool = 1i32 == v0;
        if v5 {
            US1::US1_SkipNestedRepository
        } else {
            let mut v7: bool = 2i32 == v0;
            if v7 {
                US1::US1_SkipVendorDirectory
            } else {
                let mut v9: bool = 4i32 == v0;
                if v9 {
                    US1::US1_SkipGeneratedOutput
                } else {
                    let mut v11: bool = 5i32 == v0;
                    if v11 {
                        US1::US1_SkipLinkedEntry
                    } else {
                        US1::US1_SkipUnknownSyntax
                    }
                }
            }
        }
    }
}
fn counted_path_6(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: US1) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) {
    let mut v12: i32 = v0 + 1;
    let (mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83, mut v84): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = match &v11 {
        US1::US1_ScanPath(v13) => {
            let mut v13: US2 = v13.clone();
            (v12, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10)
        }
        _ => {
            let (mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = match &v11 {
                US1::US1_SkipNestedRepository => {
                    let mut v14: i32 = v3 + 1;
                    (v12, v1, v2, v14, v4, v5, v6, v7, v8, v9, v10)
                }
                _ => {
                    let (mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = match &v11 {
                        US1::US1_SkipVendorDirectory => {
                            let mut v15: i32 = v4 + 1;
                            (v12, v1, v2, v3, v15, v5, v6, v7, v8, v9, v10)
                        }
                        _ => {
                            let (mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = match &v11 {
                                US1::US1_SkipUnknownSyntax => {
                                    let mut v16: i32 = v5 + 1;
                                    (v12, v1, v2, v3, v4, v16, v6, v7, v8, v9, v10)
                                }
                                _ => {
                                    let (mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = match &v11 {
                                        US1::US1_SkipGeneratedOutput => {
                                            let mut v17: i32 = v6 + 1;
                                            (v12, v1, v2, v3, v4, v5, v17, v7, v8, v9, v10)
                                        }
                                        _ => {
                                            let (mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = match &v11 {
                                                US1::US1_SkipLinkedEntry => {
                                                    let mut v18: i32 = v9 + 1;
                                                    (v12, v1, v2, v3, v4, v5, v6, v7, v8, v18, v10)
                                                }
                                                _ => unreachable!(),
                                            };
                                            (v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29)
                                        }
                                    };
                                    (v30, v31, v32, v33, v34, v35, v36, v37, v38, v39, v40)
                                }
                            };
                            (v41, v42, v43, v44, v45, v46, v47, v48, v49, v50, v51)
                        }
                    };
                    (v52, v53, v54, v55, v56, v57, v58, v59, v60, v61, v62)
                }
            };
            (v63, v64, v65, v66, v67, v68, v69, v70, v71, v72, v73)
        }
    };
    (v74, v75, v76, v77, v78, v79, v80, v81, v82, v83, v84)
}
fn counted_content_7(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: US3) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) {
    let (mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = match &v11 {
        US3::US3_ScanContent => {
            let mut v12: i32 = v1 + 1;
            (v0, v12, v2, v3, v4, v5, v6, v7, v8, v9, v10)
        }
        _ => {
            let (mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = match &v11 {
                US3::US3_SkipBinary => {
                    let mut v13: i32 = v7 + 1;
                    (v0, v1, v2, v3, v4, v5, v6, v13, v8, v9, v10)
                }
                _ => {
                    let (mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = match &v11 {
                        US3::US3_SkipLarge => {
                            let mut v14: i32 = v8 + 1;
                            (v0, v1, v2, v3, v4, v5, v6, v7, v14, v9, v10)
                        }
                        _ => {
                            let (mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = match &v11 {
                                US3::US3_SkipUnreadable => {
                                    let mut v15: i32 = v10 + 1;
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
fn counted_todos_8(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: i32) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) {
    let mut v12: i32 = v2 + v11;
    (v0, v1, v12, v3, v4, v5, v6, v7, v8, v9, v10)
}
fn standings_from_9(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: bool, mut v12: bool, mut v13: Rc<str>, mut v14: i32, mut v15: i32, mut v16: Rc<str>, mut v17: i32) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, Rc<str>, i32, i32) {
    let mut v18: i32 = v16.len() as i32;
    let mut v19: bool = v18 <= v17;
    let (mut v113, mut v114, mut v115, mut v116, mut v117, mut v118, mut v119, mut v120, mut v121, mut v122, mut v123, mut v124, mut v125, mut v126, mut v127, mut v128): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, Rc<str>, i32, i32) = if v19 {
        (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13.clone(), v14, v15)
    } else {
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v21: i32 = v16.get(v17 as usize..).and_then(|rest| rest.find(&*v20)).map(|found| (found + v17 as usize) as i32).unwrap_or(-1);
        let mut v22: bool = v21 < 0;
        let mut v24: i32 = if v22 {
            let mut v23: i32 = v16.len() as i32;
            v23
        } else {
            v21
        };
        let mut v25: Rc<str> = std::rc::Rc::<str>::from(v16.get(v17 as usize..v24 as usize).unwrap_or(""));
        let mut v26: Rc<str> = agile_todo_lines::eoie_agile_todo_id(&*v25);
        let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v28: bool = v26 == v27 ;
        let (mut v80, mut v81, mut v82, mut v83, mut v84, mut v85, mut v86, mut v87, mut v88, mut v89, mut v90, mut v91, mut v92, mut v93, mut v94, mut v95): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, Rc<str>, i32, i32) = if v28 {
            (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13.clone(), v14, v15)
        } else {
            let mut v29: Rc<str> = agile_task_census::eoie_agile_task_status(&*v13, &*v26);
            let mut v30: bool = v29 == v27 ;
            let mut v37: US4 = if v30 {
                US4::US4_UnknownTodo
            } else {
                let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Done"); } LIT.with(|lit| lit.clone()) };
                let mut v33: bool = v29 == v32 ;
                if v33 {
                    US4::US4_StaleTodo
                } else {
                    US4::US4_OpenTodo
                }
            };
            let (mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73, mut v74, mut v75, mut v76, mut v77, mut v78, mut v79): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, Rc<str>, i32, i32) = match &v37 {
                US4::US4_OpenTodo => {
                    (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13.clone(), v14, v15)
                }
                _ => {
                    let mut v45: i32 = if v11 {
                        0i32
                    } else {
                        let mut v41: Rc<str> = match &v37 {
                            US4::US4_StaleTodo => {
                                let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stale (item is Done): "); } LIT.with(|lit| lit.clone()) };
                                v38.clone()
                            }
                            US4::US4_UnknownTodo => {
                                let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown (no backlog item): "); } LIT.with(|lit| lit.clone()) };
                                v39.clone()
                            }
                            _ => unreachable!(),
                        };
                        let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" at "); } LIT.with(|lit| lit.clone()) };
                        let mut v43: Rc<str> = std::rc::Rc::<str>::from([&*v41, &*v26, &*v42, &*v25, &*v27].concat());
                        let mut v44: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v43.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                        v44
                    };
                    let (mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, Rc<str>, i32, i32) = match &v37 {
                        US4::US4_StaleTodo => {
                            let mut v46: i32 = v14 + 1;
                            (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13.clone(), v46, v15)
                        }
                        _ => {
                            let mut v47: i32 = v15 + 1;
                            (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13.clone(), v14, v47)
                        }
                    };
                    (v48, v49, v50, v51, v52, v53, v54, v55, v56, v57, v58, v59, v60, v61.clone(), v62, v63)
                }
            };
            (v64, v65, v66, v67, v68, v69, v70, v71, v72, v73, v74, v75, v76, v77.clone(), v78, v79)
        };
        let mut v96: i32 = v24 + 1;
        let (mut v97, mut v98, mut v99, mut v100, mut v101, mut v102, mut v103, mut v104, mut v105, mut v106, mut v107, mut v108, mut v109, mut v110, mut v111, mut v112): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, Rc<str>, i32, i32) = standings_from_9(v80, v81, v82, v83, v84, v85, v86, v87, v88, v89, v90, v91, v92, v93.clone(), v94, v95, v16.clone(), v96);
        (v97, v98, v99, v100, v101, v102, v103, v104, v105, v106, v107, v108, v109, v110.clone(), v111, v112)
    };
    (v113, v114, v115, v116, v117, v118, v119, v120, v121, v122, v123, v124, v125, v126.clone(), v127, v128)
}
fn scanned_file_3(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: bool, mut v12: bool, mut v13: Rc<str>, mut v14: i32, mut v15: i32, mut v16: Rc<str>, mut v17: Rc<str>, mut v18: Rc<str>) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, Rc<str>, i32, i32) {
    let mut v19: i32 = std::fs::symlink_metadata(&*v17).map(|metadata| if metadata.file_type().is_symlink() { 3 } else if metadata.is_dir() { 2 } else if metadata.is_file() { 1 } else { 0 }).unwrap_or(0);
    let mut v20: bool = 1i32 == v19;
    let mut v29: US0 = if v20 {
        US0::US0_RegularFile
    } else {
        let mut v22: bool = 2i32 == v19;
        if v22 {
            US0::US0_Directory
        } else {
            let mut v24: bool = 3i32 == v19;
            if v24 {
                US0::US0_LinkedEntry
            } else {
                US0::US0_AbsentEntry
            }
        }
    };
    let (mut v273, mut v274, mut v275, mut v276, mut v277, mut v278, mut v279, mut v280, mut v281, mut v282, mut v283, mut v284, mut v285, mut v286, mut v287, mut v288): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, Rc<str>, i32, i32) = match &v29 {
        US0::US0_RegularFile => {
            let mut v30: bool = std::path::Path::new(&*v17).with_extension("spi").is_file();
            let mut v31: i32 = agile_todo_paths::eoie_agile_todo_path_verdict(&*v18, v30);
            let mut v32: US1 = path_verdict_of_code_4(v31);
            let (mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = counted_path_6(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v32.clone());
            match &v32 {
                US1::US1_ScanPath(v44) => {
                    let mut v44: US2 = v44.clone();
                    let mut v45: u64 = std::fs::metadata(&*v17).map(|metadata| metadata.len()).unwrap_or(0);
                    let mut v46: bool = v45 > 1048576u64;
                    let mut v49: US3 = if v46 {
                        US3::US3_SkipLarge
                    } else {
                        US3::US3_ScanContent
                    };
                    match &v49 {
                        US3::US3_SkipLarge => {
                            let mut v50: US3 = US3::US3_SkipLarge;
                            let (mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = counted_content_7(v33, v34, v35, v36, v37, v38, v39, v40, v41, v42, v43, v50.clone());
                            (v51, v52, v53, v54, v55, v56, v57, v58, v59, v60, v61, v11, v12, v13.clone(), v14, v15)
                        }
                        _ => {
                            let (mut v62, mut v63): (bool, Rc<str>) = match std::fs::read(&*v17) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
                            let mut v66: US3 = if v62 {
                                US3::US3_SkipBinary
                            } else {
                                US3::US3_ScanContent
                            };
                            let (mut v155, mut v156, mut v157, mut v158, mut v159, mut v160, mut v161, mut v162, mut v163, mut v164, mut v165, mut v166, mut v167, mut v168, mut v169, mut v170): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, Rc<str>, i32, i32) = match &v66 {
                                US3::US3_ScanContent => {
                                    let mut v77: i32 = match &v44 {
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
                                    let (mut v78, mut v79): (Rc<str>, i32) = agile_todo_lines::eoie_agile_todo_findings(v77, &*v63, &*v16, v11);
                                    let mut v80: US3 = US3::US3_ScanContent;
                                    let (mut v81, mut v82, mut v83, mut v84, mut v85, mut v86, mut v87, mut v88, mut v89, mut v90, mut v91): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = counted_content_7(v33, v34, v35, v36, v37, v38, v39, v40, v41, v42, v43, v80.clone());
                                    let (mut v92, mut v93, mut v94, mut v95, mut v96, mut v97, mut v98, mut v99, mut v100, mut v101, mut v102): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = counted_todos_8(v81, v82, v83, v84, v85, v86, v87, v88, v89, v90, v91, v79);
                                    let mut v103: bool = v79 <= 0i32;
                                    if v103 {
                                        (v92, v93, v94, v95, v96, v97, v98, v99, v100, v101, v102, v11, v12, v13.clone(), v14, v15)
                                    } else {
                                        let mut v104: bool = v11 && v12;
                                        let mut v108: Rc<str> = if v104 {
                                            let mut v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                                            let mut v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                            let mut v107: Rc<str> = std::rc::Rc::<str>::from([&*v105, &*v78, &*v106, &*v106, &*v106].concat());
                                            v107.clone()
                                        } else {
                                            v78.clone()
                                        };
                                        let mut v109: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v108.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                                        let mut v110: bool = true;
                                        let mut v111: i32 = 0i32;
                                        let (mut v112, mut v113, mut v114, mut v115, mut v116, mut v117, mut v118, mut v119, mut v120, mut v121, mut v122, mut v123, mut v124, mut v125, mut v126, mut v127): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, Rc<str>, i32, i32) = standings_from_9(v92, v93, v94, v95, v96, v97, v98, v99, v100, v101, v102, v11, v110, v13.clone(), v14, v15, v78.clone(), v111);
                                        (v112, v113, v114, v115, v116, v117, v118, v119, v120, v121, v122, v123, v124, v125.clone(), v126, v127)
                                    }
                                }
                                _ => {
                                    let (mut v144, mut v145, mut v146, mut v147, mut v148, mut v149, mut v150, mut v151, mut v152, mut v153, mut v154): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = counted_content_7(v33, v34, v35, v36, v37, v38, v39, v40, v41, v42, v43, v66.clone());
                                    (v144, v145, v146, v147, v148, v149, v150, v151, v152, v153, v154, v11, v12, v13.clone(), v14, v15)
                                }
                            };
                            (v155, v156, v157, v158, v159, v160, v161, v162, v163, v164, v165, v166, v167, v168.clone(), v169, v170)
                        }
                    }
                }
                _ => {
                    (v33, v34, v35, v36, v37, v38, v39, v40, v41, v42, v43, v11, v12, v13.clone(), v14, v15)
                }
            }
        }
        _ => {
            let (mut v257, mut v258, mut v259, mut v260, mut v261, mut v262, mut v263, mut v264, mut v265, mut v266, mut v267, mut v268, mut v269, mut v270, mut v271, mut v272): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, Rc<str>, i32, i32) = match &v29 {
                US0::US0_LinkedEntry => {
                    let mut v203: US1 = US1::US1_SkipLinkedEntry;
                    let (mut v204, mut v205, mut v206, mut v207, mut v208, mut v209, mut v210, mut v211, mut v212, mut v213, mut v214): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = counted_path_6(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v203.clone());
                    (v204, v205, v206, v207, v208, v209, v210, v211, v212, v213, v214, v11, v12, v13.clone(), v14, v15)
                }
                _ => {
                    let mut v215: i32 = agile_todo_paths::eoie_agile_todo_path_verdict(&*v18, false);
                    let mut v216: US1 = path_verdict_of_code_4(v215);
                    let (mut v217, mut v218, mut v219, mut v220, mut v221, mut v222, mut v223, mut v224, mut v225, mut v226, mut v227): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = counted_path_6(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v216.clone());
                    let (mut v241, mut v242, mut v243, mut v244, mut v245, mut v246, mut v247, mut v248, mut v249, mut v250, mut v251, mut v252, mut v253, mut v254, mut v255, mut v256): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, Rc<str>, i32, i32) = match &v216 {
                        US1::US1_ScanPath(v228) => {
                            let mut v228: US2 = v228.clone();
                            let mut v229: US3 = US3::US3_SkipUnreadable;
                            let (mut v230, mut v231, mut v232, mut v233, mut v234, mut v235, mut v236, mut v237, mut v238, mut v239, mut v240): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = counted_content_7(v217, v218, v219, v220, v221, v222, v223, v224, v225, v226, v227, v229.clone());
                            (v230, v231, v232, v233, v234, v235, v236, v237, v238, v239, v240, v11, v12, v13.clone(), v14, v15)
                        }
                        _ => {
                            (v217, v218, v219, v220, v221, v222, v223, v224, v225, v226, v227, v11, v12, v13.clone(), v14, v15)
                        }
                    };
                    (v241, v242, v243, v244, v245, v246, v247, v248, v249, v250, v251, v252, v253, v254.clone(), v255, v256)
                }
            };
            (v257, v258, v259, v260, v261, v262, v263, v264, v265, v266, v267, v268, v269, v270.clone(), v271, v272)
        }
    };
    (v273, v274, v275, v276, v277, v278, v279, v280, v281, v282, v283, v284, v285, v286.clone(), v287, v288)
}
fn listed_files_10(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: bool, mut v12: bool, mut v13: Rc<str>, mut v14: i32, mut v15: i32, mut v16: Rc<str>, mut v17: Rc<str>, mut v18: Rc<str>, mut v19: i32) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, Rc<str>, i32, i32) {
    let mut v20: i32 = v18.len() as i32;
    let mut v21: bool = v20 <= v19;
    let (mut v113, mut v114, mut v115, mut v116, mut v117, mut v118, mut v119, mut v120, mut v121, mut v122, mut v123, mut v124, mut v125, mut v126, mut v127, mut v128): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, Rc<str>, i32, i32) = if v21 {
        (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13.clone(), v14, v15)
    } else {
        let mut v22: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
        let mut v23: i32 = v18.get(v19 as usize..).and_then(|rest| rest.find(&*v22)).map(|found| (found + v19 as usize) as i32).unwrap_or(-1);
        let mut v24: bool = v23 < 0;
        let mut v26: i32 = if v24 {
            let mut v25: i32 = v18.len() as i32;
            v25
        } else {
            v23
        };
        let mut v27: Rc<str> = std::rc::Rc::<str>::from(v18.get(v19 as usize..v26 as usize).unwrap_or(""));
        let mut v28: i32 = v26 + 1;
        let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v30: bool = v27 == v29 ;
        let (mut v97, mut v98, mut v99, mut v100, mut v101, mut v102, mut v103, mut v104, mut v105, mut v106, mut v107, mut v108, mut v109, mut v110, mut v111, mut v112): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, Rc<str>, i32, i32) = if v30 {
            let (mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, Rc<str>, i32, i32) = listed_files_10(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13.clone(), v14, v15, v16.clone(), v17.clone(), v18.clone(), v28);
            (v31, v32, v33, v34, v35, v36, v37, v38, v39, v40, v41, v42, v43, v44.clone(), v45, v46)
        } else {
            let mut v47: bool = v27 == v29 ;
            let mut v55: Rc<str> = if v47 {
                v17.clone()
            } else {
                let mut v48: bool = v17 == v29 ;
                if v48 {
                    v27.clone()
                } else {
                    let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                    let mut v50: bool = v17.ends_with(&*v49);
                    if v50 {
                        let mut v51: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v27, &*v29, &*v29, &*v29].concat());
                        v51.clone()
                    } else {
                        let mut v52: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v49, &*v27, &*v29, &*v29].concat());
                        v52.clone()
                    }
                }
            };
            let mut v56: bool = v27 == v29 ;
            let mut v64: Rc<str> = if v56 {
                v16.clone()
            } else {
                let mut v57: bool = v16 == v29 ;
                if v57 {
                    v27.clone()
                } else {
                    let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                    let mut v59: bool = v16.ends_with(&*v58);
                    if v59 {
                        let mut v60: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v27, &*v29, &*v29, &*v29].concat());
                        v60.clone()
                    } else {
                        let mut v61: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v58, &*v27, &*v29, &*v29].concat());
                        v61.clone()
                    }
                }
            };
            let (mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73, mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, Rc<str>, i32, i32) = scanned_file_3(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13.clone(), v14, v15, v55.clone(), v64.clone(), v27.clone());
            let (mut v81, mut v82, mut v83, mut v84, mut v85, mut v86, mut v87, mut v88, mut v89, mut v90, mut v91, mut v92, mut v93, mut v94, mut v95, mut v96): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, Rc<str>, i32, i32) = listed_files_10(v65, v66, v67, v68, v69, v70, v71, v72, v73, v74, v75, v76, v77, v78.clone(), v79, v80, v16.clone(), v17.clone(), v18.clone(), v28);
            (v81, v82, v83, v84, v85, v86, v87, v88, v89, v90, v91, v92, v93, v94.clone(), v95, v96)
        };
        (v97, v98, v99, v100, v101, v102, v103, v104, v105, v106, v107, v108, v109, v110.clone(), v111, v112)
    };
    (v113, v114, v115, v116, v117, v118, v119, v120, v121, v122, v123, v124, v125, v126.clone(), v127, v128)
}
fn scanned_root_2(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: bool, mut v12: bool, mut v13: Rc<str>, mut v14: i32, mut v15: i32, mut v16: Rc<str>) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, Rc<str>, i32, i32) {
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v19: Rc<str> = std::rc::Rc::<str>::from(v16.replace(&*v17, &v18));
    let mut v20: i32 = std::fs::symlink_metadata(&*v16).map(|metadata| if metadata.file_type().is_symlink() { 3 } else if metadata.is_dir() { 2 } else if metadata.is_file() { 1 } else { 0 }).unwrap_or(0);
    let mut v21: bool = 1i32 == v20;
    let mut v30: US0 = if v21 {
        US0::US0_RegularFile
    } else {
        let mut v23: bool = 2i32 == v20;
        if v23 {
            US0::US0_Directory
        } else {
            let mut v25: bool = 3i32 == v20;
            if v25 {
                US0::US0_LinkedEntry
            } else {
                US0::US0_AbsentEntry
            }
        }
    };
    let (mut v99, mut v100, mut v101, mut v102, mut v103, mut v104, mut v105, mut v106, mut v107, mut v108, mut v109, mut v110, mut v111, mut v112, mut v113, mut v114): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, Rc<str>, i32, i32) = match &v30 {
        US0::US0_RegularFile => {
            let mut v31: Rc<str> = std::rc::Rc::<str>::from(v16.replace(&*v17, &v18));
            scanned_file_3(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13.clone(), v14, v15, v19.clone(), v16.clone(), v31.clone())
        }
        _ => {
            let (mut v83, mut v84, mut v85, mut v86, mut v87, mut v88, mut v89, mut v90, mut v91, mut v92, mut v93, mut v94, mut v95, mut v96, mut v97, mut v98): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, Rc<str>, i32, i32) = match &v30 {
                US0::US0_Directory => {
                    let (mut v48, mut v49): (bool, Rc<str>) = agile_todo_sources::eoie_agile_todo_sources(&*v16);
                    if v48 {
                        let mut v50: i32 = 0i32;
                        let (mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64, mut v65, mut v66): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, Rc<str>, i32, i32) = listed_files_10(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13.clone(), v14, v15, v16.clone(), v19.clone(), v49.clone(), v50);
                        (v51, v52, v53, v54, v55, v56, v57, v58, v59, v60, v61, v62, v63, v64.clone(), v65, v66)
                    } else {
                        (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13.clone(), v14, v15)
                    }
                }
                _ => {
                    (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13.clone(), v14, v15)
                }
            };
            (v83, v84, v85, v86, v87, v88, v89, v90, v91, v92, v93, v94, v95, v96.clone(), v97, v98)
        }
    };
    (v99, v100, v101, v102, v103, v104, v105, v106, v107, v108, v109, v110, v111, v112.clone(), v113, v114)
}
fn scanned_arguments_1(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: bool, mut v12: bool, mut v13: Rc<str>, mut v14: i32, mut v15: i32, mut v16: i32, mut v17: i32, mut v18: i32) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, Rc<str>, i32, i32, i32) {
    loop {
        let mut v19: bool = v17 <= v16;
        if v19 {
            return (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13.clone(), v14, v15, v18);
        } else {
            let mut v20: Rc<str> = usize::try_from(v16).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--json"); } LIT.with(|lit| lit.clone()) };
            let mut v22: bool = v20 == v21 ;
            if v22 {
                let mut v23: i32 = v16 + 1;
                (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18) = (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13.clone(), v14, v15, v23, v17, v18);
                continue;
            } else {
                let (mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55, mut v56): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, Rc<str>, i32, i32) = scanned_root_2(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13.clone(), v14, v15, v20.clone());
                let mut v57: i32 = v16 + 1;
                let mut v58: i32 = v18 + 1;
                (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18) = (v41, v42, v43, v44, v45, v46, v47, v48, v49, v50, v51, v52, v53, v54.clone(), v55, v56, v57, v17, v58);
                continue;
            }
        }
    }
}
fn todos_entry_0() -> i32 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v1: i32 = std::env::args().skip(2).count() as i32;
    let mut v2: bool = v1 <= 1i32;
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile todos <root> [<path>...] [--json]"); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile todos rejected: "); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v3, &*v5, &*v5, &*v5].concat());
        let mut v7: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v6 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v7
    } else {
        let mut v8: i32 = 1i32;
        let mut v9: Rc<str> = usize::try_from(v8).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v10: i32 = std::fs::symlink_metadata(&*v9).map(|metadata| if metadata.file_type().is_symlink() { 3 } else if metadata.is_dir() { 2 } else if metadata.is_file() { 1 } else { 0 }).unwrap_or(0);
        let mut v11: bool = 1i32 == v10;
        let mut v20: US0 = if v11 {
            US0::US0_RegularFile
        } else {
            let mut v13: bool = 2i32 == v10;
            if v13 {
                US0::US0_Directory
            } else {
                let mut v15: bool = 3i32 == v10;
                if v15 {
                    US0::US0_LinkedEntry
                } else {
                    US0::US0_AbsentEntry
                }
            }
        };
        match &v20 {
            US0::US0_Directory => {
                let mut v21: bool = std::env::args().skip(4).any(|argument| argument == "--json");
                let mut v24: i32 = if v21 {
                    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"todos\":["); } LIT.with(|lit| lit.clone()) };
                    let mut v23: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v22.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                    v23
                } else {
                    0i32
                };
                let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
                let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v27: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v25, &*v26, &*v26, &*v26].concat());
                let (mut v28, mut v29): (bool, Rc<str>) = match std::fs::read(&*v27) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
                let mut v30: i32 = 0i32;
                let mut v31: i32 = 0i32;
                let mut v32: i32 = 0i32;
                let mut v33: i32 = 0i32;
                let mut v34: i32 = 0i32;
                let mut v35: i32 = 0i32;
                let mut v36: i32 = 0i32;
                let mut v37: i32 = 0i32;
                let mut v38: i32 = 0i32;
                let mut v39: i32 = 0i32;
                let mut v40: i32 = 0i32;
                let mut v41: bool = false;
                let mut v42: i32 = 0i32;
                let mut v43: i32 = 0i32;
                let mut v44: i32 = 2i32;
                let mut v45: i32 = 0i32;
                let (mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, Rc<str>, i32, i32, i32) = scanned_arguments_1(v30, v31, v32, v33, v34, v35, v36, v37, v38, v39, v40, v21, v41, v29.clone(), v42, v43, v44, v1, v45);
                let mut v63: bool = v62 == 0i32;
                let (mut v80, mut v81, mut v82, mut v83, mut v84, mut v85, mut v86, mut v87, mut v88, mut v89, mut v90, mut v91, mut v92, mut v93, mut v94, mut v95, mut v96): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, Rc<str>, i32, i32, i32) = if v63 {
                    let (mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73, mut v74, mut v75, mut v76, mut v77, mut v78, mut v79): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, Rc<str>, i32, i32) = scanned_root_2(v46, v47, v48, v49, v50, v51, v52, v53, v54, v55, v56, v57, v58, v59.clone(), v60, v61, v9.clone());
                    (v64, v65, v66, v67, v68, v69, v70, v71, v72, v73, v74, v75, v76, v77.clone(), v78, v79, 1i32)
                } else {
                    (v46, v47, v48, v49, v50, v51, v52, v53, v54, v55, v56, v57, v58, v59.clone(), v60, v61, v62)
                };
                if v21 {
                    let mut v97: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                    let mut v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("],\"roots\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v99: Rc<str> = std::rc::Rc::<str>::from((v96).to_string());
                    let mut v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"files\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v101: Rc<str> = std::rc::Rc::<str>::from((v80).to_string());
                    let mut v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"scanned\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v103: Rc<str> = std::rc::Rc::<str>::from([&*v98, &*v99, &*v100, &*v101, &*v102].concat());
                    let mut v104: Rc<str> = std::rc::Rc::<str>::from((v81).to_string());
                    let mut v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"todos\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v106: Rc<str> = std::rc::Rc::<str>::from((v82).to_string());
                    let mut v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"skipped\":{\"nested\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v108: Rc<str> = std::rc::Rc::<str>::from((v83).to_string());
                    let mut v109: Rc<str> = std::rc::Rc::<str>::from([&*v104, &*v105, &*v106, &*v107, &*v108].concat());
                    let mut v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"vendor\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v111: Rc<str> = std::rc::Rc::<str>::from((v84).to_string());
                    let mut v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"generated\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v113: Rc<str> = std::rc::Rc::<str>::from((v86).to_string());
                    let mut v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"syntax\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v115: Rc<str> = std::rc::Rc::<str>::from([&*v110, &*v111, &*v112, &*v113, &*v114].concat());
                    let mut v116: Rc<str> = std::rc::Rc::<str>::from((v85).to_string());
                    let mut v117: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"binary\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v118: Rc<str> = std::rc::Rc::<str>::from((v87).to_string());
                    let mut v119: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"large\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v120: Rc<str> = std::rc::Rc::<str>::from((v88).to_string());
                    let mut v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"links\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v122: Rc<str> = std::rc::Rc::<str>::from((v89).to_string());
                    let mut v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"unreadable\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v124: Rc<str> = std::rc::Rc::<str>::from((v90).to_string());
                    let mut v125: Rc<str> = std::rc::Rc::<str>::from([&*v120, &*v121, &*v122, &*v123, &*v124].concat());
                    let mut v126: Rc<str> = std::rc::Rc::<str>::from([&*v116, &*v117, &*v118, &*v119, &*v125].concat());
                    let mut v127: Rc<str> = std::rc::Rc::<str>::from([&*v103, &*v109, &*v115, &*v126, &*v26].concat());
                    let mut v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("},\"elapsed_ms\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v129: Rc<str> = std::rc::Rc::<str>::from((v97).to_string());
                    let mut v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
                    let mut v131: Rc<str> = std::rc::Rc::<str>::from([&*v127, &*v128, &*v129, &*v130, &*v26].concat());
                    let mut v132: i32 = v131.len() as i32;
                    let mut v133: i32 = v132 - 1;
                    let mut v134: Rc<str> = std::rc::Rc::<str>::from(v131.get(0i32 as usize..v133 as usize).unwrap_or(""));
                    let mut v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"stale\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v136: Rc<str> = std::rc::Rc::<str>::from((v94).to_string());
                    let mut v137: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"unknown\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v138: Rc<str> = std::rc::Rc::<str>::from((v95).to_string());
                    let mut v139: Rc<str> = std::rc::Rc::<str>::from([&*v138, &*v130, &*v26, &*v26, &*v26].concat());
                    let mut v140: Rc<str> = std::rc::Rc::<str>::from([&*v134, &*v135, &*v136, &*v137, &*v139].concat());
                    let mut v141: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v140.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                    v141
                } else {
                    let mut v142: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                    let mut v143: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie agile todos ok roots="); } LIT.with(|lit| lit.clone()) };
                    let mut v144: Rc<str> = std::rc::Rc::<str>::from((v96).to_string());
                    let mut v145: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" files="); } LIT.with(|lit| lit.clone()) };
                    let mut v146: Rc<str> = std::rc::Rc::<str>::from((v80).to_string());
                    let mut v147: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" scanned="); } LIT.with(|lit| lit.clone()) };
                    let mut v148: Rc<str> = std::rc::Rc::<str>::from([&*v143, &*v144, &*v145, &*v146, &*v147].concat());
                    let mut v149: Rc<str> = std::rc::Rc::<str>::from((v81).to_string());
                    let mut v150: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" todos="); } LIT.with(|lit| lit.clone()) };
                    let mut v151: Rc<str> = std::rc::Rc::<str>::from((v82).to_string());
                    let mut v152: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" skipped="); } LIT.with(|lit| lit.clone()) };
                    let mut v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("nested:"); } LIT.with(|lit| lit.clone()) };
                    let mut v154: Rc<str> = std::rc::Rc::<str>::from((v83).to_string());
                    let mut v155: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",vendor:"); } LIT.with(|lit| lit.clone()) };
                    let mut v156: Rc<str> = std::rc::Rc::<str>::from((v84).to_string());
                    let mut v157: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",generated:"); } LIT.with(|lit| lit.clone()) };
                    let mut v158: Rc<str> = std::rc::Rc::<str>::from([&*v153, &*v154, &*v155, &*v156, &*v157].concat());
                    let mut v159: Rc<str> = std::rc::Rc::<str>::from((v86).to_string());
                    let mut v160: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",syntax:"); } LIT.with(|lit| lit.clone()) };
                    let mut v161: Rc<str> = std::rc::Rc::<str>::from((v85).to_string());
                    let mut v162: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",binary:"); } LIT.with(|lit| lit.clone()) };
                    let mut v163: Rc<str> = std::rc::Rc::<str>::from((v87).to_string());
                    let mut v164: Rc<str> = std::rc::Rc::<str>::from([&*v159, &*v160, &*v161, &*v162, &*v163].concat());
                    let mut v165: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",large:"); } LIT.with(|lit| lit.clone()) };
                    let mut v166: Rc<str> = std::rc::Rc::<str>::from((v88).to_string());
                    let mut v167: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",links:"); } LIT.with(|lit| lit.clone()) };
                    let mut v168: Rc<str> = std::rc::Rc::<str>::from((v89).to_string());
                    let mut v169: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",unreadable:"); } LIT.with(|lit| lit.clone()) };
                    let mut v170: Rc<str> = std::rc::Rc::<str>::from((v90).to_string());
                    let mut v171: Rc<str> = std::rc::Rc::<str>::from([&*v167, &*v168, &*v169, &*v170, &*v26].concat());
                    let mut v172: Rc<str> = std::rc::Rc::<str>::from([&*v158, &*v164, &*v165, &*v166, &*v171].concat());
                    let mut v173: Rc<str> = std::rc::Rc::<str>::from([&*v149, &*v150, &*v151, &*v152, &*v172].concat());
                    let mut v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                    let mut v175: Rc<str> = std::rc::Rc::<str>::from((v142).to_string());
                    let mut v176: Rc<str> = std::rc::Rc::<str>::from([&*v148, &*v173, &*v174, &*v175, &*v26].concat());
                    let mut v177: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" stale="); } LIT.with(|lit| lit.clone()) };
                    let mut v178: Rc<str> = std::rc::Rc::<str>::from((v94).to_string());
                    let mut v179: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" unknown="); } LIT.with(|lit| lit.clone()) };
                    let mut v180: Rc<str> = std::rc::Rc::<str>::from((v95).to_string());
                    let mut v181: Rc<str> = std::rc::Rc::<str>::from([&*v177, &*v178, &*v179, &*v180, &*v26].concat());
                    let mut v182: Rc<str> = std::rc::Rc::<str>::from([&*v176, &*v181, &*v26, &*v26, &*v26].concat());
                    let mut v183: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v182.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                    v183
                }
            }
            _ => {
                let mut v185: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the backlog root "); } LIT.with(|lit| lit.clone()) };
                let mut v186: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is not a directory"); } LIT.with(|lit| lit.clone()) };
                let mut v187: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v188: Rc<str> = std::rc::Rc::<str>::from([&*v185, &*v9, &*v186, &*v187, &*v187].concat());
                let mut v189: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile todos rejected: "); } LIT.with(|lit| lit.clone()) };
                let mut v190: Rc<str> = std::rc::Rc::<str>::from([&*v189, &*v188, &*v187, &*v187, &*v187].concat());
                let mut v191: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v190 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v191
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        todos_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_todos_run() -> i32 {
    closure0()()
}
