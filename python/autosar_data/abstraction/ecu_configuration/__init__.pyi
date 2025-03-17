# Stub file for ecu_configuration

from typing import final, Iterator, List, Optional, Tuple, TypeAlias, Union
from autosar_data import Element
from autosar_data.abstraction import System

EcucContainerDef: TypeAlias = Union[EcucParamConfContainerDef, EcucChoiceContainerDef]
EcucAnyReferenceDef: TypeAlias = Union[
    EcucReferenceDef,
    EcucUriReferenceDef,
    EcucForeignReferenceDef,
    EcucInstanceReferenceDef,
    EcucChoiceReferenceDef,
]
EcucParameterDef: TypeAlias = Union[
    EcucAddInfoParamDef,
    EcucBooleanParamDef,
    EcucIntegerParamDef,
    EcucEnumerationParamDef,
    EcucFloatParamDef,
    EcucFunctionNameDef,
    EcucLinkerSymbolDef,
    EcucMultilineStringParamDef,
    EcucStringParamDef,
]
EcucParameterValue: TypeAlias = Union[
    EcucNumericalParamValue, EcucTextualParamValue, EcucAddInfoParamValue
]

@final
class EcucAddInfoParamDef:
    """
    `EcucAddInfoParamDef` is used to specify the need for formated text in the ECU configuration value description
    """

    def __init__(self, element: Element, /) -> EcucAddInfoParamDef: ...
    element: Element
    lower_multiplicity: Optional[int]
    """get or set the lower multiplicity attribute"""
    multiplicity_config_classes: List[
        Tuple[EcucConfigurationClass, EcucConfigurationVariant]
    ]
    """get or set the multiplicity config classes of the parameter definition.
    If an empty list is provided, the multiplicity config classes are removed.
    
    This setting is required if the containing EcucModuleDef has the category VENDOR_SPECIFIC_MODULE_DEFINITION."""
    name: str
    origin: Optional[str]
    """get or set the origin of the parameter definition
    
    The origin is a string that describes if the parameter was defined in the AUTOSAR standard or by a vendor.
    Standardized parameters use the origin "AUTOSAR_ECUC", while vendors are supposed to use string like 'VendorXyz_v1.3'"""
    post_build_variant_multiplicity: Optional[bool]
    """get or set the postBuildVariantMultiplicity attribute
    
    If postBuildVariantMultiplicity is true, then the parameter or reference
    may have a different number of instances in different post-build variants."""
    post_build_variant_value: Optional[bool]
    """get or set the postBuildVariantValue attribute
    
    If postBuildVariantValue is true, then the parameter or reference
    may have different values in different post-build variants."""
    requires_index: Optional[bool]
    """get or set the requiresIndex attribute"""
    upper_multiplicity: Optional[int]
    """get or set the upper multiplicity attribute"""
    upper_multiplicity_infinite: Optional[bool]
    """get or set the upper multiplicity infinite attribute
    
    if this attribute is set to true, the upper multiplicity is infinite
    (i.e. the module definition can be used an arbitrary number of times)
    When this attribute is true, the upper multiplicity attribute may not be used."""
    value_config_classes: List[Tuple[EcucConfigurationClass, EcucConfigurationVariant]]
    """set the value config classes of the parameter definition.
    
    If an empty list is provided, the value config classes are removed.
    According to the specification setting is required if the containing EcucModuleDef
    has the category VENDOR_SPECIFIC_MODULE_DEFINITION, but in practice it is rarely used."""
    with_auto: Optional[bool]
    """get or set the withAuto attribute
    
    If withAuto is true, then the parameter or reference is allowed to set its isAutoValue attribute to true."""

@final
class EcucAddInfoParamValue:
    """
    The `EcucAddInfoParamValue` holds descriptive text and takes the role of a parameter in the ECU configuration
    """

    def __init__(self, element: Element, /) -> EcucAddInfoParamValue: ...
    element: Element

@final
class EcucBooleanParamDef:
    """
    `EcucBooleanParamDef` is used to specify a boolean parameter in the ECU configuration
    """

    def __init__(self, element: Element, /) -> EcucBooleanParamDef: ...
    default_value: Optional[bool]
    """set the default value of the boolean parameter"""
    element: Element
    lower_multiplicity: Optional[int]
    """get or set the lower multiplicity attribute"""
    multiplicity_config_classes: List[
        Tuple[EcucConfigurationClass, EcucConfigurationVariant]
    ]
    """set the multiplicity config classes of the parameter definition.
    If an empty list is provided, the multiplicity config classes are removed.
    
    This setting is required if the containing EcucModuleDef has the category VENDOR_SPECIFIC_MODULE_DEFINITION."""
    name: str
    origin: Optional[str]
    """set the origin of the parameter definition
    
    The origin is a string that describes if the parameter was defined in the AUTOSAR standard or by a vendor.
    Standardized parameters use the origin "AUTOSAR_ECUC", while vendors are supposed to use string like 'VendorXyz_v1.3'"""
    post_build_variant_multiplicity: Optional[bool]
    """get or set the postBuildVariantMultiplicity attribute
    
    If postBuildVariantMultiplicity is true, then the parameter or reference
    may have a different number of instances in different post-build variants."""
    post_build_variant_value: Optional[bool]
    """get or set the postBuildVariantValue attribute
    
    If postBuildVariantValue is true, then the parameter or reference
    may have different values in different post-build variants."""
    requires_index: Optional[bool]
    """get or set the requiresIndex attribute"""
    upper_multiplicity: Optional[int]
    """get or set the upper multiplicity attribute"""
    upper_multiplicity_infinite: Optional[bool]
    """get or set the upper multiplicity infinite attribute
    
    if this attribute is set to true, the upper multiplicity is infinite
    (i.e. the module definition can be used an arbitrary number of times)
    When this attribute is true, the upper multiplicity attribute may not be used."""
    value_config_classes: List[Tuple[EcucConfigurationClass, EcucConfigurationVariant]]
    """set the value config classes of the parameter definition.
    
    If an empty list is provided, the value config classes are removed.
    According to the specification setting is required if the containing EcucModuleDef
    has the category VENDOR_SPECIFIC_MODULE_DEFINITION, but in practice it is rarely used."""
    with_auto: Optional[bool]
    """get or set the withAuto attribute
    
    If withAuto is true, then the parameter or reference is allowed to set its isAutoValue attribute to true."""

@final
class EcucChoiceContainerDef:
    """
    Marker trait for container definitions
    The `EcucChoiceContainerDef` is used to define configuration containers
    that provide a choice between several EcucParamConfContainerDef
    """

    def __init__(self, element: Element, /) -> EcucChoiceContainerDef: ...
    def choices(self, /) -> Iterator[EcucParamConfContainerDef]:
        """iterate over the choices in the container"""
        ...

    def create_param_conf_container_def(
        self, name: str, /
    ) -> EcucParamConfContainerDef:
        """create a new `EcucParamConfContainerDef` as one of the choices in this choice container"""
        ...
    element: Element
    lower_multiplicity: Optional[int]
    """get or set the lower multiplicity attribute"""
    name: str
    upper_multiplicity: Optional[int]
    """get or set the upper multiplicity attribute"""
    upper_multiplicity_infinite: Optional[bool]
    """get or set the upper multiplicity infinite attribute
    
    if this attribute is set to true, the upper multiplicity is infinite
    (i.e. the module definition can be used an arbitrary number of times)
    When this attribute is true, the upper multiplicity attribute may not be used."""

