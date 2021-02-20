use crate::types::{Error, WasmtimeSandboxInstance};

use anyhow::Result;
use wasmtime::*;
use wasmtime_wasi::Wasi;
use wasi_cap_std_sync::WasiCtxBuilder;

use std::ffi::{c_void, CStr};
use std::os::raw::c_char;
use std::ptr;
use std::sync::Arc;
use std::mem;
use std::alloc::{self, Layout};
use std::convert::TryFrom;

// #[no_mangle]
// pub extern "C" fn wasmtime_ensure_linked() -> Result<()> {
    // wasmtime_runtime::wasmtime_internal_ensure_linked();
    // wasmtime_wasi::hostcalls::ensure_linked();
// }

#[no_mangle]
pub extern "C" fn wasmtime_load_module(wasmtime_module_path: *const c_char, allow_stdio: bool) -> *mut c_void {
    // let module_path = unsafe {
    //     CStr::from_ptr(wasmtime_module_path)
    //         .to_string_lossy()
    //         .into_owned()
    // };
    let module_path = "/home/shr/Code/LibrarySandboxing/rlbox_wasmtime_sandbox/build_debug/wasm/glue_lib_wasmtime.wasm";
    print!("VAL: {}", module_path);
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
    linker.module("", &module).unwrap();
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

// fn wasmtime_load_module_helper(module_path: &String, allow_stdio: bool) -> Result<WasmtimeSandboxInstance, Error> {
//     panic!("Not implemented");
    // let module = DlModule::load(module_path)?;

    // //Replicating calculations used in wasmtime examples
    // let min_globals_size = module.globals().len() * std::mem::size_of::<u64>();
    // // Nearest 4k
    // let globals_size = ((min_globals_size + 4096 - 1) / 4096) * 4096;

    // let region = MmapRegion::create_aligned(
    //     1,
    //     &Limits {
    //         heap_memory_size: 4 * 1024 * 1024 * 1024,        // 4GB
    //         heap_address_space_size: 8 * 1024 * 1024 * 1024, // 8GB
    //         stack_size: 8 * 1024 * 1024,                     // 8MB - pthread default
    //         globals_size: globals_size,
    //         ..Limits::default()
    //     },
    //     4 * 1024 * 1024 * 1024,                              // 4GB heap alignment
    // )?;

    // let sig = module.get_signatures().to_vec();

    // // put the path to the module on the front for argv[0]
    // let mut builder = WasiCtxBuilder::new()
    //     .args(&[&module_path]);

    // if allow_stdio {
    //     builder = builder.inherit_stdio_no_syscall();
    // }

    // let ctx = builder.build()?;

    // let instance_handle = region
    //     .new_instance_builder(module as Arc<dyn Module>)
    //     .with_embed_ctx(ctx)
    //     .build()?;

    // let opaque_instance = WasmtimeSandboxInstance {
    //     region: region,
    //     instance_handle: instance_handle,
    //     signatures: sig,
    // };

    // return Ok(opaque_instance);
// }
