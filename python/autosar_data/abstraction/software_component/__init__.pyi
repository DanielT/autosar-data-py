# Stub file for autosar_data.abstraction.software_component

from typing import final, Iterator, List, Optional, Tuple, TypeAlias, Union
from autosar_data import Element
from autosar_data.abstraction.datatype import (
    AutosarDataType,
    DataTypeMappingSet,
    ValueSpecification,
)

PortInterface: TypeAlias = Union[
    SenderReceiverInterface,
    ClientServerInterface,
    ModeSwitchInterface,
    NvDataInterface,
    ParameterInterface,
    TriggerInterface,
]
PortPrototype: TypeAlias = Union[PPortPrototype, PRPortPrototype, RPortPrototype]
SwComponentType: TypeAlias = Union[
    ApplicationSwComponentType,
    ComplexDeviceDriverSwComponentType,
    CompositionSwComponentType,
    EcuAbstractionSwComponentType,
    SensorActuatorSwComponentType,
    ServiceSwComponentType,
]
RTEEvent: TypeAlias = Union[
    AsynchronousServerCallReturnsEvent,
    BackgroundEvent,
    DataReceiveErrorEvent,
    DataReceivedEvent,
    DataSendCompletedEvent,
    DataWriteCompletedEvent,
    ExternalTriggerOccurredEvent,
    InitEvent,
    InternalTriggerOccurredEvent,
    ModeSwitchedAckEvent,
    OperationInvokedEvent,
    OsTaskExecutionEvent,
    SwcModeManagerErrorEvent,
    SwcModeSwitchEvent,
    TimingEvent,
    TransformerHardErrorEvent,
]

@final
class ApplicationError:
    """
    An `ApplicationError` represents an error that can be returned by a client server operation
    """

    def __init__(self, element: Element, /) -> ApplicationError: ...
    element: Element
    error_code: int
    """the error code of the application error"""
    name: str

@final
class ApplicationSwComponentType:
    """
    An `ApplicationSwComponentType` is a software component that provides application functionality

    Use [`ArPackage::create_application_sw_component_type`] to create a new application sw component type.
    """

    def __init__(self, element: Element, /) -> ApplicationSwComponentType: ...
    def create_p_port(
        self, name: str, port_interface: PortInterface, /
    ) -> PPortPrototype:
        """create a new provided port with the given name and port interface"""
        ...

    def create_port_group(self, name: str, /) -> PortGroup:
        """create a new port group"""
        ...

    def create_pr_port(
        self, name: str, port_interface: PortInterface, /
    ) -> PRPortPrototype:
        """create a new provided required port with the given name and port interface"""
        ...

    def create_r_port(
        self, name: str, port_interface: PortInterface, /
    ) -> RPortPrototype:
        """create a new required port with the given name and port interface"""
        ...

    def create_swc_internal_behavior(self, name: str, /) -> SwcInternalBehavior:
        """create an SwcInternalBehavior for the component

        A component can have only one internal behavior, but since the internal behavior is a variation point,
        more than one internal behavior can be created. In this case the variation point settings must ensure that only one
        internal behavior is active."""
        ...
    element: Element
    def instances(self, /) -> List[SwComponentPrototype]:
        """list all instances of the component type"""
        ...
    name: str
    def parent_compositions(self, /) -> List[CompositionSwComponentType]:
        """list all compositions containing instances of the component type"""
        ...

    def ports(self, /) -> Iterator[PortPrototype]:
        """get an iterator over the ports of the component"""
        ...

    def swc_internal_behaviors(self, /) -> Iterator[SwcInternalBehavior]:
        """iterate over all swc internal behaviors - typically zero or one"""
        ...

@final
class ArgumentDataPrototype:
    """
    An `ArgumentDataPrototype` represents an argument in a `ClientServerOperation`
    """

    def __init__(self, element: Element, /) -> ArgumentDataPrototype: ...
    data_type: Optional[AutosarDataType]
    """data type of the argument"""
    direction: Optional[ArgumentDirection]
    """direction of the argument"""
    element: Element
    name: str

@final
class ArgumentDirection:
    """
    The `ArgumentDirection` defines the direction of an argument in a `ClientServerOperation`

    Input arguments are used to pass data from the client to the server and are usualy passed by value.
    Output arguments are used to pass data from the server to the client and are usually passed by reference.
    In/Out arguments are used to pass data in both directions and are usually passed by reference.
    """

    In: ArgumentDirection
    InOut: ArgumentDirection
    Out: ArgumentDirection

@final
class AssemblySwConnector:
    """
    An `AssemblySwConnector` connects ports of two `SwCompositionType`s.
    """

    def __init__(self, element: Element, /) -> AssemblySwConnector: ...
    element: Element
    name: str
    p_port: Optional[PortPrototype]
    """get the provided port of the assembly connector"""
    p_sw_component: Optional[SwComponentPrototype]
    """get the software component that contains the provided port of the assembly connector"""
    r_port: Optional[PortPrototype]
    """get the required port of the assembly connector"""
    r_sw_component: Optional[SwComponentPrototype]
    """get the software component that contains the required port of the assembly connector"""

@final
class AsynchronousServerCallReturnsEvent:
    """
    an asynchronous server call completed
    """

    def __init__(self, element: Element, /) -> AsynchronousServerCallReturnsEvent: ...
    element: Element
    name: str
    runnable_entity: Optional[RunnableEntity]
    """`RunnableEntity` that is triggered by the `AsynchronousServerCallCompleted`"""
    swc_internal_behavior: Optional[SwcInternalBehavior]
    """Get the `SwcInternalBehavior` that contains the event"""

@final
class BackgroundEvent:
    """
    starts a runnable for background processing at low priority
    """

    def __init__(self, element: Element, /) -> BackgroundEvent: ...
    element: Element
    name: str
    runnable_entity: Optional[RunnableEntity]
    """`RunnableEntity` that is triggered by the `AsynchronousServerCallCompleted`"""
    swc_internal_behavior: Optional[SwcInternalBehavior]
    """Get the `SwcInternalBehavior` that contains the event"""

