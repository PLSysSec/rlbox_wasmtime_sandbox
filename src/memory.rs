use crate::types::WasmtimeSandboxInstance;

use std::ffi::c_void;

#[no_mangle]
pub extern "C" fn wasmtime_get_heap_base(inst_ptr: *mut c_void) -> *mut c_void {
    let inst = unsafe { &mut *(inst_ptr as *mut WasmtimeSandboxInstance) };
    let m = inst.instance.get_memory("memory").unwrap();
    let heap_base = m.data_ptr() as *mut c_void;
    return heap_base;
}

#[no_mangle]
pub extern "C" fn wasmtime_get_heap_size(inst_ptr: *mut c_void) -> usize {
    let inst = unsafe { &mut *(inst_ptr as *mut WasmtimeSandboxInstance) };
    let m = inst.instance.get_memory("memory").unwrap();
    let heap_size = m.data_size();
    return heap_size;
}
