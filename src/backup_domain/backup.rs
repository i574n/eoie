#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn backup_slot_for_0(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 < 0i32;
    if v2 {
        v1
    } else {
        v0
    }
}
fn backup_next_count_1(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 < 0i32;
    if v2 {
        let mut v3: i32 = v1.wrapping_add(1i32);
        v3
    } else {
        v1
    }
}
fn backup_should_capture_2(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v1 < 0i32;
    if v2 {
        0i32
    } else {
        let mut v3: bool = v0 < 0i32;
        if v3 {
            1i32
        } else {
            0i32
        }
    }
}
fn backup_binding_slot_3(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 < 0i32;
    if v2 {
        -1i32
    } else {
        let mut v3: bool = v1 < 0i32;
        if v3 {
            -1i32
        } else {
            v0
        }
    }
}
fn capture_authorized_4(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 < 0i32;
    if v2 {
        0i32
    } else {
        let mut v3: bool = v1 < 0i32;
        if v3 {
            0i32
        } else {
            let mut v4: bool = v1 < v0;
            if v4 {
                0i32
            } else {
                1i32
            }
        }
    }
}
fn capture_binding_slot_5(mut v0: i32, mut v1: i32) -> i32 {
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
fn promotion_slot_authorized_6(mut v0: i32, mut v1: i32) -> i32 {
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
fn rollback_next_8(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v1 < v0;
    if v2 {
        let mut v3: i32 = v0.wrapping_sub(v1);
        let mut v4: i32 = v3.wrapping_sub(1i32);
        v4
    } else {
        -1i32
    }
}
fn restoration_select_7(mut v0: i32, mut v1: i32) -> i32 {
    rollback_next_8(v0, v1)
}
fn restoration_bind_9(mut v0: i32, mut v1: i32) -> i32 {
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
                    v0
                }
            }
        }
    }
}
fn restoration_complete_10(mut v0: i32, mut v1: i32) -> i32 {
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
fn restoration_attempt_count_11(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 < 0i32;
    if v2 {
        -1i32
    } else {
        let mut v3: bool = v1 < 0i32;
        if v3 {
            -1i32
        } else {
            let mut v4: i32 = 2147483647i32.wrapping_sub(v0);
            let mut v5: bool = v4 < v1;
            if v5 {
                -1i32
            } else {
                let mut v6: i32 = v0.wrapping_add(v1);
                v6
            }
        }
    }
}
fn restoration_attempt_authorized_12(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 < 0i32;
    if v2 {
        0i32
    } else {
        let mut v3: bool = v1 < 0i32;
        if v3 {
            0i32
        } else {
            let mut v4: bool = v1 < v0;
            if v4 {
                1i32
            } else {
                0i32
            }
        }
    }
}
fn restoration_counter_next_13(mut v0: i32, mut v1: i32) -> i32 {
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
                let mut v5: bool = v0 < 2147483647i32;
                if v5 {
                    let mut v6: i32 = v0.wrapping_add(1i32);
                    v6
                } else {
                    -1i32
                }
            }
        }
    }
}
fn restoration_accumulator_outcome_14(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v1 < 0i32;
    if v2 {
        -1i32
    } else {
        let mut v3: bool = v0 < 0i32;
        if v3 {
            -1i32
        } else {
            let mut v4: bool = 1i32 < v0;
            if v4 {
                -1i32
            } else {
                let mut v5: bool = v0 < 1i32;
                if v5 {
                    0i32
                } else {
                    let mut v6: bool = v1 < 1i32;
                    if v6 {
                        1i32
                    } else {
                        2i32
                    }
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        backup_slot_for_0(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure1() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        backup_next_count_1(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure2() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        backup_should_capture_2(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure3() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        backup_binding_slot_3(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure4() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        capture_authorized_4(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure5() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        capture_binding_slot_5(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure6() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        promotion_slot_authorized_6(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure7() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        restoration_select_7(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure8() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        restoration_bind_9(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure9() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        restoration_complete_10(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure10() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        restoration_attempt_count_11(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure11() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        restoration_attempt_authorized_12(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure12() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        restoration_counter_next_13(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure13() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        restoration_accumulator_outcome_14(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_patch_backup_slot(v0: i32, v1: i32) -> i32 {
    closure0()(v0, v1)
}
pub fn eoie_patch_backup_next_count(v0: i32, v1: i32) -> i32 {
    closure1()(v0, v1)
}
pub fn eoie_patch_backup_should_capture(v0: i32, v1: i32) -> i32 {
    closure2()(v0, v1)
}
pub fn eoie_patch_backup_binding_slot(v0: i32, v1: i32) -> i32 {
    closure3()(v0, v1)
}
pub fn eoie_patch_capture_authorized(v0: i32, v1: i32) -> i32 {
    closure4()(v0, v1)
}
pub fn eoie_patch_capture_binding_slot(v0: i32, v1: i32) -> i32 {
    closure5()(v0, v1)
}
pub fn eoie_patch_promotion_slot_authorized(v0: i32, v1: i32) -> i32 {
    closure6()(v0, v1)
}
pub fn eoie_patch_restoration_select(v0: i32, v1: i32) -> i32 {
    closure7()(v0, v1)
}
pub fn eoie_patch_restoration_bind(v0: i32, v1: i32) -> i32 {
    closure8()(v0, v1)
}
pub fn eoie_patch_restoration_complete(v0: i32, v1: i32) -> i32 {
    closure9()(v0, v1)
}
pub fn eoie_patch_restoration_attempt_count(v0: i32, v1: i32) -> i32 {
    closure10()(v0, v1)
}
pub fn eoie_patch_restoration_attempt_authorized(v0: i32, v1: i32) -> i32 {
    closure11()(v0, v1)
}
pub fn eoie_patch_restoration_counter_next(v0: i32, v1: i32) -> i32 {
    closure12()(v0, v1)
}
pub fn eoie_patch_restoration_accumulator_outcome(v0: i32, v1: i32) -> i32 {
    closure13()(v0, v1)
}
