from autosar_data.abstraction import *
from autosar_data.abstraction.datatype import *
from autosar_data.abstraction.software_component import *


def test_composition_sw_component_type() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    # CompositionSwComponentType
    sw_component_type = package.create_composition_sw_component_type(
        "CompositionSwComponentType"
    )
    assert isinstance(sw_component_type, CompositionSwComponentType)
    # get and set the name
    assert sw_component_type.name == "CompositionSwComponentType"
    sw_component_type.name = "CompositionSwComponentType_modified"
    assert sw_component_type.name == "CompositionSwComponentType_modified"

    # create some components in the composition
    sub_component_type = package.create_composition_sw_component_type(
        "CompositionSwComponentType2"
    )
    sub_composition = sw_component_type.create_component(
        "SubComponent", sub_component_type
    )
    assert list(sub_component_type.instances()) == [sub_composition]

    app_component_type = package.create_application_sw_component_type(
        "ApplicationSwComponentType"
    )
    app_component = sw_component_type.create_component(
        "AppComponent", app_component_type
    )
    app_component2 = sw_component_type.create_component(
        "AppComponent2", app_component_type
    )

    assert sw_component_type.is_parent_of(sub_component_type)
    assert list(sw_component_type.components()) == [
        sub_composition,
        app_component,
        app_component2,
    ]

    # create an interface and some ports
    sr_interface = package.create_sender_receiver_interface("SRInterface")
    component_r_port = sw_component_type.create_r_port("RPort", sr_interface)
    component_p_port = sw_component_type.create_p_port("PPort", sr_interface)
    component_pr_port = sw_component_type.create_pr_port("PRPort", sr_interface)
    assert list(sw_component_type.ports()) == [
        component_r_port,
        component_p_port,
        component_pr_port,
    ]
    assert component_r_port.component_type == sw_component_type
    assert component_p_port.component_type == sw_component_type
    assert component_pr_port.component_type == sw_component_type

    assert list(sub_component_type.parent_compositions()) == [sw_component_type]

    sub_component_r_port = sub_component_type.create_r_port("RPort", sr_interface)
    delegation_connector = sw_component_type.create_delegation_connector(
        "DelegationConnector", sub_component_r_port, sub_composition, component_r_port
    )

    app_component_p_port = app_component_type.create_p_port("PPort", sr_interface)
    app_component_r_port = app_component_type.create_r_port("RPort", sr_interface)
    # create an assembly connector, connecting app_component.p_port to app_component2.r_port
    assembly_connector = sw_component_type.create_assembly_connector(
        "AssemblyConnector",
        app_component_p_port,
        app_component,
        app_component_r_port,
        app_component2,
    )

    passthrough_connector = sw_component_type.create_pass_through_connector(
        "PassthroughConnector", component_p_port, component_r_port
    )
    passthrough_connector2 = sw_component_type.create_pass_through_connector(
        "PassthroughConnector2", component_pr_port, component_r_port
    )

    assert list(sw_component_type.connectors()) == [
        delegation_connector,
        assembly_connector,
        passthrough_connector,
        passthrough_connector2,
    ]

    port_group = sw_component_type.create_port_group("PortGroup")
    assert isinstance(port_group, PortGroup)

    # check if the sw component type can be constructed from an element and is equal to the original one
    element = sw_component_type.element
    sw_component_type_copy = CompositionSwComponentType(element)
    assert sw_component_type == sw_component_type_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in CompositionSwComponentType.__dict__
    assert sw_component_type.__repr__()


def test_application_sw_component_type() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    # ApplicationSwComponentType
    sw_component_type = package.create_application_sw_component_type(
        "ApplicationSwComponentType"
    )
    assert isinstance(sw_component_type, ApplicationSwComponentType)
    # get and set the name
    assert sw_component_type.name == "ApplicationSwComponentType"
    sw_component_type.name = "ApplicationSwComponentType_modified"
    assert sw_component_type.name == "ApplicationSwComponentType_modified"

    # create some ports
    sr_interface = package.create_sender_receiver_interface("SRInterface")
    r_port = sw_component_type.create_r_port("RPort", sr_interface)
    p_port = sw_component_type.create_p_port("PPort", sr_interface)
    pr_port = sw_component_type.create_pr_port("PRPort", sr_interface)
    assert list(sw_component_type.ports()) == [r_port, p_port, pr_port]
    assert r_port.component_type == sw_component_type
    assert p_port.component_type == sw_component_type
    assert pr_port.component_type == sw_component_type

    # create a composition and add the sw component type as a sub-component
    composition = package.create_composition_sw_component_type("Composition")
    sw_component = composition.create_component("SwComponent", sw_component_type)
    assert list(sw_component_type.parent_compositions()) == [composition]
    assert list(sw_component_type.instances()) == [sw_component]
    sw_component_type.create_port_group("PortGroup")

    internal_behavior = sw_component_type.create_swc_internal_behavior(
        "InternalBehavior"
    )
    assert list(sw_component_type.swc_internal_behaviors()) == [internal_behavior]

    # check if the sw component type can be constructed from an element and is equal to the original one
    element = sw_component_type.element
    sw_component_type_copy = ApplicationSwComponentType(element)
    assert sw_component_type == sw_component_type_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ApplicationSwComponentType.__dict__
    assert sw_component_type.__repr__()


