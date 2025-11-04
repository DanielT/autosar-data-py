# Stub file for autosar_data.abstraction.communication

from typing import List, Optional, Tuple, TypeAlias, Union, Iterator, Type, final
from autosar_data import Element
from autosar_data.abstraction import ByteOrder, EcuInstance, System
from autosar_data.abstraction.datatype import (
    CompuMethod,
    DataConstr,
    SwBaseType,
    Unit,
    ValueSpecification,
)

Pdu: TypeAlias = Union[
    ContainerIPdu,
    DcmIPdu,
    GeneralPurposePdu,
    GeneralPurposeIPdu,
    NPdu,
    ISignalIPdu,
    NmPdu,
    SecuredIPdu,
    MultiplexedIPdu,
]
IPdu: TypeAlias = Union[
    ContainerIPdu,
    DcmIPdu,
    GeneralPurposeIPdu,
    ISignalIPdu,
    SecuredIPdu,
    NPdu,
    MultiplexedIPdu,
]
Cluster: TypeAlias = Union[CanCluster, FlexrayCluster, EthernetCluster, LinCluster]
CommunicationController: TypeAlias = Union[
    CanCommunicationController,
    FlexrayCommunicationController,
    EthernetCommunicationController,
    LinMaster,
    LinSlave,
]
Frame: TypeAlias = Union[
    CanFrame,
    FlexrayFrame,
    LinUnconditionalFrame,
    LinSporadicFrame,
    LinEventTriggeredFrame,
]
TransformationTechnologyConfig: TypeAlias = Union[
    ComTransformationTechnologyConfig,
    E2ETransformationTechnologyConfig,
    SomeIpTransformationTechnologyConfig,
    GenericTransformationTechnologyConfig,
]
PhysicalChannel: TypeAlias = Union[
    CanPhysicalChannel,
    FlexrayPhysicalChannel,
    EthernetPhysicalChannel,
    LinPhysicalChannel,
]

@final
class CanAddressingMode:
    """
    The addressing mode for a CAN frame
    """

    Extended: CanAddressingMode
    Standard: CanAddressingMode

@final
class CanCluster:
    """
    A `CanCluster` contains all configuration items associated with a CAN network.
    The cluster connects multiple ECUs.
    """

    def __init__(self, element: Element) -> CanCluster: ...
    baudrate: int
    """get or set the baudrate of the cluster"""
    can_fd_baudrate: Optional[int]
    """get or set the CAN FD baudrate of the cluster"""
    can_xl_baudrate: Optional[int]
    """get or set the CAN XL baudrate of the cluster"""
    def create_physical_channel(self, channel_name: str, /) -> CanPhysicalChannel:
        """Create a new physical channel for the cluster

        A can cluster must contain exactly one physical channel; trying to add a second one triggers an error.
        """
        ...
    element: Element
    name: str
    physical_channel: Optional[CanPhysicalChannel]
    """get or set the settings of this `CanCluster` with new values for the baudrates"""
    system: Optional[System]
    """get the `System` that contains this `CanCluster`"""

@final
class CanCommunicationConnector:
    """
    A connector between a [`CanCommunicationController`] in an ECU and a [`CanPhysicalChannel`]
    """

    def __init__(self, element: Element) -> CanCommunicationConnector: ...
    controller: CanCommunicationController
    """Get the controller of the `CommunicationConnector`"""
    ecu_instance: EcuInstance
    """`EcuInstance` that contains this `CommunicationConnector`"""
    element: Element
    name: str

@final
class CanCommunicationController:
    """
    An `EcuInstance` needs a `CanCommunicationController` in order to connect to a CAN cluster.
    """

    def __init__(self, element: Element) -> CanCommunicationController: ...
    def connect_physical_channel(
        self, connection_name: str, can_channel: CanPhysicalChannel, /
    ) -> CanCommunicationConnector:
        """Connect this [`CanCommunicationController`] inside an [`EcuInstance`] to a [`CanPhysicalChannel`] in the [`crate::System`]

        Creates a [`CanCommunicationConnector`] in the [`EcuInstance`] that contains this [`CanCommunicationController`].

        This function establishes the relationships:
         - [`CanPhysicalChannel`] -> [`CanCommunicationConnector`]
         - [`CanCommunicationConnector`] -> [`CanCommunicationController`]"""
        ...

    def connected_channels(self, /) -> Iterator[CanPhysicalChannel]:
        """return an iterator over the [`CanPhysicalChannel`]s connected to this controller"""
        ...
    ecu_instance: EcuInstance
    """Get the `EcuInstance` that contains this `CanCommunicationController`"""
    element: Element
    name: str

@final
class CanFrame:
    """
    A frame on a CAN bus
    """

    def __init__(self, element: Element) -> CanFrame: ...
    element: Element
    def frame_triggerings(self, /) -> List[CanFrameTriggering]:
        """List all [`FrameTriggering`]s using this frame"""
        ...
    length: Optional[int]
    """length of the frame"""
    def map_pdu(
        self,
        pdu: Pdu,
        start_position: int,
        byte_order: ByteOrder,
        /,
        *,
        update_bit: Optional[int] = None,
    ) -> PduToFrameMapping:
        """map a PDU to the frame"""
        ...

    def mapped_pdus(self, /) -> Iterator[Pdu]:
        """returns an iterator over all PDUs in the frame"""
        ...
    name: str

@final
class CanFrameTriggering:
    """
    The frame triggering connects a frame to a physical channel
    """

    def __init__(self, element: Element) -> CanFrameTriggering: ...
    addressing_mode: Optional[CanAddressingMode]
    """set the addressing mode for this frame triggering"""
    def connect_to_ecu(
        self, ecu: EcuInstance, direction: CommunicationDirection, /
    ) -> FramePort:
        """connect this frame triggering to an ECU

        The direction parameter specifies if the communication is incoming or outgoing
        """
        ...
    element: Element
    frame: Optional[CanFrame]
    """get the frame associated with this frame triggering"""
    def frame_ports(self, /) -> Iterator[FramePort]:
        """iterate over all frame ports for this frame triggering"""
        ...
    frame_type: Optional[CanFrameType]
    """frame type for this frame triggering"""
    identifier: Optional[int]
    """can id associated with this frame"""
    name: str
    def pdu_triggerings(self, /) -> Iterator[PduTriggering]:
        """iterate over all PDU triggerings referenced by the frame triggering"""
        ...
    physical_channel: CanPhysicalChannel
    """get the physical channel that contains this frame triggering"""

@final
class CanFrameType:
    """
    The type of a CAN frame
    """

    Any: CanFrameType
    Can20: CanFrameType
    CanFd: CanFrameType

@final
class CanNmCluster:
    """
    Can specific `NmCluster` attributes
    """

    def __init__(self, element: Element) -> CanNmCluster: ...
    channel_sleep_master: Optional[bool]
    """set or remove the nmChannelSleepMaster flag"""
    communication_cluster: Optional[CanCluster]
    """get or set the referenced `CanCluster`"""
    def create_can_nm_node(
        self, name: str, controller: CanCommunicationController, nm_ecu: NmEcu, /
    ) -> CanNmNode:
        """add a `CanNmNode` to the cluster"""
        ...
    element: Element
    name: str
    nm_busload_reduction_active: Optional[bool]
    """nmBusloadReductionActive flag"""
    nm_immediate_nm_transmissions: Optional[int]
    """nmImmediateNmTransmissions value"""
    nm_message_timeout_time: Optional[float]
    """get or set the nmMessageTimeoutTime"""
    nm_msg_cycle_time: Optional[float]
    """get or set the nmMsgCycleTime"""
    nm_network_timeout: Optional[float]
    """get or set the nmNetworkTimeout"""
    def nm_nodes(self, /) -> Iterator[CanNmNode]:
        """iterate over all `NmNodes` in this cluster"""
        ...
    nm_remote_sleep_indication_time: Optional[float]
    """get or set the nmRemoteSleepIndicationTime"""
    nm_repeat_message_time: Optional[float]
    """get or set the nmRepeatMessageTime"""
    nm_wait_bus_sleep_time: Optional[float]
    """get or set the nmWaitBusSleepTime"""
    node_detection_enabled: Optional[bool]
    """get or set the nmNodeDetectionEnabled flag"""
    node_id_enabled: Optional[bool]
    """get or set the nmNodeIdEnabled flag"""
    pnc_cluster_vector_length: Optional[int]
    """get or set the pncClusterVectorLength"""
    pnc_participation: Optional[bool]
    """get or set the nmPncParticipation flag"""
    repeat_msg_ind_enabled: Optional[bool]
    """get or set the nmRepeatMsgIndEnabled flag"""
    synchronizing_network: Optional[bool]
    """get or set the nmSynchronizingNetwork flag"""

@final
class CanNmClusterCoupling:
    """
    A `CanNmClusterCoupling` couples multiple `CanNmCluster`s, and contains CAN specific settings.
    """

    def __init__(self, element: Element) -> CanNmClusterCoupling: ...
    def add_coupled_cluster(self, cluster: CanNmCluster, /) -> None:
        """add a reference to a coupled `NmCluster`"""
        ...

    def coupled_clusters(self, /) -> Iterator[CanNmCluster]:
        """iterate over all coupled `NmClusters`"""
        ...
    element: Element
    nm_busload_reduction_enabled: Optional[bool]
    """get or set the nmBusloadReductionEnabled flag"""
    nm_immediate_restart_enabled: Optional[bool]
    """get or set the nmImmediateRestartEnabled flag"""

@final
class CanNmClusterSettings:
    """
    Mandatory settings for a `CanNmCluster`

    These settings are mandatory for a `CanNmCluster` and must be set during creation.
    Additional optional settings can be set using the `CanNmCluster` methods.
    """

    def __init__(
        self,
        *,
        nm_busload_reduction_active: bool,
        nm_immediate_nm_transmissions: int,
        nm_message_timeout_time: float,
        nm_msg_cycle_time: float,
        nm_network_timeout: float,
        nm_remote_sleep_indication_time: float,
        nm_repeat_message_time: float,
        nm_wait_bus_sleep_time: float,
    ) -> CanNmClusterSettings: ...
    nm_busload_reduction_active: bool
    """nmBusloadReductionActive: Determines if bus load reduction for the respective `CanNm` channel is active."""
    nm_immediate_nm_transmissions: int
    """nmImmediateNmTransmissions: Defines the number of immediate `NmPdus` which shall be transmitted.
    If the value is zero no immediate `NmPdus` are transmitted."""
    nm_message_timeout_time: float
    """nmMessageTimeoutTime: Timeout of an `NmPdu` in seconds."""
    nm_msg_cycle_time: float
    """nmMsgCycleTime: Period of a `NmPdu` in seconds"""
    nm_network_timeout: float
    """nmNetworkTimeout: Network Timeout for `NmPdus` in seconds."""
    nm_remote_sleep_indication_time: float
    """nmRemoteSleepIndicationTime: Timeout for Remote Sleep Indication in seconds."""
    nm_repeat_message_time: float
    """nmRepeatMessageTime: Timeout for Repeat Message State in seconds."""
    nm_wait_bus_sleep_time: float
    """nmWaitBusSleepTime: Timeout for bus calm down phase in seconds."""

@final
class CanNmNode:
    """
    A `CanNmNode` represents a node in a `CanNmCluster`.

    The node connects to a `CanCommunicationController` and an `NmEcu`.
    """

    def __init__(self, element: Element) -> CanNmNode: ...
    def add_rx_nm_pdu(self, nm_pdu: NmPdu, /) -> None:
        """add an Rx `NmPdu`

        Every `NmNode` must have at least one Rx `NmPdu`"""
        ...

    def add_tx_nm_pdu(self, nm_pdu: NmPdu, /) -> None:
        """add a Tx `NmPdu`

        Active `NmNodes` must have at least one Tx `NmPdu`, while passive `NmNodes` may have none.
        """
        ...
    communication_controller: Optional[CanCommunicationController]
    """get or set the referenced `CanCommunicationController`"""
    element: Element
    name: str
    nm_ecu: Optional[NmEcu]
    """get or set the referenced `NmEcu`"""
    node_id: Optional[int]
    """get or set the nmNodeId
    This value is optional; if it is set to Some(x) the value is created, if it is set to None the value is removed."""
    passive_mode: Optional[bool]
    """get or set ot remove the nmPassiveModeEnabled flag

    This flag is optional; if it is set to Some(x) the value is created, if it is set to None the value is removed."""
    def rx_nm_pdus(self, /) -> Iterator[NmPdu]:
        """iterate over all RX `NmPdus`"""
        ...

    def tx_nm_pdus(self, /) -> Iterator[NmPdu]:
        """iterate over all TX `NmPdus`"""
        ...

@final
class CanPhysicalChannel:
    """
    The `CanPhysicalChannel contains all of the communication on a CAN network
    """

    def __init__(self, element: Element) -> CanPhysicalChannel: ...
    cluster: CanCluster
    """get the cluster containing this physical channel"""
    element: Element
    def frame_triggerings(self, /) -> Iterator[CanFrameTriggering]:
        """iterate over all frame triggerings of this physical channel"""
        ...

    def signal_triggerings(self, /) -> Iterator[ISignalTriggering]:
        """iterate over all signal triggerings of this physical channel"""
        ...

    def pdu_triggerings(self, /) -> Iterator[PduTriggering]:
        """iterate over all PDU triggerings of this physical channel"""
        ...
    name: str
    def trigger_frame(
        self,
        frame: CanFrame,
        identifier: int,
        addressing_mode: CanAddressingMode,
        frame_type: CanFrameType,
        /,
    ) -> CanFrameTriggering:
        """add a trigger for a CAN frame in this physical channel"""
        ...

@final
class CanTpAddress:
    """
    A `CanTpAddress` represents a logical address in the `CanTp` module
    """

    def __init__(self, element: Element) -> CanTpAddress: ...
    element: Element
    name: str
    tp_address: Optional[int]
    """get or set the address of the `CanTpAddress`"""

@final
class CanTpAddressingFormat:
    """
    The addressing format of a `CanTpConnection`
    """

    Extended: CanTpAddressingFormat
    Mixed: CanTpAddressingFormat
    Mixed29Bit: CanTpAddressingFormat
    NormalFixed: CanTpAddressingFormat
    Standard: CanTpAddressingFormat

@final
class CanTpChannel:
    """
    A `CanTpChannel` represents a channel in the `CanTp` module
    """

    def __init__(self, element: Element) -> CanTpChannel: ...
    channel_id: Optional[int]
    """get or set the channel id of the channel"""
    channel_mode: Optional[CanTpChannelMode]
    """get or set the channel mode of the channel"""
    element: Element
    name: str

@final
class CanTpChannelMode:
    """
    The mode of a `CanTpChannel`
    """

    FullDuplex: CanTpChannelMode
    HalfDuplex: CanTpChannelMode

@final
class CanTpConfig:
    """
    Container for `CanTp` configuration

    There should be one `CanTpConfig` for each CAN network in the system
    """

    def __init__(self, element: Element) -> CanTpConfig: ...
    def can_tp_addresses(self, /) -> Iterator[CanTpAddress]:
        """get all of the Can Tp addresses in the configuration"""
        ...

    def can_tp_channels(self, /) -> Iterator[CanTpChannel]:
        """iterate over all `CanTpChannel`s in the configuration"""
        ...

    def can_tp_connections(self, /) -> Iterator[CanTpConnection]:
        """get all of the `CanTpConnections` in the configuration"""
        ...

    def can_tp_ecus(self, /) -> Iterator[CanTpEcu]:
        """get an iterator over all ECUs in the configuration"""
        ...

    def can_tp_nodes(self, /) -> Iterator[CanTpNode]:
        """get all of the `CanTpNodes` in the configuration"""
        ...
    cluster: Optional[CanCluster]
    """get or set the `CanCluster` associated with this configuration"""
    def create_can_tp_address(self, name: str, address: int) -> CanTpAddress:
        """create a new `CanTpAddress` in the configuration"""
        ...

    def create_can_tp_channel(
        self, name: str, channel_id: int, mode: CanTpChannelMode, /
    ) -> CanTpChannel:
        """create a new `CanTpChannel` in the configuration"""
        ...

    def create_can_tp_connection(
        self,
        name: Optional[str],
        addressing_format: CanTpAddressingFormat,
        can_tp_channel: CanTpChannel,
        data_pdu: NPdu,
        tp_sdu: IPdu,
        padding_activation: bool,
        /,
    ) -> CanTpConnection:
        """create a new `CanTpConnection` in the configuration"""
        ...

    def create_can_tp_ecu(
        self,
        ecu_instance: EcuInstance,
        /,
        *,
        cycle_time_main_function: Optional[float] = None,
    ) -> CanTpEcu:
        """create a `CanTp` ECU in the configuration"""
        ...

    def create_can_tp_node(self, name: str, /) -> CanTpNode:
        """create a new `CanTpNode` in the configuration"""
        ...
    element: Element
    name: str

@final
class CanTpConnection:
    """
    A connection identifies the sender and the receiver of this particular communication.
    The `CanTp` module routes a Pdu through this connection.
    """

    def __init__(self, element: Element) -> CanTpConnection: ...
    def add_receiver(self, receiver: CanTpNode, /) -> None:
        """add a receiver to the connection

        This is a `CanTpNode` representing an ECU that will receive the data"""
        ...
    addressing_format: Optional[CanTpAddressingFormat]
    """get or set the addressing format of the connection"""
    channel: Optional[CanTpChannel]
    """get or set the `CanTpChannel` associated with this connection"""
    data_pdu: Optional[NPdu]
    """get or set the `NPdu` associated with this connection"""
    element: Element
    name: str
    padding_activation: Optional[bool]
    """get or set the padding activation of the connection"""
    def receivers(self, /) -> Iterator[CanTpNode]:
        """get all of the receivers of the connection"""
        ...
    tp_sdu: Optional[IPdu]
    """get or set the `IPdu` associated with this connection"""
    transmitter: Optional[CanTpNode]
    """set the transmitter of the connection
    
    This is a `CanTpNode` representing an ECU that will send the data"""

@final
class CanTpEcu:
    """
    A `CanTpEcu` represents an ECU that is using the `CanTp` module
    """

    def __init__(self, element: Element) -> CanTpEcu: ...
    cycle_time_main_function: Optional[float]
    """get or set the cycle time of the `CanTp` main function of the ECU"""
    ecu_instance: Optional[EcuInstance]
    """get or set the ECU instance of the `CanTpEcu`"""
    element: Element

