#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern fn evmc_destroy(instance: *mut evmc_instance) {
    println!("destroy called");
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
    let ret = evmc_result {
        status_code: -1,
        gas_left: 0,
        output_data: 0 as *const u8,
        output_size: 0,
        release: None,
        create_address: evmc_address { bytes: [0u8;20] },
        padding: [0u8;4]
    };
    ret
}

#[no_mangle]
pub extern fn evmc_create() -> evmc_instance {
    println!("evmc_create called...");
    evmc_instance {
      abi_version: EVMC_ABI_VERSION as i32,
      name: "hera.rs\0".as_ptr() as *const i8,
      version: "0.0.0\0".as_ptr() as *const i8,
      destroy: Some(evmc_destroy),
      execute: Some(evmc_execute),
      set_option: None,
      set_tracer: None
    }
//    Box::new(evmc_instance{ abi_version: 3, name: CString::new("a").unwrap(), version: CString::new("b").unwrap()  })
//    evmc_instance::new()
//    let ptr = ::std::ptr::null_mut();
//    evmc_instance::new(ptr);
//    ::std::ptr::read(ptr)
}

#[no_mangle]
pub extern fn evmc_create_hera() -> evmc_instance {
    evmc_create()
}
