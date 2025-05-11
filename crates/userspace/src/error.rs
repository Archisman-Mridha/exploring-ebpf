use serde::Serialize;

// Error codes 0-99 are reserved for well-known errors.
// Values of 100+ can be freely used for plugin specific errors.
#[derive(Serialize)]
pub enum ErrorCode {
  IncompatibleCNIVersion = 1,
  UnsupportedFieldInNetworkConfiguration = 2,
  ContainerNotFound = 3,
  InvalidNecessaryEnvs = 4,
  IOFailure = 5,
  DecodingFailure = 6,
  InvalidNetworkConfig = 7,

  Unknown = 100,
}

pub enum Error {
  IncompatibleCNIVersion,
  UnsupportedFieldInNetworkConfiguration,
  ContainerNotFound,
  InvalidNecessaryEnvs,
  IOFailure,
  DecodingFailure,
  InvalidNetworkConfig,

  Unknown,
}

#[derive(Serialize)]
pub struct ProtocolError {
  pub cniVersion: String,
  pub code:       ErrorCode,

  // A short message characterizing the error.
  #[serde(rename = "msg")]
  pub message:     String,
  //
  // A longer message describing the error.
  pub description: String,
}

impl From<Error> for ProtocolError {
  fn from(value: Error) -> Self {
    unimplemented!()
  }
}