@final
class EcucChoiceReferenceDef:
    """
    The `EcucChoiceReferenceDef` specifies alternative references where only one of the specified
    references will be used in the ECU configuration.
    """

    def __init__(self, element: Element, /) -> EcucChoiceReferenceDef: ...
    def add_destination(self, /, destination) -> None:
        """add a reference to a destination container"""
        ...

    def destination_refs(self, /) -> Iterator[EcucContainerDef]:
        """get the references to the destination containers"""
        ...
    element: Element
    lower_multiplicity: Optional[int]
    """get or set the lower multiplicity attribute"""
    multiplicity_config_classes: List[
        Tuple[EcucConfigurationClass, EcucConfigurationVariant]
    ]
    """set the multiplicity config classes of the parameter definition.
    If an empty list is provided, the multiplicity config classes are removed.
    
    This setting is required if the containing EcucModuleDef has the category VENDOR_SPECIFIC_MODULE_DEFINITION."""
    name: str
    origin: Optional[str]
    """set the origin of the parameter definition
    
    The origin is a string that describes if the parameter was defined in the AUTOSAR standard or by a vendor.
    Standardized parameters use the origin "AUTOSAR_ECUC", while vendors are supposed to use string like 'VendorXyz_v1.3'"""
    post_build_variant_multiplicity: Optional[bool]
    """get or set the postBuildVariantMultiplicity attribute
    
    If postBuildVariantMultiplicity is true, then the parameter or reference
    may have a different number of instances in different post-build variants."""
    post_build_variant_value: Optional[bool]
    """get or set the postBuildVariantValue attribute
    
    If postBuildVariantValue is true, then the parameter or reference
    may have different values in different post-build variants."""
    requires_index: Optional[bool]
    """get or set the requiresIndex attribute"""
    upper_multiplicity: Optional[int]
    """get or set the upper multiplicity attribute"""
    upper_multiplicity_infinite: Optional[bool]
    """get or set the upper multiplicity infinite attribute
    
    if this attribute is set to true, the upper multiplicity is infinite
    (i.e. the module definition can be used an arbitrary number of times)
    When this attribute is true, the upper multiplicity attribute may not be used."""
    value_config_classes: List[Tuple[EcucConfigurationClass, EcucConfigurationVariant]]
    """set the value config classes of the parameter definition.
    
    If an empty list is provided, the value config classes are removed.
    According to the specification setting is required if the containing EcucModuleDef
    has the category VENDOR_SPECIFIC_MODULE_DEFINITION, but in practice it is rarely used."""
    with_auto: Optional[bool]
    """get or set the withAuto attribute
    
    If withAuto is true, then the parameter or reference is allowed to set its isAutoValue attribute to true."""

@final
class EcucConfigurationClass:
    """
    `EcucConfigurationClass` provides the different configuration classes for Autosar configuration parameters
    """

    Link: EcucConfigurationClass
    PostBuild: EcucConfigurationClass
    PreCompile: EcucConfigurationClass
    PublishedInformation: EcucConfigurationClass

@final
class EcucConfigurationVariant:
    """
    `EcucConfigurationVariant` provides the different configuration variants that
    can be used by the module definition.
    """

    PreconfiguredConfiguration: EcucConfigurationVariant
    RecommendedConfiguration: EcucConfigurationVariant
    VariantLinkTime: EcucConfigurationVariant
    VariantPostBuild: EcucConfigurationVariant
    VariantPostBuildLoadable: EcucConfigurationVariant
    VariantPostBuildSelectable: EcucConfigurationVariant
    VariantPreCompile: EcucConfigurationVariant

@final
class EcucContainerValue:
    """
    The `EcucContainerValue` is a container in the ECU configuration
    """

    def __init__(self, element: Element, /) -> EcucContainerValue: ...
    def create_add_info_param_value(
        self, definition: EcucAddInfoParamDef, /
    ) -> EcucAddInfoParamValue:
        """create a new `EcucTextualParamValue` in the container"""
        ...

    def create_instance_reference(
        self,
        definition: EcucInstanceReferenceDef,
        target_context: List[Element],
        target: Element,
        /,
    ) -> EcucInstanceReferenceValue:
        """create a new instance reference value in the container"""
        ...

    def create_numerical_param_value(
        self, definition: EcucParameterDef, value: str, /
    ) -> EcucNumericalParamValue:
        """create a new `EcucNumericalParamValue` in the container"""
        ...

    def create_reference_value(
        self, definition: EcucReferenceDef, target: Element, /
    ) -> EcucReferenceValue:
        """create a new reference value in the container"""
        ...

    def create_sub_container(
        self, name: str, definition: EcucContainerDef, /
    ) -> EcucContainerValue:
        """create a sub-container"""
        ...

    def create_textual_param_value(
        self, definition: EcucParameterDef, value: str, /
    ) -> EcucTextualParamValue:
        """create a new `EcucTextualParamValue` in the container"""
        ...
    definition: Optional[EcucContainerDef]
    """set the container definition reference"""
    definition_ref: Optional[str]
    """get the definition reference as a string
    
    This function is an alternative to `definition()`; it is useful when the
    referenced definition is not loaded and can't be resolved."""
    element: Element
    index: Optional[int]
    """set the index of the container
    
    If the container definition has `requiresIndex` set to `true`, then the container
    must have an index. Otherwise the index is meaningless."""
    name: str
    def parameter_values(self, /) -> Iterator[EcucParameterValue]:
        """iterate over the parameter values in the container"""
        ...

    def reference_values(
        self, /
    ) -> Iterator[Union[EcucReferenceValue, EcucInstanceReferenceValue]]:
        """iterate over the reference values in the container"""
        ...

    def sub_containers(self, /) -> Iterator[EcucContainerValue]:
        """iterate over the sub-containers in this container"""
        ...

@final
class EcucDefinitionCollection:
    """
    The `EcucDefinitionCollection` is a container for all module definitions in the ECU configuration
    """

    def __init__(self, element: Element, /) -> EcucDefinitionCollection: ...
    def add_module_def(self, module_def: EcucModuleDef, /) -> None:
        """add a reference to a module definition to the collection"""
        ...
    element: Element
    def module_defs(self, /) -> Iterator[EcucModuleDef]:
        """iterate over all module definitions in the collection"""
        ...
    name: str

@final
class EcucDestinationUriDef:
    """
    A `EcucDestinationUriDef` defines a target for an `EcucUriReferenceDef`
    """

    def __init__(self, element: Element, /) -> EcucDestinationUriDef: ...
    def containers(
        self, /
    ) -> Iterator[Union[EcucChoiceContainerDef, EcucParamConfContainerDef]]:
        """iterate over all containers in the destination uri policy"""
        ...

    def create_choice_container_def(self, name: str, /) -> EcucChoiceContainerDef:
        """create an `EcucChoiceContainerDef` in the destination uri policy"""
        ...

    def create_param_conf_container_def(
        self, name: str, /
    ) -> EcucParamConfContainerDef:
        """create an `EcucParamConfContainerDef` in the destination uri policy"""
        ...
    element: Element
    name: str
    nesting_contract: Optional[EcucDestinationUriNestingContract]
    """set the nesting contract for the destination uri"""

