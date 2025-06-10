from autosar_data.abstraction import *
from autosar_data.abstraction.datatype import *


def test_application_array_data_type() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    compu_method = package.create_compu_method(
        "Compumethod", CompuMethodContent.Identical()
    )
    unit = package.create_unit("Unit")
    data_constr = package.create_data_constr("DataConstraint")

    element_type = package.create_application_primitive_data_type(
        "ApplicationPrimitiveDataType",
        ApplicationPrimitiveCategory.Value,
        compu_method=compu_method,
        unit=unit,
        data_constraint=data_constr,
    )
    array_size_spec = ApplicationArraySize.Fixed(10)

    # ApplicationArrayDataType
    application_array_data_type = package.create_application_array_data_type(
        "ApplicationArrayDataType", element_type, array_size_spec
    )
    assert isinstance(application_array_data_type, ApplicationArrayDataType)

    application_array_data_type.name = "ApplicationArrayDataType2"
    assert application_array_data_type.name == "ApplicationArrayDataType2"
    assert isinstance(
        application_array_data_type.array_element, ApplicationArrayElement
    )
    assert application_array_data_type.array_element.data_type == element_type

    application_array_data_type.set_size(array_size_spec)
    assert application_array_data_type.size() == array_size_spec

    # array of record data types
    record_data_type = package.create_application_record_data_type(
        "ApplicationRecordDataType"
    )
    array_size_spec = ApplicationArraySize.Fixed(10)
    application_array_data_type2 = package.create_application_array_data_type(
        "ArrayOfRecords", record_data_type, array_size_spec
    )
    assert isinstance(application_array_data_type2, ApplicationArrayDataType)

    # array of arrays
    array_size_spec = ApplicationArraySize.Fixed(10)
    application_array_data_type3 = package.create_application_array_data_type(
        "ArrayOfArrays", application_array_data_type, array_size_spec
    )
    assert isinstance(application_array_data_type3, ApplicationArrayDataType)

    # check if the application array data type can be constructed from an element and is equal to the original application array data type
    element = application_array_data_type.element
    application_array_data_type_copy = ApplicationArrayDataType(element)
    assert application_array_data_type == application_array_data_type_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ApplicationArrayDataType.__dict__
    assert len(str(application_array_data_type)) > 0


def test_application_array_size() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    element_type = package.create_application_primitive_data_type(
        "ApplicationPrimitiveDataType", ApplicationPrimitiveCategory.Value
    )
    application_array_data_type = package.create_application_array_data_type(
        "ApplicationArrayDataType", element_type, ApplicationArraySize.Fixed(10)
    )

    # ApplicationArraySize
    size_fixed = ApplicationArraySize.Fixed(10)
    application_array_data_type.set_size(size_fixed)
    assert application_array_data_type.size() == size_fixed

    size_var_linear = ApplicationArraySize.VariableLinear(20)
    application_array_data_type.set_size(size_var_linear)
    assert application_array_data_type.size() == size_var_linear

    # create an array of array data types for the following tests
    size_var_square = ApplicationArraySize.VariableSquare()
    application_array_data_type2 = package.create_application_array_data_type(
        "ApplicationArrayDataType2", application_array_data_type, size_var_square
    )
    assert application_array_data_type2.size() == size_var_square

    size_var_rect = ApplicationArraySize.VariableRectangular(30)
    application_array_data_type2.set_size(size_var_rect)
    assert application_array_data_type2.size() == size_var_rect

    size_var_flex = ApplicationArraySize.VariableFullyFlexible(40)
    application_array_data_type2.set_size(size_var_flex)
    assert application_array_data_type2.size() == size_var_flex