@final
class CanTpNode:
    """
    A `CanTpNode` provides the TP address and the connection to the topology description in a `CanTpConfig`
    """

    def __init__(self, element: Element) -> CanTpNode: ...
    address: Optional[CanTpAddress]
    """get or set the `CanTpAddress` of this Node"""
    connector: Optional[CanCommunicationConnector]
    """set the reference to a `CanCommunicationConnector` between an `EcuInstance` and a `CanPhysicalChannel`
    
    The connector connects the ECU to the physical channel, so by setting this reference, the
    ECU is also connected to the `CanTpNode`"""
    element: Element
    name: str

@final
class ComTransformationTechnologyConfig:
    """
    Configuration for a COM transformation
    """

    def __init__(
        self, *, isignal_ipdu_length: int
    ) -> ComTransformationTechnologyConfig: ...
    isignal_ipdu_length: int
    """The length of the `ISignalIpdu` tha will be transformed by this Com transformer.
    The value is only used up to AUTOSAR R20-11 (`AUTOSAR_00049`), where it is needed to calculate the buffer size."""

@final
class CommonServiceDiscoveryConfig:
    """
    A `CommonServiceDiscoveryConfig` contains common configuration settings for `System::configure_service_discovery_for_ecu`.

    This struct contains ECU-independent settings that should be re-used for all ECUs that are configured for SD.
    """

    def __init__(
        self,
        *,
        multicast_rx_socket: SocketAddress,
        multicast_rx_pdu: GeneralPurposePdu,
        remote_socket: SocketAddress,
        prefer_static_socket_connections: bool,
        ipdu_identifier_set: Optional[SocketConnectionIpduIdentifierSet] = None,
        name_prefix: Optional[str] = None,
    ) -> CommonServiceDiscoveryConfig: ...
    ipdu_identifier_set: Optional[SocketConnectionIpduIdentifierSet]
    """an ipdu identifier set in which `PduTriggerings` are created. Only needed for `StaticSocketConnections`."""
    multicast_rx_pdu: GeneralPurposeIPdu
    """the multicast rx PDU used by all SD ECUs"""
    multicast_rx_socket: SocketAddress
    """the socket address used for multicast rx by all SD ECUs"""
    name_prefix: Optional[str]
    """an optional prefix for the names of the created elements"""
    prefer_static_socket_connections: bool
    """`configure_service_discovery_for_ecu` checks if any `SocketConnectionBundles` exist. If so, the old configuration method must be used.
    If none are found and the version is new enough, both methods are possible, and this flag determines which one to use."""
    remote_socket: SocketAddress
    """the remote socket used for SD communication. This socket must have an IP address (v4 or v6) set to ANY."""

@final
class CommunicationDirection:
    """
    The [`CommunicationDirection`] is used by the communication ports for frames, PDUs and signals
    """

    In: CommunicationDirection
    Out: CommunicationDirection

@final
class ConsumedEventGroup:
    """
    A `ConsumedEventGroup` is a group of events in a `ConsumedServiceInstance` that are consumed by an ECU
    """

    def __init__(self, element: Element) -> ConsumedEventGroup: ...
    def add_event_multicast_address(self, address: SocketAddress, /) -> None:
        """add an event multicast address to this `ConsumedEventGroup`"""
        ...

    def create_pdu_activation_routing_group(
        self, name: str, event_group_control_type: EventGroupControlType, /
    ) -> PduActivationRoutingGroup:
        """create a new `PduActivationRoutingGroup` in this `ConsumedEventGroup`"""
        ...
    element: Element
    event_group_identifier: Optional[int]
    """get or set the event group identifier of this `ConsumedEventGroup`"""
    def event_multicast_addresses(self, /) -> Iterator[SocketAddress]:
        """get the event multicast addresses"""
        ...
    name: str
    def pdu_activation_routing_groups(self, /) -> Iterator[PduActivationRoutingGroup]:
        """iterate over the `PduActivationRoutingGroup`s in this `ConsumedEventGroup`"""
        ...
    sd_client_timer_config: Optional[SomeipSdClientEventGroupTimingConfig]
    """get or set the SD client timer configuration for this `ConsumedEventGroup`"""

@final
class ConsumedEventGroupV1:
    """
    A `ConsumedEventGroupV1` is a SD event group of a service instance that is consumed by this ECU.

    This is the old V1 version of the service definition.
    """

    def __init__(self, element: Element) -> ConsumedEventGroupV1: ...
    def add_routing_group(self, routing_group: SoAdRoutingGroup, /) -> None:
        """add a reference to a `SoAdRoutingGroup` to this `ConsumedEventGroup`"""
        ...
    application_endpoint: Optional[SocketAddress]
    """set the `SocketAddress` that receives events from this `ConsumedEventGroup`
    This may be a different `SocketAddress` than the one that is used to send requests."""
    element: Element
    event_group_identifier: Optional[int]
    """get or set the event group identifier of this `ConsumedEventGroup`"""
    def event_handlers(self, /) -> List[EventHandlerV1]:
        """list all `EventHandlerV1`s that reference this `ConsumedEventGroupV1`"""
        ...
    name: str
    def routing_groups(self, /) -> Iterator[SoAdRoutingGroup]:
        """get the routing groups referenced by this `ConsumedEventGroup`"""
        ...
    sd_client_config: Optional[SdEventConfig]
    """get or set the SD client configuration for this `ConsumedEventGroup`"""

@final
class ConsumedServiceInstance:
    """
    A `ConsumedServiceInstance` is a service that is consumed by an ECU
    """

    def __init__(self, element: Element) -> ConsumedServiceInstance: ...
    def consumed_event_groups(self, /) -> Iterator[ConsumedEventGroup]:
        """get the `ConsumedEventGroup`s in this `ConsumedServiceInstance`"""
        ...

    def create_consumed_event_group(
        self, name: str, event_group_identifier: int, /
    ) -> ConsumedEventGroup:
        """create a new `ConsumedEventGrup` in this `ConsumedServiceInstance`"""
        ...
    element: Element
    instance_identifier: Optional[int]
    """get or set the instance identifier of this `ConsumedServiceInstance`"""
    def local_unicast_addresses(self, /) -> Iterator[SocketAddress]:
        """iterate over the local unicast addresses"""
        ...
    major_version: Optional[int]
    """get or set the major version of this `ConsumedServiceInstance`"""
    minor_version: Optional[Union[int, str]]
    """get or set the minor version of this `ConsumedServiceInstance`
    
    The minor version can be a number or the String "ANY"."""
    name: str
    sd_client_instance_config: Optional[SomeipSdClientServiceInstanceConfig]
    """get or set the SD client instance configuration for this `ConsumedServiceInstance`"""
    service_identifier: Optional[int]
    """get or set the service identifier of this `ConsumedServiceInstance`"""
    def set_local_unicast_address(self, address: SocketAddress, /) -> None:
        """set a local unicast address for this `ConsumedServiceInstance`

        The CSI may use two local unicast addresses, one each for UDP and TCP.
        If the consumed service instance does not specify a local unicast address
        because it only receives multicast messages, then the `ConsumedEventGroup`
        must have an eventMulticastAddress."""
        ...

@final
class ConsumedServiceInstanceV1:
    """
    A `ConsumedServiceInstanceV1` is a SD service instance that is consumed by this ECU.

    This is the old V1 version of the service definition.
    """

    def __init__(self, element: Element) -> ConsumedServiceInstanceV1: ...
    def consumed_event_groups(self, /) -> Iterator[ConsumedEventGroupV1]:
        """get the `ConsumedEventGroup`s in this `ConsumedServiceInstanceV1`"""
        ...

    def create_consumed_event_group(
        self, name: str, event_group_identifier: int, event_handler: EventHandlerV1, /
    ) -> ConsumedEventGroupV1:
        """create a new `ConsumedEventGroupV1` in this `ConsumedServiceInstanceV1`"""
        ...
    element: Element
    name: str
    provided_service_instance: Optional[ProvidedServiceInstanceV1]
    """get the `ProvidedServiceInstanceV1` referenced by this `ConsumedServiceInstanceV1`"""
    def sd_client_config(self) -> Optional[SdConfig]:
        """get the SD client configuration for this `ConsumedServiceInstanceV1`"""
        ...

    def set_sd_client_config(self, sd_client_config: SdConfig, /) -> None:
        """set the SD client configuration for this `ConsumedServiceInstanceV1`"""
        ...

@final
class ContainedIPduCollectionSemantics:
    """collection semantics for the ContainedIPdu"""

    LastIsBest: ContainedIPduCollectionSemantics
    Queued: ContainedIPduCollectionSemantics

@final
class ContainedIPduProps:
    """Properties for an IPdu that is transmitted in a container IPdu"""

    def __init__(
        self,
        *,
        collection_semantics: Optional[ContainedIPduCollectionSemantics] = None,
        header_id_long: Optional[int] = None,
        header_id_short: Optional[int] = None,
        offset: Optional[int] = None,
        priority: Optional[int] = None,
        timeout: Optional[float] = None,
        trigger: Optional[PduCollectionTrigger] = None,
        update_indication_bit_position: Optional[int] = None,
    ) -> ContainedIPduProps: ...
    collection_semantics: Optional[ContainedIPduCollectionSemantics]
    """collection semantics: LastIsBest or Queued"""
    header_id_long: Optional[int]
    """header id of the contained IPdu, used when the header type is LongHeader"""
    header_id_short: Optional[int]
    """header id of the contained IPdu, used when the header type is ShortHeader"""
    offset: Optional[int]
    """offset of the contained IPdu in the container IPdu, used when the header type is NoHeader"""
    priority: Optional[int]
    """priority of the contained IPdu. 255: lowest, 0: highest"""
    timeout: Optional[float]
    """sender timeout. Ignored on the receiver side"""
    trigger: Optional[PduCollectionTrigger]
    """defines whether the contained IPdu triggers transmission of the container IPdu"""
    update_indication_bit_position: Optional[int]
    """update indication bit position of the contained IPdu"""

@final
class ContainerIPdu:
    """
    Several `IPdus` can be collected in one `ContainerIPdu` based on the headerType
    """

    def __init__(self, element: Element) -> ContainerIPdu: ...
    contained_ipdu_props: Optional[ContainedIPduProps]
    """set the ContainedIPduProps for this `IPdu`
    This is only needed when the `IPdu` is contained in a `ContainerIPdu`"""
    def contained_ipdu_triggerings(self, /) -> Iterator[PduTriggering]:
        """iterate over all contained IPdu triggerings"""
        ...
    container_timeout: Optional[float]
    container_trigger: Optional[ContainerIPduTrigger]
    """get or set the container trigger of this `ContainerIPdu`"""
    element: Element
    header_type: ContainerIPduHeaderType
    length: Optional[int]
    """get or set the length of this PDU"""
    def map_ipdu(
        self, ipdu: IPdu, physical_channel: PhysicalChannel, /
    ) -> PduTriggering:
        """map an `IPdu` to this `ContainerIPdu` and create a `PduTriggering` for it in the `PhysicalChannel`"""
        ...
    name: str
    rx_accept_contained_ipdu: Optional[RxAcceptContainedIPdu]
    """get or set the rx accept of this `ContainerIPdu`"""
    def pdu_triggerings(self, /) -> List[PduTriggering]:
        """list all `PduTriggerings` that trigger this PDU"""
        ...

@final
class ContainerIPduHeaderType:
    """The header type of a `ContainerIPdu`"""

    LongHeader: ContainerIPduHeaderType
    NoHeader: ContainerIPduHeaderType
    ShortHeader: ContainerIPduHeaderType

@final
class ContainerIPduTrigger:
    """Defines when the transmission of the ContainerIPdu shall be requested"""

    DefaultTrigger: ContainerIPduTrigger
    FirstContainedTrigger: ContainerIPduTrigger

@final
class CycleRepetition:
    """
    The cycle repetition of a Flexray frame, from the Flexray standard
    """

    C1: CycleRepetition
    C2: CycleRepetition
    C4: CycleRepetition
    C5: CycleRepetition
    C8: CycleRepetition
    C10: CycleRepetition
    C16: CycleRepetition
    C20: CycleRepetition
    C32: CycleRepetition
    C40: CycleRepetition
    C50: CycleRepetition
    C64: CycleRepetition

@final
class CyclicTiming:
    """
    Cyclic timing parameters for an IPDU
    """

    def __init__(
        self,
        time_period: float,
        /,
        *,
        time_offset: Optional[float] = None,
    ) -> CyclicTiming: ...
    time_period: float
    """period of repetition in seconds"""
    time_offset: Optional[float]
    """delay until the first transmission of the PDU in seconds"""

@final
class DataIdMode:
    """
    data ID modes for E2E profiles 01 and 11
    """

    All16Bit: DataIdMode
    Alternating8Bit: DataIdMode
    Lower12Bit: DataIdMode
    Lower8Bit: DataIdMode

@final
class DataTransformation:
    """
    A `DataTransformation` is a chain of `TransformationTechnology`s that are used to transform data
    """

    def __init__(self, element: Element) -> DataTransformation: ...
    data_transformation_set: Optional[DataTransformationSet]
    """get the `DataTransformationSet` that contains this `DataTransformation`"""
    element: Element
    name: str
    def transformation_technologies(self, /) -> Iterator[TransformationTechnology]:
        """Create an iterator over the `TransformationTechnologies` in the `DataTransformation`"""
        ...

@final
class DataTransformationSet:
    """
    A [`DataTransformationSet`] contains `DataTransformation`s and `TransformationTechnology`s used in communication

    Use [`ArPackage::create_data_transformation_set`] to create a new `DataTransformationSet`
    """

    def __init__(self, element: Element) -> DataTransformationSet: ...
    def create_data_transformation(
        self,
        name: str,
        transformations: List[TransformationTechnology],
        execute_despite_data_unavailability: bool,
        /,
    ) -> DataTransformation:
        """Create a new `DataTransformation` in the `DataTransformationSet`"""
        ...

    def create_transformation_technology(
        self, name: str, config: TransformationTechnologyConfig, /
    ) -> TransformationTechnology:
        """Create a new `TransformationTechnology` in the `DataTransformationSet`"""
        ...

    def data_transformations(self, /) -> Iterator[DataTransformation]:
        """Iterate over all `DataTransformation`s in the `DataTransformationSet`"""
        ...
    element: Element
    name: str
    def transformation_technologies(self, /) -> Iterator[TransformationTechnology]:
        """Iterate over all `TransformationTechnology`s in the `DataTransformationSet`"""
        ...

@final
class DcmIPdu:
    """
    Represents the `IPdus` handled by Dcm
    """

    def __init__(self, element: Element) -> DcmIPdu: ...
    contained_ipdu_props: Optional[ContainedIPduProps]
    """set the ContainedIPduProps for this `IPdu`
    This is only needed when the `IPdu` is contained in a `ContainerIPdu`"""

    element: Element
    length: Optional[int]
    """get or set the length of this PDU"""
    name: str
    def pdu_triggerings(self, /) -> List[PduTriggering]:
        """list all `PduTriggerings` that trigger this PDU"""
        ...
    diag_pdu_type: Optional[DiagPduType]
    """get or set the type of this diagnostic PDU"""

@final
class DiagPduType:
    """
    The type of a diagnostic PDU
    """

    DiagRequest: DiagPduType
    DiagResponse: DiagPduType

@final
class DoIpLogicAddress:
    """
    This element defines the logical address of a `DoIp` connection
    """

    def __init__(self, element: Element) -> DoIpLogicAddress: ...
    address: Optional[int]
    """get or set the address of this `DoIpLogicAddress`"""
    element: Element
    name: str

@final
class DoIpTpConfig:
    """
    Container for `DoIp` TP configuration
    """

    def __init__(self, element: Element) -> DoIpTpConfig: ...
    cluster: Optional[EthernetCluster]
    """get or set the reference to the `EthernetCluster` for this `DoIpTpConfig`"""
    def create_doip_logic_address(self, name: str, address: int, /) -> DoIpLogicAddress:
        """create a new `DoIpLogicAddress`"""
        ...

    def create_doip_tp_connection(
        self,
        name: Optional[str],
        source: DoIpLogicAddress,
        target: DoIpLogicAddress,
        tp_sdu_triggering: PduTriggering,
        /,
    ) -> DoIpTpConnection:
        """create a new `DoIpTpConnection`"""
        ...

    def doip_logic_addresses(self, /) -> Iterator[DoIpLogicAddress]:
        """iterate over all `DoIpLogicAddresss`"""
        ...

    def doip_tp_connections(self, /) -> Iterator[DoIpTpConnection]:
        """iterate over all `DoIpTpConnections`"""
        ...
    element: Element
    name: str

@final
class DoIpTpConnection:
    """
    The `DoIpTpConnection` defines a `DoIp` transport protocol connection
    """

    def __init__(self, element: Element) -> DoIpTpConnection: ...
    element: Element
    name: str
    source: Optional[DoIpLogicAddress]
    """get or set the source `DoIpLogicAddress`"""
    target: Optional[DoIpLogicAddress]
    """get or set the target `DoIpLogicAddress`"""
    tp_sdu_triggering: Optional[PduTriggering]
    """get or set the `PduTriggering` for this connection"""

@final
class E2EProfile:
    """
    enumeration of the possible E2E profiles
    """

    P01: E2EProfile
    P02: E2EProfile
    P04: E2EProfile
    P04m: E2EProfile
    P05: E2EProfile
    P06: E2EProfile
    P07: E2EProfile
    P07m: E2EProfile
    P08: E2EProfile
    P08m: E2EProfile
    P11: E2EProfile
    P22: E2EProfile
    P44: E2EProfile
    P44m: E2EProfile

@final
class E2EProfileBehavior:
    """
    there are two standardized behaviors for E2E profiles, which can be selected for each E2E transformation
    """

    PreR4_2: E2EProfileBehavior
    R4_2: E2EProfileBehavior

