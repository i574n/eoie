#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US1 {
    US1_HashFiles(Rc<str>),
    US1_EnvValues(Rc<str>),
    US1_PathsExist(Rc<str>),
    US1_NearestMarker(Rc<str>, Rc<str>, Rc<str>),
    US1_JsonFields(Rc<str>, Rc<str>),
    US1_LinesCounted(Rc<str>, Rc<str>),
    US1_FilesCounted(Rc<str>),
    US1_TableCounted(Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>),
    US1_DirsClassified(i32),
    US1_DirsDetail,
}
#[derive(Clone)]
enum US0 {
    US0_FactParsed(US1),
    US0_FactRejected(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_Succeeded,
    US2_Failed,
}
fn tables_run_1(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: i32 = eoie_do_tables::eoie_do_tables_run(&*v0, &*v1);
    v2
}
fn now_ms_2() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn records_of_3(mut v0: Rc<str>) -> std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> {
    let mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new((&*v0).split(char::from(30u8)).map(|record| record.split(char::from(31u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
    v1.clone()
}
fn field_at_5(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let mut v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).cloned().unwrap_or_default();
    v3.clone()
}
fn listed_fact_7(mut v0: Rc<str>, mut v1: Rc<str>) -> US0 {
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": expected a list of names"); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v3, &*v4, &*v4, &*v4].concat());
        US0::US0_FactRejected(v5.clone())
    } else {
        let mut v7: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("hash"); } LIT.with(|lit| lit.clone()) };
        if v7 {
            let mut v8: US1 = US1::US1_HashFiles(v1.clone());
            US0::US0_FactParsed(v8.clone())
        } else {
            let mut v10: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("env"); } LIT.with(|lit| lit.clone()) };
            if v10 {
                let mut v11: US1 = US1::US1_EnvValues(v1.clone());
                US0::US0_FactParsed(v11.clone())
            } else {
                let mut v13: US1 = US1::US1_PathsExist(v1.clone());
                US0::US0_FactParsed(v13.clone())
            }
        }
    }
}
fn first_of_kind_9(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: u8) -> Rc<str> {
    let mut v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.iter().skip(2).find(|field| field.as_bytes().first() == Some(&v2)).map(|field| std::rc::Rc::<str>::from(&field[1..]))).unwrap_or_default();
    v3.clone()
}
fn field_index_10(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: Rc<str>) -> i32 {
    let mut v3: i32 = v0.get(v1 as usize).and_then(|fields| fields.iter().position(|field| field.len() > 1 && field.starts_with(char::from(119u8)) && &field[1..] == &*v2)).map(|index| index as i32).unwrap_or(-1);
    v3
}
fn body_at_11(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let mut v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).filter(|field| !field.is_empty()).map(|field| std::rc::Rc::<str>::from(&field[1..])).unwrap_or_default();
    v3.clone()
}
fn items_span_12(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32, mut v3: i32) -> Rc<str> {
    let mut v4: Rc<str> = { let fields = v0.get(v1 as usize).cloned().unwrap_or_default(); let until = if v3 < 0 { fields.len() } else { (v3 as usize).min(fields.len()) }; let mut items = Vec::new(); for raw in fields.iter().take(until).skip(v2 as usize) { if raw.is_empty() { continue; } let (kind, body) = raw.split_at(1); if kind == "q" { items.push(body.to_owned()); } else { items.extend(body.split(|c: char| c == char::from(44u8) || c.is_whitespace()).filter(|item| !item.is_empty()).map(str::to_owned)); } } std::rc::Rc::<str>::from(items.join("
")) };
    v4.clone()
}
fn nearest_fact_8(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let mut v2: u8 = 113u8;
    let mut v3: Rc<str> = first_of_kind_9(v0.clone(), v1, v2);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lines"); } LIT.with(|lit| lit.clone()) };
    let mut v5: i32 = field_index_10(v0.clone(), v1, v4.clone());
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("in"); } LIT.with(|lit| lit.clone()) };
    let mut v7: i32 = field_index_10(v0.clone(), v1, v6.clone());
    let mut v8: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v8 {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("nearest: expected a quoted pattern"); } LIT.with(|lit| lit.clone()) };
        US0::US0_FactRejected(v9.clone())
    } else {
        let mut v11: bool = v5 < 0i32;
        if v11 {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("nearest: expected before lines <n,...>"); } LIT.with(|lit| lit.clone()) };
            US0::US0_FactRejected(v12.clone())
        } else {
            let mut v14: bool = v7 < v5;
            if v14 {
                let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("nearest: expected in <file> after the lines"); } LIT.with(|lit| lit.clone()) };
                US0::US0_FactRejected(v15.clone())
            } else {
                let mut v17: i32 = v7.wrapping_add(1i32);
                let mut v18: Rc<str> = body_at_11(v0.clone(), v1, v17);
                let mut v19: bool = v18.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                if v19 {
                    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("in: expected a value after it"); } LIT.with(|lit| lit.clone()) };
                    US0::US0_FactRejected(v20.clone())
                } else {
                    let mut v22: i32 = v5.wrapping_add(1i32);
                    let mut v23: Rc<str> = items_span_12(v0.clone(), v1, v22, v7);
                    let mut v24: US1 = US1::US1_NearestMarker(v3.clone(), v23.clone(), v18.clone());
                    US0::US0_FactParsed(v24.clone())
                }
            }
        }
    }
}
fn json_fact_13(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let mut v2: u8 = 119u8;
    let mut v3: Rc<str> = first_of_kind_9(v0.clone(), v1, v2);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("get"); } LIT.with(|lit| lit.clone()) };
    let mut v5: i32 = field_index_10(v0.clone(), v1, v4.clone());
    let mut v6: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v8: bool = if v6 {
        true
    } else {
        let mut v7: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("get"); } LIT.with(|lit| lit.clone()) };
        v7
    };
    if v8 {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("json: expected a file"); } LIT.with(|lit| lit.clone()) };
        US0::US0_FactRejected(v9.clone())
    } else {
        let mut v11: bool = v5 < 0i32;
        if v11 {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("json: expected get <field,...>"); } LIT.with(|lit| lit.clone()) };
            US0::US0_FactRejected(v12.clone())
        } else {
            let mut v14: i32 = v5.wrapping_add(1i32);
            let mut v15: i32 = -1i32;
            let mut v16: Rc<str> = items_span_12(v0.clone(), v1, v14, v15);
            let mut v17: bool = v16.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v17 {
                let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("get: expected a value after it"); } LIT.with(|lit| lit.clone()) };
                US0::US0_FactRejected(v18.clone())
            } else {
                let mut v20: US1 = US1::US1_JsonFields(v3.clone(), v16.clone());
                US0::US0_FactParsed(v20.clone())
            }
        }
    }
}
fn counted_fact_14(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: Rc<str>) -> US0 {
    let mut v3: u8 = 119u8;
    let mut v4: Rc<str> = first_of_kind_9(v0.clone(), v1, v3);
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("by"); } LIT.with(|lit| lit.clone()) };
    let mut v6: i32 = field_index_10(v0.clone(), v1, v5.clone());
    let mut v7: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v9: bool = if v7 {
        true
    } else {
        let mut v8: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("count"); } LIT.with(|lit| lit.clone()) };
        v8
    };
    if v9 {
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": expected a path"); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v10, &*v11, &*v11, &*v11].concat());
        US0::US0_FactRejected(v12.clone())
    } else {
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("count"); } LIT.with(|lit| lit.clone()) };
        let mut v15: i32 = field_index_10(v0.clone(), v1, v14.clone());
        let mut v16: bool = v15 < 0i32;
        let mut v18: bool = if v16 {
            true
        } else {
            let mut v17: bool = v6 < 0i32;
            v17
        };
        if v18 {
            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": expected count by ..."); } LIT.with(|lit| lit.clone()) };
            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v21: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v19, &*v20, &*v20, &*v20].concat());
            US0::US0_FactRejected(v21.clone())
        } else {
            let mut v23: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files"); } LIT.with(|lit| lit.clone()) };
            if v23 {
                let mut v24: i32 = v6.wrapping_add(1i32);
                let mut v25: Rc<str> = body_at_11(v0.clone(), v1, v24);
                let mut v26: bool = v25.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("extension"); } LIT.with(|lit| lit.clone()) };
                if v26 {
                    let mut v27: US1 = US1::US1_FilesCounted(v4.clone());
                    US0::US0_FactParsed(v27.clone())
                } else {
                    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files count by: only extension is supported"); } LIT.with(|lit| lit.clone()) };
                    US0::US0_FactRejected(v29.clone())
                }
            } else {
                let mut v32: u8 = 113u8;
                let mut v33: Rc<str> = first_of_kind_9(v0.clone(), v1, v32);
                let mut v34: bool = v33.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                if v34 {
                    let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lines count by: expected a quoted regex"); } LIT.with(|lit| lit.clone()) };
                    US0::US0_FactRejected(v35.clone())
                } else {
                    let mut v37: US1 = US1::US1_LinesCounted(v4.clone(), v33.clone());
                    US0::US0_FactParsed(v37.clone())
                }
            }
        }
    }
}
fn table_fact_15(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let mut v2: u8 = 119u8;
    let mut v3: Rc<str> = first_of_kind_9(v0.clone(), v1, v2);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("where"); } LIT.with(|lit| lit.clone()) };
    let mut v5: i32 = field_index_10(v0.clone(), v1, v4.clone());
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("by"); } LIT.with(|lit| lit.clone()) };
    let mut v7: i32 = field_index_10(v0.clone(), v1, v6.clone());
    let mut v8: bool = v5 < 0i32;
    let mut v12: Rc<str> = if v8 {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
        v9.clone()
    } else {
        let mut v10: i32 = v5.wrapping_add(2i32);
        body_at_11(v0.clone(), v1, v10)
    };
    let mut v13: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v15: bool = if v13 {
        true
    } else {
        let mut v14: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("where"); } LIT.with(|lit| lit.clone()) };
        v14
    };
    let mut v17: bool = if v15 {
        true
    } else {
        let mut v16: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("count"); } LIT.with(|lit| lit.clone()) };
        v16
    };
    if v17 {
        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table: expected a file"); } LIT.with(|lit| lit.clone()) };
        US0::US0_FactRejected(v18.clone())
    } else {
        let mut v20: bool = v7 < 0i32;
        if v20 {
            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table: expected count by <column,...>"); } LIT.with(|lit| lit.clone()) };
            US0::US0_FactRejected(v21.clone())
        } else {
            let mut v23: bool = v12.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
            let mut v24: bool = v23 != true;
            let mut v27: bool = if v24 {
                let mut v25: bool = v12.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<>"); } LIT.with(|lit| lit.clone()) };
                let mut v26: bool = v25 != true;
                v26
            } else {
                false
            };
            if v27 {
                let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("where: expected <column> = \"value\" or <column> <> \"value\""); } LIT.with(|lit| lit.clone()) };
                US0::US0_FactRejected(v28.clone())
            } else {
                let mut v30: i32 = v7.wrapping_add(1i32);
                let mut v31: i32 = -1i32;
                let mut v32: Rc<str> = items_span_12(v0.clone(), v1, v30, v31);
                let mut v36: Rc<str> = if v8 {
                    let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v33.clone()
                } else {
                    let mut v34: i32 = v5.wrapping_add(1i32);
                    body_at_11(v0.clone(), v1, v34)
                };
                let mut v37: bool = v32.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                if v37 {
                    let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("by: expected a value after it"); } LIT.with(|lit| lit.clone()) };
                    US0::US0_FactRejected(v38.clone())
                } else {
                    let mut v40: i32 = v5.wrapping_add(3i32);
                    let mut v41: Rc<str> = body_at_11(v0.clone(), v1, v40);
                    let mut v42: US1 = US1::US1_TableCounted(v3.clone(), v36.clone(), v23, v41.clone(), v32.clone());
                    US0::US0_FactParsed(v42.clone())
                }
            }
        }
    }
}
fn dirs_fact_16(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let mut v2: u8 = 119u8;
    let mut v3: Rc<str> = first_of_kind_9(v0.clone(), v1, v2);
    let mut v4: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: bool = if v4 {
        true
    } else {
        let mut v5: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("having"); } LIT.with(|lit| lit.clone()) };
        v5
    };
    if v6 {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dirs: expected a folder or folder/*"); } LIT.with(|lit| lit.clone()) };
        US0::US0_FactRejected(v7.clone())
    } else {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("having"); } LIT.with(|lit| lit.clone()) };
        let mut v10: i32 = field_index_10(v0.clone(), v1, v9.clone());
        let mut v11: bool = v10 < 0i32;
        if v11 {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dirs: expected having <file> [and <file>...]"); } LIT.with(|lit| lit.clone()) };
            US0::US0_FactRejected(v12.clone())
        } else {
            let mut v14: US1 = US1::US1_DirsClassified(v1);
            US0::US0_FactParsed(v14.clone())
        }
    }
}
fn dirs_above_17(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> bool {
    loop {
        let mut v2: bool = v1 < 1i32;
        if v2 {
            return false;
        } else {
            let mut v3: i32 = 1i32;
            let mut v4: Rc<str> = field_at_5(v0.clone(), v1, v3);
            let mut v5: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dirs"); } LIT.with(|lit| lit.clone()) };
            if v5 {
                return true;
            } else {
                let mut v6: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tag"); } LIT.with(|lit| lit.clone()) };
                let mut v8: bool = if v6 {
                    true
                } else {
                    let mut v7: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("count"); } LIT.with(|lit| lit.clone()) };
                    v7
                };
                if v8 {
                    let mut v9: i32 = v1.wrapping_sub(1i32);
                    (v0, v1) = (v0.clone(), v9);
                    continue;
                } else {
                    return false;
                }
            }
        }
    }
}
fn fact_parsed_6(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let mut v2: i32 = 1i32;
    let mut v3: Rc<str> = field_at_5(v0.clone(), v1, v2);
    let mut v4: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("hash"); } LIT.with(|lit| lit.clone()) };
    let mut v6: bool = if v4 {
        true
    } else {
        let mut v5: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("env"); } LIT.with(|lit| lit.clone()) };
        v5
    };
    let mut v8: bool = if v6 {
        true
    } else {
        let mut v7: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exists"); } LIT.with(|lit| lit.clone()) };
        v7
    };
    if v8 {
        let mut v9: i32 = 2i32;
        let mut v10: i32 = -1i32;
        let mut v11: Rc<str> = items_span_12(v0.clone(), v1, v9, v10);
        listed_fact_7(v3.clone(), v11.clone())
    } else {
        let mut v13: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("nearest"); } LIT.with(|lit| lit.clone()) };
        if v13 {
            nearest_fact_8(v0.clone(), v1)
        } else {
            let mut v15: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("json"); } LIT.with(|lit| lit.clone()) };
            if v15 {
                json_fact_13(v0.clone(), v1)
            } else {
                let mut v17: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lines"); } LIT.with(|lit| lit.clone()) };
                let mut v19: bool = if v17 {
                    true
                } else {
                    let mut v18: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files"); } LIT.with(|lit| lit.clone()) };
                    v18
                };
                if v19 {
                    counted_fact_14(v0.clone(), v1, v3.clone())
                } else {
                    let mut v21: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table"); } LIT.with(|lit| lit.clone()) };
                    if v21 {
                        table_fact_15(v0.clone(), v1)
                    } else {
                        let mut v23: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dirs"); } LIT.with(|lit| lit.clone()) };
                        if v23 {
                            dirs_fact_16(v0.clone(), v1)
                        } else {
                            let mut v25: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tag"); } LIT.with(|lit| lit.clone()) };
                            let mut v27: bool = if v25 {
                                true
                            } else {
                                let mut v26: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("count"); } LIT.with(|lit| lit.clone()) };
                                v26
                            };
                            if v27 {
                                let mut v28: i32 = v1.wrapping_sub(1i32);
                                let mut v29: bool = dirs_above_17(v0.clone(), v28);
                                if v29 {
                                    let mut v30: US1 = US1::US1_DirsDetail;
                                    US0::US0_FactParsed(v30.clone())
                                } else {
                                    let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": needs a dirs line above it"); } LIT.with(|lit| lit.clone()) };
                                    let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let mut v34: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v32, &*v33, &*v33, &*v33].concat());
                                    US0::US0_FactRejected(v34.clone())
                                }
                            } else {
                                let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cannot share one program with hash, env, exists, nearest, json, table, dirs or count by yet; run it as its own call"); } LIT.with(|lit| lit.clone()) };
                                let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v39: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v37, &*v38, &*v38, &*v38].concat());
                                US0::US0_FactRejected(v39.clone())
                            }
                        }
                    }
                }
            }
        }
    }
}
fn first_rejection_4(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> Rc<str> {
    loop {
        let mut v2: i32 = v0.len() as i32;
        let mut v3: bool = v1 < v2;
        if v3 {
            let mut v4: i32 = 0i32;
            let mut v5: Rc<str> = field_at_5(v0.clone(), v1, v4);
            let mut v6: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("query"); } LIT.with(|lit| lit.clone()) };
            let mut v7: bool = v6 != true;
            if v7 {
                let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit and query statements cannot share one program yet; run them as two calls"); } LIT.with(|lit| lit.clone()) };
                return v8.clone();
            } else {
                let mut v9: US0 = fact_parsed_6(v0.clone(), v1);
                match &v9 {
                    US0::US0_FactParsed(v11) => {
                        let mut v11: US1 = v11.clone();
                        let mut v12: i32 = v1.wrapping_add(1i32);
                        (v0, v1) = (v0.clone(), v12);
                        continue;
                    }
                    US0::US0_FactRejected(v10) => {
                        let mut v10: Rc<str> = v10.clone();
                        return v10.clone();
                    }
                }
            }
        } else {
            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v17.clone();
        }
    }
}
fn empty_fact_tally_18() -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0.clone(), false, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64)
}
fn dirs_classified_21(mut v0: Rc<str>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: i32) -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> {
    let mut v3: std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> = (|| -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> { let root = std::path::Path::new(&*v0); let all = &*v1; let start = v2 as usize; let words = |fields: &Vec<std::rc::Rc<str>>| fields.iter().skip(2).filter(|field| !field.is_empty()).map(|field| (field.starts_with(char::from(113u8)), field[1..].to_owned())).collect::<Vec<_>>(); let head = words(&all[start]); let target = head.first().map(|word| word.1.clone()).unwrap_or_default(); let required = head.iter().skip_while(|word| word.1 != "having").skip(1).filter(|word| word.1 != "and").map(|word| word.1.clone()).collect::<Vec<_>>(); let (folder, mask) = match target.rsplit_once(char::from(47u8)) { Some((folder, mask)) if mask.contains(char::from(42u8)) => (folder.to_owned(), mask.to_owned()), _ => (target.clone(), String::new()) }; let wildcard = |mask: &str, name: &str| { let parts = mask.split(char::from(42u8)).collect::<Vec<_>>(); if parts.len() == 1 { return mask == name; } name.starts_with(parts[0]) && name.ends_with(parts[parts.len() - 1]) && name.len() >= parts[0].len() + parts[parts.len() - 1].len() }; let mut candidates = if mask.is_empty() { vec![target.clone()] } else { std::fs::read_dir(root.join(&folder)).map(|entries| entries.filter_map(|entry| entry.ok()).filter(|entry| entry.path().is_dir()).map(|entry| entry.file_name().to_string_lossy().into_owned()).filter(|name| wildcard(&mask, name)).map(|name| format!("{folder}/{name}")).collect::<Vec<_>>()).unwrap_or_default() }; candidates.sort(); let matched = candidates.iter().filter(|dir| required.iter().all(|file| root.join(dir.as_str()).join(file).is_file())).cloned().collect::<Vec<_>>(); let mut tags: Vec<(String, Vec<(bool, String)>)> = Vec::new(); let mut grouping: Option<String> = None; for fields in all.iter().skip(start + 1) { match fields.get(1).map(|verb| &**verb) { Some("tag") => { let line = words(fields); let name = line.first().map(|word| word.1.clone()).unwrap_or_default(); tags.push((name, line.into_iter().skip(2).collect())); } Some("count") => { grouping = words(fields).iter().skip_while(|word| word.1 != "by").nth(1).map(|word| word.1.clone()); } _ => break } } let evaluate = |dir: &str, expression: &[(bool, String)]| -> Result<bool, String> { let mut any = false; let mut all = true; let mut negate = false; let mut index = 0; while index < expression.len() { let (quoted, word) = &expression[index]; if !quoted && word == "and" { index += 1; continue; } if !quoted && word == "or" { any = any || all; all = true; index += 1; continue; } if !quoted && word == "not" { negate = !negate; index += 1; continue; } let file = word.clone(); let pattern = match (expression.get(index + 1), expression.get(index + 2)) { (Some((false, tilde)), Some((true, pattern))) if tilde == "~" => pattern.clone(), _ => return Err(format!("tag: expected <file> ~ <quoted regex> near {file}")) }; let pattern = regex::Regex::new(&pattern).map_err(|error| format!("tag: invalid pattern: {error}"))?; let text = std::fs::read(root.join(dir).join(&file)).map(|bytes| String::from_utf8_lossy(&bytes).into_owned()).unwrap_or_default(); let hit = pattern.is_match(&text) != negate; all = all && hit; negate = false; index += 3; } Ok(any || all) }; let short = |dir: &String| dir.rsplit(char::from(47u8)).next().unwrap_or(dir).to_owned(); let mut values: Vec<Vec<String>> = Vec::new(); for dir in &matched { let mut row = Vec::new(); for (_, expression) in &tags { match evaluate(dir, expression) { Ok(value) => row.push(value.to_string()), Err(problem) => return std::rc::Rc::new((std::rc::Rc::<str>::from(problem), [matched.len() as u64, candidates.len() as u64, 0, 0], true)) } } values.push(row); } let shown = match grouping.as_ref().and_then(|name| tags.iter().position(|tag| &tag.0 == name).map(|index| (name.clone(), index))) { Some((name, index)) => { let mut groups: Vec<(String, u64, Vec<String>)> = Vec::new(); for (dir, row) in matched.iter().zip(&values) { match groups.iter_mut().find(|group| group.0 == row[index]) { Some(group) => { group.1 += 1; group.2.push(short(dir)); } None => groups.push((row[index].clone(), 1, vec![short(dir)])) } } groups.sort_by(|left, right| right.1.cmp(&left.1)); let width = groups.iter().map(|group| group.0.len()).max().unwrap_or(0).max(name.len()); let mut lines = vec![format!("{:<width$}  {:<5}  {}", name, "count", "dirs")]; for group in &groups { lines.push(format!("{:<width$}  {:<5}  {}", group.0, group.1, group.2.join(", "))); } lines } None => { if let Some(name) = &grouping { return std::rc::Rc::new((std::rc::Rc::<str>::from(format!("count by: no tag named {name}")), [matched.len() as u64, candidates.len() as u64, 0, 0], true)); } let width = matched.iter().map(|dir| short(dir).len()).max().unwrap_or(0).max(3); let mut lines = vec![std::iter::once(format!("{:<width$}", "dir")).chain(tags.iter().map(|tag| format!("{:<w$}", tag.0, w = tag.0.len().max(5)))).collect::<Vec<_>>().join("  ").trim_end().to_owned()]; for (dir, row) in matched.iter().zip(&values) { lines.push(std::iter::once(format!("{:<width$}", short(dir))).chain(row.iter().zip(&tags).map(|(value, tag)| format!("{:<w$}", value, w = tag.0.len().max(5)))).collect::<Vec<_>>().join("  ").trim_end().to_owned()); } lines } }; std::rc::Rc::new((std::rc::Rc::<str>::from(shown.join("
")), [matched.len() as u64, candidates.len() as u64, 0, 0], false)) })();
    v3.clone()
}
fn nothing_found_22() -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> {
    let mut v0: std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> = std::rc::Rc::new(({ thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, [0, 0, 0, 0], false));
    v0.clone()
}
fn environment_23(mut v0: Rc<str>) -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> {
    let mut v1: std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> = { let names = (&*v0).lines().filter(|name| !name.is_empty()).map(str::to_owned).collect::<Vec<_>>(); let width = names.iter().map(String::len).max().unwrap_or(0); let mut set = 0u64; let shown = names.iter().map(|name| match std::env::var(name) { Ok(value) => { set += 1; format!("{:<width$}  {:?}", name, value) } Err(_) => format!("{:<width$}  unset", name) }).collect::<Vec<_>>(); std::rc::Rc::new((std::rc::Rc::<str>::from(shown.join("
")), [names.len() as u64, set, 0, 0], false)) };
    v1.clone()
}
fn files_by_extension_24(mut v0: Rc<str>, mut v1: Rc<str>) -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> {
    let mut v2: std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> = { let root = std::path::Path::new(&*v0); let paths = (&*v1).lines().filter(|path| !path.is_empty()).map(str::to_owned).collect::<Vec<_>>(); let mut groups: Vec<(String, u64, u64)> = Vec::new(); let mut total = 0u64; for path in &paths { let extension = std::path::Path::new(path).extension().map(|extension| format!(".{}", extension.to_string_lossy())).unwrap_or_else(|| "(none)".to_owned()); let bytes = std::fs::metadata(root.join(path)).map(|metadata| metadata.len()).unwrap_or(0); total += bytes; match groups.iter_mut().find(|group| group.0 == extension) { Some(group) => { group.1 += 1; group.2 += bytes; } None => groups.push((extension, 1, bytes)) } } groups.sort_by(|left, right| right.1.cmp(&left.1)); let width = groups.iter().map(|group| group.0.len()).max().unwrap_or(0).max(3); let files_width = groups.iter().map(|group| group.1.to_string().len()).max().unwrap_or(0).max(5); let mut shown = vec![format!("{:<width$}  {:<files_width$}  {}", "ext", "files", "bytes")]; for group in &groups { shown.push(format!("{:<width$}  {:<files_width$}  {}", group.0, group.1, group.2)); } std::rc::Rc::new((std::rc::Rc::<str>::from(shown.join("
")), [paths.len() as u64, total, groups.len() as u64, 0], paths.is_empty())) };
    v2.clone()
}
fn hashed_25(mut v0: Rc<str>, mut v1: Rc<str>) -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> {
    let mut v2: std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> = { use sha2::Digest; let root = std::path::Path::new(&*v0); let rows = (&*v1).lines().filter(|path| !path.is_empty()).map(|path| (path.to_owned(), std::fs::read(root.join(path)).ok().map(|bytes| (bytes.len() as u64, sha2::Sha256::digest(&bytes).iter().map(|byte| format!("{byte:02x}")).collect::<String>())))).collect::<Vec<_>>(); let path_width = rows.iter().map(|row| row.0.len()).max().unwrap_or(0).max(4); let bytes_width = rows.iter().map(|row| row.1.as_ref().map(|found| found.0.to_string().len()).unwrap_or(1)).max().unwrap_or(0).max(5); let mut shown = vec![format!("{:<path_width$}  {:<bytes_width$}  {}", "file", "bytes", "sha256")]; let mut failed = rows.is_empty(); let mut total = 0u64; let mut groups: Vec<(String, Vec<String>)> = Vec::new(); for (path, found) in &rows { match found { Some((bytes, digest)) => { total += bytes; shown.push(format!("{:<path_width$}  {:<bytes_width$}  {}", path, bytes, digest)); match groups.iter_mut().find(|group| &group.0 == digest) { Some(group) => group.1.push(path.clone()), None => groups.push((digest.clone(), vec![path.clone()])) } } None => { failed = true; shown.push(format!("{:<path_width$}  {:<bytes_width$}  unreadable", path, "-")); } } } for group in groups.iter().filter(|group| group.1.len() > 1) { shown.push(format!("equal  {}", group.1.join(" = "))); } std::rc::Rc::new((std::rc::Rc::<str>::from(shown.join("
")), [rows.len() as u64, total, groups.len() as u64, 0], failed)) };
    v2.clone()
}
fn json_fields_26(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> {
    let mut v3: std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> = (|| -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> { let path = &*v1; let text = match std::fs::read_to_string(std::path::Path::new(&*v0).join(path)) { Ok(text) => text, Err(error) => return std::rc::Rc::new((std::rc::Rc::<str>::from(format!("{path}  unreadable: {error}")), [1, 0, 0, 0], true)) }; let value: serde_json::Value = match serde_json::from_str(&text) { Ok(value) => value, Err(error) => return std::rc::Rc::new((std::rc::Rc::<str>::from(format!("{path}  invalid json: {error}")), [1, 0, 0, 0], true)) }; let fields = (&*v2).lines().filter(|field| !field.is_empty()).map(str::to_owned).collect::<Vec<_>>(); let width = fields.iter().map(String::len).max().unwrap_or(0); let mut failed = false; let shown = fields.iter().map(|field| { let segments = field.split(char::from(46u8)).collect::<Vec<_>>(); let wants_length = segments.len() > 1 && segments.last() == Some(&"length"); let walk = if wants_length { &segments[..segments.len() - 1] } else { &segments[..] }; let found = walk.iter().try_fold(&value, |current, segment| match current { serde_json::Value::Array(items) => segment.parse::<usize>().ok().and_then(|index| items.get(index)), other => other.get(*segment) }); let shown = match found { None => { failed = true; "missing".to_owned() } Some(found) if wants_length => match found { serde_json::Value::Array(items) => items.len().to_string(), serde_json::Value::Object(entries) => entries.len().to_string(), serde_json::Value::String(text) => text.chars().count().to_string(), _ => { failed = true; "no length".to_owned() } }, Some(found) => found.to_string() }; format!("{:<width$}  {}", field, shown) }).collect::<Vec<_>>(); std::rc::Rc::new((std::rc::Rc::<str>::from(shown.join("
")), [1, fields.len() as u64, 0, 0], failed)) })();
    v3.clone()
}
fn lines_grouped_27(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> {
    let mut v3: std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> = (|| -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> { let pattern = match regex::Regex::new(&*v2) { Ok(pattern) => pattern, Err(error) => return std::rc::Rc::new((std::rc::Rc::<str>::from(format!("lines: invalid pattern: {error}")), [1, 0, 0, 0], true)) }; let path = &*v1; let text = match std::fs::read(std::path::Path::new(&*v0).join(path)) { Ok(bytes) => String::from_utf8_lossy(&bytes).into_owned(), Err(error) => return std::rc::Rc::new((std::rc::Rc::<str>::from(format!("{path}  unreadable: {error}")), [1, 0, 0, 0], true)) }; let mut groups: Vec<(String, u64)> = Vec::new(); let mut total = 0u64; for line in text.lines() { total += 1; let key = match pattern.captures(line) { Some(captures) => captures.get(1).or_else(|| captures.get(0)).map(|found| found.as_str().to_owned()).unwrap_or_default(), None => "(no match)".to_owned() }; match groups.iter_mut().find(|group| group.0 == key) { Some(group) => group.1 += 1, None => groups.push((key, 1)) } } groups.sort_by(|left, right| right.1.cmp(&left.1)); let width = groups.iter().map(|group| group.0.len()).max().unwrap_or(0).max(5); let mut shown = vec![format!("{:<width$}  {}", "group", "count")]; for group in &groups { shown.push(format!("{:<width$}  {}", group.0, group.1)); } std::rc::Rc::new((std::rc::Rc::<str>::from(shown.join("
")), [1, total, groups.len() as u64, 0], false)) })();
    v3.clone()
}
fn nearest_marker_28(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> {
    let mut v4: std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> = (|| -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> { let pattern = match regex::Regex::new(&*v2) { Ok(pattern) => pattern, Err(error) => return std::rc::Rc::new((std::rc::Rc::<str>::from(format!("nearest: invalid pattern: {error}")), [1, 0, 0, 0], true)) }; let path = &*v1; let text = match std::fs::read(std::path::Path::new(&*v0).join(path)) { Ok(bytes) => String::from_utf8_lossy(&bytes).into_owned(), Err(error) => return std::rc::Rc::new((std::rc::Rc::<str>::from(format!("{path}  unreadable: {error}")), [1, 0, 0, 0], true)) }; let lines = text.lines().collect::<Vec<_>>(); let targets = (&*v3).lines().filter_map(|target| target.trim().parse::<usize>().ok()).collect::<Vec<_>>(); let rows = targets.iter().map(|target| { let found = (0..(*target).min(lines.len())).rev().find(|index| pattern.is_match(lines[*index])); (target.to_string(), found.map(|index| (index + 1).to_string()).unwrap_or_else(|| "-".to_owned()), found.map(|index| lines[index].to_owned()).unwrap_or_else(|| "(none above)".to_owned())) }).collect::<Vec<_>>(); let line_width = rows.iter().map(|row| row.0.len()).max().unwrap_or(0).max(4); let nearest_width = rows.iter().map(|row| row.1.len()).max().unwrap_or(0).max(7); let mut shown = vec![format!("{:<line_width$}  {:<nearest_width$}  {}", "line", "nearest", "text")]; for row in &rows { shown.push(format!("{:<line_width$}  {:<nearest_width$}  {}", row.0, row.1, row.2)); } std::rc::Rc::new((std::rc::Rc::<str>::from(shown.join("
")), [1, rows.len() as u64, 0, 0], false)) })();
    v4.clone()
}
fn existence_29(mut v0: Rc<str>, mut v1: Rc<str>) -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> {
    let mut v2: std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> = { let root = std::path::Path::new(&*v0); let paths = (&*v1).lines().filter(|path| !path.is_empty()).map(str::to_owned).collect::<Vec<_>>(); let width = paths.iter().map(String::len).max().unwrap_or(0); let mut present = 0u64; let shown = paths.iter().map(|path| { let full = root.join(path); match std::fs::metadata(&full) { Ok(metadata) if metadata.is_dir() => { present += 1; let entries = std::fs::read_dir(&full).map(|entries| entries.count()).unwrap_or(0); format!("{:<width$}  directory  {} {}", path, entries, if entries == 1 { "entry" } else { "entries" }) } Ok(metadata) => { present += 1; format!("{:<width$}  file  {} bytes", path, metadata.len()) } Err(_) => format!("{:<width$}  missing", path) } }).collect::<Vec<_>>(); std::rc::Rc::new((std::rc::Rc::<str>::from(shown.join("
")), [paths.len() as u64, present, 0, 0], false)) };
    v2.clone()
}
fn table_counted_30(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: bool, mut v4: Rc<str>, mut v5: Rc<str>) -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> {
    let mut v6: std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> = (|| -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> { let path = &*v1; let text = match std::fs::read_to_string(std::path::Path::new(&*v0).join(path)) { Ok(text) => text, Err(error) => return std::rc::Rc::new((std::rc::Rc::<str>::from(format!("{path}  unreadable: {error}")), [0, 0, 0, 0], true)) }; let separator = if path.ends_with(".csv") { char::from(44u8) } else { char::from(9u8) }; let mut lines = text.lines(); let header = lines.next().unwrap_or("").split(separator).map(str::trim).collect::<Vec<_>>(); let position = |name: &str| header.iter().position(|title| *title == name); let wanted = (&*v5).lines().filter(|name| !name.is_empty()).collect::<Vec<_>>(); let mut missing = wanted.iter().filter(|name| position(name).is_none()).map(|name| name.to_string()).collect::<Vec<_>>(); let filter = if (&*v2).is_empty() { None } else { match position(&*v2) { Some(index) => Some(index), None => { missing.push((&*v2).to_owned()); None } } }; if !missing.is_empty() { return std::rc::Rc::new((std::rc::Rc::<str>::from(format!("{path}  no column {} (columns: {})", missing.join(", "), header.join(", "))), [0, 0, 0, 0], true)); } let indexes = wanted.iter().filter_map(|name| position(name)).collect::<Vec<_>>(); let mut rows = 0u64; let mut matched = 0u64; let mut counts: Vec<(Vec<String>, u64)> = Vec::new(); for line in lines.filter(|line| !line.trim().is_empty()) { rows += 1; let cells = line.split(separator).map(str::trim).collect::<Vec<_>>(); if let Some(index) = filter { if (cells.get(index).copied().unwrap_or("") == &*v4) != v3 { continue; } } matched += 1; let key = indexes.iter().map(|index| cells.get(*index).copied().unwrap_or("").to_owned()).collect::<Vec<_>>(); match counts.iter_mut().find(|entry| entry.0 == key) { Some(entry) => entry.1 += 1, None => counts.push((key, 1)) } } counts.sort_by(|left, right| right.1.cmp(&left.1)); let widths = wanted.iter().enumerate().map(|(column, name)| counts.iter().map(|entry| entry.0[column].len()).max().unwrap_or(0).max(name.len())).collect::<Vec<_>>(); let render = |cells: Vec<String>, count: String| { let mut parts = cells.iter().zip(&widths).map(|(cell, width)| format!("{:<width$}", cell, width = *width)).collect::<Vec<_>>(); parts.push(count); parts.join("  ") }; let mut shown = vec![render(wanted.iter().map(|name| name.to_string()).collect(), "count".to_owned())]; for entry in &counts { shown.push(render(entry.0.clone(), entry.1.to_string())); } std::rc::Rc::new((std::rc::Rc::<str>::from(shown.join("
")), [rows, matched, counts.len() as u64, 0], false)) })();
    v6.clone()
}
fn expanded_31(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from((&*v1).lines().filter(|item| !item.is_empty()).flat_map(|item| eoie_do_read::eoie_do_read_matching(&*v0, item).lines().map(str::to_owned).collect::<Vec<_>>()).collect::<Vec<_>>().join("
"));
    v2.clone()
}
fn fact_step_20(mut v0: Rc<str>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: US1) -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> {
    match &v2 {
        US1::US1_DirsClassified(v29) => {
            let mut v29: i32 = *v29;
            dirs_classified_21(v0.clone(), v1.clone(), v29)
        }
        US1::US1_DirsDetail => {
            nothing_found_22()
        }
        US1::US1_EnvValues(v6) => {
            let mut v6: Rc<str> = v6.clone();
            environment_23(v6.clone())
        }
        US1::US1_FilesCounted(v20) => {
            let mut v20: Rc<str> = v20.clone();
            let mut v21: Rc<str> = expanded_31(v0.clone(), v20.clone());
            files_by_extension_24(v0.clone(), v21.clone())
        }
        US1::US1_HashFiles(v3) => {
            let mut v3: Rc<str> = v3.clone();
            let mut v4: Rc<str> = expanded_31(v0.clone(), v3.clone());
            hashed_25(v0.clone(), v4.clone())
        }
        US1::US1_JsonFields(v14, v15) => {
            let mut v14: Rc<str> = v14.clone();
            let mut v15: Rc<str> = v15.clone();
            json_fields_26(v0.clone(), v14.clone(), v15.clone())
        }
        US1::US1_LinesCounted(v17, v18) => {
            let mut v17: Rc<str> = v17.clone();
            let mut v18: Rc<str> = v18.clone();
            lines_grouped_27(v0.clone(), v17.clone(), v18.clone())
        }
        US1::US1_NearestMarker(v10, v11, v12) => {
            let mut v10: Rc<str> = v10.clone();
            let mut v11: Rc<str> = v11.clone();
            let mut v12: Rc<str> = v12.clone();
            nearest_marker_28(v0.clone(), v12.clone(), v10.clone(), v11.clone())
        }
        US1::US1_PathsExist(v8) => {
            let mut v8: Rc<str> = v8.clone();
            existence_29(v0.clone(), v8.clone())
        }
        US1::US1_TableCounted(v23, v24, v25, v26, v27) => {
            let mut v23: Rc<str> = v23.clone();
            let mut v24: Rc<str> = v24.clone();
            let mut v25: bool = *v25;
            let mut v26: Rc<str> = v26.clone();
            let mut v27: Rc<str> = v27.clone();
            table_counted_30(v0.clone(), v23.clone(), v24.clone(), v25, v26.clone(), v27.clone())
        }
    }
}
fn lines_joined_33(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v2 {
        v0.clone()
    } else {
        let mut v3: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v3 {
            v1.clone()
        } else {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v4, &*v1, &*v5, &*v5].concat());
            v6.clone()
        }
    }
}
fn outcome_merged_32(mut v0: Rc<str>, mut v1: bool, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: u64, mut v16: u64, mut v17: u64, mut v18: u64, mut v19: u64, mut v20: u64, mut v21: u64, mut v22: u64, mut v23: u64, mut v24: u64, mut v25: u64, mut v26: u64, mut v27: US1, mut v28: std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)>) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let mut v29: Rc<str> = v28.0.clone();
    let mut v30: Rc<str> = lines_joined_33(v0.clone(), v29.clone());
    let mut v32: bool = if v1 {
        true
    } else {
        let mut v31: bool = v28.clone().2;
        v31
    };
    let mut v33: u64 = 0u64;
    let mut v34: u64 = v28.1[v33 as usize];
    let mut v35: u64 = 1u64;
    let mut v36: u64 = v28.1[v35 as usize];
    let mut v37: u64 = 2u64;
    let mut v38: u64 = v28.1[v37 as usize];
    let (mut v329, mut v330, mut v331, mut v332, mut v333, mut v334, mut v335, mut v336, mut v337, mut v338, mut v339, mut v340, mut v341, mut v342, mut v343, mut v344, mut v345, mut v346, mut v347, mut v348, mut v349, mut v350, mut v351, mut v352, mut v353, mut v354, mut v355): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v27 {
        US1::US1_HashFiles(v39) => {
            let mut v39: Rc<str> = v39.clone();
            let mut v40: u64 = v2.wrapping_add(v34);
            let mut v41: u64 = v3.wrapping_add(v36);
            let mut v42: u64 = v4.wrapping_add(v38);
            let mut v43: u64 = v5.wrapping_add(1u64);
            (v30.clone(), v32, v40, v41, v42, v43, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26)
        }
        _ => {
            let (mut v302, mut v303, mut v304, mut v305, mut v306, mut v307, mut v308, mut v309, mut v310, mut v311, mut v312, mut v313, mut v314, mut v315, mut v316, mut v317, mut v318, mut v319, mut v320, mut v321, mut v322, mut v323, mut v324, mut v325, mut v326, mut v327, mut v328): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v27 {
                US1::US1_EnvValues(v44) => {
                    let mut v44: Rc<str> = v44.clone();
                    let mut v45: u64 = v14.wrapping_add(v34);
                    let mut v46: u64 = v15.wrapping_add(v36);
                    let mut v47: u64 = v16.wrapping_add(1u64);
                    (v30.clone(), v32, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v45, v46, v47, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26)
                }
                _ => {
                    let (mut v275, mut v276, mut v277, mut v278, mut v279, mut v280, mut v281, mut v282, mut v283, mut v284, mut v285, mut v286, mut v287, mut v288, mut v289, mut v290, mut v291, mut v292, mut v293, mut v294, mut v295, mut v296, mut v297, mut v298, mut v299, mut v300, mut v301): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v27 {
                        US1::US1_PathsExist(v48) => {
                            let mut v48: Rc<str> = v48.clone();
                            let mut v49: u64 = v17.wrapping_add(v34);
                            let mut v50: u64 = v18.wrapping_add(v36);
                            let mut v51: u64 = v19.wrapping_add(1u64);
                            (v30.clone(), v32, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v49, v50, v51, v20, v21, v22, v23, v24, v25, v26)
                        }
                        _ => {
                            let (mut v248, mut v249, mut v250, mut v251, mut v252, mut v253, mut v254, mut v255, mut v256, mut v257, mut v258, mut v259, mut v260, mut v261, mut v262, mut v263, mut v264, mut v265, mut v266, mut v267, mut v268, mut v269, mut v270, mut v271, mut v272, mut v273, mut v274): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v27 {
                                US1::US1_NearestMarker(v52, v53, v54) => {
                                    let mut v52: Rc<str> = v52.clone();
                                    let mut v53: Rc<str> = v53.clone();
                                    let mut v54: Rc<str> = v54.clone();
                                    let mut v55: u64 = v2.wrapping_add(v34);
                                    let mut v56: u64 = v6.wrapping_add(v36);
                                    let mut v57: u64 = v7.wrapping_add(1u64);
                                    (v30.clone(), v32, v55, v3, v4, v5, v56, v57, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26)
                                }
                                _ => {
                                    let (mut v221, mut v222, mut v223, mut v224, mut v225, mut v226, mut v227, mut v228, mut v229, mut v230, mut v231, mut v232, mut v233, mut v234, mut v235, mut v236, mut v237, mut v238, mut v239, mut v240, mut v241, mut v242, mut v243, mut v244, mut v245, mut v246, mut v247): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v27 {
                                        US1::US1_JsonFields(v58, v59) => {
                                            let mut v58: Rc<str> = v58.clone();
                                            let mut v59: Rc<str> = v59.clone();
                                            let mut v60: u64 = v2.wrapping_add(v34);
                                            let mut v61: u64 = v8.wrapping_add(v36);
                                            let mut v62: u64 = v9.wrapping_add(1u64);
                                            (v30.clone(), v32, v60, v3, v4, v5, v6, v7, v61, v62, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26)
                                        }
                                        _ => {
                                            let (mut v194, mut v195, mut v196, mut v197, mut v198, mut v199, mut v200, mut v201, mut v202, mut v203, mut v204, mut v205, mut v206, mut v207, mut v208, mut v209, mut v210, mut v211, mut v212, mut v213, mut v214, mut v215, mut v216, mut v217, mut v218, mut v219, mut v220): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v27 {
                                                US1::US1_LinesCounted(v63, v64) => {
                                                    let mut v63: Rc<str> = v63.clone();
                                                    let mut v64: Rc<str> = v64.clone();
                                                    let mut v65: u64 = v2.wrapping_add(v34);
                                                    let mut v66: u64 = v10.wrapping_add(v36);
                                                    let mut v67: u64 = v11.wrapping_add(v38);
                                                    let mut v68: u64 = v12.wrapping_add(1u64);
                                                    (v30.clone(), v32, v65, v3, v4, v5, v6, v7, v8, v9, v66, v67, v68, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26)
                                                }
                                                _ => {
                                                    let (mut v167, mut v168, mut v169, mut v170, mut v171, mut v172, mut v173, mut v174, mut v175, mut v176, mut v177, mut v178, mut v179, mut v180, mut v181, mut v182, mut v183, mut v184, mut v185, mut v186, mut v187, mut v188, mut v189, mut v190, mut v191, mut v192, mut v193): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v27 {
                                                        US1::US1_FilesCounted(v69) => {
                                                            let mut v69: Rc<str> = v69.clone();
                                                            let mut v70: u64 = v2.wrapping_add(v34);
                                                            let mut v71: u64 = v3.wrapping_add(v36);
                                                            let mut v72: u64 = v13.wrapping_add(1u64);
                                                            (v30.clone(), v32, v70, v71, v4, v5, v6, v7, v8, v9, v10, v11, v12, v72, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26)
                                                        }
                                                        _ => {
                                                            let (mut v140, mut v141, mut v142, mut v143, mut v144, mut v145, mut v146, mut v147, mut v148, mut v149, mut v150, mut v151, mut v152, mut v153, mut v154, mut v155, mut v156, mut v157, mut v158, mut v159, mut v160, mut v161, mut v162, mut v163, mut v164, mut v165, mut v166): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v27 {
                                                                US1::US1_TableCounted(v73, v74, v75, v76, v77) => {
                                                                    let mut v73: Rc<str> = v73.clone();
                                                                    let mut v74: Rc<str> = v74.clone();
                                                                    let mut v75: bool = *v75;
                                                                    let mut v76: Rc<str> = v76.clone();
                                                                    let mut v77: Rc<str> = v77.clone();
                                                                    let mut v78: u64 = v20.wrapping_add(v34);
                                                                    let mut v79: u64 = v21.wrapping_add(v36);
                                                                    let mut v80: u64 = v22.wrapping_add(v38);
                                                                    let mut v81: u64 = v23.wrapping_add(1u64);
                                                                    (v30.clone(), v32, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v78, v79, v80, v81, v24, v25, v26)
                                                                }
                                                                _ => {
                                                                    let (mut v113, mut v114, mut v115, mut v116, mut v117, mut v118, mut v119, mut v120, mut v121, mut v122, mut v123, mut v124, mut v125, mut v126, mut v127, mut v128, mut v129, mut v130, mut v131, mut v132, mut v133, mut v134, mut v135, mut v136, mut v137, mut v138, mut v139): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v27 {
                                                                        US1::US1_DirsClassified(v82) => {
                                                                            let mut v82: i32 = *v82;
                                                                            let mut v83: u64 = v24.wrapping_add(v34);
                                                                            let mut v84: u64 = v25.wrapping_add(v36);
                                                                            let mut v85: u64 = v26.wrapping_add(1u64);
                                                                            (v30.clone(), v32, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v83, v84, v85)
                                                                        }
                                                                        _ => {
                                                                            let (mut v86, mut v87, mut v88, mut v89, mut v90, mut v91, mut v92, mut v93, mut v94, mut v95, mut v96, mut v97, mut v98, mut v99, mut v100, mut v101, mut v102, mut v103, mut v104, mut v105, mut v106, mut v107, mut v108, mut v109, mut v110, mut v111, mut v112): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v27 {
                                                                                US1::US1_DirsDetail => {
                                                                                    (v0.clone(), v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26)
                                                                                }
                                                                                _ => unreachable!(),
                                                                            };
                                                                            (v86.clone(), v87, v88, v89, v90, v91, v92, v93, v94, v95, v96, v97, v98, v99, v100, v101, v102, v103, v104, v105, v106, v107, v108, v109, v110, v111, v112)
                                                                        }
                                                                    };
                                                                    (v113.clone(), v114, v115, v116, v117, v118, v119, v120, v121, v122, v123, v124, v125, v126, v127, v128, v129, v130, v131, v132, v133, v134, v135, v136, v137, v138, v139)
                                                                }
                                                            };
                                                            (v140.clone(), v141, v142, v143, v144, v145, v146, v147, v148, v149, v150, v151, v152, v153, v154, v155, v156, v157, v158, v159, v160, v161, v162, v163, v164, v165, v166)
                                                        }
                                                    };
                                                    (v167.clone(), v168, v169, v170, v171, v172, v173, v174, v175, v176, v177, v178, v179, v180, v181, v182, v183, v184, v185, v186, v187, v188, v189, v190, v191, v192, v193)
                                                }
                                            };
                                            (v194.clone(), v195, v196, v197, v198, v199, v200, v201, v202, v203, v204, v205, v206, v207, v208, v209, v210, v211, v212, v213, v214, v215, v216, v217, v218, v219, v220)
                                        }
                                    };
                                    (v221.clone(), v222, v223, v224, v225, v226, v227, v228, v229, v230, v231, v232, v233, v234, v235, v236, v237, v238, v239, v240, v241, v242, v243, v244, v245, v246, v247)
                                }
                            };
                            (v248.clone(), v249, v250, v251, v252, v253, v254, v255, v256, v257, v258, v259, v260, v261, v262, v263, v264, v265, v266, v267, v268, v269, v270, v271, v272, v273, v274)
                        }
                    };
                    (v275.clone(), v276, v277, v278, v279, v280, v281, v282, v283, v284, v285, v286, v287, v288, v289, v290, v291, v292, v293, v294, v295, v296, v297, v298, v299, v300, v301)
                }
            };
            (v302.clone(), v303, v304, v305, v306, v307, v308, v309, v310, v311, v312, v313, v314, v315, v316, v317, v318, v319, v320, v321, v322, v323, v324, v325, v326, v327, v328)
        }
    };
    (v329.clone(), v330, v331, v332, v333, v334, v335, v336, v337, v338, v339, v340, v341, v342, v343, v344, v345, v346, v347, v348, v349, v350, v351, v352, v353, v354, v355)
}
fn facts_run_19(mut v0: Rc<str>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: i32, mut v3: Rc<str>, mut v4: bool, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: u64, mut v16: u64, mut v17: u64, mut v18: u64, mut v19: u64, mut v20: u64, mut v21: u64, mut v22: u64, mut v23: u64, mut v24: u64, mut v25: u64, mut v26: u64, mut v27: u64, mut v28: u64, mut v29: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let mut v30: i32 = v1.len() as i32;
    let mut v31: bool = v2 < v30;
    let (mut v173, mut v174, mut v175, mut v176, mut v177, mut v178, mut v179, mut v180, mut v181, mut v182, mut v183, mut v184, mut v185, mut v186, mut v187, mut v188, mut v189, mut v190, mut v191, mut v192, mut v193, mut v194, mut v195, mut v196, mut v197, mut v198, mut v199): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = if v31 {
        let mut v32: US0 = fact_parsed_6(v1.clone(), v2);
        let (mut v146, mut v147, mut v148, mut v149, mut v150, mut v151, mut v152, mut v153, mut v154, mut v155, mut v156, mut v157, mut v158, mut v159, mut v160, mut v161, mut v162, mut v163, mut v164, mut v165, mut v166, mut v167, mut v168, mut v169, mut v170, mut v171, mut v172): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v32 {
            US0::US0_FactParsed(v33) => {
                let mut v33: US1 = v33.clone();
                let mut v34: i32 = v2.wrapping_add(1i32);
                let mut v35: std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> = fact_step_20(v0.clone(), v1.clone(), v33.clone());
                let (mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = outcome_merged_32(v3.clone(), v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v33.clone(), v35.clone());
                facts_run_19(v0.clone(), v1.clone(), v34, v36.clone(), v37, v38, v39, v40, v41, v42, v43, v44, v45, v46, v47, v48, v49, v50, v51, v52, v53, v54, v55, v56, v57, v58, v59, v60, v61, v62)
            }
            _ => {
                let (mut v119, mut v120, mut v121, mut v122, mut v123, mut v124, mut v125, mut v126, mut v127, mut v128, mut v129, mut v130, mut v131, mut v132, mut v133, mut v134, mut v135, mut v136, mut v137, mut v138, mut v139, mut v140, mut v141, mut v142, mut v143, mut v144, mut v145): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v32 {
                    US0::US0_FactRejected(v90) => {
                        let mut v90: Rc<str> = v90.clone();
                        let mut v91: i32 = v2.wrapping_add(1i32);
                        facts_run_19(v0.clone(), v1.clone(), v91, v3.clone(), v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29)
                    }
                    _ => unreachable!(),
                };
                (v119.clone(), v120, v121, v122, v123, v124, v125, v126, v127, v128, v129, v130, v131, v132, v133, v134, v135, v136, v137, v138, v139, v140, v141, v142, v143, v144, v145)
            }
        };
        (v146.clone(), v147, v148, v149, v150, v151, v152, v153, v154, v155, v156, v157, v158, v159, v160, v161, v162, v163, v164, v165, v166, v167, v168, v169, v170, v171, v172)
    } else {
        (v3.clone(), v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29)
    };
    (v173.clone(), v174, v175, v176, v177, v178, v179, v180, v181, v182, v183, v184, v185, v186, v187, v188, v189, v190, v191, v192, v193, v194, v195, v196, v197, v198, v199)
}
fn segments_joined_36(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v2 {
        v1.clone()
    } else {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v3, &*v1, &*v4, &*v4].concat());
        v5.clone()
    }
}
fn decimal_text_37(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1.clone()
}
fn counted_segment_35(mut v0: bool, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64) -> Rc<str> {
    if v0 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = decimal_text_37(v3);
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v4, &*v5, &*v6, &*v6].concat());
        segments_joined_36(v1.clone(), v7.clone())
    } else {
        v1.clone()
    }
}
fn receipt_counts_34(mut v0: Rc<str>, mut v1: bool, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: u64, mut v16: u64, mut v17: u64, mut v18: u64, mut v19: u64, mut v20: u64, mut v21: u64, mut v22: u64, mut v23: u64, mut v24: u64, mut v25: u64, mut v26: u64) -> Rc<str> {
    let mut v27: bool = v26 > 0u64;
    let mut v35: Rc<str> = if v27 {
        let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dirs "); } LIT.with(|lit| lit.clone()) };
        let mut v29: Rc<str> = decimal_text_37(v24);
        let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" of "); } LIT.with(|lit| lit.clone()) };
        let mut v31: Rc<str> = decimal_text_37(v25);
        let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" matched"); } LIT.with(|lit| lit.clone()) };
        let mut v33: Rc<str> = std::rc::Rc::<str>::from([&*v28, &*v29, &*v30, &*v31, &*v32].concat());
        v33.clone()
    } else {
        let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v34.clone()
    };
    let mut v36: u64 = v5.wrapping_add(v7);
    let mut v37: u64 = v36.wrapping_add(v9);
    let mut v38: u64 = v37.wrapping_add(v12);
    let mut v39: u64 = v38.wrapping_add(v13);
    let mut v40: bool = v39 > 0u64;
    let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files"); } LIT.with(|lit| lit.clone()) };
    let mut v42: Rc<str> = counted_segment_35(v40, v35.clone(), v41.clone(), v2);
    let mut v43: u64 = v5.wrapping_add(v13);
    let mut v44: bool = v43 > 0u64;
    let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bytes"); } LIT.with(|lit| lit.clone()) };
    let mut v46: Rc<str> = counted_segment_35(v44, v42.clone(), v45.clone(), v3);
    let mut v47: bool = v5 > 0u64;
    let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("distinct"); } LIT.with(|lit| lit.clone()) };
    let mut v49: Rc<str> = counted_segment_35(v47, v46.clone(), v48.clone(), v4);
    let mut v50: bool = v7 > 0u64;
    let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lookups"); } LIT.with(|lit| lit.clone()) };
    let mut v52: Rc<str> = counted_segment_35(v50, v49.clone(), v51.clone(), v6);
    let mut v53: bool = v9 > 0u64;
    let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fields"); } LIT.with(|lit| lit.clone()) };
    let mut v55: Rc<str> = counted_segment_35(v53, v52.clone(), v54.clone(), v8);
    let mut v56: bool = v12 > 0u64;
    let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lines"); } LIT.with(|lit| lit.clone()) };
    let mut v58: Rc<str> = counted_segment_35(v56, v55.clone(), v57.clone(), v10);
    let mut v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("groups"); } LIT.with(|lit| lit.clone()) };
    let mut v60: Rc<str> = counted_segment_35(v56, v58.clone(), v59.clone(), v11);
    let mut v61: bool = v16 > 0u64;
    let mut v69: Rc<str> = if v61 {
        let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("env "); } LIT.with(|lit| lit.clone()) };
        let mut v63: Rc<str> = decimal_text_37(v14);
        let mut v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ("); } LIT.with(|lit| lit.clone()) };
        let mut v65: Rc<str> = decimal_text_37(v15);
        let mut v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" set)"); } LIT.with(|lit| lit.clone()) };
        let mut v67: Rc<str> = std::rc::Rc::<str>::from([&*v62, &*v63, &*v64, &*v65, &*v66].concat());
        segments_joined_36(v60.clone(), v67.clone())
    } else {
        v60.clone()
    };
    let mut v70: bool = v19 > 0u64;
    let mut v78: Rc<str> = if v70 {
        let mut v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("paths "); } LIT.with(|lit| lit.clone()) };
        let mut v72: Rc<str> = decimal_text_37(v17);
        let mut v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ("); } LIT.with(|lit| lit.clone()) };
        let mut v74: Rc<str> = decimal_text_37(v18);
        let mut v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" present)"); } LIT.with(|lit| lit.clone()) };
        let mut v76: Rc<str> = std::rc::Rc::<str>::from([&*v71, &*v72, &*v73, &*v74, &*v75].concat());
        segments_joined_36(v69.clone(), v76.clone())
    } else {
        v69.clone()
    };
    let mut v79: bool = v23 > 0u64;
    let mut v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rows"); } LIT.with(|lit| lit.clone()) };
    let mut v81: Rc<str> = counted_segment_35(v79, v78.clone(), v80.clone(), v20);
    let mut v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("matched"); } LIT.with(|lit| lit.clone()) };
    let mut v83: Rc<str> = counted_segment_35(v79, v81.clone(), v82.clone(), v21);
    counted_segment_35(v79, v83.clone(), v59.clone(), v22)
}
fn elapsed_text_39(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1.clone()
}
fn local_offset_minutes_41() -> i64 {
    let mut v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn clock_text_40() -> Rc<str> {
    let mut v0: i64 = local_offset_minutes_41();
    let mut v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1.clone()
}
fn receipt_line_38(mut v0: US2, mut v1: Rc<str>, mut v2: u64) -> () {
    let mut v6: Rc<str> = match &v0 {
        US2::US2_Failed => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US2::US2_Succeeded => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
    };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = elapsed_text_39(v2);
    let mut v9: Rc<str> = clock_text_40();
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v7, &*v9, &*v10, &*v10].concat());
    let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v7, &*v1, &*v7, &*v11].concat());
    println!("{}", &*v12);
    ()
}
fn facts_program_run_0(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: u64 = now_ms_2();
    let mut v3: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = records_of_3(v1.clone());
    let mut v4: i32 = 1i32;
    let mut v5: Rc<str> = first_rejection_4(v3.clone(), v4);
    let mut v6: bool = v3.iter().skip(1).any(|fields| fields.get(1).map(|verb| &**verb == "diff").unwrap_or(false));
    if v6 {
        tables_run_1(v0.clone(), v1.clone())
    } else {
        let mut v8: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v8 {
            let mut v9: i32 = 1i32;
            let (mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = empty_fact_tally_18();
            let (mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = facts_run_19(v0.clone(), v3.clone(), v9, v10.clone(), v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31, v32, v33, v34, v35, v36);
            let mut v64: bool = v37.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v64 {
                ()
            } else {
                println!("{}", &*v37);
                ()
            };
            let mut v65: Rc<str> = receipt_counts_34(v37.clone(), v38, v39, v40, v41, v42, v43, v44, v45, v46, v47, v48, v49, v50, v51, v52, v53, v54, v55, v56, v57, v58, v59, v60, v61, v62, v63);
            if v38 {
                let mut v66: US2 = US2::US2_Failed;
                receipt_line_38(v66.clone(), v65.clone(), v2);
                1i32
            } else {
                let mut v67: US2 = US2::US2_Succeeded;
                receipt_line_38(v67.clone(), v65.clone(), v2);
                0i32
            }
        } else {
            let mut v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("program rejected: "); } LIT.with(|lit| lit.clone()) };
            let mut v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v71: Rc<str> = std::rc::Rc::<str>::from([&*v69, &*v5, &*v70, &*v70, &*v70].concat());
            println!("{}", &*v71);
            let mut v72: US2 = US2::US2_Failed;
            let mut v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files 0"); } LIT.with(|lit| lit.clone()) };
            receipt_line_38(v72.clone(), v73.clone(), v2);
            2i32
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        facts_program_run_0(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_do_facts_run(v0: &str, v1: &str) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
