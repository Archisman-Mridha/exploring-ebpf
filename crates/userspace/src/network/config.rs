// CNI defines a network configuration format, which contains directives for both the container
// runtime as well as the plugins to consume.
// At plugin execution time, this configuration format is interpreted by the runtime and
// transformed in to a form to be passed to the plugins.
pub struct NetworkConfig {
  // Semantic Version 2.0 of CNI specification to which this configuration list and all the
  // individual configurations conform.
  pub cniVersion: String,

  // List of all CNI versions which this configuration supports.
  pub cniVersions: Vec<String>,

  // Network name.
  // This should be unique across all network configurations on a host.
  pub name: String,

  // If true, runtimes must not call CHECK for this network configuration list.
  pub disableCheck: bool,
  //
  // If true, runtimes must not call GC for this network configuration list.
  pub disableGC:    bool,

  // A list of inlined plugin configuration objects.
  pub plugins:                Vec<PluginConfig>,
  //
  // If false, indicates plugin configuration objects can be aggregated from multiple sources.
  // Any valid plugin configuration objects aggregated from other sources must be appended to the
  // final list of plugins for that network name.
  pub loadOnlyInlinedPlugins: bool,
}

/*
  A plugin is a program that applies a specified network configuration.

  NOTE : Runtimes may aggregate plugin configuration objects from multiple sources, and must
         unambiguously associate each loaded plugin configuration object with a single, valid
         network configuration.
         All aggregated plugin configuration objects must be validated, and each plugin with a
         valid configuration object must be invoked.
*/
pub struct PluginConfig {
  // Name of the CNI plugin binary on disk.
  pub r#type: String,
}
