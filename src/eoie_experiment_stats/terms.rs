#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_WelchTwoSided,
    US0_OneSampleT,
    US0_BinomialAtMost,
    US0_WilsonBand,
    US0_MedianBand,
    US0_BinomialBand,
}
#[derive(Clone)]
enum US1 {
    US1_LowEdge,
    US1_HighEdge,
}
#[derive(Clone)]
enum US2 {
    US2_BandHolds,
    US2_BandFails,
}
#[derive(Clone)]
enum US3 {
    US3_TestKnown(US0),
    US3_TestUnknown,
}
fn median_of_1(mut v0: Rc<str>) -> f64 {
    let mut v1: f64 = { let mut numbers = (v0).split(char::from(44u8)).filter_map(|cell| cell.trim().parse::<f64>().ok()).filter(|value| value.is_finite()).collect::<Vec<_>>(); numbers.sort_by(|a, b| a.total_cmp(b)); let count = numbers.len(); if count == 0 { f64::NAN } else if count % 2 == 1 { numbers[count / 2] } else { (numbers[count / 2 - 1] + numbers[count / 2]) / 2.0 } };
    v1
}
fn number_shown_2(mut v0: f64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(format!("{:.6}", v0).trim_end_matches(char::from(48u8)).trim_end_matches(char::from(46u8)));
    v1.clone()
}
fn median_terms_0(mut v0: Rc<str>, mut v1: f64, mut v2: f64) -> Rc<str> {
    let mut v3: f64 = median_of_1(v0.clone());
    let mut v4: bool = v3 >= v1;
    let mut v6: bool = if v4 {
        let mut v5: bool = v3 <= v2;
        v5
    } else {
        false
    };
    let mut v7: Rc<str> = number_shown_2(v1);
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" band=["); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v7));
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
    let mut v12: Rc<str> = number_shown_2(v2);
    let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]"); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
    let mut v16: Rc<str> = number_shown_2(v3);
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median="); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v16));
    let mut v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v15));
    let mut v22: Rc<str> = if v6 {
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" median_inside=yes"); } LIT.with(|lit| lit.clone()) };
        v20.clone()
    } else {
        let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" median_inside=no"); } LIT.with(|lit| lit.clone()) };
        v21.clone()
    };
    let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v22));
    v23.clone()
}
fn closure0() -> Rc<dyn Fn(Rc<str>, f64, f64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, f64, f64) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: f64, mut v2: f64| -> Rc<str> {
        median_terms_0(v0.clone(), v1, v2)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn ln_gamma_7(mut v0: f64) -> f64 {
    let mut v1: f64 = v0 + 1.0f64;
    let mut v2: f64 = 76.18009172947146f64 / v1;
    let mut v3: f64 = 1.000000000190015f64 + v2;
    let mut v4: f64 = v0 + 2.0f64;
    let mut v5: f64 = 86.50532032941678f64 / v4;
    let mut v6: f64 = v3 - v5;
    let mut v7: f64 = v0 + 3.0f64;
    let mut v8: f64 = 24.01409824083091f64 / v7;
    let mut v9: f64 = v6 + v8;
    let mut v10: f64 = v0 + 4.0f64;
    let mut v11: f64 = 1.231739572450155f64 / v10;
    let mut v12: f64 = v9 - v11;
    let mut v13: f64 = v0 + 5.0f64;
    let mut v14: f64 = 0.001208650973866179f64 / v13;
    let mut v15: f64 = v12 + v14;
    let mut v16: f64 = v0 + 6.0f64;
    let mut v17: f64 = 5.395239384953E-06f64 / v16;
    let mut v18: f64 = v15 - v17;
    let mut v19: f64 = v0 + 0.5f64;
    let mut v20: f64 = v0 + 5.5f64;
    let mut v21: f64 = (v20).ln();
    let mut v22: f64 = v19 * v21;
    let mut v23: f64 = v22 - v20;
    let mut v24: f64 = 2.5066282746310007f64 * v18;
    let mut v25: f64 = v24 / v0;
    let mut v26: f64 = (v25).ln();
    let mut v27: f64 = v23 + v26;
    v27
}
fn binomial_terms_6(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: f64, mut v4: f64) -> f64 {
    loop {
        let mut v5: bool = v2 > v1;
        if v5 {
            return v4;
        } else {
            let mut v6: f64 = v0 as f64;
            let mut v7: f64 = v6 + 1.0f64;
            let mut v8: f64 = ln_gamma_7(v7);
            let mut v9: f64 = v2 as f64;
            let mut v10: f64 = v9 + 1.0f64;
            let mut v11: f64 = ln_gamma_7(v10);
            let mut v12: f64 = v8 - v11;
            let mut v13: i32 = v0.wrapping_sub(v2);
            let mut v14: f64 = v13 as f64;
            let mut v15: f64 = v14 + 1.0f64;
            let mut v16: f64 = ln_gamma_7(v15);
            let mut v17: f64 = v12 - v16;
            let mut v18: i32 = v2.wrapping_add(1i32);
            let mut v19: f64 = v2 as f64;
            let mut v20: f64 = (v3).ln();
            let mut v21: f64 = v19 * v20;
            let mut v22: f64 = v17 + v21;
            let mut v23: f64 = v13 as f64;
            let mut v24: f64 = 1.0f64 - v3;
            let mut v25: f64 = (v24).ln();
            let mut v26: f64 = v23 * v25;
            let mut v27: f64 = v22 + v26;
            let mut v28: f64 = (v27).exp();
            let mut v29: f64 = v4 + v28;
            (v0, v1, v2, v3, v4) = (v0, v1, v18, v3, v29);
            continue;
        }
    }
}
fn binomial_at_most_5(mut v0: i32, mut v1: i32, mut v2: f64) -> f64 {
    let mut v3: bool = v0 >= v1;
    if v3 {
        1.0f64
    } else {
        let mut v4: bool = v0 < 0i32;
        if v4 {
            0.0f64
        } else {
            let mut v5: bool = v2 <= 0.0f64;
            if v5 {
                1.0f64
            } else {
                let mut v6: bool = v2 >= 1.0f64;
                if v6 {
                    0.0f64
                } else {
                    let mut v7: i32 = 0i32;
                    let mut v8: f64 = 0.0f64;
                    binomial_terms_6(v1, v0, v7, v2, v8)
                }
            }
        }
    }
}
fn i32_text_8(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string().as_str());
    v1.clone()
}
fn edge_count_10(mut v0: US1, mut v1: i32, mut v2: i32, mut v3: f64, mut v4: f64) -> i32 {
    loop {
        let mut v5: f64 = binomial_at_most_5(v1, v2, v3);
        let mut v12: bool = match &v0 {
            US1::US1_HighEdge => {
                let mut v8: f64 = v4 / 2.0f64;
                let mut v9: f64 = 1.0f64 - v8;
                let mut v10: bool = v5 >= v9;
                v10
            }
            US1::US1_LowEdge => {
                let mut v6: f64 = v4 / 2.0f64;
                let mut v7: bool = v5 > v6;
                v7
            }
        };
        let mut v13: bool = v1 >= v2;
        let mut v14: bool = v13 || v12;
        if v14 {
            return v1;
        } else {
            let mut v15: i32 = v1.wrapping_add(1i32);
            (v0, v1, v2, v3, v4) = (v0.clone(), v15, v2, v3, v4);
            continue;
        }
    }
}
fn binomial_band_9(mut v0: i32, mut v1: f64, mut v2: f64) -> (i32, i32) {
    let mut v3: f64 = 1.0f64 - v2;
    let mut v4: US1 = US1::US1_LowEdge;
    let mut v5: i32 = 0i32;
    let mut v6: i32 = edge_count_10(v4.clone(), v5, v0, v1, v3);
    let mut v7: US1 = US1::US1_HighEdge;
    let mut v8: i32 = 0i32;
    let mut v9: i32 = edge_count_10(v7.clone(), v8, v0, v1, v3);
    (v6, v9)
}
fn sample_of_11(mut v0: i32, mut v1: f64, mut v2: f64, mut v3: f64, mut v4: i32) -> (f64, f64, f64) {
    let mut v5: bool = v1 > 0.0f64;
    let mut v7: f64 = if v5 {
        let mut v6: f64 = v2 / v1;
        v6
    } else {
        0.0f64
    };
    let mut v8: bool = v1 > 1.0f64;
    let mut v14: f64 = if v8 {
        let mut v9: f64 = v2 * v2;
        let mut v10: f64 = v9 / v1;
        let mut v11: f64 = v3 - v10;
        let mut v12: f64 = v1 - 1.0f64;
        let mut v13: f64 = v11 / v12;
        v13
    } else {
        0.0f64
    };
    (v1, v7, v14)
}
fn fraction_step_19(mut v0: f64, mut v1: f64, mut v2: f64, mut v3: i32, mut v4: f64, mut v5: f64, mut v6: f64) -> f64 {
    loop {
        let mut v7: bool = v3 > 300i32;
        if v7 {
            return v6;
        } else {
            let mut v8: f64 = v3 as f64;
            let mut v9: f64 = v1 - v8;
            let mut v10: f64 = v8 * v9;
            let mut v11: f64 = v10 * v2;
            let mut v12: f64 = v0 - 1.0f64;
            let mut v13: f64 = 2.0f64 * v8;
            let mut v14: f64 = v12 + v13;
            let mut v15: f64 = v0 + v13;
            let mut v16: f64 = v14 * v15;
            let mut v17: f64 = v11 / v16;
            let mut v18: f64 = v17 * v5;
            let mut v19: f64 = 1.0f64 + v18;
            let mut v20: f64 = (v19).abs();
            let mut v21: bool = v20 < 1E-30f64;
            let mut v22: f64 = if v21 {
                1E-30f64
            } else {
                v19
            };
            let mut v23: f64 = 1.0f64 / v22;
            let mut v24: f64 = v17 / v4;
            let mut v25: f64 = 1.0f64 + v24;
            let mut v26: f64 = (v25).abs();
            let mut v27: bool = v26 < 1E-30f64;
            let mut v28: f64 = if v27 {
                1E-30f64
            } else {
                v25
            };
            let mut v29: f64 = v6 * v23;
            let mut v30: f64 = v29 * v28;
            let mut v31: f64 = v0 + v8;
            let mut v32: f64 = v0 + v1;
            let mut v33: f64 = v32 + v8;
            let mut v34: f64 = v31 * v33;
            let mut v35: f64 = v34 * v2;
            let mut v36: f64 = v0 + 1.0f64;
            let mut v37: f64 = v36 + v13;
            let mut v38: f64 = v15 * v37;
            let mut v39: f64 = v35 / v38;
            let mut v40: f64 = -(v39);
            let mut v41: f64 = v40 * v23;
            let mut v42: f64 = 1.0f64 + v41;
            let mut v43: f64 = (v42).abs();
            let mut v44: bool = v43 < 1E-30f64;
            let mut v45: f64 = if v44 {
                1E-30f64
            } else {
                v42
            };
            let mut v46: f64 = 1.0f64 / v45;
            let mut v47: f64 = v40 / v28;
            let mut v48: f64 = 1.0f64 + v47;
            let mut v49: f64 = (v48).abs();
            let mut v50: bool = v49 < 1E-30f64;
            let mut v51: f64 = if v50 {
                1E-30f64
            } else {
                v48
            };
            let mut v52: f64 = v46 * v51;
            let mut v53: f64 = v52 - 1.0f64;
            let mut v54: f64 = (v53).abs();
            let mut v55: bool = v54 < 3E-14f64;
            if v55 {
                let mut v56: f64 = v30 * v52;
                return v56;
            } else {
                let mut v57: i32 = v3.wrapping_add(1i32);
                let mut v58: f64 = v30 * v52;
                (v0, v1, v2, v3, v4, v5, v6) = (v0, v1, v2, v57, v51, v46, v58);
                continue;
            }
        }
    }
}
fn beta_fraction_18(mut v0: f64, mut v1: f64, mut v2: f64) -> f64 {
    let mut v3: f64 = v0 + v1;
    let mut v4: f64 = v3 * v2;
    let mut v5: f64 = v0 + 1.0f64;
    let mut v6: f64 = v4 / v5;
    let mut v7: f64 = 1.0f64 - v6;
    let mut v8: f64 = (v7).abs();
    let mut v9: bool = v8 < 1E-30f64;
    let mut v10: f64 = if v9 {
        1E-30f64
    } else {
        v7
    };
    let mut v11: f64 = 1.0f64 / v10;
    let mut v12: i32 = 1i32;
    let mut v13: f64 = 1.0f64;
    fraction_step_19(v0, v1, v2, v12, v13, v11, v11)
}
fn regularized_beta_17(mut v0: f64, mut v1: f64, mut v2: f64) -> f64 {
    let mut v3: bool = v2 <= 0.0f64;
    if v3 {
        0.0f64
    } else {
        let mut v4: bool = v2 >= 1.0f64;
        if v4 {
            1.0f64
        } else {
            let mut v5: f64 = v0 + v1;
            let mut v6: f64 = ln_gamma_7(v5);
            let mut v7: f64 = ln_gamma_7(v0);
            let mut v8: f64 = v6 - v7;
            let mut v9: f64 = ln_gamma_7(v1);
            let mut v10: f64 = v8 - v9;
            let mut v11: f64 = (v2).ln();
            let mut v12: f64 = v0 * v11;
            let mut v13: f64 = v10 + v12;
            let mut v14: f64 = 1.0f64 - v2;
            let mut v15: f64 = (v14).ln();
            let mut v16: f64 = v1 * v15;
            let mut v17: f64 = v13 + v16;
            let mut v18: f64 = (v17).exp();
            let mut v19: f64 = v0 + 1.0f64;
            let mut v20: f64 = v5 + 2.0f64;
            let mut v21: f64 = v19 / v20;
            let mut v22: bool = v2 < v21;
            if v22 {
                let mut v23: f64 = beta_fraction_18(v0, v1, v2);
                let mut v24: f64 = v18 * v23;
                let mut v25: f64 = v24 / v0;
                v25
            } else {
                let mut v26: f64 = beta_fraction_18(v1, v0, v14);
                let mut v27: f64 = v18 * v26;
                let mut v28: f64 = v27 / v1;
                let mut v29: f64 = 1.0f64 - v28;
                v29
            }
        }
    }
}
fn two_sided_p_16(mut v0: f64, mut v1: f64) -> f64 {
    let mut v2: f64 = v1 / 2.0f64;
    let mut v3: f64 = 0.5f64;
    let mut v4: f64 = v0 * v0;
    let mut v5: f64 = v1 + v4;
    let mut v6: f64 = v1 / v5;
    regularized_beta_17(v2, v3, v6)
}
fn student_cdf_15(mut v0: f64, mut v1: f64) -> f64 {
    let mut v2: f64 = two_sided_p_16(v0, v1);
    let mut v3: f64 = v2 / 2.0f64;
    let mut v4: bool = v0 >= 0.0f64;
    if v4 {
        let mut v5: f64 = 1.0f64 - v3;
        v5
    } else {
        v3
    }
}
fn quantile_search_14(mut v0: f64, mut v1: f64, mut v2: f64, mut v3: f64, mut v4: i32) -> f64 {
    loop {
        let mut v5: f64 = v2 + v3;
        let mut v6: f64 = v5 / 2.0f64;
        let mut v7: bool = v4 == 0i32;
        if v7 {
            return v6;
        } else {
            let mut v8: f64 = student_cdf_15(v6, v1);
            let mut v9: bool = v8 < v0;
            if v9 {
                let mut v10: i32 = v4.wrapping_sub(1i32);
                (v0, v1, v2, v3, v4) = (v0, v1, v6, v3, v10);
                continue;
            } else {
                let mut v12: i32 = v4.wrapping_sub(1i32);
                (v0, v1, v2, v3, v4) = (v0, v1, v2, v6, v12);
                continue;
            }
        }
    }
}
fn student_quantile_13(mut v0: f64, mut v1: f64) -> f64 {
    let mut v2: bool = v0 >= 0.5f64;
    if v2 {
        let mut v3: f64 = 0.0f64;
        let mut v4: f64 = 10000.0f64;
        let mut v5: i32 = 120i32;
        quantile_search_14(v0, v1, v3, v4, v5)
    } else {
        let mut v7: f64 = 1.0f64 - v0;
        let mut v8: f64 = 0.0f64;
        let mut v9: f64 = 10000.0f64;
        let mut v10: i32 = 120i32;
        let mut v11: f64 = quantile_search_14(v7, v1, v8, v9, v10);
        let mut v12: f64 = -(v11);
        v12
    }
}
fn one_sample_12(mut v0: f64, mut v1: f64, mut v2: f64, mut v3: f64, mut v4: f64) -> (f64, f64, f64, f64, f64, f64) {
    let mut v5: f64 = v2 / v0;
    let mut v6: f64 = (v5).max(0.0).sqrt();
    let mut v7: f64 = v1 - v3;
    let mut v8: f64 = v7 / v6;
    let mut v9: f64 = v0 - 1.0f64;
    let mut v10: f64 = 1.0f64 - v4;
    let mut v11: f64 = v10 / 2.0f64;
    let mut v12: f64 = 1.0f64 - v11;
    let mut v13: f64 = student_quantile_13(v12, v9);
    let mut v14: f64 = v13 * v6;
    let mut v15: f64 = two_sided_p_16(v8, v9);
    let mut v16: f64 = v1 - v14;
    let mut v17: f64 = v1 + v14;
    (v7, v8, v9, v15, v16, v17)
}
fn t_terms_20(mut v0: f64, mut v1: f64, mut v2: f64, mut v3: f64, mut v4: f64, mut v5: f64, mut v6: f64) -> Rc<str> {
    let mut v7: Rc<str> = number_shown_2(v1);
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("t="); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v7));
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" df="); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
    let mut v12: Rc<str> = number_shown_2(v2);
    let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" p="); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
    let mut v16: bool = v3 < v6;
    let mut v19: Rc<str> = if v16 {
        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" reject_h0=yes"); } LIT.with(|lit| lit.clone()) };
        v17.clone()
    } else {
        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" reject_h0=no"); } LIT.with(|lit| lit.clone()) };
        v18.clone()
    };
    let mut v20: Rc<str> = number_shown_2(v3);
    let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v20));
    let mut v22: Rc<str> = number_shown_2(v4);
    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ci=["); } LIT.with(|lit| lit.clone()) };
    let mut v24: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v22));
    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v26: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v25));
    let mut v27: Rc<str> = number_shown_2(v5);
    let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v27));
    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]"); } LIT.with(|lit| lit.clone()) };
    let mut v30: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v29));
    let mut v31: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v30));
    let mut v32: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v19));
    v32.clone()
}
fn welch_21(mut v0: f64, mut v1: f64, mut v2: f64, mut v3: f64, mut v4: f64, mut v5: f64, mut v6: f64) -> (f64, f64, f64, f64, f64, f64) {
    let mut v7: f64 = v2 / v0;
    let mut v8: f64 = v5 / v3;
    let mut v9: f64 = v7 + v8;
    let mut v10: f64 = (v9).max(0.0).sqrt();
    let mut v11: f64 = v1 - v4;
    let mut v12: f64 = v11 / v10;
    let mut v13: f64 = v9 * v9;
    let mut v14: f64 = v7 * v7;
    let mut v15: f64 = v0 - 1.0f64;
    let mut v16: f64 = v14 / v15;
    let mut v17: f64 = v8 * v8;
    let mut v18: f64 = v3 - 1.0f64;
    let mut v19: f64 = v17 / v18;
    let mut v20: f64 = v16 + v19;
    let mut v21: f64 = v13 / v20;
    let mut v22: f64 = 1.0f64 - v6;
    let mut v23: f64 = v22 / 2.0f64;
    let mut v24: f64 = 1.0f64 - v23;
    let mut v25: f64 = student_quantile_13(v24, v21);
    let mut v26: f64 = v25 * v10;
    let mut v27: f64 = two_sided_p_16(v12, v21);
    let mut v28: f64 = v11 - v26;
    let mut v29: f64 = v11 + v26;
    (v11, v12, v21, v27, v28, v29)
}
fn erfc_of_25(mut v0: f64) -> f64 {
    let mut v1: f64 = (v0).abs();
    let mut v2: f64 = 0.5f64 * v1;
    let mut v3: f64 = 1.0f64 + v2;
    let mut v4: f64 = 1.0f64 / v3;
    let mut v5: f64 = v4 * 0.17087277f64;
    let mut v6: f64 = -0.82215223f64 + v5;
    let mut v7: f64 = v4 * v6;
    let mut v8: f64 = 1.48851587f64 + v7;
    let mut v9: f64 = v4 * v8;
    let mut v10: f64 = -1.13520398f64 + v9;
    let mut v11: f64 = v4 * v10;
    let mut v12: f64 = 0.27886807f64 + v11;
    let mut v13: f64 = v4 * v12;
    let mut v14: f64 = -0.18628806f64 + v13;
    let mut v15: f64 = v4 * v14;
    let mut v16: f64 = 0.09678418f64 + v15;
    let mut v17: f64 = v4 * v16;
    let mut v18: f64 = 0.37409196f64 + v17;
    let mut v19: f64 = v4 * v18;
    let mut v20: f64 = 1.00002368f64 + v19;
    let mut v21: f64 = v4 * v20;
    let mut v22: f64 = -1.26551223f64 + v21;
    let mut v23: f64 = v1 * v1;
    let mut v24: f64 = -(v23);
    let mut v25: f64 = v24 + v22;
    let mut v26: f64 = (v25).exp();
    let mut v27: f64 = v4 * v26;
    let mut v28: bool = v0 >= 0.0f64;
    if v28 {
        v27
    } else {
        let mut v29: f64 = 2.0f64 - v27;
        v29
    }
}
fn normal_cdf_24(mut v0: f64) -> f64 {
    let mut v1: f64 = v0 / 1.4142135623730951f64;
    let mut v2: f64 = erfc_of_25(v1);
    let mut v3: f64 = 0.5f64 * v2;
    let mut v4: f64 = 1.0f64 - v3;
    v4
}
fn normal_search_23(mut v0: f64, mut v1: f64, mut v2: f64, mut v3: i32) -> f64 {
    loop {
        let mut v4: f64 = v1 + v2;
        let mut v5: f64 = v4 / 2.0f64;
        let mut v6: bool = v3 == 0i32;
        if v6 {
            return v5;
        } else {
            let mut v7: f64 = normal_cdf_24(v5);
            let mut v8: bool = v7 < v0;
            if v8 {
                let mut v9: i32 = v3.wrapping_sub(1i32);
                (v0, v1, v2, v3) = (v0, v5, v2, v9);
                continue;
            } else {
                let mut v11: i32 = v3.wrapping_sub(1i32);
                (v0, v1, v2, v3) = (v0, v1, v5, v11);
                continue;
            }
        }
    }
}
fn normal_quantile_22(mut v0: f64) -> f64 {
    let mut v1: f64 = -40.0f64;
    let mut v2: f64 = 40.0f64;
    let mut v3: i32 = 100i32;
    normal_search_23(v0, v1, v2, v3)
}
fn wilson_26(mut v0: i32, mut v1: i32, mut v2: f64) -> (f64, f64) {
    let mut v3: f64 = v1 as f64;
    let mut v4: f64 = v0 as f64;
    let mut v5: f64 = v4 / v3;
    let mut v6: f64 = v2 * v2;
    let mut v7: f64 = v6 / v3;
    let mut v8: f64 = 1.0f64 + v7;
    let mut v9: f64 = 2.0f64 * v3;
    let mut v10: f64 = v6 / v9;
    let mut v11: f64 = v5 + v10;
    let mut v12: f64 = v11 / v8;
    let mut v13: f64 = 1.0f64 - v5;
    let mut v14: f64 = v5 * v13;
    let mut v15: f64 = v14 / v3;
    let mut v16: f64 = 4.0f64 * v3;
    let mut v17: f64 = v16 * v3;
    let mut v18: f64 = v6 / v17;
    let mut v19: f64 = v15 + v18;
    let mut v20: f64 = (v19).max(0.0).sqrt();
    let mut v21: f64 = v2 * v20;
    let mut v22: f64 = v21 / v8;
    let mut v23: f64 = v12 - v22;
    let mut v24: f64 = v12 + v22;
    (v23, v24)
}
fn test_terms_4(mut v0: US0, mut v1: i32, mut v2: f64, mut v3: f64, mut v4: f64, mut v5: i32, mut v6: i32, mut v7: f64, mut v8: f64, mut v9: f64, mut v10: i32, mut v11: f64, mut v12: f64) -> Rc<str> {
    let mut v13: f64 = 1.0f64 - v12;
    match &v0 {
        US0::US0_BinomialAtMost => {
            let mut v63: f64 = binomial_at_most_5(v5, v1, v11);
            let mut v64: Rc<str> = i32_text_8(v5);
            let mut v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("successes="); } LIT.with(|lit| lit.clone()) };
            let mut v66: Rc<str> = Rc::<str>::from(format!("{}{}", v65, v64));
            let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let mut v68: Rc<str> = Rc::<str>::from(format!("{}{}", v66, v67));
            let mut v69: Rc<str> = i32_text_8(v1);
            let mut v70: Rc<str> = Rc::<str>::from(format!("{}{}", v68, v69));
            let mut v71: Rc<str> = number_shown_2(v63);
            let mut v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" p_at_most="); } LIT.with(|lit| lit.clone()) };
            let mut v73: Rc<str> = Rc::<str>::from(format!("{}{}", v72, v71));
            let mut v74: bool = v63 < v13;
            let mut v77: Rc<str> = if v74 {
                let mut v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" reject_h0=yes"); } LIT.with(|lit| lit.clone()) };
                v75.clone()
            } else {
                let mut v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" reject_h0=no"); } LIT.with(|lit| lit.clone()) };
                v76.clone()
            };
            let mut v78: Rc<str> = Rc::<str>::from(format!("{}{}", v73, v77));
            let mut v79: Rc<str> = Rc::<str>::from(format!("{}{}", v70, v78));
            v79.clone()
        }
        US0::US0_BinomialBand => {
            let (mut v108, mut v109): (i32, i32) = binomial_band_9(v1, v11, v12);
            let mut v110: Rc<str> = i32_text_8(v5);
            let mut v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("successes="); } LIT.with(|lit| lit.clone()) };
            let mut v112: Rc<str> = Rc::<str>::from(format!("{}{}", v111, v110));
            let mut v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let mut v114: Rc<str> = Rc::<str>::from(format!("{}{}", v112, v113));
            let mut v115: Rc<str> = i32_text_8(v1);
            let mut v116: Rc<str> = Rc::<str>::from(format!("{}{}", v114, v115));
            let mut v117: Rc<str> = i32_text_8(v108);
            let mut v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" band=["); } LIT.with(|lit| lit.clone()) };
            let mut v119: Rc<str> = Rc::<str>::from(format!("{}{}", v118, v117));
            let mut v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let mut v121: Rc<str> = Rc::<str>::from(format!("{}{}", v119, v120));
            let mut v122: Rc<str> = i32_text_8(v109);
            let mut v123: Rc<str> = Rc::<str>::from(format!("{}{}", v121, v122));
            let mut v124: bool = v5 >= v108;
            let mut v126: bool = if v124 {
                let mut v125: bool = v5 <= v109;
                v125
            } else {
                false
            };
            let mut v129: US2 = if v126 {
                US2::US2_BandHolds
            } else {
                US2::US2_BandFails
            };
            let mut v133: Rc<str> = match &v129 {
                US2::US2_BandFails => {
                    let mut v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fails"); } LIT.with(|lit| lit.clone()) };
                    v131.clone()
                }
                US2::US2_BandHolds => {
                    let mut v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("holds"); } LIT.with(|lit| lit.clone()) };
                    v130.clone()
                }
            };
            let mut v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("] verdict="); } LIT.with(|lit| lit.clone()) };
            let mut v135: Rc<str> = Rc::<str>::from(format!("{}{}", v134, v133));
            let mut v136: Rc<str> = Rc::<str>::from(format!("{}{}", v123, v135));
            let mut v137: Rc<str> = Rc::<str>::from(format!("{}{}", v116, v136));
            v137.clone()
        }
        US0::US0_MedianBand => {
            let mut v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v107.clone()
        }
        US0::US0_OneSampleT => {
            let (mut v40, mut v41, mut v42): (f64, f64, f64) = sample_of_11(v1, v2, v3, v4, v5);
            let mut v43: Rc<str> = number_shown_2(v41);
            let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("mean="); } LIT.with(|lit| lit.clone()) };
            let mut v45: Rc<str> = Rc::<str>::from(format!("{}{}", v44, v43));
            let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" prediction="); } LIT.with(|lit| lit.clone()) };
            let mut v47: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v46));
            let mut v48: Rc<str> = number_shown_2(v11);
            let mut v49: Rc<str> = Rc::<str>::from(format!("{}{}", v47, v48));
            let (mut v50, mut v51, mut v52): (f64, f64, f64) = sample_of_11(v1, v2, v3, v4, v5);
            let (mut v53, mut v54, mut v55, mut v56, mut v57, mut v58): (f64, f64, f64, f64, f64, f64) = one_sample_12(v50, v51, v52, v11, v12);
            let mut v59: Rc<str> = t_terms_20(v53, v54, v55, v56, v57, v58, v13);
            let mut v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
            let mut v61: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v59));
            let mut v62: Rc<str> = Rc::<str>::from(format!("{}{}", v49, v61));
            v62.clone()
        }
        US0::US0_WelchTwoSided => {
            let (mut v14, mut v15, mut v16): (f64, f64, f64) = sample_of_11(v1, v2, v3, v4, v5);
            let (mut v17, mut v18, mut v19): (f64, f64, f64) = sample_of_11(v6, v7, v8, v9, v10);
            let mut v20: f64 = v15 - v18;
            let mut v21: Rc<str> = number_shown_2(v20);
            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("difference="); } LIT.with(|lit| lit.clone()) };
            let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v21));
            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
            let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v24));
            let (mut v26, mut v27, mut v28): (f64, f64, f64) = sample_of_11(v1, v2, v3, v4, v5);
            let (mut v29, mut v30, mut v31): (f64, f64, f64) = sample_of_11(v6, v7, v8, v9, v10);
            let (mut v32, mut v33, mut v34, mut v35, mut v36, mut v37): (f64, f64, f64, f64, f64, f64) = welch_21(v26, v27, v28, v29, v30, v31, v12);
            let mut v38: Rc<str> = t_terms_20(v32, v33, v34, v35, v36, v37, v13);
            let mut v39: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v38));
            v39.clone()
        }
        US0::US0_WilsonBand => {
            let mut v80: f64 = v13 / 2.0f64;
            let mut v81: f64 = 1.0f64 - v80;
            let mut v82: f64 = normal_quantile_22(v81);
            let (mut v83, mut v84): (f64, f64) = wilson_26(v5, v1, v82);
            let mut v85: bool = v11 >= v83;
            let mut v87: bool = if v85 {
                let mut v86: bool = v11 <= v84;
                v86
            } else {
                false
            };
            let mut v88: Rc<str> = i32_text_8(v5);
            let mut v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("successes="); } LIT.with(|lit| lit.clone()) };
            let mut v90: Rc<str> = Rc::<str>::from(format!("{}{}", v89, v88));
            let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let mut v92: Rc<str> = Rc::<str>::from(format!("{}{}", v90, v91));
            let mut v93: Rc<str> = i32_text_8(v1);
            let mut v94: Rc<str> = Rc::<str>::from(format!("{}{}", v92, v93));
            let mut v95: Rc<str> = number_shown_2(v83);
            let mut v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" band=["); } LIT.with(|lit| lit.clone()) };
            let mut v97: Rc<str> = Rc::<str>::from(format!("{}{}", v96, v95));
            let mut v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let mut v99: Rc<str> = Rc::<str>::from(format!("{}{}", v97, v98));
            let mut v100: Rc<str> = number_shown_2(v84);
            let mut v101: Rc<str> = Rc::<str>::from(format!("{}{}", v99, v100));
            let mut v104: Rc<str> = if v87 {
                let mut v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("] prediction_inside=yes"); } LIT.with(|lit| lit.clone()) };
                v102.clone()
            } else {
                let mut v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("] prediction_inside=no"); } LIT.with(|lit| lit.clone()) };
                v103.clone()
            };
            let mut v105: Rc<str> = Rc::<str>::from(format!("{}{}", v101, v104));
            let mut v106: Rc<str> = Rc::<str>::from(format!("{}{}", v94, v105));
            v106.clone()
        }
    }
}
fn test_of_27(mut v0: Rc<str>) -> US3 {
    let mut v1: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
    if v1 {
        let mut v2: US0 = US0::US0_WelchTwoSided;
        US3::US3_TestKnown(v2.clone())
    } else {
        let mut v4: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
        if v4 {
            let mut v5: US0 = US0::US0_OneSampleT;
            US3::US3_TestKnown(v5.clone())
        } else {
            let mut v7: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
            if v7 {
                let mut v8: US0 = US0::US0_BinomialAtMost;
                US3::US3_TestKnown(v8.clone())
            } else {
                let mut v10: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                if v10 {
                    let mut v11: US0 = US0::US0_WilsonBand;
                    US3::US3_TestKnown(v11.clone())
                } else {
                    let mut v13: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                    if v13 {
                        let mut v14: US0 = US0::US0_MedianBand;
                        US3::US3_TestKnown(v14.clone())
                    } else {
                        let mut v16: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                        if v16 {
                            let mut v17: US0 = US0::US0_BinomialBand;
                            US3::US3_TestKnown(v17.clone())
                        } else {
                            US3::US3_TestUnknown
                        }
                    }
                }
            }
        }
    }
}
fn terms_text_3(mut v0: Rc<str>, mut v1: i32, mut v2: f64, mut v3: f64, mut v4: f64, mut v5: i32, mut v6: i32, mut v7: f64, mut v8: f64, mut v9: f64, mut v10: i32, mut v11: f64, mut v12: f64) -> Rc<str> {
    let mut v13: US3 = test_of_27(v0.clone());
    match &v13 {
        US3::US3_TestKnown(v14) => {
            let mut v14: US0 = v14.clone();
            test_terms_4(v14.clone(), v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12)
        }
        US3::US3_TestUnknown => {
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v16.clone()
        }
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>, i32, f64, f64, f64, i32, i32, f64, f64, f64, i32, f64, f64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, i32, f64, f64, f64, i32, i32, f64, f64, f64, i32, f64, f64) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: i32, mut v2: f64, mut v3: f64, mut v4: f64, mut v5: i32, mut v6: i32, mut v7: f64, mut v8: f64, mut v9: f64, mut v10: i32, mut v11: f64, mut v12: f64| -> Rc<str> {
        terms_text_3(v0.clone(), v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_median_terms(v0: &str, v1: f64, v2: f64) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), v1, v2)
}
pub fn eoie_experiment_terms(v0: &str, v1: i32, v2: f64, v3: f64, v4: f64, v5: i32, v6: i32, v7: f64, v8: f64, v9: f64, v10: i32, v11: f64, v12: f64) -> Rc<str> {
    closure1()(Rc::<str>::from(v0), v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12)
}
