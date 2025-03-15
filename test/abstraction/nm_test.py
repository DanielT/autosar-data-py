from autosar_data.abstraction import *
from autosar_data.abstraction.communication import *


def test_nm_config() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    can_cluster = system.create_can_cluster("CanCluster", package)
    flexray_cluster = system.create_flexray_cluster(
        "FlexrayCluster", package, FlexrayClusterSettings()
    )
    ethernet_cluster = system.create_ethernet_cluster("EthernetCluster", package)

    # NmConfig
    nm_config = system.create_nm_config("NmConfig", package)
    assert isinstance(nm_config, NmConfig)
    # get and set the name
    assert nm_config.name == "NmConfig"
    nm_config.name = "NmConfig2"
    assert nm_config.name == "NmConfig2"

    can_nm_cluster_settings = CanNmClusterSettings(
        nm_busload_reduction_active=True,
        nm_immediate_nm_transmissions=3,
        nm_message_timeout_time=1.11,
        nm_msg_cycle_time=2.22,
        nm_network_timeout=3.33,
        nm_remote_sleep_indication_time=4.44,
        nm_repeat_message_time=5.55,
        nm_wait_bus_sleep_time=6.66,
    )
    can_nm_cluster = nm_config.create_can_nm_cluster(
        "CanNmCluster", can_nm_cluster_settings, can_cluster
    )
    flexray_nm_cluster_settings = FlexrayNmClusterSettings(
        nm_data_cycle=3,
        nm_remote_sleep_indication_time=1.11,
        nm_repeat_message_time=2.22,
        nm_repetition_cycle=3,
        nm_voting_cycle=4,
    )
    flexray_nm_cluster = nm_config.create_flexray_nm_cluster(
        "FlexrayNmCluster", flexray_nm_cluster_settings, flexray_cluster
    )
    udp_nm_cluster_settings = UdpNmClusterSettings(
        nm_msg_cycle_time=1.11,
        nm_msg_timeout_time=2.22,
        nm_network_timeout=3.33,
        nm_remote_sleep_indication_time=4.44,
        nm_repeat_message_time=5.55,
        nm_wait_bus_sleep_time=6.66,
    )
    udp_nm_cluster = nm_config.create_udp_nm_cluster(
        "UdpNmCluster", udp_nm_cluster_settings, ethernet_cluster
    )
    assert list(nm_config.nm_clusters()) == [
        can_nm_cluster,
        flexray_nm_cluster,
        udp_nm_cluster,
    ]

    can_nm_cluster_coupling = nm_config.create_can_nm_cluster_coupling(True, True)
    flexray_nm_cluster_coupling = nm_config.create_flexray_nm_cluster_coupling(
        FlexrayNmScheduleVariant.ScheduleVariant1
    )
    udp_nm_cluster_coupling = nm_config.create_udp_nm_cluster_coupling()
    assert list(nm_config.nm_cluster_couplings()) == [
        can_nm_cluster_coupling,
        flexray_nm_cluster_coupling,
        udp_nm_cluster_coupling,
    ]

    ecu_instance = system.create_ecu_instance("EcuInstance", package)
    nm_ecu = nm_config.create_nm_ecu("NmEcu", ecu_instance)
    assert list(nm_config.nm_ecus()) == [nm_ecu]

    # check if the config can be constructed from an element and is equal to the original config
    element = nm_config.element
    nm_config2 = NmConfig(element)
    assert nm_config == nm_config2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in NmConfig.__dict__
    assert len(str(nm_config)) > 0

    assert "__repr__" in CanNmClusterSettings.__dict__
    assert len(str(can_nm_cluster_settings)) > 0

    assert "__repr__" in FlexrayNmClusterSettings.__dict__
    assert len(str(flexray_nm_cluster_settings)) > 0

    assert "__repr__" in UdpNmClusterSettings.__dict__
    assert len(str(udp_nm_cluster_settings)) > 0


