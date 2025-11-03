from autosar_data.abstraction import *
from autosar_data.abstraction.communication import *


def test_flexray_ar_tp_config() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    flexray_cluster = system.create_flexray_cluster(
        "FlexrayCluster", package, FlexrayClusterSettings()
    )

    # FlexrayArTpConfig
    flexray_ar_tp_config = system.create_flexray_ar_tp_config(
        "FlexrayArTpConfig", package, flexray_cluster
    )
    assert isinstance(flexray_ar_tp_config, FlexrayArTpConfig)

    flexray_ar_tp_config.name = "FlexrayArTpConfig2"
    assert flexray_ar_tp_config.name == "FlexrayArTpConfig2"

    flexray_ar_tp_config.cluster = flexray_cluster
    assert flexray_ar_tp_config.cluster == flexray_cluster

    tp_address = flexray_ar_tp_config.create_tp_address("TpAddress", 1234)
    assert list(flexray_ar_tp_config.tp_addresses()) == [tp_address]

    flexray_ar_tp_channel = flexray_ar_tp_config.create_flexray_ar_tp_channel(
        FrArTpAckType.AckWithoutRt, True, MaximumMessageLengthType.I4g, 0.005, True
    )
    assert list(flexray_ar_tp_config.flexray_ar_tp_channels()) == [
        flexray_ar_tp_channel
    ]

    flexray_ar_tp_node = flexray_ar_tp_config.create_flexray_ar_tp_node(
        "FlexrayArTpNode"
    )
    assert list(flexray_ar_tp_config.flexray_ar_tp_nodes()) == [flexray_ar_tp_node]

    # check if the flexray ar tp config can be constructed from an element and is equal to the original flexray ar tp config
    element = flexray_ar_tp_config.element
    flexray_ar_tp_config2 = FlexrayArTpConfig(element)
    assert flexray_ar_tp_config == flexray_ar_tp_config2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in FlexrayArTpConfig.__dict__
    assert len(str(flexray_ar_tp_config)) > 0


def test_tp_address() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    flexray_cluster = system.create_flexray_cluster(
        "FlexrayCluster", package, FlexrayClusterSettings()
    )

    # FlexrayArTpConfig
    flexray_ar_tp_config = system.create_flexray_ar_tp_config(
        "FlexrayArTpConfig", package, flexray_cluster
    )
    tp_address = flexray_ar_tp_config.create_tp_address("TpAddress", 1234)
    assert isinstance(tp_address, TpAddress)

    tp_address.name = "TpAddress2"
    assert tp_address.name == "TpAddress2"

    tp_address.address = 1235
    assert tp_address.address == 1235

    # check if the tp address can be constructed from an element and is equal to the original tp address
    element = tp_address.element
    tp_address2 = TpAddress(element)
    assert tp_address == tp_address2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in TpAddress.__dict__
    assert len(str(tp_address)) > 0


def test_flexray_ar_tp_channel() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    flexray_cluster = system.create_flexray_cluster(
        "FlexrayCluster", package, FlexrayClusterSettings()
    )

    # FlexrayArTpConfig
    flexray_ar_tp_config = system.create_flexray_ar_tp_config(
        "FlexrayArTpConfig", package, flexray_cluster
    )
    flexray_ar_tp_channel = flexray_ar_tp_config.create_flexray_ar_tp_channel(
        FrArTpAckType.AckWithoutRt, True, MaximumMessageLengthType.I4g, 0.005, True
    )
    assert isinstance(flexray_ar_tp_channel, FlexrayArTpChannel)

    flexray_ar_tp_channel.ack_type = FrArTpAckType.AckWithRt
    assert flexray_ar_tp_channel.ack_type == FrArTpAckType.AckWithRt
    flexray_ar_tp_channel.ack_type = FrArTpAckType.AckWithoutRt
    assert flexray_ar_tp_channel.ack_type == FrArTpAckType.AckWithoutRt
    flexray_ar_tp_channel.ack_type = FrArTpAckType.NoAck
    assert flexray_ar_tp_channel.ack_type == FrArTpAckType.NoAck

    flexray_ar_tp_channel.extended_addressing = False
    assert flexray_ar_tp_channel.extended_addressing == False

    flexray_ar_tp_channel.maximum_message_length = MaximumMessageLengthType.I4g
    assert flexray_ar_tp_channel.maximum_message_length == MaximumMessageLengthType.I4g
    flexray_ar_tp_channel.maximum_message_length = MaximumMessageLengthType.Iso
    assert flexray_ar_tp_channel.maximum_message_length == MaximumMessageLengthType.Iso
    flexray_ar_tp_channel.maximum_message_length = MaximumMessageLengthType.Iso6
    assert flexray_ar_tp_channel.maximum_message_length == MaximumMessageLengthType.Iso6

    flexray_ar_tp_channel.minimum_separation_time = 1.5
    assert flexray_ar_tp_channel.minimum_separation_time == 1.5

    flexray_ar_tp_channel.multicast_segmentation = False
    assert flexray_ar_tp_channel.multicast_segmentation == False

    flexray_ar_tp_node = flexray_ar_tp_config.create_flexray_ar_tp_node(
        "FlexrayArTpNode"
    )
    direct_tp_sdu = system.create_dcm_ipdu("DirectTpSdu", package, 100, DiagPduType.DiagRequest)
    flexray_ar_tp_connection = flexray_ar_tp_channel.create_flexray_ar_tp_connection(
        "FlexrayArTpConnection", direct_tp_sdu, flexray_ar_tp_node, flexray_ar_tp_node
    )
    assert list(flexray_ar_tp_channel.flexray_ar_tp_connections()) == [
        flexray_ar_tp_connection
    ]

    n_pdu = system.create_n_pdu("NPdu", package, 100)
    flexray_ar_tp_channel.add_n_pdu(n_pdu)
    assert list(flexray_ar_tp_channel.n_pdus()) == [n_pdu]

    # check if the flexray ar tp channel can be constructed from an element and is equal to the original flexray ar tp channel
    element = flexray_ar_tp_channel.element
    flexray_ar_tp_channel2 = FlexrayArTpChannel(element)
    assert flexray_ar_tp_channel == flexray_ar_tp_channel2
    # quick check if a custom __repr__ method is implemented


