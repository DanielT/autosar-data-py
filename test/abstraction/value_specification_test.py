from autosar_data.abstraction import *
from autosar_data.abstraction.datatype import *
from autosar_data.abstraction.software_component import *


def test_numerical_value_specification() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    constant_specification = package.create_constant_specification("c", 0)

    spec = NumericalValueSpecification(0)
    constant_specification.value_specification = spec
    assert constant_specification.value_specification == spec
    assert spec.label is None
    assert spec.value == 0

    spec.value = 42
    spec.label = "test_label"
    constant_specification.value_specification = spec
    assert constant_specification.value_specification == spec

    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in NumericalValueSpecification.__dict__
    assert len(str(spec)) > 0


def test_text_value_specification() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    constant_specification = package.create_constant_specification("c", 0)

    spec = TextValueSpecification("text")
    constant_specification.value_specification = spec
    assert constant_specification.value_specification == spec
    assert spec.label is None
    assert spec.value == "text"

    spec.value = "another text"
    spec.label = "test_label"
    constant_specification.value_specification = spec
    assert constant_specification.value_specification == spec

    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in TextValueSpecification.__dict__
    assert len(str(spec)) > 0


def test_constant_value_reference() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    constant_specification = package.create_constant_specification("c", 0)

    spec = ConstantReference(constant_specification)
    constant_specification.value_specification = spec
    assert constant_specification.value_specification == spec
    assert spec.label is None
    assert spec.constant == constant_specification

    spec.label = "test_label"
    constant_specification.value_specification = spec
    assert constant_specification.value_specification == spec

    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ConstantReference.__dict__
    assert len(str(spec)) > 0


def test_array_value_specification() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    constant_specification = package.create_constant_specification("c", 0)

    spec = ArrayValueSpecification([NumericalValueSpecification(1)])
    constant_specification.value_specification = spec
    assert constant_specification.value_specification == spec
    assert spec.label is None
    assert spec.values[0] == NumericalValueSpecification(1)

    spec.values.append(NumericalValueSpecification(2))
    spec.label = "test_label"
    constant_specification.value_specification = spec
    assert constant_specification.value_specification == spec

    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ArrayValueSpecification.__dict__
    assert len(str(spec)) > 0


def test_record_value_specification() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    constant_specification = package.create_constant_specification("c", 0)

    spec = RecordValueSpecification([NumericalValueSpecification(1)])
    constant_specification.value_specification = spec
    assert constant_specification.value_specification == spec
    assert spec.label is None
    assert spec.values[0] == NumericalValueSpecification(1)

    spec.values.append(NumericalValueSpecification(2))
    spec.label = "test_label"
    constant_specification.value_specification = spec
    assert constant_specification.value_specification == spec

    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in RecordValueSpecification.__dict__
    assert len(str(spec)) > 0


def test_not_available_value_specification() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    constant_specification = package.create_constant_specification("c", 0)

    spec = NotAvailableValueSpecification()
    constant_specification.value_specification = spec
    assert constant_specification.value_specification == spec
    assert spec.label is None
    assert spec.default_pattern is None

    spec2 = NotAvailableValueSpecification(default_pattern=0xFF)
    constant_specification.value_specification = spec2
    assert constant_specification.value_specification == spec2
    assert spec2.default_pattern == 0xFF

    spec3 = NotAvailableValueSpecification(label="test_label")
    constant_specification.value_specification = spec3
    assert constant_specification.value_specification == spec3
    assert spec3.label == "test_label"

    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in NotAvailableValueSpecification.__dict__
    assert len(str(spec)) > 0


def test_application_value_specification() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    constant_specification = package.create_constant_specification("c", 0)

    spec = ApplicationValueSpecification(
        ApplicationPrimitiveCategory.Cube4,
        [
            SwAxisCont(
                SwAxisContCategory.StdAxis,
                [3],
                0,
                [
                    SwValue.V(1),
                    SwValue.Vf(2),
                    SwValue.Vt("vt"),
                    SwValue.Vg([], label="VgLabel"),
                    SwValue.VtfNumber(2),
                    SwValue.VtfText("VtfText"),
                ],
            )
        ],
        SwValueCont([3], [SwValue.Vf(1)]),
    )
    constant_specification.value_specification = spec
    assert constant_specification.value_specification == spec
    assert spec.label is None
    assert spec.category == ApplicationPrimitiveCategory.Cube4

    spec.category = ApplicationPrimitiveCategory.ResAxis
    spec.label = "Label"
    constant_specification.value_specification = spec
    assert constant_specification.value_specification == spec

    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ApplicationValueSpecification.__dict__
    assert len(str(spec)) > 0