def test_nm_ecu() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    nm_config = system.create_nm_config("NmConfig", package)
    ecu_instance = system.create_ecu_instance("EcuInstance", package)

    # NmEcu
    nm_ecu = nm_config.create_nm_ecu("NmEcu", ecu_instance)
    assert isinstance(nm_ecu, NmEcu)
    # get and set the name
    assert nm_ecu.name == "NmEcu"
    nm_ecu.name = "NmEcu2"
    assert nm_ecu.name == "NmEcu2"
    # check attributes
    nm_ecu.ecu_instance = ecu_instance
    assert nm_ecu.ecu_instance == ecu_instance

    nm_ecu.nm_bus_synchronization_enabled = True
    assert nm_ecu.nm_bus_synchronization_enabled == True
    nm_ecu.nm_com_control_enabled = True
    assert nm_ecu.nm_com_control_enabled == True
    nm_ecu.cycle_time_main_function = 1.11
    assert nm_ecu.cycle_time_main_function == 1.11

    # check if the ecu can be constructed from an element and is equal to the original ecu
    element = nm_ecu.element
    nm_ecu2 = NmEcu(element)
    assert nm_ecu == nm_ecu2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in NmEcu.__dict__
    assert len(str(nm_ecu)) > 0


def test_can_nm_cluster() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    can_cluster = system.create_can_cluster("CanCluster", package)
    ecu_instance = system.create_ecu_instance("EcuInstance", package)
    can_controller = ecu_instance.create_can_communication_controller("CanController")
    nm_config = system.create_nm_config("NmConfig", package)

    # CanNmCluster
    can_nm_cluster_settings = CanNmClusterSettings(
        nm_busload_reduction_active=True,
        nm_immediate_nm_transmissions=3,
        nm_message_timeout_time=1.11,
        nm_msg_cycle_time=2.22,
        nm_network_timeout=3.33,
        nm_remote_sleep_indication_time=4.44,
        nm_repeat_message_time=5.55,
        nm_wait_bus_sleep_time=6.66,
    )
    can_nm_cluster = nm_config.create_can_nm_cluster(
        "CanNmCluster", can_nm_cluster_settings, can_cluster
    )
    assert isinstance(can_nm_cluster, CanNmCluster)
    # get and set the name
    assert can_nm_cluster.name == "CanNmCluster"
    can_nm_cluster.name = "CanNmCluster2"
    assert can_nm_cluster.name == "CanNmCluster2"
    # check attributes
    can_nm_cluster.communication_cluster = can_cluster
    assert can_nm_cluster.communication_cluster == can_cluster
    can_nm_cluster.nm_immediate_nm_transmissions = 4
    assert can_nm_cluster.nm_immediate_nm_transmissions == 4
    can_nm_cluster.nm_message_timeout_time = 2.22
    assert can_nm_cluster.nm_message_timeout_time == 2.22
    can_nm_cluster.nm_msg_cycle_time = 3.33
    assert can_nm_cluster.nm_msg_cycle_time == 3.33
    can_nm_cluster.nm_network_timeout = 4.44
    assert can_nm_cluster.nm_network_timeout == 4.44
    can_nm_cluster.nm_remote_sleep_indication_time = 5.55
    assert can_nm_cluster.nm_remote_sleep_indication_time == 5.55
    can_nm_cluster.nm_repeat_message_time = 6.66
    assert can_nm_cluster.nm_repeat_message_time == 6.66
    can_nm_cluster.nm_wait_bus_sleep_time = 7.77
    assert can_nm_cluster.nm_wait_bus_sleep_time == 7.77
    can_nm_cluster.nm_busload_reduction_active = False
    assert can_nm_cluster.nm_busload_reduction_active == False
    can_nm_cluster.channel_sleep_master = True
    assert can_nm_cluster.channel_sleep_master == True
    can_nm_cluster.node_detection_enabled = True
    assert can_nm_cluster.node_detection_enabled == True
    can_nm_cluster.node_id_enabled = True
    assert can_nm_cluster.node_id_enabled == True
    can_nm_cluster.pnc_participation = True
    assert can_nm_cluster.pnc_participation == True
    can_nm_cluster.pnc_cluster_vector_length = 6
    assert can_nm_cluster.pnc_cluster_vector_length == 6
    can_nm_cluster.repeat_msg_ind_enabled = True
    assert can_nm_cluster.repeat_msg_ind_enabled == True
    can_nm_cluster.synchronizing_network = True
    assert can_nm_cluster.synchronizing_network == True

    nm_ecu = nm_config.create_nm_ecu("NmEcu", ecu_instance)
    can_nm_node = can_nm_cluster.create_can_nm_node("CanNmNode", can_controller, nm_ecu)
    assert list(can_nm_cluster.nm_nodes()) == [can_nm_node]

    # check if the cluster can be constructed from an element and is equal to the original cluster
    element = can_nm_cluster.element
    can_nm_cluster2 = CanNmCluster(element)
    assert can_nm_cluster == can_nm_cluster2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in CanNmCluster.__dict__
    assert len(str(can_nm_cluster)) > 0


