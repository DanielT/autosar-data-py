from autosar_data import AutosarVersion
from autosar_data.abstraction import *
from autosar_data.abstraction.communication import *
from autosar_data.abstraction.datatype import *


def test_isignal() -> None:
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
    can_frame.map_pdu(isignal_ipdu, 0, ByteOrder.MostSignificantByteLast)

    # ISignal
    system_signal = package.create_system_signal("SystemSignal")
    isignal = system.create_isignal("ISignal", package, 8, system_signal)
    assert isinstance(isignal, ISignal)
    # get and set the name
    assert isignal.name == "ISignal"
    isignal.name = "ISignal2"
    assert isignal.name == "ISignal2"
    # attributes
    isignal.length = 32
    assert isignal.length == 32
    assert isignal.init_value is None
    isignal.init_value = 1
    assert isignal.init_value == NumericalValueSpecification(1) # 1 is automatically converted to a NumericalValueSpecification

    assert isignal.system_signal == system_signal
    system_signal2 = package.create_system_signal("SystemSignal2")
    isignal.system_signal = system_signal2
    assert isignal.system_signal == system_signal2
    isignal.system_signal = system_signal

    sw_base_type = package.create_sw_base_type(
        "SwBaseType", 32, BaseTypeEncoding.TwosComplement
    )
    isignal.datatype = sw_base_type
    assert isignal.datatype == sw_base_type

    system_signal_group = package.create_system_signal_group("SystemSignalGroup")
    isignal_group = system.create_isignal_group(
        "ISignalGroup", package, system_signal_group
    )
    system_signal_group.add_signal(system_signal)
    isignal_group.add_signal(isignal)

    isignal_ipdu.map_signal_group(isignal_group)
    isignal_ipdu.map_signal(isignal, 0, ByteOrder.MostSignificantByteLast)
    assert len(isignal.mappings()) == 1

    assert isignal.signal_group == isignal_group


def test_isignal_transformations() -> None:
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
    can_frame.map_pdu(isignal_ipdu, 0, ByteOrder.MostSignificantByteLast)

    # ISignal
    system_signal = package.create_system_signal("SystemSignal")
    isignal = system.create_isignal("ISignal", package, 8, system_signal)
    sw_base_type = package.create_sw_base_type(
        "SwBaseType", 32, BaseTypeEncoding.TwosComplement
    )
    isignal.datatype = sw_base_type

    # data transformation
    dts = package.create_data_transformation_set("DTS")
    e2e_transformer_config = E2ETransformationTechnologyConfig(
        profile=E2EProfile.P01,
        zero_header_length=False,
        transform_in_place=True,
        offset=0,
        max_delta_counter=0,
        max_error_state_init=0,
        max_error_state_invalid=0,
        max_error_state_valid=0,
        max_no_new_or_repeated_data=0,
        min_ok_state_init=0,
        min_ok_state_invalid=0,
        min_ok_state_valid=0,
        window_size=0,
        window_size_init=0,
        window_size_invalid=0,
        window_size_valid=0,
        profile_behavior=E2EProfileBehavior.R4_2,
        data_id_mode=DataIdMode.All16Bit,
        crc_offset=0,
        counter_offset=0,
    )
    e2e_transformer = dts.create_transformation_technology(
        "E2ETransformer",
        e2e_transformer_config,
    )
    assert e2e_transformer.config() == e2e_transformer_config
    someip_transformer_config = SomeIpTransformationTechnologyConfig(
        alignment=0,
        byte_order=ByteOrder.MostSignificantByteLast,
        interface_version=33,
    )
    someip_transformer = dts.create_transformation_technology(
        "SomeIpTransformer",
        someip_transformer_config,
    )
    assert someip_transformer.config() == someip_transformer_config
    chain = dts.create_data_transformation(
        "TransformationChain", [someip_transformer, e2e_transformer], True
    )

    isignal.add_data_transformation(chain)
    assert isignal.data_transformations().__next__() == chain

    e2e_props = isignal.create_e2e_transformation_isignal_props(e2e_transformer)
    someip_props = isignal.create_someip_transformation_isignal_props(
        someip_transformer
    )
    assert list(isignal.transformation_isignal_props()) == [e2e_props, someip_props]

    # check if the transformation properties can be constructed from an element and are equal to the original properties
    element = isignal.element
    isignal2 = ISignal(element)
    assert isignal == isignal2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ISignal.__dict__
    assert len(str(isignal)) > 0


