use crate::types::{WasmtimeSandboxInstance, WasmtimeValue};

use wasmtime::*;

use std::convert::TryInto;
use std::ffi::{c_void, CStr};
use std::os::raw::{c_char, c_int};

#[no_mangle]
pub extern "C" fn wasmtime_run_function_return_void(
    inst_ptr: *mut c_void,
    func_ptr: *mut c_void,
    argc: c_int,
    argv: *mut WasmtimeValue,
) {
    wasmtime_run_function_helper(inst_ptr, func_ptr, argc, argv);
}

#[no_mangle]
pub extern "C" fn wasmtime_run_function_return_u32(
    inst_ptr: *mut c_void,
    func_ptr: *mut c_void,
    argc: c_int,
    argv: *mut WasmtimeValue,
) -> u32 {
    let ret = wasmtime_run_function_helper(inst_ptr, func_ptr, argc, argv);
    return ret.unwrap().unwrap_i32() as u32;
}

#[no_mangle]
pub extern "C" fn wasmtime_run_function_return_u64(
    inst_ptr: *mut c_void,
    func_ptr: *mut c_void,
    argc: c_int,
    argv: *mut WasmtimeValue,
) -> u64 {
    let ret = wasmtime_run_function_helper(inst_ptr, func_ptr, argc, argv);
    return ret.unwrap().unwrap_i64() as u64;
}

#[no_mangle]
pub extern "C" fn wasmtime_run_function_return_f32(
    inst_ptr: *mut c_void,
    func_ptr: *mut c_void,
    argc: c_int,
    argv: *mut WasmtimeValue,
) -> f32 {
    let ret = wasmtime_run_function_helper(inst_ptr, func_ptr, argc, argv);
    return ret.unwrap().unwrap_f32();
}

#[no_mangle]
pub extern "C" fn wasmtime_run_function_return_f64(
    inst_ptr: *mut c_void,
    func_ptr: *mut c_void,
    argc: c_int,
    argv: *mut WasmtimeValue,
) -> f64 {
    let ret = wasmtime_run_function_helper(inst_ptr, func_ptr, argc, argv);
    return ret.unwrap().unwrap_f64();
}

fn wasmtime_run_function_helper(
    inst_ptr: *mut c_void,
    func_ptr: *mut c_void,
    argc: c_int,
    argv: *mut WasmtimeValue,
) -> Option<Val> {
    let inst = unsafe { &mut *(inst_ptr as *mut WasmtimeSandboxInstance) };
    let func_charp : *const c_char = func_ptr as *const c_char;
    let func = unsafe {
        CStr::from_ptr(func_charp)
            .to_string_lossy()
            .into_owned()
    };
    let f = inst.instance.get_func(&func).unwrap();
    if argc != f.param_arity().try_into().unwrap() {
        panic!("Wrong number of arguments");
    }

    let args = if argc == 0 {
        vec![]
    } else {
        unsafe { std::slice::from_raw_parts(argv, argc as usize) }
            .into_iter()
            .map(|v| v.into())
            .collect()
    };

    let ret = f.call(&args).unwrap();
    let v = (*ret).first().map(|a| a.clone());
    return v;
}