def test_service_sw_component_type() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    # ServiceSwComponentType
    sw_component_type = package.create_service_sw_component_type(
        "ServiceSwComponentType"
    )
    assert isinstance(sw_component_type, ServiceSwComponentType)
    # get and set the name
    assert sw_component_type.name == "ServiceSwComponentType"
    sw_component_type.name = "ServiceSwComponentType_modified"
    assert sw_component_type.name == "ServiceSwComponentType_modified"

    # create some ports
    sr_interface = package.create_sender_receiver_interface("SRInterface")
    r_port = sw_component_type.create_r_port("RPort", sr_interface)
    p_port = sw_component_type.create_p_port("PPort", sr_interface)
    pr_port = sw_component_type.create_pr_port("PRPort", sr_interface)
    assert list(sw_component_type.ports()) == [r_port, p_port, pr_port]
    assert r_port.component_type == sw_component_type
    assert p_port.component_type == sw_component_type
    assert pr_port.component_type == sw_component_type

    # create a composition and add the sw component type as a sub-component
    composition = package.create_composition_sw_component_type("Composition")
    sw_component = composition.create_component("SwComponent", sw_component_type)
    assert list(sw_component_type.parent_compositions()) == [composition]
    assert list(sw_component_type.instances()) == [sw_component]
    sw_component_type.create_port_group("PortGroup")

    internal_behavior = sw_component_type.create_swc_internal_behavior(
        "InternalBehavior"
    )
    assert list(sw_component_type.swc_internal_behaviors()) == [internal_behavior]

    # check if the sw component type can be constructed from an element and is equal to the original one
    element = sw_component_type.element
    sw_component_type_copy = ServiceSwComponentType(element)
    assert sw_component_type == sw_component_type_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ServiceSwComponentType.__dict__
    assert sw_component_type.__repr__()


def test_complex_device_driver_sw_component_type() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    # ComplexDeviceDriverSwComponentType
    sw_component_type = package.create_complex_device_driver_sw_component_type(
        "ComplexDeviceDriverSwComponentType"
    )
    assert isinstance(sw_component_type, ComplexDeviceDriverSwComponentType)
    # get and set the name
    assert sw_component_type.name == "ComplexDeviceDriverSwComponentType"
    sw_component_type.name = "ComplexDeviceDriverSwComponentType_modified"
    assert sw_component_type.name == "ComplexDeviceDriverSwComponentType_modified"

    # create some ports
    sr_interface = package.create_sender_receiver_interface("SRInterface")
    r_port = sw_component_type.create_r_port("RPort", sr_interface)
    p_port = sw_component_type.create_p_port("PPort", sr_interface)
    pr_port = sw_component_type.create_pr_port("PRPort", sr_interface)
    assert list(sw_component_type.ports()) == [r_port, p_port, pr_port]
    assert r_port.component_type == sw_component_type
    assert p_port.component_type == sw_component_type
    assert pr_port.component_type == sw_component_type

    # create a composition and add the sw component type as a sub-component
    composition = package.create_composition_sw_component_type("Composition")
    sw_component = composition.create_component("SwComponent", sw_component_type)
    assert list(sw_component_type.parent_compositions()) == [composition]
    assert list(sw_component_type.instances()) == [sw_component]
    sw_component_type.create_port_group("PortGroup")

    internal_behavior = sw_component_type.create_swc_internal_behavior(
        "InternalBehavior"
    )
    assert list(sw_component_type.swc_internal_behaviors()) == [internal_behavior]

    # check if the sw component type can be constructed from an element and is equal to the original one
    element = sw_component_type.element
    sw_component_type_copy = ComplexDeviceDriverSwComponentType(element)
    assert sw_component_type == sw_component_type_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ComplexDeviceDriverSwComponentType.__dict__
    assert sw_component_type.__repr__()


def test_sensor_actuator_sw_component_type() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    # SensorActuatorSwComponentType
    sw_component_type = package.create_sensor_actuator_sw_component_type(
        "SensorActuatorSwComponentType"
    )
    assert isinstance(sw_component_type, SensorActuatorSwComponentType)
    # get and set the name
    assert sw_component_type.name == "SensorActuatorSwComponentType"
    sw_component_type.name = "SensorActuatorSwComponentType_modified"
    assert sw_component_type.name == "SensorActuatorSwComponentType_modified"

    # create some ports
    sr_interface = package.create_sender_receiver_interface("SRInterface")
    r_port = sw_component_type.create_r_port("RPort", sr_interface)
    p_port = sw_component_type.create_p_port("PPort", sr_interface)
    pr_port = sw_component_type.create_pr_port("PRPort", sr_interface)
    assert list(sw_component_type.ports()) == [r_port, p_port, pr_port]
    assert r_port.component_type == sw_component_type
    assert p_port.component_type == sw_component_type
    assert pr_port.component_type == sw_component_type

    # create a composition and add the sw component type as a sub-component
    composition = package.create_composition_sw_component_type("Composition")
    sw_component = composition.create_component("SwComponent", sw_component_type)
    assert list(sw_component_type.parent_compositions()) == [composition]
    assert list(sw_component_type.instances()) == [sw_component]
    sw_component_type.create_port_group("PortGroup")

    internal_behavior = sw_component_type.create_swc_internal_behavior(
        "InternalBehavior"
    )
    assert list(sw_component_type.swc_internal_behaviors()) == [internal_behavior]

    # check if the sw component type can be constructed from an element and is equal to the original one
    element = sw_component_type.element
    sw_component_type_copy = SensorActuatorSwComponentType(element)
    assert sw_component_type == sw_component_type_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in SensorActuatorSwComponentType.__dict__
    assert sw_component_type.__repr__()


