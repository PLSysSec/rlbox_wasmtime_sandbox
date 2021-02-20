#ifndef WASMTIME_SANDBOX_H
#define WASMTIME_SANDBOX_H

#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif

struct WasmtimeSandboxInstance;
typedef struct WasmtimeSandboxInstance WasmtimeSandboxInstance;

enum WasmtimeValueType {
  WasmtimeValueType_I32,
  WasmtimeValueType_I64,
  WasmtimeValueType_F32,
  WasmtimeValueType_F64,
  WasmtimeValueType_Void
};

typedef struct {
  enum WasmtimeValueType val_type;
  union {
    uint32_t u32;
    uint64_t u64;
    float f32;
    double f64;
  };
} WasmtimeValue;

typedef struct {
  enum WasmtimeValueType ret;
  uint32_t parameter_cnt;
  WasmtimeValueType *parameters;
} WasmtimeFunctionSignature;

typedef struct {
  uint64_t ty;
  uint64_t rf;
} WasmtimeFunctionTableElement;

typedef struct {
  WasmtimeFunctionTableElement *data;
  size_t length;
} WasmtimeFunctionTable;

void wasmtime_ensure_linked();
WasmtimeSandboxInstance *wasmtime_load_module(const char *wasmtime_module_path,
                                        bool allow_stdio);
void wasmtime_drop_module(WasmtimeSandboxInstance *inst);

void* wasmtime_lookup_function(WasmtimeSandboxInstance *inst,
                            const char *fn_name);
void wasmtime_set_curr_instance(WasmtimeSandboxInstance *inst);
void wasmtime_clear_curr_instance(WasmtimeSandboxInstance *inst);


void wasmtime_run_function_return_void(WasmtimeSandboxInstance *inst,
                                    void* func_ptr, int argc,
                                    WasmtimeValue *argv);
uint32_t wasmtime_run_function_return_u32(WasmtimeSandboxInstance *inst,
                                       void* func_ptr, int argc,
                                       WasmtimeValue *argv);
uint64_t wasmtime_run_function_return_u64(WasmtimeSandboxInstance *inst,
                                       void* func_ptr, int argc,
                                       WasmtimeValue *argv);
float wasmtime_run_function_return_f32(WasmtimeSandboxInstance *inst,
                                    void* func_ptr, int argc,
                                    WasmtimeValue *argv);
double wasmtime_run_function_return_f64(WasmtimeSandboxInstance *inst,
                                     void* func_ptr, int argc,
                                     WasmtimeValue *argv);

uintptr_t wasmtime_get_reserved_callback_slot_val(void *inst,
                                               uint32_t slot_number);
WasmtimeFunctionTable wasmtime_get_function_pointer_table(void *inst);
int32_t wasmtime_get_function_type_index(void *inst, WasmtimeFunctionSignature csig);

void *wasmtime_get_heap_base(WasmtimeSandboxInstance *inst);
size_t wasmtime_get_heap_size(WasmtimeSandboxInstance *inst);
uint32_t wasmtime_get_export_function_id(void *inst, void *unsandboxed_ptr);

#ifdef __cplusplus
}
#endif

#endif