def test_fr_ar_tp_connection() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    flexray_cluster = system.create_flexray_cluster(
        "FlexrayCluster", package, FlexrayClusterSettings()
    )
    flexray_ar_tp_config = system.create_flexray_ar_tp_config(
        "FlexrayArTpConfig", package, flexray_cluster
    )
    flexray_ar_tp_channel = flexray_ar_tp_config.create_flexray_ar_tp_channel(
        FrArTpAckType.AckWithoutRt, True, MaximumMessageLengthType.I4g, 0.005, True
    )
    flexray_ar_tp_node = flexray_ar_tp_config.create_flexray_ar_tp_node(
        "FlexrayArTpNode"
    )
    direct_tp_sdu = system.create_dcm_ipdu("DirectTpSdu", package, 100, DiagPduType.DiagRequest)
    reversed_tp_sdu = system.create_dcm_ipdu("ReversedTpSdu", package, 100, DiagPduType.DiagResponse)

    # FlexrayArTpConnection
    flexray_ar_tp_connection = flexray_ar_tp_channel.create_flexray_ar_tp_connection(
        "FlexrayArTpConnection", direct_tp_sdu, flexray_ar_tp_node, flexray_ar_tp_node
    )
    assert isinstance(flexray_ar_tp_connection, FlexrayArTpConnection)

    flexray_ar_tp_connection.name = "FlexrayArTpConnection2"
    assert flexray_ar_tp_connection.name == "FlexrayArTpConnection2"

    flexray_ar_tp_connection.direct_tp_sdu = direct_tp_sdu
    assert flexray_ar_tp_connection.direct_tp_sdu == direct_tp_sdu

    flexray_ar_tp_connection.source = flexray_ar_tp_node
    assert flexray_ar_tp_connection.source == flexray_ar_tp_node

    flexray_ar_tp_connection.add_target(flexray_ar_tp_node)
    # note: the connection already had one target fom the constructor
    assert list(flexray_ar_tp_connection.targets()) == [
        flexray_ar_tp_node,
        flexray_ar_tp_node,
    ]

    flexray_ar_tp_connection.reversed_tp_sdu = reversed_tp_sdu
    assert flexray_ar_tp_connection.reversed_tp_sdu == reversed_tp_sdu

    # check if the flexray ar tp connection can be constructed from an element and is equal to the original flexray ar tp connection
    element = flexray_ar_tp_connection.element
    flexray_ar_tp_connection2 = FlexrayArTpConnection(element)
    assert flexray_ar_tp_connection == flexray_ar_tp_connection2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in FlexrayArTpConnection.__dict__
    assert len(str(flexray_ar_tp_connection)) > 0


def test_flexray_ar_tp_node() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    flexray_cluster = system.create_flexray_cluster(
        "FlexrayCluster", package, FlexrayClusterSettings()
    )
    flexray_channel = flexray_cluster.create_physical_channel(
        "FlexrayChannel", FlexrayChannelName.A
    )
    ecu_instance = system.create_ecu_instance("EcuInstance", package)
    flexray_contoller = ecu_instance.create_flexray_communication_controller(
        "Controller"
    )
    connector = flexray_contoller.connect_physical_channel("Connector", flexray_channel)
    flexray_ar_tp_config = system.create_flexray_ar_tp_config(
        "FlexrayArTpConfig", package, flexray_cluster
    )

    # FlexrayArTpNode
    flexray_ar_tp_node = flexray_ar_tp_config.create_flexray_ar_tp_node(
        "FlexrayArTpNode"
    )
    assert isinstance(flexray_ar_tp_node, FlexrayArTpNode)

    flexray_ar_tp_node.name = "FlexrayArTpNode2"
    assert flexray_ar_tp_node.name == "FlexrayArTpNode2"

    tp_address = flexray_ar_tp_config.create_tp_address("TpAddress", 1234)
    flexray_ar_tp_node.tp_address = tp_address
    assert flexray_ar_tp_node.tp_address == tp_address

    flexray_ar_tp_node.add_communication_connector(connector)
    assert list(flexray_ar_tp_node.communication_connectors()) == [connector]

    # check if the flexray ar tp node can be constructed from an element and is equal to the original flexray ar tp node
    element = flexray_ar_tp_node.element
    flexray_ar_tp_node2 = FlexrayArTpNode(element)
    assert flexray_ar_tp_node == flexray_ar_tp_node2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in FlexrayArTpNode.__dict__
    assert len(str(flexray_ar_tp_node)) > 0