def test_system_signal() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system_signal = package.create_system_signal("SystemSignal")
    assert isinstance(system_signal, SystemSignal)
    # get and set the name
    assert system_signal.name == "SystemSignal"
    system_signal.name = "SystemSignal2"
    assert system_signal.name == "SystemSignal2"

    system_signal_group = package.create_system_signal_group("SystemSignalGroup")
    system_signal_group.add_signal(system_signal)
    assert system_signal.signal_group == system_signal_group

    unit = package.create_unit("Unit")
    system_signal.unit = unit
    assert system_signal.unit == unit

    compu_method_content = CompuMethodContent.Identical()
    compu_method = package.create_compu_method("CompuMethod", compu_method_content)
    system_signal.compu_method = compu_method
    assert system_signal.compu_method == compu_method

    data_constr = package.create_data_constr("DataConstr")
    system_signal.data_constr = data_constr
    assert system_signal.data_constr == data_constr

    # check if the signal can be constructed from an element and is equal to the original signal
    element = system_signal.element
    system_signal2 = SystemSignal(element)
    assert system_signal == system_signal2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in SystemSignal.__dict__
    assert len(str(system_signal)) > 0


def test_isignal_group() -> None:
    model = AutosarModelAbstraction.create(
        "test.arxml", version=AutosarVersion.AUTOSAR_00048
    )
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    system_signal_group = package.create_system_signal_group("SystemSignalGroup")

    # ISignalGroup
    isignal_group = system.create_isignal_group(
        "ISignalGroup", package, system_signal_group
    )
    assert isinstance(isignal_group, ISignalGroup)
    # get and set the name
    assert isignal_group.name == "ISignalGroup"
    isignal_group.name = "ISignalGroup2"
    assert isignal_group.name == "ISignalGroup2"

    # attributes
    assert isignal_group.system_signal_group == system_signal_group

    system_signal = package.create_system_signal("SystemSignal")
    isignal = system.create_isignal("ISignal", package, 8, system_signal)
    system_signal_group.add_signal(system_signal)
    isignal_group.add_signal(isignal)
    assert list(isignal_group.signals()) == [isignal]

    # data transformation
    dts = package.create_data_transformation_set("DTS")
    e2e_transformer_config = E2ETransformationTechnologyConfig(
        profile=E2EProfile.P11,
        zero_header_length=False,
        transform_in_place=True,
        offset=0,
        max_delta_counter=0,
        max_error_state_init=0,
        max_error_state_invalid=0,
        max_error_state_valid=0,
        max_no_new_or_repeated_data=0,
        min_ok_state_init=0,
        min_ok_state_invalid=0,
        min_ok_state_valid=0,
        window_size=0,
        window_size_init=0,
        window_size_invalid=0,
        window_size_valid=0,
        profile_behavior=E2EProfileBehavior.R4_2,
        data_id_mode=DataIdMode.All16Bit,
        crc_offset=0,
        counter_offset=0,
    )
    e2e_transformer = dts.create_transformation_technology(
        "E2ETransformer",
        e2e_transformer_config,
    )
    assert e2e_transformer.config() == e2e_transformer_config
    someip_transformer_config = SomeIpTransformationTechnologyConfig(
        alignment=0,
        byte_order=ByteOrder.MostSignificantByteLast,
        interface_version=33,
    )
    someip_transformer = dts.create_transformation_technology(
        "SomeIpTransformer",
        someip_transformer_config,
    )
    assert someip_transformer.config() == someip_transformer_config
    chain = dts.create_data_transformation(
        "TransformationChain",
        [someip_transformer, e2e_transformer],
        True,
    )

    isignal_group.add_data_transformation(chain)
    assert isignal_group.data_transformations().__next__() == chain

    e2e_props = isignal_group.create_e2e_transformation_isignal_props(e2e_transformer)
    someip_props = isignal_group.create_someip_transformation_isignal_props(
        someip_transformer
    )
    assert list(isignal_group.transformation_isignal_props()) == [
        e2e_props,
        someip_props,
    ]

    # check if the signal group can be constructed from an element and is equal to the original signal group
    element = isignal_group.element
    isignal_group2 = ISignalGroup(element)
    assert isignal_group == isignal_group2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ISignalGroup.__dict__
    assert len(str(isignal_group)) > 0


