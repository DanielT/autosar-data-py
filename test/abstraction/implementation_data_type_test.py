from autosar_data.abstraction import *
from autosar_data.abstraction.datatype import *


def test_implementation_data_type_value() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    sw_base_type = package.create_sw_base_type(
        "SwBaseType", 32, BaseTypeEncoding.TwosComplement
    )
    compu_method = package.create_compu_method(
        "CompuMethod", CompuMethodContent.Identical()
    )
    data_constr = package.create_data_constr("DataConstraint")

    # ImplementationDataType
    settings = ImplementationDataTypeSettings.Value(
        "ImplValue",
        base_type=sw_base_type,
        compu_method=compu_method,
        data_constraint=data_constr,
    )
    implementation_data_type = package.create_implementation_data_type(settings)
    assert isinstance(implementation_data_type, ImplementationDataType)
    assert implementation_data_type.settings() == settings

    implementation_data_type.name = "ImplementationDataType2"
    assert implementation_data_type.name == "ImplementationDataType2"

    assert implementation_data_type.category == ImplementationDataCategory.Value
    assert implementation_data_type.base_type == sw_base_type
    assert implementation_data_type.data_constraint == data_constr
    assert implementation_data_type.compu_method == compu_method
    assert list(implementation_data_type.sub_elements()) == []
    assert implementation_data_type.referenced_type is None
    assert implementation_data_type.array_size is None
    assert implementation_data_type.data_pointer_target is None

    other_settings = ImplementationDataTypeSettings.Array(
        "ImplArray",
        length=10,
        element_type=ImplementationDataTypeSettings.Value(
            "ImplValue", base_type=sw_base_type
        ),
    )
    implementation_data_type.apply_settings(other_settings)
    assert implementation_data_type.settings() == other_settings

    # check if the implementation data type can be constructed from a name and is equal to the original implementation data type
    element = implementation_data_type.element
    implementation_data_type2 = ImplementationDataType(element)
    assert implementation_data_type == implementation_data_type2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ImplementationDataType.__dict__
    assert len(str(implementation_data_type)) > 0
    assert "__repr__" in ImplementationDataTypeSettings.Value.__dict__
    assert len(str(settings)) > 0


def test_implmentation_data_type_array() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    sw_base_type = package.create_sw_base_type(
        "SwBaseType", 32, BaseTypeEncoding.TwosComplement
    )

    # ImplementationDataType
    settings = ImplementationDataTypeSettings.Array(
        "ImplArray",
        length=10,
        element_type=ImplementationDataTypeSettings.Value(
            "ImplValue", base_type=sw_base_type
        ),
    )
    implementation_data_type = package.create_implementation_data_type(settings)
    assert isinstance(implementation_data_type, ImplementationDataType)
    assert implementation_data_type.settings() == settings

    implementation_data_type.name = "ImplementationDataType2"
    assert implementation_data_type.name == "ImplementationDataType2"

    assert implementation_data_type.category == ImplementationDataCategory.Array
    assert implementation_data_type.base_type is None
    assert implementation_data_type.data_constraint is None
    assert implementation_data_type.compu_method is None
    assert len(list(implementation_data_type.sub_elements())) == 1
    assert implementation_data_type.referenced_type is None
    assert implementation_data_type.array_size == 10
    assert implementation_data_type.data_pointer_target is None

    # check if the implementation data type can be constructed from a name and is equal to the original implementation data type
    element = implementation_data_type.element
    implementation_data_type2 = ImplementationDataType(element)
    assert implementation_data_type == implementation_data_type2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ImplementationDataType.__dict__
    assert len(str(implementation_data_type)) > 0
    assert "__repr__" in ImplementationDataTypeSettings.Array.__dict__
    assert len(str(settings)) > 0


def test_implmentation_data_type_structure() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    sw_base_type = package.create_sw_base_type(
        "SwBaseType", 32, BaseTypeEncoding.TwosComplement
    )

    # ImplementationDataType
    settings = ImplementationDataTypeSettings.Structure(
        "ImplStruct",
        elements=[
            ImplementationDataTypeSettings.Value("ImplValue", base_type=sw_base_type)
        ],
    )
    implementation_data_type = package.create_implementation_data_type(settings)
    assert isinstance(implementation_data_type, ImplementationDataType)
    assert implementation_data_type.settings() == settings

    implementation_data_type.name = "ImplementationDataType2"
    assert implementation_data_type.name == "ImplementationDataType2"

    assert implementation_data_type.category == ImplementationDataCategory.Structure
    assert implementation_data_type.base_type is None
    assert implementation_data_type.data_constraint is None
    assert implementation_data_type.compu_method is None
    assert len(list(implementation_data_type.sub_elements())) == 1
    assert implementation_data_type.referenced_type is None
    assert implementation_data_type.array_size is None
    assert implementation_data_type.data_pointer_target is None

    # check if the implementation data type can be constructed from a name and is equal to the original implementation data type
    element = implementation_data_type.element
    implementation_data_type2 = ImplementationDataType(element)
    assert implementation_data_type == implementation_data_type2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ImplementationDataType.__dict__
    assert len(str(implementation_data_type)) > 0
    assert "__repr__" in ImplementationDataTypeSettings.Structure.__dict__
    assert len(str(settings)) > 0


