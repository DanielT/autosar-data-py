from autosar_data.abstraction import *
from autosar_data.abstraction.communication import *


def test_isignal_ipdu() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    can_cluster = system.create_can_cluster("CanCluster", package)
    can_physical_channel = can_cluster.create_physical_channel("CanPhysicalChannel")
    can_frame = system.create_can_frame("CanFrame", package, 64)
    can_frame_triggering = can_physical_channel.trigger_frame(
        can_frame, 0x101, CanAddressingMode.Extended, CanFrameType.CanFd
    )

    # ISignalIPdu
    isignal_ipdu = system.create_isignal_ipdu("ISignalIPdu", package, 64)
    assert isinstance(isignal_ipdu, ISignalIPdu)
    assert list(system.pdus()) == [isignal_ipdu]
    # get and set the name
    assert isignal_ipdu.name == "ISignalIPdu"
    isignal_ipdu.name = "ISignalIPdu2"
    assert isignal_ipdu.name == "ISignalIPdu2"
    # attributes
    assert isignal_ipdu.length == 64
    isignal_ipdu.length = 32
    assert isignal_ipdu.length == 32
    timing = IpduTiming(
        transmission_mode_true_timing=TransmissionModeTiming(
            cyclic_timing=CyclicTiming(0.01)
        )
    )
    isignal_ipdu.set_timing(timing)
    assert isignal_ipdu.timing() == timing
    can_frame.map_pdu(isignal_ipdu, 0, ByteOrder.MostSignificantByteLast)
    assert len(list(can_frame_triggering.pdu_triggerings())) == 1
    pt = can_frame_triggering.pdu_triggerings().__next__()
    assert pt.pdu == isignal_ipdu
    assert len(list(isignal_ipdu.pdu_triggerings())) == 1
    assert isignal_ipdu.pdu_triggerings()[0] == pt

    system_signal = package.create_system_signal("SystemSignal")
    isignal = system.create_isignal("ISignal", package, 8, system_signal)
    isignal_mapping = isignal_ipdu.map_signal(
        isignal, 0, ByteOrder.MostSignificantByteLast
    )
    assert len(list(isignal_ipdu.mapped_signals())) == 1
    assert isignal_ipdu.mapped_signals().__next__() == isignal_mapping

    system_signal_group = package.create_system_signal_group("SystemSignalGroup")
    isignal_group = system.create_isignal_group(
        "ISignalGroup", package, system_signal_group
    )
    isignal_group_mapping = isignal_ipdu.map_signal_group(isignal_group)
    assert len(list(isignal_ipdu.mapped_signals())) == 2
    assert list(isignal_ipdu.mapped_signals())[1] == isignal_group_mapping

    contained_props = ContainedIPduProps(header_id_long=333)
    isignal_ipdu.contained_ipdu_props = contained_props
    assert isignal_ipdu.contained_ipdu_props == contained_props

    # check if the signal ipdu can be constructed from an element and is equal to the original signal ipdu
    element = isignal_ipdu.element
    isignal_ipdu2 = ISignalIPdu(element)
    assert isignal_ipdu == isignal_ipdu2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ISignalIPdu.__dict__
    assert len(str(isignal_ipdu)) > 0


def test_nm_pdu() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    can_cluster = system.create_can_cluster("CanCluster", package)
    can_physical_channel = can_cluster.create_physical_channel("CanPhysicalChannel")
    can_frame = system.create_can_frame("CanFrame", package, 64)
    can_frame_triggering = can_physical_channel.trigger_frame(
        can_frame, 0x101, CanAddressingMode.Extended, CanFrameType.CanFd
    )

    # NmPdu
    nm_pdu = system.create_nm_pdu("NmPdu", package, 64)
    assert isinstance(nm_pdu, NmPdu)
    assert list(system.pdus()) == [nm_pdu]
    # get and set the name
    assert nm_pdu.name == "NmPdu"
    nm_pdu.name = "NmPdu2"
    assert nm_pdu.name == "NmPdu2"
    # attributes
    assert nm_pdu.length == 64
    nm_pdu.length = 32
    assert nm_pdu.length == 32
    can_frame.map_pdu(nm_pdu, 0, ByteOrder.MostSignificantByteLast)
    assert len(list(can_frame_triggering.pdu_triggerings())) == 1
    pt = can_frame_triggering.pdu_triggerings().__next__()
    assert pt.pdu == nm_pdu
    assert len(list(nm_pdu.pdu_triggerings())) == 1
    assert nm_pdu.pdu_triggerings()[0] == pt

    # check if the nm pdu can be constructed from an element and is equal to the original nm pdu
    element = nm_pdu.element
    nm_pdu2 = NmPdu(element)
    assert nm_pdu == nm_pdu2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in NmPdu.__dict__
    assert len(str(nm_pdu)) > 0