@final
class E2ETransformationTechnologyConfig:
    """
    Configuration for an E2E transformation
    """

    def __init__(
        self,
        *,
        profile: E2EProfile,
        zero_header_length: bool,
        transform_in_place: bool,
        offset: int,
        max_delta_counter: int,
        max_error_state_init: int,
        max_error_state_invalid: int,
        max_error_state_valid: int,
        max_no_new_or_repeated_data: int,
        min_ok_state_init: int,
        min_ok_state_invalid: int,
        min_ok_state_valid: int,
        window_size: int,
        window_size_init: Optional[int] = None,
        window_size_invalid: Optional[int] = None,
        window_size_valid: Optional[int] = None,
        profile_behavior: Optional[E2EProfileBehavior] = None,
        sync_counter_init: Optional[int] = None,
        data_id_mode: Optional[DataIdMode] = None,
        data_id_nibble_offset: Optional[int] = None,
        crc_offset: Optional[int] = None,
        counter_offset: Optional[int] = None,
    ) -> E2ETransformationTechnologyConfig: ...
    counter_offset: Optional[int]
    """Offset of the counter in the Data[] array in bits. Required for E2E profiles 01 and 11, unused otherwise"""
    crc_offset: Optional[int]
    """Offset of the crc in the Data[] array in bits. Required for E2E profiles 01 and 11, unused otherwise"""
    data_id_mode: Optional[DataIdMode]
    """The data ID mode to use; required for E2E profiles 01 and 11, unused otherwise"""
    data_id_nibble_offset: Optional[int]
    """Offset of the data ID in the Data[] array in bits. Required for E2E profiles 01 and 11 when `data_id_mode` is `Lower12Bit`, unused otherwise"""
    max_delta_counter: int
    """Maximum jump in the counter value between two consecutive messages"""
    max_error_state_init: int
    """The maximum allowed number of consecutive failed counter checks in the init state"""
    max_error_state_invalid: int
    """The maximum allowed number of consecutive failed counter checks in the invalid state"""
    max_error_state_valid: int
    """The maximum allowed number of consecutive failed counter checks in the valid state"""
    max_no_new_or_repeated_data: int
    """The maximum allowed number of consecutive failed counter checks"""
    min_ok_state_init: int
    """The minimum allowed number of consecutive successful counter checks in the init state"""
    min_ok_state_invalid: int
    """The minimum allowed number of consecutive successful counter checks in the invalid state"""
    min_ok_state_valid: int
    """The minimum allowed number of consecutive successful counter checks in the valid state"""
    offset: int
    """The offset in bits from the start of the buffer where the E2E data should be placed
    If E2E is used after COM, the offset should be 0; if E2E is used after SOMEIP, the offset should be 64"""
    profile: E2EProfile
    """E2E profile to use"""
    profile_behavior: Optional[E2EProfileBehavior]
    """Behavior of the check functionality"""
    sync_counter_init: Optional[int]
    """Number of successful checks required for validating the consistency of the counter"""
    transform_in_place: bool
    """Should the E2E transformation take place in the existing buffer or in a separate buffer?"""
    window_size: int
    """window size: Size of the monitoring window for the E2E state machine.
    This can be directly set up to AUTOSAR 4.4.0 (`AUTOSAR_00047`).
    For newer files this only provides the default if `window_size_init`, `window_size_invalid` and `window_size_valid` are not set"""
    window_size_init: Optional[int]
    """window size in the init state - only valid in AUTOSAR 4.5.0 (`AUTOSAR_00048`) and newer. if it is not set, this will default to `window_size`"""
    window_size_invalid: Optional[int]
    """window size in the invalid state - only valid in AUTOSAR 4.5.0 (`AUTOSAR_00048`) and newer. if it is not set, this will default to `window_size`"""
    window_size_valid: Optional[int]
    """window size in the valid state - only valid in AUTOSAR 4.5.0 (`AUTOSAR_00048`) and newer. if it is not set, this will default to `window_size`"""
    zero_header_length: bool
    """When E2E is used in a transformer chain after COM, the header length must be zero.
    In this configuration you are expected to provide space for the E2E data inside the signal group layout, and `zero_header_length` should be set to true.
    If `zero_header_length` is set to false, the appropriate header length for the chosen E2E profile will be used (e.g. 24 bits for `PROFILE_05`)"""

@final
class EndToEndTransformationISignalProps:
    """
    Properties for the End to End transformation of an ISignal(Group)
    """

    def __init__(self, element: Element) -> EndToEndTransformationISignalProps: ...
    data_ids: List[int]
    """get or set the data IDs that are used for the E2E transformation"""
    data_length: Optional[int]
    """get or set the length of payload and E2E header in bits"""
    element: Element
    max_data_length: Optional[int]
    """get or set the maximum data length"""
    min_data_length: Optional[int]
    """get or set the minimum data length"""
    source_id: Optional[int]
    """get or set the source ID"""
    transformer: Optional[TransformationTechnology]
    """get or set the transformer reference of the E2E transformation properties"""

@final
class EthernetCluster:
    """
    An `EthernetCluster` contains all configuration items associated with an ethernet network.
    The cluster connects multiple ECUs.
    """

    def __init__(self, element: Element) -> EthernetCluster: ...
    def create_physical_channel(
        self, channel_name: str, /, *, vlan_info: Optional[EthernetVlanInfo] = None
    ) -> EthernetPhysicalChannel:
        """Create a new physical channel for the cluster

        The supplied VLAN info must be unique - there cannot be two VLANs with the same vlan identifier.
        One channel may be created without VLAN information; it carries untagged traffic.
        """
        ...
    element: Element
    name: str
    def physical_channels(self, /) -> Iterator[EthernetPhysicalChannel]:
        """returns an iterator over all [`EthernetPhysicalChannel`]s in the cluster"""
        ...
    system: Optional[System]
    """get the `System` that this `EthernetCluster` is part of"""

@final
class EthernetCommunicationConnector:
    """
    A connector between an [`EthernetCommunicationController`] in an ECU and an [`EthernetPhysicalChannel`]
    """

    def __init__(self, element: Element) -> EthernetCommunicationConnector: ...
    controller: EthernetCommunicationController
    """Get the controller of the `CommunicationConnector`"""
    ecu_instance: EcuInstance
    """Get the `EcuInstance` that contains this `CommunicationConnector`"""
    element: Element
    name: str

@final
class EthernetCommunicationController:
    """
    An `EcuInstance` needs an `EthernetCommunicationController` in order to connect to an ethernet cluster.
    """

    def __init__(self, element: Element) -> EthernetCommunicationController: ...
    def connect_physical_channel(
        self, connection_name: str, eth_channel: EthernetPhysicalChannel, /
    ) -> EthernetCommunicationConnector:
        """Connect this [`EthernetCommunicationController`] inside an [`EcuInstance`] to an [`EthernetPhysicalChannel`] in the [`crate::System`]

        Creates an `EthernetCommunicationConnector` in the [`EcuInstance`] that contains this [`EthernetCommunicationController`].

        This function establishes the relationships:
         - [`EthernetPhysicalChannel`] -> `EthernetCommunicationConnector`
         - `EthernetCommunicationConnector` -> [`EthernetCommunicationController`]"""
        ...

    def connected_channels(self, /) -> Iterator[EthernetPhysicalChannel]:
        """return an iterator over the [`EthernetPhysicalChannel`]s connected to this controller"""
        ...
    ecu_instance: EcuInstance
    """Get the `EcuInstance` that contains this `EthernetCommunicationController`"""
    element: Element
    name: str

@final
class EthernetPhysicalChannel:
    """
    The `EthernetPhysicalChannel` represents a VLAN or untagged traffic
    """

    def __init__(self, element: Element) -> EthernetPhysicalChannel: ...
    cluster: EthernetCluster
    """get the cluster containing this physical channel"""
    def configure_service_discovery_for_ecu(
        self,
        ecu: EcuInstance,
        unicast_socket: SocketAddress,
        unicast_rx_pdu: GeneralPurposePdu,
        unicast_tx_pdu: GeneralPurposePdu,
        common_config: CommonServiceDiscoveryConfig,
        /,
    ) -> None:
        """configure SOME/IP service discovery (SD) for an ECU connected to this channel

        SD is used to broadcast service offers on the network and subscribe to services offered by other ECUs.
        This function configures the ECU to use the SOME/IP SD protocol.

        SD uses either socket connection bundles or static socket connections to communicate.

        `ecu` is the ECU that should be configured for SD.
        `unicast_socket` is the socket address used for unicast rx/tx communication by the ECU.
        `unicast_rx_pdu` and `unicast_tx_pdu` are the `GeneralPurposePdus` used for the unicast communication.
        `common_config` contains common configuration settings that can be used for all SD ECUs.
         - `multicast_rx_socket` is the socket address used for multicast communication by all SD ECUs.
         - `remote_socket` is a socket whose IP is set to ANY with UDP port 0, acting as the remote address in the SD communication.
         - `name_prefix` is an optional prefix for the names of the created elements.
         - `prefer_static_socket_connections` is a flag that determines if `SocketConnectionBundles` should be used instead of `StaticSocketConnections`.
            This is only relevant if the type can't be detected automatically.
         - `ipdu_identifier_set` is contains the `IPduIdentifiers` that are used in `StaticSocketConnections`.

        Note:
        Usually `SomeIP` SD is expected to use port 30490, but this is not mandatory.
        The port number is set in the sockets, and must be the same for all SD sockets.
        """
        ...

    def create_network_endpoint(
        self,
        name: str,
        address: NetworkEndpointAddress,
        /,
        *,
        ecu: Optional[EcuInstance] = None,
    ) -> NetworkEndpoint:
        """create a network endpoint - IPv4 or IPv6 address - for this channel

        In older versions of the Autosar standard, up to version 4.4.0, the `NetworkEndpoint` could be linked to an Ecu.
        The parameter `ecu` specifies the target.
        The link is obsoleted in newer versions, and will only be created if the file version allows it.
        """
        ...

    def create_socket_address(
        self,
        name: str,
        network_endpoint: NetworkEndpoint,
        tp_config: TpConfig,
        sa_type: SocketAddressType,
        /,
    ) -> SocketAddress:
        """create a socket address in the ethernet channel

        It contains the settings of the TCP/UDP port and links to a [`NetworkEndpoint`] which contains the IP address.
        The socket address can either be a unicast adress which is associated with a single ECU, or a multicast address
        """
        ...

    def create_socket_connection_bundle(
        self, name: str, server_port: SocketAddress, /
    ) -> SocketConnectionBundle:
        """create a socket connection bundle

        The `SocketConnectionBundle` is the "old" way to establish a connection between two sockets.
        It is deprecated in newer versions of the Autosar standard, but remains available for compatibility.
        """
        ...

    def create_static_socket_connection_pair(
        self,
        name: str,
        port_1: SocketAddress,
        port_2: SocketAddress,
        /,
        *,
        tcp_connect_timeout: Optional[float] = None,
    ) -> Tuple[StaticSocketConnection, StaticSocketConnection]:
        """create a pair of static socket connections

        Static socket connections are usually created as a pair, one on each socket involved on the connection.
        This helper function creates both at once. To create a single connection, use [`SocketAddress::create_static_socket_connection`].

        If the connection is a TCP connection, the first port connects to the second port, and the second port listens for incoming connection.
        The ordering of `port_1` and `port_2` has no impact on the direction of the transported PDUs. This is defined in the `PduTriggering`.

        `StaticSocketConnection`s are the "new" way to establish a connection between two sockets.
        It was introduced in Autosar 4.5.0 (`AUTOSAR_00048`) and is the recommended way to create connections.

        `SocketConnectionBundles` (old) and `StaticSocketConnections` (new) may never be used in the same file.
        """
        ...
    element: Element
    def has_socket_connections(self, /) -> bool:
        """check if the channel contains any `SocketConnectionBundles` (old) or `SocketConnections` (very old)"""
        ...
    name: str
    def signal_triggerings(self, /) -> Iterator[ISignalTriggering]:
        """iterate over all signal triggerings of this physical channel"""
        ...

    def network_endpoints(self, /) -> Iterator[NetworkEndpoint]:
        """create an iterator over all [`NetworkEndpoint`]s in this channel"""
        ...

    def pdu_triggerings(self, /) -> Iterator[PduTriggering]:
        """iterate over all PDU triggerings of this physical channel"""
        ...

    def socket_addresses(self, /) -> Iterator[SocketAddress]:
        """create an iterator over all [`SocketAddress`]es in this channel"""
        ...

    def socket_connection_bundles(self, /) -> Iterator[SocketConnectionBundle]:
        """iterate over all socket connection bundles in this channel

        The `SocketConnectionBundle` is the "old" way to establish a connection between two sockets.
        It is deprecated in newer versions of the Autosar standard, but remains available for compatibility.
        """
        ...

    def vlan_info(self) -> Optional[EthernetVlanInfo]:
        """get or set the VLAN information of the channel.
        In an EthernetCluster, each physical channel must have unique VLAN settings; only one channel can omit VLAN information - it carries untagged traffic.
        Setting duplicate VLAN information will result in an error."""
        ...

    def set_vlan_info(self, vlan_info: Optional[EthernetVlanInfo], /) -> None:
        """get or set the VLAN information of the channel.

        In an EthernetCluster, each physical channel must have unique VLAN settings; only one channel can omit VLAN information - it carries untagged traffic.
        Setting duplicate VLAN information will result in an error."""
        ...

@final
class EthernetVlanInfo:
    """
    Provides information about the VLAN of an [`EthernetPhysicalChannel`]
    """

    def __init__(self, *, vlan_name: str, vlan_id: int) -> EthernetVlanInfo: ...
    vlan_id: int
    vlan_name: str

@final
class EventControlledTiming:
    """
    Event controlled timing parameters for an IPDU
    """

    def __init__(
        self,
        number_of_repetitions: int,
        /,
        *,
        repetition_period: Optional[float] = None,
    ) -> EventControlledTiming: ...
    number_of_repetitions: int
    """The PDU will be sent (number of repetitions + 1) times. If number of repetitions is 0, then the PDU is sent exactly once."""
    repetition_period: Optional[float]
    """time in seconds between two transmissions of the PDU"""

@final
class EventGroupControlType:
    """
    control types used in routing groups for SOME/IP events
    """

    ActivationAndTriggerUnicast: EventGroupControlType
    ActivationMulticast: EventGroupControlType
    ActivationUnicast: EventGroupControlType
    TriggerUnicast: EventGroupControlType

@final
class EventHandler:
    """
    An `EventHandler` describes the handling of a single event in a `ProvidedServiceInstance`
    """

    def __init__(self, element: Element) -> EventHandler: ...
    def create_pdu_activation_routing_group(
        self, name: str, event_group_control_type: EventGroupControlType, /
    ) -> PduActivationRoutingGroup:
        """create a new `PduActivationRoutingGroup` in this `EventHandler`"""
        ...
    element: Element
    event_group_identifier: Optional[int]
    """get or set the event group identifier of this `EventHandler`"""
    name: str
    def pdu_activation_routing_groups(self, /) -> Iterator[PduActivationRoutingGroup]:
        """get the `PduActivationRoutingGroup`s in this `EventHandler`"""
        ...
    sd_server_event_group_timing_config: Optional[SomeipSdServerEventGroupTimingConfig]
    """get or set the SD server event group timing configuration for this `EventHandler`"""

@final
class EventHandlerV1:
    """
    An `EventHandlerV1` is a SD event handler that is used to receive events from other ECUs.

    This is the old V1 version of the service definition.
    """

    def __init__(self, element: Element) -> EventHandlerV1: ...
    def add_consumed_event_group(
        self, consumed_event_group: ConsumedEventGroupV1, /
    ) -> None:
        """add a reference to a `ConsumedEventGroupV1` to this `EventHandlerV1`"""
        ...

    def add_routing_group(self, routing_group: SoAdRoutingGroup, /) -> None:
        """add a reference to a `SoAdRoutingGroup` to this `EventHandler`"""
        ...

    def consumed_event_groups(self, /) -> Iterator[ConsumedEventGroupV1]:
        """get the consumed event groups referenced by this `EventHandler`"""
        ...
    element: Element
    name: str
    def routing_groups(self, /) -> Iterator[SoAdRoutingGroup]:
        """get the routing groups referenced by this `EventHandler`"""
        ...

    def sd_server_config(self) -> Optional[SdEventConfig]:
        """get the SD server event configuration for this `EventHandlerV1`"""
        ...

    def set_sd_server_config(self, sd_event_config: SdEventConfig) -> None:
        """set the SD server event configuration for this `EventHandlerV1`"""
        ...

@final
class FlexrayArTpChannel:
    """
    The `FlexrayArTpChannel` represents a channel in the Flexray Autosar Transport Protocol
    """

    def __init__(self, element: Element) -> FlexrayArTpChannel: ...
    ack_type: Optional[FrArTpAckType]
    """get or set the ack type of the channel"""
    def add_n_pdu(self, n_pdu: NPdu, /) -> None:
        """add an N-PDU to the channel

        The `NPdus` are logically assembled into a pool of Rx `NPdus` and another pool of Tx `NPdus`.
        This function is supported on autosar 4.1 and later, while Autosar 4.0 uses a different approach.
        """
        ...

    def create_flexray_ar_tp_connection(
        self,
        name: Optional[str],
        direct_tp_sdu: IPdu,
        source: FlexrayArTpNode,
        target: FlexrayArTpNode,
        /,
    ) -> FlexrayArTpConnection:
        """create a new `FlexrayArTpConnection` for this channel"""
        ...

    def flexray_ar_tp_connections(self, /) -> Iterator[FlexrayArTpConnection]:
        """get an iterator over the connections in the channel"""
        ...
    element: Element
    extended_addressing: Optional[bool]
    """get or set the extended addressing attribute"""
    maximum_message_length: Optional[MaximumMessageLengthType]
    """get or set the maximum message length type"""
    minimum_separation_time: Optional[float]
    """get or set the minimum separation time"""
    multicast_segmentation: Optional[bool]
    """get or set the multicast segmentation"""
    def n_pdus(self, /) -> Iterator[NPdu]:
        """iterate over the `NPdus` of the channel"""
        ...

