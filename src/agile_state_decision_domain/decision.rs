#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn series_sequence_binding_1(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 < 0i32;
    if v2 {
        0i32
    } else {
        let mut v3: i32 = v0.wrapping_add(1i32);
        let mut v4: bool = v3 == v1;
        if v4 {
            1i32
        } else {
            0i32
        }
    }
}
fn agile_series_sequence_binding_0(mut v0: i32, mut v1: i32) -> i32 {
    series_sequence_binding_1(v0, v1)
}
fn closure0() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        agile_series_sequence_binding_0(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn series_source_binding_3(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 < 1i32;
    if v2 {
        0i32
    } else {
        let mut v3: bool = v1 == 32i32;
        if v3 {
            1i32
        } else {
            0i32
        }
    }
}
fn agile_series_source_binding_2(mut v0: i32, mut v1: i32) -> i32 {
    series_source_binding_3(v0, v1)
}
fn closure1() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        agile_series_source_binding_2(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn series_pair_binding_5(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 == 1i32;
    if v2 {
        let mut v3: bool = v1 == 1i32;
        if v3 {
            1i32
        } else {
            0i32
        }
    } else {
        0i32
    }
}
fn agile_series_pair_binding_4(mut v0: i32, mut v1: i32) -> i32 {
    series_pair_binding_5(v0, v1)
}
fn closure2() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        agile_series_pair_binding_4(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn series_monotonic_binding_7(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 < 0i32;
    if v2 {
        0i32
    } else {
        let mut v3: bool = v1 < v0;
        if v3 {
            0i32
        } else {
            1i32
        }
    }
}
fn agile_series_monotonic_binding_6(mut v0: i32, mut v1: i32) -> i32 {
    series_monotonic_binding_7(v0, v1)
}
fn closure3() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        agile_series_monotonic_binding_6(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn series_fingerprint_binding_9(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 == 1i32;
    if v2 {
        let mut v3: bool = v1 == 1i32;
        if v3 {
            1i32
        } else {
            0i32
        }
    } else {
        0i32
    }
}
fn agile_series_fingerprint_binding_8(mut v0: i32, mut v1: i32) -> i32 {
    series_fingerprint_binding_9(v0, v1)
}
fn closure4() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        agile_series_fingerprint_binding_8(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_state_entry_kind_binding_10(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 == 1i32;
    if v2 {
        let mut v3: bool = v1 == 1i32;
        if v3 {
            0i32
        } else {
            1i32
        }
    } else {
        let mut v5: bool = v1 == 1i32;
        if v5 {
            2i32
        } else {
            0i32
        }
    }
}
fn closure5() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        agile_state_entry_kind_binding_10(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_transient_state_name_code_11(mut v0: Rc<str>) -> u64 {
    let mut v1: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("patch_resume.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = if v1 {
        true
    } else {
        let mut v2: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("toolchain_process_receipt.spi"); } LIT.with(|lit| lit.clone()) };
        v2
    };
    if v3 {
        1u64
    } else {
        0u64
    }
}
fn closure6() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        agile_transient_state_name_code_11(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_record_required_name_bit_12(mut v0: Rc<str>) -> u64 {
    let mut v1: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("history.spi"); } LIT.with(|lit| lit.clone()) };
    if v1 {
        1u64
    } else {
        let mut v2: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ratings.spi"); } LIT.with(|lit| lit.clone()) };
        if v2 {
            2u64
        } else {
            let mut v3: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bench.spi"); } LIT.with(|lit| lit.clone()) };
            if v3 {
                4u64
            } else {
                let mut v4: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("migration.spi"); } LIT.with(|lit| lit.clone()) };
                if v4 {
                    8u64
                } else {
                    0u64
                }
            }
        }
    }
}
fn closure7() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        agile_record_required_name_bit_12(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_record_required_catalog_13(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("v1"); } LIT.with(|lit| lit.clone()) };
    if v1 {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("history.spi|ratings.spi|bench.spi|migration.spi"); } LIT.with(|lit| lit.clone()) };
        v2.clone()
    } else {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v3.clone()
    }
}
fn closure8() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        agile_record_required_catalog_13(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_record_preflight_binding_14(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 < 0i32;
    let mut v4: bool = if v2 {
        true
    } else {
        let mut v3: bool = 15i32 < v0;
        v3
    };
    let mut v6: bool = if v4 {
        true
    } else {
        let mut v5: bool = v1 < 0i32;
        v5
    };
    let mut v8: bool = if v6 {
        true
    } else {
        let mut v7: bool = 0i32 < v1;
        v7
    };
    if v8 {
        -1i32
    } else {
        let mut v9: bool = v0 == 15i32;
        if v9 {
            1i32
        } else {
            0i32
        }
    }
}
fn closure9() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        agile_record_preflight_binding_14(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_attestation_decision_binding_15(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v1 < 0i32;
    let mut v4: bool = if v2 {
        true
    } else {
        let mut v3: bool = 7i32 < v1;
        v3
    };
    let mut v6: bool = if v4 {
        true
    } else {
        let mut v5: bool = v1 == 4i32;
        v5
    };
    let mut v8: bool = if v6 {
        true
    } else {
        let mut v7: bool = v1 == 5i32;
        v7
    };
    if v8 {
        0i32
    } else {
        let mut v9: bool = v0 == 2i32;
        if v9 {
            let mut v10: bool = v1 == 1i32;
            let mut v12: bool = if v10 {
                true
            } else {
                let mut v11: bool = v1 == 3i32;
                v11
            };
            let mut v14: bool = if v12 {
                true
            } else {
                let mut v13: bool = v1 == 7i32;
                v13
            };
            if v14 {
                1i32
            } else {
                2i32
            }
        } else {
            let mut v16: bool = v0 == 1i32;
            if v16 {
                let mut v17: bool = v1 == 7i32;
                let mut v19: bool = if v17 {
                    true
                } else {
                    let mut v18: bool = v1 == 1i32;
                    v18
                };
                if v19 {
                    1i32
                } else {
                    let mut v20: bool = v1 == 2i32;
                    let mut v22: bool = if v20 {
                        true
                    } else {
                        let mut v21: bool = v1 == 3i32;
                        v21
                    };
                    let mut v24: bool = if v22 {
                        true
                    } else {
                        let mut v23: bool = v1 == 6i32;
                        v23
                    };
                    if v24 {
                        3i32
                    } else {
                        let mut v25: bool = v1 == 0i32;
                        if v25 {
                            4i32
                        } else {
                            0i32
                        }
                    }
                }
            } else {
                0i32
            }
        }
    }
}
fn closure10() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        agile_attestation_decision_binding_15(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_state_series_sequence_binding(v0: i32, v1: i32) -> i32 {
    closure0()(v0, v1)
}
pub fn eoie_agile_state_series_source_binding(v0: i32, v1: i32) -> i32 {
    closure1()(v0, v1)
}
pub fn eoie_agile_state_series_pair_binding(v0: i32, v1: i32) -> i32 {
    closure2()(v0, v1)
}
pub fn eoie_agile_state_series_monotonic_binding(v0: i32, v1: i32) -> i32 {
    closure3()(v0, v1)
}
pub fn eoie_agile_state_series_fingerprint_binding(v0: i32, v1: i32) -> i32 {
    closure4()(v0, v1)
}
pub fn eoie_agile_state_entry_kind_binding(v0: i32, v1: i32) -> i32 {
    closure5()(v0, v1)
}
pub fn eoie_agile_state_transient_name_code(v0: &str) -> u64 {
    closure6()(Rc::<str>::from(v0))
}
pub fn eoie_agile_state_record_required_name_bit(v0: &str) -> u64 {
    closure7()(Rc::<str>::from(v0))
}
pub fn eoie_agile_state_record_required_catalog(v0: &str) -> Rc<str> {
    closure8()(Rc::<str>::from(v0))
}
pub fn eoie_agile_state_record_preflight_binding(v0: i32, v1: i32) -> i32 {
    closure9()(v0, v1)
}
pub fn eoie_agile_state_attestation_decision_binding(v0: i32, v1: i32) -> i32 {
    closure10()(v0, v1)
}
