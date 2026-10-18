#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_JobPending,
    US0_JobRunning,
    US0_JobFinished(i32),
    US0_JobTimedOut,
    US0_JobSpawnFailed(Rc<str>),
    US0_JobLost,
}
#[derive(Clone)]
enum US1 {
    US1_PlainStart,
    US1_HeavyStart,
}
#[derive(Clone)]
enum US2 {
    US2_JobStart,
    US2_JobSupervise,
    US2_JobStatus,
    US2_JobWait,
    US2_JobList,
    US2_JobTail,
    US2_JobUnknown,
}
fn decimal_u64_2(mut v0: Rc<str>) -> u64 {
    let mut v1: u64 = v0.trim().parse::<u64>().unwrap_or(0);
    v1
}
fn jobs_root_3() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("EOIE_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("eoie").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("eoie").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".eoie-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn job_state_of_7(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: u64) -> US0 {
    let mut v4: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: bool = v4 != true;
    if v5 {
        let mut v6: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(2).unwrap_or("").trim());
        let mut v7: bool = v6.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished"); } LIT.with(|lit| lit.clone()) };
        if v7 {
            let mut v8: i32 = v0.split(char::from(9u8)).next().and_then(|code| code.trim().parse::<i32>().ok()).unwrap_or(1);
            US0::US0_JobFinished(v8)
        } else {
            let mut v10: bool = v6.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed-out"); } LIT.with(|lit| lit.clone()) };
            if v10 {
                US0::US0_JobTimedOut
            } else {
                let mut v12: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(3).unwrap_or("").trim());
                US0::US0_JobSpawnFailed(v12.clone())
            }
        }
    } else {
        let mut v16: bool = v1 > 0u64;
        if v16 {
            let mut v17: u64 = v1.wrapping_add(15000u64);
            let mut v18: bool = v17 < v3;
            if v18 {
                US0::US0_JobLost
            } else {
                US0::US0_JobRunning
            }
        } else {
            let mut v22: u64 = v2.wrapping_add(15000u64);
            let mut v23: bool = v22 < v3;
            if v23 {
                US0::US0_JobLost
            } else {
                US0::US0_JobPending
            }
        }
    }
}
fn joined_10(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2.clone()
}
fn job_directory_11(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = jobs_root_3();
    joined_10(v1.clone(), v0.clone())
}
fn job_file_9(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = job_directory_11(v0.clone());
    joined_10(v2.clone(), v1.clone())
}
fn text_or_empty_12(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1.clone()
}
fn now_ms_13() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn job_record_of_8(mut v0: Rc<str>) -> (Rc<str>, u64, u64, u64) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = job_file_9(v0.clone(), v1.clone());
    let mut v3: Rc<str> = text_or_empty_12(v2.clone());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = job_file_9(v0.clone(), v4.clone());
    let mut v6: Rc<str> = text_or_empty_12(v5.clone());
    let mut v7: u64 = decimal_u64_2(v6.clone());
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("started_ms"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = job_file_9(v0.clone(), v8.clone());
    let mut v10: Rc<str> = text_or_empty_12(v9.clone());
    let mut v11: u64 = decimal_u64_2(v10.clone());
    let mut v12: u64 = now_ms_13();
    (v3.clone(), v7, v11, v12)
}
fn job_state_now_6(mut v0: Rc<str>) -> US0 {
    let (mut v1, mut v2, mut v3, mut v4): (Rc<str>, u64, u64, u64) = job_record_of_8(v0.clone());
    job_state_of_7(v1.clone(), v2, v3, v4)
}
fn elapsed_of_14(mut v0: Rc<str>) -> u64 {
    let (mut v1, mut v2, mut v3, mut v4): (Rc<str>, u64, u64, u64) = job_record_of_8(v0.clone());
    let mut v5: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v8: u64 = if v5 {
        v4
    } else {
        let mut v6: Rc<str> = std::rc::Rc::<str>::from(v1.split(char::from(9u8)).nth(1).unwrap_or("0"));
        decimal_u64_2(v6.clone())
    };
    let mut v9: bool = v8 > v3;
    if v9 {
        let mut v10: u64 = v8.wrapping_sub(v3);
        v10
    } else {
        0u64
    }
}
fn decimal_text_15(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1.clone()
}
fn file_bytes_16(mut v0: Rc<str>) -> u64 {
    let mut v1: u64 = std::fs::metadata(&*v0).map(|metadata| metadata.len()).unwrap_or(0);
    v1
}
fn job_summary_5(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: US0 = job_state_now_6(v1.clone());
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie job "); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ok id="); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" state="); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v0, &*v4, &*v1, &*v5].concat());
    let mut v20: Rc<str> = match &v2 {
        US0::US0_JobFinished(v9) => {
            let mut v9: i32 = *v9;
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished"); } LIT.with(|lit| lit.clone()) };
            v10.clone()
        }
        US0::US0_JobLost => {
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lost"); } LIT.with(|lit| lit.clone()) };
            v14.clone()
        }
        US0::US0_JobPending => {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pending"); } LIT.with(|lit| lit.clone()) };
            v7.clone()
        }
        US0::US0_JobRunning => {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("running"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US0::US0_JobSpawnFailed(v12) => {
            let mut v12: Rc<str> = v12.clone();
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spawn-failed"); } LIT.with(|lit| lit.clone()) };
            v13.clone()
        }
        US0::US0_JobTimedOut => {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed-out"); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
    };
    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" exit="); } LIT.with(|lit| lit.clone()) };
    let mut v29: i32 = match &v2 {
        US0::US0_JobFinished(v22) => {
            let mut v22: i32 = *v22;
            v22
        }
        US0::US0_JobLost => {
            125i32
        }
        US0::US0_JobPending => {
            3i32
        }
        US0::US0_JobRunning => {
            3i32
        }
        US0::US0_JobSpawnFailed(v23) => {
            let mut v23: Rc<str> = v23.clone();
            125i32
        }
        US0::US0_JobTimedOut => {
            124i32
        }
    };
    let mut v30: Rc<str> = std::rc::Rc::<str>::from((v29).to_string());
    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v32: u64 = elapsed_of_14(v1.clone());
    let mut v33: Rc<str> = decimal_text_15(v32);
    let mut v34: Rc<str> = std::rc::Rc::<str>::from([&*v20, &*v21, &*v30, &*v31, &*v33].concat());
    let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" stdout_bytes="); } LIT.with(|lit| lit.clone()) };
    let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stdout.log"); } LIT.with(|lit| lit.clone()) };
    let mut v37: Rc<str> = job_file_9(v1.clone(), v36.clone());
    let mut v38: u64 = file_bytes_16(v37.clone());
    let mut v39: Rc<str> = decimal_text_15(v38);
    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" stderr_bytes="); } LIT.with(|lit| lit.clone()) };
    let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stderr.log"); } LIT.with(|lit| lit.clone()) };
    let mut v42: Rc<str> = job_file_9(v1.clone(), v41.clone());
    let mut v43: u64 = file_bytes_16(v42.clone());
    let mut v44: Rc<str> = decimal_text_15(v43);
    let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v46: Rc<str> = std::rc::Rc::<str>::from([&*v35, &*v39, &*v40, &*v44, &*v45].concat());
    let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" label="); } LIT.with(|lit| lit.clone()) };
    let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("label"); } LIT.with(|lit| lit.clone()) };
    let mut v49: Rc<str> = job_file_9(v1.clone(), v48.clone());
    let mut v50: Rc<str> = text_or_empty_12(v49.clone());
    let mut v51: Rc<str> = std::rc::Rc::<str>::from([&*v47, &*v50, &*v45, &*v45, &*v45].concat());
    let mut v52: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v34, &*v46, &*v51, &*v45].concat());
    v52.clone()
}
fn stdout_line_17(mut v0: Rc<str>) -> () {
    println!("{}", &*v0);
    ()
}
fn listed_4(mut v0: Rc<str>) -> () {
    loop {
        let mut v1: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v1 != true;
        if v2 {
            let mut v3: Rc<str> = std::rc::Rc::<str>::from(v0.lines().next().unwrap_or(""));
            let mut v4: Rc<str> = std::rc::Rc::<str>::from(v0.lines().skip(1).collect::<Vec<_>>().join("
"));
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list"); } LIT.with(|lit| lit.clone()) };
            let mut v6: Rc<str> = job_summary_5(v5.clone(), v3.clone());
            stdout_line_17(v6.clone());
            v0 = v4.clone();
            continue;
        }
        return ();
    }
}
fn job_listed_1() -> i32 {
    let mut v0: i32 = std::env::args().count() as i32;
    let mut v1: bool = v0 > 3i32;
    let mut v5: u64 = if v1 {
        let mut v2: i32 = 3i32;
        let mut v3: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v2 as usize).unwrap_or_default());
        decimal_u64_2(v3.clone())
    } else {
        20u64
    };
    let mut v6: Rc<str> = jobs_root_3();
    let mut v7: Rc<str> = { let mut names = std::fs::read_dir(&*v6).map(|entries| entries.filter_map(|entry| entry.ok()).filter(|entry| entry.path().is_dir()).map(|entry| entry.file_name().to_string_lossy().into_owned()).filter(|name| name.starts_with("job-")).collect::<Vec<_>>()).unwrap_or_default(); names.sort(); names.reverse(); names.truncate(v5 as usize); std::rc::Rc::<str>::from(names.join("
")) };
    listed_4(v7.clone());
    0i32
}
fn stderr_line_20(mut v0: Rc<str>) -> () {
    eprintln!("{}", &*v0);
    ()
}
fn rejected_19(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: job "); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v0, &*v2, &*v2, &*v2].concat());
    stderr_line_20(v3.clone());
    2i32
}
fn heavy_gate_script_22() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("EOIE_HEAVY_GATE").map(std::path::PathBuf::from).filter(|path| path.is_file()).or_else(|| std::env::current_dir().ok().and_then(|cwd| cwd.ancestors().map(|base| base.join("spiral").join("apps").join("compiler").join("tmp").join("scripts").join("heavy.ps1")).find(|candidate| candidate.is_file()))).map(|path| path.to_string_lossy().into_owned()).unwrap_or_default());
    v0.clone()
}
fn text_written_23(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2.clone()
}
fn job_argv_21(mut v0: US1, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    match &v0 {
        US1::US1_HeavyStart => {
            let mut v4: Rc<str> = heavy_gate_script_22();
            let mut v5: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v5 {
                let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v6.clone()
            } else {
                let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("argv.inner"); } LIT.with(|lit| lit.clone()) };
                let mut v8: Rc<str> = job_file_9(v1.clone(), v7.clone());
                let mut v9: Rc<str> = text_written_23(v8.clone(), v3.clone());
                let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-wrapper.ps1"); } LIT.with(|lit| lit.clone()) };
                let mut v11: Rc<str> = job_file_9(v1.clone(), v10.clone());
                let mut v12: Rc<str> = job_file_9(v1.clone(), v7.clone());
                let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$argv = @(Get-Content -LiteralPath '"); } LIT.with(|lit| lit.clone()) };
                let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("')\n. '"); } LIT.with(|lit| lit.clone()) };
                let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'\n"); } LIT.with(|lit| lit.clone()) };
                let mut v16: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v12, &*v14, &*v4, &*v15].concat());
                let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Invoke-Heavy -Label '"); } LIT.with(|lit| lit.clone()) };
                let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (eoie job "); } LIT.with(|lit| lit.clone()) };
                let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")' { & $argv[0] @($argv | Select-Object -Skip 1) }\n"); } LIT.with(|lit| lit.clone()) };
                let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v21: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v18, &*v1, &*v19, &*v20].concat());
                let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit $LASTEXITCODE\n"); } LIT.with(|lit| lit.clone()) };
                let mut v23: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v17, &*v21, &*v22, &*v20].concat());
                let mut v24: Rc<str> = text_written_23(v11.clone(), v23.clone());
                let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pwsh\n-NoProfile\n-File\n"); } LIT.with(|lit| lit.clone()) };
                let mut v26: Rc<str> = std::rc::Rc::<str>::from([&*v25, &*v11, &*v20, &*v20, &*v20].concat());
                v26.clone()
            }
        }
        US1::US1_PlainStart => {
            v3.clone()
        }
    }
}
fn supervisor_executable_24(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { let copy = || -> std::io::Result<std::path::PathBuf> { let source = std::env::current_exe()?; let metadata = std::fs::metadata(&source)?; let stamp = metadata.modified().ok().and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok()).map(|elapsed| elapsed.as_secs()).unwrap_or(0); let folder = std::path::Path::new(&*v0).join("supervisor"); std::fs::create_dir_all(&folder)?; let target = folder.join(format!("eoie-{}-{}{}", metadata.len(), stamp, std::env::consts::EXE_SUFFIX)); if !target.is_file() { let temporary = target.with_extension("tmp"); std::fs::copy(&source, &temporary)?; std::fs::rename(&temporary, &target)?; } Ok(target) }; std::rc::Rc::<str>::from(copy().map(|path| path.to_string_lossy().into_owned()).unwrap_or_default()) };
    v1.clone()
}
fn job_started_18() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--heavy"); } LIT.with(|lit| lit.clone()) };
    let mut v5: US1 = if v2 {
        US1::US1_HeavyStart
    } else {
        US1::US1_PlainStart
    };
    let mut v7: i32 = match &v5 {
        US1::US1_HeavyStart => {
            1i32
        }
        US1::US1_PlainStart => {
            0i32
        }
    };
    let mut v8: i32 = 3i32.wrapping_add(v7);
    let mut v9: i32 = std::env::args().count() as i32;
    let mut v10: i32 = v8.wrapping_add(3i32);
    let mut v11: bool = v9 < v10;
    if v11 {
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job start [--heavy] <label> <timeout-ms> <program> [args...] | job status <id> | job wait <id> <max-ms> | job list [count] | job tail <id> [lines]"); } LIT.with(|lit| lit.clone()) };
        rejected_19(v12.clone())
    } else {
        let mut v14: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v8 as usize).unwrap_or_default());
        let mut v15: i32 = v8.wrapping_add(1i32);
        let mut v16: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v15 as usize).unwrap_or_default());
        let mut v17: u64 = decimal_u64_2(v16.clone());
        let mut v18: bool = v17 == 0u64;
        if v18 {
            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout-ms must be a positive integer"); } LIT.with(|lit| lit.clone()) };
            rejected_19(v19.clone())
        } else {
            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
            let mut v22: u64 = now_ms_13();
            let mut v23: Rc<str> = decimal_text_15(v22);
            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let mut v25: Rc<str> = std::rc::Rc::<str>::from(v14.chars().map(|c| if c.is_ascii_alphanumeric() || c == char::from(45u8) { c } else { char::from(95u8) }).take(40).collect::<String>());
            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v27: Rc<str> = std::rc::Rc::<str>::from([&*v21, &*v23, &*v24, &*v25, &*v26].concat());
            let mut v28: Rc<str> = job_directory_11(v27.clone());
            let mut v29: Rc<str> = std::rc::Rc::<str>::from(std::fs::create_dir_all(&*v28).err().map(|error| format!("create {}: {error}", &*v28)).unwrap_or_default());
            let mut v30: bool = v29.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v31: bool = v30 != true;
            if v31 {
                rejected_19(v29.clone())
            } else {
                let mut v33: i32 = v8.wrapping_add(2i32);
                let mut v34: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(v33 as usize).collect::<Vec<_>>().join("
"));
                let mut v35: Rc<str> = job_argv_21(v5.clone(), v27.clone(), v14.clone(), v34.clone());
                let mut v36: bool = v35.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                if v36 {
                    let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--heavy needs spiral's scripts/heavy.ps1: set EOIE_HEAVY_GATE or run from inside a tree that has a spiral checkout"); } LIT.with(|lit| lit.clone()) };
                    rejected_19(v37.clone())
                } else {
                    let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("label"); } LIT.with(|lit| lit.clone()) };
                    let mut v40: Rc<str> = job_file_9(v27.clone(), v39.clone());
                    let mut v41: Rc<str> = text_written_23(v40.clone(), v14.clone());
                    let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout_ms"); } LIT.with(|lit| lit.clone()) };
                    let mut v43: Rc<str> = job_file_9(v27.clone(), v42.clone());
                    let mut v44: Rc<str> = decimal_text_15(v17);
                    let mut v45: Rc<str> = text_written_23(v43.clone(), v44.clone());
                    let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cwd"); } LIT.with(|lit| lit.clone()) };
                    let mut v47: Rc<str> = job_file_9(v27.clone(), v46.clone());
                    let mut v48: Rc<str> = std::rc::Rc::<str>::from(std::env::current_dir().map(|path| path.to_string_lossy().into_owned()).unwrap_or_default());
                    let mut v49: Rc<str> = text_written_23(v47.clone(), v48.clone());
                    let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("argv"); } LIT.with(|lit| lit.clone()) };
                    let mut v51: Rc<str> = job_file_9(v27.clone(), v50.clone());
                    let mut v52: Rc<str> = text_written_23(v51.clone(), v35.clone());
                    let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("started_ms"); } LIT.with(|lit| lit.clone()) };
                    let mut v54: Rc<str> = job_file_9(v27.clone(), v53.clone());
                    let mut v55: u64 = now_ms_13();
                    let mut v56: Rc<str> = decimal_text_15(v55);
                    let mut v57: Rc<str> = text_written_23(v54.clone(), v56.clone());
                    let mut v58: Rc<str> = std::rc::Rc::<str>::from([&*v41, &*v45, &*v49, &*v52, &*v57].concat());
                    let mut v59: bool = v58.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v60: bool = v59 != true;
                    if v60 {
                        rejected_19(v58.clone())
                    } else {
                        let mut v62: Rc<str> = jobs_root_3();
                        let mut v63: Rc<str> = supervisor_executable_24(v62.clone());
                        let mut v64: Rc<str> = { let launch = |flags: u32| { let program = if (&*v63).is_empty() { std::env::current_exe()? } else { std::path::PathBuf::from(&*v63) }; let mut command = std::process::Command::new(program); command.arg("job").arg("supervise").arg(&*v27).stdin(std::process::Stdio::null()).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()); #[cfg(windows)] { use std::os::windows::process::CommandExt; command.creation_flags(flags); } #[cfg(unix)] { use std::os::unix::process::CommandExt; command.process_group(0); let _ = flags; } command.spawn() }; match launch(0x0100_0208).or_else(|_| launch(0x0000_0208)) { Ok(child) => std::rc::Rc::<str>::from(child.id().to_string()), Err(error) => std::rc::Rc::<str>::from(format!("error: {error}")) } };
                        let mut v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error:"); } LIT.with(|lit| lit.clone()) };
                        let mut v66: bool = v64.starts_with(&*v65);
                        if v66 {
                            rejected_19(v64.clone())
                        } else {
                            let mut v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie job start ok id="); } LIT.with(|lit| lit.clone()) };
                            let mut v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" supervisor_pid="); } LIT.with(|lit| lit.clone()) };
                            let mut v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" timeout_ms="); } LIT.with(|lit| lit.clone()) };
                            let mut v71: Rc<str> = decimal_text_15(v17);
                            let mut v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" gate="); } LIT.with(|lit| lit.clone()) };
                            let mut v76: Rc<str> = match &v5 {
                                US1::US1_HeavyStart => {
                                    let mut v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy"); } LIT.with(|lit| lit.clone()) };
                                    v74.clone()
                                }
                                US1::US1_PlainStart => {
                                    let mut v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("plain"); } LIT.with(|lit| lit.clone()) };
                                    v73.clone()
                                }
                            };
                            let mut v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" label="); } LIT.with(|lit| lit.clone()) };
                            let mut v78: Rc<str> = std::rc::Rc::<str>::from([&*v77, &*v14, &*v26, &*v26, &*v26].concat());
                            let mut v79: Rc<str> = std::rc::Rc::<str>::from([&*v70, &*v71, &*v72, &*v76, &*v78].concat());
                            let mut v80: Rc<str> = std::rc::Rc::<str>::from([&*v68, &*v27, &*v69, &*v64, &*v79].concat());
                            stdout_line_17(v80.clone());
                            0i32
                        }
                    }
                }
            }
        }
    }
}
fn job_status_25() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v1.starts_with(&*v2);
    let mut v6: bool = if v3 {
        let mut v4: Rc<str> = job_directory_11(v1.clone());
        let mut v5: bool = std::path::Path::new(&*v4).is_dir();
        v5
    } else {
        false
    };
    if v6 {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = job_summary_5(v7.clone(), v1.clone());
        stdout_line_17(v8.clone());
        0i32
    } else {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status: unknown job id "); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v1, &*v10, &*v10, &*v10].concat());
        rejected_19(v11.clone())
    }
}
fn supervised_27(mut v0: std::rc::Rc<std::cell::RefCell<std::process::Child>>, mut v1: Rc<str>, mut v2: u64, mut v3: u64) -> i32 {
    loop {
        let mut v4: i64 = match v0.clone().borrow_mut().try_wait() { Ok(Some(status)) => status.code().map(|code| code as i64).unwrap_or(-2), Ok(None) => -1, Err(_) => -2 };
        let mut v5: bool = v4 == -1i64;
        let mut v6: bool = v5 != true;
        if v6 {
            let mut v7: i32 = v4 as i32;
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
            let mut v9: Rc<str> = job_file_9(v1.clone(), v8.clone());
            let mut v10: u64 = now_ms_13();
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v12: Rc<str> = std::rc::Rc::<str>::from((v7).to_string());
            let mut v13: Rc<str> = std::rc::Rc::<str>::from((v10).to_string());
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished"); } LIT.with(|lit| lit.clone()) };
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v14, &*v11, &*v16, &*v11].concat());
            let mut v18: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v14, &*v13, &*v14, &*v17].concat());
            let mut v19: Rc<str> = text_written_23(v9.clone(), v18.clone());
            return v7;
        } else {
            let mut v20: u64 = now_ms_13();
            let mut v21: bool = v20 > v2;
            if v21 {
                { let mut child = v0.borrow_mut(); let _ = child.kill(); let _ = child.wait(); };
                let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
                let mut v23: Rc<str> = job_file_9(v1.clone(), v22.clone());
                let mut v24: u64 = now_ms_13();
                let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wall-clock timeout"); } LIT.with(|lit| lit.clone()) };
                let mut v26: Rc<str> = std::rc::Rc::<str>::from((124i32).to_string());
                let mut v27: Rc<str> = std::rc::Rc::<str>::from((v24).to_string());
                let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed-out"); } LIT.with(|lit| lit.clone()) };
                let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v32: Rc<str> = std::rc::Rc::<str>::from([&*v29, &*v28, &*v25, &*v30, &*v31].concat());
                let mut v33: Rc<str> = std::rc::Rc::<str>::from([&*v26, &*v28, &*v27, &*v28, &*v32].concat());
                let mut v34: Rc<str> = text_written_23(v23.clone(), v33.clone());
                return 124i32;
            } else {
                let mut v35: u64 = now_ms_13();
                let mut v36: u64 = v3.wrapping_add(1000u64);
                let mut v37: bool = v35 > v36;
                let mut v42: u64 = if v37 {
                    let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
                    let mut v39: Rc<str> = job_file_9(v1.clone(), v38.clone());
                    let mut v40: Rc<str> = decimal_text_15(v35);
                    let mut v41: Rc<str> = text_written_23(v39.clone(), v40.clone());
                    v35
                } else {
                    v3
                };
                let mut v43: u64 = 200u64;
                std::thread::sleep(std::time::Duration::from_millis(v43));
                (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v2, v42);
                continue;
            }
        }
    }
}
fn job_supervised_26() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("argv"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = job_file_9(v1.clone(), v2.clone());
    let mut v4: Rc<str> = text_or_empty_12(v3.clone());
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cwd"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = job_file_9(v1.clone(), v5.clone());
    let mut v7: Rc<str> = text_or_empty_12(v6.clone());
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stdout.log"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = job_file_9(v1.clone(), v8.clone());
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stderr.log"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = job_file_9(v1.clone(), v10.clone());
    let mut v12: Result<std::rc::Rc<std::cell::RefCell<std::process::Child>>, std::rc::Rc<str>> = { let mut parts = v4.lines(); let program = parts.next().unwrap_or_default().to_owned(); let arguments = parts.map(str::to_owned).collect::<Vec<_>>(); let outcome = (|| -> std::io::Result<std::process::Child> { let stdout = std::fs::File::create(&*v9)?; let stderr = std::fs::File::create(&*v11)?; std::process::Command::new(&program).args(&arguments).current_dir(&*v7).stdin(std::process::Stdio::null()).stdout(stdout).stderr(stderr).spawn() })(); outcome.map(|child| std::rc::Rc::new(std::cell::RefCell::new(child))).map_err(|error| std::rc::Rc::<str>::from(format!("spawn {program}: {error}"))) };
    let mut v13: bool = v12.is_ok();
    if v13 {
        let mut v14: std::rc::Rc<std::cell::RefCell<std::process::Child>> = v12.clone().unwrap();
        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pid"); } LIT.with(|lit| lit.clone()) };
        let mut v16: Rc<str> = job_file_9(v1.clone(), v15.clone());
        let mut v17: u64 = v14.borrow().id() as u64;
        let mut v18: Rc<str> = decimal_text_15(v17);
        let mut v19: Rc<str> = text_written_23(v16.clone(), v18.clone());
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
        let mut v21: Rc<str> = job_file_9(v1.clone(), v20.clone());
        let mut v22: u64 = now_ms_13();
        let mut v23: Rc<str> = decimal_text_15(v22);
        let mut v24: Rc<str> = text_written_23(v21.clone(), v23.clone());
        let mut v25: u64 = now_ms_13();
        let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout_ms"); } LIT.with(|lit| lit.clone()) };
        let mut v27: Rc<str> = job_file_9(v1.clone(), v26.clone());
        let mut v28: Rc<str> = text_or_empty_12(v27.clone());
        let mut v29: u64 = decimal_u64_2(v28.clone());
        let mut v30: u64 = v25.wrapping_add(v29);
        let mut v31: u64 = now_ms_13();
        supervised_27(v14.clone(), v1.clone(), v30, v31)
    } else {
        let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
        let mut v34: Rc<str> = job_file_9(v1.clone(), v33.clone());
        let mut v35: u64 = now_ms_13();
        let mut v36: Rc<str> = v12.clone().err().unwrap_or_default();
        let mut v37: Rc<str> = std::rc::Rc::<str>::from((125i32).to_string());
        let mut v38: Rc<str> = std::rc::Rc::<str>::from((v35).to_string());
        let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
        let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spawn-failed"); } LIT.with(|lit| lit.clone()) };
        let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v43: Rc<str> = std::rc::Rc::<str>::from([&*v40, &*v39, &*v36, &*v41, &*v42].concat());
        let mut v44: Rc<str> = std::rc::Rc::<str>::from([&*v37, &*v39, &*v38, &*v39, &*v43].concat());
        let mut v45: Rc<str> = text_written_23(v34.clone(), v44.clone());
        125i32
    }
}
fn last_lines_29(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let mut v2: Rc<str> = { let text = std::fs::read_to_string(&*v0).unwrap_or_default(); let lines = text.lines().collect::<Vec<_>>(); let from = lines.len().saturating_sub(v1 as usize); std::rc::Rc::<str>::from(lines[from..].join("
")) };
    v2.clone()
}
fn job_tailed_28() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v1.starts_with(&*v2);
    let mut v6: bool = if v3 {
        let mut v4: Rc<str> = job_directory_11(v1.clone());
        let mut v5: bool = std::path::Path::new(&*v4).is_dir();
        v5
    } else {
        false
    };
    if v6 {
        let mut v7: i32 = std::env::args().count() as i32;
        let mut v8: bool = v7 > 4i32;
        let mut v12: u64 = if v8 {
            let mut v9: i32 = 4i32;
            let mut v10: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v9 as usize).unwrap_or_default());
            decimal_u64_2(v10.clone())
        } else {
            20u64
        };
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tail"); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<str> = job_summary_5(v13.clone(), v1.clone());
        stdout_line_17(v14.clone());
        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--- stdout (last "); } LIT.with(|lit| lit.clone()) };
        let mut v16: Rc<str> = decimal_text_15(v12);
        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" lines)\n"); } LIT.with(|lit| lit.clone()) };
        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stdout.log"); } LIT.with(|lit| lit.clone()) };
        let mut v19: Rc<str> = job_file_9(v1.clone(), v18.clone());
        let mut v20: Rc<str> = last_lines_29(v19.clone(), v12);
        let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v22: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v16, &*v17, &*v20, &*v21].concat());
        stdout_line_17(v22.clone());
        let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--- stderr (last "); } LIT.with(|lit| lit.clone()) };
        let mut v24: Rc<str> = decimal_text_15(v12);
        let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stderr.log"); } LIT.with(|lit| lit.clone()) };
        let mut v26: Rc<str> = job_file_9(v1.clone(), v25.clone());
        let mut v27: Rc<str> = last_lines_29(v26.clone(), v12);
        let mut v28: Rc<str> = std::rc::Rc::<str>::from([&*v23, &*v24, &*v17, &*v27, &*v21].concat());
        stdout_line_17(v28.clone());
        0i32
    } else {
        let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tail: unknown job id "); } LIT.with(|lit| lit.clone()) };
        let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v31: Rc<str> = std::rc::Rc::<str>::from([&*v29, &*v1, &*v30, &*v30, &*v30].concat());
        rejected_19(v31.clone())
    }
}
fn waited_31(mut v0: Rc<str>, mut v1: u64) -> US0 {
    loop {
        let mut v2: US0 = job_state_now_6(v0.clone());
        let mut v4: bool = match &v2 {
            US0::US0_JobPending => {
                false
            }
            US0::US0_JobRunning => {
                false
            }
            _ => {
                true
            }
        };
        let mut v7: bool = if v4 {
            true
        } else {
            let mut v5: u64 = now_ms_13();
            let mut v6: bool = v5 > v1;
            v6
        };
        if v7 {
            return v2.clone();
        } else {
            let mut v8: u64 = 250u64;
            std::thread::sleep(std::time::Duration::from_millis(v8));
            (v0, v1) = (v0.clone(), v1);
            continue;
        }
    }
}
fn job_waited_30() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let mut v2: Rc<str> = jobs_root_3();
    let mut v3: Rc<str> = supervisor_executable_24(v2.clone());
    let mut v4: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: bool = v4 != true;
    let mut v8: bool = if v5 {
        let mut v6: bool = std::env::var_os("EOIE_JOB_RELAUNCHED").is_some();
        let mut v7: bool = v6 == false;
        v7
    } else {
        false
    };
    let mut v11: bool = if v8 {
        let mut v9: bool = std::env::current_exe().map(|path| path.to_string_lossy() == &*v3).unwrap_or(false);
        let mut v10: bool = v9 == false;
        v10
    } else {
        false
    };
    if v11 {
        let mut v12: i32 = match std::process::Command::new(&*v3).args(std::env::args().skip(1)).env("EOIE_JOB_RELAUNCHED", "1").status() { Ok(status) => status.code().unwrap_or(1), Err(error) => { eprintln!("eoie error: job relaunch: {error}"); 2 } };
        v12
    } else {
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
        let mut v14: bool = v1.starts_with(&*v13);
        let mut v17: bool = if v14 {
            let mut v15: Rc<str> = job_directory_11(v1.clone());
            let mut v16: bool = std::path::Path::new(&*v15).is_dir();
            v16
        } else {
            false
        };
        if v17 {
            let mut v18: u64 = now_ms_13();
            let mut v19: i32 = 4i32;
            let mut v20: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v19 as usize).unwrap_or_default());
            let mut v21: u64 = decimal_u64_2(v20.clone());
            let mut v22: u64 = v18.wrapping_add(v21);
            let mut v23: US0 = waited_31(v1.clone(), v22);
            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait"); } LIT.with(|lit| lit.clone()) };
            let mut v25: Rc<str> = job_summary_5(v24.clone(), v1.clone());
            stdout_line_17(v25.clone());
            match &v23 {
                US0::US0_JobFinished(v26) => {
                    let mut v26: i32 = *v26;
                    v26
                }
                US0::US0_JobLost => {
                    125i32
                }
                US0::US0_JobPending => {
                    3i32
                }
                US0::US0_JobRunning => {
                    3i32
                }
                US0::US0_JobSpawnFailed(v27) => {
                    let mut v27: Rc<str> = v27.clone();
                    125i32
                }
                US0::US0_JobTimedOut => {
                    124i32
                }
            }
        } else {
            let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait: unknown job id "); } LIT.with(|lit| lit.clone()) };
            let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v36: Rc<str> = std::rc::Rc::<str>::from([&*v34, &*v1, &*v35, &*v35, &*v35].concat());
            rejected_19(v36.clone())
        }
    }
}
fn job_run_0() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("start"); } LIT.with(|lit| lit.clone()) };
    let mut v20: US2 = if v2 {
        US2::US2_JobStart
    } else {
        let mut v4: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("supervise"); } LIT.with(|lit| lit.clone()) };
        if v4 {
            US2::US2_JobSupervise
        } else {
            let mut v6: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
            if v6 {
                US2::US2_JobStatus
            } else {
                let mut v8: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait"); } LIT.with(|lit| lit.clone()) };
                if v8 {
                    US2::US2_JobWait
                } else {
                    let mut v10: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list"); } LIT.with(|lit| lit.clone()) };
                    if v10 {
                        US2::US2_JobList
                    } else {
                        let mut v12: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tail"); } LIT.with(|lit| lit.clone()) };
                        if v12 {
                            US2::US2_JobTail
                        } else {
                            US2::US2_JobUnknown
                        }
                    }
                }
            }
        }
    };
    match &v20 {
        US2::US2_JobList => {
            job_listed_1()
        }
        US2::US2_JobStart => {
            job_started_18()
        }
        US2::US2_JobStatus => {
            job_status_25()
        }
        US2::US2_JobSupervise => {
            job_supervised_26()
        }
        US2::US2_JobTail => {
            job_tailed_28()
        }
        US2::US2_JobUnknown => {
            let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job start [--heavy] <label> <timeout-ms> <program> [args...] | job status <id> | job wait <id> <max-ms> | job list [count] | job tail <id> [lines]"); } LIT.with(|lit| lit.clone()) };
            rejected_19(v27.clone())
        }
        US2::US2_JobWait => {
            job_waited_30()
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        job_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_job_run() -> i32 {
    closure0()()
}
