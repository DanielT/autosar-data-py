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


def create_socket_addresses(
    channel: EthernetPhysicalChannel,
) -> Tuple[SocketAddress, SocketAddress]:
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
    return socket_address_server, socket_address_client


def test_socket_connection_bundle() -> None:
    model, package, system, cluster, channel, ecu_instance = prepare_model()
    socket_address_server, socket_address_client = create_socket_addresses(channel)

    # SocketConnectionBundle
    socket_connection_bundle = channel.create_socket_connection_bundle(
        "SocketConnectionBundle", socket_address_server
    )
    assert isinstance(socket_connection_bundle, SocketConnectionBundle)
    assert list(channel.socket_connection_bundles()) == [socket_connection_bundle]
    # get and set the name
    assert socket_connection_bundle.name == "SocketConnectionBundle"
    socket_connection_bundle.name = "SocketConnectionBundle2"
    assert socket_connection_bundle.name == "SocketConnectionBundle2"
    # attributes
    assert socket_connection_bundle.physical_channel == channel
    assert socket_connection_bundle.server_port == socket_address_server
    socket_connection_bundle.server_port = socket_address_client
    assert socket_connection_bundle.server_port == socket_address_client
    # check if the socket connection bundle can be constructed from an element and is equal to the original socket connection bundle
    element = socket_connection_bundle.element
    socket_connection_bundle2 = SocketConnectionBundle(element)
    assert socket_connection_bundle == socket_connection_bundle2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in SocketConnectionBundle.__dict__
    assert len(str(socket_connection_bundle)) > 0


def test_socket_connection() -> None:
    model, package, system, cluster, channel, ecu_instance = prepare_model()
    socket_address_server, socket_address_client = create_socket_addresses(channel)

    # SocketConnectionBundle
    scb = channel.create_socket_connection_bundle(
        "SocketConnectionBundle", socket_address_server
    )

    # SocketConnection
    socket_connection = scb.create_bundled_connection(socket_address_client)
    assert isinstance(socket_connection, SocketConnection)
    assert list(scb.bundled_connections()) == [socket_connection]
    # attributes and functions
    assert socket_connection.socket_connection_bundle == scb
    assert socket_connection.client_port == socket_address_client
    socket_connection.client_port = socket_address_server
    assert socket_connection.client_port == socket_address_server
    socket_connection.client_ip_addr_from_connection_request = False
    assert socket_connection.client_ip_addr_from_connection_request == False
    socket_connection.client_port_from_connection_request = False
    assert socket_connection.client_port_from_connection_request == False
    socket_connection.runtime_ip_address_configuration = False
    assert socket_connection.runtime_ip_address_configuration == False
    socket_connection.runtime_port_configuration = False
    assert socket_connection.runtime_port_configuration == False
    # check if the socket connection can be constructed from an element and is equal to the original socket connection
    element = socket_connection.element
    socket_connection2 = SocketConnection(element)
    assert socket_connection == socket_connection2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in SocketConnection.__dict__
    assert len(str(socket_connection)) > 0

    # create a Pdu
    pdu = system.create_nm_pdu("Pdu", package, 8)

    # SocketConnectionIpduIdentifier
    (ipdu_identifier, pt) = socket_connection.create_socket_connection_ipdu_identifier(
        pdu, 0xDEAD, timeout=0.1, collection_trigger=PduCollectionTrigger.Always
    )
    assert list(socket_connection.socket_connection_ipdu_identifiers()) == [
        ipdu_identifier
    ]
    assert list(socket_connection.pdu_triggerings()) == [pt]
    assert ipdu_identifier.socket_connection == socket_connection