def test_npdu() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    can_cluster = system.create_can_cluster("CanCluster", package)
    can_physical_channel = can_cluster.create_physical_channel("CanPhysicalChannel")
    can_frame = system.create_can_frame("CanFrame", package, 64)
    can_frame_triggering = can_physical_channel.trigger_frame(
        can_frame, 0x101, CanAddressingMode.Extended, CanFrameType.CanFd
    )

    # NPdu
    npdu = system.create_n_pdu("Npdu", package, 64)
    assert isinstance(npdu, NPdu)
    assert list(system.pdus()) == [npdu]
    # get and set the name
    assert npdu.name == "Npdu"
    npdu.name = "Npdu2"
    assert npdu.name == "Npdu2"
    # attributes
    assert npdu.length == 64
    npdu.length = 32
    assert npdu.length == 32
    can_frame.map_pdu(npdu, 0, ByteOrder.MostSignificantByteLast)
    assert len(list(can_frame_triggering.pdu_triggerings())) == 1
    pt = can_frame_triggering.pdu_triggerings().__next__()
    assert pt.pdu == npdu
    assert len(list(npdu.pdu_triggerings())) == 1
    assert npdu.pdu_triggerings()[0] == pt

    contained_props = ContainedIPduProps(header_id_long=333)
    npdu.contained_ipdu_props = contained_props
    assert npdu.contained_ipdu_props == contained_props

    # check if the npdu can be constructed from an element and is equal to the original npdu
    element = npdu.element
    npdu2 = NPdu(element)
    assert npdu == npdu2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in NPdu.__dict__
    assert len(str(npdu)) > 0


def test_dcm_i_pdu() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    can_cluster = system.create_can_cluster("CanCluster", package)
    can_physical_channel = can_cluster.create_physical_channel("CanPhysicalChannel")
    can_frame = system.create_can_frame("CanFrame", package, 64)
    can_frame_triggering = can_physical_channel.trigger_frame(
        can_frame, 0x101, CanAddressingMode.Extended, CanFrameType.CanFd
    )

    # DcmIPdu
    dcm_ipdu = system.create_dcm_ipdu("DcmIPdu", package, 64, DiagPduType.DiagRequest)
    assert isinstance(dcm_ipdu, DcmIPdu)
    assert list(system.pdus()) == [dcm_ipdu]
    # get and set the name
    assert dcm_ipdu.name == "DcmIPdu"
    dcm_ipdu.name = "DcmIPdu2"
    assert dcm_ipdu.name == "DcmIPdu2"
    # attributes
    assert dcm_ipdu.length == 64
    dcm_ipdu.length = 32
    assert dcm_ipdu.length == 32
    can_frame.map_pdu(dcm_ipdu, 0, ByteOrder.MostSignificantByteLast)
    assert len(list(can_frame_triggering.pdu_triggerings())) == 1
    pt = can_frame_triggering.pdu_triggerings().__next__()
    assert pt.pdu == dcm_ipdu
    assert len(list(dcm_ipdu.pdu_triggerings())) == 1
    assert dcm_ipdu.pdu_triggerings()[0] == pt
    assert dcm_ipdu.diag_pdu_type == DiagPduType.DiagRequest

    contained_props = ContainedIPduProps(header_id_long=333)
    dcm_ipdu.contained_ipdu_props = contained_props
    assert dcm_ipdu.contained_ipdu_props == contained_props

    # check if the dcm ipdu can be constructed from an element and is equal to the original dcm ipdu
    element = dcm_ipdu.element
    dcm_ipdu2 = DcmIPdu(element)
    assert dcm_ipdu == dcm_ipdu2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in DcmIPdu.__dict__
    assert len(str(dcm_ipdu)) > 0


