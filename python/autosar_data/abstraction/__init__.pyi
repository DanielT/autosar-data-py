from typing import final, Iterator, List, Optional, Tuple
from autosar_data.abstraction.communication import *
from autosar_data.abstraction.datatype import *
from autosar_data.abstraction.ecu_configuration import *
from autosar_data.abstraction.software_component import *

from autosar_data import ArxmlFile, AutosarModel, AutosarVersion, Element

@final
class ArPackage:
    """
    An `ArPackage` is an Autosar package, which can contain other packages or elements
    """

    def __init__(self, element: Element) -> ArPackage: ...
    def create_application_array_data_type(
        self,
        name: str,
        element_type: ApplicationDataType,
        size: ApplicationArraySize,
        /,
    ) -> ApplicationArrayDataType:
        """create a new `ApplicationArrayDataType` in the package"""
        ...

    def create_application_primitive_data_type(
        self,
        name: str,
        category: ApplicationPrimitiveCategory,
        /,
        *,
        compu_method: CompuMethod = None,
        unit: Unit = None,
        data_constraint: DataConstr = None,
    ) -> ApplicationPrimitiveDataType:
        """create a new `ApplicationPrimitiveDataType` in the package"""
        ...

    def create_application_record_data_type(
        self, name: str
    ) -> ApplicationRecordDataType:
        """create a new `ApplicationRecordDataType` in the package"""
        ...

    def create_application_sw_component_type(
        self, name: str
    ) -> ApplicationSwComponentType:
        """create a new `ApplicationSwComponentType` in the package"""
        ...

    def create_client_server_interface(self, name: str) -> ClientServerInterface:
        """create a new `ClientServerInterface` in the package"""
        ...

    def create_complex_device_driver_sw_component_type(
        self, name: str
    ) -> ComplexDeviceDriverSwComponentType:
        """create a new `ComplexDeviceDriverSwComponentType` in the package"""
        ...

    def create_composition_sw_component_type(
        self, name: str
    ) -> CompositionSwComponentType:
        """create a new `CompositionSwComponentType` in the package"""
        ...

    def create_compu_method(
        self, name: str, content: CompuMethodContent
    ) -> CompuMethod:
        """create a new `CompuMethod` in the package"""
        ...

    def create_constant_specification(
        self, name: str, value: ValueSpecification
    ) -> ConstantSpecification:
        """create a new `ConstantSpecification` in the package"""
        ...

    def create_data_constr(self, name: str) -> DataConstr:
        """create a new `DataConstr` in the package"""
        ...

    def create_data_transformation_set(self, name: str) -> DataTransformationSet:
        """create a new `DataTransformationSet` in the package"""
        ...

    def create_data_type_mapping_set(self, name: str) -> DataTypeMappingSet:
        """create a new `DataTypeMappingSet` in the package"""
        ...

    def create_ecu_abstraction_sw_component_type(
        self, name: str
    ) -> EcuAbstractionSwComponentType:
        """create a new `EcuAbstractionSwComponentType` in the package"""
        ...

    def create_ecuc_definition_collection(self, name: str) -> EcucDefinitionCollection:
        """create a new `EcucDefinitionCollection` in the package"""
        ...

    def create_ecuc_destination_uri_def_set(
        self, name: str
    ) -> EcucDestinationUriDefSet:
        """create a new `EcucDestinationUriDefSet` in the package"""
        ...

    def create_ecuc_module_configuration_values(
        self, name: str, definition: EcucModuleDef
    ) -> EcucModuleConfigurationValues:
        """create a new `EcucModuleConfigurationValues` in the package"""
        ...

    def create_ecuc_module_def(self, name: str) -> EcucModuleDef:
        """create a new `EcucModuleDef` in the package"""
        ...

    def create_ecuc_value_collection(self, name: str) -> EcucValueCollection:
        """create a new `EcucValueCollection` in the package"""
        ...

    def create_implementation_data_type(
        self, settings: ImplementationDataTypeSettings
    ) -> ImplementationDataType:
        """create a new `ImplementationDataType` in the package"""
        ...

    def create_mode_declaration_group(
        self, name: str, *, category: Optional[ModeDeclarationGroupCategory] = None
    ) -> ModeDeclarationGroup:
        """create a new `ModeDeclarationGroup` in the package"""
        ...

    def create_mode_switch_interface(self, name: str) -> ModeSwitchInterface:
        """create a new `ModeSwitchInterface` in the package"""
        ...

    def create_nv_data_interface(self, name: str) -> NvDataInterface:
        """create a new `NvDataInterface` in the package"""
        ...

    def create_parameter_interface(self, name: str) -> ParameterInterface:
        """create a new `ParameterInterface` in the package"""
        ...

    def create_sender_receiver_interface(self, name: str) -> SenderReceiverInterface:
        """create a new `SenderReceiverInterface` in the package"""
        ...

    def create_sensor_actuator_sw_component_type(
        self, name: str
    ) -> SensorActuatorSwComponentType:
        """create a new `SensorActuatorSwComponentType` in the package"""
        ...

    def create_service_sw_component_type(self, name: str) -> ServiceSwComponentType:
        """create a new `ServiceSwComponentType` in the package"""
        ...

    def create_someip_sd_client_event_group_timing_config(
        self, name: str, time_to_live: int
    ) -> SomeipSdClientEventGroupTimingConfig:
        """create a new `SomeipSdClientEventGroupTimingConfig` in the package"""
        ...

    def create_someip_sd_client_service_instance_config(
        self, name: str
    ) -> SomeipSdClientServiceInstanceConfig:
        """create a new `SomeipSdClientServiceInstanceConfig` in the package"""
        ...

    def create_someip_sd_server_event_group_timing_config(
        self, name: str, request_response_delay: RequestResponseDelay
    ) -> SomeipSdServerEventGroupTimingConfig:
        """create a new `SomeipSdServerEventGroupTimingConfig` in the package"""
        ...

    def create_someip_sd_server_service_instance_config(
        self, name: str, ttl: int
    ) -> SomeipSdServerServiceInstanceConfig:
        """create a new `SomeipSdServerServiceInstanceConfig` in the package"""
        ...

    def create_sw_base_type(
        self,
        name: str,
        bit_length: int,
        base_type_encoding: BaseTypeEncoding,
        /,
        *,
        byte_order: Optional[ByteOrder] = None,
        mem_alignment: Optional[int] = None,
        native_declaration: Optional[str] = None,
    ) -> SwBaseType:
        """create a new `SwBaseType` in the package"""
        ...

    def create_system(self, name: str, category: SystemCategory, /) -> System:
        """create a new System in the package

        Note that an Autosar model should ony contain one SYSTEM. This is not checked here.
        """
        ...

    def create_system_signal(self, name: str) -> SystemSignal:
        """create a new `SystemSignal` in the package"""
        ...

    def create_system_signal_group(self, name: str) -> SystemSignalGroup:
        """create a new `SystemSignalGroup` in the package"""
        ...

    def create_trigger_interface(self, name: str) -> TriggerInterface:
        """create a new `TriggerInterface` in the package"""
        ...

    def create_unit(self, name: str, /, *, display_name: Optional[str] = None) -> Unit:
        """create a new `Unit` in the package"""
        ...

    def create_sub_package(self, name: str, /) -> ArPackage:
        """create a new sub-package in the package"""
        ...
    element: Element
    def elements(self) -> Iterator[Element]:
        """iterate over all elements in the package"""
        ...

    def sub_packages(self) -> Iterator[ArPackage]:
        """iterate over all sub-packages in the package"""
        ...
    name: str

