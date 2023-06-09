set(CARGO_ENV_VARS "${CARGO_ENV_VARS} SHADOW_VERSION=\"${SHADOW_VERSION_STRING_CONF}\"")

if(SHADOW_USE_PERF_TIMERS STREQUAL ON)
  set(RUST_FEATURES "${RUST_FEATURES} perf_timers")
endif()

# Propagate global C and C++ flags to the Rust build
set(RUST_CFLAGS "${CMAKE_C_FLAGS}")
set(RUST_CXXFLAGS "${CMAKE_CXX_FLAGS}")
get_property(COMPILE_OPTIONS DIRECTORY . PROPERTY COMPILE_OPTIONS)
foreach(FLAG IN LISTS COMPILE_OPTIONS COMPILE_DEFINITIONS)
  set(RUST_CFLAGS "${RUST_CFLAGS} ${FLAG}")
  set(RUST_CXXFLAGS "${RUST_CXXFLAGS} ${FLAG}")
endforeach()
get_property(COMPILE_DEFINITIONS DIRECTORY . PROPERTY COMPILE_DEFINITIONS)
foreach(DEFINITION IN LISTS COMPILE_DEFINITIONS)
  set(RUST_CFLAGS "${RUST_CFLAGS} -D${DEFINITION}")
  set(RUST_CXXFLAGS "${RUST_CXXFLAGS} -D${DEFINITION}")
endforeach()

set(RUST_TARGETS "--lib --bins")
if(SHADOW_TEST STREQUAL ON)
  set(RUST_TARGETS "${RUST_TARGETS} --tests")
endif()

if(SHADOW_COVERAGE STREQUAL ON)
    # https://github.com/shadow/shadow/issues/1236
    set(RUSTFLAGS "${RUSTFLAGS} --remap-path-prefix \"=${CMAKE_CURRENT_SOURCE_DIR}/\"")

    ## from https://github.com/mozilla/grcov
    set(RUSTFLAGS "${RUSTFLAGS} -Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code \
                -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort")
    set(CARGO_ENV_VARS "${CARGO_ENV_VARS} CARGO_INCREMENTAL=0 RUSTDOCFLAGS=\"-Cpanic=abort\"")

    # Use an explicit target to workaround incompatibility of -Cpanic=abort with
    # procmacro crates. See
    # https://github.com/intellij-rust/intellij-rust/issues/5973#issuecomment-684867383
    #
    # Avoid using in non-coverage builds in case it inadvertently disables
    # native CPU features, though we haven't confirmed whether it does or not.
    set(RUST_TARGET "x86_64-unknown-linux-gnu")
    set(RUST_TARGET_FLAG "--target=${RUST_TARGET}")
else()
    # With no explicit target, cargo will build for the host platform.
    set(RUST_TARGET "")
    set(RUST_TARGET_FLAG "")
endif()