def test_application_array_element() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    element_type = package.create_application_primitive_data_type(
        "ApplicationPrimitiveDataType", ApplicationPrimitiveCategory.Value
    )
    application_array_data_type = package.create_application_array_data_type(
        "ApplicationArrayDataType", element_type, ApplicationArraySize.Fixed(10)
    )

    # ApplicationArrayElement
    array_element = application_array_data_type.array_element
    array_element.data_type = element_type
    assert array_element.data_type == element_type

    array_element.name = "ApplicationArrayElement"
    assert array_element.name == "ApplicationArrayElement"

    # check if the application array element can be constructed from an application array data type and is equal to the original application array element
    element = array_element.element
    array_element2 = ApplicationArrayElement(element)
    assert array_element == array_element2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ApplicationArrayElement.__dict__
    assert len(str(array_element)) > 0


def test_application_primitive_data_type() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    # ApplicationPrimitiveDataType
    primitive_data_type = package.create_application_primitive_data_type(
        "ApplicationPrimitiveDataType", ApplicationPrimitiveCategory.Value
    )
    assert isinstance(primitive_data_type, ApplicationPrimitiveDataType)

    primitive_data_type.name = "ApplicationPrimitiveDataType2"
    assert primitive_data_type.name == "ApplicationPrimitiveDataType2"
    assert primitive_data_type.category == ApplicationPrimitiveCategory.Value

    primitive_data_type.category = ApplicationPrimitiveCategory.Boolean
    assert primitive_data_type.category == ApplicationPrimitiveCategory.Boolean
    primitive_data_type.category = ApplicationPrimitiveCategory.ComAxis
    assert primitive_data_type.category == ApplicationPrimitiveCategory.ComAxis
    primitive_data_type.category = ApplicationPrimitiveCategory.Cube4
    assert primitive_data_type.category == ApplicationPrimitiveCategory.Cube4
    primitive_data_type.category = ApplicationPrimitiveCategory.Cube5
    assert primitive_data_type.category == ApplicationPrimitiveCategory.Cube5
    primitive_data_type.category = ApplicationPrimitiveCategory.Cuboid
    assert primitive_data_type.category == ApplicationPrimitiveCategory.Cuboid
    primitive_data_type.category = ApplicationPrimitiveCategory.Curve
    assert primitive_data_type.category == ApplicationPrimitiveCategory.Curve
    primitive_data_type.category = ApplicationPrimitiveCategory.Map
    assert primitive_data_type.category == ApplicationPrimitiveCategory.Map
    primitive_data_type.category = ApplicationPrimitiveCategory.ResAxis
    assert primitive_data_type.category == ApplicationPrimitiveCategory.ResAxis
    primitive_data_type.category = ApplicationPrimitiveCategory.ValBlk
    assert primitive_data_type.category == ApplicationPrimitiveCategory.ValBlk
    primitive_data_type.category = ApplicationPrimitiveCategory.String
    assert primitive_data_type.category == ApplicationPrimitiveCategory.String

    compu_method = package.create_compu_method(
        "CompuMethod", CompuMethodContent.Identical()
    )
    primitive_data_type.compu_method = compu_method
    assert primitive_data_type.compu_method == compu_method

    unit = package.create_unit("Unit")
    primitive_data_type.unit = unit
    assert primitive_data_type.unit == unit

    data_constraint = package.create_data_constr("DataConstraint")
    primitive_data_type.data_constraint = data_constraint
    assert primitive_data_type.data_constraint == data_constraint

    # check if the application primitive data type can be constructed from a category and is equal to the original application primitive data type
    element = primitive_data_type.element
    primitive_data_type2 = ApplicationPrimitiveDataType(element)
    assert primitive_data_type == primitive_data_type2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ApplicationPrimitiveDataType.__dict__
    assert len(str(primitive_data_type)) > 0


