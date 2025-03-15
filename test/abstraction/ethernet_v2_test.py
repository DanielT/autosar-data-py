from typing import Tuple
from autosar_data.abstraction import *
from autosar_data.abstraction.communication import *


def prepare_model() -> Tuple[
    AutosarModelAbstraction,
    ArPackage,
    System,
    EthernetCluster,
    EthernetPhysicalChannel,
    EcuInstance,
]:
    """Common setup for all test cases"""
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)

    cluster = system.create_ethernet_cluster("EthernetCluster", package)
    channel = cluster.create_physical_channel("EthernetPhysicalChannel")
    ecu_instance = system.create_ecu_instance("EcuInstance", package)
    controller = ecu_instance.create_ethernet_communication_controller(
        "EthernetController"
    )
    controller.connect_physical_channel("EthernetConnector", channel)
    return model, package, system, cluster, channel, ecu_instance


def test_static_socket_connection() -> None:
    model, package, system, cluster, channel, ecu_instance = prepare_model()

    network_endpoint_server = channel.create_network_endpoint(
        "NetworkEndpointServer",
        NetworkEndpointAddress.IPv4(
            address="192.168.0.1", address_source=IPv4AddressSource.Fixed
        ),
    )
    network_endpoint_client = channel.create_network_endpoint(
        "NetworkEndpointClient",
        NetworkEndpointAddress.IPv4(
            address="192.168.0.2", address_source=IPv4AddressSource.Fixed
        ),
    )
    socket_address_server = channel.create_socket_address(
        "SocketAddressServer",
        network_endpoint_server,
        TpConfig.TcpTp(port_number=1234),
        SocketAddressType.Unicast(None),
    )
    socket_address_client = channel.create_socket_address(
        "SocketAddressClient",
        network_endpoint_client,
        TpConfig.TcpTp(port_number=1234),
        SocketAddressType.Unicast(None),
    )

    # StaticSocketConnection
    static_socket_connection_server = (
        socket_address_server.create_static_socket_connection(
            "StaticSocketConnection", socket_address_client, tcp_role=TcpRole.Listen
        )
    )
    static_socket_connection_client = (
        socket_address_client.create_static_socket_connection(
            "StaticSocketConnection", socket_address_server, tcp_role=TcpRole.Connect
        )
    )
    assert isinstance(static_socket_connection_server, StaticSocketConnection)
    assert isinstance(static_socket_connection_client, StaticSocketConnection)
    assert list(socket_address_server.static_socket_connections()) == [
        static_socket_connection_server
    ]
    assert list(socket_address_client.static_socket_connections()) == [
        static_socket_connection_client
    ]

    # alternative way of creating a pair of static socket connections
    (static_socket_connection_server_alt, static_socket_connection_client_alt) = (
        channel.create_static_socket_connection_pair(
            "StaticSocketConnectionAlternative",
            socket_address_server,
            socket_address_client,
            tcp_connect_timeout=0.1,
        )
    )
    assert isinstance(static_socket_connection_server_alt, StaticSocketConnection)
    assert isinstance(static_socket_connection_client_alt, StaticSocketConnection)
    assert list(socket_address_server.static_socket_connections()) == [
        static_socket_connection_server,
        static_socket_connection_server_alt,
    ]
    assert list(socket_address_client.static_socket_connections()) == [
        static_socket_connection_client,
        static_socket_connection_client_alt,
    ]
    # get and set the name
    assert static_socket_connection_server.name == "StaticSocketConnection"
    static_socket_connection_server.name = "StaticSocketConnection2"
    assert static_socket_connection_server.name == "StaticSocketConnection2"
    # attributes
    assert static_socket_connection_server.socket_address == socket_address_server
    assert static_socket_connection_server.remote_socket == socket_address_client
    static_socket_connection_server.remote_socket = socket_address_server
    assert static_socket_connection_server.remote_socket == socket_address_server
    static_socket_connection_server.remote_socket = socket_address_client
    assert static_socket_connection_server.tcp_role == TcpRole.Listen
    static_socket_connection_server.tcp_role = TcpRole.Connect
    assert static_socket_connection_server.tcp_role == TcpRole.Connect
    static_socket_connection_server.tcp_connect_timeout = 0.2
    assert static_socket_connection_server.tcp_connect_timeout == 0.2
    # check if the static socket connection can be constructed from an element and is equal to the original static socket connection
    element = static_socket_connection_server.element
    static_socket_connection_server2 = StaticSocketConnection(element)
    assert static_socket_connection_server == static_socket_connection_server2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in StaticSocketConnection.__dict__
    assert len(str(static_socket_connection_server)) > 0

    # IpduIdentifierSet
    ipdu_identifier_set = system.create_socket_connection_ipdu_identifier_set(
        "SocketConnectionIpduIdentifierSet", package
    )
    pdu = system.create_nm_pdu("NmPdu", package, 64)

    # SoConIPduIdentifier
    so_con_ipdu_identifier = ipdu_identifier_set.create_socon_ipdu_identifier(
        "SoConIPduIdentifier",
        pdu,
        channel,
        header_id=0xDEAD,
        timeout=0.1,
        collection_trigger=PduCollectionTrigger.Always,
    )
    static_socket_connection_server.add_ipdu_identifier(so_con_ipdu_identifier)
    assert list(static_socket_connection_server.ipdu_identifiers()) == [
        so_con_ipdu_identifier
    ]