@final
class AutosarModelAbstraction:
    def __init__(self, model: AutosarModel) -> AutosarModelAbstraction: ...
    @classmethod
    def create(
        cls, filename: str, /, *, version: Optional[AutosarVersion] = None
    ) -> AutosarModelAbstraction:
        """create a new `AutosarModelAbstraction` with an empty `AutosarModel`"""
        ...

    def create_file(
        self, filename: str, /, *, version: Optional[AutosarVersion] = None
    ) -> ArxmlFile:
        """Create a new file in the model"""
        ...

    def files(self, /) -> Iterator[ArxmlFile]:
        """iterate over all files in the model"""
        ...

    def find_system(self, /) -> Optional[System]:
        """find an existing SYSTEM in the model, if it exists"""
        ...

    @classmethod
    def from_file(cls, filename: str, /) -> AutosarModelAbstraction:
        """create an `AutosarModelAbstraction` from a file on disk"""
        ...

    def get_element_by_path(self, path: str, /) -> Optional[Element]:
        """Get an element by its path"""
        ...

    def get_or_create_package(self, path: str, /) -> ArPackage:
        """Get a package by its path or create it if it does not exist"""
        ...

    def load_file(
        self, filename: str, /, *, strict: bool = False
    ) -> Tuple[ArxmlFile, List[str]]:
        """Load a file into the model"""
        ...
    model: AutosarModel
    """Get the underlying `AutosarModel` from the abstraction model"""
    def packages(self, /) -> Iterator[ArPackage]:
        """iterate over all top-level packages"""
        ...
    root_element: Element
    """Get the root element of the model"""
    def write(self, /) -> None:
        """write the model to disk, creating or updating all files in the model"""
        ...