@final
class EcucDestinationUriDefSet:
    """
    A `EcucDestinationUriDefSet` contains a list of `EcucDestinationUriDef`s
    """

    def __init__(self, element: Element, /) -> EcucDestinationUriDefSet: ...
    def create_destination_uri_def(
        self, name: str, contract: EcucDestinationUriNestingContract, /
    ) -> EcucDestinationUriDef:
        """create a new `EcucDestinationUriDef`"""
        ...

    def destination_uri_defs(self, /) -> Iterator[EcucDestinationUriDef]:
        """iterate over all destination uri definitions in the set"""
        ...
    element: Element
    name: str

@final
class EcucDestinationUriNestingContract:
    """
    `EcucDestinationUriNestingContract` provides the different nesting contracts for destination URIs
    """

    LeafOfTargetContainer: EcucDestinationUriNestingContract
    TargetContainer: EcucDestinationUriNestingContract
    VertexOfTargetContainer: EcucDestinationUriNestingContract

@final
class EcucEnumerationLiteralDef:
    """
    `EcucEnumerationLiteralDef` is used to specify an enumeration literal in the ECU configuration
    """

    def __init__(self, element: Element, /) -> EcucEnumerationLiteralDef: ...
    element: Element
    name: str

@final
class EcucEnumerationParamDef:
    """
    `EcucEnumerationParamDef` is used to specify an enumeration parameter in the ECU configuration
    """

    def __init__(self, element: Element, /) -> EcucEnumerationParamDef: ...
    def create_enumeration_literal(self, /, name) -> EcucEnumerationLiteralDef:
        """create a new enumeration literal"""
        ...
    default_value: Optional[str]
    """set the default value of the enumeration parameter
    
    Note: enumeration literals must be created first, since the default value must match one of the literals"""
    element: Element
    def enumeration_literals(self, /) -> Iterator[EcucEnumerationLiteralDef]:
        """iterate over all enumeration literals"""
        ...
    lower_multiplicity: Optional[int]
    """get or set the lower multiplicity attribute"""
    multiplicity_config_classes: List[
        Tuple[EcucConfigurationClass, EcucConfigurationVariant]
    ]
    """set the multiplicity config classes of the parameter definition.
    If an empty list is provided, the multiplicity config classes are removed.
    
    This setting is required if the containing EcucModuleDef has the category VENDOR_SPECIFIC_MODULE_DEFINITION."""
    name: str
    origin: Optional[str]
    """set the origin of the parameter definition
    
    The origin is a string that describes if the parameter was defined in the AUTOSAR standard or by a vendor.
    Standardized parameters use the origin "AUTOSAR_ECUC", while vendors are supposed to use string like 'VendorXyz_v1.3'"""
    post_build_variant_multiplicity: Optional[bool]
    """get or set the postBuildVariantMultiplicity attribute
    
    If postBuildVariantMultiplicity is true, then the parameter or reference
    may have a different number of instances in different post-build variants."""
    post_build_variant_value: Optional[bool]
    """get or set the postBuildVariantValue attribute
    
    If postBuildVariantValue is true, then the parameter or reference
    may have different values in different post-build variants."""
    requires_index: Optional[bool]
    """get or set the requiresIndex attribute"""
    upper_multiplicity: Optional[int]
    """get or set the upper multiplicity attribute"""
    upper_multiplicity_infinite: Optional[bool]
    """get or set the upper multiplicity infinite attribute
    
    if this attribute is set to true, the upper multiplicity is infinite
    (i.e. the module definition can be used an arbitrary number of times)
    When this attribute is true, the upper multiplicity attribute may not be used."""
    value_config_classes: List[Tuple[EcucConfigurationClass, EcucConfigurationVariant]]
    """set the value config classes of the parameter definition.
    
    If an empty list is provided, the value config classes are removed.
    According to the specification setting is required if the containing EcucModuleDef
    has the category VENDOR_SPECIFIC_MODULE_DEFINITION, but in practice it is rarely used."""
    with_auto: Optional[bool]
    """get or set the withAuto attribute
    
    If withAuto is true, then the parameter or reference is allowed to set its isAutoValue attribute to true."""

@final
class EcucFloatParamDef:
    """
    `EcucFloatParamDef` is used to specify a float parameter in the ECU configuration
    """

    def __init__(self, element: Element, /) -> EcucFloatParamDef: ...
    default_value: Optional[float]
    """set the default value of the float parameter"""
    element: Element
    lower_multiplicity: Optional[int]
    """get or set the lower multiplicity attribute"""
    max: Optional[float]
    """set the max value of the float parameter"""
    min: Optional[float]
    """set the min value of the float parameter"""
    multiplicity_config_classes: List[
        Tuple[EcucConfigurationClass, EcucConfigurationVariant]
    ]
    """set the multiplicity config classes of the parameter definition.
    If an empty list is provided, the multiplicity config classes are removed.
    
    This setting is required if the containing EcucModuleDef has the category VENDOR_SPECIFIC_MODULE_DEFINITION."""
    name: str
    origin: Optional[str]
    """set the origin of the parameter definition
    
    The origin is a string that describes if the parameter was defined in the AUTOSAR standard or by a vendor.
    Standardized parameters use the origin "AUTOSAR_ECUC", while vendors are supposed to use string like 'VendorXyz_v1.3'"""
    post_build_variant_multiplicity: Optional[bool]
    """get or set the postBuildVariantMultiplicity attribute
    
    If postBuildVariantMultiplicity is true, then the parameter or reference
    may have a different number of instances in different post-build variants."""
    post_build_variant_value: Optional[bool]
    """get or set the postBuildVariantValue attribute
    
    If postBuildVariantValue is true, then the parameter or reference
    may have different values in different post-build variants."""
    requires_index: Optional[bool]
    """get or set the requiresIndex attribute"""
    upper_multiplicity: Optional[int]
    """get or set the upper multiplicity attribute"""
    upper_multiplicity_infinite: Optional[bool]
    """get or set the upper multiplicity infinite attribute
    
    if this attribute is set to true, the upper multiplicity is infinite
    (i.e. the module definition can be used an arbitrary number of times)
    When this attribute is true, the upper multiplicity attribute may not be used."""
    value_config_classes: List[Tuple[EcucConfigurationClass, EcucConfigurationVariant]]
    """set the value config classes of the parameter definition.
    
    If an empty list is provided, the value config classes are removed.
    According to the specification setting is required if the containing EcucModuleDef
    has the category VENDOR_SPECIFIC_MODULE_DEFINITION, but in practice it is rarely used."""
    with_auto: Optional[bool]
    """get or set the withAuto attribute
    
    If withAuto is true, then the parameter or reference is allowed to set its isAutoValue attribute to true."""