def test_ipdu_identifier_set() -> None:
    model, package, system, cluster, channel, ecu_instance = prepare_model()

    # IpduIdentifierSet
    ipdu_identifier_set = system.create_socket_connection_ipdu_identifier_set(
        "SocketConnectionIpduIdentifierSet", package
    )
    assert isinstance(ipdu_identifier_set, SocketConnectionIpduIdentifierSet)
    assert ipdu_identifier_set.name == "SocketConnectionIpduIdentifierSet"
    ipdu_identifier_set.name = "SocketConnectionIpduIdentifierSet2"
    assert ipdu_identifier_set.name == "SocketConnectionIpduIdentifierSet2"
    # check if the ipdu identifier set can be constructed from an element and is equal to the original ipdu identifier set
    element = ipdu_identifier_set.element
    ipdu_identifier_set2 = SocketConnectionIpduIdentifierSet(element)
    assert ipdu_identifier_set == ipdu_identifier_set2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in SocketConnectionIpduIdentifierSet.__dict__
    assert len(str(ipdu_identifier_set)) > 0
    pdu = system.create_nm_pdu("NmPdu", package, 64)
    so_con_ipdu_identifier = ipdu_identifier_set.create_socon_ipdu_identifier(
        "SoConIPduIdentifier",
        pdu,
        channel,
    )
    assert list(ipdu_identifier_set.socon_ipdu_identifiers()) == [
        so_con_ipdu_identifier
    ]


def test_socon_ipdu_identifier() -> None:
    model, package, system, cluster, channel, ecu_instance = prepare_model()
    ipdu_identifier_set = system.create_socket_connection_ipdu_identifier_set(
        "SocketConnectionIpduIdentifierSet", package
    )

    # Pdu
    pdu = system.create_isignal_ipdu("Pdu", package, 64)
    pdu2 = system.create_nm_pdu("NmPdu", package, 64)
    system_signal = package.create_system_signal("SystemSignal")
    isignal = system.create_isignal("ISignal", package, 8, system_signal)
    pdu.map_signal(isignal, 0, ByteOrder.MostSignificantByteLast)

    # SoConIPduIdentifier
    so_con_ipdu_identifier = ipdu_identifier_set.create_socon_ipdu_identifier(
        "SoConIPduIdentifier",
        pdu,
        channel,
        header_id=0xDEAD,
        timeout=0.1,
        collection_trigger=PduCollectionTrigger.Always,
    )
    assert isinstance(so_con_ipdu_identifier, SoConIPduIdentifier)
    # get and set the name
    assert so_con_ipdu_identifier.name == "SoConIPduIdentifier"
    so_con_ipdu_identifier.name = "SoConIPduIdentifier2"
    assert so_con_ipdu_identifier.name == "SoConIPduIdentifier2"
    # attributes
    so_con_ipdu_identifier.header_id = 0xBEEF
    assert so_con_ipdu_identifier.header_id == 0xBEEF
    so_con_ipdu_identifier.timeout = 0.2
    assert so_con_ipdu_identifier.timeout == 0.2
    so_con_ipdu_identifier.collection_trigger = PduCollectionTrigger.Never
    assert so_con_ipdu_identifier.collection_trigger == PduCollectionTrigger.Never
    pt = so_con_ipdu_identifier.pdu_triggering
    assert isinstance(pt, PduTriggering)
    assert pt.pdu == pdu

    # when the pdu triggering is created, a signal triggering for the mapped signal is also created
    # the physical channel can iterate over both the pdu triggerings and the signal triggerings
    assert len(list(channel.pdu_triggerings())) == 1
    assert channel.pdu_triggerings().__next__() == pt
    assert len(list(channel.signal_triggerings())) == 1

    so_con_ipdu_identifier.set_pdu(pdu2, channel)
    pt2 = so_con_ipdu_identifier.pdu_triggering
    assert isinstance(pt2, PduTriggering)
    assert pt2.pdu == pdu2
    # check if the socon ipdu identifier can be constructed from an element and is equal to the original socon ipdu identifier
    element = so_con_ipdu_identifier.element
    so_con_ipdu_identifier2 = SoConIPduIdentifier(element)
    assert so_con_ipdu_identifier == so_con_ipdu_identifier2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in SoConIPduIdentifier.__dict__
    assert len(str(so_con_ipdu_identifier)) > 0


