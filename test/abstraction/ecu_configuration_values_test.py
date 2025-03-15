from autosar_data.abstraction import *
from autosar_data.abstraction.ecu_configuration import *


def test_ecuc_value_collection() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    # create an Ecuc value collection
    ecuc_value_collection = package.create_ecuc_value_collection("EcucValueCollection")
    assert isinstance(ecuc_value_collection, EcucValueCollection)
    # get and set the name
    assert ecuc_value_collection.name == "EcucValueCollection"
    ecuc_value_collection.name = "EcucValueCollection2"
    assert ecuc_value_collection.name == "EcucValueCollection2"

    ecuc_module_def = package.create_ecuc_module_def("EcucModuleDef")
    ecuc_configuration_values = package.create_ecuc_module_configuration_values(
        "EcucConfigurationValues", ecuc_module_def
    )
    ecuc_value_collection.add_module_configuration(ecuc_configuration_values)
    assert list(ecuc_value_collection.module_configurations()) == [
        ecuc_configuration_values
    ]

    system = package.create_system("System", SystemCategory.EcuExtract)
    ecuc_value_collection.ecu_extract_reference = system
    assert ecuc_value_collection.ecu_extract_reference == system

    # check if the ecuc value collection can be created from an existing element and is equal to the existing one
    element = ecuc_value_collection.element
    ecuc_value_collection2 = EcucValueCollection(element)
    assert ecuc_value_collection == ecuc_value_collection2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EcucValueCollection.__dict__
    assert ecuc_value_collection.__repr__()


def test_ecuc_module_configuration_values() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    ecuc_module_def = package.create_ecuc_module_def("EcucModuleDef")
    ecuc_container_def = ecuc_module_def.create_param_conf_container_def(
        "EcucContainerDef"
    )
    ecuc_module_def2 = package.create_ecuc_module_def("EcucModuleDef2")

    # EcucModuleConfigurationValues
    ecuc_configuration_values = package.create_ecuc_module_configuration_values(
        "EcucConfigurationValues", ecuc_module_def
    )
    assert isinstance(ecuc_configuration_values, EcucModuleConfigurationValues)
    # get and set the name
    assert ecuc_configuration_values.name == "EcucConfigurationValues"
    ecuc_configuration_values.name = "EcucConfigurationValues2"
    assert ecuc_configuration_values.name == "EcucConfigurationValues2"

    assert ecuc_configuration_values.definition == ecuc_module_def
    ecuc_configuration_values.definition = ecuc_module_def2
    assert ecuc_configuration_values.definition == ecuc_module_def2
    assert ecuc_configuration_values.definition_ref == ecuc_module_def2.element.path

    ecuc_container_value = ecuc_configuration_values.create_container_value(
        "EcucContainerValue", ecuc_container_def
    )
    assert list(ecuc_configuration_values.container_values()) == [ecuc_container_value]

    # check if the ecuc module configuration values can be created from an existing element and is equal to the existing one
    element = ecuc_configuration_values.element
    ecuc_configuration_values2 = EcucModuleConfigurationValues(element)
    assert ecuc_configuration_values == ecuc_configuration_values2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EcucModuleConfigurationValues.__dict__
    assert ecuc_configuration_values.__repr__()


