#![allow(non_snake_case)]

use {
  anyhow::{Context as _, anyhow},
  aya_build::cargo_metadata,
};

fn main() -> anyhow::Result<()> {
  let cargo_metadata::Metadata { packages, .. } = cargo_metadata::MetadataCommand::new()
    .no_deps()
    .exec()
    .context("MetadataCommand::exec")?;

  // Building the eBPF component package isn't that straightforward! Rustc flags and other
  // necessary options need to be set while executing `cargo build`.
  // aya_build::build_ebpf will execute `cargo build` with all those necessary CLI flags and build
  // the eBPF component package for us.

  let ebpfComponentPackage = packages
    .into_iter()
    .find(|cargo_metadata::Package { name, .. }| name == "xdp-starter-kernelspace")
    .ok_or_else(|| anyhow!("xdp-starter-kernelspace package not found"))?;

  aya_build::build_ebpf([ebpfComponentPackage])
}
