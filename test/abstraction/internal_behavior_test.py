from autosar_data.abstraction import *
from autosar_data.abstraction.datatype import *
from autosar_data.abstraction.software_component import *


def test_internal_behavior() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    app_component_type = package.create_application_sw_component_type(
        "ApplicationSwComponentType"
    )
    client_server_interface = package.create_client_server_interface("ClientServer")
    p_port = app_component_type.create_p_port("Port", client_server_interface)
    operation = client_server_interface.create_operation("Operation")

    # SwcInternalBehavior
    internal_behavior = app_component_type.create_swc_internal_behavior(
        "InternalBehavior"
    )
    assert isinstance(internal_behavior, SwcInternalBehavior)
    # get and set the name
    assert internal_behavior.name == "InternalBehavior"
    internal_behavior.name = "InternalBehavior2"
    assert internal_behavior.name == "InternalBehavior2"

    assert internal_behavior.sw_component_type == app_component_type

    # Runnable
    init_runnable = internal_behavior.create_runnable_entity("InitRunnable")
    operation_runnable = internal_behavior.create_runnable_entity("OperationRunnable")
    timed_runnable = internal_behavior.create_runnable_entity("TimedRunnable")
    background_runnable = internal_behavior.create_runnable_entity("BackgroundRunnable")
    ostask_runnable = internal_behavior.create_runnable_entity("OsTaskRunnable")
    assert list(internal_behavior.runnable_entities()) == [
        init_runnable,
        operation_runnable,
        timed_runnable,
        background_runnable,
        ostask_runnable,
    ]

    # DataTypeMappingSet
    data_type_mapping_set = package.create_data_type_mapping_set("DataTypeMappingSet")
    internal_behavior.add_data_type_mapping_set(data_type_mapping_set)
    assert list(internal_behavior.data_type_mapping_sets()) == [data_type_mapping_set]

    # Create Events
    event1 = internal_behavior.create_init_event("InitEvent", init_runnable)
    event2 = internal_behavior.create_operation_invoked_event(
        "OperationInvokedEvent", operation_runnable, operation, p_port
    )
    event3 = internal_behavior.create_timing_event("TimingEvent", timed_runnable, 0.01)
    event4 = internal_behavior.create_background_event(
        "BackgroundEvent", background_runnable
    )
    event5 = internal_behavior.create_os_task_execution_event(
        "OsTaskEvent", ostask_runnable
    )
    assert list(internal_behavior.events()) == [event1, event2, event3, event4, event5]

    # check if the swc_internal_behavior can be constructed from an element and is equal to the original one
    element = internal_behavior.element
    internal_behavior2 = SwcInternalBehavior(element)
    assert internal_behavior == internal_behavior2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in SwcInternalBehavior.__dict__
    assert internal_behavior.__repr__()


def test_runnable_entity() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    app_component_type = package.create_application_sw_component_type(
        "ApplicationSwComponentType"
    )
    internal_behavior = app_component_type.create_swc_internal_behavior(
        "InternalBehavior"
    )
    app_data_type = package.create_application_primitive_data_type(
        "PrimitiveType", ApplicationPrimitiveCategory.Boolean
    )
    sender_receiver_interface = package.create_sender_receiver_interface(
        "SenderReceiver"
    )
    r_port = app_component_type.create_r_port("RPort", sender_receiver_interface)
    p_port = app_component_type.create_p_port("PPort", sender_receiver_interface)
    variable = sender_receiver_interface.create_data_element(
        "DataElement", app_data_type
    )

    # Runnable
    runnable = internal_behavior.create_runnable_entity("Runnable")
    assert isinstance(runnable, RunnableEntity)
    # get and set the name
    assert runnable.name == "Runnable"
    runnable.name = "Runnable2"
    assert runnable.name == "Runnable2"

    assert runnable.swc_internal_behavior == internal_behavior

    event = internal_behavior.create_init_event("InitEvent", runnable)
    assert list(runnable.events()) == [event]

    # test all types of variable access
    read_access = runnable.create_data_read_access("read_access", variable, r_port)
    assert list(runnable.data_read_accesses()) == [read_access]
    receive_by_arg_access = runnable.create_data_receive_point_by_argument(
        "receive_point_by_argument", variable, r_port
    )
    assert list(runnable.data_receive_points_by_argument()) == [receive_by_arg_access]
    receive_by_val_access = runnable.create_data_receive_point_by_value(
        "receive_point_by_value", variable, r_port
    )
    assert list(runnable.data_receive_points_by_value()) == [receive_by_val_access]
    write_access = runnable.create_data_write_access("write_access", variable, p_port)
    assert list(runnable.data_write_accesses()) == [write_access]
    send_point = runnable.create_data_send_point("send_point", variable, p_port)
    assert list(runnable.data_send_points()) == [send_point]

    # check if the runnable_entity can be constructed from an element and is equal to the original one
    element = runnable.element
    runnable2 = RunnableEntity(element)
    assert runnable == runnable2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in RunnableEntity.__dict__
    assert runnable.__repr__()