@final
class ClientServerInterface:
    """
    A `ClientServerInterface` defines a set of operations that can be implemented by a server and called by a client

    Use [`ArPackage::create_client_server_interface`] to create a new client server interface
    """

    def __init__(self, element: Element, /) -> ClientServerInterface: ...
    def create_operation(self, name: str, /) -> ClientServerOperation:
        """add an operation to the client server interface"""
        ...

    def create_possible_error(self, name: str, error_code: int, /) -> ApplicationError:
        """Create a new `ClientServerInterface`
        Add a possible error to the client server interface"""
        ...
    element: Element
    name: str
    def operations(self, /) -> Iterator[ClientServerOperation]:
        """iterate over all operations"""
        ...

    def possible_errors(self, /) -> Iterator[ApplicationError]:
        """iterate over all application errors"""
        ...
    is_service: Optional[bool]
    """Get/Set if the client server interface is a service interface"""

@final
class ClientServerOperation:
    """
    A `ClientServerOperation` defines an operation in a `ClientServerInterface`
    """

    def __init__(self, element: Element, /) -> ClientServerOperation: ...
    def add_possible_error(self, error: ApplicationError, /) -> None:
        """add a reference to possible error to the operation"""
        ...

    def arguments(self, /) -> Iterator[ArgumentDataPrototype]:
        """iterate over all arguments"""
        ...

    def create_argument(
        self, name: str, data_type: AutosarDataType, direction: ArgumentDirection, /
    ) -> ArgumentDataPrototype:
        """Create a new `ClientServerOperation`
        Add an argument to the operation"""
        ...
    element: Element
    name: str
    def possible_errors(self, /) -> Iterator[ApplicationError]:
        """Get the possible errors of the operation"""
        ...

@final
class ComplexDeviceDriverSwComponentType:
    """
    A `ComplexDeviceDriverSwComponentType` is a software component that provides complex device driver functionality

    Use [`ArPackage::create_complex_device_driver_sw_component_type`] to create a new complex device driver sw component type.
    """

    def __init__(self, element: Element, /) -> ComplexDeviceDriverSwComponentType: ...
    def create_p_port(
        self, name: str, port_interface: PortInterface, /
    ) -> PPortPrototype:
        """create a new provided port with the given name and port interface"""
        ...

    def create_port_group(self, name: str, /) -> PortGroup:
        """create a new port group"""
        ...

    def create_pr_port(
        self, name: str, port_interface: PortInterface, /
    ) -> PRPortPrototype:
        """create a new provided required port with the given name and port interface"""
        ...

    def create_r_port(
        self, name: str, port_interface: PortInterface, /
    ) -> RPortPrototype:
        """create a new required port with the given name and port interface"""
        ...

    def create_swc_internal_behavior(self, name: str, /) -> SwcInternalBehavior:
        """create an SwcInternalBehavior for the component

        A component can have only one internal behavior, but since the internal behavior is a variation point,
        more than one internal behavior can be created. In this case the variation point settings must ensure that only one
        internal behavior is active."""
        ...
    element: Element
    def instances(self, /) -> List[SwComponentPrototype]:
        """list of all instances of the component type"""
        ...
    name: str
    def parent_compositions(self, /) -> List[CompositionSwComponentType]:
        """list all compositions containing instances of the component type"""
        ...

    def ports(self, /) -> Iterator[PortPrototype]:
        """get an iterator over the ports of the component"""
        ...

    def swc_internal_behaviors(self, /) -> Iterator[SwcInternalBehavior]:
        """iterate over all swc internal behaviors - typically zero or one"""
        ...

@final
class CompositionSwComponentType:
    """
    A `CompositionSwComponentType` is a software component that contains other software components

    Use [`ArPackage::create_composition_sw_component_type`] to create a new composition sw component type.
    """

    def __init__(self, element: Element, /) -> CompositionSwComponentType: ...
    def components(self, /) -> Iterator[SwComponentPrototype]:
        """get an iterator over the components of the composition"""
        ...

    def connectors(self, /) -> Iterator[AssemblySwConnector]:
        """iterate over all connectors"""
        ...

    def create_assembly_connector(
        self,
        name: str,
        port_1: PortPrototype,
        sw_prototype_1: SwComponentPrototype,
        port_2: PortPrototype,
        sw_prototype_2: SwComponentPrototype,
        /,
    ) -> AssemblySwConnector:
        """create a new delegation connector between an inner port and an outer port
        this is the actual implementation of the public method, but without the generic parameters
        create a new assembly connector between two ports of contained software components

        The two ports must be compatible."""
        ...

    def create_component(
        self, name: str, component_type: SwComponentType, /
    ) -> SwComponentPrototype:
        """create a component of type `component_type` in the composition

        It is not allowed to form cycles in the composition hierarchy, and this will return an error
        """
        ...

    def create_delegation_connector(
        self,
        name: str,
        inner_port: PortPrototype,
        inner_sw_prototype: SwComponentPrototype,
        outer_port: PortPrototype,
        /,
    ) -> DelegationSwConnector:
        """create a new delegation connector between an inner port and an outer port

        The two ports must be compatible."""
        ...

    def create_p_port(
        self, name: str, port_interface: PortInterface, /
    ) -> PPortPrototype:
        """create a new provided port with the given name and port interface"""
        ...

    def create_pass_through_connector(
        self, name: str, port_1: PortPrototype, port_2: PortPrototype, /
    ) -> PassThroughSwConnector:
        """create a new passthrough connector between two outer ports of the composition

        The two ports must be compatible."""
        ...

    def create_port_group(self, name: str, /) -> PortGroup:
        """create a new port group"""
        ...

    def create_pr_port(
        self, name: str, port_interface: PortInterface, /
    ) -> PRPortPrototype:
        """create a new provided required port with the given name and port interface"""
        ...

    def create_r_port(
        self, name: str, port_interface: PortInterface, /
    ) -> RPortPrototype:
        """create a new required port with the given name and port interface"""
        ...
    element: Element
    def instances(self, /) -> List[SwComponentPrototype]:
        """list of all instances of the component type"""
        ...

    def is_parent_of(self, other: SwComponentType, /) -> bool:
        """check if the composition is a parent (or grand-parent, etc.) of the component"""
        ...
    name: str
    def parent_compositions(self, /) -> Iterator[CompositionSwComponentType]:
        """iterator over all compositions containing instances of the component type"""
        ...

    def ports(self, /) -> Iterator[PortPrototype]:
        """get an iterator over the ports of the component"""
        ...