def test_can_nm_cluster_coupling() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    can_cluster = system.create_can_cluster("CanCluster", package)
    nm_config = system.create_nm_config("NmConfig", package)

    # CanNmClusterCoupling
    can_nm_cluster_coupling = nm_config.create_can_nm_cluster_coupling(True, True)
    assert isinstance(can_nm_cluster_coupling, CanNmClusterCoupling)
    # attributes
    can_nm_cluster_coupling.nm_busload_reduction_enabled = False
    assert can_nm_cluster_coupling.nm_busload_reduction_enabled == False
    can_nm_cluster_coupling.nm_immediate_restart_enabled = False
    assert can_nm_cluster_coupling.nm_immediate_restart_enabled == False

    can_nm_cluster_settings = CanNmClusterSettings(
        nm_busload_reduction_active=True,
        nm_immediate_nm_transmissions=3,
        nm_message_timeout_time=1.11,
        nm_msg_cycle_time=2.22,
        nm_network_timeout=3.33,
        nm_remote_sleep_indication_time=4.44,
        nm_repeat_message_time=5.55,
        nm_wait_bus_sleep_time=6.66,
    )
    can_nm_cluster = nm_config.create_can_nm_cluster(
        "CanNmCluster", can_nm_cluster_settings, can_cluster
    )
    can_nm_cluster_coupling.add_coupled_cluster(can_nm_cluster)
    assert list(can_nm_cluster_coupling.coupled_clusters()) == [can_nm_cluster]

    # check if the coupling can be constructed from an element and is equal to the original coupling
    element = can_nm_cluster_coupling.element
    can_nm_cluster_coupling2 = CanNmClusterCoupling(element)
    assert can_nm_cluster_coupling == can_nm_cluster_coupling2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in CanNmClusterCoupling.__dict__
    assert len(str(can_nm_cluster_coupling)) > 0


def test_can_nm_node() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    can_cluster = system.create_can_cluster("CanCluster", package)
    ecu_instance = system.create_ecu_instance("EcuInstance", package)
    can_controller = ecu_instance.create_can_communication_controller("CanController")
    nm_config = system.create_nm_config("NmConfig", package)
    nm_ecu = nm_config.create_nm_ecu("NmEcu", ecu_instance)
    can_nm_cluster_settings = CanNmClusterSettings(
        nm_busload_reduction_active=True,
        nm_immediate_nm_transmissions=3,
        nm_message_timeout_time=1.11,
        nm_msg_cycle_time=2.22,
        nm_network_timeout=3.33,
        nm_remote_sleep_indication_time=4.44,
        nm_repeat_message_time=5.55,
        nm_wait_bus_sleep_time=6.66,
    )
    can_nm_cluster = nm_config.create_can_nm_cluster(
        "CanNmCluster", can_nm_cluster_settings, can_cluster
    )

    nm_pdu_rx = system.create_nm_pdu("NmPdu1", package, 8)
    nm_pdu_tx = system.create_nm_pdu("NmPdu2", package, 8)

    # CanNmNode
    can_nm_node = can_nm_cluster.create_can_nm_node("CanNmNode", can_controller, nm_ecu)
    assert isinstance(can_nm_node, CanNmNode)
    # get and set the name
    assert can_nm_node.name == "CanNmNode"
    can_nm_node.name = "CanNmNode2"
    assert can_nm_node.name == "CanNmNode2"
    # check attributes
    can_nm_node.communication_controller = can_controller
    assert can_nm_node.communication_controller == can_controller
    can_nm_node.nm_ecu = nm_ecu
    assert can_nm_node.nm_ecu == nm_ecu
    can_nm_node.node_id = 1
    assert can_nm_node.node_id == 1
    can_nm_node.passive_mode = True
    assert can_nm_node.passive_mode == True

    can_nm_node.add_rx_nm_pdu(nm_pdu_rx)
    assert list(can_nm_node.rx_nm_pdus()) == [nm_pdu_rx]
    can_nm_node.add_tx_nm_pdu(nm_pdu_tx)
    assert list(can_nm_node.tx_nm_pdus()) == [nm_pdu_tx]

    # check if the node can be constructed from an element and is equal to the original node
    element = can_nm_node.element
    can_nm_node2 = CanNmNode(element)
    assert can_nm_node == can_nm_node2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in CanNmNode.__dict__
    assert len(str(can_nm_node)) > 0


