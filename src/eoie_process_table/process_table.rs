#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Succeeded,
    US0_Failed,
}
#[derive(Clone)]
enum US1 {
    US1_Allowed(u64),
    US1_OwnProcess,
    US1_OwnAncestor,
    US1_NotAProcessId,
}
#[derive(Clone)]
enum US2 {
    US2_NoSuchProcess,
    US2_StoppedOf(u64, u64),
}
#[derive(Clone)]
enum US3 {
    US3_ListProcesses,
    US3_StopProcesses,
    US3_ProcessUnknown,
}
fn pattern_from_2(mut v0: i32, mut v1: i32) -> Rc<str> {
    loop {
        let mut v2: bool = v0 < v1;
        if v2 {
            let mut v3: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
            let mut v4: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--tree"); } LIT.with(|lit| lit.clone()) };
            if v4 {
                let mut v5: i32 = v0.wrapping_add(1i32);
                (v0, v1) = (v5, v1);
                continue;
            } else {
                let mut v7: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
                return v7.clone();
            }
        } else {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v9.clone();
        }
    }
}
fn flag_present_3(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> bool {
    loop {
        let mut v3: bool = v1 < v2;
        if v3 {
            let mut v4: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v1 as usize).unwrap_or_default());
            let mut v5: bool = v4.clone() == v0.clone();
            if v5 {
                return true;
            } else {
                let mut v6: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v6, v2);
                continue;
            }
        } else {
            return false;
        }
    }
}
fn local_offset_minutes_4() -> i64 {
    let mut v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn process_table_5(mut v0: Rc<str>, mut v1: bool, mut v2: i64) -> std::rc::Rc<(std::rc::Rc<str>, u64)> {
    let mut v3: std::rc::Rc<(std::rc::Rc<str>, u64)> = (|| -> std::rc::Rc<(std::rc::Rc<str>, u64)> { let pattern = match regex::RegexBuilder::new(&*v0).case_insensitive(true).build() { Ok(pattern) => pattern, Err(error) => return std::rc::Rc::new((std::rc::Rc::<str>::from(format!("ps: invalid pattern: {error}")), 0)) }; let system = sysinfo::System::new_with_specifics(sysinfo::RefreshKind::nothing().with_processes(sysinfo::ProcessRefreshKind::nothing().with_memory().with_cmd(sysinfo::UpdateKind::Always))); let own = std::process::id(); let command_of = |process: &sysinfo::Process| { let command = process.cmd().iter().map(|part| part.to_string_lossy().into_owned()).collect::<Vec<_>>().join(" "); if command.is_empty() { process.name().to_string_lossy().into_owned() } else { command } }; let mut children: std::collections::HashMap<u32, Vec<u32>> = std::collections::HashMap::new(); for (pid, process) in system.processes() { if let Some(parent) = process.parent() { children.entry(parent.as_u32()).or_default().push(pid.as_u32()); } } let mut matched = system.processes().iter().filter(|(pid, process)| pid.as_u32() != own && pattern.is_match(&format!("{} {}", process.name().to_string_lossy(), command_of(process)))).map(|(pid, _)| pid.as_u32()).collect::<Vec<_>>(); matched.sort_by_key(|pid| system.process(sysinfo::Pid::from_u32(*pid)).map(|process| process.start_time()).unwrap_or(0)); let offset = v2 * 60; let started = |seconds: u64| { let local = (seconds as i64 + offset).rem_euclid(86400); format!("{:02}:{:02}", local / 3600, local / 60 % 60) }; let row = |pid: u32, depth: usize| -> Option<(String, String, String, String, String)> { let process = system.process(sysinfo::Pid::from_u32(pid))?; let command = command_of(process); let command = if command.chars().count() > 200 { format!("{}...", command.chars().take(200).collect::<String>()) } else { command }; Some((pid.to_string(), process.parent().map(|parent| parent.as_u32().to_string()).unwrap_or_default(), started(process.start_time()), format!("{}", process.memory() / 1048576), format!("{}{}", "  ".repeat(depth), command))) }; let mut rows = Vec::new(); fn descend(pid: u32, depth: usize, children: &std::collections::HashMap<u32, Vec<u32>>, out: &mut Vec<(u32, usize)>) { for child in children.get(&pid).into_iter().flatten() { if out.iter().all(|(seen, _)| seen != child) { out.push((*child, depth)); descend(*child, depth + 1, children, out); } } } for pid in &matched { rows.push((*pid, 0)); if v1 { descend(*pid, 1, &children, &mut rows); } } let cells = rows.iter().filter_map(|(pid, depth)| row(*pid, *depth)).collect::<Vec<_>>(); let width = |pick: &dyn Fn(&(String, String, String, String, String)) -> usize, title: usize| cells.iter().map(|cell| pick(cell)).max().unwrap_or(0).max(title); let (pid_width, parent_width, started_width, memory_width) = (width(&|cell| cell.0.len(), 3), width(&|cell| cell.1.len(), 4), width(&|cell| cell.2.len(), 7), width(&|cell| cell.3.len(), 6)); let mut shown = vec![format!("{:<pid_width$}  {:<parent_width$}  {:<started_width$}  {:>memory_width$}  {}", "pid", "ppid", "started", "mem_mb", "command")]; for cell in &cells { shown.push(format!("{:<pid_width$}  {:<parent_width$}  {:<started_width$}  {:>memory_width$}  {}", cell.0, cell.1, cell.2, cell.3, cell.4)); } std::rc::Rc::new((std::rc::Rc::<str>::from(shown.join("
")), matched.len() as u64)) })();
    v3.clone()
}
fn decimal_text_6(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1.clone()
}
fn elapsed_text_8(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1.clone()
}
fn clock_text_9() -> Rc<str> {
    let mut v0: i64 = local_offset_minutes_4();
    let mut v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1.clone()
}
fn receipt_line_7(mut v0: US0, mut v1: Rc<str>, mut v2: u64) -> () {
    let mut v6: Rc<str> = match &v0 {
        US0::US0_Failed => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US0::US0_Succeeded => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
    };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = elapsed_text_8(v2);
    let mut v9: Rc<str> = clock_text_9();
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v7, &*v9, &*v10, &*v10].concat());
    let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v7, &*v1, &*v7, &*v11].concat());
    println!("{}", &*v12);
    ()
}
fn list_run_1(mut v0: u64) -> i32 {
    let mut v1: i32 = std::env::args().count() as i32;
    let mut v2: i32 = 2i32;
    let mut v3: Rc<str> = pattern_from_2(v2, v1);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--tree"); } LIT.with(|lit| lit.clone()) };
    let mut v5: i32 = 2i32;
    let mut v6: bool = flag_present_3(v4.clone(), v5, v1);
    let mut v7: i64 = local_offset_minutes_4();
    let mut v8: std::rc::Rc<(std::rc::Rc<str>, u64)> = process_table_5(v3.clone(), v6, v7);
    let mut v9: Rc<str> = v8.0.clone();
    println!("{}", &*v9);
    let mut v10: US0 = US0::US0_Succeeded;
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("processes "); } LIT.with(|lit| lit.clone()) };
    let mut v12: u64 = v8.1;
    let mut v13: Rc<str> = decimal_text_6(v12);
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v13, &*v14, &*v14, &*v14].concat());
    receipt_line_7(v10.clone(), v15.clone(), v0);
    0i32
}
fn ancestor_pids_11() -> Rc<str> {
    let mut v0: Rc<str> = { let system = sysinfo::System::new_with_specifics(sysinfo::RefreshKind::nothing().with_processes(sysinfo::ProcessRefreshKind::nothing())); let mut chain: Vec<u32> = Vec::new(); let mut current = sysinfo::Pid::from_u32(std::process::id()); while let Some(parent) = system.process(current).and_then(|process| process.parent()) { if chain.contains(&parent.as_u32()) { break; } chain.push(parent.as_u32()); current = parent; } std::rc::Rc::<str>::from(chain.iter().map(|pid| pid.to_string()).collect::<Vec<_>>().join(",")) };
    v0.clone()
}
fn stop_refusal_of_13(mut v0: Rc<str>, mut v1: u64, mut v2: Rc<str>) -> US1 {
    let mut v3: bool = !v0.is_empty() && v0.bytes().all(|byte| byte.is_ascii_digit());
    if v3 {
        let mut v4: u64 = v0.trim().parse::<u64>().unwrap_or(0);
        let mut v5: bool = v4 == v1;
        if v5 {
            US1::US1_OwnProcess
        } else {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v9: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v2, &*v7, &*v8, &*v8].concat());
            let mut v10: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v0, &*v7, &*v8, &*v8].concat());
            let mut v11: bool = v9.contains(&*v10);
            if v11 {
                US1::US1_OwnAncestor
            } else {
                US1::US1_Allowed(v4)
            }
        }
    } else {
        US1::US1_NotAProcessId
    }
}
fn process_stopped_15(mut v0: u64, mut v1: bool) -> (u64, u64) {
    let mut v2: (u64, u64) = { let system = sysinfo::System::new_with_specifics(sysinfo::RefreshKind::nothing().with_processes(sysinfo::ProcessRefreshKind::nothing())); let root = sysinfo::Pid::from_u32(v0 as u32); match system.process(root) { None => (0u64, 0u64), Some(_) => { let mut order = vec![root]; if v1 { let mut index = 0; while index < order.len() { let parent = order[index]; for (pid, process) in system.processes() { if process.parent() == Some(parent) && !order.contains(pid) { order.push(*pid); } } index += 1; } } for pid in order.iter().rev() { #[cfg(windows)] { unsafe extern "system" { fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void; fn TerminateProcess(process: *mut std::ffi::c_void, code: u32) -> i32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; } unsafe { let handle = OpenProcess(1, 0, pid.as_u32()); if !handle.is_null() { TerminateProcess(handle, 1); CloseHandle(handle); } } } #[cfg(not(windows))] { if let Some(process) = system.process(*pid) { process.kill(); } } } std::thread::sleep(std::time::Duration::from_millis(200)); let after = sysinfo::System::new_with_specifics(sysinfo::RefreshKind::nothing().with_processes(sysinfo::ProcessRefreshKind::nothing())); let survivors = order.iter().filter(|pid| after.process(**pid).is_some()).count(); ((order.len() - survivors) as u64, order.len() as u64) } } };
    v2.clone()
}
fn stop_once_14(mut v0: Rc<str>, mut v1: u64, mut v2: bool) -> bool {
    let mut v3: (u64, u64) = process_stopped_15(v1, v2);
    let mut v4: u64 = v3.clone().0;
    let mut v5: u64 = v3.1;
    let mut v6: bool = v5 == 0u64;
    let mut v9: US2 = if v6 {
        US2::US2_NoSuchProcess
    } else {
        US2::US2_StoppedOf(v4, v5)
    };
    let mut v40: Rc<str> = match &v9 {
        US2::US2_NoSuchProcess => {
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop "); } LIT.with(|lit| lit.clone()) };
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  not found"); } LIT.with(|lit| lit.clone()) };
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v0, &*v11, &*v12, &*v12].concat());
            v13.clone()
        }
        US2::US2_StoppedOf(v14, v15) => {
            let mut v14: u64 = *v14;
            let mut v15: u64 = *v15;
            let mut v16: bool = v14 == v15;
            let mut v17: bool = v16 != true;
            if v17 {
                let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop "); } LIT.with(|lit| lit.clone()) };
                let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  stopped "); } LIT.with(|lit| lit.clone()) };
                let mut v20: Rc<str> = std::rc::Rc::<str>::from((v14).to_string());
                let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" of "); } LIT.with(|lit| lit.clone()) };
                let mut v22: Rc<str> = std::rc::Rc::<str>::from((v15).to_string());
                let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" processes"); } LIT.with(|lit| lit.clone()) };
                let mut v24: Rc<str> = std::rc::Rc::<str>::from([&*v19, &*v20, &*v21, &*v22, &*v23].concat());
                let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v26: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v0, &*v24, &*v25, &*v25].concat());
                v26.clone()
            } else {
                let mut v27: bool = v15 == 1u64;
                if v27 {
                    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop "); } LIT.with(|lit| lit.clone()) };
                    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  stopped"); } LIT.with(|lit| lit.clone()) };
                    let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v31: Rc<str> = std::rc::Rc::<str>::from([&*v28, &*v0, &*v29, &*v30, &*v30].concat());
                    v31.clone()
                } else {
                    let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop "); } LIT.with(|lit| lit.clone()) };
                    let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  stopped (tree: "); } LIT.with(|lit| lit.clone()) };
                    let mut v34: Rc<str> = std::rc::Rc::<str>::from((v15).to_string());
                    let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" processes)"); } LIT.with(|lit| lit.clone()) };
                    let mut v36: Rc<str> = std::rc::Rc::<str>::from([&*v32, &*v0, &*v33, &*v34, &*v35].concat());
                    v36.clone()
                }
            }
        }
    };
    println!("{}", &*v40);
    match &v9 {
        US2::US2_NoSuchProcess => {
            false
        }
        US2::US2_StoppedOf(v41, v42) => {
            let mut v41: u64 = *v41;
            let mut v42: u64 = *v42;
            let mut v43: bool = v41 == v42;
            v43
        }
    }
}
fn stops_from_12(mut v0: i32, mut v1: i32, mut v2: bool, mut v3: u64, mut v4: Rc<str>, mut v5: u64, mut v6: u64) -> (u64, u64) {
    loop {
        let mut v7: bool = v0 < v1;
        if v7 {
            let mut v8: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
            let mut v9: bool = v8.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--tree"); } LIT.with(|lit| lit.clone()) };
            if v9 {
                let mut v10: i32 = v0.wrapping_add(1i32);
                (v0, v1, v2, v3, v4, v5, v6) = (v10, v1, v2, v3, v4.clone(), v5, v6);
                continue;
            } else {
                let mut v13: US1 = stop_refusal_of_13(v8.clone(), v3, v4.clone());
                match &v13 {
                    US1::US1_Allowed(v14) => {
                        let mut v14: u64 = *v14;
                        let mut v15: bool = stop_once_14(v8.clone(), v14, v2);
                        if v15 {
                            let mut v16: i32 = v0.wrapping_add(1i32);
                            let mut v17: u64 = v5.wrapping_add(1u64);
                            (v0, v1, v2, v3, v4, v5, v6) = (v16, v1, v2, v3, v4.clone(), v17, v6);
                            continue;
                        } else {
                            let mut v20: i32 = v0.wrapping_add(1i32);
                            let mut v21: u64 = v6.wrapping_add(1u64);
                            (v0, v1, v2, v3, v4, v5, v6) = (v20, v1, v2, v3, v4.clone(), v5, v21);
                            continue;
                        }
                    }
                    _ => {
                        let mut v40: Rc<str> = match &v13 {
                            US1::US1_NotAProcessId => {
                                let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop "); } LIT.with(|lit| lit.clone()) };
                                let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  refused: not a process id"); } LIT.with(|lit| lit.clone()) };
                                let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v37: Rc<str> = std::rc::Rc::<str>::from([&*v34, &*v8, &*v35, &*v36, &*v36].concat());
                                v37.clone()
                            }
                            US1::US1_OwnAncestor => {
                                let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop "); } LIT.with(|lit| lit.clone()) };
                                let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  refused: that is a parent of this eoie (the calling shell)"); } LIT.with(|lit| lit.clone()) };
                                let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v33: Rc<str> = std::rc::Rc::<str>::from([&*v30, &*v8, &*v31, &*v32, &*v32].concat());
                                v33.clone()
                            }
                            US1::US1_OwnProcess => {
                                let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop "); } LIT.with(|lit| lit.clone()) };
                                let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  refused: that is this eoie process"); } LIT.with(|lit| lit.clone()) };
                                let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v29: Rc<str> = std::rc::Rc::<str>::from([&*v26, &*v8, &*v27, &*v28, &*v28].concat());
                                v29.clone()
                            }
                            _ => unreachable!(),
                        };
                        println!("{}", &*v40);
                        let mut v41: i32 = v0.wrapping_add(1i32);
                        let mut v42: u64 = v6.wrapping_add(1u64);
                        (v0, v1, v2, v3, v4, v5, v6) = (v41, v1, v2, v3, v4.clone(), v5, v42);
                        continue;
                    }
                }
            }
        } else {
            return (v5, v6);
        }
    }
}
fn stop_run_10(mut v0: u64) -> i32 {
    let mut v1: i32 = std::env::args().count() as i32;
    let mut v2: bool = v1 < 3i32;
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: eoie "); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ps [regex] [--tree] | kill <pid>... [--tree]"); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v4, &*v5, &*v5, &*v5].concat());
        println!("{}", &*v6);
        2i32
    } else {
        let mut v7: i32 = 2i32;
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--tree"); } LIT.with(|lit| lit.clone()) };
        let mut v9: i32 = 2i32;
        let mut v10: bool = flag_present_3(v8.clone(), v9, v1);
        let mut v11: u64 = std::process::id() as u64;
        let mut v12: Rc<str> = ancestor_pids_11();
        let mut v13: u64 = 0u64;
        let mut v14: u64 = 0u64;
        let (mut v15, mut v16): (u64, u64) = stops_from_12(v7, v1, v10, v11, v12.clone(), v13, v14);
        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped "); } LIT.with(|lit| lit.clone()) };
        let mut v18: Rc<str> = decimal_text_6(v15);
        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  failed "); } LIT.with(|lit| lit.clone()) };
        let mut v20: Rc<str> = decimal_text_6(v16);
        let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v22: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v18, &*v19, &*v20, &*v21].concat());
        let mut v23: bool = v16 == 0u64;
        if v23 {
            let mut v24: US0 = US0::US0_Succeeded;
            receipt_line_7(v24.clone(), v22.clone(), v0);
            0i32
        } else {
            let mut v25: US0 = US0::US0_Failed;
            receipt_line_7(v25.clone(), v22.clone(), v0);
            1i32
        }
    }
}
fn now_ms_16() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn process_run_0() -> i32 {
    let mut v0: u64 = now_ms_16();
    let mut v1: i32 = 1i32;
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v1 as usize).unwrap_or_default());
    let mut v3: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ps"); } LIT.with(|lit| lit.clone()) };
    let mut v9: US3 = if v3 {
        US3::US3_ListProcesses
    } else {
        let mut v5: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kill"); } LIT.with(|lit| lit.clone()) };
        if v5 {
            US3::US3_StopProcesses
        } else {
            US3::US3_ProcessUnknown
        }
    };
    match &v9 {
        US3::US3_ListProcesses => {
            list_run_1(v0)
        }
        US3::US3_ProcessUnknown => {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: eoie "); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ps [regex] [--tree] | kill <pid>... [--tree]"); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v15: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v13, &*v14, &*v14, &*v14].concat());
            println!("{}", &*v15);
            2i32
        }
        US3::US3_StopProcesses => {
            stop_run_10(v0)
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        process_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_process_run() -> i32 {
    closure0()()
}
