cmake_minimum_required(VERSION 3.13)

# Mac adds extra flags 
set(HAVE_FLAG_SEARCH_PATHS_FIRST 0)

project(rlbox_wasmtime_testlib
        VERSION 0.1
        DESCRIPTION "RLBox integration with WASM modules compiled with wasmtime")

if(NOT DEFINED ENV{rlbox_SOURCE_DIR})
  message(FATAL_ERROR "Set rlbox_SOURCE_DIR environment variable")
endif()

if(NOT DEFINED ENV{wasiclang_SOURCE_DIR})
  message(FATAL_ERROR "Set wasiclang_SOURCE_DIR environment variable")
endif()

if(NOT DEFINED ENV{WASMTIME_WASI_DIR})
  message(FATAL_ERROR "Set WASMTIME_WASI_DIR environment variable")
endif()

if(NOT DEFINED ENV{WASMTIME_DIR})
  message(FATAL_ERROR "Set WASMTIME_DIR environment variable")
endif()

set(rlbox_SOURCE_DIR $ENV{rlbox_SOURCE_DIR})
set(wasiclang_SOURCE_DIR $ENV{wasiclang_SOURCE_DIR})
set(WASMTIME_WASI_DIR $ENV{WASMTIME_WASI_DIR})
set(WASMTIME_DIR $ENV{WASMTIME_DIR})

set(CMAKE_C_COMPILER ${wasiclang_SOURCE_DIR}/bin/clang)
set(CMAKE_CXX_COMPILER ${wasiclang_SOURCE_DIR}/bin/clang++)
set(CMAKE_BUILD_TYPE Release)

# Apply settings suitable for wasm module compilation
set(CMAKE_C_FLAGS
    "--sysroot ${wasiclang_SOURCE_DIR}/share/wasi-sysroot/")
# Link flags are set by default on Mac - clearing this
set(CMAKE_C_LINK_FLAGS "")
set(CMAKE_EXE_LINKER_FLAGS "-Wl,--export-all,--export-table,--growable-table")

add_executable(glue_lib_wasmtime.wasm
               wasmtime_sandbox_wrapper.c
               ${rlbox_SOURCE_DIR}/code/tests/rlbox_glue/lib/libtest.c)

if(MSVC)
  set(OBJ_EXT "obj")
else()
  set(OBJ_EXT "o")
endif()

set(GLUE_LIB_PATH "${CMAKE_BINARY_DIR}/glue_lib_wasmtime.${OBJ_EXT}")

add_custom_command(OUTPUT ${GLUE_LIB_PATH}
                   DEPENDS glue_lib_wasmtime.wasm
                   COMMAND ${WASMTIME_DIR}/wasmtime
                           wasm2obj
                           glue_lib_wasmtime.wasm
                           ${GLUE_LIB_PATH}
                   COMMENT "Compiling wasm file to native")

add_custom_target(glue_lib_obj ALL DEPENDS ${GLUE_LIB_PATH})