@final
class FlexrayArTpConfig:
    """
    The `FlexrayArTpConfig` represents the configuration of the Flexray Autosar Transport Protocol
    """

    def __init__(self, element: Element) -> FlexrayArTpConfig: ...
    cluster: Optional[FlexrayCluster]
    """get or set the Flexray cluster for the configuration"""
    def create_flexray_ar_tp_channel(
        self,
        ack_type: FrArTpAckType,
        extended_addressing: bool,
        maximum_message_length: MaximumMessageLengthType,
        minimum_separation_time: float,
        multicast_segmentation: bool,
        /,
    ) -> FlexrayArTpChannel:
        """create a new `FlexrayArTpChannel`"""
        ...

    def create_flexray_ar_tp_node(self, name: str, /) -> FlexrayArTpNode:
        """create a new `FlexrayArTpNode`"""
        ...

    def create_tp_address(self, name: str, address: int, /) -> TpAddress:
        """create a new `TpAddress`"""
        ...
    element: Element
    def flexray_ar_tp_channels(self, /) -> Iterator[FlexrayArTpChannel]:
        """get an iterator over the channels in the configuration"""
        ...

    def flexray_ar_tp_nodes(self, /) -> Iterator[FlexrayArTpNode]:
        """get an iterator over the nodes"""
        ...
    name: str
    def tp_addresses(self, /) -> Iterator[TpAddress]:
        """iterate over all `TpAddresses`"""
        ...

@final
class FlexrayArTpConnection:
    """
    `FlexrayArTpConnection` represents a connection within a `FlexrayArTpChannel`

    The connection identifies the sender and the receiver of this particular communication.
    The Flexray Autosar Tp module routes a Pdu through this connection.
    """

    def __init__(self, element: Element) -> FlexrayArTpConnection: ...
    def add_target(self, target: FlexrayArTpNode, /) -> None:
        """add a target to the connection

        The connection can have multiple targets, but at least one target is required.
        """
        ...
    direct_tp_sdu: Optional[IPdu]
    """get or set the direct TP SDU"""
    element: Element
    name: str
    reversed_tp_sdu: Optional[IPdu]
    """get or set or remove the reversed TP SDU
    
    If the connection supports both directions, then the reversed TP SDU is required.
    if Some(value) is passed, the reversed TP SDU is set to the given value, otherwise it is removed."""
    source: Optional[FlexrayArTpNode]
    """get or set the source of the connection"""
    def targets(self, /) -> Iterator[FlexrayArTpNode]:
        """get the targets"""
        ...

@final
class FlexrayArTpNode:
    """
    `FlexrayArTpNode` represents a node in the Flexray Autosar Transport Protocol

    A TP node (sender or receiver) provides the TP address and the connection to the topology description
    """

    def __init__(self, element: Element) -> FlexrayArTpNode: ...
    def add_communication_connector(
        self, connector: FlexrayCommunicationConnector, /
    ) -> None:
        """add a reference to a `FlexrayCommunicationConnector`

        The connectors define the association with a `PhysicalChannel` and an ECU.
        In a `SystemDescription`, this reference is mandatory, but in an `ECUExtract` it is optional.
        Up to 2 connectors can be added to a node."""
        ...

    def communication_connectors(self, /) -> Iterator[FlexrayCommunicationConnector]:
        """get the connectors"""
        ...
    element: Element
    name: str
    tp_address: Optional[TpAddress]
    """set or remove the TP address
    
    if Some(value) is passed, the TP address is set to the given value, otherwise it is removed."""

@final
class FlexrayChannelName:
    """
    A flexray cluster may contain the channels A and/or B.

    This enum is an abstraction over the <CHANNEL-NAME> element.
    """

    A: FlexrayChannelName
    B: FlexrayChannelName

@final
class FlexrayCluster:
    """
    A `FlexrayCluster` contains all configuration items associated with a Flexray network.
    The cluster connects multiple ECUs.
    """

    def __init__(self, element: Element) -> FlexrayCluster: ...
    def create_physical_channel(
        self, name: str, channel_name: FlexrayChannelName, /
    ) -> FlexrayPhysicalChannel:
        """Create a new physical channel for the cluster

        A cluster may contain channel A, channel B, or both A and B."""
        ...
    element: Element
    name: str
    physical_channels: FlexrayPhysicalChannelsInfo
    """get the physical channels of this cluster"""
    def settings(self) -> Optional[FlexrayClusterSettings]:
        """get the current flexray cluster settings"""
        ...

    def set_settings(self, settings: FlexrayClusterSettings, /) -> None:
        """set the current flexray cluster settings"""
        ...
    system: Optional[System]
    """get the system that contains this cluster"""

@final
class FlexrayClusterSettings:
    action_point_offset: float
    """get or set the action point offset of the cluster"""
    baudrate: int
    """get or set the baudrate of the cluster"""
    bit: float
    """get or set the bit time of the cluster"""
    cas_rx_low_max: int
    """get or set the cas rx low max of the cluster"""
    cold_start_attempts: int
    """get or set the cold start attempts of the cluster"""
    cycle: float
    """get or set the cycle time of the cluster (in seconds)"""
    cycle_count_max: int
    """get or set the cycle count max of the cluster"""
    detect_nit_error: bool
    """get or set the detect nit error status of the cluster"""
    dynamic_slot_idle_phase: int
    """get or set the dynamic slot idle phase of the cluster"""
    ignore_after_tx: int
    """get or set the ignore after tx duration of the cluster"""
    listen_noise: int
    """get or set the listen noise of the cluster"""
    macro_per_cycle: int
    """get or set the macro per cycle of the cluster"""
    macrotick_duration: float
    """get or set the macrotick duration of the cluster"""
    max_without_clock_correction_fatal: int
    """get or set the max without clock correction fatal of the cluster"""
    max_without_clock_correction_passive: int
    """get or set the max without clock correction passive of the cluster"""
    minislot_action_point_offset: int
    """get or set the minislot action point offset of the cluster"""
    minislot_duration: int
    """get or set the minislot duration of the cluster"""
    network_idle_time: int
    """get or set the network idle time of the cluster"""
    network_management_vector_length: int
    """get or set the network management vector length of the cluster"""
    number_of_minislots: int
    """get or set the number of minislots of the cluster"""
    number_of_static_slots: int
    """get or set the number of static slots of the cluster"""
    offset_correction_start: int
    """get or set the offset correction start of the cluster"""
    payload_length_static: int
    """get or set the payload length static of the cluster"""
    safety_margin: int
    """get or set the safety margin of the cluster"""
    sample_clock_period: Optional[float]
    """get or set the sample clock period of the cluster"""
    static_slot_duration: int
    """get or set the static slot duration of the cluster"""
    symbol_window: int
    """get or set the symbol window of the cluster"""
    symbol_window_action_point_offset: int
    """get or set the symbol window action point offset of the cluster"""
    sync_frame_id_count_max: int
    """get or set the sync frame id count max of the cluster"""
    transceiver_standby_delay: Optional[float]
    """get or set the transceiver standby delay of the cluster"""
    transmission_start_sequence_duration: int
    """get or set the transmission start sequence duration of the cluster"""
    def verify(self, /) -> bool:
        """verify the settings of a flexray cluster"""
        ...
    wakeup_rx_idle: int
    """get or set the wakeup rx idle of the cluster"""
    wakeup_rx_low: int
    """get or set the wakeup rx low of the cluster"""
    wakeup_rx_window: int
    """get or set the wakeup rx window of the cluster"""
    wakeup_tx_active: int
    """get or set the wakeup tx active of the cluster"""
    wakeup_tx_idle: int
    """get or set the wakeup tx idle of the cluster"""

@final
class FlexrayCommunicationConnector:
    """
    A connector between a [`FlexrayCommunicationController`] in an ECU and a [`FlexrayPhysicalChannel`]
    """

    def __init__(self, element: Element) -> FlexrayCommunicationConnector: ...
    controller: FlexrayCommunicationController
    """Get or set the controller of the `CommunicationConnector`"""
    ecu_instance: EcuInstance
    """Get the `EcuInstance` that contains this `CommunicationConnector`"""
    element: Element
    name: str

@final
class FlexrayCommunicationController:
    """
    An `EcuInstance` needs a `FlexrayCommunicationController` in order to connect to a Flexray cluster.
    """

    def __init__(self, element: Element) -> FlexrayCommunicationController: ...
    def connect_physical_channel(
        self, connection_name: str, flx_channel: FlexrayPhysicalChannel
    ) -> FlexrayCommunicationConnector:
        """Connect this [`FlexrayCommunicationController`] inside an [`EcuInstance`] to a [`FlexrayPhysicalChannel`] in the [`crate::System`]

        Creates a `FlexrayCommunicationConnector` in the [`EcuInstance`] that contains this [`FlexrayCommunicationController`].

        This function establishes the relationships:
         - [`FlexrayPhysicalChannel`] -> `FlexrayCommunicationConnector`
         - `FlexrayCommunicationConnector` -> [`FlexrayCommunicationController`]"""
        ...

    def connected_channels(self, /) -> Iterator[FlexrayPhysicalChannel]:
        """return an iterator over the [`FlexrayPhysicalChannel`]s connected to this controller"""
        ...
    element: Element
    ecu_instance: EcuInstance
    """Get the `EcuInstance` that contains this `FlexrayCommunicationController`"""
    name: str

@final
class FlexrayCommunicationCycle:
    """
    The timing settings of a Flexray frame
    """

    Counter: Type[FlexrayCommunicationCycle_Counter]
    Repetition: Type[FlexrayCommunicationCycle_Repetition]

@final
class FlexrayCommunicationCycle_Counter(FlexrayCommunicationCycle):
    def __init__(self, cycle_counter: int) -> FlexrayCommunicationCycle_Counter: ...
    cycle_counter: int

@final
class FlexrayCommunicationCycle_Repetition(FlexrayCommunicationCycle):
    def __init__(
        self, base_cycle: int, cycle_repetition: CycleRepetition
    ) -> FlexrayCommunicationCycle_Repetition: ...
    base_cycle: int
    cycle_repetition: CycleRepetition

@final
class FlexrayFrame:
    """
    a Flexray frame
    """

    def __init__(self, element: Element) -> FlexrayFrame: ...
    element: Element
    def frame_triggerings(self, /) -> List[FlexrayFrameTriggering]:
        """List all `FlexrayFrameTriggering`s using this frame"""
        ...
    length: Optional[int]
    """get or set the length of the frame"""
    def map_pdu(
        self,
        pdu: Pdu,
        start_position: int,
        byte_order: ByteOrder,
        /,
        *,
        update_bit: Optional[int] = None,
    ) -> FlexrayFrameTriggering:
        """map a PDU to the frame"""
        ...

    def mapped_pdus(self, /) -> Iterator[Pdu]:
        """returns an iterator over all PDUs in the frame"""
        ...
    name: str

@final
class FlexrayFrameTriggering:
    """
    Iterator over all [`FlexrayFrameTriggering`]s using this frame
    map a PDU to the frame
    The frame triggering connects a frame to a physical channel
    """

    def __init__(self, element: Element) -> FlexrayFrameTriggering: ...
    def connect_to_ecu(
        self, ecu: EcuInstance, direction: CommunicationDirection, /
    ) -> FramePort:
        """connect this frame triggering to an ECU

        The frame triggering may be connected to any number of ECUs."""
        ...
    element: Element
    frame: Optional[FlexrayFrame]
    """get the frame triggered by the frame triggering"""
    def frame_ports(self, /) -> Iterator[FramePort]:
        """get the frame ports connected to this frame triggering"""
        ...
    name: str
    def pdu_triggerings(self, /) -> Iterator[PduTriggering]:
        """get the PDU triggerings referenced by this frame triggering"""
        ...
    physical_channel: FlexrayPhysicalChannel
    """get the physical channel that contains this frame triggering"""
    slot: Optional[int]
    """get or set the slot id for the flexray frame triggering"""
    def timing(self) -> Optional[FlexrayCommunicationCycle]:
        """get the timing of the flexray frame

        In a well-formed file this should always return a value"""
        ...

    def set_timing(self, timing: FlexrayCommunicationCycle, /) -> None:
        """set the timing of the flexray frame"""
        ...

@final
class FlexrayNmCluster:
    """
    Flexray specific `NmCluster`
    """

    def __init__(self, element: Element) -> FlexrayNmCluster: ...
    channel_sleep_master: Optional[bool]
    """get or set or remove the nmChannelSleepMaster flag"""
    communication_cluster: Optional[FlexrayCluster]
    """get or set the referenced `FlexrayCluster`"""
    def create_flexray_nm_node(
        self, name: str, controller: FlexrayCommunicationController, nm_ecu: NmEcu, /
    ) -> FlexrayNmNode:
        """add a `FlexrayNmNode` to the cluster"""
        ...
    element: Element
    name: str
    nm_data_cycle: Optional[int]
    """get or set the nmDataCycle
    
    Number of Flexray Communication Cycles needed to transmit the Nm Data PDUs of all Flexray Nm Ecus of this `FlexrayNmCluster`."""
    def nm_nodes(self, /) -> Iterator[FlexrayNmNode]:
        """iterate over all `NmNodes` in this cluster"""
        ...
    nm_remote_sleep_indication_time: Optional[float]
    """get or set the nmRemoteSleepIndicationTime
    
    Timeout for Remote Sleep Indication in seconds."""
    nm_repeat_message_time: Optional[float]
    """get or set the nmRepeatMessageTime
    
    Timeout for Repeat Message State in seconds."""
    nm_repetition_cycle: Optional[int]
    """get or set the nmRepetitionCycle
    
    Number of Flexray Communication Cycles used to repeat the transmission of the Nm vote Pdus of all
    Flexray `NmEcus` of this `FlexrayNmCluster`. This value shall be an integral multiple of nmVotingCycle."""
    nm_voting_cycle: Optional[int]
    """get or set the nmVotingCycle
    
    The number of Flexray Communication Cycles used to transmit the Nm Vote PDUs of all Flexray Nm Ecus of this `FlexrayNmCluster`."""
    node_detection_enabled: Optional[bool]
    """get or set the nmNodeDetectionEnabled flag"""
    node_id_enabled: Optional[bool]
    """get or set the nmNodeIdEnabled flag"""
    pnc_cluster_vector_length: Optional[int]
    """get or set the pncClusterVectorLength"""
    pnc_participation: Optional[bool]
    """get or set the nmPncParticipation flag"""
    repeat_msg_ind_enabled: Optional[bool]
    """get or set the nmRepeatMsgIndEnabled flag"""
    synchronizing_network: Optional[bool]
    """get or set the nmSynchronizingNetwork flag"""

@final
class FlexrayNmClusterCoupling:
    """
    A `FlexrayNmClusterCoupling` `couples multiple `FlexrayNmCluster`s.
    """

    def __init__(self, element: Element) -> FlexrayNmClusterCoupling: ...
    def add_coupled_cluster(self, cluster: FlexrayNmCluster, /) -> None:
        """add a reference to a coupled `NmCluster`"""
        ...

    def coupled_clusters(self, /) -> Iterator[FlexrayNmCluster]:
        """iterate over all coupled `NmClusters`"""
        ...
    element: Element
    nm_schedule_variant: Optional[FlexrayNmScheduleVariant]
    """get or set the nmScheduleVariant"""

@final
class FlexrayNmClusterSettings:
    """
    Mandatory settings for a `FlexrayNmCluster`

    These settings must be provided when creating a new `FlexrayNmCluster`.
    Additional optional settings can be set using `FlexrayNmCluster` methods.
    """

    def __init__(
        self,
        *,
        nm_data_cycle: int,
        nm_remote_sleep_indication_time: float,
        nm_repeat_message_time: float,
        nm_repetition_cycle: int,
        nm_voting_cycle: int,
    ) -> FlexrayNmClusterSettings: ...
    nm_data_cycle: Optional[int]
    """nmDataCycle: Number of Flexray Communication Cycles needed to transmit the Nm Data PDUs of all Flexray Nm Ecus of this `FlexrayNmCluster`."""
    nm_remote_sleep_indication_time: Optional[float]
    """nmRemoteSleepIndicationTime: Timeout for Remote Sleep Indication in seconds."""
    nm_repeat_message_time: Optional[float]
    """nmRepeatMessageTime: Timeout for Repeat Message State in seconds."""
    nm_repetition_cycle: Optional[int]
    """nmRepetitionCycle: Number of Flexray Communication Cycles used to repeat the transmission of the Nm vote Pdus of all
    Flexray `NmEcus` of this `FlexrayNmCluster`. This value shall be an integral multiple of nmVotingCycle."""
    nm_voting_cycle: Optional[int]
    """nmVotingCycle: The number of Flexray Communication Cycles used to transmit the Nm Vote PDUs of all Flexray Nm Ecus of this `FlexrayNmCluster`."""

@final
class FlexrayNmNode:
    """
    A `FlexrayNmNode` represents a Flexray specific `NmNode`.

    It connects a `FlexrayCommunicationController` with a `NmEcu`.
    """

    def __init__(self, element: Element) -> FlexrayNmNode: ...
    def add_rx_nm_pdu(self, nm_pdu: NmPdu, /) -> None:
        """add an Rx `NmPdu`

        Every `NmNode` must have at least one Rx `NmPdu`"""
        ...

    def add_tx_nm_pdu(self, nm_pdu: NmPdu, /) -> None:
        """add a Tx `NmPdu`

        Active `NmNodes` must have at least one Tx `NmPdu`, while passive `NmNodes` may have none.
        """
        ...
    communication_controller: Optional[FlexrayCommunicationController]
    """get or set the referenced `FlexrayCommunicationController`"""
    element: Element
    name: str
    nm_ecu: Optional[NmEcu]
    """get or set the referenced `NmEcu`"""
    node_id: Optional[int]
    """get or set the nmNodeId
    This value is optional; if it is set to Some(x) the value is created, if it is set to None the value is removed."""
    passive_mode: Optional[bool]
    """get or set ot remove the nmPassiveModeEnabled flag
    
    This flag is optional; if it is set to Some(x) the value is created, if it is set to None the value is removed."""
    def rx_nm_pdus(self, /) -> Iterator[NmPdu]:
        """iterate over all RX `NmPdus`"""
        ...

    def tx_nm_pdus(self, /) -> Iterator[NmPdu]:
        """iterate over all TX `NmPdus`"""
        ...

