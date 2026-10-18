#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
use eoie_patch_compat_driver::{patch_apply_context_owned, patch_check_context_owned, patch_plan_error, patch_prepare_source_owned};

#[must_use]
fn patch_mutation_error(message: &str) -> i32 {
    eprintln!("eoie error: {message}");
    2
}

#[must_use]
fn patch_check_success(count: i32) -> i32 {
    println!("eoie patch check ok operations={count} mutation=false");
    0
}

#[must_use]
fn patch_preflight_error() -> i32 {
    eprintln!("eoie error: patch preflight contract rejected the validated plan");
    2
}

#[derive(Clone)]
enum US0 {
    US0_PatchPlanValid(i32),
    US0_PatchPlanEmpty,
    US0_PatchPlanMalformed,
    US0_PatchPlanTooLarge,
    US0_PatchPlanUnavailable,
}
#[derive(Clone)]
enum US1 {
    US1_QuotedOk(i32),
    US1_QuotedError,
}
#[derive(Clone)]
enum US2 {
    US2_ParsedPatch(i32),
    US2_InvalidPatch,
}
#[derive(Clone)]
enum US3 {
    US3_SeparatorOk(i32),
    US3_SeparatorError,
}
fn skip_spaces_3(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v1 < v2;
        if v3 {
            let mut v4: i32 = usize::try_from(v1).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
            let mut v5: bool = v4 == 32i32;
            if v5 {
                let mut v6: i32 = v1 + 1i32;
                (v0, v1, v2) = (v0.clone(), v6, v2);
                continue;
            } else {
                let mut v8: bool = v4 == 9i32;
                if v8 {
                    let mut v9: i32 = v1 + 1i32;
                    (v0, v1, v2) = (v0.clone(), v9, v2);
                    continue;
                } else {
                    return v1;
                }
            }
        } else {
            return v1;
        }
    }
}
fn scan_quoted_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> US1 {
    loop {
        let mut v3: bool = v1 < v2;
        if v3 {
            let mut v4: i32 = usize::try_from(v1).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
            let mut v5: bool = v4 == 34i32;
            if v5 {
                let mut v6: i32 = v1 + 1i32;
                return US1::US1_QuotedOk(v6);
            } else {
                let mut v8: bool = v4 == 92i32;
                if v8 {
                    let mut v9: i32 = v1 + 1i32;
                    let mut v10: bool = v9 < v2;
                    if v10 {
                        let mut v11: i32 = usize::try_from(v9).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                        let mut v12: bool = v11 == 110i32;
                        let mut v20: bool = if v12 {
                            true
                        } else {
                            let mut v13: bool = v11 == 114i32;
                            if v13 {
                                true
                            } else {
                                let mut v14: bool = v11 == 116i32;
                                if v14 {
                                    true
                                } else {
                                    let mut v15: bool = v11 == 92i32;
                                    if v15 {
                                        true
                                    } else {
                                        let mut v16: bool = v11 == 34i32;
                                        v16
                                    }
                                }
                            }
                        };
                        if v20 {
                            let mut v21: i32 = v1 + 2i32;
                            (v0, v1, v2) = (v0.clone(), v21, v2);
                            continue;
                        } else {
                            return US1::US1_QuotedError;
                        }
                    } else {
                        return US1::US1_QuotedError;
                    }
                } else {
                    let mut v27: i32 = v1 + 1i32;
                    (v0, v1, v2) = (v0.clone(), v27, v2);
                    continue;
                }
            }
        } else {
            return US1::US1_QuotedError;
        }
    }
}
fn safe_path_loop_6(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> bool {
    loop {
        let mut v4: bool = v1 < v2;
        if v4 {
            let mut v5: i32 = usize::try_from(v1).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
            let mut v6: bool = v5 == 92i32;
            if v6 {
                return false;
            } else {
                let mut v7: bool = v5 == 47i32;
                if v7 {
                    let mut v8: i32 = v1 - v3;
                    let mut v9: bool = 0 < v8;
                    let mut v24: bool = if v9 {
                        let mut v10: bool = v8 == 1i32;
                        if v10 {
                            let mut v11: i32 = usize::try_from(v3).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                            let mut v12: bool = v11 == 46i32;
                            let mut v13: bool = v12 == false;
                            v13
                        } else {
                            let mut v14: bool = v8 == 2i32;
                            if v14 {
                                let mut v15: i32 = usize::try_from(v3).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                                let mut v16: bool = v15 == 46i32;
                                if v16 {
                                    let mut v17: i32 = v3 + 1i32;
                                    let mut v18: i32 = usize::try_from(v17).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                                    let mut v19: bool = v18 == 46i32;
                                    let mut v20: bool = v19 == false;
                                    v20
                                } else {
                                    true
                                }
                            } else {
                                true
                            }
                        }
                    } else {
                        false
                    };
                    if v24 {
                        let mut v25: i32 = v1 + 1i32;
                        let mut v26: i32 = v1 + 1i32;
                        (v0, v1, v2, v3) = (v0.clone(), v25, v2, v26);
                        continue;
                    } else {
                        return false;
                    }
                } else {
                    let mut v29: i32 = v1 + 1i32;
                    (v0, v1, v2, v3) = (v0.clone(), v29, v2, v3);
                    continue;
                }
            }
        } else {
            let mut v33: i32 = v2 - v3;
            let mut v34: bool = 0 < v33;
            if v34 {
                let mut v35: bool = v33 == 1i32;
                if v35 {
                    let mut v36: i32 = usize::try_from(v3).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                    let mut v37: bool = v36 == 46i32;
                    let mut v38: bool = v37 == false;
                    return v38;
                } else {
                    let mut v39: bool = v33 == 2i32;
                    if v39 {
                        let mut v40: i32 = usize::try_from(v3).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                        let mut v41: bool = v40 == 46i32;
                        if v41 {
                            let mut v42: i32 = v3 + 1i32;
                            let mut v43: i32 = usize::try_from(v42).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                            let mut v44: bool = v43 == 46i32;
                            let mut v45: bool = v44 == false;
                            return v45;
                        } else {
                            return true;
                        }
                    } else {
                        return true;
                    }
                }
            } else {
                return false;
            }
        }
    }
}
fn safe_path_loop_5(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> bool {
    let mut v3: bool = v1 < v2;
    if v3 {
        let mut v4: i32 = usize::try_from(v1).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
        let mut v5: bool = v4 == 92i32;
        if v5 {
            false
        } else {
            let mut v6: bool = v4 == 47i32;
            if v6 {
                let mut v7: i32 = v1 - v1;
                let mut v8: bool = 0 < v7;
                let mut v23: bool = if v8 {
                    let mut v9: bool = v7 == 1i32;
                    if v9 {
                        let mut v10: i32 = usize::try_from(v1).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                        let mut v11: bool = v10 == 46i32;
                        let mut v12: bool = v11 == false;
                        v12
                    } else {
                        let mut v13: bool = v7 == 2i32;
                        if v13 {
                            let mut v14: i32 = usize::try_from(v1).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                            let mut v15: bool = v14 == 46i32;
                            if v15 {
                                let mut v16: i32 = v1 + 1i32;
                                let mut v17: i32 = usize::try_from(v16).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                                let mut v18: bool = v17 == 46i32;
                                let mut v19: bool = v18 == false;
                                v19
                            } else {
                                true
                            }
                        } else {
                            true
                        }
                    }
                } else {
                    false
                };
                if v23 {
                    let mut v24: i32 = v1 + 1i32;
                    let mut v25: i32 = v1 + 1i32;
                    safe_path_loop_6(v0.clone(), v24, v2, v25)
                } else {
                    false
                }
            } else {
                let mut v28: i32 = v1 + 1i32;
                safe_path_loop_6(v0.clone(), v28, v2, v1)
            }
        }
    } else {
        let mut v32: i32 = v2 - v1;
        let mut v33: bool = 0 < v32;
        if v33 {
            let mut v34: bool = v32 == 1i32;
            if v34 {
                let mut v35: i32 = usize::try_from(v1).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                let mut v36: bool = v35 == 46i32;
                let mut v37: bool = v36 == false;
                v37
            } else {
                let mut v38: bool = v32 == 2i32;
                if v38 {
                    let mut v39: i32 = usize::try_from(v1).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                    let mut v40: bool = v39 == 46i32;
                    if v40 {
                        let mut v41: i32 = v1 + 1i32;
                        let mut v42: i32 = usize::try_from(v41).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                        let mut v43: bool = v42 == 46i32;
                        let mut v44: bool = v43 == false;
                        v44
                    } else {
                        true
                    }
                } else {
                    true
                }
            }
        } else {
            false
        }
    }
}
fn scan_plan_2(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> US0 {
    loop {
        let mut v4: bool = v1 < v2;
        if v4 {
            let mut v5: i32 = v1 + 0i32;
            let mut v6: i32 = usize::try_from(v5).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
            let mut v7: bool = v6 == 80i32;
            let mut v51: bool = if v7 {
                let mut v8: i32 = v1 + 1i32;
                let mut v9: i32 = usize::try_from(v8).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                let mut v10: bool = v9 == 97i32;
                if v10 {
                    let mut v11: i32 = v1 + 2i32;
                    let mut v12: i32 = usize::try_from(v11).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                    let mut v13: bool = v12 == 116i32;
                    if v13 {
                        let mut v14: i32 = v1 + 3i32;
                        let mut v15: i32 = usize::try_from(v14).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                        let mut v16: bool = v15 == 99i32;
                        if v16 {
                            let mut v17: i32 = v1 + 4i32;
                            let mut v18: i32 = usize::try_from(v17).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                            let mut v19: bool = v18 == 104i32;
                            if v19 {
                                let mut v20: i32 = v1 + 5i32;
                                let mut v21: i32 = usize::try_from(v20).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                                let mut v22: bool = v21 == 69i32;
                                if v22 {
                                    let mut v23: i32 = v1 + 6i32;
                                    let mut v24: i32 = usize::try_from(v23).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                                    let mut v25: bool = v24 == 120i32;
                                    if v25 {
                                        let mut v26: i32 = v1 + 7i32;
                                        let mut v27: i32 = usize::try_from(v26).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                                        let mut v28: bool = v27 == 97i32;
                                        if v28 {
                                            let mut v29: i32 = v1 + 8i32;
                                            let mut v30: i32 = usize::try_from(v29).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                                            let mut v31: bool = v30 == 99i32;
                                            if v31 {
                                                let mut v32: i32 = v1 + 9i32;
                                                let mut v33: i32 = usize::try_from(v32).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                                                let mut v34: bool = v33 == 116i32;
                                                if v34 {
                                                    let mut v35: i32 = v1 + 10i32;
                                                    let mut v36: i32 = usize::try_from(v35).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                                                    let mut v37: bool = v36 == 32i32;
                                                    if v37 {
                                                        let mut v38: i32 = v1 + 11i32;
                                                        let mut v39: i32 = usize::try_from(v38).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                                                        let mut v40: bool = v39 == 40i32;
                                                        v40
                                                    } else {
                                                        false
                                                    }
                                                } else {
                                                    false
                                                }
                                            } else {
                                                false
                                            }
                                        } else {
                                            false
                                        }
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            };
            if v51 {
                let mut v52: i32 = v1 + 12i32;
                let mut v53: i32 = skip_spaces_3(v0.clone(), v52, v2);
                let mut v54: bool = v53 < v2;
                let mut v62: US1 = if v54 {
                    let mut v55: i32 = usize::try_from(v53).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                    let mut v56: bool = v55 == 34i32;
                    if v56 {
                        let mut v57: i32 = v53 + 1i32;
                        scan_quoted_4(v0.clone(), v57, v2)
                    } else {
                        US1::US1_QuotedError
                    }
                } else {
                    US1::US1_QuotedError
                };
                let mut v145: US2 = match &v62 {
                    US1::US1_QuotedError => {
                        US2::US2_InvalidPatch
                    }
                    US1::US1_QuotedOk(v63) => {
                        let mut v63: i32 = *v63;
                        let mut v64: i32 = v53 + 1i32;
                        let mut v65: i32 = v63 - 1i32;
                        let mut v66: i32 = v65 - v64;
                        let mut v67: bool = 0 < v66;
                        let mut v74: bool = if v67 {
                            let mut v68: i32 = usize::try_from(v64).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                            let mut v69: bool = v68 == 47i32;
                            if v69 {
                                false
                            } else {
                                let mut v70: bool = v68 == 92i32;
                                if v70 {
                                    false
                                } else {
                                    safe_path_loop_5(v0.clone(), v64, v65)
                                }
                            }
                        } else {
                            false
                        };
                        if v74 {
                            let mut v75: i32 = skip_spaces_3(v0.clone(), v63, v2);
                            let mut v76: bool = v75 < v2;
                            let mut v85: US3 = if v76 {
                                let mut v77: i32 = usize::try_from(v75).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                                let mut v78: bool = v77 == 44i32;
                                if v78 {
                                    let mut v79: i32 = v75 + 1i32;
                                    let mut v80: i32 = skip_spaces_3(v0.clone(), v79, v2);
                                    US3::US3_SeparatorOk(v80)
                                } else {
                                    US3::US3_SeparatorError
                                }
                            } else {
                                US3::US3_SeparatorError
                            };
                            match &v85 {
                                US3::US3_SeparatorError => {
                                    US2::US2_InvalidPatch
                                }
                                US3::US3_SeparatorOk(v86) => {
                                    let mut v86: i32 = *v86;
                                    let mut v87: bool = v86 < v2;
                                    let mut v95: US1 = if v87 {
                                        let mut v88: i32 = usize::try_from(v86).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                                        let mut v89: bool = v88 == 34i32;
                                        if v89 {
                                            let mut v90: i32 = v86 + 1i32;
                                            scan_quoted_4(v0.clone(), v90, v2)
                                        } else {
                                            US1::US1_QuotedError
                                        }
                                    } else {
                                        US1::US1_QuotedError
                                    };
                                    match &v95 {
                                        US1::US1_QuotedError => {
                                            US2::US2_InvalidPatch
                                        }
                                        US1::US1_QuotedOk(v96) => {
                                            let mut v96: i32 = *v96;
                                            let mut v97: i32 = skip_spaces_3(v0.clone(), v96, v2);
                                            let mut v98: bool = v97 < v2;
                                            let mut v107: US3 = if v98 {
                                                let mut v99: i32 = usize::try_from(v97).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                                                let mut v100: bool = v99 == 44i32;
                                                if v100 {
                                                    let mut v101: i32 = v97 + 1i32;
                                                    let mut v102: i32 = skip_spaces_3(v0.clone(), v101, v2);
                                                    US3::US3_SeparatorOk(v102)
                                                } else {
                                                    US3::US3_SeparatorError
                                                }
                                            } else {
                                                US3::US3_SeparatorError
                                            };
                                            match &v107 {
                                                US3::US3_SeparatorError => {
                                                    US2::US2_InvalidPatch
                                                }
                                                US3::US3_SeparatorOk(v108) => {
                                                    let mut v108: i32 = *v108;
                                                    let mut v109: bool = v108 < v2;
                                                    let mut v117: US1 = if v109 {
                                                        let mut v110: i32 = usize::try_from(v108).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                                                        let mut v111: bool = v110 == 34i32;
                                                        if v111 {
                                                            let mut v112: i32 = v108 + 1i32;
                                                            scan_quoted_4(v0.clone(), v112, v2)
                                                        } else {
                                                            US1::US1_QuotedError
                                                        }
                                                    } else {
                                                        US1::US1_QuotedError
                                                    };
                                                    match &v117 {
                                                        US1::US1_QuotedError => {
                                                            US2::US2_InvalidPatch
                                                        }
                                                        US1::US1_QuotedOk(v118) => {
                                                            let mut v118: i32 = *v118;
                                                            let mut v119: i32 = skip_spaces_3(v0.clone(), v118, v2);
                                                            let mut v120: bool = v119 < v2;
                                                            if v120 {
                                                                let mut v121: i32 = usize::try_from(v119).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                                                                let mut v122: bool = v121 == 41i32;
                                                                if v122 {
                                                                    let mut v123: i32 = v119 + 1i32;
                                                                    US2::US2_ParsedPatch(v123)
                                                                } else {
                                                                    US2::US2_InvalidPatch
                                                                }
                                                            } else {
                                                                US2::US2_InvalidPatch
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            US2::US2_InvalidPatch
                        }
                    }
                };
                match &v145 {
                    US2::US2_InvalidPatch => {
                        return US0::US0_PatchPlanMalformed;
                    }
                    US2::US2_ParsedPatch(v146) => {
                        let mut v146: i32 = *v146;
                        let mut v147: i32 = v3 + 1i32;
                        (v0, v1, v2, v3) = (v0.clone(), v146, v2, v147);
                        continue;
                    }
                }
            } else {
                let mut v152: i32 = v1 + 1i32;
                (v0, v1, v2, v3) = (v0.clone(), v152, v2, v3);
                continue;
            }
        } else {
            let mut v155: bool = 0 < v3;
            if v155 {
                return US0::US0_PatchPlanValid(v3);
            } else {
                return US0::US0_PatchPlanEmpty;
            }
        }
    }
}
fn patch_preflight_mode_binding_7(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 < 0i32;
    if v2 {
        0i32
    } else {
        let mut v3: bool = v1 < 0i32;
        if v3 {
            0i32
        } else {
            let mut v4: bool = 1i32 < v0;
            if v4 {
                0i32
            } else {
                let mut v5: bool = 1i32 < v1;
                if v5 {
                    0i32
                } else {
                    let mut v6: bool = v0 < 1i32;
                    if v6 {
                        let mut v7: bool = v1 < 1i32;
                        if v7 {
                            0i32
                        } else {
                            1i32
                        }
                    } else {
                        let mut v9: bool = v1 < 1i32;
                        if v9 {
                            1i32
                        } else {
                            0i32
                        }
                    }
                }
            }
        }
    }
}
fn patch_preflight_witness_binding_8(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 < 1i32;
    if v2 {
        0i32
    } else {
        let mut v3: bool = v1 < 1i32;
        if v3 {
            0i32
        } else {
            let mut v4: bool = v0 < v1;
            if v4 {
                0i32
            } else {
                let mut v5: bool = v1 < v0;
                if v5 {
                    0i32
                } else {
                    1i32
                }
            }
        }
    }
}
fn patch_preflight_check_binding_9(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 < 0i32;
    if v2 {
        0i32
    } else {
        let mut v3: bool = v1 < 0i32;
        if v3 {
            0i32
        } else {
            let mut v4: bool = 0i32 < v0;
            if v4 {
                0i32
            } else {
                let mut v5: bool = 0i32 < v1;
                if v5 {
                    0i32
                } else {
                    1i32
                }
            }
        }
    }
}
fn patch_entry_1() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v0 ;
    if v1 {
        let mut v2: i32 = 1i32;
        let mut v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v4: i32 = 2i32;
        let mut v5: Rc<str> = usize::try_from(v4).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v6: Rc<str> = patch_prepare_source_owned(v3.as_ref(), v5.as_ref());
        let mut v7: i32 = i32::try_from(v6.len()).unwrap_or(i32::MAX);
        let mut v8: bool = 0 < v7;
        if v8 {
            let mut v9: i32 = canonical_plan_ir_domain::plan_ir_manifest_count(&v6);
            let mut v10: bool = 0 < v9;
            let mut v25: US0 = if v10 {
                US0::US0_PatchPlanValid(v9)
            } else {
                let mut v12: bool = v9 < 0i32;
                if v12 {
                    US0::US0_PatchPlanMalformed
                } else {
                    let mut v14: i32 = i32::try_from(v6.len()).unwrap_or(i32::MAX);
                    let mut v15: bool = v14 < 0i32;
                    if v15 {
                        US0::US0_PatchPlanUnavailable
                    } else {
                        let mut v17: bool = v14 <= 262144i32;
                        if v17 {
                            let mut v18: i32 = 0i32;
                            let mut v19: i32 = 0i32;
                            scan_plan_2(v6.clone(), v18, v14, v19)
                        } else {
                            US0::US0_PatchPlanTooLarge
                        }
                    }
                }
            };
            let mut v31: i32 = match &v25 {
                US0::US0_PatchPlanEmpty => {
                    0i32
                }
                US0::US0_PatchPlanMalformed => {
                    -1i32
                }
                US0::US0_PatchPlanTooLarge => {
                    -2i32
                }
                US0::US0_PatchPlanUnavailable => {
                    -3i32
                }
                US0::US0_PatchPlanValid(v26) => {
                    let mut v26: i32 = *v26;
                    v26
                }
            };
            let mut v32: i32 = 0i32;
            let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("apply"); } LIT.with(|lit| lit.clone()) };
            let mut v34: bool = usize::try_from(v32).ok().and_then(|index| std::env::args().skip(2).nth(index)).is_some_and(|value| value.as_str() == v33.as_ref());
            if v34 {
                let mut v35: i32 = 1i32;
                let mut v36: i32 = 0i32;
                let mut v37: i32 = patch_preflight_mode_binding_7(v35, v36);
                let mut v38: bool = 1i32 == v37;
                if v38 {
                    let mut v39: bool = 0 < v31;
                    if v39 {
                        let mut v40: i32 = patch_apply_context_owned(v3.as_ref(), v6.as_ref(), v31);
                        v40
                    } else {
                        let mut v41: i32 = patch_plan_error(v31);
                        v41
                    }
                } else {
                    let mut v43: i32 = patch_preflight_error();
                    v43
                }
            } else {
                let mut v45: i32 = 0i32;
                let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("check"); } LIT.with(|lit| lit.clone()) };
                let mut v47: bool = usize::try_from(v45).ok().and_then(|index| std::env::args().skip(2).nth(index)).is_some_and(|value| value.as_str() == v46.as_ref());
                if v47 {
                    let mut v48: bool = 0 < v31;
                    if v48 {
                        let mut v49: i32 = patch_check_context_owned(v3.as_ref(), v6.as_ref(), v31);
                        let mut v50: i32 = patch_preflight_witness_binding_8(v31, v49);
                        let mut v51: bool = 1i32 == v50;
                        if v51 {
                            let mut v52: i32 = 0i32;
                            let mut v53: i32 = 1i32;
                            let mut v54: i32 = patch_preflight_mode_binding_7(v52, v53);
                            let mut v55: bool = 1i32 == v54;
                            if v55 {
                                let mut v56: i32 = 0i32;
                                let mut v57: i32 = 0i32;
                                let mut v58: i32 = patch_preflight_check_binding_9(v56, v57);
                                let mut v59: bool = 1i32 == v58;
                                if v59 {
                                    let mut v60: i32 = patch_check_success(v49);
                                    v60
                                } else {
                                    let mut v61: i32 = patch_preflight_error();
                                    v61
                                }
                            } else {
                                let mut v63: i32 = patch_preflight_error();
                                v63
                            }
                        } else {
                            let mut v65: i32 = patch_preflight_error();
                            v65
                        }
                    } else {
                        let mut v67: i32 = patch_plan_error(v31);
                        v67
                    }
                } else {
                    let mut v69: i32 = patch_mutation_error("patch supports only: apply|check <root> <plan.spi>");
                    v69
                }
            }
        } else {
            2i32
        }
    } else {
        let mut v73: i32 = patch_mutation_error("patch supports only: apply|check <root> <plan.spi>");
        v73
    }
}
fn patch_entry_public_0() -> i32 {
    patch_entry_1()
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        patch_entry_public_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_patch_run() -> i32 {
    closure0()()
}
