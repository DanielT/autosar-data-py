import os
from typing import *

from autosar_data import *
from autosar_data.abstraction import *
from autosar_data.abstraction.communication import *
from autosar_data.abstraction.datatype import *
from autosar_data.abstraction.software_component import *
from autosar_data.abstraction.ecu_configuration import *


def test_model_abstraction(tmp_path: str) -> None:
    model = AutosarModelAbstraction.create("file.arxml")
    assert isinstance(model, AutosarModelAbstraction)
    assert model.root_element is not None

    files = list(model.files())
    assert len(files) == 1

    model.create_file("file2.arxml")
    files = list(model.files())
    assert len(files) == 2

    package = model.get_or_create_package("/package")
    packages = list(model.packages())
    assert len(packages) == 1
    print(packages[0], package)
    assert packages[0] == package
    system = package.create_system("system", SystemCategory.EcuExtract)

    assert model.find_system() == system
    assert model.get_element_by_path("/package/system") == system.element

    base_model = (
        model.model
    )  # yes, the naming is awkward - the abstract model contains a base model
    assert isinstance(base_model, AutosarModel)
    assert model == AutosarModelAbstraction(base_model)

    # create another model, this time with a filename located in the tmp_path
    filename = os.path.join(tmp_path, "file.arxml")
    model2 = AutosarModelAbstraction.create(filename)
    model2.write()
    assert os.path.isfile(filename)

    # create a model from an existing file
    model3 = AutosarModelAbstraction.from_file(filename)
    assert model3 is not None
    # even though the content is the same, the models are different instances
    assert model2 != model3

    # create an empty model and load the content from the file
    model4 = AutosarModelAbstraction(AutosarModel())
    model4.load_file(filename)

    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in AutosarModelAbstraction.__dict__
    assert len(repr(model)) > 0


