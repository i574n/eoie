#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Replaced,
    US0_TemporaryWriteFailed,
    US0_RenameFailed,
    US0_ReadbackMismatch,
}
#[derive(Clone)]
enum US1 {
    US1_RootsEdited(Rc<str>),
    US1_RootsRefused(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_FileReadable(Rc<str>),
    US2_FileMissing(Rc<str>),
}
fn harvest_roots_from_2(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = HarvestRoot \""); } LIT.with(|lit| lit.clone()) };
        let mut v4: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v3)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let mut v5: bool = v4 < 0i32;
        if v5 {
            return v2.clone();
        } else {
            let mut v6: i32 = v4.wrapping_add(19i32);
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let mut v8: i32 = v0.get(v6 as usize..).and_then(|rest| rest.find(&*v7)).map(|found| (found + v6 as usize) as i32).unwrap_or(-1);
            let mut v9: Rc<str> = std::rc::Rc::<str>::from(v0.get(v6 as usize..v8 as usize).unwrap_or(""));
            let mut v10: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<str> = if v10 {
                v9.clone()
            } else {
                let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v11, &*v9, &*v12, &*v12].concat());
                v13.clone()
            };
            let mut v15: i32 = v8.wrapping_add(1i32);
            (v0, v1, v2) = (v0.clone(), v15, v14.clone());
            continue;
        }
    }
}
fn roots_printed_3(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: i32 = (v0.clone().len() as i32);
        let mut v4: bool = v3 <= v1;
        if v4 {
            return v2;
        } else {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v6: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v5)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
            let mut v7: bool = v6 < 0i32;
            let mut v8: i32 = if v7 {
                v3
            } else {
                v6
            };
            let mut v9: Rc<str> = std::rc::Rc::<str>::from(v0.get(v1 as usize..v8 as usize).unwrap_or(""));
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("root "); } LIT.with(|lit| lit.clone()) };
            let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v9));
            let mut v12: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v11.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
            let mut v13: i32 = v8.wrapping_add(1i32);
            let mut v14: i32 = v2.wrapping_add(1i32);
            (v0, v1, v2) = (v0.clone(), v13, v14);
            continue;
        }
    }
}
fn roots_receipt_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> i32 {
    let mut v3: i32 = 0i32;
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = harvest_roots_from_2(v1.clone(), v3, v4.clone());
    let mut v6: i32 = 0i32;
    let mut v7: i32 = 0i32;
    let mut v8: i32 = roots_printed_3(v5.clone(), v6, v7);
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie agile roots ok action="); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v0));
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" roots="); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v11));
    let mut v13: Rc<str> = std::rc::Rc::<str>::from((v8).to_string());
    let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
    let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v2));
    let mut v16: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v15.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v16
}
fn roots_written_5(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64) -> i32 {
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = std::rc::Rc::<str>::from(std::process::id().to_string().as_str());
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v5, &*v6, &*v7, &*v8].concat());
    let mut v10: bool = std::fs::write(&*v9, &*v3).is_ok();
    let mut v19: US0 = if v10 {
        let mut v11: bool = std::fs::rename(&*v9, &*v0).is_ok();
        if v11 {
            let mut v12: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v3);
            if v12 {
                US0::US0_Replaced
            } else {
                US0::US0_ReadbackMismatch
            }
        } else {
            US0::US0_RenameFailed
        }
    } else {
        US0::US0_TemporaryWriteFailed
    };
    match &v19 {
        US0::US0_Replaced => {
            let mut v20: u64 = v2.len() as u64;
            let mut v21: u64 = v3.len() as u64;
            let mut v22: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v4);
            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) };
            let mut v24: Rc<str> = std::rc::Rc::<str>::from((v20).to_string());
            let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) };
            let mut v26: Rc<str> = std::rc::Rc::<str>::from((v21).to_string());
            let mut v27: Rc<str> = std::rc::Rc::<str>::from([&*v23, &*v24, &*v25, &*v26, &*v8].concat());
            let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
            let mut v29: Rc<str> = std::rc::Rc::<str>::from((v22).to_string());
            let mut v30: Rc<str> = std::rc::Rc::<str>::from([&*v27, &*v28, &*v29, &*v8, &*v8].concat());
            roots_receipt_1(v1.clone(), v3.clone(), v30.clone())
        }
        _ => {
            let mut v37: Rc<str> = match &v19 {
                US0::US0_ReadbackMismatch => {
                    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                    v34.clone()
                }
                US0::US0_RenameFailed => {
                    let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                    v33.clone()
                }
                US0::US0_TemporaryWriteFailed => {
                    let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                    v32.clone()
                }
                _ => unreachable!(),
            };
            let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile roots rejected: "); } LIT.with(|lit| lit.clone()) };
            let mut v39: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v37));
            let mut v40: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v39 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v40
        }
    }
}
fn command_line_argument_count_7() -> i32 {
    let mut v0: i32 = (std::env::args().count() as i32) - 1;
    v0
}
fn argument_count_6() -> i32 {
    let mut v0: i32 = command_line_argument_count_7();
    let mut v1: bool = v0 > 0i32;
    if v1 {
        let mut v2: i32 = v0.wrapping_sub(1i32);
        v2
    } else {
        0i32
    }
}
fn rows_end_from_10(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v3: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v2)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let mut v4: bool = v3 < 0i32;
        let mut v7: i32 = if v4 {
            let mut v5: i32 = (v0.clone().len() as i32);
            v5
        } else {
            let mut v6: i32 = v3.wrapping_add(1i32);
            v6
        };
        let mut v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v1 as usize..v7 as usize).unwrap_or(""));
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = HarvestRoot \""); } LIT.with(|lit| lit.clone()) };
        let mut v10: bool = v8.contains(&*v9);
        if v10 {
            (v0, v1) = (v0.clone(), v7);
            continue;
        } else {
            return v1;
        }
    }
}
fn root_added_9(mut v0: Rc<str>, mut v1: Rc<str>) -> US1 {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from({ let forward = (v1).trim().replace(char::from(92u8), "/"); let trimmed = forward.trim_end_matches(char::from(47u8)); if trimmed.is_empty() { forward } else { trimmed.to_owned() } });
    let mut v3: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a harvest root needs a path"); } LIT.with(|lit| lit.clone()) };
        US1::US1_RootsRefused(v4.clone())
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let mut v7: bool = v2.contains(&*v6);
        if v7 {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a harvest root cannot contain a double quote"); } LIT.with(|lit| lit.clone()) };
            US1::US1_RootsRefused(v8.clone())
        } else {
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = HarvestRoot \""); } LIT.with(|lit| lit.clone()) };
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"\n"); } LIT.with(|lit| lit.clone()) };
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v2, &*v11, &*v12, &*v12].concat());
            let mut v14: bool = v0.contains(&*v13);
            if v14 {
                let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is already a harvest root"); } LIT.with(|lit| lit.clone()) };
                let mut v16: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v15, &*v12, &*v12, &*v12].concat());
                US1::US1_RootsRefused(v16.clone())
            } else {
                let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union harvest_root ="); } LIT.with(|lit| lit.clone()) };
                let mut v19: bool = v0.contains(&*v18);
                let mut v34: Rc<str> = if v19 {
                    v0.clone()
                } else {
                    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let mut v21: bool = v0.starts_with(&*v20);
                    let mut v28: i32 = if v21 {
                        0i32
                    } else {
                        let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\ninl "); } LIT.with(|lit| lit.clone()) };
                        let mut v23: i32 = v0.find(&*v22).map(|offset| offset as i32).unwrap_or(-1);
                        let mut v24: bool = v23 < 0i32;
                        if v24 {
                            let mut v25: i32 = (v0.clone().len() as i32);
                            v25
                        } else {
                            let mut v26: i32 = v23.wrapping_add(1i32);
                            v26
                        }
                    };
                    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union harvest_root =\n    | HarvestRoot :: string -> harvest_root\n\n"); } LIT.with(|lit| lit.clone()) };
                    let mut v30: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v28 as usize).unwrap_or(""));
                    let mut v31: i32 = (v0.clone().len() as i32);
                    let mut v32: Rc<str> = std::rc::Rc::<str>::from(v0.get(v28 as usize..v31 as usize).unwrap_or(""));
                    let mut v33: Rc<str> = std::rc::Rc::<str>::from([&*v30, &*v29, &*v32, &*v12, &*v12].concat());
                    v33.clone()
                };
                let mut v35: i32 = v34.find(&*v18).map(|offset| offset as i32).unwrap_or(-1);
                let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n\n"); } LIT.with(|lit| lit.clone()) };
                let mut v37: i32 = v34.get(v35 as usize..).and_then(|rest| rest.find(&*v36)).map(|found| (found + v35 as usize) as i32).unwrap_or(-1);
                let mut v38: i32 = v37.wrapping_add(2i32);
                let mut v39: i32 = rows_end_from_10(v34.clone(), v38);
                let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                let mut v41: Rc<str> = std::rc::Rc::<str>::from(format!("harvest_root_{}", (v2).to_ascii_lowercase().chars().map(|c| if c.is_ascii_alphanumeric() { c } else { char::from(95u8) }).collect::<String>().trim_matches(char::from(95u8))));
                let mut v42: Rc<str> = std::rc::Rc::<str>::from([&*v40, &*v41, &*v10, &*v2, &*v11].concat());
                let mut v43: Rc<str> = std::rc::Rc::<str>::from(v34.get(0i32 as usize..v39 as usize).unwrap_or(""));
                let mut v44: i32 = (v34.clone().len() as i32);
                let mut v45: Rc<str> = std::rc::Rc::<str>::from(v34.get(v39 as usize..v44 as usize).unwrap_or(""));
                let mut v46: Rc<str> = std::rc::Rc::<str>::from([&*v43, &*v42, &*v45, &*v12, &*v12].concat());
                US1::US1_RootsEdited(v46.clone())
            }
        }
    }
}
fn root_dropped_11(mut v0: Rc<str>, mut v1: Rc<str>) -> US1 {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from({ let forward = (v1).trim().replace(char::from(92u8), "/"); let trimmed = forward.trim_end_matches(char::from(47u8)); if trimmed.is_empty() { forward } else { trimmed.to_owned() } });
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = std::rc::Rc::<str>::from(format!("harvest_root_{}", (v2).to_ascii_lowercase().chars().map(|c| if c.is_ascii_alphanumeric() { c } else { char::from(95u8) }).collect::<String>().trim_matches(char::from(95u8))));
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = HarvestRoot \""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"\n"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v4, &*v5, &*v2, &*v6].concat());
    let mut v8: i32 = v0.find(&*v7).map(|offset| offset as i32).unwrap_or(-1);
    let mut v9: bool = v8 < 0i32;
    if v9 {
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is not a harvest root"); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v10, &*v11, &*v11, &*v11].concat());
        US1::US1_RootsRefused(v12.clone())
    } else {
        let mut v14: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v8 as usize).unwrap_or(""));
        let mut v15: i32 = (v7.clone().len() as i32);
        let mut v16: i32 = v8.wrapping_add(v15);
        let mut v17: i32 = (v0.clone().len() as i32);
        let mut v18: Rc<str> = std::rc::Rc::<str>::from(v0.get(v16 as usize..v17 as usize).unwrap_or(""));
        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v20: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v18, &*v19, &*v19, &*v19].concat());
        US1::US1_RootsEdited(v20.clone())
    }
}
fn edits_applied_8(mut v0: Rc<str>, mut v1: bool, mut v2: i32, mut v3: i32) -> US1 {
    loop {
        let mut v4: bool = v3 <= v2;
        if v4 {
            return US1::US1_RootsEdited(v0.clone());
        } else {
            let mut v6: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let mut v9: US1 = if v1 {
                root_added_9(v0.clone(), v6.clone())
            } else {
                root_dropped_11(v0.clone(), v6.clone())
            };
            match &v9 {
                US1::US1_RootsEdited(v10) => {
                    let mut v10: Rc<str> = v10.clone();
                    let mut v11: i32 = v2.wrapping_add(1i32);
                    (v0, v1, v2, v3) = (v10.clone(), v1, v11, v3);
                    continue;
                }
                US1::US1_RootsRefused(v13) => {
                    let mut v13: Rc<str> = v13.clone();
                    return US1::US1_RootsRefused(v13.clone());
                }
            }
        }
    }
}
fn roots_edited_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64) -> i32 {
    let mut v4: i32 = argument_count_6();
    let mut v5: bool = v4 <= 3i32;
    if v5 {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("add and drop need at least one path"); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile roots rejected: "); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile roots rejected: add and drop need at least one path"); } LIT.with(|lit| lit.clone()) };
        let mut v9: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v8 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v9
    } else {
        let mut v10: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("add"); } LIT.with(|lit| lit.clone()) };
        let mut v11: i32 = 3i32;
        let mut v12: i32 = argument_count_6();
        let mut v13: US1 = edits_applied_8(v2.clone(), v10, v11, v12);
        match &v13 {
            US1::US1_RootsEdited(v18) => {
                let mut v18: Rc<str> = v18.clone();
                roots_written_5(v0.clone(), v1.clone(), v2.clone(), v18.clone(), v3)
            }
            US1::US1_RootsRefused(v14) => {
                let mut v14: Rc<str> = v14.clone();
                let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile roots rejected: "); } LIT.with(|lit| lit.clone()) };
                let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v14));
                let mut v17: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v16 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v17
            }
        }
    }
}
fn roots_entry_0() -> i32 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v1: i32 = 1i32;
    let mut v2: Rc<str> = usize::try_from(v1).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v3, &*v4, &*v4, &*v4].concat());
    let mut v6: bool = std::path::Path::new(&*v5).is_file();
    let mut v9: US2 = if v6 {
        US2::US2_FileReadable(v5.clone())
    } else {
        US2::US2_FileMissing(v5.clone())
    };
    match &v9 {
        US2::US2_FileMissing(v10) => {
            let mut v10: Rc<str> = v10.clone();
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile roots rejected: "); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile roots rejected: state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let mut v14: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v13 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v14
        }
        US2::US2_FileReadable(v15) => {
            let mut v15: Rc<str> = v15.clone();
            let mut v16: Rc<str> = std::fs::read_to_string(&*v15).unwrap_or_default().into();
            let mut v17: i32 = 2i32;
            let mut v18: Rc<str> = usize::try_from(v17).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let mut v19: bool = v18.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v19 {
                let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list"); } LIT.with(|lit| lit.clone()) };
                let mut v21: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                let mut v22: Rc<str> = std::rc::Rc::<str>::from((v21).to_string());
                let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                let mut v24: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v22));
                roots_receipt_1(v20.clone(), v16.clone(), v24.clone())
            } else {
                let mut v26: bool = v18.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("add"); } LIT.with(|lit| lit.clone()) };
                let mut v28: bool = if v26 {
                    true
                } else {
                    let mut v27: bool = v18.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("drop"); } LIT.with(|lit| lit.clone()) };
                    v27
                };
                if v28 {
                    roots_edited_4(v5.clone(), v18.clone(), v16.clone(), v0)
                } else {
                    let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown action "); } LIT.with(|lit| lit.clone()) };
                    let mut v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v18));
                    let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (expected add or drop)"); } LIT.with(|lit| lit.clone()) };
                    let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
                    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile roots rejected: "); } LIT.with(|lit| lit.clone()) };
                    let mut v35: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v33));
                    let mut v36: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v35 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v36
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        roots_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_roots_run() -> i32 {
    closure0()()
}
