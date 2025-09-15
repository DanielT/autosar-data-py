use crate::{
    Element,
    abstraction::{
        AutosarAbstractionError, abstraction_err_to_pyerr,
        ecu_configuration::{
            EcucAddInfoParamDef, EcucAnyReferenceDefIterator, EcucBooleanParamDef,
            EcucChoiceReferenceDef, EcucEnumerationParamDef, EcucFloatParamDef,
            EcucForeignReferenceDef, EcucFunctionNameDef, EcucInstanceReferenceDef,
            EcucIntegerParamDef, EcucLinkerSymbolDef, EcucMultilineStringParamDef,
            EcucParameterDefIterator, EcucReferenceDef, EcucStringParamDef, EcucUriReferenceDef,
            ecuc_parameter_def_to_pyany, ecuc_reference_def_to_pyany,
        },
    },
    iterator_wrapper,
};
use autosar_data_abstraction::{
    self, AbstractionElement, IdentifiableAbstractionElement,
    ecu_configuration::EcucDefinitionElement,
};
use pyo3::{IntoPyObjectExt, prelude::*};

//##################################################################

/// Marker trait for container definitions
/// The `EcucChoiceContainerDef` is used to define configuration containers
/// that provide a choice between several EcucParamConfContainerDef
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._ecu_configuration"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct EcucChoiceContainerDef(
    pub(crate) autosar_data_abstraction::ecu_configuration::EcucChoiceContainerDef,
);