@final
class EcucForeignReferenceDef:
    """
    marker trait for all reference definitions
    The `EcucForeignReferenceDef` specifies a reference to an XML description of an entity
    described in another AUTOSAR template.
    """

    def __init__(self, element: Element, /) -> EcucForeignReferenceDef: ...
    destination_type: Optional[str]
    """set the destination type of the reference definition"""
    element: Element
    lower_multiplicity: Optional[int]
    """get or set the lower multiplicity attribute"""
    multiplicity_config_classes: List[
        Tuple[EcucConfigurationClass, EcucConfigurationVariant]
    ]
    """set the multiplicity config classes of the parameter definition.
    If an empty list is provided, the multiplicity config classes are removed.
    
    This setting is required if the containing EcucModuleDef has the category VENDOR_SPECIFIC_MODULE_DEFINITION."""
    name: str
    origin: Optional[str]
    """set the origin of the parameter definition
    
    The origin is a string that describes if the parameter was defined in the AUTOSAR standard or by a vendor.
    Standardized parameters use the origin "AUTOSAR_ECUC", while vendors are supposed to use string like 'VendorXyz_v1.3'"""
    post_build_variant_multiplicity: Optional[bool]
    """get or set the postBuildVariantMultiplicity attribute
    
    If postBuildVariantMultiplicity is true, then the parameter or reference
    may have a different number of instances in different post-build variants."""
    post_build_variant_value: Optional[bool]
    """get or set the postBuildVariantValue attribute
    
    If postBuildVariantValue is true, then the parameter or reference
    may have different values in different post-build variants."""
    requires_index: Optional[bool]
    """get or set the requiresIndex attribute"""
    upper_multiplicity: Optional[int]
    """get or set the upper multiplicity attribute"""
    upper_multiplicity_infinite: Optional[bool]
    """get or set the upper multiplicity infinite attribute
    
    if this attribute is set to true, the upper multiplicity is infinite
    (i.e. the module definition can be used an arbitrary number of times)
    When this attribute is true, the upper multiplicity attribute may not be used."""
    value_config_classes: List[Tuple[EcucConfigurationClass, EcucConfigurationVariant]]
    """set the value config classes of the parameter definition.
    
    If an empty list is provided, the value config classes are removed.
    According to the specification setting is required if the containing EcucModuleDef
    has the category VENDOR_SPECIFIC_MODULE_DEFINITION, but in practice it is rarely used."""
    with_auto: Optional[bool]
    """get or set the withAuto attribute
    
    If withAuto is true, then the parameter or reference is allowed to set its isAutoValue attribute to true."""

@final
class EcucFunctionNameDef:
    """
    `EcucFunctionNameDef` is used to specify a function name parameter in the ECU configuration
    """

    def __init__(self, element: Element, /) -> EcucFunctionNameDef: ...
    element: Element
    lower_multiplicity: Optional[int]
    """get or set the lower multiplicity attribute"""
    multiplicity_config_classes: List[
        Tuple[EcucConfigurationClass, EcucConfigurationVariant]
    ]
    """set the multiplicity config classes of the parameter definition.
    If an empty list is provided, the multiplicity config classes are removed.
    
    This setting is required if the containing EcucModuleDef has the category VENDOR_SPECIFIC_MODULE_DEFINITION."""
    name: str
    origin: Optional[str]
    """set the origin of the parameter definition
    
    The origin is a string that describes if the parameter was defined in the AUTOSAR standard or by a vendor.
    Standardized parameters use the origin "AUTOSAR_ECUC", while vendors are supposed to use string like 'VendorXyz_v1.3'"""
    post_build_variant_multiplicity: Optional[bool]
    """get or set the postBuildVariantMultiplicity attribute
    
    If postBuildVariantMultiplicity is true, then the parameter or reference
    may have a different number of instances in different post-build variants."""
    post_build_variant_value: Optional[bool]
    """get or set the postBuildVariantValue attribute
    
    If postBuildVariantValue is true, then the parameter or reference
    may have different values in different post-build variants."""
    requires_index: Optional[bool]
    """get or set the requiresIndex attribute"""
    upper_multiplicity: Optional[int]
    """get or set the upper multiplicity attribute"""
    upper_multiplicity_infinite: Optional[bool]
    """get or set the upper multiplicity infinite attribute
    
    if this attribute is set to true, the upper multiplicity is infinite
    (i.e. the module definition can be used an arbitrary number of times)
    When this attribute is true, the upper multiplicity attribute may not be used."""
    value_config_classes: List[Tuple[EcucConfigurationClass, EcucConfigurationVariant]]
    """set the value config classes of the parameter definition.
    
    If an empty list is provided, the value config classes are removed.
    According to the specification setting is required if the containing EcucModuleDef
    has the category VENDOR_SPECIFIC_MODULE_DEFINITION, but in practice it is rarely used."""
    with_auto: Optional[bool]
    """get or set the withAuto attribute
    
    If withAuto is true, then the parameter or reference is allowed to set its isAutoValue attribute to true."""
    max_length: Optional[int]
    """max length of the string parameter"""
    min_length: Optional[int]
    """min length of the string parameter"""
    regular_expression: Optional[str]
    """regular expression for the string parameter"""
    default_value: Optional[str]
    """default value of the string parameter"""

@final
class EcucInstanceReferenceDef:
    """
    The `EcucInstanceReferenceDef` specifies a reference to an XML description of an entity
    described in another AUTOSAR template using INSTANCE REFERENCE semantics.
    """

    def __init__(self, element: Element, /) -> EcucInstanceReferenceDef: ...
    destination_context: Optional[str]
    """set the destination context of the reference definition
    
    The destination context is a string of autosar element names separated by spaces.
    Additionally, the '*' character can be used to indicate multiple occurrences of the previous element.
    E.g. "SW-COMPONENT-PROTOTYPE* R-PORT-PROTOTYPE"""
    destination_type: Optional[str]
    """set the destination type of the reference definition"""
    element: Element
    lower_multiplicity: Optional[int]
    """get or set the lower multiplicity attribute"""
    multiplicity_config_classes: List[
        Tuple[EcucConfigurationClass, EcucConfigurationVariant]
    ]
    """set the multiplicity config classes of the parameter definition.
    If an empty list is provided, the multiplicity config classes are removed.
    
    This setting is required if the containing EcucModuleDef has the category VENDOR_SPECIFIC_MODULE_DEFINITION."""
    name: str
    origin: Optional[str]
    """set the origin of the parameter definition
    
    The origin is a string that describes if the parameter was defined in the AUTOSAR standard or by a vendor.
    Standardized parameters use the origin "AUTOSAR_ECUC", while vendors are supposed to use string like 'VendorXyz_v1.3'"""
    post_build_variant_multiplicity: Optional[bool]
    """get or set the postBuildVariantMultiplicity attribute
    
    If postBuildVariantMultiplicity is true, then the parameter or reference
    may have a different number of instances in different post-build variants."""
    post_build_variant_value: Optional[bool]
    """get or set the postBuildVariantValue attribute
    
    If postBuildVariantValue is true, then the parameter or reference
    may have different values in different post-build variants."""
    requires_index: Optional[bool]
    """get or set the requiresIndex attribute"""
    upper_multiplicity: Optional[int]
    """get or set the upper multiplicity attribute"""
    upper_multiplicity_infinite: Optional[bool]
    """get or set the upper multiplicity infinite attribute
    
    if this attribute is set to true, the upper multiplicity is infinite
    (i.e. the module definition can be used an arbitrary number of times)
    When this attribute is true, the upper multiplicity attribute may not be used."""
    value_config_classes: List[Tuple[EcucConfigurationClass, EcucConfigurationVariant]]
    """set the value config classes of the parameter definition.
    
    If an empty list is provided, the value config classes are removed.
    According to the specification setting is required if the containing EcucModuleDef
    has the category VENDOR_SPECIFIC_MODULE_DEFINITION, but in practice it is rarely used."""
    with_auto: Optional[bool]
    """get or set the withAuto attribute
    
    If withAuto is true, then the parameter or reference is allowed to set its isAutoValue attribute to true."""

