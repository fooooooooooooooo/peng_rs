use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use bindgen::Builder;

fn main() {
  let so_path = fs::canonicalize("./cmake-build-debug").unwrap();
  let so_path = so_path.to_str().unwrap();

  println!("cargo:rustc-link-search=native={so_path}");
  println!("cargo:rustc-link-lib=dylib=peng_wrapper");
  println!("cargo:rustc-env=LD_LIBRARY_PATH={so_path}");

  println!("cargo:rerun-if-changed=wrapper/wrapper.h");

  let peng_include_path = fs::canonicalize("./PengEngine/src").unwrap();
  let peng_include_path = peng_include_path.to_str().unwrap();

  let mut builder = bindgen::Builder::default()
    .size_t_is_usize(false)
    .clang_args(["-x", "c++", "-std=c++23"])
    .header("wrapper/wrapper.h")
    .clang_arg(format!("-I{peng_include_path}"))
    .parse_callbacks(Box::new(bindgen::CargoCallbacks));

  builder = builder
    .allowlist_file("wrapper/w_.*")
    .allowlist_recursively(false)
    .no_debug(".*");

  builder = add_class(builder, "PengEngine");
  builder = add_class(builder, "Asset");
  builder = add_class(builder, "rendering::WindowSubsystem");
  builder = add_class(builder, "scene::SceneLoader");
  builder = add_class(builder, "rendering::WindowIcon");
  builder = add_class(builder, "LogSeverity");

  let bindings = builder.generate().expect("Unable to generate bindings");

  // rust analyzer really hates include!()
  // let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
  let out_file = PathBuf::from("src/bindings/bindings.rs");

  let mut buffer = vec![0u8; 0];
  bindings.write(Box::new(&mut buffer)).expect("Couldn't write bindings");

  let clippy = "#![allow(clippy::all)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
";

  let mut file = OpenOptions::new()
    .append(false)
    .truncate(true)
    .write(true)
    .open(out_file)
    .unwrap();

  writeln!(file, "{clippy}").unwrap();
  file.write_all(&buffer).unwrap();
}

fn add_class(builder: Builder, class: &str) -> Builder {
  builder
    .allowlist_type(class)
    .opaque_type(class)
    .blocklist_function(format!("{class}.*"))
}
