#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
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
                let mut v9: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(44u8)).nth(1).unwrap_or(""));
                let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v8, &*v9, &*v10, &*v10].concat());
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
                            let mut v68: US1 = US1::US1_EndBeforeSource;
                            return US0::US0_LcovRejected(v68.clone());
                        }
                        US2::US2_InSource(v70) => {
                            let mut v70: Rc<str> = v70.clone();
                            let mut v71: US2 = US2::US2_BetweenSources;
                            (v0, v1, v2, v3, v4) = (v0.clone(), v6, v2, v71.clone(), v4.clone());
                            continue;
                        }
                    }
                }
                US3::US3_HitLine(v48) => {
                    let mut v48: Rc<str> = v48.clone();
                    match &v3 {
                        US2::US2_BetweenSources => {
                            let mut v49: US1 = US1::US1_HitBeforeSource;
                            return US0::US0_LcovRejected(v49.clone());
                        }
                        US2::US2_InSource(v51) => {
                            let mut v51: Rc<str> = v51.clone();
                            let mut v52: US4 = hit_reading_of_3(v48.clone());
                            match &v52 {
                                US4::US4_HitRead(v53) => {
                                    let mut v53: Rc<str> = v53.clone();
                                    let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                                    let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let mut v56: Rc<str> = std::rc::Rc::<str>::from([&*v51, &*v54, &*v53, &*v55, &*v55].concat());
                                    let mut v57: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let mut v60: Rc<str> = if v57 {
                                        v56.clone()
                                    } else {
                                        let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                        let mut v59: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v58, &*v56, &*v55, &*v55].concat());
                                        v59.clone()
                                    };
                                    (v0, v1, v2, v3, v4) = (v0.clone(), v6, v2, v3.clone(), v60.clone());
                                    continue;
                                }
                                US4::US4_HitRejected(v62) => {
                                    let mut v62: US1 = v62.clone();
                                    return US0::US0_LcovRejected(v62.clone());
                                }
                            }
                        }
                    }
                }
                US3::US3_MalformedLine => {
                    let mut v75: US1 = US1::US1_MalformedRecord;
                    return US0::US0_LcovRejected(v75.clone());
                }
                US3::US3_OtherLine => {
                    (v0, v1, v2, v3, v4) = (v0.clone(), v6, v2, v3.clone(), v4.clone());
                    continue;
                }
                US3::US3_SourceLine(v29) => {
                    let mut v29: Rc<str> = v29.clone();
                    match &v3 {
                        US2::US2_BetweenSources => {
                            let mut v33: Rc<str> = std::rc::Rc::<str>::from(v29.trim());
                            let mut v34: bool = v33.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            if v34 {
                                let mut v35: US1 = US1::US1_EmptySource;
                                return US0::US0_LcovRejected(v35.clone());
                            } else {
                                let mut v37: Rc<str> = std::rc::Rc::<str>::from(v29.replace(char::from(92u8), "/"));
                                let mut v38: US2 = US2::US2_InSource(v37.clone());
                                let mut v39: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v43: Rc<str> = if v39 {
                                    v37.clone()
                                } else {
                                    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                    let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let mut v42: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v40, &*v37, &*v41, &*v41].concat());
                                    v42.clone()
                                };
                                (v0, v1, v2, v3, v4) = (v0.clone(), v6, v2, v38.clone(), v43.clone());
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
                    let mut v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                    let mut v87: bool = v4.contains(&*v86);
                    if v87 {
                        return US0::US0_LcovRecords(v4.clone());
                    } else {
                        let mut v89: US1 = US1::US1_NoLineRecords;
                        return US0::US0_LcovRejected(v89.clone());
                    }
                }
                US2::US2_InSource(v83) => {
                    let mut v83: Rc<str> = v83.clone();
                    let mut v84: US1 = US1::US1_UnterminatedSource;
                    return US0::US0_LcovRejected(v84.clone());
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
fn lcov_rows_4(mut v0: Rc<str>) -> Rc<str> {
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
        lcov_rows_4(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_lcov_error(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
pub fn eoie_lcov_rows(v0: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0))
}
