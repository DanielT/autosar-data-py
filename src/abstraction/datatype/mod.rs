use crate::{
    Element,
    abstraction::{AutosarAbstractionError, abstraction_err_to_pyerr},
    iterator_wrapper,
};
use autosar_data_abstraction::{AbstractionElement, IdentifiableAbstractionElement};
use pyo3::{IntoPyObjectExt, prelude::*};

mod applicationtype;
mod basetype;
mod compu_method;
mod implementationtype;
mod mapping;
mod values;

pub(crate) use applicationtype::*;
pub(crate) use basetype::*;
pub(crate) use compu_method::*;
pub(crate) use implementationtype::*;
pub(crate) use mapping::*;
pub(crate) use values::*;

//##################################################################

/// `Unit` represents a unit of measurement.
///
/// Use [`ArPackage::create_unit`] to create a new unit.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._datatype"
)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Unit(pub(crate) autosar_data_abstraction::datatype::Unit);

#[pymethods]
impl Unit {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::datatype::Unit::try_from(element.0.clone()) {
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

    /// set the display name of the unit
    #[setter]
    fn set_display_name(&self, display_name: Option<&str>) -> PyResult<()> {
        self.0
            .set_display_name(display_name)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the display name of the unit
    #[getter]
    fn display_name(&self) -> Option<String> {
        self.0.display_name()
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

/// `DataConstr` represents a data constraint.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._datatype"
)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct DataConstr(pub(crate) autosar_data_abstraction::datatype::DataConstr);

#[pymethods]
impl DataConstr {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::datatype::DataConstr::try_from(element.0.clone()) {
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

    /// Create a data constraint rule
    #[pyo3(signature = (rule_type, /, *, lower_limit=None, upper_limit=None))]
    #[pyo3(
        text_signature = "(self, rule_type: DataConstrType, /, *, lower_limit: Optional[float] = None, upper_limit: Optional[float] = None)"
    )]
    fn create_data_constr_rule(
        &self,
        rule_type: DataConstrType,
        lower_limit: Option<f64>,
        upper_limit: Option<f64>,
    ) -> PyResult<DataConstrRule> {
        match self
            .0
            .create_data_constr_rule(rule_type.into(), lower_limit, upper_limit)
        {
            Ok(value) => Ok(DataConstrRule(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// Get all data constraint rules
    fn data_constr_rules(&self) -> DataConstrRuleIterator {
        DataConstrRuleIterator::new(self.0.data_constr_rules().map(DataConstrRule))
    }
}

//##################################################################

/// `DataConstrRule` represents a data constraint rule.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._datatype"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct DataConstrRule(pub(crate) autosar_data_abstraction::datatype::DataConstrRule);

#[pymethods]
impl DataConstrRule {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::datatype::DataConstrRule::try_from(element.0.clone()) {
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

    /// get the constraint type
    #[getter]
    fn rule_type(&self) -> DataConstrType {
        self.0.rule_type().into()
    }

    /// get the lower limit
    #[getter]
    fn lower_limit(&self) -> Option<f64> {
        self.0.lower_limit()
    }

    /// get the upper limit
    #[getter]
    fn upper_limit(&self) -> Option<f64> {
        self.0.upper_limit()
    }
}

//##################################################################

iterator_wrapper!(DataConstrRuleIterator, DataConstrRule);

//##################################################################

/// The type of a data constraint rule
#[pyclass(
    frozen,
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._datatype"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataConstrType {
    /// Internal value data constraint
    Internal,
    /// Physical value data constraint
    Physical,
}

impl From<autosar_data_abstraction::datatype::DataConstrType> for DataConstrType {
    fn from(value: autosar_data_abstraction::datatype::DataConstrType) -> Self {
        match value {
            autosar_data_abstraction::datatype::DataConstrType::Internal => Self::Internal,
            autosar_data_abstraction::datatype::DataConstrType::Physical => Self::Physical,
        }
    }
}

impl From<DataConstrType> for autosar_data_abstraction::datatype::DataConstrType {
    fn from(value: DataConstrType) -> Self {
        match value {
            DataConstrType::Internal => Self::Internal,
            DataConstrType::Physical => Self::Physical,
        }
    }
}

//##################################################################

pub(crate) fn autosar_data_type_to_pyany(
    value: autosar_data_abstraction::datatype::AutosarDataType,
) -> PyResult<Py<PyAny>> {
    Python::attach(|py| match value {
        autosar_data_abstraction::datatype::AutosarDataType::ApplicationArrayDataType(value) => {
            ApplicationArrayDataType(value).into_py_any(py)
        }
        autosar_data_abstraction::datatype::AutosarDataType::ApplicationPrimitiveDataType(
            value,
        ) => ApplicationPrimitiveDataType(value).into_py_any(py),
        autosar_data_abstraction::datatype::AutosarDataType::ApplicationRecordDataType(value) => {
            ApplicationRecordDataType(value).into_py_any(py)
        }
        autosar_data_abstraction::datatype::AutosarDataType::ImplementationDataType(value) => {
            ImplementationDataType(value).into_py_any(py)
        }
    })
}

pub(crate) fn pyany_to_autosar_data_type(
    pyobject: &Bound<'_, PyAny>,
) -> PyResult<autosar_data_abstraction::datatype::AutosarDataType> {
    if let Ok(application_array_data_type) = pyobject.extract::<ApplicationArrayDataType>() {
        Ok(
            autosar_data_abstraction::datatype::AutosarDataType::ApplicationArrayDataType(
                application_array_data_type.0,
            ),
        )
    } else if let Ok(application_primitive_data_type) =
        pyobject.extract::<ApplicationPrimitiveDataType>()
    {
        Ok(
            autosar_data_abstraction::datatype::AutosarDataType::ApplicationPrimitiveDataType(
                application_primitive_data_type.0,
            ),
        )
    } else if let Ok(application_record_data_type) = pyobject.extract::<ApplicationRecordDataType>()
    {
        Ok(
            autosar_data_abstraction::datatype::AutosarDataType::ApplicationRecordDataType(
                application_record_data_type.0,
            ),
        )
    } else if let Ok(implementation_data_type) = pyobject.extract::<ImplementationDataType>() {
        Ok(
            autosar_data_abstraction::datatype::AutosarDataType::ImplementationDataType(
                implementation_data_type.0,
            ),
        )
    } else {
        Err(AutosarAbstractionError::new_err(
            "Invalid data type".to_string(),
        ))
    }
}
