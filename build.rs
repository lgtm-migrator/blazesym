extern crate cbindgen;

use std::env;
use std::path::Path;

fn main() {
    if env::var("GEN_C_HEADER").is_err() {
	return;
    }

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let target_path = Path::new(&crate_dir).join("target").join(build_type);

    cbindgen::Builder::new()
	.with_crate(&crate_dir)
	.with_config(cbindgen::Config::from_root_or_default(&crate_dir))
	.generate()
	.expect("Unable to generate bindings")
	.write_to_file(target_path.join("blazesym.h"));
}