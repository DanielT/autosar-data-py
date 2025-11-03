from autosar_data.abstraction import *
from autosar_data.abstraction.communication import *


def test_flexray_tp_config() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    flexray_cluster = system.create_flexray_cluster(
        "FlexrayCluster", package, FlexrayClusterSettings()
    )
    ecu_instance = system.create_ecu_instance("EcuInstance", package)
    tp_sdu = system.create_dcm_ipdu("TpSdu", package, 64, DiagPduType.DiagRequest)

    # Test FlexrayTpConfig
    flexray_tp_config = system.create_flexray_tp_config(
        "FlexrayTpConfig", package, flexray_cluster
    )
    assert isinstance(flexray_tp_config, FlexrayTpConfig)

    flexray_tp_config.name = "FlexrayTpConfig2"
    assert flexray_tp_config.name == "FlexrayTpConfig2"
    flexray_tp_config.cluster = flexray_cluster
    assert flexray_tp_config.cluster == flexray_cluster
    flexray_tp_pdu_pool = flexray_tp_config.create_flexray_tp_pdu_pool("PduPool")
    assert list(flexray_tp_config.flexray_tp_pdu_pools()) == [flexray_tp_pdu_pool]
    tp_address = flexray_tp_config.create_tp_address("TpAddress", 1)
    assert list(flexray_tp_config.tp_addresses()) == [tp_address]
    tp_node = flexray_tp_config.create_flexray_tp_node("TpNode")
    assert list(flexray_tp_config.flexray_tp_nodes()) == [tp_node]
    connection_control = flexray_tp_config.create_flexray_tp_connection_control("CC")
    assert list(flexray_tp_config.flexray_tp_connection_controls()) == [
        connection_control
    ]
    tp_connection = flexray_tp_config.create_flexray_tp_connection(
        "TpConnection", tp_node, tp_sdu, connection_control
    )
    assert list(flexray_tp_config.flexray_tp_connections()) == [tp_connection]
    tp_ecu = flexray_tp_config.create_flexray_tp_ecu(ecu_instance, True)
    assert list(flexray_tp_config.flexray_tp_ecus()) == [tp_ecu]

    # check if the flexray tp config can be constructed from an element and is equal to the original flexray tp config
    element = flexray_tp_config.element
    flexray_tp_config2 = FlexrayTpConfig(element)
    assert flexray_tp_config == flexray_tp_config2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in FlexrayTpConfig.__dict__
    assert len(str(flexray_tp_config)) > 0


def test_flexray_tp_pdu_pool() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    flexray_cluster = system.create_flexray_cluster(
        "FlexrayCluster", package, FlexrayClusterSettings()
    )
    flexray_tp_config = system.create_flexray_tp_config(
        "FlexrayTpConfig", package, flexray_cluster
    )
    n_pdu = system.create_n_pdu("NPdu", package, 64)

    # Test FlexrayTpPduPool
    flexray_tp_pdu_pool = flexray_tp_config.create_flexray_tp_pdu_pool("PduPool")
    assert isinstance(flexray_tp_pdu_pool, FlexrayTpPduPool)

    flexray_tp_pdu_pool.name = "PduPool2"
    assert flexray_tp_pdu_pool.name == "PduPool2"
    flexray_tp_pdu_pool.add_n_pdu(n_pdu)
    assert list(flexray_tp_pdu_pool.n_pdus()) == [n_pdu]

    # check if the flexray tp pdu pool can be constructed from an element and is equal to the original flexray tp pdu pool
    element = flexray_tp_pdu_pool.element
    flexray_tp_pdu_pool2 = FlexrayTpPduPool(element)
    assert flexray_tp_pdu_pool == flexray_tp_pdu_pool2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in FlexrayTpPduPool.__dict__
    assert len(str(flexray_tp_pdu_pool)) > 0


def test_flexray_tp_connection() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    flexray_cluster = system.create_flexray_cluster(
        "FlexrayCluster", package, FlexrayClusterSettings()
    )
    flexray_tp_config = system.create_flexray_tp_config(
        "FlexrayTpConfig", package, flexray_cluster
    )
    tp_sdu = system.create_dcm_ipdu("TpSdu", package, 64, DiagPduType.DiagRequest)
    tp_node = flexray_tp_config.create_flexray_tp_node("TpNode")
    connection_control = flexray_tp_config.create_flexray_tp_connection_control("CC")
    pdu_pool = flexray_tp_config.create_flexray_tp_pdu_pool("PduPool")
    tp_address = flexray_tp_config.create_tp_address("TpAddress", 1)

    # Test FlexrayTpConnection
    tp_connection = flexray_tp_config.create_flexray_tp_connection(
        "TpConnection", tp_node, tp_sdu, connection_control
    )
    assert isinstance(tp_connection, FlexrayTpConnection)

    tp_connection.name = "TpConnection2"
    assert tp_connection.name == "TpConnection2"
    tp_connection.transmitter = tp_node
    assert tp_connection.transmitter == tp_node
    tp_connection.direct_tp_sdu = tp_sdu
    assert tp_connection.direct_tp_sdu == tp_sdu
    tp_connection.reversed_tp_sdu = tp_sdu
    assert tp_connection.reversed_tp_sdu == tp_sdu
    tp_connection.tx_pdu_pool = pdu_pool
    assert tp_connection.tx_pdu_pool == pdu_pool
    tp_connection.rx_pdu_pool = pdu_pool
    assert tp_connection.rx_pdu_pool == pdu_pool
    tp_connection.multicast_address = tp_address
    assert tp_connection.multicast_address == tp_address
    tp_connection.add_receiver(tp_node)
    assert list(tp_connection.receivers()) == [tp_node]
    tp_connection.connection_control = connection_control
    assert tp_connection.connection_control == connection_control

    # check if the flexray tp connection can be constructed from an element and is equal to the original flexray tp connection
    element = tp_connection.element
    tp_connection2 = FlexrayTpConnection(element)
    assert tp_connection == tp_connection2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in FlexrayTpConnection.__dict__
    assert len(str(tp_connection)) > 0


