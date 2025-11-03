use crate::{
    Element,
    abstraction::{
        AutosarAbstractionError, abstraction_err_to_pyerr,
        datatype::{
            ApplicationArrayElement, ApplicationPrimitiveCategory, ApplicationRecordElement, Unit,
        },
        software_component::{
            ArgumentDataPrototype, ParameterDataPrototype, VariableDataPrototype,
        },
    },
    pyutils::{compare_pylist, pylist_to_vec, slice_to_pylist},
};
use autosar_data_abstraction::{AbstractionElement, IdentifiableAbstractionElement};
use pyo3::{
    IntoPyObjectExt,
    prelude::*,
    types::{PyList, PyTuple},
};

//##################################################################

/// `ConstantSpecification` is a specification of a constant that can be part of a package, i.e. it can be defined stand-alone.
/// These constant values can be referenced from value specifications.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._datatype"
)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ConstantSpecification(
    pub(crate) autosar_data_abstraction::datatype::ConstantSpecification,
);

#[pymethods]
impl ConstantSpecification {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::datatype::ConstantSpecification::try_from(element.0.clone())
        {
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

    /// set the value of the constant
    #[setter]
    fn set_value_specification(&self, value: &Bound<'_, PyAny>) -> PyResult<()> {
        let value_specification = pyany_to_value_specification(value)?;
        self.0
            .set_value_specification(value_specification)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the value of the constant
    #[getter]
    fn value_specification(&self) -> Option<Py<PyAny>> {
        self.0
            .value_specification()
            .and_then(|value_spec| value_specification_to_pyany(&value_spec).ok())
    }
}

//#########################################################

pub(crate) fn value_specification_to_pyany(
    value: &autosar_data_abstraction::datatype::ValueSpecification,
) -> PyResult<Py<PyAny>> {
    use autosar_data_abstraction::datatype::ValueSpecification;
    Python::attach(|py| match value {
        ValueSpecification::Array(value) => {
            ArrayValueSpecification::try_from(value)?.into_py_any(py)
        }
        ValueSpecification::Record(value) => {
            RecordValueSpecification::try_from(value)?.into_py_any(py)
        }
        ValueSpecification::Text(value) => TextValueSpecification::from(value).into_py_any(py),
        ValueSpecification::Numerical(value) => {
            NumericalValueSpecification::from(value).into_py_any(py)
        }
        ValueSpecification::ConstantReference(value) => {
            ConstantReference::from(value).into_py_any(py)
        }
        ValueSpecification::Application(value) => {
            ApplicationValueSpecification::try_from(value)?.into_py_any(py)
        }
        ValueSpecification::NotAvailable(value) => {
            NotAvailableValueSpecification::from(value).into_py_any(py)
        }
        ValueSpecification::Reference(value) => {
            ReferenceValueSpecification::try_from(value)?.into_py_any(py)
        }
        ValueSpecification::ApplicationRuleBased(value) => {
            ApplicationRuleBasedValueSpecification::try_from(value)?.into_py_any(py)
        }
        ValueSpecification::CompositeRuleBased(value) => {
            CompositeRuleBasedValueSpecification::try_from(value)?.into_py_any(py)
        }
        ValueSpecification::NumericalRuleBased(value) => {
            NumericalRuleBasedValueSpecification::try_from(value)?.into_py_any(py)
        }
    })
}

pub(crate) fn pyany_to_value_specification(
    pyobject: &Bound<'_, PyAny>,
) -> PyResult<autosar_data_abstraction::datatype::ValueSpecification> {
    use autosar_data_abstraction::datatype::ValueSpecification;
    if let Ok(array_value_specification) = pyobject.cast_exact::<ArrayValueSpecification>() {
        (&*array_value_specification.borrow())
            .try_into()
            .map(ValueSpecification::Array)
    } else if let Ok(record_value_specification) = pyobject.cast_exact::<RecordValueSpecification>()
    {
        (&*record_value_specification.borrow())
            .try_into()
            .map(ValueSpecification::Record)
    } else if let Ok(text_value_specification) = pyobject.cast_exact::<TextValueSpecification>() {
        Ok(ValueSpecification::Text(
            (&*text_value_specification.borrow()).into(),
        ))
    } else if let Ok(numerical_value_specification) =
        pyobject.cast_exact::<NumericalValueSpecification>()
    {
        Ok(ValueSpecification::Numerical(
            (&*numerical_value_specification.borrow()).into(),
        ))
    } else if let Ok(constant_reference) = pyobject.cast_exact::<ConstantReference>() {
        Ok(ValueSpecification::ConstantReference(
            (&*constant_reference.borrow()).into(),
        ))
    } else if let Ok(application_value_specification) =
        pyobject.cast_exact::<ApplicationValueSpecification>()
    {
        (&*application_value_specification.borrow())
            .try_into()
            .map(ValueSpecification::Application)
    } else if let Ok(not_available_value_specification) =
        pyobject.cast_exact::<NotAvailableValueSpecification>()
    {
        Ok(ValueSpecification::NotAvailable(
            (&*not_available_value_specification.borrow()).into(),
        ))
    } else if let Ok(reference_value_specification) =
        pyobject.cast_exact::<ReferenceValueSpecification>()
    {
        (&*reference_value_specification.borrow())
            .try_into()
            .map(ValueSpecification::Reference)
    } else if let Ok(application_rule_based_value_specification) =
        pyobject.cast_exact::<ApplicationRuleBasedValueSpecification>()
    {
        (&*application_rule_based_value_specification.borrow())
            .try_into()
            .map(ValueSpecification::ApplicationRuleBased)
    } else if let Ok(composite_rule_based_value_specification) =
        pyobject.cast_exact::<CompositeRuleBasedValueSpecification>()
    {
        (&*composite_rule_based_value_specification.borrow())
            .try_into()
            .map(ValueSpecification::CompositeRuleBased)
    } else if let Ok(numerical_rule_based_value_specification) =
        pyobject.cast_exact::<NumericalRuleBasedValueSpecification>()
    {
        (&*numerical_rule_based_value_specification.borrow())
            .try_into()
            .map(ValueSpecification::NumericalRuleBased)
    } else if let Ok(py_list) = pyobject.cast_exact::<PyList>() {
        // If it's a PyList, we assume it's an ArrayValueSpecification. This is more convenient for simple use cases
        let values: Vec<autosar_data_abstraction::datatype::ValueSpecification> = pylist_to_vec(
            pyobject.py(),
            py_list.as_unbound(),
            pyany_to_value_specification,
        )?;
        Ok(ValueSpecification::Array(
            autosar_data_abstraction::datatype::ArrayValueSpecification {
                label: None,
                values,
            },
        ))
    } else if let Ok(py_tuple) = pyobject.cast_exact::<PyTuple>() {
        // If it's a PyTuple, we assume it's a RecordValueSpecification. This is more convenient for simple use cases
        let tuple_values = py_tuple
            .as_sequence()
            .try_iter()?
            .map(|elem| pyany_to_value_specification(&elem?))
            .collect::<PyResult<Vec<_>>>()?;
        Ok(ValueSpecification::Record(
            autosar_data_abstraction::datatype::RecordValueSpecification {
                label: None,
                values: tuple_values,
            },
        ))
    } else if let Ok(number) = pyobject.extract::<f64>() {
        // convert a number to a NumericalValueSpecification for convenience
        Ok(ValueSpecification::Numerical(
            autosar_data_abstraction::datatype::NumericalValueSpecification {
                label: None,
                value: number,
            },
        ))
    } else if let Ok(text) = pyobject.extract::<String>() {
        // convert a string to a TextValueSpecification
        Ok(ValueSpecification::Text(
            autosar_data_abstraction::datatype::TextValueSpecification {
                label: None,
                value: text,
            },
        ))
    } else {
        Err(AutosarAbstractionError::new_err(
            "Unknown value specification type",
        ))
    }
}

//##################################################################

/// array of values
#[pyclass(
    eq,
    get_all,
    set_all,
    module = "autosar_data._autosar_data._abstraction._datatype"
)]
#[derive(Debug)]
pub(crate) struct ArrayValueSpecification {
    /// SHORT-LABEL: used to identify the array in a human readable way. This is used when the array is part of a record.
    pub(crate) label: Option<String>,
    /// the values of the array
    pub(crate) values: Py<PyList>, // = Vec<ValueSpecification>,
}

#[pymethods]
impl ArrayValueSpecification {
    #[new]
    #[pyo3(signature = (values, /, *, label = None))]
    #[pyo3(
        text_signature = "(self, values: List[ValueSpecification], /, *, label: Optional[str] = None)"
    )]
    fn new(values: Py<PyList>, label: Option<String>) -> PyResult<Self> {
        Ok(Self { label, values })
    }

    fn __repr__(&self) -> String {
        if let Some(label) = &self.label {
            format!("ArrayValueSpecification(label={}, {})", label, self.values)
        } else {
            format!("ArrayValueSpecification({})", self.values)
        }
    }
}