def test_application_record_data_type() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    # ApplicationRecordDataType
    record_data_type = package.create_application_record_data_type(
        "ApplicationRecordDataType"
    )
    assert isinstance(record_data_type, ApplicationRecordDataType)

    record_data_type.name = "ApplicationRecordDataType2"
    assert record_data_type.name == "ApplicationRecordDataType2"

    element_type = package.create_application_primitive_data_type(
        "Element_ApplicationPrimitiveDataType", ApplicationPrimitiveCategory.Value
    )
    record_element = record_data_type.create_record_element("Element1", element_type)
    assert isinstance(record_element, ApplicationRecordElement)

    element_type2 = package.create_application_record_data_type(
        "Element_ApplicationRecordDataType"
    )
    record_element2 = record_data_type.create_record_element("Element2", element_type2)
    assert isinstance(record_element2, ApplicationRecordElement)

    element_type3 = package.create_application_array_data_type(
        "Element_ApplicationArrayDataType", element_type, ApplicationArraySize.Fixed(10)
    )
    record_element3 = record_data_type.create_record_element("Element3", element_type3)
    assert isinstance(record_element3, ApplicationRecordElement)

    record_element_list = list(record_data_type.record_elements())
    assert record_element_list == [record_element, record_element2, record_element3]
    assert record_element_list[0].data_type == element_type
    assert record_element_list[1].data_type == element_type2
    assert record_element_list[2].data_type == element_type3

    # check if the application record data type can be constructed from a name and is equal to the original application record data type
    element = record_data_type.element
    record_data_type2 = ApplicationRecordDataType(element)
    assert record_data_type == record_data_type2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ApplicationRecordDataType.__dict__
    assert len(str(record_data_type)) > 0


def test_application_record_element() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    record_data_type = package.create_application_record_data_type(
        "ApplicationRecordDataType"
    )
    element_type = package.create_application_primitive_data_type(
        "ApplicationPrimitiveDataType", ApplicationPrimitiveCategory.Value
    )
    record_element = record_data_type.create_record_element("Element1", element_type)

    # ApplicationRecordElement
    assert record_element.name == "Element1"
    assert record_element.data_type == element_type

    record_element.name = "Element2"
    assert record_element.name == "Element2"

    record_element.data_type = element_type
    assert record_element.data_type == element_type

    # check if the application record element can be constructed from a name and data type and is equal to the original application record element
    element = record_element.element
    record_element2 = ApplicationRecordElement(element)
    assert record_element == record_element2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ApplicationRecordElement.__dict__
    assert len(str(record_element)) > 0


