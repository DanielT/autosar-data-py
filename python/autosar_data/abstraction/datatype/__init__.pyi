# Stub file for autosar_data.abastraction.datatype

from typing import final, Iterator, List, Optional, Union, Tuple, Type, TypeAlias
from autosar_data import Element
from autosar_data.abstraction import ByteOrder
from autosar_data.abstraction.software_component import (
    ArgumentDataPrototype,
    ParameterDataPrototype,
    VariableDataPrototype,
)

ApplicationDataType: TypeAlias = Union[
    ApplicationPrimitiveDataType, ApplicationArrayDataType, ApplicationRecordDataType
]
AutosarDataType: TypeAlias = Union[ApplicationDataType, ImplementationDataType]
DataPointerTarget: TypeAlias = Union[ImplementationDataType, SwBaseType]
ValueSpecification: TypeAlias = Union[
    ArrayValueSpecification,
    RecordValueSpecification,
    TextValueSpecification,
    NumericalValueSpecification,
    ConstantReference,
    ApplicationValueSpecification,
    NotAvailableValueSpecification,
    ReferenceValueSpecification,
    ApplicationRuleBasedValueSpecification,
    CompositeRuleBasedValueSpecification,
    NumericalRuleBasedValueSpecification,
    List[ValueSpecification],  # = ArrayValueSpecification
    Tuple[ValueSpecification, ...],  # = RecordValueSpecification
    str,  # = TextValueSpecification
    float,  # = NumericalValueSpecification
]
CompositeRuleBasedValueArgument: TypeAlias = Union[
    ApplicationValueSpecification, ApplicationRuleBasedValueSpecification
]
CompositeValueSpecification: TypeAlias = Union[
    ArrayValueSpecification,
    RecordValueSpecification,
]
DataPrototype: TypeAlias = Union[
    ArgumentDataPrototype,
    ParameterDataPrototype,
    VariableDataPrototype,
    ApplicationArrayElement,
    ApplicationRecordElement,
]

@final
class ApplicationArrayDataType:
    """
    An application array data type

    Use ArPackage.create_application_array_data_type to create a new application array data type.
    """

    def __init__(self, element: Element, /) -> ApplicationArrayDataType: ...
    array_element: ApplicationArrayElement
    """array element of the array data type"""
    element: Element
    name: str
    def set_size(self, size: ApplicationArraySize, /) -> None:
        """set the size specification of the array"""
        ...

    def size(self, /) -> ApplicationArraySize:
        """get the size specification of the array: Variable, Fixed, etc."""
        ...

@final
class ApplicationArrayElement:
    """
    An element in an application array data type
    """

    def __init__(self, element: Element, /) -> ApplicationArrayElement: ...
    data_type: ApplicationDataType
    """data type of the array element"""
    element: Element
    name: str

@final
class ApplicationArraySize:
    """
    definition of the size type of an application array data type
    """

    Fixed: Type[ApplicationArraySize_Fixed]
    VariableFullyFlexible: Type[ApplicationArraySize_VariableFullyFlexible]
    VariableLinear: Type[ApplicationArraySize_VariableLinear]
    VariableRectangular: Type[ApplicationArraySize_VariableRectangular]
    VariableSquare: Type[ApplicationArraySize_VariableSquare]

@final
class ApplicationArraySize_Fixed(ApplicationArraySize):
    def __init__(self, length: int, /) -> ApplicationArraySize_Fixed: ...
    length: int

@final
class ApplicationArraySize_VariableFullyFlexible(ApplicationArraySize):
    def __init__(
        self, max_size: int, /
    ) -> ApplicationArraySize_VariableFullyFlexible: ...
    max_size: int

@final
class ApplicationArraySize_VariableLinear(ApplicationArraySize):
    def __init__(self, max_size: int, /) -> ApplicationArraySize_VariableLinear: ...
    max_size: int

@final
class ApplicationArraySize_VariableRectangular(ApplicationArraySize):
    def __init__(
        self, max_size: int, /
    ) -> ApplicationArraySize_VariableRectangular: ...
    max_size: int

@final
class ApplicationArraySize_VariableSquare(ApplicationArraySize):
    def __init__(self) -> ApplicationArraySize_VariableSquare: ...
    ...

@final
class ApplicationPrimitiveCategory:
    """
    The category of an application primitive data type
    """

    Boolean: ApplicationPrimitiveCategory
    ComAxis: ApplicationPrimitiveCategory
    Cube4: ApplicationPrimitiveCategory
    Cube5: ApplicationPrimitiveCategory
    Cuboid: ApplicationPrimitiveCategory
    Curve: ApplicationPrimitiveCategory
    Map: ApplicationPrimitiveCategory
    ResAxis: ApplicationPrimitiveCategory
    String: ApplicationPrimitiveCategory
    ValBlk: ApplicationPrimitiveCategory
    Value: ApplicationPrimitiveCategory

