use std::ffi::CString;

#[repr(C)]
//#[derive(Default)]
pub struct evmc_instance {
    abi_version: i32,
    name: *const u8,
    version: *const u8,
    destroy: extern fn(c: *mut evmc_instance),
    execute: extern fn(c: *mut evmc_instance),
    set_tracer: extern fn(c: *mut evmc_instance),
    set_option: extern fn(c: *mut evmc_instance)
//    execute: *u8,
//    set_tracer: *u8,
//    set_option: *u8
}

extern fn evmc_destroy(instance: *mut evmc_instance) {
  println!("destroy called");
  panic!()
}

extern fn evmc_execute(instance: *mut evmc_instance) {
  println!("execute called");
  panic!()
}

impl evmc_instance {
    fn destroy(&mut self) {
    }
    fn new() -> Self {
      evmc_instance{
          abi_version: 5,
          name: "hera.rs\0".as_ptr(), //CString::new("a").unwrap(),
          version: "0.0.0\0".as_ptr(), //CString::new("b").unwrap(),
          destroy: evmc_destroy,
          execute: evmc_execute,
          set_tracer: evmc_destroy,
          set_option: evmc_destroy
      }
    }
}

#[no_mangle]
pub extern fn evmc_create() -> Box<evmc_instance> {
    println!("evmc_create called...");
//    Box::new(evmc_instance{ abi_version: 3, name: CString::new("a").unwrap(), version: CString::new("b").unwrap()  })
    Box::new(evmc_instance::new())
}

#[no_mangle]
pub extern fn evmc_create_hera() -> Box<evmc_instance> {
    evmc_create()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
