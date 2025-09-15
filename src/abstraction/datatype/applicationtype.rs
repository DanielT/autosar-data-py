use crate::{
    Element,
    abstraction::{
        AutosarAbstractionError, abstraction_err_to_pyerr,
        datatype::{CompuMethod, DataConstr, Unit},
    },
    iterator_wrapper,
};
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};
use pyo3::{IntoPyObjectExt, prelude::*};

//##################################################################

/// An application array data type
///
/// Use [`ArPackage::create_application_array_data_type`] to create a new application array data type.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._datatype"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct ApplicationArrayDataType(
    pub(crate) autosar_data_abstraction::datatype::ApplicationArrayDataType,
);

#[pymethods]
impl ApplicationArrayDataType {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::datatype::ApplicationArrayDataType::try_from(
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

    /// get the array element of the array data type
    #[getter]
    fn array_element(&self) -> Option<ApplicationArrayElement> {
        self.0.array_element().map(ApplicationArrayElement)
    }

    /// set the size of the array
    #[pyo3(signature = (size, /))]
    #[pyo3(text_signature = "(self, size: ApplicationArraySize, /)")]
    fn set_size(&self, size: ApplicationArraySize) -> PyResult<()> {
        self.0
            .set_size(size.into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the size of the array
    fn size(&self) -> Option<ApplicationArraySize> {
        self.0.size().map(std::convert::Into::into)
    }
}

//#########################################################

/// definition of the size type of an application array data type
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._datatype"
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ApplicationArraySize {
    /// Fixed size array, with the given size
    #[pyo3(constructor=(length, /))]
    Fixed { length: u64 },
    /// Variable size array, with a single dimension. The maximum size is given
    VariableLinear { max_size: u64 },
    /// Variable size "square" array, with two or more dimensions. All dimensions have the same maximum size
    /// This maximum size is set in the innermost dimension; it is not set here.
    /// When the size is set to `VariableSquare`, the array element data type must also be an `ApplicationArrayDataType`
    VariableSquare(),
    /// Variable size "rectangular" array, with two or more dimensions. Each dimension has its own maximum size.
    /// The array element data type must also be an `ApplicationArrayDataType`.
    VariableRectangular { max_size: u64 },
    /// Fully flexible variable size array of arrays. The maximum number of elements of each contained array is not necessarily identical
    /// The array element data type must also be an `ApplicationArrayDataType`.
    VariableFullyFlexible { max_size: u64 },
}

impl From<ApplicationArraySize> for autosar_data_abstraction::datatype::ApplicationArraySize {
    fn from(size: ApplicationArraySize) -> Self {
        match size {
            ApplicationArraySize::Fixed { length } => {
                autosar_data_abstraction::datatype::ApplicationArraySize::Fixed(length)
            }
            ApplicationArraySize::VariableLinear { max_size } => {
                autosar_data_abstraction::datatype::ApplicationArraySize::VariableLinear(max_size)
            }
            ApplicationArraySize::VariableSquare() => {
                autosar_data_abstraction::datatype::ApplicationArraySize::VariableSquare
            }
            ApplicationArraySize::VariableRectangular { max_size } => {
                autosar_data_abstraction::datatype::ApplicationArraySize::VariableRectangular(
                    max_size,
                )
            }
            ApplicationArraySize::VariableFullyFlexible { max_size } => {
                autosar_data_abstraction::datatype::ApplicationArraySize::VariableFullyFlexible(
                    max_size,
                )
            }
        }
    }
}

impl From<autosar_data_abstraction::datatype::ApplicationArraySize> for ApplicationArraySize {
    fn from(size: autosar_data_abstraction::datatype::ApplicationArraySize) -> Self {
        match size {
            autosar_data_abstraction::datatype::ApplicationArraySize::Fixed(length) => {
                ApplicationArraySize::Fixed { length }
            }
            autosar_data_abstraction::datatype::ApplicationArraySize::VariableLinear(max_size) => {
                ApplicationArraySize::VariableLinear { max_size }
            }
            autosar_data_abstraction::datatype::ApplicationArraySize::VariableSquare => {
                ApplicationArraySize::VariableSquare()
            }
            autosar_data_abstraction::datatype::ApplicationArraySize::VariableRectangular(
                max_size,
            ) => ApplicationArraySize::VariableRectangular { max_size },
            autosar_data_abstraction::datatype::ApplicationArraySize::VariableFullyFlexible(
                max_size,
            ) => ApplicationArraySize::VariableFullyFlexible { max_size },
        }
    }
}

//#########################################################

/// An element in an application array data type
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._datatype"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct ApplicationArrayElement(
    pub(crate) autosar_data_abstraction::datatype::ApplicationArrayElement,
);

#[pymethods]
impl ApplicationArrayElement {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::datatype::ApplicationArrayElement::try_from(
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

    /// set the data type of the array element
    #[setter]
    fn set_data_type(&self, data_type: &Bound<'_, PyAny>) -> PyResult<()> {
        let data_type = pyany_to_application_data_type(data_type)?;
        self.0
            .set_data_type(&data_type)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the data type of the array element
    #[getter]
    fn data_type(&self) -> Option<Py<PyAny>> {
        self.0
            .data_type()
            .and_then(|data_type| application_data_type_to_pyany(data_type).ok())
    }
}

//#########################################################

/// An application record data type
///
/// Use [`ArPackage::create_application_record_data_type`] to create a new application record data type.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._datatype"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct ApplicationRecordDataType(
    pub(crate) autosar_data_abstraction::datatype::ApplicationRecordDataType,
);

#[pymethods]
impl ApplicationRecordDataType {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::datatype::ApplicationRecordDataType::try_from(
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

    /// create a new element in the record data type
    #[pyo3(signature = (name, data_type, /))]
    #[pyo3(text_signature = "(self, name: str, data_type: ApplicationDataType, /)")]
    fn create_record_element(
        &self,
        name: &str,
        data_type: &Bound<'_, PyAny>,
    ) -> PyResult<ApplicationRecordElement> {
        let data_type = pyany_to_application_data_type(data_type)?;
        match self.0.create_record_element(name, &data_type) {
            Ok(element) => Ok(ApplicationRecordElement(element)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    ///get an iterator over the record elements of the record data type
    fn record_elements(&self) -> ApplicationRecordElementIterator {
        ApplicationRecordElementIterator::new(
            self.0.record_elements().map(ApplicationRecordElement),
        )
    }
}

//#########################################################

/// An element in an application record data type
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._datatype"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct ApplicationRecordElement(
    pub(crate) autosar_data_abstraction::datatype::ApplicationRecordElement,
);

#[pymethods]
impl ApplicationRecordElement {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::datatype::ApplicationRecordElement::try_from(
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

    /// set the data type of the record element
    #[setter]
    fn set_data_type(&self, data_type: &Bound<'_, PyAny>) -> PyResult<()> {
        let data_type = pyany_to_application_data_type(data_type)?;
        self.0
            .set_data_type(&data_type)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the data type of the record element
    #[getter]
    fn data_type(&self) -> Option<Py<PyAny>> {
        self.0
            .data_type()
            .and_then(|data_type| application_data_type_to_pyany(data_type).ok())
    }
}

//#########################################################

iterator_wrapper!(ApplicationRecordElementIterator, ApplicationRecordElement);

//#########################################################

/// An application primitive data type
///
/// Use [`ArPackage::create_application_primitive_data_type`] to create a new application primitive data type.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._datatype"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct ApplicationPrimitiveDataType(
    pub(crate) autosar_data_abstraction::datatype::ApplicationPrimitiveDataType,
);

#[pymethods]
impl ApplicationPrimitiveDataType {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::datatype::ApplicationPrimitiveDataType::try_from(
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

    /// set the category of the primitive data type
    #[setter]
    fn set_category(&self, category: ApplicationPrimitiveCategory) -> PyResult<()> {
        self.0
            .set_category(category.into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the category of the primitive data type
    #[getter]
    fn category(&self) -> Option<ApplicationPrimitiveCategory> {
        self.0.category().map(std::convert::Into::into)
    }

    /// set the compu method of the primitive data type
    #[setter]
    fn set_compu_method(&self, compu_method: Option<&CompuMethod>) -> PyResult<()> {
        self.0
            .set_compu_method(compu_method.map(|cm| &cm.0))
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the compu method of the primitive data type
    #[getter]
    fn compu_method(&self) -> Option<CompuMethod> {
        self.0.compu_method().map(CompuMethod)
    }

    /// set the unit of the primitive data type
    #[setter]
    fn set_unit(&self, unit: Option<&Unit>) -> PyResult<()> {
        self.0
            .set_unit(unit.map(|u| &u.0))
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the unit of the primitive data type
    #[getter]
    fn unit(&self) -> Option<Unit> {
        self.0.unit().map(Unit)
    }

    /// set the data constraint of the primitive data type
    #[setter]
    fn set_data_constraint(&self, data_constraint: Option<&DataConstr>) -> PyResult<()> {
        self.0
            .set_data_constraint(data_constraint.map(|dc| &dc.0))
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the data constraint of the primitive data type
    #[getter]
    fn data_constraint(&self) -> Option<DataConstr> {
        self.0.data_constraint().map(DataConstr)
    }
}

//#########################################################

/// The category of an application primitive data type
#[pyclass(
    frozen,
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._datatype"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationPrimitiveCategory {
    /// Value
    Value,
    /// Value block
    ValBlk,
    /// String
    String,
    /// Boolean
    Boolean,
    /// Common axis
    ComAxis,
    /// Rescale axis
    ResAxis,
    /// Curve - 1D array with an axis
    Curve,
    /// Map - 2D array with two axes
    Map,
    /// Cuboid - 3D array with three axes
    Cuboid,
    /// Cube4 - 4D array with four axes
    Cube4,
    /// Cube5 - 5D array with five axes
    Cube5,
}

impl From<ApplicationPrimitiveCategory>
    for autosar_data_abstraction::datatype::ApplicationPrimitiveCategory
{
    fn from(category: ApplicationPrimitiveCategory) -> Self {
        match category {
            ApplicationPrimitiveCategory::Value => {
                autosar_data_abstraction::datatype::ApplicationPrimitiveCategory::Value
            }
            ApplicationPrimitiveCategory::ValBlk => {
                autosar_data_abstraction::datatype::ApplicationPrimitiveCategory::ValBlk
            }
            ApplicationPrimitiveCategory::String => {
                autosar_data_abstraction::datatype::ApplicationPrimitiveCategory::String
            }
            ApplicationPrimitiveCategory::Boolean => {
                autosar_data_abstraction::datatype::ApplicationPrimitiveCategory::Boolean
            }
            ApplicationPrimitiveCategory::ComAxis => {
                autosar_data_abstraction::datatype::ApplicationPrimitiveCategory::ComAxis
            }
            ApplicationPrimitiveCategory::ResAxis => {
                autosar_data_abstraction::datatype::ApplicationPrimitiveCategory::ResAxis
            }
            ApplicationPrimitiveCategory::Curve => {
                autosar_data_abstraction::datatype::ApplicationPrimitiveCategory::Curve
            }
            ApplicationPrimitiveCategory::Map => {
                autosar_data_abstraction::datatype::ApplicationPrimitiveCategory::Map
            }
            ApplicationPrimitiveCategory::Cuboid => {
                autosar_data_abstraction::datatype::ApplicationPrimitiveCategory::Cuboid
            }
            ApplicationPrimitiveCategory::Cube4 => {
                autosar_data_abstraction::datatype::ApplicationPrimitiveCategory::Cube4
            }
            ApplicationPrimitiveCategory::Cube5 => {
                autosar_data_abstraction::datatype::ApplicationPrimitiveCategory::Cube5
            }
        }
    }
}

impl From<autosar_data_abstraction::datatype::ApplicationPrimitiveCategory>
    for ApplicationPrimitiveCategory
{
    fn from(category: autosar_data_abstraction::datatype::ApplicationPrimitiveCategory) -> Self {
        match category {
            autosar_data_abstraction::datatype::ApplicationPrimitiveCategory::Value => {
                ApplicationPrimitiveCategory::Value
            }
            autosar_data_abstraction::datatype::ApplicationPrimitiveCategory::ValBlk => {
                ApplicationPrimitiveCategory::ValBlk
            }
            autosar_data_abstraction::datatype::ApplicationPrimitiveCategory::String => {
                ApplicationPrimitiveCategory::String
            }
            autosar_data_abstraction::datatype::ApplicationPrimitiveCategory::Boolean => {
                ApplicationPrimitiveCategory::Boolean
            }
            autosar_data_abstraction::datatype::ApplicationPrimitiveCategory::ComAxis => {
                ApplicationPrimitiveCategory::ComAxis
            }
            autosar_data_abstraction::datatype::ApplicationPrimitiveCategory::ResAxis => {
                ApplicationPrimitiveCategory::ResAxis
            }
            autosar_data_abstraction::datatype::ApplicationPrimitiveCategory::Curve => {
                ApplicationPrimitiveCategory::Curve
            }
            autosar_data_abstraction::datatype::ApplicationPrimitiveCategory::Map => {
                ApplicationPrimitiveCategory::Map
            }
            autosar_data_abstraction::datatype::ApplicationPrimitiveCategory::Cuboid => {
                ApplicationPrimitiveCategory::Cuboid
            }
            autosar_data_abstraction::datatype::ApplicationPrimitiveCategory::Cube4 => {
                ApplicationPrimitiveCategory::Cube4
            }
            autosar_data_abstraction::datatype::ApplicationPrimitiveCategory::Cube5 => {
                ApplicationPrimitiveCategory::Cube5
            }
        }
    }
}

//#########################################################

// convert the application data type to a python object
pub(crate) fn application_data_type_to_pyany(
    data_type: autosar_data_abstraction::datatype::ApplicationDataType,
) -> PyResult<Py<PyAny>> {
    Python::attach(|py| match data_type {
        autosar_data_abstraction::datatype::ApplicationDataType::Array(data_type) => {
            ApplicationArrayDataType(data_type).into_py_any(py)
        }
        autosar_data_abstraction::datatype::ApplicationDataType::Primitive(data_type) => {
            ApplicationPrimitiveDataType(data_type).into_py_any(py)
        }
        autosar_data_abstraction::datatype::ApplicationDataType::Record(data_type) => {
            ApplicationRecordDataType(data_type).into_py_any(py)
        }
    })
}

// convert a python object to an application data type
pub(crate) fn pyany_to_application_data_type(
    data_type: &Bound<'_, PyAny>,
) -> PyResult<autosar_data_abstraction::datatype::ApplicationDataType> {
    if let Ok(array) = data_type.extract::<ApplicationArrayDataType>() {
        Ok(autosar_data_abstraction::datatype::ApplicationDataType::Array(array.0))
    } else if let Ok(primitive) = data_type.extract::<ApplicationPrimitiveDataType>() {
        Ok(autosar_data_abstraction::datatype::ApplicationDataType::Primitive(primitive.0))
    } else if let Ok(record) = data_type.extract::<ApplicationRecordDataType>() {
        Ok(autosar_data_abstraction::datatype::ApplicationDataType::Record(record.0))
    } else {
        Err(AutosarAbstractionError::new_err(
            "Invalid application data type".to_string(),
        ))
    }
}