@final
class DataReceiveErrorEvent:
    """
    A `DataReceiveErrorEvent` is a subclass of `RTEEvent` which triggers a `RunnableEntity` when a data receive error occurs
    """

    def __init__(self, element: Element, /) -> DataReceiveErrorEvent: ...
    element: Element
    name: str
    runnable_entity: Optional[RunnableEntity]
    """`RunnableEntity` that is triggered by the `AsynchronousServerCallCompleted`"""
    swc_internal_behavior: Optional[SwcInternalBehavior]
    """Get the `SwcInternalBehavior` that contains the event"""

@final
class DataReceivedEvent:
    """
    A `DataReceivedEvent` is a subclass of `RTEEvent` which triggers a `RunnableEntity` when data is received
    """

    def __init__(self, element: Element, /) -> DataReceivedEvent: ...
    element: Element
    name: str
    runnable_entity: Optional[RunnableEntity]
    """`RunnableEntity` that is triggered by the `AsynchronousServerCallCompleted`"""
    swc_internal_behavior: Optional[SwcInternalBehavior]
    """Get the `SwcInternalBehavior` that contains the event"""
    def set_variable_data_prototype(
        self,
        variable_data_prototype: VariableDataPrototype,
        context_port: PPortPrototype,
        /,
    ) -> None:
        """Set the `VariableDataPrototype` that triggers the `DataReceivedEvent`"""
        ...
    variable_data_prototype: Optional[Tuple[VariableDataPrototype, PortPrototype]]
    """Get the `VariableDataPrototype` that triggers the `DataReceivedEvent`"""

@final
class DataSendCompletedEvent:
    """
    A `DataSendCompletedEvent` is a subclass of `RTEEvent` which triggers a `RunnableEntity` when data is sent
    """

    def __init__(self, element: Element, /) -> DataSendCompletedEvent: ...
    element: Element
    name: str
    runnable_entity: Optional[RunnableEntity]
    """`RunnableEntity` that is triggered by the `AsynchronousServerCallCompleted`"""
    swc_internal_behavior: Optional[SwcInternalBehavior]
    """Get the `SwcInternalBehavior` that contains the event"""

@final
class DataWriteCompletedEvent:
    """
    A `DataWriteCompletedEvent` is a subclass of `RTEEvent` which triggers a `RunnableEntity` when data is written
    """

    def __init__(self, element: Element, /) -> DataWriteCompletedEvent: ...
    element: Element
    name: str
    runnable_entity: Optional[RunnableEntity]
    """`RunnableEntity` that is triggered by the `AsynchronousServerCallCompleted`"""
    swc_internal_behavior: Optional[SwcInternalBehavior]
    """Get the `SwcInternalBehavior` that contains the event"""

@final
class DelegationSwConnector:
    """
    A `DelegationSwConnector` connects a port of a software component that is contained inside a `SwCompositionType` with a port of the `SwCompositionType`.
    """

    def __init__(self, element: Element, /) -> DelegationSwConnector: ...
    element: Element
    name: str
    inner_port: Optional[PortPrototype]
    """get the inner port of the delegation connector"""
    inner_sw_component: Optional[SwComponentPrototype]
    """get the software component that contains the inner port of the delegation connector"""
    outer_port: Optional[PortPrototype]
    """get the outer port of the delegation connector"""

@final
class EcuAbstractionSwComponentType:
    """
    The `EcuAbstractionSwComponentType` is a special `AtomicSwComponentType` that resides between a software-component
    that wants to access ECU periphery and the Microcontroller Abstraction

    Use [`ArPackage::create_ecu_abstraction_sw_component_type`] to create a new ECU abstraction sw component type.
    """

    def __init__(self, element: Element, /) -> EcuAbstractionSwComponentType: ...
    def create_p_port(
        self, name: str, port_interface: PortInterface, /
    ) -> PPortPrototype:
        """create a new provided port with the given name and port interface"""
        ...

    def create_port_group(self, name: str, /) -> PortGroup:
        """create a new port group"""
        ...

    def create_pr_port(
        self, name: str, port_interface: PortInterface, /
    ) -> PRPortPrototype:
        """create a new provided required port with the given name and port interface"""
        ...

    def create_r_port(
        self, name: str, port_interface: PortInterface, /
    ) -> RPortPrototype:
        """create a new required port with the given name and port interface"""
        ...

    def create_swc_internal_behavior(self, name: str, /) -> SwcInternalBehavior:
        """create an SwcInternalBehavior for the component

        A component can have only one internal behavior, but since the internal behavior is a variation point,
        more than one internal behavior can be created. In this case the variation point settings must ensure that only one
        internal behavior is active."""
        ...
    element: Element
    def instances(self, /) -> List[SwComponentPrototype]:
        """list all instances of the component type"""
        ...
    name: str
    def parent_compositions(self, /) -> List[CompositionSwComponentType]:
        """list all compositions containing instances of the component type"""
        ...

    def ports(self, /) -> Iterator[PortPrototype]:
        """get an iterator over the ports of the component"""
        ...

    def swc_internal_behaviors(self, /) -> Iterator[SwcInternalBehavior]:
        """iterate over all swc internal behaviors - typically zero or one"""
        ...

