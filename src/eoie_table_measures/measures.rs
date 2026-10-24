#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Equal,
    US0_NotEqual,
    US0_Greater,
    US0_GreaterEqual,
    US0_Less,
    US0_LessEqual,
}
#[derive(Clone)]
enum UH0 {
    UH0_NoFilters,
    UH0_FilterThen(i32, US0, Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US1 {
    US1_MeanOf,
    US1_SdOf,
    US1_SumOf,
    US1_MinOf,
    US1_MaxOf,
    US1_CountOf,
}
#[derive(Clone)]
enum UH1 {
    UH1_NoMeasures,
    UH1_MeasureThen(US1, Rc<str>, i32, Rc<UH1>),
}
#[derive(Clone)]
enum UH2 {
    UH2_NoMeasureStats,
    UH2_MeasureStatsThen(i32, i32, f64, f64, f64, f64, Rc<UH2>),
}
#[derive(Clone)]
enum UH3 {
    UH3_NoGroups,
    UH3_GroupThen(Rc<str>, i32, Rc<UH2>, Rc<UH3>),
}
#[derive(Clone)]
enum UH4 {
    UH4_NoCells,
    UH4_CellThen(Rc<str>, Rc<UH4>),
}
#[derive(Clone)]
enum US2 {
    US2_TableLoaded(std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>),
    US2_TableUnreadable(Rc<str>),
}
fn filters_from_3(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>) -> (Rc<UH0>, Rc<str>) {
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v2 {
        let mut v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoFilters); } CASE.with(|case| case.clone()) };
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (v3.clone(), v4.clone())
    } else {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = std::rc::Rc::<str>::from((v1).split(&*v5).next().unwrap_or(""));
        let mut v7: i32 = (v6).find(|c: char| c == char::from(60u8) || c == char::from(62u8) || c == char::from(61u8) || c == char::from(33u8)).map(|at| at as i32).unwrap_or(-1);
        let mut v8: bool = v7 < 0i32;
        let mut v11: Rc<str> = if v8 {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        } else {
            let mut v10: Rc<str> = std::rc::Rc::<str>::from({ let text = &(v6)[v7 as usize..]; let bytes = text.as_bytes(); if bytes.len() > 1 && bytes[1] == 61u8 && bytes[0] != 61u8 { &text[..2] } else { &text[..1] } });
            v10.clone()
        };
        let mut v13: Rc<str> = if v8 {
            v6.clone()
        } else {
            let mut v12: Rc<str> = std::rc::Rc::<str>::from(&(v6)[..v7 as usize]);
            v12.clone()
        };
        let mut v14: i32 = (v0).first().and_then(|header| header.iter().position(|item| &**item == &*v13)).map(|index| index as i32).unwrap_or(-1);
        let mut v15: Rc<str> = std::rc::Rc::<str>::from((v1).split_once(&*v5).map(|(_, rest)| rest).unwrap_or(""));
        let (mut v16, mut v17): (Rc<UH0>, Rc<str>) = filters_from_3(v0.clone(), v15.clone());
        let mut v18: bool = v14 < 0i32;
        if v18 {
            let mut v19: bool = v13.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v25: Rc<str> = if v19 {
                v17.clone()
            } else {
                let mut v20: bool = v17.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                if v20 {
                    v13.clone()
                } else {
                    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v23: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v21, &*v17, &*v22, &*v22].concat());
                    v23.clone()
                }
            };
            (v16.clone(), v25.clone())
        } else {
            let mut v26: bool = v11.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">="); } LIT.with(|lit| lit.clone()) };
            let mut v41: US0 = if v26 {
                US0::US0_GreaterEqual
            } else {
                let mut v28: bool = v11.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<="); } LIT.with(|lit| lit.clone()) };
                if v28 {
                    US0::US0_LessEqual
                } else {
                    let mut v30: bool = v11.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("!="); } LIT.with(|lit| lit.clone()) };
                    if v30 {
                        US0::US0_NotEqual
                    } else {
                        let mut v32: bool = v11.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) };
                        if v32 {
                            US0::US0_Greater
                        } else {
                            let mut v34: bool = v11.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<"); } LIT.with(|lit| lit.clone()) };
                            if v34 {
                                US0::US0_Less
                            } else {
                                US0::US0_Equal
                            }
                        }
                    }
                }
            };
            let mut v42: i32 = (v11.clone().len() as i32);
            let mut v43: i32 = v7.wrapping_add(v42);
            let mut v44: Rc<str> = std::rc::Rc::<str>::from(&(v6)[v43 as usize..]);
            let mut v45: Rc<UH0> = Rc::new(UH0::UH0_FilterThen(v14, v41.clone(), v44.clone(), v16.clone()));
            (v45.clone(), v17.clone())
        }
    }
}
fn measures_from_4(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>) -> (Rc<UH1>, Rc<str>) {
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v2 {
        let mut v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_NoMeasures); } CASE.with(|case| case.clone()) };
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (v3.clone(), v4.clone())
    } else {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = std::rc::Rc::<str>::from((v1).split(&*v5).next().unwrap_or(""));
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = std::rc::Rc::<str>::from((v6).split(&*v7).next().unwrap_or(""));
        let mut v9: Rc<str> = std::rc::Rc::<str>::from((v6).split_once(&*v7).map(|(_, rest)| rest).unwrap_or(""));
        let mut v10: i32 = (v0).first().and_then(|header| header.iter().position(|item| &**item == &*v9)).map(|index| index as i32).unwrap_or(-1);
        let mut v11: Rc<str> = std::rc::Rc::<str>::from((v1).split_once(&*v5).map(|(_, rest)| rest).unwrap_or(""));
        let (mut v12, mut v13): (Rc<UH1>, Rc<str>) = measures_from_4(v0.clone(), v11.clone());
        let mut v14: bool = v10 < 0i32;
        if v14 {
            let mut v15: bool = v9.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v20: Rc<str> = if v15 {
                v13.clone()
            } else {
                let mut v16: bool = v13.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                if v16 {
                    v9.clone()
                } else {
                    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v18: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v5, &*v13, &*v17, &*v17].concat());
                    v18.clone()
                }
            };
            (v12.clone(), v20.clone())
        } else {
            let mut v21: bool = v8.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("mean"); } LIT.with(|lit| lit.clone()) };
            let mut v24: Rc<UH1> = if v21 {
                let mut v22: US1 = US1::US1_MeanOf;
                Rc::new(UH1::UH1_MeasureThen(v22.clone(), v9.clone(), v10, v12.clone()))
            } else {
                v12.clone()
            };
            let mut v25: bool = v8.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sd"); } LIT.with(|lit| lit.clone()) };
            let mut v28: Rc<UH1> = if v25 {
                let mut v26: US1 = US1::US1_SdOf;
                Rc::new(UH1::UH1_MeasureThen(v26.clone(), v9.clone(), v10, v12.clone()))
            } else {
                v24.clone()
            };
            let mut v29: bool = v8.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sum"); } LIT.with(|lit| lit.clone()) };
            let mut v32: Rc<UH1> = if v29 {
                let mut v30: US1 = US1::US1_SumOf;
                Rc::new(UH1::UH1_MeasureThen(v30.clone(), v9.clone(), v10, v12.clone()))
            } else {
                v28.clone()
            };
            let mut v33: bool = v8.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("min"); } LIT.with(|lit| lit.clone()) };
            let mut v36: Rc<UH1> = if v33 {
                let mut v34: US1 = US1::US1_MinOf;
                Rc::new(UH1::UH1_MeasureThen(v34.clone(), v9.clone(), v10, v12.clone()))
            } else {
                v32.clone()
            };
            let mut v37: bool = v8.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("max"); } LIT.with(|lit| lit.clone()) };
            let mut v40: Rc<UH1> = if v37 {
                let mut v38: US1 = US1::US1_MaxOf;
                Rc::new(UH1::UH1_MeasureThen(v38.clone(), v9.clone(), v10, v12.clone()))
            } else {
                v36.clone()
            };
            let mut v41: bool = v8.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("count"); } LIT.with(|lit| lit.clone()) };
            let mut v44: Rc<UH1> = if v41 {
                let mut v42: US1 = US1::US1_CountOf;
                Rc::new(UH1::UH1_MeasureThen(v42.clone(), v9.clone(), v10, v12.clone()))
            } else {
                v40.clone()
            };
            (v44.clone(), v13.clone())
        }
    }
}
fn stats_empty_5(mut v0: Rc<UH1>) -> Rc<UH2> {
    match &*v0 {
        UH1::UH1_MeasureThen(v2, v3, v4, v5) => {
            let mut v2: US1 = v2.clone();
            let mut v3: Rc<str> = v3.clone();
            let mut v4: i32 = *v4;
            let mut v5: Rc<UH1> = v5.clone();
            let mut v6: Rc<UH2> = stats_empty_5(v5.clone());
            Rc::new(UH2::UH2_MeasureStatsThen(0i32, 0i32, 0.0f64, 0.0f64, 0.0f64, 0.0f64, v6.clone()))
        }
        UH1::UH1_NoMeasures => {
            { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_NoMeasureStats); } CASE.with(|case| case.clone()) }
        }
    }
}
fn ordered_9(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: i32 = match ((v0).trim().parse::<f64>(), (v1).trim().parse::<f64>()) { (Ok(left), Ok(right)) => left.partial_cmp(&right).map(|order| order as i32).unwrap_or(0), _ => (v0).cmp(&(v1)) as i32 };
    v2
}
fn compared_8(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>) -> bool {
    match &v0 {
        US0::US0_Equal => {
            let mut v3: bool = v1.clone() == v2.clone();
            v3
        }
        US0::US0_Greater => {
            let mut v5: i32 = ordered_9(v1.clone(), v2.clone());
            let mut v6: bool = v5 > 0i32;
            v6
        }
        US0::US0_GreaterEqual => {
            let mut v7: i32 = ordered_9(v1.clone(), v2.clone());
            let mut v8: bool = v7 >= 0i32;
            v8
        }
        US0::US0_Less => {
            let mut v9: i32 = ordered_9(v1.clone(), v2.clone());
            let mut v10: bool = v9 < 0i32;
            v10
        }
        US0::US0_LessEqual => {
            let mut v11: i32 = ordered_9(v1.clone(), v2.clone());
            let mut v12: bool = v11 <= 0i32;
            v12
        }
        US0::US0_NotEqual => {
            let mut v4: bool = v1.clone() != v2.clone();
            v4
        }
    }
}
fn row_passes_7(mut v0: Rc<UH0>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: i32) -> bool {
    loop {
        match &*v0 {
            UH0::UH0_FilterThen(v3, v4, v5, v6) => {
                let mut v3: i32 = *v3;
                let mut v4: US0 = v4.clone();
                let mut v5: Rc<str> = v5.clone();
                let mut v6: Rc<UH0> = v6.clone();
                let mut v7: Rc<str> = std::rc::Rc::<str>::from((v1).get(v2 as usize).and_then(|cells| cells.get(v3 as usize)).map(|cell| &**cell).unwrap_or(""));
                let mut v8: bool = compared_8(v4.clone(), v7.clone(), v5.clone());
                if v8 {
                    (v0, v1, v2) = (v6.clone(), v1.clone(), v2);
                    continue;
                } else {
                    return false;
                }
            }
            UH0::UH0_NoFilters => {
                return true;
            }
        }
    }
}
fn row_cells_10(mut v0: Rc<UH1>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: i32) -> Rc<UH4> {
    match &*v0 {
        UH1::UH1_MeasureThen(v4, v5, v6, v7) => {
            let mut v4: US1 = v4.clone();
            let mut v5: Rc<str> = v5.clone();
            let mut v6: i32 = *v6;
            let mut v7: Rc<UH1> = v7.clone();
            let mut v8: Rc<str> = std::rc::Rc::<str>::from((v1).get(v2 as usize).and_then(|cells| cells.get(v6 as usize)).map(|cell| &**cell).unwrap_or(""));
            let mut v9: Rc<UH4> = row_cells_10(v7.clone(), v1.clone(), v2);
            Rc::new(UH4::UH4_CellThen(v8.clone(), v9.clone()))
        }
        UH1::UH1_NoMeasures => {
            { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_NoCells); } CASE.with(|case| case.clone()) }
        }
    }
}
fn stats_added_12(mut v0: Rc<UH2>, mut v1: Rc<UH4>) -> Rc<UH2> {
    match &*v0 {
        UH2::UH2_MeasureStatsThen(v2, v3, v4, v5, v6, v7, v8) => {
            let mut v2: i32 = *v2;
            let mut v3: i32 = *v3;
            let mut v4: f64 = *v4;
            let mut v5: f64 = *v5;
            let mut v6: f64 = *v6;
            let mut v7: f64 = *v7;
            let mut v8: Rc<UH2> = v8.clone();
            match &*v1 {
                UH4::UH4_CellThen(v9, v10) => {
                    let mut v9: Rc<str> = v9.clone();
                    let mut v10: Rc<UH4> = v10.clone();
                    let mut v11: bool = (v9).trim().is_empty();
                    let mut v13: i32 = if v11 {
                        v3
                    } else {
                        let mut v12: i32 = v3.wrapping_add(1i32);
                        v12
                    };
                    let mut v14: bool = (v9).trim().parse::<f64>().map(|value| value.is_finite()).unwrap_or(false);
                    if v14 {
                        let mut v15: f64 = (v9).trim().parse::<f64>().unwrap_or(0.0);
                        let mut v16: bool = v2 == 0i32;
                        let mut v18: bool = if v16 {
                            true
                        } else {
                            let mut v17: bool = v15 < v6;
                            v17
                        };
                        let mut v19: f64 = if v18 {
                            v15
                        } else {
                            v6
                        };
                        let mut v21: bool = if v16 {
                            true
                        } else {
                            let mut v20: bool = v15 > v7;
                            v20
                        };
                        let mut v22: f64 = if v21 {
                            v15
                        } else {
                            v7
                        };
                        let mut v23: i32 = v2.wrapping_add(1i32);
                        let mut v24: f64 = v4 + v15;
                        let mut v25: f64 = v15 * v15;
                        let mut v26: f64 = v5 + v25;
                        let mut v27: Rc<UH2> = stats_added_12(v8.clone(), v10.clone());
                        Rc::new(UH2::UH2_MeasureStatsThen(v23, v13, v24, v26, v19, v22, v27.clone()))
                    } else {
                        let mut v29: Rc<UH2> = stats_added_12(v8.clone(), v10.clone());
                        Rc::new(UH2::UH2_MeasureStatsThen(v2, v13, v4, v5, v6, v7, v29.clone()))
                    }
                }
                _ => {
                    v0.clone()
                }
            }
        }
        _ => {
            v0.clone()
        }
    }
}
fn group_added_11(mut v0: Rc<UH3>, mut v1: Rc<str>, mut v2: Rc<UH2>, mut v3: Rc<UH4>) -> Rc<UH3> {
    match &*v0 {
        UH3::UH3_GroupThen(v7, v8, v9, v10) => {
            let mut v7: Rc<str> = v7.clone();
            let mut v8: i32 = *v8;
            let mut v9: Rc<UH2> = v9.clone();
            let mut v10: Rc<UH3> = v10.clone();
            let mut v11: bool = v7.clone() == v1.clone();
            if v11 {
                let mut v12: i32 = v8.wrapping_add(1i32);
                let mut v13: Rc<UH2> = stats_added_12(v9.clone(), v3.clone());
                Rc::new(UH3::UH3_GroupThen(v7.clone(), v12, v13.clone(), v10.clone()))
            } else {
                let mut v15: Rc<UH3> = group_added_11(v10.clone(), v1.clone(), v2.clone(), v3.clone());
                Rc::new(UH3::UH3_GroupThen(v7.clone(), v8, v9.clone(), v15.clone()))
            }
        }
        UH3::UH3_NoGroups => {
            let mut v4: Rc<UH2> = stats_added_12(v2.clone(), v3.clone());
            let mut v5: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_NoGroups); } CASE.with(|case| case.clone()) };
            Rc::new(UH3::UH3_GroupThen(v1.clone(), 1i32, v4.clone(), v5.clone()))
        }
    }
}
fn grouped_from_6(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32, mut v3: Rc<UH1>, mut v4: Rc<UH2>, mut v5: i32, mut v6: i32, mut v7: Rc<UH3>, mut v8: i32) -> (Rc<UH3>, i32) {
    loop {
        let mut v9: bool = v6 < v5;
        if v9 {
            return (v7.clone(), v8);
        } else {
            let mut v10: bool = row_passes_7(v1.clone(), v0.clone(), v5);
            if v10 {
                let mut v11: i32 = v5.wrapping_add(1i32);
                let mut v12: bool = v2 < 0i32;
                let mut v15: Rc<str> = if v12 {
                    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
                    v13.clone()
                } else {
                    let mut v14: Rc<str> = std::rc::Rc::<str>::from((v0).get(v5 as usize).and_then(|cells| cells.get(v2 as usize)).map(|cell| &**cell).unwrap_or(""));
                    v14.clone()
                };
                let mut v16: Rc<UH4> = row_cells_10(v3.clone(), v0.clone(), v5);
                let mut v17: Rc<UH3> = group_added_11(v7.clone(), v15.clone(), v4.clone(), v16.clone());
                let mut v18: i32 = v8.wrapping_add(1i32);
                (v0, v1, v2, v3, v4, v5, v6, v7, v8) = (v0.clone(), v1.clone(), v2, v3.clone(), v4.clone(), v11, v6, v17.clone(), v18);
                continue;
            } else {
                let mut v21: i32 = v5.wrapping_add(1i32);
                (v0, v1, v2, v3, v4, v5, v6, v7, v8) = (v0.clone(), v1.clone(), v2, v3.clone(), v4.clone(), v21, v6, v7.clone(), v8);
                continue;
            }
        }
    }
}
fn measure_headers_13(mut v0: Rc<UH1>) -> Rc<str> {
    match &*v0 {
        UH1::UH1_MeasureThen(v2, v3, v4, v5) => {
            let mut v2: US1 = v2.clone();
            let mut v3: Rc<str> = v3.clone();
            let mut v4: i32 = *v4;
            let mut v5: Rc<UH1> = v5.clone();
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let mut v18: Rc<str> = match &v2 {
                US1::US1_CountOf => {
                    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("count"); } LIT.with(|lit| lit.clone()) };
                    v12.clone()
                }
                US1::US1_MaxOf => {
                    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("max"); } LIT.with(|lit| lit.clone()) };
                    v11.clone()
                }
                US1::US1_MeanOf => {
                    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("mean"); } LIT.with(|lit| lit.clone()) };
                    v7.clone()
                }
                US1::US1_MinOf => {
                    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("min"); } LIT.with(|lit| lit.clone()) };
                    v10.clone()
                }
                US1::US1_SdOf => {
                    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sd"); } LIT.with(|lit| lit.clone()) };
                    v8.clone()
                }
                US1::US1_SumOf => {
                    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sum"); } LIT.with(|lit| lit.clone()) };
                    v9.clone()
                }
            };
            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
            let mut v20: Rc<str> = measure_headers_13(v5.clone());
            let mut v21: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v18, &*v19, &*v3, &*v20].concat());
            v21.clone()
        }
        UH1::UH1_NoMeasures => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v1.clone()
        }
    }
}
fn group_inserted_15(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<UH2>, mut v3: Rc<UH3>) -> Rc<UH3> {
    match &*v3 {
        UH3::UH3_GroupThen(v6, v7, v8, v9) => {
            let mut v6: Rc<str> = v6.clone();
            let mut v7: i32 = *v7;
            let mut v8: Rc<UH2> = v8.clone();
            let mut v9: Rc<UH3> = v9.clone();
            let mut v10: bool = v1 == v7;
            let mut v13: bool = if v10 {
                let mut v11: bool = (v0.clone()) < (v6.clone());
                v11
            } else {
                let mut v12: bool = v7 < v1;
                v12
            };
            if v13 {
                Rc::new(UH3::UH3_GroupThen(v0.clone(), v1, v2.clone(), v3.clone()))
            } else {
                let mut v15: Rc<UH3> = group_inserted_15(v0.clone(), v1, v2.clone(), v9.clone());
                Rc::new(UH3::UH3_GroupThen(v6.clone(), v7, v8.clone(), v15.clone()))
            }
        }
        UH3::UH3_NoGroups => {
            let mut v4: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_NoGroups); } CASE.with(|case| case.clone()) };
            Rc::new(UH3::UH3_GroupThen(v0.clone(), v1, v2.clone(), v4.clone()))
        }
    }
}
fn groups_sorted_14(mut v0: Rc<UH3>, mut v1: Rc<UH3>) -> Rc<UH3> {
    loop {
        match &*v0 {
            UH3::UH3_GroupThen(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: i32 = *v3;
                let mut v4: Rc<UH2> = v4.clone();
                let mut v5: Rc<UH3> = v5.clone();
                let mut v6: Rc<UH3> = group_inserted_15(v2.clone(), v3, v4.clone(), v1.clone());
                (v0, v1) = (v5.clone(), v6.clone());
                continue;
            }
            UH3::UH3_NoGroups => {
                return v1.clone();
            }
        }
    }
}
fn decimal_text_19(mut v0: f64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from({ let text = format!("{:.3}", v0); let trimmed = text.trim_end_matches(char::from(48u8)).trim_end_matches(char::from(46u8)); if trimmed == "-0" { "0".to_owned() } else { trimmed.to_owned() } }.as_str());
    v1.clone()
}
fn measure_value_18(mut v0: US1, mut v1: i32, mut v2: i32, mut v3: f64, mut v4: f64, mut v5: f64, mut v6: f64) -> Rc<str> {
    match &v0 {
        US1::US1_CountOf => {
            let mut v7: f64 = v2 as f64;
            decimal_text_19(v7)
        }
        US1::US1_MaxOf => {
            let mut v13: bool = v1 == 0i32;
            if v13 {
                let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v14.clone()
            } else {
                decimal_text_19(v6)
            }
        }
        US1::US1_MeanOf => {
            let mut v18: bool = v1 == 0i32;
            if v18 {
                let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v19.clone()
            } else {
                let mut v20: f64 = v1 as f64;
                let mut v21: f64 = v3 / v20;
                decimal_text_19(v21)
            }
        }
        US1::US1_MinOf => {
            let mut v9: bool = v1 == 0i32;
            if v9 {
                let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v10.clone()
            } else {
                decimal_text_19(v5)
            }
        }
        US1::US1_SdOf => {
            let mut v24: bool = v1 < 2i32;
            if v24 {
                let mut v25: bool = v1 == 0i32;
                if v25 {
                    let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v26.clone()
                } else {
                    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                    v27.clone()
                }
            } else {
                let mut v29: f64 = v3 * v3;
                let mut v30: f64 = v1 as f64;
                let mut v31: f64 = v29 / v30;
                let mut v32: f64 = v4 - v31;
                let mut v33: i32 = v1.wrapping_sub(1i32);
                let mut v34: f64 = v33 as f64;
                let mut v35: f64 = v32 / v34;
                let mut v36: f64 = (v35).max(0.0).sqrt();
                decimal_text_19(v36)
            }
        }
        US1::US1_SumOf => {
            decimal_text_19(v3)
        }
    }
}
fn measure_cells_17(mut v0: Rc<UH1>, mut v1: Rc<UH2>) -> Rc<str> {
    match &*v0 {
        UH1::UH1_MeasureThen(v2, v3, v4, v5) => {
            let mut v2: US1 = v2.clone();
            let mut v3: Rc<str> = v3.clone();
            let mut v4: i32 = *v4;
            let mut v5: Rc<UH1> = v5.clone();
            match &*v1 {
                UH2::UH2_MeasureStatsThen(v6, v7, v8, v9, v10, v11, v12) => {
                    let mut v6: i32 = *v6;
                    let mut v7: i32 = *v7;
                    let mut v8: f64 = *v8;
                    let mut v9: f64 = *v9;
                    let mut v10: f64 = *v10;
                    let mut v11: f64 = *v11;
                    let mut v12: Rc<UH2> = v12.clone();
                    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                    let mut v14: Rc<str> = measure_value_18(v2.clone(), v6, v7, v8, v9, v10, v11);
                    let mut v15: Rc<str> = measure_cells_17(v5.clone(), v12.clone());
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v14, &*v15, &*v16, &*v16].concat());
                    v17.clone()
                }
                _ => {
                    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v18.clone()
                }
            }
        }
        _ => {
            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v20.clone()
        }
    }
}
fn groups_printed_16(mut v0: Rc<UH1>, mut v1: Rc<UH3>, mut v2: u64, mut v3: i32) -> i32 {
    loop {
        match &*v1 {
            UH3::UH3_GroupThen(v4, v5, v6, v7) => {
                let mut v4: Rc<str> = v4.clone();
                let mut v5: i32 = *v5;
                let mut v6: Rc<UH2> = v6.clone();
                let mut v7: Rc<UH3> = v7.clone();
                let mut v8: bool = (v3 as u64) < v2;
                if v8 {
                    let mut v9: Rc<str> = std::rc::Rc::<str>::from((v5).to_string().as_str());
                    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                    let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
                    let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v4));
                    let mut v13: Rc<str> = measure_cells_17(v0.clone(), v6.clone());
                    let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
                    println!("{}", &*v14);
                    let mut v15: i32 = v3.wrapping_add(1i32);
                    (v0, v1, v2, v3) = (v0.clone(), v7.clone(), v2, v15);
                    continue;
                } else {
                    return v3;
                }
            }
            UH3::UH3_NoGroups => {
                return v3;
            }
        }
    }
}
fn group_count_20(mut v0: Rc<UH3>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH3::UH3_GroupThen(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: i32 = *v3;
                let mut v4: Rc<UH2> = v4.clone();
                let mut v5: Rc<UH3> = v5.clone();
                let mut v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v5.clone(), v6);
                continue;
            }
            UH3::UH3_NoGroups => {
                return v1;
            }
        }
    }
}
fn grouped_measures_2(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: u64, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool) -> i32 {
    let mut v9: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v11: i32 = if v9 {
        -1i32
    } else {
        let mut v10: i32 = (v0).first().and_then(|header| header.iter().position(|item| &**item == &*v3)).map(|index| index as i32).unwrap_or(-1);
        v10
    };
    let (mut v12, mut v13): (Rc<UH0>, Rc<str>) = filters_from_3(v0.clone(), v1.clone());
    let (mut v14, mut v15): (Rc<UH1>, Rc<str>) = measures_from_4(v0.clone(), v6.clone());
    let mut v16: bool = v9 == false;
    let mut v18: bool = if v16 {
        let mut v17: bool = v11 < 0i32;
        v17
    } else {
        false
    };
    let mut v20: Rc<str> = if v18 {
        v3.clone()
    } else {
        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v19.clone()
    };
    let mut v21: bool = v13.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v27: Rc<str> = if v21 {
        v20.clone()
    } else {
        let mut v22: bool = v20.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v22 {
            v13.clone()
        } else {
            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v25: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v23, &*v20, &*v24, &*v24].concat());
            v25.clone()
        }
    };
    let mut v28: bool = v27.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v34: Rc<str> = if v28 {
        v15.clone()
    } else {
        let mut v29: bool = v15.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v29 {
            v27.clone()
        } else {
            let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v32: Rc<str> = std::rc::Rc::<str>::from([&*v27, &*v30, &*v15, &*v31, &*v31].concat());
            v32.clone()
        }
    };
    let mut v35: bool = v34.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v35 {
        let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown columns: "); } LIT.with(|lit| lit.clone()) };
        let mut v37: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v34));
        let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (header: "); } LIT.with(|lit| lit.clone()) };
        let mut v39: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v38));
        let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let mut v41: Rc<str> = std::rc::Rc::<str>::from((v0).first().map(|header| header.iter().map(|item| &**item).collect::<Vec<_>>().join(&*v40)).unwrap_or_default().as_str());
        let mut v42: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v41));
        let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
        let mut v44: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v43));
        let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: table "); } LIT.with(|lit| lit.clone()) };
        let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v47: Rc<str> = std::rc::Rc::<str>::from([&*v45, &*v44, &*v46, &*v46, &*v46].concat());
        eprintln!("{}", &*v47);
        2i32
    } else {
        let mut v48: Rc<UH2> = stats_empty_5(v14.clone());
        let mut v49: i32 = 1i32;
        let mut v50: i32 = ((v0).len().max(1) - 1) as i32;
        let mut v51: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_NoGroups); } CASE.with(|case| case.clone()) };
        let mut v52: i32 = 0i32;
        let (mut v53, mut v54): (Rc<UH3>, i32) = grouped_from_6(v0.clone(), v12.clone(), v11, v14.clone(), v48.clone(), v49, v50, v51.clone(), v52);
        let mut v56: Rc<str> = if v9 {
            let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
            v55.clone()
        } else {
            v3.clone()
        };
        let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("count\t"); } LIT.with(|lit| lit.clone()) };
        let mut v58: Rc<str> = Rc::<str>::from(format!("{}{}", v57, v56));
        let mut v59: Rc<str> = measure_headers_13(v14.clone());
        let mut v60: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v59));
        println!("{}", &*v60);
        let mut v61: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_NoGroups); } CASE.with(|case| case.clone()) };
        let mut v62: Rc<UH3> = groups_sorted_14(v53.clone(), v61.clone());
        let mut v63: i32 = 0i32;
        let mut v64: i32 = groups_printed_16(v14.clone(), v62.clone(), v5, v63);
        let mut v65: Rc<str> = std::rc::Rc::<str>::from((v54).to_string().as_str());
        let mut v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" matched="); } LIT.with(|lit| lit.clone()) };
        let mut v67: Rc<str> = Rc::<str>::from(format!("{}{}", v66, v65));
        let mut v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" groups="); } LIT.with(|lit| lit.clone()) };
        let mut v69: Rc<str> = Rc::<str>::from(format!("{}{}", v67, v68));
        let mut v70: i32 = 0i32;
        let mut v71: i32 = group_count_20(v53.clone(), v70);
        let mut v72: Rc<str> = std::rc::Rc::<str>::from((v71).to_string().as_str());
        let mut v73: Rc<str> = Rc::<str>::from(format!("{}{}", v69, v72));
        let mut v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" shown="); } LIT.with(|lit| lit.clone()) };
        let mut v75: Rc<str> = Rc::<str>::from(format!("{}{}", v73, v74));
        if v8 {
            ()
        } else {
            let mut v76: i32 = ((v0).len().max(1) - 1) as i32;
            let mut v77: Rc<str> = std::rc::Rc::<str>::from((v76).to_string().as_str());
            let mut v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie table query ok total="); } LIT.with(|lit| lit.clone()) };
            let mut v79: Rc<str> = Rc::<str>::from(format!("{}{}", v78, v77));
            let mut v80: Rc<str> = Rc::<str>::from(format!("{}{}", v79, v75));
            let mut v81: Rc<str> = std::rc::Rc::<str>::from((v64).to_string().as_str());
            let mut v82: Rc<str> = Rc::<str>::from(format!("{}{}", v80, v81));
            println!("{}", &*v82);
            ()
        };
        0i32
    }
}
fn measured_query_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: u64, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool) -> i32 {
    let (mut v9, mut v10, mut v11): (bool, std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, Rc<str>) = { let path = &*v0; match std::fs::read_to_string(path) { Ok(text) => { let separator = if path.ends_with(".csv") { char::from(44u8) } else { char::from(9u8) }; (true, std::rc::Rc::new(text.lines().map(|line| line.split(separator).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>()), { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) } Err(error) => (false, std::rc::Rc::new(Vec::new()), std::rc::Rc::<str>::from(format!("{path}: {error}").as_str())) } };
    let mut v14: US2 = if v9 {
        US2::US2_TableLoaded(v10.clone())
    } else {
        US2::US2_TableUnreadable(v11.clone())
    };
    match &v14 {
        US2::US2_TableLoaded(v19) => {
            let mut v19: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = v19.clone();
            grouped_measures_2(v19.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5, v6.clone(), v7.clone(), v8)
        }
        US2::US2_TableUnreadable(v15) => {
            let mut v15: Rc<str> = v15.clone();
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: table "); } LIT.with(|lit| lit.clone()) };
            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v18: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v15, &*v17, &*v17, &*v17].concat());
            eprintln!("{}", &*v18);
            2i32
        }
    }
}
fn no_request_21() -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>, bool) {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0.clone(), v0.clone(), v0.clone(), v0.clone(), 20u64, v0.clone(), v0.clone(), false)
}
fn measured_entry_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: bool) -> i32 {
    let (mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>, bool) = no_request_21();
    measured_query_1(v0.clone(), v1.clone(), v7.clone(), v2.clone(), v9.clone(), v4, v3.clone(), v12.clone(), v5)
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, bool) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, bool) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: bool| -> i32 {
        measured_entry_0(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4, v5)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_table_measured(v0: &str, v1: &str, v2: &str, v3: &str, v4: u64, v5: bool) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), v4, v5)
}
