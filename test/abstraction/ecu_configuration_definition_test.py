from autosar_data.abstraction import *
from autosar_data.abstraction.ecu_configuration import *
from typing import Any


def test_ecuc_definition_collection() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    # Create an EcuC definition collection
    ecuc_definition_collection = package.create_ecuc_definition_collection("EcuC")
    assert isinstance(ecuc_definition_collection, EcucDefinitionCollection)
    # get and set the name
    assert ecuc_definition_collection.name == "EcuC"
    ecuc_definition_collection.name = "EcuC1"
    assert ecuc_definition_collection.name == "EcuC1"

    ecuc_module_def = package.create_ecuc_module_def("EcuCModuleDef")
    ecuc_definition_collection.add_module_def(ecuc_module_def)
    assert list(ecuc_definition_collection.module_defs()) == [ecuc_module_def]

    # check if the definition collection can be constructed from an element and is equal to the original one
    element = ecuc_definition_collection.element
    ecuc_definition_collection2 = EcucDefinitionCollection(element)
    assert ecuc_definition_collection == ecuc_definition_collection2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EcucDefinitionCollection.__dict__
    assert ecuc_definition_collection.__repr__()


def test_ecuc_module_def() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    # Create an EcuC module definition
    ecuc_module_def = package.create_ecuc_module_def("EcuCModuleDef")
    assert isinstance(ecuc_module_def, EcucModuleDef)
    # get and set the name
    assert ecuc_module_def.name == "EcuCModuleDef"
    ecuc_module_def.name = "EcuCModuleDef1"
    assert ecuc_module_def.name == "EcuCModuleDef1"

    choice_container = ecuc_module_def.create_choice_container_def("ChoiceContainer")
    conf_container = ecuc_module_def.create_param_conf_container_def("ConfContainer")
    assert list(ecuc_module_def.containers()) == [choice_container, conf_container]

    ecuc_module_def.api_service_prefix = "whatever"
    assert ecuc_module_def.api_service_prefix == "whatever"
    ecuc_module_def.supported_config_variants = [
        EcucConfigurationVariant.PreconfiguredConfiguration,
        EcucConfigurationVariant.VariantLinkTime,
        EcucConfigurationVariant.RecommendedConfiguration,
        EcucConfigurationVariant.VariantPostBuild,
        EcucConfigurationVariant.VariantPreCompile,
        EcucConfigurationVariant.VariantPostBuildLoadable,
        EcucConfigurationVariant.VariantPostBuildSelectable,
    ]
    assert ecuc_module_def.supported_config_variants == [
        EcucConfigurationVariant.PreconfiguredConfiguration,
        EcucConfigurationVariant.VariantLinkTime,
        EcucConfigurationVariant.RecommendedConfiguration,
        EcucConfigurationVariant.VariantPostBuild,
        EcucConfigurationVariant.VariantPreCompile,
        EcucConfigurationVariant.VariantPostBuildLoadable,
        EcucConfigurationVariant.VariantPostBuildSelectable,
    ]
    ecuc_module_def.post_build_variant_support = True
    assert ecuc_module_def.post_build_variant_support is True
    ecuc_module_def.category = EcucModuleDefCategory.VendorSpecificModuleDefinition
    assert (
        ecuc_module_def.category == EcucModuleDefCategory.VendorSpecificModuleDefinition
    )

    other_module_def = package.create_ecuc_module_def("OtherModuleDef")
    ecuc_module_def.refined_module_def = other_module_def
    assert ecuc_module_def.refined_module_def == other_module_def

    # attributes inherited from EcucDefinitionElement
    ecuc_module_def.lower_multiplicity = 1
    assert ecuc_module_def.lower_multiplicity == 1
    ecuc_module_def.upper_multiplicity = 2
    assert ecuc_module_def.upper_multiplicity == 2
    ecuc_module_def.upper_multiplicity_infinite = True
    assert ecuc_module_def.upper_multiplicity_infinite is True
    assert (
        ecuc_module_def.upper_multiplicity is None
    )  # cleared when upper_multiplicity_infinite is set to True

    # check if the module definition can be constructed from an element and is equal to the original one
    element = ecuc_module_def.element
    ecuc_module_def2 = EcucModuleDef(element)
    assert ecuc_module_def == ecuc_module_def2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EcucModuleDef.__dict__
    assert ecuc_module_def.__repr__()