@final
class ExternalTriggerOccurredEvent:
    """
    A `ExternalTriggerOccurredEvent` is a subclass of `RTEEvent` which triggers a `RunnableEntity` when an external trigger occurs
    """

    def __init__(self, element: Element, /) -> ExternalTriggerOccurredEvent: ...
    element: Element
    name: str
    runnable_entity: Optional[RunnableEntity]
    """`RunnableEntity` that is triggered by the `AsynchronousServerCallCompleted`"""
    swc_internal_behavior: Optional[SwcInternalBehavior]
    """Get the `SwcInternalBehavior` that contains the event"""

@final
class InitEvent:
    """
    A `InitEvent` is a subclass of `RTEEvent` which triggers a `RunnableEntity` when the software component is initialized
    """

    def __init__(self, element: Element, /) -> InitEvent: ...
    element: Element
    name: str
    runnable_entity: Optional[RunnableEntity]
    """`RunnableEntity` that is triggered by the `AsynchronousServerCallCompleted`"""
    swc_internal_behavior: Optional[SwcInternalBehavior]
    """Get the `SwcInternalBehavior` that contains the event"""

@final
class InternalTriggerOccurredEvent:
    """
    A `InternalTriggerOccurredEvent` is a subclass of `RTEEvent` which triggers a `RunnableEntity` when an internal trigger occurs
    """

    def __init__(self, element: Element, /) -> InternalTriggerOccurredEvent: ...
    element: Element
    name: str
    runnable_entity: Optional[RunnableEntity]
    """`RunnableEntity` that is triggered by the `AsynchronousServerCallCompleted`"""
    swc_internal_behavior: Optional[SwcInternalBehavior]
    """Get the `SwcInternalBehavior` that contains the event"""

@final
class ModeAccessPoint:
    """
    A `ModeAccessPoint`provides the ability to access the current mode of a ModeDeclarationGroup
    """

    def __init__(self, element: Element, /) -> ModeAccessPoint: ...
    element: Element
    name: str
    runnable_entity: Optional[RunnableEntity]
    """Get the `RunnableEntity` that contains the `ModeAccessPoint`"""
    def set_mode_group(
        self, mode_group: ModeGroup, context_port: PortPrototype, /
    ) -> None:
        """Set the mode group and context port of the `ModeAccessPoint`"""
        ...
    mode_group: Optional[Tuple[ModeGroup, PortPrototype]]

@final
class ModeActivationKind:
    """
    Kind of mode switch condition used for activation of an event
    """

    OnEntry: ModeActivationKind
    """
    The mode is activated on entry to the mode.
    """
    OnExit: ModeActivationKind
    """
    The mode is activated on exit from the mode.
    """
    OnTransition: ModeActivationKind
    """
    The mode is activated on transition from the first mode to the second mode.
    """

@final
class ModeDeclaration:
    """
    A `ModeDeclaration` represents a mode declaration in a `ModeDeclarationGroup`
    """

    def __init__(self, element: Element, /) -> ModeDeclaration: ...
    element: Element
    name: str
    value: Optional[int]
    """value of the mode declaration, if any."""

@final
class ModeDeclarationGroup:
    """
    A `ModeDeclarationGroup` is a collection of mode declarations.
    """

    def __init__(self, element: Element, /) -> ModeDeclarationGroup: ...
    def create_mode_declaration(self, name: str, /) -> ModeDeclaration:
        """Create a new mode declaration in the group"""
        ...

    def mode_declarations(self, /) -> Iterator[ModeDeclaration]:
        """iterate over all mode declarations in the group"""
        ...
    element: Element
    name: str
    category: Optional[ModeDeclarationGroupCategory]
    """category of the mode declaration group"""
    initial_mode: Optional[ModeDeclaration]
    """initial mode of the mode declaration group, if any"""
    on_transition_value: Optional[int]
    """
    Value to be used when switching to the mode declaration group, if any.
    This is the onTransitionValue attribute of the mode declaration group.
    """

@final
class ModeDeclarationGroupCategory:
    """
    Category of mode declaration groupy, which defines the ordering of the modes in the group
    """

    AlphabeticOrder: ModeDeclarationGroupCategory
    """
    Alphabetic order of the modes in the group.
    """
    ExplicitOrder: ModeDeclarationGroupCategory
    """
    Ordering of modes in the mode declaration group is made explicit by the value, which must be set for each mode.
    Additonally, the on_transition_value attribute must be set in this case.
    """

@final
class ModeGroup:
    """
    A `ModeGroup` represents a mode group in a `ModeSwitchInterface`
    """

    def __init__(self, element: Element, /) -> ModeGroup: ...
    element: Element
    name: str
    mode_declaration_group: ModeDeclarationGroup
    """Get/Set the mode declaration group of the mode group"""

@final
class ModeSwitchInterface:
    """
    A `ModeSwitchInterface` defines a set of modes that can be switched

    Use [`ArPackage::create_mode_switch_interface`] to create a new mode switch interface
    """

    def __init__(self, element: Element, /) -> ModeSwitchInterface: ...
    element: Element
    name: str
    def create_mode_group(
        self, name: str, mode_declaration_group: ModeDeclarationGroup, /
    ) -> ModeGroup:
        """
        Create a new mode group in the mode switch interface
        The `ModeSwitchInterface` can only contain one mode group
        """
        ...
    mode_group: Optional[ModeGroup]
    """Get the mode group of the mode switch interface"""
    is_service: Optional[bool]
    """Get/Set if the mode switch interface is a service interface"""

