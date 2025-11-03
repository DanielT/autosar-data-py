from autosar_data.abstraction import *
from autosar_data.abstraction.communication import *


def test_can_tp_config():
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    can_cluster = system.create_can_cluster("CanCluster", package)
    ecu_instance = system.create_ecu_instance("EcuInstance", package)

    # CanTpConfig
    can_tp_config = system.create_can_tp_config("CanTpConfig", package, can_cluster)
    assert isinstance(can_tp_config, CanTpConfig)
    # get and set the name
    assert can_tp_config.name == "CanTpConfig"
    can_tp_config.name = "CanTpConfig2"
    assert can_tp_config.name == "CanTpConfig2"
    # attributes and methods
    can_tp_config.cluster = can_cluster
    assert can_tp_config.cluster == can_cluster

    can_tp_ecu = can_tp_config.create_can_tp_ecu(ecu_instance)
    assert list(can_tp_config.can_tp_ecus()) == [can_tp_ecu]

    can_tp_address = can_tp_config.create_can_tp_address("CanTpAddress", 0x7F)
    assert list(can_tp_config.can_tp_addresses()) == [can_tp_address]

    can_tp_channel = can_tp_config.create_can_tp_channel(
        "CanTpChannel", 1, CanTpChannelMode.FullDuplex
    )
    assert list(can_tp_config.can_tp_channels()) == [can_tp_channel]

    n_pdu = system.create_n_pdu("NPdu", package, 8)
    tp_sdu = system.create_dcm_ipdu("TpSdu", package, 8, DiagPduType.DiagRequest)
    can_tp_connection = can_tp_config.create_can_tp_connection(
        "CanTpConnection",
        CanTpAddressingFormat.NormalFixed,
        can_tp_channel,
        n_pdu,
        tp_sdu,
        True,
    )
    assert list(can_tp_config.can_tp_connections()) == [can_tp_connection]

    can_tp_node = can_tp_config.create_can_tp_node("CanTpNode")
    assert list(can_tp_config.can_tp_nodes()) == [can_tp_node]

    # check if the can tp config can be constructed from an element and is equal to the original can tp config
    element = can_tp_config.element
    can_tp_config2 = CanTpConfig(element)
    assert can_tp_config == can_tp_config2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in CanTpConfig.__dict__
    assert len(str(can_tp_config)) > 0


def test_can_tp_ecu() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    can_cluster = system.create_can_cluster("CanCluster", package)
    ecu_instance = system.create_ecu_instance("EcuInstance", package)
    can_tp_config = system.create_can_tp_config("CanTpConfig", package, can_cluster)

    # CanTpEcu
    can_tp_ecu = can_tp_config.create_can_tp_ecu(ecu_instance)
    assert isinstance(can_tp_ecu, CanTpEcu)

    # attributes and methods
    can_tp_ecu.ecu_instance = ecu_instance
    assert can_tp_ecu.ecu_instance == ecu_instance
    can_tp_ecu.cycle_time_main_function = 10.5
    assert can_tp_ecu.cycle_time_main_function == 10.5

    # check if the can tp ecu can be constructed from an element and is equal to the original can tp ecu
    element = can_tp_ecu.element
    can_tp_ecu2 = CanTpEcu(element)
    assert can_tp_ecu == can_tp_ecu2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in CanTpEcu.__dict__
    assert len(str(can_tp_ecu)) > 0


def test_can_tp_address() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    can_cluster = system.create_can_cluster("CanCluster", package)
    ecu_instance = system.create_ecu_instance("EcuInstance", package)
    can_tp_config = system.create_can_tp_config("CanTpConfig", package, can_cluster)

    # CanTpAddress
    can_tp_address = can_tp_config.create_can_tp_address("CanTpAddress", 0x7F)
    assert isinstance(can_tp_address, CanTpAddress)

    # attributes and methods
    can_tp_address.name = "CanTpAddress"
    assert can_tp_address.name == "CanTpAddress"
    can_tp_address.tp_address = 0x7F
    assert can_tp_address.tp_address == 0x7F

    # check if the can tp address can be constructed from an element and is equal to the original can tp address
    element = can_tp_address.element
    can_tp_address2 = CanTpAddress(element)
    assert can_tp_address == can_tp_address2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in CanTpAddress.__dict__
    assert len(str(can_tp_address)) > 0