def test_flexray_nm_cluster() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    flexray_cluster = system.create_flexray_cluster(
        "FlexrayCluster", package, FlexrayClusterSettings()
    )
    ecu_instance = system.create_ecu_instance("EcuInstance", package)
    flexray_controller = ecu_instance.create_flexray_communication_controller(
        "FlexrayController"
    )
    nm_config = system.create_nm_config("NmConfig", package)

    # FlexrayNmCluster
    flexray_nm_cluster_settings = FlexrayNmClusterSettings(
        nm_data_cycle=3,
        nm_remote_sleep_indication_time=1.11,
        nm_repeat_message_time=2.22,
        nm_repetition_cycle=3,
        nm_voting_cycle=4,
    )
    flexray_nm_cluster = nm_config.create_flexray_nm_cluster(
        "FlexrayNmCluster", flexray_nm_cluster_settings, flexray_cluster
    )
    assert isinstance(flexray_nm_cluster, FlexrayNmCluster)
    # get and set the name
    assert flexray_nm_cluster.name == "FlexrayNmCluster"
    flexray_nm_cluster.name = "FlexrayNmCluster2"
    assert flexray_nm_cluster.name == "FlexrayNmCluster2"
    # check attributes
    flexray_nm_cluster.communication_cluster = flexray_cluster
    assert flexray_nm_cluster.communication_cluster == flexray_cluster
    flexray_nm_cluster.nm_data_cycle = 4
    assert flexray_nm_cluster.nm_data_cycle == 4
    flexray_nm_cluster.nm_remote_sleep_indication_time = 2.22
    assert flexray_nm_cluster.nm_remote_sleep_indication_time == 2.22
    flexray_nm_cluster.nm_repeat_message_time = 3.33
    assert flexray_nm_cluster.nm_repeat_message_time == 3.33
    flexray_nm_cluster.nm_repetition_cycle = 4
    assert flexray_nm_cluster.nm_repetition_cycle == 4
    flexray_nm_cluster.nm_voting_cycle = 5
    assert flexray_nm_cluster.nm_voting_cycle == 5
    flexray_nm_cluster.channel_sleep_master = True
    assert flexray_nm_cluster.channel_sleep_master == True
    flexray_nm_cluster.node_detection_enabled = True
    assert flexray_nm_cluster.node_detection_enabled == True
    flexray_nm_cluster.node_id_enabled = True
    assert flexray_nm_cluster.node_id_enabled == True
    flexray_nm_cluster.pnc_participation = True
    assert flexray_nm_cluster.pnc_participation == True
    flexray_nm_cluster.pnc_cluster_vector_length = 6
    assert flexray_nm_cluster.pnc_cluster_vector_length == 6
    flexray_nm_cluster.repeat_msg_ind_enabled = True
    assert flexray_nm_cluster.repeat_msg_ind_enabled == True
    flexray_nm_cluster.synchronizing_network = True
    assert flexray_nm_cluster.synchronizing_network == True

    nm_ecu = nm_config.create_nm_ecu("NmEcu", ecu_instance)
    flexray_nm_node = flexray_nm_cluster.create_flexray_nm_node(
        "FlexrayNmNode", flexray_controller, nm_ecu
    )
    assert list(flexray_nm_cluster.nm_nodes()) == [flexray_nm_node]

    # check if the cluster can be constructed from an element and is equal to the original cluster
    element = flexray_nm_cluster.element
    flexray_nm_cluster2 = FlexrayNmCluster(element)
    assert flexray_nm_cluster == flexray_nm_cluster2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in FlexrayNmCluster.__dict__
    assert len(str(flexray_nm_cluster)) > 0


