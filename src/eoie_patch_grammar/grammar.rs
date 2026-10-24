#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_QuotedOk(i32),
    US0_QuotedError,
}
#[derive(Clone)]
enum US1 {
    US1_ParsedPatch(i32),
    US1_InvalidPatch,
}
#[derive(Clone)]
enum US2 {
    US2_SeparatorOk(i32),
    US2_SeparatorError,
}
fn skip_spaces_0(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
fn scan_quoted_1(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> US0 {
    loop {
        let mut v3: bool = v1 < v2;
        if v3 {
            let mut v4: i32 = usize::try_from(v1).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
            let mut v5: bool = v4 == 34i32;
            if v5 {
                let mut v6: i32 = v1 + 1i32;
                return US0::US0_QuotedOk(v6);
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
                            return US0::US0_QuotedError;
                        }
                    } else {
                        return US0::US0_QuotedError;
                    }
                } else {
                    let mut v27: i32 = v1 + 1i32;
                    (v0, v1, v2) = (v0.clone(), v27, v2);
                    continue;
                }
            }
        } else {
            return US0::US0_QuotedError;
        }
    }
}
fn safe_path_loop_2(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> bool {
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
fn spiral_main() -> i32 {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl p () = PatchExact (\"a.txt\", \"old\", \"new\")\n"); } LIT.with(|lit| lit.clone()) };
    let mut v1: i32 = i32::try_from(v0.len()).unwrap_or(i32::MAX);
    let mut v2: i32 = 11i32 + 12i32;
    let mut v3: i32 = skip_spaces_0(v0.clone(), v2, v1);
    let mut v4: bool = v3 < v1;
    let mut v12: US0 = if v4 {
        let mut v5: i32 = usize::try_from(v3).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
        let mut v6: bool = v5 == 34i32;
        if v6 {
            let mut v7: i32 = v3 + 1i32;
            scan_quoted_1(v0.clone(), v7, v1)
        } else {
            US0::US0_QuotedError
        }
    } else {
        US0::US0_QuotedError
    };
    let mut v95: US1 = match &v12 {
        US0::US0_QuotedError => {
            US1::US1_InvalidPatch
        }
        US0::US0_QuotedOk(v13) => {
            let mut v13: i32 = *v13;
            let mut v14: i32 = v3 + 1i32;
            let mut v15: i32 = v13 - 1i32;
            let mut v16: i32 = v15 - v14;
            let mut v17: bool = 0 < v16;
            let mut v24: bool = if v17 {
                let mut v18: i32 = usize::try_from(v14).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                let mut v19: bool = v18 == 47i32;
                if v19 {
                    false
                } else {
                    let mut v20: bool = v18 == 92i32;
                    if v20 {
                        false
                    } else {
                        safe_path_loop_2(v0.clone(), v14, v15, v14)
                    }
                }
            } else {
                false
            };
            if v24 {
                let mut v25: i32 = skip_spaces_0(v0.clone(), v13, v1);
                let mut v26: bool = v25 < v1;
                let mut v35: US2 = if v26 {
                    let mut v27: i32 = usize::try_from(v25).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                    let mut v28: bool = v27 == 44i32;
                    if v28 {
                        let mut v29: i32 = v25 + 1i32;
                        let mut v30: i32 = skip_spaces_0(v0.clone(), v29, v1);
                        US2::US2_SeparatorOk(v30)
                    } else {
                        US2::US2_SeparatorError
                    }
                } else {
                    US2::US2_SeparatorError
                };
                match &v35 {
                    US2::US2_SeparatorError => {
                        US1::US1_InvalidPatch
                    }
                    US2::US2_SeparatorOk(v36) => {
                        let mut v36: i32 = *v36;
                        let mut v37: bool = v36 < v1;
                        let mut v45: US0 = if v37 {
                            let mut v38: i32 = usize::try_from(v36).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                            let mut v39: bool = v38 == 34i32;
                            if v39 {
                                let mut v40: i32 = v36 + 1i32;
                                scan_quoted_1(v0.clone(), v40, v1)
                            } else {
                                US0::US0_QuotedError
                            }
                        } else {
                            US0::US0_QuotedError
                        };
                        match &v45 {
                            US0::US0_QuotedError => {
                                US1::US1_InvalidPatch
                            }
                            US0::US0_QuotedOk(v46) => {
                                let mut v46: i32 = *v46;
                                let mut v47: i32 = skip_spaces_0(v0.clone(), v46, v1);
                                let mut v48: bool = v47 < v1;
                                let mut v57: US2 = if v48 {
                                    let mut v49: i32 = usize::try_from(v47).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                                    let mut v50: bool = v49 == 44i32;
                                    if v50 {
                                        let mut v51: i32 = v47 + 1i32;
                                        let mut v52: i32 = skip_spaces_0(v0.clone(), v51, v1);
                                        US2::US2_SeparatorOk(v52)
                                    } else {
                                        US2::US2_SeparatorError
                                    }
                                } else {
                                    US2::US2_SeparatorError
                                };
                                match &v57 {
                                    US2::US2_SeparatorError => {
                                        US1::US1_InvalidPatch
                                    }
                                    US2::US2_SeparatorOk(v58) => {
                                        let mut v58: i32 = *v58;
                                        let mut v59: bool = v58 < v1;
                                        let mut v67: US0 = if v59 {
                                            let mut v60: i32 = usize::try_from(v58).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                                            let mut v61: bool = v60 == 34i32;
                                            if v61 {
                                                let mut v62: i32 = v58 + 1i32;
                                                scan_quoted_1(v0.clone(), v62, v1)
                                            } else {
                                                US0::US0_QuotedError
                                            }
                                        } else {
                                            US0::US0_QuotedError
                                        };
                                        match &v67 {
                                            US0::US0_QuotedError => {
                                                US1::US1_InvalidPatch
                                            }
                                            US0::US0_QuotedOk(v68) => {
                                                let mut v68: i32 = *v68;
                                                let mut v69: i32 = skip_spaces_0(v0.clone(), v68, v1);
                                                let mut v70: bool = v69 < v1;
                                                if v70 {
                                                    let mut v71: i32 = usize::try_from(v69).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                                                    let mut v72: bool = v71 == 41i32;
                                                    if v72 {
                                                        let mut v73: i32 = v69 + 1i32;
                                                        US1::US1_ParsedPatch(v73)
                                                    } else {
                                                        US1::US1_InvalidPatch
                                                    }
                                                } else {
                                                    US1::US1_InvalidPatch
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
                US1::US1_InvalidPatch
            }
        }
    };
    match &v95 {
        US1::US1_InvalidPatch => {
            -1i32
        }
        US1::US1_ParsedPatch(v96) => {
            let mut v96: i32 = *v96;
            v96
        }
    }
}
fn main() { #[cfg(target_arch = "wasm32")] { spiral_main(); return; }
    let main = std::thread::Builder::new().stack_size(1 << 30).spawn(spiral_main).unwrap();
    std::process::exit(match main.join() { Ok(code) => code, Err(_) => 1 });
}
