use crate::{abstraction::*, *};
use autosar_data_abstraction::{
    self, AbstractionElement, IdentifiableAbstractionElement,
    ecu_configuration::EcucDefinitionElement,
};

mod container;
mod parameter;
mod reference;

pub(crate) use container::*;
pub(crate) use parameter::*;
pub(crate) use reference::*;

//##################################################################

/// The `EcucDefinitionCollection` is a container for all module definitions in the ECU configuration
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._ecu_configuration"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct EcucDefinitionCollection(
    pub(crate) autosar_data_abstraction::ecu_configuration::EcucDefinitionCollection,
);

#[pymethods]
impl EcucDefinitionCollection {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::ecu_configuration::EcucDefinitionCollection::try_from(
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

    /// add a reference to a module definition to the collection
    #[pyo3(signature = (module_def, /))]
    #[pyo3(text_signature = "(self, module_def: EcucModuleDef, /)")]
    fn add_module_def(&self, module_def: &EcucModuleDef) -> PyResult<()> {
        self.0
            .add_module_def(&module_def.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// iterate over all module definitions in the collection
    fn module_defs(&self) -> EcucModuleDefIterator {
        EcucModuleDefIterator::new(self.0.module_defs().map(EcucModuleDef))
    }
}

//##################################################################

/// The `EcucModuleDef` is a container for the definition of a single base software module
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._ecu_configuration"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct EcucModuleDef(
    pub(crate) autosar_data_abstraction::ecu_configuration::EcucModuleDef,
);

#[pymethods]
impl EcucModuleDef {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::ecu_configuration::EcucModuleDef::try_from(
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

    /// create a new EcucChoiceContainerDef in the module
    #[pyo3(signature = (name, /))]
    #[pyo3(text_signature = "(self, name: str, /)")]
    fn create_choice_container_def(&self, name: &str) -> PyResult<EcucChoiceContainerDef> {
        match self.0.create_choice_container_def(name) {
            Ok(value) => Ok(EcucChoiceContainerDef(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new EcucParamConfContainerDef in the module
    #[pyo3(signature = (name, /))]
    #[pyo3(text_signature = "(self, name: str, /)")]
    fn create_param_conf_container_def(&self, name: &str) -> PyResult<EcucParamConfContainerDef> {
        match self.0.create_param_conf_container_def(name) {
            Ok(value) => Ok(EcucParamConfContainerDef(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all containers in the module
    fn containers(&self) -> EcucContainerDefIterator {
        EcucContainerDefIterator::new(
            self.0
                .containers()
                .filter_map(|container| ecuc_container_def_to_pyany(container).ok()),
        )
    }

    /// set or remove the apiServicePrefix for the module
    ///
    /// for CDD modules the short name of the module is always "CDD", so
    /// this attribute is needed to define the prefix for the API services
    #[setter]
    fn set_api_service_prefix(&self, prefix: Option<&str>) -> PyResult<()> {
        self.0
            .set_api_service_prefix(prefix)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the apiServicePrefix for the module
    ///
    /// for CDD modules the short name of the module is always "CDD", so
    /// this attribute is needed to define the prefix for the API services
    #[getter]
    fn api_service_prefix(&self) -> Option<String> {
        self.0.api_service_prefix()
    }

    /// set the supported configuration variants for the module
    #[setter]
    fn set_supported_config_variants(
        &self,
        variants: Vec<EcucConfigurationVariant>,
    ) -> PyResult<()> {
        let variants = variants
            .iter()
            .map(|variant| (*variant).into())
            .collect::<Vec<_>>();
        self.0
            .set_supported_config_variants(&variants)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the supported configuration variants for the module
    #[getter]
    fn supported_config_variants(&self) -> Vec<EcucConfigurationVariant> {
        self.0
            .supported_config_variants()
            .iter()
            .map(|variant| (*variant).into())
            .collect()
    }

    /// set or remove the post build variant support attribute
    #[setter]
    fn set_post_build_variant_support(&self, support: Option<bool>) -> PyResult<()> {
        self.0
            .set_post_build_variant_support(support)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the post build variant support attribute
    #[getter]
    fn post_build_variant_support(&self) -> Option<bool> {
        self.0.post_build_variant_support()
    }

    /// set or remove the category of the module definition
    #[setter]
    fn set_category(&self, category: Option<EcucModuleDefCategory>) -> PyResult<()> {
        self.0
            .set_category(category.map(std::convert::Into::into))
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the category of the module definition
    #[getter]
    fn category(&self) -> Option<EcucModuleDefCategory> {
        self.0.category().map(EcucModuleDefCategory::from)
    }

    /// set or remove the reference to a refined standard module
    ///
    /// This reference is only used if the category is `VendorSpecificModuleDefinition`
    #[setter]
    fn set_refined_module_def(&self, refined_module_def: Option<&EcucModuleDef>) -> PyResult<()> {
        self.0
            .set_refined_module_def(refined_module_def.as_ref().map(|def| &def.0))
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the reference to a refined standard module
    ///
    /// This reference is only used if the category is `VendorSpecificModuleDefinition`
    #[getter]
    fn refined_module_def(&self) -> Option<EcucModuleDef> {
        self.0.refined_module_def().map(EcucModuleDef)
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

iterator_wrapper!(EcucModuleDefIterator, EcucModuleDef);

//##################################################################

/// `EcucConfigurationVariant` provides the different configuration variants that
/// can be used by the module definition.
#[pyclass(
    frozen,
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._ecu_configuration"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EcucConfigurationVariant {
    /// Preconfigured (i.e. fixed) configuration which cannot be changed.
    PreconfiguredConfiguration,
    /// Recommended configuration
    RecommendedConfiguration,
    /// the BSW Module implementation may use PreCompileTime and LinkTime configuration parameters
    VariantLinkTime,
    /// the BSW Module implementation may use PreCompileTime, LinkTime and PostBuild configuration parameters
    VariantPostBuild,
    /// the BSW Module implementation may use PreCompileTime configuration parameters
    VariantPreCompile,
    /// deprecated in Autosar 4.2.1 - the BSW Module implementation may use PreCompileTime, LinkTime and PostBuild loadable configuration parameters
    VariantPostBuildLoadable,
    /// deprecated in Autosar 4.2.1 - the BSW Module implementation may use PreCompileTime, LinkTime and PostBuild selectable configuration parameters
    VariantPostBuildSelectable,
}

impl From<EcucConfigurationVariant>
    for autosar_data_abstraction::ecu_configuration::EcucConfigurationVariant
{
    fn from(variant: EcucConfigurationVariant) -> Self {
        match variant {
            EcucConfigurationVariant::PreconfiguredConfiguration => {
                Self::PreconfiguredConfiguration
            }
            EcucConfigurationVariant::RecommendedConfiguration => Self::RecommendedConfiguration,
            EcucConfigurationVariant::VariantLinkTime => Self::VariantLinkTime,
            EcucConfigurationVariant::VariantPostBuild => Self::VariantPostBuild,
            EcucConfigurationVariant::VariantPreCompile => Self::VariantPreCompile,
            EcucConfigurationVariant::VariantPostBuildLoadable => Self::VariantPostBuildLoadable,
            EcucConfigurationVariant::VariantPostBuildSelectable => {
                Self::VariantPostBuildSelectable
            }
        }
    }
}

impl From<autosar_data_abstraction::ecu_configuration::EcucConfigurationVariant>
    for EcucConfigurationVariant
{
    fn from(
        variant: autosar_data_abstraction::ecu_configuration::EcucConfigurationVariant,
    ) -> Self {
        use autosar_data_abstraction::ecu_configuration::EcucConfigurationVariant as In;
        match variant {
            In::PreconfiguredConfiguration => EcucConfigurationVariant::PreconfiguredConfiguration,
            In::RecommendedConfiguration => EcucConfigurationVariant::RecommendedConfiguration,
            In::VariantLinkTime => EcucConfigurationVariant::VariantLinkTime,
            In::VariantPostBuild => EcucConfigurationVariant::VariantPostBuild,
            In::VariantPreCompile => EcucConfigurationVariant::VariantPreCompile,
            In::VariantPostBuildLoadable => EcucConfigurationVariant::VariantPostBuildLoadable,
            In::VariantPostBuildSelectable => EcucConfigurationVariant::VariantPostBuildSelectable,
        }
    }
}

//##################################################################

/// `EcucConfigurationClass` provides the different configuration classes for Autosar configuration parameters
#[pyclass(
    frozen,
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._ecu_configuration"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EcucConfigurationClass {
    /// Link Time: parts of configuration are delivered from another object code file
    Link,
    /// PostBuild: a configuration parameter can be changed after compilation
    PostBuild,
    /// PreCompile: a configuration parameter can not be changed after compilation
    PreCompile,
    /// PublishedInformation is used to specify the fact that certain information is fixed even before the pre-compile stage.
    PublishedInformation,
}

impl From<EcucConfigurationClass>
    for autosar_data_abstraction::ecu_configuration::EcucConfigurationClass
{
    fn from(class: EcucConfigurationClass) -> Self {
        match class {
            EcucConfigurationClass::Link => Self::Link,
            EcucConfigurationClass::PostBuild => Self::PostBuild,
            EcucConfigurationClass::PreCompile => Self::PreCompile,
            EcucConfigurationClass::PublishedInformation => Self::PublishedInformation,
        }
    }
}

impl From<autosar_data_abstraction::ecu_configuration::EcucConfigurationClass>
    for EcucConfigurationClass
{
    fn from(class: autosar_data_abstraction::ecu_configuration::EcucConfigurationClass) -> Self {
        use autosar_data_abstraction::ecu_configuration::EcucConfigurationClass as In;
        match class {
            In::Link => EcucConfigurationClass::Link,
            In::PostBuild => EcucConfigurationClass::PostBuild,
            In::PreCompile => EcucConfigurationClass::PreCompile,
            In::PublishedInformation => EcucConfigurationClass::PublishedInformation,
        }
    }
}

//#########################################################

/// The `EcucModuleDefCategory` represents the possible category values for a module definition
#[pyclass(
    frozen,
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._ecu_configuration"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EcucModuleDefCategory {
    /// The module definition is a standardized module (StMD)
    StandardizedModuleDefinition,
    /// The module definition is a vendor specific module (VSMD)
    VendorSpecificModuleDefinition,
}

impl From<EcucModuleDefCategory>
    for autosar_data_abstraction::ecu_configuration::EcucModuleDefCategory
{
    fn from(category: EcucModuleDefCategory) -> Self {
        match category {
            EcucModuleDefCategory::StandardizedModuleDefinition => {
                Self::StandardizedModuleDefinition
            }
            EcucModuleDefCategory::VendorSpecificModuleDefinition => {
                Self::VendorSpecificModuleDefinition
            }
        }
    }
}

impl From<autosar_data_abstraction::ecu_configuration::EcucModuleDefCategory>
    for EcucModuleDefCategory
{
    fn from(category: autosar_data_abstraction::ecu_configuration::EcucModuleDefCategory) -> Self {
        use autosar_data_abstraction::ecu_configuration::EcucModuleDefCategory as In;
        match category {
            In::StandardizedModuleDefinition => EcucModuleDefCategory::StandardizedModuleDefinition,
            In::VendorSpecificModuleDefinition => {
                EcucModuleDefCategory::VendorSpecificModuleDefinition
            }
        }
    }
}

//##################################################################

/// A `EcucDestinationUriDefSet` contains a list of `EcucDestinationUriDef`s
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._ecu_configuration"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct EcucDestinationUriDefSet(
    pub(crate) autosar_data_abstraction::ecu_configuration::EcucDestinationUriDefSet,
);

#[pymethods]
impl EcucDestinationUriDefSet {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::ecu_configuration::EcucDestinationUriDefSet::try_from(
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

    /// create a new `EcucDestinationUriDef`
    #[pyo3(signature = (name, contract, /))]
    #[pyo3(text_signature = "(self, name: str, contract: EcucDestinationUriNestingContract, /)")]
    fn create_destination_uri_def(
        &self,
        name: &str,
        contract: EcucDestinationUriNestingContract,
    ) -> PyResult<EcucDestinationUriDef> {
        match self.0.create_destination_uri_def(name, contract.into()) {
            Ok(value) => Ok(EcucDestinationUriDef(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all destination uri definitions in the set
    fn destination_uri_defs(&self) -> EcucDestinationUriDefIterator {
        EcucDestinationUriDefIterator::new(self.0.destination_uri_defs().map(EcucDestinationUriDef))
    }
}

//##################################################################

/// A `EcucDestinationUriDef` defines a target for an `EcucUriReferenceDef`
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._ecu_configuration"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct EcucDestinationUriDef(
    pub(crate) autosar_data_abstraction::ecu_configuration::EcucDestinationUriDef,
);

#[pymethods]
impl EcucDestinationUriDef {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::ecu_configuration::EcucDestinationUriDef::try_from(
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

    /// set the nesting contract for the destination uri
    #[setter]
    fn set_nesting_contract(&self, contract: EcucDestinationUriNestingContract) -> PyResult<()> {
        self.0
            .set_nesting_contract(contract.into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the nesting contract for the destination uri
    #[getter]
    fn nesting_contract(&self) -> Option<EcucDestinationUriNestingContract> {
        self.0
            .nesting_contract()
            .map(EcucDestinationUriNestingContract::from)
    }

    /// create an `EcucParamConfContainerDef` in the destination uri policy
    #[pyo3(signature = (name, /))]
    #[pyo3(text_signature = "(self, name: str, /)")]
    fn create_param_conf_container_def(&self, name: &str) -> PyResult<EcucParamConfContainerDef> {
        match self.0.create_param_conf_container_def(name) {
            Ok(value) => Ok(EcucParamConfContainerDef(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create an `EcucChoiceContainerDef` in the destination uri policy
    #[pyo3(signature = (name, /))]
    #[pyo3(text_signature = "(self, name: str, /)")]
    fn create_choice_container_def(&self, name: &str) -> PyResult<EcucChoiceContainerDef> {
        match self.0.create_choice_container_def(name) {
            Ok(value) => Ok(EcucChoiceContainerDef(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all containers in the destination uri policy
    fn containers(&self) -> EcucContainerDefIterator {
        EcucContainerDefIterator::new(
            self.0
                .containers()
                .filter_map(|container| ecuc_container_def_to_pyany(container).ok()),
        )
    }
}

//#########################################################

iterator_wrapper!(EcucDestinationUriDefIterator, EcucDestinationUriDef);

//#########################################################

/// `EcucDestinationUriNestingContract` provides the different nesting contracts for destination URIs
#[allow(clippy::enum_variant_names)] // naming is consistent with the AUTOSAR standard
#[pyclass(
    frozen,
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._ecu_configuration"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EcucDestinationUriNestingContract {
    /// EcucDestinationUriPolicy describes elements (subContainers, Parameters, References) that are directly owned by the target container.
    LeafOfTargetContainer,
    /// EcucDestinationUriPolicy describes the target container of EcucUriReferenceDef.
    TargetContainer,
    /// EcucDestinationUriPolicy describes elements (subContainers, Parameters, References) that are owned by the target container or its subContainers.
    VertexOfTargetContainer,
}

impl From<EcucDestinationUriNestingContract>
    for autosar_data_abstraction::ecu_configuration::EcucDestinationUriNestingContract
{
    fn from(contract: EcucDestinationUriNestingContract) -> Self {
        match contract {
            EcucDestinationUriNestingContract::LeafOfTargetContainer => Self::LeafOfTargetContainer,
            EcucDestinationUriNestingContract::TargetContainer => Self::TargetContainer,
            EcucDestinationUriNestingContract::VertexOfTargetContainer => {
                Self::VertexOfTargetContainer
            }
        }
    }
}

impl From<autosar_data_abstraction::ecu_configuration::EcucDestinationUriNestingContract>
    for EcucDestinationUriNestingContract
{
    fn from(
        contract: autosar_data_abstraction::ecu_configuration::EcucDestinationUriNestingContract,
    ) -> Self {
        use autosar_data_abstraction::ecu_configuration::EcucDestinationUriNestingContract as In;
        match contract {
            In::LeafOfTargetContainer => EcucDestinationUriNestingContract::LeafOfTargetContainer,
            In::TargetContainer => EcucDestinationUriNestingContract::TargetContainer,
            In::VertexOfTargetContainer => {
                EcucDestinationUriNestingContract::VertexOfTargetContainer
            }
        }
    }
}