def test_flexray_tp_connection_control() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    flexray_cluster = system.create_flexray_cluster(
        "FlexrayCluster", package, FlexrayClusterSettings()
    )
    flexray_tp_config = system.create_flexray_tp_config(
        "FlexrayTpConfig", package, flexray_cluster
    )

    # Test FlexrayTpConnectionControl
    connection_control = flexray_tp_config.create_flexray_tp_connection_control("CC")
    assert isinstance(connection_control, FlexrayTpConnectionControl)

    connection_control.name = "CC2"
    assert connection_control.name == "CC2"
    connection_control.max_fc_wait = 10
    assert connection_control.max_fc_wait == 10
    connection_control.max_number_of_npdu_per_cycle = 1
    assert connection_control.max_number_of_npdu_per_cycle == 1
    connection_control.max_retries = 2
    assert connection_control.max_retries == 2
    connection_control.separation_cycle_exponent = 3
    assert connection_control.separation_cycle_exponent == 3

    # check if the flexray tp connection control can be constructed from an element and is equal to the original flexray tp connection control
    element = connection_control.element
    connection_control2 = FlexrayTpConnectionControl(element)
    assert connection_control == connection_control2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in FlexrayTpConnectionControl.__dict__
    assert len(str(connection_control)) > 0


def test_flexray_tp_ecu() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    ecu_instance = system.create_ecu_instance("EcuInstance", package)
    flexray_cluster = system.create_flexray_cluster(
        "FlexrayCluster", package, FlexrayClusterSettings()
    )
    flexray_tp_config = system.create_flexray_tp_config(
        "FlexrayTpConfig", package, flexray_cluster
    )

    # Test FlexrayTpEcu
    tp_ecu = flexray_tp_config.create_flexray_tp_ecu(ecu_instance, True)
    assert isinstance(tp_ecu, FlexrayTpEcu)

    tp_ecu.ecu_instance = ecu_instance
    assert tp_ecu.ecu_instance == ecu_instance
    tp_ecu.ecu_instance = ecu_instance
    assert tp_ecu.ecu_instance == ecu_instance
    tp_ecu.full_duplex_enabled = False
    assert tp_ecu.full_duplex_enabled == False
    tp_ecu.cycle_time_main_function = 0.33
    assert tp_ecu.cycle_time_main_function == 0.33
    tp_ecu.cancellation = True
    assert tp_ecu.cancellation == True

    # check if the flexray tp ecu can be constructed from an element and is equal to the original flexray tp ecu
    element = tp_ecu.element
    tp_ecu2 = FlexrayTpEcu(element)
    assert tp_ecu == tp_ecu2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in FlexrayTpEcu.__dict__
    assert len(str(tp_ecu)) > 0


def test_flexray_tp_node() -> None:
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
    flexray_controller = ecu_instance.create_flexray_communication_controller(
        "Controller"
    )
    connector = flexray_controller.connect_physical_channel(
        "Connector", flexray_channel
    )
    flexray_tp_config = system.create_flexray_tp_config(
        "FlexrayTpConfig", package, flexray_cluster
    )
    tp_address = flexray_tp_config.create_tp_address("TpAddress", 1)

    # Test FlexrayTpNode
    tp_node = flexray_tp_config.create_flexray_tp_node("TpNode")
    assert isinstance(tp_node, FlexrayTpNode)

    tp_node.name = "TpNode2"
    assert tp_node.name == "TpNode2"
    tp_node.tp_address = tp_address
    assert tp_node.tp_address == tp_address
    tp_node.add_communication_connector(connector)
    assert list(tp_node.communication_connectors()) == [connector]

    # check if the flexray tp node can be constructed from an element and is equal to the original flexray tp node
    element = tp_node.element
    tp_node2 = FlexrayTpNode(element)
    assert tp_node == tp_node2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in FlexrayTpNode.__dict__
    assert len(str(tp_node)) > 0