def test_system_signal_group() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    # SystemSignalGroup
    system_signal_group = package.create_system_signal_group("SystemSignalGroup")
    assert isinstance(system_signal_group, SystemSignalGroup)
    # get and set the name
    assert system_signal_group.name == "SystemSignalGroup"
    system_signal_group.name = "SystemSignalGroup2"
    assert system_signal_group.name == "SystemSignalGroup2"

    system_signal = package.create_system_signal("SystemSignal")
    system_signal_group.add_signal(system_signal)
    assert list(system_signal_group.signals()) == [system_signal]

    # check if the signal group can be constructed from an element and is equal to the original signal group
    element = system_signal_group.element
    system_signal_group2 = SystemSignalGroup(element)
    assert system_signal_group == system_signal_group2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in SystemSignalGroup.__dict__
    assert len(str(system_signal_group)) > 0


def test_isignal_triggering() -> None:
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
    pdu_triggering = can_frame_triggering.pdu_triggerings().__next__()

    system_signal = package.create_system_signal("SystemSignal")
    isignal = system.create_isignal("ISignal", package, 8, system_signal)
    isignal_ipdu.map_signal(isignal, 0, ByteOrder.MostSignificantByteLast)

    # ISignalTriggering
    isignal_triggering = pdu_triggering.signal_triggerings().__next__()
    assert isinstance(isignal_triggering, ISignalTriggering)
    assert isignal.signal_triggerings()[0] == isignal_triggering
    # get and set the name
    isignal_triggering.name = "ISignalTriggering2"
    assert isignal_triggering.name == "ISignalTriggering2"
    # attributes
    assert isignal_triggering.physical_channel == can_physical_channel
    isignal_port = isignal_triggering.connect_to_ecu(
        ecu_instance, CommunicationDirection.In
    )
    assert list(isignal_triggering.signal_ports()) == [isignal_port]

    # check if the signal triggering can be constructed from an element and is equal to the original signal triggering
    element = isignal_triggering.element
    isignal_triggering2 = ISignalTriggering(element)
    assert isignal_triggering == isignal_triggering2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ISignalTriggering.__dict__
    assert len(str(isignal_triggering)) > 0

    # ISignalPort
    assert isinstance(isignal_port, ISignalPort)
    # get and set the name
    isignal_port.name = "ISignalPort2"
    assert isignal_port.name == "ISignalPort2"
    # attributes
    assert isignal_port.ecu == ecu_instance
    isignal_port.communication_direction = CommunicationDirection.Out
    assert isignal_port.communication_direction == CommunicationDirection.Out
    # check if the signal port can be constructed from an element and is equal to the original signal port
    element = isignal_port.element
    isignal_port2 = ISignalPort(element)
    assert isignal_port == isignal_port2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ISignalPort.__dict__
    assert len(str(isignal_port)) > 0


def test_data_transformation_set() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    # DataTransformationSet
    data_transformation_set = package.create_data_transformation_set("DTS")
    assert isinstance(data_transformation_set, DataTransformationSet)
    # get and set the name
    assert data_transformation_set.name == "DTS"
    data_transformation_set.name = "DTS2"
    assert data_transformation_set.name == "DTS2"

    com_transformer_config = ComTransformationTechnologyConfig(isignal_ipdu_length=64)
    com_transformer = data_transformation_set.create_transformation_technology(
        "ComTransformer",
        com_transformer_config,
    )
    assert list(data_transformation_set.transformation_technologies()) == [
        com_transformer
    ]

    data_transformation = data_transformation_set.create_data_transformation(
        "DataTransformation", [com_transformer], True
    )
    assert list(data_transformation_set.data_transformations()) == [data_transformation]

    # check if the data transformation set can be constructed from an element and is equal to the original data transformation set
    element = data_transformation_set.element
    dts2 = DataTransformationSet(element)
    assert data_transformation_set == dts2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in DataTransformationSet.__dict__
    assert len(str(data_transformation_set)) > 0


def test_data_transformation() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    # DataTransformationSet
    data_transformation_set = package.create_data_transformation_set("DTS")
    assert isinstance(data_transformation_set, DataTransformationSet)
    # get and set the name
    assert data_transformation_set.name == "DTS"
    data_transformation_set.name = "DTS2"
    assert data_transformation_set.name == "DTS2"

    com_transformer_config = ComTransformationTechnologyConfig(isignal_ipdu_length=64)
    com_transformer = data_transformation_set.create_transformation_technology(
        "ComTransformer",
        com_transformer_config,
    )

    # DataTransformation
    data_transformation = data_transformation_set.create_data_transformation(
        "DataTransformation", [com_transformer], True
    )
    assert isinstance(data_transformation, DataTransformation)
    # get and set the name
    assert data_transformation.name == "DataTransformation"
    data_transformation.name = "DataTransformation2"
    assert data_transformation.name == "DataTransformation2"
    # attributes
    assert data_transformation.data_transformation_set == data_transformation_set
    assert list(data_transformation.transformation_technologies()) == [com_transformer]

    # check if the data transformation can be constructed from an element and is equal to the original data transformation
    element = data_transformation.element
    data_transformation2 = DataTransformation(element)
    assert data_transformation == data_transformation2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in DataTransformation.__dict__
    assert len(str(data_transformation)) > 0