@final
class ApplicationPrimitiveDataType:
    """
    An application primitive data type

    Use [`ArPackage::create_application_primitive_data_type`] to create a new application primitive data type.
    """

    def __init__(self, element: Element, /) -> ApplicationPrimitiveDataType: ...
    category: ApplicationPrimitiveCategory
    """category of the primitive data type"""
    compu_method: CompuMethod
    """set the compu method of the primitive data type"""
    data_constraint: DataConstr
    """data constraint of the primitive data type"""
    element: Element
    name: str
    unit: Unit
    """unit of the primitive data type"""

@final
class ApplicationRecordDataType:
    """
    An application record data type

    Use [`ArPackage::create_application_record_data_type`] to create a new application record data type.
    """

    def __init__(self, element: Element, /) -> ApplicationRecordDataType: ...
    def create_record_element(
        self, name: str, data_type: ApplicationDataType, /
    ) -> ApplicationRecordElement:
        """create a new element in the record data type"""
        ...
    element: Element
    name: str
    def record_elements(self, /) -> Iterator[ApplicationRecordElement]:
        """get an iterator over the record elements of the record data type"""
        ...

@final
class ApplicationRecordElement:
    """
    An element in an application record data type
    """

    def __init__(self, element: Element, /) -> ApplicationRecordElement: ...
    data_type: ApplicationDataType
    """data type of the record element"""
    element: Element
    name: str

@final
class ApplicationRuleBasedValueSpecification:
    """
    An application rule based value specification
    """

    def __init__(
        self,
        category: ApplicationPrimitiveCategory,
        sw_axis_cont: List[RuleBasedAxisCont],
        sw_value_cont: RuleBasedValueCont,
        /,
        *,
        label: Optional[str] = None,
    ) -> ApplicationRuleBasedValueSpecification: ...
    category: ApplicationPrimitiveCategory
    """category of the application rule based value specification"""
    sw_axis_cont: List[RuleBasedAxisCont]
    """list of `RuleBasedAxisCont` objects in the application rule based value specification"""
    sw_value_cont: RuleBasedValueCont
    """`RuleBasedValueCont` of the application rule based value specification"""
    label: Optional[str]
    """label of the application rule based value specification, if any"""

@final
class ApplicationValueSpecification:
    """
    An application value specification
    """

    def __init__(
        self,
        category: ApplicationPrimitiveCategory,
        sw_axis_conts: List[SwAxisCont],
        sw_value_cont: SwValueCont,
        *,
        label: Optional[str] = None,
    ) -> ApplicationValueSpecification: ...

    category: ApplicationPrimitiveCategory
    """category of the application value specification"""
    sw_axis_conts: List[SwAxisCont]
    """list of `SwAxisCont` objects in the application value specification"""
    sw_value_cont: SwValueCont
    """`SwValueCont` of the application value specification"""
    label: Optional[str]
    """label of the application value specification, if any"""

@final
class ArrayValueSpecification:
    """
    An array value specification
    """

    def __init__(
        self, values: List[ValueSpecification], /, *, label: Optional[str] = None
    ) -> ArrayValueSpecification: ...
    values: List[ValueSpecification]
    """list of values in the array"""
    label: Optional[str]
    """label of the array value specification, if any"""

@final
class BaseTypeEncoding:
    """
    `BaseTypeEncoding` describes the encoding of a basic data type.
    """

    BcdPacked: BaseTypeEncoding
    BcdUnpacked: BaseTypeEncoding
    Boolean: BaseTypeEncoding
    DspFractional: BaseTypeEncoding
    Ieee754: BaseTypeEncoding
    Iso8859_1: BaseTypeEncoding
    Iso8859_2: BaseTypeEncoding
    NoEncoding: BaseTypeEncoding
    OnesComplement: BaseTypeEncoding
    SignMagnitude: BaseTypeEncoding
    TwosComplement: BaseTypeEncoding
    Ucs2: BaseTypeEncoding
    Utf16: BaseTypeEncoding
    Utf8: BaseTypeEncoding
    Void: BaseTypeEncoding
    Windows1252: BaseTypeEncoding

@final
class BitfieldEntry:
    """
    A single entry of a bitfield text table conversion
    """

    def __init__(self, *, text: str, value: float, mask: int) -> BitfieldEntry: ...
    mask: int
    """bit mask of this entry"""
    text: str
    """text of this entry"""
    value: float
    """numeric value of this entry"""

@final
class CompositeRuleBasedValueSpecification:
    """
    A composite rule based value specification
    """

    def __init__(
        self,
        argument: List[CompositeValueSpecification],
        compound_primitive_argument: List[CompositeRuleBasedValueArgument],
        max_size_to_fill: Optional[int],
        rule: RuleBasedFillUntil,
        /,
        *,
        label: Optional[str] = None,
    ) -> CompositeRuleBasedValueSpecification: ...
    argument: List[CompositeValueSpecification]
    """list of `CompositeValueSpecification` objects in the composite rule based value specification"""
    compound_primitive_argument: List[CompositeRuleBasedValueArgument]
    """list of `CompositeRuleBasedValueArgument` objects in the composite rule based value specification"""
    max_size_to_fill: Optional[int]
    """maximum size to fill in the composite rule based value specification"""
    rule: RuleBasedFillUntil
    """rule for filling the composite rule based value specification"""
    label: Optional[str]
    """label of the composite rule based value specification, if any"""

