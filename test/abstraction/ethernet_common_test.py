from autosar_data.abstraction import *
from autosar_data.abstraction.communication import *


def test_ethernet_cluster() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)

    # EthernetCluster
    ethernet_cluster = system.create_ethernet_cluster("EthernetCluster", package)
    assert isinstance(ethernet_cluster, EthernetCluster)
    # get and set the name
    assert ethernet_cluster.name == "EthernetCluster"
    ethernet_cluster.name = "EthernetCluster2"
    assert ethernet_cluster.name == "EthernetCluster2"
    # check if the cluster can be constructed from an element and is equal to the original cluster
    element = ethernet_cluster.element
    ethernet_cluster2 = EthernetCluster(element)
    assert ethernet_cluster == ethernet_cluster2
    assert ethernet_cluster.system == system
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EthernetCluster.__dict__
    assert len(str(ethernet_cluster)) > 0


def test_ethernet_vlan_info() -> None:
    # EthernetVlanInfo
    vlan_info = EthernetVlanInfo(vlan_name="abc", vlan_id=123)
    vlan_info.vlan_name = "VLAN100"
    assert vlan_info.vlan_name == "VLAN100"
    vlan_info.vlan_id = 100
    assert vlan_info.vlan_id == 100
    assert "__repr__" in EthernetVlanInfo.__dict__
    assert len(str(vlan_info)) > 0


def test_ethernet_physical_channel() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    ethernet_cluster = system.create_ethernet_cluster("EthernetCluster", package)
    vlan_info = EthernetVlanInfo(vlan_name="abc", vlan_id=123)

    # EthernetPhysicalChannel
    ethernet_physical_channel = ethernet_cluster.create_physical_channel(
        "EthernetPhysicalChannel", vlan_info=vlan_info
    )
    assert isinstance(ethernet_physical_channel, EthernetPhysicalChannel)
    assert list(ethernet_cluster.physical_channels()) == [ethernet_physical_channel]
    # get and set the name
    assert ethernet_physical_channel.name == "EthernetPhysicalChannel"
    ethernet_physical_channel.name = "EthernetPhysicalChannel2"
    assert ethernet_physical_channel.name == "EthernetPhysicalChannel2"
    # check attributes
    assert ethernet_physical_channel.cluster == ethernet_cluster
    assert ethernet_physical_channel.vlan_info() == vlan_info
    ethernet_physical_channel.set_vlan_info(None)
    assert ethernet_physical_channel.vlan_info() is None
    # check if the physical channel can be constructed from an element and is equal to the original physical channel
    element = ethernet_physical_channel.element
    ethernet_physical_channel2 = EthernetPhysicalChannel(element)
    assert ethernet_physical_channel == ethernet_physical_channel2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EthernetPhysicalChannel.__dict__
    assert len(str(ethernet_physical_channel)) > 0


def test_ethernet_communication_controller() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    ethernet_cluster = system.create_ethernet_cluster("EthernetCluster", package)
    vlan_info = EthernetVlanInfo(vlan_name="abc", vlan_id=123)
    ethernet_physical_channel = ethernet_cluster.create_physical_channel(
        "EthernetPhysicalChannel", vlan_info=vlan_info
    )

    # EthernetCommunicationController
    ecu_instance = system.create_ecu_instance("EcuInstance", package)
    assert isinstance(ecu_instance, EcuInstance)
    ethernet_controller = ecu_instance.create_ethernet_communication_controller(
        "EthernetController"
    )
    assert isinstance(ethernet_controller, EthernetCommunicationController)
    assert ethernet_controller.name == "EthernetController"
    ethernet_controller.name = "EthernetController2"
    assert ethernet_controller.name == "EthernetController2"
    assert ethernet_controller.ecu_instance == ecu_instance
    assert "__repr__" in EthernetCommunicationController.__dict__
    assert len(str(ethernet_controller)) > 0
    # check if the controller can be constructed from an element and is equal to the original controller
    element = ethernet_controller.element
    ethernet_controller2 = EthernetCommunicationController(element)
    assert ethernet_controller == ethernet_controller2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EthernetCommunicationController.__dict__
    assert len(str(ethernet_controller)) > 0


