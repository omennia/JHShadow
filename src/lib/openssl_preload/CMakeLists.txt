add_library(shadow_openssl_rng SHARED rng.c)
target_compile_options(shadow_openssl_rng PRIVATE -D_GNU_SOURCE)
target_link_libraries(shadow_openssl_rng shadow-shim)
install(TARGETS shadow_openssl_rng DESTINATION lib)

add_library(shadow_openssl_crypto SHARED crypto.c)
target_compile_options(shadow_openssl_crypto PRIVATE -pthread -D_GNU_SOURCE)
install(TARGETS shadow_openssl_crypto DESTINATION lib)
