#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn patch_witness_authorized_1(mut v0: i32, mut v1: i32) -> i32 {
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
fn patch_driver_witness_binding_2(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 < 1i32;
    if v2 {
        -1i32
    } else {
        let mut v3: bool = 1i32 < v0;
        if v3 {
            -1i32
        } else {
            let mut v4: bool = v1 < 1i32;
            if v4 {
                -1i32
            } else {
                v1
            }
        }
    }
}
fn orchestration_witness_binding_0(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: i32 = patch_witness_authorized_1(v0, v1);
    let mut v3: i32 = patch_driver_witness_binding_2(v2, v1);
    let mut v4: bool = v3 < 0i32;
    if v4 {
        -1i32
    } else {
        v3
    }
}
fn patch_driver_operation_binding_4(mut v0: i32, mut v1: i32) -> i32 {
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
                    let mut v6: i32 = v0.wrapping_add(1i32);
                    v6
                }
            }
        }
    }
}
fn patch_operation_authorized_5(mut v0: i32, mut v1: i32) -> i32 {
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
fn orchestration_operation_binding_3(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: i32 = patch_operation_authorized_5(v0, v1);
    let mut v3: bool = v2 < 1i32;
    if v3 {
        -1i32
    } else {
        patch_driver_operation_binding_4(v0, v1)
    }
}
fn patch_guard_identity_7(mut v0: i32, mut v1: i32) -> i32 {
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
fn orchestration_guard_identity_6(mut v0: i32, mut v1: i32) -> i32 {
    patch_guard_identity_7(v0, v1)
}
fn patch_guard_needle_binding_9(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 < 0i32;
    if v2 {
        -1i32
    } else {
        let mut v3: bool = v1 < 1i32;
        if v3 {
            -1i32
        } else {
            v0
        }
    }
}
fn orchestration_guard_needle_binding_8(mut v0: i32, mut v1: i32) -> i32 {
    patch_guard_needle_binding_9(v0, v1)
}
fn patch_guard_match_binding_11(mut v0: i32, mut v1: i32) -> i32 {
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
fn patch_match_authorized_12(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v1 < 1i32;
    if v2 {
        0i32
    } else {
        let mut v3: bool = 1i32 < v1;
        if v3 {
            0i32
        } else {
            let mut v4: bool = v0 < 1i32;
            if v4 {
                0i32
            } else {
                let mut v5: bool = 1i32 < v0;
                if v5 {
                    0i32
                } else {
                    1i32
                }
            }
        }
    }
}
fn patch_guard_operation_binding_13(mut v0: i32, mut v1: i32) -> i32 {
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
fn orchestration_guard_match_binding_10(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: i32 = patch_guard_match_binding_11(v0, v1);
    let mut v3: i32 = 1i32;
    let mut v4: i32 = patch_match_authorized_12(v1, v3);
    let mut v5: i32 = patch_guard_operation_binding_13(v2, v4);
    let mut v6: bool = v5 < 0i32;
    if v6 {
        -1i32
    } else {
        v5
    }
}
fn patch_driver_stage_count_binding_15(mut v0: i32, mut v1: i32) -> i32 {
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
fn orchestration_stage_binding_14(mut v0: i32, mut v1: i32) -> i32 {
    patch_driver_stage_count_binding_15(v0, v1)
}
fn patch_driver_promotion_binding_17(mut v0: i32, mut v1: i32) -> i32 {
    patch_driver_operation_binding_4(v0, v1)
}
fn orchestration_promotion_binding_16(mut v0: i32, mut v1: i32) -> i32 {
    patch_driver_promotion_binding_17(v0, v1)
}
fn patch_driver_complete_binding_19(mut v0: i32, mut v1: i32) -> i32 {
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
fn patch_commit_authorized_20(mut v0: i32, mut v1: i32) -> i32 {
    patch_witness_authorized_1(v0, v1)
}
fn orchestration_complete_binding_18(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: i32 = patch_driver_complete_binding_19(v0, v1);
    let mut v3: i32 = patch_commit_authorized_20(v0, v1);
    let mut v4: bool = v2 < 1i32;
    if v4 {
        0i32
    } else {
        let mut v5: bool = v3 < 1i32;
        if v5 {
            0i32
        } else {
            1i32
        }
    }
}
fn closure0() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        orchestration_witness_binding_0(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure1() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        orchestration_operation_binding_3(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure2() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        orchestration_guard_identity_6(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure3() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        orchestration_guard_needle_binding_8(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure4() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        orchestration_guard_match_binding_10(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure5() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        orchestration_stage_binding_14(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure6() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        orchestration_promotion_binding_16(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure7() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        orchestration_complete_binding_18(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_patch_orchestration_witness_binding(v0: i32, v1: i32) -> i32 {
    closure0()(v0, v1)
}
pub fn eoie_patch_orchestration_operation_binding(v0: i32, v1: i32) -> i32 {
    closure1()(v0, v1)
}
pub fn eoie_patch_orchestration_guard_identity(v0: i32, v1: i32) -> i32 {
    closure2()(v0, v1)
}
pub fn eoie_patch_orchestration_guard_needle_binding(v0: i32, v1: i32) -> i32 {
    closure3()(v0, v1)
}
pub fn eoie_patch_orchestration_guard_match_binding(v0: i32, v1: i32) -> i32 {
    closure4()(v0, v1)
}
pub fn eoie_patch_orchestration_stage_binding(v0: i32, v1: i32) -> i32 {
    closure5()(v0, v1)
}
pub fn eoie_patch_orchestration_promotion_binding(v0: i32, v1: i32) -> i32 {
    closure6()(v0, v1)
}
pub fn eoie_patch_orchestration_complete_binding(v0: i32, v1: i32) -> i32 {
    closure7()(v0, v1)
}