impl TryFrom<&autosar_data_abstraction::datatype::ArrayValueSpecification>
    for ArrayValueSpecification
{
    type Error = PyErr;

    fn try_from(
        value: &autosar_data_abstraction::datatype::ArrayValueSpecification,
    ) -> Result<Self, Self::Error> {
        Python::attach(|py| {
            let values = slice_to_pylist(py, &value.values, value_specification_to_pyany)?;
            Ok(Self {
                label: value.label.clone(),
                values,
            })
        })
    }
}

impl TryFrom<&ArrayValueSpecification>
    for autosar_data_abstraction::datatype::ArrayValueSpecification
{
    type Error = PyErr;

    fn try_from(value: &ArrayValueSpecification) -> Result<Self, Self::Error> {
        let values =
            Python::attach(|py| pylist_to_vec(py, &value.values, pyany_to_value_specification))?;
        Ok(
            autosar_data_abstraction::datatype::ArrayValueSpecification {
                label: value.label.clone(),
                values,
            },
        )
    }
}

impl PartialEq for ArrayValueSpecification {
    fn eq(&self, other: &Self) -> bool {
        Python::attach(|py| {
            self.label == other.label && compare_pylist(py, &self.values, &other.values)
        })
    }
}

//#########################################################

/// record of values. The values may be named using short-labels, but these are not mandatory.
#[pyclass(
    eq,
    get_all,
    set_all,
    module = "autosar_data._autosar_data._abstraction._datatype"
)]
#[derive(Debug)]
pub(crate) struct RecordValueSpecification {
    /// SHORT-LABEL: used to identify the record in a human readable way. This is used when the record is part of a record.
    pub(crate) label: Option<String>,
    /// the values of the record
    /// The values may be named using short-labels, but these are not mandatory.
    pub(crate) values: Py<PyList>, // = Vec<ValueSpecification>,
}

#[pymethods]
impl RecordValueSpecification {
    #[new]
    #[pyo3(signature = (values, /, *, label = None))]
    #[pyo3(
        text_signature = "(self, values: List[ValueSpecification], /, *, label: Optional[str] = None)"
    )]
    fn new(values: Py<PyList>, label: Option<String>) -> PyResult<Self> {
        Ok(Self { label, values })
    }

    fn __repr__(&self) -> String {
        if let Some(label) = &self.label {
            format!("RecordValueSpecification(label={}, {})", label, self.values)
        } else {
            format!("RecordValueSpecification({})", self.values)
        }
    }
}

impl TryFrom<&autosar_data_abstraction::datatype::RecordValueSpecification>
    for RecordValueSpecification
{
    type Error = PyErr;

    fn try_from(
        value: &autosar_data_abstraction::datatype::RecordValueSpecification,
    ) -> Result<Self, Self::Error> {
        Python::attach(|py| {
            let values = slice_to_pylist(py, &value.values, value_specification_to_pyany)?;
            Ok(Self {
                label: value.label.clone(),
                values,
            })
        })
    }
}

impl TryFrom<&RecordValueSpecification>
    for autosar_data_abstraction::datatype::RecordValueSpecification
{
    type Error = PyErr;

    fn try_from(value: &RecordValueSpecification) -> Result<Self, Self::Error> {
        let values =
            Python::attach(|py| pylist_to_vec(py, &value.values, pyany_to_value_specification))?;
        Ok(
            autosar_data_abstraction::datatype::RecordValueSpecification {
                label: value.label.clone(),
                values,
            },
        )
    }
}

impl PartialEq for RecordValueSpecification {
    fn eq(&self, other: &Self) -> bool {
        Python::attach(|py| {
            self.label == other.label && compare_pylist(py, &self.values, &other.values)
        })
    }
}

//#########################################################

/// textual value
#[pyclass(
    eq,
    get_all,
    set_all,
    module = "autosar_data._autosar_data._abstraction._datatype"
)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TextValueSpecification {
    /// SHORT-LABEL: used to identify the text in a human readable way. This is used when the text is part of a record.
    pub(crate) label: Option<String>,
    /// the text value
    pub(crate) value: String,
}

#[pymethods]
impl TextValueSpecification {
    #[new]
    #[pyo3(signature = (value, /, *, label = None))]
    #[pyo3(text_signature = "(self, value: str, /, *, label: Optional[str] = None)")]
    fn new(value: String, label: Option<String>) -> PyResult<Self> {
        Ok(Self { label, value })
    }

    fn __repr__(&self) -> String {
        if let Some(label) = &self.label {
            format!(
                "TextValueSpecification(label={}, value={})",
                label, self.value
            )
        } else {
            format!("TextValueSpecification(value={})", self.value)
        }
    }
}

impl From<&autosar_data_abstraction::datatype::TextValueSpecification> for TextValueSpecification {
    fn from(value: &autosar_data_abstraction::datatype::TextValueSpecification) -> Self {
        Self {
            label: value.label.clone(),
            value: value.value.clone(),
        }
    }
}

impl From<&TextValueSpecification> for autosar_data_abstraction::datatype::TextValueSpecification {
    fn from(value: &TextValueSpecification) -> Self {
        autosar_data_abstraction::datatype::TextValueSpecification {
            label: value.label.clone(),
            value: value.value.clone(),
        }
    }
}

//#########################################################

/// numerical value
#[pyclass(
    eq,
    get_all,
    set_all,
    module = "autosar_data._autosar_data._abstraction._datatype"
)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct NumericalValueSpecification {
    /// SHORT-LABEL: used to identify the number in a human readable way. This is used when the number is part of a record.
    pub(crate) label: Option<String>,
    /// the number value
    pub(crate) value: f64,
}

#[pymethods]
impl NumericalValueSpecification {
    #[new]
    #[pyo3(signature = (value, /, *, label = None))]
    #[pyo3(text_signature = "(self, value: float, /, *, label: Optional[str] = None)")]
    fn new(value: f64, label: Option<String>) -> PyResult<Self> {
        Ok(Self { label, value })
    }

    fn __repr__(&self) -> String {
        if let Some(label) = &self.label {
            format!(
                "NumericalValueSpecification(label={}, value={})",
                label, self.value
            )
        } else {
            format!("NumericalValueSpecification(value={})", self.value)
        }
    }
}

impl From<&autosar_data_abstraction::datatype::NumericalValueSpecification>
    for NumericalValueSpecification
{
    fn from(value: &autosar_data_abstraction::datatype::NumericalValueSpecification) -> Self {
        Self {
            label: value.label.clone(),
            value: value.value,
        }
    }
}

impl From<&NumericalValueSpecification>
    for autosar_data_abstraction::datatype::NumericalValueSpecification
{
    fn from(value: &NumericalValueSpecification) -> Self {
        autosar_data_abstraction::datatype::NumericalValueSpecification {
            label: value.label.clone(),
            value: value.value,
        }
    }
}

//#########################################################

/// reference to a `ConstantValue`
#[pyclass(
    eq,
    get_all,
    set_all,
    module = "autosar_data._autosar_data._abstraction._datatype"
)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ConstantReference {
    /// SHORT-LABEL: used to identify the constant in a human readable way. This is used when the constant is part of a record.
    pub(crate) label: Option<String>,
    /// Reference to the constant specification
    pub(crate) constant: ConstantSpecification,
}

#[pymethods]
impl ConstantReference {
    #[new]
    #[pyo3(signature = (constant, /, *, label = None))]
    #[pyo3(
        text_signature = "(self, constant: ConstantSpecification, /, *, label: Optional[str] = None)"
    )]
    fn new(constant: ConstantSpecification, label: Option<String>) -> PyResult<Self> {
        Ok(Self { label, constant })
    }

    fn __repr__(&self) -> String {
        let constant_path = self.constant.element().0.path();
        let constant_str = constant_path.unwrap_or_else(|_| "<invalid>".to_string());
        if let Some(label) = &self.label {
            format!(
                "ConstantReference(label={}, constant={constant_str})",
                label
            )
        } else {
            format!("ConstantReference(constant={constant_str})")
        }
    }
}

impl From<&autosar_data_abstraction::datatype::ConstantReference> for ConstantReference {
    fn from(value: &autosar_data_abstraction::datatype::ConstantReference) -> Self {
        Self {
            label: value.label.clone(),
            constant: ConstantSpecification(value.constant.clone()),
        }
    }
}

impl From<&ConstantReference> for autosar_data_abstraction::datatype::ConstantReference {
    fn from(value: &ConstantReference) -> Self {
        autosar_data_abstraction::datatype::ConstantReference {
            label: value.label.clone(),
            constant: value.constant.0.clone(),
        }
    }
}

//#########################################################

