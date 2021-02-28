// use wasmtime::*;
use wasi_common::*;
use wasi_common::snapshots::preview_1::wasi_snapshot_preview1::WasiSnapshotPreview1 as Snapshot1;

use wiggle::*;

#[no_mangle]
pub extern "C" fn _wasm_function___wasi_args_get<'b>(
    ctx: &WasiCtx,
    argv: &GuestPtr<'b, GuestPtr<'b, u8>>,
    argv_buf: &GuestPtr<'b, u8>,
) {
    Snapshot1::args_get(ctx, argv, argv_buf).unwrap()
}

// #[no_mangle]
// pub extern "C" fn _wasm_function___wasi_args_sizes_get<'b>(
//     ctx: &WasiCtx,
// ) -> (wiggle::types::Size, wiggle::types::Size) {
//     Snapshot1::args_sizes_get(ctx).unwrap()
// }

// #[no_mangle]
// pub extern "C" fn _wasm_function___wasi_proc_exit(
//     ctx: &WasiCtx,
//     status: wiggle::types::Exitcode
// ) -> wiggle::Trap {
//     Snapshot1::proc_exit(ctx, status)
// }

#[no_mangle]
pub extern "C" fn _wasm_function___wasi_args_sizes_get(_ctx: &WasiCtx) {
    panic!("Not implemented")
}

#[no_mangle]
pub extern "C" fn _wasm_function___wasi_proc_exit(_ctx: &WasiCtx) {
    panic!("Not implemented")
}

#[no_mangle]
pub extern "C" fn _wasm_function___wasi_fd_close(_ctx: &WasiCtx) {
    panic!("Not implemented")
}

#[no_mangle]
pub extern "C" fn _wasm_function___wasi_fd_fdstat_get(_ctx: &WasiCtx) {
    panic!("Not implemented")
}

#[no_mangle]
pub extern "C" fn _wasm_function___wasi_fd_seek(_ctx: &WasiCtx) {
    panic!("Not implemented")
}

#[no_mangle]
pub extern "C" fn _wasm_function___wasi_fd_write(_ctx: &WasiCtx) {
    panic!("Not implemented")
}
