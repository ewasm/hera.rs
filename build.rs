extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    //println!("cargo:rustc-link-lib=bz2");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")

/*
test bindgen_test_layout___mbstate_t ... ok
test bindgen_test_layout___darwin_pthread_handler_rec ... ok
test bindgen_test_layout___va_list_tag ... ok
test bindgen_test_layout__opaque_pthread_attr_t ... ok
test bindgen_test_layout__opaque_pthread_cond_t ... ok
test bindgen_test_layout__opaque_pthread_condattr_t ... ok
test bindgen_test_layout__opaque_pthread_mutex_t ... ok
test bindgen_test_layout__opaque_pthread_mutexattr_t ... ok
test bindgen_test_layout__opaque_pthread_once_t ... ok
test bindgen_test_layout__opaque_pthread_rwlock_t ... ok
test bindgen_test_layout__opaque_pthread_rwlockattr_t ... ok
test bindgen_test_layout__opaque_pthread_t ... ok
*/
	// Remove MacOS madness.
        .blacklist_type("__va_list_tag")
        .blacklist_type("__builtin_va_list")
        .blacklist_type("__darwin_va_list")

        .blacklist_type("__darwin_mbstate_t")
        .blacklist_type("__darwin_pthread_handler_rec")
        .blacklist_type("__darwin_pthread_attr_t")
        .blacklist_type("__darwin_pthread_cond_t")
        .blacklist_type("__darwin_pthread_condattr_t")
        .blacklist_type("__darwin_pthread_mutex_t")
        .blacklist_type("__darwin_pthread_mutexattr_t")
        .blacklist_type("__darwin_pthread_once_t")
        .blacklist_type("__darwin_pthread_rwlock_t")
        .blacklist_type("__darwin_pthread_rwlockattr_t")
        .blacklist_type("__darwin_pthread_t")

        .blacklist_type("_opaque_mbstate_t")
        .blacklist_type("_opaque_pthread_attr_t")
        .blacklist_type("_opaque_pthread_cond_t")
        .blacklist_type("_opaque_pthread_condattr_t")
        .blacklist_type("_opaque_pthread_mutex_t")
        .blacklist_type("_opaque_pthread_mutexattr_t")
        .blacklist_type("_opaque_pthread_once_t")
        .blacklist_type("_opaque_pthread_rwlock_t")
        .blacklist_type("_opaque_pthread_rwlockattr_t")
        .blacklist_type("_opaque_pthread_t")

        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}