def test_transformation_technology() -> None:
    model = AutosarModelAbstraction.create(
        "test.arxml", version=AutosarVersion.AUTOSAR_00048
    )
    package = model.get_or_create_package("/package")
    data_transformation_set = package.create_data_transformation_set("DTS")

    generic_transformer_config = GenericTransformationTechnologyConfig(
        protocol_name="Foo", protocol_version="1.2.3", header_length=3, in_place=True
    )
    assert "__repr__" in GenericTransformationTechnologyConfig.__dict__
    assert len(str(generic_transformer_config)) > 0
    com_transformer_config = ComTransformationTechnologyConfig(isignal_ipdu_length=64)
    assert "__repr__" in ComTransformationTechnologyConfig.__dict__
    assert len(str(com_transformer_config)) > 0
    e2e_transformer_config = E2ETransformationTechnologyConfig(
        profile=E2EProfile.P02,
        zero_header_length=False,
        transform_in_place=True,
        offset=0,
        max_delta_counter=0,
        max_error_state_init=0,
        max_error_state_invalid=0,
        max_error_state_valid=0,
        max_no_new_or_repeated_data=0,
        min_ok_state_init=0,
        min_ok_state_invalid=0,
        min_ok_state_valid=0,
        window_size=0,
        window_size_init=0,
        window_size_invalid=0,
        window_size_valid=0,
        profile_behavior=E2EProfileBehavior.R4_2,
    )
    assert "__repr__" in E2ETransformationTechnologyConfig.__dict__
    assert len(str(e2e_transformer_config)) > 0
    someip_transformer_config = SomeIpTransformationTechnologyConfig(
        alignment=0,
        byte_order=ByteOrder.MostSignificantByteLast,
        interface_version=33,
    )
    assert "__repr__" in SomeIpTransformationTechnologyConfig.__dict__
    assert len(str(someip_transformer_config)) > 0

    # TransformationTechnology
    transformation_technology = (
        data_transformation_set.create_transformation_technology(
            "GenericTransformer",
            generic_transformer_config,
        )
    )
    assert isinstance(transformation_technology, TransformationTechnology)
    # get and set the name
    assert transformation_technology.name == "GenericTransformer"
    transformation_technology.name = "GenericTransformer2"
    assert transformation_technology.name == "GenericTransformer2"
    # attributes
    assert transformation_technology.data_transformation_set == data_transformation_set
    assert transformation_technology.config() == generic_transformer_config
    # protocol name is configured, transformer class is always CUSTOM
    assert transformation_technology.protocol == "Foo"
    assert transformation_technology.transformer_class == "CUSTOM"

    transformation_technology.set_config(com_transformer_config)
    assert transformation_technology.config() == com_transformer_config
    # protocol and transformer class values are standardized for the com transformation: they are always COMBased and SERIALIZER
    assert transformation_technology.protocol == "COMBased"
    assert transformation_technology.transformer_class == "SERIALIZER"

    transformation_technology.set_config(e2e_transformer_config)
    assert transformation_technology.config() == e2e_transformer_config
    # protocol and transformer class values are standardized for the e2e transformation: they are always E2E and SAFETY
    assert transformation_technology.protocol == "E2E"
    assert transformation_technology.transformer_class == "SAFETY"

    transformation_technology.set_config(someip_transformer_config)
    assert transformation_technology.config() == someip_transformer_config
    # protocol and transformer class values are standardized for the someip transformation: they are always SOMEIP and SOMEIP
    assert transformation_technology.protocol == "SOMEIP"
    assert transformation_technology.transformer_class == "SERIALIZER"

    # check if the transformation technology can be constructed from an element and is equal to the original transformation technology
    element = transformation_technology.element
    transformation_technology2 = TransformationTechnology(element)
    assert transformation_technology == transformation_technology2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in TransformationTechnology.__dict__
    assert len(str(transformation_technology)) > 0