def test_reference_value_specification() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    constant_specification = package.create_constant_specification("c", 0)

    base_type = package.create_sw_base_type("base", 32, BaseTypeEncoding.NoEncoding)
    impl_settings = ImplementationDataTypeSettings.Value(
        name="ImplementationValue",
        base_type=base_type,
        compu_method=None,
        data_constraint=None,
    )
    datatype = package.create_implementation_data_type(impl_settings)
    app_data_type = package.create_application_primitive_data_type(
        "AppDataType",
        ApplicationPrimitiveCategory.Value,
    )

    # ArgumentDataPrototype of a ClientServerInterface
    client_server_interface = package.create_client_server_interface("CS_Interface")
    cs_operation = client_server_interface.create_operation("Operation")
    argument_data_prototype = cs_operation.create_argument(
        "adp", datatype, ArgumentDirection.In
    )

    spec = ReferenceValueSpecification(argument_data_prototype)
    constant_specification.value_specification = spec
    assert constant_specification.value_specification == spec

    # ParameterDataPrototype of a ParameterInterface
    parameter_interface = package.create_parameter_interface("P_Interface")
    parameter_data_prototype = parameter_interface.create_parameter("pdp", datatype)

    spec = ReferenceValueSpecification(parameter_data_prototype)
    constant_specification.value_specification = spec
    assert constant_specification.value_specification == spec

    # VariableDataPrototype of a SenderReceiverInterface
    sender_receiver_interface = package.create_sender_receiver_interface("SR_Interface")
    variable_data_prototype = sender_receiver_interface.create_data_element(
        "vdp", datatype
    )

    spec = ReferenceValueSpecification(variable_data_prototype)
    constant_specification.value_specification = spec
    assert constant_specification.value_specification == spec

    # ApplicationArrayElement of an ApplicationArrayDataType
    application_array_data_type = package.create_application_array_data_type(
        "ArrayDataType",
        app_data_type,
        ApplicationArraySize.Fixed(1),
    )
    application_array_element = application_array_data_type.array_element

    spec = ReferenceValueSpecification(application_array_element)
    constant_specification.value_specification = spec
    assert constant_specification.value_specification == spec

    # ApplicationRecordElement of an ApplicationRecordDataType
    application_record_data_type = package.create_application_record_data_type(
        "RecordDataType"
    )
    application_record_element = application_record_data_type.create_record_element(
        "Element", app_data_type
    )

    spec = ReferenceValueSpecification(application_record_element)
    constant_specification.value_specification = spec
    assert constant_specification.value_specification == spec
    assert spec.label is None
    assert spec.reference_value == application_record_element

    # set a label for the reference value specification
    spec.label = "test_label"
    constant_specification.value_specification = spec
    assert constant_specification.value_specification == spec
    assert spec.label == "test_label"

    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ReferenceValueSpecification.__dict__
    assert len(str(spec)) > 0