def test_general_purpose_pdu() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    can_cluster = system.create_can_cluster("CanCluster", package)
    can_physical_channel = can_cluster.create_physical_channel("CanPhysicalChannel")
    can_frame = system.create_can_frame("CanFrame", package, 64)
    can_frame_triggering = can_physical_channel.trigger_frame(
        can_frame, 0x101, CanAddressingMode.Extended, CanFrameType.CanFd
    )

    # GeneralPurposePdu
    pdu = system.create_general_purpose_pdu(
        "Pdu", package, 64, GeneralPurposePduCategory.Sd
    )
    assert isinstance(pdu, GeneralPurposePdu)
    assert list(system.pdus()) == [pdu]
    # get and set the name
    assert pdu.name == "Pdu"
    pdu.name = "Pdu2"
    assert pdu.name == "Pdu2"
    # attributes
    assert pdu.length == 64
    pdu.length = 32
    assert pdu.length == 32
    assert pdu.category == GeneralPurposePduCategory.Sd
    pdu.category = GeneralPurposePduCategory.GlobalTime
    assert pdu.category == GeneralPurposePduCategory.GlobalTime
    pdu.category = GeneralPurposePduCategory.DoIp
    assert pdu.category == GeneralPurposePduCategory.DoIp
    can_frame.map_pdu(pdu, 0, ByteOrder.MostSignificantByteLast)
    assert len(list(can_frame_triggering.pdu_triggerings())) == 1
    pt = can_frame_triggering.pdu_triggerings().__next__()
    assert pt.pdu == pdu
    assert len(list(pdu.pdu_triggerings())) == 1
    assert pdu.pdu_triggerings()[0] == pt

    # check if the pdu can be constructed from an element and is equal to the original pdu
    element = pdu.element
    pdu2 = GeneralPurposePdu(element)
    assert pdu == pdu2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in GeneralPurposePdu.__dict__
    assert len(str(pdu)) > 0


def test_general_purpose_ipdu() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    can_cluster = system.create_can_cluster("CanCluster", package)
    can_physical_channel = can_cluster.create_physical_channel("CanPhysicalChannel")
    can_frame = system.create_can_frame("CanFrame", package, 64)
    can_frame_triggering = can_physical_channel.trigger_frame(
        can_frame, 0x101, CanAddressingMode.Extended, CanFrameType.CanFd
    )

    # GeneralPurposeIPdu
    pdu = system.create_general_purpose_ipdu(
        "Pdu", package, 64, GeneralPurposeIPduCategory.Xcp
    )
    assert isinstance(pdu, GeneralPurposeIPdu)
    assert list(system.pdus()) == [pdu]
    # get and set the name
    assert pdu.name == "Pdu"
    pdu.name = "Pdu2"
    assert pdu.name == "Pdu2"
    # attributes
    assert pdu.length == 64
    pdu.length = 32
    assert pdu.length == 32
    assert pdu.category == GeneralPurposeIPduCategory.Xcp
    pdu.category = GeneralPurposeIPduCategory.Dlt
    assert pdu.category == GeneralPurposeIPduCategory.Dlt
    pdu.category = GeneralPurposeIPduCategory.SomeipSegmentedIpdu
    assert pdu.category == GeneralPurposeIPduCategory.SomeipSegmentedIpdu
    can_frame.map_pdu(pdu, 0, ByteOrder.MostSignificantByteLast)
    assert len(list(can_frame_triggering.pdu_triggerings())) == 1
    pt = can_frame_triggering.pdu_triggerings().__next__()
    assert pt.pdu == pdu
    assert len(list(pdu.pdu_triggerings())) == 1
    assert pdu.pdu_triggerings()[0] == pt

    contained_props = ContainedIPduProps(header_id_long=333)
    pdu.contained_ipdu_props = contained_props
    assert pdu.contained_ipdu_props == contained_props

    # check if the pdu can be constructed from an element and is equal to the original pdu
    element = pdu.element
    pdu2 = GeneralPurposeIPdu(element)
    assert pdu == pdu2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in GeneralPurposeIPdu.__dict__
    assert len(str(pdu)) > 0


