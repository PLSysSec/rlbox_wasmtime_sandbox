use crate::types::WasmtimeSandboxInstance;

use wasmtime::*;
use wasmtime_wasi::Wasi;
use wasi_cap_std_sync::WasiCtxBuilder;

use std::ffi::{c_void, CStr};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn wasmtime_load_module(wasmtime_module_path: *const c_char, _allow_stdio: bool) -> *mut c_void {
    let module_path = unsafe {
        CStr::from_ptr(wasmtime_module_path)
            .to_string_lossy()
            .into_owned()
    };
    let store = Store::default();
    let mut linker = Linker::new(&store);
    let wasi = Wasi::new(
        &store,
        WasiCtxBuilder::new()
            .inherit_stdio()
            .inherit_args().unwrap()
            .build().unwrap(),
    );
    wasi.add_to_linker(&mut linker).unwrap();

    let module = Module::from_file(store.engine(), module_path).unwrap();
    let instance = linker.instantiate(&module).unwrap();

    let inst = WasmtimeSandboxInstance {
        instance,
        store,
        linker,
        wasi,
        module,
    };
    let r = Box::into_raw(Box::new(inst)) as *mut c_void;
    return r;
}

#[no_mangle]
pub extern "C" fn wasmtime_drop_module(inst_ptr: *mut c_void) {
    // Need to invoke the destructor
    let _inst = unsafe {
        Box::from_raw(inst_ptr as *mut WasmtimeSandboxInstance)
    };
}

