//! https://clang.llvm.org/docs/SourceBasedCodeCoverage.html
#![no_std]
#![feature(maybe_uninit_slice)]
#![feature(maybe_uninit_uninit_array)]
use core::mem::MaybeUninit;
use core::fmt::Write;
use rtt_target::rtt_init;


#[allow(dead_code)]
mod constants {
    include!(concat!(env!("OUT_DIR"), "/constants.rs"));
}

#[no_mangle]
#[used]
pub static __llvm_profile_runtime: i32 = 0;

extern "C" {
    fn __llvm_profile_get_size_for_buffer() -> u64;

    fn __llvm_profile_write_buffer(buffer: *mut u8) -> i32;
}



fn main() {
    let length = unsafe { __llvm_profile_get_size_for_buffer()} as usize;

    let channels = rtt_init! {
        cov: {
            0: {
                size: length,
                mode: BlockIfFull, // we're only going to write at the end before stopping
                name: "coverage"
            }
        }
    };

    cfg_if::cfg_if! {
        if #[cfg(feature = "alloc")] {
            let mut buffer = Vec::<MaybeUninit<u8>>::new();
            buffer.resize(length, MaybeUninit::uninit());
        } else {
            let mut buffer: [MaybeUninit<u8>; constants::COVERAGE_BUFFER_SIZE] = MaybeUninit::uninit_array();
            if length > constants::COVERAGE_BUFFER_SIZE {
                panic!("Buffer too small {}>{}", length, constants::COVERAGE_BUFFER_SIZE);
            }
        }
    } 

    let output = unsafe {
        let res = __llvm_profile_write_buffer(buffer.as_mut_ptr() as *mut u8);
        MaybeUninit::slice_assume_init_ref(&buffer[..length])
    };

    let mut tx = channels.cov.0;
    write!(tx, &output).unwrap();

}
