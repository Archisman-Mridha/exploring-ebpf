use serde::Deserialize;

#[derive(Deserialize)]
pub struct CheckOperationEnvArgs {
  // Container is a network isolation domain, though the actual isolation technology is not defined
  // by the specification. This could be a network namespace or a virtual machine, for example.
  #[serde(rename = "CNI_CONTAINERID")]
  pub containerID: String,

  #[serde(rename = "CNI_NETNS")]
  pub networkNamespace: String,

  #[serde(rename = "CNI_IFNAME")]
  pub networkInterfaceName: String,

  // List of paths to search for CNI plugin executables.
  // Paths are separated by an OS-specific list separator (':' on Linux).
  #[serde(rename = "CNI_PATH")]
  pub pluginBinariesSearchPaths: Vec<String>,
}
