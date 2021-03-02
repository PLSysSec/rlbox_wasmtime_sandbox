use wasmtime::*;
use wasmtime_wasi::Wasi;

use std::convert::TryFrom;
use std::ffi::c_void;

pub struct WasmtimeSandboxInstance {
    pub instance: Instance,
    pub store: Store,
    pub linker: Linker,
    pub wasi: Wasi,
    pub module: Module,
    pub free_callback_slots: Vec<u32>,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum WasmtimeValueType {
    I32,
    I64,
    F32,
    F64,
    Void,
}

impl From<Option<ValType>> for WasmtimeValueType {
    fn from(value_type: Option<ValType>) -> Self {
        match value_type {
            Some(value_type_val) => match value_type_val {
                ValType::I32 => WasmtimeValueType::I32,
                ValType::I64 => WasmtimeValueType::I64,
                ValType::F32 => WasmtimeValueType::F32,
                ValType::F64 => WasmtimeValueType::F64,
                _ => panic!("Unexpected!"),
            },
            _ => WasmtimeValueType::Void,
        }
    }
}

impl From<ValType> for WasmtimeValueType {
    fn from(value_type: ValType) -> Self {
        Some(value_type).into()
    }
}

impl Into<ValType> for WasmtimeValueType {
    fn into(self) -> ValType {
        match self {
            WasmtimeValueType::I32 => ValType::I32,
            WasmtimeValueType::I64 => ValType::I64,
            WasmtimeValueType::F32 => ValType::F32,
            WasmtimeValueType::F64 => ValType::F64,
            _ => panic!("Unexpected!"),
        }
    }
}

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

impl Into<WasmtimeValue> for Val {
    fn into(self) -> WasmtimeValue {
        match self {
            Val::I32(i) => WasmtimeValue { val_type: self.ty().into(), val: WasmtimeValueUnion { as_i32 :  i } },
            Val::I64(i) => WasmtimeValue { val_type: self.ty().into(), val: WasmtimeValueUnion { as_i64 :  i } },
            Val::F32(_) => WasmtimeValue { val_type: self.ty().into(), val: WasmtimeValueUnion { as_f32 :  self.unwrap_f32() } },
            Val::F64(_) => WasmtimeValue { val_type: self.ty().into(), val: WasmtimeValueUnion { as_f64 :  self.unwrap_f64() } },
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

impl Into<FuncType> for WasmtimeFunctionSignature {
    fn into(self) -> FuncType {
        let len = usize::try_from(self.parameter_cnt).unwrap();
        let vec = unsafe { Vec::from_raw_parts(self.parameters, len, len) };
        let p: Vec<ValType> = vec.iter().map(|x| x.clone().into()).collect();
        std::mem::forget(vec);
        let r: Vec<ValType>  = [ self.ret.into() ].to_vec();
        let f = FuncType::new(p, r);
        return f;
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SizedBuffer {
    pub data: *mut c_void,
    pub length: usize,
}