/// Application value
#[pyclass(
    eq,
    get_all,
    set_all,
    module = "autosar_data._autosar_data._abstraction._datatype"
)]
#[derive(Debug)]
pub(crate) struct ApplicationValueSpecification {
    /// SHORT-LABEL: used to identify the application value in a human readable way. This is used when the application value is part of a record.
    pub(crate) label: Option<String>,
    /// category of the application value
    pub(crate) category: ApplicationPrimitiveCategory,
    /// axis values of a compound primitive data type. Required for categories `ResAxis`, Cure, Map, Cuboid, Cube4, Cube5
    pub(crate) sw_axis_conts: Py<PyList>, //= Vec<SwAxisCont>
    /// values of a compound primitive data type
    pub(crate) sw_value_cont: Py<SwValueCont>,
}

#[pymethods]
impl ApplicationValueSpecification {
    #[new]
    #[pyo3(signature = (
        category,
        sw_axis_conts,
        sw_value_cont,
        /,
        *,
        label = None
    ))]
    #[pyo3(
        text_signature = "(self, category: ApplicationPrimitiveCategory, sw_axis_conts: List[SwAxisCont], sw_value_cont: SwValueCont, /, *, label: Optional[str] = None)"
    )]
    fn new(
        category: ApplicationPrimitiveCategory,
        sw_axis_conts: Py<PyList>,
        sw_value_cont: Py<SwValueCont>,
        label: Option<String>,
    ) -> PyResult<Self> {
        Ok(Self {
            label,
            category,
            sw_axis_conts,
            sw_value_cont,
        })
    }

    fn __repr__(&self) -> String {
        if let Some(label) = &self.label {
            format!(
                "ApplicationValueSpecification(label={}, category={:?}, sw_axis_conts={}, sw_value_cont={})",
                label, self.category, self.sw_axis_conts, self.sw_value_cont
            )
        } else {
            format!(
                "ApplicationValueSpecification(category={:?}, sw_axis_conts={}, sw_value_cont={})",
                self.category, self.sw_axis_conts, self.sw_value_cont
            )
        }
    }
}

impl TryFrom<&autosar_data_abstraction::datatype::ApplicationValueSpecification>
    for ApplicationValueSpecification
{
    type Error = PyErr;

    fn try_from(
        value: &autosar_data_abstraction::datatype::ApplicationValueSpecification,
    ) -> Result<Self, Self::Error> {
        Python::attach(|py| {
            let sw_axis_conts = slice_to_pylist(py, &value.sw_axis_conts, |axis| {
                SwAxisCont::try_from(axis)?.into_py_any(py)
            })?;
            let sw_value_cont = SwValueCont::try_from(&value.sw_value_cont)?
                .into_pyobject(py)?
                .unbind();
            Ok(Self {
                label: value.label.clone(),
                category: value.category.into(),
                sw_axis_conts,
                sw_value_cont,
            })
        })
    }
}

impl TryFrom<&ApplicationValueSpecification>
    for autosar_data_abstraction::datatype::ApplicationValueSpecification
{
    type Error = PyErr;

    fn try_from(value: &ApplicationValueSpecification) -> Result<Self, Self::Error> {
        Python::attach(|py| {
            let sw_axis_conts = pylist_to_vec(py, &value.sw_axis_conts, |axis| {
                (&*axis.cast_exact::<SwAxisCont>()?.borrow()).try_into()
            })?;
            let sw_value_cont = &*value.sw_value_cont.borrow(py);
            Ok(
                autosar_data_abstraction::datatype::ApplicationValueSpecification {
                    label: value.label.clone(),
                    category: value.category.into(),
                    sw_axis_conts,
                    sw_value_cont: sw_value_cont.try_into()?,
                },
            )
        })
    }
}

impl PartialEq for ApplicationValueSpecification {
    fn eq(&self, other: &Self) -> bool {
        Python::attach(|py| {
            self.label == other.label
                && self.category == other.category
                && compare_pylist(py, &self.sw_axis_conts, &other.sw_axis_conts)
                && *self.sw_value_cont.borrow(py) == *other.sw_value_cont.borrow(py)
        })
    }
}

//#########################################################

/// Default init pattern, which is used when an optional `ApplicationRecordElement` in not available
#[pyclass(
    eq,
    get_all,
    set_all,
    module = "autosar_data._autosar_data._abstraction._datatype"
)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct NotAvailableValueSpecification {
    /// SHORT-LABEL: used to identify the default pattern in a human readable way. This is used when the default pattern is part of a record.
    pub(crate) label: Option<String>,
    /// initialization pattern for memory occupied by unavailable application record elements; available in `AUTOSAR_00049` and newer
    pub(crate) default_pattern: Option<u64>, // presumably this could be u8 to initialize bytes in memory. But the spec only says it's a positive integer
}

#[pymethods]
impl NotAvailableValueSpecification {
    #[new]
    #[pyo3(signature = (default_pattern = None, label = None))]
    #[pyo3(
        text_signature = "(self, /, *, default_pattern: Optional[int] = None, label: Optional[str] = None)"
    )]
    fn new(default_pattern: Option<u64>, label: Option<String>) -> PyResult<Self> {
        Ok(Self {
            label,
            default_pattern,
        })
    }

    fn __repr__(&self) -> String {
        if let Some(label) = &self.label {
            format!(
                "NotAvailableValueSpecification(label={}, default_pattern={:?})",
                label, self.default_pattern
            )
        } else {
            format!(
                "NotAvailableValueSpecification(default_pattern={:?})",
                self.default_pattern
            )
        }
    }
}

impl From<&autosar_data_abstraction::datatype::NotAvailableValueSpecification>
    for NotAvailableValueSpecification
{
    fn from(value: &autosar_data_abstraction::datatype::NotAvailableValueSpecification) -> Self {
        Self {
            label: value.label.clone(),
            default_pattern: value.default_pattern,
        }
    }
}

impl From<&NotAvailableValueSpecification>
    for autosar_data_abstraction::datatype::NotAvailableValueSpecification
{
    fn from(value: &NotAvailableValueSpecification) -> Self {
        autosar_data_abstraction::datatype::NotAvailableValueSpecification {
            label: value.label.clone(),
            default_pattern: value.default_pattern,
        }
    }
}

//#########################################################

/// reference to a `DataPrototype`, to be used as a pointer in the software
#[pyclass(
    eq,
    get_all,
    set_all,
    module = "autosar_data._autosar_data._abstraction._datatype"
)]
#[derive(Debug)]
pub(crate) struct ReferenceValueSpecification {
    /// SHORT-LABEL: used to identify the reference in a human readable way. This is used when the reference is part of a record.
    pub(crate) label: Option<String>,
    /// data prototype that will be referenced as a pointer in the software
    pub(crate) reference_value: Py<PyAny>, // = DataPrototype
}

#[pymethods]
impl ReferenceValueSpecification {
    #[new]
    #[pyo3(signature = (reference_value, /, *, label = None))]
    #[pyo3(
        text_signature = "(self, reference_value: DataPrototype, /, *, label: Optional[str] = None)"
    )]
    fn new(reference_value: Py<PyAny>, label: Option<String>) -> PyResult<Self> {
        Ok(Self {
            label,
            reference_value,
        })
    }

    fn __repr__(&self) -> String {
        if let Some(label) = &self.label {
            format!(
                "ReferenceValueSpecification(label={}, reference_value={})",
                label, self.reference_value
            )
        } else {
            format!(
                "ReferenceValueSpecification(reference_value={})",
                self.reference_value
            )
        }
    }
}

impl TryFrom<&autosar_data_abstraction::datatype::ReferenceValueSpecification>
    for ReferenceValueSpecification
{
    type Error = PyErr;

    fn try_from(
        value: &autosar_data_abstraction::datatype::ReferenceValueSpecification,
    ) -> Result<Self, Self::Error> {
        let reference_value = data_prototype_to_pyany(value.reference_value.clone())?;
        Ok(Self {
            label: value.label.clone(),
            reference_value,
        })
    }
}

impl TryFrom<&ReferenceValueSpecification>
    for autosar_data_abstraction::datatype::ReferenceValueSpecification
{
    type Error = PyErr;
    fn try_from(value: &ReferenceValueSpecification) -> Result<Self, Self::Error> {
        let reference_value =
            Python::attach(|py| pyany_to_data_prototype(value.reference_value.bind(py)))?;
        Ok(
            autosar_data_abstraction::datatype::ReferenceValueSpecification {
                label: value.label.clone(),
                reference_value,
            },
        )
    }
}

impl PartialEq for ReferenceValueSpecification {
    fn eq(&self, other: &Self) -> bool {
        Python::attach(|py| {
            let own_ref = pyany_to_data_prototype(self.reference_value.bind(py));
            let other_ref = pyany_to_data_prototype(other.reference_value.bind(py));
            if let (Ok(own_ref), Ok(other_ref)) = (own_ref, other_ref) {
                self.label == other.label && own_ref == other_ref
            } else {
                // if the conversion fails, we assume the references are not equal
                false
            }
        })
    }
}

//#########################################################

