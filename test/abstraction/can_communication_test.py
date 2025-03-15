from autosar_data.abstraction import *
from autosar_data.abstraction.communication import *


def test_can_communication() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)

    # CanCluster
    can_cluster = system.create_can_cluster("CanCluster", package)
    assert isinstance(can_cluster, CanCluster)
    # get and set the name
    assert can_cluster.name == "CanCluster"
    can_cluster.name = "CanCluster2"
    assert can_cluster.name == "CanCluster2"
    # get and set the settings
    can_cluster.baudrate = 500000
    assert can_cluster.baudrate == 500000
    can_cluster.can_fd_baudrate = 2000000
    assert can_cluster.can_fd_baudrate == 2000000
    can_cluster.can_xl_baudrate = 1000000
    assert can_cluster.can_xl_baudrate == 1000000

    assert can_cluster.system == system
    # check if the cluster can be constructed from an element and is equal to the original cluster
    element = can_cluster.element
    can_cluster2 = CanCluster(element)
    assert can_cluster == can_cluster2
    assert "__repr__" in CanCluster.__dict__
    assert len(str(can_cluster)) > 0

    # CanPhysicalChannel
    can_physical_channel = can_cluster.create_physical_channel("CanPhysicalChannel")
    assert isinstance(can_physical_channel, CanPhysicalChannel)
    # get and set the name
    assert can_physical_channel.name == "CanPhysicalChannel"
    can_physical_channel.name = "CanPhysicalChannel2"
    assert can_physical_channel.name == "CanPhysicalChannel2"
    # check attributes
    assert can_physical_channel.cluster == can_cluster
    assert can_cluster.physical_channel == can_physical_channel
    # check if the physical channel can be constructed from an element and is equal to the original physical channel
    element = can_physical_channel.element
    can_physical_channel2 = CanPhysicalChannel(element)
    assert can_physical_channel == can_physical_channel2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in CanPhysicalChannel.__dict__
    assert len(str(can_physical_channel)) > 0

    # CanCommunicationController
    ecu_instance = system.create_ecu_instance("EcuInstance", package)
    assert isinstance(ecu_instance, EcuInstance)
    can_controller = ecu_instance.create_can_communication_controller("CanController")
    assert isinstance(can_controller, CanCommunicationController)
    assert can_controller.name == "CanController"
    can_controller.name = "CanController2"
    assert can_controller.name == "CanController2"
    assert can_controller.ecu_instance == ecu_instance
    # check if the controller can be constructed from an element and is equal to the original controller
    element = can_controller.element
    can_controller2 = CanCommunicationController(element)
    assert can_controller == can_controller2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in CanCommunicationController.__dict__
    assert len(str(can_controller)) > 0

    # CanCommunicationConnector
    connector = can_controller.connect_physical_channel(
        "CanConnector", can_physical_channel
    )
    assert list(can_controller.connected_channels()) == [can_physical_channel]
    assert isinstance(connector, CanCommunicationConnector)
    assert connector.name == "CanConnector"
    connector.name = "CanConnector2"
    assert connector.name == "CanConnector2"
    assert connector.controller == can_controller
    assert connector.ecu_instance == ecu_instance
    # check if the connector can be constructed from an element and is equal to the original connector
    element = connector.element
    connector2 = CanCommunicationConnector(element)
    assert connector == connector2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in CanCommunicationConnector.__dict__
    assert len(str(connector)) > 0

    # CanFrame
    can_frame = system.create_can_frame("CanFrame", package, 8)
    assert isinstance(can_frame, CanFrame)
    assert can_frame.name == "CanFrame"
    can_frame.name = "CanFrame2"
    assert can_frame.name == "CanFrame2"
    assert can_frame.length == 8
    can_frame.length = 64
    assert can_frame.length == 64
    # check if the frame can be constructed from an element and is equal to the original frame
    element = can_frame.element
    can_frame2 = CanFrame(element)
    assert can_frame == can_frame2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in CanFrame.__dict__
    assert len(str(can_frame)) > 0

    # create a Pdu for the Frame
    pdu = system.create_isignal_ipdu("Pdu", package, 64)
    assert isinstance(pdu, ISignalIPdu)
    pdu_to_frame_mapping = can_frame.map_pdu(pdu, 0, ByteOrder.MostSignificantByteLast)
    assert list(can_frame.mapped_pdus()) == [pdu_to_frame_mapping]

    # CanFrameTriggering
    can_frame_triggering = can_physical_channel.trigger_frame(
        can_frame, 0x101, CanAddressingMode.Extended, CanFrameType.CanFd
    )
    assert isinstance(can_frame_triggering, CanFrameTriggering)
    assert list(can_physical_channel.frame_triggerings()) == [can_frame_triggering]
    assert list(can_frame.frame_triggerings()) == [can_frame_triggering]
    # frame triggerings are created with a default name: 'FT_' + frame name
    assert can_frame_triggering.name == "FT_CanFrame2"
    can_frame_triggering.name = "FT_CanFrame3"
    assert can_frame_triggering.name == "FT_CanFrame3"
    assert can_frame_triggering.frame == can_frame
    assert can_frame_triggering.physical_channel == can_physical_channel
    assert can_frame_triggering.frame_type == CanFrameType.CanFd
    can_frame_triggering.frame_type = CanFrameType.Can20
    assert can_frame_triggering.frame_type == CanFrameType.Can20

    assert can_frame_triggering.addressing_mode == CanAddressingMode.Extended
    can_frame_triggering.addressing_mode = CanAddressingMode.Standard
    assert can_frame_triggering.addressing_mode == CanAddressingMode.Standard

    assert can_frame_triggering.identifier == 0x101
    can_frame_triggering.identifier = 0x102
    assert can_frame_triggering.identifier == 0x102

    # triggering the frame in the channel also creates a pdu triggering for the mapped pdu
    assert len(list(can_physical_channel.pdu_triggerings())) == 1

    # check if the frame triggering can be constructed from an element and is equal to the original frame triggering
    element = can_frame_triggering.element
    can_frame_triggering2 = CanFrameTriggering(element)
    assert can_frame_triggering == can_frame_triggering2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in CanFrameTriggering.__dict__
    assert len(str(can_frame_triggering)) > 0

    # FramePort
    frame_port = can_frame_triggering.connect_to_ecu(
        ecu_instance, CommunicationDirection.In
    )
    assert isinstance(frame_port, FramePort)
    assert frame_port.ecu == ecu_instance

    # mapping a signal to a triggered pdu should automatically create a signal triggering
    system_signal = package.create_system_signal("SystemSignal")
    isignal = system.create_isignal("ISignal", package, 8, system_signal)
    pdu.map_signal(isignal, 0, ByteOrder.MostSignificantByteLast)
    assert len(list(can_physical_channel.signal_triggerings())) == 1