def test_ecu_abstraction_sw_component_type() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    # EcuAbstractionSwComponentType
    sw_component_type = package.create_ecu_abstraction_sw_component_type(
        "EcuAbstractionSwComponentType"
    )
    assert isinstance(sw_component_type, EcuAbstractionSwComponentType)
    # get and set the name
    assert sw_component_type.name == "EcuAbstractionSwComponentType"
    sw_component_type.name = "EcuAbstractionSwComponentType_modified"
    assert sw_component_type.name == "EcuAbstractionSwComponentType_modified"

    # create some ports
    sr_interface = package.create_sender_receiver_interface("SRInterface")
    r_port = sw_component_type.create_r_port("RPort", sr_interface)
    p_port = sw_component_type.create_p_port("PPort", sr_interface)
    pr_port = sw_component_type.create_pr_port("PRPort", sr_interface)
    assert list(sw_component_type.ports()) == [r_port, p_port, pr_port]
    assert r_port.component_type == sw_component_type
    assert p_port.component_type == sw_component_type
    assert pr_port.component_type == sw_component_type

    # create a composition and add the sw component type as a sub-component
    composition = package.create_composition_sw_component_type("Composition")
    sw_component = composition.create_component("SwComponent", sw_component_type)
    assert list(sw_component_type.parent_compositions()) == [composition]
    assert list(sw_component_type.instances()) == [sw_component]
    sw_component_type.create_port_group("PortGroup")

    internal_behavior = sw_component_type.create_swc_internal_behavior(
        "InternalBehavior"
    )
    assert list(sw_component_type.swc_internal_behaviors()) == [internal_behavior]

    # check if the sw component type can be constructed from an element and is equal to the original one
    element = sw_component_type.element
    sw_component_type_copy = EcuAbstractionSwComponentType(element)
    assert sw_component_type == sw_component_type_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EcuAbstractionSwComponentType.__dict__
    assert sw_component_type.__repr__()


def test_sw_component_prototype() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    sw_component_type = package.create_application_sw_component_type(
        "ApplicationSwComponentType"
    )

    # SwComponentPrototype
    composition = package.create_composition_sw_component_type("Composition")
    sw_component_prototype = composition.create_component(
        "SwComponent", sw_component_type
    )
    # get and set the name
    assert sw_component_prototype.name == "SwComponent"
    sw_component_prototype.name = "SwComponent_modified"
    assert sw_component_prototype.name == "SwComponent_modified"

    # check if the sw component prototype can be constructed from an element and is equal to the original one
    element = sw_component_prototype.element
    sw_component_prototype_copy = SwComponentPrototype(element)
    assert sw_component_prototype == sw_component_prototype_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in SwComponentPrototype.__dict__
    assert sw_component_prototype.__repr__()


def test_root_sw_composition_prototype() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    system = package.create_system("System", SystemCategory.EcuExtract)

    # RootSwCompositionPrototype
    composition_type = package.create_composition_sw_component_type("Composition")
    root_sw_composition_prototype = system.set_root_sw_composition(
        "RootSwCompositionPrototype", composition_type
    )
    assert isinstance(root_sw_composition_prototype, RootSwCompositionPrototype)

    # get and set the name
    assert root_sw_composition_prototype.name == "RootSwCompositionPrototype"
    root_sw_composition_prototype.name = "RootSwCompositionPrototype_modified"
    assert root_sw_composition_prototype.name == "RootSwCompositionPrototype_modified"

    # check if the root sw composition prototype can be constructed from an element and is equal to the original one
    element = root_sw_composition_prototype.element
    root_sw_composition_prototype_copy = RootSwCompositionPrototype(element)
    assert root_sw_composition_prototype == root_sw_composition_prototype_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in RootSwCompositionPrototype.__dict__
    assert root_sw_composition_prototype.__repr__()


