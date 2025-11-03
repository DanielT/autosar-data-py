use crate::abstraction::{
    AutosarAbstractionError,
    datatype::{CompuMethod, DataConstr, SwBaseType},
};
use crate::{abstraction::*, *};
use autosar_data_abstraction::{
    self, AbstractionElement, IdentifiableAbstractionElement,
    datatype::AbstractImplementationDataType,
};
use std::ops::Deref;

//##################################################################

/// An implementation data type; specifics are determined by the category
///
/// Use [`ArPackage::create_implementation_data_type`] to create a new implementation data type
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._datatype"
)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ImplementationDataType(
    pub(crate) autosar_data_abstraction::datatype::ImplementationDataType,
);

#[pymethods]
impl ImplementationDataType {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::datatype::ImplementationDataType::try_from(
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

    /// get the category of this implementation data type
    #[getter]
    fn category(&self) -> Option<ImplementationDataCategory> {
        self.0.category().map(std::convert::Into::into)
    }

    /// create an iterator over the sub-elements of this implementation data type
    fn sub_elements(&self) -> ImplementationDataTypeElementIterator {
        ImplementationDataTypeElementIterator::new(
            self.0.sub_elements().map(ImplementationDataTypeElement),
        )
    }

    /// get the `SwBaseType` of this implementation data type [category: VALUE]
    #[getter]
    fn base_type(&self) -> Option<SwBaseType> {
        self.0.base_type().map(SwBaseType)
    }

    /// get the `CompuMethod` of this implementation data type [category: VALUE, `TYPE_REFERENCE`]
    #[getter]
    fn compu_method(&self) -> Option<CompuMethod> {
        self.0.compu_method().map(CompuMethod)
    }

    /// get the `DataConstr` of this implementation data type [category: VALUE, `TYPE_REFERENCE`]
    #[getter]
    fn data_constraint(&self) -> Option<DataConstr> {
        self.0.data_constraint().map(DataConstr)
    }

    /// get the referenced implementation data type [category: `TYPE_REFERENCE`]
    #[getter]
    fn referenced_type(&self) -> Option<ImplementationDataType> {
        self.0.referenced_type().map(ImplementationDataType)
    }

    /// get the array size of this implementation data type [category: ARRAY]
    #[getter]
    fn array_size(&self) -> Option<u32> {
        self.0.array_size()
    }

    /// get the data pointer target of this implementation data type [category: DATA_REFERENCE]
    #[getter]
    fn data_pointer_target(&self, py: Python) -> Option<Py<PyAny>> {
        self.0
            .data_pointer_target()
            .and_then(|target| match target {
                autosar_data_abstraction::datatype::DataPointerTarget::BaseType(base_type) => {
                    SwBaseType(base_type).into_py_any(py).ok()
                }
                autosar_data_abstraction::datatype::DataPointerTarget::ImplementationDataType(
                    reftype,
                ) => ImplementationDataType(reftype).into_py_any(py).ok(),
            })
    }

    /// apply the settings to this implementation data type
    ///
    /// Calling this method completely replaces the existing settings of the implementation data type,
    /// deleting existing sub-elements and creating new ones according to the settings
    #[pyo3(signature = (settings, /))]
    #[pyo3(text_signature = "(self, settings: ImplementationDataTypeSettings, /)")]
    fn apply_settings(&self, settings: &Bound<'_, PyAny>) -> PyResult<()> {
        let settings = pyany_to_implmentation_settings(settings)?;
        self.0
            .apply_settings(&settings)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the settings of this implementation data type
    fn settings(&self) -> Option<Py<PyAny>> {
        self.0
            .settings()
            .and_then(|settings| implementation_settings_to_pyany(&settings).ok())
    }
}

//##################################################################

/// An element of an implementation data type
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._datatype"
)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ImplementationDataTypeElement(
    pub(crate) autosar_data_abstraction::datatype::ImplementationDataTypeElement,
);

#[pymethods]
impl ImplementationDataTypeElement {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::datatype::ImplementationDataTypeElement::try_from(
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

    /// get the category of this implementation data type
    #[getter]
    fn category(&self) -> Option<ImplementationDataCategory> {
        self.0.category().map(std::convert::Into::into)
    }

    /// create an iterator over the sub-elements of this implementation data type
    fn sub_elements(&self) -> ImplementationDataTypeElementIterator {
        ImplementationDataTypeElementIterator::new(
            self.0.sub_elements().map(ImplementationDataTypeElement),
        )
    }

    /// get the `SwBaseType` of this implementation data type [category: VALUE]
    #[getter]
    fn base_type(&self) -> Option<SwBaseType> {
        self.0.base_type().map(SwBaseType)
    }

    /// get the `CompuMethod` of this implementation data type [category: VALUE, `TYPE_REFERENCE`]
    #[getter]
    fn compu_method(&self) -> Option<CompuMethod> {
        self.0.compu_method().map(CompuMethod)
    }

    /// get the `DataConstr` of this implementation data type [category: VALUE, `TYPE_REFERENCE`]
    #[getter]
    fn data_constraint(&self) -> Option<DataConstr> {
        self.0.data_constraint().map(DataConstr)
    }

    /// get the referenced implementation data type [category: `TYPE_REFERENCE`]
    #[getter]
    fn referenced_type(&self) -> Option<ImplementationDataType> {
        self.0.referenced_type().map(ImplementationDataType)
    }

    /// get the array size of this implementation data type [category: ARRAY]
    #[getter]
    fn array_size(&self) -> Option<u32> {
        self.0.array_size()
    }

    /// get the data pointer target of this implementation data type [category: DATA_REFERENCE]
    #[getter]
    fn data_pointer_target(&self, py: Python) -> Option<Py<PyAny>> {
        self.0
            .data_pointer_target()
            .and_then(|target| match target {
                autosar_data_abstraction::datatype::DataPointerTarget::BaseType(base_type) => {
                    SwBaseType(base_type).into_py_any(py).ok()
                }
                autosar_data_abstraction::datatype::DataPointerTarget::ImplementationDataType(
                    reftype,
                ) => ImplementationDataType(reftype).into_py_any(py).ok(),
            })
    }

    /// apply the settings to this implementation data type
    ///
    /// Calling this method completely replaces the existing settings of the implementation data type,
    /// deleting existing sub-elements and creating new ones according to the settings
    #[pyo3(signature = (settings, /))]
    #[pyo3(text_signature = "(self, settings: ImplementationDataTypeSettings, /)")]
    fn apply_settings(&self, settings: &Bound<'_, PyAny>) -> PyResult<()> {
        let settings = pyany_to_implmentation_settings(settings)?;
        self.0
            .apply_settings(&settings)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the settings of this implementation data type
    fn settings(&self) -> Option<Py<PyAny>> {
        self.0
            .settings()
            .and_then(|settings| implementation_settings_to_pyany(&settings).ok())
    }
}

//##################################################################

iterator_wrapper!(
    ImplementationDataTypeElementIterator,
    ImplementationDataTypeElement
);

//##################################################################

/// The category of an implementation data type
#[pyclass(
    frozen,
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._datatype"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImplementationDataCategory {
    /// A simple value
    Value,
    /// a pointer to data
    DataReference,
    /// a pointer to a function
    FunctionReference,
    /// this type is a reference to another type
    TypeReference,
    /// a structure of elements
    Structure,
    /// a union of elements
    Union,
    /// an array
    Array,
}

impl From<autosar_data_abstraction::datatype::ImplementationDataCategory>
    for ImplementationDataCategory
{
    fn from(category: autosar_data_abstraction::datatype::ImplementationDataCategory) -> Self {
        match category {
            autosar_data_abstraction::datatype::ImplementationDataCategory::Value => Self::Value,
            autosar_data_abstraction::datatype::ImplementationDataCategory::DataReference => {
                Self::DataReference
            }
            autosar_data_abstraction::datatype::ImplementationDataCategory::FunctionReference => {
                Self::FunctionReference
            }
            autosar_data_abstraction::datatype::ImplementationDataCategory::TypeReference => {
                Self::TypeReference
            }
            autosar_data_abstraction::datatype::ImplementationDataCategory::Structure => {
                Self::Structure
            }
            autosar_data_abstraction::datatype::ImplementationDataCategory::Union => Self::Union,
            autosar_data_abstraction::datatype::ImplementationDataCategory::Array => Self::Array,
        }
    }
}

//##################################################################

/// Settings for an implementation data type
///
/// This structure is used to create new implementation data types
#[pyclass(
    module = "autosar_data._autosar_data._abstraction._datatype",
    get_all,
    set_all,
    eq,
    subclass
)]
#[derive(PartialEq)]
pub(crate) struct ImplementationDataTypeSettings();