def test_ecuc_container_value() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    ecuc_module_def = package.create_ecuc_module_def("EcucModuleDef")
    ecuc_container_def = ecuc_module_def.create_param_conf_container_def(
        "EcucContainerDef"
    )
    ecuc_container_def2 = ecuc_module_def.create_param_conf_container_def(
        "EcucContainerDef2"
    )
    ecuc_sub_container_def = ecuc_container_def.create_param_conf_container_def(
        "EcucSubContainerDef"
    )
    ecuc_integer_param_def = ecuc_container_def.create_integer_param_def(
        "EcucIntegerParamDef", "Vendor"
    )
    ecuc_string_param_def = ecuc_container_def.create_string_param_def(
        "EcucStringParamDef", "Vendor"
    )
    ecuc_add_info_param_def = ecuc_container_def.create_add_info_param_def(
        "EcucAddInfoParamDef", "Vendor"
    )
    ecuc_instance_reference_def = ecuc_container_def.create_instance_reference_def(
        "EcucInstanceReferenceDef", "Vendor"
    )
    ecuc_reference_def = ecuc_container_def.create_reference_def(
        "EcucReferenceDef", "Vendor"
    )
    ecuc_configuration_values = package.create_ecuc_module_configuration_values(
        "EcucConfigurationValues", ecuc_module_def
    )

    # create a valid target for the instance reference
    composition = package.create_composition_sw_component_type("Composition")
    port_interface = package.create_sender_receiver_interface("SRInterface")
    r_port_prototype = composition.create_r_port("Port", port_interface)

    # EcucContainerValue
    ecuc_container_value = ecuc_configuration_values.create_container_value(
        "EcucContainerValue", ecuc_container_def
    )
    assert isinstance(ecuc_container_value, EcucContainerValue)
    # get and set the name
    assert ecuc_container_value.name == "EcucContainerValue"
    ecuc_container_value.name = "EcucContainerValue2"
    assert ecuc_container_value.name == "EcucContainerValue2"

    assert ecuc_container_value.definition == ecuc_container_def
    ecuc_container_value.definition = ecuc_container_def2
    assert ecuc_container_value.definition == ecuc_container_def2
    assert ecuc_container_value.definition_ref == ecuc_container_def2.element.path

    ecuc_container_value.index = 42
    assert ecuc_container_value.index == 42

    # sub-containers
    sub_container = ecuc_container_value.create_sub_container(
        "EcucSubContainerValue", ecuc_sub_container_def
    )
    assert list(ecuc_container_value.sub_containers()) == [sub_container]

    # parameters
    numerical_value = ecuc_container_value.create_numerical_param_value(
        ecuc_integer_param_def, "42"
    )
    textual_value = ecuc_container_value.create_textual_param_value(
        ecuc_string_param_def, "hello world"
    )
    add_info_value = ecuc_container_value.create_add_info_param_value(
        ecuc_add_info_param_def
    )
    assert list(ecuc_container_value.parameter_values()) == [
        numerical_value,
        textual_value,
        add_info_value,
    ]

    # references
    instance_reference_value = ecuc_container_value.create_instance_reference(
        ecuc_instance_reference_def,
        [r_port_prototype.element],
        r_port_prototype.element,
    )
    reference_value = ecuc_container_value.create_reference_value(
        ecuc_reference_def, package.element
    )
    assert list(ecuc_container_value.reference_values()) == [
        instance_reference_value,
        reference_value,
    ]

    # check if the ecuc container value can be created from an existing element and is equal to the existing one
    element = ecuc_container_value.element
    ecuc_container_value2 = EcucContainerValue(element)
    assert ecuc_container_value == ecuc_container_value2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EcucContainerValue.__dict__
    assert ecuc_container_value.__repr__()


def test_ecuc_add_info_param_value() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    ecuc_module_def = package.create_ecuc_module_def("EcucModuleDef")
    ecuc_container_def = ecuc_module_def.create_param_conf_container_def(
        "EcucContainerDef"
    )
    ecuc_add_info_param_def = ecuc_container_def.create_add_info_param_def(
        "EcucAddInfoParamDef", "Vendor"
    )
    ecuc_configuration_values = package.create_ecuc_module_configuration_values(
        "EcucConfigurationValues", ecuc_module_def
    )
    ecuc_container_value = ecuc_configuration_values.create_container_value(
        "EcucContainerValue", ecuc_container_def
    )

    # EcucAddInfoParamValue
    ecuc_add_info_param_value = ecuc_container_value.create_add_info_param_value(
        ecuc_add_info_param_def
    )
    assert isinstance(ecuc_add_info_param_value, EcucAddInfoParamValue)

    # check if the ecuc add info param value can be created from an existing element and is equal to the existing one
    element = ecuc_add_info_param_value.element
    ecuc_add_info_param_value2 = EcucAddInfoParamValue(element)
    assert ecuc_add_info_param_value == ecuc_add_info_param_value2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EcucAddInfoParamValue.__dict__
    assert ecuc_add_info_param_value.__repr__()