def test_socket_connection_ipdu_identifier() -> None:
    model, package, system, cluster, channel, ecu_instance = prepare_model()
    socket_address_server, socket_address_client = create_socket_addresses(channel)

    # SocketConnectionBundle
    scb = channel.create_socket_connection_bundle(
        "SocketConnectionBundle", socket_address_server
    )

    # SocketConnection
    socket_connection = scb.create_bundled_connection(socket_address_client)

    # create a Pdu
    pdu = system.create_general_purpose_ipdu(
        "Pdu", package, 64, GeneralPurposeIPduCategory.Xcp
    )

    # SocketConnectionIpduIdentifier
    (ipdu_identifier, pt) = socket_connection.create_socket_connection_ipdu_identifier(
        pdu, 0xDEAD, timeout=0.1, collection_trigger=PduCollectionTrigger.Always
    )
    assert isinstance(ipdu_identifier, SocketConnectionIpduIdentifier)
    assert isinstance(pt, PduTriggering)
    assert ipdu_identifier.socket_connection == socket_connection
    assert ipdu_identifier.header_id == 0xDEAD
    ipdu_identifier.header_id = 0xBEEF
    assert ipdu_identifier.header_id == 0xBEEF
    assert ipdu_identifier.timeout == 0.1
    ipdu_identifier.timeout = 0.2
    assert ipdu_identifier.timeout == 0.2
    assert ipdu_identifier.collection_trigger == PduCollectionTrigger.Always
    ipdu_identifier.collection_trigger = PduCollectionTrigger.Never
    assert ipdu_identifier.collection_trigger == PduCollectionTrigger.Never
    pt = ipdu_identifier.trigger_pdu(pdu)
    assert pt.pdu == pdu
    assert ipdu_identifier.pdu_triggering == pt
    element = ipdu_identifier.element
    ipdu_identifier2 = SocketConnectionIpduIdentifier(element)
    assert ipdu_identifier == ipdu_identifier2
    assert "__repr__" in SocketConnectionIpduIdentifier.__dict__
    assert len(str(ipdu_identifier)) > 0

    soad_routing_group = system.create_so_ad_routing_group(
        "SoAdRoutingGroup",
        package,
        control_type=EventGroupControlType.ActivationAndTriggerUnicast,
    )

    ipdu_identifier.add_routing_group(soad_routing_group)
    assert list(ipdu_identifier.routing_groups()) == [soad_routing_group]


def test_soad_routing_group() -> None:
    model, package, system, cluster, channel, ecu_instance = prepare_model()

    soad_routing_group = system.create_so_ad_routing_group(
        "SoAdRoutingGroup",
        package,
        control_type=EventGroupControlType.ActivationAndTriggerUnicast,
    )
    assert isinstance(soad_routing_group, SoAdRoutingGroup)
    assert soad_routing_group.name == "SoAdRoutingGroup"
    soad_routing_group.name = "SoAdRoutingGroup2"
    assert soad_routing_group.name == "SoAdRoutingGroup2"
    assert (
        soad_routing_group.control_type
        == EventGroupControlType.ActivationAndTriggerUnicast
    )
    soad_routing_group.control_type = EventGroupControlType.ActivationMulticast
    assert soad_routing_group.control_type == EventGroupControlType.ActivationMulticast
    element = soad_routing_group.element
    soad_routing_group2 = SoAdRoutingGroup(element)
    assert soad_routing_group == soad_routing_group2
    assert "__repr__" in SoAdRoutingGroup.__dict__
    assert len(str(soad_routing_group)) > 0


def test_sd_config() -> None:
    sd_config = SdConfig(
        service_major_version=1,
        service_minor_version=2,
        initial_delay_max_value=0.5,
        initial_delay_min_value=0.1,
        initial_repetitions_base_delay=0.5,
        initial_repetitions_max=3,
        offer_cyclic_delay=None,
        request_response_delay_max_value=3.14,
        request_response_delay_min_value=2.1,
        ttl=10,
    )
    assert isinstance(sd_config, SdConfig)
    assert sd_config.service_major_version == 1
    sd_config.service_major_version = 2
    assert sd_config.service_major_version == 2
    assert sd_config.service_minor_version == 2
    sd_config.service_minor_version = 3
    assert sd_config.service_minor_version == 3
    assert sd_config.initial_delay_max_value == 0.5
    sd_config.initial_delay_max_value = 0.6
    assert sd_config.initial_delay_max_value == 0.6
    assert sd_config.initial_delay_min_value == 0.1
    sd_config.initial_delay_min_value = 0.2
    assert sd_config.initial_delay_min_value == 0.2
    assert sd_config.initial_repetitions_base_delay == 0.5
    sd_config.initial_repetitions_base_delay = 0.6
    assert sd_config.initial_repetitions_base_delay == 0.6
    assert sd_config.initial_repetitions_max == 3
    sd_config.initial_repetitions_max = 4
    assert sd_config.initial_repetitions_max == 4
    assert sd_config.offer_cyclic_delay is None
    sd_config.offer_cyclic_delay = 0.1
    assert sd_config.offer_cyclic_delay == 0.1
    assert sd_config.request_response_delay_max_value == 3.14
    sd_config.request_response_delay_max_value = 3.15
    assert sd_config.request_response_delay_max_value == 3.15
    assert sd_config.request_response_delay_min_value == 2.1
    sd_config.request_response_delay_min_value = 2.2
    assert sd_config.request_response_delay_min_value == 2.2
    assert sd_config.ttl == 10
    sd_config.ttl = 20
    assert sd_config.ttl == 20
    assert "__repr__" in SdConfig.__dict__
    assert len(str(sd_config)) > 0