def test_delegation_sw_connector() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    sw_component_type = package.create_composition_sw_component_type(
        "CompositionSwComponentType"
    )
    sub_component_type = package.create_composition_sw_component_type(
        "CompositionSwComponentType2"
    )
    sub_composition = sw_component_type.create_component(
        "SubComponent", sub_component_type
    )
    sr_interface = package.create_sender_receiver_interface("SRInterface")

    # DelegationSwConnector
    component_r_port = sw_component_type.create_r_port("RPort", sr_interface)

    sub_component_r_port = sub_component_type.create_r_port("RPort", sr_interface)
    delegation_connector = sw_component_type.create_delegation_connector(
        "DelegationConnector", sub_component_r_port, sub_composition, component_r_port
    )
    assert isinstance(delegation_connector, DelegationSwConnector)

    # get and set the name
    assert delegation_connector.name == "DelegationConnector"
    delegation_connector.name = "DelegationConnector_modified"
    assert delegation_connector.name == "DelegationConnector_modified"

    assert delegation_connector.inner_port == sub_component_r_port
    assert delegation_connector.outer_port == component_r_port
    assert delegation_connector.inner_sw_component == sub_composition

    # check if the delegation sw connector can be constructed from an element and is equal to the original one
    element = delegation_connector.element
    delegation_connector_copy = DelegationSwConnector(element)
    assert delegation_connector == delegation_connector_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in DelegationSwConnector.__dict__
    assert delegation_connector.__repr__()


def test_assembly_sw_connector() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    sw_component_type = package.create_composition_sw_component_type(
        "CompositionSwComponentType"
    )
    app_component_type = package.create_application_sw_component_type(
        "ApplicationSwComponentType"
    )
    app_component = sw_component_type.create_component(
        "AppComponent", app_component_type
    )
    sr_interface = package.create_sender_receiver_interface("SRInterface")

    # AssemblySwConnector
    app_component_p_port = app_component_type.create_p_port("PPort", sr_interface)
    app_component_r_port = app_component_type.create_r_port("RPort", sr_interface)
    app_component2 = sw_component_type.create_component(
        "AppComponent2", app_component_type
    )
    assembly_connector = sw_component_type.create_assembly_connector(
        "AssemblyConnector",
        app_component_p_port,
        app_component,
        app_component_r_port,
        app_component2,
    )
    assert isinstance(assembly_connector, AssemblySwConnector)

    # get and set the name
    assert assembly_connector.name == "AssemblyConnector"
    assembly_connector.name = "AssemblyConnector_modified"
    assert assembly_connector.name == "AssemblyConnector_modified"

    assert assembly_connector.p_port == app_component_p_port
    assert assembly_connector.r_port == app_component_r_port
    assert assembly_connector.p_sw_component == app_component
    assert assembly_connector.r_sw_component == app_component2

    # check if the assembly sw connector can be constructed from an element and is equal to the original one
    element = assembly_connector.element
    assembly_connector_copy = AssemblySwConnector(element)
    assert assembly_connector == assembly_connector_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in AssemblySwConnector.__dict__
    assert assembly_connector.__repr__()


def test_pass_through_sw_connector() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    sw_component_type = package.create_composition_sw_component_type(
        "CompositionSwComponentType"
    )
    sr_interface = package.create_sender_receiver_interface("SRInterface")

    # PassThroughSwConnector
    component_r_port = sw_component_type.create_r_port("RPort", sr_interface)
    component_p_port = sw_component_type.create_p_port("PPort", sr_interface)
    passthrough_connector = sw_component_type.create_pass_through_connector(
        "PassthroughConnector", component_p_port, component_r_port
    )
    assert isinstance(passthrough_connector, PassThroughSwConnector)

    # get and set the name
    assert passthrough_connector.name == "PassthroughConnector"
    passthrough_connector.name = "PassthroughConnector_modified"
    assert passthrough_connector.name == "PassthroughConnector_modified"

    assert passthrough_connector.p_port == component_p_port
    assert passthrough_connector.r_port == component_r_port

    # check if the pass through sw connector can be constructed from an element and is equal to the original one
    element = passthrough_connector.element
    passthrough_connector_copy = PassThroughSwConnector(element)
    assert passthrough_connector == passthrough_connector_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in PassThroughSwConnector.__dict__
    assert passthrough_connector.__repr__()


def test_client_server_interface() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    sw_component_type = package.create_composition_sw_component_type(
        "CompositionSwComponentType"
    )

    # ClientServerInterface
    client_server_interface = package.create_client_server_interface(
        "ClientServerInterface"
    )
    assert isinstance(client_server_interface, ClientServerInterface)
    # get and set the name
    assert client_server_interface.name == "ClientServerInterface"
    client_server_interface.name = "ClientServerInterface_modified"
    assert client_server_interface.name == "ClientServerInterface_modified"

    assert client_server_interface.is_service is None
    client_server_interface.is_service = True
    assert client_server_interface.is_service is True

    application_error1 = client_server_interface.create_possible_error("Error1", 403)
    application_error2 = client_server_interface.create_possible_error("Error2", 404)
    assert list(client_server_interface.possible_errors()) == [
        application_error1,
        application_error2,
    ]

    operation1 = client_server_interface.create_operation("Operation1")
    operation2 = client_server_interface.create_operation("Operation2")
    assert list(client_server_interface.operations()) == [operation1, operation2]

    p_port = sw_component_type.create_p_port("PPort", client_server_interface)
    assert p_port.port_interface == client_server_interface

    # check if the client server interface can be constructed from an element and is equal to the original one
    element = client_server_interface.element
    client_server_interface_copy = ClientServerInterface(element)
    assert client_server_interface == client_server_interface_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ClientServerInterface.__dict__
    assert client_server_interface.__repr__()


