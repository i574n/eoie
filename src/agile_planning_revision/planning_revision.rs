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
    US1_StepText(Rc<str>),
    US1_StepRejected(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_FileReadable(Rc<str>),
    US2_FileMissing(Rc<str>),
}
fn planning_edit_failure_2(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile edit "); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v1, &*v3, &*v0, &*v4].concat());
    let mut v6: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v5 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
    v6
}
fn planning_written_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: u64) -> i32 {
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v6, &*v7, &*v7, &*v7].concat());
    let mut v9: bool = std::fs::write(&*v8, &*v4).is_ok();
    let mut v18: US0 = if v9 {
        let mut v10: bool = std::fs::rename(&*v8, &*v0).is_ok();
        if v10 {
            let mut v11: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v4);
            if v11 {
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
    match &v18 {
        US0::US0_Replaced => {
            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie agile edit ok id="); } LIT.with(|lit| lit.clone()) };
            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" field="); } LIT.with(|lit| lit.clone()) };
            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
            let mut v22: Rc<str> = std::rc::Rc::<str>::from([&*v19, &*v1, &*v20, &*v2, &*v21].concat());
            let mut v23: Rc<str> = agile_planning_fields::eoie_agile_planning_summary(&*v4, &*v1);
            let mut v24: u64 = v3.len() as u64;
            let mut v25: u64 = v4.len() as u64;
            let mut v26: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v5);
            let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) };
            let mut v28: Rc<str> = std::rc::Rc::<str>::from((v24).to_string());
            let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) };
            let mut v30: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
            let mut v31: Rc<str> = std::rc::Rc::<str>::from([&*v27, &*v28, &*v29, &*v30, &*v7].concat());
            let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
            let mut v33: Rc<str> = std::rc::Rc::<str>::from((v26).to_string());
            let mut v34: Rc<str> = std::rc::Rc::<str>::from([&*v31, &*v32, &*v33, &*v7, &*v7].concat());
            let mut v35: Rc<str> = std::rc::Rc::<str>::from([&*v22, &*v23, &*v34, &*v7, &*v7].concat());
            let mut v36: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v35.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
            v36
        }
        _ => {
            let mut v42: Rc<str> = match &v18 {
                US0::US0_ReadbackMismatch => {
                    let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                    v39.clone()
                }
                US0::US0_RenameFailed => {
                    let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                    v38.clone()
                }
                US0::US0_TemporaryWriteFailed => {
                    let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                    v37.clone()
                }
                _ => unreachable!(),
            };
            planning_edit_failure_2(v42.clone(), v1.clone())
        }
    }
}
fn rewritten_5(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> US1 {
    let (mut v4, mut v5): (i32, Rc<str>) = agile_planning_rewrite::eoie_agile_planning_rewrite(&*v0, &*v1, &*v2, &*v3);
    let mut v6: bool = v4 == 0i32;
    if v6 {
        US1::US1_StepText(v5.clone())
    } else {
        US1::US1_StepRejected(v5.clone())
    }
}
fn planning_revised_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: u64) -> i32 {
    let mut v6: US1 = rewritten_5(v4.clone(), v1.clone(), v2.clone(), v3.clone());
    match &v6 {
        US1::US1_StepRejected(v7) => {
            let mut v7: Rc<str> = v7.clone();
            planning_edit_failure_2(v7.clone(), v1.clone())
        }
        US1::US1_StepText(v9) => {
            let mut v9: Rc<str> = v9.clone();
            planning_written_4(v0.clone(), v1.clone(), v2.clone(), v4.clone(), v9.clone(), v5)
        }
    }
}
fn value_token_6(mut v0: Rc<str>, mut v1: Rc<str>) -> US1 {
    let (mut v2, mut v3): (i32, Rc<str>) = agile_planning_values::eoie_agile_planning_value_token(&*v0, &*v1);
    let mut v4: bool = v2 == 0i32;
    if v4 {
        US1::US1_StepText(v3.clone())
    } else {
        US1::US1_StepRejected(v3.clone())
    }
}
fn planning_edit_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> i32 {
    let mut v4: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v5, &*v6, &*v6, &*v6].concat());
    let mut v8: bool = std::path::Path::new(&*v7).is_file();
    let mut v11: US2 = if v8 {
        US2::US2_FileReadable(v7.clone())
    } else {
        US2::US2_FileMissing(v7.clone())
    };
    match &v11 {
        US2::US2_FileMissing(v12) => {
            let mut v12: Rc<str> = v12.clone();
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            planning_edit_failure_2(v13.clone(), v1.clone())
        }
        US2::US2_FileReadable(v15) => {
            let mut v15: Rc<str> = v15.clone();
            let mut v16: US1 = value_token_6(v2.clone(), v3.clone());
            match &v16 {
                US1::US1_StepRejected(v17) => {
                    let mut v17: Rc<str> = v17.clone();
                    planning_edit_failure_2(v17.clone(), v1.clone())
                }
                US1::US1_StepText(v19) => {
                    let mut v19: Rc<str> = v19.clone();
                    let mut v20: Rc<str> = std::fs::read_to_string(&*v15).unwrap_or_default().into();
                    planning_revised_3(v7.clone(), v1.clone(), v2.clone(), v19.clone(), v20.clone(), v4)
                }
            }
        }
    }
}
fn planning_edit_entry_0() -> i32 {
    let mut v0: i32 = 1i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: i32 = 2i32;
    let mut v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v4: i32 = 3i32;
    let mut v5: Rc<str> = usize::try_from(v4).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v6: i32 = 4i32;
    let mut v7: Rc<str> = usize::try_from(v6).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    planning_edit_1(v1.clone(), v3.clone(), v5.clone(), v7.clone())
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        planning_edit_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_planning_edit_run() -> i32 {
    closure0()()
}