/// A rule to generate application values for an array value specification
#[pyclass(
    eq,
    get_all,
    set_all,
    module = "autosar_data._autosar_data._abstraction._datatype"
)]
#[derive(Debug)]
pub(crate) struct ApplicationRuleBasedValueSpecification {
    /// SHORT-LABEL: used to identify the application value in a human readable way. This is used when the application value is part of a record.
    pub(crate) label: Option<String>,
    /// category of the application value
    pub(crate) category: ApplicationPrimitiveCategory,
    /// rule-based axis values of a compound primitive data type. Required for categories `ResAxis`, Cure, Map, Cuboid, Cube4, Cube5
    pub(crate) sw_axis_cont: Py<PyList>, // = Vec<RuleBasedAxisCont>
    /// rule-based values of a compound primitive data type
    pub(crate) sw_value_cont: Py<RuleBasedValueCont>,
}

#[pymethods]
impl ApplicationRuleBasedValueSpecification {
    #[new]
    #[pyo3(signature = (
        category,
        sw_axis_cont,
        sw_value_cont,
        /,
        *,
        label = None
    ))]
    #[pyo3(
        text_signature = "(self, category: ApplicationPrimitiveCategory, sw_axis_cont: List[RuleBasedAxisCont], sw_value_cont: RuleBasedValueCont, /, *, label: Optional[str] = None)"
    )]
    fn new(
        category: ApplicationPrimitiveCategory,
        sw_axis_cont: Py<PyList>,
        sw_value_cont: Py<RuleBasedValueCont>,
        label: Option<String>,
    ) -> PyResult<Self> {
        Ok(Self {
            label,
            category,
            sw_axis_cont,
            sw_value_cont,
        })
    }

    fn __repr__(&self) -> String {
        if let Some(label) = &self.label {
            format!(
                "ApplicationRuleBasedValueSpecification(label={}, category={:?}, sw_axis_cont={}, sw_value_cont={})",
                label, self.category, self.sw_axis_cont, self.sw_value_cont
            )
        } else {
            format!(
                "ApplicationRuleBasedValueSpecification(category={:?}, sw_axis_cont={}, sw_value_cont={})",
                self.category, self.sw_axis_cont, self.sw_value_cont
            )
        }
    }
}

impl TryFrom<&autosar_data_abstraction::datatype::ApplicationRuleBasedValueSpecification>
    for ApplicationRuleBasedValueSpecification
{
    type Error = PyErr;

    fn try_from(
        value: &autosar_data_abstraction::datatype::ApplicationRuleBasedValueSpecification,
    ) -> Result<Self, Self::Error> {
        Python::attach(|py| {
            let sw_axis_cont = slice_to_pylist(py, &value.sw_axis_cont, |axis| {
                RuleBasedAxisCont::try_from(axis)?.into_py_any(py)
            })?;
            let sw_value_cont = RuleBasedValueCont::try_from(&value.sw_value_cont)?
                .into_pyobject(py)?
                .unbind();
            Ok(Self {
                label: value.label.clone(),
                category: value.category.into(),
                sw_axis_cont,
                sw_value_cont,
            })
        })
    }
}

impl TryFrom<&ApplicationRuleBasedValueSpecification>
    for autosar_data_abstraction::datatype::ApplicationRuleBasedValueSpecification
{
    type Error = PyErr;

    fn try_from(value: &ApplicationRuleBasedValueSpecification) -> Result<Self, Self::Error> {
        Python::attach(|py| {
            let sw_axis_cont = pylist_to_vec(py, &value.sw_axis_cont, |axis| {
                (&*axis.cast_exact::<RuleBasedAxisCont>()?.borrow()).try_into()
            })?;

            let sw_value_cont = &*value.sw_value_cont.borrow(py);
            Ok(
                autosar_data_abstraction::datatype::ApplicationRuleBasedValueSpecification {
                    label: value.label.clone(),
                    category: value.category.into(),
                    sw_axis_cont,
                    sw_value_cont: sw_value_cont.try_into()?,
                },
            )
        })
    }
}

impl PartialEq for ApplicationRuleBasedValueSpecification {
    fn eq(&self, other: &Self) -> bool {
        Python::attach(|py| {
            self.label == other.label
                && self.category == other.category
                && compare_pylist(py, &self.sw_axis_cont, &other.sw_axis_cont)
                && *self.sw_value_cont.borrow(py) == *other.sw_value_cont.borrow(py)
        })
    }
}

//#########################################################

/// A rule to generate composite values for an array value specification
#[pyclass(
    eq,
    get_all,
    set_all,
    module = "autosar_data._autosar_data._abstraction._datatype"
)]
#[derive(Debug)]
pub(crate) struct CompositeRuleBasedValueSpecification {
    /// SHORT-LABEL: used to identify the composite value in a human readable way. This is used when the composite value is part of a record.
    pub(crate) label: Option<String>,
    /// collection of specified compound values. The last value is used by the filling rule to fill the array
    pub(crate) argument: Py<PyList>, // = Vec<CompositeValueSpecification>
    /// collection of specified primitive values. The last value is used by the filling rule to fill the array
    pub(crate) compound_primitive_argument: Py<PyList>, // = Vec<CompositeRuleBasedValueArgument>
    /// maximum size of the array to fill. It is used if the filling rule is set to `FILL_UNTIL_MAX_SIZE`
    pub(crate) max_size_to_fill: Option<u64>,
    /// rule to fill the array
    pub(crate) rule: RuleBasedFillUntil,
}

#[pymethods]
impl CompositeRuleBasedValueSpecification {
    #[new]
    #[pyo3(signature = (
        argument,
        compound_primitive_argument,
        rule,
        /,
        *,
        max_size_to_fill = None,
        label = None
    ))]
    #[pyo3(
        text_signature = "(self, argument: List[CompositeValueSpecification], compound_primitive_argument: List[CompositeRuleBasedValueArgument], rule: RuleBasedFillUntil, /, *, max_size_to_fill: Optional[int] = None, label: Optional[str] = None)"
    )]
    fn new(
        argument: Py<PyList>,
        compound_primitive_argument: Py<PyList>,
        rule: RuleBasedFillUntil,
        max_size_to_fill: Option<u64>,
        label: Option<String>,
    ) -> PyResult<Self> {
        Ok(Self {
            label,
            argument,
            compound_primitive_argument,
            max_size_to_fill,
            rule,
        })
    }

    fn __repr__(&self) -> String {
        if let Some(label) = &self.label {
            format!(
                "CompositeRuleBasedValueSpecification(label={}, argument={}, compound_primitive_argument={}, max_size_to_fill={:?}, rule={:?})",
                label,
                self.argument,
                self.compound_primitive_argument,
                self.max_size_to_fill,
                self.rule
            )
        } else {
            format!(
                "CompositeRuleBasedValueSpecification(argument={}, compound_primitive_argument={}, max_size_to_fill={:?}, rule={:?})",
                self.argument, self.compound_primitive_argument, self.max_size_to_fill, self.rule
            )
        }
    }
}

impl TryFrom<&autosar_data_abstraction::datatype::CompositeRuleBasedValueSpecification>
    for CompositeRuleBasedValueSpecification
{
    type Error = PyErr;

    fn try_from(
        value: &autosar_data_abstraction::datatype::CompositeRuleBasedValueSpecification,
    ) -> Result<Self, Self::Error> {
        Python::attach(|py| {
            let argument =
                slice_to_pylist(py, &value.argument, composite_value_specification_to_pyany)?;
            let compound_primitive_argument = slice_to_pylist(
                py,
                &value.compound_primitive_argument,
                composite_rule_based_value_argument_to_pyany,
            )?;
            Ok(Self {
                label: value.label.clone(),
                argument,
                compound_primitive_argument,
                max_size_to_fill: value.max_size_to_fill,
                rule: value.rule.into(),
            })
        })
    }
}

impl TryFrom<&CompositeRuleBasedValueSpecification>
    for autosar_data_abstraction::datatype::CompositeRuleBasedValueSpecification
{
    type Error = PyErr;

    fn try_from(value: &CompositeRuleBasedValueSpecification) -> Result<Self, Self::Error> {
        Python::attach(|py| {
            let argument =
                pylist_to_vec(py, &value.argument, pyany_to_composite_value_specification)?;
            let compound_primitive_argument = pylist_to_vec(
                py,
                &value.compound_primitive_argument,
                pyany_to_composite_rule_based_value_argument,
            )?;
            Ok(
                autosar_data_abstraction::datatype::CompositeRuleBasedValueSpecification {
                    label: value.label.clone(),
                    argument,
                    compound_primitive_argument,
                    max_size_to_fill: value.max_size_to_fill,
                    rule: value.rule.into(),
                },
            )
        })
    }
}

impl PartialEq for CompositeRuleBasedValueSpecification {
    fn eq(&self, other: &Self) -> bool {
        Python::attach(|py| {
            self.label == other.label
                && compare_pylist(py, &self.argument, &other.argument)
                && compare_pylist(
                    py,
                    &self.compound_primitive_argument,
                    &other.compound_primitive_argument,
                )
                && self.max_size_to_fill == other.max_size_to_fill
                && self.rule == other.rule
        })
    }
}