#[pymethods]
impl EcucChoiceContainerDef {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::ecu_configuration::EcucChoiceContainerDef::try_from(
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

    /// create a new `EcucParamConfContainerDef` as one of the choices in this choice container
    #[pyo3(signature = (name, /))]
    #[pyo3(text_signature = "(self, name: str, /)")]
    fn create_param_conf_container_def(&self, name: &str) -> PyResult<EcucParamConfContainerDef> {
        match self.0.create_param_conf_container_def(name) {
            Ok(value) => Ok(EcucParamConfContainerDef(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over the choices in the container
    fn choices(&self) -> EcucParamConfContainerDefIterator {
        EcucParamConfContainerDefIterator::new(self.0.choices().map(EcucParamConfContainerDef))
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

/// The `EcucParamConfContainerDef` is used to define configuration containers
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._ecu_configuration"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct EcucParamConfContainerDef(
    pub(crate) autosar_data_abstraction::ecu_configuration::EcucParamConfContainerDef,
);

#[pymethods]
impl EcucParamConfContainerDef {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::ecu_configuration::EcucParamConfContainerDef::try_from(
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

    /// create a new `EcucChoiceContainerDef` as a sub-container
    #[pyo3(signature = (name, /))]
    #[pyo3(text_signature = "(self, name: str, /)")]
    fn create_choice_container_def(&self, name: &str) -> PyResult<EcucChoiceContainerDef> {
        match self.0.create_choice_container_def(name) {
            Ok(value) => Ok(EcucChoiceContainerDef(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `EcucParamConfContainerDef` as a sub-container
    #[pyo3(signature = (name, /))]
    #[pyo3(text_signature = "(self, name: str, /)")]
    fn create_param_conf_container_def(&self, name: &str) -> PyResult<EcucParamConfContainerDef> {
        match self.0.create_param_conf_container_def(name) {
            Ok(value) => Ok(EcucParamConfContainerDef(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over the sub-containers
    fn sub_containers(&self) -> EcucContainerDefIterator {
        EcucContainerDefIterator::new(
            self.0
                .sub_containers()
                .filter_map(|container| ecuc_container_def_to_pyany(container).ok()),
        )
    }

    /// create a new EcucAddInfoParamDef in the container
    #[pyo3(signature = (name, origin, /))]
    #[pyo3(text_signature = "(self, name: str, origin: str, /)")]
    fn create_add_info_param_def(&self, name: &str, origin: &str) -> PyResult<EcucAddInfoParamDef> {
        match self.0.create_add_info_param_def(name, origin) {
            Ok(value) => Ok(EcucAddInfoParamDef(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new EcucBooleanParamDef in the container
    #[pyo3(signature = (name, origin, /))]
    #[pyo3(text_signature = "(self, name: str, origin: str, /)")]
    fn create_boolean_param_def(&self, name: &str, origin: &str) -> PyResult<EcucBooleanParamDef> {
        match self.0.create_boolean_param_def(name, origin) {
            Ok(value) => Ok(EcucBooleanParamDef(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new EcucEnumerationParamDef in the container
    #[pyo3(signature = (name, origin, /))]
    #[pyo3(text_signature = "(self, name: str, origin: str, /)")]
    fn create_enumeration_param_def(
        &self,
        name: &str,
        origin: &str,
    ) -> PyResult<EcucEnumerationParamDef> {
        match self.0.create_enumeration_param_def(name, origin) {
            Ok(value) => Ok(EcucEnumerationParamDef(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new EcucFloatParamDef in the container
    #[pyo3(signature = (name, origin, /))]
    #[pyo3(text_signature = "(self, name: str, origin: str, /)")]
    fn create_float_param_def(&self, name: &str, origin: &str) -> PyResult<EcucFloatParamDef> {
        match self.0.create_float_param_def(name, origin) {
            Ok(value) => Ok(EcucFloatParamDef(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new EcucIntegerParamDef in the container
    #[pyo3(signature = (name, origin, /))]
    #[pyo3(text_signature = "(self, name: str, origin: str, /)")]
    fn create_integer_param_def(&self, name: &str, origin: &str) -> PyResult<EcucIntegerParamDef> {
        match self.0.create_integer_param_def(name, origin) {
            Ok(value) => Ok(EcucIntegerParamDef(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new EcucFunctionNameDef in the container
    #[pyo3(signature = (name, origin, /))]
    #[pyo3(text_signature = "(self, name: str, origin: str, /)")]
    fn create_function_name_param_def(
        &self,
        name: &str,
        origin: &str,
    ) -> PyResult<EcucFunctionNameDef> {
        match self.0.create_function_name_param_def(name, origin) {
            Ok(value) => Ok(EcucFunctionNameDef(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new EcucLinkerSymbolDef in the container
    #[pyo3(signature = (name, origin, /))]
    #[pyo3(text_signature = "(self, name: str, origin: str, /)")]
    fn create_linker_symbol_param_def(
        &self,
        name: &str,
        origin: &str,
    ) -> PyResult<EcucLinkerSymbolDef> {
        match self.0.create_linker_symbol_param_def(name, origin) {
            Ok(value) => Ok(EcucLinkerSymbolDef(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new EcucMultilineStringParamDef in the container
    #[pyo3(signature = (name, origin, /))]
    #[pyo3(text_signature = "(self, name: str, origin: str, /)")]
    fn create_multiline_string_param_def(
        &self,
        name: &str,
        origin: &str,
    ) -> PyResult<EcucMultilineStringParamDef> {
        match self.0.create_multiline_string_param_def(name, origin) {
            Ok(value) => Ok(EcucMultilineStringParamDef(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new EcucStringParamDef in the container
    #[pyo3(signature = (name, origin, /))]
    #[pyo3(text_signature = "(self, name: str, origin: str, /)")]
    fn create_string_param_def(&self, name: &str, origin: &str) -> PyResult<EcucStringParamDef> {
        match self.0.create_string_param_def(name, origin) {
            Ok(value) => Ok(EcucStringParamDef(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// get the parameters in the container
    fn parameters(&self) -> EcucParameterDefIterator {
        EcucParameterDefIterator::new(
            self.0
                .parameters()
                .filter_map(|value| ecuc_parameter_def_to_pyany(value).ok()),
        )
    }

    /// create a new EcucForeignReferenceDef in the container
    #[pyo3(signature = (name, origin, /))]
    #[pyo3(text_signature = "(self, name: str, origin: str, /)")]
    fn create_foreign_reference_def(
        &self,
        name: &str,
        origin: &str,
    ) -> PyResult<EcucForeignReferenceDef> {
        match self.0.create_foreign_reference_def(name, origin) {
            Ok(value) => Ok(EcucForeignReferenceDef(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new EcucInstanceReferenceDef in the container
    #[pyo3(signature = (name, origin, /))]
    #[pyo3(text_signature = "(self, name: str, origin: str, /)")]
    fn create_instance_reference_def(
        &self,
        name: &str,
        origin: &str,
    ) -> PyResult<EcucInstanceReferenceDef> {
        match self.0.create_instance_reference_def(name, origin) {
            Ok(value) => Ok(EcucInstanceReferenceDef(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new EcucChoiceReferenceDef in the container
    #[pyo3(signature = (name, origin, /))]
    #[pyo3(text_signature = "(self, name: str, origin: str, /)")]
    fn create_choice_reference_def(
        &self,
        name: &str,
        origin: &str,
    ) -> PyResult<EcucChoiceReferenceDef> {
        match self.0.create_choice_reference_def(name, origin) {
            Ok(value) => Ok(EcucChoiceReferenceDef(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new EcucReferenceDef in the container
    #[pyo3(signature = (name, origin, /))]
    #[pyo3(text_signature = "(self, name: str, origin: str, /)")]
    fn create_reference_def(&self, name: &str, origin: &str) -> PyResult<EcucReferenceDef> {
        match self.0.create_reference_def(name, origin) {
            Ok(value) => Ok(EcucReferenceDef(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new EcucUriReferenceDef in the container
    #[pyo3(signature = (name, origin, /))]
    #[pyo3(text_signature = "(self, name: str, origin: str, /)")]
    fn create_uri_reference_def(&self, name: &str, origin: &str) -> PyResult<EcucUriReferenceDef> {
        match self.0.create_uri_reference_def(name, origin) {
            Ok(value) => Ok(EcucUriReferenceDef(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// get the references in the container
    fn references(&self) -> EcucAnyReferenceDefIterator {
        EcucAnyReferenceDefIterator::new(
            self.0
                .references()
                .filter_map(|value| ecuc_reference_def_to_pyany(value).ok()),
        )
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

iterator_wrapper!(EcucParamConfContainerDefIterator, EcucParamConfContainerDef);
iterator_wrapper!(
    EcucContainerDefIterator,
    Py<PyAny>,
    "Union[EcucChoiceContainerDef, EcucParamConfContainerDef]"
);

//##################################################################

pub(crate) fn ecuc_container_def_to_pyany(
    container: autosar_data_abstraction::ecu_configuration::EcucContainerDef,
) -> PyResult<Py<PyAny>> {
    Python::attach(|py| match container {
        autosar_data_abstraction::ecu_configuration::EcucContainerDef::Choice(container) => {
            EcucChoiceContainerDef(container).into_py_any(py)
        }
        autosar_data_abstraction::ecu_configuration::EcucContainerDef::ParamConf(container) => {
            EcucParamConfContainerDef(container).into_py_any(py)
        }
    })
}

pub(crate) fn ecuc_container_def_from_pyany(
    py_container: &Bound<'_, PyAny>,
) -> PyResult<autosar_data_abstraction::ecu_configuration::EcucContainerDef> {
    if let Ok(container) = py_container.extract::<EcucChoiceContainerDef>() {
        Ok(autosar_data_abstraction::ecu_configuration::EcucContainerDef::Choice(container.0))
    } else if let Ok(container) = py_container.extract::<EcucParamConfContainerDef>() {
        Ok(autosar_data_abstraction::ecu_configuration::EcucContainerDef::ParamConf(container.0))
    } else {
        Err(AutosarAbstractionError::new_err(format!(
            "Invalid container type: {py_container:?}"
        )))
    }
}