def test_ecuc_destination_uri_def_set() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    # Create an EcuC destination URI definition set
    ecuc_destination_uri_def_set = package.create_ecuc_destination_uri_def_set(
        "EcuCDestinationUriDefSet"
    )
    assert isinstance(ecuc_destination_uri_def_set, EcucDestinationUriDefSet)
    # get and set the name
    assert ecuc_destination_uri_def_set.name == "EcuCDestinationUriDefSet"
    ecuc_destination_uri_def_set.name = "EcuCDestinationUriDefSet1"
    assert ecuc_destination_uri_def_set.name == "EcuCDestinationUriDefSet1"

    ecuc_destination_uri_def = ecuc_destination_uri_def_set.create_destination_uri_def(
        "EcuCDestinationUriDef", EcucDestinationUriNestingContract.LeafOfTargetContainer
    )
    assert list(ecuc_destination_uri_def_set.destination_uri_defs()) == [
        ecuc_destination_uri_def
    ]

    # check if the destination URI definition set can be constructed from an element and is equal to the original one
    element = ecuc_destination_uri_def_set.element
    ecuc_destination_uri_def_set2 = EcucDestinationUriDefSet(element)
    assert ecuc_destination_uri_def_set == ecuc_destination_uri_def_set2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EcucDestinationUriDefSet.__dict__
    assert ecuc_destination_uri_def_set.__repr__()


def test_ecuc_destination_uri_def() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    # Create an EcuC destination URI definition
    ecuc_destination_uri_def_set = package.create_ecuc_destination_uri_def_set(
        "EcuCDestinationUriDefSet"
    )
    ecuc_destination_uri_def = ecuc_destination_uri_def_set.create_destination_uri_def(
        "EcuCDestinationUriDef", EcucDestinationUriNestingContract.LeafOfTargetContainer
    )
    assert isinstance(ecuc_destination_uri_def, EcucDestinationUriDef)
    # get and set the name
    assert ecuc_destination_uri_def.name == "EcuCDestinationUriDef"
    ecuc_destination_uri_def.name = "EcuCDestinationUriDef1"
    assert ecuc_destination_uri_def.name == "EcuCDestinationUriDef1"

    choice_container = ecuc_destination_uri_def.create_choice_container_def(
        "ChoiceContainer"
    )
    conf_container = ecuc_destination_uri_def.create_param_conf_container_def(
        "ConfContainer"
    )
    assert list(ecuc_destination_uri_def.containers()) == [
        choice_container,
        conf_container,
    ]
    ecuc_destination_uri_def.nesting_contract = (
        EcucDestinationUriNestingContract.TargetContainer
    )
    assert (
        ecuc_destination_uri_def.nesting_contract
        == EcucDestinationUriNestingContract.TargetContainer
    )
    ecuc_destination_uri_def.nesting_contract = (
        EcucDestinationUriNestingContract.VertexOfTargetContainer
    )
    assert (
        ecuc_destination_uri_def.nesting_contract
        == EcucDestinationUriNestingContract.VertexOfTargetContainer
    )
    ecuc_destination_uri_def.nesting_contract = (
        EcucDestinationUriNestingContract.LeafOfTargetContainer
    )
    assert (
        ecuc_destination_uri_def.nesting_contract
        == EcucDestinationUriNestingContract.LeafOfTargetContainer
    )

    # check if the destination URI definition can be constructed from an element and is equal to the original one
    element = ecuc_destination_uri_def.element
    ecuc_destination_uri_def2 = EcucDestinationUriDef(element)
    assert ecuc_destination_uri_def == ecuc_destination_uri_def2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EcucDestinationUriDef.__dict__
    assert ecuc_destination_uri_def.__repr__()


def test_ecuc_choice_container_def() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    ecuc_module_def = package.create_ecuc_module_def("EcuCModuleDef")

    # Create an EcuC choice container definition
    ecuc_choice_container_def = ecuc_module_def.create_choice_container_def(
        "ChoiceContainer"
    )
    assert isinstance(ecuc_choice_container_def, EcucChoiceContainerDef)
    # get and set the name
    assert ecuc_choice_container_def.name == "ChoiceContainer"
    ecuc_choice_container_def.name = "ChoiceContainer1"
    assert ecuc_choice_container_def.name == "ChoiceContainer1"

    conf_container1 = ecuc_choice_container_def.create_param_conf_container_def(
        "ConfContainer1"
    )
    conf_container2 = ecuc_choice_container_def.create_param_conf_container_def(
        "ConfContainer2"
    )
    assert list(ecuc_choice_container_def.choices()) == [
        conf_container1,
        conf_container2,
    ]

    # attributes inherited from EcucDefinitionElement
    ecuc_choice_container_def.lower_multiplicity = 1
    assert ecuc_choice_container_def.lower_multiplicity == 1
    ecuc_choice_container_def.upper_multiplicity = 2
    assert ecuc_choice_container_def.upper_multiplicity == 2
    ecuc_choice_container_def.upper_multiplicity_infinite = True
    assert ecuc_choice_container_def.upper_multiplicity_infinite is True
    assert (
        ecuc_choice_container_def.upper_multiplicity is None
    )  # cleared when upper_multiplicity_infinite is set to True

    # check if the choice container definition can be constructed from an element and is equal to the original one
    element = ecuc_choice_container_def.element
    ecuc_choice_container_def2 = EcucChoiceContainerDef(element)
    assert ecuc_choice_container_def == ecuc_choice_container_def2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EcucChoiceContainerDef.__dict__
    assert ecuc_choice_container_def.__repr__()