@final
class EcucInstanceReferenceValue:
    """
    An `EcucInstanceReferenceValue` provides the mechanism to reference an instance of a prototype
    """

    def __init__(self, element: Element, /) -> EcucInstanceReferenceValue: ...
    definition: Optional[EcucInstanceReferenceDef]
    """set the parameter definition reference"""
    definition_ref: Optional[str]
    """get the parameter definition reference as a string
    
    This function is an alternative to `definition()`; it is useful when the
    referenced definition is not loaded and can't be resolved."""
    element: Element
    index: Optional[int]
    """set the index of the reference
    
    If the reference definition has `requiresIndex` set to `true`, then the reference
    must have an index. Otherwise the index is meaningless."""
    is_auto_value: Optional[bool]
    """set the isAutoValue flag
    
    If the reference definition has `withAuto` set to `true`, then the reference is allowed to have an auto value."""
    target: Optional[Tuple[List[Element], Element]]
    """Set the target of the reference
    
    An instance reference targets a specific instance of a prototype. In order to uniquely identify the target,
    the target context is required. The target context is a list of elements that are the parent elements of the
    target element. The instance reference definition specifies which context elements are required."""

@final
class EcucIntegerParamDef:
    """
    `EcucIntegerParamDef` is used to specify an integer parameter in the ECU configuration
    """

    def __init__(self, element: Element, /) -> EcucIntegerParamDef: ...
    default_value: Optional[int]
    """set the default value of the integer parameter"""
    element: Element
    lower_multiplicity: Optional[int]
    """get or set the lower multiplicity attribute"""
    max: Optional[int]
    """set the max value of the integer parameter"""
    min: Optional[int]
    """set the min value of the integer parameter"""
    multiplicity_config_classes: List[
        Tuple[EcucConfigurationClass, EcucConfigurationVariant]
    ]
    """set the multiplicity config classes of the parameter definition.
    If an empty list is provided, the multiplicity config classes are removed.
    
    This setting is required if the containing EcucModuleDef has the category VENDOR_SPECIFIC_MODULE_DEFINITION."""
    name: str
    origin: Optional[str]
    """set the origin of the parameter definition
    
    The origin is a string that describes if the parameter was defined in the AUTOSAR standard or by a vendor.
    Standardized parameters use the origin "AUTOSAR_ECUC", while vendors are supposed to use string like 'VendorXyz_v1.3'"""
    post_build_variant_multiplicity: Optional[bool]
    """get or set the postBuildVariantMultiplicity attribute
    
    If postBuildVariantMultiplicity is true, then the parameter or reference
    may have a different number of instances in different post-build variants."""
    post_build_variant_value: Optional[bool]
    """get or set the postBuildVariantValue attribute
    
    If postBuildVariantValue is true, then the parameter or reference
    may have different values in different post-build variants."""
    requires_index: Optional[bool]
    """get or set the requiresIndex attribute"""
    upper_multiplicity: Optional[int]
    """get or set the upper multiplicity attribute"""
    upper_multiplicity_infinite: Optional[bool]
    """get or set the upper multiplicity infinite attribute
    
    if this attribute is set to true, the upper multiplicity is infinite
    (i.e. the module definition can be used an arbitrary number of times)
    When this attribute is true, the upper multiplicity attribute may not be used."""
    value_config_classes: List[Tuple[EcucConfigurationClass, EcucConfigurationVariant]]
    """set the value config classes of the parameter definition.
    
    If an empty list is provided, the value config classes are removed.
    According to the specification setting is required if the containing EcucModuleDef
    has the category VENDOR_SPECIFIC_MODULE_DEFINITION, but in practice it is rarely used."""
    with_auto: Optional[bool]
    """get or set the withAuto attribute
    
    If withAuto is true, then the parameter or reference is allowed to set its isAutoValue attribute to true."""

@final
class EcucLinkerSymbolDef:
    """
    `EcucLinkerSymbolDef` is used to specify a linker symbol parameter in the ECU configuration
    """

    def __init__(self, element: Element, /) -> EcucLinkerSymbolDef: ...
    element: Element
    lower_multiplicity: Optional[int]
    """get or set the lower multiplicity attribute"""
    multiplicity_config_classes: List[
        Tuple[EcucConfigurationClass, EcucConfigurationVariant]
    ]
    """set the multiplicity config classes of the parameter definition.
    If an empty list is provided, the multiplicity config classes are removed.
    
    This setting is required if the containing EcucModuleDef has the category VENDOR_SPECIFIC_MODULE_DEFINITION."""
    name: str
    origin: Optional[str]
    """set the origin of the parameter definition
    
    The origin is a string that describes if the parameter was defined in the AUTOSAR standard or by a vendor.
    Standardized parameters use the origin "AUTOSAR_ECUC", while vendors are supposed to use string like 'VendorXyz_v1.3'"""
    post_build_variant_multiplicity: Optional[bool]
    """get or set the postBuildVariantMultiplicity attribute
    
    If postBuildVariantMultiplicity is true, then the parameter or reference
    may have a different number of instances in different post-build variants."""
    post_build_variant_value: Optional[bool]
    """get or set the postBuildVariantValue attribute
    
    If postBuildVariantValue is true, then the parameter or reference
    may have different values in different post-build variants."""
    requires_index: Optional[bool]
    """get or set the requiresIndex attribute"""
    upper_multiplicity: Optional[int]
    """get or set the upper multiplicity attribute"""
    upper_multiplicity_infinite: Optional[bool]
    """get or set the upper multiplicity infinite attribute
    
    if this attribute is set to true, the upper multiplicity is infinite
    (i.e. the module definition can be used an arbitrary number of times)
    When this attribute is true, the upper multiplicity attribute may not be used."""
    value_config_classes: List[Tuple[EcucConfigurationClass, EcucConfigurationVariant]]
    """set the value config classes of the parameter definition.
    
    If an empty list is provided, the value config classes are removed.
    According to the specification setting is required if the containing EcucModuleDef
    has the category VENDOR_SPECIFIC_MODULE_DEFINITION, but in practice it is rarely used."""
    with_auto: Optional[bool]
    """get or set the withAuto attribute
    
    If withAuto is true, then the parameter or reference is allowed to set its isAutoValue attribute to true."""
    max_length: Optional[int]
    """max length of the string parameter"""
    min_length: Optional[int]
    """min length of the string parameter"""
    regular_expression: Optional[str]
    """regular expression for the string parameter"""
    default_value: Optional[str]
    """default value of the string parameter"""