enum ImplementationDataTypeSettingsInternal<'py> {
    Value(PyRef<'py, ImplementationDataTypeSettings_Value>),
    Array(PyRef<'py, ImplementationDataTypeSettings_Array>),
    Structure(PyRef<'py, ImplementationDataTypeSettings_Structure>),
    Union(PyRef<'py, ImplementationDataTypeSettings_Union>),
    DataReference(PyRef<'py, ImplementationDataTypeSettings_DataReference>),
    FunctionReference(PyRef<'py, ImplementationDataTypeSettings_FunctionReference>),
    TypeReference(PyRef<'py, ImplementationDataTypeSettings_TypeReference>),
    Invalid,
}

impl<'py> From<&Bound<'py, PyAny>> for ImplementationDataTypeSettingsInternal<'py> {
    fn from(settings: &Bound<'py, PyAny>) -> Self {
        match settings
            .get_type()
            .name()
            .map(|pystring| pystring.to_string())
            .unwrap_or_default()
            .as_str()
        {
            "ImplementationDataTypeSettings_Value" => Self::Value(
                settings
                    .cast_exact::<ImplementationDataTypeSettings_Value>()
                    .unwrap()
                    .borrow(),
            ),
            "ImplementationDataTypeSettings_Array" => Self::Array(
                settings
                    .cast_exact::<ImplementationDataTypeSettings_Array>()
                    .unwrap()
                    .borrow(),
            ),
            "ImplementationDataTypeSettings_Structure" => Self::Structure(
                settings
                    .cast_exact::<ImplementationDataTypeSettings_Structure>()
                    .unwrap()
                    .borrow(),
            ),
            "ImplementationDataTypeSettings_Union" => Self::Union(
                settings
                    .cast_exact::<ImplementationDataTypeSettings_Union>()
                    .unwrap()
                    .borrow(),
            ),
            "ImplementationDataTypeSettings_DataReference" => Self::DataReference(
                settings
                    .cast_exact::<ImplementationDataTypeSettings_DataReference>()
                    .unwrap()
                    .borrow(),
            ),
            "ImplementationDataTypeSettings_FunctionReference" => Self::FunctionReference(
                settings
                    .cast_exact::<ImplementationDataTypeSettings_FunctionReference>()
                    .unwrap()
                    .borrow(),
            ),
            "ImplementationDataTypeSettings_TypeReference" => Self::TypeReference(
                settings
                    .cast_exact::<ImplementationDataTypeSettings_TypeReference>()
                    .unwrap()
                    .borrow(),
            ),
            _ => Self::Invalid,
        }
    }
}

impl PartialEq for ImplementationDataTypeSettingsInternal<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Value(x), Self::Value(y)) => **x == **y,
            (Self::Array(x), Self::Array(y)) => **x == **y,
            (Self::Structure(x), Self::Structure(y)) => **x == **y,
            (Self::Union(x), Self::Union(y)) => **x == **y,
            (Self::DataReference(x), Self::DataReference(y)) => **x == **y,
            (Self::FunctionReference(x), Self::FunctionReference(y)) => **x == **y,
            (Self::TypeReference(x), Self::TypeReference(y)) => **x == **y,
            _ => false,
        }
    }
}

//##################################################################

/// A simple value
#[allow(non_camel_case_types)]
#[pyclass(
    module = "autosar_data._autosar_data._abstraction._datatype",
    get_all,
    set_all,
    eq,
    extends=ImplementationDataTypeSettings
)]
#[derive(PartialEq)]
pub(crate) struct ImplementationDataTypeSettings_Value {
    /// the name of the data type
    name: String,
    /// the base type of the data type
    base_type: SwBaseType,
    /// the `CompuMethod` of the data type
    compu_method: Option<CompuMethod>,
    /// the data constraints of the data type
    data_constraint: Option<DataConstr>,
}

