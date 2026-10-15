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
fn spiral_main() -> i32 {
    let mut v0: i32 = 1i32;
    let mut v1: i32 = 1i32;
    let mut v2: i32 = archive_extract_preflight_binding_0(v0, v1);
    let mut v3: i32 = 0i32;
    let mut v4: i32 = 0i32;
    let mut v5: i32 = archive_extract_preflight_binding_0(v3, v4);
    let mut v6: i32 = 1i32;
    let mut v7: i32 = 0i32;
    let mut v8: i32 = archive_extract_process_binding_1(v6, v7);
    let mut v9: i32 = 4i32;
    let mut v10: i32 = 3i32;
    let mut v11: i32 = archive_extract_complete_binding_2(v9, v10);
    let mut v12: bool = 0i32 < v2;
    if v12 {
        1i32
    } else {
        let mut v13: bool = 0i32 < v5;
        if v13 {
            1i32
        } else {
            let mut v14: bool = 0i32 < v8;
            if v14 {
                1i32
            } else {
                let mut v15: bool = 0i32 < v11;
                if v15 {
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