## build the rust library
set(CARGO_ENV_VARS "${CARGO_ENV_VARS} RUSTFLAGS=\"${RUSTFLAGS}\"")
set(CARGO_ENV_VARS "${CARGO_ENV_VARS} CFLAGS=\"${RUST_CFLAGS}\"")
set(CARGO_ENV_VARS "${CARGO_ENV_VARS} CXXFLAGS=\"${RUST_CXXFLAGS}\"")
set(CARGO_ENV_VARS "${CARGO_ENV_VARS} CC=\"${CMAKE_C_COMPILER}\"")
set(CARGO_ENV_VARS "${CARGO_ENV_VARS} CXX=\"${CMAKE_CXX_COMPILER}\"")
set(TARGETDIR "${CMAKE_CURRENT_BINARY_DIR}/target")
include(ExternalProject)
ExternalProject_Add(
    rust-workspace-project
    PREFIX ${CMAKE_CURRENT_BINARY_DIR}
    BINARY_DIR ${CMAKE_CURRENT_SOURCE_DIR}
    BUILD_ALWAYS 1
    DOWNLOAD_COMMAND ""
    CONFIGURE_COMMAND ""
    BUILD_COMMAND bash -c " \
        ${CARGO_ENV_VARS} \
        cargo build \
        --workspace --exclude shadow-tests \
        ${RUST_TARGET_FLAG} \
        ${RUST_BUILD_FLAG} \
        ${RUST_TARGETS} \
        --target-dir \"${TARGETDIR}\" \
        --features \"${RUST_FEATURES}\" \
        "
    # always build shadow-tests with the debug profile, even when shadow is built in release mode.
    # Omit RUST_FEATURES, since the shadow-tests package doesn't define any of the optional features
    # that we trigger via this wrapper.
    COMMAND bash -c " \
        ${CARGO_ENV_VARS} \
        cargo build \
        --package shadow-tests \
        --target-dir \"${TARGETDIR}\" \
        "
    BUILD_BYPRODUCTS
      ${TARGETDIR}/debug/${RUST_TARGET}/libshadow_shim_helper_rs.a ${TARGETDIR}/release/${RUST_TARGET}/libshadow_shim_helper_rs.a
    INSTALL_COMMAND ""
    LOG_BUILD OFF
)
foreach(LIBRARY shadow-shim-helper-rs logger shadow-shmem shadow-tsc shadow-rs zermia_lib)
  add_library(${LIBRARY} STATIC IMPORTED GLOBAL)
  add_dependencies(${LIBRARY} rust-workspace-project)
  string(REPLACE "-" "_" LIBRARY_FILE "lib${LIBRARY}.a")
  set_target_properties(${LIBRARY} PROPERTIES IMPORTED_LOCATION_DEBUG ${TARGETDIR}/${RUST_TARGET}/debug/${LIBRARY_FILE})
  set_target_properties(${LIBRARY} PROPERTIES IMPORTED_LOCATION_RELEASE ${TARGETDIR}/${RUST_TARGET}/release/${LIBRARY_FILE})
endforeach()

# Unit test executables don't have predictable names: https://github.com/rust-lang/cargo/issues/1924
#
# We previously looked for executables with names matching '${UNDERBARRED_CRATENAME}*' in
# `${TARGETDIR}/${RUST_BUILD_TYPE}/deps/`, but this would require us to enumerate all the crates
# with unit tests, which would be particularly error prone now that we're not individually building
# the crates from cmake. (This is a larger set than the library-list above, which are only
# the crates that we need to link from C code)
#
# Just using `cargo test` for now. It potentially (re)builds code, but seems
# better than trying to fight the "rust way" here. It also includes doc tests,
# which normally don't have persistent executables at all.
add_test(NAME rust-unit-tests
         COMMAND bash -c " \
            ${CARGO_ENV_VARS} \
            cargo test \
            ${RUST_TARGET_FLAG} \
            ${RUST_BUILD_FLAG} \
            --manifest-path \"${CMAKE_CURRENT_SOURCE_DIR}\"/Cargo.toml \
            --target-dir \"${TARGETDIR}\" \
            --features \"${RUST_FEATURES}\" \
            "
         )
# Longer timeout here and run serially, since it's running a whole test suite,
# and may end up rebuilding code if anything has changed.
set_tests_properties(rust-unit-tests PROPERTIES TIMEOUT 240 RUN_SERIAL TRUE)

# Outside of external, inclusion of headers within the project should be
# relative to this directory.
include_directories(${CMAKE_CURRENT_SOURCE_DIR})
include_directories(${CMAKE_CURRENT_BINARY_DIR})

## check for dependencies with our own Find scripts in "./cmake"
## these don't have built-in cmake files in /usr/share/cmake/Modules/Findxxx.cmake
find_package(RT REQUIRED)
find_package(DL REQUIRED)
find_package(M REQUIRED)

## pthreads
set(CMAKE_THREAD_PREFER_PTHREAD 1)
find_package(Threads REQUIRED)
message(STATUS "Using Threads library: ${CMAKE_THREAD_LIBS_INIT}")

# Build support libraries.
add_subdirectory(lib)

## build the core simulator
add_subdirectory(main)

if(SHADOW_TEST STREQUAL ON)
    add_subdirectory(test)
endif(SHADOW_TEST STREQUAL ON)
