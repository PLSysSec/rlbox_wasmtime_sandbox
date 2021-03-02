use crate::types::{WasmtimeFunctionSignature, WasmtimeSandboxInstance, WasmtimeValue};

use wasmtime::*;

use std::ffi::{c_void, CStr};
use std::os::raw::{c_uint, c_char};

#[no_mangle]
pub extern "C" fn wasmtime_register_callback(
    inst_ptr: *mut c_void,
    csig: WasmtimeFunctionSignature,
    func_ptr: unsafe extern "C" fn(c_uint, *mut WasmtimeValue) -> WasmtimeValue,
) -> u32 {
    let inst = unsafe { &mut *(inst_ptr as *mut WasmtimeSandboxInstance) };
    let conv_sig: FuncType = csig.into();
    let fw = Func::new(&inst.store, conv_sig, move |_caller, params, results| {
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
    let existing_slot = inst.free_callback_slots.pop();
    let fref = match existing_slot {
        Some(slot) => {
            t.set(slot, Val::FuncRef(Some(fw))).unwrap();
            slot
        },
        _ => {
            t.grow(1, Val::FuncRef(Some(fw))).unwrap()
        }
    };
    fref
}

#[no_mangle]
pub extern "C" fn wasmtime_unregister_callback(
    inst_ptr: *mut c_void,
    slot: u32,
) {
    let inst = unsafe { &mut *(inst_ptr as *mut WasmtimeSandboxInstance) };
    let t = inst.instance.get_table("__indirect_function_table").expect("table");
    t.set(slot, Val::FuncRef(None)).unwrap();
    inst.free_callback_slots.push(slot);
}

#[no_mangle]
pub extern "C" fn wasmtime_register_internal_callback(
    inst_ptr: *mut c_void,
    csig: WasmtimeFunctionSignature,
    func_ptr: *mut c_void,
) -> u32 {
    let inst = unsafe { &mut *(inst_ptr as *mut WasmtimeSandboxInstance) };
    let conv_sig: FuncType = csig.into();

    let func_charp : *const c_char = func_ptr as *const c_char;
    let func = unsafe {
        CStr::from_ptr(func_charp)
            .to_string_lossy()
            .into_owned()
    };
    let fi = inst.instance.get_func(&func).unwrap();

    let fw = Func::new(&inst.store, conv_sig, move |_caller, params, results| {
        let ret = fi.call(params).unwrap();
        let v = (*ret).first().map(|a| a.clone());
        results[0] = v.unwrap();
        Ok(())
    });

    let t = inst.instance.get_table("__indirect_function_table").expect("table");
    let fref = t.grow(1, Val::FuncRef(Some(fw))).unwrap();
    fref
}

// #[no_mangle]
// pub extern "C" fn wasmtime_lookup_callback(
//     inst_ptr: *mut c_void,
//     slot: u32,
// ) -> *const c_void {
//     let inst = unsafe { &mut *(inst_ptr as *mut WasmtimeSandboxInstance) };
//     let t = inst.instance.get_table("__indirect_function_table").expect("table");
//     let v = t.get(slot).unwrap();
//     let r = match v {
//         Val::ExternRef(e /* : Option<ExternRef> */) => {
//             let val = e.unwrap().data();
//             panic!("Todo");
//         }
//         Val::FuncRef(f /*: Option<Func>*/) => {
//             let val = e.unwrap();

//         }
//         _ => panic!("Unknown callback element data")
//     };
//     panic!("Todo");
// }