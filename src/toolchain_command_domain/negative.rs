#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_patterns, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn spiral_main() -> i32 {
    let mut v0: i32 = -1i32;
    let mut v1: i32 = 0i32;
    let mut v2: i32 = toolchain_command_domain::eoie_toolchain_execution_plan(v0, v1);
    let mut v3: i32 = 0i32;
    let mut v4: i32 = 1i32;
    let mut v5: i32 = toolchain_command_domain::eoie_toolchain_execution_plan(v3, v4);
    let mut v6: i32 = 0i32;
    let mut v7: i32 = -1i32;
    let mut v8: i32 = toolchain_command_domain::eoie_toolchain_environment_entry(v6, v7);
    let mut v9: i32 = 0i32;
    let mut v10: i32 = 5i32;
    let mut v11: i32 = toolchain_command_domain::eoie_toolchain_environment_entry(v9, v10);
    let mut v12: i32 = 7i32;
    let mut v13: i32 = 0i32;
    let mut v14: i32 = toolchain_command_domain::eoie_toolchain_environment_entry(v12, v13);
    let mut v15: i32 = 3i32;
    let mut v16: i32 = 0i32;
    let mut v17: i32 = toolchain_command_domain::eoie_toolchain_environment_entry_count(v15, v16);
    let mut v18: i32 = 3i32;
    let mut v19: i32 = 7i32;
    let mut v20: i32 = toolchain_command_domain::eoie_toolchain_environment_entry(v18, v19);
    let mut v21: bool = v2 < 0i32;
    let mut v23: bool = if v21 {
        let mut v22: bool = v5 < 0i32;
        v22
    } else {
        false
    };
    let mut v25: bool = if v23 {
        let mut v24: bool = v8 < 0i32;
        v24
    } else {
        false
    };
    let mut v27: bool = if v25 {
        let mut v26: bool = v11 < 0i32;
        v26
    } else {
        false
    };
    let mut v29: bool = if v27 {
        let mut v28: bool = v14 < 0i32;
        v28
    } else {
        false
    };
    let mut v31: bool = if v29 {
        let mut v30: bool = v17 == 8i32;
        v30
    } else {
        false
    };
    let mut v33: bool = if v31 {
        let mut v32: bool = v20 == 103i32;
        v32
    } else {
        false
    };
    if v33 {
        0i32
    } else {
        61i32
    }
}
fn main() {
    let main = std::thread::Builder::new().stack_size(1 << 30).spawn(spiral_main).unwrap();
    std::process::exit(main.join().unwrap());
}
