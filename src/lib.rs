#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern fn evmc_destroy(instance: *mut evmc_instance) {
    println!("destroy called");
    if !instance.is_null() { unsafe { Box::from_raw(instance); } }
}

extern fn evmc_release_result(result: *const evmc_result) {
    println!("releasing result");
    unsafe {
        if !(*result).output_data.is_null() {
            // Must typecast here since evmc.h defines it as const,
            // so that consumers do not try to modify it.
            Box::from_raw((*result).output_data as *mut u8);
        }
    }
}

extern fn evmc_execute(
    instance: *mut evmc_instance,
    context: *mut evmc_context,
    rev: evmc_revision,
    msg: *const evmc_message,
    code: *const u8,
    code_size: usize,
) -> evmc_result {
    println!("execute called");

    let output_data = [0u8;69];
    let output_size = 69;

    let output_data = Box::into_raw(Box::new(output_data));

    evmc_result {
        status_code: 0,
        gas_left: 0,
        output_data: output_data as *const u8,
        output_size: output_size,
        release: Some(evmc_release_result),
        create_address: evmc_address { bytes: [0u8;20] },
        padding: [0u8;4]
    }
}

#[no_mangle]
pub extern "system" fn evmc_create() -> *mut evmc_instance {
    println!("evmc_create called...");
    let ret = evmc_instance {
      abi_version: EVMC_ABI_VERSION as i32,
      name: "hera.rs\0".as_ptr() as *const i8,
      version: "0.0.0\0".as_ptr() as *const i8,
      destroy: Some(evmc_destroy),
      execute: Some(evmc_execute),
      set_option: None,
      set_tracer: None
    };
    Box::into_raw(Box::new(ret))
//    Box::new(evmc_instance{ abi_version: 3, name: CString::new("a").unwrap(), version: CString::new("b").unwrap()  })
//    evmc_instance::new()
//    let ptr = ::std::ptr::null_mut();
//    evmc_instance::new(ptr);
//    ::std::ptr::read(ptr)
}

#[no_mangle]
pub extern "system" fn evmc_create_hera() -> *mut evmc_instance {
    evmc_create()
}