def test_service_instance_collection_set() -> None:
    model, package, system, cluster, channel, ecu_instance = prepare_model()

    # ServiceInstanceCollectionSet
    service_instance_collection_set = system.create_service_instance_collection_set(
        "ServiceInstanceCollectionSet", package
    )
    assert isinstance(service_instance_collection_set, ServiceInstanceCollectionSet)
    # get and set the name
    assert service_instance_collection_set.name == "ServiceInstanceCollectionSet"
    service_instance_collection_set.name = "ServiceInstanceCollectionSet2"
    assert service_instance_collection_set.name == "ServiceInstanceCollectionSet2"
    # check if the service instance collection set can be constructed from an element and is equal to the original service instance collection set
    element = service_instance_collection_set.element
    service_instance_collection_set2 = ServiceInstanceCollectionSet(element)
    assert service_instance_collection_set == service_instance_collection_set2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ServiceInstanceCollectionSet.__dict__
    assert len(str(service_instance_collection_set)) > 0

    provided_service_instance = (
        service_instance_collection_set.create_provided_service_instance(
            "ProvidedServiceInstance", 11, 22, 333, 4444
        )
    )
    consumed_service_instance = (
        service_instance_collection_set.create_consumed_service_instance(
            "ConsumedServiceInstance", 11, 22, 333, "4444"
        )
    )
    assert list(service_instance_collection_set.service_instances()) == [
        provided_service_instance,
        consumed_service_instance,
    ]


def test_provided_service_instance() -> None:
    model, package, system, cluster, channel, ecu_instance = prepare_model()

    network_endpoint_server = channel.create_network_endpoint(
        "NetworkEndpointServer",
        NetworkEndpointAddress.IPv4(
            address="192.168.0.1", address_source=IPv4AddressSource.Fixed
        ),
    )
    socket_address_server_uni_udp = channel.create_socket_address(
        "SocketAddressServerUdp",
        network_endpoint_server,
        TpConfig.UdpTp(port_number=1234),
        SocketAddressType.Unicast(ecu_instance),
    )
    socket_address_server_tcp = channel.create_socket_address(
        "SocketAddressServerTcp",
        network_endpoint_server,
        TpConfig.TcpTp(port_number=1234),
        SocketAddressType.Unicast(ecu_instance),
    )

    # ServiceInstanceCollectionSet
    service_instance_collection_set = system.create_service_instance_collection_set(
        "ServiceInstanceCollectionSet", package
    )

    # ProvidedServiceInstance
    provided_service_instance = (
        service_instance_collection_set.create_provided_service_instance(
            "ProvidedServiceInstance", 11, 22, 333, 4444
        )
    )
    assert isinstance(provided_service_instance, ProvidedServiceInstance)
    # get and set the name
    assert provided_service_instance.name == "ProvidedServiceInstance"
    provided_service_instance.name = "ProvidedServiceInstance2"
    assert provided_service_instance.name == "ProvidedServiceInstance2"
    # attributes
    assert provided_service_instance.service_identifier == 11
    provided_service_instance.service_identifier = 12
    assert provided_service_instance.service_identifier == 12
    assert provided_service_instance.instance_identifier == 22
    provided_service_instance.instance_identifier = 23
    assert provided_service_instance.instance_identifier == 23
    assert provided_service_instance.major_version == 333
    provided_service_instance.major_version = 444
    assert provided_service_instance.major_version == 444
    assert provided_service_instance.minor_version == 4444
    provided_service_instance.minor_version = 5555
    assert provided_service_instance.minor_version == 5555
    provided_service_instance.set_local_unicast_address(socket_address_server_uni_udp)
    provided_service_instance.set_local_unicast_address(socket_address_server_tcp)
    addresses = list(provided_service_instance.local_unicast_addresses())
    assert isinstance(addresses[0], LocalUnicastAddress.Udp)
    assert isinstance(addresses[1], LocalUnicastAddress.Tcp)
    assert addresses[0].address == socket_address_server_uni_udp
    assert addresses[1].address == socket_address_server_tcp

    event_handler = provided_service_instance.create_event_handler("EventHandler", 123)
    assert list(provided_service_instance.event_handlers()) == [event_handler]
    sd_server_config = package.create_someip_sd_server_service_instance_config(
        "SdServerConfig", 10
    )
    provided_service_instance.sd_server_instance_config = sd_server_config
    assert provided_service_instance.sd_server_instance_config == sd_server_config

    # check if the provided service instance can be constructed from an element and is equal to the original provided service instance
    element = provided_service_instance.element
    provided_service_instance2 = ProvidedServiceInstance(element)
    assert provided_service_instance == provided_service_instance2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ProvidedServiceInstance.__dict__
    assert len(str(provided_service_instance)) > 0


