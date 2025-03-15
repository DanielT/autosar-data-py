from autosar_data.abstraction import *
from autosar_data.abstraction.communication import *


def test_flexray_communication() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)

    # FlexRayCluster
    settings = FlexrayClusterSettings()
    # check if we can can get and set all settings
    settings.action_point_offset = 0
    settings.baudrate = 1000000
    settings.bit = 0
    settings.cas_rx_low_max = 0
    settings.cold_start_attempts = 0
    settings.cycle = 0
    settings.cycle_count_max = 0
    settings.detect_nit_error = False
    settings.dynamic_slot_idle_phase = 0
    settings.ignore_after_tx = False
    settings.listen_noise = False
    settings.macro_per_cycle = 0
    settings.macrotick_duration = 0
    settings.max_without_clock_correction_fatal = 0
    settings.max_without_clock_correction_passive = 0
    settings.minislot_action_point_offset = 0
    settings.minislot_duration = 0
    settings.network_idle_time = 0
    settings.network_management_vector_length = 0
    settings.number_of_minislots = 0
    settings.number_of_static_slots = 0
    settings.offset_correction_start = 0
    settings.payload_length_static = 0
    settings.sample_clock_period = 0
    settings.safety_margin = 0
    settings.static_slot_duration = 0
    settings.symbol_window = 0
    settings.symbol_window_action_point_offset = 0
    settings.sync_frame_id_count_max = 0
    settings.transceiver_standby_delay = 0
    settings.transmission_start_sequence_duration = 0
    settings.wakeup_rx_idle = 0
    settings.wakeup_rx_low = 0
    settings.wakeup_rx_window = 0
    settings.wakeup_tx_active = 0
    settings.wakeup_tx_idle = 0
    assert settings.action_point_offset == 0
    assert settings.baudrate == 1000000
    assert settings.bit == 0
    assert settings.cas_rx_low_max == 0
    assert settings.cold_start_attempts == 0
    assert settings.cycle == 0
    assert settings.cycle_count_max == 0
    assert settings.detect_nit_error == False
    assert settings.dynamic_slot_idle_phase == 0
    assert settings.ignore_after_tx == False
    assert settings.listen_noise == False
    assert settings.macro_per_cycle == 0
    assert settings.macrotick_duration == 0
    assert settings.max_without_clock_correction_fatal == 0
    assert settings.max_without_clock_correction_passive == 0
    assert settings.minislot_action_point_offset == 0
    assert settings.minislot_duration == 0
    assert settings.network_idle_time == 0
    assert settings.network_management_vector_length == 0
    assert settings.number_of_minislots == 0
    assert settings.number_of_static_slots == 0
    assert settings.offset_correction_start == 0
    assert settings.payload_length_static == 0
    assert settings.sample_clock_period == 0
    assert settings.safety_margin == 0
    assert settings.static_slot_duration == 0
    assert settings.symbol_window == 0
    assert settings.symbol_window_action_point_offset == 0
    assert settings.sync_frame_id_count_max == 0
    assert settings.transceiver_standby_delay == 0
    assert settings.transmission_start_sequence_duration == 0
    assert settings.wakeup_rx_idle == 0
    assert settings.wakeup_rx_low == 0
    assert settings.wakeup_rx_window == 0
    assert settings.wakeup_tx_active == 0
    assert settings.wakeup_tx_idle == 0
    # these settings are not valid
    assert settings.verify() == False
    assert "__repr__" in FlexrayClusterSettings.__dict__
    assert len(str(settings)) > 0

    # we can create a cluster with invalid settings
    flexray_cluster = system.create_flexray_cluster("FlexRayCluster", package, settings)
    assert isinstance(flexray_cluster, FlexrayCluster)
    # get and set the name
    assert flexray_cluster.name == "FlexRayCluster"
    flexray_cluster.name = "FlexRayCluster2"
    assert flexray_cluster.name == "FlexRayCluster2"
    assert flexray_cluster.system == system
    # verify that the cluster contains the settings it was created with
    assert flexray_cluster.settings() == settings
    # set different (default) settings
    settings2 = FlexrayClusterSettings()
    assert settings != settings2
    assert settings2.verify() == True
    flexray_cluster.set_settings(settings2)
    # verify that the new settings are set
    assert flexray_cluster.settings() == settings2
    # check if the cluster can be constructed from an element and is equal to the original cluster
    element = flexray_cluster.element
    flexray_cluster2 = FlexrayCluster(element)
    assert flexray_cluster == flexray_cluster2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in FlexrayCluster.__dict__
    assert len(str(flexray_cluster)) > 0

    # FlexRayPhysicalChannel
    flexray_physical_channel = flexray_cluster.create_physical_channel(
        "FlexRayPhysicalChannel", FlexrayChannelName.A
    )
    assert isinstance(flexray_physical_channel, FlexrayPhysicalChannel)
    flexray_physical_channel_b = flexray_cluster.create_physical_channel(
        "FlexRayPhysicalChannel_B", FlexrayChannelName.B
    )
    # get and set the name
    assert flexray_physical_channel.name == "FlexRayPhysicalChannel"
    flexray_physical_channel.name = "FlexRayPhysicalChannel2"
    assert flexray_physical_channel.name == "FlexRayPhysicalChannel2"
    # check attributes
    assert flexray_physical_channel.cluster == flexray_cluster
    assert flexray_physical_channel.channel_name == FlexrayChannelName.A
    assert flexray_physical_channel_b.cluster == flexray_cluster
    assert flexray_physical_channel_b.channel_name == FlexrayChannelName.B
    # check if the physical channel can be constructed from an element and is equal to the original physical channel
    element = flexray_physical_channel.element
    flexray_physical_channel_copy = FlexrayPhysicalChannel(element)
    assert flexray_physical_channel == flexray_physical_channel_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in FlexrayPhysicalChannel.__dict__
    assert len(str(flexray_physical_channel)) > 0

    # FlexrayPhysicalChannelsInfo
    channels_info = flexray_cluster.physical_channels
    assert channels_info.channel_a == flexray_physical_channel
    assert channels_info.channel_b == flexray_physical_channel_b
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in FlexrayPhysicalChannelsInfo.__dict__
    assert len(str(channels_info)) > 0

    # FlexrayCommunicationController
    ecu_instance = system.create_ecu_instance("EcuInstance", package)
    assert isinstance(ecu_instance, EcuInstance)
    flexray_controller = ecu_instance.create_flexray_communication_controller(
        "FlexrayController"
    )
    assert isinstance(flexray_controller, FlexrayCommunicationController)
    assert flexray_controller.name == "FlexrayController"
    flexray_controller.name = "FlexrayController2"
    assert flexray_controller.name == "FlexrayController2"
    assert flexray_controller.ecu_instance == ecu_instance
    # check if the controller can be constructed from an element and is equal to the original controller
    element = flexray_controller.element
    flexray_controller2 = FlexrayCommunicationController(element)
    assert flexray_controller == flexray_controller2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in FlexrayCommunicationController.__dict__
    assert len(str(flexray_controller)) > 0

    # FlexrayCommunicationConnector
    connector = flexray_controller.connect_physical_channel(
        "flexray_connector", flexray_physical_channel
    )
    assert list(flexray_controller.connected_channels()) == [flexray_physical_channel]
    assert isinstance(connector, FlexrayCommunicationConnector)
    assert connector.name == "flexray_connector"
    connector.name = "flexray_connector2"
    assert connector.name == "flexray_connector2"
    assert connector.controller == flexray_controller
    assert connector.ecu_instance == ecu_instance
    # check if the connector can be constructed from an element and is equal to the original connector
    element = connector.element
    connector2 = FlexrayCommunicationConnector(element)
    assert connector == connector2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in FlexrayCommunicationConnector.__dict__
    assert len(str(connector)) > 0

    # FlexrayFrame
    flexray_frame = system.create_flexray_frame("FlexRayFrame", package, 40)
    assert isinstance(flexray_frame, FlexrayFrame)
    # get and set the name
    assert flexray_frame.name == "FlexRayFrame"
    flexray_frame.name = "FlexRayFrame2"
    assert flexray_frame.name == "FlexRayFrame2"
    assert flexray_frame.length == 40
    flexray_frame.length = 64
    assert flexray_frame.length == 64

    # create a Pdu for the Frame
    pdu = system.create_isignal_ipdu("Pdu", package, 64)
    assert isinstance(pdu, ISignalIPdu)
    pdu_to_frame_mapping = flexray_frame.map_pdu(
        pdu, 0, ByteOrder.MostSignificantByteLast
    )
    assert list(flexray_frame.mapped_pdus()) == [pdu_to_frame_mapping]

    # check if the frame can be constructed from an element and is equal to the original frame
    element = flexray_frame.element
    flexray_frame2 = FlexrayFrame(element)
    assert flexray_frame == flexray_frame2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in FlexrayFrame.__dict__
    assert len(str(flexray_frame)) > 0

    # FlexrayFrameTriggering
    timing = FlexrayCommunicationCycle.Repetition(1, CycleRepetition.C1)
    flexray_frame_triggering = flexray_physical_channel.trigger_frame(
        flexray_frame, 1, timing
    )
    assert isinstance(flexray_frame_triggering, FlexrayFrameTriggering)
    assert list(flexray_physical_channel.frame_triggerings()) == [
        flexray_frame_triggering
    ]
    assert list(flexray_frame.frame_triggerings()) == [flexray_frame_triggering]
    # frame triggerings are created with a default name: 'FT_' + frame name
    assert flexray_frame_triggering.name == "FT_FlexRayFrame2"
    flexray_frame_triggering.name = "FT_FlexRayFrame3"
    assert flexray_frame_triggering.name == "FT_FlexRayFrame3"
    assert flexray_frame_triggering.frame == flexray_frame
    assert flexray_frame_triggering.physical_channel == flexray_physical_channel
    assert flexray_frame_triggering.timing() == timing
    timing2 = FlexrayCommunicationCycle.Repetition(2, CycleRepetition.C4)
    flexray_frame_triggering.set_timing(timing2)
    assert flexray_frame_triggering.timing() == timing2

    assert flexray_frame_triggering.slot == 1
    flexray_frame_triggering.slot = 2
    assert flexray_frame_triggering.slot == 2

    # triggering the frame in the channel also creates a pdu triggering for the mapped pdu
    assert len(list(flexray_physical_channel.pdu_triggerings())) == 1

    # check if the frame triggering can be constructed from an element and is equal to the original frame triggering
    element = flexray_frame_triggering.element
    flexray_frame_triggering2 = FlexrayFrameTriggering(element)
    assert flexray_frame_triggering == flexray_frame_triggering2
    # quick check if a custom __repr__ method is implemented and returns a non-empty
    assert "__repr__" in FlexrayFrameTriggering.__dict__
    assert len(str(flexray_frame_triggering)) > 0

    # FramePort
    frame_port = flexray_frame_triggering.connect_to_ecu(
        ecu_instance, CommunicationDirection.In
    )
    assert isinstance(frame_port, FramePort)
    assert frame_port.ecu == ecu_instance

    # mapping a signal to a triggered pdu should automatically create a signal triggering
    system_signal = package.create_system_signal("SystemSignal")
    isignal = system.create_isignal("ISignal", package, 8, system_signal)
    pdu.map_signal(isignal, 0, ByteOrder.MostSignificantByteLast)
    assert len(list(flexray_physical_channel.signal_triggerings())) == 1


