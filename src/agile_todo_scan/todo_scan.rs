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
impl US0 {
    fn tag(&self) -> i32 {
        match self {
            US0::US0_RegularFile => 0,
            US0::US0_Directory => 1,
            US0::US0_LinkedEntry => 2,
            US0::US0_AbsentEntry => 3,
        }
    }
}
impl Default for US0 {
    fn default() -> Self {
        US0::US0_RegularFile
    }
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
impl US2 {
    fn tag(&self) -> i32 {
        match self {
            US2::US2_CFamily => 0,
            US2::US2_HashFamily => 1,
            US2::US2_DashFamily => 2,
            US2::US2_HaskellFamily => 3,
            US2::US2_LeanFamily => 4,
            US2::US2_LispFamily => 5,
            US2::US2_ErlangFamily => 6,
            US2::US2_MarkupFamily => 7,
            US2::US2_MlFamily => 8,
            US2::US2_PascalFamily => 9,
            US2::US2_BatchFamily => 10,
        }
    }
}
impl Default for US2 {
    fn default() -> Self {
        US2::US2_CFamily
    }
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
impl US1 {
    fn tag(&self) -> i32 {
        match self {
            US1::US1_ScanPath(..) => 0,
            US1::US1_SkipNestedRepository => 1,
            US1::US1_SkipVendorDirectory => 2,
            US1::US1_SkipUnknownSyntax => 3,
            US1::US1_SkipGeneratedOutput => 4,
            US1::US1_SkipLinkedEntry => 5,
        }
    }
}
impl Default for US1 {
    fn default() -> Self {
        US1::US1_ScanPath(<US2>::default())
    }
}
#[derive(Clone)]
enum US3 {
    US3_ScanContent,
    US3_SkipBinary,
    US3_SkipLarge,
    US3_SkipUnreadable,
}
impl US3 {
    fn tag(&self) -> i32 {
        match self {
            US3::US3_ScanContent => 0,
            US3::US3_SkipBinary => 1,
            US3::US3_SkipLarge => 2,
            US3::US3_SkipUnreadable => 3,
        }
    }
}
impl Default for US3 {
    fn default() -> Self {
        US3::US3_ScanContent
    }
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
fn scanned_file_3(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: bool, mut v12: bool, mut v13: Rc<str>, mut v14: Rc<str>, mut v15: Rc<str>) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool) {
    let mut v16: i32 = std::fs::symlink_metadata(&*v14).map(|metadata| if metadata.file_type().is_symlink() { 3 } else if metadata.is_dir() { 2 } else if metadata.is_file() { 1 } else { 0 }).unwrap_or(0);
    let mut v17: bool = 1i32 == v16;
    let mut v26: US0 = if v17 {
        US0::US0_RegularFile
    } else {
        let mut v19: bool = 2i32 == v16;
        if v19 {
            US0::US0_Directory
        } else {
            let mut v21: bool = 3i32 == v16;
            if v21 {
                US0::US0_LinkedEntry
            } else {
                US0::US0_AbsentEntry
            }
        }
    };
    let (mut v234, mut v235, mut v236, mut v237, mut v238, mut v239, mut v240, mut v241, mut v242, mut v243, mut v244, mut v245, mut v246): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool) = match &v26 {
        US0::US0_RegularFile => {
            let mut v27: bool = std::path::Path::new(&*v14).with_extension("spi").is_file();
            let mut v28: i32 = agile_todo_paths::eoie_agile_todo_path_verdict(&*v15, v27);
            let mut v29: US1 = path_verdict_of_code_4(v28);
            let (mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = counted_path_6(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v29.clone());
            match &v29 {
                US1::US1_ScanPath(v41) => {
                    let mut v41: US2 = v41.clone();
                    let mut v42: u64 = std::fs::metadata(&*v14).map(|metadata| metadata.len()).unwrap_or(0);
                    let mut v43: bool = v42 > 1048576u64;
                    let mut v46: US3 = if v43 {
                        US3::US3_SkipLarge
                    } else {
                        US3::US3_ScanContent
                    };
                    match &v46 {
                        US3::US3_SkipLarge => {
                            let mut v47: US3 = US3::US3_SkipLarge;
                            let (mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = counted_content_7(v30, v31, v32, v33, v34, v35, v36, v37, v38, v39, v40, v47.clone());
                            (v48, v49, v50, v51, v52, v53, v54, v55, v56, v57, v58, v11, v12)
                        }
                        _ => {
                            let (mut v59, mut v60): (bool, Rc<str>) = match std::fs::read(&*v14) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
                            let mut v63: US3 = if v59 {
                                US3::US3_SkipBinary
                            } else {
                                US3::US3_ScanContent
                            };
                            let (mut v131, mut v132, mut v133, mut v134, mut v135, mut v136, mut v137, mut v138, mut v139, mut v140, mut v141, mut v142, mut v143): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool) = match &v63 {
                                US3::US3_ScanContent => {
                                    let mut v74: i32 = match &v41 {
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
                                    let (mut v75, mut v76): (Rc<str>, i32) = agile_todo_lines::eoie_agile_todo_findings(v74, &*v60, &*v13, v11);
                                    let mut v77: US3 = US3::US3_ScanContent;
                                    let (mut v78, mut v79, mut v80, mut v81, mut v82, mut v83, mut v84, mut v85, mut v86, mut v87, mut v88): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = counted_content_7(v30, v31, v32, v33, v34, v35, v36, v37, v38, v39, v40, v77.clone());
                                    let (mut v89, mut v90, mut v91, mut v92, mut v93, mut v94, mut v95, mut v96, mut v97, mut v98, mut v99): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = counted_todos_8(v78, v79, v80, v81, v82, v83, v84, v85, v86, v87, v88, v76);
                                    let mut v100: bool = v76 <= 0i32;
                                    if v100 {
                                        (v89, v90, v91, v92, v93, v94, v95, v96, v97, v98, v99, v11, v12)
                                    } else {
                                        let mut v101: bool = v11 && v12;
                                        let mut v105: Rc<str> = if v101 {
                                            let mut v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                                            let mut v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                            let mut v104: Rc<str> = std::rc::Rc::<str>::from([&*v102, &*v75, &*v103, &*v103, &*v103].concat());
                                            v104.clone()
                                        } else {
                                            v75.clone()
                                        };
                                        let mut v106: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v105.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                                        (v89, v90, v91, v92, v93, v94, v95, v96, v97, v98, v99, v11, true)
                                    }
                                }
                                _ => {
                                    let (mut v120, mut v121, mut v122, mut v123, mut v124, mut v125, mut v126, mut v127, mut v128, mut v129, mut v130): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = counted_content_7(v30, v31, v32, v33, v34, v35, v36, v37, v38, v39, v40, v63.clone());
                                    (v120, v121, v122, v123, v124, v125, v126, v127, v128, v129, v130, v11, v12)
                                }
                            };
                            (v131, v132, v133, v134, v135, v136, v137, v138, v139, v140, v141, v142, v143)
                        }
                    }
                }
                _ => {
                    (v30, v31, v32, v33, v34, v35, v36, v37, v38, v39, v40, v11, v12)
                }
            }
        }
        _ => {
            let (mut v221, mut v222, mut v223, mut v224, mut v225, mut v226, mut v227, mut v228, mut v229, mut v230, mut v231, mut v232, mut v233): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool) = match &v26 {
                US0::US0_LinkedEntry => {
                    let mut v170: US1 = US1::US1_SkipLinkedEntry;
                    let (mut v171, mut v172, mut v173, mut v174, mut v175, mut v176, mut v177, mut v178, mut v179, mut v180, mut v181): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = counted_path_6(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v170.clone());
                    (v171, v172, v173, v174, v175, v176, v177, v178, v179, v180, v181, v11, v12)
                }
                _ => {
                    let mut v182: i32 = agile_todo_paths::eoie_agile_todo_path_verdict(&*v15, false);
                    let mut v183: US1 = path_verdict_of_code_4(v182);
                    let (mut v184, mut v185, mut v186, mut v187, mut v188, mut v189, mut v190, mut v191, mut v192, mut v193, mut v194): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = counted_path_6(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v183.clone());
                    let (mut v208, mut v209, mut v210, mut v211, mut v212, mut v213, mut v214, mut v215, mut v216, mut v217, mut v218, mut v219, mut v220): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool) = match &v183 {
                        US1::US1_ScanPath(v195) => {
                            let mut v195: US2 = v195.clone();
                            let mut v196: US3 = US3::US3_SkipUnreadable;
                            let (mut v197, mut v198, mut v199, mut v200, mut v201, mut v202, mut v203, mut v204, mut v205, mut v206, mut v207): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = counted_content_7(v184, v185, v186, v187, v188, v189, v190, v191, v192, v193, v194, v196.clone());
                            (v197, v198, v199, v200, v201, v202, v203, v204, v205, v206, v207, v11, v12)
                        }
                        _ => {
                            (v184, v185, v186, v187, v188, v189, v190, v191, v192, v193, v194, v11, v12)
                        }
                    };
                    (v208, v209, v210, v211, v212, v213, v214, v215, v216, v217, v218, v219, v220)
                }
            };
            (v221, v222, v223, v224, v225, v226, v227, v228, v229, v230, v231, v232, v233)
        }
    };
    (v234, v235, v236, v237, v238, v239, v240, v241, v242, v243, v244, v245, v246)
}
fn listed_files_9(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: bool, mut v12: bool, mut v13: Rc<str>, mut v14: Rc<str>, mut v15: Rc<str>, mut v16: i32) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool) {
    let mut v17: i32 = v15.len() as i32;
    let mut v18: bool = v17 <= v16;
    let (mut v98, mut v99, mut v100, mut v101, mut v102, mut v103, mut v104, mut v105, mut v106, mut v107, mut v108, mut v109, mut v110): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool) = if v18 {
        (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12)
    } else {
        let mut v19: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
        let mut v20: i32 = v15.get(v16 as usize..).and_then(|rest| rest.find(&*v19)).map(|found| (found + v16 as usize) as i32).unwrap_or(-1);
        let mut v21: bool = v20 < 0;
        let mut v23: i32 = if v21 {
            let mut v22: i32 = v15.len() as i32;
            v22
        } else {
            v20
        };
        let mut v24: Rc<str> = std::rc::Rc::<str>::from(v15.get(v16 as usize..v23 as usize).unwrap_or(""));
        let mut v25: i32 = v23 + 1;
        let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v27: bool = v24 == v26 ;
        let (mut v85, mut v86, mut v87, mut v88, mut v89, mut v90, mut v91, mut v92, mut v93, mut v94, mut v95, mut v96, mut v97): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool) = if v27 {
            let (mut v28, mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool) = listed_files_9(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13.clone(), v14.clone(), v15.clone(), v25);
            (v28, v29, v30, v31, v32, v33, v34, v35, v36, v37, v38, v39, v40)
        } else {
            let mut v41: bool = v24 == v26 ;
            let mut v49: Rc<str> = if v41 {
                v14.clone()
            } else {
                let mut v42: bool = v14 == v26 ;
                if v42 {
                    v24.clone()
                } else {
                    let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                    let mut v44: bool = v14.ends_with(&*v43);
                    if v44 {
                        let mut v45: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v24, &*v26, &*v26, &*v26].concat());
                        v45.clone()
                    } else {
                        let mut v46: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v43, &*v24, &*v26, &*v26].concat());
                        v46.clone()
                    }
                }
            };
            let mut v50: bool = v24 == v26 ;
            let mut v58: Rc<str> = if v50 {
                v13.clone()
            } else {
                let mut v51: bool = v13 == v26 ;
                if v51 {
                    v24.clone()
                } else {
                    let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                    let mut v53: bool = v13.ends_with(&*v52);
                    if v53 {
                        let mut v54: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v24, &*v26, &*v26, &*v26].concat());
                        v54.clone()
                    } else {
                        let mut v55: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v52, &*v24, &*v26, &*v26].concat());
                        v55.clone()
                    }
                }
            };
            let (mut v59, mut v60, mut v61, mut v62, mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool) = scanned_file_3(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v49.clone(), v58.clone(), v24.clone());
            let (mut v72, mut v73, mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83, mut v84): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool) = listed_files_9(v59, v60, v61, v62, v63, v64, v65, v66, v67, v68, v69, v70, v71, v13.clone(), v14.clone(), v15.clone(), v25);
            (v72, v73, v74, v75, v76, v77, v78, v79, v80, v81, v82, v83, v84)
        };
        (v85, v86, v87, v88, v89, v90, v91, v92, v93, v94, v95, v96, v97)
    };
    (v98, v99, v100, v101, v102, v103, v104, v105, v106, v107, v108, v109, v110)
}
fn scanned_root_2(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: bool, mut v12: bool, mut v13: Rc<str>) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool) {
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = std::rc::Rc::<str>::from(v13.replace(&*v14, &v15));
    let mut v17: i32 = std::fs::symlink_metadata(&*v13).map(|metadata| if metadata.file_type().is_symlink() { 3 } else if metadata.is_dir() { 2 } else if metadata.is_file() { 1 } else { 0 }).unwrap_or(0);
    let mut v18: bool = 1i32 == v17;
    let mut v27: US0 = if v18 {
        US0::US0_RegularFile
    } else {
        let mut v20: bool = 2i32 == v17;
        if v20 {
            US0::US0_Directory
        } else {
            let mut v22: bool = 3i32 == v17;
            if v22 {
                US0::US0_LinkedEntry
            } else {
                US0::US0_AbsentEntry
            }
        }
    };
    let (mut v84, mut v85, mut v86, mut v87, mut v88, mut v89, mut v90, mut v91, mut v92, mut v93, mut v94, mut v95, mut v96): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool) = match &v27 {
        US0::US0_RegularFile => {
            let mut v28: Rc<str> = std::rc::Rc::<str>::from(v13.replace(&*v14, &v15));
            scanned_file_3(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v16.clone(), v13.clone(), v28.clone())
        }
        _ => {
            let (mut v71, mut v72, mut v73, mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool) = match &v27 {
                US0::US0_Directory => {
                    let (mut v42, mut v43): (bool, Rc<str>) = agile_todo_sources::eoie_agile_todo_sources(&*v13);
                    if v42 {
                        let mut v44: i32 = 0i32;
                        let (mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool) = listed_files_9(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13.clone(), v16.clone(), v43.clone(), v44);
                        (v45, v46, v47, v48, v49, v50, v51, v52, v53, v54, v55, v56, v57)
                    } else {
                        (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12)
                    }
                }
                _ => {
                    (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12)
                }
            };
            (v71, v72, v73, v74, v75, v76, v77, v78, v79, v80, v81, v82, v83)
        }
    };
    (v84, v85, v86, v87, v88, v89, v90, v91, v92, v93, v94, v95, v96)
}
fn scanned_arguments_1(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: bool, mut v12: bool, mut v13: i32, mut v14: i32, mut v15: i32) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, i32) {
    loop {
        let mut v16: bool = v14 <= v13;
        if v16 {
            return (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v15);
        } else {
            let mut v17: Rc<str> = usize::try_from(v13).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--json"); } LIT.with(|lit| lit.clone()) };
            let mut v19: bool = v17 == v18 ;
            if v19 {
                let mut v20: i32 = v13 + 1;
                (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15) = (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v20, v14, v15);
                continue;
            } else {
                let (mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool) = scanned_root_2(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v17.clone());
                let mut v48: i32 = v13 + 1;
                let mut v49: i32 = v15 + 1;
                (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15) = (v35, v36, v37, v38, v39, v40, v41, v42, v43, v44, v45, v46, v47, v48, v14, v49);
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
                let mut v25: i32 = 0i32;
                let mut v26: i32 = 0i32;
                let mut v27: i32 = 0i32;
                let mut v28: i32 = 0i32;
                let mut v29: i32 = 0i32;
                let mut v30: i32 = 0i32;
                let mut v31: i32 = 0i32;
                let mut v32: i32 = 0i32;
                let mut v33: i32 = 0i32;
                let mut v34: i32 = 0i32;
                let mut v35: i32 = 0i32;
                let mut v36: bool = false;
                let mut v37: i32 = 2i32;
                let mut v38: i32 = 0i32;
                let (mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, i32) = scanned_arguments_1(v25, v26, v27, v28, v29, v30, v31, v32, v33, v34, v35, v21, v36, v37, v1, v38);
                let mut v53: bool = v52 == 0i32;
                let (mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73, mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, i32) = if v53 {
                    let (mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64, mut v65, mut v66): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool) = scanned_root_2(v39, v40, v41, v42, v43, v44, v45, v46, v47, v48, v49, v50, v51, v9.clone());
                    (v54, v55, v56, v57, v58, v59, v60, v61, v62, v63, v64, v65, v66, 1i32)
                } else {
                    (v39, v40, v41, v42, v43, v44, v45, v46, v47, v48, v49, v50, v51, v52)
                };
                if v21 {
                    let mut v81: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                    let mut v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("],\"roots\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v83: Rc<str> = std::rc::Rc::<str>::from((v80).to_string());
                    let mut v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"files\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v85: Rc<str> = std::rc::Rc::<str>::from((v67).to_string());
                    let mut v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"scanned\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v87: Rc<str> = std::rc::Rc::<str>::from([&*v82, &*v83, &*v84, &*v85, &*v86].concat());
                    let mut v88: Rc<str> = std::rc::Rc::<str>::from((v68).to_string());
                    let mut v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"todos\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v90: Rc<str> = std::rc::Rc::<str>::from((v69).to_string());
                    let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"skipped\":{\"nested\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v92: Rc<str> = std::rc::Rc::<str>::from((v70).to_string());
                    let mut v93: Rc<str> = std::rc::Rc::<str>::from([&*v88, &*v89, &*v90, &*v91, &*v92].concat());
                    let mut v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"vendor\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v95: Rc<str> = std::rc::Rc::<str>::from((v71).to_string());
                    let mut v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"generated\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v97: Rc<str> = std::rc::Rc::<str>::from((v73).to_string());
                    let mut v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"syntax\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v99: Rc<str> = std::rc::Rc::<str>::from([&*v94, &*v95, &*v96, &*v97, &*v98].concat());
                    let mut v100: Rc<str> = std::rc::Rc::<str>::from((v72).to_string());
                    let mut v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"binary\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v102: Rc<str> = std::rc::Rc::<str>::from((v74).to_string());
                    let mut v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"large\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v104: Rc<str> = std::rc::Rc::<str>::from((v75).to_string());
                    let mut v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"links\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v106: Rc<str> = std::rc::Rc::<str>::from((v76).to_string());
                    let mut v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"unreadable\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v108: Rc<str> = std::rc::Rc::<str>::from((v77).to_string());
                    let mut v109: Rc<str> = std::rc::Rc::<str>::from([&*v104, &*v105, &*v106, &*v107, &*v108].concat());
                    let mut v110: Rc<str> = std::rc::Rc::<str>::from([&*v100, &*v101, &*v102, &*v103, &*v109].concat());
                    let mut v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v112: Rc<str> = std::rc::Rc::<str>::from([&*v87, &*v93, &*v99, &*v110, &*v111].concat());
                    let mut v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("},\"elapsed_ms\":"); } LIT.with(|lit| lit.clone()) };
                    let mut v114: Rc<str> = std::rc::Rc::<str>::from((v81).to_string());
                    let mut v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
                    let mut v116: Rc<str> = std::rc::Rc::<str>::from([&*v112, &*v113, &*v114, &*v115, &*v111].concat());
                    let mut v117: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v116.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                    v117
                } else {
                    let mut v118: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                    let mut v119: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie agile todos ok roots="); } LIT.with(|lit| lit.clone()) };
                    let mut v120: Rc<str> = std::rc::Rc::<str>::from((v80).to_string());
                    let mut v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" files="); } LIT.with(|lit| lit.clone()) };
                    let mut v122: Rc<str> = std::rc::Rc::<str>::from((v67).to_string());
                    let mut v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" scanned="); } LIT.with(|lit| lit.clone()) };
                    let mut v124: Rc<str> = std::rc::Rc::<str>::from([&*v119, &*v120, &*v121, &*v122, &*v123].concat());
                    let mut v125: Rc<str> = std::rc::Rc::<str>::from((v68).to_string());
                    let mut v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" todos="); } LIT.with(|lit| lit.clone()) };
                    let mut v127: Rc<str> = std::rc::Rc::<str>::from((v69).to_string());
                    let mut v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" skipped="); } LIT.with(|lit| lit.clone()) };
                    let mut v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("nested:"); } LIT.with(|lit| lit.clone()) };
                    let mut v130: Rc<str> = std::rc::Rc::<str>::from((v70).to_string());
                    let mut v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",vendor:"); } LIT.with(|lit| lit.clone()) };
                    let mut v132: Rc<str> = std::rc::Rc::<str>::from((v71).to_string());
                    let mut v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",generated:"); } LIT.with(|lit| lit.clone()) };
                    let mut v134: Rc<str> = std::rc::Rc::<str>::from([&*v129, &*v130, &*v131, &*v132, &*v133].concat());
                    let mut v135: Rc<str> = std::rc::Rc::<str>::from((v73).to_string());
                    let mut v136: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",syntax:"); } LIT.with(|lit| lit.clone()) };
                    let mut v137: Rc<str> = std::rc::Rc::<str>::from((v72).to_string());
                    let mut v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",binary:"); } LIT.with(|lit| lit.clone()) };
                    let mut v139: Rc<str> = std::rc::Rc::<str>::from((v74).to_string());
                    let mut v140: Rc<str> = std::rc::Rc::<str>::from([&*v135, &*v136, &*v137, &*v138, &*v139].concat());
                    let mut v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",large:"); } LIT.with(|lit| lit.clone()) };
                    let mut v142: Rc<str> = std::rc::Rc::<str>::from((v75).to_string());
                    let mut v143: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",links:"); } LIT.with(|lit| lit.clone()) };
                    let mut v144: Rc<str> = std::rc::Rc::<str>::from((v76).to_string());
                    let mut v145: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",unreadable:"); } LIT.with(|lit| lit.clone()) };
                    let mut v146: Rc<str> = std::rc::Rc::<str>::from((v77).to_string());
                    let mut v147: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v148: Rc<str> = std::rc::Rc::<str>::from([&*v143, &*v144, &*v145, &*v146, &*v147].concat());
                    let mut v149: Rc<str> = std::rc::Rc::<str>::from([&*v134, &*v140, &*v141, &*v142, &*v148].concat());
                    let mut v150: Rc<str> = std::rc::Rc::<str>::from([&*v125, &*v126, &*v127, &*v128, &*v149].concat());
                    let mut v151: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                    let mut v152: Rc<str> = std::rc::Rc::<str>::from((v118).to_string());
                    let mut v153: Rc<str> = std::rc::Rc::<str>::from([&*v124, &*v150, &*v151, &*v152, &*v147].concat());
                    let mut v154: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v153.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                    v154
                }
            }
            _ => {
                let mut v156: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the backlog root "); } LIT.with(|lit| lit.clone()) };
                let mut v157: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is not a directory"); } LIT.with(|lit| lit.clone()) };
                let mut v158: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v159: Rc<str> = std::rc::Rc::<str>::from([&*v156, &*v9, &*v157, &*v158, &*v158].concat());
                let mut v160: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile todos rejected: "); } LIT.with(|lit| lit.clone()) };
                let mut v161: Rc<str> = std::rc::Rc::<str>::from([&*v160, &*v159, &*v158, &*v158, &*v158].concat());
                let mut v162: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v161 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v162
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
