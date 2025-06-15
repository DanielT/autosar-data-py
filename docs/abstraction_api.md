# API Documentation: autosar_data.abstraction

## Description

The base package of the abstraction API provides a small number of classes shared by it's sub-packages.
it would usually only be used in combination with one of these:

- [autosar_data.abstraction.communication](communication_api.md): Communication over various busses
- [autosar_data.abstraction.datatype](datatype_api.md): data types used in the definition of software components and signals
- [autosar_data.abstraction.ecu_configuration](ecu_configuration_api.md): Access to the definition and values of an ECU configuration
- [autosar_data.abstraction.software_component](software_component_api.md): Modeling of software components

## Example

```python
from autosar_data.abstraction import *
from autosar_data.abstraction.communication import *
from autosar_data.abstraction.datatype import *

model = AutosarModelAbstraction.create("kitchen_comms.arxml")
system_package = model.get_or_create_package("/System")
ecu_package = model.get_or_create_package("/Ecus")
swc_package = model.get_or_create_package("/SoftwareComponentTypes")
comm_package = model.get_or_create_package("/Communication")
frames_package = model.get_or_create_package("/Frames")
pdus_package = model.get_or_create_package("/Pdus")
signals_package = model.get_or_create_package("/Signals")
datatypes_package = model.get_or_create_package("/DataTypes")
interfaces_package = model.get_or_create_package("/Interfaces")

# basic model elements
system = system_package.create_system("system", SystemCategory.EcuExtract)

toaster_ecu = system.create_ecu_instance("ToasterEcu", ecu_package)
fridge_ecu = system.create_ecu_instance("FridgeEcu", ecu_package)

# software component types: The system root contains two software components, one for a toaster and one for a fridge.
root_composition_type = swc_package.create_composition_sw_component_type(
    "RootCompositionType"
)
toaster_app_swc_type = swc_package.create_application_sw_component_type(
    "ToasterAppSwcType"
)
toaster_app_swc = root_composition_type.create_component(
    "ToasterAppSwc", toaster_app_swc_type
)
fridge_app_swc_type = swc_package.create_application_sw_component_type(
    "FridgeAppSwcType"
)
fridge_app_swc = root_composition_type.create_component(
    "FridgeAppSwc", fridge_app_swc_type
)

# map the software components to the ECUs
root_composition = system.set_root_sw_composition(
    "RootComposition", root_composition_type
)
system_mapping = system.get_or_create_mapping("SystemMapping")
system_mapping.map_swc_to_ecu("ToasterEcuMapping", toaster_app_swc, toaster_ecu)
system_mapping.map_swc_to_ecu("FridgeEcuMapping", fridge_app_swc, fridge_ecu)

# communication over can
can_cluster = system.create_can_cluster("CanBus", comm_package)
can_channel = can_cluster.create_physical_channel("CanChannel")

# both ECUs participate in the same CAN cluster
toaster_can_ctrl = toaster_ecu.create_can_communication_controller("ToasterCanCtrl")
toaster_can_ctrl.connect_physical_channel("Connection", can_channel)
fridge_can_ctrl = fridge_ecu.create_can_communication_controller("FridgeCanCtrl")
fridge_can_ctrl.connect_physical_channel("Connection", can_channel)

# create CAN frames, each containing a PDU and some signals
# Toaster to Fridge communication: Create a frame and trigger it in the channel on id 0x101.
toaster_to_fridge_frame = system.create_can_frame(
    "ToasterToFridgeFrame", frames_package, 8
)
can_channel.trigger_frame(
    toaster_to_fridge_frame, 0x101, CanAddressingMode.Standard, CanFrameType.Can20
)
toaster_to_fridge_pdu = system.create_isignal_ipdu(
    "ToasterToFridgePdu", pdus_package, 8
)
# Create a PDU and trigger it in the frame at byte offset 0. CAN Frames do not allow multiple PDUs
toaster_to_fridge_frame.map_pdu(
    toaster_to_fridge_pdu, 0, ByteOrder.MostSignificantByteLast
)
# Signal "beep" is sent from the toaster to the fridge. It is a 32-bit signal.
toaster_to_fridge_beep_syssignal = system_package.create_system_signal(
    "ToasterToFridgeBeepSysSignal"
)
beep_type = datatypes_package.create_sw_base_type(
    "BeepType",
    32,
    BaseTypeEncoding.TwosComplement,
    byte_order=ByteOrder.MostSignificantByteLast,
    native_declaration="uint32",
)
toaster_to_fridge_beep_signal = system.create_isignal(
    "ToasterToFridgeBeep",
    signals_package,
    32,
    toaster_to_fridge_beep_syssignal,
    datatype=beep_type,
)
toaster_to_fridge_pdu.map_signal(
    toaster_to_fridge_beep_signal,
    0,
    ByteOrder.MostSignificantByteLast,
    transfer_property=TransferProperty.Triggered,
)
toaster_to_fridge_beep_signal.init_value = 1

# Fridge to Toaster communication: Create a frame and trigger it in the channel on id 0x102.
fridge_to_toaster_frame = system.create_can_frame(
    "FridgeToToasterFrame", frames_package, 8
)
can_channel.trigger_frame(
    fridge_to_toaster_frame, 0x102, CanAddressingMode.Standard, CanFrameType.Can20
)
# Create a PDU and trigger it in the frame at byte offset 0. CAN Frames do not allow multiple PDUs
fridge_to_toaster_pdu = system.create_isignal_ipdu(
    "FridgeToToasterPdu", pdus_package, 8
)
fridge_to_toaster_frame.map_pdu(
    fridge_to_toaster_pdu, 0, ByteOrder.MostSignificantByteLast
)
# Signal "toot" is sent from the fridge to the toaster. It is a 16-bit signal.
fridge_to_toaster_toot_syssignal = system_package.create_system_signal(
    "FridgeToToasterTootSysSignal"
)
toot_type = datatypes_package.create_sw_base_type(
    "TootType",
    16,
    BaseTypeEncoding.TwosComplement,
    byte_order=ByteOrder.MostSignificantByteLast,
    native_declaration="uint16",
)
fridge_to_toaster_toot_signal = system.create_isignal(
    "FridgeToToasterToot",
    signals_package,
    16,
    fridge_to_toaster_toot_syssignal,
    datatype=toot_type,
)
fridge_to_toaster_pdu.map_signal(
    fridge_to_toaster_toot_signal,
    0,
    ByteOrder.MostSignificantByteLast,
    transfer_property=TransferProperty.Triggered,
)
fridge_to_toaster_toot_signal.init_value = 0

# create Sender/Receiver interfaces for the PDUs
toaster_to_fridge_interface = interfaces_package.create_sender_receiver_interface(
    "ToasterToFridgeInterface"
)
beep_impl_datatype = datatypes_package.create_implementation_data_type(
    ImplementationDataTypeSettings.Value("beep_impl", base_type=beep_type)
)
toaster_to_fridge_interface_beep = toaster_to_fridge_interface.create_data_element(
    "beep", beep_impl_datatype
)

fridge_to_toaster_interface = interfaces_package.create_sender_receiver_interface(
    "FridgeToToasterInterface"
)
toot_impl_datatype = datatypes_package.create_implementation_data_type(
    ImplementationDataTypeSettings.Value("toot_impl", base_type=toot_type)
)
fridge_to_toaster_interface_toot = fridge_to_toaster_interface.create_data_element(
    "toot", toot_impl_datatype
)

# create ports in the software components
# provide the ToasterToFridge message in the toaster (p-port) and require it in the fridge (r-port)
toaster_p_port = toaster_app_swc_type.create_p_port(
    "ToasterToFridgePort", toaster_to_fridge_interface
)
fridge_r_port = fridge_app_swc_type.create_r_port(
    "ToasterToFridgePort", toaster_to_fridge_interface
)
root_composition_type.create_assembly_connector(
    "ToasterToFridgeConnector",
    toaster_p_port,
    toaster_app_swc,
    fridge_r_port,
    fridge_app_swc,
)
# provide the FridgeToToaster message in the fridge (p-port) and require it in the toaster (r-port)
fridge_p_port = fridge_app_swc_type.create_p_port(
    "FridgeToToasterPort", fridge_to_toaster_interface
)
toaster_r_port = toaster_app_swc_type.create_r_port(
    "FridgeToToasterPort", fridge_to_toaster_interface
)
root_composition_type.create_assembly_connector(
    "FridgeToToasterConnector",
    fridge_p_port,
    fridge_app_swc,
    toaster_r_port,
    toaster_app_swc,
)

# map the ports to the PDUs
system_mapping.map_sender_receiver_to_signal(
    toaster_to_fridge_beep_syssignal,  # the system signal
    toaster_to_fridge_interface_beep,  # is mapped to a data element in the interface
    toaster_p_port,  # using the port in the software component
    [
        toaster_app_swc
    ],  # hierarchy of the software component including any non-root composition
    root_composition_prototype=root_composition,  # the root composition prototype
)
system_mapping.map_sender_receiver_to_signal(
    toaster_to_fridge_beep_syssignal,
    toaster_to_fridge_interface_beep,
    fridge_r_port,
    [fridge_app_swc],
    root_composition_prototype=root_composition,
)
system_mapping.map_sender_receiver_to_signal(
    fridge_to_toaster_toot_syssignal,
    fridge_to_toaster_interface_toot,
    fridge_p_port,
    [fridge_app_swc],
    root_composition_prototype=root_composition,
)
system_mapping.map_sender_receiver_to_signal(
    fridge_to_toaster_toot_syssignal,
    fridge_to_toaster_interface_toot,
    toaster_r_port,
    [toaster_app_swc],
    root_composition_prototype=root_composition,
)

# save the model to the filename that we specified at the beginning
model.write()
```

## API

::: autosar_data.abstraction
