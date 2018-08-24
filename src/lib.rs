#![feature(libc)]
extern crate libc;
use std::ffi::CString;

#[repr(C)]
pub struct evmc_address {
    bytes: [u8;20]
}

#[repr(C)]
#[derive(Debug)]
pub struct evmc_result {
    status_code: i32,
    gas_left: i64,
    output_data: *const u8,
    output_size: i32,
    release: *const u8, //extern fn(),
    created_address: [u8;20], //evmc_address,
    padding: [u8;4]
}

#[repr(C)]
//#[derive(Default)]
pub struct evmc_instance {
    abi_version: i32,
    name: *const u8,
    version: *const u8,
    destroy: extern fn(c: *mut evmc_instance),
    execute: extern fn(c: *mut evmc_instance, *mut libc::c_void, i32, *const libc::c_void, *const libc::c_void, u32) -> Box<evmc_result>,
    set_tracer: extern fn(c: *mut evmc_instance),
    set_option: extern fn(c: *mut evmc_instance)
}

extern fn evmc_destroy(instance: *mut evmc_instance) {
  println!("destroy called");
//  panic!()
}

extern fn evmc_execute(instance: *mut evmc_instance, context: *mut libc::c_void, rev: i32, msg: *const libc::c_void, code: *const libc::c_void, code_size: u32) -> Box<evmc_result> {
  println!("execute called {:x?} {:x?}", instance, context);
  println!("sizeof evmc_result {}", std::mem::size_of::<evmc_result>());
  let ret = Box::new(evmc_result{
    status_code: -1,
    gas_left: 0,
    output_data: 0 as *const u8, //"empty\0".as_ptr() as *const u8,
    output_size: 0,
//    output_data: "empty\0".as_ptr() as *const u8,
//    output_size: 2,
    release: 0 as *const u8,
    created_address: [0u8;20], //0 as *const u8, //evmc_address { bytes: [0u8;20] },
    padding: [0u8;4]
  });
  println!("{:x?}", ret);
  ret
}

impl evmc_instance {
//    fn destroy(&mut self) {
//    }
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