def test_sw_base_type() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    # SwBaseType
    sw_base_type = package.create_sw_base_type(
        "SwBaseType", 32, BaseTypeEncoding.TwosComplement
    )
    assert isinstance(sw_base_type, SwBaseType)

    sw_base_type.name = "SwBaseType2"
    assert sw_base_type.name == "SwBaseType2"

    sw_base_type.bit_length = 64
    assert sw_base_type.bit_length == 64

    sw_base_type.base_type_encoding = BaseTypeEncoding.OnesComplement
    assert sw_base_type.base_type_encoding == BaseTypeEncoding.OnesComplement
    sw_base_type.base_type_encoding = BaseTypeEncoding.TwosComplement
    assert sw_base_type.base_type_encoding == BaseTypeEncoding.TwosComplement
    sw_base_type.base_type_encoding = BaseTypeEncoding.SignMagnitude
    assert sw_base_type.base_type_encoding == BaseTypeEncoding.SignMagnitude
    sw_base_type.base_type_encoding = BaseTypeEncoding.BcdPacked
    assert sw_base_type.base_type_encoding == BaseTypeEncoding.BcdPacked
    sw_base_type.base_type_encoding = BaseTypeEncoding.BcdUnpacked
    assert sw_base_type.base_type_encoding == BaseTypeEncoding.BcdUnpacked
    sw_base_type.base_type_encoding = BaseTypeEncoding.DspFractional
    assert sw_base_type.base_type_encoding == BaseTypeEncoding.DspFractional
    sw_base_type.base_type_encoding = BaseTypeEncoding.Ieee754
    assert sw_base_type.base_type_encoding == BaseTypeEncoding.Ieee754
    sw_base_type.base_type_encoding = BaseTypeEncoding.Iso8859_1
    assert sw_base_type.base_type_encoding == BaseTypeEncoding.Iso8859_1
    sw_base_type.base_type_encoding = BaseTypeEncoding.Iso8859_2
    assert sw_base_type.base_type_encoding == BaseTypeEncoding.Iso8859_2
    sw_base_type.base_type_encoding = BaseTypeEncoding.Windows1252
    assert sw_base_type.base_type_encoding == BaseTypeEncoding.Windows1252
    sw_base_type.base_type_encoding = BaseTypeEncoding.Utf8
    assert sw_base_type.base_type_encoding == BaseTypeEncoding.Utf8
    sw_base_type.base_type_encoding = BaseTypeEncoding.Utf16
    assert sw_base_type.base_type_encoding == BaseTypeEncoding.Utf16
    sw_base_type.base_type_encoding = BaseTypeEncoding.Ucs2
    assert sw_base_type.base_type_encoding == BaseTypeEncoding.Ucs2
    sw_base_type.base_type_encoding = BaseTypeEncoding.Boolean
    assert sw_base_type.base_type_encoding == BaseTypeEncoding.Boolean
    sw_base_type.base_type_encoding = BaseTypeEncoding.Void
    assert sw_base_type.base_type_encoding == BaseTypeEncoding.Void
    sw_base_type.base_type_encoding = BaseTypeEncoding.NoEncoding
    assert sw_base_type.base_type_encoding == BaseTypeEncoding.NoEncoding

    sw_base_type.native_declaration = "int32_t"
    assert sw_base_type.native_declaration == "int32_t"
    sw_base_type.mem_alignment = 4
    assert sw_base_type.mem_alignment == 4
    sw_base_type.byte_order = ByteOrder.MostSignificantByteFirst
    assert sw_base_type.byte_order == ByteOrder.MostSignificantByteFirst

    # check if the sw base type can be constructed from a name, bit length, and base type encoding and is equal to the original sw base type
    element = sw_base_type.element
    sw_base_type2 = SwBaseType(element)
    assert sw_base_type == sw_base_type2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in SwBaseType.__dict__
    assert len(str(sw_base_type)) > 0


def test_compu_method() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    # CompuMethod
    compu_method = package.create_compu_method(
        "CompuMethod", CompuMethodContent.Identical()
    )
    assert isinstance(compu_method, CompuMethod)

    compu_method.name = "CompuMethod2"
    assert compu_method.name == "CompuMethod2"
    assert compu_method.category == CompuMethodCategory.Identical

    # create scale linear content with a single compu-scale
    content = CompuMethodContent.ScaleLinear(
        scales=[
            LinearConversionParameters(
                direction=CompuScaleDirection.IntToPhys,
                offset=0.0,
                factor=1.0,
                divisor=1.0,
                lower_limit=0.0,
                upper_limit=100.0,
            )
        ]
    )
    compu_method.set_content(content)
    assert compu_method.content() == content
    assert compu_method.category == CompuMethodCategory.ScaleLinear

    # manually create a second compu-scale
    cs = compu_method.create_compu_scale(
        CompuScaleDirection.IntToPhys, lower_limit=100.0, upper_limit=200.0
    )
    assert isinstance(cs, CompuScale)
    assert len(list(compu_method.int_to_phys_compu_scales())) == 2

    # replace the existing content with a new scale linear content with a single compu-scale
    # this time the direction is PhysToInt
    content2 = CompuMethodContent.ScaleLinear(
        scales=[
            LinearConversionParameters(
                direction=CompuScaleDirection.PhysToInt,
                offset=0.0,
                factor=1.0,
                divisor=1.0,
                lower_limit=0.0,
                upper_limit=100.0,
            )
        ]
    )
    compu_method.set_content(content2)
    assert compu_method.content() == content2

    # manually create a second compu-scale
    cs = compu_method.create_compu_scale(
        CompuScaleDirection.PhysToInt, lower_limit=100.0, upper_limit=200.0
    )

    assert len(list(compu_method.phys_to_int_compu_scales())) == 2

    # check if the compu method can be constructed from a name and content and is equal to the original compu method
    element = compu_method.element
    compu_method2 = CompuMethod(element)
    assert compu_method == compu_method2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in CompuMethod.__dict__
    assert len(str(compu_method)) > 0