def test_ethernet_communication_connector() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    ethernet_cluster = system.create_ethernet_cluster("EthernetCluster", package)
    vlan_info = EthernetVlanInfo(vlan_name="abc", vlan_id=123)
    ethernet_physical_channel = ethernet_cluster.create_physical_channel(
        "EthernetPhysicalChannel", vlan_info=vlan_info
    )
    ecu_instance = system.create_ecu_instance("EcuInstance", package)
    ethernet_controller = ecu_instance.create_ethernet_communication_controller(
        "EthernetController"
    )

    # EthernetCommunicationConnector
    connector = ethernet_controller.connect_physical_channel(
        "EthernetConnector", ethernet_physical_channel
    )
    assert list(ethernet_controller.connected_channels()) == [ethernet_physical_channel]
    assert isinstance(connector, EthernetCommunicationConnector)
    assert connector.name == "EthernetConnector"
    connector.name = "EthernetConnector2"
    assert connector.name == "EthernetConnector2"
    assert connector.controller == ethernet_controller
    assert connector.ecu_instance == ecu_instance
    # check if the connector can be constructed from an element and is equal to the original connector
    element = connector.element
    connector2 = EthernetCommunicationConnector(element)
    assert connector == connector2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EthernetCommunicationConnector.__dict__
    assert len(str(connector)) > 0