@final
class ByteOrder:
    """
    The `ByteOrder` is used to define the order of bytes in a multi-byte value
    """

    MostSignificantByteFirst: ByteOrder
    MostSignificantByteLast: ByteOrder
    Opaque: ByteOrder

@final
class EcuInstance:
    """
    The `EcuInstance` represents one ECU in a `System`Union[
    """

    def __init__(self, element: Element) -> EcuInstance: ...
    def communication_controllers(self, /) -> Iterator[CommunicationController]:
        """return an interator over all communication controllers in this `EcuInstance`"""
        ...

    def create_can_communication_controller(
        self, name: str, /
    ) -> CanCommunicationController:
        """Create a CAN-COMMUNICATION-CONTROLLER for this ECU-INSTANCE

        The ECU must have one controller per bus it communicates on.
        For example, if it communicates on two CAN buses, then two CAN-COMMUNICATION-CONTROLLERs are needed.
        """
        ...

    def create_ethernet_communication_controller(
        self, name: str, /, *, mac_address: Optional[str] = None
    ) -> EthernetCommunicationController:
        """Create an ETHERNET-COMMUNICATION-CONTROLLER for this ECU-INSTANCE

        The ECU must have one controller per bus it communicates on.
        For example, if it communicates on two CAN buses, then two CAN-COMMUNICATION-CONTROLLERs are needed.
        """
        ...

    def create_flexray_communication_controller(
        self, name: str, /
    ) -> FlexrayCommunicationController:
        """Create a FLEXRAY-COMMUNICATION-CONTROLLER for this ECU-INSTANCE

        The ECU must have one controller per bus it communicates on.
        For example, if it communicates on two CAN buses, then two CAN-COMMUNICATION-CONTROLLERs are needed.
        """
        ...
    element: Element
    name: str

@final
class SwcToEcuMapping:
    """
    A `SwcToEcuMapping` contains a mapping between a `SwComponentPrototype` and an `EcuInstance`
    """

    def __init__(self, element: Element) -> SwcToEcuMapping: ...
    ecu_instance: Optional[EcuInstance]
    """get the ECU instance which is the target of this mapping"""
    element: Element
    name: str
    target_component: Optional[SwComponentPrototype]
    """get the component prototype that is mapped here"""

