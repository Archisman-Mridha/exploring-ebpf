#![allow(non_snake_case, clippy::upper_case_acronyms)]

use {
  std::io::{Read, stdin},
  userspace::{self, CNIPlugin, operations::add::AddOperationEnvArgs},
};

pub mod config;

/*
  With bridge plugin, all containers (on the same host) are plugged into a bridge (virtual switch)
  that resides in the host network namespace. The containers receive one end of the veth pair with
  the other end connected to the bridge.

  An IP address is only assigned to container side peer of the veth pair.

  The bridge itself can also be assigned an IP address, turning it into a gateway for the
  containers.
  NOTE : Alternatively, the bridge can function purely in L2 mode and would need to be bridged to
         the host network interface (if other than container-to-container communication on the same
         host is desired).
*/
struct BridgeCNIPlugin {}

impl CNIPlugin for BridgeCNIPlugin {
  fn executeAddOperation() -> Result<Option<String>, userspace::Error> {
    let envArgs = envy::from_env::<AddOperationEnvArgs>().map_err(|error| match error {
      envy::Error::MissingValue(_) => userspace::Error::InvalidNecessaryEnvs,
      _ => userspace::Error::Unknown,
    })?;

    let mut jsonArgs = String::new();
    stdin()
      .read_to_string(&mut jsonArgs)
      .map_err(|_| userspace::Error::IOFailure)?;

    // TODO : Create bridge (if required).

    // TODO : Get network namespace file.

    // TODO : Create veth pair in the container (network namespace).

    // TODO : Move one of the veth peers to the host.
    //        Connect that peer with the bridge.

    unimplemented!()
  }

  fn executeCheckOperation() -> Result<Option<String>, userspace::Error> {
    unimplemented!()
  }

  fn executeDelOperation() -> Result<Option<String>, userspace::Error> {
    unimplemented!()
  }

  fn executeVersionOperation() -> Result<Option<String>, userspace::Error> {
    unimplemented!()
  }
}

fn main() {
  BridgeCNIPlugin::run();
}

#[cfg(test)]
mod tests {}