def test_event_handler() -> None:
    model, package, system, cluster, channel, ecu_instance = prepare_model()
    service_instance_collection_set = system.create_service_instance_collection_set(
        "ServiceInstanceCollectionSet", package
    )
    provided_service_instance = (
        service_instance_collection_set.create_provided_service_instance(
            "ProvidedServiceInstance", 11, 22, 333, 4444
        )
    )

    # EventHandler
    event_handler = provided_service_instance.create_event_handler("EventHandler", 123)
    assert isinstance(event_handler, EventHandler)
    # get and set the name
    assert event_handler.name == "EventHandler"
    event_handler.name = "EventHandler2"
    assert event_handler.name == "EventHandler2"
    # attributes
    assert event_handler.event_group_identifier == 123
    event_handler.event_group_identifier = 124
    assert event_handler.event_group_identifier == 124

    pdu_activation_routing_group = event_handler.create_pdu_activation_routing_group(
        "RoutingGroup", EventGroupControlType.ActivationAndTriggerUnicast
    )
    assert list(event_handler.pdu_activation_routing_groups()) == [
        pdu_activation_routing_group
    ]
    sd_event_timing_config = package.create_someip_sd_server_event_group_timing_config(
        "SdEventTimingConfig", RequestResponseDelay(min_value=0.1, max_value=0.5)
    )
    event_handler.sd_server_event_group_timing_config = sd_event_timing_config
    assert event_handler.sd_server_event_group_timing_config == sd_event_timing_config

    # check if the event handler can be constructed from an element and is equal to the original event handler
    element = event_handler.element
    event_handler2 = EventHandler(element)
    assert event_handler == event_handler2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EventHandler.__dict__
    assert len(str(event_handler)) > 0