@final
class EcucModuleConfigurationValues:
    """
    The `EcucModuleConfigurationValues` is a container for the configuration of a single base software module
    """

    def __init__(self, element: Element, /) -> EcucModuleConfigurationValues: ...
    def container_values(self, /) -> Iterator[EcucContainerValue]:
        """create an iterator over the container values in the module configuration"""
        ...

    def create_container_value(
        self, name: str, definition: EcucContainerDef, /
    ) -> EcucContainerValue:
        """Create a new `EcucContainerValue` in the module configuration"""
        ...
    definition: Optional[EcucModuleDef]
    """set the module definition reference"""
    definition_ref: Optional[str]
    """get the definition reference as a string
    
    This function is an alternative to `definition()`; it is useful when the
    referenced definition is not loaded and can't be resolved."""
    element: Element
    name: str

@final
class EcucModuleDef:
    """
    The `EcucModuleDef` is a container for the definition of a single base software module
    """

    def __init__(self, element: Element, /) -> EcucModuleDef: ...
    api_service_prefix: Optional[str]
    """get or set the apiServicePrefix for the module
    
    for CDD modules the short name of the module is always "CDD", so
    this attribute is needed to define the prefix for the API services"""
    category: Optional[EcucModuleDefCategory]
    """get or set the category of the module definition"""
    def containers(
        self, /
    ) -> Iterator[Union[EcucChoiceContainerDef, EcucParamConfContainerDef]]:
        """iterate over all containers in the module"""
        ...

    def create_choice_container_def(self, name: str, /) -> EcucChoiceContainerDef:
        """create a new EcucChoiceContainerDef in the module"""
        ...

    def create_param_conf_container_def(
        self, name: str, /
    ) -> EcucParamConfContainerDef:
        """create a new EcucParamConfContainerDef in the module"""
        ...
    element: Element
    lower_multiplicity: Optional[int]
    """get or set the lower multiplicity attribute"""
    name: str
    post_build_variant_support: Optional[bool]
    """get or set the post build variant support attribute"""
    refined_module_def: Optional[EcucModuleDef]
    """get or set the reference to a refined standard module
    
    This reference is only used if the category is `VendorSpecificModuleDefinition`"""
    supported_config_variants: List[EcucConfigurationVariant]
    """set the supported configuration variants for the module"""
    upper_multiplicity: Optional[int]
    """get or set the upper multiplicity attribute"""
    upper_multiplicity_infinite: Optional[bool]
    """get or set the upper multiplicity infinite attribute
    
    if this attribute is set to true, the upper multiplicity is infinite
    (i.e. the module definition can be used an arbitrary number of times)
    When this attribute is true, the upper multiplicity attribute may not be used."""

@final
class EcucModuleDefCategory:
    """
    The `EcucModuleDefCategory` represents the possible category values for a module definition
    """

    StandardizedModuleDefinition: EcucModuleDefCategory
    VendorSpecificModuleDefinition: EcucModuleDefCategory

@final
class EcucMultilineStringParamDef:
    """
    `EcucMultilineStringParamDef` is used to specify a multiline string parameter in the ECU configuration
    """

    def __init__(self, element: Element, /) -> EcucMultilineStringParamDef: ...
    element: Element
    lower_multiplicity: Optional[int]
    """get or set the lower multiplicity attribute"""
    multiplicity_config_classes: List[
        Tuple[EcucConfigurationClass, EcucConfigurationVariant]
    ]
    """set the multiplicity config classes of the parameter definition.
    If an empty list is provided, the multiplicity config classes are removed.
    
    This setting is required if the containing EcucModuleDef has the category VENDOR_SPECIFIC_MODULE_DEFINITION."""
    name: str
    origin: Optional[str]
    """set the origin of the parameter definition
    
    The origin is a string that describes if the parameter was defined in the AUTOSAR standard or by a vendor.
    Standardized parameters use the origin "AUTOSAR_ECUC", while vendors are supposed to use string like 'VendorXyz_v1.3'"""
    post_build_variant_multiplicity: Optional[bool]
    """get or set the postBuildVariantMultiplicity attribute
    
    If postBuildVariantMultiplicity is true, then the parameter or reference
    may have a different number of instances in different post-build variants."""
    post_build_variant_value: Optional[bool]
    """get or set the postBuildVariantValue attribute
    
    If postBuildVariantValue is true, then the parameter or reference
    may have different values in different post-build variants."""
    requires_index: Optional[bool]
    """get or set the requiresIndex attribute"""
    upper_multiplicity: Optional[int]
    """get or set the upper multiplicity attribute"""
    upper_multiplicity_infinite: Optional[bool]
    """get or set the upper multiplicity infinite attribute
    
    if this attribute is set to true, the upper multiplicity is infinite
    (i.e. the module definition can be used an arbitrary number of times)
    When this attribute is true, the upper multiplicity attribute may not be used."""
    value_config_classes: List[Tuple[EcucConfigurationClass, EcucConfigurationVariant]]
    """set the value config classes of the parameter definition.
    
    If an empty list is provided, the value config classes are removed.
    According to the specification setting is required if the containing EcucModuleDef
    has the category VENDOR_SPECIFIC_MODULE_DEFINITION, but in practice it is rarely used."""
    with_auto: Optional[bool]
    """get or set the withAuto attribute
    
    If withAuto is true, then the parameter or reference is allowed to set its isAutoValue attribute to true."""
    max_length: Optional[int]
    """max length of the string parameter"""
    min_length: Optional[int]
    """min length of the string parameter"""
    regular_expression: Optional[str]
    """regular expression for the string parameter"""
    default_value: Optional[str]
    """default value of the string parameter"""

@final
class EcucNumericalParamValue:
    """
    The `EcucNumericalParamValue` holds a numerical value and can represent boolean, float or int parameter definitions.

    Internally this value is stored as a string; in additon to the value() function, there are also
    value_bool(), value_int() and value_float() functions, which parse the string and should be used as appropriate.
    """

    def __init__(self, element: Element, /) -> EcucNumericalParamValue: ...
    definition: Optional[
        Union[EcucBooleanParamDef, EcucFloatParamDef, EcucIntegerParamDef]
    ]
    """set the parameter definition reference"""
    definition_ref: Optional[str]
    """get the parameter definition reference as a string
    
    This function is an alternative to `definition()`; it is useful when the
    referenced definition is not loaded and can't be resolved."""
    element: Element
    index: Optional[int]
    """set the index of the parameter
    
    If the parameter definition has `requiresIndex` set to `true`, then the parameter
    must have an index. Otherwise the index is meaningless."""
    is_auto_value: Optional[bool]
    """set the isAutoValue flag
    
    If the parameter definition has `withAuto` set to `true`, then the parameter is allowed to have an auto value."""
    value: Optional[str]
    """get or set the numerical value as a string"""
    value_bool: Optional[bool]
    """get the numerical value as a boolean"""
    value_float: Optional[float]
    """get the numerical value as a float"""
    value_int: Optional[int]
    """get the numerical value as an integer"""