def test_timing_event() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    app_component_type = package.create_application_sw_component_type(
        "ApplicationSwComponentType"
    )
    internal_behavior = app_component_type.create_swc_internal_behavior(
        "InternalBehavior"
    )
    runnable = internal_behavior.create_runnable_entity("Runnable")
    runnable2 = internal_behavior.create_runnable_entity("Runnable2")

    # TimingEvent
    timing_event = internal_behavior.create_timing_event("TimingEvent", runnable, 0.01)
    assert isinstance(timing_event, TimingEvent)
    # get and set the name
    assert timing_event.name == "TimingEvent"
    timing_event.name = "TimingEvent2"
    assert timing_event.name == "TimingEvent2"

    assert timing_event.swc_internal_behavior == internal_behavior
    assert timing_event.runnable_entity == runnable
    timing_event.runnable_entity = runnable2
    assert timing_event.runnable_entity == runnable2
    assert timing_event.period == 0.01
    timing_event.period = 0.02
    assert timing_event.period == 0.02

    assert list(internal_behavior.events()) == [timing_event]

    # check if the timing_event can be constructed from an element and is equal to the original one
    element = timing_event.element
    timing_event2 = TimingEvent(element)
    assert timing_event == timing_event2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in TimingEvent.__dict__
    assert timing_event.__repr__()


def test_background_event() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    app_component_type = package.create_application_sw_component_type(
        "ApplicationSwComponentType"
    )
    internal_behavior = app_component_type.create_swc_internal_behavior(
        "InternalBehavior"
    )
    runnable = internal_behavior.create_runnable_entity("Runnable")
    runnable2 = internal_behavior.create_runnable_entity("Runnable2")

    # BackgroundEvent
    background_event = internal_behavior.create_background_event(
        "BackgroundEvent", runnable
    )
    assert isinstance(background_event, BackgroundEvent)
    # get and set the name
    assert background_event.name == "BackgroundEvent"
    background_event.name = "BackgroundEvent2"
    assert background_event.name == "BackgroundEvent2"

    assert background_event.swc_internal_behavior == internal_behavior
    assert background_event.runnable_entity == runnable
    background_event.runnable_entity = runnable2
    assert background_event.runnable_entity == runnable2

    assert list(internal_behavior.events()) == [background_event]

    # check if the background_event can be constructed from an element and is equal to the original one
    element = background_event.element
    background_event2 = BackgroundEvent(element)
    assert background_event == background_event2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in BackgroundEvent.__dict__
    assert background_event.__repr__()


def test_os_task_execution_event() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    app_component_type = package.create_application_sw_component_type(
        "ApplicationSwComponentType"
    )
    internal_behavior = app_component_type.create_swc_internal_behavior(
        "InternalBehavior"
    )
    runnable = internal_behavior.create_runnable_entity("Runnable")
    runnable2 = internal_behavior.create_runnable_entity("Runnable2")

    # OsTaskExecutionEvent
    os_task_execution_event = internal_behavior.create_os_task_execution_event(
        "OsTaskExecutionEvent", runnable
    )
    assert isinstance(os_task_execution_event, OsTaskExecutionEvent)
    # get and set the name
    assert os_task_execution_event.name == "OsTaskExecutionEvent"
    os_task_execution_event.name = "OsTaskExecutionEvent2"
    assert os_task_execution_event.name == "OsTaskExecutionEvent2"

    assert os_task_execution_event.swc_internal_behavior == internal_behavior
    assert os_task_execution_event.runnable_entity == runnable
    os_task_execution_event.runnable_entity = runnable2
    assert os_task_execution_event.runnable_entity == runnable2

    assert list(internal_behavior.events()) == [os_task_execution_event]

    # check if the os_task_execution_event can be constructed from an element and is equal to the original one
    element = os_task_execution_event.element
    os_task_execution_event2 = OsTaskExecutionEvent(element)
    assert os_task_execution_event == os_task_execution_event2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in OsTaskExecutionEvent.__dict__
    assert os_task_execution_event.__repr__()


