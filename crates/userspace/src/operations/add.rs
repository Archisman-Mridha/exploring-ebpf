use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AddOperationEnvArgs {
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

#[derive(Serialize)]
pub struct AddOperationOutput {
  pub cniVersion: String,

  // An array of all interfaces created by the attachment, including any host-level interfaces.
  pub interfaces: Vec<Interface>,

  // IPs assigned by this attachment.
  // Plugins may include IPs assigned external to the container.
  pub ips: Vec<IP>,

  // Routes created by this attachment.
  pub routes: Vec<Route>,

  // DNS configuration information.
  pub dns: DNSConfig,
}

#[derive(Serialize)]
pub struct Interface {
  pub name: String,
  pub mac:  String,

  pub mtu: usize,

  // The isolation domain reference (e.g. path to network namespace) for the interface, or empty if
  // on the host.
  // For interfaces created inside the container, this should be the value passed via CNI_NETNS
  pub sandbox: String,

  // An absolute path to a socket file corresponding to this interface.
  pub socketPath: Option<String>,

  // The platform-specific identifier of the PCI device corresponding to this interface.
  pub pciID: Option<String>,
}

#[derive(Serialize)]
pub struct IP {
  pub address: String, // (in CIDR notation).

  // The default gateway for this subnet (if one exists).
  pub gateway: String,

  // The index into the interfaces list for a CNI Plugin Result indicating which interface this IP
  // configuration should be applied to.
  pub interfaceIndex: usize,
}

#[derive(Serialize)]
pub struct Route {
  #[serde(rename = "dst")]
  pub destination: String, // (in CIDR notation)

  // The next hop address.
  // If unset, a value in gateway in the ips array may be used.
  #[serde(rename = "gtw")]
  pub gateway: String,

  pub mtu: usize,

  // The MSS (Maximal Segment Size) to advertise to these destinations when establishing TCP
  // connections.
  #[serde(rename = "advmss")]
  pub advertisableMSS: usize,

  // The priority of route (lower is higher).
  pub priority: usize,

  // The table to add the route to.
  pub table: usize,

  // The scope of the destinations covered by the route prefix
  // (0 = global, 253 = link , 254 = host).
  pub scope: usize,
}

#[derive(Serialize)]
pub struct DNSConfig {
  // List of a priority-ordered list of DNS nameservers that this network is aware of.
  // Each entry in the list is a string containing either an IPv4 or an IPv6 address.
  pub nameservers: Vec<String>,

  // The local domain used for short hostname lookups.
  pub domain: String,

  // List of priority ordered search domains for short hostname lookups.
  // Will be preferred over domain by most resolvers.
  pub search: Vec<String>,

  // List of options that can be passed to the resolver.
  pub options: Vec<String>,
}
