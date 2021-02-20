use crate::types::{WasmtimeFunctionSignature, WasmtimeSandboxInstance, SizedBuffer};

// use wasmtime_module::Signature;
// use wasmtime_runtime_internals::instance::InstanceInternal;

use std::convert::TryFrom;
use std::ffi::c_void;

#[no_mangle]
pub extern "C" fn wasmtime_get_reserved_callback_slot_val(
    inst_ptr: *mut c_void,
    slot_number: u32,
) -> usize {
    panic!("Not implemented");
    // let inst = unsafe { &mut *(inst_ptr as *mut WasmtimeSandboxInstance) };

    // let name = format!("sandboxReservedCallbackSlot{}", slot_number);
    // let func = inst
    //     .instance_handle
    //     .module()
    //     .get_export_func(&name)
    //     .unwrap();
    // return func.ptr.as_usize();
}

#[no_mangle]
pub extern "C" fn wasmtime_get_function_pointer_table(inst_ptr: *mut c_void) -> SizedBuffer {
    panic!("Not implemented");
    // let inst = unsafe { &mut *(inst_ptr as *mut WasmtimeSandboxInstance) };

    // let elems = inst.instance_handle.module().table_elements().unwrap();

    // SizedBuffer {
    //     data: elems.as_ptr() as *mut c_void,
    //     length: elems.len(),
    // }
}

#[no_mangle]
pub extern "C" fn wasmtime_get_function_type_index(
    inst_ptr: *mut c_void,
    csig: WasmtimeFunctionSignature,
) -> i32 {
    panic!("Not implemented");
    // let inst = unsafe { &mut *(inst_ptr as *mut WasmtimeSandboxInstance) };

    // let conv_sig: Signature = csig.into();
    // let index = inst.signatures.iter().position(|r| *r == conv_sig);

    // match index {
    //     Some(x) => i32::try_from(x).unwrap_or(-1),
    //     _ => -1,
    // }
}