#[pymethods]
impl ImplementationDataTypeSettings_Value {
    #[pyo3(signature = (name, *, base_type, compu_method=None, data_constraint=None))]
    #[pyo3(
        text_signature = "(self, name: str, *, base_type: SwBaseType, compu_method: Optional[CompuMethod]=None, data_constraint: Optional[DataConstr]=None)"
    )]
    #[new]
    fn new(
        name: &str,
        base_type: SwBaseType,
        compu_method: Option<CompuMethod>,
        data_constraint: Option<DataConstr>,
    ) -> (Self, ImplementationDataTypeSettings) {
        (
            Self {
                name: name.to_string(),
                base_type,
                compu_method,
                data_constraint,
            },
            ImplementationDataTypeSettings(),
        )
    }

    fn __repr__(&self) -> String {
        format!(
            "ImplementationDataTypeSettings.Value(name={}, base_type={:#?}, compu_method={:#?}, data_constraint={:#?})",
            self.name, self.base_type, self.compu_method, self.data_constraint
        )
    }
}

impl From<&ImplementationDataTypeSettings_Value>
    for autosar_data_abstraction::datatype::ImplementationDataTypeSettings
{
    fn from(settings: &ImplementationDataTypeSettings_Value) -> Self {
        autosar_data_abstraction::datatype::ImplementationDataTypeSettings::Value {
            name: settings.name.clone(),
            base_type: settings.base_type.0.clone(),
            compu_method: settings.compu_method.as_ref().map(|cm| cm.0.clone()),
            data_constraint: settings.data_constraint.as_ref().map(|dc| dc.0.clone()),
        }
    }
}

