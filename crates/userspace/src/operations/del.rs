use serde::Deserialize;

#[derive(Deserialize)]
pub struct DelOperationEnvArgs {
  #[serde(rename = "CNI_CONTAINERID")]
  pub containerID: String,

  #[serde(rename = "CNI_NETNS")]
  pub networkNamespace: Option<String>,

  #[serde(rename = "CNI_IFNAME")]
  pub networkInterfaceName: String,

  #[serde(rename = "CNI_PATH")]
  pub pluginBinariesSearchPaths: Vec<String>,
}
