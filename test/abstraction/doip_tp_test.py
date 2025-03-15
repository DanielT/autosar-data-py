from autosar_data.abstraction import *
from autosar_data.abstraction.communication import *


def test_doip_tp_config() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    ethernet_cluster = system.create_ethernet_cluster("EthernetCluster", package)
    ethernet_channel = ethernet_cluster.create_physical_channel("EthernetChannel")

    tp_sdu = system.create_general_purpose_pdu(
        "TpSdu", package, 64, GeneralPurposePduCategory.DoIp
    )
    ipdu_identifier_set = system.create_socket_connection_ipdu_identifier_set(
        "SocketConnectionIpduIdentifierSet", package
    )
    so_con_ipdu_identifier = ipdu_identifier_set.create_socon_ipdu_identifier(
        "SoConIPduIdentifier",
        tp_sdu,
        ethernet_channel,
        header_id=0xDEAD,
        timeout=0.1,
        collection_trigger=PduCollectionTrigger.Always,
    )

    # DoIpTpConfig
    doip_tp_config = system.create_doip_tp_config(
        "DoIpTpConfig", package, ethernet_cluster
    )
    assert isinstance(doip_tp_config, DoIpTpConfig)

    doip_tp_config.name = "DoIpTpConfig2"
    assert doip_tp_config.name == "DoIpTpConfig2"

    doip_tp_config.cluster = ethernet_cluster
    assert doip_tp_config.cluster == ethernet_cluster

    doip_logic_address = doip_tp_config.create_doip_logic_address(
        "DoIpLogicAddress", 1234
    )
    assert list(doip_tp_config.doip_logic_addresses()) == [doip_logic_address]

    tp_sdu_triggering = so_con_ipdu_identifier.pdu_triggering
    assert isinstance(tp_sdu_triggering, PduTriggering)
    doip_tp_connection = doip_tp_config.create_doip_tp_connection(
        "DoIpTpConnection", doip_logic_address, doip_logic_address, tp_sdu_triggering
    )
    assert list(doip_tp_config.doip_tp_connections()) == [doip_tp_connection]

    # check if the doip tp config can be constructed from an element and is equal to the original doip tp config
    element = doip_tp_config.element
    doip_tp_config2 = DoIpTpConfig(element)
    assert doip_tp_config == doip_tp_config2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in DoIpTpConfig.__dict__
    assert len(str(doip_tp_config)) > 0


def test_doip_logic_address() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    ethernet_cluster = system.create_ethernet_cluster("EthernetCluster", package)

    # DoIpTpConfig
    doip_tp_config = system.create_doip_tp_config(
        "DoIpTpConfig", package, ethernet_cluster
    )
    doip_logic_address = doip_tp_config.create_doip_logic_address(
        "DoIpLogicAddress", 1234
    )
    assert isinstance(doip_logic_address, DoIpLogicAddress)

    doip_logic_address.name = "DoIpLogicAddress2"
    assert doip_logic_address.name == "DoIpLogicAddress2"

    doip_logic_address.address = 1235
    assert doip_logic_address.address == 1235

    # check if the doip logic address can be constructed from an element and is equal to the original doip logic address
    element = doip_logic_address.element
    doip_logic_address2 = DoIpLogicAddress(element)
    assert doip_logic_address == doip_logic_address2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in DoIpLogicAddress.__dict__
    assert len(str(doip_logic_address)) > 0


def test_doip_tp_connection() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    ethernet_cluster = system.create_ethernet_cluster("EthernetCluster", package)

    # DoIpTpConfig
    doip_tp_config = system.create_doip_tp_config(
        "DoIpTpConfig", package, ethernet_cluster
    )
    doip_logic_address = doip_tp_config.create_doip_logic_address(
        "DoIpLogicAddress", 1234
    )

    tp_sdu = system.create_general_purpose_pdu(
        "TpSdu", package, 64, GeneralPurposePduCategory.DoIp
    )
    ipdu_identifier_set = system.create_socket_connection_ipdu_identifier_set(
        "SocketConnectionIpduIdentifierSet", package
    )
    ethernet_channel = ethernet_cluster.create_physical_channel("EthernetChannel")
    so_con_ipdu_identifier = ipdu_identifier_set.create_socon_ipdu_identifier(
        "SoConIPduIdentifier",
        tp_sdu,
        ethernet_channel,
        header_id=0xDEAD,
        timeout=0.1,
        collection_trigger=PduCollectionTrigger.Always,
    )

    tp_sdu_triggering = so_con_ipdu_identifier.pdu_triggering
    assert isinstance(tp_sdu_triggering, PduTriggering)
    doip_tp_connection = doip_tp_config.create_doip_tp_connection(
        "DoIpTpConnection", doip_logic_address, doip_logic_address, tp_sdu_triggering
    )
    assert isinstance(doip_tp_connection, DoIpTpConnection)

    doip_tp_connection.name = "DoIpTpConnection2"
    assert doip_tp_connection.name == "DoIpTpConnection2"

    doip_tp_connection.source = doip_logic_address
    assert doip_tp_connection.source == doip_logic_address

    doip_tp_connection.target = doip_logic_address
    assert doip_tp_connection.target == doip_logic_address

    doip_tp_connection.tp_sdu_triggering = tp_sdu_triggering
    assert doip_tp_connection.tp_sdu_triggering == tp_sdu_triggering

    # check if the doip tp connection can be constructed from an element and is equal to the original doip tp connection
    element = doip_tp_connection.element
    doip_tp_connection2 = DoIpTpConnection(element)
    assert doip_tp_connection == doip_tp_connection2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in DoIpTpConnection.__dict__
    assert len(str(doip_tp_connection)) > 0