def test_network_endpoint_address() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    ethernet_cluster = system.create_ethernet_cluster("EthernetCluster", package)
    vlan_info = EthernetVlanInfo(vlan_name="abc", vlan_id=123)
    ethernet_physical_channel = ethernet_cluster.create_physical_channel(
        "EthernetPhysicalChannel", vlan_info=vlan_info
    )

    endpoint_address_v4_fixed = NetworkEndpointAddress.IPv4(
        address="192.168.0.1", address_source=IPv4AddressSource.Fixed
    )
    assert isinstance(endpoint_address_v4_fixed, NetworkEndpointAddress.IPv4)
    assert endpoint_address_v4_fixed.address == "192.168.0.1"
    assert endpoint_address_v4_fixed.address_source == IPv4AddressSource.Fixed
    assert endpoint_address_v4_fixed.default_gateway is None
    assert endpoint_address_v4_fixed.network_mask is None
    endpoint_address_v4_dhcp = NetworkEndpointAddress.IPv4(
        address_source=IPv4AddressSource.DHCPv4
    )
    assert isinstance(endpoint_address_v4_dhcp, NetworkEndpointAddress.IPv4)
    endpoint_address_v4_autoip = NetworkEndpointAddress.IPv4(
        address_source=IPv4AddressSource.AutoIp
    )
    assert isinstance(endpoint_address_v4_autoip, NetworkEndpointAddress.IPv4)
    endpoint_address_v4_autoip_doip = NetworkEndpointAddress.IPv4(
        address_source=IPv4AddressSource.AutoIpDoIp
    )
    assert isinstance(endpoint_address_v4_autoip_doip, NetworkEndpointAddress.IPv4)
    endpoint_address_v6_fixed = NetworkEndpointAddress.IPv6(
        address="2001:0db8:0000:0000:0000:0000:0000:0001",
        address_source=IPv6AddressSource.Fixed,
    )
    network_endpoint_v4 = ethernet_physical_channel.create_network_endpoint(
        "NetworkEndpoint", endpoint_address_v4_fixed
    )
    network_endpoint_v4.add_network_endpoint_address(endpoint_address_v4_dhcp)
    network_endpoint_v4.add_network_endpoint_address(endpoint_address_v4_autoip)
    network_endpoint_v4.add_network_endpoint_address(endpoint_address_v4_autoip_doip)
    assert list(network_endpoint_v4.addresses()) == [
        endpoint_address_v4_fixed,
        endpoint_address_v4_dhcp,
        endpoint_address_v4_autoip,
        endpoint_address_v4_autoip_doip,
    ]

    assert isinstance(endpoint_address_v6_fixed, NetworkEndpointAddress.IPv6)
    assert (
        endpoint_address_v6_fixed.address == "2001:0db8:0000:0000:0000:0000:0000:0001"
    )
    assert endpoint_address_v6_fixed.address_source == IPv6AddressSource.Fixed
    assert endpoint_address_v6_fixed.default_router is None
    endpoint_address_v6_dhcp = NetworkEndpointAddress.IPv6(
        address_source=IPv6AddressSource.DHCPv6
    )
    assert isinstance(endpoint_address_v6_dhcp, NetworkEndpointAddress.IPv6)
    endpoint_address_v6_link_local = NetworkEndpointAddress.IPv6(
        address_source=IPv6AddressSource.LinkLocal
    )
    assert isinstance(endpoint_address_v6_link_local, NetworkEndpointAddress.IPv6)
    endpoint_address_v6_link_local_doip = NetworkEndpointAddress.IPv6(
        address_source=IPv6AddressSource.LinkLocalDoIp
    )
    assert isinstance(endpoint_address_v6_link_local_doip, NetworkEndpointAddress.IPv6)
    endpoint_address_v6_router_advertisement = NetworkEndpointAddress.IPv6(
        address_source=IPv6AddressSource.RouterAdvertisement
    )
    assert isinstance(
        endpoint_address_v6_router_advertisement, NetworkEndpointAddress.IPv6
    )
    network_endpoint_v6 = ethernet_physical_channel.create_network_endpoint(
        "NetworkEndpointV6", endpoint_address_v6_fixed
    )
    assert isinstance(network_endpoint_v6, NetworkEndpoint)
    network_endpoint_v6.add_network_endpoint_address(endpoint_address_v6_dhcp)
    network_endpoint_v6.add_network_endpoint_address(endpoint_address_v6_link_local)
    network_endpoint_v6.add_network_endpoint_address(
        endpoint_address_v6_router_advertisement
    )
    network_endpoint_v6.add_network_endpoint_address(
        endpoint_address_v6_link_local_doip
    )
    assert list(network_endpoint_v6.addresses()) == [
        endpoint_address_v6_fixed,
        endpoint_address_v6_dhcp,
        endpoint_address_v6_link_local,
        endpoint_address_v6_router_advertisement,
        endpoint_address_v6_link_local_doip,
    ]
    assert "__repr__" in NetworkEndpointAddress.__dict__
    assert len(str(endpoint_address_v4_fixed)) > 0


def test_network_endpoint() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    ethernet_cluster = system.create_ethernet_cluster("EthernetCluster", package)
    vlan_info = EthernetVlanInfo(vlan_name="abc", vlan_id=123)
    ethernet_physical_channel = ethernet_cluster.create_physical_channel(
        "EthernetPhysicalChannel", vlan_info=vlan_info
    )
    ecu_instance = system.create_ecu_instance("EcuInstance", package)
    ethernet_controller = ecu_instance.create_ethernet_communication_controller(
        "EthernetController"
    )
    ethernet_controller.connect_physical_channel(
        "EthernetConnector", ethernet_physical_channel
    )

    # NetworkEndpoint
    endpoint_address_v4_fixed = NetworkEndpointAddress.IPv4(
        address="192.168.0.1", address_source=IPv4AddressSource.Fixed
    )
    network_endpoint_v4 = ethernet_physical_channel.create_network_endpoint(
        "NetworkEndpoint", endpoint_address_v4_fixed, ecu=ecu_instance
    )
    assert isinstance(network_endpoint_v4, NetworkEndpoint)
    assert list(ethernet_physical_channel.network_endpoints()) == [network_endpoint_v4]
    # get and set the name
    assert network_endpoint_v4.name == "NetworkEndpoint"
    network_endpoint_v4.name = "NetworkEndpoint2"
    assert network_endpoint_v4.name == "NetworkEndpoint2"
    # check if the network endpoint can be constructed from an element and is equal to the original network endpoint
    element = network_endpoint_v4.element
    network_endpoint2 = NetworkEndpoint(element)
    assert network_endpoint_v4 == network_endpoint2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in NetworkEndpoint.__dict__
    assert len(str(network_endpoint_v4)) > 0


