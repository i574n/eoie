#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_CFamily,
    US0_HashFamily,
    US0_DashFamily,
    US0_HaskellFamily,
    US0_LeanFamily,
    US0_LispFamily,
    US0_ErlangFamily,
    US0_MarkupFamily,
    US0_MlFamily,
    US0_PascalFamily,
    US0_BatchFamily,
}
#[derive(Clone)]
enum US1 {
    US1_NoTodo,
    US1_TodoComment(Rc<str>),
}
fn visible_end_3(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return 0i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: i32 = v0.as_bytes().get(v3 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v5: bool = v4 == 32i32;
            if v5 {
                (v0, v1) = (v0.clone(), v3);
                continue;
            } else {
                let mut v7: i32 = v0.as_bytes().get(v3 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let mut v8: bool = v7 == 9i32;
                if v8 {
                    (v0, v1) = (v0.clone(), v3);
                    continue;
                } else {
                    let mut v10: i32 = v0.as_bytes().get(v3 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                    let mut v11: bool = v10 == 13i32;
                    if v11 {
                        (v0, v1) = (v0.clone(), v3);
                        continue;
                    } else {
                        return v1;
                    }
                }
            }
        }
    }
}
fn leader_before_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> Rc<str> {
    loop {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
        let mut v4: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v3)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
        let mut v5: bool = v4 < 0i32;
        let mut v7: i32 = if v5 {
            let mut v6: i32 = v0.len() as i32;
            v6
        } else {
            v4
        };
        let mut v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v7 as usize).unwrap_or(""));
        let mut v9: bool = v1.ends_with(&*v8);
        if v9 {
            return v8.clone();
        } else {
            if v5 {
                let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v10.clone();
            } else {
                let mut v11: i32 = v4.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1.clone(), v11);
                continue;
            }
        }
    }
}
fn first_visible_offset_5(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: i32 = v0.len() as i32;
        let mut v3: bool = v2 <= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v5: bool = v4 == 32i32;
            if v5 {
                let mut v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v0.clone(), v6);
                continue;
            } else {
                let mut v8: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let mut v9: bool = v8 == 9i32;
                if v9 {
                    let mut v10: i32 = v1.wrapping_add(1i32);
                    (v0, v1) = (v0.clone(), v10);
                    continue;
                } else {
                    return v1;
                }
            }
        }
    }
}
fn leader_run_start_6(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v1 <= 0i32;
        if v3 {
            return 0i32;
        } else {
            let mut v4: i32 = v1.wrapping_sub(1i32);
            let mut v5: i32 = v0.as_bytes().get(v4 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v6: bool = v5 == v2;
            if v6 {
                (v0, v1, v2) = (v0.clone(), v4, v2);
                continue;
            } else {
                return v1;
            }
        }
    }
}
fn count_of_byte_7(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32) -> i32 {
    loop {
        let mut v5: bool = v3 <= v2;
        if v5 {
            return v4;
        } else {
            let mut v6: i32 = v0.as_bytes().get(v2 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v7: bool = v6 == v1;
            if v7 {
                let mut v8: i32 = v2.wrapping_add(1i32);
                let mut v9: i32 = v4.wrapping_add(1i32);
                (v0, v1, v2, v3, v4) = (v0.clone(), v1, v8, v3, v9);
                continue;
            } else {
                let mut v11: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2, v3, v4) = (v0.clone(), v1, v11, v3, v4);
                continue;
            }
        }
    }
}
fn todo_in_line_2(mut v0: US0, mut v1: Rc<str>) -> US1 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TO"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("DO"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v3, &*v4, &*v4, &*v4].concat());
    let mut v6: i32 = v1.find(&*v5).map(|offset| offset as i32).unwrap_or(-1);
    let mut v7: bool = v6 < 0i32;
    if v7 {
        US1::US1_NoTodo
    } else {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///>"); } LIT.with(|lit| lit.clone()) };
        let mut v10: bool = v1.starts_with(&*v9);
        let mut v13: bool = if v10 {
            true
        } else {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///-"); } LIT.with(|lit| lit.clone()) };
            let mut v12: bool = v1.starts_with(&*v11);
            v12
        };
        if v13 {
            US1::US1_NoTodo
        } else {
            let mut v15: i32 = v6 + 4i32;
            let mut v16: i32 = v1.as_bytes().get(v15 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v17: bool = 65i32 <= v16;
            let mut v18: bool = v16 <= 90i32;
            let mut v19: bool = v17 && v18;
            let mut v29: bool = if v19 {
                true
            } else {
                let mut v20: bool = 97i32 <= v16;
                let mut v21: bool = v16 <= 122i32;
                let mut v22: bool = v20 && v21;
                if v22 {
                    true
                } else {
                    let mut v23: bool = 48i32 <= v16;
                    let mut v24: bool = v16 <= 57i32;
                    let mut v25: bool = v23 && v24;
                    if v25 {
                        true
                    } else {
                        let mut v26: bool = v16 == 95i32;
                        v26
                    }
                }
            };
            if v29 {
                US1::US1_NoTodo
            } else {
                let mut v31: i32 = visible_end_3(v1.clone(), v6);
                let mut v32: Rc<str> = std::rc::Rc::<str>::from(v1.get(0i32 as usize..v31 as usize).unwrap_or(""));
                let mut v54: Rc<str> = match &v0 {
                    US0::US0_BatchFamily => {
                        let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("::|REM|rem"); } LIT.with(|lit| lit.clone()) };
                        v43.clone()
                    }
                    US0::US0_CFamily => {
                        let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("//|/*"); } LIT.with(|lit| lit.clone()) };
                        v33.clone()
                    }
                    US0::US0_DashFamily => {
                        let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
                        v35.clone()
                    }
                    US0::US0_ErlangFamily => {
                        let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("%"); } LIT.with(|lit| lit.clone()) };
                        v39.clone()
                    }
                    US0::US0_HashFamily => {
                        let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("#"); } LIT.with(|lit| lit.clone()) };
                        v34.clone()
                    }
                    US0::US0_HaskellFamily => {
                        let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--|{-"); } LIT.with(|lit| lit.clone()) };
                        v36.clone()
                    }
                    US0::US0_LeanFamily => {
                        let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--|/-"); } LIT.with(|lit| lit.clone()) };
                        v37.clone()
                    }
                    US0::US0_LispFamily => {
                        let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(";"); } LIT.with(|lit| lit.clone()) };
                        v38.clone()
                    }
                    US0::US0_MarkupFamily => {
                        let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<!--"); } LIT.with(|lit| lit.clone()) };
                        v40.clone()
                    }
                    US0::US0_MlFamily => {
                        let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(*"); } LIT.with(|lit| lit.clone()) };
                        v41.clone()
                    }
                    US0::US0_PascalFamily => {
                        let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("//|(*|{"); } LIT.with(|lit| lit.clone()) };
                        v42.clone()
                    }
                };
                let mut v55: i32 = 0i32;
                let mut v56: Rc<str> = leader_before_4(v54.clone(), v32.clone(), v55);
                let mut v60: Rc<str> = match &v0 {
                    US0::US0_CFamily => {
                        let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*"); } LIT.with(|lit| lit.clone()) };
                        v57.clone()
                    }
                    US0::US0_PascalFamily => {
                        let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{"); } LIT.with(|lit| lit.clone()) };
                        v58.clone()
                    }
                    _ => {
                        v4.clone()
                    }
                };
                let mut v61: i32 = v32.len() as i32;
                let mut v62: i32 = v56.len() as i32;
                let mut v63: i32 = v61 - v62;
                let mut v64: i32 = 0i32;
                let mut v65: i32 = first_visible_offset_5(v1.clone(), v64);
                let mut v66: bool = v56 == v4 ;
                if v66 {
                    let mut v67: bool = v60 == v4 ;
                    let mut v68: bool = v67 == false;
                    let mut v69: bool = v32.ends_with(&*v60);
                    let mut v70: i32 = v32.len() as i32;
                    let mut v71: i32 = v60.len() as i32;
                    let mut v72: i32 = v70 - v71;
                    let mut v73: bool = v72 == v65;
                    let mut v74: bool = v69 && v73;
                    let mut v75: bool = v68 && v74;
                    if v75 {
                        let mut v76: i32 = v1.len() as i32;
                        let mut v77: Rc<str> = std::rc::Rc::<str>::from(v1.get(v65 as usize..v76 as usize).unwrap_or(""));
                        let mut v78: Rc<str> = std::rc::Rc::<str>::from(v77.trim());
                        let mut v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-->"); } LIT.with(|lit| lit.clone()) };
                        let mut v80: bool = v78.ends_with(&*v79);
                        let mut v102: Rc<str> = if v80 {
                            let mut v81: i32 = v78.len() as i32;
                            let mut v82: i32 = v81 - 3i32;
                            let mut v83: Rc<str> = std::rc::Rc::<str>::from(v78.get(0i32 as usize..v82 as usize).unwrap_or(""));
                            v83.clone()
                        } else {
                            let mut v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*/"); } LIT.with(|lit| lit.clone()) };
                            let mut v85: bool = v78.ends_with(&*v84);
                            if v85 {
                                let mut v86: i32 = v78.len() as i32;
                                let mut v87: i32 = v86 - 2i32;
                                let mut v88: Rc<str> = std::rc::Rc::<str>::from(v78.get(0i32 as usize..v87 as usize).unwrap_or(""));
                                v88.clone()
                            } else {
                                let mut v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*)"); } LIT.with(|lit| lit.clone()) };
                                let mut v90: bool = v78.ends_with(&*v89);
                                if v90 {
                                    let mut v91: i32 = v78.len() as i32;
                                    let mut v92: i32 = v91 - 2i32;
                                    let mut v93: Rc<str> = std::rc::Rc::<str>::from(v78.get(0i32 as usize..v92 as usize).unwrap_or(""));
                                    v93.clone()
                                } else {
                                    let mut v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-}"); } LIT.with(|lit| lit.clone()) };
                                    let mut v95: bool = v78.ends_with(&*v94);
                                    if v95 {
                                        let mut v96: i32 = v78.len() as i32;
                                        let mut v97: i32 = v96 - 2i32;
                                        let mut v98: Rc<str> = std::rc::Rc::<str>::from(v78.get(0i32 as usize..v97 as usize).unwrap_or(""));
                                        v98.clone()
                                    } else {
                                        v78.clone()
                                    }
                                }
                            }
                        };
                        let mut v103: Rc<str> = std::rc::Rc::<str>::from(v102.trim());
                        US1::US1_TodoComment(v103.clone())
                    } else {
                        US1::US1_NoTodo
                    }
                } else {
                    let mut v107: i32 = v56.as_bytes().get(0i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                    let mut v108: i32 = leader_run_start_6(v1.clone(), v63, v107);
                    let mut v109: i32 = 34i32;
                    let mut v110: i32 = 0i32;
                    let mut v111: i32 = 0i32;
                    let mut v112: i32 = count_of_byte_7(v1.clone(), v109, v110, v108, v111);
                    let mut v113: i32 = v112 % 2i32;
                    let mut v114: bool = v113 == 0i32;
                    let mut v117: bool = match &v0 {
                        US0::US0_DashFamily => {
                            true
                        }
                        US0::US0_ErlangFamily => {
                            true
                        }
                        US0::US0_HashFamily => {
                            true
                        }
                        _ => {
                            false
                        }
                    };
                    let mut v125: bool = if v117 {
                        let mut v118: i32 = 39i32;
                        let mut v119: i32 = 0i32;
                        let mut v120: i32 = 0i32;
                        let mut v121: i32 = count_of_byte_7(v1.clone(), v118, v119, v108, v120);
                        let mut v122: i32 = v121 % 2i32;
                        let mut v123: bool = v122 == 0i32;
                        let mut v124: bool = v114 && v123;
                        v124
                    } else {
                        v114
                    };
                    if v125 {
                        let mut v126: i32 = v1.len() as i32;
                        let mut v127: Rc<str> = std::rc::Rc::<str>::from(v1.get(v108 as usize..v126 as usize).unwrap_or(""));
                        let mut v128: Rc<str> = std::rc::Rc::<str>::from(v127.trim());
                        let mut v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-->"); } LIT.with(|lit| lit.clone()) };
                        let mut v130: bool = v128.ends_with(&*v129);
                        let mut v152: Rc<str> = if v130 {
                            let mut v131: i32 = v128.len() as i32;
                            let mut v132: i32 = v131 - 3i32;
                            let mut v133: Rc<str> = std::rc::Rc::<str>::from(v128.get(0i32 as usize..v132 as usize).unwrap_or(""));
                            v133.clone()
                        } else {
                            let mut v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*/"); } LIT.with(|lit| lit.clone()) };
                            let mut v135: bool = v128.ends_with(&*v134);
                            if v135 {
                                let mut v136: i32 = v128.len() as i32;
                                let mut v137: i32 = v136 - 2i32;
                                let mut v138: Rc<str> = std::rc::Rc::<str>::from(v128.get(0i32 as usize..v137 as usize).unwrap_or(""));
                                v138.clone()
                            } else {
                                let mut v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*)"); } LIT.with(|lit| lit.clone()) };
                                let mut v140: bool = v128.ends_with(&*v139);
                                if v140 {
                                    let mut v141: i32 = v128.len() as i32;
                                    let mut v142: i32 = v141 - 2i32;
                                    let mut v143: Rc<str> = std::rc::Rc::<str>::from(v128.get(0i32 as usize..v142 as usize).unwrap_or(""));
                                    v143.clone()
                                } else {
                                    let mut v144: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-}"); } LIT.with(|lit| lit.clone()) };
                                    let mut v145: bool = v128.ends_with(&*v144);
                                    if v145 {
                                        let mut v146: i32 = v128.len() as i32;
                                        let mut v147: i32 = v146 - 2i32;
                                        let mut v148: Rc<str> = std::rc::Rc::<str>::from(v128.get(0i32 as usize..v147 as usize).unwrap_or(""));
                                        v148.clone()
                                    } else {
                                        v128.clone()
                                    }
                                }
                            }
                        };
                        let mut v153: Rc<str> = std::rc::Rc::<str>::from(v152.trim());
                        US1::US1_TodoComment(v153.clone())
                    } else {
                        US1::US1_NoTodo
                    }
                }
            }
        }
    }
}
fn leader_prefix_9(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
        let mut v4: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v3)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
        let mut v5: bool = v4 < 0i32;
        let mut v7: i32 = if v5 {
            let mut v6: i32 = v0.len() as i32;
            v6
        } else {
            v4
        };
        let mut v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v7 as usize).unwrap_or(""));
        let mut v9: bool = v1.starts_with(&*v8);
        if v9 {
            return true;
        } else {
            if v5 {
                return false;
            } else {
                let mut v10: i32 = v4.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1.clone(), v10);
                continue;
            }
        }
    }
}
fn comment_starts_line_8(mut v0: US0, mut v1: Rc<str>) -> bool {
    let mut v2: i32 = 0i32;
    let mut v3: i32 = first_visible_offset_5(v1.clone(), v2);
    let mut v25: Rc<str> = match &v0 {
        US0::US0_BatchFamily => {
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("::|REM|rem"); } LIT.with(|lit| lit.clone()) };
            v14.clone()
        }
        US0::US0_CFamily => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("//|/*"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US0::US0_DashFamily => {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        }
        US0::US0_ErlangFamily => {
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("%"); } LIT.with(|lit| lit.clone()) };
            v10.clone()
        }
        US0::US0_HashFamily => {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("#"); } LIT.with(|lit| lit.clone()) };
            v5.clone()
        }
        US0::US0_HaskellFamily => {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--|{-"); } LIT.with(|lit| lit.clone()) };
            v7.clone()
        }
        US0::US0_LeanFamily => {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--|/-"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US0::US0_LispFamily => {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(";"); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        }
        US0::US0_MarkupFamily => {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<!--"); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US0::US0_MlFamily => {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(*"); } LIT.with(|lit| lit.clone()) };
            v12.clone()
        }
        US0::US0_PascalFamily => {
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("//|(*|{"); } LIT.with(|lit| lit.clone()) };
            v13.clone()
        }
    };
    let mut v26: i32 = v1.len() as i32;
    let mut v27: Rc<str> = std::rc::Rc::<str>::from(v1.get(v3 as usize..v26 as usize).unwrap_or(""));
    let mut v28: i32 = 0i32;
    leader_prefix_9(v25.clone(), v27.clone(), v28)
}
fn todo_free_11(mut v0: US0, mut v1: Rc<str>) -> bool {
    let mut v2: US1 = todo_in_line_2(v0.clone(), v1.clone());
    match &v2 {
        US1::US1_NoTodo => {
            true
        }
        US1::US1_TodoComment(v3) => {
            let mut v3: Rc<str> = v3.clone();
            false
        }
    }
}
fn block_from_10(mut v0: US0, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: Rc<str>, mut v5: i32) -> (Rc<str>, i32, i32) {
    loop {
        let mut v6: i32 = v1.len() as i32;
        let mut v7: bool = v6 <= v3;
        if v7 {
            return (v4.clone(), v3, v5);
        } else {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v9: i32 = v1.get(v3 as usize..).and_then(|rest| rest.find(&*v8)).map(|found| (found + v3 as usize) as i32).unwrap_or(-1);
            let mut v10: bool = v9 < 0i32;
            let mut v12: i32 = if v10 {
                let mut v11: i32 = v1.len() as i32;
                v11
            } else {
                v9
            };
            let mut v13: Rc<str> = std::rc::Rc::<str>::from(v1.get(v3 as usize..v12 as usize).unwrap_or(""));
            let mut v14: Rc<str> = std::rc::Rc::<str>::from(v13.trim_end());
            let mut v15: i32 = 0i32;
            let mut v16: i32 = first_visible_offset_5(v14.clone(), v15);
            let mut v17: bool = v16 == v2;
            let mut v18: i32 = v14.len() as i32;
            let mut v19: Rc<str> = std::rc::Rc::<str>::from(v14.get(v2 as usize..v18 as usize).unwrap_or(""));
            let mut v20: bool = comment_starts_line_8(v0.clone(), v14.clone());
            let mut v21: bool = v17 && v20;
            let mut v22: bool = todo_free_11(v0.clone(), v14.clone());
            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///>"); } LIT.with(|lit| lit.clone()) };
            let mut v24: bool = v19.starts_with(&*v23);
            let mut v27: bool = if v24 {
                true
            } else {
                let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///-"); } LIT.with(|lit| lit.clone()) };
                let mut v26: bool = v19.starts_with(&*v25);
                v26
            };
            let mut v28: bool = v27 == false;
            let mut v29: bool = v22 && v28;
            let mut v30: bool = v21 && v29;
            if v30 {
                let mut v31: i32 = v12.wrapping_add(1i32);
                let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v33: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v8, &*v19, &*v32, &*v32].concat());
                let mut v34: i32 = v5.wrapping_add(1i32);
                (v0, v1, v2, v3, v4, v5) = (v0.clone(), v1.clone(), v2, v31, v33.clone(), v34);
                continue;
            } else {
                return (v4.clone(), v3, v5);
            }
        }
    }
}
fn id_end_14(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: i32 = v0.len() as i32;
        let mut v3: bool = v2 <= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v5: bool = 65i32 <= v4;
            let mut v11: bool = if v5 {
                let mut v6: bool = v4 <= 90i32;
                v6
            } else {
                let mut v7: bool = 48i32 <= v4;
                if v7 {
                    let mut v8: bool = v4 <= 57i32;
                    v8
                } else {
                    let mut v9: bool = v4 == 45i32;
                    v9
                }
            };
            if v11 {
                let mut v12: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v0.clone(), v12);
                continue;
            } else {
                return v1;
            }
        }
    }
}
fn todo_id_13(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TO"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("DO"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v2, &*v3, &*v3, &*v3].concat());
    let mut v5: i32 = v0.find(&*v4).map(|offset| offset as i32).unwrap_or(-1);
    let mut v6: i32 = v5 + 4i32;
    let mut v7: bool = v5 < 0i32;
    if v7 {
        v3.clone()
    } else {
        let mut v8: i32 = v0.as_bytes().get(v6 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
        let mut v9: bool = v8 == 58i32;
        if v9 {
            let mut v10: i32 = v6.wrapping_add(1i32);
            let mut v11: i32 = first_visible_offset_5(v0.clone(), v10);
            let mut v12: i32 = id_end_14(v0.clone(), v11);
            let mut v13: Rc<str> = std::rc::Rc::<str>::from(v0.get(v11 as usize..v12 as usize).unwrap_or(""));
            let mut v14: i32 = v0.len() as i32;
            let mut v15: bool = v14 <= v12;
            let mut v18: bool = if v15 {
                true
            } else {
                let mut v16: i32 = v0.as_bytes().get(v12 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let mut v17: bool = v16 == 32i32;
                v17
            };
            let mut v19: bool = v12 <= v11;
            let mut v25: bool = if v19 {
                false
            } else {
                let mut v20: i32 = v0.as_bytes().get(v11 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let mut v21: bool = 65i32 <= v20;
                let mut v22: i32 = v0.as_bytes().get(v11 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let mut v23: bool = v22 <= 90i32;
                let mut v24: bool = v21 && v23;
                v24
            };
            let mut v26: bool = v18 && v25;
            if v26 {
                v13.clone()
            } else {
                v3.clone()
            }
        } else {
            v3.clone()
        }
    }
}
fn json_escaped_15(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = std::rc::Rc::<str>::from(v0.replace(&*v1, &v2));
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = std::rc::Rc::<str>::from(v3.replace(&*v4, &v5));
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = std::rc::Rc::<str>::from(v6.replace(&*v7, &v8));
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = std::rc::Rc::<str>::from(v9.replace(&*v10, &v11));
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = std::rc::Rc::<str>::from(v12.replace(&*v13, &v14));
    v15.clone()
}
fn finding_json_12(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: Rc<str> = todo_id_13(v2.clone());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: bool = v3 == v4 ;
    let mut v10: Rc<str> = if v5 {
        v4.clone()
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"id\":\""); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = json_escaped_15(v3.clone());
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v7, &*v8, &*v4, &*v4].concat());
        v9.clone()
    };
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"path\":\""); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = json_escaped_15(v0.clone());
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\",\"line\":"); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v12, &*v13, &*v14, &*v15].concat());
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"text\":\""); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = json_escaped_15(v2.clone());
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"}"); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v10, &*v17, &*v18, &*v19].concat());
    v20.clone()
}
fn findings_from_1(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: bool, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: i32) -> (Rc<str>, i32) {
    loop {
        let mut v8: i32 = v1.len() as i32;
        let mut v9: bool = v8 <= v4;
        if v9 {
            return (v6.clone(), v7);
        } else {
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v11: i32 = v1.get(v4 as usize..).and_then(|rest| rest.find(&*v10)).map(|found| (found + v4 as usize) as i32).unwrap_or(-1);
            let mut v12: bool = v11 < 0i32;
            let mut v14: i32 = if v12 {
                let mut v13: i32 = v1.len() as i32;
                v13
            } else {
                v11
            };
            let mut v15: Rc<str> = std::rc::Rc::<str>::from(v1.get(v4 as usize..v14 as usize).unwrap_or(""));
            let mut v16: i32 = v14.wrapping_add(1i32);
            let mut v17: US1 = todo_in_line_2(v0.clone(), v15.clone());
            match &v17 {
                US1::US1_NoTodo => {
                    let mut v18: i32 = v5.wrapping_add(1i32);
                    (v0, v1, v2, v3, v4, v5, v6, v7) = (v0.clone(), v1.clone(), v2.clone(), v3, v16, v18, v6.clone(), v7);
                    continue;
                }
                US1::US1_TodoComment(v21) => {
                    let mut v21: Rc<str> = v21.clone();
                    let mut v22: bool = comment_starts_line_8(v0.clone(), v15.clone());
                    let (mut v29, mut v30, mut v31): (Rc<str>, i32, i32) = if v22 {
                        let mut v23: i32 = 0i32;
                        let mut v24: i32 = first_visible_offset_5(v15.clone(), v23);
                        let mut v25: i32 = 0i32;
                        block_from_10(v0.clone(), v1.clone(), v24, v16, v21.clone(), v25)
                    } else {
                        (v21.clone(), v16, 0i32)
                    };
                    let mut v37: Rc<str> = if v3 {
                        finding_json_12(v2.clone(), v5, v29.clone())
                    } else {
                        let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                        let mut v34: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
                        let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                        let mut v36: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v33, &*v34, &*v35, &*v29].concat());
                        v36.clone()
                    };
                    let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v39: bool = v6 == v38 ;
                    let mut v44: Rc<str> = if v39 {
                        v37.clone()
                    } else {
                        if v3 {
                            let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\n"); } LIT.with(|lit| lit.clone()) };
                            let mut v41: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v40, &*v37, &*v38, &*v38].concat());
                            v41.clone()
                        } else {
                            let mut v42: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v10, &*v37, &*v38, &*v38].concat());
                            v42.clone()
                        }
                    };
                    let mut v45: i32 = v5.wrapping_add(1i32);
                    let mut v46: i32 = v45 + v31;
                    let mut v47: i32 = v7.wrapping_add(1i32);
                    (v0, v1, v2, v3, v4, v5, v6, v7) = (v0.clone(), v1.clone(), v2.clone(), v3, v30, v46, v44.clone(), v47);
                    continue;
                }
            }
        }
    }
}
fn syntax_of_code_16(mut v0: i32) -> US0 {
    let mut v1: bool = 2i32 == v0;
    if v1 {
        US0::US0_HashFamily
    } else {
        let mut v3: bool = 3i32 == v0;
        if v3 {
            US0::US0_DashFamily
        } else {
            let mut v5: bool = 4i32 == v0;
            if v5 {
                US0::US0_HaskellFamily
            } else {
                let mut v7: bool = 5i32 == v0;
                if v7 {
                    US0::US0_LeanFamily
                } else {
                    let mut v9: bool = 6i32 == v0;
                    if v9 {
                        US0::US0_LispFamily
                    } else {
                        let mut v11: bool = 7i32 == v0;
                        if v11 {
                            US0::US0_ErlangFamily
                        } else {
                            let mut v13: bool = 8i32 == v0;
                            if v13 {
                                US0::US0_MarkupFamily
                            } else {
                                let mut v15: bool = 9i32 == v0;
                                if v15 {
                                    US0::US0_MlFamily
                                } else {
                                    let mut v17: bool = 10i32 == v0;
                                    if v17 {
                                        US0::US0_PascalFamily
                                    } else {
                                        let mut v19: bool = 11i32 == v0;
                                        if v19 {
                                            US0::US0_BatchFamily
                                        } else {
                                            US0::US0_CFamily
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
fn todo_findings_0(mut v0: i32, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: bool) -> (Rc<str>, i32) {
    let mut v4: US0 = syntax_of_code_16(v0);
    let mut v5: i32 = 0i32;
    let mut v6: i32 = 1i32;
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v8: i32 = 0i32;
    findings_from_1(v4.clone(), v1.clone(), v2.clone(), v3, v5, v6, v7.clone(), v8)
}
fn closure0() -> Rc<dyn Fn(i32, Rc<str>, Rc<str>, bool) -> (Rc<str>, i32)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, Rc<str>, Rc<str>, bool) -> (Rc<str>, i32)> = Rc::new(move |mut v0: i32, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: bool| -> (Rc<str>, i32) {
        todo_findings_0(v0, v1.clone(), v2.clone(), v3)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        todo_id_13(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_todo_findings(v0: i32, v1: &str, v2: &str, v3: bool) -> (Rc<str>, i32) {
    closure0()(v0, Rc::<str>::from(v1), Rc::<str>::from(v2), v3)
}
pub fn eoie_agile_todo_id(v0: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0))
}