@final
class ModeSwitchedAckEvent:
    """
    A `ModeSwitchedAckEvent` is a subclass of `RTEEvent` which triggers a `RunnableEntity` when a mode switch is acknowledged
    """

    def __init__(self, element: Element, /) -> ModeSwitchedAckEvent: ...
    element: Element
    name: str
    runnable_entity: Optional[RunnableEntity]
    """`RunnableEntity` that is triggered by the `AsynchronousServerCallCompleted`"""
    swc_internal_behavior: Optional[SwcInternalBehavior]
    """Get the `SwcInternalBehavior` that contains the event"""

@final
class ModeSwitchPoint:
    """
    A `ModeSwitchPoint` allows a `RunnableEntity` to switch modes in a ModeDeclarationGroup
    """

    def __init__(self, element: Element, /) -> ModeSwitchPoint: ...
    element: Element
    name: str
    runnable_entity: Optional[RunnableEntity]
    """Get the `RunnableEntity` that contains the `ModeSwitchPoint`"""
    def set_mode_group(
        self, mode_group: ModeGroup, context_port: PortPrototype, /
    ) -> None:
        """Set the mode group and context port of the `ModeSwitchPoint`"""
        ...
    mode_group: Optional[Tuple[ModeGroup, PortPrototype]]

@final
class NvDataInterface:
    """
    An `NvDataInterface` defines non-volatile data that can be accessed through the interface

    Use [`ArPackage::create_nv_data_interface`] to create a new non-volatile data interface
    """

    def __init__(self, element: Element, /) -> NvDataInterface: ...
    element: Element
    name: str
    is_service: Optional[bool]
    """Get/Set if the Nv-data interface is a service interface"""

@final
class OperationInvokedEvent:
    """
    A `OperationInvokedEvent` is a subclass of `RTEEvent` which triggers a `RunnableEntity` when an operation is invoked
    """

    def __init__(self, element: Element, /) -> OperationInvokedEvent: ...
    client_server_operation: Tuple[ClientServerOperation, PPortPrototype]
    """Get the `ClientServerOperation` that triggers the `OperationInvokedEvent`"""
    element: Element
    name: str
    runnable_entity: Optional[RunnableEntity]
    """`RunnableEntity` that is triggered by the `OperationInvokedEvent`"""
    def set_client_server_operation(
        self,
        client_server_operation: ClientServerOperation,
        context_p_port: PPortPrototype,
        /,
    ) -> None:
        """Set the `ClientServerOperation` that is triggers the `OperationInvokedEvent`"""
        ...
    swc_internal_behavior: Optional[SwcInternalBehavior]
    """`SwcInternalBehavior` that contains the event"""

@final
class OsTaskExecutionEvent:
    """
    A `OsTaskExecutionEvent` is a subclass of `RTEEvent` which triggers a `RunnableEntity` when an OS task is executed
    """

    def __init__(self, element: Element, /) -> OsTaskExecutionEvent: ...
    element: Element
    name: str
    runnable_entity: Optional[RunnableEntity]
    """`RunnableEntity` that is triggered by the `AsynchronousServerCallCompleted`"""
    swc_internal_behavior: Optional[SwcInternalBehavior]
    """Get the `SwcInternalBehavior` that contains the event"""

@final
class PPortPrototype:
    """
    `PPortPrototype` represents a provided port prototype
    """

    def __init__(self, element: Element, /) -> PPortPrototype: ...
    component_type: Optional[SwComponentType]
    """component type containing the port prototype"""
    element: Element
    name: str
    port_interface: Optional[PortInterface]
    """port interface of the port prototype"""

@final
class PRPortPrototype:
    """
    `PRPortPrototype` represents a provided and required port prototype
    """

    def __init__(self, element: Element, /) -> PRPortPrototype: ...
    component_type: Optional[SwComponentType]
    """component type containing the port prototype"""
    element: Element
    name: str
    port_interface: Optional[PortInterface]
    """port interface of the port prototype"""

@final
class ParameterDataPrototype:
    """
    A `ParameterDataPrototype` represents a parameter in a `ParameterInterface`
    """

    def __init__(self, element: Element, /) -> ParameterDataPrototype: ...
    # data_type: Optional[AutosarDataType]
    # """data type of the parameter"""
    element: Element
    name: str
    init_value: Optional[ValueSpecification]
    data_type: Optional[AutosarDataType]
    """data type of the parameter"""
    interface: Optional[SenderReceiverInterface]
    """Get the interface containing the parameter"""

@final
class ParameterInterface:
    """
    A `ParameterInterface` defines a set of parameters that can be accessed

    Use [`ArPackage::create_parameter_interface`] to create a new parameter interface
    """

    def __init__(self, element: Element, /) -> ParameterInterface: ...
    def create_parameter(
        self, name: str, data_type: AutosarDataType, /
    ) -> ParameterDataPrototype:
        """Add a new parameter to the parameter interface"""
        ...

    def parameters(self, /) -> Iterator[ParameterDataPrototype]:
        """iterate over all parameters"""
        ...
    element: Element
    name: str
    is_service: Optional[bool]
    """Get/Set if the parameter interface is a service interface"""

@final
class PassThroughSwConnector:
    """
    A `PassThroughSwConnector` connects two ports of a `SwCompositionType`.
    """

    def __init__(self, element: Element, /) -> PassThroughSwConnector: ...
    element: Element
    name: str
    p_port: Optional[PortPrototype]
    """get the provided port of the pass-through connector"""
    r_port: Optional[PortPrototype]
    """get the required port of the pass-through connector"""

@final
class PortGroup:
    """
    `PortGroup` represents a group of ports
    """

    def __init__(self, element: Element, /) -> PortGroup: ...
    element: Element
    name: str

@final
class RPortPrototype:
    """
    `RPortPrototype` represents a required port prototype
    """

    def __init__(self, element: Element, /) -> RPortPrototype: ...
    component_type: Optional[SwComponentType]
    """component type containing the port prototype"""
    element: Element
    name: str
    port_interface: Optional[PortInterface]
    """port interface of the port prototype"""

