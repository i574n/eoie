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
#[derive(Clone)]
enum US1 {
    US1_SourceBeforeEnd,
    US1_EmptySource,
    US1_HitBeforeSource,
    US1_InvalidHitFields,
    US1_InvalidHitLine,
    US1_InvalidHitCount,
    US1_EndBeforeSource,
    US1_MalformedRecord,
    US1_UnterminatedSource,
    US1_NoLineRecords,
}
#[derive(Clone)]
enum US0 {
    US0_LcovRecords(Rc<str>),
    US0_LcovRejected(US1),
}
#[derive(Clone)]
enum US2 {
    US2_BetweenSources,
    US2_InSource(Rc<str>),
}
#[derive(Clone)]
enum US3 {
    US3_SourceLine(Rc<str>),
    US3_HitLine(Rc<str>),
    US3_EndLine,
    US3_MalformedLine,
    US3_OtherLine,
}
#[derive(Clone)]
enum US4 {
    US4_HitRead(Rc<str>),
    US4_HitRejected(US1),
}
fn hit_reading_of_3(mut v0: Rc<str>) -> US4 {
    let mut v1: bool = { let parts = v0.split(char::from(44u8)).collect::<Vec<_>>(); (2..=3).contains(&parts.len()) && parts[..2].iter().all(|value| !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit())) };
    if v1 {
        let mut v2: u64 = v0.split(char::from(44u8)).next().and_then(|line| line.parse::<u32>().ok()).filter(|line| *line > 0).map(|line| line as u64).unwrap_or(0);
        let mut v3: bool = v2 == 0u64;
        if v3 {
            let mut v4: US1 = US1::US1_InvalidHitLine;
            US4::US4_HitRejected(v4.clone())
        } else {
            let mut v6: bool = v0.split(char::from(44u8)).nth(1).map(|hits| hits.parse::<u64>().is_ok()).unwrap_or(false);
            if v6 {
                let mut v7: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
                let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
                let mut v10: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(44u8)).nth(1).unwrap_or(""));
                let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
                US4::US4_HitRead(v11.clone())
            } else {
                let mut v13: US1 = US1::US1_InvalidHitCount;
                US4::US4_HitRejected(v13.clone())
            }
        }
    } else {
        let mut v17: US1 = US1::US1_InvalidHitFields;
        US4::US4_HitRejected(v17.clone())
    }
}
fn method4(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
fn method5(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn lcov_from_2(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: US2, mut v4: Rc<str>) -> US0 {
    loop {
        let mut v5: bool = v1 < v2;
        if v5 {
            let mut v6: u64 = v1.wrapping_add(1u64);
            let mut v7: Rc<str> = std::rc::Rc::<str>::from(v0.lines().nth(v1 as usize).unwrap_or(""));
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SF:"); } LIT.with(|lit| lit.clone()) };
            let mut v9: bool = v7.starts_with(&*v8);
            let mut v28: US3 = if v9 {
                let mut v10: Rc<str> = std::rc::Rc::<str>::from(v7.strip_prefix(&*v8).unwrap_or(""));
                US3::US3_SourceLine(v10.clone())
            } else {
                let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("DA:"); } LIT.with(|lit| lit.clone()) };
                let mut v13: bool = v7.starts_with(&*v12);
                if v13 {
                    let mut v14: Rc<str> = std::rc::Rc::<str>::from(v7.strip_prefix(&*v12).unwrap_or(""));
                    US3::US3_HitLine(v14.clone())
                } else {
                    let mut v16: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("end_of_record"); } LIT.with(|lit| lit.clone()) };
                    if v16 {
                        US3::US3_EndLine
                    } else {
                        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("DA"); } LIT.with(|lit| lit.clone()) };
                        let mut v19: bool = v7.starts_with(&*v18);
                        let mut v22: bool = if v19 {
                            true
                        } else {
                            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SF"); } LIT.with(|lit| lit.clone()) };
                            let mut v21: bool = v7.starts_with(&*v20);
                            v21
                        };
                        if v22 {
                            US3::US3_MalformedLine
                        } else {
                            US3::US3_OtherLine
                        }
                    }
                }
            };
            match &v28 {
                US3::US3_EndLine => {
                    match &v3 {
                        US2::US2_BetweenSources => {
                            let mut v76: US1 = US1::US1_EndBeforeSource;
                            return US0::US0_LcovRejected(v76.clone());
                        }
                        US2::US2_InSource(v78) => {
                            let mut v78: Rc<str> = v78.clone();
                            let mut v79: US2 = US2::US2_BetweenSources;
                            (v0, v1, v2, v3, v4) = (v0.clone(), v6, v2, v79.clone(), v4.clone());
                            continue;
                        }
                    }
                }
                US3::US3_HitLine(v55) => {
                    let mut v55: Rc<str> = v55.clone();
                    match &v3 {
                        US2::US2_BetweenSources => {
                            let mut v56: US1 = US1::US1_HitBeforeSource;
                            return US0::US0_LcovRejected(v56.clone());
                        }
                        US2::US2_InSource(v58) => {
                            let mut v58: Rc<str> = v58.clone();
                            let mut v59: US4 = hit_reading_of_3(v55.clone());
                            match &v59 {
                                US4::US4_HitRead(v60) => {
                                    let mut v60: Rc<str> = v60.clone();
                                    let mut v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                                    let mut v62: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v61));
                                    let mut v63: Rc<str> = Rc::<str>::from(format!("{}{}", v62, v60));
                                    let mut v64: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let mut v68: Rc<str> = if v64 {
                                        v63.clone()
                                    } else {
                                        let mut v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                        let mut v66: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v65));
                                        let mut v67: Rc<str> = Rc::<str>::from(format!("{}{}", v66, v63));
                                        v67.clone()
                                    };
                                    (v0, v1, v2, v3, v4) = (v0.clone(), v6, v2, v3.clone(), v68.clone());
                                    continue;
                                }
                                US4::US4_HitRejected(v70) => {
                                    let mut v70: US1 = v70.clone();
                                    return US0::US0_LcovRejected(v70.clone());
                                }
                            }
                        }
                    }
                }
                US3::US3_MalformedLine => {
                    let mut v83: US1 = US1::US1_MalformedRecord;
                    return US0::US0_LcovRejected(v83.clone());
                }
                US3::US3_OtherLine => {
                    (v0, v1, v2, v3, v4) = (v0.clone(), v6, v2, v3.clone(), v4.clone());
                    continue;
                }
                US3::US3_SourceLine(v29) => {
                    let mut v29: Rc<str> = v29.clone();
                    match &v3 {
                        US2::US2_BetweenSources => {
                            let mut v33: i32 = (v29.clone().len() as i32);
                            let mut v34: i32 = 0i32;
                            let mut v35: i32 = method4(v29.clone(), v33, v34);
                            let mut v36: i32 = v33.wrapping_sub(1i32);
                            let mut v37: Rc<str> = string_slice(&v29.clone(), v35 as i64, v36 as i64);
                            let mut v38: i32 = (v37.clone().len() as i32);
                            let mut v39: i32 = method5(v37.clone(), v38);
                            let mut v40: Rc<str> = string_slice(&v37.clone(), 0i32 as i64, v39 as i64);
                            let mut v41: bool = v40.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            if v41 {
                                let mut v42: US1 = US1::US1_EmptySource;
                                return US0::US0_LcovRejected(v42.clone());
                            } else {
                                let mut v44: Rc<str> = std::rc::Rc::<str>::from(v29.replace(char::from(92u8), "/"));
                                let mut v45: US2 = US2::US2_InSource(v44.clone());
                                let mut v46: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v50: Rc<str> = if v46 {
                                    v44.clone()
                                } else {
                                    let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                    let mut v48: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v47));
                                    let mut v49: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v44));
                                    v49.clone()
                                };
                                (v0, v1, v2, v3, v4) = (v0.clone(), v6, v2, v45.clone(), v50.clone());
                                continue;
                            }
                        }
                        US2::US2_InSource(v30) => {
                            let mut v30: Rc<str> = v30.clone();
                            let mut v31: US1 = US1::US1_SourceBeforeEnd;
                            return US0::US0_LcovRejected(v31.clone());
                        }
                    }
                }
            }
        } else {
            match &v3 {
                US2::US2_BetweenSources => {
                    let mut v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                    let mut v95: bool = v4.contains(&*v94);
                    if v95 {
                        return US0::US0_LcovRecords(v4.clone());
                    } else {
                        let mut v97: US1 = US1::US1_NoLineRecords;
                        return US0::US0_LcovRejected(v97.clone());
                    }
                }
                US2::US2_InSource(v91) => {
                    let mut v91: Rc<str> = v91.clone();
                    let mut v92: US1 = US1::US1_UnterminatedSource;
                    return US0::US0_LcovRejected(v92.clone());
                }
            }
        }
    }
}
fn lcov_reading_of_1(mut v0: Rc<str>) -> US0 {
    let mut v1: u64 = 0u64;
    let mut v2: u64 = v0.lines().count() as u64;
    let mut v3: US2 = US2::US2_BetweenSources;
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    lcov_from_2(v0.clone(), v1, v2, v3.clone(), v4.clone())
}
fn lcov_error_0(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: US0 = lcov_reading_of_1(v0.clone());
    match &v1 {
        US0::US0_LcovRecords(v2) => {
            let mut v2: Rc<str> = v2.clone();
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
        US0::US0_LcovRejected(v4) => {
            let mut v4: US1 = v4.clone();
            match &v4 {
                US1::US1_EmptySource => {
                    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("coverage has an empty SF record"); } LIT.with(|lit| lit.clone()) };
                    v6.clone()
                }
                US1::US1_EndBeforeSource => {
                    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("coverage has end_of_record before SF"); } LIT.with(|lit| lit.clone()) };
                    v11.clone()
                }
                US1::US1_HitBeforeSource => {
                    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("coverage has DA before SF"); } LIT.with(|lit| lit.clone()) };
                    v7.clone()
                }
                US1::US1_InvalidHitCount => {
                    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("coverage has invalid DA hits"); } LIT.with(|lit| lit.clone()) };
                    v10.clone()
                }
                US1::US1_InvalidHitFields => {
                    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("coverage has invalid DA fields"); } LIT.with(|lit| lit.clone()) };
                    v8.clone()
                }
                US1::US1_InvalidHitLine => {
                    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("coverage has invalid DA line"); } LIT.with(|lit| lit.clone()) };
                    v9.clone()
                }
                US1::US1_MalformedRecord => {
                    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("coverage has a malformed source or line record"); } LIT.with(|lit| lit.clone()) };
                    v12.clone()
                }
                US1::US1_NoLineRecords => {
                    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("coverage input has no line records"); } LIT.with(|lit| lit.clone()) };
                    v14.clone()
                }
                US1::US1_SourceBeforeEnd => {
                    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("coverage has SF before end_of_record"); } LIT.with(|lit| lit.clone()) };
                    v5.clone()
                }
                US1::US1_UnterminatedSource => {
                    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("coverage has an unterminated source record"); } LIT.with(|lit| lit.clone()) };
                    v13.clone()
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        lcov_error_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn lcov_rows_6(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: US0 = lcov_reading_of_1(v0.clone());
    match &v1 {
        US0::US0_LcovRecords(v2) => {
            let mut v2: Rc<str> = v2.clone();
            v2.clone()
        }
        US0::US0_LcovRejected(v3) => {
            let mut v3: US1 = v3.clone();
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        lcov_rows_6(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_lcov_error(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
pub fn eoie_lcov_rows(v0: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0))
}