def test_flexray_nm_cluster_coupling() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    flexray_cluster = system.create_flexray_cluster(
        "FlexrayCluster", package, FlexrayClusterSettings()
    )
    nm_config = system.create_nm_config("NmConfig", package)

    # FlexrayNmClusterCoupling
    flexray_nm_cluster_coupling = nm_config.create_flexray_nm_cluster_coupling(
        FlexrayNmScheduleVariant.ScheduleVariant2
    )
    assert isinstance(flexray_nm_cluster_coupling, FlexrayNmClusterCoupling)
    # attributes
    flexray_nm_cluster_coupling.nm_schedule_variant = (
        FlexrayNmScheduleVariant.ScheduleVariant1
    )
    assert (
        flexray_nm_cluster_coupling.nm_schedule_variant
        == FlexrayNmScheduleVariant.ScheduleVariant1
    )
    flexray_nm_cluster_coupling.nm_schedule_variant = (
        FlexrayNmScheduleVariant.ScheduleVariant2
    )
    assert (
        flexray_nm_cluster_coupling.nm_schedule_variant
        == FlexrayNmScheduleVariant.ScheduleVariant2
    )
    flexray_nm_cluster_coupling.nm_schedule_variant = (
        FlexrayNmScheduleVariant.ScheduleVariant3
    )
    assert (
        flexray_nm_cluster_coupling.nm_schedule_variant
        == FlexrayNmScheduleVariant.ScheduleVariant3
    )
    flexray_nm_cluster_coupling.nm_schedule_variant = (
        FlexrayNmScheduleVariant.ScheduleVariant4
    )
    assert (
        flexray_nm_cluster_coupling.nm_schedule_variant
        == FlexrayNmScheduleVariant.ScheduleVariant4
    )
    flexray_nm_cluster_coupling.nm_schedule_variant = (
        FlexrayNmScheduleVariant.ScheduleVariant5
    )
    assert (
        flexray_nm_cluster_coupling.nm_schedule_variant
        == FlexrayNmScheduleVariant.ScheduleVariant5
    )
    flexray_nm_cluster_coupling.nm_schedule_variant = (
        FlexrayNmScheduleVariant.ScheduleVariant6
    )
    assert (
        flexray_nm_cluster_coupling.nm_schedule_variant
        == FlexrayNmScheduleVariant.ScheduleVariant6
    )
    flexray_nm_cluster_coupling.nm_schedule_variant = (
        FlexrayNmScheduleVariant.ScheduleVariant7
    )
    assert (
        flexray_nm_cluster_coupling.nm_schedule_variant
        == FlexrayNmScheduleVariant.ScheduleVariant7
    )

    flexray_nm_cluster_settings = FlexrayNmClusterSettings(
        nm_data_cycle=3,
        nm_remote_sleep_indication_time=1.11,
        nm_repeat_message_time=2.22,
        nm_repetition_cycle=3,
        nm_voting_cycle=4,
    )
    flexray_nm_cluster = nm_config.create_flexray_nm_cluster(
        "FlexrayNmCluster", flexray_nm_cluster_settings, flexray_cluster
    )
    flexray_nm_cluster_coupling.add_coupled_cluster(flexray_nm_cluster)
    assert list(flexray_nm_cluster_coupling.coupled_clusters()) == [flexray_nm_cluster]

    # check if the coupling can be constructed from an element and is equal to the original coupling
    element = flexray_nm_cluster_coupling.element
    flexray_nm_cluster_coupling2 = FlexrayNmClusterCoupling(element)
    assert flexray_nm_cluster_coupling == flexray_nm_cluster_coupling2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in FlexrayNmClusterCoupling.__dict__
    assert len(str(flexray_nm_cluster_coupling)) > 0