def test_provided_service_instance() -> None:
    model, package, system, cluster, channel, ecu_instance = prepare_model()
    socket_address_server, socket_address_client = create_socket_addresses(channel)

    # ProvidedServiceInstanceV1
    provided_service_instance = socket_address_server.create_provided_service_instance(
        "ProvidedServiceInstance", 11, 22
    )
    assert isinstance(provided_service_instance, ProvidedServiceInstanceV1)
    assert list(socket_address_server.provided_service_instances()) == [
        provided_service_instance
    ]
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
    assert provided_service_instance.sd_server_config() is None
    sd_config = SdConfig(
        service_major_version=1,
        service_minor_version=2,
        initial_delay_max_value=0.5,
        initial_delay_min_value=0.1,
        initial_repetitions_base_delay=0.5,
        initial_repetitions_max=3,
        offer_cyclic_delay=None,
        request_response_delay_max_value=3.14,
        request_response_delay_min_value=2.1,
        ttl=10,
    )
    provided_service_instance.set_sd_server_config(sd_config)
    assert provided_service_instance.sd_server_config() == sd_config
    # check if the provided service instance can be constructed from an element and is equal to the original provided service instance
    element = provided_service_instance.element
    provided_service_instance2 = ProvidedServiceInstanceV1(element)
    assert provided_service_instance == provided_service_instance2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ProvidedServiceInstanceV1.__dict__
    assert len(str(provided_service_instance)) > 0

    event_handler = provided_service_instance.create_event_handler("EventHandler")
    assert list(provided_service_instance.event_handlers()) == [event_handler]


def test_event_handler() -> None:
    model, package, system, cluster, channel, ecu_instance = prepare_model()
    socket_address_server, socket_address_client = create_socket_addresses(channel)
    psi = socket_address_server.create_provided_service_instance("PSI", 11, 22)

    # EventHandlerV1
    event_handler = psi.create_event_handler("EventHandler")
    assert isinstance(event_handler, EventHandlerV1)
    # get and set the name
    assert event_handler.name == "EventHandler"
    event_handler.name = "EventHandler2"
    assert event_handler.name == "EventHandler2"
    # attributes
    assert event_handler.sd_server_config() is None
    sd_event_config = SdEventConfig(
        request_response_delay_min_value=0.1,
        request_response_delay_max_value=0.5,
        ttl=10,
    )
    event_handler.set_sd_server_config(sd_event_config)
    assert event_handler.sd_server_config() == sd_event_config
    soad_routing_group = system.create_so_ad_routing_group(
        "SoAdRoutingGroup",
        package,
        control_type=EventGroupControlType.ActivationAndTriggerUnicast,
    )
    event_handler.add_routing_group(soad_routing_group)
    assert list(event_handler.routing_groups()) == [soad_routing_group]
    # check if the event handler can be constructed from an element and is equal to the original event handler
    element = event_handler.element
    event_handler2 = EventHandlerV1(element)
    assert event_handler == event_handler2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EventHandlerV1.__dict__
    assert len(str(event_handler)) > 0


