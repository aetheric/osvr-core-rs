extern crate walkdir;

use walkdir::WalkDir;
use std::process::Command;
use std::env;
use std::path::Path;

fn main() {

	let out_dir = env::var("OUT_DIR")
			.unwrap();

	for file in WalkDir::new("foo") {
		let path = file.unwrap().path();
		println!("Creating bindings for {}.", path.display())
		Command::new("bindgen")
				.arg(path)
				.status()
				.unwrap();
	}

}