def test_flexray_nm_node() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    flexray_cluster = system.create_flexray_cluster(
        "FlexrayCluster", package, FlexrayClusterSettings()
    )
    ecu_instance = system.create_ecu_instance("EcuInstance", package)
    flexray_controller = ecu_instance.create_flexray_communication_controller(
        "FlexrayController"
    )
    nm_config = system.create_nm_config("NmConfig", package)
    nm_ecu = nm_config.create_nm_ecu("NmEcu", ecu_instance)
    flexray_nm_cluster_settings = FlexrayNmClusterSettings(
        nm_data_cycle=3,
        nm_remote_sleep_indication_time=1.11,
        nm_repeat_message_time=2.22,
        nm_repetition_cycle=3,
        nm_voting_cycle=4,
    )
    flexray_nm_cluster = nm_config.create_flexray_nm_cluster(
        "FlexrayNmCluster", flexray_nm_cluster_settings, flexray_cluster
    )

    nm_pdu_rx = system.create_nm_pdu("NmPdu1", package, 8)
    nm_pdu_tx = system.create_nm_pdu("NmPdu2", package, 8)

    # FlexrayNmNode
    flexray_nm_node = flexray_nm_cluster.create_flexray_nm_node(
        "FlexrayNmNode", flexray_controller, nm_ecu
    )
    assert isinstance(flexray_nm_node, FlexrayNmNode)
    # get and set the name
    assert flexray_nm_node.name == "FlexrayNmNode"
    flexray_nm_node.name = "FlexrayNmNode2"
    assert flexray_nm_node.name == "FlexrayNmNode2"
    # check attributes
    flexray_nm_node.communication_controller = flexray_controller
    assert flexray_nm_node.communication_controller == flexray_controller
    flexray_nm_node.nm_ecu = nm_ecu
    assert flexray_nm_node.nm_ecu == nm_ecu
    flexray_nm_node.node_id = 1
    assert flexray_nm_node.node_id == 1
    flexray_nm_node.passive_mode = True
    assert flexray_nm_node.passive_mode == True

    flexray_nm_node.add_rx_nm_pdu(nm_pdu_rx)
    assert list(flexray_nm_node.rx_nm_pdus()) == [nm_pdu_rx]
    flexray_nm_node.add_tx_nm_pdu(nm_pdu_tx)
    assert list(flexray_nm_node.tx_nm_pdus()) == [nm_pdu_tx]

    # check if the node can be constructed from an element and is equal to the original node
    element = flexray_nm_node.element
    flexray_nm_node2 = FlexrayNmNode(element)
    assert flexray_nm_node == flexray_nm_node2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in FlexrayNmNode.__dict__
    assert len(str(flexray_nm_node)) > 0