def test_application_rule_based_value_specification() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    constant_specification = package.create_constant_specification("c", 0)

    spec = ApplicationRuleBasedValueSpecification(
        ApplicationPrimitiveCategory.ValBlk,
        [
            RuleBasedAxisCont(
                SwAxisContCategory.StdAxis,
                [3],
                0,
                RuleBasedValueSpecification(
                    [
                        RuleArgument.V(0),
                        RuleArgument.Vf(1),
                        RuleArgument.Vt("vt"),
                        RuleArgument.VtfNumber(2),
                        RuleArgument.VtfText("VtfText"),
                    ],
                    RuleBasedFillUntil.MaxSize,
                    max_size_to_fill=42,
                ),
            )
        ],
        RuleBasedValueCont(
            RuleBasedValueSpecification(
                [
                    RuleArgument.V(0),
                    RuleArgument.Vf(1),
                    RuleArgument.Vt("vt"),
                    RuleArgument.VtfNumber(2),
                    RuleArgument.VtfText("VtfText"),
                ],
                RuleBasedFillUntil.MaxSize,
                max_size_to_fill=42,
            ),
            [33],
        ),
    )
    constant_specification.value_specification = spec
    assert constant_specification.value_specification == spec
    assert spec.label is None
    assert spec.category == ApplicationPrimitiveCategory.ValBlk
    assert len(spec.sw_axis_cont) == 1
    assert isinstance(spec.sw_axis_cont[0], RuleBasedAxisCont)
    assert isinstance(spec.sw_value_cont, RuleBasedValueCont)
    assert spec.sw_value_cont.rule_based_values.rule == RuleBasedFillUntil.MaxSize

    spec.category = ApplicationPrimitiveCategory.ResAxis
    spec.label = "Label"
    constant_specification.value_specification = spec
    assert constant_specification.value_specification == spec

    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in ApplicationRuleBasedValueSpecification.__dict__
    assert len(str(spec)) > 0


def test_composite_rule_based_value_specification() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    constant_specification = package.create_constant_specification("c", 0)

    spec = CompositeRuleBasedValueSpecification(
        [
            ArrayValueSpecification([NumericalValueSpecification(1)]),
            RecordValueSpecification([NumericalValueSpecification(2)]),
        ],
        [
            ApplicationValueSpecification(
                ApplicationPrimitiveCategory.ValBlk, [], SwValueCont([], [])
            ),
            ApplicationRuleBasedValueSpecification(
                ApplicationPrimitiveCategory.ValBlk,
                [],
                RuleBasedValueCont(
                    RuleBasedValueSpecification([], RuleBasedFillUntil.MaxSize), []
                ),
            ),
        ],
        RuleBasedFillUntil.End,
    )
    constant_specification.value_specification = spec
    assert constant_specification.value_specification == spec

    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in CompositeRuleBasedValueSpecification.__dict__
    assert len(str(spec)) > 0


def test_numerical_rule_based_value_specification() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    constant_specification = package.create_constant_specification("c", 0)

    spec = NumericalRuleBasedValueSpecification(
        RuleBasedValueSpecification(
            [RuleArgument.V(1)], RuleBasedFillUntil.MaxSize, max_size_to_fill=42
        ),
        label="NumericalRuleBasedValue",
    )
    constant_specification.value_specification = spec
    assert constant_specification.value_specification == spec

    # quick check if a custom __repr__ method is implemented and returns a non-empty string
    assert "__repr__" in NumericalRuleBasedValueSpecification.__dict__
    assert len(str(spec)) > 0


def test_convenience_conversions() -> None:
    model = AutosarModelAbstraction.create("test.arxml")
    package = model.get_or_create_package("/package")
    constant_specification = package.create_constant_specification("c", 0)

    # a number is converted to a NumericalValueSpecification
    in_spec = 1
    out_spec = NumericalValueSpecification(1)
    constant_specification.value_specification = in_spec
    assert constant_specification.value_specification == out_spec

    # a string is converted to a TextValueSpecification
    in_spec = "test"
    out_spec = TextValueSpecification("test")
    constant_specification.value_specification = in_spec
    assert constant_specification.value_specification == out_spec

    # a list gets converted to an ArrayValueSpecification
    in_spec = [1.1, 2.2, 3.3]
    out_spec = ArrayValueSpecification(
        [
            NumericalValueSpecification(1.1),
            NumericalValueSpecification(2.2),
            NumericalValueSpecification(3.3),
        ]
    )
    constant_specification.value_specification = in_spec
    assert constant_specification.value_specification == out_spec

    # a tuple gets converted to a RecordValueSpecification
    in_spec = (1.1, 2.2, 3.3)
    out_spec = RecordValueSpecification(
        [
            NumericalValueSpecification(1.1),
            NumericalValueSpecification(2.2),
            NumericalValueSpecification(3.3),
        ]
    )
    constant_specification.value_specification = in_spec
    assert constant_specification.value_specification == out_spec
