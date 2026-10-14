#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        987i32
    }); } CLOSURE.with(|closure| closure.clone())
}
fn map_binding_0(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    let mut v4: bool = v0 < 1i32;
    if v4 {
        0i32
    } else {
        let mut v5: bool = v1 < 1i32;
        if v5 {
            0i32
        } else {
            let mut v6: bool = v2 < 1i32;
            if v6 {
                0i32
            } else {
                let mut v7: bool = v3 < 0i32;
                if v7 {
                    0i32
                } else {
                    let mut v8: bool = v0 < v1;
                    if v8 {
                        0i32
                    } else {
                        let mut v9: i32 = v0.wrapping_sub(v1);
                        let mut v10: bool = v9 == v3;
                        if v10 {
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
fn score_binding_1(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    let mut v4: bool = v0 < 1i32;
    if v4 {
        0i32
    } else {
        let mut v5: bool = v1 < v0;
        if v5 {
            0i32
        } else {
            let mut v6: bool = v2 < 1i32;
            if v6 {
                0i32
            } else {
                let mut v7: bool = v3 < 500i32;
                if v7 {
                    0i32
                } else {
                    1i32
                }
            }
        }
    }
}
fn budget_binding_2(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 < 1i32;
    if v2 {
        0i32
    } else {
        let mut v3: bool = 1000i32 < v0;
        if v3 {
            0i32
        } else {
            let mut v4: bool = v1 < 1i32;
            if v4 {
                0i32
            } else {
                let mut v5: bool = 1000i32 < v1;
                if v5 {
                    0i32
                } else {
                    1i32
                }
            }
        }
    }
}
fn lease_binding_3(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 == 4500000i32;
    if v2 {
        let mut v3: bool = v1 == 600000i32;
        if v3 {
            1i32
        } else {
            0i32
        }
    } else {
        0i32
    }
}
fn authority_binding_4(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    let mut v4: i32 = v0.wrapping_mul(v1);
    let mut v5: i32 = v2.wrapping_mul(v3);
    let mut v6: i32 = v4.wrapping_mul(v5);
    v6
}
fn closure1() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        let mut v0: i32 = 162i32;
        let mut v1: i32 = 68i32;
        let mut v2: i32 = 162i32;
        let mut v3: i32 = 94i32;
        let mut v4: i32 = map_binding_0(v0, v1, v2, v3);
        let mut v5: i32 = 44i32;
        let mut v6: i32 = 69i32;
        let mut v7: i32 = 637i32;
        let mut v8: i32 = 500i32;
        let mut v9: i32 = score_binding_1(v5, v6, v7, v8);
        let mut v10: i32 = 974i32;
        let mut v11: i32 = 1000i32;
        let mut v14: i32 = budget_binding_2(v10, v11);
        let mut v15: i32 = 4500000i32;
        let mut v16: i32 = 600000i32;
        let mut v17: i32 = lease_binding_3(v15, v16);
        authority_binding_4(v4, v9, v14, v17)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn revision_binding_5(mut v0: i32, mut v1: i32, mut v2: i32) -> i32 {
    let mut v3: bool = v0 == 0i32;
    if v3 {
        let mut v4: bool = v2 == 0i32;
        if v4 {
            1i32
        } else {
            0i32
        }
    } else {
        let mut v6: bool = v1 == 1i32;
        if v6 {
            let mut v7: bool = v2 == 1i32;
            if v7 {
                1i32
            } else {
                0i32
            }
        } else {
            let mut v9: bool = v2 == 2i32;
            if v9 {
                1i32
            } else {
                0i32
            }
        }
    }
}
fn lane_binding_6(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 < 1i32;
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
fn cache_binding_7(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v1 == 0i32;
    if v2 {
        1i32
    } else {
        let mut v3: bool = v1 == 1i32;
        if v3 {
            let mut v4: bool = v0 == 1i32;
            if v4 {
                1i32
            } else {
                0i32
            }
        } else {
            0i32
        }
    }
}
fn request_binding_8(mut v0: i32) -> i32 {
    let mut v1: bool = v0 == 0i32;
    if v1 {
        1i32
    } else {
        0i32
    }
}
fn shutdown_binding_9(mut v0: i32) -> i32 {
    let mut v1: bool = v0 == 0i32;
    if v1 {
        1i32
    } else {
        0i32
    }
}
fn authority_binding_10(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32) -> i32 {
    let mut v5: i32 = v0.wrapping_mul(v1);
    let mut v6: i32 = v2.wrapping_mul(v3);
    let mut v7: i32 = v5.wrapping_mul(v6);
    let mut v8: i32 = v7.wrapping_mul(v4);
    v8
}
fn closure2() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        let mut v0: i32 = 0i32;
        let mut v1: i32 = 0i32;
        let mut v2: i32 = 0i32;
        let mut v3: i32 = revision_binding_5(v0, v1, v2);
        let mut v4: i32 = 1i32;
        let mut v5: i32 = 0i32;
        let mut v6: i32 = 2i32;
        let mut v7: i32 = revision_binding_5(v4, v5, v6);
        let mut v8: i32 = 1i32;
        let mut v9: i32 = 1i32;
        let mut v10: i32 = 1i32;
        let mut v11: i32 = revision_binding_5(v8, v9, v10);
        let mut v12: i32 = v7.wrapping_mul(v11);
        let mut v13: i32 = v3.wrapping_mul(v12);
        let mut v14: i32 = 8i32;
        let mut v15: i32 = 5i32;
        let mut v16: i32 = lane_binding_6(v14, v15);
        let mut v17: i32 = 1i32;
        let mut v18: i32 = 1i32;
        let mut v19: i32 = cache_binding_7(v17, v18);
        let mut v20: i32 = 0i32;
        let mut v21: i32 = request_binding_8(v20);
        let mut v22: i32 = 0i32;
        let mut v23: i32 = shutdown_binding_9(v22);
        authority_binding_10(v16, v13, v19, v21, v23)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn bool_pair_binding_12(mut v0: i32, mut v1: i32) -> i32 {
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
fn source_comparison_binding_11(mut v0: i32, mut v1: i32) -> i32 {
    bool_pair_binding_12(v0, v1)
}
fn ordering_binding_13(mut v0: i32, mut v1: i32) -> i32 {
    bool_pair_binding_12(v0, v1)
}
fn count_binding_14(mut v0: i32, mut v1: i32) -> i32 {
    bool_pair_binding_12(v0, v1)
}
fn nested_policy_binding_15(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 < 1i32;
    if v2 {
        0i32
    } else {
        let mut v3: bool = 1i32 < v0;
        if v3 {
            0i32
        } else {
            let mut v4: bool = v1 < 2i32;
            if v4 {
                0i32
            } else {
                let mut v5: bool = 2i32 < v1;
                if v5 {
                    0i32
                } else {
                    1i32
                }
            }
        }
    }
}
fn epoch_binding_16(mut v0: i32, mut v1: i32) -> i32 {
    bool_pair_binding_12(v0, v1)
}
fn determinism_binding_17(mut v0: i32, mut v1: i32) -> i32 {
    bool_pair_binding_12(v0, v1)
}
fn complete_binding_18(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32) -> i32 {
    let mut v6: bool = v0 < 1i32;
    if v6 {
        0i32
    } else {
        let mut v7: bool = v1 < 1i32;
        if v7 {
            0i32
        } else {
            let mut v8: bool = v2 < 1i32;
            if v8 {
                0i32
            } else {
                let mut v9: bool = v3 < 1i32;
                if v9 {
                    0i32
                } else {
                    let mut v10: bool = v4 < 1i32;
                    if v10 {
                        0i32
                    } else {
                        let mut v11: bool = v5 < 1i32;
                        if v11 {
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
fn closure3() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        let mut v0: i32 = 1i32;
        let mut v1: i32 = 1i32;
        let mut v2: i32 = source_comparison_binding_11(v0, v1);
        let mut v3: i32 = 1i32;
        let mut v4: i32 = 1i32;
        let mut v5: i32 = ordering_binding_13(v3, v4);
        let mut v6: i32 = 1i32;
        let mut v7: i32 = 1i32;
        let mut v8: i32 = count_binding_14(v6, v7);
        let mut v9: i32 = 1i32;
        let mut v10: i32 = 2i32;
        let mut v11: i32 = nested_policy_binding_15(v9, v10);
        let mut v12: i32 = 1i32;
        let mut v13: i32 = 1i32;
        let mut v14: i32 = epoch_binding_16(v12, v13);
        let mut v15: i32 = 1i32;
        let mut v16: i32 = 1i32;
        let mut v17: i32 = determinism_binding_17(v15, v16);
        complete_binding_18(v2, v5, v8, v11, v14, v17)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn manifest_binding_19(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 < 1i32;
    if v2 {
        0i32
    } else {
        let mut v3: bool = v1 < 0i32;
        if v3 {
            0i32
        } else {
            let mut v4: bool = v1 < 1i32;
            if v4 {
                1i32
            } else {
                2i32
            }
        }
    }
}
fn trim_binding_20(mut v0: i32) -> i32 {
    let mut v1: bool = v0 < 0i32;
    if v1 {
        0i32
    } else {
        let mut v2: bool = 0i32 < v0;
        if v2 {
            0i32
        } else {
            1i32
        }
    }
}
fn extracted_binding_21(mut v0: i32, mut v1: i32, mut v2: i32) -> i32 {
    let mut v3: i32 = trim_binding_20(v2);
    let mut v4: bool = v0 < 1i32;
    if v4 {
        0i32
    } else {
        let mut v5: bool = v1 < 1i32;
        if v5 {
            0i32
        } else {
            let mut v6: bool = v3 < 1i32;
            if v6 {
                0i32
            } else {
                1i32
            }
        }
    }
}
fn targets_binding_22(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 < 0i32;
    if v2 {
        0i32
    } else {
        let mut v3: bool = v1 < 0i32;
        if v3 {
            0i32
        } else {
            let mut v4: bool = v0 < 1i32;
            if v4 {
                let mut v5: bool = v1 < 1i32;
                if v5 {
                    0i32
                } else {
                    1i32
                }
            } else {
                1i32
            }
        }
    }
}
fn frontier_binding_23(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 < 1i32;
    if v2 {
        0i32
    } else {
        let mut v3: bool = v1 < 1i32;
        if v3 {
            0i32
        } else {
            let mut v4: bool = 3i32 < v1;
            if v4 {
                0i32
            } else {
                1i32
            }
        }
    }
}
fn complete_binding_24(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32) -> i32 {
    let mut v5: bool = v0 < 1i32;
    if v5 {
        0i32
    } else {
        let mut v6: bool = v1 < 1i32;
        if v6 {
            0i32
        } else {
            let mut v7: bool = v2 < 1i32;
            if v7 {
                0i32
            } else {
                let mut v8: bool = v3 < 1i32;
                if v8 {
                    0i32
                } else {
                    let mut v9: bool = v4 < 1i32;
                    if v9 {
                        0i32
                    } else {
                        1i32
                    }
                }
            }
        }
    }
}
fn closure4() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        let mut v0: i32 = 1i32;
        let mut v1: i32 = 2i32;
        let mut v2: i32 = manifest_binding_19(v0, v1);
        let mut v3: i32 = 0i32;
        let mut v4: i32 = trim_binding_20(v3);
        let mut v5: i32 = 1i32;
        let mut v6: i32 = 1i32;
        let mut v7: i32 = 0i32;
        let mut v8: i32 = extracted_binding_21(v5, v6, v7);
        let mut v9: i32 = 1i32;
        let mut v10: i32 = 2i32;
        let mut v11: i32 = targets_binding_22(v9, v10);
        let mut v12: i32 = 3i32;
        let mut v13: i32 = 3i32;
        let mut v14: i32 = frontier_binding_23(v12, v13);
        complete_binding_24(v2, v4, v8, v11, v14)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_bootstrap_settled_migration_status() -> i32 {
    closure0()()
}
pub fn eoie_bootstrap_workspace_state_status() -> i32 {
    closure1()()
}
pub fn eoie_bootstrap_compiler_session_status() -> i32 {
    closure2()()
}
pub fn eoie_bootstrap_legacy_bundle_writer_status() -> i32 {
    closure3()()
}
pub fn eoie_bootstrap_ingest_frontier_status() -> i32 {
    closure4()()
}
