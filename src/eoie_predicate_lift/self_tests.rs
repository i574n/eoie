#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_patterns, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
// Instrumented Windows test runs flush the profile before the generated launcher's process::exit.
fn native_codemod_coverage_finish(code: i32) -> i32 {
    #[cfg(all(windows, eoie_coverage))]
    {
        unsafe extern "C" { fn __llvm_profile_write_file() -> i32; }
        if unsafe { __llvm_profile_write_file() } != 0 {
            eprintln!("native codemod coverage profile flush failed");
            return if code == 0 { 1 } else { code };
        }
    }
    code
}

#[derive(Clone)]
enum US0 {
    US0_0(Rc<str>),
    US0_1(Rc<str>),
}
impl US0 {
    fn tag(&self) -> i32 {
        match self {
            US0::US0_0(..) => 0,
            US0::US0_1(..) => 1,
        }
    }
}
fn method1(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl rust_global (x : string) : () = !!!!Global(x)\ninl emit () : () = rust_global "); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = predicate_lex_domain::eoie_predicate_encode(&*v0);
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v2, &*v3, &*v4, &*v4].concat());
    v5.clone()
}
fn method2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US0 {
    let (mut v3, mut v4): (bool, Rc<str>) = predicate_rewrite_domain::eoie_predicate_rewrite(&*v0,&*v1,&*v2);
    if v3 {
        US0::US0_1(v4.clone())
    } else {
        US0::US0_0(v4.clone())
    }
}
fn method3(mut v0: bool, mut v1: Rc<str>) -> i32 {
    if v0 {
        0i32
    } else {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" failed\n"); } LIT.with(|lit| lit.clone()) };
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v2, &*v3, &*v3, &*v3].concat());
        std::io::Write::write_all(&mut std::io::stdout(),v4.as_bytes()).expect("write output");
        1i32
    }
}
fn method0() -> i32 {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fn digest(v:&str)->bool { v.len()==64 && v.bytes().all(|b|b.is_ascii_hexdigit()) }"); } LIT.with(|lit| lit.clone()) };
    let mut v1: Rc<str> = method1(v0.clone());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("digest"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sha256"); } LIT.with(|lit| lit.clone()) };
    let mut v4: US0 = method2(v1.clone(), v2.clone(), v3.clone());
    let mut v9: Rc<str> = match &v4 {
        US0::US0_1(v5) => { // Candidate
            let mut v5: Rc<str> = v5.clone();
            v5.clone()
        }
        US0::US0_0(v6) => { // Rejected
            let mut v6: Rc<str> = v6.clone();
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v7.clone()
        }
        _ => unreachable!(),
    };
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lift_sha256 \"digest\""); } LIT.with(|lit| lit.clone()) };
    let mut v11: bool = v9.contains(&*v10);
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("hash lift"); } LIT.with(|lit| lit.clone()) };
    let mut v13: i32 = method3(v11, v12.clone());
    let mut v14: US0 = method2(v9.clone(), v2.clone(), v3.clone());
    let mut v16: bool = match &v14 {
        US0::US0_0(v15) => { // Rejected
            let mut v15: Rc<str> = v15.clone();
            true
        }
        _ => {
            false
        }
    };
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("idempotent rejection"); } LIT.with(|lit| lit.clone()) };
    let mut v18: i32 = method3(v16, v17.clone());
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fn digest(v:&str)->bool{v.len()==63&&v.bytes().all(|b|b.is_ascii_hexdigit())}"); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = method1(v19.clone());
    let mut v21: US0 = method2(v20.clone(), v2.clone(), v3.clone());
    let mut v23: bool = match &v21 {
        US0::US0_0(v22) => { // Rejected
            let mut v22: Rc<str> = v22.clone();
            true
        }
        _ => {
            false
        }
    };
    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("near miss"); } LIT.with(|lit| lit.clone()) };
    let mut v25: i32 = method3(v23, v24.clone());
    let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pub fn digest(v:&str)->bool{v.len()==64&&v.bytes().all(|b|b.is_ascii_hexdigit())}"); } LIT.with(|lit| lit.clone()) };
    let mut v27: Rc<str> = method1(v26.clone());
    let mut v28: US0 = method2(v27.clone(), v2.clone(), v3.clone());
    let mut v30: bool = match &v28 {
        US0::US0_0(v29) => { // Rejected
            let mut v29: Rc<str> = v29.clone();
            true
        }
        _ => {
            false
        }
    };
    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("visibility preservation"); } LIT.with(|lit| lit.clone()) };
    let mut v32: i32 = method3(v30, v31.clone());
    let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v34: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v1, &*v33, &*v33, &*v33].concat());
    let mut v35: US0 = method2(v34.clone(), v2.clone(), v3.clone());
    let mut v37: bool = match &v35 {
        US0::US0_0(v36) => { // Rejected
            let mut v36: Rc<str> = v36.clone();
            true
        }
        _ => {
            false
        }
    };
    let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ambiguity"); } LIT.with(|lit| lit.clone()) };
    let mut v39: i32 = method3(v37, v38.clone());
    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown"); } LIT.with(|lit| lit.clone()) };
    let mut v41: US0 = method2(v1.clone(), v2.clone(), v40.clone());
    let mut v43: bool = match &v41 {
        US0::US0_0(v42) => { // Rejected
            let mut v42: Rc<str> = v42.clone();
            true
        }
        _ => {
            false
        }
    };
    let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown family"); } LIT.with(|lit| lit.clone()) };
    let mut v45: i32 = method3(v43, v44.clone());
    let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fn digest(v:&str)->bool{v.len()==64&&v.bytes().all(|b|b.is_ascii_hexdigit())} fn safe(v:&str)->bool{!v.is_empty()&&v.bytes().all(|b|b.is_ascii_alphanumeric()||matches!(b,b'.'|b'_'|b'-'|b'/'))}"); } LIT.with(|lit| lit.clone()) };
    let mut v47: Rc<str> = method1(v46.clone());
    let mut v48: US0 = method2(v47.clone(), v2.clone(), v3.clone());
    let mut v52: Rc<str> = match &v48 {
        US0::US0_1(v49) => { // Candidate
            let mut v49: Rc<str> = v49.clone();
            v49.clone()
        }
        US0::US0_0(v50) => { // Rejected
            let mut v50: Rc<str> = v50.clone();
            v33.clone()
        }
        _ => unreachable!(),
    };
    let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("safe"); } LIT.with(|lit| lit.clone()) };
    let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("receipt-text"); } LIT.with(|lit| lit.clone()) };
    let mut v55: US0 = method2(v52.clone(), v53.clone(), v54.clone());
    let mut v59: Rc<str> = match &v55 {
        US0::US0_1(v56) => { // Candidate
            let mut v56: Rc<str> = v56.clone();
            v56.clone()
        }
        US0::US0_0(v57) => { // Rejected
            let mut v57: Rc<str> = v57.clone();
            v33.clone()
        }
        _ => unreachable!(),
    };
    let mut v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lift_receipt_text \"safe\""); } LIT.with(|lit| lit.clone()) };
    let mut v61: bool = v59.contains(&*v60);
    let mut v63: bool = if v61 {
        let mut v62: bool = v59.contains(&*v10);
        v62
    } else {
        false
    };
    let mut v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("nested lifts"); } LIT.with(|lit| lit.clone()) };
    let mut v65: i32 = method3(v63, v64.clone());
    let mut v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("// rust_global \"fake function\"\n"); } LIT.with(|lit| lit.clone()) };
    let mut v67: Rc<str> = std::rc::Rc::<str>::from([&*v66, &*v1, &*v33, &*v33, &*v33].concat());
    let mut v68: US0 = method2(v67.clone(), v2.clone(), v3.clone());
    let mut v72: Rc<str> = match &v68 {
        US0::US0_1(v69) => { // Candidate
            let mut v69: Rc<str> = v69.clone();
            v69.clone()
        }
        US0::US0_0(v70) => { // Rejected
            let mut v70: Rc<str> = v70.clone();
            v33.clone()
        }
        _ => unreachable!(),
    };
    let mut v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("// rust_global"); } LIT.with(|lit| lit.clone()) };
    let mut v74: bool = v72.contains(&*v73);
    let mut v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Spiral comments"); } LIT.with(|lit| lit.clone()) };
    let mut v76: i32 = method3(v74, v75.clone());
    let mut v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("const TEXT:&str=r###\"fn digest(v:&str)->bool{false}\"###;"); } LIT.with(|lit| lit.clone()) };
    let mut v78: Rc<str> = method1(v77.clone());
    let mut v79: US0 = method2(v78.clone(), v2.clone(), v3.clone());
    let mut v81: bool = match &v79 {
        US0::US0_0(v80) => { // Rejected
            let mut v80: Rc<str> = v80.clone();
            true
        }
        _ => {
            false
        }
    };
    let mut v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("raw string isolation"); } LIT.with(|lit| lit.clone()) };
    let mut v83: i32 = method3(v81, v82.clone());
    let mut v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("// ü 路\n"); } LIT.with(|lit| lit.clone()) };
    let mut v85: Rc<str> = std::rc::Rc::<str>::from([&*v84, &*v1, &*v33, &*v33, &*v33].concat());
    let mut v86: US0 = method2(v85.clone(), v2.clone(), v3.clone());
    let mut v90: Rc<str> = match &v86 {
        US0::US0_1(v87) => { // Candidate
            let mut v87: Rc<str> = v87.clone();
            v87.clone()
        }
        US0::US0_0(v88) => { // Rejected
            let mut v88: Rc<str> = v88.clone();
            v33.clone()
        }
        _ => unreachable!(),
    };
    let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ü 路"); } LIT.with(|lit| lit.clone()) };
    let mut v92: bool = v90.contains(&*v91);
    let mut v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("UTF-8 preservation"); } LIT.with(|lit| lit.clone()) };
    let mut v94: i32 = method3(v92, v93.clone());
    let mut v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("#[cfg(test)] fn digest(v:&str)->bool{v.len()==64&&v.bytes().all(|b|b.is_ascii_hexdigit())}"); } LIT.with(|lit| lit.clone()) };
    let mut v96: Rc<str> = method1(v95.clone());
    let mut v97: US0 = method2(v96.clone(), v2.clone(), v3.clone());
    let mut v99: bool = match &v97 {
        US0::US0_0(v98) => { // Rejected
            let mut v98: Rc<str> = v98.clone();
            true
        }
        _ => {
            false
        }
    };
    let mut v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("attribute preservation"); } LIT.with(|lit| lit.clone()) };
    let mut v101: i32 = method3(v99, v100.clone());
    let mut v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust_global (typed_predicate.model.lift_sha256 /*"); } LIT.with(|lit| lit.clone()) };
    let mut v103: US0 = method2(v102.clone(), v2.clone(), v3.clone());
    let mut v105: bool = match &v103 {
        US0::US0_0(v104) => { // Rejected
            let mut v104: Rc<str> = v104.clone();
            true
        }
        _ => {
            false
        }
    };
    let mut v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("malformed wrapper comment"); } LIT.with(|lit| lit.clone()) };
    let mut v107: i32 = method3(v105, v106.clone());
    let mut v108: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("const MSG:&str=\"ü 路\"; /* nested /* fn digest() {} */ comment */ fn digest(v:&str)->bool{v.len()==64&&v.bytes().all(|b|b.is_ascii_hexdigit())}"); } LIT.with(|lit| lit.clone()) };
    let mut v109: Rc<str> = method1(v108.clone());
    let mut v110: US0 = method2(v109.clone(), v2.clone(), v3.clone());
    let mut v114: Rc<str> = match &v110 {
        US0::US0_1(v111) => { // Candidate
            let mut v111: Rc<str> = v111.clone();
            v111.clone()
        }
        US0::US0_0(v112) => { // Rejected
            let mut v112: Rc<str> = v112.clone();
            v33.clone()
        }
        _ => unreachable!(),
    };
    let mut v115: bool = v114.contains(&*v91);
    let mut v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Rust UTF-8 and nested comments"); } LIT.with(|lit| lit.clone()) };
    let mut v117: i32 = method3(v115, v116.clone());
    let mut v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fn outer(){fn digest(v:&str)->bool{v.len()==64&&v.bytes().all(|b|b.is_ascii_hexdigit())}}"); } LIT.with(|lit| lit.clone()) };
    let mut v119: Rc<str> = method1(v118.clone());
    let mut v120: US0 = method2(v119.clone(), v2.clone(), v3.clone());
    let mut v122: bool = match &v120 {
        US0::US0_0(v121) => { // Rejected
            let mut v121: Rc<str> = v121.clone();
            true
        }
        _ => {
            false
        }
    };
    let mut v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("nested function isolation"); } LIT.with(|lit| lit.clone()) };
    let mut v124: i32 = method3(v122, v123.clone());
    let mut v125: i32 = v94 + v101 ;
    let mut v126: i32 = v83 + v125 ;
    let mut v127: i32 = v76 + v126 ;
    let mut v128: i32 = v65 + v127 ;
    let mut v129: i32 = v45 + v128 ;
    let mut v130: i32 = v39 + v129 ;
    let mut v131: i32 = v32 + v130 ;
    let mut v132: i32 = v25 + v131 ;
    let mut v133: i32 = v18 + v132 ;
    let mut v134: i32 = v13 + v133 ;
    let mut v135: i32 = v117 + v124 ;
    let mut v136: i32 = v107 + v135 ;
    let mut v137: i32 = v134 + v136 ;
    let mut v138: bool = v137 == 0i32 ;
    if v138 {
        let mut v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("14 native Spiral codemod checks passed\n"); } LIT.with(|lit| lit.clone()) };
        std::io::Write::write_all(&mut std::io::stdout(),v139.as_bytes()).expect("write output");
        ()
    };
    v137
}
fn spiral_main() -> i32 {
    let mut v0: i32 = method0();
    let mut v1: i32 = native_codemod_coverage_finish(v0);
    v1
}
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let main = std::thread::Builder::new().stack_size(1 << 30).spawn(spiral_main).unwrap();
    std::process::exit(match main.join() { Ok(code) => code, Err(_) => 101 });
}
#[cfg(target_arch = "wasm32")]
fn main() {
    spiral_main();
}