def test_application_error() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    client_server_interface = package.create_client_server_interface("ClientServer")

    # ApplicationError
    application_error = client_server_interface.create_possible_error("Error", 403)
    assert isinstance(application_error, ApplicationError)
    # get and set the name and error code
    assert application_error.name == "Error"
    application_error.name = "Error_modified"
    assert application_error.name == "Error_modified"
    assert application_error.error_code == 403
    application_error.error_code = 404
    assert application_error.error_code == 404

    # check if the application error can be constructed from an element and is equal to the original one
    element = application_error.element
    application_error_copy = ApplicationError(element)
    assert application_error == application_error_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ApplicationError.__dict__
    assert application_error.__repr__()


def test_client_server_operation() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    client_server_interface = package.create_client_server_interface("ClientServer")
    application_error = client_server_interface.create_possible_error("Error", 403)
    sw_base_type = package.create_sw_base_type(
        "Unit32", 32, BaseTypeEncoding.TwosComplement
    )
    impl_data_type = package.create_implementation_data_type(
        ImplementationDataTypeSettings.Value("ImplValue", base_type=sw_base_type)
    )

    # ClientServerOperation
    operation = client_server_interface.create_operation("Operation")
    assert isinstance(operation, ClientServerOperation)
    # get and set the name
    assert operation.name == "Operation"
    operation.name = "Operation_modified"
    assert operation.name == "Operation_modified"

    operation.add_possible_error(application_error)
    assert list(operation.possible_errors()) == [application_error]

    argument = operation.create_argument(
        "Argument1", impl_data_type, ArgumentDirection.In
    )
    assert list(operation.arguments()) == [argument]

    # check if the client server operation can be constructed from an element and is equal to the original one
    element = operation.element
    operation_copy = ClientServerOperation(element)
    assert operation == operation_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ClientServerOperation.__dict__
    assert operation.__repr__()


def test_argument_data_prototype() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    client_server_interface = package.create_client_server_interface("ClientServer")
    sw_base_type = package.create_sw_base_type(
        "Unit32", 32, BaseTypeEncoding.TwosComplement
    )
    impl_data_type = package.create_implementation_data_type(
        ImplementationDataTypeSettings.Value("ImplValue", base_type=sw_base_type)
    )

    # ArgumentDataPrototype
    operation = client_server_interface.create_operation("Operation")
    argument = operation.create_argument(
        "Argument", impl_data_type, ArgumentDirection.In
    )
    assert isinstance(argument, ArgumentDataPrototype)
    # get and set the name
    assert argument.name == "Argument"
    argument.name = "Argument_modified"
    assert argument.name == "Argument_modified"

    argument.data_type = impl_data_type
    assert argument.data_type == impl_data_type
    argument.direction = ArgumentDirection.In
    assert argument.direction == ArgumentDirection.In
    argument.direction = ArgumentDirection.Out
    assert argument.direction == ArgumentDirection.Out
    argument.direction = ArgumentDirection.InOut
    assert argument.direction == ArgumentDirection.InOut

    # check if the argument data prototype can be constructed from an element and is equal to the original one
    element = argument.element
    argument_copy = ArgumentDataPrototype(element)
    assert argument == argument_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ArgumentDataPrototype.__dict__
    assert argument.__repr__()


def test_sender_receiver_interface() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    sw_base_type = package.create_sw_base_type(
        "Unit32", 32, BaseTypeEncoding.TwosComplement
    )
    impl_data_type = package.create_implementation_data_type(
        ImplementationDataTypeSettings.Value("ImplValue", base_type=sw_base_type)
    )
    app_data_type = package.create_application_primitive_data_type(
        "PrimitiveType", ApplicationPrimitiveCategory.Boolean
    )
    sw_component_type = package.create_composition_sw_component_type(
        "CompositionSwComponentType"
    )

    # SenderReceiverInterface
    sr_interface = package.create_sender_receiver_interface("SRInterface")
    assert isinstance(sr_interface, SenderReceiverInterface)
    # get and set the name
    assert sr_interface.name == "SRInterface"
    sr_interface.name = "SRInterface_modified"
    assert sr_interface.name == "SRInterface_modified"

    assert sr_interface.is_service is None
    sr_interface.is_service = False
    assert sr_interface.is_service is False

    data_element1 = sr_interface.create_data_element("DataElement1", impl_data_type)
    data_element2 = sr_interface.create_data_element("DataElement2", app_data_type)
    assert list(sr_interface.data_elements()) == [data_element1, data_element2]

    p_port = sw_component_type.create_p_port("PPort", sr_interface)
    assert p_port.port_interface == sr_interface

    # check if the sender receiver interface can be constructed from an element and is equal to the original one
    element = sr_interface.element
    sr_interface_copy = SenderReceiverInterface(element)
    assert sr_interface == sr_interface_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in SenderReceiverInterface.__dict__
    assert sr_interface.__repr__()