def test_udp_nm_cluster() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    ethernet_cluster = system.create_ethernet_cluster("EthernetCluster", package)
    ethernet_channel = ethernet_cluster.create_physical_channel("EthernetChannel")
    ecu_instance = system.create_ecu_instance("EcuInstance", package)
    ethernet_controller = ecu_instance.create_ethernet_communication_controller(
        "EthernetController"
    )
    nm_config = system.create_nm_config("NmConfig", package)

    # UdpNmCluster
    udp_nm_cluster_settings = UdpNmClusterSettings(
        nm_msg_cycle_time=1.11,
        nm_msg_timeout_time=2.22,
        nm_network_timeout=3.33,
        nm_remote_sleep_indication_time=4.44,
        nm_repeat_message_time=5.55,
        nm_wait_bus_sleep_time=6.66,
    )
    udp_nm_cluster = nm_config.create_udp_nm_cluster(
        "UdpNmCluster", udp_nm_cluster_settings, ethernet_cluster
    )
    assert isinstance(udp_nm_cluster, UdpNmCluster)
    # get and set the name
    assert udp_nm_cluster.name == "UdpNmCluster"
    udp_nm_cluster.name = "UdpNmCluster2"
    assert udp_nm_cluster.name == "UdpNmCluster2"
    # check attributes
    udp_nm_cluster.communication_cluster = ethernet_cluster
    assert udp_nm_cluster.communication_cluster == ethernet_cluster
    udp_nm_cluster.nm_msg_cycle_time = 0.001
    assert udp_nm_cluster.nm_msg_cycle_time == 0.001
    udp_nm_cluster.nm_message_timeout_time = 1.111
    assert udp_nm_cluster.nm_message_timeout_time == 1.111
    udp_nm_cluster.nm_network_timeout = 3.1415
    assert udp_nm_cluster.nm_network_timeout == 3.1415
    udp_nm_cluster.nm_remote_sleep_indication_time = 0.9999
    assert udp_nm_cluster.nm_remote_sleep_indication_time == 0.9999
    udp_nm_cluster.nm_repeat_message_time = 6.66
    assert udp_nm_cluster.nm_repeat_message_time == 6.66
    udp_nm_cluster.nm_wait_bus_sleep_time = 7.77
    assert udp_nm_cluster.nm_wait_bus_sleep_time == 7.77
    udp_nm_cluster.nm_immediate_nm_transmissions = 3
    assert udp_nm_cluster.nm_immediate_nm_transmissions == 3
    udp_nm_cluster.nm_cbv_position = 4
    assert udp_nm_cluster.nm_cbv_position == 4
    udp_nm_cluster.nm_nid_position = 5
    assert udp_nm_cluster.nm_nid_position == 5
    udp_nm_cluster.channel_sleep_master = True
    assert udp_nm_cluster.channel_sleep_master == True
    udp_nm_cluster.node_detection_enabled = True
    assert udp_nm_cluster.node_detection_enabled == True
    udp_nm_cluster.node_id_enabled = True
    assert udp_nm_cluster.node_id_enabled == True
    udp_nm_cluster.pnc_participation = True
    assert udp_nm_cluster.pnc_participation == True
    udp_nm_cluster.pnc_cluster_vector_length = 6
    assert udp_nm_cluster.pnc_cluster_vector_length == 6
    udp_nm_cluster.repeat_msg_ind_enabled = True
    assert udp_nm_cluster.repeat_msg_ind_enabled == True
    udp_nm_cluster.synchronizing_network = True
    assert udp_nm_cluster.synchronizing_network == True

    nm_ecu = nm_config.create_nm_ecu("NmEcu", ecu_instance)
    udp_nm_node = udp_nm_cluster.create_udp_nm_node(
        "UdpNmNode", ethernet_controller, nm_ecu, 0.1
    )
    assert list(udp_nm_cluster.nm_nodes()) == [udp_nm_node]

    udp_nm_cluster.vlan = ethernet_channel
    assert udp_nm_cluster.vlan == ethernet_channel

    # check if the cluster can be constructed from an element and is equal to the original cluster
    element = udp_nm_cluster.element
    udp_nm_cluster2 = UdpNmCluster(element)
    assert udp_nm_cluster == udp_nm_cluster2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in UdpNmCluster.__dict__
    assert len(str(udp_nm_cluster)) > 0


def test_udp_nm_cluster_coupling() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    ethernet_cluster = system.create_ethernet_cluster("EthernetCluster", package)
    nm_config = system.create_nm_config("NmConfig", package)

    # UdpNmClusterCoupling
    udp_nm_cluster_coupling = nm_config.create_udp_nm_cluster_coupling()
    assert isinstance(udp_nm_cluster_coupling, UdpNmClusterCoupling)

    udp_nm_cluster_coupling.nm_immediate_restart_enabled = False
    assert udp_nm_cluster_coupling.nm_immediate_restart_enabled == False

    udp_nm_cluster_settings = UdpNmClusterSettings(
        nm_msg_cycle_time=1.11,
        nm_msg_timeout_time=2.22,
        nm_network_timeout=3.33,
        nm_remote_sleep_indication_time=4.44,
        nm_repeat_message_time=5.55,
        nm_wait_bus_sleep_time=6.66,
    )
    udp_nm_cluster = nm_config.create_udp_nm_cluster(
        "UdpNmCluster", udp_nm_cluster_settings, ethernet_cluster
    )
    udp_nm_cluster_coupling.add_coupled_cluster(udp_nm_cluster)
    assert list(udp_nm_cluster_coupling.coupled_clusters()) == [udp_nm_cluster]

    # check if the coupling can be constructed from an element and is equal to the original coupling
    element = udp_nm_cluster_coupling.element
    udp_nm_cluster_coupling2 = UdpNmClusterCoupling(element)
    assert udp_nm_cluster_coupling == udp_nm_cluster_coupling2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in UdpNmClusterCoupling.__dict__
    assert len(str(udp_nm_cluster_coupling)) > 0


