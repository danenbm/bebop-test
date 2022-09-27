// build.rs
use bebop_tools as bebop;
use std::path::PathBuf;
const BEBOP_BIN: &str = "bebopc";
fn main() {
    // download the bebop binary automatically and cache it into your target directory
    // it will automatically download the same version as the package you installed
    unsafe {
        bebop::COMPILER_PATH = Some(PathBuf::from(BEBOP_BIN));
    }
    // build all `.bop` schemas in `schemas` dir and make a new module `generated` in `src` with all of them.
    bebop::build_schema_dir("schemas", "src/generated", &Default::default());
}