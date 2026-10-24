#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_PrintCensus,
    US0_WriteBaseline(Rc<str>),
    US0_MergeBaseline(Rc<str>),
    US0_CheckRatchet(Rc<str>),
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_Some(i32),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_SpiralFile,
    US3_Directory,
}
#[derive(Clone)]
enum US4 {
    US4_MayNotFall,
    US4_MayNotRise,
    US4_Unranked,
}
#[derive(Clone)]
enum UH1 {
    UH1_NoCounts,
    UH1_CountThen(Rc<str>, US4, i32, Rc<UH1>),
}
#[derive(Clone)]
enum US5 {
    US5_Held,
    US5_Regressed(Rc<str>),
}
fn text_written_3(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: bool = std::fs::write(&*v0, (v1).as_bytes()).is_ok();
    v2
}
fn stdout_line_4(mut v0: Rc<str>) -> () {
    println!("{}", &*v0);
    ()
}
fn stderr_line_5(mut v0: Rc<str>) -> () {
    eprintln!("{}", &*v0);
    ()
}
fn baseline_saved_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> i32 {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union census_row = CensusCount : string * string * i32 * i32\n\n"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v1, &*v4, &*v5, &*v5].concat());
    let mut v7: bool = text_written_3(v0.clone(), v6.clone());
    if v7 {
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie standards baseline ok packages="); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" path="); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v2, &*v9, &*v0, &*v5].concat());
        stdout_line_4(v10.clone());
        0i32
    } else {
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: standards could not write "); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v0, &*v5, &*v5, &*v5].concat());
        stderr_line_5(v12.clone());
        2i32
    }
}
fn count_text_6(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string().as_str());
    v1.clone()
}
fn file_text_7(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default().as_str());
    v1.clone()
}
fn quote_9() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(char::from(34u8).to_string().as_str());
    v0.clone()
}
fn method10(mut v0: Option<Rc<str>>) -> Option<Rc<str>> {
    v0.clone()
}
fn closure1() -> Rc<dyn Fn((Rc<str>)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((Rc<str>)) -> US1> = Rc::new(move |mut v0: (Rc<str>)| -> US1 {
        let mut v1: Rc<str> = (v0);
        US1::US1_Some(v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure2(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US2> {
    Rc::new(move |mut v2: Rc<UH0>| -> US2 {
        match &*v2 {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4.clone();
                let mut v5: Rc<UH0> = v5.clone();
                let mut v6: bool = v4.clone() == v0.clone();
                if v6 {
                    US2::US2_Some(v1)
                } else {
                    let mut v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US2> = method11(v0.clone(), v8);
                    v9(v5.clone())
                }
            }
            UH0::UH0_Nil => {
                US2::US2_None
            }
        }
    })
}
fn method11(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US2> {
    closure2(v0.clone(), v1)
}
fn appended_12(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v2 {
        v1.clone()
    } else {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v3, &*v1, &*v4, &*v4].concat());
        v5.clone()
    }
}
fn kept_rows_8(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH0>, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let mut v4: i32 = (v0.clone().borrow().len() as i32);
        let mut v5: bool = v1 >= v4;
        if v5 {
            return v3.clone();
        } else {
            let mut v6: Rc<str> = v0.clone().borrow()[v1 as usize].clone();
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl census_"); } LIT.with(|lit| lit.clone()) };
            let mut v8: bool = v6.starts_with(&*v7);
            let mut v27: bool = if v8 {
                let mut v9: Rc<str> = quote_9();
                let mut v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v6.split(&*v9).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
                let mut v11: Option<Rc<str>> = (if 1i32 < 0 { None } else { v10.borrow().get(1i32 as usize).cloned() });
                let mut v12: Option<Rc<str>> = method10(v11.clone());
                let mut v13: Rc<dyn Fn((Rc<str>)) -> US1> = closure1();
                let mut v14: Option<US1> = v12.map(|x| v13(x));
                let mut v15: US1 = US1::US1_None;
                let mut v16: US1 = v14.unwrap_or(v15);
                let mut v20: Rc<str> = match &v16 {
                    US1::US1_None => {
                        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v18.clone()
                    }
                    US1::US1_Some(v17) => {
                        let mut v17: Rc<str> = v17.clone();
                        v17.clone()
                    }
                };
                let mut v21: i32 = 0i32;
                let mut v22: Rc<dyn Fn(Rc<UH0>) -> US2> = method11(v20.clone(), v21);
                let mut v23: US2 = v22(v2.clone());
                let mut v24: bool = match &v23 {
                    US2::US2_None => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let mut v25: bool = v24 != true;
                let mut v26: bool = v25 == false;
                v26
            } else {
                false
            };
            let mut v28: i32 = v1.wrapping_add(1i32);
            let mut v30: Rc<str> = if v27 {
                appended_12(v3.clone(), v6.clone())
            } else {
                v3.clone()
            };
            (v0, v1, v2, v3) = (v0.clone(), v28, v2.clone(), v30.clone());
            continue;
        }
    }
}
fn census_report_1(mut v0: US0, mut v1: i32, mut v2: i32, mut v3: Rc<str>, mut v4: Rc<UH0>) -> i32 {
    let mut v5: Rc<str> = count_text_6(v1);
    match &v0 {
        US0::US0_CheckRatchet(v20) => {
            let mut v20: Rc<str> = v20.clone();
            let mut v21: bool = v2 == 0i32;
            let mut v24: Rc<str> = if v21 {
                let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
                v22.clone()
            } else {
                let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
                v23.clone()
            };
            let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie standards ratchet "); } LIT.with(|lit| lit.clone()) };
            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" packages="); } LIT.with(|lit| lit.clone()) };
            let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" regressions="); } LIT.with(|lit| lit.clone()) };
            let mut v28: Rc<str> = count_text_6(v2);
            let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v30: Rc<str> = std::rc::Rc::<str>::from([&*v26, &*v5, &*v27, &*v28, &*v29].concat());
            let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" baseline="); } LIT.with(|lit| lit.clone()) };
            let mut v32: Rc<str> = std::rc::Rc::<str>::from([&*v25, &*v24, &*v30, &*v31, &*v20].concat());
            stdout_line_4(v32.clone());
            if v21 {
                0i32
            } else {
                1i32
            }
        }
        US0::US0_MergeBaseline(v11) => {
            let mut v11: Rc<str> = v11.clone();
            let mut v12: Rc<str> = file_text_7(v11.clone());
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v12.split(&*v13).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
            let mut v15: i32 = 0i32;
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v17: Rc<str> = kept_rows_8(v14.clone(), v15, v4.clone(), v16.clone());
            let mut v18: Rc<str> = appended_12(v17.clone(), v3.clone());
            baseline_saved_2(v11.clone(), v18.clone(), v5.clone())
        }
        US0::US0_PrintCensus => {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie standards census ok packages="); } LIT.with(|lit| lit.clone()) };
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v5, &*v7, &*v7, &*v7].concat());
            stdout_line_4(v8.clone());
            0i32
        }
        US0::US0_WriteBaseline(v9) => {
            let mut v9: Rc<str> = v9.clone();
            baseline_saved_2(v9.clone(), v3.clone(), v5.clone())
        }
    }
}
fn command_line_argument_count_14() -> i32 {
    let mut v0: i32 = (std::env::args().count() as i32) - 1;
    v0
}
fn command_line_argument_15(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::args().nth((v0 + 1) as usize).unwrap_or_default());
    v1.clone()
}
fn argument_13(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_sub(1i32);
    let mut v2: bool = v1 >= 0i32;
    let mut v5: bool = if v2 {
        let mut v3: i32 = command_line_argument_count_14();
        let mut v4: bool = v1 < v3;
        v4
    } else {
        false
    };
    let mut v9: US1 = if v5 {
        let mut v6: Rc<str> = command_line_argument_15(v1);
        US1::US1_Some(v6.clone())
    } else {
        US1::US1_None
    };
    match &v9 {
        US1::US1_None => {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US1::US1_Some(v10) => {
            let mut v10: Rc<str> = v10.clone();
            v10.clone()
        }
    }
}
fn mode_of_16(mut v0: Rc<str>) -> US0 {
    let mut v1: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--baseline"); } LIT.with(|lit| lit.clone()) };
    if v1 {
        let mut v2: i32 = 3i32;
        let mut v3: Rc<str> = argument_13(v2);
        US0::US0_WriteBaseline(v3.clone())
    } else {
        let mut v5: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--baseline-merge"); } LIT.with(|lit| lit.clone()) };
        if v5 {
            let mut v6: i32 = 3i32;
            let mut v7: Rc<str> = argument_13(v6);
            US0::US0_MergeBaseline(v7.clone())
        } else {
            let mut v9: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--ratchet"); } LIT.with(|lit| lit.clone()) };
            if v9 {
                let mut v10: i32 = 3i32;
                let mut v11: Rc<str> = argument_13(v10);
                US0::US0_CheckRatchet(v11.clone())
            } else {
                US0::US0_PrintCensus
            }
        }
    }
}
fn census_paths_17(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(v0 as usize).collect::<Vec<_>>().join("
").as_str());
    v1.clone()
}
fn first_line_19(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(10u8)).next().unwrap_or(""));
    v1.clone()
}
fn other_lines_22(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
    v1.clone()
}
fn is_dir_24(mut v0: Rc<str>) -> bool {
    let mut v1: bool = std::path::Path::new(&*v0).is_dir();
    v1
}
fn entry_matches_23(mut v0: US3, mut v1: Rc<str>) -> bool {
    match &v0 {
        US3::US3_Directory => {
            is_dir_24(v1.clone())
        }
        US3::US3_SpiralFile => {
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
            let mut v3: bool = v1.ends_with(&*v2);
            v3
        }
    }
}
fn kept_entries_21(mut v0: US3, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let mut v3: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v3 {
            return v2.clone();
        } else {
            let mut v4: Rc<str> = first_line_19(v1.clone());
            let mut v5: Rc<str> = other_lines_22(v1.clone());
            let mut v6: bool = entry_matches_23(v0.clone(), v4.clone());
            let mut v8: Rc<str> = if v6 {
                appended_12(v2.clone(), v4.clone())
            } else {
                v2.clone()
            };
            (v0, v1, v2) = (v0.clone(), v5.clone(), v8.clone());
            continue;
        }
    }
}
fn entry_list_25(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_dir(&*v0).into_iter().flatten().flatten().map(|e| e.path().to_string_lossy().into_owned()).collect::<Vec<_>>().join("
").as_str());
    v1.clone()
}
fn sorted_lines_26(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from({ let mut lines = (v0).lines().collect::<Vec<_>>(); lines.sort(); lines.join("
") }.as_str());
    v1.clone()
}
fn spi_paths_20(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: US3 = US3::US3_SpiralFile;
    let mut v2: Rc<str> = entry_list_25(v0.clone());
    let mut v3: Rc<str> = sorted_lines_26(v2.clone());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    kept_entries_21(v1.clone(), v3.clone(), v4.clone())
}
fn child_dirs_27(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: US3 = US3::US3_Directory;
    let mut v2: Rc<str> = entry_list_25(v0.clone());
    let mut v3: Rc<str> = sorted_lines_26(v2.clone());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    kept_entries_21(v1.clone(), v3.clone(), v4.clone())
}
fn package_children_28(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        let mut v2: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v2 {
            return v1.clone();
        } else {
            let mut v3: Rc<str> = first_line_19(v0.clone());
            let mut v4: Rc<str> = other_lines_22(v0.clone());
            let mut v5: Rc<str> = spi_paths_20(v3.clone());
            let mut v6: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = if v6 {
                v1.clone()
            } else {
                appended_12(v1.clone(), v3.clone())
            };
            (v0, v1) = (v4.clone(), v8.clone());
            continue;
        }
    }
}
fn package_dirs_18(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        let mut v2: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v2 {
            return v1.clone();
        } else {
            let mut v3: Rc<str> = first_line_19(v0.clone());
            let mut v4: Rc<str> = spi_paths_20(v3.clone());
            let mut v5: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v9: Rc<str> = if v5 {
                let mut v6: Rc<str> = child_dirs_27(v3.clone());
                let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                package_children_28(v6.clone(), v7.clone())
            } else {
                v3.clone()
            };
            let mut v10: Rc<str> = other_lines_22(v0.clone());
            let mut v11: bool = v9.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = if v11 {
                v1.clone()
            } else {
                appended_12(v1.clone(), v9.clone())
            };
            (v0, v1) = (v10.clone(), v13.clone());
            continue;
        }
    }
}
fn text_of_files_31(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        let mut v2: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v2 {
            return v1.clone();
        } else {
            let mut v3: Rc<str> = first_line_19(v0.clone());
            let mut v4: Rc<str> = file_text_7(v3.clone());
            let mut v5: Rc<str> = other_lines_22(v0.clone());
            let mut v6: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v9: bool = if v6 {
                true
            } else {
                let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let mut v8: bool = v4.ends_with(&*v7);
                v8
            };
            let mut v12: Rc<str> = if v9 {
                let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v10.clone()
            } else {
                let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                v11.clone()
            };
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v4, &*v12, &*v13, &*v13].concat());
            (v0, v1) = (v5.clone(), v14.clone());
            continue;
        }
    }
}
fn matching_lines_32(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: i32 = regex::Regex::new(&*v1).map(|pattern| (v0).lines().filter(|line| pattern.is_match(line)).count() as i32).unwrap_or(0);
    v2
}
fn package_name_33(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default().as_str());
    v1.clone()
}
fn line_total_34(mut v0: Rc<str>) -> i32 {
    let mut v1: i32 = (v0).lines().count() as i32;
    v1
}
fn census_of_30(mut v0: Rc<str>) -> (Rc<str>, i32, Rc<UH1>) {
    let mut v1: Rc<str> = spi_paths_20(v0.clone());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = text_of_files_31(v1.clone(), v2.clone());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^\\s*\\|\\s*\\w+\\s*::"); } LIT.with(|lit| lit.clone()) };
    let mut v5: i32 = matching_lines_32(v3.clone(), v4.clone());
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\bexists\\s+\\w+\\."); } LIT.with(|lit| lit.clone()) };
    let mut v7: i32 = matching_lines_32(v3.clone(), v6.clone());
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\bforall\\b"); } LIT.with(|lit| lit.clone()) };
    let mut v9: i32 = matching_lines_32(v3.clone(), v8.clone());
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^\\s*union\\s+(rec\\s+)?(private\\s+)?\\w+\\s+\\w+\\s*="); } LIT.with(|lit| lit.clone()) };
    let mut v11: i32 = matching_lines_32(v3.clone(), v10.clone());
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^\\s*(prototype|instance)\\b"); } LIT.with(|lit| lit.clone()) };
    let mut v13: i32 = matching_lines_32(v3.clone(), v12.clone());
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^\\s*(union|type|nominal)\\b"); } LIT.with(|lit| lit.clone()) };
    let mut v15: i32 = matching_lines_32(v3.clone(), v14.clone());
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^\\s+.*(\\$'|\\$\\x22)"); } LIT.with(|lit| lit.clone()) };
    let mut v17: i32 = matching_lines_32(v3.clone(), v16.clone());
    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\$'[^']{200,}'"); } LIT.with(|lit| lit.clone()) };
    let mut v19: i32 = matching_lines_32(v3.clone(), v18.clone());
    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust_global"); } LIT.with(|lit| lit.clone()) };
    let mut v21: i32 = matching_lines_32(v3.clone(), v20.clone());
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("char::from\\((9|30|31)u8\\)|\\x22\\\\t\\x22"); } LIT.with(|lit| lit.clone()) };
    let mut v23: i32 = matching_lines_32(v3.clone(), v22.clone());
    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pipe_(index|member|lookup)"); } LIT.with(|lit| lit.clone()) };
    let mut v25: i32 = matching_lines_32(v3.clone(), v24.clone());
    let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^\\s*\\|\\s*\\d+(u64|i32|u32)?\\s*=>"); } LIT.with(|lit| lit.clone()) };
    let mut v27: i32 = matching_lines_32(v3.clone(), v26.clone());
    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\bconcat5\\b"); } LIT.with(|lit| lit.clone()) };
    let mut v29: i32 = matching_lines_32(v3.clone(), v28.clone());
    let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^\\S.*(\\$'|\\$\\x22)"); } LIT.with(|lit| lit.clone()) };
    let mut v31: i32 = matching_lines_32(v3.clone(), v30.clone());
    let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Rc::<str>::from"); } LIT.with(|lit| lit.clone()) };
    let mut v33: i32 = matching_lines_32(v3.clone(), v32.clone());
    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\$'\\{"); } LIT.with(|lit| lit.clone()) };
    let mut v35: i32 = matching_lines_32(v3.clone(), v34.clone());
    let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust_std_string\\."); } LIT.with(|lit| lit.clone()) };
    let mut v37: i32 = matching_lines_32(v3.clone(), v36.clone());
    let mut v38: Rc<str> = package_name_33(v0.clone());
    let mut v39: i32 = line_total_34(v3.clone());
    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("std-string-use"); } LIT.with(|lit| lit.clone()) };
    let mut v41: US4 = US4::US4_MayNotRise;
    let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("let-block"); } LIT.with(|lit| lit.clone()) };
    let mut v43: US4 = US4::US4_MayNotRise;
    let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rc-wrap"); } LIT.with(|lit| lit.clone()) };
    let mut v45: US4 = US4::US4_MayNotRise;
    let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("top-level-macro"); } LIT.with(|lit| lit.clone()) };
    let mut v47: US4 = US4::US4_MayNotRise;
    let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("string-concat"); } LIT.with(|lit| lit.clone()) };
    let mut v49: US4 = US4::US4_MayNotRise;
    let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("number-arm"); } LIT.with(|lit| lit.clone()) };
    let mut v51: US4 = US4::US4_MayNotRise;
    let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pipe-table"); } LIT.with(|lit| lit.clone()) };
    let mut v53: US4 = US4::US4_MayNotRise;
    let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("encoded-row"); } LIT.with(|lit| lit.clone()) };
    let mut v55: US4 = US4::US4_MayNotRise;
    let mut v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust-global"); } LIT.with(|lit| lit.clone()) };
    let mut v57: US4 = US4::US4_MayNotRise;
    let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wide-host-macro"); } LIT.with(|lit| lit.clone()) };
    let mut v59: US4 = US4::US4_MayNotRise;
    let mut v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host-macro"); } LIT.with(|lit| lit.clone()) };
    let mut v61: US4 = US4::US4_MayNotRise;
    let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type-decl"); } LIT.with(|lit| lit.clone()) };
    let mut v63: US4 = US4::US4_Unranked;
    let mut v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("prototype"); } LIT.with(|lit| lit.clone()) };
    let mut v65: US4 = US4::US4_MayNotFall;
    let mut v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("indexed-union"); } LIT.with(|lit| lit.clone()) };
    let mut v67: US4 = US4::US4_MayNotFall;
    let mut v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("forall"); } LIT.with(|lit| lit.clone()) };
    let mut v69: US4 = US4::US4_MayNotFall;
    let mut v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("existential"); } LIT.with(|lit| lit.clone()) };
    let mut v71: US4 = US4::US4_MayNotFall;
    let mut v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gadt-case"); } LIT.with(|lit| lit.clone()) };
    let mut v73: US4 = US4::US4_MayNotFall;
    let mut v74: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_NoCounts); } CASE.with(|case| case.clone()) };
    let mut v75: Rc<UH1> = Rc::new(UH1::UH1_CountThen(v72.clone(), v73.clone(), v5, v74.clone()));
    let mut v76: Rc<UH1> = Rc::new(UH1::UH1_CountThen(v70.clone(), v71.clone(), v7, v75.clone()));
    let mut v77: Rc<UH1> = Rc::new(UH1::UH1_CountThen(v68.clone(), v69.clone(), v9, v76.clone()));
    let mut v78: Rc<UH1> = Rc::new(UH1::UH1_CountThen(v66.clone(), v67.clone(), v11, v77.clone()));
    let mut v79: Rc<UH1> = Rc::new(UH1::UH1_CountThen(v64.clone(), v65.clone(), v13, v78.clone()));
    let mut v80: Rc<UH1> = Rc::new(UH1::UH1_CountThen(v62.clone(), v63.clone(), v15, v79.clone()));
    let mut v81: Rc<UH1> = Rc::new(UH1::UH1_CountThen(v60.clone(), v61.clone(), v17, v80.clone()));
    let mut v82: Rc<UH1> = Rc::new(UH1::UH1_CountThen(v58.clone(), v59.clone(), v19, v81.clone()));
    let mut v83: Rc<UH1> = Rc::new(UH1::UH1_CountThen(v56.clone(), v57.clone(), v21, v82.clone()));
    let mut v84: Rc<UH1> = Rc::new(UH1::UH1_CountThen(v54.clone(), v55.clone(), v23, v83.clone()));
    let mut v85: Rc<UH1> = Rc::new(UH1::UH1_CountThen(v52.clone(), v53.clone(), v25, v84.clone()));
    let mut v86: Rc<UH1> = Rc::new(UH1::UH1_CountThen(v50.clone(), v51.clone(), v27, v85.clone()));
    let mut v87: Rc<UH1> = Rc::new(UH1::UH1_CountThen(v48.clone(), v49.clone(), v29, v86.clone()));
    let mut v88: Rc<UH1> = Rc::new(UH1::UH1_CountThen(v46.clone(), v47.clone(), v31, v87.clone()));
    let mut v89: Rc<UH1> = Rc::new(UH1::UH1_CountThen(v44.clone(), v45.clone(), v33, v88.clone()));
    let mut v90: Rc<UH1> = Rc::new(UH1::UH1_CountThen(v42.clone(), v43.clone(), v35, v89.clone()));
    let mut v91: Rc<UH1> = Rc::new(UH1::UH1_CountThen(v40.clone(), v41.clone(), v37, v90.clone()));
    (v38.clone(), v39, v91.clone())
}
fn counts_text_35(mut v0: Rc<UH1>) -> Rc<str> {
    match &*v0 {
        UH1::UH1_CountThen(v2, v3, v4, v5) => {
            let mut v2: Rc<str> = v2.clone();
            let mut v3: US4 = v3.clone();
            let mut v4: i32 = *v4;
            let mut v5: Rc<UH1> = v5.clone();
            let mut v6: Rc<str> = counts_text_35(v5.clone());
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
            let mut v9: Rc<str> = count_text_6(v4);
            let mut v10: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v7, &*v2, &*v8, &*v9].concat());
            v10.clone()
        }
        UH1::UH1_NoCounts => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v1.clone()
        }
    }
}
fn binding_of_37(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_lowercase().replace(char::from(45u8), "_").as_str());
    v1.clone()
}
fn census_key_38(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("CensusCount ("); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = quote_9();
    let mut v4: Rc<str> = quote_9();
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = quote_9();
    let mut v7: Rc<str> = quote_9();
    let mut v8: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v6, &*v1, &*v7, &*v5].concat());
    let mut v9: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v3, &*v0, &*v4, &*v8].concat());
    v9.clone()
}
fn baseline_rows_36(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>) -> Rc<str> {
    loop {
        match &*v0 {
            UH1::UH1_CountThen(v4, v5, v6, v7) => {
                let mut v4: Rc<str> = v4.clone();
                let mut v5: US4 = v5.clone();
                let mut v6: i32 = *v6;
                let mut v7: Rc<UH1> = v7.clone();
                let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl census_"); } LIT.with(|lit| lit.clone()) };
                let mut v9: Rc<str> = binding_of_37(v1.clone());
                let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
                let mut v11: Rc<str> = binding_of_37(v4.clone());
                let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = "); } LIT.with(|lit| lit.clone()) };
                let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v9, &*v10, &*v11, &*v12].concat());
                let mut v14: Rc<str> = census_key_38(v1.clone(), v4.clone());
                let mut v15: Rc<str> = count_text_6(v6);
                let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("i32, "); } LIT.with(|lit| lit.clone()) };
                let mut v17: Rc<str> = count_text_6(v2);
                let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("i32)"); } LIT.with(|lit| lit.clone()) };
                let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v20: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v18, &*v19, &*v19, &*v19].concat());
                let mut v21: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v14, &*v15, &*v16, &*v20].concat());
                let mut v22: Rc<str> = appended_12(v3.clone(), v21.clone());
                (v0, v1, v2, v3) = (v7.clone(), v1.clone(), v2, v22.clone());
                continue;
            }
            UH1::UH1_NoCounts => {
                return v3.clone();
            }
        }
    }
}
fn line_after_41(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from((v0).lines().find_map(|line| line.split_once(&*v1).map(|(_, rest)| rest)).unwrap_or(""));
    v2.clone()
}
fn integer_field_42(mut v0: Rc<str>, mut v1: i32) -> i32 {
    let mut v2: i32 = (v0).split(char::from(44u8)).nth(v1 as usize).and_then(|part| part.trim().trim_end_matches(")").trim_end_matches("i32").parse::<i32>().ok()).unwrap_or(-1);
    v2
}
fn entry_of_40(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> (i32, i32) {
    let mut v3: Rc<str> = census_key_38(v1.clone(), v2.clone());
    let mut v4: Rc<str> = line_after_41(v0.clone(), v3.clone());
    let mut v5: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let (mut v10, mut v11): (i32, i32) = if v5 {
        (-1i32, 0i32)
    } else {
        let mut v6: i32 = 0i32;
        let mut v7: i32 = integer_field_42(v4.clone(), v6);
        let mut v8: i32 = 1i32;
        let mut v9: i32 = integer_field_42(v4.clone(), v8);
        (v7, v9)
    };
    (v10, v11)
}
fn count_of_44(mut v0: Rc<UH1>, mut v1: Rc<str>) -> i32 {
    loop {
        match &*v0 {
            UH1::UH1_CountThen(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: US4 = v3.clone();
                let mut v4: i32 = *v4;
                let mut v5: Rc<UH1> = v5.clone();
                let mut v6: bool = v2.clone() == v1.clone();
                if v6 {
                    return v4;
                } else {
                    (v0, v1) = (v5.clone(), v1.clone());
                    continue;
                }
            }
            UH1::UH1_NoCounts => {
                return 0i32;
            }
        }
    }
}
fn feature_total_45(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH1::UH1_CountThen(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: US4 = v3.clone();
                let mut v4: i32 = *v4;
                let mut v5: Rc<UH1> = v5.clone();
                match &v3 {
                    US4::US4_MayNotFall => {
                        let mut v6: i32 = v1.wrapping_add(v4);
                        (v0, v1) = (v5.clone(), v6);
                        continue;
                    }
                    _ => {
                        (v0, v1) = (v5.clone(), v1);
                        continue;
                    }
                }
            }
            UH1::UH1_NoCounts => {
                return v1;
            }
        }
    }
}
fn new_package_problem_43(mut v0: Rc<UH1>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust-global"); } LIT.with(|lit| lit.clone()) };
    let mut v2: i32 = count_of_44(v0.clone(), v1.clone());
    let mut v3: bool = v2 > 0i32;
    if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a new package carries rust_global"); } LIT.with(|lit| lit.clone()) };
        v4.clone()
    } else {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wide-host-macro"); } LIT.with(|lit| lit.clone()) };
        let mut v6: i32 = count_of_44(v0.clone(), v5.clone());
        let mut v7: bool = v6 > 0i32;
        if v7 {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a new package has host macros over 200 characters"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        } else {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("encoded-row"); } LIT.with(|lit| lit.clone()) };
            let mut v10: i32 = count_of_44(v0.clone(), v9.clone());
            let mut v11: bool = v10 > 0i32;
            if v11 {
                let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a new package encodes rows with tab or record separators"); } LIT.with(|lit| lit.clone()) };
                v12.clone()
            } else {
                let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type-decl"); } LIT.with(|lit| lit.clone()) };
                let mut v14: i32 = count_of_44(v0.clone(), v13.clone());
                let mut v15: bool = v14 >= 2i32;
                let mut v19: bool = if v15 {
                    let mut v16: i32 = 0i32;
                    let mut v17: i32 = feature_total_45(v0.clone(), v16);
                    let mut v18: bool = v17 == 0i32;
                    v18
                } else {
                    false
                };
                if v19 {
                    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a new package declares types but uses none of the typefeatureset (GADT cases, existentials, foralls, indexed unions, prototypes)"); } LIT.with(|lit| lit.clone()) };
                    v20.clone()
                } else {
                    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v21.clone()
                }
            }
        }
    }
}
fn regressed_47(mut v0: US4, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32) -> bool {
    let mut v5: f64 = (v1 as f64) * 1000.0 / (v2).max(1) as f64;
    let mut v6: f64 = (v3 as f64) * 1000.0 / (v4).max(1) as f64;
    match &v0 {
        US4::US4_MayNotFall => {
            let mut v12: f64 = v5 * 0.95f64;
            let mut v13: f64 = v12 - 0.5f64;
            let mut v14: bool = v6 < v13;
            if v14 {
                let mut v15: i32 = v1.wrapping_sub(v3);
                let mut v16: i32 = v2.wrapping_sub(v4);
                let mut v17: bool = v15 > v16;
                v17
            } else {
                false
            }
        }
        US4::US4_MayNotRise => {
            let mut v7: f64 = v5 * 1.05f64;
            let mut v8: f64 = v7 + 0.5f64;
            let mut v9: bool = v6 > v8;
            if v9 {
                let mut v10: bool = v3 > v1;
                v10
            } else {
                false
            }
        }
        US4::US4_Unranked => {
            false
        }
    }
}
fn probes_compared_46(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        match &*v0 {
            UH1::UH1_CountThen(v5, v6, v7, v8) => {
                let mut v5: Rc<str> = v5.clone();
                let mut v6: US4 = v6.clone();
                let mut v7: i32 = *v7;
                let mut v8: Rc<UH1> = v8.clone();
                let (mut v9, mut v10): (i32, i32) = entry_of_40(v1.clone(), v2.clone(), v5.clone());
                let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                let mut v12: Rc<str> = count_text_6(v9);
                let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                let mut v14: Rc<str> = count_text_6(v10);
                let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" -> "); } LIT.with(|lit| lit.clone()) };
                let mut v16: Rc<str> = count_text_6(v7);
                let mut v17: Rc<str> = count_text_6(v3);
                let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v19: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v13, &*v17, &*v18, &*v18].concat());
                let mut v20: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v13, &*v14, &*v15, &*v19].concat());
                let mut v21: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v11, &*v5, &*v11, &*v20].concat());
                let mut v22: bool = v9 >= 0i32;
                let mut v24: bool = if v22 {
                    regressed_47(v6.clone(), v9, v10, v7, v3)
                } else {
                    false
                };
                let mut v26: Rc<str> = if v24 {
                    appended_12(v4.clone(), v21.clone())
                } else {
                    v4.clone()
                };
                (v0, v1, v2, v3, v4) = (v8.clone(), v1.clone(), v2.clone(), v3, v26.clone());
                continue;
            }
            UH1::UH1_NoCounts => {
                return v4.clone();
            }
        }
    }
}
fn census_verdict_39(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<UH1>, mut v3: Rc<str>) -> US5 {
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type-decl"); } LIT.with(|lit| lit.clone()) };
    let (mut v5, mut v6): (i32, i32) = entry_of_40(v3.clone(), v0.clone(), v4.clone());
    let mut v7: bool = v5 < 0i32;
    if v7 {
        let mut v8: Rc<str> = new_package_problem_43(v2.clone());
        let mut v9: bool = v8.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v9 {
            US5::US5_Held
        } else {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  new  "); } LIT.with(|lit| lit.clone()) };
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v11, &*v8, &*v12, &*v12].concat());
            US5::US5_Regressed(v13.clone())
        }
    } else {
        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v17: Rc<str> = probes_compared_46(v2.clone(), v3.clone(), v0.clone(), v1, v16.clone());
        let mut v18: bool = v17.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v18 {
            US5::US5_Held
        } else {
            US5::US5_Regressed(v17.clone())
        }
    }
}
fn census_walk_29(mut v0: US0, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<UH0>) -> (i32, i32, Rc<str>, Rc<UH0>) {
    let mut v6: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let (mut v76, mut v77, mut v78, mut v79): (i32, i32, Rc<str>, Rc<UH0>) = if v6 {
        (v2, v3, v4.clone(), v5.clone())
    } else {
        let mut v7: Rc<str> = first_line_19(v1.clone());
        let (mut v8, mut v9, mut v10): (Rc<str>, i32, Rc<UH1>) = census_of_30(v7.clone());
        let mut v11: i32 = v2.wrapping_add(1i32);
        let (mut v72, mut v73, mut v74, mut v75): (i32, i32, Rc<str>, Rc<UH0>) = match &v0 {
            US0::US0_PrintCensus => {
                let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  lines="); } LIT.with(|lit| lit.clone()) };
                let mut v13: Rc<str> = count_text_6(v9);
                let mut v14: Rc<str> = counts_text_35(v10.clone());
                let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v16: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v12, &*v13, &*v14, &*v15].concat());
                stdout_line_4(v16.clone());
                let mut v17: Rc<str> = other_lines_22(v1.clone());
                census_walk_29(v0.clone(), v17.clone(), v11, v3, v4.clone(), v5.clone())
            }
            _ => {
                let (mut v68, mut v69, mut v70, mut v71): (i32, i32, Rc<str>, Rc<UH0>) = match &v0 {
                    US0::US0_WriteBaseline(v22) => {
                        let mut v22: Rc<str> = v22.clone();
                        let mut v23: Rc<str> = other_lines_22(v1.clone());
                        let mut v24: Rc<str> = baseline_rows_36(v10.clone(), v8.clone(), v9, v4.clone());
                        census_walk_29(v0.clone(), v23.clone(), v11, v3, v24.clone(), v5.clone())
                    }
                    _ => {
                        let (mut v64, mut v65, mut v66, mut v67): (i32, i32, Rc<str>, Rc<UH0>) = match &v0 {
                            US0::US0_MergeBaseline(v29) => {
                                let mut v29: Rc<str> = v29.clone();
                                let mut v30: Rc<str> = other_lines_22(v1.clone());
                                let mut v31: Rc<str> = baseline_rows_36(v10.clone(), v8.clone(), v9, v4.clone());
                                let mut v32: Rc<UH0> = Rc::new(UH0::UH0_Cons(v8.clone(), v5.clone()));
                                census_walk_29(v0.clone(), v30.clone(), v11, v3, v31.clone(), v32.clone())
                            }
                            _ => {
                                let (mut v60, mut v61, mut v62, mut v63): (i32, i32, Rc<str>, Rc<UH0>) = match &v0 {
                                    US0::US0_CheckRatchet(v37) => {
                                        let mut v37: Rc<str> = v37.clone();
                                        let mut v38: Rc<str> = file_text_7(v37.clone());
                                        let mut v39: US5 = census_verdict_39(v8.clone(), v9, v10.clone(), v38.clone());
                                        match &v39 {
                                            US5::US5_Held => {
                                                let mut v40: Rc<str> = other_lines_22(v1.clone());
                                                census_walk_29(v0.clone(), v40.clone(), v11, v3, v4.clone(), v5.clone())
                                            }
                                            _ => {
                                                let (mut v52, mut v53, mut v54, mut v55): (i32, i32, Rc<str>, Rc<UH0>) = match &v39 {
                                                    US5::US5_Regressed(v45) => {
                                                        let mut v45: Rc<str> = v45.clone();
                                                        stdout_line_4(v45.clone());
                                                        let mut v46: Rc<str> = other_lines_22(v1.clone());
                                                        let mut v47: i32 = v3.wrapping_add(1i32);
                                                        census_walk_29(v0.clone(), v46.clone(), v11, v47, v4.clone(), v5.clone())
                                                    }
                                                    _ => unreachable!(),
                                                };
                                                (v52, v53, v54.clone(), v55.clone())
                                            }
                                        }
                                    }
                                    _ => unreachable!(),
                                };
                                (v60, v61, v62.clone(), v63.clone())
                            }
                        };
                        (v64, v65, v66.clone(), v67.clone())
                    }
                };
                (v68, v69, v70.clone(), v71.clone())
            }
        };
        (v72, v73, v74.clone(), v75.clone())
    };
    (v76, v77, v78.clone(), v79.clone())
}
fn census_run_0() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: Rc<str> = argument_13(v0);
    let mut v2: US0 = mode_of_16(v1.clone());
    let mut v3: i32 = match &v2 {
        US0::US0_PrintCensus => {
            3i32
        }
        _ => {
            4i32
        }
    };
    let mut v4: Rc<str> = census_paths_17(v3);
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = package_dirs_18(v4.clone(), v5.clone());
    let mut v7: bool = v6.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v7 {
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: standards found no Spiral packages; "); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: standards --census <dir>... | standards --baseline <file.spi> <dir>... | standards --baseline-merge <file.spi> <dir>... (replaces only the walked packages' rows) | standards --ratchet <file.spi> <dir>... (a directory without .spi files stands for its package subdirectories)"); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v9, &*v5, &*v5, &*v5].concat());
        stderr_line_5(v10.clone());
        2i32
    } else {
        let mut v11: i32 = 0i32;
        let mut v12: i32 = 0i32;
        let mut v13: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let (mut v14, mut v15, mut v16, mut v17): (i32, i32, Rc<str>, Rc<UH0>) = census_walk_29(v2.clone(), v6.clone(), v11, v12, v5.clone(), v13.clone());
        census_report_1(v2.clone(), v14, v15, v16.clone(), v17.clone())
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        census_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_census_run() -> i32 {
    closure0()()
}
