fn main() {
  println!("cargo:rustc-link-search=native={}", "/usr/lib/x86_64-linux-gnu/nvidia/current");
  println!("cargo:rustc-link-lib=nvidia-ml");
}