def test_container_ipdu() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    can_cluster = system.create_can_cluster("CanCluster", package)
    can_physical_channel = can_cluster.create_physical_channel("CanPhysicalChannel")
    can_frame = system.create_can_frame("CanFrame", package, 64)
    can_frame_triggering = can_physical_channel.trigger_frame(
        can_frame, 0x101, CanAddressingMode.Extended, CanFrameType.CanFd
    )

    # ContainerIPdu
    pdu = system.create_container_ipdu(
        "Pdu",
        package,
        64,
        ContainerIPduHeaderType.LongHeader,
        RxAcceptContainedIPdu.AcceptConfigured,
    )
    assert isinstance(pdu, ContainerIPdu)
    assert list(system.pdus()) == [pdu]
    # get and set the name
    assert pdu.name == "Pdu"
    pdu.name = "Pdu2"
    assert pdu.name == "Pdu2"
    # attributes
    assert pdu.length == 64
    pdu.length = 32
    assert pdu.length == 32
    can_frame.map_pdu(pdu, 0, ByteOrder.MostSignificantByteLast)
    assert len(list(can_frame_triggering.pdu_triggerings())) == 1
    pt = can_frame_triggering.pdu_triggerings().__next__()
    assert pt.pdu == pdu
    assert len(list(pdu.pdu_triggerings())) == 1
    assert pdu.pdu_triggerings()[0] == pt

    pdu.header_type = ContainerIPduHeaderType.ShortHeader
    assert pdu.header_type == ContainerIPduHeaderType.ShortHeader
    pdu.header_type = ContainerIPduHeaderType.LongHeader
    assert pdu.header_type == ContainerIPduHeaderType.LongHeader
    pdu.header_type = ContainerIPduHeaderType.NoHeader
    assert pdu.header_type == ContainerIPduHeaderType.NoHeader
    pdu.rx_accept_contained_ipdu = RxAcceptContainedIPdu.AcceptAll
    assert pdu.rx_accept_contained_ipdu == RxAcceptContainedIPdu.AcceptAll
    pdu.rx_accept_contained_ipdu = RxAcceptContainedIPdu.AcceptConfigured
    assert pdu.rx_accept_contained_ipdu == RxAcceptContainedIPdu.AcceptConfigured
    pdu.container_timeout = 11.33
    assert pdu.container_timeout == 11.33
    pdu.container_trigger = ContainerIPduTrigger.DefaultTrigger
    assert pdu.container_trigger == ContainerIPduTrigger.DefaultTrigger

    contained_ipdu = system.create_isignal_ipdu("ContainedIPdu", package, 64)
    pt = pdu.map_ipdu(contained_ipdu, can_physical_channel)
    assert list(pdu.contained_ipdu_triggerings()) == [pt]

    contained_props = ContainedIPduProps(header_id_long=333)
    pdu.contained_ipdu_props = contained_props
    assert pdu.contained_ipdu_props == contained_props

    contained_props.collection_semantics = ContainedIPduCollectionSemantics.LastIsBest
    assert (
        contained_props.collection_semantics
        == ContainedIPduCollectionSemantics.LastIsBest
    )
    pdu.contained_ipdu_props = contained_props
    assert pdu.contained_ipdu_props == contained_props
    contained_props.collection_semantics = ContainedIPduCollectionSemantics.Queued
    assert (
        contained_props.collection_semantics == ContainedIPduCollectionSemantics.Queued
    )
    pdu.contained_ipdu_props = contained_props
    assert pdu.contained_ipdu_props == contained_props

    # check if the pdu can be constructed from an element and is equal to the original pdu
    element = pdu.element
    pdu2 = ContainerIPdu(element)
    assert pdu == pdu2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ContainerIPdu.__dict__
    assert len(str(pdu)) > 0


