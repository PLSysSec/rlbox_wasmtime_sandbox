use crate::types::{WasmtimeFunctionSignature, WasmtimeSandboxInstance, WasmtimeValue};

use wasmtime::*;

use std::ffi::c_void;
use std::os::raw::{c_uint};

#[no_mangle]
pub extern "C" fn wasmtime_register_callback(
    inst_ptr: *mut c_void,
    csig: WasmtimeFunctionSignature,
    func_ptr: unsafe extern "C" fn(c_uint, *mut WasmtimeValue) -> WasmtimeValue,
) -> u32 {
    let inst = unsafe { &mut *(inst_ptr as *mut WasmtimeSandboxInstance) };
    let conv_sig: FuncType = csig.into();
    let f = Func::new(&inst.store, conv_sig, move |_caller, params, results| {
        let mut params: Vec<WasmtimeValue> = params
            .iter()
            .map(|p| p.clone().into())
            .collect::<Vec<_>>();

        let params_len = params.len() as c_uint;
        let params_data = params.as_mut_ptr() as *mut WasmtimeValue;

        let r = unsafe { func_ptr(params_len, params_data) };
        results[0] = (&r).into();

        Ok(())
    });

    let t = inst.instance.get_table("__indirect_function_table").expect("table");
    let fref = t.grow(1, Val::FuncRef(Some(f))).unwrap();
    fref
}

#[no_mangle]
pub extern "C" fn wasmtime_unregister_callback(
    _inst_ptr: *mut c_void,
    _slot: u32,
) {
    // let inst = unsafe { &mut *(inst_ptr as *mut WasmtimeSandboxInstance) };
    // let t = inst.instance.get_table("__indirect_function_table").expect("table");
    println!("!!!!!!!!!!Unregister callback not implemented!!!!!!!");

}