def test_implmentation_data_type_union() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    sw_base_type = package.create_sw_base_type(
        "SwBaseType", 32, BaseTypeEncoding.TwosComplement
    )

    # ImplementationDataType
    settings = ImplementationDataTypeSettings.Union(
        "ImplUnion",
        elements=[
            ImplementationDataTypeSettings.Value("ImplValue", base_type=sw_base_type)
        ],
    )
    implementation_data_type = package.create_implementation_data_type(settings)
    assert isinstance(implementation_data_type, ImplementationDataType)
    assert implementation_data_type.settings() == settings

    implementation_data_type.name = "ImplementationDataType2"
    assert implementation_data_type.name == "ImplementationDataType2"

    assert implementation_data_type.category == ImplementationDataCategory.Union
    assert implementation_data_type.base_type is None
    assert implementation_data_type.data_constraint is None
    assert implementation_data_type.compu_method is None
    assert len(list(implementation_data_type.sub_elements())) == 1
    assert implementation_data_type.referenced_type is None
    assert implementation_data_type.array_size is None
    assert implementation_data_type.data_pointer_target is None

    # check if the implementation data type can be constructed from a name and is equal to the original implementation data type
    element = implementation_data_type.element
    implementation_data_type2 = ImplementationDataType(element)
    assert implementation_data_type == implementation_data_type2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ImplementationDataType.__dict__
    assert len(str(implementation_data_type)) > 0
    assert "__repr__" in ImplementationDataTypeSettings.Union.__dict__
    assert len(str(settings)) > 0


def test_implmentation_data_type_type_reference() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    sw_base_type = package.create_sw_base_type(
        "SwBaseType", 32, BaseTypeEncoding.TwosComplement
    )
    settings_target = ImplementationDataTypeSettings.Value(
        "ImplValue", base_type=sw_base_type
    )
    implementation_data_type_target = package.create_implementation_data_type(
        settings_target
    )
    compu_method = package.create_compu_method(
        "CompuMethod", CompuMethodContent.Identical()
    )
    data_constr = package.create_data_constr("DataConstraint")

    # ImplementationDataType
    settings = ImplementationDataTypeSettings.TypeReference(
        "ImplRef",
        reftype=implementation_data_type_target,
        compu_method=compu_method,
        data_constraint=data_constr,
    )
    implementation_data_type = package.create_implementation_data_type(settings)
    assert isinstance(implementation_data_type, ImplementationDataType)
    assert implementation_data_type.settings() == settings

    implementation_data_type.name = "ImplementationDataType2"
    assert implementation_data_type.name == "ImplementationDataType2"

    assert implementation_data_type.category == ImplementationDataCategory.TypeReference
    assert implementation_data_type.base_type is None
    assert implementation_data_type.data_constraint == data_constr
    assert implementation_data_type.compu_method == compu_method
    assert list(implementation_data_type.sub_elements()) == []
    assert implementation_data_type.referenced_type == implementation_data_type_target
    assert implementation_data_type.array_size is None
    assert implementation_data_type.data_pointer_target is None

    # check if the implementation data type can be constructed from a name and is equal to the original implementation data type
    element = implementation_data_type.element
    implementation_data_type2 = ImplementationDataType(element)
    assert implementation_data_type == implementation_data_type2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ImplementationDataType.__dict__
    assert len(str(implementation_data_type)) > 0
    assert "__repr__" in ImplementationDataTypeSettings.TypeReference.__dict__
    assert len(str(settings)) > 0