def test_compu_method_content_identical() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    # CompuMethodContent.Identical
    content = CompuMethodContent.Identical()
    assert isinstance(content, CompuMethodContent.Identical)
    compu_method = package.create_compu_method("CompuMethod", content)
    assert compu_method.content() == content
    assert compu_method.category == CompuMethodCategory.Identical
    assert list(compu_method.int_to_phys_compu_scales()) == []
    assert list(compu_method.phys_to_int_compu_scales()) == []

    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in CompuMethodContent.Identical.__dict__
    assert len(str(content)) > 0


def test_compu_method_content_linear() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    # CompuMethodContent.Linear
    content = CompuMethodContent.Linear(
        direction=CompuScaleDirection.IntToPhys,
        offset=0.0,
        factor=1.0,
        divisor=1.0,
        lower_limit=0.0,
        upper_limit=100.0,
    )
    assert isinstance(content, CompuMethodContent.Linear)
    content.direction = CompuScaleDirection.PhysToInt
    assert content.direction == CompuScaleDirection.PhysToInt
    content.factor = 2.0
    assert content.factor == 2.0
    content.divisor = 2.0
    assert content.divisor == 2.0
    content.lower_limit = 1.0
    assert content.lower_limit == 1.0
    content.upper_limit = 200.0
    assert content.upper_limit == 200.0
    compu_method = package.create_compu_method("CompuMethod", content)
    assert compu_method.content() == content
    assert compu_method.category == CompuMethodCategory.Linear

    content_other = CompuMethodContent.Identical()

    assert content != content_other
    assert content == content

    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in CompuMethodContent.Linear.__dict__
    assert len(str(content)) > 0


def test_compu_method_content_scale_linear() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    # CompuMethodContent.ScaleLinear
    content = CompuMethodContent.ScaleLinear(
        scales=[
            LinearConversionParameters(
                direction=CompuScaleDirection.IntToPhys,
                offset=0.0,
                factor=1.0,
                divisor=1.0,
                lower_limit=0.0,
                upper_limit=100.0,
            )
        ]
    )
    assert isinstance(content, CompuMethodContent.ScaleLinear)
    compu_method = package.create_compu_method("CompuMethod", content)
    assert compu_method.content() == content
    assert compu_method.category == CompuMethodCategory.ScaleLinear

    content_other = CompuMethodContent.Identical()

    assert content != content_other
    assert content == content

    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in CompuMethodContent.ScaleLinear.__dict__
    assert len(str(content)) > 0


def test_compu_method_content_rational() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    # CompuMethodContent.Rational
    content = CompuMethodContent.Rational(
        direction=CompuScaleDirection.IntToPhys,
        numerator=[1.0],
        denominator=[1.0],
        lower_limit=0.0,
        upper_limit=100.0,
    )
    assert isinstance(content, CompuMethodContent.Rational)
    compu_method = package.create_compu_method("CompuMethod", content)
    assert compu_method.content() == content
    assert compu_method.category == CompuMethodCategory.Rational

    content_other = CompuMethodContent.Identical()

    assert content != content_other
    assert content == content

    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in CompuMethodContent.Rational.__dict__
    assert len(str(content)) > 0


def test_compu_method_content_scale_rational() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    # CompuMethodContent.ScaleRational
    content = CompuMethodContent.ScaleRational(
        scales=[
            RationalConversionParameters(
                direction=CompuScaleDirection.IntToPhys,
                numerator=[1.0],
                denominator=[1.0],
                lower_limit=0.0,
                upper_limit=100.0,
            )
        ]
    )
    assert isinstance(content, CompuMethodContent.ScaleRational)
    compu_method = package.create_compu_method("CompuMethod", content)
    assert compu_method.content() == content
    assert compu_method.category == CompuMethodCategory.ScaleRational

    content_other = CompuMethodContent.Identical()

    assert content != content_other
    assert content == content

    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in CompuMethodContent.ScaleRational.__dict__
    assert len(str(content)) > 0


