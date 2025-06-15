# API Documentation: autosar_data.abstraction.software_component

## Example

```python
from autosar_data.abstraction import *
from autosar_data.abstraction.datatype import *
from autosar_data.abstraction.software_component import *

model = AutosarModelAbstraction.create("swc.arxml")
package = model.get_or_create_package("/Package")

# create some software components
composition_type = package.create_composition_sw_component_type("CompositionType")
app_swc1_type = package.create_application_sw_component_type("AppSwc1Type")
app_swc1 = composition_type.create_component("AppSwc1", app_swc1_type)
app_swc2_type = package.create_application_sw_component_type("AppSwc2Type")
app_swc2 = composition_type.create_component("AppSwc2", app_swc2_type)

# create a data type
sw_base_type = package.create_sw_base_type(
    "SwBaseType", 16, BaseTypeEncoding.TwosComplement
)
sr_data_type = package.create_implementation_data_type(
    ImplementationDataTypeSettings.Value("ImplValue", base_type=sw_base_type)
)

# create interfaces and ports
cs_interface = package.create_client_server_interface("CsInterface")
cs_operation = cs_interface.create_operation("CsOp")
sr_interface = package.create_sender_receiver_interface("SrInterface")
sr_data = sr_interface.create_data_element("SrData", sr_data_type)

# app1 provides the server interface, app2 requires it
app1_cs_p_port = app_swc1_type.create_p_port("P_CS", cs_interface)
app2_cs_r_port = app_swc2_type.create_r_port("R_CS", cs_interface)
# connect the ports
composition_type.create_assembly_connector(
    "CS_Assembly", app1_cs_p_port, app_swc1, app2_cs_r_port, app_swc2
)

# app1 receives data from app2 using the sender-receiver interface
app1_sr_r_port = app_swc1_type.create_r_port("R_Sr", sr_interface)
app2_sr_p_port = app_swc2_type.create_p_port("P_Sr", sr_interface)
# connect the ports
composition_type.create_assembly_connector(
    "Sr_Assembly", app1_sr_r_port, app_swc1, app2_sr_p_port, app_swc2
)

# create internal behavior for app1
app1_behavior = app_swc1_type.create_swc_internal_behavior("InternalBehavior")
# app1 needs a server runnable to process calls to its cs_operation
server_runnable = app1_behavior.create_runnable_entity("ServerRunnable")
app1_behavior.create_operation_invoked_event(
    "CsOperationEvent", server_runnable, cs_operation, app1_cs_p_port
)

# create a cyclically triggered runnable for app1
cyclic_runnable = app1_behavior.create_runnable_entity("CyclicRunnable")
app1_behavior.create_timing_event("CyclicEvent", cyclic_runnable, 0.01)

# allow the cyclic runnable to read data from the sender-receiver port
cyclic_runnable.create_data_receive_point_by_argument(
    "ReceiveData", sr_data, app1_sr_r_port
)

# create internal behavior for app2
app2_behavior = app_swc2_type.create_swc_internal_behavior("InternalBehavior")
# app2 has a cyclic runnable that sends data to app1 and calls the server operation
cyclic_runnable = app2_behavior.create_runnable_entity("CyclicRunnable")
app2_behavior.create_timing_event("CyclicEvent", cyclic_runnable, 0.01)
# allow the cyclic runnable to send data to app1
cyclic_runnable.create_data_send_point("SendData", sr_data, app2_sr_p_port)
# allow the cyclic runnable to call the server operation on app1
cyclic_runnable.create_synchronous_server_call_point(
    "ServerCall", cs_operation, app2_cs_r_port
)
```

## API

::: autosar_data.abstraction.software_component
