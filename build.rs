use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let size = env::var("COVERAGE_BUFFER_SIZE")
        .map(|x| x.parse::<usize>().unwrap_or(1024))
        .unwrap_or(1024);

    let output = Path::new(&env::var_os("OUT_DIR").unwrap()).join("constants.rs");

    fs::write(
        &output,
        format!("pub const COVERAGE_BUFFER_SIZE: usize = {};", size),
    )
    .unwrap();
}