@final
class CompuMethod:
    """
    A `CompuMethod` describes the conversion between physical and internal values

    Use [`ArPackage::create_compu_method`] to create a new `CompuMethod`
    """

    def __init__(self, element: Element, /) -> CompuMethod: ...
    category: CompuMethodCategory
    """category of the `CompuMethod`"""
    def content(self, /) -> Optional[CompuMethodContent]:
        """get the CompuMethodContent of the CompuMethod"""
        ...

    def create_compu_scale(
        self,
        direction: CompuScaleDirection,
        /,
        *,
        lower_limit: Optional[float] = None,
        upper_limit: Optional[float] = None,
    ) -> CompuScale:
        """create a `CompuScale` in the `CompuMethod`"""
        ...
    element: Element
    def int_to_phys_compu_scales(self, /) -> Iterator[CompuScale]:
        """Create an iterator over the internal-to-physical `CompuScales`"""
        ...
    name: str
    def phys_to_int_compu_scales(self, /) -> Iterator[CompuScale]:
        """Create an iterator over the physical-to-internal `CompuScales`"""
        ...

    def set_content(self, content: CompuMethodContent, /) -> None:
        """set the content of the `CompuMethod`
        Writing to this attribute removes any existing content"""
        ...

@final
class CompuMethodBitfieldTextTableContent:
    """
    A single entry of a bitfield text table conversion
    """

    mask: int
    """bit mask of this entry"""
    text: str
    """text of this entry"""
    value: float
    """numeric value of this entry"""

@final
class CompuMethodCategory:
    """
    Category of a `CompuMethod`
    """

    BitfieldTextTable: CompuMethodCategory
    Identical: CompuMethodCategory
    Linear: CompuMethodCategory
    Rational: CompuMethodCategory
    ScaleLinear: CompuMethodCategory
    ScaleLinearAndTextTable: CompuMethodCategory
    ScaleRational: CompuMethodCategory
    ScaleRationalAndTextTable: CompuMethodCategory
    TabNoInterpretation: CompuMethodCategory
    TextTable: CompuMethodCategory

@final
class CompuMethodIdenticalContent:
    """
    Description of the content of a `CompuMethod` whose category is `Identical`.
    This class is empty, as there are no additional attributes for the identical conversion.
    """

@final
class CompuMethodContent:
    """
    Content of a `CompuMethod`
    """

    BitfieldTextTable: Type[CompuMethodContent_BitfieldTextTable]
    Identical: Type[CompuMethodContent_Identical]
    Linear: Type[CompuMethodContent_Linear]
    Rational: Type[CompuMethodContent_Rational]
    ScaleLinear: Type[CompuMethodContent_ScaleLinear]
    ScaleRational: Type[CompuMethodContent_ScaleRational]
    ScaleLinearAndTextTable: Type[CompuMethodContent_ScaleLinearAndTextTable]
    ScaleRationalAndTextTable: Type[CompuMethodContent_ScaleRationalAndTextTable]
    TabNoInterpretation: Type[CompuMethodContent_TabNoInterpretation]
    TextTable: Type[CompuMethodContent_TextTable]

@final
class CompuMethodContent_BitfieldTextTable(CompuMethodContent):
    def __init__(
        self, entries: List[BitfieldEntry]
    ) -> CompuMethodContent_BitfieldTextTable: ...
    entries: List[BitfieldEntry]

@final
class CompuMethodContent_Identical(CompuMethodContent):
    def __init__(self) -> CompuMethodContent_Identical: ...

@final
class CompuMethodContent_Linear(CompuMethodContent):
    def __init__(
        self,
        *,
        direction: CompuScaleDirection,
        divisor: float,
        factor: float,
        offset: float,
        lower_limit: Optional[float] = None,
        upper_limit: Optional[float] = None,
    ) -> CompuMethodContent_Linear: ...
    direction: CompuScaleDirection
    divisor: float
    factor: float
    lower_limit: Optional[float]
    offset: float
    upper_limit: Optional[float]

@final
class CompuMethodContent_Rational(CompuMethodContent):
    def __init__(
        self,
        *,
        direction: CompuScaleDirection,
        denominator: List[float],
        numerator: List[float],
        lower_limit: float,
        upper_limit: float,
    ) -> CompuMethodContent_Rational: ...
    denominator: List[float]
    direction: CompuScaleDirection
    lower_limit: float
    numerator: List[float]
    upper_limit: float

@final
class CompuMethodContent_ScaleLinear(CompuMethodContent):
    def __init__(
        self, *, scales: List[LinearConversionParameters]
    ) -> CompuMethodContent_ScaleLinear: ...
    scales: List[LinearConversionParameters]

