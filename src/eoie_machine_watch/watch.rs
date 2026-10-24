#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_ClaudeSession(u64),
    US0_EoieJob(Rc<str>),
    US0_CellsVm(u64),
    US0_Orphan,
    US0_NotATool,
}
#[derive(Clone)]
enum US1 {
    US1_Talking,
    US1_Buffered(f64),
    US1_QuietInLiveBlock,
    US1_WaitingByDesign(Rc<str>),
    US1_CellsBusy(f64),
    US1_Silent(u64),
}
#[derive(Clone)]
enum US2 {
    US2_WindowFine,
    US2_WindowStale(Rc<str>),
}
fn option_value_2(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    loop {
        let mut v2: i32 = v1.wrapping_add(1i32);
        let mut v3: i32 = std::env::args().count() as i32;
        let mut v4: bool = v2 >= v3;
        if v4 {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v5.clone();
        } else {
            let mut v6: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v1 as usize).unwrap_or_default());
            let mut v7: bool = v6.clone() == v0.clone();
            if v7 {
                let mut v8: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v2 as usize).unwrap_or_default());
                return v8.clone();
            } else {
                (v0, v1) = (v0.clone(), v2);
                continue;
            }
        }
    }
}
fn process_rows_sampled_3(mut v0: u64) -> std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>> {
    let mut v1: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>> = { let kind = sysinfo::ProcessRefreshKind::nothing().with_cpu().with_memory().with_cmd(sysinfo::UpdateKind::Always); let mut system = sysinfo::System::new_with_specifics(sysinfo::RefreshKind::nothing().with_processes(kind)); std::thread::sleep(std::time::Duration::from_millis(v0)); system.refresh_processes_specifics(sysinfo::ProcessesToUpdate::All, true, kind); std::rc::Rc::new(system.processes().iter().map(|(pid, process)| { let command = process.cmd().iter().map(|part| part.to_string_lossy().into_owned()).collect::<Vec<_>>().join(" "); (pid.as_u32() as u64, process.parent().map(|parent| parent.as_u32() as u64).unwrap_or(0), std::rc::Rc::<str>::from(process.name().to_string_lossy().trim_end_matches(".exe")), std::rc::Rc::<str>::from(command.as_str()), process.cpu_usage() as f64 / 100.0, process.run_time() / 60, process.accumulated_cpu_time() / 60000, process.memory() / 1048576) }).collect::<Vec<_>>()) };
    v1.clone()
}
fn owner_walk_7(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v1: i32, mut v2: i32) -> US0 {
    loop {
        let mut v3: bool = v1 < 0i32;
        let mut v5: bool = if v3 {
            true
        } else {
            let mut v4: bool = v2 >= 40i32;
            v4
        };
        if v5 {
            return US0::US0_Orphan;
        } else {
            let mut v7: Rc<str> = v0[v1 as usize].2.clone();
            let mut v8: Rc<str> = v0[v1 as usize].3.clone();
            let mut v9: Rc<str> = std::rc::Rc::<str>::from((v8).split_whitespace().skip_while(|word| *word != "supervise").nth(1).filter(|word| word.starts_with("job-")).unwrap_or(""));
            let mut v10: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("claude"); } LIT.with(|lit| lit.clone()) };
            if v10 {
                let mut v11: u64 = v0[v1 as usize].0;
                return US0::US0_ClaudeSession(v11);
            } else {
                let mut v13: bool = v9.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                if v13 {
                    return US0::US0_EoieJob(v9.clone());
                } else {
                    let mut v15: u64 = v0[v1 as usize].1;
                    let mut v16: i32 = v0.iter().position(|row| row.0 == v15).map(|at| at as i32).unwrap_or(-1);
                    let mut v17: bool = v16 < 0i32;
                    if v17 {
                        let mut v18: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("erl"); } LIT.with(|lit| lit.clone()) };
                        let mut v21: bool = if v18 {
                            let mut v19: Rc<str> = v0[v1 as usize].3.clone();
                            let mut v20: bool = { let lower = (v19).to_lowercase(); lower.contains("spiral.notebook") || lower.contains("spiral-cells") || lower.contains("cells-build") };
                            v20
                        } else {
                            false
                        };
                        if v21 {
                            let mut v22: u64 = v0[v1 as usize].0;
                            return US0::US0_CellsVm(v22);
                        } else {
                            return US0::US0_Orphan;
                        }
                    } else {
                        let mut v26: i32 = v2.wrapping_add(1i32);
                        (v0, v1, v2) = (v0.clone(), v16, v26);
                        continue;
                    }
                }
            }
        }
    }
}
fn owner_of_6(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v1: i32) -> US0 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dotnet|rustc|cargo|erl|beam.smp|pwsh|powershell|node|bun|gcc|cc1|cc1plus|ld|zig|lean|java|swiftc|swift-frontend|python|kotlinc|gleam|wasmtime|link|clang|mix|escript|make|git"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = v0[v1 as usize].2.clone();
    let mut v4: bool = v2.split("|").any(|item| item == &*v3);
    if v4 {
        let mut v5: i32 = 0i32;
        owner_walk_7(v0.clone(), v1, v5)
    } else {
        US0::US0_NotATool
    }
}
fn busy_at_5(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v1: i32) -> (u64, Rc<str>, f64, u64, u64, US0) {
    let mut v2: u64 = v0[v1 as usize].0;
    let mut v3: Rc<str> = v0[v1 as usize].2.clone();
    let mut v4: f64 = v0[v1 as usize].4;
    let mut v5: u64 = v0[v1 as usize].5;
    let mut v6: u64 = v0[v1 as usize].6;
    let mut v7: US0 = owner_of_6(v0.clone(), v1);
    (v2, v3.clone(), v4, v5, v6, v7.clone())
}
fn busy_lines_4(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v1: i32, mut v2: Rc<str>, mut v3: f64) -> (Rc<str>, f64) {
    loop {
        let mut v4: i32 = v0.len() as i32;
        let mut v5: bool = v1 >= v4;
        if v5 {
            return (v2.clone(), v3);
        } else {
            let mut v6: f64 = v0[v1 as usize].4;
            let mut v7: bool = v6 < 0.1f64;
            if v7 {
                let mut v8: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2, v3) = (v0.clone(), v8, v2.clone(), v3);
                continue;
            } else {
                let (mut v11, mut v12, mut v13, mut v14, mut v15, mut v16): (u64, Rc<str>, f64, u64, u64, US0) = busy_at_5(v0.clone(), v1);
                let mut v17: Rc<str> = std::rc::Rc::<str>::from((v11).to_string());
                let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let mut v19: Rc<str> = std::rc::Rc::<str>::from(format!("{:.2}", v6));
                let mut v20: Rc<str> = std::rc::Rc::<str>::from((v14).to_string());
                let mut v21: Rc<str> = std::rc::Rc::<str>::from((v15).to_string());
                let mut v42: Rc<str> = match &v16 {
                    US0::US0_CellsVm(v31) => {
                        let mut v31: u64 = *v31;
                        let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cells-vm:"); } LIT.with(|lit| lit.clone()) };
                        let mut v33: Rc<str> = std::rc::Rc::<str>::from((v31).to_string());
                        let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v35: Rc<str> = std::rc::Rc::<str>::from([&*v32, &*v33, &*v34, &*v34, &*v34].concat());
                        v35.clone()
                    }
                    US0::US0_ClaudeSession(v22) => {
                        let mut v22: u64 = *v22;
                        let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("claude:"); } LIT.with(|lit| lit.clone()) };
                        let mut v24: Rc<str> = std::rc::Rc::<str>::from((v22).to_string());
                        let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v26: Rc<str> = std::rc::Rc::<str>::from([&*v23, &*v24, &*v25, &*v25, &*v25].concat());
                        v26.clone()
                    }
                    US0::US0_EoieJob(v27) => {
                        let mut v27: Rc<str> = v27.clone();
                        let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie:"); } LIT.with(|lit| lit.clone()) };
                        let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v30: Rc<str> = std::rc::Rc::<str>::from([&*v28, &*v27, &*v29, &*v29, &*v29].concat());
                        v30.clone()
                    }
                    US0::US0_NotATool => {
                        let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                        v37.clone()
                    }
                    US0::US0_Orphan => {
                        let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("orphan"); } LIT.with(|lit| lit.clone()) };
                        v36.clone()
                    }
                };
                let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v44: Rc<str> = std::rc::Rc::<str>::from([&*v21, &*v18, &*v42, &*v43, &*v43].concat());
                let mut v45: Rc<str> = std::rc::Rc::<str>::from([&*v19, &*v18, &*v20, &*v18, &*v44].concat());
                let mut v46: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v18, &*v12, &*v18, &*v45].concat());
                let mut v47: i32 = v1.wrapping_add(1i32);
                let mut v48: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v51: Rc<str> = if v48 {
                    v46.clone()
                } else {
                    let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let mut v50: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v49, &*v46, &*v43, &*v43].concat());
                    v50.clone()
                };
                let mut v52: f64 = v3 + v6;
                (v0, v1, v2, v3) = (v0.clone(), v47, v51.clone(), v52);
                continue;
            }
        }
    }
}
fn rogue_9(mut v0: u64, mut v1: u64, mut v2: f64, mut v3: u64, mut v4: u64, mut v5: Rc<str>, mut v6: f64, mut v7: u64, mut v8: u64, mut v9: US0) -> bool {
    let mut v10: bool = v7 >= 180u64;
    let mut v12: bool = if v10 {
        let mut v11: bool = (v8 as f64) >= 0.8 * (v7 as f64);
        v11
    } else {
        false
    };
    let mut v13: bool = match &v9 {
        US0::US0_NotATool => {
            false
        }
        _ => {
            true
        }
    };
    let mut v15: bool = if v13 {
        let mut v14: bool = v6 >= v2;
        v14
    } else {
        false
    };
    if v15 {
        let mut v16: bool = match &v9 {
            US0::US0_Orphan => {
                true
            }
            _ => {
                false
            }
        };
        let mut v18: bool = if v16 {
            let mut v17: bool = v7 >= v1;
            v17
        } else {
            false
        };
        let mut v19: bool = v18 || v12;
        v19
    } else {
        false
    }
}
fn rogue_lines_8(mut v0: u64, mut v1: u64, mut v2: f64, mut v3: u64, mut v4: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v5: i32, mut v6: Rc<str>) -> Rc<str> {
    loop {
        let mut v7: i32 = v4.len() as i32;
        let mut v8: bool = v5 >= v7;
        if v8 {
            return v6.clone();
        } else {
            let (mut v9, mut v10, mut v11, mut v12, mut v13, mut v14): (u64, Rc<str>, f64, u64, u64, US0) = busy_at_5(v4.clone(), v5);
            let mut v15: bool = rogue_9(v0, v1, v2, v3, v9, v10.clone(), v11, v12, v13, v14.clone());
            if v15 {
                let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  pid "); } LIT.with(|lit| lit.clone()) };
                let mut v17: Rc<str> = std::rc::Rc::<str>::from((v9).to_string());
                let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let mut v19: Rc<str> = std::rc::Rc::<str>::from(format!("{:.2}", v11));
                let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cores, age "); } LIT.with(|lit| lit.clone()) };
                let mut v21: Rc<str> = std::rc::Rc::<str>::from((v12).to_string());
                let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" min, owner "); } LIT.with(|lit| lit.clone()) };
                let mut v43: Rc<str> = match &v14 {
                    US0::US0_CellsVm(v32) => {
                        let mut v32: u64 = *v32;
                        let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cells-vm:"); } LIT.with(|lit| lit.clone()) };
                        let mut v34: Rc<str> = std::rc::Rc::<str>::from((v32).to_string());
                        let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v36: Rc<str> = std::rc::Rc::<str>::from([&*v33, &*v34, &*v35, &*v35, &*v35].concat());
                        v36.clone()
                    }
                    US0::US0_ClaudeSession(v23) => {
                        let mut v23: u64 = *v23;
                        let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("claude:"); } LIT.with(|lit| lit.clone()) };
                        let mut v25: Rc<str> = std::rc::Rc::<str>::from((v23).to_string());
                        let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v27: Rc<str> = std::rc::Rc::<str>::from([&*v24, &*v25, &*v26, &*v26, &*v26].concat());
                        v27.clone()
                    }
                    US0::US0_EoieJob(v28) => {
                        let mut v28: Rc<str> = v28.clone();
                        let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie:"); } LIT.with(|lit| lit.clone()) };
                        let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v31: Rc<str> = std::rc::Rc::<str>::from([&*v29, &*v28, &*v30, &*v30, &*v30].concat());
                        v31.clone()
                    }
                    US0::US0_NotATool => {
                        let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                        v38.clone()
                    }
                    US0::US0_Orphan => {
                        let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("orphan"); } LIT.with(|lit| lit.clone()) };
                        v37.clone()
                    }
                };
                let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v45: Rc<str> = std::rc::Rc::<str>::from([&*v22, &*v43, &*v44, &*v44, &*v44].concat());
                let mut v46: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v19, &*v20, &*v21, &*v45].concat());
                let mut v47: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v17, &*v18, &*v10, &*v46].concat());
                let mut v48: i32 = v5.wrapping_add(1i32);
                let mut v49: bool = v6.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v52: Rc<str> = if v49 {
                    v47.clone()
                } else {
                    let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let mut v51: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v50, &*v47, &*v44, &*v44].concat());
                    v51.clone()
                };
                (v0, v1, v2, v3, v4, v5, v6) = (v0, v1, v2, v3, v4.clone(), v48, v52.clone());
                continue;
            } else {
                let mut v54: i32 = v5.wrapping_add(1i32);
                (v0, v1, v2, v3, v4, v5, v6) = (v0, v1, v2, v3, v4.clone(), v54, v6.clone());
                continue;
            }
        }
    }
}
fn hog_lines_10(mut v0: u64, mut v1: u64, mut v2: f64, mut v3: u64, mut v4: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v5: i32, mut v6: Rc<str>) -> Rc<str> {
    loop {
        let mut v7: i32 = v4.len() as i32;
        let mut v8: bool = v5 >= v7;
        if v8 {
            return v6.clone();
        } else {
            let mut v9: u64 = v4[v5 as usize].7;
            let mut v10: bool = v9 >= v3;
            if v10 {
                let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  pid "); } LIT.with(|lit| lit.clone()) };
                let mut v12: u64 = v4[v5 as usize].0;
                let mut v13: Rc<str> = std::rc::Rc::<str>::from((v12).to_string());
                let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let mut v15: Rc<str> = v4[v5 as usize].2.clone();
                let mut v16: u64 = v4[v5 as usize].7;
                let mut v17: Rc<str> = std::rc::Rc::<str>::from((v16).to_string());
                let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" MB, owner "); } LIT.with(|lit| lit.clone()) };
                let mut v19: US0 = owner_of_6(v4.clone(), v5);
                let mut v40: Rc<str> = match &v19 {
                    US0::US0_CellsVm(v29) => {
                        let mut v29: u64 = *v29;
                        let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cells-vm:"); } LIT.with(|lit| lit.clone()) };
                        let mut v31: Rc<str> = std::rc::Rc::<str>::from((v29).to_string());
                        let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v33: Rc<str> = std::rc::Rc::<str>::from([&*v30, &*v31, &*v32, &*v32, &*v32].concat());
                        v33.clone()
                    }
                    US0::US0_ClaudeSession(v20) => {
                        let mut v20: u64 = *v20;
                        let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("claude:"); } LIT.with(|lit| lit.clone()) };
                        let mut v22: Rc<str> = std::rc::Rc::<str>::from((v20).to_string());
                        let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v24: Rc<str> = std::rc::Rc::<str>::from([&*v21, &*v22, &*v23, &*v23, &*v23].concat());
                        v24.clone()
                    }
                    US0::US0_EoieJob(v25) => {
                        let mut v25: Rc<str> = v25.clone();
                        let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie:"); } LIT.with(|lit| lit.clone()) };
                        let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v28: Rc<str> = std::rc::Rc::<str>::from([&*v26, &*v25, &*v27, &*v27, &*v27].concat());
                        v28.clone()
                    }
                    US0::US0_NotATool => {
                        let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                        v35.clone()
                    }
                    US0::US0_Orphan => {
                        let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("orphan"); } LIT.with(|lit| lit.clone()) };
                        v34.clone()
                    }
                };
                let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v42: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v17, &*v18, &*v40, &*v41].concat());
                let mut v43: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v13, &*v14, &*v15, &*v42].concat());
                let mut v44: i32 = v5.wrapping_add(1i32);
                let mut v45: bool = v6.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v48: Rc<str> = if v45 {
                    v43.clone()
                } else {
                    let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let mut v47: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v46, &*v43, &*v41, &*v41].concat());
                    v47.clone()
                };
                (v0, v1, v2, v3, v4, v5, v6) = (v0, v1, v2, v3, v4.clone(), v44, v48.clone());
                continue;
            } else {
                let mut v50: i32 = v5.wrapping_add(1i32);
                (v0, v1, v2, v3, v4, v5, v6) = (v0, v1, v2, v3, v4.clone(), v50, v6.clone());
                continue;
            }
        }
    }
}
fn windows_root_12() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("EOIE_WINDOWS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("eoie").join("windows"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("eoie").join("windows"))).unwrap_or_else(|| std::path::PathBuf::from(".eoie-windows")).to_string_lossy().as_ref());
    v0.clone()
}
fn live_block_held_13(mut v0: Rc<str>) -> bool {
    let mut v1: bool = (&*v0).lines().any(|row| { let mut fields = row.split(char::from(9u8)); fields.nth(1) == Some("live-gui") && fields.next() == Some("held") });
    v1
}
fn jobs_root_14() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("EOIE_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("eoie").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("eoie").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".eoie-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn cells_cores_15(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v1: i32, mut v2: f64) -> f64 {
    loop {
        let mut v3: i32 = v0.len() as i32;
        let mut v4: bool = v1 >= v3;
        if v4 {
            return v2;
        } else {
            let mut v5: f64 = v0[v1 as usize].4;
            let mut v6: bool = v5 >= 0.1f64;
            let mut v31: bool = if v6 {
                let mut v7: US0 = owner_of_6(v0.clone(), v1);
                let mut v28: Rc<str> = match &v7 {
                    US0::US0_CellsVm(v17) => {
                        let mut v17: u64 = *v17;
                        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cells-vm:"); } LIT.with(|lit| lit.clone()) };
                        let mut v19: Rc<str> = std::rc::Rc::<str>::from((v17).to_string());
                        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v21: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v19, &*v20, &*v20, &*v20].concat());
                        v21.clone()
                    }
                    US0::US0_ClaudeSession(v8) => {
                        let mut v8: u64 = *v8;
                        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("claude:"); } LIT.with(|lit| lit.clone()) };
                        let mut v10: Rc<str> = std::rc::Rc::<str>::from((v8).to_string());
                        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v10, &*v11, &*v11, &*v11].concat());
                        v12.clone()
                    }
                    US0::US0_EoieJob(v13) => {
                        let mut v13: Rc<str> = v13.clone();
                        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie:"); } LIT.with(|lit| lit.clone()) };
                        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v16: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v13, &*v15, &*v15, &*v15].concat());
                        v16.clone()
                    }
                    US0::US0_NotATool => {
                        let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                        v23.clone()
                    }
                    US0::US0_Orphan => {
                        let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("orphan"); } LIT.with(|lit| lit.clone()) };
                        v22.clone()
                    }
                };
                let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cells-vm:"); } LIT.with(|lit| lit.clone()) };
                let mut v30: bool = v28.starts_with(&*v29);
                v30
            } else {
                false
            };
            let mut v32: i32 = v1.wrapping_add(1i32);
            let mut v35: f64 = if v31 {
                let mut v33: f64 = v0[v1 as usize].4;
                let mut v34: f64 = v2 + v33;
                v34
            } else {
                v2
            };
            (v0, v1, v2) = (v0.clone(), v32, v35);
            continue;
        }
    }
}
fn joined_18(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2.clone()
}
fn job_directory_19(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = jobs_root_14();
    joined_18(v1.clone(), v0.clone())
}
fn job_file_17(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = job_directory_19(v0.clone());
    joined_18(v2.clone(), v1.clone())
}
fn file_bytes_20(mut v0: Rc<str>) -> u64 {
    let mut v1: u64 = std::fs::metadata(&*v0).map(|metadata| metadata.len()).unwrap_or(0);
    v1
}
fn owner_cores_21(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v1: Rc<str>, mut v2: i32, mut v3: f64) -> f64 {
    loop {
        let mut v4: i32 = v0.len() as i32;
        let mut v5: bool = v2 >= v4;
        if v5 {
            return v3;
        } else {
            let mut v6: f64 = v0[v2 as usize].4;
            let mut v7: bool = v6 >= 0.1f64;
            let mut v31: bool = if v7 {
                let mut v8: US0 = owner_of_6(v0.clone(), v2);
                let mut v29: Rc<str> = match &v8 {
                    US0::US0_CellsVm(v18) => {
                        let mut v18: u64 = *v18;
                        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cells-vm:"); } LIT.with(|lit| lit.clone()) };
                        let mut v20: Rc<str> = std::rc::Rc::<str>::from((v18).to_string());
                        let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v22: Rc<str> = std::rc::Rc::<str>::from([&*v19, &*v20, &*v21, &*v21, &*v21].concat());
                        v22.clone()
                    }
                    US0::US0_ClaudeSession(v9) => {
                        let mut v9: u64 = *v9;
                        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("claude:"); } LIT.with(|lit| lit.clone()) };
                        let mut v11: Rc<str> = std::rc::Rc::<str>::from((v9).to_string());
                        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v11, &*v12, &*v12, &*v12].concat());
                        v13.clone()
                    }
                    US0::US0_EoieJob(v14) => {
                        let mut v14: Rc<str> = v14.clone();
                        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie:"); } LIT.with(|lit| lit.clone()) };
                        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v14, &*v16, &*v16, &*v16].concat());
                        v17.clone()
                    }
                    US0::US0_NotATool => {
                        let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                        v24.clone()
                    }
                    US0::US0_Orphan => {
                        let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("orphan"); } LIT.with(|lit| lit.clone()) };
                        v23.clone()
                    }
                };
                let mut v30: bool = v29.clone() == v1.clone();
                v30
            } else {
                false
            };
            if v31 {
                let mut v32: i32 = v2.wrapping_add(1i32);
                let mut v33: f64 = v0[v2 as usize].4;
                let mut v34: f64 = v3 + v33;
                (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v32, v34);
                continue;
            } else {
                let mut v36: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v36, v3);
                continue;
            }
        }
    }
}
fn last_lines_22(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let mut v2: Rc<str> = { let text = std::fs::read_to_string(&*v0).unwrap_or_default(); let lines = text.lines().collect::<Vec<_>>(); let from = lines.len().saturating_sub(v1 as usize); std::rc::Rc::<str>::from(lines[from..].join("
")) };
    v2.clone()
}
fn quiet_verdict_of_23(mut v0: u64, mut v1: u64, mut v2: f64, mut v3: u64, mut v4: u64, mut v5: f64, mut v6: bool, mut v7: Rc<str>, mut v8: f64) -> US1 {
    let mut v9: bool = v4 < v0;
    if v9 {
        US1::US1_Talking
    } else {
        let mut v11: bool = v5 >= 0.1f64;
        if v11 {
            US1::US1_Buffered(v5)
        } else {
            if v6 {
                US1::US1_QuietInLiveBlock
            } else {
                let mut v14: Rc<str> = std::rc::Rc::<str>::from((&*v7).to_lowercase());
                let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("waiting for"); } LIT.with(|lit| lit.clone()) };
                let mut v16: bool = v14.contains(&*v15);
                if v16 {
                    US1::US1_WaitingByDesign(v7.clone())
                } else {
                    let mut v18: bool = v8 >= 0.5f64;
                    if v18 {
                        US1::US1_CellsBusy(v8)
                    } else {
                        US1::US1_Silent(v4)
                    }
                }
            }
        }
    }
}
fn text_or_empty_24(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1.clone()
}
fn verdict_text_25(mut v0: US1) -> Rc<str> {
    match &v0 {
        US1::US1_Buffered(v2) => {
            let mut v2: f64 = *v2;
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  (quiet output, tree busy "); } LIT.with(|lit| lit.clone()) };
            let mut v4: Rc<str> = std::rc::Rc::<str>::from(format!("{:.2}", v2));
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cores: buffered)"); } LIT.with(|lit| lit.clone()) };
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v7: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v4, &*v5, &*v6, &*v6].concat());
            v7.clone()
        }
        US1::US1_CellsBusy(v14) => {
            let mut v14: f64 = *v14;
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  (quiet, tree idle, detached Cells VMs use "); } LIT.with(|lit| lit.clone()) };
            let mut v16: Rc<str> = std::rc::Rc::<str>::from(format!("{:.2}", v14));
            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cores: likely this job's compiler work)"); } LIT.with(|lit| lit.clone()) };
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v19: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v16, &*v17, &*v18, &*v18].concat());
            v19.clone()
        }
        US1::US1_QuietInLiveBlock => {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  (quiet during a live block: waiting for a window is expected)"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US1::US1_Silent(v20) => {
            let mut v20: u64 = *v20;
            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  <- SILENT "); } LIT.with(|lit| lit.clone()) };
            let mut v22: Rc<str> = std::rc::Rc::<str>::from((v20).to_string());
            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" min, tree idle: check the job's own progress files"); } LIT.with(|lit| lit.clone()) };
            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v25: Rc<str> = std::rc::Rc::<str>::from([&*v21, &*v22, &*v23, &*v24, &*v24].concat());
            v25.clone()
        }
        US1::US1_Talking => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v1.clone()
        }
        US1::US1_WaitingByDesign(v9) => {
            let mut v9: Rc<str> = v9.clone();
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  (waiting by design, last line: "); } LIT.with(|lit| lit.clone()) };
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v9, &*v11, &*v12, &*v12].concat());
            v13.clone()
        }
    }
}
fn jobs_watched_16(mut v0: u64, mut v1: u64, mut v2: f64, mut v3: u64, mut v4: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v5: Rc<str>, mut v6: i32, mut v7: Rc<str>, mut v8: bool, mut v9: f64, mut v10: u64, mut v11: Rc<str>, mut v12: Rc<str>) -> (Rc<str>, Rc<str>) {
    let mut v13: i32 = (&*v5).lines().filter(|line| !line.is_empty()).count() as i32;
    let mut v14: bool = v6 >= v13;
    let (mut v73, mut v74): (Rc<str>, Rc<str>) = if v14 {
        (v11.clone(), v12.clone())
    } else {
        let mut v15: Rc<str> = std::rc::Rc::<str>::from((&*v5).lines().filter(|line| !line.is_empty()).nth(v6 as usize).unwrap_or(""));
        let mut v16: Rc<str> = eoie_job_control::eoie_job_state_word(&*v15);
        let mut v17: bool = v16.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("running"); } LIT.with(|lit| lit.clone()) };
        let (mut v71, mut v72): (Rc<str>, Rc<str>) = if v17 {
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stdout.log"); } LIT.with(|lit| lit.clone()) };
            let mut v19: Rc<str> = job_file_17(v15.clone(), v18.clone());
            let mut v20: u64 = file_bytes_20(v19.clone());
            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stderr.log"); } LIT.with(|lit| lit.clone()) };
            let mut v22: Rc<str> = job_file_17(v15.clone(), v21.clone());
            let mut v23: u64 = file_bytes_20(v22.clone());
            let mut v24: u64 = v20.wrapping_add(v23);
            let mut v25: u64 = (&*v7).lines().find_map(|line| { let mut fields = line.split(char::from(9u8)); if fields.next() == Some(&*v15) && fields.next().and_then(|value| value.parse::<u64>().ok()) == Some(v24) { fields.next().and_then(|value| value.parse::<u64>().ok()) } else { None } }).unwrap_or(v10);
            let mut v26: u64 = v10.wrapping_sub(v25);
            let mut v27: u64 = v26.wrapping_div(60000u64);
            let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie:"); } LIT.with(|lit| lit.clone()) };
            let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v30: Rc<str> = std::rc::Rc::<str>::from([&*v28, &*v15, &*v29, &*v29, &*v29].concat());
            let mut v31: i32 = 0i32;
            let mut v32: f64 = 0.0f64;
            let mut v33: f64 = owner_cores_21(v4.clone(), v30.clone(), v31, v32);
            let mut v34: Rc<str> = job_file_17(v15.clone(), v18.clone());
            let mut v35: u64 = 1u64;
            let mut v36: Rc<str> = last_lines_22(v34.clone(), v35);
            let mut v37: US1 = quiet_verdict_of_23(v0, v1, v2, v3, v27, v33, v8, v36.clone(), v9);
            let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("label"); } LIT.with(|lit| lit.clone()) };
            let mut v39: Rc<str> = job_file_17(v15.clone(), v38.clone());
            let mut v40: Rc<str> = text_or_empty_24(v39.clone());
            let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
            let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" output "); } LIT.with(|lit| lit.clone()) };
            let mut v43: Rc<str> = std::rc::Rc::<str>::from((v24).to_string());
            let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes, unchanged "); } LIT.with(|lit| lit.clone()) };
            let mut v45: Rc<str> = std::rc::Rc::<str>::from((v27).to_string());
            let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" min, tree "); } LIT.with(|lit| lit.clone()) };
            let mut v47: Rc<str> = std::rc::Rc::<str>::from(format!("{:.2}", v33));
            let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cores"); } LIT.with(|lit| lit.clone()) };
            let mut v49: Rc<str> = verdict_text_25(v37.clone());
            let mut v50: Rc<str> = std::rc::Rc::<str>::from([&*v48, &*v49, &*v29, &*v29, &*v29].concat());
            let mut v51: Rc<str> = std::rc::Rc::<str>::from([&*v44, &*v45, &*v46, &*v47, &*v50].concat());
            let mut v52: Rc<str> = std::rc::Rc::<str>::from([&*v41, &*v40, &*v42, &*v43, &*v51].concat());
            let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let mut v54: Rc<str> = std::rc::Rc::<str>::from((v24).to_string());
            let mut v55: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
            let mut v56: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v53, &*v54, &*v53, &*v55].concat());
            let mut v57: i32 = v6.wrapping_add(1i32);
            let mut v58: bool = v11.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v61: Rc<str> = if v58 {
                v52.clone()
            } else {
                let mut v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let mut v60: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v59, &*v52, &*v29, &*v29].concat());
                v60.clone()
            };
            let mut v62: bool = v12.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v65: Rc<str> = if v62 {
                v56.clone()
            } else {
                let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let mut v64: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v63, &*v56, &*v29, &*v29].concat());
                v64.clone()
            };
            let (mut v66, mut v67): (Rc<str>, Rc<str>) = jobs_watched_16(v0, v1, v2, v3, v4.clone(), v5.clone(), v57, v7.clone(), v8, v9, v10, v61.clone(), v65.clone());
            (v66.clone(), v67.clone())
        } else {
            let mut v68: i32 = v6.wrapping_add(1i32);
            let (mut v69, mut v70): (Rc<str>, Rc<str>) = jobs_watched_16(v0, v1, v2, v3, v4.clone(), v5.clone(), v68, v7.clone(), v8, v9, v10, v11.clone(), v12.clone());
            (v69.clone(), v70.clone())
        };
        (v71.clone(), v72.clone())
    };
    (v73.clone(), v74.clone())
}
fn gate_summary_26() -> Rc<str> {
    let mut v0: Rc<str> = { let path = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")).map(std::path::PathBuf::from).unwrap_or_default().join(".claude").join("autonomous").join("gate.log"); let text = std::fs::read_to_string(&path).unwrap_or_default(); let mut sessions: std::collections::BTreeMap<String, (u64, u64)> = std::collections::BTreeMap::new(); let lines = text.lines().collect::<Vec<_>>(); for line in lines.iter().rev().take(4000) { let fields = line.split_whitespace().collect::<Vec<_>>(); if fields.len() < 4 { continue; } let entry = sessions.entry(fields[2].to_owned()).or_insert((0, 0)); if fields[3] == "block" { entry.0 += 1; } else if line.contains("idle") && fields[3] == "pass" { entry.1 += 1; } } std::rc::Rc::<str>::from(sessions.into_iter().map(|(session, (blocks, idle))| format!("  {session}: blocks {blocks}, idle passes {idle}{}", if idle > 0 { "  <- idled" } else { "" })).collect::<Vec<_>>().join("
").as_str()) };
    v0.clone()
}
fn window_health_of_28(mut v0: bool, mut v1: Rc<str>) -> US2 {
    let mut v3: bool = if v0 {
        let mut v2: bool = v1.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v2
    } else {
        false
    };
    let mut v5: bool = if v3 {
        let mut v4: bool = v1.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("running"); } LIT.with(|lit| lit.clone()) };
        v4
    } else {
        false
    };
    let mut v7: bool = if v5 {
        let mut v6: bool = v1.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("queued"); } LIT.with(|lit| lit.clone()) };
        v6
    } else {
        false
    };
    if v7 {
        US2::US2_WindowStale(v1.clone())
    } else {
        US2::US2_WindowFine
    }
}
fn windows_watched_27(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let mut v3: i32 = (&*v0).lines().filter(|line| !line.is_empty()).count() as i32;
        let mut v4: bool = v1 >= v3;
        if v4 {
            return v2.clone();
        } else {
            let mut v5: Rc<str> = std::rc::Rc::<str>::from((&*v0).lines().filter(|line| !line.is_empty()).nth(v1 as usize).unwrap_or(""));
            let mut v6: Rc<str> = std::rc::Rc::<str>::from((&*v5).split(char::from(9u8)).nth(3i32 as usize).unwrap_or(""));
            let mut v7: Rc<str> = std::rc::Rc::<str>::from((&*v6).split("(eoie job ").nth(1).and_then(|rest| rest.split(char::from(41u8)).next()).unwrap_or(""));
            let mut v8: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v11: Rc<str> = if v8 {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v9.clone()
            } else {
                let mut v10: Rc<str> = eoie_job_control::eoie_job_state_word(&*v7);
                v10.clone()
            };
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = std::rc::Rc::<str>::from((&*v5).split(char::from(9u8)).nth(0i32 as usize).unwrap_or(""));
            let mut v14: Rc<str> = std::rc::Rc::<str>::from((&*v5).split(char::from(9u8)).nth(1i32 as usize).unwrap_or(""));
            let mut v15: Rc<str> = std::rc::Rc::<str>::from((&*v5).split(char::from(9u8)).nth(2i32 as usize).unwrap_or(""));
            let mut v16: Rc<str> = std::rc::Rc::<str>::from((&*v5).split(char::from(9u8)).nth(3i32 as usize).unwrap_or(""));
            let mut v17: Rc<str> = std::rc::Rc::<str>::from((&*v5).split(char::from(9u8)).nth(2i32 as usize).unwrap_or(""));
            let mut v18: bool = v17.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
            let mut v19: US2 = window_health_of_28(v18, v11.clone());
            let mut v26: Rc<str> = match &v19 {
                US2::US2_WindowFine => {
                    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v20.clone()
                }
                US2::US2_WindowStale(v21) => {
                    let mut v21: Rc<str> = v21.clone();
                    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  <- STALE: job is "); } LIT.with(|lit| lit.clone()) };
                    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v24: Rc<str> = std::rc::Rc::<str>::from([&*v22, &*v21, &*v23, &*v23, &*v23].concat());
                    v24.clone()
                }
            };
            let mut v27: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v15, &*v12, &*v16, &*v26].concat());
            let mut v28: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v13, &*v12, &*v14, &*v27].concat());
            let mut v29: i32 = v1.wrapping_add(1i32);
            let mut v30: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v34: Rc<str> = if v30 {
                v28.clone()
            } else {
                let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v33: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v31, &*v28, &*v32, &*v32].concat());
                v33.clone()
            };
            (v0, v1, v2) = (v0.clone(), v29, v34.clone());
            continue;
        }
    }
}
fn jobs_and_windows_11(mut v0: u64, mut v1: u64, mut v2: f64, mut v3: u64, mut v4: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>) -> Rc<str> {
    let mut v5: Rc<str> = windows_root_12();
    let mut v6: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_dir(&*v5).map(|entries| entries.filter_map(|entry| entry.ok()).filter(|entry| entry.path().extension().map(|extension| extension == "window").unwrap_or(false)).filter_map(|entry| std::fs::read_to_string(entry.path()).ok()).map(|text| text.lines().next().unwrap_or("").to_owned()).collect::<Vec<_>>().join("
")).unwrap_or_default().as_str());
    let mut v7: bool = live_block_held_13(v6.clone());
    let mut v8: Rc<str> = jobs_root_14();
    let mut v9: u64 = 60u64;
    let mut v10: Rc<str> = { let mut names = std::fs::read_dir(&*v8).map(|entries| entries.filter_map(|entry| entry.ok()).filter(|entry| entry.path().is_dir()).map(|entry| entry.file_name().to_string_lossy().into_owned()).filter(|name| name.starts_with("job-")).collect::<Vec<_>>()).unwrap_or_default(); names.sort(); names.reverse(); names.truncate(v9 as usize); std::rc::Rc::<str>::from(names.join("
")) };
    let mut v11: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("LOCALAPPDATA").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir).join("eoie").join("machine-watch").join("quiet.tsv").to_string_lossy().as_ref());
    let mut v12: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v11).unwrap_or_default().as_str());
    let mut v13: i32 = 0i32;
    let mut v14: i32 = 0i32;
    let mut v15: f64 = 0.0f64;
    let mut v16: f64 = cells_cores_15(v4.clone(), v14, v15);
    let mut v17: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let (mut v19, mut v20): (Rc<str>, Rc<str>) = jobs_watched_16(v0, v1, v2, v3, v4.clone(), v10.clone(), v13, v12.clone(), v7, v16, v17, v18.clone(), v18.clone());
    let mut v21: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("LOCALAPPDATA").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir).join("eoie").join("machine-watch").join("quiet.tsv").to_string_lossy().as_ref());
    { let path = std::path::PathBuf::from(&*v21); if let Some(parent) = path.parent() { let _ = std::fs::create_dir_all(parent); } let _ = std::fs::write(&path, (&*v20).as_bytes()); };
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("== eoie jobs running: "); } LIT.with(|lit| lit.clone()) };
    let mut v23: i32 = (&*v20).lines().filter(|line| !line.is_empty()).count() as i32;
    let mut v24: u64 = v23 as u64;
    let mut v25: Rc<str> = std::rc::Rc::<str>::from((v24).to_string());
    let mut v27: Rc<str> = if v7 {
        let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (a live block holds the machine)"); } LIT.with(|lit| lit.clone()) };
        v26.clone()
    } else {
        v18.clone()
    };
    let mut v28: Rc<str> = std::rc::Rc::<str>::from([&*v22, &*v25, &*v27, &*v18, &*v18].concat());
    let mut v29: bool = v19.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v32: Rc<str> = if v29 {
        v28.clone()
    } else {
        let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v31: Rc<str> = std::rc::Rc::<str>::from([&*v28, &*v30, &*v19, &*v18, &*v18].concat());
        v31.clone()
    };
    let mut v33: Rc<str> = gate_summary_26();
    let mut v34: bool = v33.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v38: Rc<str> = if v34 {
        let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("== stop gate: no gate.log"); } LIT.with(|lit| lit.clone()) };
        v35.clone()
    } else {
        let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("== stop gate (gate.log, newest 4000 lines)\n"); } LIT.with(|lit| lit.clone()) };
        let mut v37: Rc<str> = std::rc::Rc::<str>::from([&*v36, &*v33, &*v18, &*v18, &*v18].concat());
        v37.clone()
    };
    let mut v39: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    let mut v40: Rc<str> = eoie_machine_crowding::eoie_machine_crowding(&*v6, v39);
    let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n== windows\n"); } LIT.with(|lit| lit.clone()) };
    let mut v43: i32 = 0i32;
    let mut v44: Rc<str> = windows_watched_27(v6.clone(), v43, v18.clone());
    let mut v45: Rc<str> = std::rc::Rc::<str>::from([&*v32, &*v41, &*v38, &*v42, &*v44].concat());
    let mut v46: bool = v40.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v46 {
        v45.clone()
    } else {
        let mut v47: Rc<str> = std::rc::Rc::<str>::from([&*v45, &*v41, &*v40, &*v18, &*v18].concat());
        v47.clone()
    }
}
fn watch_run_1() -> i32 {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--sample-seconds"); } LIT.with(|lit| lit.clone()) };
    let mut v1: i32 = 2i32;
    let mut v2: Rc<str> = option_value_2(v0.clone(), v1);
    let mut v3: u64 = v2.trim().parse::<u64>().unwrap_or(0);
    let mut v4: bool = v3 == 0u64;
    let mut v5: u64 = if v4 {
        10u64
    } else {
        v3
    };
    let mut v6: u64 = v5.wrapping_mul(1000u64);
    let mut v7: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>> = process_rows_sampled_3(v6);
    let mut v8: i32 = 0i32;
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v10: f64 = 0.0f64;
    let (mut v11, mut v12): (Rc<str>, f64) = busy_lines_4(v7.clone(), v8, v9.clone(), v10);
    let mut v13: u64 = 60u64;
    let mut v14: u64 = 30u64;
    let mut v15: f64 = 0.8f64;
    let mut v16: u64 = 8192u64;
    let mut v17: i32 = 0i32;
    let mut v18: Rc<str> = rogue_lines_8(v13, v14, v15, v16, v7.clone(), v17, v9.clone());
    let mut v19: u64 = 60u64;
    let mut v20: u64 = 30u64;
    let mut v21: f64 = 0.8f64;
    let mut v22: u64 = 8192u64;
    let mut v23: i32 = 0i32;
    let mut v24: Rc<str> = hog_lines_10(v19, v20, v21, v22, v7.clone(), v23, v9.clone());
    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("== machine: "); } LIT.with(|lit| lit.clone()) };
    let mut v26: Rc<str> = std::rc::Rc::<str>::from(format!("{:.2}", v12));
    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cores busy over "); } LIT.with(|lit| lit.clone()) };
    let mut v28: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" s, "); } LIT.with(|lit| lit.clone()) };
    let mut v30: u64 = { let mut system = sysinfo::System::new_with_specifics(sysinfo::RefreshKind::nothing().with_memory(sysinfo::MemoryRefreshKind::nothing().with_ram())); system.refresh_memory(); system.available_memory() / 1048576 };
    let mut v31: Rc<str> = std::rc::Rc::<str>::from((v30).to_string());
    let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" MB free"); } LIT.with(|lit| lit.clone()) };
    let mut v33: Rc<str> = std::rc::Rc::<str>::from([&*v29, &*v31, &*v32, &*v9, &*v9].concat());
    let mut v34: Rc<str> = std::rc::Rc::<str>::from([&*v25, &*v26, &*v27, &*v28, &*v33].concat());
    println!("{}", &*v34);
    let mut v35: Rc<str> = { let mut rows = (&*v11).lines().collect::<Vec<_>>(); rows.sort_by(|left, right| { let cores = |row: &str| row.split(char::from(9u8)).nth(2).and_then(|value| value.parse::<f64>().ok()).unwrap_or(0.0); cores(right).total_cmp(&cores(left)) }); std::rc::Rc::<str>::from(rows.into_iter().take(12).map(|row| format!("  {}", row.replace(char::from(9u8), "  "))).collect::<Vec<_>>().join("
")) };
    println!("{}", &*v35);
    let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("== rogue candidates: "); } LIT.with(|lit| lit.clone()) };
    let mut v37: u64 = (&*v18).lines().filter(|line| !line.is_empty()).count() as u64;
    let mut v38: Rc<str> = std::rc::Rc::<str>::from((v37).to_string());
    let mut v39: Rc<str> = std::rc::Rc::<str>::from([&*v36, &*v38, &*v9, &*v9, &*v9].concat());
    println!("{}", &*v39);
    let mut v40: bool = v18.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v40 {
        ()
    } else {
        println!("{}", &*v18);
        ()
    };
    let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("== memory hogs (>= "); } LIT.with(|lit| lit.clone()) };
    let mut v42: Rc<str> = std::rc::Rc::<str>::from((8192u64).to_string());
    let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" MB): "); } LIT.with(|lit| lit.clone()) };
    let mut v44: u64 = (&*v24).lines().filter(|line| !line.is_empty()).count() as u64;
    let mut v45: Rc<str> = std::rc::Rc::<str>::from((v44).to_string());
    let mut v46: Rc<str> = std::rc::Rc::<str>::from([&*v41, &*v42, &*v43, &*v45, &*v9].concat());
    println!("{}", &*v46);
    let mut v47: bool = v24.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v47 {
        ()
    } else {
        println!("{}", &*v24);
        ()
    };
    let mut v48: u64 = 60u64;
    let mut v49: u64 = 30u64;
    let mut v50: f64 = 0.8f64;
    let mut v51: u64 = 8192u64;
    let mut v52: Rc<str> = jobs_and_windows_11(v48, v49, v50, v51, v7.clone());
    println!("{}", &*v52);
    let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie machine watch ok processes="); } LIT.with(|lit| lit.clone()) };
    let mut v54: i32 = v7.len() as i32;
    let mut v55: u64 = v54 as u64;
    let mut v56: Rc<str> = std::rc::Rc::<str>::from((v55).to_string());
    let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rogues="); } LIT.with(|lit| lit.clone()) };
    let mut v58: u64 = (&*v18).lines().filter(|line| !line.is_empty()).count() as u64;
    let mut v59: Rc<str> = std::rc::Rc::<str>::from((v58).to_string());
    let mut v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" hogs="); } LIT.with(|lit| lit.clone()) };
    let mut v61: u64 = (&*v24).lines().filter(|line| !line.is_empty()).count() as u64;
    let mut v62: Rc<str> = std::rc::Rc::<str>::from((v61).to_string());
    let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" unix="); } LIT.with(|lit| lit.clone()) };
    let mut v64: Rc<str> = std::rc::Rc::<str>::from(format!("{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0)));
    let mut v65: Rc<str> = std::rc::Rc::<str>::from([&*v60, &*v62, &*v63, &*v64, &*v9].concat());
    let mut v66: Rc<str> = std::rc::Rc::<str>::from([&*v53, &*v56, &*v57, &*v59, &*v65].concat());
    println!("{}", &*v66);
    0i32
}
fn machine_run_0() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("watch"); } LIT.with(|lit| lit.clone()) };
    if v2 {
        watch_run_1()
    } else {
        eprintln!("eoie error: machine: usage: machine watch [--sample-seconds n]");
        2i32
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        machine_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_machine_watch_run() -> i32 {
    closure0()()
}