def test_consumed_service_instance() -> None:
    model, package, system, cluster, channel, ecu_instance = prepare_model()
    network_endpoint_client = channel.create_network_endpoint(
        "NetworkEndpointServer",
        NetworkEndpointAddress.IPv4(
            address="192.168.0.2", address_source=IPv4AddressSource.Fixed
        ),
    )
    socket_address_client_uni_udp = channel.create_socket_address(
        "SocketAddressServerUdp",
        network_endpoint_client,
        TpConfig.UdpTp(port_number=1234),
        SocketAddressType.Unicast(ecu_instance),
    )
    service_instance_collection_set = system.create_service_instance_collection_set(
        "ServiceInstanceCollectionSet", package
    )

    # ConsumedServiceInstance
    consumed_service_instance = (
        service_instance_collection_set.create_consumed_service_instance(
            "ConsumedServiceInstance", 11, 22, 333, "4444"
        )
    )
    assert isinstance(consumed_service_instance, ConsumedServiceInstance)
    # get and set the name
    assert consumed_service_instance.name == "ConsumedServiceInstance"
    consumed_service_instance.name = "ConsumedServiceInstance2"
    assert consumed_service_instance.name == "ConsumedServiceInstance2"
    # attributes
    assert consumed_service_instance.service_identifier == 11
    consumed_service_instance.service_identifier = 12
    assert consumed_service_instance.service_identifier == 12
    assert consumed_service_instance.instance_identifier == 22
    consumed_service_instance.instance_identifier = 23
    assert consumed_service_instance.instance_identifier == 23
    assert consumed_service_instance.major_version == 333
    consumed_service_instance.major_version = 444
    assert consumed_service_instance.major_version == 444
    assert consumed_service_instance.minor_version == "4444"
    consumed_service_instance.minor_version = "5555"
    assert consumed_service_instance.minor_version == "5555"

    sd_client_config = package.create_someip_sd_client_service_instance_config(
        "SdClientConfig"
    )
    consumed_service_instance.sd_client_instance_config = sd_client_config
    assert consumed_service_instance.sd_client_instance_config == sd_client_config

    consumed_event_group = consumed_service_instance.create_consumed_event_group(
        "ConsumedEventGroup", 123
    )
    assert list(consumed_service_instance.consumed_event_groups()) == [
        consumed_event_group
    ]
    consumed_service_instance.set_local_unicast_address(socket_address_client_uni_udp)
    addresses = list(consumed_service_instance.local_unicast_addresses())
    assert isinstance(addresses[0], LocalUnicastAddress.Udp)
    assert addresses[0].address == socket_address_client_uni_udp

    # check if the consumed service instance can be constructed from an element and is equal to the original consumed service instance
    element = consumed_service_instance.element
    consumed_service_instance2 = ConsumedServiceInstance(element)
    assert consumed_service_instance == consumed_service_instance2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ConsumedServiceInstance.__dict__
    assert len(str(consumed_service_instance)) > 0


def test_consumed_event_group() -> None:
    model, package, system, cluster, channel, ecu_instance = prepare_model()
    network_endpoint_client = channel.create_network_endpoint(
        "NetworkEndpointServer",
        NetworkEndpointAddress.IPv4(
            address="192.168.0.2", address_source=IPv4AddressSource.Fixed
        ),
    )
    socket_address_client_uni_multi = channel.create_socket_address(
        "SocketAddressServerUdp",
        network_endpoint_client,
        TpConfig.UdpTp(port_number=1234),
        SocketAddressType.Multicast([ecu_instance]),
    )
    service_instance_collection_set = system.create_service_instance_collection_set(
        "ServiceInstanceCollectionSet", package
    )
    consumed_service_instance = (
        service_instance_collection_set.create_consumed_service_instance(
            "ConsumedServiceInstance", 11, 22, 333, "4444"
        )
    )

    # ConsumedEventGroup
    consumed_event_group = consumed_service_instance.create_consumed_event_group(
        "ConsumedEventGroup", 123
    )
    assert isinstance(consumed_event_group, ConsumedEventGroup)
    # get and set the name
    assert consumed_event_group.name == "ConsumedEventGroup"
    consumed_event_group.name = "ConsumedEventGroup2"
    assert consumed_event_group.name == "ConsumedEventGroup2"
    # attributes
    assert consumed_event_group.event_group_identifier == 123
    consumed_event_group.event_group_identifier = 124
    assert consumed_event_group.event_group_identifier == 124

    pdu_activation_routing_group = (
        consumed_event_group.create_pdu_activation_routing_group(
            "RoutingGroup", EventGroupControlType.ActivationAndTriggerUnicast
        )
    )
    assert list(consumed_event_group.pdu_activation_routing_groups()) == [
        pdu_activation_routing_group
    ]
    consumed_event_group.add_event_multicast_address(socket_address_client_uni_multi)
    assert list(consumed_event_group.event_multicast_addresses()) == [
        socket_address_client_uni_multi
    ]
    sd_client_timer_config = package.create_someip_sd_client_event_group_timing_config(
        "SdClientTimerConfig", 10
    )
    consumed_event_group.sd_client_timer_config = sd_client_timer_config
    assert consumed_event_group.sd_client_timer_config == sd_client_timer_config

    # check if the consumed event group can be constructed from an element and is equal to the original consumed event group
    element = consumed_event_group.element
    consumed_event_group2 = ConsumedEventGroup(element)
    assert consumed_event_group == consumed_event_group2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ConsumedEventGroup.__dict__
    assert len(str(consumed_event_group)) > 0