def test_consumed_service_instance() -> None:
    model, package, system, cluster, channel, ecu_instance = prepare_model()
    socket_address_server, socket_address_client = create_socket_addresses(channel)
    psi = socket_address_server.create_provided_service_instance("PSI", 11, 22)
    event_handler = psi.create_event_handler("EventHandler")

    # ConsumedServiceInstanceV1
    consumed_service_instance = socket_address_client.create_consumed_service_instance(
        "ConsumedServiceInstance", psi
    )
    assert isinstance(consumed_service_instance, ConsumedServiceInstanceV1)
    assert list(socket_address_client.consumed_service_instances()) == [
        consumed_service_instance
    ]
    # get and set the name
    assert consumed_service_instance.name == "ConsumedServiceInstance"
    consumed_service_instance.name = "ConsumedServiceInstance2"
    assert consumed_service_instance.name == "ConsumedServiceInstance2"
    # attributes
    assert consumed_service_instance.provided_service_instance == psi
    assert consumed_service_instance.sd_client_config() is None
    sd_config = SdConfig(
        service_major_version=1,
        service_minor_version=2,
        initial_delay_max_value=0.5,
        initial_delay_min_value=0.1,
        initial_repetitions_base_delay=0.5,
        initial_repetitions_max=3,
        offer_cyclic_delay=None,
        request_response_delay_max_value=0,  # not used for consumed service instances
        request_response_delay_min_value=0,  # not used for consumed service instances
        ttl=10,
    )
    consumed_service_instance.set_sd_client_config(sd_config)
    assert consumed_service_instance.sd_client_config() == sd_config
    # check if the consumed service instance can be constructed from an element and is equal to the original consumed service instance
    element = consumed_service_instance.element
    consumed_service_instance2 = ConsumedServiceInstanceV1(element)
    assert consumed_service_instance == consumed_service_instance2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ConsumedServiceInstanceV1.__dict__
    assert len(str(consumed_service_instance)) > 0
    consumed_event_group = consumed_service_instance.create_consumed_event_group(
        "ConsumedEventGroup", 123, event_handler
    )
    assert list(consumed_service_instance.consumed_event_groups()) == [
        consumed_event_group
    ]


def test_consumed_event_group() -> None:
    model, package, system, cluster, channel, ecu_instance = prepare_model()
    socket_address_server, socket_address_client = create_socket_addresses(channel)
    psi = socket_address_server.create_provided_service_instance("PSI", 11, 22)
    event_handler = psi.create_event_handler("EventHandler")
    csi = socket_address_client.create_consumed_service_instance("CSI", psi)

    # ConsumedEventGroupV1
    consumed_event_group = csi.create_consumed_event_group(
        "ConsumedEventGroup", 123, event_handler
    )
    assert isinstance(consumed_event_group, ConsumedEventGroupV1)
    assert list(event_handler.consumed_event_groups()) == [consumed_event_group]
    # get and set the name
    assert consumed_event_group.name == "ConsumedEventGroup"
    consumed_event_group.name = "ConsumedEventGroup2"
    assert consumed_event_group.name == "ConsumedEventGroup2"
    # attributes
    assert list(consumed_event_group.event_handlers()) == [event_handler]
    event_handler.add_consumed_event_group(consumed_event_group)
    assert list(consumed_event_group.event_handlers()) == [event_handler, event_handler]
    assert consumed_event_group.event_group_identifier == 123
    consumed_event_group.event_group_identifier = 234
    assert consumed_event_group.event_group_identifier == 234
    assert consumed_event_group.application_endpoint == socket_address_client
    consumed_event_group.application_endpoint = socket_address_server
    assert consumed_event_group.application_endpoint == socket_address_server
    soad_routing_group = system.create_so_ad_routing_group(
        "SoAdRoutingGroup",
        package,
        control_type=EventGroupControlType.ActivationAndTriggerUnicast,
    )
    consumed_event_group.add_routing_group(soad_routing_group)
    assert list(consumed_event_group.routing_groups()) == [soad_routing_group]
    assert consumed_event_group.sd_client_config is None
    sd_event_config = SdEventConfig(
        request_response_delay_min_value=0.1,
        request_response_delay_max_value=0.5,
        ttl=10,
    )
    consumed_event_group.sd_client_config = sd_event_config
    assert consumed_event_group.sd_client_config == sd_event_config
    # check if the consumed event group can be constructed from an element and is equal to the original consumed event group
    element = consumed_event_group.element
    consumed_event_group2 = ConsumedEventGroupV1(element)
    assert consumed_event_group == consumed_event_group2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ConsumedEventGroupV1.__dict__
    assert len(str(consumed_event_group)) > 0

    assert "__repr__" in SdEventConfig.__dict__
    assert len(str(sd_event_config)) > 0