def test_operation_invoked_event() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    app_component_type = package.create_application_sw_component_type(
        "ApplicationSwComponentType"
    )
    client_server_interface = package.create_client_server_interface("ClientServer")
    p_port = app_component_type.create_p_port("Port", client_server_interface)
    operation = client_server_interface.create_operation("Operation")
    internal_behavior = app_component_type.create_swc_internal_behavior(
        "InternalBehavior"
    )
    runnable = internal_behavior.create_runnable_entity("Runnable")
    runnable2 = internal_behavior.create_runnable_entity("Runnable2")

    # OperationInvokedEvent
    operation_invoked_event = internal_behavior.create_operation_invoked_event(
        "OperationInvokedEvent", runnable, operation, p_port
    )
    assert isinstance(operation_invoked_event, OperationInvokedEvent)
    # get and set the name
    assert operation_invoked_event.name == "OperationInvokedEvent"
    operation_invoked_event.name = "OperationInvokedEvent2"
    assert operation_invoked_event.name == "OperationInvokedEvent2"

    assert operation_invoked_event.swc_internal_behavior == internal_behavior
    assert operation_invoked_event.runnable_entity == runnable
    operation_invoked_event.runnable_entity = runnable2
    assert operation_invoked_event.runnable_entity == runnable2

    operation_invoked_event.set_client_server_operation(operation, p_port)
    assert operation_invoked_event.client_server_operation == (operation, p_port)

    assert list(internal_behavior.events()) == [operation_invoked_event]

    # check if the operation_invoked_event can be constructed from an element and is equal to the original one
    element = operation_invoked_event.element
    operation_invoked_event2 = OperationInvokedEvent(element)
    assert operation_invoked_event == operation_invoked_event2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in OperationInvokedEvent.__dict__
    assert operation_invoked_event.__repr__()


def test_init_event() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    app_component_type = package.create_application_sw_component_type(
        "ApplicationSwComponentType"
    )
    internal_behavior = app_component_type.create_swc_internal_behavior(
        "InternalBehavior"
    )
    runnable = internal_behavior.create_runnable_entity("Runnable")
    runnable2 = internal_behavior.create_runnable_entity("Runnable2")

    # InitEvent
    init_event = internal_behavior.create_init_event("InitEvent", runnable)
    assert isinstance(init_event, InitEvent)
    # get and set the name
    assert init_event.name == "InitEvent"
    init_event.name = "InitEvent2"
    assert init_event.name == "InitEvent2"

    assert init_event.swc_internal_behavior == internal_behavior
    assert init_event.runnable_entity == runnable
    init_event.runnable_entity = runnable2
    assert init_event.runnable_entity == runnable2

    assert list(internal_behavior.events()) == [init_event]

    # check if the init_event can be constructed from an element and is equal to the original one
    element = init_event.element
    init_event2 = InitEvent(element)
    assert init_event == init_event2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in InitEvent.__dict__
    assert init_event.__repr__()


def test_mode_switch_event() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    app_component_type = package.create_application_sw_component_type(
        "ApplicationSwComponentType"
    )
    mode_declacration_group = package.create_mode_declaration_group(
        "ModeDeclarationGroup"
    )
    mode_declaration_1 = mode_declacration_group.create_mode_declaration(
        "ModeDeclaration1"
    )
    mode_declaration_2 = mode_declacration_group.create_mode_declaration(
        "ModeDeclaration2"
    )
    mode_switch_interface = package.create_mode_switch_interface("ModeSwitchInterface")
    mode_switch_interface.create_mode_group("ModeGroup", mode_declacration_group)
    r_port = app_component_type.create_r_port("Port", mode_switch_interface)

    internal_behavior = app_component_type.create_swc_internal_behavior(
        "InternalBehavior"
    )
    runnable = internal_behavior.create_runnable_entity("Runnable")
    runnable2 = internal_behavior.create_runnable_entity("Runnable2")

    # ModeSwitchEvent
    mode_switch_event = internal_behavior.create_mode_switch_event(
        "ModeSwitchEvent",
        runnable,
        ModeActivationKind.OnTransition,
        r_port,
        mode_declaration_1,
        mode_declaration_2,
    )
    assert isinstance(mode_switch_event, SwcModeSwitchEvent)
    # get and set the name
    assert mode_switch_event.name == "ModeSwitchEvent"
    mode_switch_event.name = "ModeSwitchEvent2"
    assert mode_switch_event.name == "ModeSwitchEvent2"

    assert mode_switch_event.swc_internal_behavior == internal_behavior
    assert mode_switch_event.runnable_entity == runnable
    mode_switch_event.runnable_entity = runnable2
    assert mode_switch_event.runnable_entity == runnable2

    assert mode_switch_event.mode_activation_kind == ModeActivationKind.OnTransition
    mode_switch_event.mode_activation_kind = ModeActivationKind.OnEntry
    assert mode_switch_event.mode_activation_kind == ModeActivationKind.OnEntry
    (mode_declarations, context_port) = mode_switch_event.mode_declarations()
    assert mode_declarations == [mode_declaration_1, mode_declaration_2]
    assert context_port == r_port

    assert list(internal_behavior.events()) == [mode_switch_event]

    # check if the mode_switch_event can be constructed from an element and is equal to the original one
    element = mode_switch_event.element
    mode_switch_event2 = SwcModeSwitchEvent(element)
    assert mode_switch_event == mode_switch_event2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in SwcModeSwitchEvent.__dict__
    assert mode_switch_event.__repr__()