def test_ecuc_param_conf_container_def() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    ecuc_module_def = package.create_ecuc_module_def("EcuCModuleDef")

    # Create an EcuC parameter configuration container definition
    ecuc_param_conf_container_def = ecuc_module_def.create_param_conf_container_def(
        "ConfContainer"
    )
    assert isinstance(ecuc_param_conf_container_def, EcucParamConfContainerDef)
    # get and set the name
    assert ecuc_param_conf_container_def.name == "ConfContainer"
    ecuc_param_conf_container_def.name = "ConfContainer1"
    assert ecuc_param_conf_container_def.name == "ConfContainer1"

    # sub containers
    choice_container = ecuc_param_conf_container_def.create_choice_container_def(
        "ChoiceContainer"
    )
    param_container = ecuc_param_conf_container_def.create_param_conf_container_def(
        "ParamContainer"
    )
    assert list(ecuc_param_conf_container_def.sub_containers()) == [
        choice_container,
        param_container,
    ]

    # parameters
    add_info_param_def = ecuc_param_conf_container_def.create_add_info_param_def(
        "AddInfoParamDef", "Vendor"
    )
    boolean_param_def = ecuc_param_conf_container_def.create_boolean_param_def(
        "BooleanParamDef", "Vendor"
    )
    enum_param_def = ecuc_param_conf_container_def.create_enumeration_param_def(
        "EnumParamDef", "Vendor"
    )
    float_param_def = ecuc_param_conf_container_def.create_float_param_def(
        "FloatParamDef", "Vendor"
    )
    integer_param_def = ecuc_param_conf_container_def.create_integer_param_def(
        "IntegerParamDef", "Vendor"
    )
    string_param_def = ecuc_param_conf_container_def.create_string_param_def(
        "StringParamDef", "Vendor"
    )
    linker_symbol_param_def = (
        ecuc_param_conf_container_def.create_linker_symbol_param_def(
            "LinkerSymbolParamDef", "Vendor"
        )
    )
    function_name_param_def = (
        ecuc_param_conf_container_def.create_function_name_param_def(
            "FunctionNameParamDef", "Vendor"
        )
    )
    multiline_string_param_def = (
        ecuc_param_conf_container_def.create_multiline_string_param_def(
            "MultilineStringParamDef", "Vendor"
        )
    )
    assert list(ecuc_param_conf_container_def.parameters()) == [
        add_info_param_def,
        boolean_param_def,
        enum_param_def,
        float_param_def,
        integer_param_def,
        string_param_def,
        linker_symbol_param_def,
        function_name_param_def,
        multiline_string_param_def,
    ]

    # references
    reference_def = ecuc_param_conf_container_def.create_reference_def(
        "ReferenceDef", "Vendor"
    )
    instance_reference_def = (
        ecuc_param_conf_container_def.create_instance_reference_def(
            "InstanceReferenceDef", "Vendor"
        )
    )
    choice_reference_def = ecuc_param_conf_container_def.create_choice_reference_def(
        "ChoiceReferenceDef", "Vendor"
    )
    foreign_reference_def = ecuc_param_conf_container_def.create_foreign_reference_def(
        "ForeignReferenceDef", "Vendor"
    )
    uri_reference_def = ecuc_param_conf_container_def.create_uri_reference_def(
        "UriReferenceDef", "Vendor"
    )
    assert list(ecuc_param_conf_container_def.references()) == [
        reference_def,
        instance_reference_def,
        choice_reference_def,
        foreign_reference_def,
        uri_reference_def,
    ]

    # attributes inherited from EcucDefinitionElement
    ecuc_param_conf_container_def.lower_multiplicity = 1
    assert ecuc_param_conf_container_def.lower_multiplicity == 1
    ecuc_param_conf_container_def.upper_multiplicity = 2
    assert ecuc_param_conf_container_def.upper_multiplicity == 2
    ecuc_param_conf_container_def.upper_multiplicity_infinite = True
    assert ecuc_param_conf_container_def.upper_multiplicity_infinite is True
    assert (
        ecuc_param_conf_container_def.upper_multiplicity is None
    )  # cleared when upper_multiplicity_infinite is set to True

    # check if the parameter configuration container definition can be constructed from an element and is equal to the original one
    element = ecuc_param_conf_container_def.element
    ecuc_param_conf_container_def2 = EcucParamConfContainerDef(element)
    assert ecuc_param_conf_container_def == ecuc_param_conf_container_def2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EcucParamConfContainerDef.__dict__
    assert ecuc_param_conf_container_def.__repr__()