def test_package() -> None:
    model = AutosarModelAbstraction.create("file.arxml")
    package = model.get_or_create_package("/package")
    assert isinstance(package, ArPackage)
    assert package.element.path == "/package"
    assert package.name == "package"
    package.name = "new_package"
    assert package.name == "new_package"
    package.name = "package"

    element = package.element
    package_copy = ArPackage(element)
    assert package == package_copy

    sub_package = package.create_sub_package("sub_package")
    assert isinstance(sub_package, ArPackage)
    assert sub_package.element.path == "/package/sub_package"
    assert list(package.sub_packages()) == [sub_package]

    package2 = model.get_or_create_package("/package")
    assert package == package2

    other_package = model.get_or_create_package("/other_package")
    assert package != other_package

    app_primitive_data_type = package.create_application_primitive_data_type(
        "primitive_data_type",
        ApplicationPrimitiveCategory.Value,
    )

    app_array_size = ApplicationArraySize.Fixed(16)
    package.create_application_array_data_type(
        "array_data_type", app_primitive_data_type, app_array_size
    )

    application_record_data_type = package.create_application_record_data_type(
        "record_data_type"
    )
    assert isinstance(application_record_data_type, ApplicationRecordDataType)

    application_sw_component_type = package.create_application_sw_component_type(
        "sw_component_type"
    )
    assert isinstance(application_sw_component_type, ApplicationSwComponentType)

    client_server_interface = package.create_client_server_interface(
        "client_server_interface"
    )
    assert isinstance(client_server_interface, ClientServerInterface)

    complex_device_driver_sw_component_type = (
        package.create_complex_device_driver_sw_component_type("complex_device_driver")
    )
    assert isinstance(
        complex_device_driver_sw_component_type, ComplexDeviceDriverSwComponentType
    )

    composition_sw_component_type = package.create_composition_sw_component_type(
        "composition_sw_component"
    )
    assert isinstance(composition_sw_component_type, CompositionSwComponentType)

    compu_method_content = CompuMethodContent.Identical()
    compu_method = package.create_compu_method("compu_method", compu_method_content)
    assert isinstance(compu_method, CompuMethod)

    data_constr = package.create_data_constr("data_constr")
    assert isinstance(data_constr, DataConstr)

    data_transformation_set = package.create_data_transformation_set(
        "data_transformation_set"
    )
    assert isinstance(data_transformation_set, DataTransformationSet)

    data_type_mapping_set = package.create_data_type_mapping_set(
        "data_type_mapping_set"
    )
    assert isinstance(data_type_mapping_set, DataTypeMappingSet)

    ecu_abstraction_sw_component = package.create_ecu_abstraction_sw_component_type(
        "ecu_abstraction_sw_component"
    )
    assert isinstance(ecu_abstraction_sw_component, EcuAbstractionSwComponentType)

    ecuc_definition_collection = package.create_ecuc_definition_collection(
        "ecuc_definition_collection"
    )
    assert isinstance(ecuc_definition_collection, EcucDefinitionCollection)

    ecuc_destination_uri_def_set = package.create_ecuc_destination_uri_def_set(
        "ecuc_destination_uri_def_set"
    )
    assert isinstance(ecuc_destination_uri_def_set, EcucDestinationUriDefSet)

    ecuc_module_def = package.create_ecuc_module_def("ecuc_module_def")
    assert isinstance(ecuc_module_def, EcucModuleDef)

    ecuc_module_configuration_values = package.create_ecuc_module_configuration_values(
        "ecuc_module_configuration_values", ecuc_module_def
    )
    assert isinstance(ecuc_module_configuration_values, EcucModuleConfigurationValues)

    ecuc_value_collection = package.create_ecuc_value_collection(
        "ecuc_value_collection"
    )
    assert isinstance(ecuc_value_collection, EcucValueCollection)

    base_type = package.create_sw_base_type(
        "sw_base_type",
        32,
        BaseTypeEncoding.NoEncoding,
        byte_order=ByteOrder.MostSignificantByteFirst,
        mem_alignment=4,
        native_declaration="int32",
    )
    assert isinstance(base_type, SwBaseType)

    implementation_data_type_settings = ImplementationDataTypeSettings.Value(
        "implementation_value",
        base_type=base_type,
        compu_method=compu_method,
        data_constraint=data_constr,
    )
    implementation_data_type = package.create_implementation_data_type(
        implementation_data_type_settings
    )
    assert isinstance(implementation_data_type, ImplementationDataType)

    mode_switch_interface = package.create_mode_switch_interface(
        "mode_switch_interface"
    )
    assert isinstance(mode_switch_interface, ModeSwitchInterface)

    nv_data_interface = package.create_nv_data_interface("nv_data_interface")
    assert isinstance(nv_data_interface, NvDataInterface)

    parameter_interface = package.create_parameter_interface("parameter_interface")
    assert isinstance(parameter_interface, ParameterInterface)

    sender_receiver_interface = package.create_sender_receiver_interface(
        "sender_receiver_interface"
    )
    assert isinstance(sender_receiver_interface, SenderReceiverInterface)

    sensor_actuator_sw_component = package.create_sensor_actuator_sw_component_type(
        "sensor_actuator_sw_component"
    )
    assert isinstance(sensor_actuator_sw_component, SensorActuatorSwComponentType)

    service_sw_component = package.create_service_sw_component_type(
        "service_sw_component"
    )
    assert isinstance(service_sw_component, ServiceSwComponentType)

    request_response_delay = RequestResponseDelay(min_value=0.01, max_value=1.1)
    someip_sd_server_event_group_timing_config = (
        package.create_someip_sd_server_event_group_timing_config(
            "someip_sd_server_event_group_timing_config", request_response_delay
        )
    )
    assert isinstance(
        someip_sd_server_event_group_timing_config, SomeipSdServerEventGroupTimingConfig
    )

    someip_sd_client_event_group_timing_config = (
        package.create_someip_sd_client_event_group_timing_config(
            "someip_sd_client_event_group_timing_config", 100
        )
    )
    assert isinstance(
        someip_sd_client_event_group_timing_config, SomeipSdClientEventGroupTimingConfig
    )

    someip_sd_client_service_instance_config = (
        package.create_someip_sd_client_service_instance_config(
            "someip_sd_client_service_instance_config"
        )
    )
    assert isinstance(
        someip_sd_client_service_instance_config, SomeipSdClientServiceInstanceConfig
    )

    someip_sd_server_service_instance_config = (
        package.create_someip_sd_server_service_instance_config(
            "someip_sd_server_service_instance_config", 200
        )
    )
    assert isinstance(
        someip_sd_server_service_instance_config, SomeipSdServerServiceInstanceConfig
    )

    system = package.create_system("system", SystemCategory.EcuExtract)
    assert isinstance(system, System)

    system_signal = package.create_system_signal("system_signal")
    assert isinstance(system_signal, SystemSignal)

    system_signal_group = package.create_system_signal_group("system_signal_group")
    assert isinstance(system_signal_group, SystemSignalGroup)

    trigger_interface = package.create_trigger_interface("trigger_interface")
    assert isinstance(trigger_interface, TriggerInterface)

    unit = package.create_unit("unit", display_name="Unit Display Name")
    assert isinstance(unit, Unit)

    assert len(list(package.elements())) == 34


