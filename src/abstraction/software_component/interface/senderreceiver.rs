use crate::{
    abstraction::{
        datatype::{
            autosar_data_type_to_pyany, pyany_to_autosar_data_type, pyany_to_value_specification,
            value_specification_to_pyany,
        },
        *,
    },
    *,
};
use autosar_data_abstraction::{
    self, AbstractionElement, IdentifiableAbstractionElement,
    software_component::AbstractPortInterface,
};

//##################################################################

/// A `SenderReceiverInterface` defines a set of data elements that can be sent and received
///
/// Use [`ArPackage::create_sender_receiver_interface`] to create a new sender receiver interface
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct SenderReceiverInterface(
    pub(crate) autosar_data_abstraction::software_component::SenderReceiverInterface,
);

#[pymethods]
impl SenderReceiverInterface {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::SenderReceiverInterface::try_from(
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

    /// Add a new data element to the sender receiver interface
    #[pyo3(signature = (name, data_type, /))]
    #[pyo3(text_signature = "(self, name: str, data_type: AutosarDataType, /)")]
    fn create_data_element(
        &self,
        name: &str,
        data_type: &Bound<'_, PyAny>,
    ) -> PyResult<VariableDataPrototype> {
        let data_type = pyany_to_autosar_data_type(data_type)?;
        match self.0.create_data_element(name, &data_type) {
            Ok(value) => Ok(VariableDataPrototype(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all data elements
    fn data_elements(&self) -> VariableDataPrototypeIterator {
        VariableDataPrototypeIterator::new(self.0.data_elements().map(VariableDataPrototype))
    }

    /// Set the is_service flag for this `SenderReceiverInterface`
    #[setter]
    fn set_is_service(&self, is_service: Option<bool>) -> PyResult<()> {
        self.0
            .set_is_service(is_service)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the is_service flag for this `SenderReceiverInterface`
    #[getter]
    fn is_service(&self) -> Option<bool> {
        self.0.is_service()
    }
}

//##################################################################

/// A `VariableDataPrototype` represents a data element in a `SenderReceiverInterface`
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct VariableDataPrototype(
    pub(crate) autosar_data_abstraction::software_component::VariableDataPrototype,
);

#[pymethods]
impl VariableDataPrototype {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::VariableDataPrototype::try_from(
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

    /// Set the data type of the data element
    #[setter]
    fn set_data_type(&self, data_type: &Bound<'_, PyAny>) -> PyResult<()> {
        let data_type = pyany_to_autosar_data_type(data_type)?;
        self.0
            .set_data_type(&data_type)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the data type of the data element
    #[getter]
    fn data_type(&self) -> Option<Py<PyAny>> {
        self.0
            .data_type()
            .and_then(|value| autosar_data_type_to_pyany(value).ok())
    }

    /// Get the interface containing the data element
    #[getter]
    fn interface(&self) -> PyResult<SenderReceiverInterface> {
        match self.0.interface() {
            Ok(value) => Ok(SenderReceiverInterface(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// set the init value for the data element
    #[setter]
    fn set_init_value(&self, init_value: Option<&Bound<'_, PyAny>>) -> PyResult<()> {
        let init_value = init_value
            .map(|val| pyany_to_value_specification(val))
            .transpose()?;
        self.0
            .set_init_value(init_value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the init value for the data element
    #[getter]
    fn init_value(&self) -> Option<Py<PyAny>> {
        self.0
            .init_value()
            .and_then(|value_spec| value_specification_to_pyany(&value_spec).ok())
    }
}

//##################################################################

iterator_wrapper!(VariableDataPrototypeIterator, VariableDataPrototype);
