#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_FileReadable(Rc<str>),
    US0_FileMissing(Rc<str>),
}
#[derive(Clone)]
enum US1 {
    US1_SitesKept,
    US1_SitesAppeared(Rc<str>),
    US1_SitesMoved(Rc<str>, Rc<str>),
    US1_SitesVanished(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_StateWritten,
    US2_StateChangedSinceRead,
    US2_StateWriteFailed(Rc<str>),
}
#[derive(Clone)]
enum US3 {
    US3_Replaced,
    US3_TemporaryWriteFailed,
    US3_RenameFailed,
    US3_ReadbackMismatch,
}
fn harvested_ledger_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let mut v3: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v3 {
            let mut v4: Rc<str> = std::rc::Rc::<str>::from({ let mut lines = (v2).lines().filter(|line| !line.is_empty()).collect::<Vec<_>>(); lines.sort(); lines.dedup(); lines.join("
") }.as_str());
            return v4.clone();
        } else {
            let mut v5: Rc<str> = std::rc::Rc::<str>::from((v1).split(char::from(10u8)).next().unwrap_or(""));
            let mut v6: Rc<str> = agile_todo_lines::eoie_agile_todo_id(&*v0, &*v5);
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
            let mut v8: i32 = v5.find(&*v7).map(|offset| offset as i32).unwrap_or(-1);
            let mut v9: bool = v6.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v11: bool = if v9 {
                true
            } else {
                let mut v10: bool = v8 < 0i32;
                v10
            };
            let mut v17: bool = if v11 {
                true
            } else {
                let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TO"); } LIT.with(|lit| lit.clone()) };
                let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("DO"); } LIT.with(|lit| lit.clone()) };
                let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TODO"); } LIT.with(|lit| lit.clone()) };
                let mut v15: i32 = v5.find(&*v14).map(|offset| offset as i32).unwrap_or(-1);
                let mut v16: bool = v15 < 0i32;
                v16
            };
            let mut v25: Rc<str> = if v17 {
                let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v18.clone()
            } else {
                let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v19));
                let mut v21: Rc<str> = std::rc::Rc::<str>::from(v5.get(0i32 as usize..v8 as usize).unwrap_or(""));
                let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
                let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let mut v24: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v23));
                v24.clone()
            };
            let mut v26: Rc<str> = std::rc::Rc::<str>::from((v1).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
            let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v25));
            (v0, v1, v2) = (v0.clone(), v26.clone(), v27.clone());
            continue;
        }
    }
}
fn ledger_from_rows_2(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = TodoSite "); } LIT.with(|lit| lit.clone()) };
        let mut v4: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v3)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let mut v5: bool = v4 < 0i32;
        if v5 {
            let mut v6: Rc<str> = std::rc::Rc::<str>::from({ let mut lines = (v2).lines().filter(|line| !line.is_empty()).collect::<Vec<_>>(); lines.sort(); lines.dedup(); lines.join("
") }.as_str());
            return v6.clone();
        } else {
            let mut v7: i32 = v4.wrapping_add(15i32);
            let mut v8: i32 = v7.wrapping_add(2i32);
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let mut v10: i32 = v0.get(v8 as usize..).and_then(|rest| rest.find(&*v9)).map(|found| (found + v8 as usize) as i32).unwrap_or(-1);
            let mut v11: i32 = v10.wrapping_add(1i32);
            let mut v12: i32 = v0.get(v11 as usize..).and_then(|rest| rest.find(&*v9)).map(|found| (found + v11 as usize) as i32).unwrap_or(-1);
            let mut v13: i32 = v12.wrapping_add(1i32);
            let mut v14: i32 = v0.get(v13 as usize..).and_then(|rest| rest.find(&*v9)).map(|found| (found + v13 as usize) as i32).unwrap_or(-1);
            let mut v15: Rc<str> = std::rc::Rc::<str>::from(v0.get(v8 as usize..v10 as usize).unwrap_or(""));
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
            let mut v18: Rc<str> = std::rc::Rc::<str>::from(v0.get(v13 as usize..v14 as usize).unwrap_or(""));
            let mut v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v20));
            let mut v22: i32 = v14.wrapping_add(1i32);
            let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v21));
            (v0, v1, v2) = (v0.clone(), v22, v23.clone());
            continue;
        }
    }
}
fn record_start_3(mut v0: Rc<str>) -> (Rc<str>, i32, i32, i32, i32) {
    (v0.clone(), 0i32, 0i32, 0i32, 0i32)
}
fn civil_of_days_5(mut v0: u64) -> (u64, u64, u64) {
    let mut v1: u64 = v0.wrapping_add(719468u64);
    let mut v2: u64 = v1.wrapping_div(146097u64);
    let mut v3: u64 = v2.wrapping_mul(146097u64);
    let mut v4: u64 = v1.wrapping_sub(v3);
    let mut v5: u64 = v4.wrapping_div(1460u64);
    let mut v6: u64 = v4.wrapping_sub(v5);
    let mut v7: u64 = v4.wrapping_div(36524u64);
    let mut v8: u64 = v6.wrapping_add(v7);
    let mut v9: u64 = v4.wrapping_div(146096u64);
    let mut v10: u64 = v8.wrapping_sub(v9);
    let mut v11: u64 = v10.wrapping_div(365u64);
    let mut v12: u64 = 365u64.wrapping_mul(v11);
    let mut v13: u64 = v11.wrapping_div(4u64);
    let mut v14: u64 = v12.wrapping_add(v13);
    let mut v15: u64 = v11.wrapping_div(100u64);
    let mut v16: u64 = v14.wrapping_sub(v15);
    let mut v17: u64 = v4.wrapping_sub(v16);
    let mut v18: u64 = 5u64.wrapping_mul(v17);
    let mut v19: u64 = v18.wrapping_add(2u64);
    let mut v20: u64 = v19.wrapping_div(153u64);
    let mut v21: u64 = 153u64.wrapping_mul(v20);
    let mut v22: u64 = v21.wrapping_add(2u64);
    let mut v23: u64 = v22.wrapping_div(5u64);
    let mut v24: u64 = v17.wrapping_sub(v23);
    let mut v25: u64 = v24.wrapping_add(1u64);
    let mut v26: bool = v20 < 10u64;
    let mut v29: u64 = if v26 {
        let mut v27: u64 = v20.wrapping_add(3u64);
        v27
    } else {
        let mut v28: u64 = v20.wrapping_sub(9u64);
        v28
    };
    let mut v30: u64 = v2.wrapping_mul(400u64);
    let mut v31: u64 = v11.wrapping_add(v30);
    let mut v32: bool = v29 < 3u64;
    let mut v33: u64 = if v32 {
        1u64
    } else {
        0u64
    };
    let mut v34: u64 = v31.wrapping_add(v33);
    (v34, v29, v25)
}
fn utc_stamp_4(mut v0: u64) -> Rc<str> {
    let mut v1: u64 = v0.wrapping_div(1000u64);
    let mut v2: u64 = v1.wrapping_div(86400u64);
    let (mut v3, mut v4, mut v5): (u64, u64, u64) = civil_of_days_5(v2);
    let mut v6: u64 = v1.wrapping_rem(86400u64);
    let mut v7: u64 = v6.wrapping_div(60u64);
    let mut v8: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let mut v10: bool = v4 < 10u64;
    let mut v16: Rc<str> = if v10 {
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v12, &*v13, &*v13, &*v13].concat());
        v14.clone()
    } else {
        let mut v15: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
        v15.clone()
    };
    let mut v17: bool = v5 < 10u64;
    let mut v23: Rc<str> = if v17 {
        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let mut v19: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v21: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v19, &*v20, &*v20, &*v20].concat());
        v21.clone()
    } else {
        let mut v22: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
        v22.clone()
    };
    let mut v24: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v9, &*v16, &*v9, &*v23].concat());
    let mut v25: u64 = v7.wrapping_div(60u64);
    let mut v26: bool = v25 < 10u64;
    let mut v32: Rc<str> = if v26 {
        let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let mut v28: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
        let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v30: Rc<str> = std::rc::Rc::<str>::from([&*v27, &*v28, &*v29, &*v29, &*v29].concat());
        v30.clone()
    } else {
        let mut v31: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
        v31.clone()
    };
    let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let mut v34: u64 = v7.wrapping_rem(60u64);
    let mut v35: bool = v34 < 10u64;
    let mut v41: Rc<str> = if v35 {
        let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let mut v37: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
        let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v39: Rc<str> = std::rc::Rc::<str>::from([&*v36, &*v37, &*v38, &*v38, &*v38].concat());
        v39.clone()
    } else {
        let mut v40: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
        v40.clone()
    };
    let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Z"); } LIT.with(|lit| lit.clone()) };
    let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v44: Rc<str> = std::rc::Rc::<str>::from([&*v32, &*v33, &*v41, &*v42, &*v43].concat());
    let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("T"); } LIT.with(|lit| lit.clone()) };
    let mut v46: Rc<str> = std::rc::Rc::<str>::from([&*v24, &*v45, &*v44, &*v43, &*v43].concat());
    v46.clone()
}
fn sites_of_7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let mut v3: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v3 {
            return v2.clone();
        } else {
            let mut v4: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(10u8)).next().unwrap_or(""));
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v5));
            let mut v7: bool = v4.starts_with(&*v6);
            let mut v19: Rc<str> = if v7 {
                let mut v8: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                if v8 {
                    let mut v9: i32 = (v6.clone().len() as i32);
                    let mut v10: i32 = (v4.clone().len() as i32);
                    let mut v11: Rc<str> = std::rc::Rc::<str>::from(v4.get(v9 as usize..v10 as usize).unwrap_or(""));
                    v11.clone()
                } else {
                    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
                    let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v12));
                    let mut v14: i32 = (v6.clone().len() as i32);
                    let mut v15: i32 = (v4.clone().len() as i32);
                    let mut v16: Rc<str> = std::rc::Rc::<str>::from(v4.get(v14 as usize..v15 as usize).unwrap_or(""));
                    let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v16));
                    v17.clone()
                }
            } else {
                v2.clone()
            };
            let mut v20: Rc<str> = std::rc::Rc::<str>::from((v0).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
            (v0, v1, v2) = (v20.clone(), v1.clone(), v19.clone());
            continue;
        }
    }
}
fn declared_from_9(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
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
fn note_schema_ensured_10(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_note ="); } LIT.with(|lit| lit.clone()) };
    let mut v2: i32 = v0.find(&*v1).map(|offset| offset as i32).unwrap_or(-1);
    let mut v3: bool = v2 < 0i32;
    if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-> task_planning"); } LIT.with(|lit| lit.clone()) };
        let mut v5: i32 = v0.find(&*v4).map(|offset| offset as i32).unwrap_or(-1);
        let mut v6: bool = v5 < 0i32;
        let mut v9: i32 = if v6 {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-> agile_item"); } LIT.with(|lit| lit.clone()) };
            let mut v8: i32 = v0.find(&*v7).map(|offset| offset as i32).unwrap_or(-1);
            v8
        } else {
            v5
        };
        let mut v10: bool = v9 < 0i32;
        if v10 {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_note =\n    | Note :: string * u32 * string * string * string -> task_note"); } LIT.with(|lit| lit.clone()) };
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n\n"); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v12, &*v0, &*v13, &*v13].concat());
            v14.clone()
        } else {
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v16: i32 = v0.get(v9 as usize..).and_then(|rest| rest.find(&*v15)).map(|found| (found + v9 as usize) as i32).unwrap_or(-1);
            let mut v17: bool = v16 < 0i32;
            let mut v19: i32 = if v17 {
                let mut v18: i32 = (v0.clone().len() as i32);
                v18
            } else {
                v16
            };
            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n\n"); } LIT.with(|lit| lit.clone()) };
            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_note =\n    | Note :: string * u32 * string * string * string -> task_note"); } LIT.with(|lit| lit.clone()) };
            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v23: Rc<str> = std::rc::Rc::<str>::from([&*v20, &*v21, &*v22, &*v22, &*v22].concat());
            let mut v24: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v19 as usize).unwrap_or(""));
            let mut v25: i32 = (v0.clone().len() as i32);
            let mut v26: Rc<str> = std::rc::Rc::<str>::from(v0.get(v19 as usize..v25 as usize).unwrap_or(""));
            let mut v27: Rc<str> = std::rc::Rc::<str>::from([&*v24, &*v23, &*v26, &*v22, &*v22].concat());
            v27.clone()
        }
    } else {
        v0.clone()
    }
}
fn declared_count_11(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: i32 = declared_from_9(v0.clone(), v1.clone(), v2);
        let mut v5: bool = v4 < 0i32;
        if v5 {
            return v3;
        } else {
            let mut v6: i32 = v4.wrapping_add(1i32);
            let mut v7: i32 = v3.wrapping_add(1i32);
            (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v6, v7);
            continue;
        }
    }
}
fn note_row_12(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note_of_"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = std::rc::Rc::<str>::from(v0.replace(&*v7, &v8));
    let mut v10: Rc<str> = std::rc::Rc::<str>::from(v9.to_ascii_lowercase());
    let mut v11: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v10, &*v8, &*v11, &*v12].concat());
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = Note ("); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = std::rc::Rc::<str>::from(v0.replace(&*v16, &v17));
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = std::rc::Rc::<str>::from(v18.replace(&*v15, &v19));
    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = std::rc::Rc::<str>::from(v20.replace(&*v21, &v22));
    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let mut v26: Rc<str> = std::rc::Rc::<str>::from(v23.replace(&*v24, &v25));
    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let mut v29: Rc<str> = std::rc::Rc::<str>::from(v26.replace(&*v27, &v28));
    let mut v30: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v29, &*v15, &*v12, &*v12].concat());
    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
    let mut v32: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v13, &*v14, &*v30, &*v31].concat());
    let mut v33: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32, "); } LIT.with(|lit| lit.clone()) };
    let mut v35: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v16, &v17));
    let mut v36: Rc<str> = std::rc::Rc::<str>::from(v35.replace(&*v15, &v19));
    let mut v37: Rc<str> = std::rc::Rc::<str>::from(v36.replace(&*v21, &v22));
    let mut v38: Rc<str> = std::rc::Rc::<str>::from(v37.replace(&*v24, &v25));
    let mut v39: Rc<str> = std::rc::Rc::<str>::from(v38.replace(&*v27, &v28));
    let mut v40: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v39, &*v15, &*v12, &*v12].concat());
    let mut v41: Rc<str> = std::rc::Rc::<str>::from(v3.replace(&*v16, &v17));
    let mut v42: Rc<str> = std::rc::Rc::<str>::from(v41.replace(&*v15, &v19));
    let mut v43: Rc<str> = std::rc::Rc::<str>::from(v42.replace(&*v21, &v22));
    let mut v44: Rc<str> = std::rc::Rc::<str>::from(v43.replace(&*v24, &v25));
    let mut v45: Rc<str> = std::rc::Rc::<str>::from(v44.replace(&*v27, &v28));
    let mut v46: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v45, &*v15, &*v12, &*v12].concat());
    let mut v47: Rc<str> = std::rc::Rc::<str>::from([&*v33, &*v34, &*v40, &*v31, &*v46].concat());
    let mut v48: Rc<str> = std::rc::Rc::<str>::from(v4.replace(&*v16, &v17));
    let mut v49: Rc<str> = std::rc::Rc::<str>::from(v48.replace(&*v15, &v19));
    let mut v50: Rc<str> = std::rc::Rc::<str>::from(v49.replace(&*v21, &v22));
    let mut v51: Rc<str> = std::rc::Rc::<str>::from(v50.replace(&*v24, &v25));
    let mut v52: Rc<str> = std::rc::Rc::<str>::from(v51.replace(&*v27, &v28));
    let mut v53: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v52, &*v15, &*v12, &*v12].concat());
    let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let mut v55: Rc<str> = std::rc::Rc::<str>::from([&*v32, &*v47, &*v31, &*v53, &*v54].concat());
    v55.clone()
}
fn last_declared_15(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: i32 = declared_from_9(v0.clone(), v1.clone(), v2);
        let mut v5: bool = v4 < 0i32;
        if v5 {
            return v3;
        } else {
            let mut v6: i32 = v4.wrapping_add(1i32);
            (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v6, v4);
            continue;
        }
    }
}
fn notes_insertion_14(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v3, &*v1, &*v4, &*v5].concat());
    let mut v7: i32 = 0i32;
    let mut v8: i32 = declared_from_9(v0.clone(), v6.clone(), v7);
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planning ("); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v3, &*v1, &*v4, &*v5].concat());
    let mut v11: i32 = 0i32;
    let mut v12: i32 = declared_from_9(v0.clone(), v10.clone(), v11);
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v3, &*v1, &*v4, &*v5].concat());
    let mut v15: i32 = 0i32;
    let mut v16: i32 = -1i32;
    let mut v17: i32 = last_declared_15(v0.clone(), v14.clone(), v15, v16);
    let mut v18: bool = v12 <= v17;
    let mut v19: i32 = if v18 {
        v17
    } else {
        v12
    };
    let mut v20: bool = v8 <= v19;
    let mut v21: i32 = if v20 {
        v19
    } else {
        v8
    };
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v23: i32 = v0.get(v21 as usize..).and_then(|rest| rest.find(&*v22)).map(|found| (found + v21 as usize) as i32).unwrap_or(-1);
    let mut v24: bool = v23 < 0i32;
    if v24 {
        let mut v25: i32 = (v0.clone().len() as i32);
        v25
    } else {
        v23
    }
}
fn notes_appended_13(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: i32 = notes_insertion_14(v0.clone(), v1.clone());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v2, &*v5, &*v5, &*v5].concat());
    let mut v7: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v3 as usize).unwrap_or(""));
    let mut v8: i32 = (v0.clone().len() as i32);
    let mut v9: Rc<str> = std::rc::Rc::<str>::from(v0.get(v3 as usize..v8 as usize).unwrap_or(""));
    let mut v10: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v6, &*v9, &*v5, &*v5].concat());
    v10.clone()
}
fn noted_8(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>) -> (Rc<str>, i32, i32, i32, i32) {
    let mut v8: bool = v6.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v19: bool = if v8 {
        true
    } else {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v10, &*v5, &*v11, &*v12].concat());
        let mut v14: i32 = 0i32;
        let mut v15: i32 = declared_from_9(v0.clone(), v13.clone(), v14);
        let mut v16: bool = v15 < 0i32;
        let mut v17: bool = v16 == false;
        let mut v18: bool = v17 == false;
        v18
    };
    let (mut v34, mut v35, mut v36, mut v37, mut v38): (Rc<str>, i32, i32, i32, i32) = if v19 {
        (v0.clone(), v1, v2, v3, v4)
    } else {
        let mut v20: Rc<str> = note_schema_ensured_10(v0.clone());
        let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
        let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
        let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v25: Rc<str> = std::rc::Rc::<str>::from([&*v21, &*v22, &*v5, &*v23, &*v24].concat());
        let mut v26: i32 = 0i32;
        let mut v27: i32 = 0i32;
        let mut v28: i32 = declared_count_11(v20.clone(), v25.clone(), v26, v27);
        let mut v29: i32 = v28.wrapping_add(1i32);
        let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("todos"); } LIT.with(|lit| lit.clone()) };
        let mut v31: Rc<str> = note_row_12(v5.clone(), v29, v7.clone(), v30.clone(), v6.clone());
        let mut v32: Rc<str> = notes_appended_13(v20.clone(), v5.clone(), v31.clone());
        let mut v33: i32 = v4.wrapping_add(1i32);
        (v32.clone(), v1, v2, v3, v33)
    };
    (v34.clone(), v35, v36, v37, v38)
}
fn changes_recorded_6(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32) -> (Rc<str>, i32, i32, i32, i32) {
    let mut v9: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let (mut v81, mut v82, mut v83, mut v84, mut v85): (Rc<str>, i32, i32, i32, i32) = if v9 {
        (v4.clone(), v5, v6, v7, v8)
    } else {
        let mut v10: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(10u8)).next().unwrap_or(""));
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = sites_of_7(v1.clone(), v10.clone(), v11.clone());
        let mut v13: Rc<str> = sites_of_7(v2.clone(), v10.clone(), v11.clone());
        let mut v14: bool = v12.clone() == v13.clone();
        let mut v23: US1 = if v14 {
            US1::US1_SitesKept
        } else {
            let mut v16: bool = v12.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v16 {
                US1::US1_SitesAppeared(v13.clone())
            } else {
                let mut v18: bool = v13.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                if v18 {
                    US1::US1_SitesVanished(v12.clone())
                } else {
                    US1::US1_SitesMoved(v12.clone(), v13.clone())
                }
            }
        };
        let (mut v46, mut v47, mut v48, mut v49, mut v50): (Rc<str>, i32, i32, i32, i32) = match &v23 {
            US1::US1_SitesKept => {
                (v4.clone(), v5, v6, v7, v8)
            }
            _ => {
                let (mut v41, mut v42, mut v43, mut v44, mut v45): (Rc<str>, i32, i32, i32, i32) = match &v23 {
                    US1::US1_SitesAppeared(v24) => {
                        let mut v24: Rc<str> = v24.clone();
                        let mut v25: i32 = v5.wrapping_add(1i32);
                        (v4.clone(), v25, v6, v7, v8)
                    }
                    _ => {
                        let (mut v36, mut v37, mut v38, mut v39, mut v40): (Rc<str>, i32, i32, i32, i32) = match &v23 {
                            US1::US1_SitesMoved(v26, v27) => {
                                let mut v26: Rc<str> = v26.clone();
                                let mut v27: Rc<str> = v27.clone();
                                let mut v28: i32 = v6.wrapping_add(1i32);
                                (v4.clone(), v5, v28, v7, v8)
                            }
                            _ => {
                                let (mut v31, mut v32, mut v33, mut v34, mut v35): (Rc<str>, i32, i32, i32, i32) = match &v23 {
                                    US1::US1_SitesVanished(v29) => {
                                        let mut v29: Rc<str> = v29.clone();
                                        let mut v30: i32 = v7.wrapping_add(1i32);
                                        (v4.clone(), v5, v6, v30, v8)
                                    }
                                    _ => unreachable!(),
                                };
                                (v31.clone(), v32, v33, v34, v35)
                            }
                        };
                        (v36.clone(), v37, v38, v39, v40)
                    }
                };
                (v41.clone(), v42, v43, v44, v45)
            }
        };
        let mut v69: Rc<str> = match &v23 {
            US1::US1_SitesAppeared(v51) => {
                let mut v51: Rc<str> = v51.clone();
                let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("todos: block appeared at "); } LIT.with(|lit| lit.clone()) };
                let mut v53: Rc<str> = Rc::<str>::from(format!("{}{}", v52, v51));
                v53.clone()
            }
            US1::US1_SitesKept => {
                v11.clone()
            }
            US1::US1_SitesMoved(v54, v55) => {
                let mut v54: Rc<str> = v54.clone();
                let mut v55: Rc<str> = v55.clone();
                let mut v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("todos: block moved from "); } LIT.with(|lit| lit.clone()) };
                let mut v57: Rc<str> = Rc::<str>::from(format!("{}{}", v56, v54));
                let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" to "); } LIT.with(|lit| lit.clone()) };
                let mut v59: Rc<str> = Rc::<str>::from(format!("{}{}", v57, v58));
                let mut v60: Rc<str> = Rc::<str>::from(format!("{}{}", v59, v55));
                v60.clone()
            }
            US1::US1_SitesVanished(v61) => {
                let mut v61: Rc<str> = v61.clone();
                let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("todos: block vanished from "); } LIT.with(|lit| lit.clone()) };
                let mut v63: Rc<str> = Rc::<str>::from(format!("{}{}", v62, v61));
                let mut v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (cut on purpose, or lost)"); } LIT.with(|lit| lit.clone()) };
                let mut v65: Rc<str> = Rc::<str>::from(format!("{}{}", v63, v64));
                v65.clone()
            }
        };
        let (mut v70, mut v71, mut v72, mut v73, mut v74): (Rc<str>, i32, i32, i32, i32) = noted_8(v46.clone(), v47, v48, v49, v50, v10.clone(), v69.clone(), v3.clone());
        let mut v75: Rc<str> = std::rc::Rc::<str>::from((v0).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
        let (mut v76, mut v77, mut v78, mut v79, mut v80): (Rc<str>, i32, i32, i32, i32) = changes_recorded_6(v75.clone(), v1.clone(), v2.clone(), v3.clone(), v70.clone(), v71, v72, v73, v74);
        (v76.clone(), v77, v78, v79, v80)
    };
    (v81.clone(), v82, v83, v84, v85)
}
fn rows_removed_17(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = TodoSite "); } LIT.with(|lit| lit.clone()) };
        let mut v2: i32 = v0.find(&*v1).map(|offset| offset as i32).unwrap_or(-1);
        let mut v3: bool = v2 < 0i32;
        if v3 {
            return v0.clone();
        } else {
            let mut v4: i32 = v0.get(..v2 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v6: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v5)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
            let mut v7: bool = v6 < 0i32;
            let mut v10: i32 = if v7 {
                let mut v8: i32 = (v0.clone().len() as i32);
                v8
            } else {
                let mut v9: i32 = v6.wrapping_add(1i32);
                v9
            };
            let mut v11: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v4 as usize).unwrap_or(""));
            let mut v12: i32 = (v0.clone().len() as i32);
            let mut v13: Rc<str> = std::rc::Rc::<str>::from(v0.get(v10 as usize..v12 as usize).unwrap_or(""));
            let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v13));
            v0 = v14.clone();
            continue;
        }
    }
}
fn site_rows_18(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let mut v3: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v3 {
            return v2.clone();
        } else {
            let mut v4: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(10u8)).next().unwrap_or(""));
            let mut v5: Rc<str> = std::rc::Rc::<str>::from((v4).split(char::from(9u8)).next().unwrap_or(""));
            let mut v6: Rc<str> = std::rc::Rc::<str>::from((v4).split_once(char::from(9u8)).map(|(_, site)| site).unwrap_or(""));
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = TodoSite "); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = std::rc::Rc::<str>::from(format!("inl todo_site_{:04}{}({}{}{}, {}{}{})
", v1, &*(v7), char::from(34u8), &*v5, char::from(34u8), char::from(34u8), &*v6, char::from(34u8)).as_str());
            let mut v9: Rc<str> = std::rc::Rc::<str>::from((v0).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
            let mut v10: i32 = v1.wrapping_add(1i32);
            let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v8));
            (v0, v1, v2) = (v9.clone(), v10, v11.clone());
            continue;
        }
    }
}
fn ledger_written_16(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = rows_removed_17(v0.clone());
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union todo_site ="); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = v2.contains(&*v3);
    let mut v28: Rc<str> = if v4 {
        v2.clone()
    } else {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
        let mut v6: bool = v2.starts_with(&*v5);
        let mut v13: i32 = if v6 {
            0i32
        } else {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\ninl "); } LIT.with(|lit| lit.clone()) };
            let mut v8: i32 = v2.find(&*v7).map(|offset| offset as i32).unwrap_or(-1);
            let mut v9: bool = v8 < 0i32;
            if v9 {
                let mut v10: i32 = (v2.clone().len() as i32);
                v10
            } else {
                let mut v11: i32 = v8.wrapping_add(1i32);
                v11
            }
        };
        let mut v14: Rc<str> = std::rc::Rc::<str>::from(v2.get(0i32 as usize..v13 as usize).unwrap_or(""));
        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union todo_site =\n    | TodoSite :: string * string -> todo_site\n\n"); } LIT.with(|lit| lit.clone()) };
        let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v15));
        let mut v17: bool = v2.starts_with(&*v5);
        let mut v24: i32 = if v17 {
            0i32
        } else {
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\ninl "); } LIT.with(|lit| lit.clone()) };
            let mut v19: i32 = v2.find(&*v18).map(|offset| offset as i32).unwrap_or(-1);
            let mut v20: bool = v19 < 0i32;
            if v20 {
                let mut v21: i32 = (v2.clone().len() as i32);
                v21
            } else {
                let mut v22: i32 = v19.wrapping_add(1i32);
                v22
            }
        };
        let mut v25: i32 = (v2.clone().len() as i32);
        let mut v26: Rc<str> = std::rc::Rc::<str>::from(v2.get(v24 as usize..v25 as usize).unwrap_or(""));
        let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v26));
        v27.clone()
    };
    let mut v29: i32 = v28.find(&*v3).map(|offset| offset as i32).unwrap_or(-1);
    let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n\n"); } LIT.with(|lit| lit.clone()) };
    let mut v31: i32 = v28.get(v29 as usize..).and_then(|rest| rest.find(&*v30)).map(|found| (found + v29 as usize) as i32).unwrap_or(-1);
    let mut v32: i32 = v31.wrapping_add(2i32);
    let mut v33: Rc<str> = std::rc::Rc::<str>::from(v28.get(0i32 as usize..v32 as usize).unwrap_or(""));
    let mut v34: i32 = 0i32;
    let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v36: Rc<str> = site_rows_18(v1.clone(), v34, v35.clone());
    let mut v37: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v36));
    let mut v38: i32 = (v28.clone().len() as i32);
    let mut v39: Rc<str> = std::rc::Rc::<str>::from(v28.get(v32 as usize..v38 as usize).unwrap_or(""));
    let mut v40: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v39));
    v40.clone()
}
fn state_written_19(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US2 {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".paused"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v3, &*v4, &*v4, &*v4].concat());
    std::env::var("EOIE_AGILE_TEST_PAUSE_MS").ok().and_then(|value| value.parse::<u64>().ok()).map(|ms| { let _ = std::fs::write(&*v5, b""); std::thread::sleep(std::time::Duration::from_millis(ms)); let _ = std::fs::remove_file(&*v5); }).unwrap_or(());
    let mut v6: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v1);
    if v6 {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = std::rc::Rc::<str>::from(std::process::id().to_string().as_str());
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v7, &*v8, &*v9, &*v4].concat());
        let mut v11: bool = std::fs::write(&*v10, &*v2).is_ok();
        let mut v20: US3 = if v11 {
            let mut v12: bool = std::fs::rename(&*v10, &*v0).is_ok();
            if v12 {
                let mut v13: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v2);
                if v13 {
                    US3::US3_Replaced
                } else {
                    US3::US3_ReadbackMismatch
                }
            } else {
                US3::US3_RenameFailed
            }
        } else {
            US3::US3_TemporaryWriteFailed
        };
        match &v20 {
            US3::US3_Replaced => {
                US2::US2_StateWritten
            }
            _ => {
                let mut v27: Rc<str> = match &v20 {
                    US3::US3_ReadbackMismatch => {
                        let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                        v24.clone()
                    }
                    US3::US3_RenameFailed => {
                        let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                        v23.clone()
                    }
                    US3::US3_TemporaryWriteFailed => {
                        let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                        v22.clone()
                    }
                    _ => unreachable!(),
                };
                US2::US2_StateWriteFailed(v27.clone())
            }
        }
    } else {
        US2::US2_StateChangedSinceRead
    }
}
fn record_entry_0() -> i32 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v1: i32 = 1i32;
    let mut v2: Rc<str> = usize::try_from(v1).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v3, &*v4, &*v4, &*v4].concat());
    let mut v6: bool = std::path::Path::new(&*v5).is_file();
    let mut v9: US0 = if v6 {
        US0::US0_FileReadable(v5.clone())
    } else {
        US0::US0_FileMissing(v5.clone())
    };
    match &v9 {
        US0::US0_FileMissing(v10) => {
            let mut v10: Rc<str> = v10.clone();
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile todos --record rejected: "); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile todos --record rejected: state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let mut v14: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v13 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v14
        }
        US0::US0_FileReadable(v15) => {
            let mut v15: Rc<str> = v15.clone();
            let mut v16: Rc<str> = std::fs::read_to_string(&*v15).unwrap_or_default().into();
            let mut v17: Rc<str> = agile_todo_scan::eoie_agile_todo_sites(&*v2, "*");
            let mut v18: Rc<str> = harvested_ledger_1(v16.clone(), v17.clone(), v4.clone());
            let mut v19: i32 = 0i32;
            let mut v20: Rc<str> = ledger_from_rows_2(v16.clone(), v19, v4.clone());
            let (mut v21, mut v22, mut v23, mut v24, mut v25): (Rc<str>, i32, i32, i32, i32) = record_start_3(v16.clone());
            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v26));
            let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v18));
            let mut v29: Rc<str> = std::rc::Rc::<str>::from({ let mut ids = (v28).lines().filter_map(|line| line.split(char::from(9u8)).next()).filter(|id| !id.is_empty()).collect::<Vec<_>>(); ids.sort(); ids.dedup(); ids.join("
") }.as_str());
            let mut v30: Rc<str> = utc_stamp_4(v0);
            let (mut v31, mut v32, mut v33, mut v34, mut v35): (Rc<str>, i32, i32, i32, i32) = changes_recorded_6(v29.clone(), v20.clone(), v18.clone(), v30.clone(), v21.clone(), v22, v23, v24, v25);
            let mut v36: Rc<str> = ledger_written_16(v31.clone(), v18.clone());
            let mut v37: Rc<str> = std::rc::Rc::<str>::from((v32).to_string());
            let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" appeared="); } LIT.with(|lit| lit.clone()) };
            let mut v39: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v37));
            let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" moved="); } LIT.with(|lit| lit.clone()) };
            let mut v41: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v40));
            let mut v42: Rc<str> = std::rc::Rc::<str>::from((v33).to_string());
            let mut v43: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v42));
            let mut v44: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
            let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" vanished="); } LIT.with(|lit| lit.clone()) };
            let mut v46: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v44));
            let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" notes="); } LIT.with(|lit| lit.clone()) };
            let mut v48: Rc<str> = Rc::<str>::from(format!("{}{}", v46, v47));
            let mut v49: Rc<str> = std::rc::Rc::<str>::from((v35).to_string());
            let mut v50: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v49));
            let mut v51: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v50));
            let mut v52: bool = v36.clone() == v16.clone();
            if v52 {
                let mut v53: i32 = (v18).lines().count() as i32;
                let mut v54: Rc<str> = std::rc::Rc::<str>::from((v53).to_string());
                let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie agile todos record ok sites="); } LIT.with(|lit| lit.clone()) };
                let mut v56: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v54));
                let mut v57: Rc<str> = Rc::<str>::from(format!("{}{}", v56, v51));
                let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" unchanged=true elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                let mut v59: Rc<str> = Rc::<str>::from(format!("{}{}", v57, v58));
                let mut v60: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                let mut v61: Rc<str> = std::rc::Rc::<str>::from((v60).to_string());
                let mut v62: Rc<str> = Rc::<str>::from(format!("{}{}", v59, v61));
                let mut v63: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v62.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                v63
            } else {
                let mut v64: US2 = state_written_19(v5.clone(), v16.clone(), v36.clone());
                match &v64 {
                    US2::US2_StateWritten => {
                        let mut v65: i32 = (v18).lines().count() as i32;
                        let mut v66: Rc<str> = std::rc::Rc::<str>::from((v65).to_string());
                        let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie agile todos record ok sites="); } LIT.with(|lit| lit.clone()) };
                        let mut v68: Rc<str> = Rc::<str>::from(format!("{}{}", v67, v66));
                        let mut v69: Rc<str> = Rc::<str>::from(format!("{}{}", v68, v51));
                        let mut v70: u64 = v16.len() as u64;
                        let mut v71: u64 = v36.len() as u64;
                        let mut v72: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                        let mut v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) };
                        let mut v74: Rc<str> = std::rc::Rc::<str>::from((v70).to_string());
                        let mut v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) };
                        let mut v76: Rc<str> = std::rc::Rc::<str>::from((v71).to_string());
                        let mut v77: Rc<str> = std::rc::Rc::<str>::from([&*v73, &*v74, &*v75, &*v76, &*v4].concat());
                        let mut v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                        let mut v79: Rc<str> = std::rc::Rc::<str>::from((v72).to_string());
                        let mut v80: Rc<str> = std::rc::Rc::<str>::from([&*v77, &*v78, &*v79, &*v4, &*v4].concat());
                        let mut v81: Rc<str> = Rc::<str>::from(format!("{}{}", v69, v80));
                        let mut v82: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v81.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                        v82
                    }
                    _ => {
                        let mut v86: Rc<str> = match &v64 {
                            US2::US2_StateChangedSinceRead => {
                                let mut v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                                v83.clone()
                            }
                            US2::US2_StateWriteFailed(v84) => {
                                let mut v84: Rc<str> = v84.clone();
                                v84.clone()
                            }
                            _ => unreachable!(),
                        };
                        let mut v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile todos --record rejected: "); } LIT.with(|lit| lit.clone()) };
                        let mut v88: Rc<str> = Rc::<str>::from(format!("{}{}", v87, v86));
                        let mut v89: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v88 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                        v89
                    }
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        record_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_todos_record_run() -> i32 {
    closure0()()
}
