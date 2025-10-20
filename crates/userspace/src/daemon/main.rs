#![allow(non_snake_case)]

use {
  anyhow::anyhow,
  futures::TryStreamExt,
  rtnetlink::{Handle, LinkMessageBuilder, LinkVxlan, new_connection},
  tracing::info,
};

const DEFAULT_PHYSICAL_DEVICE: &str = "eth0";
const VXLAN_DEVICE: &str = "arccni_vxlan";

#[tokio::main]
async fn main() {
  // Opening a connection to the netlink route socket.
  // NOTE : Rtnetlink allows the kernel's routing tables to be read and altered.
  //        It is based on netlink messages.
  let (netlinkRouteSocketConnection, netlinkRouteSocketConnectionHandle, _) =
    new_connection().expect("Failed opening connection to netlink route socket");

  /*
    Under the hood, the netlink socket connection is an MPSC channel.

    (thread A) message -> handle -
                                 |
                                 |
                                 |-> (thread X) MPSC channel consumer -> network route socket.
                                 |
                                 |
    (thread B) message -> handle -
  */
  tokio::spawn(netlinkRouteSocketConnection);

  createVXLAN(
    netlinkRouteSocketConnectionHandle,
    String::from("enp2s0"),
    String::from("arccni_vxlan"),
  )
  .await
  .unwrap();
}

// Creates a VXLAN device named vxlan0, if it doesn't already exist.
async fn createVXLAN(
  netlinkRouteSocketConnectionHandle: Handle,
  physicalDeviceName: String,
  vxlanDeviceName: String,
) -> anyhow::Result<()> {
  // Get the physical device.
  let physicalDevice = netlinkRouteSocketConnectionHandle
    .link()
    .get()
    .match_name(physicalDeviceName)
    .execute()
    .try_next()
    .await
    .expect("Failed getting next link (when trying to get index of the physical device")
    .expect("No link found for the physical device");

  let vxlanDevice = LinkMessageBuilder::<LinkVxlan>::new(&vxlanDeviceName)
    .dev(physicalDevice.header.index)
    .port(4789)
    .learning(false) // Disable MAC address learning.
    .up()
    .build();

  match netlinkRouteSocketConnectionHandle
    .link()
    .get()
    .match_name(vxlanDeviceName.clone())
    .execute()
    .try_next()
    .await
  {
    Ok(Some(_existingVXLANDevice)) => {
      info!("VXLAN device with proper configuration already exists");
      return Ok(());
    }

    Ok(None) => {}

    Err(error)
      if error.to_string() == "Received a netlink error message No such device (os error 19)" => {}

    Err(error) => {
      return Err(anyhow!(
        "Failed while trying to detect whether an existing VXLAN device with the same name exists or not : {error}",
      ));
    }
  }

  /*
    Equivalent CLI command :

      ip link add arccni_vxlan \
        type vxlan \
        id 42 \
        group 239.1.1.1 \
        dev eth1 \
        dstport 4789
  */
  netlinkRouteSocketConnectionHandle
    .link()
    .add(vxlanDevice)
    .execute()
    .await
    .expect("Failed creating VXLAN device");

  Ok(())
}