def test_tp_config() -> None:
    # TpConfig
    tp_config_tcp = TpConfig.TcpTp(port_number=1234)
    assert isinstance(tp_config_tcp, TpConfig)
    assert tp_config_tcp.port_number == 1234
    tp_config_udp = TpConfig.UdpTp(port_number=1234)
    assert isinstance(tp_config_udp, TpConfig)
    assert tp_config_udp.port_number == 1234
    tp_config_udp_dynamic = TpConfig.UdpTp(port_dynamically_assigned=True)
    assert isinstance(tp_config_udp_dynamic, TpConfig)
    assert tp_config_udp_dynamic.port_dynamically_assigned == True
    assert "__repr__" in TpConfig.__dict__
    assert len(str(tp_config_tcp)) > 0


def test_socket_address() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    ethernet_cluster = system.create_ethernet_cluster("EthernetCluster", package)
    vlan_info = EthernetVlanInfo(vlan_name="abc", vlan_id=123)
    ethernet_physical_channel = ethernet_cluster.create_physical_channel(
        "EthernetPhysicalChannel", vlan_info=vlan_info
    )
    ecu_instance = system.create_ecu_instance("EcuInstance", package)
    ethernet_controller = ecu_instance.create_ethernet_communication_controller(
        "EthernetController"
    )
    ethernet_controller.connect_physical_channel(
        "EthernetConnector", ethernet_physical_channel
    )

    # NetworkEndpoint
    endpoint_address_v4_fixed = NetworkEndpointAddress.IPv4(
        address="192.168.0.1", address_source=IPv4AddressSource.Fixed
    )
    network_endpoint_v4 = ethernet_physical_channel.create_network_endpoint(
        "NetworkEndpoint", endpoint_address_v4_fixed, ecu=ecu_instance
    )

    # TpConfig
    tp_config = TpConfig.TcpTp(port_number=1234)

    # SocketAddress
    socket_address = ethernet_physical_channel.create_socket_address(
        "SocketAddress",
        network_endpoint_v4,
        tp_config,
        SocketAddressType.Unicast(None),
    )
    assert isinstance(socket_address, SocketAddress)
    assert list(ethernet_physical_channel.socket_addresses()) == [socket_address]
    # get and set the name
    assert socket_address.name == "SocketAddress"
    socket_address.name = "SocketAddress2"
    assert socket_address.name == "SocketAddress2"
    assert socket_address.physical_channel == ethernet_physical_channel
    assert socket_address.network_endpoint == network_endpoint_v4
    # note: SocketAddressType.Unicast(None) is a no-op, setting neither unicast nor multicast info in the model
    assert socket_address.socket_address_type is None
    assert socket_address.tp_config == tp_config
    socket_address.set_unicast_ecu(ecu_instance)
    assert socket_address.socket_address_type == SocketAddressType.Unicast(ecu_instance)
    # check if the socket address can be constructed from an element and is equal to the original socket address
    element = socket_address.element
    socket_address2 = SocketAddress(element)
    assert socket_address == socket_address2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in SocketAddress.__dict__
    assert len(str(socket_address)) > 0

    socket_address_multicast = ethernet_physical_channel.create_socket_address(
        "SocketAddressMulticast",
        network_endpoint_v4,
        TpConfig.UdpTp(port_number=1234),
        SocketAddressType.Multicast([]),
    )
    socket_address_multicast.add_multicast_ecu(ecu_instance)
    assert socket_address_multicast.socket_address_type == SocketAddressType.Multicast(
        [ecu_instance]
    )


