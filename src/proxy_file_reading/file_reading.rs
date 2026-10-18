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
fn read_receipt_text_2(mut v0: Rc<str>, mut v1: u64, mut v2: i32, mut v3: u64) -> Rc<str> {
    let mut v4: u64 = eoie_rust_std_fs::rooted_regular_size(std::path::Path::new(&*v0)).unwrap_or(0);
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie proxy fs-read ok bytes="); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" shown="); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let mut v9: bool = v1 < v4;
    let mut v12: US2 = if v9 {
        US2::US2_LeadingBytesShown
    } else {
        US2::US2_WholeFileShown
    };
    let mut v26: Rc<str> = match &v12 {
        US2::US2_LeadingBytesShown => {
            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" truncated=true"); } LIT.with(|lit| lit.clone()) };
            v24.clone()
        }
        US2::US2_WholeFileShown => {
            let mut v13: bool = v2 < 0i32;
            let mut v19: US3 = if v13 {
                US3::US3_ContentClosed
            } else {
                let mut v15: bool = v2 == 10i32;
                if v15 {
                    US3::US3_ContentClosed
                } else {
                    US3::US3_ContentOpen
                }
            };
            match &v19 {
                US3::US3_ContentClosed => {
                    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v20.clone()
                }
                US3::US3_ContentOpen => {
                    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" no_final_newline=true"); } LIT.with(|lit| lit.clone()) };
                    v21.clone()
                }
            }
        }
    };
    let mut v27: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v6, &*v7, &*v8, &*v26].concat());
    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v29: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v3);
    let mut v30: Rc<str> = std::rc::Rc::<str>::from((v29).to_string());
    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v32: Rc<str> = std::rc::Rc::<str>::from([&*v27, &*v28, &*v30, &*v31, &*v31].concat());
    v32.clone()
}
fn file_reading_outcome_1() -> US0 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v1: i32 = std::env::args().skip(2).count() as i32;
    let mut v2: bool = v1 == 3;
    if v2 {
        let mut v3: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 1i32 as usize).unwrap_or_default());
        let mut v4: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 2i32 as usize).unwrap_or_default());
        let mut v5: Rc<str> = eoie_rust_std_fs::rooted_path(std::path::Path::new(&*v3), &*v4).map(|path| std::rc::Rc::<str>::from(path.to_string_lossy().as_ref())).unwrap_or_else(|_| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v7: bool = v5 == v6 ;
        if v7 {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-read path rejected: "); } LIT.with(|lit| lit.clone()) };
            let mut v9: Rc<str> = std::rc::Rc::<str>::from(eoie_rust_std_fs::rooted_path(std::path::Path::new(&*v3), &*v4).err().unwrap_or_default().as_str());
            let mut v10: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v9, &*v6, &*v6, &*v6].concat());
            US0::US0_ReadRejected(v10.clone())
        } else {
            let mut v12: Rc<str> = std::rc::Rc::<str>::from(eoie_rust_std_fs::rooted_open_regular_read(std::path::Path::new(&*v5)).err().unwrap_or_default().as_str());
            let mut v13: bool = v12 == v6 ;
            if v13 {
                let mut v14: u64 = u64::MAX;
                let mut v15: u64 = eoie_rust_std_fs::rooted_open_regular_read(std::path::Path::new(&*v5)).ok().and_then(|file| std::io::copy(&mut std::io::Read::take(file, v14), &mut std::io::stdout().lock()).ok()).unwrap_or(u64::MAX);
                let mut v16: u64 = u64::MAX;
                let mut v17: bool = v15 == v16;
                let mut v20: US1 = if v17 {
                    US1::US1_StreamFailed
                } else {
                    US1::US1_Streamed(v15)
                };
                match &v20 {
                    US1::US1_StreamFailed => {
                        let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-read failed while streaming: "); } LIT.with(|lit| lit.clone()) };
                        let mut v22: Rc<str> = std::rc::Rc::<str>::from([&*v21, &*v5, &*v6, &*v6, &*v6].concat());
                        US0::US0_ReadRejected(v22.clone())
                    }
                    US1::US1_Streamed(v24) => {
                        let mut v24: u64 = *v24;
                        let mut v25: i32 = std::io::Write::flush(&mut std::io::stdout()).map(|_| 0i32).unwrap_or(2i32);
                        let mut v26: bool = v25 == 0i32;
                        if v26 {
                            let mut v27: i32 = eoie_rust_std_fs::rooted_open_regular_read(std::path::Path::new(&*v5)).ok().and_then(|mut file| { let offset = (v24).checked_sub(1)?; std::io::Seek::seek(&mut file, std::io::SeekFrom::Start(offset)).ok()?; let mut byte = [0u8; 1]; std::io::Read::read_exact(&mut file, &mut byte).ok()?; Some(i32::from(byte[0])) }).unwrap_or(-1);
                            let mut v28: Rc<str> = read_receipt_text_2(v5.clone(), v24, v27, v0);
                            let mut v29: i32 = std::io::Write::write_all(&mut std::io::stderr(), (v28.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                            US0::US0_ReadShown(v29)
                        } else {
                            let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-read could not flush stdout: "); } LIT.with(|lit| lit.clone()) };
                            let mut v32: Rc<str> = std::rc::Rc::<str>::from([&*v31, &*v5, &*v6, &*v6, &*v6].concat());
                            US0::US0_ReadRejected(v32.clone())
                        }
                    }
                }
            } else {
                let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-read failed: "); } LIT.with(|lit| lit.clone()) };
                let mut v38: Rc<str> = std::rc::Rc::<str>::from([&*v37, &*v12, &*v6, &*v6, &*v6].concat());
                US0::US0_ReadRejected(v38.clone())
            }
        }
    } else {
        let mut v42: bool = v1 == 4;
        if v42 {
            let mut v43: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 1i32 as usize).unwrap_or_default());
            let mut v44: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 2i32 as usize).unwrap_or_default());
            let mut v45: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 3i32 as usize).unwrap_or_default());
            let mut v46: bool = v45.parse::<u64>().is_ok();
            let mut v51: US4 = if v46 {
                let mut v47: u64 = v45.parse::<u64>().unwrap_or(0);
                let mut v48: US5 = US5::US5_LeadingBytes(v47);
                US4::US4_Valid(v48.clone())
            } else {
                US4::US4_Invalid
            };
            match &v51 {
                US4::US4_Invalid => {
                    let mut v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("invalid byte limit"); } LIT.with(|lit| lit.clone()) };
                    US0::US0_ReadRejected(v93.clone())
                }
                US4::US4_Valid(v52) => {
                    let mut v52: US5 = v52.clone();
                    let mut v53: Rc<str> = eoie_rust_std_fs::rooted_path(std::path::Path::new(&*v43), &*v44).map(|path| std::rc::Rc::<str>::from(path.to_string_lossy().as_ref())).unwrap_or_else(|_| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                    let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v55: bool = v53 == v54 ;
                    if v55 {
                        let mut v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-read path rejected: "); } LIT.with(|lit| lit.clone()) };
                        let mut v57: Rc<str> = std::rc::Rc::<str>::from(eoie_rust_std_fs::rooted_path(std::path::Path::new(&*v43), &*v44).err().unwrap_or_default().as_str());
                        let mut v58: Rc<str> = std::rc::Rc::<str>::from([&*v56, &*v57, &*v54, &*v54, &*v54].concat());
                        US0::US0_ReadRejected(v58.clone())
                    } else {
                        let mut v60: Rc<str> = std::rc::Rc::<str>::from(eoie_rust_std_fs::rooted_open_regular_read(std::path::Path::new(&*v53)).err().unwrap_or_default().as_str());
                        let mut v61: bool = v60 == v54 ;
                        if v61 {
                            let mut v65: u64 = match &v52 {
                                US5::US5_LeadingBytes(v63) => {
                                    let mut v63: u64 = *v63;
                                    v63
                                }
                                US5::US5_WholeFile => {
                                    let mut v62: u64 = u64::MAX;
                                    v62
                                }
                            };
                            let mut v66: u64 = eoie_rust_std_fs::rooted_open_regular_read(std::path::Path::new(&*v53)).ok().and_then(|file| std::io::copy(&mut std::io::Read::take(file, v65), &mut std::io::stdout().lock()).ok()).unwrap_or(u64::MAX);
                            let mut v67: u64 = u64::MAX;
                            let mut v68: bool = v66 == v67;
                            let mut v71: US1 = if v68 {
                                US1::US1_StreamFailed
                            } else {
                                US1::US1_Streamed(v66)
                            };
                            match &v71 {
                                US1::US1_StreamFailed => {
                                    let mut v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-read failed while streaming: "); } LIT.with(|lit| lit.clone()) };
                                    let mut v73: Rc<str> = std::rc::Rc::<str>::from([&*v72, &*v53, &*v54, &*v54, &*v54].concat());
                                    US0::US0_ReadRejected(v73.clone())
                                }
                                US1::US1_Streamed(v75) => {
                                    let mut v75: u64 = *v75;
                                    let mut v76: i32 = std::io::Write::flush(&mut std::io::stdout()).map(|_| 0i32).unwrap_or(2i32);
                                    let mut v77: bool = v76 == 0i32;
                                    if v77 {
                                        let mut v78: i32 = eoie_rust_std_fs::rooted_open_regular_read(std::path::Path::new(&*v53)).ok().and_then(|mut file| { let offset = (v75).checked_sub(1)?; std::io::Seek::seek(&mut file, std::io::SeekFrom::Start(offset)).ok()?; let mut byte = [0u8; 1]; std::io::Read::read_exact(&mut file, &mut byte).ok()?; Some(i32::from(byte[0])) }).unwrap_or(-1);
                                        let mut v79: Rc<str> = read_receipt_text_2(v53.clone(), v75, v78, v0);
                                        let mut v80: i32 = std::io::Write::write_all(&mut std::io::stderr(), (v79.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                                        US0::US0_ReadShown(v80)
                                    } else {
                                        let mut v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-read could not flush stdout: "); } LIT.with(|lit| lit.clone()) };
                                        let mut v83: Rc<str> = std::rc::Rc::<str>::from([&*v82, &*v53, &*v54, &*v54, &*v54].concat());
                                        US0::US0_ReadRejected(v83.clone())
                                    }
                                }
                            }
                        } else {
                            let mut v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-read failed: "); } LIT.with(|lit| lit.clone()) };
                            let mut v89: Rc<str> = std::rc::Rc::<str>::from([&*v88, &*v60, &*v54, &*v54, &*v54].concat());
                            US0::US0_ReadRejected(v89.clone())
                        }
                    }
                }
            }
        } else {
            let mut v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-read expects <root> <relative> [max-bytes]"); } LIT.with(|lit| lit.clone()) };
            US0::US0_ReadRejected(v97.clone())
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
