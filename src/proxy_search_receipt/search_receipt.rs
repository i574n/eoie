#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_AllHitsShown,
    US0_HitsTruncated,
}
#[derive(Clone)]
enum US1 {
    US1_SliceFilled,
    US1_SliceReachedEnd,
}
fn search_clock_ms_0() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    v0
}
fn closure0() -> Rc<dyn Fn() -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> u64> = Rc::new(move || -> u64 {
        search_clock_ms_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn parallel_search_receipt_text_2(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: u64, mut v4: u64, mut v5: u64) -> Rc<str> {
    let mut v6: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie proxy parallel-search ok workers="); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v6));
    let mut v9: bool = v1 > v2;
    let mut v12: US0 = if v9 {
        US0::US0_HitsTruncated
    } else {
        US0::US0_AllHitsShown
    };
    let mut v24: Rc<str> = match &v12 {
        US0::US0_AllHitsShown => {
            let mut v13: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" hits="); } LIT.with(|lit| lit.clone()) };
            let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v13));
            v15.clone()
        }
        US0::US0_HitsTruncated => {
            let mut v16: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" hits="); } LIT.with(|lit| lit.clone()) };
            let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v16));
            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" truncated=true found_at_least="); } LIT.with(|lit| lit.clone()) };
            let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v19));
            let mut v21: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
            let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
            v22.clone()
        }
    };
    let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v24));
    let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" parity=exact ordering=deterministic excludes=.git,node_modules,build-output,symlinks"); } LIT.with(|lit| lit.clone()) };
    let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v26));
    let mut v28: u64 = (v4).saturating_sub(v3);
    let mut v29: Rc<str> = std::rc::Rc::<str>::from((v28).to_string());
    let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" serial_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v29));
    let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" parallel_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
    let mut v34: u64 = (v5).saturating_sub(v4);
    let mut v35: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
    let mut v36: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v35));
    let mut v37: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v36));
    let mut v38: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v3);
    let mut v39: Rc<str> = std::rc::Rc::<str>::from((v38).to_string());
    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v41: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v39));
    let mut v42: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v41));
    v42.clone()
}
fn parallel_search_receipt_1(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: u64, mut v4: u64, mut v5: u64) -> i32 {
    let mut v6: Rc<str> = parallel_search_receipt_text_2(v0, v1, v2, v3, v4, v5);
    let mut v7: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v6.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v7
}
fn closure1() -> Rc<dyn Fn(i32, i32, i32, u64, u64, u64) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32, i32, u64, u64, u64) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32, mut v2: i32, mut v3: u64, mut v4: u64, mut v5: u64| -> i32 {
        parallel_search_receipt_1(v0, v1, v2, v3, v4, v5)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn fs_search_receipt_text_4(mut v0: i32, mut v1: i32, mut v2: u64) -> Rc<str> {
    let mut v3: bool = v0 > v1;
    let mut v6: US0 = if v3 {
        US0::US0_HitsTruncated
    } else {
        US0::US0_AllHitsShown
    };
    let mut v18: Rc<str> = match &v6 {
        US0::US0_AllHitsShown => {
            let mut v7: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" hits="); } LIT.with(|lit| lit.clone()) };
            let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v7));
            v9.clone()
        }
        US0::US0_HitsTruncated => {
            let mut v10: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" hits="); } LIT.with(|lit| lit.clone()) };
            let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v10));
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" truncated=true found_at_least="); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
            let mut v15: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
            let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v15));
            v16.clone()
        }
    };
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie proxy fs-search ok"); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v18));
    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" excludes=.git,node_modules,build-output,symlinks"); } LIT.with(|lit| lit.clone()) };
    let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
    let mut v23: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v2);
    let mut v24: Rc<str> = std::rc::Rc::<str>::from((v23).to_string());
    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v24));
    let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v26));
    v27.clone()
}
fn fs_search_receipt_3(mut v0: i32, mut v1: i32, mut v2: u64) -> i32 {
    let mut v3: Rc<str> = fs_search_receipt_text_4(v0, v1, v2);
    let mut v4: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v3.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v4
}
fn closure2() -> Rc<dyn Fn(i32, i32, u64) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32, u64) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32, mut v2: u64| -> i32 {
        fs_search_receipt_3(v0, v1, v2)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn fs_slice_receipt_text_6(mut v0: u64, mut v1: u64, mut v2: u64, mut v3: u64) -> Rc<str> {
    let mut v4: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie proxy fs-slice ok start="); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v4));
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" lines="); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
    let mut v9: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" requested="); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v11));
    let mut v13: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
    let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
    let mut v15: bool = v1 < v2;
    let mut v18: US1 = if v15 {
        US1::US1_SliceReachedEnd
    } else {
        US1::US1_SliceFilled
    };
    let mut v22: Rc<str> = match &v18 {
        US1::US1_SliceFilled => {
            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v19.clone()
        }
        US1::US1_SliceReachedEnd => {
            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" end_of_file=true"); } LIT.with(|lit| lit.clone()) };
            v20.clone()
        }
    };
    let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v22));
    let mut v24: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v3);
    let mut v25: Rc<str> = std::rc::Rc::<str>::from((v24).to_string());
    let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v25));
    let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v27));
    v28.clone()
}
fn fs_slice_receipt_5(mut v0: u64, mut v1: u64, mut v2: u64, mut v3: u64) -> i32 {
    let mut v4: Rc<str> = fs_slice_receipt_text_6(v0, v1, v2, v3);
    let mut v5: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v4.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v5
}
fn closure3() -> Rc<dyn Fn(u64, u64, u64, u64) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u64, u64, u64, u64) -> i32> = Rc::new(move |mut v0: u64, mut v1: u64, mut v2: u64, mut v3: u64| -> i32 {
        fs_slice_receipt_5(v0, v1, v2, v3)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn fs_context_receipt_text_8(mut v0: u64, mut v1: u64, mut v2: u64, mut v3: u64) -> Rc<str> {
    let mut v4: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie proxy fs-context ok offset="); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v4));
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" window_bytes="); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
    let mut v9: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" file_bytes="); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v11));
    let mut v13: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
    let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
    let mut v15: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v3);
    let mut v16: Rc<str> = std::rc::Rc::<str>::from((v15).to_string());
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v16));
    let mut v19: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v18));
    v19.clone()
}
fn fs_context_receipt_7(mut v0: u64, mut v1: u64, mut v2: u64, mut v3: u64) -> i32 {
    let mut v4: Rc<str> = fs_context_receipt_text_8(v0, v1, v2, v3);
    let mut v5: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v4.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v5
}
fn closure4() -> Rc<dyn Fn(u64, u64, u64, u64) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u64, u64, u64, u64) -> i32> = Rc::new(move |mut v0: u64, mut v1: u64, mut v2: u64, mut v3: u64| -> i32 {
        fs_context_receipt_7(v0, v1, v2, v3)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn hash_receipt_text_10(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let mut v2: u64 = std::fs::metadata(&*v0).map(|meta| meta.len()).unwrap_or(0);
    let mut v3: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie proxy hash ok bytes="); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v3));
    let mut v6: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v1);
    let mut v7: Rc<str> = std::rc::Rc::<str>::from((v6).to_string());
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v7));
    let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v9));
    v10.clone()
}
fn hash_receipt_9(mut v0: Rc<str>, mut v1: u64) -> i32 {
    let mut v2: Rc<str> = hash_receipt_text_10(v0.clone(), v1);
    let mut v3: i32 = std::io::Write::write_all(&mut std::io::stderr(), (v2.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v3
}
fn closure5() -> Rc<dyn Fn(Rc<str>, u64) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, u64) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: u64| -> i32 {
        hash_receipt_9(v0.clone(), v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn hash_tree_receipt_text_12(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie proxy hash-tree ok relative="); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v0));
    let mut v4: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v1);
    let mut v5: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v5));
    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v7));
    v8.clone()
}
fn hash_tree_receipt_11(mut v0: Rc<str>, mut v1: u64) -> i32 {
    let mut v2: Rc<str> = hash_tree_receipt_text_12(v0.clone(), v1);
    let mut v3: i32 = std::io::Write::write_all(&mut std::io::stderr(), (v2.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v3
}
fn closure6() -> Rc<dyn Fn(Rc<str>, u64) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, u64) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: u64| -> i32 {
        hash_tree_receipt_11(v0.clone(), v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn parallel_manifest_receipt_text_14(mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: u64, mut v4: u64, mut v5: u64) -> Rc<str> {
    let mut v6: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie proxy parallel-manifest ok workers="); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v6));
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" entries="); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
    let mut v11: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v11));
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" sha256="); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
    let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v2));
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" parity=exact ordering=deterministic"); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
    let mut v18: u64 = (v4).saturating_sub(v3);
    let mut v19: Rc<str> = std::rc::Rc::<str>::from((v18).to_string());
    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" serial_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v19));
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" parallel_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v22));
    let mut v24: u64 = (v5).saturating_sub(v4);
    let mut v25: Rc<str> = std::rc::Rc::<str>::from((v24).to_string());
    let mut v26: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v25));
    let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v26));
    let mut v28: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v3);
    let mut v29: Rc<str> = std::rc::Rc::<str>::from((v28).to_string());
    let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v29));
    let mut v32: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v31));
    v32.clone()
}
fn parallel_manifest_receipt_13(mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: u64, mut v4: u64, mut v5: u64) -> i32 {
    let mut v6: Rc<str> = parallel_manifest_receipt_text_14(v0, v1, v2.clone(), v3, v4, v5);
    let mut v7: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v6.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v7
}
fn closure7() -> Rc<dyn Fn(i32, i32, Rc<str>, u64, u64, u64) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32, Rc<str>, u64, u64, u64) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: u64, mut v4: u64, mut v5: u64| -> i32 {
        parallel_manifest_receipt_13(v0, v1, v2.clone(), v3, v4, v5)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_search_clock_ms() -> u64 {
    closure0()()
}
pub fn eoie_parallel_search_receipt(v0: i32, v1: i32, v2: i32, v3: u64, v4: u64, v5: u64) -> i32 {
    closure1()(v0, v1, v2, v3, v4, v5)
}
pub fn eoie_fs_search_receipt(v0: i32, v1: i32, v2: u64) -> i32 {
    closure2()(v0, v1, v2)
}
pub fn eoie_fs_slice_receipt(v0: u64, v1: u64, v2: u64, v3: u64) -> i32 {
    closure3()(v0, v1, v2, v3)
}
pub fn eoie_fs_context_receipt(v0: u64, v1: u64, v2: u64, v3: u64) -> i32 {
    closure4()(v0, v1, v2, v3)
}
pub fn eoie_hash_receipt(v0: &str, v1: u64) -> i32 {
    closure5()(Rc::<str>::from(v0), v1)
}
pub fn eoie_hash_tree_receipt(v0: &str, v1: u64) -> i32 {
    closure6()(Rc::<str>::from(v0), v1)
}
pub fn eoie_parallel_manifest_receipt(v0: i32, v1: i32, v2: &str, v3: u64, v4: u64, v5: u64) -> i32 {
    closure7()(v0, v1, Rc::<str>::from(v2), v3, v4, v5)
}