def test_implmentation_data_type_data_reference() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    sw_base_type = package.create_sw_base_type(
        "SwBaseType", 32, BaseTypeEncoding.TwosComplement
    )
    settings_target = ImplementationDataTypeSettings.Value(
        "ImplValue", base_type=sw_base_type
    )
    implementation_data_type_target = package.create_implementation_data_type(
        settings_target
    )

    # ImplementationDataType
    settings = ImplementationDataTypeSettings.DataReference(
        "ImplRef",
        target=implementation_data_type_target,
    )
    implementation_data_type = package.create_implementation_data_type(settings)
    assert isinstance(implementation_data_type, ImplementationDataType)
    assert implementation_data_type.settings() == settings

    implementation_data_type.name = "ImplementationDataType2"
    assert implementation_data_type.name == "ImplementationDataType2"

    assert implementation_data_type.category == ImplementationDataCategory.DataReference
    assert implementation_data_type.base_type is None
    assert implementation_data_type.data_constraint is None
    assert implementation_data_type.compu_method is None
    assert list(implementation_data_type.sub_elements()) == []
    assert implementation_data_type.referenced_type is None
    assert implementation_data_type.array_size is None
    assert (
        implementation_data_type.data_pointer_target == implementation_data_type_target
    )

    # check if the implementation data type can be constructed from a name and is equal to the original implementation data type
    element = implementation_data_type.element
    implementation_data_type2 = ImplementationDataType(element)
    assert implementation_data_type == implementation_data_type2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ImplementationDataType.__dict__
    assert len(str(implementation_data_type)) > 0
    assert "__repr__" in ImplementationDataTypeSettings.DataReference.__dict__
    assert len(str(settings)) > 0


def test_implementation_data_type_function_reference() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    # ImplementationDataType
    settings = ImplementationDataTypeSettings.FunctionReference(
        "ImplFuncRef",
    )
    implementation_data_type = package.create_implementation_data_type(settings)
    assert isinstance(implementation_data_type, ImplementationDataType)
    assert implementation_data_type.settings() == settings

    implementation_data_type.name = "ImplementationDataType2"
    assert implementation_data_type.name == "ImplementationDataType2"

    assert (
        implementation_data_type.category
        == ImplementationDataCategory.FunctionReference
    )
    assert implementation_data_type.base_type is None
    assert implementation_data_type.data_constraint is None
    assert implementation_data_type.compu_method is None
    assert list(implementation_data_type.sub_elements()) == []
    assert implementation_data_type.referenced_type is None
    assert implementation_data_type.array_size is None
    assert implementation_data_type.data_pointer_target is None

    # check if the implementation data type can be constructed from a name and is equal to the original implementation data type
    element = implementation_data_type.element
    implementation_data_type2 = ImplementationDataType(element)
    assert implementation_data_type == implementation_data_type2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ImplementationDataType.__dict__
    assert len(str(implementation_data_type)) > 0
    assert "__repr__" in ImplementationDataTypeSettings.FunctionReference.__dict__
    assert len(str(settings)) > 0