@final
class FlexrayNmScheduleVariant:
    """
    The `FlexrayNmScheduleVariant` defines the way the NM-Vote and NM-Data are transmitted within the Flexray network.
    """

    ScheduleVariant1: FlexrayNmScheduleVariant
    ScheduleVariant2: FlexrayNmScheduleVariant
    ScheduleVariant3: FlexrayNmScheduleVariant
    ScheduleVariant4: FlexrayNmScheduleVariant
    ScheduleVariant5: FlexrayNmScheduleVariant
    ScheduleVariant6: FlexrayNmScheduleVariant
    ScheduleVariant7: FlexrayNmScheduleVariant

@final
class FlexrayPhysicalChannel:
    """
    the `FlexrayPhysicalChannel` represents either channel A or B of Flexray cluster
    """

    def __init__(self, element: Element) -> FlexrayPhysicalChannel: ...
    channel_name: Optional[FlexrayChannelName]
    """get the channel name of a `FlexrayPhysicalChannel`"""
    cluster: FlexrayCluster
    """get the cluster containing this physical channel"""
    element: Element
    def frame_triggerings(self, /) -> Iterator[FlexrayFrameTriggering]:
        """iterate over all frame triggerings of this physical channel"""
        ...

    def signal_triggerings(self, /) -> Iterator[ISignalTriggering]:
        """iterate over all signal triggerings of this physical channel"""
        ...

    def pdu_triggerings(self, /) -> Iterator[PduTriggering]:
        """iterate over all PDU triggerings of this physical channel"""
        ...
    name: str
    def trigger_frame(
        self, frame: FlexrayFrame, slot_id: int, timing: FlexrayCommunicationCycle, /
    ) -> FlexrayFrameTriggering:
        """add a trigger for a flexray frame in this physical channel"""
        ...

@final
class FlexrayPhysicalChannelsInfo:
    """
    Information about the flexray physical channels present inside a cluster
    """

    channel_a: Optional[FlexrayPhysicalChannel]
    """get the channel A of the cluster"""
    channel_b: Optional[FlexrayPhysicalChannel]
    """get the channel B of the cluster"""

@final
class FlexrayTpConfig:
    """
    `FlexrayTpConfig` defines exactly one Flexray ISO TP Configuration
    """

    def __init__(self, element: Element) -> FlexrayTpConfig: ...
    cluster: Optional[FlexrayCluster]
    """get or set the `FlexrayCluster` of the `FlexrayTpConfig`"""
    def create_flexray_tp_connection(
        self,
        name: Optional[str],
        transmitter: FlexrayTpNode,
        direct_tp_sdu: IPdu,
        connection_control: FlexrayTpConnectionControl,
        /,
    ) -> FlexrayTpConnection:
        """create a new `FlexrayTpConnection`"""
        ...

    def create_flexray_tp_connection_control(
        self, name: str, /
    ) -> FlexrayTpConnectionControl:
        """create a new `FlexrayTpConnectionControl`"""
        ...

    def create_flexray_tp_ecu(
        self, ecu_instance: EcuInstance, full_duplex_enabled: bool, /
    ) -> FlexrayTpEcu:
        """add a `FlexrayTpEcu` to the `FlexrayTpConfig`"""
        ...

    def create_flexray_tp_node(self, name: str, /) -> FlexrayTpNode:
        """create a new `FlexrayTpNode`"""
        ...

    def create_flexray_tp_pdu_pool(self, name: str, /) -> FlexrayTpPduPool:
        """create a new `FlexrayTpPduPool`"""
        ...

    def create_tp_address(self, name: str, address: int, /) -> TpAddress:
        """create a new `TpAddress`"""
        ...
    element: Element
    def flexray_tp_connection_controls(self, /) -> Iterator[FlexrayTpConnectionControl]:
        """iterate over all `FlexrayTpConnectionControls`"""
        ...

    def flexray_tp_connections(self, /) -> Iterator[FlexrayTpConnection]:
        """iterate over all `FlexrayTpConnections`"""
        ...

    def flexray_tp_ecus(self, /) -> Iterator[FlexrayTpEcu]:
        """iterate over all `FlexrayTpEcus`"""
        ...

    def flexray_tp_nodes(self, /) -> Iterator[FlexrayTpNode]:
        """iterate over all `FlexrayTpNodes`"""
        ...

    def flexray_tp_pdu_pools(self, /) -> Iterator[FlexrayTpPduPool]:
        """iterate over all `FlexrayTpPduPools`"""
        ...
    name: str
    def tp_addresses(self, /) -> Iterator[TpAddress]:
        """iterate over all `TpAddresses`"""
        ...

@final
class FlexrayTpConnection:
    """
    A `FlexrayTpConnection` defines a connection between `FlexrayTpNodes`
    """

    def __init__(self, element: Element) -> FlexrayTpConnection: ...
    def add_receiver(self, receiver: FlexrayTpNode, /) -> None:
        """add a receiver to the connection"""
        ...
    connection_control: Optional[FlexrayTpConnectionControl]
    """get or set the connection control of the connection"""
    direct_tp_sdu: Optional[IPdu]
    """get or set the direct TP SDU of the connection"""
    element: Element
    multicast_address: Optional[TpAddress]
    """get or set the multicast `TpAddress` of the connection"""
    name: str
    def receivers(self, /) -> Iterator[FlexrayTpNode]:
        """iterate over all receivers of the connection"""
        ...
    reversed_tp_sdu: Optional[IPdu]
    """get or set the reversed TP SDU of the connection
    This is used if the connection supports both sending and receiving"""
    rx_pdu_pool: Optional[FlexrayTpPduPool]
    """get or set the RX `FlexrayTpPduPool` of the connection"""
    transmitter: Optional[FlexrayTpNode]
    """get or set the transmitter of the connection"""
    tx_pdu_pool: Optional[FlexrayTpPduPool]
    """get or set the TX `FlexrayTpPduPool` of the connection"""

@final
class FlexrayTpConnectionControl:
    """
    A `FlexrayTpConnectionControl` defines the connection control parameters for a `FlexrayTpConnection`
    """

    def __init__(self, element: Element) -> FlexrayTpConnectionControl: ...
    element: Element
    max_fc_wait: Optional[int]
    """get or set the maxFcWait value"""
    max_number_of_npdu_per_cycle: Optional[int]
    """get or set the maxNumberOfNpduPerCycle value"""
    max_retries: Optional[int]
    """get or set the maxRetries value"""
    name: str
    separation_cycle_exponent: Optional[int]
    """get or set the separationCycleExponent value"""

@final
class FlexrayTpEcu:
    """
    A `FlexrayTpEcu` represents an ECU within the `FlexrayTpConfig`
    """

    def __init__(self, element: Element) -> FlexrayTpEcu: ...
    cancellation: Optional[bool]
    """get or set the cancellation status of the `FlexrayTpEcu`"""
    cycle_time_main_function: Optional[float]
    """get or set the cycle time of the TP main function in seconds"""
    ecu_instance: Optional[EcuInstance]
    """get or set the ECU instance of the `FlexrayTpEcu`"""
    element: Element
    full_duplex_enabled: Optional[bool]
    """get or set the full duplex enabled flag of the `FlexrayTpEcu`"""

@final
class FlexrayTpNode:
    """
    A `FlexrayTpNode` provides the TP address and the connection to the topology description in a `FlexrayTpConfig`
    """

    def __init__(self, element: Element) -> FlexrayTpNode: ...
    def add_communication_connector(
        self, connector: FlexrayCommunicationConnector, /
    ) -> None:
        """add a `FlexrayCommunicationConnector` to the node
        The node can be associated with up to 2 connectors.
        In a system description this reference is mandatory."""
        ...

    def communication_connectors(self, /) -> Iterator[FlexrayCommunicationConnector]:
        """iterate over all `FlexrayCommunicationConnectors` of the node"""
        ...
    element: Element
    name: str
    tp_address: Optional[TpAddress]
    """set or remove `FlexrayTpAddress` of the node
    A TP address is mandatory for unicast nodes, but optional for multicast nodes
    Setting None will remove the element"""

@final
class FlexrayTpPduPool:
    """
    A `FlexrayTpPduPool` contains a set of `NPdus` that can be used for sending and receiving
    """

    def __init__(self, element: Element) -> FlexrayTpPduPool: ...
    def add_n_pdu(self, n_pdu: NPdu, /) -> None:
        """add an `NPdu` to the `PduPool`"""
        ...
    element: Element
    name: str
    def n_pdus(self, /) -> Iterator[NPdu]:
        """iterate over all referenced `NPdus`"""
        ...

@final
class FrArTpAckType:
    """
    Types of Acknowledgement that can be used in an `FlexrayArTpChannel`
    """

    AckWithRt: FrArTpAckType
    AckWithoutRt: FrArTpAckType
    NoAck: FrArTpAckType

@final
class FramePort:
    """
    The `FramePort` allows an ECU to send or receive a frame
    """

    def __init__(self, element: Element) -> FramePort: ...
    communication_direction: Optional[CommunicationDirection]
    """get or set the communication direction of the frame port"""
    ecu: EcuInstance
    """get the ECU instance that contains this frame port"""
    element: Element
    name: str

@final
class GeneralPurposeIPdu:
    """
    This element is used for AUTOSAR Pdus without attributes that are routed by the `PduR`
    """

    def __init__(self, element: Element) -> GeneralPurposeIPdu: ...
    category: Optional[GeneralPurposeIPduCategory]
    """get the category of this PDU"""
    contained_ipdu_props: Optional[ContainedIPduProps]
    """set the ContainedIPduProps for this `IPdu`
    This is only needed when the `IPdu` is contained in a `ContainerIPdu`"""
    element: Element
    length: Optional[int]
    """get or set the length of this PDU"""
    name: str
    def pdu_triggerings(self, /) -> List[PduTriggering]:
        """list all `PduTriggerings` that trigger this PDU"""
        ...

@final
class GeneralPurposeIPduCategory:
    """
    The category of a `GeneralPurposeIPdu`

    The Autosar standard defines the following categories:
    - XCP
    - SOMEIP_SEGMENTED_IPDU
    - DLT
    """

    Dlt: GeneralPurposeIPduCategory
    SomeipSegmentedIpdu: GeneralPurposeIPduCategory
    Xcp: GeneralPurposeIPduCategory

@final
class GeneralPurposePdu:
    """
    This element is used for AUTOSAR Pdus without additional attributes that are routed by a bus interface
    """

    def __init__(self, element: Element) -> GeneralPurposePdu: ...
    category: Optional[GeneralPurposePduCategory]
    """get or set the category of this PDU"""
    element: Element
    length: Optional[int]
    """get or set the length of this PDU"""
    name: str
    def pdu_triggerings(self, /) -> List[PduTriggering]:
        """list all `PduTriggerings` that trigger this PDU"""
        ...

@final
class GeneralPurposePduCategory:
    """
    The category of a `GeneralPurposePdu`

    The Autosar standard defines the following categories:
    - `SD`
    - `GLOBAL_TIME`
    - `DOIP`
    """

    DoIp: GeneralPurposePduCategory
    GlobalTime: GeneralPurposePduCategory
    Sd: GeneralPurposePduCategory

@final
class GenericTransformationTechnologyConfig:
    """
    Configuration for a generic transformation technology
    For a generic trasformation, the mandatory values must be chosen by the user
    """

    def __init__(
        self,
        *,
        protocol_name: str,
        protocol_version: str,
        header_length: int,
        in_place: bool,
    ) -> GenericTransformationTechnologyConfig: ...
    header_length: int
    """The length of the header in bits"""
    in_place: bool
    """Should the transformation take place in the existing buffer or in a separate buffer?"""
    protocol_name: str
    """The name of the custom protocol"""
    protocol_version: str
    """The version of the custom protocol"""

@final
class IPduPort:
    """
    The `IPduPort` allows an ECU to send or receive a PDU
    """

    def __init__(self, element: Element) -> IPduPort: ...
    communication_direction: Optional[CommunicationDirection]
    """get or set the communication direction of this `IPduPort`"""
    ecu: EcuInstance
    """get the ECU instance that contains this `IPduPort`"""
    element: Element
    name: str

@final
class IPv4AddressSource:
    """
    `IPv4AddressSource` defines how the address of an IPv4 `NetworkEndpoint` is obtained
    """

    AutoIp: IPv4AddressSource
    AutoIpDoIp: IPv4AddressSource
    DHCPv4: IPv4AddressSource
    Fixed: IPv4AddressSource

@final
class IPv6AddressSource:
    """
    `IPv6AddressSource` defines how the address of an IPv6 `NetworkEndpoint` is obtained
    """

    DHCPv6: IPv6AddressSource
    Fixed: IPv6AddressSource
    LinkLocal: IPv6AddressSource
    LinkLocalDoIp: IPv6AddressSource
    RouterAdvertisement: IPv6AddressSource

@final
class ISignal:
    """
    Signal of the Interaction Layer
    """

    def __init__(self, element: Element) -> ISignal: ...
    def add_data_transformation(
        self, data_transformation: DataTransformation, /
    ) -> None:
        """add a data transformation to this signal"""
        ...

    def create_e2e_transformation_isignal_props(
        self, transformer: TransformationTechnology, /
    ) -> EndToEndTransformationISignalProps:
        """create E2E transformation properties for this signal"""
        ...

    def create_someip_transformation_isignal_props(
        self, transformer: TransformationTechnology, /
    ) -> SomeIpTransformationISignalProps:
        """create SomeIp transformation properties for this signal"""
        ...

    def data_transformations(self, /) -> Iterator[DataTransformation]:
        """get all data transformations that are applied to this signal"""
        ...
    datatype: Optional[SwBaseType]
    """get or set the data type for this signal"""
    element: Element
    length: Optional[int]
    """set the length of this signal in bits"""
    def mappings(self, /) -> List[ISignalToIPduMapping]:
        """list all `ISignalToIPduMapping` for this signal

        Usually a signal should only be mapped to a single PDU,
        so this list is expected to contain either zero or one items in ordinary cases.
        """
        ...
    name: str
    signal_group: Optional[ISignalGroup]
    """get the signal group that contains this signal, if any"""
    def signal_triggerings(self, /) -> List[ISignalTriggering]:
        """list all `ISignalTriggering`s that trigger this signal"""
        ...
    system_signal: Optional[SystemSignal]
    """get the system signal that corresponds to this isignal"""
    def transformation_isignal_props(
        self, /
    ) -> Iterator[
        Union[EndToEndTransformationISignalProps, SomeIpTransformationISignalProps]
    ]:
        """get all transformation properties that are applied to this signal"""
        ...
    init_value: Optional[ValueSpecification]
    """get or set the initial value of the signal"""

@final
class ISignalGroup:
    """
    An `ISignalGroup` groups signals that should always be kept together
    """

    def __init__(self, element: Element) -> ISignalGroup: ...
    def add_data_transformation(
        self, data_transformation: DataTransformation, /
    ) -> None:
        """add a data transformation to this signal group"""
        ...

    def add_signal(self, /, signal: ISignal) -> None:
        """Add a signal to the signal group"""
        ...

    def create_e2e_transformation_isignal_props(
        self, transformer: TransformationTechnology, /
    ) -> EndToEndTransformationISignalProps:
        """create E2E transformation properties for this signal group"""
        ...

    def create_someip_transformation_isignal_props(
        self, transformer: TransformationTechnology, /
    ) -> SomeIpTransformationISignalProps:
        """create SomeIp transformation properties for this signal group"""
        ...

    def data_transformations(self, /) -> Iterator[DataTransformation]:
        """iterate over all data transformations that are applied to this signal group"""
        ...
    element: Element
    name: str
    def signals(self, /) -> Iterator[ISignal]:
        """Iterator over all [`ISignal`]s in this group

        # Example"""
        ...
    system_signal_group: Optional[SystemSignalGroup]
    """get the system signal group that is associated with this signal group"""
    def transformation_isignal_props(
        self, /
    ) -> Iterator[
        Union[EndToEndTransformationISignalProps, SomeIpTransformationISignalProps]
    ]:
        """get all transformation properties that are applied to this signal group"""
        ...

@final
class ISignalIPdu:
    """
    Represents the `IPdus` handled by Com
    """

    def __init__(self, element: Element) -> ISignalIPdu: ...
    contained_ipdu_props: Optional[ContainedIPduProps]
    """set the ContainedIPduProps for this `IPdu`
    This is only needed when the `IPdu` is contained in a `ContainerIPdu`"""
    element: Element
    length: Optional[int]
    """get or set the length of this PDU"""
    def map_signal(
        self,
        signal: ISignal,
        start_position: int,
        byte_order: ByteOrder,
        /,
        *,
        update_bit: Optional[int] = None,
        transfer_property: TransferProperty = TransferProperty.Pending,
    ) -> ISignalToIPduMapping:
        """map a signal to the `ISignalIPdu`

        If this signal is part of a signal group, then the group must be mapped first"""
        ...

    def map_signal_group(self, signal_group: ISignalGroup, /) -> ISignalToIPduMapping:
        """map a signal group to the PDU"""
        ...

    def mapped_signals(self, /) -> Iterator[Union[ISignal, ISignalGroup]]:
        """returns an iterator over all signals and signal groups mapped to the PDU"""
        ...
    name: str
    def pdu_triggerings(self, /) -> List[PduTriggering]:
        """list all `PduTriggerings` that trigger this PDU"""
        ...

    def set_timing(self, /, timing_spec: IpduTiming) -> None:
        """set the transmission timing of the PDU"""
        ...

    def timing(self, /) -> IpduTiming:
        """Helper function to set the transmission mode timing, used by `ISignalIPdu::set_timing` for both true and false timing
        get the transmission timing of the PDU"""
        ...

@final
class ISignalPort:
    """
    The `ISignalPort` allows an ECU to send or receive a Signal
    """

    def __init__(self, element: Element) -> ISignalPort: ...
    communication_direction: Optional[CommunicationDirection]
    """get or set the communication direction of this port"""
    ecu: EcuInstance
    """get the ECU that is connected to this signal port"""
    element: Element
    name: str