//##################################################################

/// An array of elements
#[allow(non_camel_case_types)]
#[pyclass(
    module = "autosar_data._autosar_data._abstraction._datatype",
    get_all,
    set_all,
    eq,
    extends=ImplementationDataTypeSettings
)]
pub(crate) struct ImplementationDataTypeSettings_Array {
    /// the name of the data type
    name: String,
    /// the length of the array
    length: u32,
    /// settings to construct the element type of the array
    element_type: Py<PyAny>, // = ImplementationDataTypeSettings
}

#[pymethods]
impl ImplementationDataTypeSettings_Array {
    #[pyo3(signature = (name, *, length, element_type))]
    #[pyo3(
        text_signature = "(self, name: str, *, length: int, element_type: ImplementationDataTypeSettings)"
    )]
    #[new]
    fn new(
        name: &str,
        length: u32,
        element_type: Py<PyAny>,
    ) -> (Self, ImplementationDataTypeSettings) {
        (
            Self {
                name: name.to_string(),
                length,
                element_type,
            },
            ImplementationDataTypeSettings(),
        )
    }

    fn __repr__(&self) -> String {
        format!(
            "ImplementationDataTypeSettings.Array(name={}, length={}, element_type={})",
            self.name, self.length, self.element_type
        )
    }
}

impl PartialEq for ImplementationDataTypeSettings_Array {
    fn eq(&self, other: &Self) -> bool {
        Python::attach(|py| {
            self.name == other.name
                && self.length == other.length
                && ImplementationDataTypeSettingsInternal::from(self.element_type.bind(py))
                    == ImplementationDataTypeSettingsInternal::from(other.element_type.bind(py))
        })
    }
}

impl TryFrom<&ImplementationDataTypeSettings_Array>
    for autosar_data_abstraction::datatype::ImplementationDataTypeSettings
{
    type Error = PyErr;

    fn try_from(settings: &ImplementationDataTypeSettings_Array) -> PyResult<Self> {
        let element_type =
            Python::attach(|py| pyany_to_implmentation_settings(settings.element_type.bind(py)))?;
        Ok(
            autosar_data_abstraction::datatype::ImplementationDataTypeSettings::Array {
                name: settings.name.clone(),
                length: settings.length,
                element_type: Box::new(element_type),
            },
        )
    }
}

//##################################################################

/// A structure of elements
#[allow(non_camel_case_types)]
#[pyclass(
    module = "autosar_data._autosar_data._abstraction._datatype",
    get_all,
    set_all,
    eq,
    extends=ImplementationDataTypeSettings
)]
pub(crate) struct ImplementationDataTypeSettings_Structure {
    /// the name of the structure
    name: String,
    /// settings for the elements of the structure
    elements: Py<PyList>, // = Vec<ImplementationDataTypeSettings>
}