def test_can_tp_channel() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    can_cluster = system.create_can_cluster("CanCluster", package)
    ecu_instance = system.create_ecu_instance("EcuInstance", package)
    can_tp_config = system.create_can_tp_config("CanTpConfig", package, can_cluster)

    # CanTpChannel
    can_tp_channel = can_tp_config.create_can_tp_channel(
        "CanTpChannel", 1, CanTpChannelMode.HalfDuplex
    )
    assert isinstance(can_tp_channel, CanTpChannel)

    # attributes and methods
    can_tp_channel.name = "CanTpChannel"
    assert can_tp_channel.name == "CanTpChannel"
    can_tp_channel.channel_id = 1
    assert can_tp_channel.channel_id == 1
    can_tp_channel.channel_mode = CanTpChannelMode.FullDuplex
    assert can_tp_channel.channel_mode == CanTpChannelMode.FullDuplex
    can_tp_channel.channel_mode = CanTpChannelMode.HalfDuplex
    assert can_tp_channel.channel_mode == CanTpChannelMode.HalfDuplex

    # check if the can tp channel can be constructed from an element and is equal to the original can tp channel
    element = can_tp_channel.element
    can_tp_channel2 = CanTpChannel(element)
    assert can_tp_channel == can_tp_channel2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in CanTpChannel.__dict__
    assert len(str(can_tp_channel)) > 0


def test_can_tp_connection() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    can_cluster = system.create_can_cluster("CanCluster", package)
    ecu_instance = system.create_ecu_instance("EcuInstance", package)
    can_tp_config = system.create_can_tp_config("CanTpConfig", package, can_cluster)

    # CanTpConnection
    n_pdu = system.create_n_pdu("NPdu", package, 8)
    tp_sdu = system.create_dcm_ipdu("TpSdu", package, 8, DiagPduType.DiagResponse)
    can_tp_channel = can_tp_config.create_can_tp_channel(
        "CanTpChannel", 1, CanTpChannelMode.HalfDuplex
    )
    can_tp_connection = can_tp_config.create_can_tp_connection(
        "CanTpConnection",
        CanTpAddressingFormat.NormalFixed,
        can_tp_channel,
        n_pdu,
        tp_sdu,
        True,
    )
    assert isinstance(can_tp_connection, CanTpConnection)

    # attributes and methods
    can_tp_connection.name = "CanTpConnection"
    assert can_tp_connection.name == "CanTpConnection"
    can_tp_connection.addressing_format = CanTpAddressingFormat.NormalFixed
    assert can_tp_connection.addressing_format == CanTpAddressingFormat.NormalFixed
    can_tp_connection.addressing_format = CanTpAddressingFormat.Extended
    assert can_tp_connection.addressing_format == CanTpAddressingFormat.Extended
    can_tp_connection.addressing_format = CanTpAddressingFormat.Standard
    assert can_tp_connection.addressing_format == CanTpAddressingFormat.Standard
    can_tp_connection.addressing_format = CanTpAddressingFormat.Mixed
    assert can_tp_connection.addressing_format == CanTpAddressingFormat.Mixed
    can_tp_connection.addressing_format = CanTpAddressingFormat.Mixed29Bit
    assert can_tp_connection.addressing_format == CanTpAddressingFormat.Mixed29Bit
    can_tp_connection.channel = can_tp_channel
    assert can_tp_connection.channel == can_tp_channel
    can_tp_connection.data_pdu = n_pdu
    assert can_tp_connection.data_pdu == n_pdu
    can_tp_connection.tp_sdu = tp_sdu
    assert can_tp_connection.tp_sdu == tp_sdu
    can_tp_connection.padding_activation = True
    assert can_tp_connection.padding_activation == True

    can_tp_node = can_tp_config.create_can_tp_node("CanTpNode")
    can_tp_connection.add_receiver(can_tp_node)
    assert list(can_tp_connection.receivers()) == [can_tp_node]
    can_tp_connection.transmitter = can_tp_node
    assert can_tp_connection.transmitter == can_tp_node

    # check if the can tp connection can be constructed from an element and is equal to the original can tp connection
    element = can_tp_connection.element
    can_tp_connection2 = CanTpConnection(element)
    assert can_tp_connection == can_tp_connection2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in CanTpConnection.__dict__
    assert len(str(can_tp_connection)) > 0


def test_can_tp_node() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    can_cluster = system.create_can_cluster("CanCluster", package)
    can_channel = can_cluster.create_physical_channel("CanChannel")
    ecu_instance = system.create_ecu_instance("EcuInstance", package)
    can_controller = ecu_instance.create_can_communication_controller("CanController")
    can_connector = can_controller.connect_physical_channel("CanConnector", can_channel)
    can_tp_config = system.create_can_tp_config("CanTpConfig", package, can_cluster)

    # CanTpNode
    can_tp_node = can_tp_config.create_can_tp_node("CanTpNode")
    assert isinstance(can_tp_node, CanTpNode)
    can_tp_node.name = "CanTpNode"
    assert can_tp_node.name == "CanTpNode"

    # attributes and methods
    can_tp_address = can_tp_config.create_can_tp_address("CanTpAddress", 0x7F)
    can_tp_node.address = can_tp_address
    assert can_tp_node.address == can_tp_address
    can_tp_node.connector = can_connector
    assert can_tp_node.connector == can_connector

    # check if the can tp node can be constructed from an element and is equal to the original can tp node
    element = can_tp_node.element
    can_tp_node2 = CanTpNode(element)
    assert can_tp_node == can_tp_node2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in CanTpNode.__dict__
    assert len(str(can_tp_node)) > 0
