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
enum US0 {
    US0_FileReadable(Rc<str>),
    US0_FileMissing(Rc<str>),
}
#[derive(Clone)]
enum US1 {
    US1_StateWritten,
    US1_StateChangedSinceRead,
    US1_StateWriteFailed(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_Replaced,
    US2_TemporaryWriteFailed,
    US2_RenameFailed,
    US2_ReadbackMismatch,
}
#[derive(Clone)]
enum US3 {
    US3_AuthorDefault,
    US3_AuthorNamed(Rc<str>),
    US3_AuthorUnnamedAgent,
    US3_AuthorMalformed,
}
#[derive(Clone)]
enum US4 {
    US4_GroomedText(Rc<str>),
    US4_GroomedRefused(Rc<str>),
}
fn declared_from_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
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
fn note_schema_ensured_3(mut v0: Rc<str>) -> Rc<str> {
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
fn declared_count_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: i32 = declared_from_2(v0.clone(), v1.clone(), v2);
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
fn civil_of_days_6(mut v0: u64) -> (u64, u64, u64) {
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
fn utc_stamp_5(mut v0: u64) -> Rc<str> {
    let mut v1: u64 = v0.wrapping_div(1000u64);
    let mut v2: u64 = v1.wrapping_div(86400u64);
    let (mut v3, mut v4, mut v5): (u64, u64, u64) = civil_of_days_6(v2);
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
fn note_row_7(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
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
fn last_declared_10(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: i32 = declared_from_2(v0.clone(), v1.clone(), v2);
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
fn notes_insertion_9(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v3, &*v1, &*v4, &*v5].concat());
    let mut v7: i32 = 0i32;
    let mut v8: i32 = declared_from_2(v0.clone(), v6.clone(), v7);
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planning ("); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v3, &*v1, &*v4, &*v5].concat());
    let mut v11: i32 = 0i32;
    let mut v12: i32 = declared_from_2(v0.clone(), v10.clone(), v11);
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v3, &*v1, &*v4, &*v5].concat());
    let mut v15: i32 = 0i32;
    let mut v16: i32 = -1i32;
    let mut v17: i32 = last_declared_10(v0.clone(), v14.clone(), v15, v16);
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
fn notes_appended_8(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: i32 = notes_insertion_9(v0.clone(), v1.clone());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v2, &*v5, &*v5, &*v5].concat());
    let mut v7: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v3 as usize).unwrap_or(""));
    let mut v8: i32 = (v0.clone().len() as i32);
    let mut v9: Rc<str> = std::rc::Rc::<str>::from(v0.get(v3 as usize..v8 as usize).unwrap_or(""));
    let mut v10: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v6, &*v9, &*v5, &*v5].concat());
    v10.clone()
}
fn state_written_11(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US1 {
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
        let mut v20: US2 = if v11 {
            let mut v12: bool = std::fs::rename(&*v10, &*v0).is_ok();
            if v12 {
                let mut v13: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v2);
                if v13 {
                    US2::US2_Replaced
                } else {
                    US2::US2_ReadbackMismatch
                }
            } else {
                US2::US2_RenameFailed
            }
        } else {
            US2::US2_TemporaryWriteFailed
        };
        match &v20 {
            US2::US2_Replaced => {
                US1::US1_StateWritten
            }
            _ => {
                let mut v27: Rc<str> = match &v20 {
                    US2::US2_ReadbackMismatch => {
                        let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                        v24.clone()
                    }
                    US2::US2_RenameFailed => {
                        let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                        v23.clone()
                    }
                    US2::US2_TemporaryWriteFailed => {
                        let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                        v22.clone()
                    }
                    _ => unreachable!(),
                };
                US1::US1_StateWriteFailed(v27.clone())
            }
        }
    } else {
        US1::US1_StateChangedSinceRead
    }
}
fn note_added_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> i32 {
    let mut v4: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v5, &*v6, &*v6, &*v6].concat());
    let mut v8: bool = std::path::Path::new(&*v7).is_file();
    let mut v11: US0 = if v8 {
        US0::US0_FileReadable(v7.clone())
    } else {
        US0::US0_FileMissing(v7.clone())
    };
    match &v11 {
        US0::US0_FileMissing(v12) => {
            let mut v12: Rc<str> = v12.clone();
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note "); } LIT.with(|lit| lit.clone()) };
            let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v1));
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
            let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
            let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v13));
            let mut v19: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v18 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v19
        }
        US0::US0_FileReadable(v20) => {
            let mut v20: Rc<str> = v20.clone();
            let mut v21: Rc<str> = std::fs::read_to_string(&*v20).unwrap_or_default().into();
            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
            let mut v25: Rc<str> = std::rc::Rc::<str>::from([&*v22, &*v23, &*v1, &*v24, &*v6].concat());
            let mut v26: i32 = 0i32;
            let mut v27: i32 = declared_from_2(v21.clone(), v25.clone(), v26);
            let mut v28: bool = v27 < 0i32;
            let mut v29: bool = v28 == false;
            if v29 {
                let mut v30: Rc<str> = note_schema_ensured_3(v21.clone());
                let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
                let mut v32: Rc<str> = std::rc::Rc::<str>::from([&*v31, &*v23, &*v1, &*v24, &*v6].concat());
                let mut v33: i32 = 0i32;
                let mut v34: i32 = 0i32;
                let mut v35: i32 = declared_count_4(v30.clone(), v32.clone(), v33, v34);
                let mut v36: i32 = v35.wrapping_add(1i32);
                let mut v37: Rc<str> = utc_stamp_5(v4);
                let mut v38: Rc<str> = note_row_7(v1.clone(), v36, v37.clone(), v2.clone(), v3.clone());
                let mut v39: Rc<str> = notes_appended_8(v30.clone(), v1.clone(), v38.clone());
                let mut v40: US1 = state_written_11(v7.clone(), v21.clone(), v39.clone());
                match &v40 {
                    US1::US1_StateWritten => {
                        let mut v41: u64 = v21.len() as u64;
                        let mut v42: u64 = v39.len() as u64;
                        let mut v43: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v4);
                        let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie agile note ok id="); } LIT.with(|lit| lit.clone()) };
                        let mut v45: Rc<str> = Rc::<str>::from(format!("{}{}", v44, v1));
                        let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" note="); } LIT.with(|lit| lit.clone()) };
                        let mut v47: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v46));
                        let mut v48: Rc<str> = std::rc::Rc::<str>::from((v36).to_string());
                        let mut v49: Rc<str> = Rc::<str>::from(format!("{}{}", v47, v48));
                        let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" at="); } LIT.with(|lit| lit.clone()) };
                        let mut v51: Rc<str> = Rc::<str>::from(format!("{}{}", v49, v50));
                        let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" author="); } LIT.with(|lit| lit.clone()) };
                        let mut v53: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v52));
                        let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                        let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
                        let mut v56: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v54, &v55));
                        let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
                        let mut v58: Rc<str> = std::rc::Rc::<str>::from(v56.replace(&*v23, &v57));
                        let mut v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let mut v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
                        let mut v61: Rc<str> = std::rc::Rc::<str>::from(v58.replace(&*v59, &v60));
                        let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                        let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
                        let mut v64: Rc<str> = std::rc::Rc::<str>::from(v61.replace(&*v62, &v63));
                        let mut v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                        let mut v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
                        let mut v67: Rc<str> = std::rc::Rc::<str>::from(v64.replace(&*v65, &v66));
                        let mut v68: Rc<str> = std::rc::Rc::<str>::from([&*v23, &*v67, &*v23, &*v6, &*v6].concat());
                        let mut v69: Rc<str> = Rc::<str>::from(format!("{}{}", v53, v68));
                        let mut v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" text_bytes="); } LIT.with(|lit| lit.clone()) };
                        let mut v71: Rc<str> = Rc::<str>::from(format!("{}{}", v69, v70));
                        let mut v72: i32 = (v3.clone().len() as i32);
                        let mut v73: Rc<str> = std::rc::Rc::<str>::from((v72).to_string());
                        let mut v74: Rc<str> = Rc::<str>::from(format!("{}{}", v71, v73));
                        let mut v75: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v74));
                        let mut v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) };
                        let mut v77: Rc<str> = std::rc::Rc::<str>::from((v41).to_string());
                        let mut v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) };
                        let mut v79: Rc<str> = std::rc::Rc::<str>::from((v42).to_string());
                        let mut v80: Rc<str> = std::rc::Rc::<str>::from([&*v76, &*v77, &*v78, &*v79, &*v6].concat());
                        let mut v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                        let mut v82: Rc<str> = std::rc::Rc::<str>::from((v43).to_string());
                        let mut v83: Rc<str> = std::rc::Rc::<str>::from([&*v80, &*v81, &*v82, &*v6, &*v6].concat());
                        let mut v84: Rc<str> = Rc::<str>::from(format!("{}{}", v75, v83));
                        let mut v85: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v84.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                        v85
                    }
                    _ => {
                        let mut v89: Rc<str> = match &v40 {
                            US1::US1_StateChangedSinceRead => {
                                let mut v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                                v86.clone()
                            }
                            US1::US1_StateWriteFailed(v87) => {
                                let mut v87: Rc<str> = v87.clone();
                                v87.clone()
                            }
                            _ => unreachable!(),
                        };
                        let mut v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note "); } LIT.with(|lit| lit.clone()) };
                        let mut v91: Rc<str> = Rc::<str>::from(format!("{}{}", v90, v1));
                        let mut v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                        let mut v93: Rc<str> = Rc::<str>::from(format!("{}{}", v91, v92));
                        let mut v94: Rc<str> = Rc::<str>::from(format!("{}{}", v93, v89));
                        let mut v95: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v94 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                        v95
                    }
                }
            } else {
                let mut v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no task with this id"); } LIT.with(|lit| lit.clone()) };
                let mut v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note "); } LIT.with(|lit| lit.clone()) };
                let mut v99: Rc<str> = Rc::<str>::from(format!("{}{}", v98, v1));
                let mut v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let mut v101: Rc<str> = Rc::<str>::from(format!("{}{}", v99, v100));
                let mut v102: Rc<str> = Rc::<str>::from(format!("{}{}", v101, v97));
                let mut v103: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v102 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v103
            }
        }
    }
}
fn method12(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
fn method13(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn command_line_argument_count_15() -> i32 {
    let mut v0: i32 = (std::env::args().count() as i32) - 1;
    v0
}
fn argument_count_14() -> i32 {
    let mut v0: i32 = command_line_argument_count_15();
    let mut v1: bool = v0 > 0i32;
    if v1 {
        let mut v2: i32 = v0.wrapping_sub(1i32);
        v2
    } else {
        0i32
    }
}
fn note_entry_0() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: i32 = 3i32;
    let mut v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v4: i32 = (v3.clone().len() as i32);
    let mut v5: i32 = 0i32;
    let mut v6: i32 = method12(v3.clone(), v4, v5);
    let mut v7: i32 = v4.wrapping_sub(1i32);
    let mut v8: Rc<str> = string_slice(&v3.clone(), v6 as i64, v7 as i64);
    let mut v9: i32 = (v8.clone().len() as i32);
    let mut v10: i32 = method13(v8.clone(), v9);
    let mut v11: Rc<str> = string_slice(&v8.clone(), 0i32 as i64, v10 as i64);
    let mut v12: bool = v11.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v12 {
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the note text is empty"); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note "); } LIT.with(|lit| lit.clone()) };
        let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v1));
        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
        let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
        let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v13));
        let mut v19: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v18 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v19
    } else {
        let mut v20: i32 = argument_count_14();
        let mut v21: bool = v20 == 4i32;
        let mut v95: US3 = if v21 {
            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("EOIE_AUTHOR"); } LIT.with(|lit| lit.clone()) };
            let mut v23: Rc<str> = std::env::var(&*v22).map_or_else(|_| std::rc::Rc::<str>::default(), std::rc::Rc::<str>::from);
            let mut v24: i32 = (v23.clone().len() as i32);
            let mut v25: i32 = 0i32;
            let mut v26: i32 = method12(v23.clone(), v24, v25);
            let mut v27: i32 = v24.wrapping_sub(1i32);
            let mut v28: Rc<str> = string_slice(&v23.clone(), v26 as i64, v27 as i64);
            let mut v29: i32 = (v28.clone().len() as i32);
            let mut v30: i32 = method13(v28.clone(), v29);
            let mut v31: Rc<str> = string_slice(&v28.clone(), 0i32 as i64, v30 as i64);
            let mut v32: bool = v31.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v33: bool = v32 == false;
            if v33 {
                let mut v34: Rc<str> = std::env::var(&*v22).map_or_else(|_| std::rc::Rc::<str>::default(), std::rc::Rc::<str>::from);
                let mut v35: i32 = (v34.clone().len() as i32);
                let mut v36: i32 = 0i32;
                let mut v37: i32 = method12(v34.clone(), v35, v36);
                let mut v38: i32 = v35.wrapping_sub(1i32);
                let mut v39: Rc<str> = string_slice(&v34.clone(), v37 as i64, v38 as i64);
                let mut v40: i32 = (v39.clone().len() as i32);
                let mut v41: i32 = method13(v39.clone(), v40);
                let mut v42: Rc<str> = string_slice(&v39.clone(), 0i32 as i64, v41 as i64);
                US3::US3_AuthorNamed(v42.clone())
            } else {
                let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("AI_AGENT"); } LIT.with(|lit| lit.clone()) };
                let mut v45: Rc<str> = std::env::var(&*v44).map_or_else(|_| std::rc::Rc::<str>::default(), std::rc::Rc::<str>::from);
                let mut v46: i32 = (v45.clone().len() as i32);
                let mut v47: i32 = 0i32;
                let mut v48: i32 = method12(v45.clone(), v46, v47);
                let mut v49: i32 = v46.wrapping_sub(1i32);
                let mut v50: Rc<str> = string_slice(&v45.clone(), v48 as i64, v49 as i64);
                let mut v51: i32 = (v50.clone().len() as i32);
                let mut v52: i32 = method13(v50.clone(), v51);
                let mut v53: Rc<str> = string_slice(&v50.clone(), 0i32 as i64, v52 as i64);
                let mut v54: bool = v53.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v55: bool = v54 == false;
                let mut v68: bool = if v55 {
                    true
                } else {
                    let mut v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("CLAUDECODE"); } LIT.with(|lit| lit.clone()) };
                    let mut v57: Rc<str> = std::env::var(&*v56).map_or_else(|_| std::rc::Rc::<str>::default(), std::rc::Rc::<str>::from);
                    let mut v58: i32 = (v57.clone().len() as i32);
                    let mut v59: i32 = 0i32;
                    let mut v60: i32 = method12(v57.clone(), v58, v59);
                    let mut v61: i32 = v58.wrapping_sub(1i32);
                    let mut v62: Rc<str> = string_slice(&v57.clone(), v60 as i64, v61 as i64);
                    let mut v63: i32 = (v62.clone().len() as i32);
                    let mut v64: i32 = method13(v62.clone(), v63);
                    let mut v65: Rc<str> = string_slice(&v62.clone(), 0i32 as i64, v64 as i64);
                    let mut v66: bool = v65.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v67: bool = v66 == false;
                    v67
                };
                if v68 {
                    US3::US3_AuthorUnnamedAgent
                } else {
                    US3::US3_AuthorDefault
                }
            }
        } else {
            let mut v73: bool = v20 == 6i32;
            if v73 {
                let mut v74: i32 = 4i32;
                let mut v75: Rc<str> = usize::try_from(v74).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let mut v76: bool = v75.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--as"); } LIT.with(|lit| lit.clone()) };
                if v76 {
                    let mut v77: i32 = 5i32;
                    let mut v78: Rc<str> = usize::try_from(v77).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    let mut v79: i32 = (v78.clone().len() as i32);
                    let mut v80: i32 = 0i32;
                    let mut v81: i32 = method12(v78.clone(), v79, v80);
                    let mut v82: i32 = v79.wrapping_sub(1i32);
                    let mut v83: Rc<str> = string_slice(&v78.clone(), v81 as i64, v82 as i64);
                    let mut v84: i32 = (v83.clone().len() as i32);
                    let mut v85: i32 = method13(v83.clone(), v84);
                    let mut v86: Rc<str> = string_slice(&v83.clone(), 0i32 as i64, v85 as i64);
                    let mut v87: bool = v86.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    if v87 {
                        US3::US3_AuthorMalformed
                    } else {
                        US3::US3_AuthorNamed(v86.clone())
                    }
                } else {
                    US3::US3_AuthorMalformed
                }
            } else {
                US3::US3_AuthorMalformed
            }
        };
        match &v95 {
            US3::US3_AuthorDefault => {
                let mut v96: i32 = 1i32;
                let mut v97: Rc<str> = usize::try_from(v96).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let mut v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("user"); } LIT.with(|lit| lit.clone()) };
                note_added_1(v97.clone(), v1.clone(), v98.clone(), v11.clone())
            }
            US3::US3_AuthorMalformed => {
                let mut v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile note <root> <id> <text> [--as <author>]"); } LIT.with(|lit| lit.clone()) };
                let mut v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note "); } LIT.with(|lit| lit.clone()) };
                let mut v113: Rc<str> = Rc::<str>::from(format!("{}{}", v112, v1));
                let mut v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let mut v115: Rc<str> = Rc::<str>::from(format!("{}{}", v113, v114));
                let mut v116: Rc<str> = Rc::<str>::from(format!("{}{}", v115, v111));
                let mut v117: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v116 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v117
            }
            US3::US3_AuthorNamed(v100) => {
                let mut v100: Rc<str> = v100.clone();
                let mut v101: i32 = 1i32;
                let mut v102: Rc<str> = usize::try_from(v101).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                note_added_1(v102.clone(), v1.clone(), v100.clone(), v11.clone())
            }
            US3::US3_AuthorUnnamedAgent => {
                let mut v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("an agent session must name the note's author: agile note <root> <id> <text> --as <author> (or set EOIE_AUTHOR)"); } LIT.with(|lit| lit.clone()) };
                let mut v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note "); } LIT.with(|lit| lit.clone()) };
                let mut v106: Rc<str> = Rc::<str>::from(format!("{}{}", v105, v1));
                let mut v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let mut v108: Rc<str> = Rc::<str>::from(format!("{}{}", v106, v107));
                let mut v109: Rc<str> = Rc::<str>::from(format!("{}{}", v108, v104));
                let mut v110: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v109 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v110
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        note_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn planned_17(mut v0: US4, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> US4 {
    match &v0 {
        US4::US4_GroomedRefused(v4) => {
            let mut v4: Rc<str> = v4.clone();
            v0.clone()
        }
        US4::US4_GroomedText(v5) => {
            let mut v5: Rc<str> = v5.clone();
            let mut v6: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v6 {
                v0.clone()
            } else {
                let (mut v7, mut v8): (i32, Rc<str>) = agile_planning_values::eoie_agile_planning_value_token(&*v2, &*v3);
                let mut v9: bool = v7 == 0i32;
                let mut v12: US4 = if v9 {
                    US4::US4_GroomedText(v8.clone())
                } else {
                    US4::US4_GroomedRefused(v8.clone())
                };
                match &v12 {
                    US4::US4_GroomedRefused(v13) => {
                        let mut v13: Rc<str> = v13.clone();
                        US4::US4_GroomedRefused(v13.clone())
                    }
                    US4::US4_GroomedText(v15) => {
                        let mut v15: Rc<str> = v15.clone();
                        let (mut v16, mut v17): (i32, Rc<str>) = agile_planning_rewrite::eoie_agile_planning_rewrite(&*v5, &*v1, &*v2, &*v15);
                        let mut v18: bool = v16 == 0i32;
                        if v18 {
                            US4::US4_GroomedText(v17.clone())
                        } else {
                            US4::US4_GroomedRefused(v17.clone())
                        }
                    }
                }
            }
        }
    }
}
fn groomed_text_16(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>) -> (i32, Rc<str>) {
    let mut v7: US4 = US4::US4_GroomedText(v0.clone());
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("estimate"); } LIT.with(|lit| lit.clone()) };
    let mut v9: US4 = planned_17(v7.clone(), v1.clone(), v8.clone(), v2.clone());
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("load"); } LIT.with(|lit| lit.clone()) };
    let mut v11: US4 = planned_17(v9.clone(), v1.clone(), v10.clone(), v3.clone());
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("assignee"); } LIT.with(|lit| lit.clone()) };
    let mut v13: US4 = planned_17(v11.clone(), v1.clone(), v12.clone(), v4.clone());
    match &v13 {
        US4::US4_GroomedRefused(v14) => {
            let mut v14: Rc<str> = v14.clone();
            (1i32, v14.clone())
        }
        US4::US4_GroomedText(v15) => {
            let mut v15: Rc<str> = v15.clone();
            let mut v16: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v16 {
                (0i32, v15.clone())
            } else {
                let mut v17: Rc<str> = note_schema_ensured_3(v15.clone());
                let mut v18: i32 = 1i32;
                let mut v19: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
                let mut v20: Rc<str> = utc_stamp_5(v19);
                let mut v21: Rc<str> = note_row_7(v1.clone(), v18, v20.clone(), v6.clone(), v5.clone());
                let mut v22: Rc<str> = notes_appended_8(v17.clone(), v1.clone(), v21.clone());
                (0i32, v22.clone())
            }
        }
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> (i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> (i32, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>| -> (i32, Rc<str>) {
        groomed_text_16(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure2() -> Rc<dyn Fn(u64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u64) -> Rc<str>> = Rc::new(move |mut v0: u64| -> Rc<str> {
        utc_stamp_5(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_note_run() -> i32 {
    closure0()()
}
pub fn eoie_agile_groomed(v0: &str, v1: &str, v2: &str, v3: &str, v4: &str, v5: &str, v6: &str) -> (i32, Rc<str>) {
    closure1()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), Rc::<str>::from(v4), Rc::<str>::from(v5), Rc::<str>::from(v6))
}
pub fn eoie_utc_stamp(v0: u64) -> Rc<str> {
    closure2()(v0)
}