def test_compu_method_text_table() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    # CompuMethodContent.TextTable
    content = CompuMethodContent.TextTable(
        texts=[
            TextTableEntry(value=0.0, text="zero"),
            TextTableEntry(value=1.0, text="one"),
            TextTableEntry(value=2.0, text="two"),
        ]
    )
    assert isinstance(content, CompuMethodContent.TextTable)
    compu_method = package.create_compu_method("CompuMethod", content)
    assert compu_method.content() == content
    assert compu_method.category == CompuMethodCategory.TextTable

    content_other = CompuMethodContent.Identical()

    assert content != content_other
    assert content == content

    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in CompuMethodContent.TextTable.__dict__
    assert len(str(content)) > 0


def test_compu_method_bitfield_text_table() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    # CompuMethodContent.BitfieldTextTable
    content = CompuMethodContent.BitfieldTextTable(
        entries=[
            BitfieldEntry(value=0, mask=1, text="zero"),
            BitfieldEntry(value=1, mask=1, text="one"),
        ]
    )
    assert isinstance(content, CompuMethodContent.BitfieldTextTable)
    compu_method = package.create_compu_method("CompuMethod", content)
    assert compu_method.content() == content
    assert compu_method.category == CompuMethodCategory.BitfieldTextTable

    content_other = CompuMethodContent.Identical()

    assert content != content_other
    assert content == content

    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in CompuMethodContent.BitfieldTextTable.__dict__
    assert len(str(content)) > 0


def test_compu_method_scale_linear_and_text_table() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    # CompuMethodContent.ScaleLinearAndTextTable
    content = CompuMethodContent.ScaleLinearAndTextTable(
        scales=[
            LinearConversionParameters(
                direction=CompuScaleDirection.IntToPhys,
                offset=0.0,
                factor=1.0,
                divisor=1.0,
                lower_limit=1.0,
                upper_limit=100.0,
            )
        ],
        texts=[
            TextTableEntry(value=0.0, text="uninintialized"),
            TextTableEntry(value=255.0, text="error"),
        ],
    )
    compu_method = package.create_compu_method("CompuMethod", content)
    assert compu_method.content() == content
    assert compu_method.category == CompuMethodCategory.ScaleLinearAndTextTable

    content_other = CompuMethodContent.Identical()

    assert content != content_other
    assert content == content

    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in CompuMethodContent.ScaleLinearAndTextTable.__dict__
    assert len(str(content)) > 0


def test_compu_method_scale_rational_and_text_table() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    # CompuMethodContent.ScaleRationalAndTextTable
    content = CompuMethodContent.ScaleRationalAndTextTable(
        scales=[
            RationalConversionParameters(
                direction=CompuScaleDirection.IntToPhys,
                numerator=[1.0],
                denominator=[1.0],
                lower_limit=1.0,
                upper_limit=100.0,
            )
        ],
        texts=[
            TextTableEntry(value=0.0, text="uninintialized"),
            TextTableEntry(value=255.0, text="error"),
        ],
    )
    compu_method = package.create_compu_method("CompuMethod", content)
    assert compu_method.content() == content
    assert compu_method.category == CompuMethodCategory.ScaleRationalAndTextTable

    content_other = CompuMethodContent.Identical()

    assert content != content_other
    assert content == content

    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in CompuMethodContent.ScaleRationalAndTextTable.__dict__
    assert len(str(content)) > 0


