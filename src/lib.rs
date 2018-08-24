use std::ffi::CString;

#[repr(C)]
//#[derive(Default)]
pub struct evmc_instance {
    abi_version: i32,
    name: CString,
    version: CString,
    destroy: extern fn(c: *mut evmc_instance),
//    execute: *u8,
//    set_tracer: *u8,
//    set_option: *u8
}

extern fn evmc_destroy(instance: *mut evmc_instance) {
  panic!()
}

impl evmc_instance {
    fn destroy(&mut self) {
    }
    fn new() -> Self {
      evmc_instance{
          abi_version: 3,
          name: CString::new("a").unwrap(),
          version: CString::new("b").unwrap(),
          destroy: evmc_destroy
      }
    }
}

#[no_mangle]
pub extern fn evmc_create() -> Box<evmc_instance> {
//    Box::new(evmc_instance{ abi_version: 3, name: CString::new("a").unwrap(), version: CString::new("b").unwrap()  })
    Box::new(evmc_instance::new())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
