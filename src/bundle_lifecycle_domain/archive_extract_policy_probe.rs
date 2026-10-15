#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
#![allow(clippy::needless_return, clippy::needless_late_init)]
use std::cell::RefCell;
use std::rc::Rc;
fn archive_extract_preflight_binding_0(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 < 0i32;
    if v2 {
        0i32
    } else {
        let mut v3: bool = 1i32 < v0;
        if v3 {
            0i32
        } else {
            let mut v4: bool = v1 < 1i32;
            if v4 {
                0i32
            } else {
                let mut v5: bool = 1i32 < v1;
                if v5 {
                    0i32
                } else {
                    let mut v6: bool = v0 < 1i32;
                    if v6 {
                        1i32
                    } else {
                        0i32
                    }
                }
            }
        }
    }
}
fn archive_extract_process_binding_1(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 < 1i32;
    if v2 {
        0i32
    } else {
        let mut v3: bool = 1i32 < v0;
        if v3 {
            0i32
        } else {
            let mut v4: bool = v1 < 1i32;
            if v4 {
                0i32
            } else {
                let mut v5: bool = 1i32 < v1;
                if v5 {
                    0i32
                } else {
                    1i32
                }
            }
        }
    }
}
fn bundle_root_binding_5(mut v0: i32, mut v1: i32) -> i32 {
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
fn bundle_entry_binding_4(mut v0: i32, mut v1: i32) -> i32 {
    bundle_root_binding_5(v0, v1)
}
fn bundle_extract_binding_3(mut v0: i32, mut v1: i32) -> i32 {
    bundle_entry_binding_4(v0, v1)
}
fn archive_extract_complete_binding_2(mut v0: i32, mut v1: i32) -> i32 {
    bundle_extract_binding_3(v0, v1)
}
fn bundle_rehydration_binding_8(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 < 1i32;
    if v2 {
        0i32
    } else {
        let mut v3: bool = 1i32 < v0;
        if v3 {
            0i32
        } else {
            let mut v4: bool = v1 < 1i32;
            if v4 {
                0i32
            } else {
                let mut v5: bool = 1i32 < v1;
                if v5 {
                    0i32
                } else {
                    1i32
                }
            }
        }
    }
}
fn bundle_complete_binding_7(mut v0: i32, mut v1: i32) -> i32 {
    bundle_rehydration_binding_8(v0, v1)
}
fn archive_extract_receipt_binding_6(mut v0: i32, mut v1: i32) -> i32 {
    bundle_complete_binding_7(v0, v1)
}
fn spiral_main() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: i32 = 1i32;
    let mut v2: i32 = archive_extract_preflight_binding_0(v0, v1);
    let mut v3: bool = v2 < 1i32;
    let mut v5: bool = if v3 {
        true
    } else {
        let mut v4: bool = 1i32 < v2;
        v4
    };
    if v5 {
        1i32
    } else {
        let mut v6: i32 = 1i32;
        let mut v7: i32 = 1i32;
        let mut v8: i32 = archive_extract_process_binding_1(v6, v7);
        let mut v9: bool = v8 < 1i32;
        let mut v11: bool = if v9 {
            true
        } else {
            let mut v10: bool = 1i32 < v8;
            v10
        };
        if v11 {
            1i32
        } else {
            let mut v12: i32 = 7i32;
            let mut v13: i32 = 7i32;
            let mut v14: i32 = archive_extract_complete_binding_2(v12, v13);
            let mut v15: bool = v14 < 1i32;
            let mut v17: bool = if v15 {
                true
            } else {
                let mut v16: bool = 1i32 < v14;
                v16
            };
            if v17 {
                1i32
            } else {
                let mut v18: i32 = 1i32;
                let mut v19: i32 = 1i32;
                let mut v20: i32 = archive_extract_receipt_binding_6(v18, v19);
                let mut v21: bool = v20 < 1i32;
                let mut v23: bool = if v21 {
                    true
                } else {
                    let mut v22: bool = 1i32 < v20;
                    v22
                };
                if v23 {
                    1i32
                } else {
                    0i32
                }
            }
        }
    }
}
fn main() { #[cfg(target_arch = "wasm32")] { spiral_main(); return; }
    let main = std::thread::Builder::new().stack_size(1 << 30).spawn(spiral_main).unwrap();
    std::process::exit(match main.join() { Ok(code) => code, Err(_) => 101 });
}