def test_initial_sd_delay_config() -> None:
    # InitialSdDelayConfig
    initial_sd_delay_config = InitialSdDelayConfig(
        initial_delay_max_value=0.5, initial_delay_min_value=0.2
    )
    assert isinstance(initial_sd_delay_config, InitialSdDelayConfig)
    initial_sd_delay_config.initial_delay_max_value = 0.4
    assert initial_sd_delay_config.initial_delay_max_value == 0.4
    initial_sd_delay_config.initial_delay_min_value = 0.1
    assert initial_sd_delay_config.initial_delay_min_value == 0.1
    initial_sd_delay_config.initial_repetitions_base_delay = 0.5
    assert initial_sd_delay_config.initial_repetitions_base_delay == 0.5
    initial_sd_delay_config.initial_repetitions_max = 3
    assert initial_sd_delay_config.initial_repetitions_max == 3
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in InitialSdDelayConfig.__dict__
    assert len(str(initial_sd_delay_config)) > 0


def test_local_unicast_address() -> None:
    model, package, system, cluster, channel, ecu_instance = prepare_model()
    network_endpoint_client = channel.create_network_endpoint(
        "NetworkEndpointServer",
        NetworkEndpointAddress.IPv4(
            address="192.168.0.2", address_source=IPv4AddressSource.Fixed
        ),
    )
    socket_address_client_uni_udp = channel.create_socket_address(
        "SocketAddressServerUdp",
        network_endpoint_client,
        TpConfig.UdpTp(port_number=1234),
        SocketAddressType.Unicast(ecu_instance),
    )
    socket_address_client_tcp = channel.create_socket_address(
        "SocketAddressServerTcp",
        network_endpoint_client,
        TpConfig.TcpTp(port_number=1234),
        SocketAddressType.Unicast(ecu_instance),
    )

    lua_udp = LocalUnicastAddress.Udp(socket_address_client_uni_udp)
    lua_tcp = LocalUnicastAddress.Tcp(socket_address_client_tcp)
    assert isinstance(lua_udp, LocalUnicastAddress.Udp)
    assert isinstance(lua_tcp, LocalUnicastAddress.Tcp)
    assert lua_udp.address == socket_address_client_uni_udp
    assert lua_tcp.address == socket_address_client_tcp
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in LocalUnicastAddress.__dict__
    assert len(str(lua_udp)) > 0
    assert len(str(lua_tcp)) > 0


def test_pdu_activation_routing_group() -> None:
    model, package, system, cluster, channel, ecu_instance = prepare_model()
    service_instance_collection_set = system.create_service_instance_collection_set(
        "ServiceInstanceCollectionSet", package
    )
    provided_service_instance = (
        service_instance_collection_set.create_provided_service_instance(
            "ProvidedServiceInstance", 11, 22, 333, 4444
        )
    )
    event_handler = provided_service_instance.create_event_handler("EventHandler", 123)

    # PduActivationRoutingGroup
    pdu_activation_routing_group = event_handler.create_pdu_activation_routing_group(
        "PduActivationRoutingGroup", EventGroupControlType.ActivationAndTriggerUnicast
    )
    assert isinstance(pdu_activation_routing_group, PduActivationRoutingGroup)
    # get and set the name
    assert pdu_activation_routing_group.name == "PduActivationRoutingGroup"
    pdu_activation_routing_group.name = "PduActivationRoutingGroup2"
    assert pdu_activation_routing_group.name == "PduActivationRoutingGroup2"
    # attributes
    assert (
        pdu_activation_routing_group.event_group_control_type
        == EventGroupControlType.ActivationAndTriggerUnicast
    )
    pdu_activation_routing_group.event_group_control_type = (
        EventGroupControlType.ActivationMulticast
    )
    assert (
        pdu_activation_routing_group.event_group_control_type
        == EventGroupControlType.ActivationMulticast
    )
    pdu_activation_routing_group.event_group_control_type = (
        EventGroupControlType.ActivationUnicast
    )
    assert (
        pdu_activation_routing_group.event_group_control_type
        == EventGroupControlType.ActivationUnicast
    )
    pdu_activation_routing_group.event_group_control_type = (
        EventGroupControlType.TriggerUnicast
    )
    assert (
        pdu_activation_routing_group.event_group_control_type
        == EventGroupControlType.TriggerUnicast
    )
    ipdu_identifier_set = system.create_socket_connection_ipdu_identifier_set(
        "SocketConnectionIpduIdentifierSet", package
    )
    pdu = system.create_nm_pdu("NmPdu", package, 64)
    ipdu_identifier_udp = ipdu_identifier_set.create_socon_ipdu_identifier(
        "SoConIPduIdentifier",
        pdu,
        channel,
    )
    ipdu_identifier_tcp = ipdu_identifier_set.create_socon_ipdu_identifier(
        "SoConIPduIdentifier2",
        pdu,
        channel,
    )
    pdu_activation_routing_group.add_ipdu_identifier_udp(ipdu_identifier_udp)
    assert list(pdu_activation_routing_group.ipdu_identifiers_udp()) == [
        ipdu_identifier_udp
    ]
    pdu_activation_routing_group.add_ipdu_identifier_tcp(ipdu_identifier_tcp)
    assert list(pdu_activation_routing_group.ipdu_identifiers_tcp()) == [
        ipdu_identifier_tcp
    ]
    # check if the pdu activation routing group can be constructed from an element and is equal to the original pdu activation routing group
    element = pdu_activation_routing_group.element
    pdu_activation_routing_group2 = PduActivationRoutingGroup(element)
    assert pdu_activation_routing_group == pdu_activation_routing_group2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in PduActivationRoutingGroup.__dict__
    assert len(str(pdu_activation_routing_group)) > 0