#[pymethods]
impl ImplementationDataTypeSettings_Structure {
    #[pyo3(signature = (name, *, elements))]
    #[pyo3(text_signature = "(self, name: str, *, elements: List[ImplementationDataTypeSettings])")]
    #[new]
    fn new(name: &str, elements: Py<PyList>) -> (Self, ImplementationDataTypeSettings) {
        (
            Self {
                name: name.to_string(),
                elements,
            },
            ImplementationDataTypeSettings(),
        )
    }

    fn __repr__(&self) -> String {
        format!(
            "ImplementationDataTypeSettings.Structure(name={}, elements={})",
            self.name, self.elements
        )
    }
}

impl PartialEq for ImplementationDataTypeSettings_Structure {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && compare_settings_pylist(&self.elements, &other.elements)
    }
}

impl TryFrom<&ImplementationDataTypeSettings_Structure>
    for autosar_data_abstraction::datatype::ImplementationDataTypeSettings
{
    type Error = PyErr;

    fn try_from(settings: &ImplementationDataTypeSettings_Structure) -> PyResult<Self> {
        let elements = Python::attach(|py| {
            settings
                .elements
                .bind(py)
                .as_sequence()
                .try_iter()?
                .map(|elem| pyany_to_implmentation_settings(&elem?))
                .collect::<PyResult<Vec<_>>>()
        })?;
        Ok(
            autosar_data_abstraction::datatype::ImplementationDataTypeSettings::Structure {
                name: settings.name.clone(),
                elements,
            },
        )
    }
}

//##################################################################

/// A union of elements
#[allow(non_camel_case_types)]
#[pyclass(
    module = "autosar_data._autosar_data._abstraction._datatype",
    get_all,
    set_all,
    eq,
    extends=ImplementationDataTypeSettings
)]
pub(crate) struct ImplementationDataTypeSettings_Union {
    /// the name of the union
    name: String,
    /// settings for the elements of the union
    elements: Py<PyList>, // = Vec<ImplementationDataTypeSettings>
}

#[pymethods]
impl ImplementationDataTypeSettings_Union {
    #[pyo3(signature = (name, *, elements))]
    #[pyo3(text_signature = "(self, name: str, *, elements: List[ImplementationDataTypeSettings])")]
    #[new]
    fn new(name: &str, elements: Py<PyList>) -> (Self, ImplementationDataTypeSettings) {
        (
            Self {
                name: name.to_string(),
                elements,
            },
            ImplementationDataTypeSettings(),
        )
    }

    fn __repr__(&self) -> String {
        format!(
            "ImplementationDataTypeSettings.Union(name={}, elements={})",
            self.name, self.elements
        )
    }
}

impl PartialEq for ImplementationDataTypeSettings_Union {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && compare_settings_pylist(&self.elements, &other.elements)
    }
}

impl TryFrom<&ImplementationDataTypeSettings_Union>
    for autosar_data_abstraction::datatype::ImplementationDataTypeSettings
{
    type Error = PyErr;

    fn try_from(settings: &ImplementationDataTypeSettings_Union) -> PyResult<Self> {
        let elements = Python::attach(|py| {
            settings
                .elements
                .bind(py)
                .as_sequence()
                .try_iter()?
                .map(|elem| pyany_to_implmentation_settings(&elem?))
                .collect::<PyResult<Vec<_>>>()
        })?;
        Ok(
            autosar_data_abstraction::datatype::ImplementationDataTypeSettings::Union {
                name: settings.name.clone(),
                elements,
            },
        )
    }
}

//##################################################################

/// A pointer to data
#[allow(non_camel_case_types)]
#[pyclass(
    module = "autosar_data._autosar_data._abstraction._datatype",
    get_all,
    set_all,
    eq,
    extends=ImplementationDataTypeSettings
)]
pub(crate) struct ImplementationDataTypeSettings_DataReference {
    /// the name of the data type
    name: String,
    /// the target of the data pointer; either an SwBaseType or an ImplementationDataType
    target: Py<PyAny>,
}