//#########################################################

/// A rule to generate numerical values for an array value specification
#[pyclass(
    eq,
    get_all,
    set_all,
    module = "autosar_data._autosar_data._abstraction._datatype"
)]
#[derive(Debug)]
pub(crate) struct NumericalRuleBasedValueSpecification {
    /// SHORT-LABEL: used to identify the numerical value in a human readable way. This is used when the numerical value is part of a record.
    pub(crate) label: Option<String>,
    /// rule-based values for the array
    pub(crate) rule_based_values: Py<RuleBasedValueSpecification>,
}

#[pymethods]
impl NumericalRuleBasedValueSpecification {
    #[new]
    #[pyo3(signature = (rule_based_values, /, *, label = None))]
    #[pyo3(
        text_signature = "(self, rule_based_values: RuleBasedValueSpecification, /, *, label: Optional[str] = None)"
    )]
    fn new(
        rule_based_values: Py<RuleBasedValueSpecification>,
        label: Option<String>,
    ) -> PyResult<Self> {
        Ok(Self {
            label,
            rule_based_values,
        })
    }

    fn __repr__(&self) -> String {
        if let Some(label) = &self.label {
            format!(
                "NumericalRuleBasedValueSpecification(label={}, rule_based_values={})",
                label, self.rule_based_values
            )
        } else {
            format!(
                "NumericalRuleBasedValueSpecification(rule_based_values={})",
                self.rule_based_values
            )
        }
    }
}

impl TryFrom<&autosar_data_abstraction::datatype::NumericalRuleBasedValueSpecification>
    for NumericalRuleBasedValueSpecification
{
    type Error = PyErr;

    fn try_from(
        value: &autosar_data_abstraction::datatype::NumericalRuleBasedValueSpecification,
    ) -> Result<Self, Self::Error> {
        Python::attach(|py| {
            let rule_based_values =
                RuleBasedValueSpecification::try_from(&value.rule_based_values)?
                    .into_pyobject(py)?
                    .unbind();
            Ok(Self {
                label: value.label.clone(),
                rule_based_values,
            })
        })
    }
}

impl TryFrom<&NumericalRuleBasedValueSpecification>
    for autosar_data_abstraction::datatype::NumericalRuleBasedValueSpecification
{
    type Error = PyErr;

    fn try_from(value: &NumericalRuleBasedValueSpecification) -> Result<Self, Self::Error> {
        Python::attach(|py| {
            let rule_based_values = &*value.rule_based_values.borrow(py);
            Ok(
                autosar_data_abstraction::datatype::NumericalRuleBasedValueSpecification {
                    label: value.label.clone(),
                    rule_based_values: rule_based_values.try_into()?,
                },
            )
        })
    }
}

impl PartialEq for NumericalRuleBasedValueSpecification {
    fn eq(&self, other: &Self) -> bool {
        Python::attach(|py| {
            self.label == other.label
                && *self.rule_based_values.borrow(py) == *other.rule_based_values.borrow(py)
        })
    }
}

//#########################################################

/// standard fill rules for rule based value specifications
#[pyclass(
    frozen,
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._datatype"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RuleBasedFillUntil {
    /// `FILL_UNTIL_END`: fills the value of the last RuleBasedValueSpecification.arguments
    /// until the last element of the array has been filled
    End,
    /// `FILL_UNTIL_MAX_SIZE`: fills the value of the last RuleBasedValueSpecification.arguments
    /// until maxSizeToFill elements of the array have been filled
    MaxSize,
}

impl From<autosar_data_abstraction::datatype::RuleBasedFillUntil> for RuleBasedFillUntil {
    fn from(value: autosar_data_abstraction::datatype::RuleBasedFillUntil) -> Self {
        match value {
            autosar_data_abstraction::datatype::RuleBasedFillUntil::End => Self::End,
            autosar_data_abstraction::datatype::RuleBasedFillUntil::MaxSize => Self::MaxSize,
        }
    }
}

impl From<RuleBasedFillUntil> for autosar_data_abstraction::datatype::RuleBasedFillUntil {
    fn from(value: RuleBasedFillUntil) -> Self {
        match value {
            RuleBasedFillUntil::End => Self::End,
            RuleBasedFillUntil::MaxSize => Self::MaxSize,
        }
    }
}

//#########################################################

/// specification of the axis values of a compound primitive data type (curve, map)
#[pyclass(
    eq,
    module = "autosar_data._autosar_data._abstraction._datatype",
    get_all,
    set_all
)]
#[derive(Debug)]
pub(crate) struct SwAxisCont {
    /// category of the axis; one of `STD_AXIS`, `COM_AXIS`, `COM_AXIS`, `RES_AXIS`
    pub(crate) category: SwAxisContCategory,
    /// dimensions of the axis, used if the category is `RES_AXIS`, otherwise it should be empty
    pub(crate) sw_array_size: Vec<u64>,
    /// index of the axis. Here 1 is the x axis, 2 is the y axis, ...
    pub(crate) sw_axis_index: u64,
    /// axis values in the physical domain
    pub(crate) sw_values_phys: Py<PyList>, // = Vec<SwValue>
    /// pyhsical unit of the axis values
    pub(crate) unit: Option<Unit>,
    /// display name of the unit
    pub(crate) unit_display_name: Option<String>,
}

#[pymethods]
impl SwAxisCont {
    /// create a new sw axis content
    #[new]
    #[pyo3(signature = (
        category,
        sw_array_size,
        sw_axis_index,
        sw_values_phys,
        /,
        *,
        unit = None,
        unit_display_name = None
    ))]
    #[pyo3(
        text_signature = "(self, category: SwAxisContCategory, sw_array_size: List[int], sw_axis_index: int, sw_values_phys: List[SwValue], /, *, unit: Optional[Unit] = None, unit_display_name: Optional[str] = None)"
    )]
    fn new(
        category: SwAxisContCategory,
        sw_array_size: Vec<u64>,
        sw_axis_index: u64,
        sw_values_phys: Py<PyList>,
        unit: Option<Unit>,
        unit_display_name: Option<String>,
    ) -> PyResult<Self> {
        Ok(Self {
            category,
            sw_array_size,
            sw_axis_index,
            sw_values_phys,
            unit,
            unit_display_name,
        })
    }

    fn __repr__(&self) -> String {
        let mut text = format!(
            "SwAxisCont(category={:?}, sw_array_size={:?}, sw_axis_index={}, sw_values_phys={}",
            self.category, self.sw_array_size, self.sw_axis_index, self.sw_values_phys,
        );
        if let Some(unit) = &self.unit {
            text.push_str(", unit=");
            text.push_str(
                &unit
                    .element()
                    .0
                    .path()
                    .unwrap_or_else(|_| "<invalid>".to_string()),
            );
        }
        if let Some(unit_display_name) = &self.unit_display_name {
            text.push_str(", unit_display_name=");
            text.push_str(unit_display_name);
        }

        text.push(')');
        text
    }
}

impl TryFrom<&autosar_data_abstraction::datatype::SwAxisCont> for SwAxisCont {
    type Error = PyErr;

    fn try_from(
        value: &autosar_data_abstraction::datatype::SwAxisCont,
    ) -> Result<Self, Self::Error> {
        Python::attach(|py| {
            let sw_values_phys = slice_to_pylist(py, &value.sw_values_phys, |sw_value| {
                SwValue::try_from(sw_value)?.into_py_any(py)
            })?;
            Ok(Self {
                category: value.category.into(),
                sw_array_size: value.sw_array_size.clone(),
                sw_axis_index: value.sw_axis_index,
                sw_values_phys,
                unit: value.unit.as_ref().map(|unit| Unit(unit.clone())),
                unit_display_name: value.unit_display_name.clone(),
            })
        })
    }
}

impl TryFrom<&SwAxisCont> for autosar_data_abstraction::datatype::SwAxisCont {
    type Error = PyErr;

    fn try_from(value: &SwAxisCont) -> Result<Self, Self::Error> {
        Python::attach(|py| {
            let sw_values_phys = pylist_to_vec(py, &value.sw_values_phys, |sw_value| {
                (&*sw_value.cast::<SwValue>()?.borrow()).try_into()
            })?;
            let unit = value.unit.as_ref().map(|unit| unit.0.clone());
            Ok(autosar_data_abstraction::datatype::SwAxisCont {
                category: value.category.into(),
                sw_array_size: value.sw_array_size.clone(),
                sw_axis_index: value.sw_axis_index,
                sw_values_phys,
                unit,
                unit_display_name: value.unit_display_name.clone(),
            })
        })
    }
}

impl PartialEq for SwAxisCont {
    fn eq(&self, other: &Self) -> bool {
        Python::attach(|py| {
            self.category == other.category
                && self.sw_array_size == other.sw_array_size
                && self.sw_axis_index == other.sw_axis_index
                && compare_pylist(py, &self.sw_values_phys, &other.sw_values_phys)
                && self.unit == other.unit
                && self.unit_display_name == other.unit_display_name
        })
    }
}

