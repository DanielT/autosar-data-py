use crate::{
    Element,
    abstraction::{
        AutosarAbstractionError, abstraction_err_to_pyerr,
        ecu_configuration::{EcucConfigurationClass, EcucConfigurationVariant},
    },
    iterator_wrapper,
};
use autosar_data_abstraction::{
    self, AbstractionElement, IdentifiableAbstractionElement,
    ecu_configuration::{EcucAbstractStringParamDef, EcucCommonAttributes, EcucDefinitionElement},
};
use pyo3::{IntoPyObjectExt, prelude::*};

//##################################################################

/// `EcucAddInfoParamDef` is used to specify the need for formated text in the ECU configuration value description
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._ecu_configuration"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct EcucAddInfoParamDef(
    pub(crate) autosar_data_abstraction::ecu_configuration::EcucAddInfoParamDef,
);

#[pymethods]
impl EcucAddInfoParamDef {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::ecu_configuration::EcucAddInfoParamDef::try_from(
            element.0.clone(),
        ) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    #[setter]
    fn set_name(&self, name: &str) -> PyResult<()> {
        self.0.set_name(name).map_err(abstraction_err_to_pyerr)
    }

    #[getter]
    fn name(&self) -> Option<String> {
        self.0.name()
    }

    #[getter]
    fn element(&self) -> Element {
        Element(self.0.element().clone())
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self.0)
    }

    // ------- EcucCommonAttributes -------

    /// set the multiplicity config classes of the parameter definition.
    /// If an empty list is provided, the multiplicity config classes are removed.
    ///
    /// This setting is required if the containing EcucModuleDef has the category VENDOR_SPECIFIC_MODULE_DEFINITION.
    #[setter]
    fn set_multiplicity_config_classes(
        &self,
        config: Vec<(EcucConfigurationClass, EcucConfigurationVariant)>,
    ) -> PyResult<()> {
        let config: Vec<_> = config
            .iter()
            .map(|(class, variant)| ((*class).into(), (*variant).into()))
            .collect();
        self.0
            .set_multiplicity_config_classes(&config)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the multiplicity config classes of the parameter definition
    #[getter]
    fn multiplicity_config_classes(
        &self,
    ) -> Vec<(EcucConfigurationClass, EcucConfigurationVariant)> {
        self.0
            .multiplicity_config_classes()
            .iter()
            .map(|(class, variant)| {
                (
                    EcucConfigurationClass::from(*class),
                    EcucConfigurationVariant::from(*variant),
                )
            })
            .collect()
    }

    /// set the origin of the parameter definition
    ///
    /// The origin is a string that describes if the parameter was defined in the AUTOSAR standard or by a vendor.
    /// Standardized parameters use the origin "AUTOSAR_ECUC", while vendors are supposed to use string like "VendorXyz_v1.3"
    #[setter]
    fn set_origin(&self, origin: &str) -> PyResult<()> {
        self.0.set_origin(origin).map_err(abstraction_err_to_pyerr)
    }

    /// get the origin of the parameter definition
    ///
    /// The origin is a string that describes if the parameter was defined in the AUTOSAR standard or by a vendor.
    /// Standardized parameters use the origin "AUTOSAR_ECUC", while vendors are supposed to use string like "VendorXyz_v1.3"
    #[getter]
    fn origin(&self) -> Option<String> {
        self.0.origin()
    }

    /// set or remove the postBuildVariantMultiplicity attribute
    ///
    /// If postBuildVariantMultiplicity is true, then the parameter or reference
    /// may have a different number of instances in different post-build variants.
    #[setter]
    fn set_post_build_variant_multiplicity(
        &self,
        post_build_variant_multiplicity: Option<bool>,
    ) -> PyResult<()> {
        self.0
            .set_post_build_variant_multiplicity(post_build_variant_multiplicity)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the postBuildVariantMultiplicity attribute
    ///
    /// If postBuildVariantMultiplicity is true, then the parameter or reference
    /// may have a different number of instances in different post-build variants.
    #[getter]
    fn post_build_variant_multiplicity(&self) -> Option<bool> {
        self.0.post_build_variant_multiplicity()
    }

    /// set or remove the postBuildVariantValue attribute
    ///
    /// If postBuildVariantValue is true, then the parameter or reference
    /// may have different values in different post-build variants.
    #[setter]
    fn set_post_build_variant_value(&self, post_build_variant_value: Option<bool>) -> PyResult<()> {
        self.0
            .set_post_build_variant_value(post_build_variant_value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the postBuildVariantValue attribute
    ///
    /// If postBuildVariantValue is true, then the parameter or reference
    /// may have different values in different post-build variants.
    #[getter]
    fn post_build_variant_value(&self) -> Option<bool> {
        self.0.post_build_variant_value()
    }

    /// set or remove the requiresIndex attribute
    #[setter]
    fn set_requires_index(&self, requires_index: Option<bool>) -> PyResult<()> {
        self.0
            .set_requires_index(requires_index)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the requiresIndex attribute
    #[getter]
    fn requires_index(&self) -> Option<bool> {
        self.0.requires_index()
    }

    /// set the value config classes of the parameter definition.
    ///
    /// If an empty list is provided, the value config classes are removed.
    /// According to the specification setting is required if the containing EcucModuleDef
    /// has the category VENDOR_SPECIFIC_MODULE_DEFINITION, but in practice it is rarely used.
    #[setter]
    fn set_value_config_classes(
        &self,
        config: Vec<(EcucConfigurationClass, EcucConfigurationVariant)>,
    ) -> PyResult<()> {
        let config: Vec<_> = config
            .iter()
            .map(|(class, variant)| ((*class).into(), (*variant).into()))
            .collect();
        self.0
            .set_value_config_classes(&config)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the value config classes of the parameter definition
    ///
    /// According to the specification setting is required if the containing EcucModuleDef
    /// has the category VENDOR_SPECIFIC_MODULE_DEFINITION, but in practice it is rarely used.
    #[getter]
    fn value_config_classes(&self) -> Vec<(EcucConfigurationClass, EcucConfigurationVariant)> {
        self.0
            .value_config_classes()
            .iter()
            .map(|(class, variant)| {
                (
                    EcucConfigurationClass::from(*class),
                    EcucConfigurationVariant::from(*variant),
                )
            })
            .collect()
    }

    /// set or remove the withAuto attribute
    ///
    /// If withAuto is true, then the parameter or reference is allowed to set its isAutoValue attribute to true.
    #[setter]
    fn set_with_auto(&self, with_auto: Option<bool>) -> PyResult<()> {
        self.0
            .set_with_auto(with_auto)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the withAuto attribute
    #[getter]
    fn with_auto(&self) -> Option<bool> {
        self.0.with_auto()
    }

    // ------- EcucDefinitionElement -------
    /// set or remove the lower multiplicity attribute
    #[setter]
    fn set_lower_multiplicity(&self, lower_multiplicity: Option<u32>) -> PyResult<()> {
        self.0
            .set_lower_multiplicity(lower_multiplicity)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the lower multiplicity attribute
    #[getter]
    fn lower_multiplicity(&self) -> Option<u32> {
        self.0.lower_multiplicity()
    }

    /// set or remove the upper multiplicity attribute
    #[setter]
    fn set_upper_multiplicity(&self, upper_multiplicity: Option<u32>) -> PyResult<()> {
        self.0
            .set_upper_multiplicity(upper_multiplicity)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the upper multiplicity attribute
    #[getter]
    fn upper_multiplicity(&self) -> Option<u32> {
        self.0.upper_multiplicity()
    }

    /// set or remove the upper multiplicity infinite attribute
    ///
    /// if this attribute is set to true, the upper multiplicity is infinite
    /// (i.e. the module definition can be used an arbitrary number of times)
    /// When this attribute is true, the upper multiplicity attribute may not be used.
    #[setter]
    fn set_upper_multiplicity_infinite(&self, infinite: Option<bool>) -> PyResult<()> {
        self.0
            .set_upper_multiplicity_infinite(infinite)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the upper multiplicity infinite attribute
    #[getter]
    fn upper_multiplicity_infinite(&self) -> Option<bool> {
        self.0.upper_multiplicity_infinite()
    }
}

//##################################################################

/// `EcucBooleanParamDef` is used to specify a boolean parameter in the ECU configuration
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._ecu_configuration"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct EcucBooleanParamDef(
    pub(crate) autosar_data_abstraction::ecu_configuration::EcucBooleanParamDef,
);

#[pymethods]
impl EcucBooleanParamDef {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::ecu_configuration::EcucBooleanParamDef::try_from(
            element.0.clone(),
        ) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    #[setter]
    fn set_name(&self, name: &str) -> PyResult<()> {
        self.0.set_name(name).map_err(abstraction_err_to_pyerr)
    }

    #[getter]
    fn name(&self) -> Option<String> {
        self.0.name()
    }

    #[getter]
    fn element(&self) -> Element {
        Element(self.0.element().clone())
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self.0)
    }

    /// set the default value of the boolean parameter
    #[setter]
    fn set_default_value(&self, default_value: Option<bool>) -> PyResult<()> {
        self.0
            .set_default_value(default_value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the default value of the boolean parameter
    #[getter]
    fn default_value(&self) -> Option<bool> {
        self.0.default_value()
    }

    // ------- EcucCommonAttributes -------

    /// set the multiplicity config classes of the parameter definition.
    /// If an empty list is provided, the multiplicity config classes are removed.
    ///
    /// This setting is required if the containing EcucModuleDef has the category VENDOR_SPECIFIC_MODULE_DEFINITION.
    #[setter]
    fn set_multiplicity_config_classes(
        &self,
        config: Vec<(EcucConfigurationClass, EcucConfigurationVariant)>,
    ) -> PyResult<()> {
        let config: Vec<_> = config
            .iter()
            .map(|(class, variant)| ((*class).into(), (*variant).into()))
            .collect();
        self.0
            .set_multiplicity_config_classes(&config)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the multiplicity config classes of the parameter definition
    #[getter]
    fn multiplicity_config_classes(
        &self,
    ) -> Vec<(EcucConfigurationClass, EcucConfigurationVariant)> {
        self.0
            .multiplicity_config_classes()
            .iter()
            .map(|(class, variant)| {
                (
                    EcucConfigurationClass::from(*class),
                    EcucConfigurationVariant::from(*variant),
                )
            })
            .collect()
    }

    /// set the origin of the parameter definition
    ///
    /// The origin is a string that describes if the parameter was defined in the AUTOSAR standard or by a vendor.
    /// Standardized parameters use the origin "AUTOSAR_ECUC", while vendors are supposed to use string like "VendorXyz_v1.3"
    #[setter]
    fn set_origin(&self, origin: &str) -> PyResult<()> {
        self.0.set_origin(origin).map_err(abstraction_err_to_pyerr)
    }

    /// get the origin of the parameter definition
    ///
    /// The origin is a string that describes if the parameter was defined in the AUTOSAR standard or by a vendor.
    /// Standardized parameters use the origin "AUTOSAR_ECUC", while vendors are supposed to use string like "VendorXyz_v1.3"
    #[getter]
    fn origin(&self) -> Option<String> {
        self.0.origin()
    }

    /// set or remove the postBuildVariantMultiplicity attribute
    ///
    /// If postBuildVariantMultiplicity is true, then the parameter or reference
    /// may have a different number of instances in different post-build variants.
    #[setter]
    fn set_post_build_variant_multiplicity(
        &self,
        post_build_variant_multiplicity: Option<bool>,
    ) -> PyResult<()> {
        self.0
            .set_post_build_variant_multiplicity(post_build_variant_multiplicity)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the postBuildVariantMultiplicity attribute
    ///
    /// If postBuildVariantMultiplicity is true, then the parameter or reference
    /// may have a different number of instances in different post-build variants.
    #[getter]
    fn post_build_variant_multiplicity(&self) -> Option<bool> {
        self.0.post_build_variant_multiplicity()
    }

    /// set or remove the postBuildVariantValue attribute
    ///
    /// If postBuildVariantValue is true, then the parameter or reference
    /// may have different values in different post-build variants.
    #[setter]
    fn set_post_build_variant_value(&self, post_build_variant_value: Option<bool>) -> PyResult<()> {
        self.0
            .set_post_build_variant_value(post_build_variant_value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the postBuildVariantValue attribute
    ///
    /// If postBuildVariantValue is true, then the parameter or reference
    /// may have different values in different post-build variants.
    #[getter]
    fn post_build_variant_value(&self) -> Option<bool> {
        self.0.post_build_variant_value()
    }

    /// set or remove the requiresIndex attribute
    #[setter]
    fn set_requires_index(&self, requires_index: Option<bool>) -> PyResult<()> {
        self.0
            .set_requires_index(requires_index)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the requiresIndex attribute
    #[getter]
    fn requires_index(&self) -> Option<bool> {
        self.0.requires_index()
    }

    /// set the value config classes of the parameter definition.
    ///
    /// If an empty list is provided, the value config classes are removed.
    /// According to the specification setting is required if the containing EcucModuleDef
    /// has the category VENDOR_SPECIFIC_MODULE_DEFINITION, but in practice it is rarely used.
    #[setter]
    fn set_value_config_classes(
        &self,
        config: Vec<(EcucConfigurationClass, EcucConfigurationVariant)>,
    ) -> PyResult<()> {
        let config: Vec<_> = config
            .iter()
            .map(|(class, variant)| ((*class).into(), (*variant).into()))
            .collect();
        self.0
            .set_value_config_classes(&config)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the value config classes of the parameter definition
    ///
    /// According to the specification setting is required if the containing EcucModuleDef
    /// has the category VENDOR_SPECIFIC_MODULE_DEFINITION, but in practice it is rarely used.
    #[getter]
    fn value_config_classes(&self) -> Vec<(EcucConfigurationClass, EcucConfigurationVariant)> {
        self.0
            .value_config_classes()
            .iter()
            .map(|(class, variant)| {
                (
                    EcucConfigurationClass::from(*class),
                    EcucConfigurationVariant::from(*variant),
                )
            })
            .collect()
    }

    /// set or remove the withAuto attribute
    ///
    /// If withAuto is true, then the parameter or reference is allowed to set its isAutoValue attribute to true.
    #[setter]
    fn set_with_auto(&self, with_auto: Option<bool>) -> PyResult<()> {
        self.0
            .set_with_auto(with_auto)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the withAuto attribute
    #[getter]
    fn with_auto(&self) -> Option<bool> {
        self.0.with_auto()
    }

    // ------- EcucDefinitionElement -------
    /// set or remove the lower multiplicity attribute
    #[setter]
    fn set_lower_multiplicity(&self, lower_multiplicity: Option<u32>) -> PyResult<()> {
        self.0
            .set_lower_multiplicity(lower_multiplicity)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the lower multiplicity attribute
    #[getter]
    fn lower_multiplicity(&self) -> Option<u32> {
        self.0.lower_multiplicity()
    }

    /// set or remove the upper multiplicity attribute
    #[setter]
    fn set_upper_multiplicity(&self, upper_multiplicity: Option<u32>) -> PyResult<()> {
        self.0
            .set_upper_multiplicity(upper_multiplicity)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the upper multiplicity attribute
    #[getter]
    fn upper_multiplicity(&self) -> Option<u32> {
        self.0.upper_multiplicity()
    }

    /// set or remove the upper multiplicity infinite attribute
    ///
    /// if this attribute is set to true, the upper multiplicity is infinite
    /// (i.e. the module definition can be used an arbitrary number of times)
    /// When this attribute is true, the upper multiplicity attribute may not be used.
    #[setter]
    fn set_upper_multiplicity_infinite(&self, infinite: Option<bool>) -> PyResult<()> {
        self.0
            .set_upper_multiplicity_infinite(infinite)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the upper multiplicity infinite attribute
    #[getter]
    fn upper_multiplicity_infinite(&self) -> Option<bool> {
        self.0.upper_multiplicity_infinite()
    }
}

//##################################################################

/// `EcucEnumerationParamDef` is used to specify an enumeration parameter in the ECU configuration
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._ecu_configuration"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct EcucEnumerationParamDef(
    pub(crate) autosar_data_abstraction::ecu_configuration::EcucEnumerationParamDef,
);

#[pymethods]
impl EcucEnumerationParamDef {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::ecu_configuration::EcucEnumerationParamDef::try_from(
            element.0.clone(),
        ) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    #[setter]
    fn set_name(&self, name: &str) -> PyResult<()> {
        self.0.set_name(name).map_err(abstraction_err_to_pyerr)
    }

    #[getter]
    fn name(&self) -> Option<String> {
        self.0.name()
    }

    #[getter]
    fn element(&self) -> Element {
        Element(self.0.element().clone())
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self.0)
    }

    /// create a new enumeration literal
    fn create_enumeration_literal(&self, name: &str) -> PyResult<EcucEnumerationLiteralDef> {
        match self.0.create_enumeration_literal(name) {
            Ok(value) => Ok(EcucEnumerationLiteralDef(value)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// iterate over all enumeration literals
    fn enumeration_literals(&self) -> EcucEnumerationLiteralDefIterator {
        EcucEnumerationLiteralDefIterator::new(
            self.0.enumeration_literals().map(EcucEnumerationLiteralDef),
        )
    }

    /// set the default value of the enumeration parameter
    ///
    /// Note: enumeration literals must be created first, since the default value must match one of the literals
    #[setter]
    fn set_default_value(&self, default_value: Option<&str>) -> PyResult<()> {
        self.0
            .set_default_value(default_value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the default value of the enumeration parameter
    #[getter]
    fn default_value(&self) -> Option<String> {
        self.0.default_value()
    }

    // ------- EcucCommonAttributes -------

    /// set the multiplicity config classes of the parameter definition.
    /// If an empty list is provided, the multiplicity config classes are removed.
    ///
    /// This setting is required if the containing EcucModuleDef has the category VENDOR_SPECIFIC_MODULE_DEFINITION.
    #[setter]
    fn set_multiplicity_config_classes(
        &self,
        config: Vec<(EcucConfigurationClass, EcucConfigurationVariant)>,
    ) -> PyResult<()> {
        let config: Vec<_> = config
            .iter()
            .map(|(class, variant)| ((*class).into(), (*variant).into()))
            .collect();
        self.0
            .set_multiplicity_config_classes(&config)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the multiplicity config classes of the parameter definition
    #[getter]
    fn multiplicity_config_classes(
        &self,
    ) -> Vec<(EcucConfigurationClass, EcucConfigurationVariant)> {
        self.0
            .multiplicity_config_classes()
            .iter()
            .map(|(class, variant)| {
                (
                    EcucConfigurationClass::from(*class),
                    EcucConfigurationVariant::from(*variant),
                )
            })
            .collect()
    }

    /// set the origin of the parameter definition
    ///
    /// The origin is a string that describes if the parameter was defined in the AUTOSAR standard or by a vendor.
    /// Standardized parameters use the origin "AUTOSAR_ECUC", while vendors are supposed to use string like "VendorXyz_v1.3"
    #[setter]
    fn set_origin(&self, origin: &str) -> PyResult<()> {
        self.0.set_origin(origin).map_err(abstraction_err_to_pyerr)
    }

    /// get the origin of the parameter definition
    ///
    /// The origin is a string that describes if the parameter was defined in the AUTOSAR standard or by a vendor.
    /// Standardized parameters use the origin "AUTOSAR_ECUC", while vendors are supposed to use string like "VendorXyz_v1.3"
    #[getter]
    fn origin(&self) -> Option<String> {
        self.0.origin()
    }

    /// set or remove the postBuildVariantMultiplicity attribute
    ///
    /// If postBuildVariantMultiplicity is true, then the parameter or reference
    /// may have a different number of instances in different post-build variants.
    #[setter]
    fn set_post_build_variant_multiplicity(
        &self,
        post_build_variant_multiplicity: Option<bool>,
    ) -> PyResult<()> {
        self.0
            .set_post_build_variant_multiplicity(post_build_variant_multiplicity)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the postBuildVariantMultiplicity attribute
    ///
    /// If postBuildVariantMultiplicity is true, then the parameter or reference
    /// may have a different number of instances in different post-build variants.
    #[getter]
    fn post_build_variant_multiplicity(&self) -> Option<bool> {
        self.0.post_build_variant_multiplicity()
    }

    /// set or remove the postBuildVariantValue attribute
    ///
    /// If postBuildVariantValue is true, then the parameter or reference
    /// may have different values in different post-build variants.
    #[setter]
    fn set_post_build_variant_value(&self, post_build_variant_value: Option<bool>) -> PyResult<()> {
        self.0
            .set_post_build_variant_value(post_build_variant_value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the postBuildVariantValue attribute
    ///
    /// If postBuildVariantValue is true, then the parameter or reference
    /// may have different values in different post-build variants.
    #[getter]
    fn post_build_variant_value(&self) -> Option<bool> {
        self.0.post_build_variant_value()
    }

    /// set or remove the requiresIndex attribute
    #[setter]
    fn set_requires_index(&self, requires_index: Option<bool>) -> PyResult<()> {
        self.0
            .set_requires_index(requires_index)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the requiresIndex attribute
    #[getter]
    fn requires_index(&self) -> Option<bool> {
        self.0.requires_index()
    }

    /// set the value config classes of the parameter definition.
    ///
    /// If an empty list is provided, the value config classes are removed.
    /// According to the specification setting is required if the containing EcucModuleDef
    /// has the category VENDOR_SPECIFIC_MODULE_DEFINITION, but in practice it is rarely used.
    #[setter]
    fn set_value_config_classes(
        &self,
        config: Vec<(EcucConfigurationClass, EcucConfigurationVariant)>,
    ) -> PyResult<()> {
        let config: Vec<_> = config
            .iter()
            .map(|(class, variant)| ((*class).into(), (*variant).into()))
            .collect();
        self.0
            .set_value_config_classes(&config)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the value config classes of the parameter definition
    ///
    /// According to the specification setting is required if the containing EcucModuleDef
    /// has the category VENDOR_SPECIFIC_MODULE_DEFINITION, but in practice it is rarely used.
    #[getter]
    fn value_config_classes(&self) -> Vec<(EcucConfigurationClass, EcucConfigurationVariant)> {
        self.0
            .value_config_classes()
            .iter()
            .map(|(class, variant)| {
                (
                    EcucConfigurationClass::from(*class),
                    EcucConfigurationVariant::from(*variant),
                )
            })
            .collect()
    }

    /// set or remove the withAuto attribute
    ///
    /// If withAuto is true, then the parameter or reference is allowed to set its isAutoValue attribute to true.
    #[setter]
    fn set_with_auto(&self, with_auto: Option<bool>) -> PyResult<()> {
        self.0
            .set_with_auto(with_auto)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the withAuto attribute
    #[getter]
    fn with_auto(&self) -> Option<bool> {
        self.0.with_auto()
    }

    // ------- EcucDefinitionElement -------
    /// set or remove the lower multiplicity attribute
    #[setter]
    fn set_lower_multiplicity(&self, lower_multiplicity: Option<u32>) -> PyResult<()> {
        self.0
            .set_lower_multiplicity(lower_multiplicity)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the lower multiplicity attribute
    #[getter]
    fn lower_multiplicity(&self) -> Option<u32> {
        self.0.lower_multiplicity()
    }

    /// set or remove the upper multiplicity attribute
    #[setter]
    fn set_upper_multiplicity(&self, upper_multiplicity: Option<u32>) -> PyResult<()> {
        self.0
            .set_upper_multiplicity(upper_multiplicity)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the upper multiplicity attribute
    #[getter]
    fn upper_multiplicity(&self) -> Option<u32> {
        self.0.upper_multiplicity()
    }

    /// set or remove the upper multiplicity infinite attribute
    ///
    /// if this attribute is set to true, the upper multiplicity is infinite
    /// (i.e. the module definition can be used an arbitrary number of times)
    /// When this attribute is true, the upper multiplicity attribute may not be used.
    #[setter]
    fn set_upper_multiplicity_infinite(&self, infinite: Option<bool>) -> PyResult<()> {
        self.0
            .set_upper_multiplicity_infinite(infinite)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the upper multiplicity infinite attribute
    #[getter]
    fn upper_multiplicity_infinite(&self) -> Option<bool> {
        self.0.upper_multiplicity_infinite()
    }
}

//##################################################################

/// `EcucEnumerationLiteralDef` is used to specify an enumeration literal in the ECU configuration
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._ecu_configuration"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct EcucEnumerationLiteralDef(
    pub(crate) autosar_data_abstraction::ecu_configuration::EcucEnumerationLiteralDef,
);

#[pymethods]
impl EcucEnumerationLiteralDef {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::ecu_configuration::EcucEnumerationLiteralDef::try_from(
            element.0.clone(),
        ) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    #[setter]
    fn set_name(&self, name: &str) -> PyResult<()> {
        self.0.set_name(name).map_err(abstraction_err_to_pyerr)
    }

    #[getter]
    fn name(&self) -> Option<String> {
        self.0.name()
    }

    #[getter]
    fn element(&self) -> Element {
        Element(self.0.element().clone())
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self.0)
    }
}

//##################################################################

iterator_wrapper!(EcucEnumerationLiteralDefIterator, EcucEnumerationLiteralDef);

//##################################################################

/// `EcucFloatParamDef` is used to specify a float parameter in the ECU configuration
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._ecu_configuration"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct EcucFloatParamDef(
    pub(crate) autosar_data_abstraction::ecu_configuration::EcucFloatParamDef,
);

#[pymethods]
impl EcucFloatParamDef {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::ecu_configuration::EcucFloatParamDef::try_from(
            element.0.clone(),
        ) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    #[setter]
    fn set_name(&self, name: &str) -> PyResult<()> {
        self.0.set_name(name).map_err(abstraction_err_to_pyerr)
    }

    #[getter]
    fn name(&self) -> Option<String> {
        self.0.name()
    }

    #[getter]
    fn element(&self) -> Element {
        Element(self.0.element().clone())
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self.0)
    }

    /// set the default value of the float parameter
    #[setter]
    fn set_default_value(&self, default_value: Option<f64>) -> PyResult<()> {
        self.0
            .set_default_value(default_value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the default value of the float parameter
    #[getter]
    fn default_value(&self) -> Option<f64> {
        self.0.default_value()
    }

    /// set the min value of the float parameter
    #[setter]
    fn set_min(&self, min: Option<f64>) -> PyResult<()> {
        self.0.set_min(min).map_err(abstraction_err_to_pyerr)
    }

    /// get the min value of the float parameter
    #[getter]
    fn min(&self) -> Option<f64> {
        self.0.min()
    }

    /// set the max value of the float parameter
    #[setter]
    fn set_max(&self, max: Option<f64>) -> PyResult<()> {
        self.0.set_max(max).map_err(abstraction_err_to_pyerr)
    }

    /// get the max value of the float parameter
    #[getter]
    fn max(&self) -> Option<f64> {
        self.0.max()
    }

    // ------- EcucCommonAttributes -------

    /// set the multiplicity config classes of the parameter definition.
    /// If an empty list is provided, the multiplicity config classes are removed.
    ///
    /// This setting is required if the containing EcucModuleDef has the category VENDOR_SPECIFIC_MODULE_DEFINITION.
    #[setter]
    fn set_multiplicity_config_classes(
        &self,
        config: Vec<(EcucConfigurationClass, EcucConfigurationVariant)>,
    ) -> PyResult<()> {
        let config: Vec<_> = config
            .iter()
            .map(|(class, variant)| ((*class).into(), (*variant).into()))
            .collect();
        self.0
            .set_multiplicity_config_classes(&config)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the multiplicity config classes of the parameter definition
    #[getter]
    fn multiplicity_config_classes(
        &self,
    ) -> Vec<(EcucConfigurationClass, EcucConfigurationVariant)> {
        self.0
            .multiplicity_config_classes()
            .iter()
            .map(|(class, variant)| {
                (
                    EcucConfigurationClass::from(*class),
                    EcucConfigurationVariant::from(*variant),
                )
            })
            .collect()
    }

    /// set the origin of the parameter definition
    ///
    /// The origin is a string that describes if the parameter was defined in the AUTOSAR standard or by a vendor.
    /// Standardized parameters use the origin "AUTOSAR_ECUC", while vendors are supposed to use string like "VendorXyz_v1.3"
    #[setter]
    fn set_origin(&self, origin: &str) -> PyResult<()> {
        self.0.set_origin(origin).map_err(abstraction_err_to_pyerr)
    }

    /// get the origin of the parameter definition
    ///
    /// The origin is a string that describes if the parameter was defined in the AUTOSAR standard or by a vendor.
    /// Standardized parameters use the origin "AUTOSAR_ECUC", while vendors are supposed to use string like "VendorXyz_v1.3"
    #[getter]
    fn origin(&self) -> Option<String> {
        self.0.origin()
    }

    /// set or remove the postBuildVariantMultiplicity attribute
    ///
    /// If postBuildVariantMultiplicity is true, then the parameter or reference
    /// may have a different number of instances in different post-build variants.
    #[setter]
    fn set_post_build_variant_multiplicity(
        &self,
        post_build_variant_multiplicity: Option<bool>,
    ) -> PyResult<()> {
        self.0
            .set_post_build_variant_multiplicity(post_build_variant_multiplicity)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the postBuildVariantMultiplicity attribute
    ///
    /// If postBuildVariantMultiplicity is true, then the parameter or reference
    /// may have a different number of instances in different post-build variants.
    #[getter]
    fn post_build_variant_multiplicity(&self) -> Option<bool> {
        self.0.post_build_variant_multiplicity()
    }

    /// set or remove the postBuildVariantValue attribute
    ///
    /// If postBuildVariantValue is true, then the parameter or reference
    /// may have different values in different post-build variants.
    #[setter]
    fn set_post_build_variant_value(&self, post_build_variant_value: Option<bool>) -> PyResult<()> {
        self.0
            .set_post_build_variant_value(post_build_variant_value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the postBuildVariantValue attribute
    ///
    /// If postBuildVariantValue is true, then the parameter or reference
    /// may have different values in different post-build variants.
    #[getter]
    fn post_build_variant_value(&self) -> Option<bool> {
        self.0.post_build_variant_value()
    }

    /// set or remove the requiresIndex attribute
    #[setter]
    fn set_requires_index(&self, requires_index: Option<bool>) -> PyResult<()> {
        self.0
            .set_requires_index(requires_index)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the requiresIndex attribute
    #[getter]
    fn requires_index(&self) -> Option<bool> {
        self.0.requires_index()
    }

    /// set the value config classes of the parameter definition.
    ///
    /// If an empty list is provided, the value config classes are removed.
    /// According to the specification setting is required if the containing EcucModuleDef
    /// has the category VENDOR_SPECIFIC_MODULE_DEFINITION, but in practice it is rarely used.
    #[setter]
    fn set_value_config_classes(
        &self,
        config: Vec<(EcucConfigurationClass, EcucConfigurationVariant)>,
    ) -> PyResult<()> {
        let config: Vec<_> = config
            .iter()
            .map(|(class, variant)| ((*class).into(), (*variant).into()))
            .collect();
        self.0
            .set_value_config_classes(&config)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the value config classes of the parameter definition
    ///
    /// According to the specification setting is required if the containing EcucModuleDef
    /// has the category VENDOR_SPECIFIC_MODULE_DEFINITION, but in practice it is rarely used.
    #[getter]
    fn value_config_classes(&self) -> Vec<(EcucConfigurationClass, EcucConfigurationVariant)> {
        self.0
            .value_config_classes()
            .iter()
            .map(|(class, variant)| {
                (
                    EcucConfigurationClass::from(*class),
                    EcucConfigurationVariant::from(*variant),
                )
            })
            .collect()
    }

    /// set or remove the withAuto attribute
    ///
    /// If withAuto is true, then the parameter or reference is allowed to set its isAutoValue attribute to true.
    #[setter]
    fn set_with_auto(&self, with_auto: Option<bool>) -> PyResult<()> {
        self.0
            .set_with_auto(with_auto)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the withAuto attribute
    #[getter]
    fn with_auto(&self) -> Option<bool> {
        self.0.with_auto()
    }

    // ------- EcucDefinitionElement -------
    /// set or remove the lower multiplicity attribute
    #[setter]
    fn set_lower_multiplicity(&self, lower_multiplicity: Option<u32>) -> PyResult<()> {
        self.0
            .set_lower_multiplicity(lower_multiplicity)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the lower multiplicity attribute
    #[getter]
    fn lower_multiplicity(&self) -> Option<u32> {
        self.0.lower_multiplicity()
    }

    /// set or remove the upper multiplicity attribute
    #[setter]
    fn set_upper_multiplicity(&self, upper_multiplicity: Option<u32>) -> PyResult<()> {
        self.0
            .set_upper_multiplicity(upper_multiplicity)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the upper multiplicity attribute
    #[getter]
    fn upper_multiplicity(&self) -> Option<u32> {
        self.0.upper_multiplicity()
    }

    /// set or remove the upper multiplicity infinite attribute
    ///
    /// if this attribute is set to true, the upper multiplicity is infinite
    /// (i.e. the module definition can be used an arbitrary number of times)
    /// When this attribute is true, the upper multiplicity attribute may not be used.
    #[setter]
    fn set_upper_multiplicity_infinite(&self, infinite: Option<bool>) -> PyResult<()> {
        self.0
            .set_upper_multiplicity_infinite(infinite)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the upper multiplicity infinite attribute
    #[getter]
    fn upper_multiplicity_infinite(&self) -> Option<bool> {
        self.0.upper_multiplicity_infinite()
    }
}

//##################################################################

/// `EcucIntegerParamDef` is used to specify an integer parameter in the ECU configuration
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._ecu_configuration"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct EcucIntegerParamDef(
    pub(crate) autosar_data_abstraction::ecu_configuration::EcucIntegerParamDef,
);

#[pymethods]
impl EcucIntegerParamDef {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::ecu_configuration::EcucIntegerParamDef::try_from(
            element.0.clone(),
        ) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    #[setter]
    fn set_name(&self, name: &str) -> PyResult<()> {
        self.0.set_name(name).map_err(abstraction_err_to_pyerr)
    }

    #[getter]
    fn name(&self) -> Option<String> {
        self.0.name()
    }

    #[getter]
    fn element(&self) -> Element {
        Element(self.0.element().clone())
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self.0)
    }

    /// set the default value of the integer parameter
    #[setter]
    fn set_default_value(&self, default_value: Option<i64>) -> PyResult<()> {
        self.0
            .set_default_value(default_value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the default value of the integer parameter
    #[getter]
    fn default_value(&self) -> Option<i64> {
        self.0.default_value()
    }

    /// set the min value of the integer parameter
    #[setter]
    fn set_min(&self, min: Option<i64>) -> PyResult<()> {
        self.0.set_min(min).map_err(abstraction_err_to_pyerr)
    }

    /// get the min value of the integer parameter
    #[getter]
    fn min(&self) -> Option<i64> {
        self.0.min()
    }

    /// set the max value of the integer parameter
    #[setter]
    fn set_max(&self, max: Option<i64>) -> PyResult<()> {
        self.0.set_max(max).map_err(abstraction_err_to_pyerr)
    }

    /// get the max value of the integer parameter
    #[getter]
    fn max(&self) -> Option<i64> {
        self.0.max()
    }

    // ------- EcucCommonAttributes -------

    /// set the multiplicity config classes of the parameter definition.
    /// If an empty list is provided, the multiplicity config classes are removed.
    ///
    /// This setting is required if the containing EcucModuleDef has the category VENDOR_SPECIFIC_MODULE_DEFINITION.
    #[setter]
    fn set_multiplicity_config_classes(
        &self,
        config: Vec<(EcucConfigurationClass, EcucConfigurationVariant)>,
    ) -> PyResult<()> {
        let config: Vec<_> = config
            .iter()
            .map(|(class, variant)| ((*class).into(), (*variant).into()))
            .collect();
        self.0
            .set_multiplicity_config_classes(&config)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the multiplicity config classes of the parameter definition
    #[getter]
    fn multiplicity_config_classes(
        &self,
    ) -> Vec<(EcucConfigurationClass, EcucConfigurationVariant)> {
        self.0
            .multiplicity_config_classes()
            .iter()
            .map(|(class, variant)| {
                (
                    EcucConfigurationClass::from(*class),
                    EcucConfigurationVariant::from(*variant),
                )
            })
            .collect()
    }

    /// set the origin of the parameter definition
    ///
    /// The origin is a string that describes if the parameter was defined in the AUTOSAR standard or by a vendor.
    /// Standardized parameters use the origin "AUTOSAR_ECUC", while vendors are supposed to use string like "VendorXyz_v1.3"
    #[setter]
    fn set_origin(&self, origin: &str) -> PyResult<()> {
        self.0.set_origin(origin).map_err(abstraction_err_to_pyerr)
    }

    /// get the origin of the parameter definition
    ///
    /// The origin is a string that describes if the parameter was defined in the AUTOSAR standard or by a vendor.
    /// Standardized parameters use the origin "AUTOSAR_ECUC", while vendors are supposed to use string like "VendorXyz_v1.3"
    #[getter]
    fn origin(&self) -> Option<String> {
        self.0.origin()
    }

    /// set or remove the postBuildVariantMultiplicity attribute
    ///
    /// If postBuildVariantMultiplicity is true, then the parameter or reference
    /// may have a different number of instances in different post-build variants.
    #[setter]
    fn set_post_build_variant_multiplicity(
        &self,
        post_build_variant_multiplicity: Option<bool>,
    ) -> PyResult<()> {
        self.0
            .set_post_build_variant_multiplicity(post_build_variant_multiplicity)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the postBuildVariantMultiplicity attribute
    ///
    /// If postBuildVariantMultiplicity is true, then the parameter or reference
    /// may have a different number of instances in different post-build variants.
    #[getter]
    fn post_build_variant_multiplicity(&self) -> Option<bool> {
        self.0.post_build_variant_multiplicity()
    }

    /// set or remove the postBuildVariantValue attribute
    ///
    /// If postBuildVariantValue is true, then the parameter or reference
    /// may have different values in different post-build variants.
    #[setter]
    fn set_post_build_variant_value(&self, post_build_variant_value: Option<bool>) -> PyResult<()> {
        self.0
            .set_post_build_variant_value(post_build_variant_value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the postBuildVariantValue attribute
    ///
    /// If postBuildVariantValue is true, then the parameter or reference
    /// may have different values in different post-build variants.
    #[getter]
    fn post_build_variant_value(&self) -> Option<bool> {
        self.0.post_build_variant_value()
    }

    /// set or remove the requiresIndex attribute
    #[setter]
    fn set_requires_index(&self, requires_index: Option<bool>) -> PyResult<()> {
        self.0
            .set_requires_index(requires_index)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the requiresIndex attribute
    #[getter]
    fn requires_index(&self) -> Option<bool> {
        self.0.requires_index()
    }

    /// set the value config classes of the parameter definition.
    ///
    /// If an empty list is provided, the value config classes are removed.
    /// According to the specification setting is required if the containing EcucModuleDef
    /// has the category VENDOR_SPECIFIC_MODULE_DEFINITION, but in practice it is rarely used.
    #[setter]
    fn set_value_config_classes(
        &self,
        config: Vec<(EcucConfigurationClass, EcucConfigurationVariant)>,
    ) -> PyResult<()> {
        let config: Vec<_> = config
            .iter()
            .map(|(class, variant)| ((*class).into(), (*variant).into()))
            .collect();
        self.0
            .set_value_config_classes(&config)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the value config classes of the parameter definition
    ///
    /// According to the specification setting is required if the containing EcucModuleDef
    /// has the category VENDOR_SPECIFIC_MODULE_DEFINITION, but in practice it is rarely used.
    #[getter]
    fn value_config_classes(&self) -> Vec<(EcucConfigurationClass, EcucConfigurationVariant)> {
        self.0
            .value_config_classes()
            .iter()
            .map(|(class, variant)| {
                (
                    EcucConfigurationClass::from(*class),
                    EcucConfigurationVariant::from(*variant),
                )
            })
            .collect()
    }

    /// set or remove the withAuto attribute
    ///
    /// If withAuto is true, then the parameter or reference is allowed to set its isAutoValue attribute to true.
    #[setter]
    fn set_with_auto(&self, with_auto: Option<bool>) -> PyResult<()> {
        self.0
            .set_with_auto(with_auto)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the withAuto attribute
    #[getter]
    fn with_auto(&self) -> Option<bool> {
        self.0.with_auto()
    }

    // ------- EcucDefinitionElement -------
    /// set or remove the lower multiplicity attribute
    #[setter]
    fn set_lower_multiplicity(&self, lower_multiplicity: Option<u32>) -> PyResult<()> {
        self.0
            .set_lower_multiplicity(lower_multiplicity)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the lower multiplicity attribute
    #[getter]
    fn lower_multiplicity(&self) -> Option<u32> {
        self.0.lower_multiplicity()
    }

    /// set or remove the upper multiplicity attribute
    #[setter]
    fn set_upper_multiplicity(&self, upper_multiplicity: Option<u32>) -> PyResult<()> {
        self.0
            .set_upper_multiplicity(upper_multiplicity)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the upper multiplicity attribute
    #[getter]
    fn upper_multiplicity(&self) -> Option<u32> {
        self.0.upper_multiplicity()
    }

    /// set or remove the upper multiplicity infinite attribute
    ///
    /// if this attribute is set to true, the upper multiplicity is infinite
    /// (i.e. the module definition can be used an arbitrary number of times)
    /// When this attribute is true, the upper multiplicity attribute may not be used.
    #[setter]
    fn set_upper_multiplicity_infinite(&self, infinite: Option<bool>) -> PyResult<()> {
        self.0
            .set_upper_multiplicity_infinite(infinite)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the upper multiplicity infinite attribute
    #[getter]
    fn upper_multiplicity_infinite(&self) -> Option<bool> {
        self.0.upper_multiplicity_infinite()
    }
}

//##################################################################

/// `EcucFunctionNameDef` is used to specify a function name parameter in the ECU configuration
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._ecu_configuration"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct EcucFunctionNameDef(
    pub(crate) autosar_data_abstraction::ecu_configuration::EcucFunctionNameDef,
);

#[pymethods]
impl EcucFunctionNameDef {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::ecu_configuration::EcucFunctionNameDef::try_from(
            element.0.clone(),
        ) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    #[setter]
    fn set_name(&self, name: &str) -> PyResult<()> {
        self.0.set_name(name).map_err(abstraction_err_to_pyerr)
    }

    #[getter]
    fn name(&self) -> Option<String> {
        self.0.name()
    }

    #[getter]
    fn element(&self) -> Element {
        Element(self.0.element().clone())
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self.0)
    }

    // ------- EcucAbstractStringParamDef -------

    /// set or remove the max length attribute
    #[setter]
    fn set_max_length(&self, max_length: Option<u32>) -> PyResult<()> {
        self.0
            .set_max_length(max_length)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the max length attribute
    #[getter]
    fn max_length(&self) -> Option<u32> {
        self.0.max_length()
    }

    /// set or remove the min length attribute
    #[setter]
    fn set_min_length(&self, min_length: Option<u32>) -> PyResult<()> {
        self.0
            .set_min_length(min_length)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the min length attribute
    #[getter]
    fn min_length(&self) -> Option<u32> {
        self.0.min_length()
    }

    /// set or remove the regular expression attribute
    /// The regular expression is a string that is used to validate the string parameter
    #[setter]
    fn set_regular_expression(&self, regular_expression: Option<&str>) -> PyResult<()> {
        self.0
            .set_regular_expression(regular_expression)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the regular expression attribute
    /// The regular expression is a string that is used to validate the string parameter
    #[getter]
    fn regular_expression(&self) -> Option<String> {
        self.0.regular_expression()
    }

    /// set or remove the default value attribute
    #[setter]
    fn set_default_value(&self, default_value: Option<&str>) -> PyResult<()> {
        self.0
            .set_default_value(default_value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the default value attribute
    #[getter]
    fn default_value(&self) -> Option<String> {
        self.0.default_value()
    }

    // ------- EcucCommonAttributes -------

    /// set the multiplicity config classes of the parameter definition.
    /// If an empty list is provided, the multiplicity config classes are removed.
    ///
    /// This setting is required if the containing EcucModuleDef has the category VENDOR_SPECIFIC_MODULE_DEFINITION.
    #[setter]
    fn set_multiplicity_config_classes(
        &self,
        config: Vec<(EcucConfigurationClass, EcucConfigurationVariant)>,
    ) -> PyResult<()> {
        let config: Vec<_> = config
            .iter()
            .map(|(class, variant)| ((*class).into(), (*variant).into()))
            .collect();
        self.0
            .set_multiplicity_config_classes(&config)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the multiplicity config classes of the parameter definition
    #[getter]
    fn multiplicity_config_classes(
        &self,
    ) -> Vec<(EcucConfigurationClass, EcucConfigurationVariant)> {
        self.0
            .multiplicity_config_classes()
            .iter()
            .map(|(class, variant)| {
                (
                    EcucConfigurationClass::from(*class),
                    EcucConfigurationVariant::from(*variant),
                )
            })
            .collect()
    }

    /// set the origin of the parameter definition
    ///
    /// The origin is a string that describes if the parameter was defined in the AUTOSAR standard or by a vendor.
    /// Standardized parameters use the origin "AUTOSAR_ECUC", while vendors are supposed to use string like "VendorXyz_v1.3"
    #[setter]
    fn set_origin(&self, origin: &str) -> PyResult<()> {
        self.0.set_origin(origin).map_err(abstraction_err_to_pyerr)
    }

    /// get the origin of the parameter definition
    ///
    /// The origin is a string that describes if the parameter was defined in the AUTOSAR standard or by a vendor.
    /// Standardized parameters use the origin "AUTOSAR_ECUC", while vendors are supposed to use string like "VendorXyz_v1.3"
    #[getter]
    fn origin(&self) -> Option<String> {
        self.0.origin()
    }

    /// set or remove the postBuildVariantMultiplicity attribute
    ///
    /// If postBuildVariantMultiplicity is true, then the parameter or reference
    /// may have a different number of instances in different post-build variants.
    #[setter]
    fn set_post_build_variant_multiplicity(
        &self,
        post_build_variant_multiplicity: Option<bool>,
    ) -> PyResult<()> {
        self.0
            .set_post_build_variant_multiplicity(post_build_variant_multiplicity)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the postBuildVariantMultiplicity attribute
    ///
    /// If postBuildVariantMultiplicity is true, then the parameter or reference
    /// may have a different number of instances in different post-build variants.
    #[getter]
    fn post_build_variant_multiplicity(&self) -> Option<bool> {
        self.0.post_build_variant_multiplicity()
    }

    /// set or remove the postBuildVariantValue attribute
    ///
    /// If postBuildVariantValue is true, then the parameter or reference
    /// may have different values in different post-build variants.
    #[setter]
    fn set_post_build_variant_value(&self, post_build_variant_value: Option<bool>) -> PyResult<()> {
        self.0
            .set_post_build_variant_value(post_build_variant_value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the postBuildVariantValue attribute
    ///
    /// If postBuildVariantValue is true, then the parameter or reference
    /// may have different values in different post-build variants.
    #[getter]
    fn post_build_variant_value(&self) -> Option<bool> {
        self.0.post_build_variant_value()
    }

    /// set or remove the requiresIndex attribute
    #[setter]
    fn set_requires_index(&self, requires_index: Option<bool>) -> PyResult<()> {
        self.0
            .set_requires_index(requires_index)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the requiresIndex attribute
    #[getter]
    fn requires_index(&self) -> Option<bool> {
        self.0.requires_index()
    }

    /// set the value config classes of the parameter definition.
    ///
    /// If an empty list is provided, the value config classes are removed.
    /// According to the specification setting is required if the containing EcucModuleDef
    /// has the category VENDOR_SPECIFIC_MODULE_DEFINITION, but in practice it is rarely used.
    #[setter]
    fn set_value_config_classes(
        &self,
        config: Vec<(EcucConfigurationClass, EcucConfigurationVariant)>,
    ) -> PyResult<()> {
        let config: Vec<_> = config
            .iter()
            .map(|(class, variant)| ((*class).into(), (*variant).into()))
            .collect();
        self.0
            .set_value_config_classes(&config)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the value config classes of the parameter definition
    ///
    /// According to the specification setting is required if the containing EcucModuleDef
    /// has the category VENDOR_SPECIFIC_MODULE_DEFINITION, but in practice it is rarely used.
    #[getter]
    fn value_config_classes(&self) -> Vec<(EcucConfigurationClass, EcucConfigurationVariant)> {
        self.0
            .value_config_classes()
            .iter()
            .map(|(class, variant)| {
                (
                    EcucConfigurationClass::from(*class),
                    EcucConfigurationVariant::from(*variant),
                )
            })
            .collect()
    }

    /// set or remove the withAuto attribute
    ///
    /// If withAuto is true, then the parameter or reference is allowed to set its isAutoValue attribute to true.
    #[setter]
    fn set_with_auto(&self, with_auto: Option<bool>) -> PyResult<()> {
        self.0
            .set_with_auto(with_auto)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the withAuto attribute
    #[getter]
    fn with_auto(&self) -> Option<bool> {
        self.0.with_auto()
    }

    // ------- EcucDefinitionElement -------
    /// set or remove the lower multiplicity attribute
    #[setter]
    fn set_lower_multiplicity(&self, lower_multiplicity: Option<u32>) -> PyResult<()> {
        self.0
            .set_lower_multiplicity(lower_multiplicity)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the lower multiplicity attribute
    #[getter]
    fn lower_multiplicity(&self) -> Option<u32> {
        self.0.lower_multiplicity()
    }

    /// set or remove the upper multiplicity attribute
    #[setter]
    fn set_upper_multiplicity(&self, upper_multiplicity: Option<u32>) -> PyResult<()> {
        self.0
            .set_upper_multiplicity(upper_multiplicity)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the upper multiplicity attribute
    #[getter]
    fn upper_multiplicity(&self) -> Option<u32> {
        self.0.upper_multiplicity()
    }

    /// set or remove the upper multiplicity infinite attribute
    ///
    /// if this attribute is set to true, the upper multiplicity is infinite
    /// (i.e. the module definition can be used an arbitrary number of times)
    /// When this attribute is true, the upper multiplicity attribute may not be used.
    #[setter]
    fn set_upper_multiplicity_infinite(&self, infinite: Option<bool>) -> PyResult<()> {
        self.0
            .set_upper_multiplicity_infinite(infinite)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the upper multiplicity infinite attribute
    #[getter]
    fn upper_multiplicity_infinite(&self) -> Option<bool> {
        self.0.upper_multiplicity_infinite()
    }
}

//##################################################################

/// `EcucLinkerSymbolDef` is used to specify a linker symbol parameter in the ECU configuration
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._ecu_configuration"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct EcucLinkerSymbolDef(
    pub(crate) autosar_data_abstraction::ecu_configuration::EcucLinkerSymbolDef,
);

#[pymethods]
impl EcucLinkerSymbolDef {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::ecu_configuration::EcucLinkerSymbolDef::try_from(
            element.0.clone(),
        ) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    #[setter]
    fn set_name(&self, name: &str) -> PyResult<()> {
        self.0.set_name(name).map_err(abstraction_err_to_pyerr)
    }

    #[getter]
    fn name(&self) -> Option<String> {
        self.0.name()
    }

    #[getter]
    fn element(&self) -> Element {
        Element(self.0.element().clone())
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self.0)
    }

    // ------- EcucAbstractStringParamDef -------

    /// set or remove the max length attribute
    #[setter]
    fn set_max_length(&self, max_length: Option<u32>) -> PyResult<()> {
        self.0
            .set_max_length(max_length)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the max length attribute
    #[getter]
    fn max_length(&self) -> Option<u32> {
        self.0.max_length()
    }

    /// set or remove the min length attribute
    #[setter]
    fn set_min_length(&self, min_length: Option<u32>) -> PyResult<()> {
        self.0
            .set_min_length(min_length)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the min length attribute
    #[getter]
    fn min_length(&self) -> Option<u32> {
        self.0.min_length()
    }

    /// set or remove the regular expression attribute
    /// The regular expression is a string that is used to validate the string parameter
    #[setter]
    fn set_regular_expression(&self, regular_expression: Option<&str>) -> PyResult<()> {
        self.0
            .set_regular_expression(regular_expression)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the regular expression attribute
    /// The regular expression is a string that is used to validate the string parameter
    #[getter]
    fn regular_expression(&self) -> Option<String> {
        self.0.regular_expression()
    }

    /// set or remove the default value attribute
    #[setter]
    fn set_default_value(&self, default_value: Option<&str>) -> PyResult<()> {
        self.0
            .set_default_value(default_value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the default value attribute
    #[getter]
    fn default_value(&self) -> Option<String> {
        self.0.default_value()
    }

    // ------- EcucCommonAttributes -------

    /// set the multiplicity config classes of the parameter definition.
    /// If an empty list is provided, the multiplicity config classes are removed.
    ///
    /// This setting is required if the containing EcucModuleDef has the category VENDOR_SPECIFIC_MODULE_DEFINITION.
    #[setter]
    fn set_multiplicity_config_classes(
        &self,
        config: Vec<(EcucConfigurationClass, EcucConfigurationVariant)>,
    ) -> PyResult<()> {
        let config: Vec<_> = config
            .iter()
            .map(|(class, variant)| ((*class).into(), (*variant).into()))
            .collect();
        self.0
            .set_multiplicity_config_classes(&config)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the multiplicity config classes of the parameter definition
    #[getter]
    fn multiplicity_config_classes(
        &self,
    ) -> Vec<(EcucConfigurationClass, EcucConfigurationVariant)> {
        self.0
            .multiplicity_config_classes()
            .iter()
            .map(|(class, variant)| {
                (
                    EcucConfigurationClass::from(*class),
                    EcucConfigurationVariant::from(*variant),
                )
            })
            .collect()
    }

    /// set the origin of the parameter definition
    ///
    /// The origin is a string that describes if the parameter was defined in the AUTOSAR standard or by a vendor.
    /// Standardized parameters use the origin "AUTOSAR_ECUC", while vendors are supposed to use string like "VendorXyz_v1.3"
    #[setter]
    fn set_origin(&self, origin: &str) -> PyResult<()> {
        self.0.set_origin(origin).map_err(abstraction_err_to_pyerr)
    }

    /// get the origin of the parameter definition
    ///
    /// The origin is a string that describes if the parameter was defined in the AUTOSAR standard or by a vendor.
    /// Standardized parameters use the origin "AUTOSAR_ECUC", while vendors are supposed to use string like "VendorXyz_v1.3"
    #[getter]
    fn origin(&self) -> Option<String> {
        self.0.origin()
    }

    /// set or remove the postBuildVariantMultiplicity attribute
    ///
    /// If postBuildVariantMultiplicity is true, then the parameter or reference
    /// may have a different number of instances in different post-build variants.
    #[setter]
    fn set_post_build_variant_multiplicity(
        &self,
        post_build_variant_multiplicity: Option<bool>,
    ) -> PyResult<()> {
        self.0
            .set_post_build_variant_multiplicity(post_build_variant_multiplicity)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the postBuildVariantMultiplicity attribute
    ///
    /// If postBuildVariantMultiplicity is true, then the parameter or reference
    /// may have a different number of instances in different post-build variants.
    #[getter]
    fn post_build_variant_multiplicity(&self) -> Option<bool> {
        self.0.post_build_variant_multiplicity()
    }

    /// set or remove the postBuildVariantValue attribute
    ///
    /// If postBuildVariantValue is true, then the parameter or reference
    /// may have different values in different post-build variants.
    #[setter]
    fn set_post_build_variant_value(&self, post_build_variant_value: Option<bool>) -> PyResult<()> {
        self.0
            .set_post_build_variant_value(post_build_variant_value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the postBuildVariantValue attribute
    ///
    /// If postBuildVariantValue is true, then the parameter or reference
    /// may have different values in different post-build variants.
    #[getter]
    fn post_build_variant_value(&self) -> Option<bool> {
        self.0.post_build_variant_value()
    }

    /// set or remove the requiresIndex attribute
    #[setter]
    fn set_requires_index(&self, requires_index: Option<bool>) -> PyResult<()> {
        self.0
            .set_requires_index(requires_index)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the requiresIndex attribute
    #[getter]
    fn requires_index(&self) -> Option<bool> {
        self.0.requires_index()
    }

    /// set the value config classes of the parameter definition.
    ///
    /// If an empty list is provided, the value config classes are removed.
    /// According to the specification setting is required if the containing EcucModuleDef
    /// has the category VENDOR_SPECIFIC_MODULE_DEFINITION, but in practice it is rarely used.
    #[setter]
    fn set_value_config_classes(
        &self,
        config: Vec<(EcucConfigurationClass, EcucConfigurationVariant)>,
    ) -> PyResult<()> {
        let config: Vec<_> = config
            .iter()
            .map(|(class, variant)| ((*class).into(), (*variant).into()))
            .collect();
        self.0
            .set_value_config_classes(&config)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the value config classes of the parameter definition
    ///
    /// According to the specification setting is required if the containing EcucModuleDef
    /// has the category VENDOR_SPECIFIC_MODULE_DEFINITION, but in practice it is rarely used.
    #[getter]
    fn value_config_classes(&self) -> Vec<(EcucConfigurationClass, EcucConfigurationVariant)> {
        self.0
            .value_config_classes()
            .iter()
            .map(|(class, variant)| {
                (
                    EcucConfigurationClass::from(*class),
                    EcucConfigurationVariant::from(*variant),
                )
            })
            .collect()
    }

    /// set or remove the withAuto attribute
    ///
    /// If withAuto is true, then the parameter or reference is allowed to set its isAutoValue attribute to true.
    #[setter]
    fn set_with_auto(&self, with_auto: Option<bool>) -> PyResult<()> {
        self.0
            .set_with_auto(with_auto)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the withAuto attribute
    #[getter]
    fn with_auto(&self) -> Option<bool> {
        self.0.with_auto()
    }

    // ------- EcucDefinitionElement -------
    /// set or remove the lower multiplicity attribute
    #[setter]
    fn set_lower_multiplicity(&self, lower_multiplicity: Option<u32>) -> PyResult<()> {
        self.0
            .set_lower_multiplicity(lower_multiplicity)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the lower multiplicity attribute
    #[getter]
    fn lower_multiplicity(&self) -> Option<u32> {
        self.0.lower_multiplicity()
    }

    /// set or remove the upper multiplicity attribute
    #[setter]
    fn set_upper_multiplicity(&self, upper_multiplicity: Option<u32>) -> PyResult<()> {
        self.0
            .set_upper_multiplicity(upper_multiplicity)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the upper multiplicity attribute
    #[getter]
    fn upper_multiplicity(&self) -> Option<u32> {
        self.0.upper_multiplicity()
    }

    /// set or remove the upper multiplicity infinite attribute
    ///
    /// if this attribute is set to true, the upper multiplicity is infinite
    /// (i.e. the module definition can be used an arbitrary number of times)
    /// When this attribute is true, the upper multiplicity attribute may not be used.
    #[setter]
    fn set_upper_multiplicity_infinite(&self, infinite: Option<bool>) -> PyResult<()> {
        self.0
            .set_upper_multiplicity_infinite(infinite)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the upper multiplicity infinite attribute
    #[getter]
    fn upper_multiplicity_infinite(&self) -> Option<bool> {
        self.0.upper_multiplicity_infinite()
    }
}

//##################################################################

/// `EcucMultilineStringParamDef` is used to specify a multiline string parameter in the ECU configuration
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._ecu_configuration"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct EcucMultilineStringParamDef(
    pub(crate) autosar_data_abstraction::ecu_configuration::EcucMultilineStringParamDef,
);

#[pymethods]
impl EcucMultilineStringParamDef {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::ecu_configuration::EcucMultilineStringParamDef::try_from(
            element.0.clone(),
        ) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    #[setter]
    fn set_name(&self, name: &str) -> PyResult<()> {
        self.0.set_name(name).map_err(abstraction_err_to_pyerr)
    }

    #[getter]
    fn name(&self) -> Option<String> {
        self.0.name()
    }

    #[getter]
    fn element(&self) -> Element {
        Element(self.0.element().clone())
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self.0)
    }

    // ------- EcucAbstractStringParamDef -------

    /// set or remove the max length attribute
    #[setter]
    fn set_max_length(&self, max_length: Option<u32>) -> PyResult<()> {
        self.0
            .set_max_length(max_length)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the max length attribute
    #[getter]
    fn max_length(&self) -> Option<u32> {
        self.0.max_length()
    }

    /// set or remove the min length attribute
    #[setter]
    fn set_min_length(&self, min_length: Option<u32>) -> PyResult<()> {
        self.0
            .set_min_length(min_length)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the min length attribute
    #[getter]
    fn min_length(&self) -> Option<u32> {
        self.0.min_length()
    }

    /// set or remove the regular expression attribute
    /// The regular expression is a string that is used to validate the string parameter
    #[setter]
    fn set_regular_expression(&self, regular_expression: Option<&str>) -> PyResult<()> {
        self.0
            .set_regular_expression(regular_expression)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the regular expression attribute
    /// The regular expression is a string that is used to validate the string parameter
    #[getter]
    fn regular_expression(&self) -> Option<String> {
        self.0.regular_expression()
    }

    /// set or remove the default value attribute
    #[setter]
    fn set_default_value(&self, default_value: Option<&str>) -> PyResult<()> {
        self.0
            .set_default_value(default_value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the default value attribute
    #[getter]
    fn default_value(&self) -> Option<String> {
        self.0.default_value()
    }

    // ------- EcucCommonAttributes -------

    /// set the multiplicity config classes of the parameter definition.
    /// If an empty list is provided, the multiplicity config classes are removed.
    ///
    /// This setting is required if the containing EcucModuleDef has the category VENDOR_SPECIFIC_MODULE_DEFINITION.
    #[setter]
    fn set_multiplicity_config_classes(
        &self,
        config: Vec<(EcucConfigurationClass, EcucConfigurationVariant)>,
    ) -> PyResult<()> {
        let config: Vec<_> = config
            .iter()
            .map(|(class, variant)| ((*class).into(), (*variant).into()))
            .collect();
        self.0
            .set_multiplicity_config_classes(&config)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the multiplicity config classes of the parameter definition
    #[getter]
    fn multiplicity_config_classes(
        &self,
    ) -> Vec<(EcucConfigurationClass, EcucConfigurationVariant)> {
        self.0
            .multiplicity_config_classes()
            .iter()
            .map(|(class, variant)| {
                (
                    EcucConfigurationClass::from(*class),
                    EcucConfigurationVariant::from(*variant),
                )
            })
            .collect()
    }

    /// set the origin of the parameter definition
    ///
    /// The origin is a string that describes if the parameter was defined in the AUTOSAR standard or by a vendor.
    /// Standardized parameters use the origin "AUTOSAR_ECUC", while vendors are supposed to use string like "VendorXyz_v1.3"
    #[setter]
    fn set_origin(&self, origin: &str) -> PyResult<()> {
        self.0.set_origin(origin).map_err(abstraction_err_to_pyerr)
    }

    /// get the origin of the parameter definition
    ///
    /// The origin is a string that describes if the parameter was defined in the AUTOSAR standard or by a vendor.
    /// Standardized parameters use the origin "AUTOSAR_ECUC", while vendors are supposed to use string like "VendorXyz_v1.3"
    #[getter]
    fn origin(&self) -> Option<String> {
        self.0.origin()
    }

    /// set or remove the postBuildVariantMultiplicity attribute
    ///
    /// If postBuildVariantMultiplicity is true, then the parameter or reference
    /// may have a different number of instances in different post-build variants.
    #[setter]
    fn set_post_build_variant_multiplicity(
        &self,
        post_build_variant_multiplicity: Option<bool>,
    ) -> PyResult<()> {
        self.0
            .set_post_build_variant_multiplicity(post_build_variant_multiplicity)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the postBuildVariantMultiplicity attribute
    ///
    /// If postBuildVariantMultiplicity is true, then the parameter or reference
    /// may have a different number of instances in different post-build variants.
    #[getter]
    fn post_build_variant_multiplicity(&self) -> Option<bool> {
        self.0.post_build_variant_multiplicity()
    }

    /// set or remove the postBuildVariantValue attribute
    ///
    /// If postBuildVariantValue is true, then the parameter or reference
    /// may have different values in different post-build variants.
    #[setter]
    fn set_post_build_variant_value(&self, post_build_variant_value: Option<bool>) -> PyResult<()> {
        self.0
            .set_post_build_variant_value(post_build_variant_value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the postBuildVariantValue attribute
    ///
    /// If postBuildVariantValue is true, then the parameter or reference
    /// may have different values in different post-build variants.
    #[getter]
    fn post_build_variant_value(&self) -> Option<bool> {
        self.0.post_build_variant_value()
    }

    /// set or remove the requiresIndex attribute
    #[setter]
    fn set_requires_index(&self, requires_index: Option<bool>) -> PyResult<()> {
        self.0
            .set_requires_index(requires_index)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the requiresIndex attribute
    #[getter]
    fn requires_index(&self) -> Option<bool> {
        self.0.requires_index()
    }

    /// set the value config classes of the parameter definition.
    ///
    /// If an empty list is provided, the value config classes are removed.
    /// According to the specification setting is required if the containing EcucModuleDef
    /// has the category VENDOR_SPECIFIC_MODULE_DEFINITION, but in practice it is rarely used.
    #[setter]
    fn set_value_config_classes(
        &self,
        config: Vec<(EcucConfigurationClass, EcucConfigurationVariant)>,
    ) -> PyResult<()> {
        let config: Vec<_> = config
            .iter()
            .map(|(class, variant)| ((*class).into(), (*variant).into()))
            .collect();
        self.0
            .set_value_config_classes(&config)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the value config classes of the parameter definition
    ///
    /// According to the specification setting is required if the containing EcucModuleDef
    /// has the category VENDOR_SPECIFIC_MODULE_DEFINITION, but in practice it is rarely used.
    #[getter]
    fn value_config_classes(&self) -> Vec<(EcucConfigurationClass, EcucConfigurationVariant)> {
        self.0
            .value_config_classes()
            .iter()
            .map(|(class, variant)| {
                (
                    EcucConfigurationClass::from(*class),
                    EcucConfigurationVariant::from(*variant),
                )
            })
            .collect()
    }

    /// set or remove the withAuto attribute
    ///
    /// If withAuto is true, then the parameter or reference is allowed to set its isAutoValue attribute to true.
    #[setter]
    fn set_with_auto(&self, with_auto: Option<bool>) -> PyResult<()> {
        self.0
            .set_with_auto(with_auto)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the withAuto attribute
    #[getter]
    fn with_auto(&self) -> Option<bool> {
        self.0.with_auto()
    }

    // ------- EcucDefinitionElement -------
    /// set or remove the lower multiplicity attribute
    #[setter]
    fn set_lower_multiplicity(&self, lower_multiplicity: Option<u32>) -> PyResult<()> {
        self.0
            .set_lower_multiplicity(lower_multiplicity)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the lower multiplicity attribute
    #[getter]
    fn lower_multiplicity(&self) -> Option<u32> {
        self.0.lower_multiplicity()
    }

    /// set or remove the upper multiplicity attribute
    #[setter]
    fn set_upper_multiplicity(&self, upper_multiplicity: Option<u32>) -> PyResult<()> {
        self.0
            .set_upper_multiplicity(upper_multiplicity)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the upper multiplicity attribute
    #[getter]
    fn upper_multiplicity(&self) -> Option<u32> {
        self.0.upper_multiplicity()
    }

    /// set or remove the upper multiplicity infinite attribute
    ///
    /// if this attribute is set to true, the upper multiplicity is infinite
    /// (i.e. the module definition can be used an arbitrary number of times)
    /// When this attribute is true, the upper multiplicity attribute may not be used.
    #[setter]
    fn set_upper_multiplicity_infinite(&self, infinite: Option<bool>) -> PyResult<()> {
        self.0
            .set_upper_multiplicity_infinite(infinite)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the upper multiplicity infinite attribute
    #[getter]
    fn upper_multiplicity_infinite(&self) -> Option<bool> {
        self.0.upper_multiplicity_infinite()
    }
}

//##################################################################

/// `EcucStringParamDef` is used to specify a string parameter in the ECU configuration
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._ecu_configuration"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct EcucStringParamDef(
    pub(crate) autosar_data_abstraction::ecu_configuration::EcucStringParamDef,
);

#[pymethods]
impl EcucStringParamDef {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::ecu_configuration::EcucStringParamDef::try_from(
            element.0.clone(),
        ) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    #[setter]
    fn set_name(&self, name: &str) -> PyResult<()> {
        self.0.set_name(name).map_err(abstraction_err_to_pyerr)
    }

    #[getter]
    fn name(&self) -> Option<String> {
        self.0.name()
    }

    #[getter]
    fn element(&self) -> Element {
        Element(self.0.element().clone())
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self.0)
    }

    // ------- EcucAbstractStringParamDef -------

    /// set or remove the max length attribute
    #[setter]
    fn set_max_length(&self, max_length: Option<u32>) -> PyResult<()> {
        self.0
            .set_max_length(max_length)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the max length attribute
    #[getter]
    fn max_length(&self) -> Option<u32> {
        self.0.max_length()
    }

    /// set or remove the min length attribute
    #[setter]
    fn set_min_length(&self, min_length: Option<u32>) -> PyResult<()> {
        self.0
            .set_min_length(min_length)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the min length attribute
    #[getter]
    fn min_length(&self) -> Option<u32> {
        self.0.min_length()
    }

    /// set or remove the regular expression attribute
    /// The regular expression is a string that is used to validate the string parameter
    #[setter]
    fn set_regular_expression(&self, regular_expression: Option<&str>) -> PyResult<()> {
        self.0
            .set_regular_expression(regular_expression)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the regular expression attribute
    /// The regular expression is a string that is used to validate the string parameter
    #[getter]
    fn regular_expression(&self) -> Option<String> {
        self.0.regular_expression()
    }

    /// set or remove the default value attribute
    #[setter]
    fn set_default_value(&self, default_value: Option<&str>) -> PyResult<()> {
        self.0
            .set_default_value(default_value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the default value attribute
    #[getter]
    fn default_value(&self) -> Option<String> {
        self.0.default_value()
    }

    // ------- EcucCommonAttributes -------

    /// set the multiplicity config classes of the parameter definition.
    /// If an empty list is provided, the multiplicity config classes are removed.
    ///
    /// This setting is required if the containing EcucModuleDef has the category VENDOR_SPECIFIC_MODULE_DEFINITION.
    #[setter]
    fn set_multiplicity_config_classes(
        &self,
        config: Vec<(EcucConfigurationClass, EcucConfigurationVariant)>,
    ) -> PyResult<()> {
        let config: Vec<_> = config
            .iter()
            .map(|(class, variant)| ((*class).into(), (*variant).into()))
            .collect();
        self.0
            .set_multiplicity_config_classes(&config)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the multiplicity config classes of the parameter definition
    #[getter]
    fn multiplicity_config_classes(
        &self,
    ) -> Vec<(EcucConfigurationClass, EcucConfigurationVariant)> {
        self.0
            .multiplicity_config_classes()
            .iter()
            .map(|(class, variant)| {
                (
                    EcucConfigurationClass::from(*class),
                    EcucConfigurationVariant::from(*variant),
                )
            })
            .collect()
    }

    /// set the origin of the parameter definition
    ///
    /// The origin is a string that describes if the parameter was defined in the AUTOSAR standard or by a vendor.
    /// Standardized parameters use the origin "AUTOSAR_ECUC", while vendors are supposed to use string like "VendorXyz_v1.3"
    #[setter]
    fn set_origin(&self, origin: &str) -> PyResult<()> {
        self.0.set_origin(origin).map_err(abstraction_err_to_pyerr)
    }

    /// get the origin of the parameter definition
    ///
    /// The origin is a string that describes if the parameter was defined in the AUTOSAR standard or by a vendor.
    /// Standardized parameters use the origin "AUTOSAR_ECUC", while vendors are supposed to use string like "VendorXyz_v1.3"
    #[getter]
    fn origin(&self) -> Option<String> {
        self.0.origin()
    }

    /// set or remove the postBuildVariantMultiplicity attribute
    ///
    /// If postBuildVariantMultiplicity is true, then the parameter or reference
    /// may have a different number of instances in different post-build variants.
    #[setter]
    fn set_post_build_variant_multiplicity(
        &self,
        post_build_variant_multiplicity: Option<bool>,
    ) -> PyResult<()> {
        self.0
            .set_post_build_variant_multiplicity(post_build_variant_multiplicity)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the postBuildVariantMultiplicity attribute
    ///
    /// If postBuildVariantMultiplicity is true, then the parameter or reference
    /// may have a different number of instances in different post-build variants.
    #[getter]
    fn post_build_variant_multiplicity(&self) -> Option<bool> {
        self.0.post_build_variant_multiplicity()
    }

    /// set or remove the postBuildVariantValue attribute
    ///
    /// If postBuildVariantValue is true, then the parameter or reference
    /// may have different values in different post-build variants.
    #[setter]
    fn set_post_build_variant_value(&self, post_build_variant_value: Option<bool>) -> PyResult<()> {
        self.0
            .set_post_build_variant_value(post_build_variant_value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the postBuildVariantValue attribute
    ///
    /// If postBuildVariantValue is true, then the parameter or reference
    /// may have different values in different post-build variants.
    #[getter]
    fn post_build_variant_value(&self) -> Option<bool> {
        self.0.post_build_variant_value()
    }

    /// set or remove the requiresIndex attribute
    #[setter]
    fn set_requires_index(&self, requires_index: Option<bool>) -> PyResult<()> {
        self.0
            .set_requires_index(requires_index)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the requiresIndex attribute
    #[getter]
    fn requires_index(&self) -> Option<bool> {
        self.0.requires_index()
    }

    /// set the value config classes of the parameter definition.
    ///
    /// If an empty list is provided, the value config classes are removed.
    /// According to the specification setting is required if the containing EcucModuleDef
    /// has the category VENDOR_SPECIFIC_MODULE_DEFINITION, but in practice it is rarely used.
    #[setter]
    fn set_value_config_classes(
        &self,
        config: Vec<(EcucConfigurationClass, EcucConfigurationVariant)>,
    ) -> PyResult<()> {
        let config: Vec<_> = config
            .iter()
            .map(|(class, variant)| ((*class).into(), (*variant).into()))
            .collect();
        self.0
            .set_value_config_classes(&config)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the value config classes of the parameter definition
    ///
    /// According to the specification setting is required if the containing EcucModuleDef
    /// has the category VENDOR_SPECIFIC_MODULE_DEFINITION, but in practice it is rarely used.
    #[getter]
    fn value_config_classes(&self) -> Vec<(EcucConfigurationClass, EcucConfigurationVariant)> {
        self.0
            .value_config_classes()
            .iter()
            .map(|(class, variant)| {
                (
                    EcucConfigurationClass::from(*class),
                    EcucConfigurationVariant::from(*variant),
                )
            })
            .collect()
    }

    /// set or remove the withAuto attribute
    ///
    /// If withAuto is true, then the parameter or reference is allowed to set its isAutoValue attribute to true.
    #[setter]
    fn set_with_auto(&self, with_auto: Option<bool>) -> PyResult<()> {
        self.0
            .set_with_auto(with_auto)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the withAuto attribute
    #[getter]
    fn with_auto(&self) -> Option<bool> {
        self.0.with_auto()
    }

    // ------- EcucDefinitionElement -------
    /// set or remove the lower multiplicity attribute
    #[setter]
    fn set_lower_multiplicity(&self, lower_multiplicity: Option<u32>) -> PyResult<()> {
        self.0
            .set_lower_multiplicity(lower_multiplicity)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the lower multiplicity attribute
    #[getter]
    fn lower_multiplicity(&self) -> Option<u32> {
        self.0.lower_multiplicity()
    }

    /// set or remove the upper multiplicity attribute
    #[setter]
    fn set_upper_multiplicity(&self, upper_multiplicity: Option<u32>) -> PyResult<()> {
        self.0
            .set_upper_multiplicity(upper_multiplicity)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the upper multiplicity attribute
    #[getter]
    fn upper_multiplicity(&self) -> Option<u32> {
        self.0.upper_multiplicity()
    }

    /// set or remove the upper multiplicity infinite attribute
    ///
    /// if this attribute is set to true, the upper multiplicity is infinite
    /// (i.e. the module definition can be used an arbitrary number of times)
    /// When this attribute is true, the upper multiplicity attribute may not be used.
    #[setter]
    fn set_upper_multiplicity_infinite(&self, infinite: Option<bool>) -> PyResult<()> {
        self.0
            .set_upper_multiplicity_infinite(infinite)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the upper multiplicity infinite attribute
    #[getter]
    fn upper_multiplicity_infinite(&self) -> Option<bool> {
        self.0.upper_multiplicity_infinite()
    }
}

//##################################################################

iterator_wrapper!(EcucParameterDefIterator, Py<PyAny>, "EcucParameterDef");

//##################################################################

pub(crate) fn ecuc_parameter_def_to_pyany(
    parameter_def: autosar_data_abstraction::ecu_configuration::EcucParameterDef,
) -> PyResult<Py<PyAny>> {
    Python::attach(|py| match parameter_def {
        autosar_data_abstraction::ecu_configuration::EcucParameterDef::AddInfo(value) => {
            EcucAddInfoParamDef(value).into_py_any(py)
        }
        autosar_data_abstraction::ecu_configuration::EcucParameterDef::Boolean(value) => {
            EcucBooleanParamDef(value).into_py_any(py)
        }
        autosar_data_abstraction::ecu_configuration::EcucParameterDef::Enumeration(value) => {
            EcucEnumerationParamDef(value).into_py_any(py)
        }
        autosar_data_abstraction::ecu_configuration::EcucParameterDef::Float(value) => {
            EcucFloatParamDef(value).into_py_any(py)
        }
        autosar_data_abstraction::ecu_configuration::EcucParameterDef::Integer(value) => {
            EcucIntegerParamDef(value).into_py_any(py)
        }
        autosar_data_abstraction::ecu_configuration::EcucParameterDef::FunctionName(value) => {
            EcucFunctionNameDef(value).into_py_any(py)
        }
        autosar_data_abstraction::ecu_configuration::EcucParameterDef::LinkerSymbol(value) => {
            EcucLinkerSymbolDef(value).into_py_any(py)
        }
        autosar_data_abstraction::ecu_configuration::EcucParameterDef::MultilineString(value) => {
            EcucMultilineStringParamDef(value).into_py_any(py)
        }
        autosar_data_abstraction::ecu_configuration::EcucParameterDef::String(value) => {
            EcucStringParamDef(value).into_py_any(py)
        }
    })
}

pub(crate) fn pyany_to_ecuc_parameter_def(
    pyobject: &Bound<'_, PyAny>,
) -> PyResult<autosar_data_abstraction::ecu_configuration::EcucParameterDef> {
    if let Ok(value) = pyobject.extract::<EcucAddInfoParamDef>() {
        Ok(autosar_data_abstraction::ecu_configuration::EcucParameterDef::AddInfo(value.0))
    } else if let Ok(value) = pyobject.extract::<EcucBooleanParamDef>() {
        Ok(autosar_data_abstraction::ecu_configuration::EcucParameterDef::Boolean(value.0))
    } else if let Ok(value) = pyobject.extract::<EcucEnumerationParamDef>() {
        Ok(autosar_data_abstraction::ecu_configuration::EcucParameterDef::Enumeration(value.0))
    } else if let Ok(value) = pyobject.extract::<EcucFloatParamDef>() {
        Ok(autosar_data_abstraction::ecu_configuration::EcucParameterDef::Float(value.0))
    } else if let Ok(value) = pyobject.extract::<EcucIntegerParamDef>() {
        Ok(autosar_data_abstraction::ecu_configuration::EcucParameterDef::Integer(value.0))
    } else if let Ok(value) = pyobject.extract::<EcucFunctionNameDef>() {
        Ok(autosar_data_abstraction::ecu_configuration::EcucParameterDef::FunctionName(value.0))
    } else if let Ok(value) = pyobject.extract::<EcucLinkerSymbolDef>() {
        Ok(autosar_data_abstraction::ecu_configuration::EcucParameterDef::LinkerSymbol(value.0))
    } else if let Ok(value) = pyobject.extract::<EcucMultilineStringParamDef>() {
        Ok(autosar_data_abstraction::ecu_configuration::EcucParameterDef::MultilineString(value.0))
    } else if let Ok(value) = pyobject.extract::<EcucStringParamDef>() {
        Ok(autosar_data_abstraction::ecu_configuration::EcucParameterDef::String(value.0))
    } else {
        Err(AutosarAbstractionError::new_err(format!(
            "Unsupported parameter definition: {pyobject:?}"
        )))
    }
}