#[pymethods]
impl ImplementationDataTypeSettings_DataReference {
    #[pyo3(signature = (name, *, target))]
    #[pyo3(text_signature = "(self, name: str, *, target: DataPointerTarget)")]
    #[new]
    fn new(name: &str, target: Py<PyAny>) -> PyResult<(Self, ImplementationDataTypeSettings)> {
        pyany_to_data_pointer_target(&target)?;
        Ok((
            Self {
                name: name.to_string(),
                target,
            },
            ImplementationDataTypeSettings(),
        ))
    }

    fn __repr__(&self) -> String {
        format!(
            "ImplementationDataTypeSettings.DataReference(name={})",
            self.name
        )
    }
}

impl PartialEq for ImplementationDataTypeSettings_DataReference {
    fn eq(&self, other: &Self) -> bool {
        Python::attach(|py| {
            if let (Ok(self_target), Ok(other_target)) = (
                self.target.cast_bound::<SwBaseType>(py),
                other.target.cast_bound::<SwBaseType>(py),
            ) {
                self.name == other.name && *self_target.borrow() == *other_target.borrow()
            } else if let (Ok(self_target), Ok(other_target)) = (
                self.target.cast_bound::<ImplementationDataType>(py),
                other.target.cast_bound::<ImplementationDataType>(py),
            ) {
                self.name == other.name && *self_target.borrow() == *other_target.borrow()
            } else {
                false
            }
        })
    }
}

impl TryFrom<&ImplementationDataTypeSettings_DataReference>
    for autosar_data_abstraction::datatype::ImplementationDataTypeSettings
{
    type Error = PyErr;

    fn try_from(settings: &ImplementationDataTypeSettings_DataReference) -> PyResult<Self> {
        let target = pyany_to_data_pointer_target(&settings.target)?;
        Ok(
            autosar_data_abstraction::datatype::ImplementationDataTypeSettings::DataReference {
                name: settings.name.clone(),
                target,
            },
        )
    }
}

fn pyany_to_data_pointer_target(
    target: &Py<PyAny>,
) -> PyResult<autosar_data_abstraction::datatype::DataPointerTarget> {
    Python::attach(|py| {
        if let Ok(target) = target.extract::<SwBaseType>(py) {
            Ok(autosar_data_abstraction::datatype::DataPointerTarget::BaseType(target.0))
        } else if let Ok(target) = target.extract::<ImplementationDataType>(py) {
            Ok(
                autosar_data_abstraction::datatype::DataPointerTarget::ImplementationDataType(
                    target.0,
                ),
            )
        } else {
            Err(AutosarAbstractionError::new_err(
                "Invalid data pointer target".to_string(),
            ))
        }
    })
}

//##################################################################

/// A pointer to a function
#[allow(non_camel_case_types)]
#[pyclass(
    module = "autosar_data._autosar_data._abstraction._datatype",
    get_all,
    set_all,
    eq,
    extends=ImplementationDataTypeSettings
)]
#[derive(PartialEq)]
pub(crate) struct ImplementationDataTypeSettings_FunctionReference {
    /// the name of the data type
    name: String,
    // TODO: Add reference to the referenced function type
}

#[pymethods]
impl ImplementationDataTypeSettings_FunctionReference {
    #[pyo3(signature = (name, /))]
    #[pyo3(text_signature = "(self, name: str, /)")]
    #[new]
    fn new(name: &str) -> (Self, ImplementationDataTypeSettings) {
        (
            Self {
                name: name.to_string(),
            },
            ImplementationDataTypeSettings(),
        )
    }

    fn __repr__(&self) -> String {
        format!(
            "ImplementationDataTypeSettings.FunctionReference(name={})",
            self.name
        )
    }
}

impl From<&ImplementationDataTypeSettings_FunctionReference>
    for autosar_data_abstraction::datatype::ImplementationDataTypeSettings
{
    fn from(settings: &ImplementationDataTypeSettings_FunctionReference) -> Self {
        autosar_data_abstraction::datatype::ImplementationDataTypeSettings::FunctionReference {
            name: settings.name.clone(),
        }
    }
}

//##################################################################

