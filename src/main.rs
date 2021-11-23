//! https://clang.llvm.org/docs/SourceBasedCodeCoverage.html
#![no_std]
#![no_main]
use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::rtt_init;


#[allow(dead_code)]
mod constants {
    include!(concat!(env!("OUT_DIR"), "/constants.rs"));
}

#[entry]
fn main() -> ! {
    let length = minicov::get_coverage_data_size();

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
            let output = minicov::capture_coverage();
        } else {
            let mut buffer: [u8; constants::COVERAGE_BUFFER_SIZE] = [0; constants::COVERAGE_BUFFER_SIZE];
            if length > constants::COVERAGE_BUFFER_SIZE {
                panic!("Buffer too small {}>{}", length, constants::COVERAGE_BUFFER_SIZE);
            }
            minicov::capture_coverage_to_buffer(&mut buffer[..length]);
            let output = &buffer[..length];
        }
    }

    let x = channels.up.0.write(&output);
    panic!("Failed to write everything. {}<{}", x, length);
}