def test_ecuc_numerical_param_value() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    ecuc_module_def = package.create_ecuc_module_def("EcucModuleDef")
    ecuc_container_def = ecuc_module_def.create_param_conf_container_def(
        "EcucContainerDef"
    )
    ecuc_integer_param_def = ecuc_container_def.create_integer_param_def(
        "EcucIntegerParamDef", "Vendor"
    )
    ecuc_configuration_values = package.create_ecuc_module_configuration_values(
        "EcucConfigurationValues", ecuc_module_def
    )
    ecuc_container_value = ecuc_configuration_values.create_container_value(
        "EcucContainerValue", ecuc_container_def
    )

    # EcucNumericalParamValue
    ecuc_numerical_param_value = ecuc_container_value.create_numerical_param_value(
        ecuc_integer_param_def, "42"
    )
    assert isinstance(ecuc_numerical_param_value, EcucNumericalParamValue)

    ecuc_numerical_param_value.value = "43"
    assert ecuc_numerical_param_value.value == "43"
    assert ecuc_numerical_param_value.value_int == 43
    assert ecuc_numerical_param_value.value_float == 43.0
    assert ecuc_numerical_param_value.value_bool is None
    ecuc_numerical_param_value.definition = ecuc_integer_param_def
    assert ecuc_numerical_param_value.definition == ecuc_integer_param_def
    assert (
        ecuc_numerical_param_value.definition_ref == ecuc_integer_param_def.element.path
    )
    ecuc_numerical_param_value.index = 3
    assert ecuc_numerical_param_value.index == 3
    ecuc_numerical_param_value.is_auto_value = True
    assert ecuc_numerical_param_value.is_auto_value

    # check if the ecuc numerical param value can be created from an existing element and is equal to the existing one
    element = ecuc_numerical_param_value.element
    ecuc_numerical_param_value2 = EcucNumericalParamValue(element)
    assert ecuc_numerical_param_value == ecuc_numerical_param_value2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EcucNumericalParamValue.__dict__
    assert ecuc_numerical_param_value.__repr__()


def test_ecuc_textual_param_value() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    ecuc_module_def = package.create_ecuc_module_def("EcucModuleDef")
    ecuc_container_def = ecuc_module_def.create_param_conf_container_def(
        "EcucContainerDef"
    )
    ecuc_string_param_def = ecuc_container_def.create_string_param_def(
        "EcucStringParamDef", "Vendor"
    )
    ecuc_configuration_values = package.create_ecuc_module_configuration_values(
        "EcucConfigurationValues", ecuc_module_def
    )
    ecuc_container_value = ecuc_configuration_values.create_container_value(
        "EcucContainerValue", ecuc_container_def
    )

    # EcucTextualParamValue
    ecuc_textual_param_value = ecuc_container_value.create_textual_param_value(
        ecuc_string_param_def, "hello world"
    )
    assert isinstance(ecuc_textual_param_value, EcucTextualParamValue)

    ecuc_textual_param_value.value = "hello world 2"
    assert ecuc_textual_param_value.value == "hello world 2"
    ecuc_textual_param_value.definition = ecuc_string_param_def
    assert ecuc_textual_param_value.definition == ecuc_string_param_def
    assert ecuc_textual_param_value.definition_ref == ecuc_string_param_def.element.path
    ecuc_textual_param_value.index = 3
    assert ecuc_textual_param_value.index == 3
    ecuc_textual_param_value.is_auto_value = True
    assert ecuc_textual_param_value.is_auto_value

    # check if the ecuc textual param value can be created from an existing element and is equal to the existing one
    element = ecuc_textual_param_value.element
    ecuc_textual_param_value2 = EcucTextualParamValue(element)
    assert ecuc_textual_param_value == ecuc_textual_param_value2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EcucTextualParamValue.__dict__
    assert ecuc_textual_param_value.__repr__()


