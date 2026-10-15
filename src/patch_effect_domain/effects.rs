#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn patch_read_limit_binding_0(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 < 1i32;
    if v2 {
        -1i32
    } else {
        let mut v3: bool = v1 < 0i32;
        if v3 {
            -1i32
        } else {
            let mut v4: bool = v0 < v1;
            if v4 {
                -1i32
            } else {
                v1
            }
        }
    }
}
fn patch_read_receipt_binding_1(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 < 0i32;
    if v2 {
        -1i32
    } else {
        let mut v3: bool = v1 < 0i32;
        if v3 {
            -1i32
        } else {
            let mut v4: bool = v0 < v1;
            if v4 {
                -1i32
            } else {
                let mut v5: bool = v1 < v0;
                if v5 {
                    -1i32
                } else {
                    v1
                }
            }
        }
    }
}
fn patch_stage_identity_2(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 < 0i32;
    if v2 {
        -1i32
    } else {
        let mut v3: bool = v1 < 0i32;
        if v3 {
            -1i32
        } else {
            v1
        }
    }
}
fn patch_write_bytes_authorized_3(mut v0: i32, mut v1: i32) -> i32 {
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
fn patch_write_slot_binding_4(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 < 0i32;
    if v2 {
        -1i32
    } else {
        let mut v3: bool = v1 < 1i32;
        if v3 {
            -1i32
        } else {
            let mut v4: bool = 1i32 < v1;
            if v4 {
                -1i32
            } else {
                v0
            }
        }
    }
}
fn patch_rename_stage_authorized_5(mut v0: i32, mut v1: i32) -> i32 {
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
fn patch_rename_slot_authorized_6(mut v0: i32, mut v1: i32) -> i32 {
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
fn patch_authority_pair_7(mut v0: i32, mut v1: i32) -> i32 {
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
fn patch_rename_slot_binding_8(mut v0: i32, mut v1: i32) -> i32 {
    patch_write_slot_binding_4(v0, v1)
}
fn patch_remove_receipt_binding_9(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 < 0i32;
    if v2 {
        -1i32
    } else {
        let mut v3: bool = v1 < 1i32;
        if v3 {
            -1i32
        } else {
            let mut v4: bool = 1i32 < v1;
            if v4 {
                -1i32
            } else {
                v0
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        patch_read_limit_binding_0(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure1() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        patch_read_receipt_binding_1(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure2() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        patch_stage_identity_2(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure3() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        patch_write_bytes_authorized_3(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure4() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        patch_write_slot_binding_4(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure5() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        patch_rename_stage_authorized_5(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure6() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        patch_rename_slot_authorized_6(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure7() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        patch_authority_pair_7(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure8() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        patch_rename_slot_binding_8(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure9() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        patch_remove_receipt_binding_9(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_patch_read_limit_binding(v0: i32, v1: i32) -> i32 {
    closure0()(v0, v1)
}
pub fn eoie_patch_read_receipt_binding(v0: i32, v1: i32) -> i32 {
    closure1()(v0, v1)
}
pub fn eoie_patch_stage_identity(v0: i32, v1: i32) -> i32 {
    closure2()(v0, v1)
}
pub fn eoie_patch_write_bytes_authorized(v0: i32, v1: i32) -> i32 {
    closure3()(v0, v1)
}
pub fn eoie_patch_write_slot_binding(v0: i32, v1: i32) -> i32 {
    closure4()(v0, v1)
}
pub fn eoie_patch_rename_stage_authorized(v0: i32, v1: i32) -> i32 {
    closure5()(v0, v1)
}
pub fn eoie_patch_rename_slot_authorized(v0: i32, v1: i32) -> i32 {
    closure6()(v0, v1)
}
pub fn eoie_patch_authority_pair(v0: i32, v1: i32) -> i32 {
    closure7()(v0, v1)
}
pub fn eoie_patch_rename_slot_binding(v0: i32, v1: i32) -> i32 {
    closure8()(v0, v1)
}
pub fn eoie_patch_remove_receipt_binding(v0: i32, v1: i32) -> i32 {
    closure9()(v0, v1)
}