def test_e2e_transformation_isignal_props() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    system_signal = package.create_system_signal("SystemSignal")
    isignal = system.create_isignal("ISignal", package, 8, system_signal)
    dts = package.create_data_transformation_set("DTS")

    e2e_transformer_config = E2ETransformationTechnologyConfig(
        profile=E2EProfile.P05,
        zero_header_length=False,
        transform_in_place=True,
        offset=0,
        max_delta_counter=0,
        max_error_state_init=0,
        max_error_state_invalid=0,
        max_error_state_valid=0,
        max_no_new_or_repeated_data=0,
        min_ok_state_init=0,
        min_ok_state_invalid=0,
        min_ok_state_valid=0,
        window_size=0,
        window_size_init=0,
        window_size_invalid=0,
        window_size_valid=0,
    )
    e2e_transformer = dts.create_transformation_technology(
        "E2ETransformer",
        e2e_transformer_config,
    )
    e2e_transformer2 = dts.create_transformation_technology(
        "E2ETransformer2",
        e2e_transformer_config,
    )

    # EndToEndTransformationISignalProps
    e2e_props = isignal.create_e2e_transformation_isignal_props(e2e_transformer)
    assert isinstance(e2e_props, EndToEndTransformationISignalProps)
    # attributes
    assert e2e_props.transformer == e2e_transformer
    e2e_props.transformer = e2e_transformer2
    assert e2e_props.transformer == e2e_transformer2

    e2e_props.data_ids = [0x100, 0x101]
    assert e2e_props.data_ids == [0x100, 0x101]

    e2e_props.data_length = 64
    assert e2e_props.data_length == 64

    e2e_props.max_data_length = 64
    assert e2e_props.max_data_length == 64

    e2e_props.min_data_length = 64
    assert e2e_props.min_data_length == 64

    e2e_props.source_id = 0xAA
    assert e2e_props.source_id == 0xAA

    # check if the transformation properties can be constructed from an element and are equal to the original properties
    element = e2e_props.element
    e2e_props2 = EndToEndTransformationISignalProps(element)
    assert e2e_props == e2e_props2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EndToEndTransformationISignalProps.__dict__
    assert len(str(e2e_props)) > 0


def test_someip_transformation_isignal_props() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    system_signal = package.create_system_signal("SystemSignal")
    isignal = system.create_isignal("ISignal", package, 8, system_signal)
    dts = package.create_data_transformation_set("DTS")

    someip_transformer_config = SomeIpTransformationTechnologyConfig(
        alignment=0,
        byte_order=ByteOrder.MostSignificantByteLast,
        interface_version=33,
    )
    someip_transformer = dts.create_transformation_technology(
        "SomeIpTransformer",
        someip_transformer_config,
    )
    someip_transformer2 = dts.create_transformation_technology(
        "SomeIpTransformer2",
        someip_transformer_config,
    )

    # SomeIpTransformationISignalProps
    someip_props = isignal.create_someip_transformation_isignal_props(
        someip_transformer
    )
    assert isinstance(someip_props, SomeIpTransformationISignalProps)
    # attributes
    assert someip_props.transformer == someip_transformer
    someip_props.transformer = someip_transformer2
    assert someip_props.transformer == someip_transformer2

    someip_props.legacy_strings = True
    assert someip_props.legacy_strings == True

    someip_props.interface_version = 33
    assert someip_props.interface_version == 33

    someip_props.dynamic_length = True
    assert someip_props.dynamic_length == True

    someip_props.message_type = SomeIpMessageType.Notification
    assert someip_props.message_type == SomeIpMessageType.Notification
    someip_props.message_type = SomeIpMessageType.Request
    assert someip_props.message_type == SomeIpMessageType.Request
    someip_props.message_type = SomeIpMessageType.Response
    assert someip_props.message_type == SomeIpMessageType.Response
    someip_props.message_type = SomeIpMessageType.RequestNoReturn
    assert someip_props.message_type == SomeIpMessageType.RequestNoReturn

    someip_props.size_of_array_length = 3
    assert someip_props.size_of_array_length == 3

    someip_props.size_of_string_length = 3
    assert someip_props.size_of_string_length == 3

    someip_props.size_of_struct_length = 3
    assert someip_props.size_of_struct_length == 3

    someip_props.size_of_union_length = 3
    assert someip_props.size_of_union_length == 3

    # check if the transformation properties can be constructed from an element and are equal to the original properties
    element = someip_props.element
    someip_props2 = SomeIpTransformationISignalProps(element)
    assert someip_props == someip_props2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in SomeIpTransformationISignalProps.__dict__
    assert len(str(someip_props)) > 0