def test_udp_nm_node() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    ethernet_cluster = system.create_ethernet_cluster("EthernetCluster", package)
    ecu_instance = system.create_ecu_instance("EcuInstance", package)
    ethernet_controller = ecu_instance.create_ethernet_communication_controller(
        "EthernetController"
    )
    nm_config = system.create_nm_config("NmConfig", package)
    nm_ecu = nm_config.create_nm_ecu("NmEcu", ecu_instance)
    udp_nm_cluster_settings = UdpNmClusterSettings(
        nm_msg_cycle_time=1.11,
        nm_msg_timeout_time=2.22,
        nm_network_timeout=3.33,
        nm_remote_sleep_indication_time=4.44,
        nm_repeat_message_time=5.55,
        nm_wait_bus_sleep_time=6.66,
    )
    udp_nm_cluster = nm_config.create_udp_nm_cluster(
        "UdpNmCluster", udp_nm_cluster_settings, ethernet_cluster
    )

    nm_pdu_rx = system.create_nm_pdu("NmPdu1", package, 8)
    nm_pdu_tx = system.create_nm_pdu("NmPdu2", package, 8)

    # UdpNmNode
    udp_nm_node = udp_nm_cluster.create_udp_nm_node(
        "UdpNmNode", ethernet_controller, nm_ecu, 0.1
    )
    assert isinstance(udp_nm_node, UdpNmNode)
    # get and set the name
    assert udp_nm_node.name == "UdpNmNode"
    udp_nm_node.name = "UdpNmNode2"
    assert udp_nm_node.name == "UdpNmNode2"
    # check attributes
    udp_nm_node.communication_controller = ethernet_controller
    assert udp_nm_node.communication_controller == ethernet_controller
    udp_nm_node.nm_ecu = nm_ecu
    assert udp_nm_node.nm_ecu == nm_ecu
    udp_nm_node.node_id = 1
    assert udp_nm_node.node_id == 1
    udp_nm_node.passive_mode = True
    assert udp_nm_node.passive_mode == True
    udp_nm_node.nm_msg_cycle_offset = 0.1
    assert udp_nm_node.nm_msg_cycle_offset == 0.1
    udp_nm_node.all_nm_messages_keep_awake = True
    assert udp_nm_node.all_nm_messages_keep_awake == True
    udp_nm_node.node_id = 1
    assert udp_nm_node.node_id == 1
    udp_nm_node.passive_mode = True
    assert udp_nm_node.passive_mode == True
    udp_nm_node.communication_controller = ethernet_controller
    assert udp_nm_node.communication_controller == ethernet_controller
    udp_nm_node.nm_ecu = nm_ecu
    assert udp_nm_node.nm_ecu == nm_ecu

    udp_nm_node.add_rx_nm_pdu(nm_pdu_rx)
    assert list(udp_nm_node.rx_nm_pdus()) == [nm_pdu_rx]
    udp_nm_node.add_tx_nm_pdu(nm_pdu_tx)
    assert list(udp_nm_node.tx_nm_pdus()) == [nm_pdu_tx]

    # check if the node can be constructed from an element and is equal to the original node
    element = udp_nm_node.element
    udp_nm_node2 = UdpNmNode(element)
    assert udp_nm_node == udp_nm_node2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in UdpNmNode.__dict__
    assert len(str(udp_nm_node)) > 0
