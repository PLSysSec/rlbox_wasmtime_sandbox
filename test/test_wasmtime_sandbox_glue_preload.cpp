#define RLBOX_USE_EXCEPTIONS
#define RLBOX_ENABLE_DEBUG_ASSERTIONS
#define RLBOX_SINGLE_THREADED_INVOCATIONS
#include "rlbox_wasmtime_sandbox.hpp"

// NOLINTNEXTLINE
#define TestName "rlbox_wasmtime_sandbox preload"
// NOLINTNEXTLINE
#define TestType rlbox::rlbox_wasmtime_sandbox

#ifndef GLUE_LIB_WASMTIME_PATH
#  error "Missing definition for GLUE_LIB_WASMTIME_PATH"
#endif

// Test simulating the application preloading the sandboxed library
// Applications such as Firefox sometimes preload dynamic libraries to ensure it
// plays well with the content (filepaths) sandbox

#include <dlfcn.h>

#undef CreateSandbox
// NOLINTNEXTLINE
#define CreateSandbox(sandbox)                                                 \
  dlopen(GLUE_LIB_WASMTIME_PATH, RTLD_LAZY);                                      \
  (sandbox).create_sandbox(                                                    \
    GLUE_LIB_WASMTIME_PATH, true /* external loads */, false /* allow_stdio */)
#include "test_sandbox_glue.inc.cpp"
