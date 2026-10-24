#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
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
fn bundle_rehydrate_structure_binding_0(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 == 2i32;
    if v2 {
        let mut v3: bool = v1 == 3i32;
        let mut v5: bool = if v3 {
            true
        } else {
            let mut v4: bool = v1 == 7i32;
            v4
        };
        if v5 {
            1i32
        } else {
            0i32
        }
    } else {
        let mut v7: bool = v1 == 4i32;
        let mut v9: bool = if v7 {
            true
        } else {
            let mut v8: bool = v1 == 5i32;
            v8
        };
        let mut v11: bool = if v9 {
            true
        } else {
            let mut v10: bool = v1 == 6i32;
            v10
        };
        let mut v13: bool = if v11 {
            true
        } else {
            let mut v12: bool = v1 == 7i32;
            v12
        };
        if v13 {
            1i32
        } else {
            0i32
        }
    }
}
fn bundle_profile_signature_binding_1(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 == 3i32;
    let mut v4: bool = if v2 {
        let mut v3: bool = v1 == 3i32;
        v3
    } else {
        false
    };
    if v4 {
        1i32
    } else {
        0i32
    }
}
fn starts_with_loop_5(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: i32) -> bool {
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
fn find_text_loop_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: i32) -> i32 {
    loop {
        let mut v5: i32 = v2 + v4 ;
        let mut v6: bool = v3 < v5;
        if v6 {
            return v3;
        } else {
            let mut v7: i32 = (v0.clone().len() as i32);
            let mut v8: i32 = (v1.clone().len() as i32);
            let mut v9: i32 = v2 + v8 ;
            let mut v10: bool = v7 < v9;
            let mut v13: bool = if v10 {
                false
            } else {
                let mut v11: i32 = 0i32;
                starts_with_loop_5(v0.clone(), v1.clone(), v2, v11, v8)
            };
            if v13 {
                return v2;
            } else {
                let mut v14: i32 = v2 + 1i32 ;
                (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v14, v3, v4);
                continue;
            }
        }
    }
}
fn find_char_loop_6(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: u8) -> i32 {
    loop {
        let mut v4: bool = v1 == v2 ;
        if v4 {
            return v2;
        } else {
            let mut v5: u8 = v0.clone().as_bytes()[v1 as usize];
            let mut v6: bool = v5 == v3 ;
            if v6 {
                return v1;
            } else {
                let mut v7: i32 = v1 + 1i32 ;
                (v0, v1, v2, v3) = (v0.clone(), v7, v2, v3);
                continue;
            }
        }
    }
}
fn codec_runtime_char_7(mut v0: Rc<str>) -> u8 {
    let mut v1: u8 = v0.clone().as_bytes()[0i32 as usize];
    v1
}
fn method9() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    v0.clone()
}
fn codec_empty_8() -> Rc<str> {
    method9()
}
fn skip_spaces_loop_11(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v1 == v2 ;
        if v3 {
            return v2;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v1 as usize];
            let mut v5: bool = v4 == b' ' ;
            if v5 {
                let mut v6: i32 = v1 + 1i32 ;
                (v0, v1, v2) = (v0.clone(), v6, v2);
                continue;
            } else {
                let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let mut v9: u8 = codec_runtime_char_7(v8.clone());
                let mut v10: bool = v4 == v9 ;
                if v10 {
                    let mut v11: i32 = v1 + 1i32 ;
                    (v0, v1, v2) = (v0.clone(), v11, v2);
                    continue;
                } else {
                    return v1;
                }
            }
        }
    }
}
fn state_receipt_trim_right_loop_12(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 == 0i32 ;
        if v2 {
            return 0i32;
        } else {
            let mut v3: i32 = v1 - 1i32 ;
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let mut v5: bool = v4 == b' ' ;
            if v5 {
                (v0, v1) = (v0.clone(), v3);
                continue;
            } else {
                let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let mut v8: u8 = codec_runtime_char_7(v7.clone());
                let mut v9: bool = v4 == v8 ;
                if v9 {
                    (v0, v1) = (v0.clone(), v3);
                    continue;
                } else {
                    return v1;
                }
            }
        }
    }
}
fn state_receipt_trim_10(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = 0i32;
    let mut v2: i32 = (v0.clone().len() as i32);
    let mut v3: i32 = skip_spaces_loop_11(v0.clone(), v1, v2);
    let mut v4: i32 = state_receipt_trim_right_loop_12(v0.clone(), v2);
    let mut v5: bool = v4 < v3;
    if v5 {
        codec_empty_8()
    } else {
        let mut v7: bool = v3 == v4 ;
        if v7 {
            codec_empty_8()
        } else {
            let mut v9: i32 = v4 - 1i32 ;
            let mut v10: Rc<str> = string_slice(&v0.clone(), v3 as i64, v9 as i64);
            v10.clone()
        }
    }
}
fn codec_u64_decimal_valid_loop_15(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: u64) -> bool {
    loop {
        let mut v4: bool = v1 == v2 ;
        if v4 {
            return true;
        } else {
            let mut v5: u8 = v0.clone().as_bytes()[v1 as usize];
            let mut v6: bool = v5 == b'0' ;
            let mut v25: u64 = if v6 {
                0u64
            } else {
                let mut v7: bool = v5 == b'1' ;
                if v7 {
                    1u64
                } else {
                    let mut v8: bool = v5 == b'2' ;
                    if v8 {
                        2u64
                    } else {
                        let mut v9: bool = v5 == b'3' ;
                        if v9 {
                            3u64
                        } else {
                            let mut v10: bool = v5 == b'4' ;
                            if v10 {
                                4u64
                            } else {
                                let mut v11: bool = v5 == b'5' ;
                                if v11 {
                                    5u64
                                } else {
                                    let mut v12: bool = v5 == b'6' ;
                                    if v12 {
                                        6u64
                                    } else {
                                        let mut v13: bool = v5 == b'7' ;
                                        if v13 {
                                            7u64
                                        } else {
                                            let mut v14: bool = v5 == b'8' ;
                                            if v14 {
                                                8u64
                                            } else {
                                                let mut v15: bool = v5 == b'9' ;
                                                if v15 {
                                                    9u64
                                                } else {
                                                    10u64
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            let mut v26: bool = v25 == 10u64 ;
            if v26 {
                return false;
            } else {
                let mut v27: bool = v3 > 1844674407370955161u64;
                if v27 {
                    return false;
                } else {
                    let mut v28: bool = v3 == 1844674407370955161u64 ;
                    if v28 {
                        let mut v29: bool = v25 > 5u64;
                        if v29 {
                            return false;
                        } else {
                            let mut v30: i32 = v1 + 1i32 ;
                            let mut v31: u64 = v3 * 10u64 ;
                            let mut v32: u64 = v31 + v25 ;
                            (v0, v1, v2, v3) = (v0.clone(), v30, v2, v32);
                            continue;
                        }
                    } else {
                        let mut v35: i32 = v1 + 1i32 ;
                        let mut v36: u64 = v3 * 10u64 ;
                        let mut v37: u64 = v36 + v25 ;
                        (v0, v1, v2, v3) = (v0.clone(), v35, v2, v37);
                        continue;
                    }
                }
            }
        }
    }
}
fn codec_u64_decimal_normalize_14(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: bool = v1 == 0i32 ;
    if v2 {
        codec_empty_8()
    } else {
        let mut v4: i32 = 0i32;
        let mut v5: u64 = 0u64;
        let mut v6: bool = codec_u64_decimal_valid_loop_15(v0.clone(), v4, v1, v5);
        if v6 {
            v0.clone()
        } else {
            codec_empty_8()
        }
    }
}
fn codec_u64_decimal_value_loop_17(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: u64) -> u64 {
    loop {
        let mut v4: bool = v1 == v2 ;
        if v4 {
            return v3;
        } else {
            let mut v5: u8 = v0.clone().as_bytes()[v1 as usize];
            let mut v6: bool = v5 == b'0' ;
            let mut v25: u64 = if v6 {
                0u64
            } else {
                let mut v7: bool = v5 == b'1' ;
                if v7 {
                    1u64
                } else {
                    let mut v8: bool = v5 == b'2' ;
                    if v8 {
                        2u64
                    } else {
                        let mut v9: bool = v5 == b'3' ;
                        if v9 {
                            3u64
                        } else {
                            let mut v10: bool = v5 == b'4' ;
                            if v10 {
                                4u64
                            } else {
                                let mut v11: bool = v5 == b'5' ;
                                if v11 {
                                    5u64
                                } else {
                                    let mut v12: bool = v5 == b'6' ;
                                    if v12 {
                                        6u64
                                    } else {
                                        let mut v13: bool = v5 == b'7' ;
                                        if v13 {
                                            7u64
                                        } else {
                                            let mut v14: bool = v5 == b'8' ;
                                            if v14 {
                                                8u64
                                            } else {
                                                let mut v15: bool = v5 == b'9' ;
                                                if v15 {
                                                    9u64
                                                } else {
                                                    10u64
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            let mut v26: i32 = v1 + 1i32 ;
            let mut v27: u64 = v3 * 10u64 ;
            let mut v28: u64 = v27 + v25 ;
            (v0, v1, v2, v3) = (v0.clone(), v26, v2, v28);
            continue;
        }
    }
}
fn codec_u64_decimal_value_16(mut v0: Rc<str>) -> u64 {
    let mut v1: Rc<str> = codec_u64_decimal_normalize_14(v0.clone());
    let mut v2: i32 = (v1.clone().len() as i32);
    let mut v3: bool = v2 == 0i32 ;
    if v3 {
        0u64
    } else {
        let mut v4: i32 = 0i32;
        let mut v5: u64 = 0u64;
        codec_u64_decimal_value_loop_17(v1.clone(), v4, v2, v5)
    }
}
fn state_receipt_u32_literal_value_checked_13(mut v0: Rc<str>) -> (u64, bool) {
    let mut v1: Rc<str> = state_receipt_trim_10(v0.clone());
    let mut v2: i32 = (v1.clone().len() as i32);
    let mut v3: bool = v2 < 4i32;
    if v3 {
        (0u64, false)
    } else {
        let mut v4: i32 = v2 - 3i32 ;
        let mut v5: i32 = v4 + 3i32 ;
        let mut v6: bool = v2 < v5;
        let mut v11: bool = if v6 {
            false
        } else {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
            let mut v8: i32 = 0i32;
            let mut v9: i32 = 3i32;
            starts_with_loop_5(v1.clone(), v7.clone(), v4, v8, v9)
        };
        if v11 {
            let mut v12: bool = 0i32 == v4 ;
            let mut v16: Rc<str> = if v12 {
                codec_empty_8()
            } else {
                let mut v14: i32 = v4 - 1i32 ;
                let mut v15: Rc<str> = string_slice(&v1.clone(), 0i32 as i64, v14 as i64);
                v15.clone()
            };
            let mut v17: Rc<str> = codec_u64_decimal_normalize_14(v16.clone());
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v19: bool = v17 == v18 ;
            if v19 {
                (0u64, false)
            } else {
                let mut v20: u64 = codec_u64_decimal_value_16(v17.clone());
                let mut v21: bool = v20 <= 4294967295u64;
                if v21 {
                    (v20, true)
                } else {
                    (0u64, false)
                }
            }
        } else {
            (0u64, false)
        }
    }
}
fn state_receipt_named_u32_assignment_or_max_3(mut v0: Rc<str>, mut v1: Rc<str>) -> u64 {
    let mut v2: i32 = (v0.clone().len() as i32);
    let mut v3: i32 = 0i32;
    let mut v4: i32 = (v1.clone().len() as i32);
    let mut v5: i32 = find_text_loop_4(v0.clone(), v1.clone(), v3, v2, v4);
    let mut v6: bool = v5 == v2 ;
    if v6 {
        18446744073709551615u64
    } else {
        let mut v7: i32 = v5 + v4 ;
        let mut v8: u8 = b'=';
        let mut v9: i32 = find_char_loop_6(v0.clone(), v7, v2, v8);
        let mut v10: bool = v9 == v2 ;
        if v10 {
            18446744073709551615u64
        } else {
            let mut v11: i32 = v9 + 1i32 ;
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v13: u8 = codec_runtime_char_7(v12.clone());
            let mut v14: i32 = find_char_loop_6(v0.clone(), v11, v2, v13);
            let mut v15: bool = v11 == v14 ;
            let mut v19: Rc<str> = if v15 {
                codec_empty_8()
            } else {
                let mut v17: i32 = v14 - 1i32 ;
                let mut v18: Rc<str> = string_slice(&v0.clone(), v11 as i64, v17 as i64);
                v18.clone()
            };
            let mut v20: Rc<str> = state_receipt_trim_10(v19.clone());
            let (mut v21, mut v22): (u64, bool) = state_receipt_u32_literal_value_checked_13(v20.clone());
            if v22 {
                v21
            } else {
                18446744073709551615u64
            }
        }
    }
}
fn state_receipt_recovered_topology_count_or_max_2(mut v0: Rc<str>) -> u64 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl recovered_capability_fixture_count"); } LIT.with(|lit| lit.clone()) };
    let mut v2: u64 = state_receipt_named_u32_assignment_or_max_3(v0.clone(), v1.clone());
    let mut v3: bool = v2 < 18446744073709551615u64;
    if v3 {
        v2
    } else {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl dogfood_recovered_topology_count"); } LIT.with(|lit| lit.clone()) };
        state_receipt_named_u32_assignment_or_max_3(v0.clone(), v4.clone())
    }
}
fn state_receipt_recovered_fixture_scan_loop_20(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64) -> (u64, Rc<str>) {
    loop {
        let mut v5: i32 = (v0.clone().len() as i32);
        let mut v6: bool = v1 == v5 ;
        if v6 {
            return (v4, v3.clone());
        } else {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v8: u8 = codec_runtime_char_7(v7.clone());
            let mut v9: i32 = find_char_loop_6(v0.clone(), v1, v5, v8);
            let mut v10: bool = v9 < v5;
            let mut v12: i32 = if v10 {
                let mut v11: i32 = v9 + 1i32 ;
                v11
            } else {
                v5
            };
            let mut v13: bool = v1 == v9 ;
            let mut v17: Rc<str> = if v13 {
                codec_empty_8()
            } else {
                let mut v15: i32 = v9 - 1i32 ;
                let mut v16: Rc<str> = string_slice(&v0.clone(), v1 as i64, v15 as i64);
                v16.clone()
            };
            let mut v18: Rc<str> = state_receipt_trim_10(v17.clone());
            let mut v19: i32 = (v18.clone().len() as i32);
            let mut v20: bool = v19 >= 21i32;
            let mut v26: bool = if v20 {
                let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("// recovered-fixture|"); } LIT.with(|lit| lit.clone()) };
                let mut v22: i32 = 0i32;
                let mut v23: i32 = 21i32;
                let mut v24: i32 = find_text_loop_4(v18.clone(), v21.clone(), v22, v19, v23);
                let mut v25: bool = v24 == 0i32 ;
                v25
            } else {
                false
            };
            if v26 {
                let mut v27: i32 = 0i32;
                let mut v28: u8 = b'|';
                let mut v29: i32 = find_char_loop_6(v18.clone(), v27, v19, v28);
                let mut v30: i32 = v29 + 1i32 ;
                let mut v31: u8 = b'|';
                let mut v32: i32 = find_char_loop_6(v18.clone(), v30, v19, v31);
                let mut v33: i32 = v32 + 1i32 ;
                let mut v34: u8 = b'|';
                let mut v35: i32 = find_char_loop_6(v18.clone(), v33, v19, v34);
                let mut v36: i32 = v35 + 1i32 ;
                let mut v37: u8 = b'|';
                let mut v38: i32 = find_char_loop_6(v18.clone(), v36, v19, v37);
                let mut v39: i32 = v38 + 1i32 ;
                let mut v40: u8 = b'|';
                let mut v41: i32 = find_char_loop_6(v18.clone(), v39, v19, v40);
                let mut v42: bool = v29 == v19 ;
                let mut v44: bool = if v42 {
                    true
                } else {
                    let mut v43: bool = v32 == v19 ;
                    v43
                };
                let mut v46: bool = if v44 {
                    true
                } else {
                    let mut v45: bool = v35 == v19 ;
                    v45
                };
                let mut v48: bool = if v46 {
                    true
                } else {
                    let mut v47: bool = v38 == v19 ;
                    v47
                };
                let mut v50: bool = if v48 {
                    true
                } else {
                    let mut v49: bool = v41 < v19;
                    v49
                };
                if v50 {
                    let mut v51: Rc<str> = codec_empty_8();
                    return (18446744073709551615u64, v51.clone());
                } else {
                    let mut v52: i32 = v29 + 1i32 ;
                    let mut v53: bool = v52 == v32 ;
                    let mut v57: Rc<str> = if v53 {
                        codec_empty_8()
                    } else {
                        let mut v55: i32 = v32 - 1i32 ;
                        let mut v56: Rc<str> = string_slice(&v18.clone(), v52 as i64, v55 as i64);
                        v56.clone()
                    };
                    let mut v58: i32 = v32 + 1i32 ;
                    let mut v59: bool = v58 == v35 ;
                    let mut v63: Rc<str> = if v59 {
                        codec_empty_8()
                    } else {
                        let mut v61: i32 = v35 - 1i32 ;
                        let mut v62: Rc<str> = string_slice(&v18.clone(), v58 as i64, v61 as i64);
                        v62.clone()
                    };
                    let mut v64: i32 = v35 + 1i32 ;
                    let mut v65: bool = v64 == v38 ;
                    let mut v69: Rc<str> = if v65 {
                        codec_empty_8()
                    } else {
                        let mut v67: i32 = v38 - 1i32 ;
                        let mut v68: Rc<str> = string_slice(&v18.clone(), v64 as i64, v67 as i64);
                        v68.clone()
                    };
                    let mut v70: i32 = v38 + 1i32 ;
                    let mut v71: bool = v70 == v19 ;
                    let mut v75: Rc<str> = if v71 {
                        codec_empty_8()
                    } else {
                        let mut v73: i32 = v19 - 1i32 ;
                        let mut v74: Rc<str> = string_slice(&v18.clone(), v70 as i64, v73 as i64);
                        v74.clone()
                    };
                    let mut v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v77: bool = v63 == v76 ;
                    let mut v79: bool = if v77 {
                        true
                    } else {
                        let mut v78: bool = v69 == v76 ;
                        v78
                    };
                    let mut v81: bool = if v79 {
                        true
                    } else {
                        let mut v80: bool = v75 == v76 ;
                        v80
                    };
                    if v81 {
                        let mut v82: Rc<str> = codec_empty_8();
                        return (18446744073709551615u64, v82.clone());
                    } else {
                        let mut v83: Rc<str> = Rc::<str>::from(format!("{}{}", v57.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                        let mut v84: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }, v83.clone()));
                        let mut v85: i32 = (v2.clone().len() as i32);
                        let mut v86: i32 = 0i32;
                        let mut v87: i32 = (v84.clone().len() as i32);
                        let mut v88: i32 = find_text_loop_4(v2.clone(), v84.clone(), v86, v85, v87);
                        let mut v89: bool = v88 < v85;
                        if v89 {
                            let mut v90: Rc<str> = codec_empty_8();
                            return (18446744073709551615u64, v90.clone());
                        } else {
                            let mut v91: Rc<str> = Rc::<str>::from(format!("{}{}", v2.clone(), v83.clone()));
                            let mut v92: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }, v75.clone()));
                            let mut v93: Rc<str> = Rc::<str>::from(format!("{}{}", v69.clone(), v92.clone()));
                            let mut v94: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }, v93.clone()));
                            let mut v95: Rc<str> = Rc::<str>::from(format!("{}{}", v63.clone(), v94.clone()));
                            let mut v96: bool = v3 == v76 ;
                            let mut v99: Rc<str> = if v96 {
                                v95.clone()
                            } else {
                                let mut v97: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }, v95.clone()));
                                let mut v98: Rc<str> = Rc::<str>::from(format!("{}{}", v3.clone(), v97.clone()));
                                v98.clone()
                            };
                            let mut v100: u64 = v4 + 1u64 ;
                            (v0, v1, v2, v3, v4) = (v0.clone(), v12, v91.clone(), v99.clone(), v100);
                            continue;
                        }
                    }
                }
            } else {
                (v0, v1, v2, v3, v4) = (v0.clone(), v12, v2.clone(), v3.clone(), v4);
                continue;
            }
        }
    }
}
fn state_receipt_recovered_fixture_scan_19(mut v0: Rc<str>) -> (u64, Rc<str>) {
    let mut v1: i32 = 0i32;
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = codec_empty_8();
    let mut v4: u64 = 0u64;
    state_receipt_recovered_fixture_scan_loop_20(v0.clone(), v1, v2.clone(), v3.clone(), v4)
}
fn state_receipt_recovered_fixture_record_count_or_max_18(mut v0: Rc<str>) -> u64 {
    let (mut v1, mut v2): (u64, Rc<str>) = state_receipt_recovered_fixture_scan_19(v0.clone());
    v1
}
fn state_receipt_recovered_fixture_paths_21(mut v0: Rc<str>) -> Rc<str> {
    let (mut v1, mut v2): (u64, Rc<str>) = state_receipt_recovered_fixture_scan_19(v0.clone());
    let mut v3: bool = v1 == 18446744073709551615u64 ;
    if v3 {
        codec_empty_8()
    } else {
        v2.clone()
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        state_receipt_recovered_topology_count_or_max_2(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        state_receipt_recovered_fixture_record_count_or_max_18(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        state_receipt_recovered_fixture_paths_21(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure3() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        bundle_rehydrate_structure_binding_0(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure4() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        bundle_profile_signature_binding_1(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_state_receipt_recovered_topology_count_or_max(v0: &str) -> u64 {
    closure0()(Rc::<str>::from(v0))
}
pub fn eoie_state_receipt_recovered_fixture_record_count_or_max(v0: &str) -> u64 {
    closure1()(Rc::<str>::from(v0))
}
pub fn eoie_state_receipt_recovered_fixture_paths(v0: &str) -> Rc<str> {
    closure2()(Rc::<str>::from(v0))
}
pub fn eoie_bundle_rehydrate_structure_binding(v0: i32, v1: i32) -> i32 {
    closure3()(v0, v1)
}
pub fn eoie_bundle_profile_signature_binding(v0: i32, v1: i32) -> i32 {
    closure4()(v0, v1)
}
