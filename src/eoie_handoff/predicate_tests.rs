#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_patterns, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[cfg(test)] mod typed_predicate_tests { #[test] fn ascii_and_unicode_match_original_rust_predicates() { assert_eq!(super::eoie_test_typed_predicates(),0); } }

fn method2(mut v0: u64, mut v1: u64) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::string::String::from_utf8(std::iter::repeat_n(v0 as u8,v1 as usize).collect::<Vec<_>>()).unwrap());
    v2.clone()
}
fn method4(mut v0: bool) -> () {
    if v0 { () } else { std::panic::panic_any("typed predicate differential mismatch") };
    ()
}
fn method5(mut v0: Rc<str>, mut v1: u64, mut v2: u64) -> bool {
    loop {
        let mut v3: bool = v2 == v1;
        if v3 {
            return true;
        } else {
            let mut v4: u64 = v0.as_bytes()[v2 as usize] as u64;
            let mut v5: bool = v4 < 48u64;
            let mut v7: bool = if v5 {
                false
            } else {
                let mut v6: bool = v4 <= 57u64;
                v6
            };
            let mut v15: bool = if v7 {
                true
            } else {
                let mut v8: bool = v4 < 65u64;
                let mut v10: bool = if v8 {
                    false
                } else {
                    let mut v9: bool = v4 <= 70u64;
                    v9
                };
                if v10 {
                    true
                } else {
                    let mut v11: bool = v4 < 97u64;
                    if v11 {
                        false
                    } else {
                        let mut v12: bool = v4 <= 102u64;
                        v12
                    }
                }
            };
            if v15 {
                let mut v16: u64 = v2 + 1u64;
                (v0, v1, v2) = (v0.clone(), v1, v16);
                continue;
            } else {
                return false;
            }
        }
    }
}
fn method6(mut v0: Rc<str>, mut v1: u64, mut v2: u64) -> bool {
    loop {
        let mut v3: bool = v2 == v1;
        if v3 {
            return true;
        } else {
            let mut v4: u64 = v0.as_bytes()[v2 as usize] as u64;
            let mut v5: bool = v4 < 48u64;
            let mut v7: bool = if v5 {
                false
            } else {
                let mut v6: bool = v4 <= 57u64;
                v6
            };
            let mut v15: bool = if v7 {
                true
            } else {
                let mut v8: bool = v4 < 65u64;
                let mut v10: bool = if v8 {
                    false
                } else {
                    let mut v9: bool = v4 <= 90u64;
                    v9
                };
                if v10 {
                    true
                } else {
                    let mut v11: bool = v4 < 97u64;
                    if v11 {
                        false
                    } else {
                        let mut v12: bool = v4 <= 122u64;
                        v12
                    }
                }
            };
            let mut v23: bool = if v15 {
                true
            } else {
                let mut v16: bool = v4 < 45u64;
                let mut v18: bool = if v16 {
                    false
                } else {
                    let mut v17: bool = v4 <= 47u64;
                    v17
                };
                if v18 {
                    true
                } else {
                    let mut v19: bool = v4 < 95u64;
                    if v19 {
                        false
                    } else {
                        let mut v20: bool = v4 <= 95u64;
                        v20
                    }
                }
            };
            if v23 {
                let mut v24: u64 = v2 + 1u64;
                (v0, v1, v2) = (v0.clone(), v1, v24);
                continue;
            } else {
                return false;
            }
        }
    }
}
fn method3(mut v0: Rc<str>) -> () {
    let mut v1: bool = v0.len() == 64 && v0.bytes().all(|byte| byte.is_ascii_hexdigit());
    let mut v2: bool = (v0.len() > 0) && v0.bytes().all(|byte| byte.is_ascii_alphanumeric() || [45u8,46,47,95].contains(&byte));
    let mut v3: u64 = (v0.clone().len() as u64);
    let mut v4: bool = v3 < 64u64;
    let mut v6: bool = if v4 {
        false
    } else {
        let mut v5: bool = v3 <= 64u64;
        v5
    };
    let mut v9: bool = if v6 {
        let mut v7: u64 = 0u64;
        method5(v0.clone(), v3, v7)
    } else {
        false
    };
    let mut v10: bool = 1u64 <= v3;
    let mut v13: bool = if v10 {
        let mut v11: u64 = 0u64;
        method6(v0.clone(), v3, v11)
    } else {
        false
    };
    let mut v14: bool = v9 == v1;
    method4(v14);
    let mut v15: bool = v13 == v2;
    method4(v15);
    let mut v16: bool = eoie_handoff::binary_install_hash_text(&*v0) == v1 ;
    method4(v16);
    let mut v17: bool = eoie_handoff::binary_install_safe_text(&*v0) == v2 ;
    method4(v17)
}
fn method1(mut v0: u64) -> u64 {
    loop {
        let mut v1: bool = v0 < 128u64;
        if v1 {
            let mut v2: u64 = 1u64;
            let mut v3: Rc<str> = method2(v0, v2);
            method3(v3.clone());
            let mut v4: u64 = 63u64;
            let mut v5: Rc<str> = method2(v0, v4);
            method3(v5.clone());
            let mut v6: u64 = 64u64;
            let mut v7: Rc<str> = method2(v0, v6);
            method3(v7.clone());
            let mut v8: u64 = 65u64;
            let mut v9: Rc<str> = method2(v0, v8);
            method3(v9.clone());
            let mut v10: u64 = v0 + 1u64;
            v0 = v10;
            continue;
        } else {
            return v0;
        }
    }
}
fn method0() -> i32 {
    let mut v0: u64 = 0u64;
    let mut v1: u64 = method1(v0);
    let mut v2: bool = v1 == 128u64;
    method4(v2);
    let mut v3: Rc<str> = Rc::<str>::from("");
    method3(v3.clone());
    let mut v4: Rc<str> = Rc::<str>::from("aB09-._/");
    method3(v4.clone());
    let mut v5: Rc<str> = Rc::<str>::from("é");
    method3(v5.clone());
    let mut v6: Rc<str> = Rc::<str>::from("路/receipt");
    method3(v6.clone());
    let mut v7: Rc<str> = Rc::<str>::from("ABCDEFabcdef0123456789ABCDEFabcdef0123456789ABCDEFabcdef0123456789ABCD");
    method3(v7.clone());
    0i32
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    Rc::new(move || -> i32 {
        method0()
    })
}
pub fn eoie_test_typed_predicates() -> i32 {
    closure0()()
}
