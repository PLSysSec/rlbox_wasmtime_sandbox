// use wasmtime_module::{Signature, ValueType};
// use wasmtime_runtime::{InstanceHandle, MmapRegion};
// use wasmtime_runtime_internals::val::Val;
use wasmtime::*;
use wasmtime_wasi::Wasi;

use std::convert::TryFrom;
use std::ffi::c_void;
use std::sync::Arc;

pub struct WasmtimeSandboxInstance {
    pub instance: Instance,
    pub store: Store,
    pub linker: Linker,
    pub wasi: Wasi,
    pub module: Module,

    // pub region: Arc<MmapRegion>,
    // pub instance_handle: InstanceHandle,
    // pub signatures: Vec<Signature>,
}

#[derive(Debug)]
pub enum Error {
    // GoblinError(goblin::error::Error),
    // WasiError(failure::Error),
    // WasmtimeRuntimeError(wasmtime_runtime_internals::error::Error),
    // WasmtimeModuleError(wasmtime_module::Error),
}

// impl From<goblin::error::Error> for Error {
//     fn from(e: goblin::error::Error) -> Self {
//         Error::GoblinError(e)
//     }
// }

// impl From<failure::Error> for Error {
//     fn from(e: failure::Error) -> Self {
//         Error::WasiError(e)
//     }
// }

// impl From<wasmtime_runtime_internals::error::Error> for Error {
//     fn from(e: wasmtime_runtime_internals::error::Error) -> Self {
//         Error::WasmtimeRuntimeError(e)
//     }
// }

// impl From<wasmtime_module::Error> for Error {
//     fn from(e: wasmtime_module::Error) -> Self {
//         Error::WasmtimeModuleError(e)
//     }
// }

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum WasmtimeValueType {
    I32,
    I64,
    F32,
    F64,
    Void,
}

// impl From<Option<ValueType>> for WasmtimeValueType {
//     fn from(value_type: Option<ValueType>) -> Self {
//         match value_type {
//             Some(value_type_val) => match value_type_val {
//                 ValueType::I32 => WasmtimeValueType::I32,
//                 ValueType::I64 => WasmtimeValueType::I64,
//                 ValueType::F32 => WasmtimeValueType::F32,
//                 ValueType::F64 => WasmtimeValueType::F64,
//             },
//             _ => WasmtimeValueType::Void,
//         }
//     }
// }

// impl From<ValueType> for WasmtimeValueType {
//     fn from(value_type: ValueType) -> Self {
//         match value_type {
//             ValueType::I32 => WasmtimeValueType::I32,
//             ValueType::I64 => WasmtimeValueType::I64,
//             ValueType::F32 => WasmtimeValueType::F32,
//             ValueType::F64 => WasmtimeValueType::F64,
//         }
//     }
// }

// impl Into<Option<ValueType>> for WasmtimeValueType {
//     fn into(self) -> Option<ValueType> {
//         match self {
//             WasmtimeValueType::I32 => Some(ValueType::I32),
//             WasmtimeValueType::I64 => Some(ValueType::I64),
//             WasmtimeValueType::F32 => Some(ValueType::F32),
//             WasmtimeValueType::F64 => Some(ValueType::F64),
//             _ => None,
//         }
//     }
// }

// impl Into<ValueType> for WasmtimeValueType {
//     fn into(self) -> ValueType {
//         match self {
//             WasmtimeValueType::I32 => ValueType::I32,
//             WasmtimeValueType::I64 => ValueType::I64,
//             WasmtimeValueType::F32 => ValueType::F32,
//             WasmtimeValueType::F64 => ValueType::F64,
//             _ => panic!("Unexpected!"),
//         }
//     }
// }

#[repr(C)]
#[derive(Clone, Copy)]
pub union WasmtimeValueUnion {
    as_u32: u32,
    as_u64: u64,
    as_i32: i32,
    as_i64: i64,
    as_f32: f32,
    as_f64: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct WasmtimeValue {
    val_type: WasmtimeValueType,
    val : WasmtimeValueUnion,
}

impl From<&WasmtimeValue> for Val {
    fn from(val: &WasmtimeValue) -> Val {
        match val.val_type {
            WasmtimeValueType::I32 => Val::I32(unsafe { val.val.as_i32 }),
            WasmtimeValueType::I64 => Val::I64(unsafe { val.val.as_i64 }),
            WasmtimeValueType::F32 => Val::F32(unsafe { val.val.as_u32 }),
            WasmtimeValueType::F64 => Val::F64(unsafe { val.val.as_u64 }),
            _ => panic!("Unexpected!"),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct WasmtimeFunctionSignature {
    ret: WasmtimeValueType,
    parameter_cnt: u32,
    parameters: *mut WasmtimeValueType,
}

// impl Into<Signature> for WasmtimeFunctionSignature {
//     fn into(self) -> Signature {
//         let len = usize::try_from(self.parameter_cnt).unwrap();
//         let vec = unsafe { Vec::from_raw_parts(self.parameters, len, len) };
//         let p: Vec<ValueType> = vec.iter().map(|x| x.clone().into()).collect();
//         std::mem::forget(vec);
//         Signature {
//             params: p,
//             ret_ty: self.ret.into(),
//         }
//     }
// }

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SizedBuffer {
    pub data: *mut c_void,
    pub length: usize,
}
