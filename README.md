# eBPF exercises

## Developer guide

Make sure you meet the requirements specified in the [Aya book](https://aya-rs.dev/book/start/development/).

> [!NOTE]
> You can skip `cargo-generate` and `bpftool`.

You can see a `flake.nix` file at the repository root. So, if you have Nix installed, run `nix develop` to spin up a nix shell with all the pre-requisites installed.

If you have [direnv](https://direnv.net) installed, then the nix shell will automatically activate, when you `cd` into this repository.

## REFERENCEs

- [Making a dev shell with nix flakes](https://fasterthanli.me/series/building-a-rust-service-with-nix/part-10)

- [Rustup terminology](https://rust-lang.github.io/rustup/concepts/index.html)

- [rust overlay](https://github.com/oxalica/rust-overlay?tab=readme-ov-file)