@final
class ISignalToIPduMapping:
    """
    `ISignalToIPduMapping` connects an `ISignal` or `ISignalGroup` to an `ISignalToIPdu`
    """

    def __init__(self, element: Element) -> ISignalToIPduMapping: ...
    byte_order: Optional[ByteOrder]
    """get or set the byte order of the data in the mapped signal."""
    element: Element
    name: str
    signal: Optional[ISignal]
    """Reference to the signal that is mapped to the PDU.
    Every mapping contains either a signal or a signal group."""
    signal_group: Optional[ISignalGroup]
    """Reference to the signal group that is mapped to the PDU.
    Every mapping contains either a signal or a signal group."""
    start_position: Optional[int]
    """Start position of the signal data within the PDU (bit position).
    The start position is mandatory if the mapping describes a signal."""
    transfer_property: Optional[TransferProperty]
    """Set the transfer property of the mapped signal"""
    update_bit: Optional[int]
    """Bit position of the update bit for the mapped signal. Not all signals use an update bit.
    This is never used for signal groups"""

@final
class ISignalTriggering:
    """
    an `ISignalTriggering` triggers a signal in a PDU
    """

    def __init__(self, element: Element) -> ISignalTriggering: ...
    def connect_to_ecu(
        self, ecu: EcuInstance, direction: CommunicationDirection, /
    ) -> ISignalPort:
        """connect this signal triggering to an ECU"""
        ...
    element: Element
    name: str
    physical_channel: PhysicalChannel
    """get the physical channel that contains this signal triggering"""
    def signal_ports(self, /) -> Iterator[ISignalPort]:
        """create an iterator over all signal ports that are connected to this signal triggering"""
        ...

@final
class InitialSdDelayConfig:
    """
    A `InitialSdDelayConfig` contains the configuration for the initial delay of an SD client or server
    """

    def __init__(
        self,
        *,
        initial_delay_max_value: float,
        initial_delay_min_value: float,
        initial_repetitions_base_delay: Optional[float] = None,
        initial_repetitions_max: Optional[int] = None,
    ) -> InitialSdDelayConfig: ...
    initial_delay_max_value: float
    """maximum value of the randomized delay in seconds"""
    initial_delay_min_value: float
    """minimum value of the randomized delay in seconds"""
    initial_repetitions_base_delay: Optional[float]
    """base delay for repetitions in seconds"""
    initial_repetitions_max: Optional[int]
    """maximum number of repetitions"""

@final
class IpduTiming:
    """
    Timing specification for an IPDU
    """

    def __init__(
        self,
        *,
        minimum_delay: Optional[float] = None,
        transmission_mode_true_timing: Optional[TransmissionModeTiming] = None,
        transmission_mode_false_timing: Optional[TransmissionModeTiming] = None,
    ) -> IpduTiming: ...
    minimum_delay: Optional[float]
    """minimum delay in seconds between two transmissions of the PDU"""
    transmission_mode_false_timing: Optional[TransmissionModeTiming]
    """timing specification if the COM transmission mode is false"""
    transmission_mode_true_timing: Optional[TransmissionModeTiming]
    """timing specification if the COM transmission mode is true"""

@final
class LinCluster:
    """
    A `LinCluster` represents a LIN cluster in a LIN network
    """

    def __init__(self, element: Element) -> LinCluster: ...
    element: Element
    name: str

@final
class LinEventTriggeredFrame:
    def __init__(self, element: Element) -> LinEventTriggeredFrame: ...
    element: Element
    name: str

@final
class LinMaster:
    """
    A `LinMaster` represents a LIN master node in a LIN cluster
    """

    def __init__(self, element: Element) -> LinMaster: ...
    element: Element
    name: str

@final
class LinPhysicalChannel:
    def __init__(self, element: Element) -> LinPhysicalChannel: ...
    element: Element
    name: str

@final
class LinSlave:
    """
    A `LinSlave` represents a LIN slave node in a LIN cluster
    """

    def __init__(self, element: Element) -> LinSlave: ...
    element: Element
    name: str

@final
class LinSporadicFrame:
    def __init__(self, element: Element) -> LinSporadicFrame: ...
    element: Element
    name: str

@final
class LinUnconditionalFrame:
    def __init__(self, element: Element) -> LinUnconditionalFrame: ...
    element: Element
    name: str

@final
class LocalUnicastAddress:
    """
    A `LocalUnicastAddress` is a local address (TCP or UDP) that can be used for a `ProvidedServiceInstance` or `ConsumedServiceInstance`
    """

    Tcp: Type[LocalUnicastAddress_Tcp]
    Udp: Type[LocalUnicastAddress_Udp]

@final
class LocalUnicastAddress_Tcp(LocalUnicastAddress):
    def __init__(self, address: SocketAddress) -> LocalUnicastAddress_Tcp: ...
    address: SocketAddress

@final
class LocalUnicastAddress_Udp(LocalUnicastAddress):
    def __init__(self, address: SocketAddress) -> LocalUnicastAddress_Udp: ...
    address: SocketAddress

@final
class MaximumMessageLengthType:
    """
    Types of Maximum Message Length that can be used in an `FlexrayArTpChannel`
    """

    I4g: MaximumMessageLengthType
    Iso: MaximumMessageLengthType
    Iso6: MaximumMessageLengthType

@final
class MultiplexedIPdu:
    """
    The multiplexed pdu contains one of serveral signal pdus
    """

    def __init__(self, element: Element) -> MultiplexedIPdu: ...
    contained_ipdu_props: Optional[ContainedIPduProps]
    """set the ContainedIPduProps for this `IPdu`
    This is only needed when the `IPdu` is contained in a `ContainerIPdu`"""
    element: Element
    length: Optional[int]
    """get or set the length of this PDU"""
    name: str
    def pdu_triggerings(self, /) -> List[PduTriggering]:
        """list all `PduTriggerings` that trigger this PDU"""
        ...

@final
class NPdu:
    """
    This is a Pdu of the transport layer. The main purpose of the TP layer is to segment and reassemble `IPdus`.
    """

    def __init__(self, element: Element) -> NPdu: ...
    contained_ipdu_props: Optional[ContainedIPduProps]
    """set the ContainedIPduProps for this `IPdu`
    This is only needed when the `IPdu` is contained in a `ContainerIPdu`"""
    element: Element
    length: Optional[int]
    """get or set the length of this PDU"""
    name: str
    def pdu_triggerings(self, /) -> List[PduTriggering]:
        """list all `PduTriggerings` that trigger this PDU"""
        ...

@final
class NetworkEndpoint:
    """
    A network endpoint contains address information for a connection
    """

    def __init__(self, element: Element) -> NetworkEndpoint: ...
    def add_network_endpoint_address(self, address: NetworkEndpointAddress, /) -> None:
        """add a network endpoint address to this `NetworkEndpoint`

        A `NetworkEndpoint` may have multiple sets of address information. The following restrictions apply:

        - all addresses must have the same type, i.e. all IPv4 or all IPv6
        - only one of them may be a `Fixed` address, all others must be dynamic (DHCP, automatic link local, etc.)
        """
        ...

    def addresses(self, /) -> Iterator[NetworkEndpointAddress]:
        """iterator over all addresses in the `NetworkEndpoint`"""
        ...
    element: Element
    name: str

@final
class NetworkEndpointAddress:
    """
    address information for a network endpoint
    """

    IPv4: Type[NetworkEndpointAddress_IPv4]
    IPv6: Type[NetworkEndpointAddress_IPv6]

@final
class NetworkEndpointAddress_IPv4(NetworkEndpointAddress):
    def __init__(
        self,
        *,
        address: Optional[str] = None,
        address_source: Optional[IPv4AddressSource] = None,
        default_gateway: Optional[str] = None,
        network_mask: Optional[str] = None,
    ) -> NetworkEndpointAddress_IPv4: ...
    address: Optional[str]
    address_source: Optional[IPv4AddressSource]
    default_gateway: Optional[str]
    network_mask: Optional[str]

@final
class NetworkEndpointAddress_IPv6(NetworkEndpointAddress):
    def __init__(
        self,
        *,
        address: Optional[str] = None,
        address_source: Optional[IPv6AddressSource] = None,
        default_router: Optional[str] = None,
    ) -> NetworkEndpointAddress_IPv6: ...
    address: Optional[str]
    address_source: Optional[IPv6AddressSource]
    default_router: Optional[str]

@final
class NmConfig:
    """
    The `NmConfig` is the root element for the network management configuration.

    Only one config may exist per `System`, and this configuration may contain multiple `NmClusters` for different bus types.

    Use [`System::create_nm_config`](crate::System::create_nm_config) to create a new `NmConfig` in a `System`.
    """

    def __init__(self, element: Element) -> NmConfig: ...
    def create_can_nm_cluster(
        self, name: str, settings: CanNmClusterSettings, can_cluster: CanCluster
    ) -> CanNmCluster:
        """create a new `CanNmCluster`"""
        ...

    def create_can_nm_cluster_coupling(
        self, nm_busload_reduction_enabled: bool, nm_immediate_restart_enabled: bool, /
    ) -> CanNmClusterCoupling:
        """create a new `CanNmClusterCoupling`"""
        ...

    def create_flexray_nm_cluster(
        self,
        name: str,
        settings: FlexrayNmClusterSettings,
        flexray_cluster: FlexrayCluster,
    ) -> FlexrayNmCluster:
        """create a new `FlexrayNmCluster`"""
        ...

    def create_flexray_nm_cluster_coupling(
        self, nm_schedule_variant: FlexrayNmScheduleVariant, /
    ) -> FlexrayNmClusterCoupling:
        """create a new `FlexrayNmClusterCoupling`"""
        ...

    def create_nm_ecu(self, name: str, ecu_instance: EcuInstance, /) -> NmEcu:
        """create a new `NmEcu`"""
        ...

    def create_udp_nm_cluster(
        self,
        name: str,
        settings: UdpNmClusterSettings,
        ethernet_cluster: EthernetCluster,
    ) -> UdpNmCluster:
        """create a new `UdpNmCluster`"""
        ...

    def create_udp_nm_cluster_coupling(self, /) -> UdpNmClusterCoupling:
        """create a new `UdpNmClusterCoupling`"""
        ...
    element: Element
    name: str
    def nm_cluster_couplings(
        self, /
    ) -> Iterator[
        Union[CanNmClusterCoupling, FlexrayNmClusterCoupling, UdpNmClusterCoupling]
    ]:
        """iterate over all `NmClusterCouplings`"""
        ...

    def nm_clusters(
        self, /
    ) -> Iterator[Union[CanNmCluster, FlexrayNmCluster, UdpNmCluster]]:
        """get all `NmClusters`"""
        ...

    def nm_ecus(self, /) -> Iterator[NmEcu]:
        """iterate over all `NmEcus`"""
        ...

@final
class NmEcu:
    """
    The `NmEcu` represents an `EcuInstance` wich participates in network management.
    """

    def __init__(self, element: Element) -> NmEcu: ...
    cycle_time_main_function: Optional[float]
    """get or set or remove the nmCycletimeMainFunction value"""
    ecu_instance: Optional[EcuInstance]
    """get or set the referenced `EcuInstance`"""
    element: Element
    name: str
    nm_bus_synchronization_enabled: Optional[bool]
    """get or set the nmBusSynchronizationEnabled flag"""
    nm_com_control_enabled: Optional[bool]
    """get or set the nmComControlEnabled flag"""

@final
class NmPdu:
    """
    Network Management Pdu
    """

    def __init__(self, element: Element) -> NmPdu: ...
    element: Element
    length: Optional[int]
    """get or set the length of this PDU"""
    name: str
    def pdu_triggerings(self, /) -> List[PduTriggering]:
        """List all `PduTriggerings` that trigger this PDU"""
        ...

@final
class PduActivationRoutingGroup:
    """
    A group of Pdus that can be activated or deactivated for transmission over a socket connection.
    It is used by `EventHandler`s in `ProvidedServiceInstance`s and `ConsumedServiceInstance`s.
    """

    def __init__(self, element: Element) -> PduActivationRoutingGroup: ...
    def add_ipdu_identifier_tcp(self, ipdu_identifier: SoConIPduIdentifier, /) -> None:
        """add a reference to a `SoConIPduIdentifier` for TCP communication to this `PduActivationRoutingGroup`"""
        ...

    def add_ipdu_identifier_udp(self, ipdu_identifier: SoConIPduIdentifier, /) -> None:
        """add a reference to a `SoConIPduIdentifier` for UDP communication to this `PduActivationRoutingGroup`"""
        ...
    element: Element
    event_group_control_type: Optional[EventGroupControlType]
    """get or set the event group control type of this `PduActivationRoutingGroup`"""
    def ipdu_identifiers_tcp(self, /) -> Iterator[SoConIPduIdentifier]:
        """get all `SoConIPduIdentifier`s for TCP communication in this `PduActivationRoutingGroup`"""
        ...

    def ipdu_identifiers_udp(self, /) -> Iterator[SoConIPduIdentifier]:
        """get all `SoConIPduIdentifier`s for UDP communication in this `PduActivationRoutingGroup`"""
        ...
    name: str

@final
class PduCollectionTrigger:
    """
    The collction trigger defines whether a Pdu contributes to the triggering
    of the data transmission if Pdu collection is enabled
    """

    Always: PduCollectionTrigger
    Never: PduCollectionTrigger

@final
class PduToFrameMapping:
    """
    `PduToFrameMapping` connects a PDU to a frame
    """

    def __init__(self, element: Element) -> PduToFrameMapping: ...
    byte_order: Optional[ByteOrder]
    """get or set the byte order of the data in the PDU.
    
    All `PduToFrameMappings` within a frame must have the same byte order.
    PDUs may not use the byte order value `Opaque`.
    
    Note: If the byte order is swapped, then the start position must be adjusted accordingly."""
    element: Element
    name: str
    pdu: Optional[Pdu]
    """Reference to the PDU that is mapped into the frame. The PDU reference is mandatory."""
    start_position: Optional[int]
    """set the start position of the PDU data within the frame (bit position).
    
    PDUs are byte aligned.
    For little-endian data the values 0, 8, 16, ... are allowed;
    for big-endian data the values 7, 15, 23, ... are allowed.
    
    Note: if you intend to change both the byte order and the start position, then you should change the byte order first.
    New values set here must match the configured byte order."""
    update_bit: Optional[int]
    """set or clear the bit position of the update bit for the mapped PDU."""

@final
class PduTriggering:
    """
    a `PduTriggering` triggers a PDU in a frame or ethernet connection
    """

    def __init__(self, element: Element) -> PduTriggering: ...
    def create_pdu_port(
        self, ecu: EcuInstance, direction: CommunicationDirection, /
    ) -> IPduPort:
        """create an `IPduPort` to connect a `PduTriggering` to an `EcuInstance`"""
        ...
    element: Element
    name: str
    pdu: Optional[Pdu]
    """get the Pdu that is triggered by this pdu triggering"""
    def pdu_ports(self, /) -> Iterator[IPduPort]:
        """create an iterator over the `IPduPorts` that are connected to this `PduTriggering`"""
        ...
    physical_channel: PhysicalChannel
    """get the physical channel that contains this pdu triggering"""
    def signal_triggerings(self, /) -> Iterator[ISignalTriggering]:
        """create an iterator over the `ISignalTriggerings` that are triggered by this `PduTriggering`"""
        ...

@final
class ProvidedServiceInstance:
    """
    A `ProvidedServiceInstance` is a service that is provided by an ECU
    """

    def __init__(self, element: Element) -> ProvidedServiceInstance: ...
    def create_event_handler(
        self, name: str, event_group_identifier: int, /
    ) -> EventHandler:
        """create a new `EventHandler` in this `ProvidedServiceInstance`"""
        ...
    element: Element
    def event_handlers(self, /) -> Iterator[EventHandler]:
        """get the `EventHandler`s in this `ProvidedServiceInstance`"""
        ...
    instance_identifier: Optional[int]
    """get or set the instance identifier of this `ProvidedServiceInstance`"""
    def local_unicast_addresses(self, /) -> Iterator[LocalUnicastAddress]:
        """iterate over the local unicast addresses"""
        ...
    major_version: Optional[int]
    """get or set the major version of this `ProvidedServiceInstance`"""
    minor_version: Optional[int]
    """get or set the minor version of this `ProvidedServiceInstance`"""
    name: str
    sd_server_instance_config: Optional[SomeipSdServerServiceInstanceConfig]
    """get or set the SD server instance configuration for this `ProvidedServiceInstance`"""
    service_identifier: Optional[int]
    """get or set the service identifier of this `ProvidedServiceInstance`"""
    def set_local_unicast_address(self, address: SocketAddress, /) -> None:
        """set a local unicast address for this `ProvidedServiceInstance`

        The PSI may use two local unicast addresses, one each for UDP and TCP.
        The unicast address is used to assign the service to a specific ECU, and may not be empty.
        """
        ...

@final
class ProvidedServiceInstanceV1:
    """
    A `ProvidedServiceInstanceV1` is a SD service instance that is provided by this ECU.

    This is the old V1 version of the service definition.
    """

    def __init__(self, element: Element) -> ProvidedServiceInstanceV1: ...
    def create_event_handler(self, name: str, /) -> EventHandlerV1:
        """create a new `EventHandlerV1` in this `ProvidedServiceInstance`"""
        ...
    element: Element
    def event_handlers(self, /) -> Iterator[EventHandlerV1]:
        """get the `EventHandlerV1`s in this `ProvidedServiceInstance`"""
        ...
    instance_identifier: Optional[int]
    """get or set the instance identifier of this `ProvidedServiceInstance`"""
    name: str
    def sd_server_config(self) -> Optional[SdConfig]:
        """get or set the SD server configuration for this `ProvidedServiceInstance`"""
        ...

    def set_sd_server_config(self, sd_server_config: SdConfig) -> None:
        """get or set the SD server configuration for this `ProvidedServiceInstance`"""
        ...
    service_identifier: Optional[int]
    """get or set the service identifier of this `ProvidedServiceInstance`"""

@final
class RequestResponseDelay:
    """
    A RequestResponseDelay contains the minimum and maximum delay for a request-response cycle
    """

    def __init__(
        self, *, min_value: float, max_value: float
    ) -> RequestResponseDelay: ...
    max_value: float
    """set the maximum value of this `RequestResponseDelay`"""
    min_value: float
    """set the minimum value of this `RequestResponseDelay`"""

@final
class RxAcceptContainedIPdu:
    """The `RxAcceptContainedIPdu` enum defines whether a fixed set of contained IPdus is accepted or all contained IPdus"""

    AcceptAll: RxAcceptContainedIPdu
    AcceptConfigured: RxAcceptContainedIPdu