def test_ecuc_add_info_param_def() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    ecuc_module_def = package.create_ecuc_module_def("EcuCModuleDef")
    ecuc_param_conf_container_def = ecuc_module_def.create_param_conf_container_def(
        "ConfContainer"
    )

    # Create an EcuC additional info parameter definition
    ecuc_add_info_param_def = ecuc_param_conf_container_def.create_add_info_param_def(
        "AddInfoParamDef", "Vendor"
    )
    assert isinstance(ecuc_add_info_param_def, EcucAddInfoParamDef)
    # get and set the name
    assert ecuc_add_info_param_def.name == "AddInfoParamDef"
    ecuc_add_info_param_def.name = "AddInfoParamDef1"
    assert ecuc_add_info_param_def.name == "AddInfoParamDef1"

    helper_ecuc_common_attributes(ecuc_add_info_param_def)

    # check if the additional info parameter definition can be constructed from an element and is equal to the original one
    element = ecuc_add_info_param_def.element
    ecuc_add_info_param_def2 = EcucAddInfoParamDef(element)
    assert ecuc_add_info_param_def == ecuc_add_info_param_def2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EcucAddInfoParamDef.__dict__
    assert ecuc_add_info_param_def.__repr__()


def test_ecuc_boolean_param_def() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    ecuc_module_def = package.create_ecuc_module_def("EcuCModuleDef")
    ecuc_param_conf_container_def = ecuc_module_def.create_param_conf_container_def(
        "ConfContainer"
    )

    # Create an EcuC boolean parameter definition
    ecuc_boolean_param_def = ecuc_param_conf_container_def.create_boolean_param_def(
        "BooleanParamDef", "Vendor"
    )
    assert isinstance(ecuc_boolean_param_def, EcucBooleanParamDef)
    # get and set the name
    assert ecuc_boolean_param_def.name == "BooleanParamDef"
    ecuc_boolean_param_def.name = "BooleanParamDef1"
    assert ecuc_boolean_param_def.name == "BooleanParamDef1"

    ecuc_boolean_param_def.default_value = True
    assert ecuc_boolean_param_def.default_value is True

    helper_ecuc_common_attributes(ecuc_boolean_param_def)

    # check if the boolean parameter definition can be constructed from an element and is equal to the original one
    element = ecuc_boolean_param_def.element
    ecuc_boolean_param_def2 = EcucBooleanParamDef(element)
    assert ecuc_boolean_param_def == ecuc_boolean_param_def2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EcucBooleanParamDef.__dict__
    assert ecuc_boolean_param_def.__repr__()


def test_ecuc_enumeration_param_def() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    ecuc_module_def = package.create_ecuc_module_def("EcuCModuleDef")
    ecuc_param_conf_container_def = ecuc_module_def.create_param_conf_container_def(
        "ConfContainer"
    )

    # Create an EcuC enumeration parameter definition
    ecuc_enum_param_def = ecuc_param_conf_container_def.create_enumeration_param_def(
        "EnumParamDef", "Vendor"
    )
    assert isinstance(ecuc_enum_param_def, EcucEnumerationParamDef)
    # get and set the name
    assert ecuc_enum_param_def.name == "EnumParamDef"
    ecuc_enum_param_def.name = "EnumParamDef1"
    assert ecuc_enum_param_def.name == "EnumParamDef1"

    literal1 = ecuc_enum_param_def.create_enumeration_literal("Literal1")
    literal2 = ecuc_enum_param_def.create_enumeration_literal("Literal2")
    assert list(ecuc_enum_param_def.enumeration_literals()) == [literal1, literal2]

    ecuc_enum_param_def.default_value = "Literal1"
    assert ecuc_enum_param_def.default_value == "Literal1"

    helper_ecuc_common_attributes(ecuc_enum_param_def)

    # check if the enumeration parameter definition can be constructed from an element and is equal to the original one
    element = ecuc_enum_param_def.element
    ecuc_enum_param_def2 = EcucEnumerationParamDef(element)
    assert ecuc_enum_param_def == ecuc_enum_param_def2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EcucEnumerationParamDef.__dict__
    assert ecuc_enum_param_def.__repr__()


def test_ecuc_enumeration_literal_def() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    ecuc_module_def = package.create_ecuc_module_def("EcuCModuleDef")
    ecuc_param_conf_container_def = ecuc_module_def.create_param_conf_container_def(
        "ConfContainer"
    )
    ecuc_enum_param_def = ecuc_param_conf_container_def.create_enumeration_param_def(
        "EnumParamDef", "Vendor"
    )

    # Create an EcuC enumeration literal
    ecuc_enum_literal = ecuc_enum_param_def.create_enumeration_literal("Literal")
    assert isinstance(ecuc_enum_literal, EcucEnumerationLiteralDef)
    # get and set the name
    assert ecuc_enum_literal.name == "Literal"
    ecuc_enum_literal.name = "Literal1"
    assert ecuc_enum_literal.name == "Literal1"

    # check if the enumeration literal can be constructed from an element and is equal to the original one
    element = ecuc_enum_literal.element
    ecuc_enum_literal2 = EcucEnumerationLiteralDef(element)
    assert ecuc_enum_literal == ecuc_enum_literal2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EcucEnumerationLiteralDef.__dict__
    assert ecuc_enum_literal.__repr__()