def test_socket_address_type() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    ethernet_cluster = system.create_ethernet_cluster("EthernetCluster", package)
    vlan_info = EthernetVlanInfo(vlan_name="abc", vlan_id=123)
    ethernet_physical_channel = ethernet_cluster.create_physical_channel(
        "EthernetPhysicalChannel", vlan_info=vlan_info
    )
    ecu_instance = system.create_ecu_instance("EcuInstance", package)
    ethernet_controller = ecu_instance.create_ethernet_communication_controller(
        "EthernetController"
    )
    ethernet_controller.connect_physical_channel(
        "EthernetConnector", ethernet_physical_channel
    )

    # NetworkEndpoint
    endpoint_address_v4_fixed = NetworkEndpointAddress.IPv4(
        address="192.168.0.1", address_source=IPv4AddressSource.Fixed
    )
    network_endpoint_v4 = ethernet_physical_channel.create_network_endpoint(
        "NetworkEndpoint", endpoint_address_v4_fixed, ecu=ecu_instance
    )

    # Unicast
    unicast = SocketAddressType.Unicast(ecu_instance)
    assert isinstance(unicast, SocketAddressType.Unicast)
    socket_address_unicast = ethernet_physical_channel.create_socket_address(
        "SocketAddress",
        network_endpoint_v4,
        TpConfig.UdpTp(port_number=1234),
        unicast,
    )
    assert isinstance(
        socket_address_unicast.socket_address_type, SocketAddressType.Unicast
    )
    assert socket_address_unicast.socket_address_type == SocketAddressType.Unicast(
        ecu_instance
    )
    assert socket_address_unicast.socket_address_type.ecu == ecu_instance

    multicast = SocketAddressType.Multicast([ecu_instance])
    assert isinstance(multicast, SocketAddressType.Multicast)
    socket_address_multicast = ethernet_physical_channel.create_socket_address(
        "SocketAddressMulticast",
        network_endpoint_v4,
        TpConfig.UdpTp(port_number=1234),
        multicast,
    )
    assert isinstance(
        socket_address_multicast.socket_address_type, SocketAddressType.Multicast
    )
    assert socket_address_multicast.socket_address_type == SocketAddressType.Multicast(
        [ecu_instance]
    )
    assert socket_address_multicast.socket_address_type.ecus == [ecu_instance]
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in SocketAddressType.__dict__
    assert len(str(unicast)) > 0
    assert len(str(multicast)) > 0


def test_someip_tp_config() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    cluster = system.create_ethernet_cluster("EthernetCluster", package)
    channel = cluster.create_physical_channel("EthernetPhysicalChannel")

    # SomeipTpConfig
    someip_tp_config = system.create_someip_tp_config(
        "SomeIpTpConfig", package, cluster
    )
    assert isinstance(someip_tp_config, SomeipTpConfig)
    assert someip_tp_config.name == "SomeIpTpConfig"
    someip_tp_config.name = "SomeIpTpConfig2"
    assert someip_tp_config.name == "SomeIpTpConfig2"
    assert someip_tp_config.cluster == cluster

    someip_tp_channel = someip_tp_config.create_someip_tp_channel("SomeIpTpChannel")
    assert list(someip_tp_config.someip_tp_channels()) == [someip_tp_channel]

    tp_sdu = system.create_isignal_ipdu("TpSdu", package, 100)
    ipdu_identifier_set = system.create_socket_connection_ipdu_identifier_set(
        "SocketConnectionIpduIdentifierSet", package
    )
    pdu = system.create_general_purpose_ipdu(
        "NPdu", package, 20, GeneralPurposeIPduCategory.SomeipSegmentedIpdu
    )
    scii = ipdu_identifier_set.create_socon_ipdu_identifier(
        "SoConIPduIdentifier",
        pdu,
        channel,
    )
    pt = scii.pdu_triggering
    assert isinstance(pt, PduTriggering)
    somip_tp_connection = someip_tp_config.create_someip_tp_connection(
        tp_sdu, pt, tp_channel=someip_tp_channel
    )
    assert list(someip_tp_config.someip_tp_connections()) == [somip_tp_connection]

    # check if the config can be constructed from an element and is equal to the original config
    element = someip_tp_config.element
    someip_tp_config2 = SomeipTpConfig(element)
    assert someip_tp_config == someip_tp_config2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in SomeipTpConfig.__dict__
    assert len(str(someip_tp_config)) > 0