def test_data_received_event() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    app_component_type = package.create_application_sw_component_type(
        "ApplicationSwComponentType"
    )
    app_data_type = package.create_application_primitive_data_type(
        "PrimitiveType", ApplicationPrimitiveCategory.Boolean
    )
    sender_receiver_interface = package.create_sender_receiver_interface(
        "SenderReceiver"
    )
    r_port = app_component_type.create_r_port("Port", sender_receiver_interface)
    variable_data_prototype = sender_receiver_interface.create_data_element(
        "DataElement", app_data_type
    )
    internal_behavior = app_component_type.create_swc_internal_behavior(
        "InternalBehavior"
    )
    runnable = internal_behavior.create_runnable_entity("Runnable")
    runnable2 = internal_behavior.create_runnable_entity("Runnable2")

    # DataReceivedEvent
    data_received_event = internal_behavior.create_data_received_event(
        "DataReceivedEvent", runnable, variable_data_prototype, r_port
    )
    assert isinstance(data_received_event, DataReceivedEvent)
    # get and set the name
    assert data_received_event.name == "DataReceivedEvent"
    data_received_event.name = "DataReceivedEvent2"
    assert data_received_event.name == "DataReceivedEvent2"

    assert data_received_event.swc_internal_behavior == internal_behavior
    assert data_received_event.runnable_entity == runnable
    data_received_event.runnable_entity = runnable2
    assert data_received_event.runnable_entity == runnable2

    data_received_event.set_variable_data_prototype(variable_data_prototype, r_port)
    assert data_received_event.variable_data_prototype == (
        variable_data_prototype,
        r_port,
    )

    assert list(internal_behavior.events()) == [data_received_event]

    # check if the data_received_event can be constructed from an element and is equal to the original one
    element = data_received_event.element
    data_received_event2 = DataReceivedEvent(element)
    assert data_received_event == data_received_event2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in DataReceivedEvent.__dict__
    assert data_received_event.__repr__()


def test_variable_access() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    app_component_type = package.create_application_sw_component_type(
        "ApplicationSwComponentType"
    )
    internal_behavior = app_component_type.create_swc_internal_behavior(
        "InternalBehavior"
    )
    app_data_type = package.create_application_primitive_data_type(
        "PrimitiveType", ApplicationPrimitiveCategory.Boolean
    )
    sender_receiver_interface = package.create_sender_receiver_interface(
        "SenderReceiver"
    )
    r_port = app_component_type.create_r_port("RPort", sender_receiver_interface)
    variable = sender_receiver_interface.create_data_element(
        "DataElement", app_data_type
    )

    runnable = internal_behavior.create_runnable_entity("Runnable")

    # DataReadAccess
    variable_access = runnable.create_data_read_access(
        "DataReadAccess", variable, r_port
    )
    assert isinstance(variable_access, VariableAccess)
    # get and set the name
    assert variable_access.name == "DataReadAccess"
    variable_access.name = "DataReadAccess2"
    assert variable_access.name == "DataReadAccess2"

    assert variable_access.runnable_entity == runnable

    variable_access.set_accessed_variable(variable, r_port)
    assert variable_access.accessed_variable == (variable, r_port)

    # check if the variable_access can be constructed from an element and is equal to the original one
    element = variable_access.element
    variable_access2 = VariableAccess(element)
    assert variable_access == variable_access2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in VariableAccess.__dict__
    assert variable_access.__repr__()


