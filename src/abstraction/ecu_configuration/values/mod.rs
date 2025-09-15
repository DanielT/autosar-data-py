use crate::{
    Element,
    abstraction::{
        AutosarAbstractionError, System, abstraction_err_to_pyerr,
        ecu_configuration::{
            EcucAddInfoParamDef, EcucInstanceReferenceDef, EcucModuleDef,
            ecuc_container_def_from_pyany, ecuc_container_def_to_pyany,
            pyany_to_ecuc_parameter_def, pyany_to_ecuc_reference_def,
        },
    },
    iterator_wrapper,
};
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};
use pyo3::prelude::*;

mod parameter;
mod reference;

pub(crate) use parameter::*;
pub(crate) use reference::*;

//##################################################################

/// `EcucValueCollection` collects references to all the separate modules that form the ECU configuration
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._ecu_configuration"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct EcucValueCollection(
    pub(crate) autosar_data_abstraction::ecu_configuration::EcucValueCollection,
);

#[pymethods]
impl EcucValueCollection {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::ecu_configuration::EcucValueCollection::try_from(
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

    /// Add a reference to a module configuration to the collection
    #[pyo3(signature = (module_configuration, /))]
    #[pyo3(text_signature = "(self, module_configuration: EcucModuleConfigurationValues)")]
    fn add_module_configuration(
        &self,
        module_configuration: &EcucModuleConfigurationValues,
    ) -> PyResult<()> {
        self.0
            .add_module_configuration(&module_configuration.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the module configurations in the collection
    fn module_configurations(&self) -> EcucModuleConfigurationValuesIterator {
        EcucModuleConfigurationValuesIterator::new(
            self.0
                .module_configurations()
                .map(EcucModuleConfigurationValues),
        )
    }

    /// Set the ecu extract reference, which links a `System` to the ECU configuration
    #[setter]
    fn set_ecu_extract_reference(&self, system: &System) -> PyResult<()> {
        self.0
            .set_ecu_extract_reference(&system.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the system that the ECU configuration is linked to
    #[getter]
    fn ecu_extract_reference(&self) -> Option<System> {
        self.0.ecu_extract_reference().map(System)
    }
}

//##################################################################

/// The `EcucModuleConfigurationValues` is a container for the configuration of a single base software module
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._ecu_configuration"
)]
#[derive(Clone, PartialEq)]

pub(crate) struct EcucModuleConfigurationValues(
    pub(crate) autosar_data_abstraction::ecu_configuration::EcucModuleConfigurationValues,
);

#[pymethods]
impl EcucModuleConfigurationValues {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::ecu_configuration::EcucModuleConfigurationValues::try_from(
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

    /// set the module definition reference
    #[setter]
    fn set_definition(&self, module_definition: &EcucModuleDef) -> PyResult<()> {
        self.0
            .set_definition(&module_definition.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the module definition
    ///
    /// This function returns the definition as an `EcucModuleDef` object.
    /// If the definition is not loaded, use `definition_ref()` instead.
    #[getter]
    fn definition(&self) -> Option<EcucModuleDef> {
        self.0.definition().map(EcucModuleDef)
    }

    /// get the definition reference as a string
    ///
    /// This function is an alternative to `definition()`; it is useful when the
    /// referenced definition is not loaded and can't be resolved.
    #[getter]
    fn definition_ref(&self) -> Option<String> {
        self.0.definition_ref()
    }

    /// Create a new `EcucContainerValue` in the module configuration
    #[pyo3(signature = (name, definition, /))]
    #[pyo3(text_signature = "(self, name: str, definition: EcucContainerDef, /)")]
    fn create_container_value(
        &self,
        name: &str,
        definition: &Bound<'_, PyAny>,
    ) -> PyResult<EcucContainerValue> {
        let definition = ecuc_container_def_from_pyany(definition)?;
        match self.0.create_container_value(name, &definition) {
            Ok(value) => Ok(EcucContainerValue(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create an iterator over the container values in the module configuration
    fn container_values(&self) -> EcucContainerValueIterator {
        EcucContainerValueIterator::new(self.0.container_values().map(EcucContainerValue))
    }
}

//##################################################################

iterator_wrapper!(
    EcucModuleConfigurationValuesIterator,
    EcucModuleConfigurationValues
);

//##################################################################

/// The `EcucContainerValue` is a container in the ECU configuration
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._ecu_configuration"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct EcucContainerValue(
    pub(crate) autosar_data_abstraction::ecu_configuration::EcucContainerValue,
);

#[pymethods]
impl EcucContainerValue {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::ecu_configuration::EcucContainerValue::try_from(
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

    /// set the container definition reference
    #[setter]
    fn set_definition(&self, definition: &Bound<'_, PyAny>) -> PyResult<()> {
        let definition = ecuc_container_def_from_pyany(definition)?;
        self.0
            .set_definition(&definition)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the container definition
    ///
    /// This function returns the definition as an `EcucContainerDef` object.
    /// If the definition is not loaded, use `definition_ref()` instead.
    #[getter]
    fn definition(&self) -> Option<Py<PyAny>> {
        self.0
            .definition()
            .and_then(|def| ecuc_container_def_to_pyany(def).ok())
    }

    /// get the definition reference as a string
    ///
    /// This function is an alternative to `definition()`; it is useful when the
    /// referenced definition is not loaded and can't be resolved.
    #[getter]
    fn definition_ref(&self) -> Option<String> {
        self.0.definition_ref()
    }

    /// create a sub-container
    #[pyo3(signature = (name, definition, /))]
    #[pyo3(text_signature = "(self, name: str, definition: EcucContainerDef, /)")]
    fn create_sub_container(
        &self,
        name: &str,
        definition: &Bound<'_, PyAny>,
    ) -> PyResult<EcucContainerValue> {
        let definition = ecuc_container_def_from_pyany(definition)?;
        match self.0.create_sub_container(name, &definition) {
            Ok(value) => Ok(EcucContainerValue(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over the sub-containers in this container
    fn sub_containers(&self) -> EcucContainerValueIterator {
        EcucContainerValueIterator::new(self.0.sub_containers().map(EcucContainerValue))
    }

    /// set the index of the container
    ///
    /// If the container definition has `requiresIndex` set to `true`, then the container
    /// must have an index. Otherwise the index is meaningless.
    #[setter]
    fn set_index(&self, index: Option<u64>) -> PyResult<()> {
        self.0.set_index(index).map_err(abstraction_err_to_pyerr)
    }

    /// get the index of the container
    ///
    /// If the container definition has `requiresIndex` set to `true`, then the container
    /// must have an index. Otherwise the index is meaningless.
    #[getter]
    fn index(&self) -> Option<u64> {
        self.0.index()
    }

    /// create a new `EcucNumericalParamValue` in the container
    #[pyo3(signature = (definition, value, /))]
    #[pyo3(text_signature = "(self, definition: EcucParameterDef, value: str, /)")]
    fn create_numerical_param_value(
        &self,
        definition: &Bound<'_, PyAny>,
        value: &str,
    ) -> PyResult<EcucNumericalParamValue> {
        let definition = pyany_to_ecuc_parameter_def(definition)?;
        match self.0.create_numerical_param_value(&definition, value) {
            Ok(value) => Ok(EcucNumericalParamValue(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `EcucTextualParamValue` in the container
    #[pyo3(signature = (definition, value, /))]
    #[pyo3(text_signature = "(self, definition: EcucParameterDef, value: str, /)")]
    fn create_textual_param_value(
        &self,
        definition: &Bound<'_, PyAny>,
        value: &str,
    ) -> PyResult<EcucTextualParamValue> {
        let definition = pyany_to_ecuc_parameter_def(definition)?;
        match self.0.create_textual_param_value(&definition, value) {
            Ok(value) => Ok(EcucTextualParamValue(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `EcucAddInfoParamValue` in the container
    #[pyo3(signature = (definition, /))]
    #[pyo3(text_signature = "(self, definition: EcucAddInfoParameterDef, /)")]
    fn create_add_info_param_value(
        &self,
        definition: &EcucAddInfoParamDef,
    ) -> PyResult<EcucAddInfoParamValue> {
        match self.0.create_add_info_param_value(&definition.0) {
            Ok(value) => Ok(EcucAddInfoParamValue(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over the parameter values in the container
    fn parameter_values(&self) -> EcucParameterValueIterator {
        EcucParameterValueIterator::new(
            self.0
                .parameter_values()
                .filter_map(|val| ecuc_parameter_value_to_pyany(&val).ok()),
        )
    }

    /// create a new instance reference value in the container
    #[pyo3(signature = (definition, target_context, target, /))]
    #[pyo3(
        text_signature = "(self, definition: EcucInstanceReferenceDef, target_context: List[Element], target: Element, /)"
    )]
    fn create_instance_reference(
        &self,
        definition: &EcucInstanceReferenceDef,
        target_context: Vec<Element>,
        target: &Element,
    ) -> PyResult<EcucInstanceReferenceValue> {
        let target_context = target_context.iter().map(|e| &e.0).collect::<Vec<_>>();
        match self
            .0
            .create_instance_reference(&definition.0, &target_context, &target.0)
        {
            Ok(value) => Ok(EcucInstanceReferenceValue(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new reference value in the container
    #[pyo3(signature = (definition, target, /))]
    #[pyo3(text_signature = "(self, definition: EcucReferenceDef, target: Element, /)")]
    fn create_reference_value(
        &self,
        definition: &Bound<'_, PyAny>,
        target: &Element,
    ) -> PyResult<EcucReferenceValue> {
        let definition = pyany_to_ecuc_reference_def(definition)?;
        match self.0.create_reference_value(&definition, &target.0) {
            Ok(value) => Ok(EcucReferenceValue(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over the reference values in the container
    fn reference_values(&self) -> EcucAnyReferenceValueIterator {
        EcucAnyReferenceValueIterator::new(
            self.0
                .reference_values()
                .filter_map(|val| ecuc_reference_value_to_pyany(&val).ok()),
        )
    }
}

//##################################################################

iterator_wrapper!(EcucContainerValueIterator, EcucContainerValue);
