//! https://clang.llvm.org/docs/SourceBasedCodeCoverage.html
#![no_std]
#![no_main]
#![feature(maybe_uninit_slice)]
#![feature(maybe_uninit_uninit_array)]
use core::mem::MaybeUninit;
use cortex_m_rt::entry;
use panic_halt as _;
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

#[entry]
fn main() -> ! {
    let length = unsafe { __llvm_profile_get_size_for_buffer() } as usize;

    let mut channels = rtt_init! {
        up: {
            0: {
                size: 1024
                mode: BlockIfFull // we're only going to write at the end before stopping
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
        let _res = __llvm_profile_write_buffer(buffer.as_mut_ptr() as *mut u8);
        MaybeUninit::slice_assume_init_ref(&buffer[..length])
    };

    let x = channels.up.0.write(&output);
    panic!("Failed to write everything. {}<{}", x, length);
}