def test_someip_tp_connection() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    cluster = system.create_ethernet_cluster("EthernetCluster", package)
    channel = cluster.create_physical_channel("EthernetPhysicalChannel")
    someip_tp_config = system.create_someip_tp_config(
        "SomeIpTpConfig", package, cluster
    )
    someip_tp_channel = someip_tp_config.create_someip_tp_channel("SomeIpTpChannel")
    tp_sdu = system.create_isignal_ipdu("TpSdu", package, 100)
    ipdu_identifier_set = system.create_socket_connection_ipdu_identifier_set(
        "SocketConnectionIpduIdentifierSet", package
    )
    pdu = system.create_general_purpose_ipdu(
        "NPdu", package, 20, GeneralPurposeIPduCategory.SomeipSegmentedIpdu
    )
    scii = ipdu_identifier_set.create_socon_ipdu_identifier(
        "SoConIPduIdentifier",
        pdu,
        channel,
    )
    pt = scii.pdu_triggering
    assert isinstance(pt, PduTriggering)

    # SomeipTpConnection
    someip_tp_connection = someip_tp_config.create_someip_tp_connection(tp_sdu, pt)
    assert isinstance(someip_tp_connection, SomeipTpConnection)
    assert someip_tp_connection.someip_tp_config == someip_tp_config
    someip_tp_connection.tp_sdu = tp_sdu
    assert someip_tp_connection.tp_sdu == tp_sdu
    someip_tp_connection.transport_pdu_triggering = pt
    assert someip_tp_connection.transport_pdu_triggering == pt
    someip_tp_connection.tp_channel = someip_tp_channel
    assert someip_tp_connection.tp_channel == someip_tp_channel
    assert "__repr__" in SomeipTpConnection.__dict__
    assert len(str(someip_tp_connection)) > 0
    # check if the connection can be constructed from an element and is equal to the original connection
    element = someip_tp_connection.element
    someip_tp_connection2 = SomeipTpConnection(element)
    assert someip_tp_connection == someip_tp_connection2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in SomeipTpConnection.__dict__
    assert len(str(someip_tp_connection)) > 0


def test_someip_tp_channel() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    cluster = system.create_ethernet_cluster("EthernetCluster", package)
    channel = cluster.create_physical_channel("EthernetPhysicalChannel")
    someip_tp_config = system.create_someip_tp_config(
        "SomeIpTpConfig", package, cluster
    )
    someip_tp_channel = someip_tp_config.create_someip_tp_channel("SomeIpTpChannel")
    assert isinstance(someip_tp_channel, SomeipTpChannel)
    assert someip_tp_channel.name == "SomeIpTpChannel"
    someip_tp_channel.name = "SomeIpTpChannel2"
    assert someip_tp_channel.name == "SomeIpTpChannel2"
    someip_tp_channel.rx_timeout_time = 1.234
    assert someip_tp_channel.rx_timeout_time == 1.234
    someip_tp_channel.separation_time = 1.234
    assert someip_tp_channel.separation_time == 1.234
    # check if the channel can be constructed from an element and is equal to the original channel
    element = someip_tp_channel.element
    someip_tp_channel2 = SomeipTpChannel(element)
    assert someip_tp_channel == someip_tp_channel2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in SomeipTpChannel.__dict__
    assert len(str(someip_tp_channel)) > 0


