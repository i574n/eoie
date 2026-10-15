#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)] #![recursion_limit = "256"]
#![allow(clippy::needless_late_init)]
use std::cell::RefCell;
use std::rc::Rc;
fn bundle_rehydration_binding_1(mut v0: i32, mut v1: i32) -> i32 {
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
fn bundle_rehydrate_preflight_binding_0(mut v0: i32, mut v1: i32) -> i32 {
    bundle_rehydration_binding_1(v0, v1)
}
fn bundle_rehydrate_extract_binding_2(mut v0: i32, mut v1: i32) -> i32 {
    bundle_rehydration_binding_1(v0, v1)
}
fn bundle_rehydrate_validate_binding_3(mut v0: i32, mut v1: i32) -> i32 {
    bundle_rehydration_binding_1(v0, v1)
}
fn bundle_complete_binding_5(mut v0: i32, mut v1: i32) -> i32 {
    bundle_rehydration_binding_1(v0, v1)
}
fn bundle_rehydrate_receipt_binding_4(mut v0: i32, mut v1: i32) -> i32 {
    bundle_complete_binding_5(v0, v1)
}
fn archive_extract_preflight_binding_6(mut v0: i32, mut v1: i32) -> i32 {
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
fn archive_extract_process_binding_7(mut v0: i32, mut v1: i32) -> i32 {
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
fn bundle_root_binding_11(mut v0: i32, mut v1: i32) -> i32 {
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
fn bundle_entry_binding_10(mut v0: i32, mut v1: i32) -> i32 {
    bundle_root_binding_11(v0, v1)
}
fn bundle_extract_binding_9(mut v0: i32, mut v1: i32) -> i32 {
    bundle_entry_binding_10(v0, v1)
}
fn archive_extract_complete_binding_8(mut v0: i32, mut v1: i32) -> i32 {
    bundle_extract_binding_9(v0, v1)
}
fn archive_extract_receipt_binding_12(mut v0: i32, mut v1: i32) -> i32 {
    bundle_complete_binding_5(v0, v1)
}
fn preflight_binding_14(mut v0: i32, mut v1: i32) -> i32 {
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
fn bundle_output_preflight_binding_13(mut v0: i32, mut v1: i32) -> i32 {
    preflight_binding_14(v0, v1)
}
fn complete_binding_16(mut v0: i32, mut v1: i32) -> i32 {
    preflight_binding_14(v0, v1)
}
fn bundle_output_complete_binding_15(mut v0: i32, mut v1: i32) -> i32 {
    complete_binding_16(v0, v1)
}
fn preflight_binding_18(mut v0: i32, mut v1: i32) -> i32 {
    preflight_binding_14(v0, v1)
}
fn bundle_verify_preflight_binding_17(mut v0: i32, mut v1: i32) -> i32 {
    preflight_binding_18(v0, v1)
}
fn complete_binding_20(mut v0: i32, mut v1: i32) -> i32 {
    complete_binding_16(v0, v1)
}
fn bundle_verify_complete_binding_19(mut v0: i32, mut v1: i32) -> i32 {
    complete_binding_20(v0, v1)
}
fn closure0() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        bundle_root_binding_11(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn coverage_checkpoint_binding_21(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 < 0i32;
    if v2 {
        0i32
    } else {
        let mut v3: bool = 1i32 < v0;
        if v3 {
            0i32
        } else {
            let mut v4: bool = v1 < 0i32;
            if v4 {
                0i32
            } else {
                let mut v5: bool = 2i32 < v1;
                if v5 {
                    0i32
                } else {
                    let mut v6: bool = v0 < 1i32;
                    if v6 {
                        1i32
                    } else {
                        let mut v7: bool = v1 < 2i32;
                        if v7 {
                            0i32
                        } else {
                            1i32
                        }
                    }
                }
            }
        }
    }
}
fn closure1() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        coverage_checkpoint_binding_21(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure2() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        bundle_entry_binding_10(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn bundle_size_binding_22(mut v0: i32, mut v1: i32) -> i32 {
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
                1i32
            }
        }
    }
}
fn closure3() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        bundle_size_binding_22(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure4() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        bundle_extract_binding_9(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn bundle_payload_count_binding_23(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 < 1i32;
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
                let mut v5: i32 = v0.wrapping_sub(v1);
                v5
            }
        }
    }
}
fn closure5() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        bundle_payload_count_binding_23(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure6() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        bundle_rehydration_binding_1(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure7() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        bundle_complete_binding_5(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure8() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        bundle_rehydrate_preflight_binding_0(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure9() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        bundle_rehydrate_extract_binding_2(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure10() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        bundle_rehydrate_validate_binding_3(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure11() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        bundle_rehydrate_receipt_binding_4(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure12() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        archive_extract_preflight_binding_6(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure13() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        archive_extract_process_binding_7(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure14() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        archive_extract_complete_binding_8(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure15() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        archive_extract_receipt_binding_12(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure16() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        bundle_output_preflight_binding_13(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure17() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        bundle_output_complete_binding_15(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure18() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        bundle_verify_preflight_binding_17(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure19() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        bundle_verify_complete_binding_19(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_bundle_root_binding(v0: i32, v1: i32) -> i32 {
    closure0()(v0, v1)
}
pub fn eoie_bundle_coverage_checkpoint_binding(v0: i32, v1: i32) -> i32 {
    closure1()(v0, v1)
}
pub fn eoie_bundle_entry_binding(v0: i32, v1: i32) -> i32 {
    closure2()(v0, v1)
}
pub fn eoie_bundle_size_binding(v0: i32, v1: i32) -> i32 {
    closure3()(v0, v1)
}
pub fn eoie_bundle_extract_binding(v0: i32, v1: i32) -> i32 {
    closure4()(v0, v1)
}
pub fn eoie_bundle_payload_count_binding(v0: i32, v1: i32) -> i32 {
    closure5()(v0, v1)
}
pub fn eoie_bundle_rehydration_binding(v0: i32, v1: i32) -> i32 {
    closure6()(v0, v1)
}
pub fn eoie_bundle_complete_binding(v0: i32, v1: i32) -> i32 {
    closure7()(v0, v1)
}
pub fn eoie_bundle_rehydrate_preflight_binding(v0: i32, v1: i32) -> i32 {
    closure8()(v0, v1)
}
pub fn eoie_bundle_rehydrate_extract_binding(v0: i32, v1: i32) -> i32 {
    closure9()(v0, v1)
}
pub fn eoie_bundle_rehydrate_validate_binding(v0: i32, v1: i32) -> i32 {
    closure10()(v0, v1)
}
pub fn eoie_bundle_rehydrate_receipt_binding(v0: i32, v1: i32) -> i32 {
    closure11()(v0, v1)
}
pub fn eoie_archive_extract_preflight_binding(v0: i32, v1: i32) -> i32 {
    closure12()(v0, v1)
}
pub fn eoie_archive_extract_process_binding(v0: i32, v1: i32) -> i32 {
    closure13()(v0, v1)
}
pub fn eoie_archive_extract_complete_binding(v0: i32, v1: i32) -> i32 {
    closure14()(v0, v1)
}
pub fn eoie_archive_extract_receipt_binding(v0: i32, v1: i32) -> i32 {
    closure15()(v0, v1)
}
pub fn eoie_bundle_output_preflight_binding(v0: i32, v1: i32) -> i32 {
    closure16()(v0, v1)
}
pub fn eoie_bundle_output_complete_binding(v0: i32, v1: i32) -> i32 {
    closure17()(v0, v1)
}
pub fn eoie_bundle_verify_preflight_binding(v0: i32, v1: i32) -> i32 {
    closure18()(v0, v1)
}
pub fn eoie_bundle_verify_complete_binding(v0: i32, v1: i32) -> i32 {
    closure19()(v0, v1)
}
