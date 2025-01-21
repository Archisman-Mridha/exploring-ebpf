# eBPF exercises

## TODOs

- [ ] Study about [cargo metadata](https://doc.rust-lang.org/cargo/commands/cargo-metadata.html)

## GOTCHAs

- A kernelspace component package is built using its corresponding userspace component package, with the help of `aya-build`.
  For example, the code in xdp-starter-userspace's [build.rs](./xdp-starter/build.rs) is responsible for building the corresponding kernelspace component (xdp-starter-kernelspace).

## Developer guide

Make sure you meet the requirements specified in the [Aya book](https://aya-rs.dev/book/start/development/).

> [!NOTE]
> You can skip `cargo-generate` and `bpftool`.

You can see a `flake.nix` file at the repository root. So, if you have Nix installed, run `nix develop` to spin up a nix shell with all the pre-requisites installed.

If you have [direnv](https://direnv.net) installed, then the nix shell will automatically activate, when you `cd` into this repository.

## REFERENCEs

- [Making a dev shell with nix flakes](https://fasterthanli.me/series/building-a-rust-service-with-nix/part-10)

- [Rustup terminology](https://rust-lang.github.io/rustup/concepts/index.html)

- [Rustc codegen options](https://doc.rust-lang.org/rustc/codegen-options/index.html)

- [The rust toolchain file](https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file)

- [Rust overlay for Nix](https://github.com/oxalica/rust-overlay?tab=readme-ov-file)

- [The aya book](https://aya-rs.dev/book/)

- [aya examples](https://github.com/vadorovsky/aya-examples)
