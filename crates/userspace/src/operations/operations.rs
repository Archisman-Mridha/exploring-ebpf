use strum::EnumString;

pub mod add;
pub mod check;
pub mod del;
pub mod version;

#[derive(Debug, PartialEq, EnumString)]
pub enum Operation {
  // Add container to network, or apply modifications.
  ADD,

  // Check container's networking is as expected.
  CHECK,

  // Remove container from network, or un-apply modifications.
  DEL,

  // Clean up any stale resources.
  GC,

  // Probe plugin version support.
  VERSION,
}