def test_secured_ipdu() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    can_cluster = system.create_can_cluster("CanCluster", package)
    can_physical_channel = can_cluster.create_physical_channel("CanPhysicalChannel")
    can_frame = system.create_can_frame("CanFrame", package, 64)
    can_frame_triggering = can_physical_channel.trigger_frame(
        can_frame, 0x101, CanAddressingMode.Extended, CanFrameType.CanFd
    )

    # SecuredIPdu
    pdu = system.create_secured_ipdu("Pdu", package, 64, SecureCommunicationProps())
    assert isinstance(pdu, SecuredIPdu)
    assert list(system.pdus()) == [pdu]
    # get and set the name
    assert pdu.name == "Pdu"
    pdu.name = "Pdu2"
    assert pdu.name == "Pdu2"
    # attributes
    assert pdu.length == 64
    pdu.length = 32
    assert pdu.length == 32
    can_frame.map_pdu(pdu, 0, ByteOrder.MostSignificantByteLast)
    assert len(list(can_frame_triggering.pdu_triggerings())) == 1
    pt = can_frame_triggering.pdu_triggerings().__next__()
    assert pt.pdu == pdu
    assert len(list(pdu.pdu_triggerings())) == 1
    assert pdu.pdu_triggerings()[0] == pt

    secure_pdu = system.create_isignal_ipdu("SecurePdu", package, 32)
    pt = pdu.set_payload_ipdu(secure_pdu, can_physical_channel)
    assert pdu.payload_pdu_triggering == pt
    assert pt.pdu == secure_pdu
    pdu.payload_pdu_triggering = pt
    assert pdu.payload_pdu_triggering == pt

    pdu.use_as_cryptographic_ipdu = True
    assert pdu.use_as_cryptographic_ipdu is True

    sec_props = SecureCommunicationProps(
        auth_data_freshness_length=1,
        auth_data_freshness_start_position=2,
        authentication_build_attempts=3,
        authentication_retries=4,
        data_id=5,
        freshness_value_id=6,
        message_link_length=7,
        message_link_position=8,
        secondary_freshness_value_id=9,
        secured_area_length=10,
        secured_area_offset=11,
    )
    pdu.secure_communication_props = sec_props
    assert pdu.secure_communication_props == sec_props

    contained_props = ContainedIPduProps(header_id_long=333)
    pdu.contained_ipdu_props = contained_props
    assert pdu.contained_ipdu_props == contained_props

    # check if the pdu can be constructed from an element and is equal to the original pdu
    element = pdu.element
    pdu2 = SecuredIPdu(element)
    assert pdu == pdu2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in SecuredIPdu.__dict__
    assert len(str(pdu)) > 0


def test_multiplexed_ipdu() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    can_cluster = system.create_can_cluster("CanCluster", package)
    can_physical_channel = can_cluster.create_physical_channel("CanPhysicalChannel")
    can_frame = system.create_can_frame("CanFrame", package, 64)
    can_frame_triggering = can_physical_channel.trigger_frame(
        can_frame, 0x101, CanAddressingMode.Extended, CanFrameType.CanFd
    )

    # MultiplexedIPdu
    pdu = system.create_multiplexed_ipdu("Pdu", package, 64)
    assert isinstance(pdu, MultiplexedIPdu)
    assert list(system.pdus()) == [pdu]
    # get and set the name
    assert pdu.name == "Pdu"
    pdu.name = "Pdu2"
    assert pdu.name == "Pdu2"
    # attributes
    assert pdu.length == 64
    pdu.length = 32
    assert pdu.length == 32
    can_frame.map_pdu(pdu, 0, ByteOrder.MostSignificantByteLast)
    assert len(list(can_frame_triggering.pdu_triggerings())) == 1
    pt = can_frame_triggering.pdu_triggerings().__next__()
    assert pt.pdu == pdu
    assert len(list(pdu.pdu_triggerings())) == 1
    assert pdu.pdu_triggerings()[0] == pt

    contained_props = ContainedIPduProps(header_id_long=333)
    pdu.contained_ipdu_props = contained_props
    assert pdu.contained_ipdu_props == contained_props

    # check if the pdu can be constructed from an element and is equal to the original pdu
    element = pdu.element
    pdu2 = MultiplexedIPdu(element)
    assert pdu == pdu2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in MultiplexedIPdu.__dict__
    assert len(str(pdu)) > 0


