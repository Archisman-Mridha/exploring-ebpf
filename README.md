# ARCCNI : An eBPF based CNI written in Rust

## TODOs

- [ ] Study about [cargo metadata](https://doc.rust-lang.org/cargo/commands/cargo-metadata.html)

- [ ] Properly explain, why we need VLAN / VXLAN (stateless) to establish a single virtual Layer 2 network across multiple physical LANs connected via a layer 3 switch.

- [ ] Understand how an MPSC channel works. Maybe, build one yourself 😉.

## GOTCHAs

- A kernelspace component package is built using its corresponding userspace component package, with the help of `aya-build`.
  For example, the code in userspace's [build.rs](./crates/build.rs) is responsible for building the corresponding kernelspace component.

## REFERENCEs

- [CNI Essentials: Kubernetes Networking under the Hood](https://tetrate.io/blog/kubernetes-networking/)

- [Demystifying CNI Writing a CNI from Scratch - Filip Nikolic, Isovalent](https://www.youtube.com/watch?v=WmSNPtwOb0Y)
  > You can find the blog version [here](https://isovalent.com/blog/post/demystifying-cni/?utm_source=chatgpt.com).

- [Container Network Interface (CNI) Specification](https://github.com/containernetworking/cni/blob/main/SPEC.md)

- [Kubernetes Docs : Network Plugins](https://kubernetes.io/docs/concepts/extend-kubernetes/compute-storage-net/network-plugins/)

- [CNI Docs : Plugins](https://www.cni.dev/plugins/current/)

- [Official CNI plugin implementations](https://github.com/containernetworking/plugins)

**Writing eBPF programs in Rust** :

- [Making a dev shell with nix flakes](https://fasterthanli.me/series/building-a-rust-service-with-nix/part-10)

- [Rustup terminology](https://rust-lang.github.io/rustup/concepts/index.html)

- [Rustc codegen options](https://doc.rust-lang.org/rustc/codegen-options/index.html)

- [The rust toolchain file](https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file)

- [Rust overlay for Nix](https://github.com/oxalica/rust-overlay?tab=readme-ov-file)

- [The aya book](https://aya-rs.dev/book/)

- [aya examples](https://github.com/vadorovsky/aya-examples)

**About VLAN and VXLAN** :

- [Virtual LAN (VLAN)](https://www.youtube.com/watch?v=ez24W5oTU3U)

- [VXLAN Technology: A Comprehensive Guide to Modern Network Virtualization](https://medium.com/@mattouchi6/vxlan-technology-a-comprehensive-guide-to-modern-network-virtualization-72a1077c63b2)

- [vxlan-docker-hands-on](https://github.com/faysalmehedi/vxlan-docker-hands-on)
  > Hands-on: Multi-node container / Overlay communication using linux vxlan feature.

- [Difference between layer-2 and layer-3 switches](https://www.geeksforgeeks.org/difference-between-layer-2-and-layer-3-switches/)

- [rtnetlink(7) — Linux manual page](https://man7.org/linux/man-pages/man7/rtnetlink.7.html)

- [Virtual eXtensible Local Area Networking documentation](https://docs.kernel.org/networking/vxlan.html)

- [Creating a Simple VXLAN Overlay Network using Linux Network Namespaces and Bridges](https://medium.com/@mdshofiur/creating-a-simple-vxlan-overlay-network-using-linux-network-namespaces-and-bridges-7116039b4882)

**CNI implementations** :

- [rust_cni](https://github.com/masap/rust_cni/)
  > This is a Kubernetes CNI plugin written with Rust.

- [cni-ng](https://github.com/arthur-zhang/cni-ng)
  > Implementation of the Container Network Interface (CNI) in Rust.

- [bvcni](https://github.com/royroyee/bvcni)
  > A Simple CNI plugin based on Linux bridge and VXLAN for Kubernetes.