def test_ecuc_float_param_def() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    ecuc_module_def = package.create_ecuc_module_def("EcuCModuleDef")
    ecuc_param_conf_container_def = ecuc_module_def.create_param_conf_container_def(
        "ConfContainer"
    )

    # Create an EcuC float parameter definition
    ecuc_float_param_def = ecuc_param_conf_container_def.create_float_param_def(
        "FloatParamDef", "Vendor"
    )
    assert isinstance(ecuc_float_param_def, EcucFloatParamDef)
    # get and set the name
    assert ecuc_float_param_def.name == "FloatParamDef"
    ecuc_float_param_def.name = "FloatParamDef1"
    assert ecuc_float_param_def.name == "FloatParamDef1"

    ecuc_float_param_def.default_value = 1.0
    assert ecuc_float_param_def.default_value == 1.0
    ecuc_float_param_def.min = 0.0
    assert ecuc_float_param_def.min == 0.0
    ecuc_float_param_def.max = 2.0
    assert ecuc_float_param_def.max == 2.0

    helper_ecuc_common_attributes(ecuc_float_param_def)

    # check if the float parameter definition can be constructed from an element and is equal to the original one
    element = ecuc_float_param_def.element
    ecuc_float_param_def2 = EcucFloatParamDef(element)
    assert ecuc_float_param_def == ecuc_float_param_def2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EcucFloatParamDef.__dict__
    assert ecuc_float_param_def.__repr__()


def test_ecuc_integer_param_def() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    ecuc_module_def = package.create_ecuc_module_def("EcuCModuleDef")
    ecuc_param_conf_container_def = ecuc_module_def.create_param_conf_container_def(
        "ConfContainer"
    )

    # Create an EcuC integer parameter definition
    ecuc_integer_param_def = ecuc_param_conf_container_def.create_integer_param_def(
        "IntegerParamDef", "Vendor"
    )
    assert isinstance(ecuc_integer_param_def, EcucIntegerParamDef)
    # get and set the name
    assert ecuc_integer_param_def.name == "IntegerParamDef"
    ecuc_integer_param_def.name = "IntegerParamDef1"
    assert ecuc_integer_param_def.name == "IntegerParamDef1"

    ecuc_integer_param_def.default_value = 1
    assert ecuc_integer_param_def.default_value == 1
    ecuc_integer_param_def.min = 0
    assert ecuc_integer_param_def.min == 0
    ecuc_integer_param_def.max = 2
    assert ecuc_integer_param_def.max == 2

    helper_ecuc_common_attributes(ecuc_integer_param_def)

    # check if the integer parameter definition can be constructed from an element and is equal to the original one
    element = ecuc_integer_param_def.element
    ecuc_integer_param_def2 = EcucIntegerParamDef(element)
    assert ecuc_integer_param_def == ecuc_integer_param_def2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EcucIntegerParamDef.__dict__
    assert ecuc_integer_param_def.__repr__()


def test_ecuc_string_param_def() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    ecuc_module_def = package.create_ecuc_module_def("EcuCModuleDef")
    ecuc_param_conf_container_def = ecuc_module_def.create_param_conf_container_def(
        "ConfContainer"
    )

    # Create an EcuC string parameter definition
    ecuc_string_param_def = ecuc_param_conf_container_def.create_string_param_def(
        "StringParamDef", "Vendor"
    )
    assert isinstance(ecuc_string_param_def, EcucStringParamDef)
    # get and set the name
    assert ecuc_string_param_def.name == "StringParamDef"
    ecuc_string_param_def.name = "StringParamDef1"
    assert ecuc_string_param_def.name == "StringParamDef1"

    helper_ecuc_string_param(ecuc_string_param_def)
    helper_ecuc_common_attributes(ecuc_string_param_def)

    # check if the string parameter definition can be constructed from an element and is equal to the original one
    element = ecuc_string_param_def.element
    ecuc_string_param_def2 = EcucStringParamDef(element)
    assert ecuc_string_param_def == ecuc_string_param_def2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EcucStringParamDef.__dict__
    assert ecuc_string_param_def.__repr__()


