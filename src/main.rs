//! https://clang.llvm.org/docs/SourceBasedCodeCoverage.html
#![feature(maybe_uninit_slice)]
#![feature(maybe_uninit_uninit_array)]
use core::mem::MaybeUninit;
use std::fs::write;


#[no_mangle]
#[used]
pub static __llvm_profile_runtime: i32 = 0;

extern "C" {
    fn __llvm_profile_get_size_for_buffer() -> u64;

    fn __llvm_profile_write_buffer(buffer: *mut char) -> i32;
}



fn main() {
    let length = unsafe { __llvm_profile_get_size_for_buffer()} as usize;
    
    let mut buffer: [MaybeUninit<u8>; 512] = MaybeUninit::uninit_array();
    if length > 512 {
        panic!("Buffer too small {}>{}", length, 512);
    }

    let output = unsafe {
        let res = __llvm_profile_write_buffer(buffer.as_mut_ptr() as *mut char);
        println!("Buffer write {}", res);
        MaybeUninit::slice_assume_init_ref(&buffer[..length])
    };

   
    write("default.profraw", &output).unwrap();

}