def test_variable_data_prototype() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    sr_interface = package.create_sender_receiver_interface("SRInterface")
    sw_base_type = package.create_sw_base_type(
        "Unit32", 32, BaseTypeEncoding.TwosComplement
    )
    impl_data_type = package.create_implementation_data_type(
        ImplementationDataTypeSettings.Value("ImplValue", base_type=sw_base_type)
    )
    app_data_type = package.create_application_primitive_data_type(
        "PrimitiveType", ApplicationPrimitiveCategory.Boolean
    )

    # VariableDataPrototype
    variable_data_prototype = sr_interface.create_data_element(
        "DataElement", impl_data_type
    )
    assert isinstance(variable_data_prototype, VariableDataPrototype)
    # get and set the name
    assert variable_data_prototype.name == "DataElement"
    variable_data_prototype.name = "DataElement_modified"
    assert variable_data_prototype.name == "DataElement_modified"

    assert variable_data_prototype.interface == sr_interface

    variable_data_prototype.data_type = impl_data_type
    assert variable_data_prototype.data_type == impl_data_type
    variable_data_prototype.data_type = app_data_type
    assert variable_data_prototype.data_type == app_data_type

    variable_data_prototype.init_value = 42
    assert variable_data_prototype.init_value == NumericalValueSpecification(42)

    # check if the variable data prototype can be constructed from an element and is equal to the original one
    element = variable_data_prototype.element
    variable_data_prototype_copy = VariableDataPrototype(element)
    assert variable_data_prototype == variable_data_prototype_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in VariableDataPrototype.__dict__
    assert variable_data_prototype.__repr__()


def test_mode_switch_interface() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    sw_component_type = package.create_composition_sw_component_type(
        "CompositionSwComponentType"
    )

    # ModeSwitchInterface
    mode_switch_interface = package.create_mode_switch_interface("ModeSwitchInterface")
    assert isinstance(mode_switch_interface, ModeSwitchInterface)
    # get and set the name
    assert mode_switch_interface.name == "ModeSwitchInterface"
    mode_switch_interface.name = "ModeSwitchInterface_modified"
    assert mode_switch_interface.name == "ModeSwitchInterface_modified"

    assert mode_switch_interface.is_service is None
    mode_switch_interface.is_service = False
    assert mode_switch_interface.is_service is False

    # create a mode group
    mode_declaration_group = package.create_mode_declaration_group(
        "ModeDeclarationGroup"
    )
    mode_group = mode_switch_interface.create_mode_group(
        "ModeGroup", mode_declaration_group
    )
    assert isinstance(mode_group, ModeGroup)
    assert mode_switch_interface.mode_group == mode_group

    p_port = sw_component_type.create_p_port("PPort", mode_switch_interface)
    assert p_port.port_interface == mode_switch_interface

    # check if the mode switch interface can be constructed from an element and is equal to the original one
    element = mode_switch_interface.element
    mode_switch_interface_copy = ModeSwitchInterface(element)
    assert mode_switch_interface == mode_switch_interface_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ModeSwitchInterface.__dict__
    assert mode_switch_interface.__repr__()


def test_mode_group() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    mode_switch_interface = package.create_mode_switch_interface("ModeSwitchInterface")

    # ModeGroup
    mode_declaration_group = package.create_mode_declaration_group(
        "ModeDeclarationGroup"
    )
    mode_group = mode_switch_interface.create_mode_group(
        "ModeGroup", mode_declaration_group
    )
    assert isinstance(mode_group, ModeGroup)
    # get and set the name
    assert mode_group.name == "ModeGroup"
    mode_group.name = "ModeGroup_modified"
    assert mode_group.name == "ModeGroup_modified"

    assert mode_group.mode_declaration_group == mode_declaration_group
    mode_declaration_group2 = package.create_mode_declaration_group(
        "ModeDeclarationGroup2"
    )
    mode_group.mode_declaration_group = mode_declaration_group2
    assert mode_group.mode_declaration_group == mode_declaration_group2

    # check if the mode group can be constructed from an element and is equal to the original one
    element = mode_group.element
    mode_group_copy = ModeGroup(element)
    assert mode_group == mode_group_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ModeGroup.__dict__
    assert mode_group.__repr__()


def test_parameter_interface() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    sw_component_type = package.create_composition_sw_component_type(
        "CompositionSwComponentType"
    )
    sw_base_type = package.create_sw_base_type(
        "Unit32", 32, BaseTypeEncoding.TwosComplement
    )
    impl_data_type = package.create_implementation_data_type(
        ImplementationDataTypeSettings.Value("ImplValue", base_type=sw_base_type)
    )

    # ParameterInterface
    parameter_interface = package.create_parameter_interface("ParameterInterface")
    assert isinstance(parameter_interface, ParameterInterface)
    # get and set the name
    assert parameter_interface.name == "ParameterInterface"
    parameter_interface.name = "ParameterInterface_modified"
    assert parameter_interface.name == "ParameterInterface_modified"

    assert parameter_interface.is_service is None
    parameter_interface.is_service = True
    assert parameter_interface.is_service is True

    p_port = sw_component_type.create_p_port("PPort", parameter_interface)
    assert p_port.port_interface == parameter_interface

    pdp = parameter_interface.create_parameter("ParameterData", impl_data_type)
    assert isinstance(pdp, ParameterDataPrototype)
    assert list(parameter_interface.parameters()) == [pdp]

    # check if the parameter interface can be constructed from an element and is equal to the original one
    element = parameter_interface.element
    parameter_interface_copy = ParameterInterface(element)
    assert parameter_interface == parameter_interface_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ParameterInterface.__dict__
    assert parameter_interface.__repr__()


