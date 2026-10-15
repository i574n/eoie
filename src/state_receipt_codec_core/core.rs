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
fn method3() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    v0.clone()
}
fn codec_empty_2() -> Rc<str> {
    method3()
}
fn codec_u64_decimal_valid_loop_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: u64) -> bool {
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
fn codec_u64_decimal_normalize_1(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: bool = v1 == 0i32 ;
    if v2 {
        codec_empty_2()
    } else {
        let mut v4: i32 = 0i32;
        let mut v5: u64 = 0u64;
        let mut v6: bool = codec_u64_decimal_valid_loop_4(v0.clone(), v4, v1, v5);
        if v6 {
            v0.clone()
        } else {
            codec_empty_2()
        }
    }
}
fn state_receipt_u64_decimal_valid_0(mut v0: Rc<str>) -> u64 {
    let mut v1: Rc<str> = codec_u64_decimal_normalize_1(v0.clone());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v1 == v2 ;
    if v3 {
        0u64
    } else {
        1u64
    }
}
fn codec_u64_decimal_value_loop_7(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: u64) -> u64 {
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
fn codec_u64_decimal_value_6(mut v0: Rc<str>) -> u64 {
    let mut v1: Rc<str> = codec_u64_decimal_normalize_1(v0.clone());
    let mut v2: i32 = (v1.clone().len() as i32);
    let mut v3: bool = v2 == 0i32 ;
    if v3 {
        0u64
    } else {
        let mut v4: i32 = 0i32;
        let mut v5: u64 = 0u64;
        codec_u64_decimal_value_loop_7(v1.clone(), v4, v2, v5)
    }
}
fn state_receipt_u64_decimal_value_5(mut v0: Rc<str>) -> u64 {
    codec_u64_decimal_value_6(v0.clone())
}
fn state_receipt_u32_decimal_valid_8(mut v0: Rc<str>) -> u64 {
    let mut v1: Rc<str> = codec_u64_decimal_normalize_1(v0.clone());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v1 == v2 ;
    if v3 {
        0u64
    } else {
        let mut v4: u64 = codec_u64_decimal_value_6(v1.clone());
        let mut v5: bool = v4 <= 4294967295u64;
        if v5 {
            1u64
        } else {
            0u64
        }
    }
}
fn starts_with_loop_11(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: i32) -> bool {
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
fn state_receipt_u32_spi_literal_decimal_10(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: bool = v1 >= 3i32;
    let mut v11: bool = if v2 {
        let mut v3: i32 = v1 - 3i32 ;
        let mut v4: i32 = v3 + 3i32 ;
        let mut v5: bool = v1 < v4;
        if v5 {
            false
        } else {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
            let mut v7: i32 = 0i32;
            let mut v8: i32 = 3i32;
            starts_with_loop_11(v0.clone(), v6.clone(), v3, v7, v8)
        }
    } else {
        false
    };
    if v11 {
        let mut v12: i32 = v1 - 3i32 ;
        let mut v13: bool = 0i32 == v12 ;
        if v13 {
            codec_empty_2()
        } else {
            let mut v15: i32 = v12 - 1i32 ;
            let mut v16: Rc<str> = string_slice(&v0.clone(), 0i32 as i64, v15 as i64);
            v16.clone()
        }
    } else {
        v0.clone()
    }
}
fn state_receipt_u32_spi_literal_valid_9(mut v0: Rc<str>) -> u64 {
    let mut v1: Rc<str> = state_receipt_u32_spi_literal_decimal_10(v0.clone());
    state_receipt_u32_decimal_valid_8(v1.clone())
}
fn state_receipt_u32_spi_literal_value_12(mut v0: Rc<str>) -> u64 {
    let mut v1: Rc<str> = state_receipt_u32_spi_literal_decimal_10(v0.clone());
    codec_u64_decimal_value_6(v1.clone())
}
fn state_receipt_dash_count_loop_14(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: u64) -> u64 {
    loop {
        let mut v4: bool = v1 == v2 ;
        if v4 {
            return v3;
        } else {
            let mut v5: u8 = v0.clone().as_bytes()[v1 as usize];
            let mut v6: bool = v5 == b'-' ;
            let mut v8: u64 = if v6 {
                let mut v7: u64 = v3 + 1u64 ;
                v7
            } else {
                v3
            };
            let mut v9: i32 = v1 + 1i32 ;
            (v0, v1, v2, v3) = (v0.clone(), v9, v2, v8);
            continue;
        }
    }
}
fn state_receipt_dash_shape_13(mut v0: Rc<str>) -> u64 {
    let mut v1: i32 = 0i32;
    let mut v2: i32 = (v0.clone().len() as i32);
    let mut v3: u64 = 0u64;
    let mut v4: u64 = state_receipt_dash_count_loop_14(v0.clone(), v1, v2, v3);
    let mut v5: u64 = v4 + 1u64 ;
    let mut v6: u64 = v5 * 10u64 ;
    let mut v7: u64 = v6 + v4 ;
    v7
}
fn find_text_loop_17(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: i32) -> i32 {
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
                starts_with_loop_11(v0.clone(), v1.clone(), v2, v11, v8)
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
fn codec_runtime_char_18(mut v0: Rc<str>) -> u8 {
    let mut v1: u8 = v0.clone().as_bytes()[0i32 as usize];
    v1
}
fn find_char_loop_19(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: u8) -> i32 {
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
fn state_receipt_find_last_char_loop_20(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: u8, mut v4: i32) -> i32 {
    loop {
        let mut v5: bool = v1 == v2 ;
        if v5 {
            return v4;
        } else {
            let mut v6: u8 = v0.clone().as_bytes()[v1 as usize];
            let mut v7: bool = v6 == v3 ;
            let mut v8: i32 = if v7 {
                v1
            } else {
                v4
            };
            let mut v9: i32 = v1 + 1i32 ;
            (v0, v1, v2, v3, v4) = (v0.clone(), v9, v2, v3, v8);
            continue;
        }
    }
}
fn state_receipt_constructor_payload_16(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let mut v3: i32 = 0i32;
    let mut v4: i32 = (v1.clone().len() as i32);
    let mut v5: i32 = find_text_loop_17(v0.clone(), v1.clone(), v3, v2, v4);
    let mut v6: bool = v5 == v2 ;
    if v6 {
        codec_empty_2()
    } else {
        let mut v8: i32 = v5 + v4 ;
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v10: u8 = codec_runtime_char_18(v9.clone());
        let mut v11: i32 = find_char_loop_19(v0.clone(), v8, v2, v10);
        let mut v12: u8 = b')';
        let mut v13: i32 = -1i32;
        let mut v14: i32 = state_receipt_find_last_char_loop_20(v0.clone(), v8, v11, v12, v13);
        let mut v15: bool = v14 < v8;
        if v15 {
            codec_empty_2()
        } else {
            let mut v17: bool = v8 == v14 ;
            if v17 {
                codec_empty_2()
            } else {
                let mut v19: i32 = v14 - 1i32 ;
                let mut v20: Rc<str> = string_slice(&v0.clone(), v8 as i64, v19 as i64);
                v20.clone()
            }
        }
    }
}
fn state_receipt_migration_denominator_payload_15(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("MigrationDenominator ("); } LIT.with(|lit| lit.clone()) };
    state_receipt_constructor_payload_16(v0.clone(), v1.clone())
}
fn state_receipt_constructor_payload_from_25(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> (Rc<str>, i32, i32) {
    let mut v3: i32 = (v0.clone().len() as i32);
    let mut v4: i32 = (v1.clone().len() as i32);
    let mut v5: i32 = find_text_loop_17(v0.clone(), v1.clone(), v2, v3, v4);
    let mut v6: bool = v5 == v3 ;
    if v6 {
        let mut v7: Rc<str> = codec_empty_2();
        (v7.clone(), v3, 0i32)
    } else {
        let mut v8: i32 = v5 + v4 ;
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v10: u8 = codec_runtime_char_18(v9.clone());
        let mut v11: i32 = find_char_loop_19(v0.clone(), v8, v3, v10);
        let mut v12: bool = v11 < v3;
        let mut v14: i32 = if v12 {
            let mut v13: i32 = v11 + 1i32 ;
            v13
        } else {
            v3
        };
        let mut v15: u8 = b')';
        let mut v16: i32 = -1i32;
        let mut v17: i32 = state_receipt_find_last_char_loop_20(v0.clone(), v8, v11, v15, v16);
        let mut v18: bool = v17 < v8;
        if v18 {
            let mut v19: Rc<str> = codec_empty_2();
            (v19.clone(), v14, 2i32)
        } else {
            let mut v20: bool = v8 == v17 ;
            let mut v24: Rc<str> = if v20 {
                codec_empty_2()
            } else {
                let mut v22: i32 = v17 - 1i32 ;
                let mut v23: Rc<str> = string_slice(&v0.clone(), v8 as i64, v22 as i64);
                v23.clone()
            };
            (v24.clone(), v14, 1i32)
        }
    }
}
fn skip_spaces_loop_28(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
                let mut v9: u8 = codec_runtime_char_18(v8.clone());
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
fn state_receipt_trim_right_loop_29(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
                let mut v8: u8 = codec_runtime_char_18(v7.clone());
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
fn state_receipt_trim_27(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = 0i32;
    let mut v2: i32 = (v0.clone().len() as i32);
    let mut v3: i32 = skip_spaces_loop_28(v0.clone(), v1, v2);
    let mut v4: i32 = state_receipt_trim_right_loop_29(v0.clone(), v2);
    let mut v5: bool = v4 < v3;
    if v5 {
        codec_empty_2()
    } else {
        let mut v7: bool = v3 == v4 ;
        if v7 {
            codec_empty_2()
        } else {
            let mut v9: i32 = v4 - 1i32 ;
            let mut v10: Rc<str> = string_slice(&v0.clone(), v3 as i64, v9 as i64);
            v10.clone()
        }
    }
}
fn state_receipt_u32_literal_value_checked_26(mut v0: Rc<str>) -> (u64, bool) {
    let mut v1: Rc<str> = state_receipt_trim_27(v0.clone());
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
            starts_with_loop_11(v1.clone(), v7.clone(), v4, v8, v9)
        };
        if v11 {
            let mut v12: bool = 0i32 == v4 ;
            let mut v16: Rc<str> = if v12 {
                codec_empty_2()
            } else {
                let mut v14: i32 = v4 - 1i32 ;
                let mut v15: Rc<str> = string_slice(&v1.clone(), 0i32 as i64, v14 as i64);
                v15.clone()
            };
            let mut v17: Rc<str> = codec_u64_decimal_normalize_1(v16.clone());
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v19: bool = v17 == v18 ;
            if v19 {
                (0u64, false)
            } else {
                let mut v20: u64 = codec_u64_decimal_value_6(v17.clone());
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
fn state_receipt_series_records_loop_24(mut v0: Rc<str>, mut v1: i32, mut v2: u64, mut v3: bool, mut v4: Rc<str>) -> (bool, Rc<str>) {
    loop {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SeriesRecord ("); } LIT.with(|lit| lit.clone()) };
        let (mut v6, mut v7, mut v8): (Rc<str>, i32, i32) = state_receipt_constructor_payload_from_25(v0.clone(), v5.clone(), v1);
        let mut v9: bool = v8 == 0i32 ;
        if v9 {
            return (true, v4.clone());
        } else {
            let mut v10: bool = v8 == 2i32 ;
            if v10 {
                let mut v11: Rc<str> = codec_empty_2();
                return (false, v11.clone());
            } else {
                let mut v12: i32 = (v6.clone().len() as i32);
                let mut v13: i32 = 0i32;
                let mut v14: u8 = b',';
                let mut v15: i32 = find_char_loop_19(v6.clone(), v13, v12, v14);
                let mut v16: bool = v15 == v12 ;
                if v16 {
                    let mut v17: Rc<str> = codec_empty_2();
                    return (false, v17.clone());
                } else {
                    let mut v18: bool = 0i32 == v15 ;
                    let mut v22: Rc<str> = if v18 {
                        codec_empty_2()
                    } else {
                        let mut v20: i32 = v15 - 1i32 ;
                        let mut v21: Rc<str> = string_slice(&v6.clone(), 0i32 as i64, v20 as i64);
                        v21.clone()
                    };
                    let (mut v23, mut v24): (u64, bool) = state_receipt_u32_literal_value_checked_26(v22.clone());
                    if v24 {
                        let mut v26: bool = if v3 {
                            let mut v25: bool = v23 <= v2;
                            v25
                        } else {
                            false
                        };
                        if v26 {
                            let mut v27: Rc<str> = codec_empty_2();
                            return (false, v27.clone());
                        } else {
                            let mut v28: bool = true;
                            (v0, v1, v2, v3, v4) = (v0.clone(), v7, v23, v28, v6.clone());
                            continue;
                        }
                    } else {
                        let mut v33: Rc<str> = codec_empty_2();
                        return (false, v33.clone());
                    }
                }
            }
        }
    }
}
fn state_receipt_last_series_record_payload_23(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = 0i32;
    let mut v2: u64 = 0u64;
    let mut v3: bool = false;
    let mut v4: Rc<str> = codec_empty_2();
    let (mut v5, mut v6): (bool, Rc<str>) = state_receipt_series_records_loop_24(v0.clone(), v1, v2, v3, v4.clone());
    if v5 {
        v6.clone()
    } else {
        codec_empty_2()
    }
}
fn state_receipt_series_records_valid_22(mut v0: Rc<str>) -> bool {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SeriesRecord ("); } LIT.with(|lit| lit.clone()) };
    let mut v3: i32 = 0i32;
    let mut v4: i32 = 14i32;
    let mut v5: i32 = find_text_loop_17(v0.clone(), v2.clone(), v3, v1, v4);
    let mut v6: bool = v5 == v1 ;
    if v6 {
        true
    } else {
        let mut v7: Rc<str> = state_receipt_last_series_record_payload_23(v0.clone());
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v9: bool = v7 == v8 ;
        let mut v10: bool = v9 == false;
        v10
    }
}
fn state_receipt_series_records_valid_u64_21(mut v0: Rc<str>) -> u64 {
    let mut v1: bool = state_receipt_series_records_valid_22(v0.clone());
    if v1 {
        1u64
    } else {
        0u64
    }
}
fn state_receipt_last_series_record_payload_export_30(mut v0: Rc<str>) -> Rc<str> {
    state_receipt_last_series_record_payload_23(v0.clone())
}
fn state_receipt_coverage_checkpoint_kind_31(mut v0: Rc<str>) -> u64 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("CoverageCheckpointDurable ("); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = state_receipt_constructor_payload_16(v0.clone(), v1.clone());
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = v2 == v3 ;
    if v4 {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("CoverageCheckpointMissing ("); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = state_receipt_constructor_payload_16(v0.clone(), v5.clone());
        let mut v7: bool = v6 == v3 ;
        if v7 {
            0u64
        } else {
            1u64
        }
    } else {
        2u64
    }
}
fn state_receipt_coverage_checkpoint_sha_32(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("CoverageCheckpointDurable ("); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = state_receipt_constructor_payload_16(v0.clone(), v1.clone());
    let mut v3: i32 = (v2.clone().len() as i32);
    let mut v4: bool = v3 == 0i32 ;
    if v4 {
        codec_empty_2()
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let mut v7: u8 = codec_runtime_char_18(v6.clone());
        let mut v8: i32 = 0i32;
        let mut v9: i32 = find_char_loop_19(v2.clone(), v8, v3, v7);
        let mut v10: bool = v9 == v3 ;
        if v10 {
            codec_empty_2()
        } else {
            let mut v12: i32 = v9 + 1i32 ;
            let mut v13: u8 = codec_runtime_char_18(v6.clone());
            let mut v14: i32 = find_char_loop_19(v2.clone(), v12, v3, v13);
            let mut v15: bool = v14 == v3 ;
            if v15 {
                codec_empty_2()
            } else {
                let mut v17: i32 = v9 + 1i32 ;
                let mut v18: bool = v17 == v14 ;
                if v18 {
                    codec_empty_2()
                } else {
                    let mut v20: i32 = v14 - 1i32 ;
                    let mut v21: Rc<str> = string_slice(&v2.clone(), v17 as i64, v20 as i64);
                    v21.clone()
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        state_receipt_u64_decimal_valid_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        state_receipt_u64_decimal_value_5(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        state_receipt_u32_decimal_valid_8(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure3() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        state_receipt_u32_spi_literal_valid_9(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure4() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        state_receipt_u32_spi_literal_value_12(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure5() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        state_receipt_dash_shape_13(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure6() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        state_receipt_migration_denominator_payload_15(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure7() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        state_receipt_series_records_valid_u64_21(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure8() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        state_receipt_last_series_record_payload_export_30(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure9() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        state_receipt_coverage_checkpoint_kind_31(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure10() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        state_receipt_coverage_checkpoint_sha_32(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_state_receipt_u64_decimal_valid(v0: &str) -> u64 {
    closure0()(Rc::<str>::from(v0))
}
pub fn eoie_state_receipt_u64_decimal_value(v0: &str) -> u64 {
    closure1()(Rc::<str>::from(v0))
}
pub fn eoie_state_receipt_u32_decimal_valid(v0: &str) -> u64 {
    closure2()(Rc::<str>::from(v0))
}
pub fn eoie_state_receipt_u32_spi_literal_valid(v0: &str) -> u64 {
    closure3()(Rc::<str>::from(v0))
}
pub fn eoie_state_receipt_u32_spi_literal_value(v0: &str) -> u64 {
    closure4()(Rc::<str>::from(v0))
}
pub fn eoie_state_receipt_dash_shape(v0: &str) -> u64 {
    closure5()(Rc::<str>::from(v0))
}
pub fn eoie_state_receipt_migration_denominator_payload(v0: &str) -> Rc<str> {
    closure6()(Rc::<str>::from(v0))
}
pub fn eoie_state_receipt_series_records_valid(v0: &str) -> u64 {
    closure7()(Rc::<str>::from(v0))
}
pub fn eoie_state_receipt_last_series_record_payload(v0: &str) -> Rc<str> {
    closure8()(Rc::<str>::from(v0))
}
pub fn eoie_state_receipt_coverage_checkpoint_kind(v0: &str) -> u64 {
    closure9()(Rc::<str>::from(v0))
}
pub fn eoie_state_receipt_coverage_checkpoint_sha(v0: &str) -> Rc<str> {
    closure10()(Rc::<str>::from(v0))
}