def test_request_response_delay() -> None:
    # RequestResponseDelay
    request_response_delay = RequestResponseDelay(min_value=0.1, max_value=0.5)
    assert isinstance(request_response_delay, RequestResponseDelay)
    request_response_delay.min_value = 0.2
    assert request_response_delay.min_value == 0.2
    request_response_delay.max_value = 0.6
    assert request_response_delay.max_value == 0.6
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in RequestResponseDelay.__dict__
    assert len(str(request_response_delay)) > 0


def test_someip_sd_server_config() -> None:
    model, package, system, cluster, channel, ecu_instance = prepare_model()
    initial_sd_delay_config = InitialSdDelayConfig(
        initial_delay_max_value=0.5, initial_delay_min_value=0.2
    )
    request_response_delay = RequestResponseDelay(min_value=0.1, max_value=0.5)

    # SomeipSdServerServiceInstanceConfig
    sd_server_config = package.create_someip_sd_server_service_instance_config(
        "SomeipSdServerServiceInstanceConfig", 10
    )
    assert isinstance(sd_server_config, SomeipSdServerServiceInstanceConfig)
    assert sd_server_config.name == "SomeipSdServerServiceInstanceConfig"
    sd_server_config.name = "SomeipSdServerServiceInstanceConfig2"
    assert sd_server_config.name == "SomeipSdServerServiceInstanceConfig2"
    # attributes
    sd_server_config.set_initial_offer_behavior(initial_sd_delay_config)
    assert sd_server_config.initial_offer_behavior() == initial_sd_delay_config
    sd_server_config.offer_cyclic_delay = 0.3
    assert sd_server_config.offer_cyclic_delay == 0.3
    sd_server_config.service_offer_time_to_live = 10
    assert sd_server_config.service_offer_time_to_live == 10
    sd_server_config.priority = 5
    assert sd_server_config.priority == 5
    sd_server_config.set_request_response_delay(request_response_delay)
    assert sd_server_config.request_response_delay() == request_response_delay
    # check if the sd server config can be constructed from an element and is equal to the original sd server config
    element = sd_server_config.element
    sd_server_config2 = SomeipSdServerServiceInstanceConfig(element)
    assert sd_server_config == sd_server_config2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in SomeipSdServerServiceInstanceConfig.__dict__
    assert len(str(sd_server_config)) > 0


