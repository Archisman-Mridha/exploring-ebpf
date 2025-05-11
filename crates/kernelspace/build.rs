#![allow(non_snake_case)]

use which::which;

/*
  Building this crate has an undeclared dependency on the `bpf-linker` binary. This would be
  better expressed by [artifact-dependencies][bindeps] but issues such as
  https://github.com/rust-lang/cargo/issues/12385 make their use impractical for the time being.

  This file implements an imperfect solution: it causes cargo to rebuild the crate whenever the
  mtime of `which bpf-linker` changes.
*/
fn main() {
  let bpfLinker = which("bpf-linker").unwrap();
  println!("cargo:rerun-if-changed={}", bpfLinker.to_str().unwrap());
}