//#########################################################

/// enumeration of the axis categories.
/// This is a restricted version of the `CalprmAxisCategoryEnum`: `FixAxis` is not permitted in `SwAxisCont`
#[pyclass(
    frozen,
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._datatype"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)] // named as per AUTOSAR standard
pub(crate) enum SwAxisContCategory {
    /// standard axis
    StdAxis,
    /// commmon axis
    ComAxis,
    /// rescale axis
    ResAxis,
}

impl From<autosar_data_abstraction::datatype::SwAxisContCategory> for SwAxisContCategory {
    fn from(value: autosar_data_abstraction::datatype::SwAxisContCategory) -> Self {
        match value {
            autosar_data_abstraction::datatype::SwAxisContCategory::StdAxis => Self::StdAxis,
            autosar_data_abstraction::datatype::SwAxisContCategory::ComAxis => Self::ComAxis,
            autosar_data_abstraction::datatype::SwAxisContCategory::ResAxis => Self::ResAxis,
        }
    }
}

impl From<SwAxisContCategory> for autosar_data_abstraction::datatype::SwAxisContCategory {
    fn from(value: SwAxisContCategory) -> Self {
        match value {
            SwAxisContCategory::StdAxis => Self::StdAxis,
            SwAxisContCategory::ComAxis => Self::ComAxis,
            SwAxisContCategory::ResAxis => Self::ResAxis,
        }
    }
}

//#########################################################

/// specification of the values of a compound primitive data type (curve, map)
#[pyclass(
    eq,
    module = "autosar_data._autosar_data._abstraction._datatype",
    get_all,
    set_all
)]
#[derive(Debug)]
pub(crate) struct SwValueCont {
    /// dimensions of the array
    pub(crate) sw_array_size: Vec<u64>,
    /// values in the physical domain
    pub(crate) sw_values_phys: Py<PyList>, // = Vec<SwValue>
}

#[pymethods]
impl SwValueCont {
    /// create a new sw value content
    #[new]
    #[pyo3(text_signature = "(self, sw_array_size: List[int], sw_values_phys: List[SwValue])")]
    fn new(sw_array_size: Vec<u64>, sw_values_phys: Py<PyList>) -> PyResult<Self> {
        Ok(Self {
            sw_array_size,
            sw_values_phys,
        })
    }

    fn __repr__(&self) -> String {
        format!(
            "SwValueCont(sw_array_size={:?}, sw_values_phys={})",
            self.sw_array_size, self.sw_values_phys
        )
    }
}

impl TryFrom<&autosar_data_abstraction::datatype::SwValueCont> for SwValueCont {
    type Error = PyErr;

    fn try_from(
        value: &autosar_data_abstraction::datatype::SwValueCont,
    ) -> Result<Self, Self::Error> {
        Python::attach(|py| {
            let sw_values_phys = slice_to_pylist(py, &value.sw_values_phys, |sw_value| {
                SwValue::try_from(sw_value)?.into_py_any(py)
            })?;
            Ok(Self {
                sw_array_size: value.sw_array_size.clone(),
                sw_values_phys,
            })
        })
    }
}

impl TryFrom<&SwValueCont> for autosar_data_abstraction::datatype::SwValueCont {
    type Error = PyErr;

    fn try_from(value: &SwValueCont) -> Result<Self, Self::Error> {
        Python::attach(|py| {
            let sw_values_phys = pylist_to_vec(py, &value.sw_values_phys, |sw_value| {
                (&*sw_value.cast::<SwValue>()?.borrow()).try_into()
            })?;
            Ok(autosar_data_abstraction::datatype::SwValueCont {
                sw_array_size: value.sw_array_size.clone(),
                sw_values_phys,
            })
        })
    }
}

impl PartialEq for SwValueCont {
    fn eq(&self, other: &Self) -> bool {
        Python::attach(|py| {
            self.sw_array_size == other.sw_array_size
                && compare_pylist(py, &self.sw_values_phys, &other.sw_values_phys)
        })
    }
}

//#########################################################

/// a single value of a compound primitive data type (curve, map)
#[pyclass(
    eq,
    module = "autosar_data._autosar_data._abstraction._datatype",
    get_all,
    set_all
)]
#[derive(Debug)]
pub(crate) enum SwValue {
    /// numerical value
    #[pyo3(constructor = (value))]
    V { value: f64 },
    /// numerical value
    #[pyo3(constructor = (value))]
    Vf { value: f64 },
    /// value group
    #[pyo3(constructor = (values, label = None))]
    Vg {
        /// content of the value group
        values: Py<PyList>, // = Vec<SwValue>,
        /// label of the value group
        label: Option<String>,
    },
    /// textual value
    Vt(String),
    /// Vtf element with numerical value
    #[pyo3(constructor = (value))]
    VtfNumber { value: f64 },
    /// Vtf element with textual value
    VtfText(String),
}

impl TryFrom<&autosar_data_abstraction::datatype::SwValue> for SwValue {
    type Error = PyErr;

    fn try_from(value: &autosar_data_abstraction::datatype::SwValue) -> Result<Self, Self::Error> {
        Ok(match value {
            autosar_data_abstraction::datatype::SwValue::V(v) => Self::V { value: *v },
            autosar_data_abstraction::datatype::SwValue::Vf(vf) => Self::Vf { value: *vf },
            autosar_data_abstraction::datatype::SwValue::Vg { label, vg_content } => {
                let vg_content = Python::attach(|py| {
                    slice_to_pylist(py, vg_content, |sw_value| {
                        SwValue::try_from(sw_value)?.into_py_any(py)
                    })
                })?;
                Self::Vg {
                    label: label.clone(),
                    values: vg_content,
                }
            }
            autosar_data_abstraction::datatype::SwValue::Vt(vt) => Self::Vt(vt.clone()),
            autosar_data_abstraction::datatype::SwValue::VtfNumber(vtf_number) => {
                Self::VtfNumber { value: *vtf_number }
            }
            autosar_data_abstraction::datatype::SwValue::VtfText(vtf_text) => {
                Self::VtfText(vtf_text.clone())
            }
        })
    }
}

impl TryFrom<&SwValue> for autosar_data_abstraction::datatype::SwValue {
    type Error = PyErr;
    fn try_from(value: &SwValue) -> Result<Self, Self::Error> {
        Ok(match value {
            SwValue::V { value } => Self::V(*value),
            SwValue::Vf { value } => Self::Vf(*value),
            SwValue::Vg { label, values } => {
                let vg_content = Python::attach(|py| {
                    pylist_to_vec(py, values, |sw_value| {
                        (&*sw_value.cast_exact::<SwValue>()?.borrow()).try_into()
                    })
                })?;
                Self::Vg {
                    label: label.clone(),
                    vg_content,
                }
            }
            SwValue::Vt(vt) => Self::Vt(vt.clone()),
            SwValue::VtfNumber { value } => Self::VtfNumber(*value),
            SwValue::VtfText(vtf_text) => Self::VtfText(vtf_text.clone()),
        })
    }
}

impl PartialEq for SwValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SwValue::V { value: v1 }, SwValue::V { value: v2 }) => v1 == v2,
            (SwValue::Vf { value: vf1 }, SwValue::Vf { value: vf2 }) => vf1 == vf2,
            (
                SwValue::Vg {
                    label: l1,
                    values: v1,
                },
                SwValue::Vg {
                    label: l2,
                    values: v2,
                },
            ) => Python::attach(|py| l1 == l2 && compare_pylist(py, v1, v2)),
            (SwValue::Vt(t1), SwValue::Vt(t2)) => t1 == t2,
            (SwValue::VtfNumber { value: n1 }, SwValue::VtfNumber { value: n2 }) => n1 == n2,
            (SwValue::VtfText(t1), SwValue::VtfText(t2)) => t1 == t2,
            _ => false,
        }
    }
}

#[pymethods]
impl SwValue {
    fn __repr__(&self) -> String {
        match self {
            SwValue::V { value } => format!("SwValue.V(value={})", value),
            SwValue::Vf { value } => format!("SwValue.Vf(value={})", value),
            SwValue::Vg { values, label } => {
                let label_str = label.as_deref().unwrap_or("None");
                format!("SwValue.Vg(values={}, label={})", values, label_str)
            }
            SwValue::Vt(text) => format!("SwValue.Vt(text={})", text),
            SwValue::VtfNumber { value } => format!("SwValue.VtfNumber(value={})", value),
            SwValue::VtfText(text) => format!("SwValue.VtfText(text={})", text),
        }
    }
}

//#########################################################