def test_ecuc_instance_reference_value() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    ecuc_module_def = package.create_ecuc_module_def("EcucModuleDef")
    ecuc_container_def = ecuc_module_def.create_param_conf_container_def(
        "EcucContainerDef"
    )
    ecuc_instance_reference_def = ecuc_container_def.create_instance_reference_def(
        "EcucInstanceReferenceDef", "Vendor"
    )
    ecuc_configuration_values = package.create_ecuc_module_configuration_values(
        "EcucConfigurationValues", ecuc_module_def
    )
    ecuc_container_value = ecuc_configuration_values.create_container_value(
        "EcucContainerValue", ecuc_container_def
    )

    # create a valid target for the instance reference
    composition = package.create_composition_sw_component_type("Composition")
    port_interface = package.create_sender_receiver_interface("SRInterface")
    r_port_prototype = composition.create_r_port("Port", port_interface)

    # EcucInstanceReferenceValue
    ecuc_instance_reference_value = ecuc_container_value.create_instance_reference(
        ecuc_instance_reference_def,
        [r_port_prototype.element],
        r_port_prototype.element,
    )
    assert isinstance(ecuc_instance_reference_value, EcucInstanceReferenceValue)

    ecuc_instance_reference_value.definition = ecuc_instance_reference_def
    assert ecuc_instance_reference_value.definition == ecuc_instance_reference_def
    assert (
        ecuc_instance_reference_value.definition_ref
        == ecuc_instance_reference_def.element.path
    )
    ecuc_instance_reference_value.index = 3
    assert ecuc_instance_reference_value.index == 3
    ecuc_instance_reference_value.is_auto_value = True
    assert ecuc_instance_reference_value.is_auto_value
    ecuc_instance_reference_value.target = ([], r_port_prototype.element)
    assert ecuc_instance_reference_value.target == ([], r_port_prototype.element)

    # check if the ecuc instance reference value can be created from an existing element and is equal to the existing one
    element = ecuc_instance_reference_value.element
    ecuc_instance_reference_value2 = EcucInstanceReferenceValue(element)
    assert ecuc_instance_reference_value == ecuc_instance_reference_value2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EcucInstanceReferenceValue.__dict__
    assert ecuc_instance_reference_value.__repr__()


def test_ecuc_reference_value() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    ecuc_module_def = package.create_ecuc_module_def("EcucModuleDef")
    ecuc_container_def = ecuc_module_def.create_param_conf_container_def(
        "EcucContainerDef"
    )
    ecuc_reference_def = ecuc_container_def.create_reference_def(
        "EcucReferenceDef", "Vendor"
    )
    ecuc_configuration_values = package.create_ecuc_module_configuration_values(
        "EcucConfigurationValues", ecuc_module_def
    )
    ecuc_container_value = ecuc_configuration_values.create_container_value(
        "EcucContainerValue", ecuc_container_def
    )

    # EcucReferenceValue
    ecuc_reference_value = ecuc_container_value.create_reference_value(
        ecuc_reference_def, package.element
    )
    assert isinstance(ecuc_reference_value, EcucReferenceValue)

    ecuc_reference_value.definition = ecuc_reference_def
    assert ecuc_reference_value.definition == ecuc_reference_def
    assert ecuc_reference_value.definition_ref == ecuc_reference_def.element.path
    ecuc_reference_value.index = 3
    assert ecuc_reference_value.index == 3
    ecuc_reference_value.is_auto_value = True
    assert ecuc_reference_value.is_auto_value
    ecuc_reference_value.target = package.element
    assert ecuc_reference_value.target == package.element

    # check if the ecuc reference value can be created from an existing element and is equal to the existing one
    element = ecuc_reference_value.element
    ecuc_reference_value2 = EcucReferenceValue(element)
    assert ecuc_reference_value == ecuc_reference_value2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EcucReferenceValue.__dict__
    assert ecuc_reference_value.__repr__()