def test_compu_method_tab_no_interpretation() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    # CompuMethodContent.TabNoInterpretation
    content = CompuMethodContent.TabNoInterpretation(
        entries=[
            TabNoIntpEntry(value_in=0.0, value_out=0.0),
            TabNoIntpEntry(value_in=1.0, value_out=33.3),
            TabNoIntpEntry(value_in=2.0, value_out=12345),
        ]
    )
    assert isinstance(content, CompuMethodContent.TabNoInterpretation)
    compu_method = package.create_compu_method("CompuMethod", content)
    assert compu_method.content() == content
    assert compu_method.category == CompuMethodCategory.TabNoInterpretation

    content_other = CompuMethodContent.Identical()

    assert content != content_other
    assert content == content

    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in CompuMethodContent.TabNoInterpretation.__dict__
    assert len(str(content)) > 0


def test_compu_scale() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    content = CompuMethodContent.ScaleRational(
        scales=[
            RationalConversionParameters(
                direction=CompuScaleDirection.IntToPhys,
                numerator=[1.0],
                denominator=[1.0],
                lower_limit=0.0,
                upper_limit=100.0,
            )
        ]
    )
    compu_method = package.create_compu_method("CompuMethod", content)

    # CompuScale
    compu_scale = compu_method.create_compu_scale(
        CompuScaleDirection.IntToPhys, lower_limit=0.0, upper_limit=100.0
    )
    assert compu_scale.lower_limit == 0.0
    assert compu_scale.upper_limit == 100.0
    compu_scale.mask = 0xFF
    assert compu_scale.mask == 0xFF

    compu_scale.content = 33
    assert compu_scale.content == 33
    compu_scale.content = "text"
    assert compu_scale.content == "text"
    rational_coefficients = CompuScaleRationalCoefficients(
        numerator=[1.0], denominator=[1.0]
    )
    compu_scale.content = rational_coefficients
    assert compu_scale.content == rational_coefficients

    # check if the compu scale can be constructed from a direction and limits and is equal to the original compu scale
    element = compu_scale.element
    compu_scale2 = CompuScale(element)
    assert compu_scale == compu_scale2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in CompuScale.__dict__
    assert len(str(compu_scale)) > 0


def test_data_type_mapping_set() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    sw_base_type = package.create_sw_base_type(
        "SwBaseType", 32, BaseTypeEncoding.TwosComplement
    )
    implementation_data_type = package.create_implementation_data_type(
        ImplementationDataTypeSettings.Value(
            "ImplementationDataType", base_type=sw_base_type
        )
    )
    application_data_type = package.create_application_primitive_data_type(
        "ApplicationPrimitiveDataType", ApplicationPrimitiveCategory.Value
    )

    # DataTypeMappingSet
    data_type_mapping_set = package.create_data_type_mapping_set("DataTypeMappingSet")
    assert isinstance(data_type_mapping_set, DataTypeMappingSet)

    data_type_mapping_set.name = "DataTypeMappingSet2"
    assert data_type_mapping_set.name == "DataTypeMappingSet2"

    data_type_mapping = data_type_mapping_set.create_data_type_map(
        implementation_data_type, application_data_type
    )
    assert list(data_type_mapping_set.data_type_maps()) == [data_type_mapping]
    assert isinstance(data_type_mapping, DataTypeMap)

    assert data_type_mapping.implementation_data_type == implementation_data_type
    assert data_type_mapping.application_data_type == application_data_type

    # check if the data type mapping set can be constructed from a name and is equal to the original data type mapping set
    element = data_type_mapping_set.element
    data_type_mapping_set_copy = DataTypeMappingSet(element)
    assert data_type_mapping_set == data_type_mapping_set_copy
    element = data_type_mapping.element
    data_type_mapping_copy = DataTypeMap(element)
    assert data_type_mapping == data_type_mapping_copy
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in DataTypeMappingSet.__dict__
    assert len(str(data_type_mapping_set)) > 0
    assert "__repr__" in DataTypeMap.__dict__
    assert len(str(data_type_mapping)) > 0