@final
class CompuMethodContent_ScaleRational(CompuMethodContent):
    def __init__(
        self, *, scales: List[RationalConversionParameters]
    ) -> CompuMethodContent_ScaleRational: ...
    scales: List[RationalConversionParameters]

@final
class CompuMethodContent_ScaleLinearAndTextTable(CompuMethodContent):
    def __init__(
        self, *, scales: List[LinearConversionParameters], texts: List[TextTableEntry]
    ) -> CompuMethodContent_ScaleLinearAndTextTable: ...
    scales: List[LinearConversionParameters]
    texts: List[TextTableEntry]

@final
class CompuMethodContent_ScaleRationalAndTextTable(CompuMethodContent):
    def __init__(
        self, *, scales: List[RationalConversionParameters], texts: List[TextTableEntry]
    ) -> CompuMethodContent_ScaleRationalAndTextTable: ...
    scales: List[RationalConversionParameters]
    texts: List[TextTableEntry]

@final
class CompuMethodContent_TabNoInterpretation(CompuMethodContent):
    def __init__(
        self, *, entries: List[TabNoIntpEntry]
    ) -> CompuMethodContent_TabNoInterpretation: ...
    entries: List[TabNoIntpEntry]

@final
class CompuMethodContent_TextTable(CompuMethodContent):
    def __init__(
        self, *, texts: List[TextTableEntry]
    ) -> CompuMethodContent_TextTable: ...
    texts: List[TextTableEntry]

@final
class CompuScale:
    """
    A `CompuScale` describes the conversion between physical and internal values, as well as the limits of the scale
    """

    def __init__(self, element: Element, /) -> CompuScale: ...
    content: Union[CompuScaleRationalCoefficients, str, int]
    """content of the `CompuScale`"""
    element: Element
    lower_limit: Optional[float]
    """lower limit of the `CompuScale`"""
    mask: Optional[int]
    """mask of the `CompuScale`, applicable for `BitfieldTextTable`"""
    upper_limit: Optional[float]
    """upper limit of the `CompuScale`"""

@final
class CompuScaleDirection:
    """
    Direction of a `CompuScale`
    """

    IntToPhys: CompuScaleDirection
    PhysToInt: CompuScaleDirection

@final
class CompuScaleRationalCoefficients:
    """
    Rational coefficients of a CompuScale
    """

    def __init__(
        self, *, numerator: List[float], denominator: List[float]
    ) -> CompuScaleRationalCoefficients: ...
    denominator: List[float]
    """list of denominator coefficients"""
    numerator: List[float]
    """list of numerator coefficients"""

@final
class ConstantReference:
    """
    A `ConstantReference` is a reference to a ConstantSpecification which can be used in a ValueSpecification.
    """

    def __init__(
        self,
        constant: ConstantSpecification,
        /,
        *,
        label: Optional[str] = None,
    ) -> ConstantReference: ...
    constant: ConstantSpecification
    """the constant specification that is referenced"""
    label: Optional[str]
    """label of the `ConstantReference`, if any"""

@final
class ConstantSpecification:
    """
    A `ConstantSpecification` is a specification of a constant value.

    Use [`ArPackage::create_constant_specification`] to create a new `ConstantSpecification`.
    """

    def __init__(self, element: Element, /) -> ConstantSpecification: ...
    element: Element
    name: str
    value_specification: ValueSpecification

@final
class DataConstr:
    """
    `DataConstr` represents a data constraint.
    """

    def __init__(self, element: Element, /) -> DataConstr: ...
    def create_data_constr_rule(
        self,
        rule_type: DataConstrType,
        /,
        *,
        lower_limit: Optional[float] = None,
        upper_limit: Optional[float] = None,
    ) -> DataConstrRule:
        """Create a data constraint rule"""
        ...

    def data_constr_rules(self, /) -> Iterator[DataConstrRule]:
        """Get all data constraint rules"""
        ...
    element: Element
    name: str

@final
class DataConstrRule:
    """
    `DataConstrRule` represents a data constraint rule.
    """

    def __init__(self, element: Element, /) -> DataConstrRule: ...
    element: Element
    lower_limit: Optional[float]
    """get the lower limit"""
    rule_type: DataConstrType
    """get the constraint type"""
    upper_limit: Optional[float]
    """get the upper limit"""

@final
class DataConstrType:
    """
    The type of a data constraint rule
    """

    Internal: DataConstrType
    Physical: DataConstrType

@final
class DataTypeMap:
    """
    A `DataTypeMap` maps an `ImplementationDataType` to an `ApplicationDataType`
    """

    def __init__(self, element: Element, /) -> DataTypeMap: ...
    application_data_type: ApplicationDataType
    """Get the `ApplicationDataType` of the `DataTypeMap`"""
    element: Element
    implementation_data_type: ImplementationDataType
    """Get the `ImplementationDataType` of the `DataTypeMap`"""

