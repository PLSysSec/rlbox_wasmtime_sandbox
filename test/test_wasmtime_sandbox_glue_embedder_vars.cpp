#define RLBOX_USE_EXCEPTIONS
#define RLBOX_ENABLE_DEBUG_ASSERTIONS
#define RLBOX_SINGLE_THREADED_INVOCATIONS
#define RLBOX_EMBEDDER_PROVIDES_TLS_STATIC_VARIABLES
#include "rlbox_wasmtime_sandbox.hpp"
RLBOX_WASMTIME_SANDBOX_STATIC_VARIABLES();

// NOLINTNEXTLINE
#define TestName "rlbox_wasmtime_sandbox embedder"
// NOLINTNEXTLINE
#define TestType rlbox::rlbox_wasmtime_sandbox

#ifndef GLUE_LIB_WASMTIME_PATH
#  error "Missing definition for GLUE_LIB_WASMTIME_PATH"
#endif

// NOLINTNEXTLINE
#define CreateSandbox(sandbox) sandbox.create_sandbox(GLUE_LIB_WASMTIME_PATH)
#include "test_sandbox_glue.inc.cpp"
