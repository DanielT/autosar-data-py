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

    # check if the init_event can be constructed from an element and is equal to the original one
    element = init_event.element
    init_event2 = InitEvent(element)
    assert init_event == init_event2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in InitEvent.__dict__
    assert init_event.__repr__()
