use crate::{
    Element,
    abstraction::{
        AutosarAbstractionError, abstraction_err_to_pyerr,
        ecu_configuration::{
            EcucConfigurationClass, EcucConfigurationVariant, EcucContainerDefIterator,
            EcucDestinationUriDef, ecuc_container_def_from_pyany, ecuc_container_def_to_pyany,
        },
    },
    iterator_wrapper,
};
use autosar_data_abstraction::{
    self, AbstractionElement, IdentifiableAbstractionElement,
    ecu_configuration::{EcucCommonAttributes, EcucDefinitionElement},
};
use pyo3::{IntoPyObjectExt, prelude::*};

//##################################################################

/// marker trait for all reference definitions
/// The `EcucForeignReferenceDef` specifies a reference to an XML description of an entity
/// described in another AUTOSAR template.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._ecu_configuration"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct EcucForeignReferenceDef(
    pub(crate) autosar_data_abstraction::ecu_configuration::EcucForeignReferenceDef,
);

#[pymethods]
impl EcucForeignReferenceDef {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::ecu_configuration::EcucForeignReferenceDef::try_from(
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

    /// set the destination type of the reference definition
    #[setter]
    fn set_destination_type(&self, destination_type: Option<&str>) -> PyResult<()> {
        self.0
            .set_destination_type(destination_type)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the destination type of the reference definition
    #[getter]
    fn destination_type(&self) -> Option<String> {
        self.0.destination_type()
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

/// The `EcucInstanceReferenceDef` specifies a reference to an XML description of an entity
/// described in another AUTOSAR template using INSTANCE REFERENCE semantics.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._ecu_configuration"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct EcucInstanceReferenceDef(
    pub(crate) autosar_data_abstraction::ecu_configuration::EcucInstanceReferenceDef,
);

#[pymethods]
impl EcucInstanceReferenceDef {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::ecu_configuration::EcucInstanceReferenceDef::try_from(
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

    /// set the destination type of the reference definition
    #[setter]
    fn set_destination_type(&self, destination_type: Option<&str>) -> PyResult<()> {
        self.0
            .set_destination_type(destination_type)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the destination type of the reference definition
    #[getter]
    fn destination_type(&self) -> Option<String> {
        self.0.destination_type()
    }

    /// set the destination context of the reference definition
    ///
    /// The destination context is a string of autosar element names separated by spaces.
    /// Additionally, the '*' character can be used to indicate multiple occurrences of the previous element.
    /// E.g. "SW-COMPONENT-PROTOTYPE* R-PORT-PROTOTYPE"
    #[setter]
    fn set_destination_context(&self, destination_context: Option<&str>) -> PyResult<()> {
        self.0
            .set_destination_context(destination_context)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the destination context of the reference definition
    ///
    /// The destination context is a string of autosar element names separated by spaces.
    #[getter]
    fn destination_context(&self) -> Option<String> {
        self.0.destination_context()
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

/// The `EcucChoiceReferenceDef` specifies alternative references where only one of the specified
/// references will be used in the ECU configuration.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._ecu_configuration"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct EcucChoiceReferenceDef(
    pub(crate) autosar_data_abstraction::ecu_configuration::EcucChoiceReferenceDef,
);

#[pymethods]
impl EcucChoiceReferenceDef {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::ecu_configuration::EcucChoiceReferenceDef::try_from(
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

    /// add a reference to a destination container
    fn add_destination(&self, destination: &Bound<'_, PyAny>) -> PyResult<()> {
        let destination = ecuc_container_def_from_pyany(destination)?;
        self.0
            .add_destination(&destination)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the references to the destination containers
    fn destination_refs(&self) -> EcucContainerDefIterator {
        EcucContainerDefIterator::new(
            self.0
                .destination_refs()
                .filter_map(|container| ecuc_container_def_to_pyany(container).ok()),
        )
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

/// The `EcuReferenceDef` specifies references between parameters in the ECU configuration.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._ecu_configuration"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct EcucReferenceDef(
    pub(crate) autosar_data_abstraction::ecu_configuration::EcucReferenceDef,
);

#[pymethods]
impl EcucReferenceDef {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::ecu_configuration::EcucReferenceDef::try_from(
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

    /// set the destination container of the reference
    #[setter]
    fn set_destination(&self, destination: Option<&Bound<'_, PyAny>>) -> PyResult<()> {
        let destination = destination.and_then(|d| ecuc_container_def_from_pyany(d).ok());
        self.0
            .set_destination(destination.as_ref())
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the destination container of the reference
    #[getter]
    fn destination(&self) -> Option<Py<PyAny>> {
        self.0
            .destination()
            .and_then(|d| ecuc_container_def_to_pyany(d).ok())
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

/// The `EcucUriReferenceDef` defines a reference with a destination that is specified via a destinationUri
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._ecu_configuration"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct EcucUriReferenceDef(
    pub(crate) autosar_data_abstraction::ecu_configuration::EcucUriReferenceDef,
);

#[pymethods]
impl EcucUriReferenceDef {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::ecu_configuration::EcucUriReferenceDef::try_from(
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

    /// set the destination uri of the reference definition
    #[setter]
    fn set_destination_uri(&self, destination_uri: Option<&EcucDestinationUriDef>) -> PyResult<()> {
        let destination_uri = destination_uri.as_ref().map(|d| &d.0);
        self.0
            .set_destination_uri(destination_uri)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the destination uri of the reference definition
    #[getter]
    fn destination_uri(&self) -> Option<EcucDestinationUriDef> {
        self.0.destination_uri().map(EcucDestinationUriDef)
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

iterator_wrapper!(
    EcucAnyReferenceDefIterator,
    Py<PyAny>,
    "EcucAnyReferenceDef"
);

//##################################################################

pub(crate) fn ecuc_reference_def_to_pyany(
    reference_def: autosar_data_abstraction::ecu_configuration::EcucAnyReferenceDef,
) -> PyResult<Py<PyAny>> {
    Python::attach(|py| match reference_def {
        autosar_data_abstraction::ecu_configuration::EcucAnyReferenceDef::Foreign(value) => {
            EcucForeignReferenceDef(value).into_py_any(py)
        }
        autosar_data_abstraction::ecu_configuration::EcucAnyReferenceDef::Instance(value) => {
            EcucInstanceReferenceDef(value).into_py_any(py)
        }
        autosar_data_abstraction::ecu_configuration::EcucAnyReferenceDef::Choice(value) => {
            EcucChoiceReferenceDef(value).into_py_any(py)
        }
        autosar_data_abstraction::ecu_configuration::EcucAnyReferenceDef::Normal(value) => {
            EcucReferenceDef(value).into_py_any(py)
        }
        autosar_data_abstraction::ecu_configuration::EcucAnyReferenceDef::Uri(value) => {
            EcucUriReferenceDef(value).into_py_any(py)
        }
    })
}

pub(crate) fn pyany_to_ecuc_reference_def(
    py_reference_def: &Bound<'_, PyAny>,
) -> PyResult<autosar_data_abstraction::ecu_configuration::EcucAnyReferenceDef> {
    if let Ok(reference_def) = py_reference_def.extract::<EcucForeignReferenceDef>() {
        Ok(
            autosar_data_abstraction::ecu_configuration::EcucAnyReferenceDef::Foreign(
                reference_def.0,
            ),
        )
    } else if let Ok(reference_def) = py_reference_def.extract::<EcucInstanceReferenceDef>() {
        Ok(
            autosar_data_abstraction::ecu_configuration::EcucAnyReferenceDef::Instance(
                reference_def.0,
            ),
        )
    } else if let Ok(reference_def) = py_reference_def.extract::<EcucChoiceReferenceDef>() {
        Ok(
            autosar_data_abstraction::ecu_configuration::EcucAnyReferenceDef::Choice(
                reference_def.0,
            ),
        )
    } else if let Ok(reference_def) = py_reference_def.extract::<EcucReferenceDef>() {
        Ok(
            autosar_data_abstraction::ecu_configuration::EcucAnyReferenceDef::Normal(
                reference_def.0,
            ),
        )
    } else if let Ok(reference_def) = py_reference_def.extract::<EcucUriReferenceDef>() {
        Ok(autosar_data_abstraction::ecu_configuration::EcucAnyReferenceDef::Uri(reference_def.0))
    } else {
        Err(AutosarAbstractionError::new_err(format!(
            "Invalid reference definition: {py_reference_def:?}"
        )))
    }
}