@final
class RootSwCompositionPrototype:
    """
    The `RootSwCompositionPrototype` is a special kind of `SwComponentPrototype` that represents the root of the composition hierarchy
    """

    def __init__(self, element: Element, /) -> RootSwCompositionPrototype: ...
    composition: Optional[CompositionSwComponentType]
    """composition that this root component is based on"""
    element: Element
    name: str

@final
class RunnableEntity:
    """
    A `RunnableEntity` is a function that can be executed by the RTE
    """

    def __init__(self, element: Element, /) -> RunnableEntity: ...
    element: Element
    def events(self, /) -> List[RTEEvent]:
        """Iterate over all events that can trigger the `RunnableEntity`"""
        ...
    name: str
    swc_internal_behavior: Optional[SwcInternalBehavior]
    """`SwcInternalBehavior` that contains the `RunnableEntity`"""
    def create_data_read_access(
        self, name: str, data_element: VariableDataPrototype, port: PortPrototype, /
    ) -> VariableAccess:
        """
        add implicit read access to a data element of a sender-receiver `PortPrototype`

        this results in `Rte_IRead_<port>_<data_element>` being generated
        """
        ...

    def data_read_accesses(self, /) -> Iterator[VariableAccess]:
        """iterate over all data read accesses of the runnable entity"""
        ...

    def create_data_write_access(
        self, name: str, data_element: VariableDataPrototype, port: PortPrototype, /
    ) -> VariableAccess:
        """
        add implicit write access to a data element of a sender-receiver `PortPrototype`

        this results in `Rte_IWrite_<port>_<data_element>` being generated
        """
        ...

    def data_write_accesses(self, /) -> Iterator[VariableAccess]:
        """iterate over all data write accesses of the runnable entity"""
        ...

    def create_data_send_point(
        self, name: str, data_element: VariableDataPrototype, port: PortPrototype, /
    ) -> VariableAccess:
        """
        add a data send point to a data element of a sender-receiver `PortPrototype`
        """
        ...

    def data_send_points(self, /) -> Iterator[VariableAccess]:
        """iterate over all data send points of the runnable entity"""
        ...

    def create_data_receive_point_by_argument(
        self, name: str, data_element: VariableDataPrototype, port: PortPrototype, /
    ) -> VariableAccess:
        """
        add explicit read access by argument to a data element of a sender-receiver `PortPrototype`
        """
        ...

    def data_receive_points_by_argument(self, /) -> Iterator[VariableAccess]:
        """iterate over all data receive points by argument of the runnable entity"""
        ...

    def create_data_receive_point_by_value(
        self, name: str, data_element: VariableDataPrototype, port: PortPrototype, /
    ) -> VariableAccess:
        """
        add explicit read access by value to a data element of a sender-receiver `PortPrototype`
        """
        ...

    def data_receive_points_by_value(self, /) -> Iterator[VariableAccess]:
        """iterate over all data receive points by value of the runnable entity"""
        ...

    def create_synchronous_server_call_point(
        self, name: str, operation: ClientServerOperation, port: PPortPrototype, /
    ) -> SynchronousServerCallPoint:
        """
        create a synchronous server call point that allows the runnable to call a server operation
        """
        ...

    def synchronous_server_call_points(self, /) -> Iterator[SynchronousServerCallPoint]:
        """iterate over all synchronous server call points of the runnable entity"""
        ...

    def create_mode_switch_point(
        self, name: str, mode_group: ModeGroup, context_port: PortPrototype, /
    ) -> ModeSwitchPoint:
        """
        create a mode switch point that allows the runnable to switch modes in a mode group
        """
        ...

    def mode_switch_points(self, /) -> Iterator[ModeSwitchPoint]:
        """iterate over all mode switch points of the runnable entity"""
        ...

    def create_mode_access_point(
        self, name: str, mode_group: ModeGroup, context_port: PortPrototype, /
    ) -> ModeAccessPoint:
        """
        create a mode access point that allows the runnable to access the current mode of a mode group
        """
        ...

    def mode_access_points(self, /) -> Iterator[ModeAccessPoint]:
        """iterate over all mode access points of the runnable entity"""
        ...

@final
class SenderReceiverInterface:
    """
    A `SenderReceiverInterface` defines a set of data elements that can be sent and received

    Use [`ArPackage::create_sender_receiver_interface`] to create a new sender receiver interface
    """

    def __init__(self, element: Element, /) -> SenderReceiverInterface: ...
    def create_data_element(
        self, name: str, data_type: AutosarDataType, /
    ) -> VariableDataPrototype:
        """Add a new data element to the sender receiver interface"""
        ...

    def data_elements(self, /) -> Iterator[VariableDataPrototype]:
        """iterate over all data elements"""
        ...
    element: Element
    name: str
    is_service: Optional[bool]
    """Get/Set if the sender/receiver interface is a service interface"""

@final
class SensorActuatorSwComponentType:
    """
    `SensorActuatorSwComponentType` is used to connect sensor/acutator devices to the ECU configuration

    Use [`ArPackage::create_sensor_actuator_sw_component_type`] to create a new sensor/actuator sw component type.
    """

    def __init__(self, element: Element, /) -> SensorActuatorSwComponentType: ...
    def create_p_port(
        self, name: str, port_interface: PortInterface, /
    ) -> PPortPrototype:
        """create a new provided port with the given name and port interface"""
        ...

    def create_port_group(self, name: str, /) -> PortGroup:
        """create a new port group"""
        ...

    def create_pr_port(
        self, name: str, port_interface: PortInterface, /
    ) -> PRPortPrototype:
        """create a new provided required port with the given name and port interface"""
        ...

    def create_r_port(
        self, name: str, port_interface: PortInterface, /
    ) -> RPortPrototype:
        """create a new required port with the given name and port interface"""
        ...

    def create_swc_internal_behavior(self, name: str, /) -> SwcInternalBehavior:
        """create an SwcInternalBehavior for the component

        A component can have only one internal behavior, but since the internal behavior is a variation point,
        more than one internal behavior can be created. In this case the variation point settings must ensure that only one
        internal behavior is active."""
        ...
    element: Element
    def instances(self, /) -> List[SwComponentPrototype]:
        """list all instances of the component type"""
        ...
    name: str
    def parent_compositions(self, /) -> List[CompositionSwComponentType]:
        """list all compositions containing instances of the component type"""
        ...

    def ports(self, /) -> Iterator[PortPrototype]:
        """get an iterator over the ports of the component"""
        ...

    def swc_internal_behaviors(self, /) -> Iterator[SwcInternalBehavior]:
        """iterate over all swc internal behaviors - typically zero or one"""
        ...