def test_isignal_to_ipdu_mapping() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    can_cluster = system.create_can_cluster("CanCluster", package)
    can_physical_channel = can_cluster.create_physical_channel("CanPhysicalChannel")
    can_frame = system.create_can_frame("CanFrame", package, 64)
    can_frame_triggering = can_physical_channel.trigger_frame(
        can_frame, 0x101, CanAddressingMode.Extended, CanFrameType.CanFd
    )
    isignal_ipdu = system.create_isignal_ipdu("ISignalIPdu", package, 64)
    system_signal = package.create_system_signal("SystemSignal")
    isignal = system.create_isignal("ISignal", package, 8, system_signal)

    # ISignalToIPduMapping
    isignal_to_idpu_mapping = isignal_ipdu.map_signal(
        isignal, 0, ByteOrder.MostSignificantByteLast
    )
    assert isinstance(isignal_to_idpu_mapping, ISignalToIPduMapping)
    assert list(isignal_ipdu.mapped_signals()) == [isignal_to_idpu_mapping]
    # get and set the name
    isignal_to_idpu_mapping.name = "ISignal2"
    assert isignal_to_idpu_mapping.name == "ISignal2"
    # attributes
    assert isignal_to_idpu_mapping.signal == isignal
    assert isignal_to_idpu_mapping.signal_group is None
    assert isignal_to_idpu_mapping.start_position == 0
    assert isignal_to_idpu_mapping.update_bit is None
    assert isignal_to_idpu_mapping.byte_order == ByteOrder.MostSignificantByteLast
    isignal_to_idpu_mapping.byte_order = ByteOrder.MostSignificantByteFirst
    assert isignal_to_idpu_mapping.byte_order == ByteOrder.MostSignificantByteFirst
    isignal_to_idpu_mapping.transfer_property = TransferProperty.Pending
    assert isignal_to_idpu_mapping.transfer_property == TransferProperty.Pending

    # check if the signal to ipdu mapping can be constructed from an element and is equal to the original signal to ipdu mapping
    element = isignal_to_idpu_mapping.element
    isignal_to_idpu_mapping2 = ISignalToIPduMapping(element)
    assert isignal_to_idpu_mapping == isignal_to_idpu_mapping2

    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ISignalToIPduMapping.__dict__
    assert len(str(isignal_to_idpu_mapping)) > 0


def test_timing() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    can_cluster = system.create_can_cluster("CanCluster", package)
    can_physical_channel = can_cluster.create_physical_channel("CanPhysicalChannel")
    can_frame = system.create_can_frame("CanFrame", package, 64)
    can_frame_triggering = can_physical_channel.trigger_frame(
        can_frame, 0x101, CanAddressingMode.Extended, CanFrameType.CanFd
    )
    isignal_ipdu = system.create_isignal_ipdu("ISignalIPdu", package, 64)

    # IpduTiming
    cyclic_timing = CyclicTiming(0.01)
    assert isinstance(cyclic_timing, CyclicTiming)
    assert "__repr__" in CyclicTiming.__dict__
    assert len(str(cyclic_timing)) > 0

    event_controlled_timing = EventControlledTiming(0, repetition_period=0.01)
    assert isinstance(event_controlled_timing, EventControlledTiming)
    assert "__repr__" in EventControlledTiming.__dict__
    assert len(str(event_controlled_timing)) > 0

    transmission_mode_timing = TransmissionModeTiming(
        cyclic_timing=cyclic_timing, event_controlled_timing=event_controlled_timing
    )
    assert isinstance(transmission_mode_timing, TransmissionModeTiming)
    assert transmission_mode_timing.cyclic_timing == cyclic_timing
    assert transmission_mode_timing.event_controlled_timing == event_controlled_timing
    assert "__repr__" in TransmissionModeTiming.__dict__
    assert len(str(transmission_mode_timing)) > 0

    timing = IpduTiming(transmission_mode_true_timing=transmission_mode_timing)
    assert isinstance(timing, IpduTiming)
    assert timing.transmission_mode_true_timing == transmission_mode_timing
    assert "__repr__" in IpduTiming.__dict__
    assert len(str(timing)) > 0

    isignal_ipdu.set_timing(timing)
    assert isignal_ipdu.timing() == timing