@final
class System:
    """
    The System is the top level of a system template

    It defines how ECUs communicate with each other over various networks.
    It also contains the mapping of software components to ECUs.
    """

    def __init__(self, element: Element) -> System: ...
    category: Optional[SystemCategory]
    """category of the system"""
    def clusters(self, /) -> Iterator[Cluster]:
        """Create an iterator over all clusters connected to the SYSTEM"""
        ...

    def create_can_cluster(
        self,
        cluster_name: str,
        package: ArPackage,
        /,
        *,
        can_baudrate: Optional[int] = 500000,
    ) -> CanCluster:
        """create a new CAN-CLUSTER

        The cluster must have a channel to be valid, but this channel is not created automatically.
        Call [`CanCluster::create_physical_channel`] to create it.
        """
        ...

    def create_can_frame(
        self, name: str, package: ArPackage, byte_length: int, /
    ) -> CanFrame:
        """create a new [`CanFrame`]

        This new frame needs to be linked to a `CanPhysicalChannel`"""
        ...

    def create_can_tp_config(
        self, name: str, package: ArPackage, can_cluster: CanCluster, /
    ) -> CanTpConfig:
        """Create a `CanTpConfig` in the SYSTEM

        `CanTpConfig`s contain the configuration how to segment or reassemble diagnostic messages on a CAN bus.
        """
        ...

    def create_container_ipdu(
        self,
        name: str,
        package: ArPackage,
        length: int,
        header_type: ContainerIPduHeaderType,
        rx_accept: RxAcceptContainedIPdu,
        /,
    ) -> ContainerIPdu:
        """create a [`ContainerIPdu`] in the [`System`]"""
        ...

    def create_dcm_ipdu(self, name: str, package: ArPackage, length: int, diag_pdu_type: DiagPduType, /) -> DcmIPdu:
        """create a [`DcmIPdu`] in the [`System`]"""
        ...

    def create_doip_tp_config(
        self, name: str, package: ArPackage, eth_cluster: EthernetCluster, /
    ) -> DoIpTpConfig:
        """Create a `DoIpTpConfig` in the SYSTEM

        `DoIpTpConfig`s contain the configuration how to transmit diagnostic messages over IP networks.
        """
        ...

    def create_ecu_instance(self, name: str, package: ArPackage, /) -> EcuInstance:
        """create an `EcuInstance` that is connected to this System"""
        ...

    def create_ethernet_cluster(
        self, cluster_name: str, package: ArPackage, /
    ) -> EthernetCluster:
        """create a new ETHERNET-CLUSTER and connect it to the SYSTEM

        The cluster must have at least one channel to be valid.
        Call [`EthernetCluster.create_physical_channel`] to create it."""
        ...

    def create_fibex_element_ref(self, elem: Element, /) -> None:
        """connect an element to the SYSTEM by creating a FIBEX-ELEMENT-REF

        If there is already a FIBEX-ELEMENT-REF, this function does nothing, successfully.
        """
        ...

    def create_flexray_ar_tp_config(
        self, name: str, package: ArPackage, flexray_cluster: FlexrayCluster, /
    ) -> FlexrayArTpConfig:
        """Create a `FlexrayArTpConfig` in the SYSTEM

        `FlexrayArTpConfig`s describe how to segment or reassemble diagnostic messages on a Flexray bus.
        This configuration type is used for Flexray AUTOSAR TP communication."""
        ...

    def create_flexray_cluster(
        self, cluster_name: str, package: ArPackage, settings: FlexrayClusterSettings, /
    ) -> FlexrayCluster:
        """create a new FLEXRAY-CLUSTER and connect it to the SYSTEM

        A `FlexrayClusterSettings` structure containing the timings and parameters for the Flexray cluster must be provided.

        The cluster must have at least one channel to be valid.
        Call [`FlexrayCluster.create_physical_channel`] to create it."""
        ...

    def create_flexray_frame(
        self, name: str, package: ArPackage, byte_length: int, /
    ) -> FlexrayFrame:
        """create a new [`FlexrayFrame`]

        This new frame needs to be linked to a `FlexrayPhysicalChannel`"""
        ...

    def create_flexray_tp_config(
        self, name: str, package: ArPackage, flexray_cluster: FlexrayCluster, /
    ) -> FlexrayTpConfig:
        """Create a `FlexrayTpConfig` in the SYSTEM

        `FlexrayTpConfig`s describe how to segment or reassemble diagnostic messages on a Flexray bus.
        This configuration type is used for Flexray ISO TP communication."""
        ...

    def create_general_purpose_ipdu(
        self,
        name: str,
        package: ArPackage,
        length: int,
        category: GeneralPurposeIPduCategory,
        /,
    ) -> GeneralPurposeIPdu:
        """create a [`GeneralPurposeIPdu`] in the [`System`]"""
        ...

    def create_general_purpose_pdu(
        self,
        name: str,
        package: ArPackage,
        length: int,
        category: GeneralPurposePduCategory,
        /,
    ) -> GeneralPurposePdu:
        """create a [`GeneralPurposePdu`] in the [`System`]"""
        ...

    def create_isignal(
        self,
        name: str,
        package: ArPackage,
        bit_length: int,
        syssignal: SystemSignal,
        /,
        *,
        datatype: Optional[SwBaseType] = None,
    ) -> ISignal:
        """create a new isignal in the [`System`]"""
        ...

    def create_isignal_group(
        self, name: str, package: ArPackage, system_signal_group: SystemSignalGroup, /
    ) -> ISignalGroup:
        """create a new signal group in the [`System`]

        `I-SIGNAL-GROUP` and `SYSTEM-SIGNAL-GROUP` are created using the same name; therefore they must be placed in
        different packages: `sig_package` and `sys_package` may not be identical."""
        ...

    def create_isignal_ipdu(
        self, name: str, package: ArPackage, length: int, /
    ) -> ISignalIPdu:
        """create an [`ISignalIPdu`] in the [`System`]"""
        ...

    def create_multiplexed_ipdu(
        self, name: str, package: ArPackage, length: int, /
    ) -> MultiplexedIPdu:
        """create a [`MultiplexedIPdu`] in the [`System`]"""
        ...

    def create_n_pdu(self, name: str, package: ArPackage, length: int, /) -> NPdu:
        """create an [`NPdu`] in the [`System`]"""
        ...

    def create_nm_config(self, name: str, package: ArPackage, /) -> NmConfig:
        """Create a new `NmConfig` in the SYSTEM

        `NmConfig`s contain the configuration for network management.
        The System may contain zero or one `NmConfig`s."""
        ...

    def create_nm_pdu(self, name: str, package: ArPackage, length: int, /) -> NmPdu:
        """create an [`NmPdu`] in the [`System`]"""
        ...

    def create_secured_ipdu(
        self,
        name: str,
        package: ArPackage,
        length: int,
        secure_props: SecureCommunicationProps,
        /,
    ) -> SecuredIPdu:
        """create a [`SecuredIPdu`] in the [`System`]"""
        ...

    def create_service_instance_collection_set(
        self, name: str, package: ArPackage, /
    ) -> ServiceInstanceCollectionSet:
        """Create a `ServiceInstanceCollectionSet` in the SYSTEM

        `ServiceInstanceCollectionSet`s are part of the new ethernet modeling that was introduced in Autosar 4.5.0 (`AUTOSAR_00048`).
        """
        ...

    def create_so_ad_routing_group(
        self,
        name: str,
        package: ArPackage,
        /,
        *,
        control_type: Optional[EventGroupControlType] = None,
    ) -> SoAdRoutingGroup:
        """Create a `SoAdRoutingGroup` in the SYSTEM

        `SoAdRoutingGroup` are part of the old ethernet modeling that was used prior to Autosar 4.5.0 (`AUTOSAR_00048`).
        The elements are still present (but obsolete) in newer versions of the standard.
        Old and new elements may not be mixed in the same model."""
        ...

    def create_socket_connection_ipdu_identifier_set(
        self, name: str, package: ArPackage, /
    ) -> SocketConnectionIpduIdentifierSet:
        """Create a `SocketConnectionIpduIdentifierSet` in the SYSTEM

        `SocketConnectionIpduIdentifierSet` are part of the new ethernet modeling that was introduced in Autosar 4.5.0 (`AUTOSAR_00048`).
        """
        ...

    def create_someip_tp_config(
        self, name: str, package: ArPackage, cluster: Cluster, /
    ) -> SomeipTpConfig:
        """Create a `SomeipTpConfig` in the SYSTEM

        `SomeipTpConfig`s contain the configuration how to segment or reassemble large `SomeipTp` PDUs.
        """
        ...

    def ecu_instances(self, /) -> Iterator[EcuInstance]:
        """get an iterator over all ECU-INSTANCEs in this SYSTEM"""
        ...
    element: Element
    def frames(self, /) -> Iterator[Frame]:
        """iterate over all Frames in the System"""
        ...

    def get_or_create_mapping(self, name: str, /) -> SystemMapping:
        """get or create a mapping for this system

        There does not seem to be any benefit to having multiple mappings for a single system, so this function
        will return the first mapping if it exists. Otherwise a new mapping will be created with the provided name.
        """
        ...

    def isignal_groups(self, /) -> Iterator[ISignalGroup]:
        """iterate over all ISignalGroups in the System"""
        ...

    def isignals(self, /) -> Iterator[ISignal]:
        """iterate over all ISignals in the System

        This iterator returns all ISignals that are connected to the System using a FibexElementRef.
        """
        ...
    name: str
    def nm_config(self, /) -> Optional[NmConfig]:
        """Get the `NmConfig` of the SYSTEM, if any

        The System may contain zero or one `NmConfig`s."""
        ...

    def pdus(self, /) -> Iterator[Pdu]:
        """iterate over all PDUs in the System

        This iterator returns all PDUs that are connected to the System using a FibexElementRef.
        """
        ...
    pnc_vector_length: Optional[int]
    """set the pncVectorLength of the system"""
    pnc_vector_offset: Optional[int]
    """set the pncVectorOffset of the system"""
    root_sw_composition: Optional[RootSwCompositionPrototype]
    """get the root software composition of the system"""
    def set_root_sw_composition(
        self, name: str, composition_type: CompositionSwComponentType, /
    ) -> RootSwCompositionPrototype:
        """set the root software composition of the system

        This function will remove any existing root software composition"""
        ...