@final
class ServiceSwComponentType:
    """
    `ServiceSwComponentType` is used for configuring services for a given ECU. Instances of this class should only
    be created in ECU Configuration phase for the specific purpose of the service configuration.

    Use [`ArPackage::create_service_sw_component_type`] to create a new service sw component type.
    """

    def __init__(self, element: Element, /) -> ServiceSwComponentType: ...
    def create_p_port(
        self, name: str, port_interface: PortInterface, /
    ) -> PPortPrototype:
        """create a new provided port with the given name and port interface"""
        ...

    def create_port_group(self, name: str, /) -> PortGroup:
        """create a new port group"""
        ...

    def create_pr_port(
        self, name: str, port_interface: PortInterface, /
    ) -> PRPortPrototype:
        """create a new provided required port with the given name and port interface"""
        ...

    def create_r_port(
        self, name: str, port_interface: PortInterface, /
    ) -> RPortPrototype:
        """create a new required port with the given name and port interface"""
        ...

    def create_swc_internal_behavior(self, name: str, /) -> SwcInternalBehavior:
        """create an SwcInternalBehavior for the component

        A component can have only one internal behavior, but since the internal behavior is a variation point,
        more than one internal behavior can be created. In this case the variation point settings must ensure that only one
        internal behavior is active."""
        ...
    element: Element
    def instances(self, /) -> List[SwComponentPrototype]:
        """list all instances of the component type"""
        ...
    name: str
    def parent_compositions(self, /) -> List[CompositionSwComponentType]:
        """list all compositions containing instances of the component type"""
        ...

    def ports(self, /) -> Iterator[PortPrototype]:
        """get an iterator over the ports of the component"""
        ...

    def swc_internal_behaviors(self, /) -> Iterator[SwcInternalBehavior]:
        """iterate over all swc internal behaviors - typically zero or one"""
        ...

@final
class SwComponentPrototype:
    """
    A `SwComponentPrototype` is an instance of a software component type
    """

    def __init__(self, element: Element, /) -> SwComponentPrototype: ...
    element: Element
    name: str

@final
class SwcInternalBehavior:
    """
    The `SwcInternalBehavior` of a software component type describes the
    details that are needed to generate the RTE.
    """

    def __init__(self, element: Element, /) -> SwcInternalBehavior: ...
    def add_data_type_mapping_set(
        self, data_type_mapping_set: DataTypeMappingSet, /
    ) -> None:
        """Add a reference to a `DataTypeMappingSet` to the `SwcInternalBehavior`"""
        ...

    def create_background_event(
        self, name: str, runnable: RunnableEntity, /
    ) -> BackgroundEvent:
        """Create a new `BackgroundEvent` in the `SwcInternalBehavior` that triggers a runnable at low priority"""
        ...

    def create_init_event(self, name: str, runnable: RunnableEntity, /) -> InitEvent:
        """Create a new `InitEvent` in the `SwcInternalBehavior`"""
        ...

    def create_operation_invoked_event(
        self,
        name: str,
        runnable: RunnableEntity,
        client_server_operation: ClientServerOperation,
        context_p_port: PPortPrototype,
        /,
    ) -> OperationInvokedEvent:
        """Create a new `OperationInvokedEvent` in the `SwcInternalBehavior`"""
        ...

    def create_os_task_execution_event(
        self, name: str, runnable: RunnableEntity, /
    ) -> OsTaskExecutionEvent:
        """Create a new `OsTaskExecutionEvent` in the `SwcInternalBehavior` that triggers a runnable when an OS task is executed"""
        ...

    def create_runnable_entity(self, name: str, /) -> RunnableEntity:
        """Create a new RunnableEntity in the SwcInternalBehavior"""
        ...

    def create_timing_event(
        self, name: str, runnable: RunnableEntity, period: float, /
    ) -> TimingEvent:
        """Create a timing event that triggers a runnable in the `SwcInternalBehavior`"""
        ...

    def data_type_mapping_sets(self, /) -> Iterator[DataTypeMappingSet]:
        """iterator over all `DataTypeMappingSet` references in the `SwcInternalBehavior`"""
        ...
    element: Element
    def events(self, /) -> Iterator[RTEEvent]:
        """create an iterator over all events in the `SwcInternalBehavior`"""
        ...
    name: str
    def runnable_entities(self, /) -> Iterator[RunnableEntity]:
        """Get an iterator over all RunnableEntities in the SwcInternalBehavior"""
        ...
    sw_component_type: Optional[SwComponentType]
    """software component type that contains the `SwcInternalBehavior`"""
    def create_mode_switch_event(
        self,
        name: str,
        runnable: RunnableEntity,
        activation: ModeActivationKind,
        context_port: PortPrototype,
        mode_declaration: ModeDeclaration,
        /,
        second_mode_declaration: Optional[ModeDeclaration] = None,
    ) -> SwcModeSwitchEvent:
        """create a mode switch event that triggers a runnable in the `SwcInternalBehavior` when the mode is switched"""
        ...

    def create_data_received_event(
        self,
        name: str,
        runnable: RunnableEntity,
        variable_data_prototype: VariableDataPrototype,
        context_port: PortPrototype,
        /,
    ) -> DataReceivedEvent:
        """Create a new `DataReceivedEvent` in the `SwcInternalBehavior` that triggers a runnable when data is received"""
        ...