@final
class DataTypeMappingSet:
    """
    A [`DataTypeMappingSet`] contains `DataTypeMap`s

    Use [`ArPackage::create_data_type_mapping_set`] to create a new `DataTypeMappingSet`
    """

    def __init__(self, element: Element, /) -> DataTypeMappingSet: ...
    def create_data_type_map(
        self,
        implementation_data_type: ImplementationDataType,
        application_data_type: ApplicationDataType,
        /,
    ) -> DataTypeMap:
        """Create a new `DataTypeMap` in the `DataTypeMappingSet`"""
        ...

    def data_type_maps(self, /) -> Iterator[DataTypeMap]:
        """Get an iterator over the `DataTypeMap`s in the `DataTypeMappingSet`"""
        ...
    element: Element
    name: str

@final
class ImplementationDataCategory:
    """
    The category of an implementation data type
    """

    Array: ImplementationDataCategory
    DataReference: ImplementationDataCategory
    FunctionReference: ImplementationDataCategory
    Structure: ImplementationDataCategory
    TypeReference: ImplementationDataCategory
    Union: ImplementationDataCategory
    Value: ImplementationDataCategory

@final
class ImplementationDataType:
    """
    An implementation data type; specifics are determined by the category

    Use [`ArPackage::create_implementation_data_type`] to create a new implementation data type
    """

    def __init__(self, element: Element, /) -> ImplementationDataType: ...
    def apply_settings(self, settings: ImplementationDataTypeSettings, /) -> None:
        """apply the settings to this implementation data type

        Calling this method completely replaces the existing settings of the implementation data type,
        deleting existing sub-elements and creating new ones according to the settings
        """
        ...
    array_size: Optional[int]
    """array size of this implementation data type [category: ARRAY]"""
    base_type: Optional[SwBaseType]
    """`SwBaseType` of this implementation data type [category: VALUE]"""
    category: Optional[ImplementationDataCategory]
    """category of this implementation data type"""
    compu_method: Optional[CompuMethod]
    """`CompuMethod` of this implementation data type [category: VALUE, `TYPE_REFERENCE`]"""
    data_constraint: Optional[DataConstr]
    """`DataConstr` of this implementation data type [category: VALUE, `TYPE_REFERENCE`]"""
    data_pointer_target: Optional[DataPointerTarget]
    """get the target type of the data pointer [category: DATA_REFERENCE]"""
    element: Element
    name: str
    referenced_type: Optional[ImplementationDataType]
    """get the referenced implementation data type [category: `TYPE_REFERENCE`]"""
    def settings(self, /) -> Optional[ImplementationDataTypeSettings]:
        """get the settings of this implementation data type"""
        ...

    def sub_elements(self, /) -> Iterator[ImplementationDataTypeElement]:
        """create an iterator over the sub-elements of this implementation data type"""
        ...

@final
class ImplementationDataTypeElement:
    """
    An element of an implementation data type
    """

    def __init__(self, element: Element, /) -> ImplementationDataTypeElement: ...
    def apply_settings(self, settings: ImplementationDataTypeSettings, /) -> None:
        """apply the settings to this implementation data type

        Calling this method completely replaces the existing settings of the implementation data type,
        deleting existing sub-elements and creating new ones according to the settings
        """
        ...
    array_size: Optional[int]
    """array size of this implementation data type [category: ARRAY]"""
    base_type: Optional[SwBaseType]
    """`SwBaseType` of this implementation data type [category: VALUE]"""
    category: Optional[ImplementationDataCategory]
    """category of this implementation data type"""
    compu_method: Optional[CompuMethod]
    """`CompuMethod` of this implementation data type [category: VALUE, `TYPE_REFERENCE`]"""
    data_constraint: Optional[DataConstr]
    """`DataConstr` of this implementation data type [category: VALUE, `TYPE_REFERENCE`]"""
    data_pointer_target: Optional[DataPointerTarget]
    """get the target type of the data pointer [category: DATA_REFERENCE]"""
    element: Element
    name: str
    referenced_type: Optional[ImplementationDataType]
    """get the referenced implementation data type [category: `TYPE_REFERENCE`]"""
    def settings(self) -> ImplementationDataTypeSettings:
        """get the settings of this implementation data type"""
        ...

    def sub_elements(self, /) -> Iterator[ImplementationDataTypeElement]:
        """create an iterator over the sub-elements of this implementation data type"""
        ...