def test_system() -> None:
    model = AutosarModelAbstraction.create("file.arxml")
    package = model.get_or_create_package("/package")

    system = package.create_system("System", SystemCategory.EcuExtract)
    assert isinstance(system, System)

    system.name = "System2"
    assert system.name == "System2"

    can_cluster = system.create_can_cluster("can_cluster", package)
    assert isinstance(can_cluster, CanCluster)

    assert system.category == SystemCategory.EcuExtract
    system.category = SystemCategory.SystemDescription
    assert system.category == SystemCategory.SystemDescription
    system.category = SystemCategory.SystemDescription
    assert system.category == SystemCategory.SystemDescription
    system.category = SystemCategory.SystemConstraints
    assert system.category == SystemCategory.SystemConstraints
    system.category = SystemCategory.SystemExtract
    assert system.category == SystemCategory.SystemExtract
    system.category = SystemCategory.AbstractSystemDescription
    assert system.category == SystemCategory.AbstractSystemDescription
    system.category = SystemCategory.EcuSystemDescription
    assert system.category == SystemCategory.EcuSystemDescription
    system.category = SystemCategory.SwClusterSystemDescription
    assert system.category == SystemCategory.SwClusterSystemDescription
    system.category = SystemCategory.RptSystem
    assert system.category == SystemCategory.RptSystem
    system.pnc_vector_length = 10
    assert system.pnc_vector_length == 10
    system.pnc_vector_offset = 0
    assert system.pnc_vector_offset == 0

    can_frame = system.create_can_frame("can_frame", package, 8)
    assert isinstance(can_frame, CanFrame)

    can_tp_config = system.create_can_tp_config("can_tp_config", package, can_cluster)
    assert isinstance(can_tp_config, CanTpConfig)

    container_ipdu = system.create_container_ipdu(
        "container_ipdu",
        package,
        8,
        ContainerIPduHeaderType.ShortHeader,
        RxAcceptContainedIPdu.AcceptAll,
    )
    assert isinstance(container_ipdu, ContainerIPdu)

    dcm_ipdu = system.create_dcm_ipdu("dcm_ipdu", package, 8, DiagPduType.DiagRequest)
    assert isinstance(dcm_ipdu, DcmIPdu)

    ecu_instance = system.create_ecu_instance("ecu_instance", package)
    assert isinstance(ecu_instance, EcuInstance)

    ethernet_cluster = system.create_ethernet_cluster("ethernet_cluster", package)
    assert isinstance(ethernet_cluster, EthernetCluster)

    doip_tp_config = system.create_doip_tp_config(
        "doip_tp_config", package, ethernet_cluster
    )
    assert isinstance(doip_tp_config, DoIpTpConfig)

    flexray_cluster_settings = FlexrayClusterSettings()
    flexray_cluster = system.create_flexray_cluster(
        "flexray_cluster", package, flexray_cluster_settings
    )
    assert isinstance(flexray_cluster, FlexrayCluster)

    flexray_frame = system.create_flexray_frame("flexray_frame", package, 8)
    assert isinstance(flexray_frame, FlexrayFrame)

    flexray_tp_config = system.create_flexray_tp_config(
        "flexray_tp_config", package, flexray_cluster
    )
    assert isinstance(flexray_tp_config, FlexrayTpConfig)

    general_purpose_ipdu = system.create_general_purpose_ipdu(
        "general_purpose_ipdu", package, 8, GeneralPurposeIPduCategory.Xcp
    )
    assert isinstance(general_purpose_ipdu, GeneralPurposeIPdu)

    general_purpose_pdu = system.create_general_purpose_pdu(
        "general_purpose_pdu", package, 8, GeneralPurposePduCategory.DoIp
    )
    assert isinstance(general_purpose_pdu, GeneralPurposePdu)

    system_signal = package.create_system_signal("system_signal")
    isignal = system.create_isignal("isignal", package, 8, system_signal)
    assert isinstance(isignal, ISignal)

    system_signal_group = package.create_system_signal_group("system_signal_group")
    isignal_group = system.create_isignal_group(
        "isignal_group", package, system_signal_group
    )
    assert isinstance(isignal_group, ISignalGroup)

    isignal_ipdu = system.create_isignal_ipdu("isignal_ipdu", package, 8)
    assert isinstance(isignal_ipdu, ISignalIPdu)

    multiplexed_ipdu = system.create_multiplexed_ipdu("multiplexed_ipdu", package, 8)
    assert isinstance(multiplexed_ipdu, MultiplexedIPdu)

    n_pdu = system.create_n_pdu("n_pdu", package, 8)
    assert isinstance(n_pdu, NPdu)

    nm_config = system.create_nm_config("nm_config", package)
    assert isinstance(nm_config, NmConfig)

    nm_pdu = system.create_nm_pdu("nm_pdu", package, 8)
    assert isinstance(nm_pdu, NmPdu)

    secured_ipdu = system.create_secured_ipdu(
        "secured_ipdu", package, 8, SecureCommunicationProps()
    )
    assert isinstance(secured_ipdu, SecuredIPdu)

    service_instance_collection_set = system.create_service_instance_collection_set(
        "service_instance_collection_set", package
    )
    assert isinstance(service_instance_collection_set, ServiceInstanceCollectionSet)

    so_ad_routing_group = system.create_so_ad_routing_group(
        "so_ad_routing_group", package
    )
    assert isinstance(so_ad_routing_group, SoAdRoutingGroup)

    socket_connection_ipdu_identifier_set = (
        system.create_socket_connection_ipdu_identifier_set(
            "socket_connection_ipdu_identifier_set", package
        )
    )
    assert isinstance(
        socket_connection_ipdu_identifier_set, SocketConnectionIpduIdentifierSet
    )

    somip_tp_config = system.create_someip_tp_config(
        "somip_tp_config", package, ethernet_cluster
    )
    assert isinstance(somip_tp_config, SomeipTpConfig)

    system_mapping = system.get_or_create_mapping("system_mapping")
    assert isinstance(system_mapping, SystemMapping)

    composition_sw_component_type = package.create_composition_sw_component_type(
        "Composition"
    )
    assert system.root_sw_composition is None
    system.set_root_sw_composition("root_sw_composition", composition_sw_component_type)
    assert isinstance(system.root_sw_composition, RootSwCompositionPrototype)
    assert system.root_sw_composition.composition == composition_sw_component_type

    # manually create an ISignalIPdu, and link it to the System
    manual_ipdu = package.element.get_or_create_sub_element(
        "ELEMENTS"
    ).create_named_sub_element("I-SIGNAL-I-PDU", "ISignalIPdu_manual")
    assert len(model.model.get_references_to(manual_ipdu.path)) == 0
    system.create_fibex_element_ref(manual_ipdu)
    assert len(model.model.get_references_to(manual_ipdu.path)) == 1

    assert list(system.clusters()) == [can_cluster, ethernet_cluster, flexray_cluster]
    assert list(system.ecu_instances()) == [ecu_instance]
    assert list(system.frames()) == [can_frame, flexray_frame]
    assert list(system.isignal_groups()) == [isignal_group]
    assert list(system.isignals()) == [isignal]
    assert list(system.pdus()) == [
        container_ipdu,
        dcm_ipdu,
        general_purpose_ipdu,
        general_purpose_pdu,
        isignal_ipdu,
        multiplexed_ipdu,
        n_pdu,
        nm_pdu,
        secured_ipdu,
        ISignalIPdu(manual_ipdu),
    ]
    assert system.nm_config() == nm_config

    # check if the system can be created from an existing element
    element = system.element
    system_copy = System(element)
    assert system == system_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in System.__dict__
    assert len(repr(system)) > 0