def test_pdu_to_frame_mapping() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    can_cluster = system.create_can_cluster("CanCluster", package)
    can_physical_channel = can_cluster.create_physical_channel("CanPhysicalChannel")
    can_frame = system.create_can_frame("CanFrame", package, 64)
    can_frame_triggering = can_physical_channel.trigger_frame(
        can_frame, 0x101, CanAddressingMode.Extended, CanFrameType.CanFd
    )
    isignal_ipdu = system.create_isignal_ipdu("ISignalIPdu", package, 64)

    # PduToFrameMapping
    pdu_to_frame_mapping = can_frame.map_pdu(
        isignal_ipdu, 0, ByteOrder.MostSignificantByteLast
    )
    assert isinstance(pdu_to_frame_mapping, PduToFrameMapping)
    assert list(can_frame.mapped_pdus()) == [pdu_to_frame_mapping]
    # get and set the name
    pdu_to_frame_mapping.name = "Pdu2"
    assert pdu_to_frame_mapping.name == "Pdu2"
    # attributes
    assert pdu_to_frame_mapping.pdu == isignal_ipdu
    assert pdu_to_frame_mapping.start_position == 0
    pdu_to_frame_mapping.start_position = 8
    assert pdu_to_frame_mapping.start_position == 8
    pdu_to_frame_mapping.update_bit = 0
    assert pdu_to_frame_mapping.update_bit == 0
    assert pdu_to_frame_mapping.byte_order == ByteOrder.MostSignificantByteLast
    pdu_to_frame_mapping.byte_order = ByteOrder.MostSignificantByteFirst
    assert pdu_to_frame_mapping.byte_order == ByteOrder.MostSignificantByteFirst

    # check if the pdu to frame mapping can be constructed from an element and is equal to the original pdu to frame mapping
    element = pdu_to_frame_mapping.element
    pdu_to_frame_mapping2 = PduToFrameMapping(element)
    assert pdu_to_frame_mapping == pdu_to_frame_mapping2

    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in PduToFrameMapping.__dict__
    assert len(str(pdu_to_frame_mapping)) > 0


def test_pdu_triggering() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    can_cluster = system.create_can_cluster("CanCluster", package)
    can_physical_channel = can_cluster.create_physical_channel("CanPhysicalChannel")
    ecu_instance = system.create_ecu_instance("EcuInstance", package)
    can_controller = ecu_instance.create_can_communication_controller("CanController")
    can_controller.connect_physical_channel("Connection", can_physical_channel)
    can_frame = system.create_can_frame("CanFrame", package, 64)
    can_frame_triggering = can_physical_channel.trigger_frame(
        can_frame, 0x101, CanAddressingMode.Extended, CanFrameType.CanFd
    )
    isignal_ipdu = system.create_isignal_ipdu("ISignalIPdu", package, 64)
    can_frame.map_pdu(isignal_ipdu, 0, ByteOrder.MostSignificantByteLast)

    # PduTriggering
    pdu_triggering = can_frame_triggering.pdu_triggerings().__next__()
    assert isinstance(pdu_triggering, PduTriggering)
    assert list(can_frame_triggering.pdu_triggerings()) == [pdu_triggering]
    # get and set the name
    assert pdu_triggering.name == "PT_ISignalIPdu"
    pdu_triggering.name = "PT_ISignalIPdu2"
    assert pdu_triggering.name == "PT_ISignalIPdu2"
    # attributes
    assert pdu_triggering.physical_channel == can_physical_channel
    ipdu_port = pdu_triggering.create_pdu_port(ecu_instance, CommunicationDirection.In)
    assert list(pdu_triggering.pdu_ports()) == [ipdu_port]

    system_signal = package.create_system_signal("SystemSignal")
    isignal = system.create_isignal("ISignal", package, 8, system_signal)
    isignal_ipdu.map_signal(isignal, 0, ByteOrder.MostSignificantByteLast)
    # a signal triggering is created when the signal is mapped to the triggered ipdu
    assert len(list(pdu_triggering.signal_triggerings())) == 1

    # check if the pdu triggering can be constructed from an element and is equal to the original pdu triggering
    element = pdu_triggering.element
    pdu_triggering2 = PduTriggering(element)
    assert pdu_triggering == pdu_triggering2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in PduTriggering.__dict__
    assert len(str(pdu_triggering)) > 0


