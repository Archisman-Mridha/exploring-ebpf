#![allow(non_snake_case, clippy::upper_case_acronyms)]

use {
  error::ProtocolError,
  operations::Operation,
  std::{
    env::{self, VarError},
    io::{Write, stderr, stdout},
    str::FromStr,
  },
};

#[path = "./network/network.rs"]
pub mod network;

#[path = "./operations/operations.rs"]
pub mod operations;

mod error;
pub use error::Error;

/*
  A CNI plugin is responsible for configuring a container's network interface in some manner.

  Plugins fall in to two broad categories:

    (1) Interface plugins : which create a network interface inside the container and ensure it has
                            connectivity.

    (2) Chained plugins : which adjust the configuration of an already-created interface (but may
                          need to create more interfaces to do so).

  The runtime must execute the plugin in the runtime's networking domain. (For most cases, this
  means the root network namespace / dom0).
*/
pub trait CNIPlugin {
  // The runtime passes parameters to the plugin via environment variables and configuration. The
  // configuration is supplied via stdin.
  // The plugin returns a result on stdout on success, or an error on stderr if the operation fails.
  // Configuration and results are encoded in JSON.
  fn run() {
    let result = match env::var("CNI_COMMAND") {
      // Report container plugin status.
      Err(VarError::NotPresent) => Err(crate::Error::InvalidNecessaryEnvs),

      Err(_) => Err(crate::Error::IOFailure),

      // The operation that the CNI needs to execute, is passed through the CNI_COMMAND environment
      // variable.
      Ok(value) => match Operation::from_str(&value) {
        Err(_) => Err(crate::Error::InvalidNecessaryEnvs),

        Ok(operation) => Self::executeOperation(operation),
      },
    };

    match result {
      Err(error) => {
        let protocolError = ProtocolError::from(error);
        let serializedProtocolError = serde_json::to_string(&protocolError).unwrap();

        stderr()
          .write_all(serializedProtocolError.as_bytes())
          .unwrap();
      }

      Ok(Some(output)) => stdout().write_all(output.as_bytes()).unwrap(),

      Ok(None) => {}
    }
  }

  fn executeOperation(operation: Operation) -> Result<Option<String>, crate::Error> {
    match operation {
      Operation::ADD => Self::executeAddOperation(),
      Operation::CHECK => Self::executeCheckOperation(),
      Operation::DEL => Self::executeDelOperation(),

      Operation::GC => unimplemented!(),

      Operation::VERSION => Self::executeVersionOperation(),
    }
  }

  /*
    Should either :

      (1) create the interface defined by CNI_IFNAME inside the container at CNI_NETNS, or

      (2) adjust the configuration of the interface defined by CNI_IFNAME inside the container at
          CNI_NETNS.

    If an interface of the requested name already exists in the container, the CNI plugin MUST
    return with an error.

    A runtime should not call ADD twice consecutively for the same (CNI_CONTAINERID, CNI_IFNAME)
    tuple.
    This implies that a given container ID may be added to a specific network more than once only
    if each addition is done with a different interface name.

    NOTE :  If the plugin was supplied a prevResult as part of its input configuration, it MUST
            handle prevResult by either passing it through, or modifying it appropriately.
  */
  fn executeAddOperation() -> Result<Option<String>, crate::Error>;

  fn executeCheckOperation() -> Result<Option<String>, crate::Error>;

  /*
    Should either :

      (1) delete the interface defined by CNI_IFNAME inside the container at CNI_NETNS, or

      (2) undo any modifications applied in the plugin's ADD functionality

    A prevResult must be supplied to CNI plugins as part of a DEL command. Plugins should still
    return without error if prevResult is empty for a DEL command, however.

    DEL command invocations are always considered best-effort - plugins should always complete a
    DEL action without error to the fullest extent possible, even if some resources or state are
    missing.
    For example, an IPAM plugin should generally release an IP allocation and return success even
    if the container network namespace no longer exists, unless that network namespace is critical
    for IPAM management.

    Plugins MUST accept multiple DEL calls for the same (CNI_CONTAINERID, CNI_IFNAME) pair, and
    return success if the interface in question, or any modifications added, are missing.
  */
  fn executeDelOperation() -> Result<Option<String>, crate::Error>;

  fn executeVersionOperation() -> Result<Option<String>, crate::Error>;
}
