use serde::Deserialize;

#[derive(Deserialize)]
pub struct BridgePluginConfig {
  r#type: String,

  // The name of the network.
  network: String,

  // Name of the bridge to use/create.
  bridge: String,

  // Assign an IP address to the bridge.
  isGateway:        bool,
  //
  // Sets isGateway to true and makes the assigned IP the default route.
  isDefaultGateway: bool,
}