@final
class SwcModeManagerErrorEvent:
    """
    A `SwcModeManagerErrorEvent` is a subclass of `RTEEvent` which triggers a `RunnableEntity` when a mode manager error occurs
    """

    def __init__(self, element: Element, /) -> SwcModeManagerErrorEvent: ...
    element: Element
    name: str
    runnable_entity: Optional[RunnableEntity]
    """`RunnableEntity` that is triggered by the `AsynchronousServerCallCompleted`"""
    swc_internal_behavior: Optional[SwcInternalBehavior]
    """Get the `SwcInternalBehavior` that contains the event"""

@final
class SwcModeSwitchEvent:
    """
    A `SwcModeSwitchEvent` is a subclass of `RTEEvent` which triggers a `RunnableEntity` when a mode switch occurs
    """

    def __init__(self, element: Element, /) -> SwcModeSwitchEvent: ...
    element: Element
    name: str
    runnable_entity: Optional[RunnableEntity]
    """`RunnableEntity` that is triggered by the `AsynchronousServerCallCompleted`"""
    swc_internal_behavior: Optional[SwcInternalBehavior]
    """Get the `SwcInternalBehavior` that contains the event"""
    mode_activation_kind: Optional[ModeActivationKind]
    """Get/Set the mode activation kind of the `SwcModeSwitchEvent`"""
    def set_mode_declaration(
        self,
        ontext_port: PortPrototype,
        mode_declaration: ModeDeclaration,
        /,
        second_mode_declaration: Optional[ModeDeclaration] = None,
    ) -> None:
        """
        Set the mode declaration within a context port that triggers the `SwcModeSwitchEvent`

        The second mode must be provided if the activation kind `OnTransition` is configured.
        In that case only transitions between the two modes trigger the event.
        """
        ...

    def mode_declarations(self) -> Optional[List[ModeDeclaration]]:
        """
        Get the mode declarations that trigger the `SwcModeSwitchEvent`

        The list contains one or two mode declarations, depending on the activation kind.
        If the activation kind is `OnTransition`, the list contains two mode declarations.
        Otherwise, it contains one mode declaration.
        """
        ...

@final
class SynchronousServerCallPoint:
    """
    A `SynchronousServerCallPoint` allows a `RunnableEntity` to call a server operation synchronously
    """

    def __init__(self, element: Element, /) -> SynchronousServerCallPoint: ...
    element: Element
    name: str
    def set_client_server_operation(
        self,
        client_server_operation: ClientServerOperation,
        context_p_port: PPortPrototype,
        /,
    ) -> None:
        """Set the `ClientServerOperation` that is called by the `SynchronousServerCallPoint`"""
        ...
    client_server_operation: Optional[Tuple[ClientServerOperation, PPortPrototype]]
    """Get the `ClientServerOperation` that is called by the `SynchronousServerCallPoint`"""
    runnable_entity: Optional[RunnableEntity]
    """`RunnableEntity` that contains the `SynchronousServerCallPoint`"""

@final
class TimingEvent:
    """
    A `TimingEvent` is a subclass of `RTEEvent` which triggers a `RunnableEntity` periodically
    """

    def __init__(self, element: Element, /) -> TimingEvent: ...
    element: Element
    name: str
    period: Optional[float]
    """period of the `TimingEvent`"""
    runnable_entity: Optional[RunnableEntity]
    """`RunnableEntity` that is triggered by the `AsynchronousServerCallCompleted`"""
    swc_internal_behavior: Optional[SwcInternalBehavior]
    """Get the `SwcInternalBehavior` that contains the event"""

@final
class TransformerHardErrorEvent:
    """
    A `TransformerHardErrorEvent` is a subclass of `RTEEvent` which triggers a `RunnableEntity` when a transformer hard error occurs
    """

    def __init__(self, element: Element, /) -> TransformerHardErrorEvent: ...
    element: Element
    """element of the `TransformerHardErrorEvent`"""
    name: str
    """name of the `TransformerHardErrorEvent`"""
    runnable_entity: Optional[RunnableEntity]
    """`RunnableEntity` that is triggered by the `AsynchronousServerCallCompleted`"""
    swc_internal_behavior: Optional[SwcInternalBehavior]
    """Get the `SwcInternalBehavior` that contains the event"""

@final
class TriggerInterface:
    """
    A `TriggerInterface` declares a number of triggers that can be sent by an trigger source

    Use [`ArPackage::create_trigger_interface`] to create a new trigger interface
    """

    def __init__(self, element: Element, /) -> TriggerInterface: ...
    element: Element
    name: str
    is_service: Optional[bool]
    """Get/Set if the trigger interface is a service interface"""

@final
class VariableAccess:
    """
    A `VariableAccess` allows a `RunnableEntity` to access a variable in various contexts
    """

    def __init__(self, element: Element, /) -> VariableAccess: ...
    element: Element
    name: str
    def set_accessed_variable(
        self, variable: VariableDataPrototype, context_port: PortPrototype, /
    ) -> None:
        """Set the variable that is accessed by the `VariableAccess`"""
        ...
    accessed_variable: Optional[Tuple[VariableDataPrototype, PortPrototype]]
    """Get the variable that is accessed by the `VariableAccess`"""
    runnable_entity: Optional[RunnableEntity]
    """`RunnableEntity` that contains the `VariableAccess`"""

@final
class VariableDataPrototype:
    """
    A `VariableDataPrototype` represents a data element in a `SenderReceiverInterface`
    """

    def __init__(self, element: Element, /) -> VariableDataPrototype: ...
    data_type: Optional[AutosarDataType]
    """data type of the data element"""
    element: Element
    interface: Optional[SenderReceiverInterface]
    """Get the interface containing the data element"""
    name: str
    ...
    init_value: Optional[ValueSpecification]
    """
    initial value of the data element, if any
    """