@final
class SdConfig:
    """
    SD configuration for a service instance

    This struct is used to configure the SD server and client behavior for a service instance.
    it is used for the old V1 service definitions.
    """

    def __init__(
        self,
        *,
        service_major_version: int,
        service_minor_version: int,
        initial_delay_max_value: float,
        initial_delay_min_value: float,
        initial_repetitions_base_delay: Optional[float] = None,
        initial_repetitions_max: int,
        offer_cyclic_delay: Optional[float] = None,
        request_response_delay_max_value: float,
        request_response_delay_min_value: float,
        ttl: int,
    ) -> SdConfig: ...
    initial_delay_max_value: float
    """The maximum delay for the initial offer"""
    initial_delay_min_value: float
    """The minimum delay for the initial offer"""
    initial_repetitions_base_delay: Optional[float]
    """The base delay for offer repetitions (if aggregated by `SdServerConfig`) or find repetitions (if aggregated by `SdClientConfig`)"""
    initial_repetitions_max: int
    """The maximum number of repetitions for the initial offer or find"""
    offer_cyclic_delay: Optional[float]
    """The delay between two offers (if aggregated by `SdServerConfig`) or finds (if aggregated by `SdClientConfig`)"""
    request_response_delay_max_value: float
    """The maximum delay for a request-response cycle"""
    request_response_delay_min_value: float
    """The minimum delay for a request-response cycle"""
    service_major_version: int
    """The major version of the service"""
    service_minor_version: int
    """The minor version of the service"""
    ttl: int
    """The time-to-live for the service offer"""

@final
class SdEventConfig:
    """
    Configuration for an SD event handler
    """

    def __init__(
        self,
        *,
        request_response_delay_max_value: float,
        request_response_delay_min_value: float,
        ttl: int,
    ) -> SdEventConfig: ...
    request_response_delay_max_value: float
    """The maximum delay for a request-response cycle"""
    request_response_delay_min_value: float
    """The minimum delay for a request-response cycle"""
    ttl: int
    """The time-to-live for the service offer"""

@final
class SecureCommunicationProps:
    """The properties of a `SecuredIPdu`"""

    def __init__(
        self,
        /,
        *,
        auth_data_freshness_length: Optional[int] = None,
        auth_data_freshness_start_position: Optional[int] = None,
        authentication_build_attempts: Optional[int] = None,
        authentication_retries: Optional[int] = None,
        data_id: Optional[int] = None,
        freshness_value_id: Optional[int] = None,
        message_link_length: Optional[int] = None,
        message_link_position: Optional[int] = None,
        secondary_freshness_value_id: Optional[int] = None,
        secured_area_length: Optional[int] = None,
        secured_area_offset: Optional[int] = None,
    ) -> SecureCommunicationProps: ...
    auth_data_freshness_length: Optional[int]
    """length in bits of the authentic PDU data"""
    auth_data_freshness_start_position: Optional[int]
    """start position in bits of the authentic PDU data"""
    authentication_build_attempts: Optional[int]
    """number of authentication build attempts"""
    authentication_retries: Optional[int]
    """number of additional authentication attempts. If this value is zero, the authentication is not repeated"""
    data_id: Optional[int]
    """numerical identifier of the secured IPdu"""
    freshness_value_id: Optional[int]
    """id of the freshness value"""
    message_link_length: Optional[int]
    """message link length in bits"""
    message_link_position: Optional[int]
    """message link start position in bits"""
    secondary_freshness_value_id: Optional[int]
    """seconday freshness value id"""
    secured_area_length: Optional[int]
    """length in bytes of the secure area inside the payload pdu"""
    secured_area_offset: Optional[int]
    """start position in bytes of the secure area inside the payload pdu"""

@final
class SecuredIPdu:
    """
    Wraps an `IPdu` to protect it from unauthorized manipulation
    """

    def __init__(self, element: Element) -> SecuredIPdu: ...
    contained_ipdu_props: Optional[ContainedIPduProps]
    """set the ContainedIPduProps for this `IPdu`
    This is only needed when the `IPdu` is contained in a `ContainerIPdu`"""
    element: Element
    length: Optional[int]
    """get or set the length of this PDU"""
    name: str
    payload_pdu_triggering: Optional[PduTriggering]
    """get or set the `PduTriggering` that triggers the payload PDU
    
    When use_as_cryptographic_ipdu is true, this attribute can be used to directly set PduTriggering of the payload PDU.
    When use_as_cryptographic_ipdu is false, the function `set_payload_ipdu` should be used to create a new PduTriggering and set it."""
    def pdu_triggerings(self, /) -> List[PduTriggering]:
        """list all `PduTriggerings` that trigger this PDU"""
        ...
    secure_communication_props: Optional[SecureCommunicationProps]
    def set_payload_ipdu(
        self, pdu: IPdu, physical_channel: PhysicalChannel, /
    ) -> PduTriggering:
        """set the payload PduTriggering based on an IPdu

        This function should be used when useAsCryptographicIPdu is false or not set.
        A PduTriggering is created for the Pdu"""
        ...
    use_as_cryptographic_ipdu: Optional[bool]

@final
class ServiceInstanceCollectionSet:
    """
    A `ServiceInstanceCollectionSet` contains `ServiceInstance`s that are provided or consumed by an ECU
    """

    def __init__(self, element: Element) -> ServiceInstanceCollectionSet: ...
    def create_consumed_service_instance(
        self,
        name: str,
        service_identifier: int,
        instance_identifier: int,
        major_version: int,
        minor_version: str,
        /,
    ) -> ConsumedServiceInstance:
        """create a new `ConsumedServiceInstance` in this `ServiceInstanceCollectionSet`"""
        ...

    def create_provided_service_instance(
        self,
        name: str,
        service_identifier: int,
        instance_identifier: int,
        major_version: int,
        minor_version: int,
        /,
    ) -> ProvidedServiceInstance:
        """create a new `ProvidedServiceInstance` in this `ServiceInstanceCollectionSet`"""
        ...
    element: Element
    name: str
    def service_instances(
        self, /
    ) -> Iterator[Union[ConsumedServiceInstance, ProvidedServiceInstance]]:
        """create an iterator over all `ServiceInstances` in this set"""
        ...

@final
class SoAdRoutingGroup:
    """
    A `SoAdRoutingGroup` is used to link `SomeIp` settings in Consumed/ProvidedServiceInstances
    to the `SocketConnectionBundles` used for transmission.
    `SoAdRoutingGroups` are part of the old way of configuring Ethernet communication in AUTOSAR.
    """

    def __init__(self, element: Element) -> SoAdRoutingGroup: ...
    control_type: Optional[EventGroupControlType]
    """get or set the `EventGroupControlType` of this `SoAdRoutingGroup`"""
    element: Element
    name: str

@final
class SoConIPduIdentifier:
    """
    A `SoConIPduIdentifier` describes a PDU that is transported over a static socket connection.
    """

    def __init__(self, element: Element) -> SoConIPduIdentifier: ...
    collection_trigger: Optional[PduCollectionTrigger]
    """get or set the collection trigger for this `SoConIPduIdentifier`"""
    element: Element
    header_id: Optional[int]
    """get or set the header id for this `SoConIPduIdentifier`"""
    name: str
    pdu_triggering: Optional[PduTriggering]
    """get the `PduTriggering` referenced by this `SoConIPduIdentifier`"""
    def set_pdu(self, pdu: Pdu, channel: EthernetPhysicalChannel, /) -> None:
        """create a new `PduTriggering` for the pdu and reference it in this `SoConIPduIdentifier`"""
        ...
    timeout: Optional[float]
    """set the timeout for this `SoConIPduIdentifier`"""

@final
class SocketAddress:
    """
    A socket address establishes the link between one or more ECUs and a `NetworkEndpoint`.
    It contains all settings that are relevant for this combination.
    """

    def __init__(self, element: Element) -> SocketAddress: ...
    def add_multicast_ecu(self, ecu: EcuInstance, /) -> None:
        """add an `EcuInstance` to this multicast `SocketAddress`"""
        ...

    def consumed_service_instances(self, /) -> Iterator[ConsumedServiceInstanceV1]:
        """get the `ConsumedServiceInstance`s in this `SocketAddress`"""
        ...

    def create_consumed_service_instance(
        self, name: str, provided_service_instance: ProvidedServiceInstanceV1, /
    ) -> ConsumedServiceInstanceV1:
        """create a `ConsumedServiceInstanceV1` in this `SocketAddress`

        Creating a `ConsumedServiceInstanceV1` in a `SocketAddress` is part of the old way of defining services (<= Autosar 4.5.0).
        It is obsolete in newer versions of the standard.

        When using the new way of defining services, a `ConsumedServiceInstance` should be created in a `ServiceInstanceCollectionSet` instead.
        """
        ...

    def create_provided_service_instance(
        self, name: str, service_identifier: int, instance_identifier: int, /
    ) -> ProvidedServiceInstanceV1:
        """create a `ProvidedServiceInstanceV1` in this `SocketAddress`

        Creating a `ProvidedServiceInstanceV1` in a `SocketAddress` is part of the old way of defining services (<= Autosar 4.5.0).
        It is obsolete in newer versions of the standard.

        When using the new way of defining services, a `ProvidedServiceInstance` should be created in a `ServiceInstanceCollectionSet` instead.
        """
        ...

    def create_static_socket_connection(
        self,
        name: str,
        remote_address: SocketAddress,
        /,
        *,
        tcp_role: Optional[TcpRole] = None,
        tcp_connect_timeout: Optional[float] = None,
    ) -> StaticSocketConnection:
        """create a new `StaticSocketConnection` from this `SocketAddress` to a remote `SocketAddress`"""
        ...
    element: Element
    name: str
    network_endpoint: Optional[NetworkEndpoint]
    """get the network endpoint of this `SocketAddress`"""
    physical_channel: EthernetPhysicalChannel
    """get the `EthernetPhysicalChannel` containing this `SocketAddress`"""
    def provided_service_instances(self, /) -> Iterator[ProvidedServiceInstanceV1]:
        """get the `ProvidedServiceInstanceV1`s in this `SocketAddress`"""
        ...

    def set_unicast_ecu(self, ecu: EcuInstance, /) -> None:
        """set the `EcuInstance` for this unicast `SocketAddress`"""
        ...
    socket_address_type: Optional[SocketAddressType]
    """get the socket address type: unicast / multicast, as well as the connected ecus"""
    def static_socket_connections(self, /) -> Iterator[StaticSocketConnection]:
        """iterate over all `StaticSocketConnection`s in this `SocketAddress`"""
        ...
    tp_config: Optional[TpConfig]
    """get the transport protocol settings for this `SocketAddress`"""

@final
class SocketAddressType:
    """
    Describes if a [`SocketAddress`] is used for unicast or multicast
    """

    Unicast: Type[SocketAddressType_Unicast]
    """SocketAddressType.Unicast(ecu | None)"""
    Multicast: Type[SocketAddressType_Multicast]

@final
class SocketAddressType_Unicast(SocketAddressType):
    def __init__(
        self, ecu: Optional[EcuInstance] = None, /
    ) -> SocketAddressType_Unicast: ...
    ecu: Optional[EcuInstance]

@final
class SocketAddressType_Multicast(SocketAddressType):
    def __init__(
        self, ecus: List[EcuInstance] = [], /
    ) -> SocketAddressType_Multicast: ...
    ecus: List[EcuInstance]

@final
class SocketConnection:
    """
    A socketConnection inside a `SocketConnectionBundle` describes a single connection to a specific client port.
    """

    def __init__(self, element: Element) -> SocketConnection: ...
    client_ip_addr_from_connection_request: Optional[bool]
    """get or set the `client_ip_addr_from_connection_request` attribute for this socket connection
    
    if the value is Some(true), the attribute is set to "true"
    if the value is Some(false), the attribute is set to "false"
    if the value is None, the attribute is removed"""
    client_port: Optional[SocketAddress]
    """get or set the client port of this socket connection"""
    client_port_from_connection_request: Optional[bool]
    """get or set the `client_port_from_connection_request` attribute for this socket connection
    
    if the value is Some(true), the attribute is set to "true"
    if the value is Some(false), the attribute is set to "false"
    if the value is None, the attribute is removed"""
    def create_socket_connection_ipdu_identifier(
        self,
        pdu: Pdu,
        header_id: int,
        /,
        *,
        timeout: Optional[float] = None,
        collection_trigger: Optional[PduCollectionTrigger] = None,
    ) -> Tuple[SocketConnectionIpduIdentifier, PduTriggering]:
        """add a PDU to the socket connection, returning a `PduTriggering`"""
        ...
    element: Element
    def pdu_triggerings(self, /) -> Iterator[PduTriggering]:
        """create an iterator over all PDU triggerings in this socket connection"""
        ...
    runtime_ip_address_configuration: bool
    """get or set the value of the RuntimeIpAddressConfiguration attribute for this socket connection"""
    runtime_port_configuration: bool
    """get or set the value of the RuntimePortConfiguration attribute for this socket connection"""
    socket_connection_bundle: SocketConnectionBundle
    """get the socket connection bundle containing this socket connection"""
    def socket_connection_ipdu_identifiers(
        self, /
    ) -> Iterator[SocketConnectionIpduIdentifier]:
        """create an iterator over all `SocketConnectionIpduIdentifiers` in this socket connection"""
        ...

@final
class SocketConnectionBundle:
    """
    A `SocketConnectionBundle` describes a connection between a server port and multiple client ports.
    It contains multiple bundled connections, each transporting one or more PDUs.
    """

    def __init__(self, element: Element) -> SocketConnectionBundle: ...
    def bundled_connections(self, /) -> Iterator[SocketConnection]:
        """create an iterator over all bundled connections in this socket connection bundle"""
        ...

    def create_bundled_connection(
        self, client_port: SocketAddress, /
    ) -> SocketConnection:
        """create a bundled `SocketConnection` between the server port and a client port"""
        ...
    element: Element
    name: str
    physical_channel: EthernetPhysicalChannel
    """get the physical channel containing this socket connection bundle"""
    server_port: Optional[SocketAddress]
    """get or set the server port of this socket connection bundle"""

@final
class SocketConnectionIpduIdentifier:
    """
    A `SocketConnectionIpduIdentifier` is used to trigger a PDU in a `SocketConnection`.

    In addition to the Pdu Triggering, it also contains associated settings like the
    header id, timeout and collection trigger.
    """

    def __init__(self, element: Element) -> SocketConnectionIpduIdentifier: ...
    def add_routing_group(self, routing_group: SoAdRoutingGroup, /) -> None:
        """add a reference to a `SoAdRoutingGroup` to this `SocketConnectionIpduIdentifier`"""
        ...
    collection_trigger: Optional[PduCollectionTrigger]
    """set the collection trigger for this `SocketConnectionIpduIdentifier`"""
    element: Element
    header_id: Optional[int]
    """set the header id for this `SocketConnectionIpduIdentifier`"""
    pdu_triggering: Optional[PduTriggering]
    """get the `PduTriggering` associated with this `SocketConnectionIpduIdentifier`"""
    def routing_groups(self, /) -> Iterator[SoAdRoutingGroup]:
        """create an iterator over all `SoAdRoutingGroups` referenced by this `SocketConnectionIpduIdentifier`"""
        ...
    socket_connection: SocketConnection
    """get the SocketConnection containing this `SocketConnectionIpduIdentifier`"""
    timeout: Optional[float]
    """set the timeout for this `SocketConnectionIpduIdentifier`"""
    def trigger_pdu(self, pdu: Pdu, /) -> PduTriggering:
        """trigger a PDU in this `SocketConnectionIpduIdentifier`, creating a `PduTriggering`"""
        ...

@final
class SocketConnectionIpduIdentifierSet:
    """
    A `SocketConnectionIpduIdentifierSet` contains a set of `SoConIPduIdentifiers`, which are used in static socket connections and in `SomeIp` events.
    """

    def __init__(self, element: Element) -> SocketConnectionIpduIdentifierSet: ...
    def create_socon_ipdu_identifier(
        self,
        name: str,
        pdu: Pdu,
        channel: EthernetPhysicalChannel,
        /,
        *,
        header_id: Optional[int] = None,
        timeout: Optional[float] = None,
        collection_trigger: Optional[PduCollectionTrigger] = None,
    ) -> SoConIPduIdentifier:
        """create a new `SoConIPduIdentifier` in this set"""
        ...
    element: Element
    name: str
    def socon_ipdu_identifiers(self, /) -> Iterator[SoConIPduIdentifier]:
        """create an iterator over all `SoConIPduIdentifiers` in this set"""
        ...

@final
class SomeIpMessageType:
    """
    message types that can be used in a SOME/IP message header, depending on the type of communication
    """

    Notification: SomeIpMessageType
    Request: SomeIpMessageType
    RequestNoReturn: SomeIpMessageType
    Response: SomeIpMessageType

@final
class SomeIpTransformationISignalProps:
    """
    Properties for the SOMEIP transformation of an ISignal(Group)
    """

    def __init__(self, element: Element) -> SomeIpTransformationISignalProps: ...
    dynamic_length: Optional[bool]
    """get or set the dynamic length property"""
    element: Element
    interface_version: Optional[int]
    """get or set the interface version property"""
    legacy_strings: Optional[bool]
    """get or set the legacy strings property"""
    message_type: Optional[SomeIpMessageType]
    """get or set the message type property"""
    size_of_array_length: Optional[int]
    """get or set the size of array length property"""
    size_of_string_length: Optional[int]
    """get or set the size of string length property"""
    size_of_struct_length: Optional[int]
    """get or set the size of struct length property"""
    size_of_union_length: Optional[int]
    """get or set the size of union length property"""
    transformer: Optional[TransformationTechnology]
    """get or set the transformer reference of the E2E transformation properties"""

@final
class SomeIpTransformationTechnologyConfig:
    """
    Configuration for a SOMEIP transformation
    """

    def __init__(
        self, *, alignment: int, byte_order: ByteOrder, interface_version: int
    ) -> SomeIpTransformationTechnologyConfig: ...
    alignment: int
    """The alignment of the data in bits"""
    byte_order: ByteOrder
    """The byte order of the data"""
    interface_version: int
    """The interface version the SOME/IP transformer shall use."""