/// specification of the axis values of a compound primitive data type (curve, map) in a rule-based definition
#[pyclass(
    eq,
    module = "autosar_data._autosar_data._abstraction._datatype",
    get_all,
    set_all
)]
#[derive(Debug)]
pub(crate) struct RuleBasedAxisCont {
    /// category of the axis; one of `STD_AXIS`, `COM_AXIS`, `COM_AXIS`, `RES_AXIS`
    pub(crate) category: SwAxisContCategory,
    /// dimensions of the axis, used if the category is `RES_AXIS`, otherwise it should be empty
    pub(crate) sw_array_size: Vec<u64>,
    /// index of the axis. Here 1 is the x axis, 2 is the y axis, ...
    pub(crate) sw_axis_index: u64,
    /// axis values in the physical domain
    pub(crate) rule_based_values: Py<RuleBasedValueSpecification>,
    /// pyhsical unit of the axis values
    pub(crate) unit: Option<Unit>,
}

#[pymethods]
impl RuleBasedAxisCont {
    /// create a new rule-based axis content
    #[new]
    #[pyo3(signature = (
        category,
        sw_array_size,
        sw_axis_index,
        rule_based_values,
        /,
        *,
        unit = None
    ))]
    #[pyo3(
        text_signature = "(self,  category: SwAxisContCategory, sw_array_size: List[int], sw_axis_index: int, rule_based_values: RuleBasedValueSpecification, unit: Optional[Unit] = None)"
    )]
    fn new(
        category: SwAxisContCategory,
        sw_array_size: Vec<u64>,
        sw_axis_index: u64,
        rule_based_values: Py<RuleBasedValueSpecification>,
        unit: Option<Unit>,
    ) -> PyResult<Self> {
        Ok(Self {
            category,
            sw_array_size,
            sw_axis_index,
            rule_based_values,
            unit,
        })
    }

    fn __repr__(&self) -> String {
        let mut text = format!(
            "RuleBasedAxisCont(category={:?}, sw_array_size={:?}, sw_axis_index={}, rule_based_values={}",
            self.category, self.sw_array_size, self.sw_axis_index, self.rule_based_values,
        );
        if let Some(unit) = &self.unit {
            text.push_str(", unit=");
            text.push_str(
                &unit
                    .element()
                    .0
                    .path()
                    .unwrap_or_else(|_| "<invalid>".to_string()),
            );
        }
        text.push(')');
        text
    }
}

impl TryFrom<&autosar_data_abstraction::datatype::RuleBasedAxisCont> for RuleBasedAxisCont {
    type Error = PyErr;

    fn try_from(
        value: &autosar_data_abstraction::datatype::RuleBasedAxisCont,
    ) -> Result<Self, PyErr> {
        Python::attach(|py| {
            let rule_based_values =
                RuleBasedValueSpecification::try_from(&value.rule_based_values)?
                    .into_pyobject(py)?
                    .unbind();
            Ok(Self {
                category: value.category.into(),
                sw_array_size: value.sw_array_size.clone(),
                sw_axis_index: value.sw_axis_index,
                rule_based_values,
                unit: value.unit.as_ref().map(|unit| Unit(unit.clone())),
            })
        })
    }
}

impl TryFrom<&RuleBasedAxisCont> for autosar_data_abstraction::datatype::RuleBasedAxisCont {
    type Error = PyErr;

    fn try_from(value: &RuleBasedAxisCont) -> Result<Self, Self::Error> {
        Python::attach(|py| {
            let rule_based_values = &*value.rule_based_values.borrow(py);
            Ok(autosar_data_abstraction::datatype::RuleBasedAxisCont {
                category: value.category.into(),
                sw_array_size: value.sw_array_size.clone(),
                sw_axis_index: value.sw_axis_index,
                rule_based_values: rule_based_values.try_into()?,
                unit: value.unit.as_ref().map(|unit| unit.0.clone()),
            })
        })
    }
}

impl PartialEq for RuleBasedAxisCont {
    fn eq(&self, other: &Self) -> bool {
        Python::attach(|py| {
            self.category == other.category
                && self.sw_array_size == other.sw_array_size
                && self.sw_axis_index == other.sw_axis_index
                && *self.rule_based_values.bind_borrowed(py).borrow()
                    == *other.rule_based_values.bind_borrowed(py).borrow()
                && self.unit == other.unit
        })
    }
}

//#########################################################

/// specification of the values of a compound primitive data type (curve, map) in a rule-based definition
#[pyclass(
    eq,
    module = "autosar_data._autosar_data._abstraction._datatype",
    get_all,
    set_all
)]
#[derive(Debug)]
pub(crate) struct RuleBasedValueCont {
    /// values
    pub(crate) rule_based_values: Py<RuleBasedValueSpecification>,
    /// dimensions of the array
    pub(crate) sw_array_size: Vec<u64>,
    /// physical unit of the values
    pub(crate) unit: Option<Unit>,
}

#[pymethods]
impl RuleBasedValueCont {
    /// create a new rule-based value content
    #[new]
    #[pyo3(signature = (
        rule_based_values,
        sw_array_size,
        unit = None
    ))]
    #[pyo3(
        text_signature = "(self, rule_based_values: RuleBasedValueSpecification, sw_array_size: List[int], unit: Optional[Unit] = None)"
    )]
    fn new(
        rule_based_values: Py<RuleBasedValueSpecification>,
        sw_array_size: Vec<u64>,
        unit: Option<Unit>,
    ) -> PyResult<Self> {
        Ok(Self {
            rule_based_values,
            sw_array_size,
            unit,
        })
    }

    fn __repr__(&self) -> String {
        let mut text = format!(
            "RuleBasedValueCont(rule_based_values={}, sw_array_size={:?}",
            self.rule_based_values, self.sw_array_size
        );
        if let Some(unit) = &self.unit {
            text.push_str(", unit=");
            text.push_str(
                &unit
                    .element()
                    .0
                    .path()
                    .unwrap_or_else(|_| "<invalid>".to_string()),
            );
        }
        text.push(')');
        text
    }
}

impl TryFrom<&autosar_data_abstraction::datatype::RuleBasedValueCont> for RuleBasedValueCont {
    type Error = PyErr;

    fn try_from(
        value: &autosar_data_abstraction::datatype::RuleBasedValueCont,
    ) -> Result<Self, Self::Error> {
        Python::attach(|py| {
            let rule_based_values =
                RuleBasedValueSpecification::try_from(&value.rule_based_values)?
                    .into_pyobject(py)?
                    .unbind();
            Ok(Self {
                rule_based_values,
                sw_array_size: value.sw_array_size.clone(),
                unit: value.unit.as_ref().map(|unit| Unit(unit.clone())),
            })
        })
    }
}

impl TryFrom<&RuleBasedValueCont> for autosar_data_abstraction::datatype::RuleBasedValueCont {
    type Error = PyErr;

    fn try_from(value: &RuleBasedValueCont) -> Result<Self, Self::Error> {
        Python::attach(|py| {
            let rule_based_values = &*value.rule_based_values.borrow(py);
            Ok(autosar_data_abstraction::datatype::RuleBasedValueCont {
                rule_based_values: rule_based_values.try_into()?,
                sw_array_size: value.sw_array_size.clone(),
                unit: value.unit.as_ref().map(|unit| unit.0.clone()),
            })
        })
    }
}

impl PartialEq for RuleBasedValueCont {
    fn eq(&self, other: &Self) -> bool {
        Python::attach(|py| {
            *self.rule_based_values.bind_borrowed(py).borrow()
                == *other.rule_based_values.bind_borrowed(py).borrow()
                && self.sw_array_size == other.sw_array_size
                && self.unit == other.unit
        })
    }
}

//#########################################################

/// rule based value specification
#[pyclass(
    eq,
    module = "autosar_data._autosar_data._abstraction._datatype",
    get_all,
    set_all
)]
#[derive(Debug)]
pub(crate) struct RuleBasedValueSpecification {
    /// arguments of the rule-based value specification; they are filled in-order, andf the last one is repeated as required
    pub(crate) arguments: Py<PyList>, // = Vec<RuleArgument>,
    /// maximum size of the array to fill. It is used if the filling rule is set to `FILL_UNTIL_MAX_SIZE`
    pub(crate) max_size_to_fill: Option<u64>,
    /// rule to fill the array
    pub(crate) rule: RuleBasedFillUntil,
}

#[pymethods]
impl RuleBasedValueSpecification {
    /// create a new rule-based value specification
    #[new]
    #[pyo3(signature = (arguments, rule, /, *, max_size_to_fill = None))]
    #[pyo3(
        text_signature = "(self, arguments: List[RuleArgument], rule: RuleBasedFillUntil, /, *, max_size_to_fill: Optional[int] = None)"
    )]
    fn new(
        arguments: Py<PyList>,
        rule: RuleBasedFillUntil,
        max_size_to_fill: Option<u64>,
    ) -> PyResult<Self> {
        Ok(Self {
            arguments,
            max_size_to_fill,
            rule,
        })
    }

    fn __repr__(&self) -> String {
        let mut text = format!(
            "RuleBasedValueSpecification(arguments={}, rule={:?}",
            self.arguments, self.rule
        );
        if let Some(max_size_to_fill) = self.max_size_to_fill {
            text.push_str(&format!(", max_size_to_fill={}", max_size_to_fill));
        }
        text.push(')');
        text
    }
}