def test_frame_port() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    ecu_instance = system.create_ecu_instance("EcuInstance", package)
    can_cluster = system.create_can_cluster("CanCluster", package)
    can_physical_channel = can_cluster.create_physical_channel("CanPhysicalChannel")
    can_controller = ecu_instance.create_can_communication_controller("CanController")
    can_controller.connect_physical_channel("Connection", can_physical_channel)
    can_frame = system.create_can_frame("CanFrame", package, 64)
    can_frame_triggering = can_physical_channel.trigger_frame(
        can_frame, 0x101, CanAddressingMode.Extended, CanFrameType.CanFd
    )

    # FramePort
    frame_port = can_frame_triggering.connect_to_ecu(
        ecu_instance, CommunicationDirection.In
    )
    assert isinstance(frame_port, FramePort)
    assert list(can_frame_triggering.frame_ports()) == [frame_port]
    # get and set the name
    frame_port.name = "FramePort"
    assert frame_port.name == "FramePort"
    # attributes
    assert frame_port.ecu == ecu_instance
    assert frame_port.communication_direction == CommunicationDirection.In
    frame_port.communication_direction = CommunicationDirection.Out
    assert frame_port.communication_direction == CommunicationDirection.Out
    # check if the frame port can be constructed from an element and is equal to the original frame port
    element = frame_port.element
    frame_port2 = FramePort(element)
    assert frame_port == frame_port2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in FramePort.__dict__
    assert len(str(frame_port)) > 0


def test_ipdu_port() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    ecu_instance = system.create_ecu_instance("EcuInstance", package)
    can_cluster = system.create_can_cluster("CanCluster", package)
    can_physical_channel = can_cluster.create_physical_channel("CanPhysicalChannel")
    can_controller = ecu_instance.create_can_communication_controller("CanController")
    can_controller.connect_physical_channel("Connection", can_physical_channel)
    can_frame = system.create_can_frame("CanFrame", package, 64)
    can_frame_triggering = can_physical_channel.trigger_frame(
        can_frame, 0x101, CanAddressingMode.Extended, CanFrameType.CanFd
    )
    isignal_ipdu = system.create_isignal_ipdu("ISignalIPdu", package, 64)
    can_frame.map_pdu(isignal_ipdu, 0, ByteOrder.MostSignificantByteLast)

    # IPduPort
    pdu_triggering = can_frame_triggering.pdu_triggerings().__next__()
    ipdu_port = pdu_triggering.create_pdu_port(ecu_instance, CommunicationDirection.Out)
    assert isinstance(ipdu_port, IPduPort)
    assert list(pdu_triggering.pdu_ports()) == [ipdu_port]
    # get and set the name
    ipdu_port.name = "IPduPort"
    assert ipdu_port.name == "IPduPort"
    # attributes
    assert ipdu_port.ecu == ecu_instance
    assert ipdu_port.communication_direction == CommunicationDirection.Out
    ipdu_port.communication_direction = CommunicationDirection.In
    assert ipdu_port.communication_direction == CommunicationDirection.In
    # check if the ipdu port can be constructed from an element and is equal to the original ipdu port
    element = ipdu_port.element
    ipdu_port2 = IPduPort(element)
    assert ipdu_port == ipdu_port2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in IPduPort.__dict__
    assert len(str(ipdu_port)) > 0
