#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)] #![recursion_limit = "256"]
use std::cell::RefCell;
use std::rc::Rc;
fn string_slice(value: &str, from: i64, to: i64) -> Rc<str> {
    let bytes = value.as_bytes();
    let length = bytes.len() as i64;
    if from < 0 || from > length || to < from - 1 || to >= length { std::process::abort(); }
    if to < from { return { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }; }
    if (bytes[from as usize] & 0xC0) == 0x80 || (to + 1 < length && (bytes[(to + 1) as usize] & 0xC0) == 0x80) { std::process::exit(3); }
    let slice = &bytes[from as usize..(to + 1) as usize];
    match std::str::from_utf8(slice) { Ok(text) => Rc::<str>::from(text), Err(error) => Rc::<str>::from(std::str::from_utf8(&slice[..error.valid_up_to()]).unwrap_or("")) }
}
fn agile_state_mutation_target_decision_binding_0(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 == 1i32;
    if v2 {
        let mut v3: bool = v1 == 1i32;
        if v3 {
            2i32
        } else {
            1i32
        }
    } else {
        0i32
    }
}
fn closure0() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        agile_state_mutation_target_decision_binding_0(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_lease_budget_decision_binding_1(mut v0: i32, mut v1: i32) -> i32 {
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
fn closure1() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        agile_lease_budget_decision_binding_1(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_lease_readback_decision_binding_2(mut v0: i32, mut v1: i32) -> i32 {
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
fn closure2() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        agile_lease_readback_decision_binding_2(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_prompt_title_code_3(mut v0: Rc<str>) -> u64 {
    let mut v1: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v1 {
        0u64
    } else {
        1u64
    }
}
fn closure3() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        agile_prompt_title_code_3(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_status_name_code_4(mut v0: Rc<str>) -> u64 {
    let mut v1: u64 = u64::MAX;
    let mut v2: bool = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planned"); } LIT.with(|lit| lit.clone()) } == v0.clone();
    if v2 {
        let mut v3: u64 = 0i32 as u64;
        v3
    } else {
        let mut v4: bool = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Active"); } LIT.with(|lit| lit.clone()) } == v0.clone();
        if v4 {
            let mut v5: u64 = 1i32 as u64;
            v5
        } else {
            let mut v6: bool = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Blocked"); } LIT.with(|lit| lit.clone()) } == v0.clone();
            if v6 {
                let mut v7: u64 = 2i32 as u64;
                v7
            } else {
                let mut v8: bool = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Paused"); } LIT.with(|lit| lit.clone()) } == v0.clone();
                if v8 {
                    let mut v9: u64 = 3i32 as u64;
                    v9
                } else {
                    let mut v10: bool = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Done"); } LIT.with(|lit| lit.clone()) } == v0.clone();
                    if v10 {
                        let mut v11: u64 = 4i32 as u64;
                        v11
                    } else {
                        v1
                    }
                }
            }
        }
    }
}
fn closure4() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        agile_status_name_code_4(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method6(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let mut v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let mut v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                let mut v12: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1, v12);
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method7(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let mut v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let mut v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                (v0, v1) = (v0.clone(), v3);
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn agile_task_line_candidate_code_5(mut v0: Rc<str>) -> u64 {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = 0i32;
    let mut v3: i32 = method6(v0.clone(), v1, v2);
    let mut v4: i32 = v1.wrapping_sub(1i32);
    let mut v5: Rc<str> = string_slice(&v0.clone(), v3 as i64, v4 as i64);
    let mut v6: i32 = (v5.clone().len() as i32);
    let mut v7: i32 = method7(v5.clone(), v6);
    let mut v8: Rc<str> = string_slice(&v5.clone(), 0i32 as i64, v7 as i64);
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let mut v10: bool = v8.starts_with(&*v9);
    if v10 {
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
        let mut v12: bool = v8.contains(&*v11);
        if v12 {
            1u64
        } else {
            0u64
        }
    } else {
        0u64
    }
}
fn closure5() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        agile_task_line_candidate_code_5(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn receipt_row_kind_code_9(mut v0: Rc<str>) -> u64 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("snapshot"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0 == v1 ;
    if v2 {
        1u64
    } else {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("module"); } LIT.with(|lit| lit.clone()) };
        let mut v4: bool = v0 == v3 ;
        if v4 {
            2u64
        } else {
            0u64
        }
    }
}
fn agile_receipt_row_kind_code_8(mut v0: Rc<str>) -> u64 {
    receipt_row_kind_code_9(v0.clone())
}
fn closure6() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        agile_receipt_row_kind_code_8(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn receipt_row_kind_text_11(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0 == v1 ;
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("snapshot"); } LIT.with(|lit| lit.clone()) };
        v3.clone()
    } else {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("2"); } LIT.with(|lit| lit.clone()) };
        let mut v5: bool = v0 == v4 ;
        if v5 {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("module"); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        } else {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v7.clone()
        }
    }
}
fn agile_receipt_row_kind_text_10(mut v0: Rc<str>) -> Rc<str> {
    receipt_row_kind_text_11(v0.clone())
}
fn closure7() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        agile_receipt_row_kind_text_10(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn starts_with_loop_14(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: i32) -> bool {
    loop {
        let mut v5: bool = v3 == v4 ;
        if v5 {
            return true;
        } else {
            let mut v6: u8 = v0.clone().as_bytes()[v2 as usize];
            let mut v7: u8 = v1.clone().as_bytes()[v3 as usize];
            let mut v8: bool = v6 == v7 ;
            if v8 {
                let mut v9: i32 = v2 + 1i32 ;
                let mut v10: i32 = v3 + 1i32 ;
                (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v9, v10, v4);
                continue;
            } else {
                return false;
            }
        }
    }
}
fn receipt_attestation_kind_code_13(mut v0: Rc<str>) -> u64 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("runtime"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0 == v1 ;
    if v2 {
        1u64
    } else {
        let mut v3: i32 = (v0.clone().len() as i32);
        let mut v4: bool = v3 < 6i32;
        let mut v10: bool = if v4 {
            false
        } else {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("build:"); } LIT.with(|lit| lit.clone()) };
            let mut v6: i32 = 0i32;
            let mut v7: i32 = 0i32;
            let mut v8: i32 = 6i32;
            starts_with_loop_14(v0.clone(), v5.clone(), v6, v7, v8)
        };
        if v10 {
            2u64
        } else {
            0u64
        }
    }
}
fn agile_receipt_attestation_kind_code_12(mut v0: Rc<str>) -> u64 {
    receipt_attestation_kind_code_13(v0.clone())
}
fn closure8() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        agile_receipt_attestation_kind_code_12(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn receipt_attestation_text_16(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0 == v1 ;
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("runtime"); } LIT.with(|lit| lit.clone()) };
        v3.clone()
    } else {
        let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("build:"); } LIT.with(|lit| lit.clone()) }, v0.clone()));
        v4.clone()
    }
}
fn agile_receipt_attestation_text_15(mut v0: Rc<str>) -> Rc<str> {
    receipt_attestation_text_16(v0.clone())
}
fn closure9() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        agile_receipt_attestation_text_15(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_prompt_lease_status_code_17(mut v0: Rc<str>) -> u64 {
    let mut v1: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("LeaseActive"); } LIT.with(|lit| lit.clone()) };
    if v1 {
        1u64
    } else {
        0u64
    }
}
fn closure10() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        agile_prompt_lease_status_code_17(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_prompt_lease_marker_text_18(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
    if v1 {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("PromptLease ("); } LIT.with(|lit| lit.clone()) };
        v2.clone()
    } else {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v3.clone()
    }
}
fn closure11() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        agile_prompt_lease_marker_text_18(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_prompt_lease_shape_binding_19(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 == 7i32;
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
fn closure12() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        agile_prompt_lease_shape_binding_19(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_lease_clock_phase_binding_20(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v1 == 1i32;
    if v2 {
        2i32
    } else {
        let mut v3: bool = v0 == 1i32;
        if v3 {
            1i32
        } else {
            0i32
        }
    }
}
fn closure13() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        agile_lease_clock_phase_binding_20(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_lease_policy_from_text_21(mut v0: Rc<str>) -> u64 {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = 0i32;
    let mut v3: i32 = method6(v0.clone(), v1, v2);
    let mut v4: i32 = v1.wrapping_sub(1i32);
    let mut v5: Rc<str> = string_slice(&v0.clone(), v3 as i64, v4 as i64);
    let mut v6: i32 = (v5.clone().len() as i32);
    let mut v7: i32 = method7(v5.clone(), v6);
    let mut v8: Rc<str> = string_slice(&v5.clone(), 0i32 as i64, v7 as i64);
    let mut v9: bool = v8.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("renewing"); } LIT.with(|lit| lit.clone()) };
    if v9 {
        1u64
    } else {
        0u64
    }
}
fn closure14() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        agile_lease_policy_from_text_21(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_lease_effective_phase_binding_22(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 == 1i32;
    if v2 {
        0i32
    } else {
        v1
    }
}
fn closure15() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        agile_lease_effective_phase_binding_22(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_status_text_23(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((0i32).to_string());
    let mut v2: bool = v0.clone() == v1.clone();
    let mut v5: Rc<str> = if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planned"); } LIT.with(|lit| lit.clone()) };
        v3.clone()
    } else {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v4.clone()
    };
    let mut v6: Rc<str> = std::rc::Rc::<str>::from((1i32).to_string());
    let mut v7: bool = v0.clone() == v6.clone();
    let mut v9: Rc<str> = if v7 {
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Active"); } LIT.with(|lit| lit.clone()) };
        v8.clone()
    } else {
        v5.clone()
    };
    let mut v10: Rc<str> = std::rc::Rc::<str>::from((2i32).to_string());
    let mut v11: bool = v0.clone() == v10.clone();
    let mut v13: Rc<str> = if v11 {
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Blocked"); } LIT.with(|lit| lit.clone()) };
        v12.clone()
    } else {
        v9.clone()
    };
    let mut v14: Rc<str> = std::rc::Rc::<str>::from((3i32).to_string());
    let mut v15: bool = v0.clone() == v14.clone();
    let mut v17: Rc<str> = if v15 {
        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Paused"); } LIT.with(|lit| lit.clone()) };
        v16.clone()
    } else {
        v13.clone()
    };
    let mut v18: Rc<str> = std::rc::Rc::<str>::from((4i32).to_string());
    let mut v19: bool = v0.clone() == v18.clone();
    if v19 {
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Done"); } LIT.with(|lit| lit.clone()) };
        v20.clone()
    } else {
        v17.clone()
    }
}
fn closure16() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        agile_status_text_23(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_spi_escape_text_24(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v3.replace(&*v4, &*v5));
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from(v6.replace(&*v7, &*v8));
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = Rc::<str>::from(v9.replace(&*v10, &*v11));
    v12.clone()
}
fn closure17() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        agile_spi_escape_text_24(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_state_package_line_code_25(mut v0: Rc<str>) -> u64 {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = 0i32;
    let mut v3: i32 = method6(v0.clone(), v1, v2);
    let mut v4: i32 = v1.wrapping_sub(1i32);
    let mut v5: Rc<str> = string_slice(&v0.clone(), v3 as i64, v4 as i64);
    let mut v6: i32 = (v5.clone().len() as i32);
    let mut v7: i32 = method7(v5.clone(), v6);
    let mut v8: Rc<str> = string_slice(&v5.clone(), 0i32 as i64, v7 as i64);
    let mut v9: bool = v8.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:"); } LIT.with(|lit| lit.clone()) };
    if v9 {
        1u64
    } else {
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let mut v11: bool = v0.starts_with(&*v10);
        let mut v14: bool = if v11 {
            true
        } else {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let mut v13: bool = v0.starts_with(&*v12);
            v13
        };
        if v14 {
            let mut v15: bool = v8.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v18: bool = if v15 {
                true
            } else {
                let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("//"); } LIT.with(|lit| lit.clone()) };
                let mut v17: bool = v8.starts_with(&*v16);
                v17
            };
            if v18 {
                0u64
            } else {
                2u64
            }
        } else {
            3u64
        }
    }
}
fn closure18() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        agile_state_package_line_code_25(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_27(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(1i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0.clone(), 0i32 as i64, v5 as i64);
            v0 = v6.clone();
            continue;
        } else {
            return v0.clone();
        }
    }
}
fn loop_28(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(1i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0.clone(), 0i32 as i64, v5 as i64);
            v0 = v6.clone();
            continue;
        } else {
            return v0.clone();
        }
    }
}
fn agile_state_package_module_name_26(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = 0i32;
    let mut v3: i32 = method6(v0.clone(), v1, v2);
    let mut v4: i32 = v1.wrapping_sub(1i32);
    let mut v5: Rc<str> = string_slice(&v0.clone(), v3 as i64, v4 as i64);
    let mut v6: i32 = (v5.clone().len() as i32);
    let mut v7: i32 = method7(v5.clone(), v6);
    let mut v8: Rc<str> = string_slice(&v5.clone(), 0i32 as i64, v7 as i64);
    let mut v9: Rc<str> = loop_27(v8.clone());
    let mut v10: Rc<str> = loop_28(v9.clone());
    let mut v11: i32 = (v10.clone().len() as i32);
    let mut v12: i32 = 0i32;
    let mut v13: i32 = method6(v10.clone(), v11, v12);
    let mut v14: i32 = v11.wrapping_sub(1i32);
    let mut v15: Rc<str> = string_slice(&v10.clone(), v13 as i64, v14 as i64);
    let mut v16: i32 = (v15.clone().len() as i32);
    let mut v17: i32 = method7(v15.clone(), v16);
    let mut v18: Rc<str> = string_slice(&v15.clone(), 0i32 as i64, v17 as i64);
    let mut v19: bool = v18.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v22: bool = if v19 {
        true
    } else {
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let mut v21: bool = v18.contains(&*v20);
        v21
    };
    let mut v25: bool = if v22 {
        true
    } else {
        let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let mut v24: bool = v18.contains(&*v23);
        v24
    };
    if v25 {
        let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v26.clone()
    } else {
        let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
        let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v27));
        v28.clone()
    }
}
fn closure19() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        agile_state_package_module_name_26(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_state_mutation_target_decision_binding(v0: i32, v1: i32) -> i32 {
    closure0()(v0, v1)
}
pub fn eoie_agile_lease_budget_decision_binding(v0: i32, v1: i32) -> i32 {
    closure1()(v0, v1)
}
pub fn eoie_agile_lease_readback_decision_binding(v0: i32, v1: i32) -> i32 {
    closure2()(v0, v1)
}
pub fn eoie_agile_prompt_title_code(v0: &str) -> u64 {
    closure3()(Rc::<str>::from(v0))
}
pub fn eoie_agile_status_name_code(v0: &str) -> u64 {
    closure4()(Rc::<str>::from(v0))
}
pub fn eoie_agile_task_line_candidate_code(v0: &str) -> u64 {
    closure5()(Rc::<str>::from(v0))
}
pub fn eoie_agile_receipt_row_kind_code(v0: &str) -> u64 {
    closure6()(Rc::<str>::from(v0))
}
pub fn eoie_agile_receipt_row_kind_text(v0: &str) -> Rc<str> {
    closure7()(Rc::<str>::from(v0))
}
pub fn eoie_agile_receipt_attestation_kind_code(v0: &str) -> u64 {
    closure8()(Rc::<str>::from(v0))
}
pub fn eoie_agile_receipt_attestation_text(v0: &str) -> Rc<str> {
    closure9()(Rc::<str>::from(v0))
}
pub fn eoie_agile_prompt_lease_status_code(v0: &str) -> u64 {
    closure10()(Rc::<str>::from(v0))
}
pub fn eoie_agile_prompt_lease_marker_text(v0: &str) -> Rc<str> {
    closure11()(Rc::<str>::from(v0))
}
pub fn eoie_agile_prompt_lease_shape_binding(v0: i32, v1: i32) -> i32 {
    closure12()(v0, v1)
}
pub fn eoie_agile_lease_clock_phase_binding(v0: i32, v1: i32) -> i32 {
    closure13()(v0, v1)
}
pub fn eoie_agile_lease_policy_from_text(v0: &str) -> u64 {
    closure14()(Rc::<str>::from(v0))
}
pub fn eoie_agile_lease_effective_phase_binding(v0: i32, v1: i32) -> i32 {
    closure15()(v0, v1)
}
pub fn eoie_agile_status_text(v0: &str) -> Rc<str> {
    closure16()(Rc::<str>::from(v0))
}
pub fn eoie_agile_spi_escape_text(v0: &str) -> Rc<str> {
    closure17()(Rc::<str>::from(v0))
}
pub fn eoie_agile_state_package_line_code(v0: &str) -> u64 {
    closure18()(Rc::<str>::from(v0))
}
pub fn eoie_agile_state_package_module_name(v0: &str) -> Rc<str> {
    closure19()(Rc::<str>::from(v0))
}