def test_system_mapping() -> None:
    model = AutosarModelAbstraction.create("file.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    ecu_instance = system.create_ecu_instance("ecu_instance", package)
    sw_component_type = package.create_composition_sw_component_type(
        "CompositionSwComponentType"
    )
    sub_component_type = package.create_composition_sw_component_type(
        "CompositionSwComponentType2"
    )
    root_composition_prototype = system.set_root_sw_composition(
        "RootSwComposition", sw_component_type
    )
    sub_composition = sw_component_type.create_component(
        "SubComponent", sub_component_type
    )

    sw_base_type = package.create_sw_base_type(
        "sw_base_type", 32, BaseTypeEncoding.TwosComplement
    )
    impl_data_type = package.create_implementation_data_type(
        ImplementationDataTypeSettings.Value("impl_data_type", base_type=sw_base_type)
    )
    sender_receiver_interface = package.create_sender_receiver_interface(
        "sender_receiver_interface"
    )
    sr_data_element = sender_receiver_interface.create_data_element(
        "data_element", impl_data_type
    )
    p_port = sub_component_type.create_p_port("p_port", sender_receiver_interface)

    system_signal = package.create_system_signal("system_signal")

    # SystemMapping
    system_mapping = system.get_or_create_mapping("system_mapping")
    assert isinstance(system_mapping, SystemMapping)
    # get and set the name
    assert system_mapping.name == "system_mapping"
    system_mapping.name = "system_mapping2"
    assert system_mapping.name == "system_mapping2"

    assert system_mapping.system == system

    ecu_mapping = system_mapping.map_swc_to_ecu(
        "SwcToEcuMapping", sub_composition, ecu_instance
    )
    assert isinstance(ecu_mapping, SwcToEcuMapping)

    system_mapping.map_sender_receiver_to_signal(
        system_signal,
        sr_data_element,
        p_port,
        [sub_composition],
        root_composition_prototype=root_composition_prototype,
    )

    # check if the system mapping can be created from an existing element
    element = system_mapping.element
    system_mapping_copy = SystemMapping(element)
    assert system_mapping == system_mapping_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in SystemMapping.__dict__
    assert len(repr(system_mapping)) > 0


def test_swc_to_ecu_mapping() -> None:
    model = AutosarModelAbstraction.create("file.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)
    ecu_instance = system.create_ecu_instance("ecu_instance", package)
    sw_component_type = package.create_composition_sw_component_type(
        "CompositionSwComponentType"
    )
    sub_component_type = package.create_composition_sw_component_type(
        "CompositionSwComponentType2"
    )
    system.set_root_sw_composition("RootSwComposition", sw_component_type)
    sub_composition = sw_component_type.create_component(
        "SubComponent", sub_component_type
    )
    system_mapping = system.get_or_create_mapping("system_mapping")

    # SwcToEcuMapping
    swc_to_ecu_mapping = system_mapping.map_swc_to_ecu(
        "SwcToEcuMapping", sub_composition, ecu_instance
    )
    assert isinstance(swc_to_ecu_mapping, SwcToEcuMapping)
    assert swc_to_ecu_mapping.name == "SwcToEcuMapping"
    swc_to_ecu_mapping.name = "SwcToEcuMapping2"
    assert swc_to_ecu_mapping.name == "SwcToEcuMapping2"

    assert swc_to_ecu_mapping.target_component == sub_composition
    assert swc_to_ecu_mapping.ecu_instance == ecu_instance

    # check if the swc to ecu mapping can be created from an existing element
    element = swc_to_ecu_mapping.element
    swc_to_ecu_mapping_copy = SwcToEcuMapping(element)
    assert swc_to_ecu_mapping == swc_to_ecu_mapping_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in SwcToEcuMapping.__dict__
    assert len(repr(swc_to_ecu_mapping)) > 0


def test_ecu_instance() -> None:
    model = AutosarModelAbstraction.create("file.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)

    ecu_instance = system.create_ecu_instance("ecu_instance", package)
    assert isinstance(ecu_instance, EcuInstance)

    ecu_instance.name = "ecu_instance2"
    assert ecu_instance.name == "ecu_instance2"

    can_controller = ecu_instance.create_can_communication_controller("can_controller")
    ethernet_controller = ecu_instance.create_ethernet_communication_controller(
        "ethernet_controller"
    )
    flexray_controller = ecu_instance.create_flexray_communication_controller(
        "flexray_controller"
    )
    assert list(ecu_instance.communication_controllers()) == [
        can_controller,
        ethernet_controller,
        flexray_controller,
    ]

    # check if the ecu instance can be created from an existing element
    element = ecu_instance.element
    ecu_instance_copy = EcuInstance(element)
    assert ecu_instance == ecu_instance_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EcuInstance.__dict__
    assert len(repr(ecu_instance)) > 0