/// A reference to another implementation data type
#[allow(non_camel_case_types)]
#[pyclass(
    module = "autosar_data._autosar_data._abstraction._datatype",
    get_all,
    set_all,
    eq,
    extends=ImplementationDataTypeSettings
)]
#[derive(PartialEq)]
pub(crate) struct ImplementationDataTypeSettings_TypeReference {
    /// the name of the data type
    name: String,
    /// the referenced data type
    reftype: ImplementationDataType,
    /// the `CompuMethod` of the data type
    compu_method: Option<CompuMethod>,
    /// the data constraints of the data type
    data_constraint: Option<DataConstr>,
}

#[pymethods]
impl ImplementationDataTypeSettings_TypeReference {
    #[pyo3(signature = (name, *, reftype, compu_method=None, data_constraint=None))]
    #[pyo3(
        text_signature = "(self, name: str, *, reftype: ImplementationDataType, compu_method: Optional[CompuMethod]=None, data_constraint: Optional[DataConstr]=None)"
    )]
    #[new]
    fn new(
        name: &str,
        reftype: ImplementationDataType,
        compu_method: Option<CompuMethod>,
        data_constraint: Option<DataConstr>,
    ) -> (Self, ImplementationDataTypeSettings) {
        (
            Self {
                name: name.to_string(),
                reftype,
                compu_method,
                data_constraint,
            },
            ImplementationDataTypeSettings(),
        )
    }

    fn __repr__(&self) -> String {
        format!(
            "ImplementationDataTypeSettings.TypeReference(name={}, reftype={:?}, compu_method={:?}, data_constraint={:?})",
            self.name, self.reftype, self.compu_method, self.data_constraint
        )
    }
}

impl From<&ImplementationDataTypeSettings_TypeReference>
    for autosar_data_abstraction::datatype::ImplementationDataTypeSettings
{
    fn from(settings: &ImplementationDataTypeSettings_TypeReference) -> Self {
        autosar_data_abstraction::datatype::ImplementationDataTypeSettings::TypeReference {
            name: settings.name.clone(),
            reftype: settings.reftype.0.clone(),
            compu_method: settings.compu_method.as_ref().map(|cm| cm.0.clone()),
            data_constraint: settings.data_constraint.as_ref().map(|dc| dc.0.clone()),
        }
    }
}

//##################################################################

fn compare_settings_pylist(seq1: &Py<PyList>, seq2: &Py<PyList>) -> bool {
    Python::attach(|py| {
        if let (Ok(seq1_len), Ok(seq2_len)) = (
            seq1.bind_borrowed(py).as_sequence().len(),
            seq2.bind_borrowed(py).as_sequence().len(),
        ) {
            // first, make sure the lengths are the same, since iter.zip stops when the shorter iterator is exhausted
            if seq1_len != seq2_len {
                // lengths are not equal
                return false;
            }

            let mut seq1_try_iter = seq1.bind_borrowed(py).as_sequence().try_iter();
            let mut seq2_try_iter = seq2.bind_borrowed(py).as_sequence().try_iter();
            if let (Ok(seq1_iter), Ok(seq2_iter)) = (&mut seq1_try_iter, &mut seq2_try_iter) {
                seq1_iter.zip(seq2_iter).all(|(scale1, scale2)| {
                    // try to get a ref to the PyClass T from the Bound<PyAny> on each side
                    let item1 = scale1
                        .as_ref()
                        .ok()
                        .map(ImplementationDataTypeSettingsInternal::from);
                    let item2 = scale2
                        .as_ref()
                        .ok()
                        .map(ImplementationDataTypeSettingsInternal::from);

                    item1 == item2
                })
            } else {
                // could not get iterators for the sequences - it's not clear that his case is reachable, since we're able to get lengths
                false
            }
        } else {
            // could not get lengths for the sequences. At least one of them is not a sequence.
            false
        }
    })
}

//##################################################################