def test_parameter_data_prototype() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    parameter_interface = package.create_parameter_interface("ParameterInterface")
    sw_base_type = package.create_sw_base_type(
        "Unit32", 32, BaseTypeEncoding.TwosComplement
    )
    impl_data_type = package.create_implementation_data_type(
        ImplementationDataTypeSettings.Value("ImplValue", base_type=sw_base_type)
    )

    # ParameterDataPrototype
    parameter_data_prototype = parameter_interface.create_parameter(
        "ParameterData", impl_data_type
    )
    assert isinstance(parameter_data_prototype, ParameterDataPrototype)
    # get and set the name
    assert parameter_data_prototype.name == "ParameterData"
    parameter_data_prototype.name = "ParameterData_modified"
    assert parameter_data_prototype.name == "ParameterData_modified"

    parameter_data_prototype.init_value = 42
    assert parameter_data_prototype.init_value == NumericalValueSpecification(42)
    assert parameter_data_prototype.interface == parameter_interface
    assert parameter_data_prototype.data_type == impl_data_type

    # check if the parameter data prototype can be constructed from an element and is equal to the original one
    element = parameter_data_prototype.element
    parameter_data_prototype_copy = ParameterDataPrototype(element)
    assert parameter_data_prototype == parameter_data_prototype_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ParameterDataPrototype.__dict__
    assert parameter_data_prototype.__repr__()


def test_nv_data_interface() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    sw_component_type = package.create_composition_sw_component_type(
        "CompositionSwComponentType"
    )

    # NvDataInterface
    nv_data_interface = package.create_nv_data_interface("NvDataInterface")
    assert isinstance(nv_data_interface, NvDataInterface)
    # get and set the name
    assert nv_data_interface.name == "NvDataInterface"
    nv_data_interface.name = "NvDataInterface_modified"
    assert nv_data_interface.name == "NvDataInterface_modified"

    assert nv_data_interface.is_service is None
    nv_data_interface.is_service = False
    assert nv_data_interface.is_service is False

    p_port = sw_component_type.create_p_port("PPort", nv_data_interface)
    assert p_port.port_interface == nv_data_interface

    # check if the nv data interface can be constructed from an element and is equal to the original one
    element = nv_data_interface.element
    nv_data_interface_copy = NvDataInterface(element)
    assert nv_data_interface == nv_data_interface_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in NvDataInterface.__dict__
    assert nv_data_interface.__repr__()


def test_trigger_interface() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    sw_component_type = package.create_composition_sw_component_type(
        "CompositionSwComponentType"
    )

    # TriggerInterface
    trigger_interface = package.create_trigger_interface("TriggerInterface")
    assert isinstance(trigger_interface, TriggerInterface)
    # get and set the name
    assert trigger_interface.name == "TriggerInterface"
    trigger_interface.name = "TriggerInterface_modified"
    assert trigger_interface.name == "TriggerInterface_modified"

    assert trigger_interface.is_service is None
    trigger_interface.is_service = False
    assert trigger_interface.is_service is False

    p_port = sw_component_type.create_p_port("PPort", trigger_interface)
    assert p_port.port_interface == trigger_interface

    # check if the trigger interface can be constructed from an element and is equal to the original one
    element = trigger_interface.element
    trigger_interface_copy = TriggerInterface(element)
    assert trigger_interface == trigger_interface_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in TriggerInterface.__dict__
    assert trigger_interface.__repr__()


def test_r_port_prototype() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    sr_interface = package.create_sender_receiver_interface("SRInterface")
    sw_component_type = package.create_composition_sw_component_type(
        "CompositionSwComponentType"
    )

    # RPortPrototype
    r_port_prototype = sw_component_type.create_r_port("RPort", sr_interface)
    assert isinstance(r_port_prototype, RPortPrototype)
    # get and set the name
    assert r_port_prototype.name == "RPort"
    r_port_prototype.name = "RPort_modified"
    assert r_port_prototype.name == "RPort_modified"

    assert r_port_prototype.port_interface == sr_interface
    assert r_port_prototype.component_type == sw_component_type

    # check if the r port prototype can be constructed from an element and is equal to the original one
    element = r_port_prototype.element
    r_port_prototype_copy = RPortPrototype(element)
    assert r_port_prototype == r_port_prototype_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in RPortPrototype.__dict__
    assert r_port_prototype.__repr__()


