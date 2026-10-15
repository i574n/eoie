#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn authority_state_binding_0(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v1 == 0i32;
    if v2 {
        let mut v3: bool = v0 == 0i32;
        if v3 {
            1i32
        } else {
            0i32
        }
    } else {
        -1i32
    }
}
fn authority_live_mask_binding_1(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v1 == 0i32;
    if v2 {
        let mut v3: bool = v0 < 0i32;
        if v3 {
            -1i32
        } else {
            let mut v4: bool = 31i32 < v0;
            if v4 {
                -1i32
            } else {
                let mut v5: i32 = 31i32.wrapping_sub(v0);
                v5
            }
        }
    } else {
        -1i32
    }
}
fn authority_declared_mask_binding_2(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 == 0i32;
    if v2 {
        let mut v3: bool = v1 == 0i32;
        if v3 {
            0i32
        } else {
            -1i32
        }
    } else {
        -1i32
    }
}
fn authority_blocker_bit_binding_3(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v1 == 0i32;
    if v2 {
        let mut v3: bool = v0 == 0i32;
        if v3 {
            1i32
        } else {
            let mut v4: bool = v0 == 1i32;
            if v4 {
                2i32
            } else {
                let mut v5: bool = v0 == 2i32;
                if v5 {
                    4i32
                } else {
                    let mut v6: bool = v0 == 3i32;
                    if v6 {
                        8i32
                    } else {
                        let mut v7: bool = v0 == 4i32;
                        if v7 {
                            16i32
                        } else {
                            -1i32
                        }
                    }
                }
            }
        }
    } else {
        -1i32
    }
}
fn authority_blocker_labels_4(mut v0: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("v1"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("semantic-closeout"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = v1.split("|").zip(v2.split(char::from(10u8))).find_map(|(key,item)| if key == &*v0 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cold-proof-v4"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = v1.split("|").zip(v4.split(char::from(10u8))).find_map(|(key,item)| if key == &*v0 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("differential-catalog"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = v1.split("|").zip(v6.split(char::from(10u8))).find_map(|(key,item)| if key == &*v0 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("family-contracts"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = v1.split("|").zip(v8.split(char::from(10u8))).find_map(|(key,item)| if key == &*v0 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cold-rebuild"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = v1.split("|").zip(v10.split(char::from(10u8))).find_map(|(key,item)| if key == &*v0 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
    (v3.clone(), v5.clone(), v7.clone(), v9.clone(), v11.clone())
}
fn closure0() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        authority_state_binding_0(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure1() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        authority_declared_mask_binding_2(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure2() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        authority_live_mask_binding_1(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure3() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        authority_blocker_bit_binding_3(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure4() -> Rc<dyn Fn(Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = Rc::new(move |mut v0: Rc<str>| -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
        authority_blocker_labels_4(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_authority_state(v0: i32, v1: i32) -> i32 {
    closure0()(v0, v1)
}
pub fn eoie_authority_declared_mask(v0: i32, v1: i32) -> i32 {
    closure1()(v0, v1)
}
pub fn eoie_authority_live_mask(v0: i32, v1: i32) -> i32 {
    closure2()(v0, v1)
}
pub fn eoie_authority_blocker_bit(v0: i32, v1: i32) -> i32 {
    closure3()(v0, v1)
}
pub fn eoie_authority_blocker_labels(v0: &str) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    closure4()(Rc::<str>::from(v0))
}