@final
class SystemCategory:
    """
    The category of a System
    """

    AbstractSystemDescription: SystemCategory
    EcuExtract: SystemCategory
    EcuSystemDescription: SystemCategory
    RptSystem: SystemCategory
    SwClusterSystemDescription: SystemCategory
    SystemConstraints: SystemCategory
    SystemDescription: SystemCategory
    SystemExtract: SystemCategory

@final
class SystemMapping:
    """
    A `SystemMapping` contains mappings in the `System`

    it contains mappings between SWCs and ECUs, as well as between ports and signals
    """

    def __init__(self, element: Element) -> SystemMapping: ...
    element: Element
    def map_sender_receiver_to_signal(
        self,
        signal: SystemSignal,
        data_element: VariableDataPrototype,
        port_prototype: PortPrototype,
        context_components: List[SwComponentPrototype],
        /,
        *,
        root_composition_prototype: Optional[RootSwCompositionPrototype] = None,
    ) -> None:
        """create a new mapping between a sender/receiver port and a signal

        `signal`: the system signal that the port is mapped to

        `data_element`: the data element that is mapped to the signal

        `port_prototype`: the port prototype that contains the data element

        `context_components`: a list of component prototypes from the root up to the component that directly contains the port.
        This list may be empty, or it could only contain the final application component prototype containing the port.

        `root_composition_prototype`: the root composition prototype that contains the `swc_prototype`.
        Rarely required, but may be needed if multiple root compositions use the same composition/component hierarchy.
        """
        ...

    def map_swc_to_ecu(
        self, name: str, component_prototype: SwComponentPrototype, ecu: EcuInstance, /
    ) -> SwcToEcuMapping:
        """create a new mapping between a SWC and an ECU"""
        ...
    name: str
    system: Optional[System]
    """get the system that contains this mapping"""
    ...