@final
class EcucParamConfContainerDef:
    """
    The `EcucParamConfContainerDef` is used to define configuration containers
    """

    def __init__(self, element: Element, /) -> EcucParamConfContainerDef: ...
    def create_add_info_param_def(
        self, name: str, origin: str, /
    ) -> EcucAddInfoParamDef:
        """create a new EcucAddInfoParamDef in the container"""
        ...

    def create_boolean_param_def(
        self, name: str, origin: str, /
    ) -> EcucBooleanParamDef:
        """create a new EcucBooleanParamDef in the container"""
        ...

    def create_choice_container_def(self, name: str, /) -> EcucChoiceContainerDef:
        """create a new `EcucChoiceContainerDef` as a sub-container"""
        ...

    def create_choice_reference_def(
        self, name: str, origin: str, /
    ) -> EcucChoiceReferenceDef:
        """create a new EcucChoiceReferenceDef in the container"""
        ...

    def create_enumeration_param_def(
        self, name: str, origin: str, /
    ) -> EcucEnumerationParamDef:
        """create a new EcucEnumerationParamDef in the container"""
        ...

    def create_float_param_def(self, name: str, origin: str, /) -> EcucFloatParamDef:
        """create a new EcucFloatParamDef in the container"""
        ...

    def create_foreign_reference_def(
        self, name: str, origin: str, /
    ) -> EcucForeignReferenceDef:
        """create a new EcucForeignReferenceDef in the container"""
        ...

    def create_function_name_param_def(
        self, name: str, origin: str, /
    ) -> EcucFunctionNameDef:
        """create a new EcucFunctionNameDef in the container"""
        ...

    def create_instance_reference_def(
        self, name: str, origin: str, /
    ) -> EcucInstanceReferenceDef:
        """create a new EcucInstanceReferenceDef in the container"""
        ...

    def create_integer_param_def(
        self, name: str, origin: str, /
    ) -> EcucIntegerParamDef:
        """create a new EcucIntegerParamDef in the container"""
        ...

    def create_linker_symbol_param_def(
        self, name: str, origin: str, /
    ) -> EcucLinkerSymbolDef:
        """create a new EcucLinkerSymbolDef in the container"""
        ...

    def create_multiline_string_param_def(
        self, name: str, origin: str, /
    ) -> EcucMultilineStringParamDef:
        """create a new EcucMultilineStringParamDef in the container"""
        ...

    def create_param_conf_container_def(
        self, name: str, /
    ) -> EcucParamConfContainerDef:
        """create a new `EcucParamConfContainerDef` as a sub-container"""
        ...

    def create_reference_def(self, name: str, origin: str, /) -> EcucReferenceDef:
        """create a new EcucReferenceDef in the container"""
        ...

    def create_string_param_def(self, name: str, origin: str, /) -> EcucStringParamDef:
        """create a new EcucStringParamDef in the container"""
        ...

    def create_uri_reference_def(
        self, name: str, origin: str, /
    ) -> EcucUriReferenceDef:
        """create a new EcucUriReferenceDef in the container"""
        ...
    element: Element
    lower_multiplicity: Optional[int]
    """get or set the lower multiplicity attribute"""
    name: str
    def parameters(self, /) -> Iterator[EcucParameterDef]:
        """get the parameters in the container"""
        ...

    def references(self, /) -> Iterator[EcucAnyReferenceDef]:
        """get the references in the container"""
        ...

    def sub_containers(self, /) -> Iterator[EcucContainerDef]:
        """iterate over the sub-containers"""
        ...
    upper_multiplicity: Optional[int]
    """get or set the upper multiplicity attribute"""
    upper_multiplicity_infinite: Optional[bool]
    """get or set the upper multiplicity infinite attribute
    
    if this attribute is set to true, the upper multiplicity is infinite
    (i.e. the module definition can be used an arbitrary number of times)
    When this attribute is true, the upper multiplicity attribute may not be used."""

@final
class EcucReferenceDef:
    """
    The `EcuReferenceDef` specifies references between parameters in the ECU configuration.
    """

    def __init__(self, element: Element, /) -> EcucReferenceDef: ...
    destination: Optional[EcucContainerDef]
    """destination container of the reference"""
    element: Element
    lower_multiplicity: Optional[int]
    """get or set the lower multiplicity attribute"""
    multiplicity_config_classes: List[
        Tuple[EcucConfigurationClass, EcucConfigurationVariant]
    ]
    """set the multiplicity config classes of the parameter definition.
    If an empty list is provided, the multiplicity config classes are removed.
    
    This setting is required if the containing EcucModuleDef has the category VENDOR_SPECIFIC_MODULE_DEFINITION."""
    name: str
    origin: Optional[str]
    """set the origin of the parameter definition
    
    The origin is a string that describes if the parameter was defined in the AUTOSAR standard or by a vendor.
    Standardized parameters use the origin "AUTOSAR_ECUC", while vendors are supposed to use string like 'VendorXyz_v1.3'"""
    post_build_variant_multiplicity: Optional[bool]
    """get or set the postBuildVariantMultiplicity attribute
    
    If postBuildVariantMultiplicity is true, then the parameter or reference
    may have a different number of instances in different post-build variants."""
    post_build_variant_value: Optional[bool]
    """get or set the postBuildVariantValue attribute
    
    If postBuildVariantValue is true, then the parameter or reference
    may have different values in different post-build variants."""
    requires_index: Optional[bool]
    """get or set the requiresIndex attribute"""
    upper_multiplicity: Optional[int]
    """get or set the upper multiplicity attribute"""
    upper_multiplicity_infinite: Optional[bool]
    """get or set the upper multiplicity infinite attribute
    
    if this attribute is set to true, the upper multiplicity is infinite
    (i.e. the module definition can be used an arbitrary number of times)
    When this attribute is true, the upper multiplicity attribute may not be used."""
    value_config_classes: List[Tuple[EcucConfigurationClass, EcucConfigurationVariant]]
    """set the value config classes of the parameter definition.
    
    If an empty list is provided, the value config classes are removed.
    According to the specification setting is required if the containing EcucModuleDef
    has the category VENDOR_SPECIFIC_MODULE_DEFINITION, but in practice it is rarely used."""
    with_auto: Optional[bool]
    """get or set the withAuto attribute
    
    If withAuto is true, then the parameter or reference is allowed to set its isAutoValue attribute to true."""

@final
class EcucReferenceValue:
    """
    An `EcucReferenceValue` allows the ecu tonfiguration to refer to any identifiable element in the Autosar model
    """

    def __init__(self, element: Element, /) -> EcucReferenceValue: ...
    definition: Optional[EcucReferenceDef]
    """set the parameter definition reference"""
    definition_ref: Optional[str]
    """get the referenced definition ref as a string
    
    This function is an alternative to `definition()`; it is useful when the
    referenced definition is not loaded and can't be resolved."""
    element: Element
    index: Optional[int]
    """set the index of the reference
    
    If the reference definition has `requiresIndex` set to `true`, then the reference
    must have an index. Otherwise the index is meaningless."""
    is_auto_value: Optional[bool]
    """set the isAutoValue flag
    
    If the reference definition has `withAuto` set to `true`, then the reference is allowed to have an auto value."""
    target: Optional[Element]
    """Set the target of the reference"""

