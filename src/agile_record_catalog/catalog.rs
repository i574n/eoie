#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_RecordCatalogComplete,
    US0_RecordCatalogPartial(Rc<str>),
}
fn agile_record_catalog_text_0(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>) -> Rc<str> {
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie agile records catalog="); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v0));
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" present="); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v6));
    let mut v8: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
    let mut v12: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
    let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
    let mut v14: bool = v1 == v2;
    let mut v17: US0 = if v14 {
        US0::US0_RecordCatalogComplete
    } else {
        US0::US0_RecordCatalogPartial(v3.clone())
    };
    match &v17 {
        US0::US0_RecordCatalogComplete => {
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" missing=none"); } LIT.with(|lit| lit.clone()) };
            let mut v19: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v18));
            v19.clone()
        }
        US0::US0_RecordCatalogPartial(v20) => {
            let mut v20: Rc<str> = v20.clone();
            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" missing="); } LIT.with(|lit| lit.clone()) };
            let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v21));
            let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v20));
            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (agile check does not need them)"); } LIT.with(|lit| lit.clone()) };
            let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v24));
            v25.clone()
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, i32, i32, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, i32, i32, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>| -> Rc<str> {
        agile_record_catalog_text_0(v0.clone(), v1, v2, v3.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_record_catalog_text(v0: &str, v1: i32, v2: i32, v3: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), v1, v2, Rc::<str>::from(v3))
}