def test_p_port_prototype() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    sr_interface = package.create_sender_receiver_interface("SRInterface")
    sw_component_type = package.create_composition_sw_component_type(
        "CompositionSwComponentType"
    )

    # PPortPrototype
    p_port_prototype = sw_component_type.create_p_port("PPort", sr_interface)
    assert isinstance(p_port_prototype, PPortPrototype)
    # get and set the name
    assert p_port_prototype.name == "PPort"
    p_port_prototype.name = "PPort_modified"
    assert p_port_prototype.name == "PPort_modified"

    assert p_port_prototype.port_interface == sr_interface
    assert p_port_prototype.component_type == sw_component_type

    # check if the p port prototype can be constructed from an element and is equal to the original one
    element = p_port_prototype.element
    p_port_prototype_copy = PPortPrototype(element)
    assert p_port_prototype == p_port_prototype_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in PPortPrototype.__dict__
    assert p_port_prototype.__repr__()


def test_pr_port_prototype() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    sr_interface = package.create_sender_receiver_interface("SRInterface")
    sw_component_type = package.create_composition_sw_component_type(
        "CompositionSwComponentType"
    )

    # PRPortPrototype
    pr_port_prototype = sw_component_type.create_pr_port("PRPort", sr_interface)
    assert isinstance(pr_port_prototype, PRPortPrototype)
    # get and set the name
    assert pr_port_prototype.name == "PRPort"
    pr_port_prototype.name = "PRPort_modified"
    assert pr_port_prototype.name == "PRPort_modified"

    assert pr_port_prototype.port_interface == sr_interface
    assert pr_port_prototype.component_type == sw_component_type

    # check if the pr port prototype can be constructed from an element and is equal to the original one
    element = pr_port_prototype.element
    pr_port_prototype_copy = PRPortPrototype(element)
    assert pr_port_prototype == pr_port_prototype_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in PRPortPrototype.__dict__
    assert pr_port_prototype.__repr__()


def test_port_group() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    sw_component_type = package.create_composition_sw_component_type(
        "CompositionSwComponentType"
    )

    # PortGroup
    port_group = sw_component_type.create_port_group("PortGroup")
    assert isinstance(port_group, PortGroup)
    # get and set the name
    assert port_group.name == "PortGroup"
    port_group.name = "PortGroup_modified"
    assert port_group.name == "PortGroup_modified"

    # check if the port group can be constructed from an element and is equal to the original one
    element = port_group.element
    port_group_copy = PortGroup(element)
    assert port_group == port_group_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in PortGroup.__dict__
    assert port_group.__repr__()


def test_mode_declaration_group() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    # ModeDeclarationGroup
    mode_declaration_group = package.create_mode_declaration_group(
        "ModeDeclarationGroup"
    )
    assert isinstance(mode_declaration_group, ModeDeclarationGroup)
    # get and set the name
    assert mode_declaration_group.name == "ModeDeclarationGroup"
    mode_declaration_group.name = "ModeDeclarationGroup_modified"
    assert mode_declaration_group.name == "ModeDeclarationGroup_modified"

    assert mode_declaration_group.category is None
    mode_declaration_group.category = ModeDeclarationGroupCategory.AlphabeticOrder
    assert (
        mode_declaration_group.category == ModeDeclarationGroupCategory.AlphabeticOrder
    )
    mode_declaration_group.category = ModeDeclarationGroupCategory.ExplicitOrder
    assert mode_declaration_group.category == ModeDeclarationGroupCategory.ExplicitOrder

    mode_declaration = mode_declaration_group.create_mode_declaration("ModeDeclaration")
    assert isinstance(mode_declaration, ModeDeclaration)
    list(mode_declaration_group.mode_declarations()) == [mode_declaration]

    assert mode_declaration_group.initial_mode is None
    mode_declaration_group.initial_mode = mode_declaration
    assert mode_declaration_group.initial_mode == mode_declaration

    assert mode_declaration_group.on_transition_value is None
    mode_declaration_group.on_transition_value = 42
    assert mode_declaration_group.on_transition_value == 42

    # check if the mode declaration group can be constructed from an element and is equal to the original one
    element = mode_declaration_group.element
    mode_declaration_group_copy = ModeDeclarationGroup(element)
    assert mode_declaration_group == mode_declaration_group_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ModeDeclarationGroup.__dict__
    assert mode_declaration_group.__repr__()


def test_mode_declaration() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    mode_declaration_group = package.create_mode_declaration_group(
        "ModeDeclarationGroup"
    )

    # ModeDeclaration
    mode_declaration = mode_declaration_group.create_mode_declaration("ModeDeclaration")
    assert isinstance(mode_declaration, ModeDeclaration)
    # get and set the name
    assert mode_declaration.name == "ModeDeclaration"
    mode_declaration.name = "ModeDeclaration_modified"
    assert mode_declaration.name == "ModeDeclaration_modified"

    assert mode_declaration.value is None
    mode_declaration.value = 42
    assert mode_declaration.value == 42

    # check if the mode declaration can be constructed from an element and is equal to the original one
    element = mode_declaration.element
    mode_declaration_copy = ModeDeclaration(element)
    assert mode_declaration == mode_declaration_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ModeDeclaration.__dict__
    assert mode_declaration.__repr__()