def test_ecuc_linker_symbol_param_def() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    ecuc_module_def = package.create_ecuc_module_def("EcuCModuleDef")
    ecuc_param_conf_container_def = ecuc_module_def.create_param_conf_container_def(
        "ConfContainer"
    )

    # Create an EcuC linker symbol parameter definition
    ecuc_linker_symbol_param_def = (
        ecuc_param_conf_container_def.create_linker_symbol_param_def(
            "LinkerSymbolParamDef", "Vendor"
        )
    )
    assert isinstance(ecuc_linker_symbol_param_def, EcucLinkerSymbolDef)
    # get and set the name
    assert ecuc_linker_symbol_param_def.name == "LinkerSymbolParamDef"
    ecuc_linker_symbol_param_def.name = "LinkerSymbolParamDef1"
    assert ecuc_linker_symbol_param_def.name == "LinkerSymbolParamDef1"

    helper_ecuc_string_param(ecuc_linker_symbol_param_def)
    helper_ecuc_common_attributes(ecuc_linker_symbol_param_def)

    # check if the linker symbol parameter definition can be constructed from an element and is equal to the original one
    element = ecuc_linker_symbol_param_def.element
    ecuc_linker_symbol_param_def2 = EcucLinkerSymbolDef(element)
    assert ecuc_linker_symbol_param_def == ecuc_linker_symbol_param_def2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EcucLinkerSymbolDef.__dict__
    assert ecuc_linker_symbol_param_def.__repr__()


def test_ecuc_function_name_param_def() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    ecuc_module_def = package.create_ecuc_module_def("EcuCModuleDef")
    ecuc_param_conf_container_def = ecuc_module_def.create_param_conf_container_def(
        "ConfContainer"
    )

    # Create an EcuC function name parameter definition
    ecuc_function_name_param_def = (
        ecuc_param_conf_container_def.create_function_name_param_def(
            "FunctionNameParamDef", "Vendor"
        )
    )
    assert isinstance(ecuc_function_name_param_def, EcucFunctionNameDef)
    # get and set the name
    assert ecuc_function_name_param_def.name == "FunctionNameParamDef"
    ecuc_function_name_param_def.name = "FunctionNameParamDef1"
    assert ecuc_function_name_param_def.name == "FunctionNameParamDef1"

    helper_ecuc_string_param(ecuc_function_name_param_def)
    helper_ecuc_common_attributes(ecuc_function_name_param_def)

    # check if the function name parameter definition can be constructed from an element and is equal to the original one
    element = ecuc_function_name_param_def.element
    ecuc_function_name_param_def2 = EcucFunctionNameDef(element)
    assert ecuc_function_name_param_def == ecuc_function_name_param_def2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EcucFunctionNameDef.__dict__
    assert ecuc_function_name_param_def.__repr__()


def test_ecuc_multiline_string_param_def() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    ecuc_module_def = package.create_ecuc_module_def("EcuCModuleDef")
    ecuc_param_conf_container_def = ecuc_module_def.create_param_conf_container_def(
        "ConfContainer"
    )

    # Create an EcuC multiline string parameter definition
    ecuc_multiline_string_param_def = (
        ecuc_param_conf_container_def.create_multiline_string_param_def(
            "MultilineStringParamDef", "Vendor"
        )
    )
    assert isinstance(ecuc_multiline_string_param_def, EcucMultilineStringParamDef)
    # get and set the name
    assert ecuc_multiline_string_param_def.name == "MultilineStringParamDef"
    ecuc_multiline_string_param_def.name = "MultilineStringParamDef1"
    assert ecuc_multiline_string_param_def.name == "MultilineStringParamDef1"

    helper_ecuc_string_param(ecuc_multiline_string_param_def)
    helper_ecuc_common_attributes(ecuc_multiline_string_param_def)

    # check if the multiline string parameter definition can be constructed from an element and is equal to the original one
    element = ecuc_multiline_string_param_def.element
    ecuc_multiline_string_param_def2 = EcucMultilineStringParamDef(element)
    assert ecuc_multiline_string_param_def == ecuc_multiline_string_param_def2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EcucMultilineStringParamDef.__dict__
    assert ecuc_multiline_string_param_def.__repr__()


def test_ecuc_foreign_reference_def() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    ecuc_module_def = package.create_ecuc_module_def("EcuCModuleDef")
    ecuc_param_conf_container_def = ecuc_module_def.create_param_conf_container_def(
        "ConfContainer"
    )

    # Create an EcuC foreign reference definition
    ecuc_foreign_reference_def = (
        ecuc_param_conf_container_def.create_foreign_reference_def(
            "ForeignReferenceDef", "Vendor"
        )
    )
    assert isinstance(ecuc_foreign_reference_def, EcucForeignReferenceDef)
    # get and set the name
    assert ecuc_foreign_reference_def.name == "ForeignReferenceDef"
    ecuc_foreign_reference_def.name = "ForeignReferenceDef1"
    assert ecuc_foreign_reference_def.name == "ForeignReferenceDef1"

    ecuc_foreign_reference_def.destination_type = "DestinationType"
    assert ecuc_foreign_reference_def.destination_type == "DestinationType"

    helper_ecuc_common_attributes(ecuc_foreign_reference_def)

    # check if the foreign reference definition can be constructed from an element and is equal to the original one
    element = ecuc_foreign_reference_def.element
    ecuc_foreign_reference_def2 = EcucForeignReferenceDef(element)
    assert ecuc_foreign_reference_def == ecuc_foreign_reference_def2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EcucForeignReferenceDef.__dict__
    assert ecuc_foreign_reference_def.__repr__()