def test_synchronous_server_call_point() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    app_component_type = package.create_application_sw_component_type(
        "ApplicationSwComponentType"
    )
    client_server_interface = package.create_client_server_interface("ClientServer")
    r_port = app_component_type.create_r_port("Port", client_server_interface)
    operation = client_server_interface.create_operation("Operation")
    internal_behavior = app_component_type.create_swc_internal_behavior(
        "InternalBehavior"
    )
    runnable = internal_behavior.create_runnable_entity("Runnable")

    # SynchronousServerCallPoint
    sync_call_point = runnable.create_synchronous_server_call_point(
        "SyncCallPoint", operation, r_port
    )
    assert isinstance(sync_call_point, SynchronousServerCallPoint)
    # get and set the name
    assert sync_call_point.name == "SyncCallPoint"
    sync_call_point.name = "SyncCallPoint2"
    assert sync_call_point.name == "SyncCallPoint2"

    assert sync_call_point.runnable_entity == runnable

    sync_call_point.set_client_server_operation(operation, r_port)
    assert sync_call_point.client_server_operation == (operation, r_port)

    assert list(runnable.synchronous_server_call_points()) == [sync_call_point]

    # check if the synchronous_server_call_point can be constructed from an element and is equal to the original one
    element = sync_call_point.element
    sync_call_point2 = SynchronousServerCallPoint(element)
    assert sync_call_point == sync_call_point2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in SynchronousServerCallPoint.__dict__
    assert sync_call_point.__repr__()


def test_mode_access_point() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    app_component_type = package.create_application_sw_component_type(
        "ApplicationSwComponentType"
    )
    mode_declacration_group = package.create_mode_declaration_group(
        "ModeDeclarationGroup"
    )
    mode_switch_interface = package.create_mode_switch_interface("ModeSwitchInterface")
    mode_group = mode_switch_interface.create_mode_group(
        "ModeGroup", mode_declacration_group
    )
    r_port = app_component_type.create_r_port("Port", mode_switch_interface)
    internal_behavior = app_component_type.create_swc_internal_behavior(
        "InternalBehavior"
    )
    runnable = internal_behavior.create_runnable_entity("Runnable")

    # ModeAccessPoint
    mode_access_point = runnable.create_mode_access_point(
        "ModeAccessPoint",
        mode_group,
        r_port,
    )
    assert isinstance(mode_access_point, ModeAccessPoint)
    # get and set the name
    assert mode_access_point.name == "ModeAccessPoint"
    mode_access_point.name = "ModeAccessPoint2"
    assert mode_access_point.name == "ModeAccessPoint2"

    assert mode_access_point.runnable_entity == runnable
    mode_access_point.set_mode_group(mode_group, r_port)
    assert mode_access_point.mode_group == (mode_group, r_port)

    assert list(runnable.mode_access_points()) == [mode_access_point]

    # check if the mode_access_point can be constructed from an element and is equal to the original one
    element = mode_access_point.element
    mode_access_point2 = ModeAccessPoint(element)
    assert mode_access_point == mode_access_point2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ModeAccessPoint.__dict__


def test_mode_switch_point() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    app_component_type = package.create_application_sw_component_type(
        "ApplicationSwComponentType"
    )
    mode_declacration_group = package.create_mode_declaration_group(
        "ModeDeclarationGroup"
    )
    mode_switch_interface = package.create_mode_switch_interface("ModeSwitchInterface")
    mode_group = mode_switch_interface.create_mode_group(
        "ModeGroup", mode_declacration_group
    )
    p_port = app_component_type.create_p_port("Port", mode_switch_interface)
    internal_behavior = app_component_type.create_swc_internal_behavior(
        "InternalBehavior"
    )
    runnable = internal_behavior.create_runnable_entity("Runnable")

    # ModeSwitchPoint
    mode_switch_point = runnable.create_mode_switch_point(
        "ModeSwitchPoint",
        mode_group,
        p_port,
    )
    assert isinstance(mode_switch_point, ModeSwitchPoint)
    # get and set the name
    assert mode_switch_point.name == "ModeSwitchPoint"
    mode_switch_point.name = "ModeSwitchPoint2"
    assert mode_switch_point.name == "ModeSwitchPoint2"

    assert mode_switch_point.runnable_entity == runnable
    mode_switch_point.set_mode_group(mode_group, p_port)
    assert mode_switch_point.mode_group == (mode_group, p_port)

    assert list(runnable.mode_switch_points()) == [mode_switch_point]

    # check if the mode_switch_point can be constructed from an element and is equal to the original one
    element = mode_switch_point.element
    mode_switch_point2 = ModeSwitchPoint(element)
    assert mode_switch_point == mode_switch_point2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ModeSwitchPoint.__dict__