#[cfg(windows)]
fn main() {
  println!("cargo:rustc-link-search=native=./");
}

#[cfg(unix)]
fn main() {
  println!("cargo:rustc-link-search=native=./");
  println!("cargo:rerun-if-changed=build.rs");

  let target = std::env::var("TARGET").unwrap();

  if target.contains("linux") {
    // Linux: $ORIGIN resolves to the directory containing the executable.
    println!("cargo:rustc-link-arg=-Wl,--enable-new-dtags,-rpath,$ORIGIN:$ORIGIN/..");
  } else if target.contains("apple") {
    // macOS (Cleaner syntax that usually works better with Rust's build system)
    println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path");
    println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path/..");
  }
}