impl TryFrom<&autosar_data_abstraction::datatype::RuleBasedValueSpecification>
    for RuleBasedValueSpecification
{
    type Error = PyErr;
    fn try_from(
        value: &autosar_data_abstraction::datatype::RuleBasedValueSpecification,
    ) -> Result<Self, Self::Error> {
        Python::attach(|py| {
            let arguments = slice_to_pylist(py, &value.arguments, |rule_argument| {
                RuleArgument::from(rule_argument).into_py_any(py)
            })?;
            Ok(Self {
                arguments,
                max_size_to_fill: value.max_size_to_fill,
                rule: value.rule.into(),
            })
        })
    }
}

impl TryFrom<&RuleBasedValueSpecification>
    for autosar_data_abstraction::datatype::RuleBasedValueSpecification
{
    type Error = PyErr;

    fn try_from(value: &RuleBasedValueSpecification) -> Result<Self, Self::Error> {
        Python::attach(|py| {
            let arguments = pylist_to_vec(py, &value.arguments, |elem| {
                Ok((&*elem.cast::<RuleArgument>()?.borrow()).into())
            })?;
            Ok(
                autosar_data_abstraction::datatype::RuleBasedValueSpecification {
                    arguments,
                    max_size_to_fill: value.max_size_to_fill,
                    rule: value.rule.into(),
                },
            )
        })
    }
}

impl PartialEq for RuleBasedValueSpecification {
    fn eq(&self, other: &Self) -> bool {
        Python::attach(|py| {
            compare_pylist(py, &self.arguments, &other.arguments)
                && self.max_size_to_fill == other.max_size_to_fill
                && self.rule == other.rule
        })
    }
}

//#########################################################

pub(crate) fn composite_value_specification_to_pyany(
    value: &autosar_data_abstraction::datatype::CompositeValueSpecification,
) -> PyResult<Py<PyAny>> {
    use autosar_data_abstraction::datatype::CompositeValueSpecification;
    Python::attach(|py| match value {
        CompositeValueSpecification::Array(value) => {
            ArrayValueSpecification::try_from(value)?.into_py_any(py)
        }
        CompositeValueSpecification::Record(value) => {
            RecordValueSpecification::try_from(value)?.into_py_any(py)
        }
    })
}

pub(crate) fn pyany_to_composite_value_specification(
    pyobject: &Bound<'_, PyAny>,
) -> PyResult<autosar_data_abstraction::datatype::CompositeValueSpecification> {
    use autosar_data_abstraction::datatype::CompositeValueSpecification;
    if let Ok(array_value_specification) = pyobject.cast_exact::<ArrayValueSpecification>() {
        (&*array_value_specification.borrow())
            .try_into()
            .map(CompositeValueSpecification::Array)
    } else if let Ok(record_value_specification) = pyobject.cast_exact::<RecordValueSpecification>()
    {
        (&*record_value_specification.borrow())
            .try_into()
            .map(CompositeValueSpecification::Record)
    } else {
        Err(AutosarAbstractionError::new_err(
            "Unknown composite value specification type",
        ))
    }
}

//#########################################################

pub(crate) fn composite_rule_based_value_argument_to_pyany(
    value: &autosar_data_abstraction::datatype::CompositeRuleBasedValueArgument,
) -> PyResult<Py<PyAny>> {
    use autosar_data_abstraction::datatype::CompositeRuleBasedValueArgument;
    Python::attach(|py| match value {
        CompositeRuleBasedValueArgument::Application(value) => {
            ApplicationValueSpecification::try_from(value)?.into_py_any(py)
        }
        CompositeRuleBasedValueArgument::ApplicationRuleBased(value) => {
            ApplicationRuleBasedValueSpecification::try_from(value)?.into_py_any(py)
        }
    })
}

pub(crate) fn pyany_to_composite_rule_based_value_argument(
    pyobject: &Bound<'_, PyAny>,
) -> PyResult<autosar_data_abstraction::datatype::CompositeRuleBasedValueArgument> {
    if let Ok(application_value_specification) =
        pyobject.cast_exact::<ApplicationValueSpecification>()
    {
        (&*application_value_specification.borrow())
            .try_into()
            .map(autosar_data_abstraction::datatype::CompositeRuleBasedValueArgument::Application)
    } else if let Ok(application_rule_based_value_specification) =
        pyobject.cast_exact::<ApplicationRuleBasedValueSpecification>()
    {
        (&*application_rule_based_value_specification.borrow())
            .try_into()
            .map(autosar_data_abstraction::datatype::CompositeRuleBasedValueArgument::ApplicationRuleBased)
    } else {
        Err(AutosarAbstractionError::new_err(
            "Unknown composite rule-based value argument type",
        ))
    }
}

//#########################################################

/// argument of a rule-based value specification
#[pyclass(
    module = "autosar_data._autosar_data._abstraction._datatype",
    get_all,
    set_all,
    eq
)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum RuleArgument {
    /// V: argument is a numerical value
    V(f64),
    /// VF: argument is a numerical value
    Vf(f64),
    /// VT: argument is a text value
    Vt(String),
    /// VTF: argument is a numerical value
    VtfNumber(f64),
    /// VTF: argument is a text value
    VtfText(String),
}

impl From<&autosar_data_abstraction::datatype::RuleArgument> for RuleArgument {
    fn from(value: &autosar_data_abstraction::datatype::RuleArgument) -> Self {
        match value {
            autosar_data_abstraction::datatype::RuleArgument::V(value) => Self::V(*value),
            autosar_data_abstraction::datatype::RuleArgument::Vf(value) => Self::Vf(*value),
            autosar_data_abstraction::datatype::RuleArgument::Vt(value) => Self::Vt(value.clone()),
            autosar_data_abstraction::datatype::RuleArgument::VtfNumber(value) => {
                Self::VtfNumber(*value)
            }
            autosar_data_abstraction::datatype::RuleArgument::VtfText(value) => {
                Self::VtfText(value.clone())
            }
        }
    }
}

impl From<&RuleArgument> for autosar_data_abstraction::datatype::RuleArgument {
    fn from(value: &RuleArgument) -> Self {
        match value {
            RuleArgument::V(value) => Self::V(*value),
            RuleArgument::Vf(value) => Self::Vf(*value),
            RuleArgument::Vt(value) => Self::Vt(value.clone()),
            RuleArgument::VtfNumber(value) => Self::VtfNumber(*value),
            RuleArgument::VtfText(value) => Self::VtfText(value.clone()),
        }
    }
}

//#########################################################

pub(crate) fn data_prototype_to_pyany(
    data_prototype: autosar_data_abstraction::datatype::DataPrototype,
) -> PyResult<Py<PyAny>> {
    use autosar_data_abstraction::datatype::DataPrototype;
    Python::attach(|py| match data_prototype {
        DataPrototype::ArgumentDataPrototype(value) => ArgumentDataPrototype(value).into_py_any(py),
        DataPrototype::ParameterDataPrototype(value) => {
            ParameterDataPrototype(value).into_py_any(py)
        }
        DataPrototype::VariableDataPrototype(value) => VariableDataPrototype(value).into_py_any(py),
        DataPrototype::ApplicationArrayElement(value) => {
            ApplicationArrayElement(value).into_py_any(py)
        }
        DataPrototype::ApplicationRecordElement(value) => {
            ApplicationRecordElement(value).into_py_any(py)
        }
    })
}

pub(crate) fn pyany_to_data_prototype(
    pyobject: &Bound<'_, PyAny>,
) -> PyResult<autosar_data_abstraction::datatype::DataPrototype> {
    use autosar_data_abstraction::datatype::DataPrototype;
    if let Ok(argument_data_prototype) = pyobject.extract::<ArgumentDataPrototype>() {
        Ok(DataPrototype::ArgumentDataPrototype(
            argument_data_prototype.0,
        ))
    } else if let Ok(parameter_data_prototype) = pyobject.extract::<ParameterDataPrototype>() {
        Ok(DataPrototype::ParameterDataPrototype(
            parameter_data_prototype.0,
        ))
    } else if let Ok(variable_data_prototype) = pyobject.extract::<VariableDataPrototype>() {
        Ok(DataPrototype::VariableDataPrototype(
            variable_data_prototype.0,
        ))
    } else if let Ok(application_array_element) = pyobject.extract::<ApplicationArrayElement>() {
        Ok(DataPrototype::ApplicationArrayElement(
            application_array_element.0,
        ))
    } else if let Ok(application_record_element) = pyobject.extract::<ApplicationRecordElement>() {
        Ok(DataPrototype::ApplicationRecordElement(
            application_record_element.0,
        ))
    } else {
        Err(AutosarAbstractionError::new_err(
            "Unknown data prototype type",
        ))
    }
}