@final
class SomeipSdClientEventGroupTimingConfig:
    """
    A `SomeipSdClientEventGroupTimingConfig` contains the configuration for the timing of a `ConsumedEventGroup`

    This configuration is a named element that is created separately and can be used by multiple `ConsumedEventGroup`s.

    Use [`ArPackage::create_someip_sd_client_event_group_timing_config`] to create a new `SomeipSdClientEventGroupTimingConfig`.
    """

    def __init__(self, element: Element) -> SomeipSdClientEventGroupTimingConfig: ...
    element: Element
    name: str
    def request_response_delay(self) -> Optional[RequestResponseDelay]:
        """get the request response delay of this `SomeipSdClientEventGroupTimingConfig`"""
        ...

    def set_request_response_delay(
        self,
        request_response_delay: RequestResponseDelay,
        /,
    ) -> None:
        """set the request response delay of this `SomeipSdClientEventGroupTimingConfig`"""
        ...
    subscribe_eventgroup_retry_delay: Optional[float]
    """get or set the subscribe eventgroup retry delay of this `SomeipSdClientEventGroupTimingConfig`"""
    subscribe_eventgroup_retry_max: Optional[int]
    """get or set subscribe eventgroup retry max of this `SomeipSdClientEventGroupTimingConfig`"""
    time_to_live: Optional[int]
    """get or set the time to live of this `SomeipSdClientEventGroupTimingConfig`"""

@final
class SomeipSdClientServiceInstanceConfig:
    """
    A `SomeipSdClientServiceInstanceConfig` is a configuration for a `ConsumedServiceInstance`

    This configuration is a named element that is created separately and can be used by multiple `ConsumedServiceInstance`s.

    Use [`ArPackage::create_someip_sd_client_service_instance_config`] to create a new `SomeipSdClientServiceInstanceConfig`.
    """

    def __init__(self, element: Element) -> SomeipSdClientServiceInstanceConfig: ...
    element: Element
    def initial_find_behavior(self) -> Optional[InitialSdDelayConfig]:
        """get the initial find behavior of this `SomeipSdClientServiceInstanceConfig`"""
        ...

    def set_initial_find_behavior(
        self, initial_find_behavior: InitialSdDelayConfig, /
    ) -> None:
        """set the initial find behavior of this `SomeipSdClientServiceInstanceConfig`"""
        ...
    name: str
    priority: Optional[int]
    """get or set the priority of this `SomeipSdClientServiceInstanceConfig`
    
    Available since R21-11 (`AUTOSAR_00050`)"""

@final
class SomeipSdServerEventGroupTimingConfig:
    """
    A `SomeipSdServerEventGroupTimingConfig` contains the configuration for the timing of an `EventHandler`

    This configuration is a named element that is created separately and can be used by multiple `EventHandler`s.

    Use [`ArPackage::create_someip_sd_server_event_group_timing_config`] to create a new `SomeipSdServerEventGroupTimingConfig`.
    """

    def __init__(self, element: Element) -> SomeipSdServerEventGroupTimingConfig: ...
    element: Element
    name: str
    def request_response_delay(self) -> Optional[RequestResponseDelay]:
        """get the request response delay of this `SomeipSdServerEventGroupTimingConfig`"""
        ...

    def set_request_response_delay(
        self, request_response_Delay: RequestResponseDelay, /
    ) -> None:
        """set the request response delay of this `SomeipSdServerEventGroupTimingConfig`"""
        ...

@final
class SomeipSdServerServiceInstanceConfig:
    """
    A `SomeipSdServerServiceInstanceConfig` is a configuration for a `ProvidedServiceInstance`

    This configuration is a named element that is created separately and can be used by multiple `ProvidedServiceInstance`s.

    Use [`ArPackage::create_someip_sd_server_service_instance_config`] to create a new `SomeipSdServerServiceInstanceConfig`.
    """

    def __init__(self, element: Element) -> SomeipSdServerServiceInstanceConfig: ...
    element: Element
    def initial_offer_behavior(self) -> Optional[InitialSdDelayConfig]:
        """get the initial offer behavior of this `SomeipSdServerServiceInstanceConfig`"""
        ...

    def set_initial_offer_behavior(
        self, initial_offer_behavior: InitialSdDelayConfig, /
    ) -> None:
        """set the initial offer behavior of this `SomeipSdServerServiceInstanceConfig`"""
        ...
    name: str
    offer_cyclic_delay: Optional[float]
    """get or set the offer cyclic delay of this `SomeipSdServerServiceInstanceConfig`"""
    priority: Optional[int]
    """get or set the priority of this `SomeipSdServerServiceInstanceConfig`
    
    Available since R21-11 (`AUTOSAR_00050`)"""
    def request_response_delay(self) -> Optional[RequestResponseDelay]:
        """get the request response delay of this `SomeipSdServerServiceInstanceConfig`"""
        ...

    def set_request_response_delay(
        self, request_response_delay: RequestResponseDelay, /
    ) -> None:
        """set the request response delay of this `SomeipSdServerServiceInstanceConfig`"""
        ...
    service_offer_time_to_live: Optional[int]
    """get or set the service offer time to live of this `SomeipSdServerServiceInstanceConfig`"""

@final
class SomeipTpChannel:
    """
    General settings for a `SomeIp` TP channel

    version >= `AUTOSAR_00046`
    """

    def __init__(self, element: Element) -> SomeipTpChannel: ...
    element: Element
    name: str
    rx_timeout_time: Optional[float]
    """set the rxTimeoutTime for the `SomeIpTpChannel`"""
    separation_time: Optional[float]
    """set the separationTime for the `SomeIpTpChannel`"""

@final
class SomeipTpConfig:
    """
    A `SomipTpConfig` contains the configuration of individual `SomeIp` TP connections
    """

    def __init__(self, element: Element) -> SomeipTpConfig: ...
    cluster: Optional[Union[CanCluster, FlexrayCluster, EthernetCluster]]
    """get the communication cluster of this `SomeipTpConfig`"""
    def create_someip_tp_channel(self, name: str, /) -> SomeipTpChannel:
        """create a new `SomeipTpChannel` in this `SomeipTpConfig`

        version >= `AUTOSAR_00046`"""
        ...

    def create_someip_tp_connection(
        self,
        tp_sdu: ISignalIPdu,
        transport_pdu_triggering: PduTriggering,
        /,
        *,
        tp_channel: Optional[SomeipTpChannel] = None,
    ) -> SomeipTpConnection:
        """create a new SomeIp TP connection in this `SomeipTpConfig`"""
        ...
    element: Element
    name: str
    def someip_tp_channels(self, /) -> Iterator[SomeipTpChannel]:
        """iterate over all `SomeipTpChannel`s in this `SomeipTpConfig`"""
        ...

    def someip_tp_connections(self, /) -> Iterator[SomeipTpConnection]:
        """get all `SomeipTpConnection`s in this `SomeipTpConfig`"""
        ...

@final
class SomeipTpConnection:
    """
    A `SomeipTpConnection` contains the configuration of a single `SomeIp` TP connection
    """

    def __init__(self, element: Element) -> SomeipTpConnection: ...
    element: Element
    someip_tp_config: SomeipTpConfig
    """get the `SomeipTpConfig` that contains this `SomeipTpConnection`"""
    tp_channel: Optional[SomeipTpChannel]
    """set the `TpChannel` of this `SomeipTpConnection`"""
    tp_sdu: Optional[ISignalIPdu]
    """set the `TpSdu` of this `SomeipTpConnection`"""
    transport_pdu_triggering: Optional[PduTriggering]
    """get or set the `PduTriggering` for the transport PDU of this `SomeipTpConnection`"""

@final
class StaticSocketConnection:
    """
    A static socket connection is a connection between two sockets.

    This is the new way to establish a connection. It was introduced in Autosar 4.5.0 (`AUTOSAR_00048`).
    """

    def __init__(self, element: Element) -> StaticSocketConnection: ...
    def add_ipdu_identifier(self, identifier: SoConIPduIdentifier, /) -> None:
        """add a `SoConIPduIdentifier` to this static socket connection"""
        ...
    element: Element
    def ipdu_identifiers(self, /) -> Iterator[SoConIPduIdentifier]:
        """create an iterator over all `SoConIPduIdentifiers` in this static socket connection"""
        ...
    name: str
    remote_socket: Optional[SocketAddress]
    """get or set the remote socket of this connection"""
    socket_address: SocketAddress
    """get the socket address containing this static socket connection"""
    tcp_connect_timeout: Optional[float]
    """get or set the TCP connect timeout of this static socket connection"""
    tcp_role: Optional[TcpRole]
    """get or set the TCP role of this static socket connection"""

@final
class SystemSignal:
    """
    The system signal represents the communication system's view of data exchanged between SW components which reside on different ECUs

    Use [`ArPackage::create_system_signal`] to create a new system signal
    """

    def __init__(self, element: Element) -> SystemSignal: ...
    compu_method: Optional[CompuMethod]
    """get or set the compu method for this signal"""
    data_constr: Optional[DataConstr]
    """get or set the data constraint for this signal"""
    element: Element
    name: str
    signal_group: Optional[SystemSignalGroup]
    """get the signal group that contains this signal"""
    unit: Optional[Unit]
    """get or set the unit for this signal"""

@final
class SystemSignalGroup:
    """
    A signal group refers to a set of signals that shall always be kept together. A signal group is used to
    guarantee the atomic transfer of AUTOSAR composite data types.

    Use [`ArPackage::create_system_signal_group`] to create a new system signal group
    """

    def __init__(self, element: Element) -> SystemSignalGroup: ...
    def add_signal(self, signal: SystemSignal, /) -> None:
        """Add a signal to the signal group"""
        ...
    element: Element
    name: str
    def signals(self, /) -> Iterator[ISignal]:
        """Iterate over all signals in the signal group"""
        ...

@final
class TcpRole:
    """
    The role of a TCP connection in a static socket connection can either be `Connect` (=client) or `Listen` (=server).
    """

    Connect: TcpRole
    Listen: TcpRole

@final
class TpAddress:
    """
    Represents an ECUs transport layer address on the referenced channel

    The `TpAddress` element is used by `FlexrayArTpConfig` and `FlexrayTpConfig`
    """

    def __init__(self, element: Element) -> TpAddress: ...
    address: Optional[int]
    """get or set the value of the address"""
    element: Element
    name: str

@final
class TpConfig:
    """
    transport protocol settings of a [`SocketAddress`]
    """

    @staticmethod
    def TcpTp(
        *,
        port_number: Optional[int] = None,
        port_dynamically_assigned: Optional[bool] = None,
    ) -> TpConfig_TcpTp: ...
    @staticmethod
    def UdpTp(
        *,
        port_number: Optional[int] = None,
        port_dynamically_assigned: Optional[bool] = None,
    ) -> TpConfig_UdpTp: ...

@final
class TpConfig_TcpTp(TpConfig):
    port_number: Optional[int]
    port_dynamically_assigned: Optional[bool]

@final
class TpConfig_UdpTp(TpConfig):
    port_number: Optional[int]
    port_dynamically_assigned: Optional[bool]

@final
class TransferProperty:
    """
    The `TransferProperty` defines if or how the signal influences the transfer of the PDU
    """

    Pending: TransferProperty
    Triggered: TransferProperty
    TriggeredOnChange: TransferProperty
    TriggeredOnChangeWithoutRepetition: TransferProperty
    TriggeredWithoutRepetition: TransferProperty

@final
class TransformationTechnology:
    """
    A `TransformationTechnology` describes how to transform signal or PDU data
    """

    def __init__(self, element: Element) -> TransformationTechnology: ...
    def config(self) -> Optional[TransformationTechnologyConfig]:
        """get the configuration of the `TransformationTechnology`"""
        ...

    def set_config(self, config: TransformationTechnologyConfig, /) -> None:
        """set the configuration of the `TransformationTechnology`"""
        ...
    data_transformation_set: Optional[DataTransformationSet]
    """get the `DataTransformationSet` that contains this `TransformationTechnology`"""
    element: Element
    name: str
    protocol: Optional[str]
    """Get the protocol of the `TransformationTechnology`. It can be set by replacing the whole config"""
    transformer_class: Optional[str]
    """Get the transformer class of the `TransformationTechnology`"""

@final
class TransmissionModeTiming:
    """
    Cyclic and event controlled timing parameters for an IPDU
    """

    def __init__(
        self,
        *,
        cyclic_timing: Optional[CyclicTiming] = None,
        event_controlled_timing: Optional[EventControlledTiming] = None,
    ) -> TransmissionModeTiming: ...
    cyclic_timing: Optional[CyclicTiming]
    """cyclic timing parameters"""
    event_controlled_timing: Optional[EventControlledTiming]
    """event controlled timing parameters"""

@final
class UdpNmCluster:
    """
    Udp / Ethernet specific `NmCluster`
    """

    def __init__(self, element: Element) -> UdpNmCluster: ...
    channel_sleep_master: Optional[bool]
    """get or set the nmChannelSleepMaster flag"""
    communication_cluster: Optional[EthernetCluster]
    """set the referenced `EthernetCluster`"""
    def create_udp_nm_node(
        self,
        name: str,
        controller: EthernetCommunicationController,
        nm_ecu: NmEcu,
        nm_msg_cycle_offset: float,
        /,
    ) -> UdpNmNode:
        """add a `UdpNmNode` to the cluster"""
        ...
    element: Element
    name: str
    nm_cbv_position: Optional[int]
    """get or set the value nmCbvPosition"""
    nm_immediate_nm_transmissions: Optional[int]
    """get or set the value nmImmediateNmTransmissions"""
    nm_message_timeout_time: Optional[float]
    """get or set the nmMessageTimeoutTime"""
    nm_msg_cycle_time: Optional[float]
    """get or set the nmMsgCycleTime"""
    nm_network_timeout: Optional[float]
    """get or set the `NmNetworkTimeout`"""
    nm_nid_position: Optional[int]
    """get or set the value nmNidPosition"""
    def nm_nodes(self, /) -> Iterator[UdpNmNode]:
        """iterate over all `NmNodes` in this cluster"""
        ...
    nm_remote_sleep_indication_time: Optional[float]
    """get or set the `NmRemoteSleepIndicationTime`"""
    nm_repeat_message_time: Optional[float]
    """get or set the `NmRepeatMessageTime`"""
    nm_wait_bus_sleep_time: Optional[float]
    """get or set the `NmWaitBusSleepTime`"""
    node_detection_enabled: Optional[bool]
    """get or set the nmNodeDetectionEnabled flag"""
    node_id_enabled: Optional[bool]
    """get or set the nmNodeIdEnabled flag"""
    pnc_cluster_vector_length: Optional[int]
    """get or set the pncClusterVectorLength"""
    pnc_participation: Optional[bool]
    """get or set the nmPncParticipation flag"""
    repeat_msg_ind_enabled: Optional[bool]
    """get or set the nmRepeatMsgIndEnabled flag"""
    synchronizing_network: Optional[bool]
    """get or set the nmSynchronizingNetwork flag"""
    vlan: Optional[EthernetPhysicalChannel]
    """get or set the Vlan associated with the cluster through an `EthernetPhysicalChannel` reference."""

@final
class UdpNmClusterCoupling:
    """
    Udp / Ethernet specific `NmClusterCoupling`

    It couples multiple `UdpNmCluster`s and provides UdpNm-specific settings
    """

    def __init__(self, element: Element) -> UdpNmClusterCoupling: ...
    def add_coupled_cluster(self, cluster: UdpNmCluster, /) -> None:
        """add a reference to a coupled `NmCluster`"""
        ...

    def coupled_clusters(self, /) -> Iterator[UdpNmCluster]:
        """iterate over all coupled `NmClusters`"""
        ...
    element: Element
    nm_immediate_restart_enabled: Optional[bool]
    """set or remove the nmImmediateRestartEnabled flag"""

@final
class UdpNmClusterSettings:
    """
    `UdpNmClusterSettings` encapsulates the mandatory settings for a `UdpNmCluster`
    """

    def __init__(
        self,
        *,
        nm_msg_cycle_time: float,
        nm_msg_timeout_time: float,
        nm_network_timeout: float,
        nm_remote_sleep_indication_time: float,
        nm_repeat_message_time: float,
        nm_wait_bus_sleep_time: float,
    ) -> UdpNmClusterSettings: ...
    nm_msg_cycle_time: float
    """Period of an `NmPdu` in seconds"""
    nm_msg_timeout_time: float
    """Timeout of a `NmPdu` in seconds"""
    nm_network_timeout: float
    """Network Timeout for `NmPdus` in seconds"""
    nm_remote_sleep_indication_time: float
    """Timeout for Remote Sleep Indication in seconds"""
    nm_repeat_message_time: float
    """Timeout for Repeat Message State in seconds"""
    nm_wait_bus_sleep_time: float
    """Timeout for bus calm down phase in seconds"""

@final
class UdpNmNode:
    """
    Udp / Ethernet specific `NmNode`
    """

    def __init__(self, element: Element) -> UdpNmNode: ...
    def add_rx_nm_pdu(self, /, nm_pdu: NmPdu) -> None:
        """add an Rx `NmPdu`

        Every `NmNode` must have at least one Rx `NmPdu`"""
        ...

    def add_tx_nm_pdu(self, /, nm_pdu: NmPdu) -> None:
        """add a Tx `NmPdu`

        Active `NmNodes` must have at least one Tx `NmPdu`, while passive `NmNodes` may have none.
        """
        ...
    all_nm_messages_keep_awake: Optional[bool]
    """set ot remove the allNmMessagesKeepAwake flag
    
    If `enabled` is `Some`, the flag is set to the value of `enabled`. If `enabled` is `None`, the flag is removed."""
    communication_controller: Optional[EthernetCommunicationController]
    """get or set the referenced `EthernetCommunicationController`"""
    element: Element
    name: str
    nm_ecu: Optional[NmEcu]
    """get or set the referenced `NmEcu`"""
    nm_msg_cycle_offset: Optional[float]
    """get or set the `NmMsgCycleOffset`"""
    node_id: Optional[int]
    """set the nmNodeId"""
    passive_mode: Optional[bool]
    """set ot remove the nmPassiveModeEnabled flag"""
    def rx_nm_pdus(self, /) -> Iterator[NmPdu]:
        """iterate over all RX `NmPdus`"""
        ...

    def tx_nm_pdus(self, /) -> Iterator[NmPdu]:
        """iterate over all TX `NmPdus`"""
        ...
    ...