@final
class ImplementationDataTypeSettings:
    """
    Settings for an implementation data type

    This structure is used to create new implementation data types
    """

    @staticmethod
    def Array(
        name: str, *, length: int, element_type: ImplementationDataTypeSettings
    ) -> ImplementationDataTypeSettings: ...
    @staticmethod
    def DataReference(
        name: str, *, target: DataPointerTarget
    ) -> ImplementationDataTypeSettings: ...
    @staticmethod
    def FunctionReference(name: str) -> ImplementationDataTypeSettings: ...
    @staticmethod
    def Structure(
        name: str, *, elements: List[ImplementationDataTypeSettings]
    ) -> ImplementationDataTypeSettings: ...
    @staticmethod
    def TypeReference(
        name: str,
        *,
        reftype: ImplementationDataType,
        compu_method: Optional[CompuMethod] = None,
        data_constraint: Optional[DataConstr] = None,
    ) -> ImplementationDataTypeSettings: ...
    @staticmethod
    def Union(
        name: str, *, elements: List[ImplementationDataTypeSettings]
    ) -> ImplementationDataTypeSettings: ...
    @staticmethod
    def Value(
        name: str,
        *,
        base_type: SwBaseType,
        compu_method: Optional[CompuMethod] = None,
        data_constraint: Optional[DataConstr] = None,
    ) -> ImplementationDataTypeSettings: ...

@final
class ImplementationDataTypeSettings_Array(ImplementationDataTypeSettings):
    name: str
    element_type: ImplementationDataType
    length: int

@final
class ImplementationDataTypeSettings_DataReference(ImplementationDataTypeSettings):
    name: str
    data_pointer_target: DataPointerTarget
    """get the target type of the data pointer"""

@final
class ImplementationDataTypeSettings_FunctionReference(ImplementationDataTypeSettings):
    name: str

@final
class ImplementationDataTypeSettings_Structure(ImplementationDataTypeSettings):
    name: str
    elements: List[ImplementationDataTypeElement]

@final
class ImplementationDataTypeSettings_TypeReference(ImplementationDataTypeSettings):
    name: str
    compu_method: CompuMethod
    data_constraint: DataConstr
    reftype: ImplementationDataType

@final
class ImplementationDataTypeSettings_Union(ImplementationDataTypeSettings):
    name: str
    elements: List[ImplementationDataTypeElement]

@final
class ImplementationDataTypeSettings_Value(ImplementationDataTypeSettings):
    name: str
    base_type: SwBaseType
    compu_method: CompuMethod
    data_constraint: DataConstr

@final
class LinearConversionParameters:
    """
    Linear conversion parameters for CompuMethodScaleLinearContent and CompuMethodScaleLinearAndTextTable
    """

    def __init__(
        self,
        *,
        direction: CompuScaleDirection,
        offset: float,
        factor: float,
        divisor: float,
        lower_limit: float,
        upper_limit: float,
    ) -> LinearConversionParameters: ...
    direction: CompuScaleDirection
    """direction of the conversion"""
    divisor: float
    """divisor"""
    factor: float
    """factor"""
    lower_limit: float
    """lower limit of the scale"""
    offset: float
    """offset"""
    upper_limit: float
    """upper limit of the scale"""

@final
class NotAvailableValueSpecification:
    """
    A value specification that is not available. It is used for ApplicationRecordElements where the attribute isOptional ist set to the value True.
    """

    def __init__(
        self, /, *, default_pattern: Optional[int] = None, label: Optional[str] = None
    ) -> NotAvailableValueSpecification: ...
    label: Optional[str]
    """label of the `NotAvailableValueSpecification`, if any"""
    default_pattern: Optional[int]
    """default pattern to fill unused memory, if any"""

@final
class NumericalRuleBasedValueSpecification:
    """
    A numerical rule based value specification

    This is a value specification that contains a rule based value specification for numerical values.
    """

    def __init__(
        self,
        rule_based_values: RuleBasedValueSpecification,
        /,
        *,
        label: Optional[str] = None,
    ) -> NumericalRuleBasedValueSpecification: ...
    rule_based_values: RuleBasedValueSpecification
    """the rule based value specification that contains the numerical values"""
    label: Optional[str]
    """label of the numerical rule based value specification, if any"""

@final
class NumericalValueSpecification:
    """
    A numerical value specification

    This is a simple value specification that contains a single float value.
    """

    def __init__(
        self, value: float, /, *, label: Optional[str] = None
    ) -> NumericalValueSpecification: ...
    value: float
    """the numerical value"""
    label: Optional[str]
    """label of the numerical value specification, if any"""

@final
class RationalConversionParameters:
    """
    Description of the content of a `CompuMethod` whose category is `Rational`
    """

    def __init__(
        self,
        *,
        direction: CompuScaleDirection,
        denominator: List[float],
        numerator: List[float],
        lower_limit: float,
        upper_limit: float,
    ) -> RationalConversionParameters:
        """Initialize self.  See help(type(self)) for accurate signature."""
        ...
    denominator: List[float]
    """list of numerator coefficients"""
    direction: CompuScaleDirection
    """direction of the conversion"""
    lower_limit: float
    """lower limit of the scale"""
    numerator: List[float]
    """list of denominator coefficients"""
    upper_limit: float
    """upper limit of the scale"""
    ...

@final
class RecordValueSpecification:
    """
    A record value specification
    """

    def __init__(
        self, values: List[ValueSpecification], /, *, label: Optional[str] = None
    ) -> RecordValueSpecification: ...
    values: List[ValueSpecification]
    """list of values in the record"""
    label: Optional[str]
    """label of the record value specification, if any"""

