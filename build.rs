use std::fs;

fn main() {
  let path = fs::canonicalize("./cmake-build-debug").unwrap();
  let path = path.to_str().unwrap();

  println!("cargo:rustc-link-search=native={path}");
  println!("cargo:rustc-env=LD_LIBRARY_PATH={path}")
}
