# API Documentation: autosar_data.abstraction.communication

## Examples

### CAN

```python
from autosar_data.abstraction import *
from autosar_data.abstraction.communication import *

model = AutosarModelAbstraction.create("can_example.arxml")
package = model.get_or_create_package("/Package")

# basic model elements
system = package.create_system("system", SystemCategory.EcuExtract)
ecu = system.create_ecu_instance("Ecu", package)
can_cluster = system.create_can_cluster("CanBus", package)
can_channel = can_cluster.create_physical_channel("CanChannel")
ecu_can_ctrl = ecu.create_can_communication_controller("CanController")
ecu_can_ctrl.connect_physical_channel("Connection", can_channel)

# communication elements
frame = system.create_can_frame("Frame", package, 8)
can_channel.trigger_frame(frame, 0x101, CanAddressingMode.Standard, CanFrameType.Can20)
isignal_ipdu = system.create_isignal_ipdu("Pdu", package, 8)
frame.map_pdu(isignal_ipdu, 0, ByteOrder.MostSignificantByteLast)
syssignal = package.create_system_signal("SysSignal")
signal = system.create_isignal("Signal", package, 12, syssignal)
isignal_ipdu.map_signal(signal, 0, ByteOrder.MostSignificantByteLast)
signal.init_value = 1
```

### Ethernet (old)

The way ethernet networks are modeled was changed substantially in Autosar 4.5.0 (`AUTOSAR_00048`).
This example shows the old way, which still exists but is deprecated.

```python
from autosar_data import AutosarVersion
from autosar_data.abstraction import *
from autosar_data.abstraction.communication import *

model = AutosarModelAbstraction.create(
    "ethernet_old.arxml", version=AutosarVersion.AUTOSAR_00047
)
package = model.get_or_create_package("/Package")

# basic model elements
system = package.create_system("system", SystemCategory.EcuExtract)
ecu = system.create_ecu_instance("Ecu", package)
eth_cluster = system.create_ethernet_cluster("EthCluster", package)
eth_channel = eth_cluster.create_physical_channel(
    "EthChannel", vlan_info=EthernetVlanInfo(vlan_name="VLAN_12", vlan_id=12)
)
ecu_eth_ctrl = ecu.create_ethernet_communication_controller("EthCtrl")
ecu_eth_ctrl.connect_physical_channel("Connection", eth_channel)

# network addresses
ecu_address = eth_channel.create_network_endpoint(
    "ecu_address", NetworkEndpointAddress.IPv4(address="192.168.12.100"), ecu=ecu
)
remote_address = eth_channel.create_network_endpoint(
    "remote_address", NetworkEndpointAddress.IPv4(address="ANY")
)

# create network sockets
ecu_socket = eth_channel.create_socket_address(
    "ecu_socket",
    ecu_address,
    TpConfig.UdpTp(port_number=1234),
    SocketAddressType.Unicast(ecu),
)
remote_socket = eth_channel.create_socket_address(
    "remote_socket",
    remote_address,
    TpConfig.UdpTp(port_number=1234),
    SocketAddressType.Unicast(None),
)

# socket connection bundles handle the connection between sockets
socket_connection_bundle = eth_channel.create_socket_connection_bundle(
    "SCB", ecu_socket
)
socket_connection = socket_connection_bundle.create_bundled_connection(remote_socket)

# create a PDU and associate it with the socket connection
idpu = system.create_isignal_ipdu("PDU", package, 16)
socket_connection.create_socket_connection_ipdu_identifier(idpu, 0xBEEF)

# signals could be mapped to the PDU [...]
```

### Ethernet (new)

This example shows the new way of modeling an ethernet network which was introduced in Autosar 4.5.0 (`AUTOSAR_00048`).

```python
from autosar_data import AutosarVersion
from autosar_data.abstraction import *
from autosar_data.abstraction.communication import *

model = AutosarModelAbstraction.create(
    "ethernet_new.arxml", version=AutosarVersion.LATEST
)
package = model.get_or_create_package("/Package")

# basic model elements
system = package.create_system("system", SystemCategory.EcuExtract)
ecu = system.create_ecu_instance("Ecu", package)
eth_cluster = system.create_ethernet_cluster("EthCluster", package)
eth_channel = eth_cluster.create_physical_channel(
    "EthChannel", vlan_info=EthernetVlanInfo(vlan_name="VLAN_12", vlan_id=12)
)
ecu_eth_ctrl = ecu.create_ethernet_communication_controller("EthCtrl")
ecu_eth_ctrl.connect_physical_channel("Connection", eth_channel)

# network addresses
ecu_address = eth_channel.create_network_endpoint(
    "ecu_address", NetworkEndpointAddress.IPv4(address="192.168.12.100"), ecu=ecu
)
remote_address = eth_channel.create_network_endpoint(
    "remote_address", NetworkEndpointAddress.IPv4(address="ANY")
)

# create network sockets
ecu_socket = eth_channel.create_socket_address(
    "ecu_socket",
    ecu_address,
    TpConfig.UdpTp(port_number=1234),
    SocketAddressType.Unicast(ecu),
)
remote_socket = eth_channel.create_socket_address(
    "remote_socket",
    remote_address,
    TpConfig.UdpTp(port_number=1234),
    SocketAddressType.Unicast(None),
)

# socket connection bundles handle the connection between sockets
static_socket_connection = ecu_socket.create_static_socket_connection(
    "StaticConnection", remote_socket
)

pdu = system.create_isignal_ipdu("Pdu", package, 33)
ipdu_identifier_set = system.create_socket_connection_ipdu_identifier_set(
    "IpduIdentifierSet", package
)
ipdu_identifier = ipdu_identifier_set.create_socon_ipdu_identifier(
    "IPduIdentifier", pdu, eth_channel
)
static_socket_connection.add_ipdu_identifier(ipdu_identifier)
```

### Flexray

```python
from autosar_data import AutosarVersion
from autosar_data.abstraction import *
from autosar_data.abstraction.communication import *

model = AutosarModelAbstraction.create("flexray.arxml")
package = model.get_or_create_package("/Package")

# basic model elements
system = package.create_system("system", SystemCategory.EcuExtract)
ecu = system.create_ecu_instance("Ecu", package)
flx_cluster_settings = FlexrayClusterSettings()
flx_cluster_settings.baudrate = 10_000_000
# customize other cluster settings ...
flx_cluster = system.create_flexray_cluster(
    "FlexrayCluster", package, flx_cluster_settings
)
flx_channel = flx_cluster.create_physical_channel(
    "FlexrayChannel", FlexrayChannelName.A
)
ecu_flx_ctrl = ecu.create_flexray_communication_controller("FlexrayController")

# communication elements
frame = system.create_flexray_frame("Frame", package, 32)
flx_channel.trigger_frame(
    frame, 1, FlexrayCommunicationCycle.Repetition(1, CycleRepetition.C1)
)
isignal_ipdu = system.create_isignal_ipdu("Pdu", package, 32)
frame.map_pdu(isignal_ipdu, 0, ByteOrder.MostSignificantByteLast)
syssignal = package.create_system_signal("SysSignal")
signal = system.create_isignal("Signal", package, 12, syssignal)
isignal_ipdu.map_signal(signal, 0, ByteOrder.MostSignificantByteLast)
signal.init_value = 1
```


## API

::: autosar_data.abstraction.communication