@final
class ReferenceValueSpecification:
    """
    A reference value specification

    This is a value specification that contains a reference to a DataPrototype which defines the target value specification.
    """

    def __init__(
        self, reference_value: DataPrototype, /, *, label: Optional[str] = None
    ) -> ReferenceValueSpecification: ...
    reference_value: DataPrototype
    """the target data prototype that is referenced"""
    label: Optional[str]
    """label of the `ReferenceValueSpecification`, if any"""

@final
class RuleArgument:
    """
    argument of a rule-based value specification
    """

    V: Type[RuleArgument_V]
    """V: argument is a numerical value"""
    Vf: Type[RuleArgument_Vf]
    """VF: argument is a numerical value"""
    Vt: Type[RuleArgument_Vt]
    """VT: argument is a text value"""
    VtfNumber: Type[RuleArgument_VtfNumber]
    """VTF: argument is a numerical value"""
    VtfText: Type[RuleArgument_VtfText]
    """VTF: argument is a text value"""

@final
class RuleArgument_V(RuleArgument):
    """
    argument is a numerical value
    """

    def __init__(self, value: float, /) -> RuleArgument_V: ...
    value: float
    """the numerical value of the argument"""

@final
class RuleArgument_Vf(RuleArgument):
    """
    argument is a numerical value
    """

    def __init__(self, value: float, /) -> RuleArgument_Vf: ...
    value: float
    """the numerical value of the argument"""

@final
class RuleArgument_Vt(RuleArgument):
    """
    argument is a text value
    """

    def __init__(self, value: str, /) -> RuleArgument_Vt: ...
    value: str
    """the text value of the argument"""

@final
class RuleArgument_VtfNumber(RuleArgument):
    """
    argument is a numerical value
    """

    def __init__(self, value: float, /) -> RuleArgument_VtfNumber: ...
    value: float
    """the numerical value of the argument"""

@final
class RuleArgument_VtfText(RuleArgument):
    """
    argument is a text value
    """

    def __init__(self, value: str, /) -> RuleArgument_VtfText: ...
    value: str
    """the text value of the argument"""

@final
class RuleBasedAxisCont:
    """
    Specification of the axis values of a compound primitive data type (curve, map) in a rule-based definition
    """

    def __init__(
        self,
        category: SwAxisContCategory,
        sw_array_size: List[int],
        sw_axis_index: int,
        rule_based_values: RuleBasedValueSpecification,
        unit: Optional[Unit] = None,
    ) -> RuleBasedAxisCont: ...
    category: SwAxisContCategory
    """category of the axis content"""
    sw_array_size: List[int]
    """array size of the axis content"""
    sw_axis_index: int
    """index of the axis in the compound primitive data type"""
    rule_based_values: RuleBasedValueSpecification
    """rule based value specification of the axis content"""
    unit: Optional[Unit]
    """unit of the axis content, if any"""

@final
class RuleBasedFillUntil:
    """
    standard fill rules for rule based value specifications
    """

    End: RuleBasedFillUntil
    """
    `FILL_UNTIL_END`: fills the value of the last RuleBasedValueSpecification.arguments
    until the last element of the array has been filled
    """
    MaxSize: RuleBasedFillUntil
    """
    `FILL_UNTIL_MAX_SIZE`: fills the value of the last RuleBasedValueSpecification.arguments
    until maxSizeToFill elements of the array have been filled
    """

@final
class RuleBasedValueCont:
    """
    Specification of the value content of a compound primitive data type (curve, map) in a rule-based definition
    """

    def __init__(
        self,
        rule_based_values: RuleBasedValueSpecification,
        sw_array_size: List[int],
        unit: Optional[Unit] = None,
    ) -> RuleBasedValueCont: ...
    rule_based_values: RuleBasedValueSpecification
    """rule based value specification of the value content"""
    sw_array_size: List[int]
    """array size of the value content"""
    unit: Optional[Unit]
    """unit of the value content, if any"""

@final
class RuleBasedValueSpecification:
    """
    A rule based value specification
    """

    def __init__(
        self,
        arguments: List[RuleArgument],
        rule: RuleBasedFillUntil,
        max_size_to_fill: Optional[int] = None,
    ) -> RuleBasedValueSpecification: ...
    arguments: List[RuleArgument]
    """list of `RuleArgument` objects in the rule based value specification"""
    rule: RuleBasedFillUntil
    """rule for filling the rule based value specification"""
    max_size_to_fill: Optional[int]
    """maximum size to fill in the rule based value specification, if any"""

@final
class SwAxisCont:
    """
    Specification of the axis values of a compound primitive data type (curve, map)
    """

    def __init__(
        self,
        category: SwAxisContCategory,
        sw_array_size: List[int],
        sw_axis_index: int,
        sw_values_phys: List[SwValue],
        /,
        *,
        unit: Optional[Unit] = None,
        unit_display_name: Optional[str] = None,
    ) -> SwAxisCont: ...
    category: SwAxisContCategory
    """category of the axis content"""
    sw_array_size: List[int]
    """array size of the axis content"""
    sw_axis_index: int
    """index of the axis in the compound primitive data type"""
    sw_values_phys: List[SwValue]
    """list of `SwValue` objects in the axis content"""
    unit: Optional[Unit]
    """unit of the axis content, if any"""
    unit_display_name: Optional[str]
    """display name of the unit, if any"""

