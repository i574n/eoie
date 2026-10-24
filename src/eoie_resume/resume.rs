#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_NextWork,
    US1_RecentJobs,
    US1_MachineWindows,
    US1_AuthorNotes,
}
#[derive(Clone)]
enum US2 {
    US2_ScanPlain,
    US2_ScanQuoted,
    US2_ScanEscaped,
}
fn command_line_argument_count_2() -> i32 {
    let mut v0: i32 = (std::env::args().count() as i32) - 1;
    v0
}
fn command_line_argument_3(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::args().nth((v0 + 1) as usize).unwrap_or_default());
    v1.clone()
}
fn argument_1(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_sub(1i32);
    let mut v2: bool = v1 >= 0i32;
    let mut v5: bool = if v2 {
        let mut v3: i32 = command_line_argument_count_2();
        let mut v4: bool = v1 < v3;
        v4
    } else {
        false
    };
    let mut v9: US0 = if v5 {
        let mut v6: Rc<str> = command_line_argument_3(v1);
        US0::US0_Some(v6.clone())
    } else {
        US0::US0_None
    };
    match &v9 {
        US0::US0_None => {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US0::US0_Some(v10) => {
            let mut v10: Rc<str> = v10.clone();
            v10.clone()
        }
    }
}
fn declared_from_7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        let mut v3: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v1)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
        let mut v4: bool = v3 < 0i32;
        if v4 {
            return v3;
        } else {
            let mut v5: i32 = v0.get(..v3 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let mut v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v3 as usize).unwrap_or(""));
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let mut v8: bool = v6.starts_with(&*v7);
            if v8 {
                return v3;
            } else {
                let mut v9: i32 = v3.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1.clone(), v9);
                continue;
            }
        }
    }
}
fn field_end_10(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US2) -> i32 {
    loop {
        let mut v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let mut v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = match &v3 {
                US2::US2_ScanPlain => {
                    let mut v6: bool = v5 == 44i32;
                    if v6 {
                        true
                    } else {
                        let mut v7: bool = v5 == 41i32;
                        v7
                    }
                }
                _ => {
                    false
                }
            };
            if v9 {
                return v1;
            } else {
                let mut v10: i32 = v1.wrapping_add(1i32);
                let mut v25: US2 = match &v3 {
                    US2::US2_ScanEscaped => {
                        US2::US2_ScanQuoted
                    }
                    US2::US2_ScanPlain => {
                        let mut v19: bool = v5 == 34i32;
                        if v19 {
                            US2::US2_ScanQuoted
                        } else {
                            US2::US2_ScanPlain
                        }
                    }
                    US2::US2_ScanQuoted => {
                        let mut v12: bool = v5 == 92i32;
                        if v12 {
                            US2::US2_ScanEscaped
                        } else {
                            let mut v14: bool = v5 == 34i32;
                            if v14 {
                                US2::US2_ScanPlain
                            } else {
                                US2::US2_ScanQuoted
                            }
                        }
                    }
                };
                (v0, v1, v2, v3) = (v0.clone(), v10, v2, v25.clone());
                continue;
            }
        }
    }
}
fn skip_spaces_11(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
        let mut v3: bool = v2 == 32i32;
        if v3 {
            let mut v4: i32 = v1.wrapping_add(1i32);
            (v0, v1) = (v0.clone(), v4);
            continue;
        } else {
            return v1;
        }
    }
}
fn field_start_9(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let mut v5: US2 = US2::US2_ScanPlain;
            let mut v6: i32 = field_end_10(v0.clone(), v1, v2, v5.clone());
            let mut v7: i32 = v6.wrapping_add(1i32);
            let mut v8: i32 = skip_spaces_11(v0.clone(), v7);
            let mut v9: i32 = v3.wrapping_sub(1i32);
            (v0, v1, v2, v3) = (v0.clone(), v8, v2, v9);
            continue;
        }
    }
}
fn unescaped_from_13(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let mut v5: bool = v3 <= v1;
        if v5 {
            let mut v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v3 as usize).unwrap_or(""));
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v6, &*v7, &*v7, &*v7].concat());
            return v8.clone();
        } else {
            let mut v9: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v10: bool = v9 == 92i32;
            if v10 {
                let mut v11: i32 = v1.wrapping_add(1i32);
                let mut v12: i32 = v11.wrapping_add(1i32);
                let mut v13: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v1 as usize).unwrap_or(""));
                let mut v14: i32 = v0.as_bytes().get(v11 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let mut v15: bool = v14 == 110i32;
                let mut v24: Rc<str> = if v15 {
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    v16.clone()
                } else {
                    let mut v17: bool = v14 == 114i32;
                    if v17 {
                        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                        v18.clone()
                    } else {
                        let mut v19: bool = v14 == 116i32;
                        if v19 {
                            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            v20.clone()
                        } else {
                            let mut v21: Rc<str> = std::rc::Rc::<str>::from(v0.get(v11 as usize..v12 as usize).unwrap_or(""));
                            v21.clone()
                        }
                    }
                };
                let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v26: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v13, &*v24, &*v25, &*v25].concat());
                (v0, v1, v2, v3, v4) = (v0.clone(), v12, v12, v3, v26.clone());
                continue;
            } else {
                let mut v28: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2, v3, v4) = (v0.clone(), v28, v2, v3, v4.clone());
                continue;
            }
        }
    }
}
fn unquoted_12(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    let mut v3: Rc<str> = std::rc::Rc::<str>::from(v0.get(1i32 as usize..v2 as usize).unwrap_or(""));
    let mut v4: i32 = 0i32;
    let mut v5: i32 = 0i32;
    let mut v6: i32 = (v3.clone().len() as i32);
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    unescaped_from_13(v3.clone(), v4, v5, v6, v7.clone())
}
fn note_at_8(mut v0: Rc<str>, mut v1: i32) -> (Rc<str>, Rc<str>, Rc<str>) {
    let mut v2: i32 = v1 + 6i32;
    let mut v3: i32 = (v0.clone().len() as i32);
    let mut v4: i32 = 2i32;
    let mut v5: i32 = field_start_9(v0.clone(), v2, v3, v4);
    let mut v6: US2 = US2::US2_ScanPlain;
    let mut v7: i32 = field_end_10(v0.clone(), v5, v3, v6.clone());
    let mut v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v7 as usize).unwrap_or(""));
    let mut v9: Rc<str> = unquoted_12(v8.clone());
    let mut v10: i32 = v1 + 6i32;
    let mut v11: i32 = 3i32;
    let mut v12: i32 = field_start_9(v0.clone(), v10, v3, v11);
    let mut v13: US2 = US2::US2_ScanPlain;
    let mut v14: i32 = field_end_10(v0.clone(), v12, v3, v13.clone());
    let mut v15: Rc<str> = std::rc::Rc::<str>::from(v0.get(v12 as usize..v14 as usize).unwrap_or(""));
    let mut v16: Rc<str> = unquoted_12(v15.clone());
    let mut v17: i32 = v1 + 6i32;
    let mut v18: i32 = 4i32;
    let mut v19: i32 = field_start_9(v0.clone(), v17, v3, v18);
    let mut v20: US2 = US2::US2_ScanPlain;
    let mut v21: i32 = field_end_10(v0.clone(), v19, v3, v20.clone());
    let mut v22: Rc<str> = std::rc::Rc::<str>::from(v0.get(v19 as usize..v21 as usize).unwrap_or(""));
    let mut v23: Rc<str> = unquoted_12(v22.clone());
    (v9.clone(), v16.clone(), v23.clone())
}
fn authored_row_14(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v4));
    let mut v6: i32 = v1 + 6i32;
    let mut v7: i32 = (v0.clone().len() as i32);
    let mut v8: i32 = 0i32;
    let mut v9: i32 = field_start_9(v0.clone(), v6, v7, v8);
    let mut v10: US2 = US2::US2_ScanPlain;
    let mut v11: i32 = field_end_10(v0.clone(), v9, v7, v10.clone());
    let mut v12: Rc<str> = std::rc::Rc::<str>::from(v0.get(v9 as usize..v11 as usize).unwrap_or(""));
    let mut v13: Rc<str> = unquoted_12(v12.clone());
    let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v13));
    let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v4));
    let mut v16: Rc<str> = std::rc::Rc::<str>::from((v2).replace(char::from(10u8), " ").chars().take(160i32 as usize).collect::<String>().as_str());
    let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
    v17.clone()
}
fn authored_rows_from_6(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let mut v5: i32 = declared_from_7(v0.clone(), v1.clone(), v3);
        let mut v6: bool = v5 < 0i32;
        if v6 {
            return v4.clone();
        } else {
            let (mut v7, mut v8, mut v9): (Rc<str>, Rc<str>, Rc<str>) = note_at_8(v0.clone(), v5);
            let mut v10: bool = v8.clone() == v2.clone();
            let mut v15: Rc<str> = if v10 {
                let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v11));
                let mut v13: Rc<str> = authored_row_14(v0.clone(), v5, v9.clone(), v7.clone());
                let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
                v14.clone()
            } else {
                v4.clone()
            };
            let mut v16: i32 = v5.wrapping_add(1i32);
            (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v2.clone(), v16, v15.clone());
            continue;
        }
    }
}
fn author_notes_5(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(format!("{}/state/agile.spi", &*v0)).unwrap_or_default().as_str());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note (\""); } LIT.with(|lit| lit.clone()) };
    let mut v7: i32 = 0i32;
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = authored_rows_from_6(v3.clone(), v6.clone(), v2.clone(), v7, v8.clone());
    let mut v10: Rc<str> = std::rc::Rc::<str>::from({ let mut rows = (v9).split(char::from(10u8)).filter(|row| !row.is_empty()).collect::<Vec<_>>(); rows.sort_by(|a, b| b.cmp(a)); rows.truncate(8i32 as usize); rows.join(&char::from(10u8).to_string()) }.as_str());
    let mut v11: bool = v10.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v11 {
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no notes by "); } LIT.with(|lit| lit.clone()) };
        let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v2));
        v13.clone()
    } else {
        v10.clone()
    }
}
fn section_shown_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: US1, mut v4: i32) -> i32 {
    let mut v7: bool = match &v3 {
        US1::US1_AuthorNotes => {
            let mut v5: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v5 == false;
            v6
        }
        _ => {
            true
        }
    };
    if v7 {
        let mut v15: Rc<str> = match &v3 {
            US1::US1_AuthorNotes => {
                let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("== recent notes by author (agile notes --as)"); } LIT.with(|lit| lit.clone()) };
                v11.clone()
            }
            US1::US1_MachineWindows => {
                let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("== machine windows (window list)"); } LIT.with(|lit| lit.clone()) };
                v10.clone()
            }
            US1::US1_NextWork => {
                let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("== next work (agile next, WSJF)"); } LIT.with(|lit| lit.clone()) };
                v8.clone()
            }
            US1::US1_RecentJobs => {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("== recent jobs (job list)"); } LIT.with(|lit| lit.clone()) };
                v9.clone()
            }
        };
        println!("{}", &*v15);
        let mut v33: Rc<str> = match &v3 {
            US1::US1_AuthorNotes => {
                author_notes_5(v0.clone(), v1.clone(), v2.clone())
            }
            _ => {
                let mut v31: Rc<str> = match &v3 {
                    US1::US1_MachineWindows => {
                        let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("window\nlist"); } LIT.with(|lit| lit.clone()) };
                        v28.clone()
                    }
                    US1::US1_NextWork => {
                        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile\nnext\n"); } LIT.with(|lit| lit.clone()) };
                        let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v0));
                        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n8"); } LIT.with(|lit| lit.clone()) };
                        let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v19));
                        let mut v21: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v25: Rc<str> = if v21 {
                            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v22.clone()
                        } else {
                            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n--team\n"); } LIT.with(|lit| lit.clone()) };
                            let mut v24: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v1));
                            v24.clone()
                        };
                        let mut v26: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v25));
                        v26.clone()
                    }
                    US1::US1_RecentJobs => {
                        let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job\nlist\n12"); } LIT.with(|lit| lit.clone()) };
                        v27.clone()
                    }
                    _ => unreachable!(),
                };
                let mut v32: Rc<str> = std::rc::Rc::<str>::from(std::env::current_exe().ok().and_then(|exe| std::process::Command::new(exe).args((v31).split(char::from(10u8)).filter(|item| !item.is_empty())).output().ok()).map(|out| String::from_utf8_lossy(&out.stdout).trim_end().to_owned()).unwrap_or_default().as_str());
                v32.clone()
            }
        };
        println!("{}", &*v33);
        let mut v34: i32 = v4.wrapping_add(1i32);
        v34
    } else {
        v4
    }
}
fn resume_run_0() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: Rc<str> = argument_1(v0);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--team"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = std::rc::Rc::<str>::from({ let args = std::env::args().collect::<Vec<_>>(); args.iter().position(|argument| argument == &*v2).and_then(|at| args.get(at + 1)).cloned().unwrap_or_default() }.as_str());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--as"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = std::rc::Rc::<str>::from({ let args = std::env::args().collect::<Vec<_>>(); args.iter().position(|argument| argument == &*v4).and_then(|at| args.get(at + 1)).cloned().unwrap_or_default() }.as_str());
    let mut v6: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v9: bool = if v6 {
        true
    } else {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
        let mut v8: bool = v1.starts_with(&*v7);
        v8
    };
    if v9 {
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: usage: eoie resume <backlog-root> [--team <team>] [--as <author>]"); } LIT.with(|lit| lit.clone()) };
        eprintln!("{}", &*v10);
        2i32
    } else {
        let mut v11: US1 = US1::US1_NextWork;
        let mut v12: i32 = 0i32;
        let mut v13: i32 = section_shown_4(v1.clone(), v3.clone(), v5.clone(), v11.clone(), v12);
        let mut v14: US1 = US1::US1_RecentJobs;
        let mut v15: i32 = section_shown_4(v1.clone(), v3.clone(), v5.clone(), v14.clone(), v13);
        let mut v16: US1 = US1::US1_MachineWindows;
        let mut v17: i32 = section_shown_4(v1.clone(), v3.clone(), v5.clone(), v16.clone(), v15);
        let mut v18: US1 = US1::US1_AuthorNotes;
        let mut v19: i32 = section_shown_4(v1.clone(), v3.clone(), v5.clone(), v18.clone(), v17);
        let mut v20: Rc<str> = std::rc::Rc::<str>::from((v19).to_string().as_str());
        let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie resume ok sections="); } LIT.with(|lit| lit.clone()) };
        let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v20));
        let mut v23: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v27: Rc<str> = if v23 {
            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v24.clone()
        } else {
            let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" team="); } LIT.with(|lit| lit.clone()) };
            let mut v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v3));
            v26.clone()
        };
        let mut v28: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v32: Rc<str> = if v28 {
            let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v29.clone()
        } else {
            let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" author="); } LIT.with(|lit| lit.clone()) };
            let mut v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v5));
            v31.clone()
        };
        let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v32));
        let mut v34: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v33));
        let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" unix="); } LIT.with(|lit| lit.clone()) };
        let mut v36: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v35));
        let mut v37: Rc<str> = std::rc::Rc::<str>::from(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0).to_string().as_str());
        let mut v38: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v37));
        println!("{}", &*v38);
        0i32
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        resume_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_resume_run() -> i32 {
    closure0()()
}