def test_flexray_communication_cycle() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)

    # FlexRayCluster
    settings = FlexrayClusterSettings()
    cluster = system.create_flexray_cluster("FlexRayCluster", package, settings)
    channel = cluster.create_physical_channel(
        "FlexRayPhysicalChannel", FlexrayChannelName.A
    )
    frame = system.create_flexray_frame("FlexRayFrame", package, 40)
    timing_counter = FlexrayCommunicationCycle.Repetition(1, CycleRepetition.C1)
    assert isinstance(timing_counter, FlexrayCommunicationCycle.Repetition)
    assert timing_counter.base_cycle == 1
    assert timing_counter.cycle_repetition == CycleRepetition.C1

    # check the __repr__ method
    assert "__repr__" in FlexrayCommunicationCycle.__dict__
    assert len(str(timing_counter)) > 0

    ft = channel.trigger_frame(frame, 1, timing_counter)
    assert ft.timing() == timing_counter

    timing_counter = FlexrayCommunicationCycle.Repetition(1, CycleRepetition.C2)
    ft = channel.trigger_frame(frame, 2, timing_counter)
    assert ft.timing() == timing_counter

    timing_counter = FlexrayCommunicationCycle.Repetition(1, CycleRepetition.C4)
    ft = channel.trigger_frame(frame, 3, timing_counter)
    assert ft.timing() == timing_counter

    timing_counter = FlexrayCommunicationCycle.Repetition(1, CycleRepetition.C5)
    ft = channel.trigger_frame(frame, 4, timing_counter)
    assert ft.timing() == timing_counter

    timing_counter = FlexrayCommunicationCycle.Repetition(1, CycleRepetition.C8)
    ft = channel.trigger_frame(frame, 5, timing_counter)
    assert ft.timing() == timing_counter

    timing_counter = FlexrayCommunicationCycle.Repetition(1, CycleRepetition.C10)
    ft = channel.trigger_frame(frame, 6, timing_counter)
    assert ft.timing() == timing_counter

    timing_counter = FlexrayCommunicationCycle.Repetition(1, CycleRepetition.C16)
    ft = channel.trigger_frame(frame, 7, timing_counter)
    assert ft.timing() == timing_counter

    timing_counter = FlexrayCommunicationCycle.Repetition(1, CycleRepetition.C20)
    ft = channel.trigger_frame(frame, 8, timing_counter)
    assert ft.timing() == timing_counter

    timing_counter = FlexrayCommunicationCycle.Repetition(1, CycleRepetition.C32)
    ft = channel.trigger_frame(frame, 9, timing_counter)
    assert ft.timing() == timing_counter

    timing_counter = FlexrayCommunicationCycle.Repetition(1, CycleRepetition.C40)
    ft = channel.trigger_frame(frame, 10, timing_counter)
    assert ft.timing() == timing_counter

    timing_counter = FlexrayCommunicationCycle.Repetition(1, CycleRepetition.C50)
    ft = channel.trigger_frame(frame, 10, timing_counter)
    assert ft.timing() == timing_counter

    timing_counter = FlexrayCommunicationCycle.Repetition(1, CycleRepetition.C64)
    ft = channel.trigger_frame(frame, 11, timing_counter)
    assert ft.timing() == timing_counter

    timing_repetition = FlexrayCommunicationCycle.Counter(1)
    assert isinstance(timing_repetition, FlexrayCommunicationCycle.Counter)
    assert timing_repetition.cycle_counter == 1

    ft = channel.trigger_frame(frame, 12, timing_repetition)
    assert ft.timing() == timing_repetition