def test_implmentation_data_type_element() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    sw_base_type = package.create_sw_base_type(
        "SwBaseType", 32, BaseTypeEncoding.TwosComplement
    )
    other_impl_type = package.create_implementation_data_type(
        ImplementationDataTypeSettings.Value("OtherImplType", base_type=sw_base_type)
    )

    # ImplementationDataType
    settings_element_0 = ImplementationDataTypeSettings.Value(
        "ImplValue", base_type=sw_base_type
    )
    settings_element_1 = ImplementationDataTypeSettings.Union(
        "ImplUnion",
        elements=[
            ImplementationDataTypeSettings.Value(
                "UnionImplValue", base_type=sw_base_type
            )
        ],
    )
    settings_element_2 = ImplementationDataTypeSettings.Structure(
        "ImplStruct",
        elements=[
            ImplementationDataTypeSettings.Value(
                "StructImplValue", base_type=sw_base_type
            )
        ],
    )
    settings_element_3 = ImplementationDataTypeSettings.Array(
        "ImplArray",
        length=10,
        element_type=ImplementationDataTypeSettings.Value(
            "ArrayImplValue", base_type=sw_base_type
        ),
    )
    settings_element_4 = ImplementationDataTypeSettings.TypeReference(
        "ImplTypeRef", reftype=other_impl_type
    )
    settings_element_5 = ImplementationDataTypeSettings.DataReference(
        "ImplDataRef", target=sw_base_type
    )
    settings_element_6 = ImplementationDataTypeSettings.FunctionReference("ImplFuncRef")
    settings = ImplementationDataTypeSettings.Structure(
        "ImpleDataType",
        elements=[
            settings_element_0,
            settings_element_1,
            settings_element_2,
            settings_element_3,
            settings_element_4,
            settings_element_5,
            settings_element_6,
        ],
    )
    implementation_data_type = package.create_implementation_data_type(settings)
    assert isinstance(implementation_data_type, ImplementationDataType)
    assert implementation_data_type.settings() == settings

    elements = list(implementation_data_type.sub_elements())
    assert len(elements) == 7

    assert isinstance(elements[0], ImplementationDataTypeElement)
    assert elements[0].settings() == settings_element_0
    assert elements[0].name == "ImplValue"
    assert elements[0].category == ImplementationDataCategory.Value
    assert elements[0].base_type == sw_base_type
    assert elements[0].data_constraint is None
    assert elements[0].compu_method is None
    assert elements[0].array_size is None
    assert len(list(elements[0].sub_elements())) == 0
    assert elements[0].referenced_type is None
    assert elements[0].data_pointer_target is None
    elements[0].name = "ImplValue2"
    assert elements[0].name == "ImplValue2"
    other_settings = ImplementationDataTypeSettings.Value(
        "abcdef", base_type=sw_base_type
    )
    elements[0].apply_settings(other_settings)
    assert elements[0].settings() == other_settings

    assert isinstance(elements[1], ImplementationDataTypeElement)
    print(elements[1].settings())
    print(settings_element_1)
    assert elements[1].settings() == settings_element_1
    assert elements[1].name == "ImplUnion"
    assert elements[1].category == ImplementationDataCategory.Union
    assert elements[1].base_type is None
    assert elements[1].data_constraint is None
    assert elements[1].compu_method is None
    assert elements[1].array_size is None
    assert len(list(elements[1].sub_elements())) == 1
    assert elements[1].referenced_type is None
    assert elements[1].data_pointer_target is None

    assert isinstance(elements[2], ImplementationDataTypeElement)
    assert elements[2].settings() == settings_element_2
    assert elements[2].name == "ImplStruct"
    assert elements[2].category == ImplementationDataCategory.Structure
    assert elements[2].base_type is None
    assert elements[2].data_constraint is None
    assert elements[2].compu_method is None
    assert elements[2].array_size is None
    assert len(list(elements[2].sub_elements())) == 1
    assert elements[2].referenced_type is None
    assert elements[2].data_pointer_target is None

    assert isinstance(elements[3], ImplementationDataTypeElement)
    assert elements[3].settings() == settings_element_3
    assert elements[3].name == "ImplArray"
    assert elements[3].category == ImplementationDataCategory.Array
    assert elements[3].base_type is None
    assert elements[3].data_constraint is None
    assert elements[3].compu_method is None
    assert elements[3].array_size == 10
    assert len(list(elements[3].sub_elements())) == 1
    assert elements[3].referenced_type is None
    assert elements[3].data_pointer_target is None

    assert isinstance(elements[4], ImplementationDataTypeElement)
    assert elements[4].settings() == settings_element_4
    assert elements[4].name == "ImplTypeRef"
    assert elements[4].category == ImplementationDataCategory.TypeReference
    assert elements[4].base_type is None
    assert elements[4].data_constraint is None
    assert elements[4].compu_method is None
    assert elements[4].array_size is None
    assert len(list(elements[4].sub_elements())) == 0
    assert elements[4].referenced_type == other_impl_type
    assert elements[4].data_pointer_target is None

    assert isinstance(elements[5], ImplementationDataTypeElement)
    assert elements[5].settings() == settings_element_5
    assert elements[5].name == "ImplDataRef"
    assert elements[5].category == ImplementationDataCategory.DataReference
    assert elements[5].base_type is None
    assert elements[5].data_constraint is None
    assert elements[5].compu_method is None
    assert elements[5].array_size is None
    assert len(list(elements[5].sub_elements())) == 0
    assert elements[5].referenced_type is None
    assert elements[5].data_pointer_target == sw_base_type

    assert isinstance(elements[6], ImplementationDataTypeElement)
    assert elements[6].settings() == settings_element_6
    assert elements[6].name == "ImplFuncRef"
    assert elements[6].category == ImplementationDataCategory.FunctionReference
    assert elements[6].base_type is None
    assert elements[6].data_constraint is None
    assert elements[6].compu_method is None
    assert elements[6].array_size is None
    assert len(list(elements[6].sub_elements())) == 0
    assert elements[6].referenced_type is None
    assert elements[6].data_pointer_target is None

    # check if the implementation data type can be constructed from a name and is equal to the original implementation data type
    element = elements[6].element
    implementation_data_type_element_copy = ImplementationDataTypeElement(element)
    assert elements[6] == implementation_data_type_element_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ImplementationDataTypeElement.__dict__
    assert len(str(elements[0])) > 0
    assert len(str(elements[1])) > 0
    assert len(str(elements[2])) > 0
    assert len(str(elements[3])) > 0
    assert len(str(elements[4])) > 0
    assert len(str(elements[5])) > 0
    assert len(str(elements[6])) > 0