def test_unit() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    # Unit
    unit = package.create_unit("Unit", display_name="A")
    assert isinstance(unit, Unit)

    unit.name = "Unit2"
    assert unit.name == "Unit2"

    unit.display_name = "nm"
    assert unit.display_name == "nm"

    # check if the unit can be constructed from a name and is equal to the original unit
    element = unit.element
    unit2 = Unit(element)
    assert unit == unit2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in Unit.__dict__
    assert len(str(unit)) > 0


def test_data_constr() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    # DataConstraint
    data_constr = package.create_data_constr("DataConstraint")
    assert isinstance(data_constr, DataConstr)

    data_constr.name = "DataConstraint2"
    assert data_constr.name == "DataConstraint2"

    data_constr_rule = data_constr.create_data_constr_rule(
        DataConstrType.Internal, lower_limit=0, upper_limit=100
    )
    assert list(data_constr.data_constr_rules()) == [data_constr_rule]
    assert isinstance(data_constr_rule, DataConstrRule)
    assert data_constr_rule.rule_type == DataConstrType.Internal
    assert data_constr_rule.lower_limit == 0
    assert data_constr_rule.upper_limit == 100

    # check if the data constraint can be constructed from a name and is equal to the original data constraint
    element = data_constr.element
    data_constraint2 = DataConstr(element)
    assert data_constr == data_constraint2
    element = data_constr_rule.element
    data_constr_rule2 = DataConstrRule(element)
    assert data_constr_rule == data_constr_rule2
    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in DataConstr.__dict__
    assert len(str(data_constr)) > 0
    assert "__repr__" in DataConstrRule.__dict__
    assert len(str(data_constr_rule)) > 0


def test_autosar_data_type_conversion() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    sender_receiver_interface = package.create_sender_receiver_interface(
        "SenderReceiverInterface"
    )
    base_type = package.create_sw_base_type(
        "BaseType", 32, BaseTypeEncoding.TwosComplement
    )
    implementation_data_type = package.create_implementation_data_type(
        ImplementationDataTypeSettings.Value(
            "ImplementationDataType", base_type=base_type
        )
    )
    application_primitive_data_type = package.create_application_primitive_data_type(
        "ApplicationPrimitiveDataType", ApplicationPrimitiveCategory.Value
    )
    application_array_data_type = package.create_application_array_data_type(
        "ApplicationArrayDataType",
        application_primitive_data_type,
        ApplicationArraySize.Fixed(10),
    )
    application_record_data_type = package.create_application_record_data_type(
        "ApplicationRecordDataType"
    )

    sender_receiver_interface.create_data_element("item", implementation_data_type)
    sender_receiver_interface.create_data_element(
        "item2", application_primitive_data_type
    )
    sender_receiver_interface.create_data_element("item3", application_array_data_type)
    sender_receiver_interface.create_data_element("item4", application_record_data_type)

    sr_elements = list(sender_receiver_interface.data_elements())
    assert sr_elements[0].data_type == implementation_data_type
    assert sr_elements[1].data_type == application_primitive_data_type
    assert sr_elements[2].data_type == application_array_data_type
    assert sr_elements[3].data_type == application_record_data_type

    sr_elements[0].data_type = application_primitive_data_type
    assert sr_elements[0].data_type == application_primitive_data_type


def test_constant_specification() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")

    # ConstantSpecification
    constant_specification = package.create_constant_specification(
        "ConstantSpecification", [0, 0, 0]
    )
    assert isinstance(constant_specification, ConstantSpecification)

    constant_specification.name = "ConstantSpecification2"
    assert constant_specification.name == "ConstantSpecification2"

    constant_specification.value_specification = NumericalValueSpecification(0)
    assert constant_specification.value_specification == NumericalValueSpecification(0)

    # check if the constant  specification can be constructed from a name and is equal to the original
    element = constant_specification.element
    constant_specification2 = ConstantSpecification(element)
    assert constant_specification == constant_specification2

    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ConstantSpecification.__dict__
    assert len(str(constant_specification)) > 0