def test_configure_sd() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    cluster = system.create_ethernet_cluster("EthernetCluster", package)
    channel = cluster.create_physical_channel("EthernetPhysicalChannel")
    ecu_instance_a = system.create_ecu_instance("EcuInstanceA", package)
    controller = ecu_instance_a.create_ethernet_communication_controller("Controller")
    controller.connect_physical_channel("Connector", channel)

    network_endpoint_ecu_a = channel.create_network_endpoint(
        "local_endpoint",
        NetworkEndpointAddress.IPv4(
            address="192.168.0.1", address_source=IPv4AddressSource.Fixed
        ),
    )
    unicast_socket = channel.create_socket_address(
        "UnicastSocket",
        network_endpoint_ecu_a,
        TpConfig.UdpTp(port_number=30490),
        SocketAddressType.Unicast(ecu_instance_a),
    )
    multicast_rx_endpoint = channel.create_network_endpoint(
        "MulticastEndpoint",
        NetworkEndpointAddress.IPv4(
            address="239.0.0.1",
            address_source=IPv4AddressSource.Fixed,
        ),
    )
    multicast_rx_socket = channel.create_socket_address(
        "MulticastSocket",
        multicast_rx_endpoint,
        TpConfig.UdpTp(
            port_number=30490,
        ),
        SocketAddressType.Multicast([ecu_instance_a]),
    )
    remote_anyaddr_endpoint = channel.create_network_endpoint(
        "RemoteEndpoint",
        NetworkEndpointAddress.IPv4(
            address="ANY",
        ),
    )
    remote_anyaddr_socket = channel.create_socket_address(
        "RemoteSocket",
        remote_anyaddr_endpoint,
        TpConfig.UdpTp(port_number=0),
        SocketAddressType.Unicast(None),
    )

    unicast_rx_pdu = system.create_general_purpose_pdu(
        "UnicastRxPdu", package, 0, GeneralPurposePduCategory.Sd
    )
    unicast_tx_pdu = system.create_general_purpose_pdu(
        "UnicastTxPdu", package, 0, GeneralPurposePduCategory.Sd
    )
    multicast_rx_pdu = system.create_general_purpose_pdu(
        "MulticastRxPdu", package, 0, GeneralPurposePduCategory.Sd
    )

    ipdu_identifier_set = system.create_socket_connection_ipdu_identifier_set(
        "SocketConnectionIpduIdentifierSet", package
    )

    # CommonServiceDiscoveryConfig
    common_service_discovery_config = CommonServiceDiscoveryConfig(
        multicast_rx_socket=multicast_rx_socket,
        multicast_rx_pdu=multicast_rx_pdu,
        remote_socket=remote_anyaddr_socket,
        prefer_static_socket_connections=True,
        ipdu_identifier_set=ipdu_identifier_set,
    )
    assert isinstance(common_service_discovery_config, CommonServiceDiscoveryConfig)
    assert common_service_discovery_config.multicast_rx_socket == multicast_rx_socket
    assert common_service_discovery_config.multicast_rx_pdu == multicast_rx_pdu
    assert common_service_discovery_config.remote_socket == remote_anyaddr_socket
    assert common_service_discovery_config.prefer_static_socket_connections == True
    assert common_service_discovery_config.ipdu_identifier_set == ipdu_identifier_set
    assert "__repr__" in CommonServiceDiscoveryConfig.__dict__
    assert len(str(common_service_discovery_config)) > 0

    channel.configure_service_discovery_for_ecu(
        ecu_instance_a,
        unicast_socket,
        unicast_rx_pdu,
        unicast_tx_pdu,
        common_service_discovery_config,
    )

    # with prefer_static_socket_connections=True, the function should create StaticSocketConnections instead of SocketConnectionBundles
    assert channel.has_socket_connections() == False
    assert len(list(channel.socket_connection_bundles())) == 0
