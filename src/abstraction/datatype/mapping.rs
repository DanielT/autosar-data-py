use crate::{
    Element,
    abstraction::{
        AutosarAbstractionError, abstraction_err_to_pyerr,
        datatype::{
            ImplementationDataType, application_data_type_to_pyany, pyany_to_application_data_type,
        },
    },
    iterator_wrapper,
};
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};
use pyo3::prelude::*;

//##################################################################

/// A [`DataTypeMappingSet`] contains `DataTypeMap`s
///
/// Use [`ArPackage::create_data_type_mapping_set`] to create a new `DataTypeMappingSet`
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._datatype"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct DataTypeMappingSet(
    pub(crate) autosar_data_abstraction::datatype::DataTypeMappingSet,
);

#[pymethods]
impl DataTypeMappingSet {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::datatype::DataTypeMappingSet::try_from(element.0.clone()) {
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

    /// Create a new `DataTypeMap` in the `DataTypeMappingSet`
    #[pyo3(signature = (implementation_data_type, application_data_type, /))]
    #[pyo3(
        text_signature = "(self, implementation_data_type: ImplementationDataType, application_data_type: ApplicationDataType, /)"
    )]
    fn create_data_type_map(
        &self,
        implementation_data_type: &ImplementationDataType,
        application_data_type: &Bound<'_, PyAny>,
    ) -> PyResult<DataTypeMap> {
        let application_data_type = pyany_to_application_data_type(application_data_type)?;
        match self
            .0
            .create_data_type_map(&implementation_data_type.0, &application_data_type)
        {
            Ok(value) => Ok(DataTypeMap(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// Get an iterator over the `DataTypeMap`s in the `DataTypeMappingSet`
    fn data_type_maps(&self) -> DataTypeMapIterator {
        DataTypeMapIterator::new(self.0.data_type_maps().map(DataTypeMap))
    }
}

//##################################################################

iterator_wrapper!(DataTypeMappingSetIterator, DataTypeMappingSet);

//##################################################################

/// A `DataTypeMap` maps an `ImplementationDataType` to an `ApplicationDataType`
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._datatype"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct DataTypeMap(pub(crate) autosar_data_abstraction::datatype::DataTypeMap);

#[pymethods]
impl DataTypeMap {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::datatype::DataTypeMap::try_from(element.0.clone()) {
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

    /// Get the `ImplementationDataType` of the `DataTypeMap`
    #[getter]
    fn implementation_data_type(&self) -> Option<ImplementationDataType> {
        self.0
            .implementation_data_type()
            .map(ImplementationDataType)
    }

    /// Get the `ApplicationDataType` of the `DataTypeMap`
    #[getter]
    fn application_data_type(&self) -> Option<Py<PyAny>> {
        self.0
            .application_data_type()
            .and_then(|dt| application_data_type_to_pyany(dt).ok())
    }
}

//##################################################################

iterator_wrapper!(DataTypeMapIterator, DataTypeMap);
