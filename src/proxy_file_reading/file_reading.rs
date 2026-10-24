#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_ReadShown(i32),
    US0_ReadRejected(Rc<str>),
}
#[derive(Clone)]
enum US1 {
    US1_Streamed(u64),
    US1_StreamFailed,
}
#[derive(Clone)]
enum US2 {
    US2_WholeFileShown,
    US2_LeadingBytesShown,
}
#[derive(Clone)]
enum US3 {
    US3_ContentClosed,
    US3_ContentOpen,
}
#[derive(Clone)]
enum US5 {
    US5_WholeFile,
    US5_LeadingBytes(u64),
}
#[derive(Clone)]
enum US4 {
    US4_Valid(US5),
    US4_Invalid,
}
fn command_line_argument_count_3() -> i32 {
    let mut v0: i32 = (std::env::args().count() as i32) - 1;
    v0
}
fn proxy_argument_count_2() -> i32 {
    let mut v0: i32 = command_line_argument_count_3();
    let mut v1: bool = v0 > 0i32;
    if v1 {
        let mut v2: i32 = v0.wrapping_sub(1i32);
        v2
    } else {
        0i32
    }
}
fn read_receipt_text_4(mut v0: Rc<str>, mut v1: u64, mut v2: i32, mut v3: u64) -> Rc<str> {
    let mut v4: u64 = eoie_rust_std_fs::rooted_regular_size(std::path::Path::new(&*v0)).unwrap_or(0);
    let mut v5: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie proxy fs-read ok bytes="); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v5));
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" shown="); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
    let mut v10: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
    let mut v12: bool = v1 < v4;
    let mut v15: US2 = if v12 {
        US2::US2_LeadingBytesShown
    } else {
        US2::US2_WholeFileShown
    };
    let mut v29: Rc<str> = match &v15 {
        US2::US2_LeadingBytesShown => {
            let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" truncated=true"); } LIT.with(|lit| lit.clone()) };
            v27.clone()
        }
        US2::US2_WholeFileShown => {
            let mut v16: bool = v2 < 0i32;
            let mut v22: US3 = if v16 {
                US3::US3_ContentClosed
            } else {
                let mut v18: bool = v2 == 10i32;
                if v18 {
                    US3::US3_ContentClosed
                } else {
                    US3::US3_ContentOpen
                }
            };
            match &v22 {
                US3::US3_ContentClosed => {
                    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v23.clone()
                }
                US3::US3_ContentOpen => {
                    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" no_final_newline=true"); } LIT.with(|lit| lit.clone()) };
                    v24.clone()
                }
            }
        }
    };
    let mut v30: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v29));
    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v32: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v31));
    let mut v33: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v3);
    let mut v34: Rc<str> = std::rc::Rc::<str>::from((v33).to_string());
    let mut v35: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v34));
    v35.clone()
}
fn file_reading_outcome_1() -> US0 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v1: i32 = proxy_argument_count_2();
    let mut v2: bool = v1 == 3;
    if v2 {
        let mut v3: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 1i32 as usize).unwrap_or_default());
        let mut v4: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 2i32 as usize).unwrap_or_default());
        let mut v5: Rc<str> = eoie_rust_std_fs::rooted_path(std::path::Path::new(&*v3), &*v4).map(|path| std::rc::Rc::<str>::from(path.to_string_lossy().as_ref())).unwrap_or_else(|_| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
        let mut v6: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v6 {
            let mut v7: Rc<str> = std::rc::Rc::<str>::from(eoie_rust_std_fs::rooted_path(std::path::Path::new(&*v3), &*v4).err().unwrap_or_default().as_str());
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-read path rejected: "); } LIT.with(|lit| lit.clone()) };
            let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v7));
            US0::US0_ReadRejected(v9.clone())
        } else {
            let mut v11: Rc<str> = std::rc::Rc::<str>::from(eoie_rust_std_fs::rooted_open_regular_read(std::path::Path::new(&*v5)).err().unwrap_or_default().as_str());
            let mut v12: bool = v11.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v12 {
                let mut v13: u64 = u64::MAX;
                let mut v14: u64 = eoie_rust_std_fs::rooted_open_regular_read(std::path::Path::new(&*v5)).ok().and_then(|file| std::io::copy(&mut std::io::Read::take(file, v13), &mut std::io::stdout().lock()).ok()).unwrap_or(u64::MAX);
                let mut v15: u64 = u64::MAX;
                let mut v16: bool = v14 == v15;
                let mut v19: US1 = if v16 {
                    US1::US1_StreamFailed
                } else {
                    US1::US1_Streamed(v14)
                };
                match &v19 {
                    US1::US1_StreamFailed => {
                        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-read failed while streaming: "); } LIT.with(|lit| lit.clone()) };
                        let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v5));
                        US0::US0_ReadRejected(v21.clone())
                    }
                    US1::US1_Streamed(v23) => {
                        let mut v23: u64 = *v23;
                        let mut v24: i32 = std::io::Write::flush(&mut std::io::stdout()).map(|_| 0i32).unwrap_or(2i32);
                        let mut v25: bool = v24 == 0i32;
                        if v25 {
                            let mut v26: i32 = eoie_rust_std_fs::rooted_open_regular_read(std::path::Path::new(&*v5)).ok().and_then(|mut file| { let offset = (v23).checked_sub(1)?; std::io::Seek::seek(&mut file, std::io::SeekFrom::Start(offset)).ok()?; let mut byte = [0u8; 1]; std::io::Read::read_exact(&mut file, &mut byte).ok()?; Some(i32::from(byte[0])) }).unwrap_or(-1);
                            let mut v27: Rc<str> = read_receipt_text_4(v5.clone(), v23, v26, v0);
                            let mut v28: i32 = std::io::Write::write_all(&mut std::io::stderr(), (v27.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                            US0::US0_ReadShown(v28)
                        } else {
                            let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-read could not flush stdout: "); } LIT.with(|lit| lit.clone()) };
                            let mut v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v5));
                            US0::US0_ReadRejected(v31.clone())
                        }
                    }
                }
            } else {
                let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-read failed: "); } LIT.with(|lit| lit.clone()) };
                let mut v37: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v11));
                US0::US0_ReadRejected(v37.clone())
            }
        }
    } else {
        let mut v41: bool = v1 == 4;
        if v41 {
            let mut v42: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 1i32 as usize).unwrap_or_default());
            let mut v43: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 2i32 as usize).unwrap_or_default());
            let mut v44: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 3i32 as usize).unwrap_or_default());
            let mut v45: bool = v44.parse::<u64>().is_ok();
            let mut v50: US4 = if v45 {
                let mut v46: u64 = v44.parse::<u64>().unwrap_or(0);
                let mut v47: US5 = US5::US5_LeadingBytes(v46);
                US4::US4_Valid(v47.clone())
            } else {
                US4::US4_Invalid
            };
            match &v50 {
                US4::US4_Invalid => {
                    let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("invalid byte limit"); } LIT.with(|lit| lit.clone()) };
                    US0::US0_ReadRejected(v91.clone())
                }
                US4::US4_Valid(v51) => {
                    let mut v51: US5 = v51.clone();
                    let mut v52: Rc<str> = eoie_rust_std_fs::rooted_path(std::path::Path::new(&*v42), &*v43).map(|path| std::rc::Rc::<str>::from(path.to_string_lossy().as_ref())).unwrap_or_else(|_| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                    let mut v53: bool = v52.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    if v53 {
                        let mut v54: Rc<str> = std::rc::Rc::<str>::from(eoie_rust_std_fs::rooted_path(std::path::Path::new(&*v42), &*v43).err().unwrap_or_default().as_str());
                        let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-read path rejected: "); } LIT.with(|lit| lit.clone()) };
                        let mut v56: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v54));
                        US0::US0_ReadRejected(v56.clone())
                    } else {
                        let mut v58: Rc<str> = std::rc::Rc::<str>::from(eoie_rust_std_fs::rooted_open_regular_read(std::path::Path::new(&*v52)).err().unwrap_or_default().as_str());
                        let mut v59: bool = v58.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        if v59 {
                            let mut v63: u64 = match &v51 {
                                US5::US5_LeadingBytes(v61) => {
                                    let mut v61: u64 = *v61;
                                    v61
                                }
                                US5::US5_WholeFile => {
                                    let mut v60: u64 = u64::MAX;
                                    v60
                                }
                            };
                            let mut v64: u64 = eoie_rust_std_fs::rooted_open_regular_read(std::path::Path::new(&*v52)).ok().and_then(|file| std::io::copy(&mut std::io::Read::take(file, v63), &mut std::io::stdout().lock()).ok()).unwrap_or(u64::MAX);
                            let mut v65: u64 = u64::MAX;
                            let mut v66: bool = v64 == v65;
                            let mut v69: US1 = if v66 {
                                US1::US1_StreamFailed
                            } else {
                                US1::US1_Streamed(v64)
                            };
                            match &v69 {
                                US1::US1_StreamFailed => {
                                    let mut v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-read failed while streaming: "); } LIT.with(|lit| lit.clone()) };
                                    let mut v71: Rc<str> = Rc::<str>::from(format!("{}{}", v70, v52));
                                    US0::US0_ReadRejected(v71.clone())
                                }
                                US1::US1_Streamed(v73) => {
                                    let mut v73: u64 = *v73;
                                    let mut v74: i32 = std::io::Write::flush(&mut std::io::stdout()).map(|_| 0i32).unwrap_or(2i32);
                                    let mut v75: bool = v74 == 0i32;
                                    if v75 {
                                        let mut v76: i32 = eoie_rust_std_fs::rooted_open_regular_read(std::path::Path::new(&*v52)).ok().and_then(|mut file| { let offset = (v73).checked_sub(1)?; std::io::Seek::seek(&mut file, std::io::SeekFrom::Start(offset)).ok()?; let mut byte = [0u8; 1]; std::io::Read::read_exact(&mut file, &mut byte).ok()?; Some(i32::from(byte[0])) }).unwrap_or(-1);
                                        let mut v77: Rc<str> = read_receipt_text_4(v52.clone(), v73, v76, v0);
                                        let mut v78: i32 = std::io::Write::write_all(&mut std::io::stderr(), (v77.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                                        US0::US0_ReadShown(v78)
                                    } else {
                                        let mut v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-read could not flush stdout: "); } LIT.with(|lit| lit.clone()) };
                                        let mut v81: Rc<str> = Rc::<str>::from(format!("{}{}", v80, v52));
                                        US0::US0_ReadRejected(v81.clone())
                                    }
                                }
                            }
                        } else {
                            let mut v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-read failed: "); } LIT.with(|lit| lit.clone()) };
                            let mut v87: Rc<str> = Rc::<str>::from(format!("{}{}", v86, v58));
                            US0::US0_ReadRejected(v87.clone())
                        }
                    }
                }
            }
        } else {
            let mut v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-read expects <root> <relative> [max-bytes]"); } LIT.with(|lit| lit.clone()) };
            US0::US0_ReadRejected(v95.clone())
        }
    }
}
fn file_reading_report_0() -> (i32, Rc<str>) {
    let mut v0: US0 = file_reading_outcome_1();
    match &v0 {
        US0::US0_ReadRejected(v3) => {
            let mut v3: Rc<str> = v3.clone();
            (2i32, v3.clone())
        }
        US0::US0_ReadShown(v1) => {
            let mut v1: i32 = *v1;
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (0i32, v2.clone())
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> (i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> (i32, Rc<str>)> = Rc::new(move || -> (i32, Rc<str>) {
        file_reading_report_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_file_reading_outcome() -> (i32, Rc<str>) {
    closure0()()
}