pub(crate) fn pyany_to_implmentation_settings(
    settings: &Bound<'_, PyAny>,
) -> PyResult<autosar_data_abstraction::datatype::ImplementationDataTypeSettings> {
    let settings = ImplementationDataTypeSettingsInternal::from(settings);
    match settings {
        ImplementationDataTypeSettingsInternal::Value(settings) => Ok(settings.deref().into()),
        ImplementationDataTypeSettingsInternal::Array(settings) => settings.deref().try_into(),
        ImplementationDataTypeSettingsInternal::Structure(settings) => settings.deref().try_into(),
        ImplementationDataTypeSettingsInternal::Union(settings) => settings.deref().try_into(),
        ImplementationDataTypeSettingsInternal::DataReference(settings) => {
            settings.deref().try_into()
        }
        ImplementationDataTypeSettingsInternal::FunctionReference(settings) => {
            Ok(settings.deref().into())
        }
        ImplementationDataTypeSettingsInternal::TypeReference(settings) => {
            Ok(settings.deref().into())
        }
        ImplementationDataTypeSettingsInternal::Invalid => Err(AutosarAbstractionError::new_err(
            "Invalid implementation data type settings".to_string(),
        )),
    }
}

pub(crate) fn implementation_settings_to_pyany(
    settings: &autosar_data_abstraction::datatype::ImplementationDataTypeSettings,
) -> PyResult<Py<PyAny>> {
    Python::attach(|py| match settings {
        autosar_data_abstraction::datatype::ImplementationDataTypeSettings::Value {
            name,
            base_type,
            compu_method,
            data_constraint,
        } => {
            let settings = ImplementationDataTypeSettings_Value::new(
                name,
                SwBaseType(base_type.clone()),
                compu_method.as_ref().map(|cm| CompuMethod(cm.clone())),
                data_constraint.as_ref().map(|dc| DataConstr(dc.clone())),
            );
            Py::new(py, settings)?.into_py_any(py)
        }
        autosar_data_abstraction::datatype::ImplementationDataTypeSettings::Array {
            name,
            length,
            element_type,
        } => {
            let element_type = implementation_settings_to_pyany(element_type)?;
            let settings = ImplementationDataTypeSettings_Array::new(name, *length, element_type);
            Py::new(py, settings)?.into_py_any(py)
        }
        autosar_data_abstraction::datatype::ImplementationDataTypeSettings::Structure {
            name,
            elements,
        } => {
            let py_elements = PyList::empty(py);
            for elem in elements {
                let elem = implementation_settings_to_pyany(elem)?;
                py_elements.append(elem)?;
            }
            let settings =
                ImplementationDataTypeSettings_Structure::new(name, py_elements.unbind());
            Py::new(py, settings)?.into_py_any(py)
        }
        autosar_data_abstraction::datatype::ImplementationDataTypeSettings::Union {
            name,
            elements,
        } => {
            let py_elements = PyList::empty(py);
            for elem in elements {
                let elem = implementation_settings_to_pyany(elem)?;
                py_elements.append(elem)?;
            }
            let settings = ImplementationDataTypeSettings_Union::new(name, py_elements.unbind());
            Py::new(py, settings)?.into_py_any(py)
        }
        autosar_data_abstraction::datatype::ImplementationDataTypeSettings::DataReference {
            name,
            target,
        } => {
            let py_target = match target {
                autosar_data_abstraction::datatype::DataPointerTarget::BaseType(base_type) => {
                    Py::new(py, SwBaseType(base_type.clone()))?.into_py_any(py)
                }
                autosar_data_abstraction::datatype::DataPointerTarget::ImplementationDataType(
                    reftype,
                ) => Py::new(py, ImplementationDataType(reftype.clone()))?.into_py_any(py),
            }?;
            let settings = ImplementationDataTypeSettings_DataReference::new(name, py_target)?;
            Py::new(py, settings)?.into_py_any(py)
        }
        autosar_data_abstraction::datatype::ImplementationDataTypeSettings::FunctionReference {
            name,
        } => {
            let settings = ImplementationDataTypeSettings_FunctionReference::new(name);
            Py::new(py, settings)?.into_py_any(py)
        }
        autosar_data_abstraction::datatype::ImplementationDataTypeSettings::TypeReference {
            name,
            reftype,
            compu_method,
            data_constraint,
        } => {
            let settings = ImplementationDataTypeSettings_TypeReference::new(
                name,
                ImplementationDataType(reftype.clone()),
                compu_method.as_ref().map(|cm| CompuMethod(cm.clone())),
                data_constraint.as_ref().map(|dc| DataConstr(dc.clone())),
            );
            Py::new(py, settings)?.into_py_any(py)
        }
    })
}