@final
class SwAxisContCategory:
    """
    Category of a `SwAxisCont`
    """

    StdAxis: SwAxisContCategory
    """standard axis"""
    ComAxis: SwAxisContCategory
    """commmon axis"""
    ResAxis: SwAxisContCategory
    """rescale axis"""

@final
class SwBaseType:
    """
    `SwBaseType` is a basic data type.

    It is used to define the data types of signals and variables.
    """

    def __init__(self, element: Element, /) -> SwBaseType: ...
    base_type_encoding: BaseTypeEncoding
    """set the base type encoding of the `SwBaseType`"""
    bit_length: Optional[int]
    """get the bit length of the `SwBaseType`"""
    byte_order: Optional[ByteOrder]
    """set the byte order of the `SwBaseType`
        
    The byte order is platform specific and should only be set when it is really needed."""
    element: Element
    mem_alignment: Optional[int]
    """set the memory alignment of the `SwBaseType`
    
    The memory alignment describes the slignement in bits. Example: 8 means that the type is aligned to a byte.
    Since the memory alignment is platform specific, it should only be set when it is really needed."""
    name: str
    native_declaration: Optional[str]
    """set the native declaration of the `SwBaseType`

    The native declaration is a string that represents the type in the native programming language."""

@final
class SwValue:
    """
    a single value of a compound primitive data type (curve, map)
    """

    V: Type[SwValue_V]
    """V: value is a numerical value"""
    Vf: Type[SwValue_Vf]
    """VF: value is a numerical value"""
    Vt: Type[SwValue_Vt]
    """VT: value is a text value"""
    Vg: Type[SwValue_Vg]
    """VG: value is a group of values"""
    VtfNumber: Type[SwValue_VtfNumber]
    """VTF: value is a numerical value"""
    VtfText: Type[SwValue_VtfText]

@final
class SwValue_V(SwValue):
    """
    value is a numerical value
    """

    def __init__(self, value: float, /) -> SwValue_V: ...
    value: float
    """the numerical value of the value"""

@final
class SwValue_Vf(SwValue):
    """
    value is a numerical value
    """

    def __init__(self, value: float, /) -> SwValue_Vf: ...
    value: float
    """the numerical value of the value"""

@final
class SwValue_Vt(SwValue):
    """
    value is a text value
    """

    def __init__(self, value: str, /) -> SwValue_Vt: ...
    value: str
    """the text value of the value"""

@final
class SwValue_Vg(SwValue):
    """
    value is a group of values
    """

    def __init__(
        self, values: List[SwValue], label: Optional[str] = None
    ) -> SwValue_Vg: ...
    values: List[SwValue]
    """list of `SwValue` objects in the group"""
    label: Optional[str]
    """label of the group, if any"""

@final
class SwValue_VtfNumber(SwValue):
    """
    value is a numerical value
    """

    def __init__(self, value: float, /) -> SwValue_VtfNumber: ...
    value: float
    """the numerical value of the value"""

@final
class SwValue_VtfText(SwValue):
    """
    value is a text value
    """

    def __init__(self, value: str, /) -> SwValue_VtfText: ...
    value: str
    """the text value of the value"""

@final
class SwValueCont:
    """
    Specification of the values of a compound primitive data type (curve, map)
    """

    def __init__(
        self, sw_array_size: List[int], sw_values_phys: List[SwValue]
    ) -> SwValueCont: ...
    sw_array_size: List[int]
    """array size of the value content"""
    sw_values_phys: List[SwValue]
    """list of `SwValue` objects in the value content"""

@final
class TabNoIntpEntry:
    """
    a single entry of a `CompuMethod` whose category is `TabNoInterpretation`
    """

    def __init__(self, *, value_in: float, value_out: float) -> TabNoIntpEntry: ...
    value_in: float
    """input value"""
    value_out: float
    """output value"""

@final
class TextTableEntry:
    def __init__(self, *, text: str, value: float) -> TextTableEntry: ...
    text: str
    """text"""
    value: float
    """value"""

@final
class TextValueSpecification:
    """
    A text value specification

    This is a simple value specification that contains a single string value.
    """

    def __init__(
        self, value: str, /, *, label: Optional[str] = None
    ) -> TextValueSpecification: ...
    value: str
    """the text value"""
    label: Optional[str]
    """label of the text value specification, if any"""

@final
class Unit:
    """
    `Unit` represents a unit of measurement.

    Use [`ArPackage::create_unit`] to create a new unit.
    """

    def __init__(self, element: Element, /) -> Unit: ...
    display_name: Optional[str]
    """display name of the unit"""
    element: Element
    name: str