def test_someip_sd_server_event_config() -> None:
    model, package, system, cluster, channel, ecu_instance = prepare_model()
    request_response_delay = RequestResponseDelay(min_value=0.1, max_value=0.5)

    # SomeipSdServerEventGroupTimingConfig
    sd_event_timing_config = package.create_someip_sd_server_event_group_timing_config(
        "SomeipSdServerEventGroupTimingConfig", request_response_delay
    )
    assert isinstance(sd_event_timing_config, SomeipSdServerEventGroupTimingConfig)
    assert sd_event_timing_config.name == "SomeipSdServerEventGroupTimingConfig"
    sd_event_timing_config.name = "SomeipSdServerEventGroupTimingConfig2"
    assert sd_event_timing_config.name == "SomeipSdServerEventGroupTimingConfig2"
    # attributes
    sd_event_timing_config.set_request_response_delay(request_response_delay)
    assert sd_event_timing_config.request_response_delay() == request_response_delay
    # check if the sd event timing config can be constructed from an element and is equal to the original sd event timing config
    element = sd_event_timing_config.element
    sd_event_timing_config2 = SomeipSdServerEventGroupTimingConfig(element)
    assert sd_event_timing_config == sd_event_timing_config2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in SomeipSdServerEventGroupTimingConfig.__dict__
    assert len(str(sd_event_timing_config)) > 0


def test_someip_sd_client_config() -> None:
    model, package, system, cluster, channel, ecu_instance = prepare_model()
    initial_sd_delay_config = InitialSdDelayConfig(
        initial_delay_max_value=0.5, initial_delay_min_value=0.2
    )

    # SomeipSdClientServiceInstanceConfig
    sd_client_config = package.create_someip_sd_client_service_instance_config(
        "SomeipSdClientServiceInstanceConfig"
    )
    assert isinstance(sd_client_config, SomeipSdClientServiceInstanceConfig)
    assert sd_client_config.name == "SomeipSdClientServiceInstanceConfig"
    sd_client_config.name = "SomeipSdClientServiceInstanceConfig2"
    assert sd_client_config.name == "SomeipSdClientServiceInstanceConfig2"
    # attributes
    sd_client_config.set_initial_find_behavior(initial_sd_delay_config)
    assert sd_client_config.initial_find_behavior() == initial_sd_delay_config
    sd_client_config.priority = 5
    assert sd_client_config.priority == 5
    # check if the sd client config can be constructed from an element and is equal to the original sd client config
    element = sd_client_config.element
    sd_client_config2 = SomeipSdClientServiceInstanceConfig(element)
    assert sd_client_config == sd_client_config2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in SomeipSdClientServiceInstanceConfig.__dict__
    assert len(str(sd_client_config)) > 0


def test_someip_sd_client_event_group_timing_config() -> None:
    model, package, system, cluster, channel, ecu_instance = prepare_model()
    request_response_delay = RequestResponseDelay(min_value=0.1, max_value=0.5)

    # SomeipSdClientEventGroupTimingConfig
    sd_client_timer_config = package.create_someip_sd_client_event_group_timing_config(
        "SomeipSdClientEventGroupTimingConfig", 10
    )
    assert isinstance(sd_client_timer_config, SomeipSdClientEventGroupTimingConfig)
    assert sd_client_timer_config.name == "SomeipSdClientEventGroupTimingConfig"
    sd_client_timer_config.name = "SomeipSdClientEventGroupTimingConfig2"
    assert sd_client_timer_config.name == "SomeipSdClientEventGroupTimingConfig2"
    # attributes
    sd_client_timer_config.set_request_response_delay(request_response_delay)
    assert sd_client_timer_config.request_response_delay() == request_response_delay
    sd_client_timer_config.set_request_response_delay(
        RequestResponseDelay(min_value=0.2, max_value=0.6)
    )
    assert sd_client_timer_config.request_response_delay() == RequestResponseDelay(
        min_value=0.2, max_value=0.6
    )
    sd_client_timer_config.time_to_live = 13
    assert sd_client_timer_config.time_to_live == 13
    sd_client_timer_config.subscribe_eventgroup_retry_delay = 0.3
    assert sd_client_timer_config.subscribe_eventgroup_retry_delay == 0.3
    sd_client_timer_config.subscribe_eventgroup_retry_max = 3
    assert sd_client_timer_config.subscribe_eventgroup_retry_max == 3
    # check if the sd client timer config can be constructed from an element and is equal to the original sd client timer config
    element = sd_client_timer_config.element
    sd_client_timer_config2 = SomeipSdClientEventGroupTimingConfig(element)
    assert sd_client_timer_config == sd_client_timer_config2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in SomeipSdClientEventGroupTimingConfig.__dict__
    assert len(str(sd_client_timer_config)) > 0
