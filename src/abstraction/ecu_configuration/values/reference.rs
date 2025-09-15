use crate::{
    abstraction::{
        ecu_configuration::{
            EcucInstanceReferenceDef, ecuc_reference_def_to_pyany, pyany_to_ecuc_reference_def,
        },
        *,
    },
    *,
};
use autosar_data_abstraction::{self, AbstractionElement};

//##################################################################

/// An `EcucInstanceReferenceValue` provides the mechanism to reference an instance of a prototype
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._ecu_configuration"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct EcucInstanceReferenceValue(
    pub(crate) autosar_data_abstraction::ecu_configuration::EcucInstanceReferenceValue,
);

#[pymethods]
impl EcucInstanceReferenceValue {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::ecu_configuration::EcucInstanceReferenceValue::try_from(
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
    fn set_definition(&self, definition: &EcucInstanceReferenceDef) -> PyResult<()> {
        self.0
            .set_definition(&definition.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the parameter definition
    ///
    /// This function returns the definition as an `EcucParameterDef` enum, which
    /// could contain either an `EcucFloatParamDef` or an `EcucIntegerParamDef`.
    /// If the definition is not loaded, use `definition_ref()` instead.
    #[getter]
    fn definition(&self) -> Option<EcucInstanceReferenceDef> {
        self.0.definition().map(EcucInstanceReferenceDef)
    }

    /// get the parameter definition reference as a string
    ///
    /// This function is an alternative to `definition()`; it is useful when the
    /// referenced definition is not loaded and can't be resolved.
    #[getter]
    fn definition_ref(&self) -> Option<String> {
        self.0.definition_ref()
    }

    /// Set the target of the reference
    ///
    /// An instance reference targets a specific instance of a prototype. In order to uniquely identify the target,
    /// the target context is required. The target context is a list of elements that are the parent elements of the
    /// target element. The instance reference definition specifies which context elements are required.
    #[setter]
    fn set_target(&self, spec: (Vec<Element>, Element)) -> PyResult<()> {
        let target = spec.1;
        let target_context: Vec<_> = spec.0.iter().map(|e| &e.0).collect();
        self.0
            .set_target(&target_context, &target.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the target of the reference
    ///
    /// Returns the targt element of the instance reference, as well as the context elements that are needed to uniquely
    /// identify the target.
    #[getter]
    fn target(&self) -> Option<(Vec<Element>, Element)> {
        match self.0.target() {
            Some((context, target)) => {
                let context: Vec<Element> = context.into_iter().map(Element).collect();
                Some((context, Element(target.clone())))
            }
            None => None,
        }
    }

    /// set the index of the reference
    ///
    /// If the reference definition has `requiresIndex` set to `true`, then the reference
    /// must have an index. Otherwise the index is meaningless.
    #[setter]
    fn set_index(&self, index: Option<u64>) -> PyResult<()> {
        self.0.set_index(index).map_err(abstraction_err_to_pyerr)
    }

    /// get the index of the reference
    ///
    /// If the reference definition has `requiresIndex` set to `true`, then the reference
    /// must have an index. Otherwise the index is meaningless.
    #[getter]
    fn index(&self) -> Option<u64> {
        self.0.index()
    }

    /// set the isAutoValue flag
    ///
    /// If the reference definition has `withAuto` set to `true`, then the reference is allowed to have an auto value.
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

/// An `EcucReferenceValue` allows the ecu tonfiguration to refer to any identifiable element in the Autosar model
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._ecu_configuration"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct EcucReferenceValue(
    pub(crate) autosar_data_abstraction::ecu_configuration::EcucReferenceValue,
);

#[pymethods]
impl EcucReferenceValue {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::ecu_configuration::EcucReferenceValue::try_from(
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
        let definition = pyany_to_ecuc_reference_def(definition)?;
        self.0
            .set_definition(&definition)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the reference definition
    ///
    /// This function returns the definition as an `EcucParameterDef` enum, which
    /// could contain either an `EcucFloatParamDef` or an `EcucIntegerParamDef`.
    /// If the definition is not loaded, use `definition_ref()` instead.
    #[getter]
    fn definition(&self) -> Option<Py<PyAny>> {
        self.0
            .definition()
            .map(|value| ecuc_reference_def_to_pyany(value).unwrap())
    }

    /// get the referenced definition ref as a string
    ///
    /// This function is an alternative to `definition()`; it is useful when the
    /// referenced definition is not loaded and can't be resolved.
    #[getter]
    fn definition_ref(&self) -> Option<String> {
        self.0.definition_ref()
    }

    /// Set the target of the reference
    #[setter]
    fn set_target(&self, target: &Element) -> PyResult<()> {
        self.0
            .set_target(&target.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the target of the reference
    #[getter]
    fn target(&self) -> Option<Element> {
        self.0.target().map(|value| Element(value.clone()))
    }

    /// set the index of the reference
    ///
    /// If the reference definition has `requiresIndex` set to `true`, then the reference
    /// must have an index. Otherwise the index is meaningless.
    #[setter]
    fn set_index(&self, index: Option<u64>) -> PyResult<()> {
        self.0.set_index(index).map_err(abstraction_err_to_pyerr)
    }

    /// get the index of the reference
    ///
    /// If the reference definition has `requiresIndex` set to `true`, then the reference
    /// must have an index. Otherwise the index is meaningless.
    #[getter]
    fn index(&self) -> Option<u64> {
        self.0.index()
    }

    /// set the isAutoValue flag
    ///
    /// If the reference definition has `withAuto` set to `true`, then the reference is allowed to have an auto value.
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

iterator_wrapper!(
    EcucAnyReferenceValueIterator,
    Py<PyAny>,
    "Union[EcucInstanceReferenceValue, EcucReferenceValue]"
);

//##################################################################

pub(crate) fn ecuc_reference_value_to_pyany(
    value: &autosar_data_abstraction::ecu_configuration::EcucAnyReferenceValue,
) -> PyResult<Py<PyAny>> {
    Python::attach(|py| match value {
        autosar_data_abstraction::ecu_configuration::EcucAnyReferenceValue::Instance(value) => {
            EcucInstanceReferenceValue(value.clone()).into_py_any(py)
        }
        autosar_data_abstraction::ecu_configuration::EcucAnyReferenceValue::Reference(value) => {
            EcucReferenceValue(value.clone()).into_py_any(py)
        }
    })
}