def test_ecuc_instance_reference_def() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    ecuc_module_def = package.create_ecuc_module_def("EcuCModuleDef")
    ecuc_param_conf_container_def = ecuc_module_def.create_param_conf_container_def(
        "ConfContainer"
    )

    # Create an EcuC instance reference definition
    ecuc_instance_reference_def = (
        ecuc_param_conf_container_def.create_instance_reference_def(
            "InstanceReferenceDef", "Vendor"
        )
    )
    assert isinstance(ecuc_instance_reference_def, EcucInstanceReferenceDef)
    # get and set the name
    assert ecuc_instance_reference_def.name == "InstanceReferenceDef"
    ecuc_instance_reference_def.name = "InstanceReferenceDef1"
    assert ecuc_instance_reference_def.name == "InstanceReferenceDef1"

    ecuc_instance_reference_def.destination_type = "DestinationType"
    assert ecuc_instance_reference_def.destination_type == "DestinationType"
    ecuc_instance_reference_def.destination_context = "ABC DEF GHI"
    assert ecuc_instance_reference_def.destination_context == "ABC DEF GHI"

    helper_ecuc_common_attributes(ecuc_instance_reference_def)

    # check if the instance reference definition can be constructed from an element and is equal to the original one
    element = ecuc_instance_reference_def.element
    ecuc_instance_reference_def2 = EcucInstanceReferenceDef(element)
    assert ecuc_instance_reference_def == ecuc_instance_reference_def2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EcucInstanceReferenceDef.__dict__
    assert ecuc_instance_reference_def.__repr__()


def test_ecuc_choice_reference_def() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    ecuc_module_def = package.create_ecuc_module_def("EcuCModuleDef")
    ecuc_param_conf_container_def = ecuc_module_def.create_param_conf_container_def(
        "ConfContainer"
    )
    ecuc_param_conf_container_def2 = ecuc_module_def.create_param_conf_container_def(
        "ConfContainer2"
    )

    # Create an EcuC choice reference definition
    ecuc_choice_reference_def = (
        ecuc_param_conf_container_def.create_choice_reference_def(
            "ChoiceReferenceDef", "Vendor"
        )
    )
    assert isinstance(ecuc_choice_reference_def, EcucChoiceReferenceDef)
    # get and set the name
    assert ecuc_choice_reference_def.name == "ChoiceReferenceDef"
    ecuc_choice_reference_def.name = "ChoiceReferenceDef1"
    assert ecuc_choice_reference_def.name == "ChoiceReferenceDef1"

    ecuc_choice_reference_def.add_destination(ecuc_param_conf_container_def2)
    assert list(ecuc_choice_reference_def.destination_refs()) == [
        ecuc_param_conf_container_def2
    ]

    helper_ecuc_common_attributes(ecuc_choice_reference_def)

    # check if the choice reference definition can be constructed from an element and is equal to the original one
    element = ecuc_choice_reference_def.element
    ecuc_choice_reference_def2 = EcucChoiceReferenceDef(element)
    assert ecuc_choice_reference_def == ecuc_choice_reference_def2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EcucChoiceReferenceDef.__dict__
    assert ecuc_choice_reference_def.__repr__()


def test_ecuc_reference_def() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    ecuc_module_def = package.create_ecuc_module_def("EcuCModuleDef")
    ecuc_param_conf_container_def = ecuc_module_def.create_param_conf_container_def(
        "ConfContainer"
    )
    ecuc_param_conf_container_def2 = ecuc_module_def.create_param_conf_container_def(
        "ConfContainer2"
    )

    # Create an EcuC reference definition
    ecuc_reference_def = ecuc_param_conf_container_def.create_reference_def(
        "ReferenceDef", "Vendor"
    )
    assert isinstance(ecuc_reference_def, EcucReferenceDef)
    # get and set the name
    assert ecuc_reference_def.name == "ReferenceDef"
    ecuc_reference_def.name = "ReferenceDef1"
    assert ecuc_reference_def.name == "ReferenceDef1"

    ecuc_reference_def.destination = ecuc_param_conf_container_def2
    assert ecuc_reference_def.destination == ecuc_param_conf_container_def2

    helper_ecuc_common_attributes(ecuc_reference_def)

    # check if the reference definition can be constructed from an element and is equal to the original one
    element = ecuc_reference_def.element
    ecuc_reference_def2 = EcucReferenceDef(element)
    assert ecuc_reference_def == ecuc_reference_def2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EcucReferenceDef.__dict__
    assert ecuc_reference_def.__repr__()


