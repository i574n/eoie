#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_patterns, unreachable_code, while_true)]
#![allow(clippy::needless_return, clippy::needless_late_init)]
use std::cell::RefCell;
use std::rc::Rc;
fn method0(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 < 1i32;
    if v2 {
        -1i32
    } else {
        let mut v3: bool = v1 < 3i32;
        if v3 {
            -1i32
        } else {
            let mut v4: bool = 3i32 < v1;
            if v4 {
                -1i32
            } else {
                let mut v5: bool = v0 < v1;
                if v5 {
                    0i32
                } else {
                    let mut v6: i32 = v0 - v1;
                    v6
                }
            }
        }
    }
}
fn method1(mut v0: i32, mut v1: i32) -> i32 {
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
fn method2(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 < 0i32;
    if v2 {
        0i32
    } else {
        let mut v3: bool = v1 < 0i32;
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
fn method3(mut v0: i32, mut v1: i32) -> i32 {
    method1(v0, v1)
}
fn method4(mut v0: i32, mut v1: i32) -> i32 {
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
fn spiral_main() -> i32 {
    let mut v0: i32 = 5i32;
    let mut v1: i32 = 3i32;
    let mut v2: i32 = method0(v0, v1);
    let mut v3: i32 = 1i32;
    let mut v4: i32 = 1i32;
    let mut v5: i32 = method1(v3, v4);
    let mut v6: i32 = 2i32;
    let mut v7: i32 = method2(v6, v2);
    let mut v8: i32 = 1i32;
    let mut v9: i32 = method3(v7, v8);
    let mut v10: i32 = 3i32;
    let mut v11: i32 = 3i32;
    let mut v12: i32 = method4(v10, v11);
    let mut v13: bool = v2 < 2i32;
    if v13 {
        1i32
    } else {
        let mut v14: bool = 2i32 < v2;
        if v14 {
            1i32
        } else {
            let mut v15: bool = v5 < 1i32;
            if v15 {
                1i32
            } else {
                let mut v16: bool = v7 < 1i32;
                if v16 {
                    1i32
                } else {
                    let mut v17: bool = v9 < 1i32;
                    if v17 {
                        1i32
                    } else {
                        let mut v18: bool = v12 < 1i32;
                        if v18 {
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
fn main() {
    let main = std::thread::Builder::new().stack_size(1 << 30).spawn(spiral_main).unwrap();
    std::process::exit(main.join().unwrap());
}
