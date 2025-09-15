use crate::{
    abstraction::{
        ecu_configuration::{ecuc_parameter_def_to_pyany, pyany_to_ecuc_parameter_def},
        *,
    },
    *,
};
use autosar_data_abstraction::{self, AbstractionElement};

//##################################################################

/// The `EcucAddInfoParamValue` holds descriptive text and takes the role of a parameter in the ECU configuration
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._ecu_configuration"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct EcucAddInfoParamValue(
    pub(crate) autosar_data_abstraction::ecu_configuration::EcucAddInfoParamValue,
);

#[pymethods]
impl EcucAddInfoParamValue {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::ecu_configuration::EcucAddInfoParamValue::try_from(
            element.0.clone(),
        ) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
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

/// The `EcucNumericalParamValue` holds a numerical value and can represent boolean, float or int parameter definitions.
///
/// Internally this value is stored as a string; in additon to the value() function, there are also
/// value_bool(), value_int() and value_float() functions, which parse the string and should be used as appropriate.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._ecu_configuration"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct EcucNumericalParamValue(
    pub(crate) autosar_data_abstraction::ecu_configuration::EcucNumericalParamValue,
);

#[pymethods]
impl EcucNumericalParamValue {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::ecu_configuration::EcucNumericalParamValue::try_from(
            element.0.clone(),
        ) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    #[getter]
    fn element(&self) -> Element {
        Element(self.0.element().clone())
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self.0)
    }

    /// set the parameter definition reference
    #[setter]
    fn set_definition(&self, definition: &Bound<'_, PyAny>) -> PyResult<()> {
        let definition = pyany_to_ecuc_parameter_def(definition)?;
        self.0
            .set_definition(&definition)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the parameter definition
    ///
    /// This function returns the definition as an `EcucParameterDef` enum, which
    /// could contain either an `EcucFloatParamDef` or an `EcucIntegerParamDef`.
    /// If the definition is not loaded, use `definition_ref()` instead.
    #[getter]
    fn definition(&self) -> Option<Py<PyAny>> {
        self.0
            .definition()
            .and_then(|def| ecuc_parameter_def_to_pyany(def).ok())
    }

    /// get the parameter definition reference as a string
    ///
    /// This function is an alternative to `definition()`; it is useful when the
    /// referenced definition is not loaded and can't be resolved.
    #[getter]
    fn definition_ref(&self) -> Option<String> {
        self.0.definition_ref()
    }

    /// set the numerical value as a string
    #[setter]
    fn set_value(&self, value: &str) -> PyResult<()> {
        self.0.set_value(value).map_err(abstraction_err_to_pyerr)
    }

    /// get the numerical value as a string
    #[getter]
    fn value(&self) -> Option<String> {
        self.0.value()
    }

    /// get the numerical value as a boolean
    #[getter]
    fn value_bool(&self) -> Option<bool> {
        self.0.value_bool()
    }

    /// get the numerical value as an integer
    #[getter]
    fn value_int(&self) -> Option<i64> {
        self.0.value_int()
    }

    /// get the numerical value as a float
    #[getter]
    fn value_float(&self) -> Option<f64> {
        self.0.value_float()
    }

    /// set the index of the parameter
    ///
    /// If the parameter definition has `requiresIndex` set to `true`, then the parameter
    /// must have an index. Otherwise the index is meaningless.
    #[setter]
    fn set_index(&self, index: Option<u64>) -> PyResult<()> {
        self.0.set_index(index).map_err(abstraction_err_to_pyerr)
    }

    /// get the index of the parameter
    ///
    /// If the parameter definition has `requiresIndex` set to `true`, then the parameter
    /// must have an index. Otherwise the index is meaningless.
    #[getter]
    fn index(&self) -> Option<u64> {
        self.0.index()
    }

    /// set the isAutoValue flag
    ///
    /// If the parameter definition has `withAuto` set to `true`, then the parameter is allowed to have an auto value.
    #[setter]
    fn set_is_auto_value(&self, is_auto_value: Option<bool>) -> PyResult<()> {
        self.0
            .set_is_auto_value(is_auto_value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the isAutoValue flag
    #[getter]
    fn is_auto_value(&self) -> Option<bool> {
        self.0.is_auto_value()
    }
}

//##################################################################

/// The `EcucTextualParamValue` holds a string value and can represent a enumeration,
///  string, multi-line string, function name or linker symbol parameter definition.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._ecu_configuration"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct EcucTextualParamValue(
    pub(crate) autosar_data_abstraction::ecu_configuration::EcucTextualParamValue,
);

#[pymethods]
impl EcucTextualParamValue {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::ecu_configuration::EcucTextualParamValue::try_from(
            element.0.clone(),
        ) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    #[getter]
    fn element(&self) -> Element {
        Element(self.0.element().clone())
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self.0)
    }

    /// set the parameter definition reference
    #[setter]
    fn set_definition(&self, definition: &Bound<'_, PyAny>) -> PyResult<()> {
        let definition = pyany_to_ecuc_parameter_def(definition)?;
        self.0
            .set_definition(&definition)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the parameter definition
    ///
    /// This function returns the definition as an `EcucParameterDef` enum, which
    /// could contain either an `EcucStringParamDef`, `EcucMultiStringParamDef`,
    /// `EcucFunctionNameDef` or `EcucLinkerSymbolDef`.
    /// If the definition is not loaded, use `definition_ref()` instead.
    #[getter]
    fn definition(&self) -> Option<Py<PyAny>> {
        self.0
            .definition()
            .and_then(|def| ecuc_parameter_def_to_pyany(def).ok())
    }

    /// get the parameter definition reference as a string
    ///
    /// This function is an alternative to `definition()`; it is useful when the
    /// referenced definition is not loaded and can't be resolved.
    #[getter]
    fn definition_ref(&self) -> Option<String> {
        self.0.definition_ref()
    }

    /// set the textual value
    #[setter]
    fn set_value(&self, value: &str) -> PyResult<()> {
        self.0.set_value(value).map_err(abstraction_err_to_pyerr)
    }

    /// get the textual value
    #[getter]
    fn value(&self) -> Option<String> {
        self.0.value()
    }

    /// set the index of the parameter
    ///
    /// If the parameter definition has `requiresIndex` set to `true`, then the parameter
    /// must have an index. Otherwise the index is meaningless.
    #[setter]
    fn set_index(&self, index: Option<u64>) -> PyResult<()> {
        self.0.set_index(index).map_err(abstraction_err_to_pyerr)
    }

    /// get the index of the parameter
    ///
    /// If the parameter definition has `requiresIndex` set to `true`, then the parameter
    /// must have an index. Otherwise the index is meaningless.
    #[getter]
    fn index(&self) -> Option<u64> {
        self.0.index()
    }

    /// set the isAutoValue flag
    ///
    /// If the parameter definition has `withAuto` set to `true`, then the parameter is allowed to have an auto value.
    #[setter]
    fn set_is_auto_value(&self, is_auto_value: Option<bool>) -> PyResult<()> {
        self.0
            .set_is_auto_value(is_auto_value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the isAutoValue flag
    ///
    /// If the parameter definition has `withAuto` set to `true`, then the parameter is allowed to have an auto value.
    #[getter]
    fn is_auto_value(&self) -> Option<bool> {
        self.0.is_auto_value()
    }
}

//##################################################################

iterator_wrapper!(EcucParameterValueIterator, Py<PyAny>, "EcucParameterValue");

//##################################################################

pub(crate) fn ecuc_parameter_value_to_pyany(
    value: &autosar_data_abstraction::ecu_configuration::EcucParameterValue,
) -> PyResult<Py<PyAny>> {
    Python::attach(|py| match value {
        autosar_data_abstraction::ecu_configuration::EcucParameterValue::AddInfo(value) => {
            EcucAddInfoParamValue(value.clone()).into_py_any(py)
        }
        autosar_data_abstraction::ecu_configuration::EcucParameterValue::Numerical(value) => {
            EcucNumericalParamValue(value.clone()).into_py_any(py)
        }
        autosar_data_abstraction::ecu_configuration::EcucParameterValue::Textual(value) => {
            EcucTextualParamValue(value.clone()).into_py_any(py)
        }
    })
}