@final
class EcucStringParamDef:
    """
    `EcucStringParamDef` is used to specify a string parameter in the ECU configuration
    """

    def __init__(self, element: Element, /) -> EcucStringParamDef: ...
    element: Element
    lower_multiplicity: Optional[int]
    """get or set the lower multiplicity attribute"""
    multiplicity_config_classes: List[
        Tuple[EcucConfigurationClass, EcucConfigurationVariant]
    ]
    """set the multiplicity config classes of the parameter definition.
    If an empty list is provided, the multiplicity config classes are removed.
    
    This setting is required if the containing EcucModuleDef has the category VENDOR_SPECIFIC_MODULE_DEFINITION."""
    name: str
    origin: Optional[str]
    """set the origin of the parameter definition
    
    The origin is a string that describes if the parameter was defined in the AUTOSAR standard or by a vendor.
    Standardized parameters use the origin "AUTOSAR_ECUC", while vendors are supposed to use string like 'VendorXyz_v1.3'"""
    post_build_variant_multiplicity: Optional[bool]
    """get or set the postBuildVariantMultiplicity attribute
    
    If postBuildVariantMultiplicity is true, then the parameter or reference
    may have a different number of instances in different post-build variants."""
    post_build_variant_value: Optional[bool]
    """get or set the postBuildVariantValue attribute
    
    If postBuildVariantValue is true, then the parameter or reference
    may have different values in different post-build variants."""
    requires_index: Optional[bool]
    """get or set the requiresIndex attribute"""
    upper_multiplicity: Optional[int]
    """get or set the upper multiplicity attribute"""
    upper_multiplicity_infinite: Optional[bool]
    """get or set the upper multiplicity infinite attribute
    
    if this attribute is set to true, the upper multiplicity is infinite
    (i.e. the module definition can be used an arbitrary number of times)
    When this attribute is true, the upper multiplicity attribute may not be used."""
    value_config_classes: List[Tuple[EcucConfigurationClass, EcucConfigurationVariant]]
    """set the value config classes of the parameter definition.
    
    If an empty list is provided, the value config classes are removed.
    According to the specification setting is required if the containing EcucModuleDef
    has the category VENDOR_SPECIFIC_MODULE_DEFINITION, but in practice it is rarely used."""
    with_auto: Optional[bool]
    """get or set the withAuto attribute
    
    If withAuto is true, then the parameter or reference is allowed to set its isAutoValue attribute to true."""
    max_length: Optional[int]
    """max length of the string parameter"""
    min_length: Optional[int]
    """min length of the string parameter"""
    regular_expression: Optional[str]
    """regular expression for the string parameter"""
    default_value: Optional[str]
    """default value of the string parameter"""

@final
class EcucTextualParamValue:
    """
    The `EcucTextualParamValue` holds a string value and can represent a enumeration,
     string, multi-line string, function name or linker symbol parameter definition.
    """

    def __init__(self, element: Element, /) -> EcucTextualParamValue: ...
    definition: Optional[EcucParameterDef]
    """set the parameter definition reference"""
    definition_ref: Optional[str]
    """get the parameter definition reference as a string
    
    This function is an alternative to `definition()`; it is useful when the
    referenced definition is not loaded and can't be resolved."""
    element: Element
    index: Optional[int]
    """set the index of the parameter
    
    If the parameter definition has `requiresIndex` set to `true`, then the parameter
    must have an index. Otherwise the index is meaningless."""
    is_auto_value: Optional[bool]
    """set the isAutoValue flag
    
    If the parameter definition has `withAuto` set to `true`, then the parameter is allowed to have an auto value."""
    value: str
    """the textual value of the parameter"""

@final
class EcucUriReferenceDef:
    """
    The `EcucUriReferenceDef` defines a reference with a destination that is specified via a destinationUri
    """

    def __init__(self, element: Element, /) -> EcucUriReferenceDef: ...
    destination_uri: Optional[EcucDestinationUriDef]
    """set the destination uri of the reference definition"""
    element: Element
    lower_multiplicity: Optional[int]
    """get or set the lower multiplicity attribute"""
    multiplicity_config_classes: List[
        Tuple[EcucConfigurationClass, EcucConfigurationVariant]
    ]
    """set the multiplicity config classes of the parameter definition.
    If an empty list is provided, the multiplicity config classes are removed.
    
    This setting is required if the containing EcucModuleDef has the category VENDOR_SPECIFIC_MODULE_DEFINITION."""
    name: str
    origin: Optional[str]
    """set the origin of the parameter definition
    
    The origin is a string that describes if the parameter was defined in the AUTOSAR standard or by a vendor.
    Standardized parameters use the origin "AUTOSAR_ECUC", while vendors are supposed to use string like 'VendorXyz_v1.3'"""
    post_build_variant_multiplicity: Optional[bool]
    """get or set the postBuildVariantMultiplicity attribute
    
    If postBuildVariantMultiplicity is true, then the parameter or reference
    may have a different number of instances in different post-build variants."""
    post_build_variant_value: Optional[bool]
    """get or set the postBuildVariantValue attribute
    
    If postBuildVariantValue is true, then the parameter or reference
    may have different values in different post-build variants."""
    requires_index: Optional[bool]
    """get or set the requiresIndex attribute"""
    upper_multiplicity: Optional[int]
    """get or set the upper multiplicity attribute"""
    upper_multiplicity_infinite: Optional[bool]
    """get or set the upper multiplicity infinite attribute
    
    if this attribute is set to true, the upper multiplicity is infinite
    (i.e. the module definition can be used an arbitrary number of times)
    When this attribute is true, the upper multiplicity attribute may not be used."""
    value_config_classes: List[Tuple[EcucConfigurationClass, EcucConfigurationVariant]]
    """set the value config classes of the parameter definition.
    
    If an empty list is provided, the value config classes are removed.
    According to the specification setting is required if the containing EcucModuleDef
    has the category VENDOR_SPECIFIC_MODULE_DEFINITION, but in practice it is rarely used."""
    with_auto: Optional[bool]
    """get or set the withAuto attribute
    
    If withAuto is true, then the parameter or reference is allowed to set its isAutoValue attribute to true."""

@final
class EcucValueCollection:
    """
    `EcucValueCollection` collects references to all the separate modules that form the ECU configuration
    """

    def __init__(self, element: Element, /) -> EcucValueCollection: ...
    def add_module_configuration(
        self, module_configuration: EcucModuleConfigurationValues
    ) -> None:
        """Add a reference to a module configuration to the collection"""
        ...
    ecu_extract_reference: Optional[System]
    """Set the ecu extract reference, which links a `System` to the ECU configuration"""
    element: Element
    def module_configurations(self, /) -> Iterator[EcucModuleConfigurationValues]:
        """Get the module configurations in the collection"""
        ...
    name: str
    ...