def test_ecuc_uri_reference_def() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    ecuc_module_def = package.create_ecuc_module_def("EcuCModuleDef")
    ecuc_param_conf_container_def = ecuc_module_def.create_param_conf_container_def(
        "ConfContainer"
    )
    ecuc_destination_uri_def_set = package.create_ecuc_destination_uri_def_set(
        "DestinationUriDefSet"
    )
    ecuc_destination_uri_def = ecuc_destination_uri_def_set.create_destination_uri_def(
        "DestinationUriDef",
        EcucDestinationUriNestingContract.TargetContainer,
    )

    # Create an EcuC URI reference definition
    ecuc_uri_reference_def = ecuc_param_conf_container_def.create_uri_reference_def(
        "UriReferenceDef", "Vendor"
    )
    assert isinstance(ecuc_uri_reference_def, EcucUriReferenceDef)
    # get and set the name
    assert ecuc_uri_reference_def.name == "UriReferenceDef"
    ecuc_uri_reference_def.name = "UriReferenceDef1"
    assert ecuc_uri_reference_def.name == "UriReferenceDef1"

    ecuc_uri_reference_def.destination_uri = ecuc_destination_uri_def
    assert ecuc_uri_reference_def.destination_uri == ecuc_destination_uri_def

    helper_ecuc_common_attributes(ecuc_uri_reference_def)

    # check if the URI reference definition can be constructed from an element and is equal to the original one
    element = ecuc_uri_reference_def.element
    ecuc_uri_reference_def2 = EcucUriReferenceDef(element)
    assert ecuc_uri_reference_def == ecuc_uri_reference_def2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in EcucUriReferenceDef.__dict__
    assert ecuc_uri_reference_def.__repr__()


def helper_ecuc_definition_element(def_elem: Any) -> None:
    # attributes inherited from EcucDefinitionElement
    def_elem.lower_multiplicity = 1
    assert def_elem.lower_multiplicity == 1
    def_elem.upper_multiplicity = 2
    assert def_elem.upper_multiplicity == 2
    def_elem.upper_multiplicity_infinite = True
    assert def_elem.upper_multiplicity_infinite is True
    assert (
        def_elem.upper_multiplicity is None
    )  # cleared when upper_multiplicity_infinite is set to True


def helper_ecuc_common_attributes(common_attrs_elem: Any) -> None:
    # attributes inherited from EcucCommonAttributes
    common_attrs_elem.multiplicity_config_classes = [
        (
            EcucConfigurationClass.PostBuild,
            EcucConfigurationVariant.VariantPostBuild,
        ),
        (
            EcucConfigurationClass.PreCompile,
            EcucConfigurationVariant.VariantPreCompile,
        ),
    ]
    assert common_attrs_elem.multiplicity_config_classes == [
        (
            EcucConfigurationClass.PostBuild,
            EcucConfigurationVariant.VariantPostBuild,
        ),
        (
            EcucConfigurationClass.PreCompile,
            EcucConfigurationVariant.VariantPreCompile,
        ),
    ]
    common_attrs_elem.origin = "Vendor"
    assert common_attrs_elem.origin == "Vendor"
    common_attrs_elem.post_build_variant_multiplicity = True
    assert common_attrs_elem.post_build_variant_multiplicity is True
    common_attrs_elem.post_build_variant_value = True
    assert common_attrs_elem.post_build_variant_value is True
    common_attrs_elem.requires_index = True
    assert common_attrs_elem.requires_index is True
    common_attrs_elem.value_config_classes = [
        (
            EcucConfigurationClass.PostBuild,
            EcucConfigurationVariant.VariantPostBuild,
        ),
        (
            EcucConfigurationClass.PreCompile,
            EcucConfigurationVariant.VariantPreCompile,
        ),
    ]
    assert common_attrs_elem.value_config_classes == [
        (
            EcucConfigurationClass.PostBuild,
            EcucConfigurationVariant.VariantPostBuild,
        ),
        (
            EcucConfigurationClass.PreCompile,
            EcucConfigurationVariant.VariantPreCompile,
        ),
    ]
    common_attrs_elem.with_auto = True
    assert common_attrs_elem.with_auto is True

    # every EcucCommonAttribute is also an EcucDefinitionElement
    helper_ecuc_definition_element(common_attrs_elem)


def helper_ecuc_string_param(string_param: Any) -> None:
    string_param.default_value = "default"
    assert string_param.default_value == "default"
    string_param.min_length = 1
    assert string_param.min_length == 1
    string_param.max_length = 2
    assert string_param.max_length == 2
    string_param.regular_expression = "regex"
    assert string_param.regular_expression == "regex"
