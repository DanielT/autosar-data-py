use crate::abstraction::{
    datatype::{autosar_data_type_to_pyany, pyany_to_autosar_data_type},
    software_component::*,
};
use autosar_data_abstraction::{
    self, AbstractionElement, IdentifiableAbstractionElement,
    software_component::AbstractPortInterface,
};

//##################################################################

/// A `ClientServerInterface` defines a set of operations that can be implemented by a server and called by a client
///
/// Use [`ArPackage::create_client_server_interface`] to create a new client server interface
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct ClientServerInterface(
    pub(crate) autosar_data_abstraction::software_component::ClientServerInterface,
);

#[pymethods]
impl ClientServerInterface {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::ClientServerInterface::try_from(
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

    /// Add a possible error to the client server interface
    #[pyo3(signature = (name, error_code, /))]
    #[pyo3(text_signature = "(self, name: str, error_code: int, /)")]
    fn create_possible_error(&self, name: &str, error_code: u64) -> PyResult<ApplicationError> {
        match self.0.create_possible_error(name, error_code) {
            Ok(value) => Ok(ApplicationError(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all application errors
    fn possible_errors(&self) -> ApplicationErrorIterator {
        ApplicationErrorIterator::new(self.0.possible_errors().map(ApplicationError))
    }

    /// add an operation to the client server interface
    #[pyo3(signature = (name, /))]
    #[pyo3(text_signature = "(self, name: str, /)")]
    fn create_operation(&self, name: &str) -> PyResult<ClientServerOperation> {
        match self.0.create_operation(name) {
            Ok(value) => Ok(ClientServerOperation(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all operations
    fn operations(&self) -> ClientServerOperationIterator {
        ClientServerOperationIterator::new(self.0.operations().map(ClientServerOperation))
    }

    /// Set the is_service flag for this `ClientServerInterface`
    #[setter]
    fn set_is_service(&self, is_service: Option<bool>) -> PyResult<()> {
        self.0
            .set_is_service(is_service)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the is_service flag for this `ClientServerInterface`
    #[getter]
    fn is_service(&self) -> Option<bool> {
        self.0.is_service()
    }
}

//##################################################################

/// An `ApplicationError` represents an error that can be returned by a client server operation
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct ApplicationError(
    pub(crate) autosar_data_abstraction::software_component::ApplicationError,
);

#[pymethods]
impl ApplicationError {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::ApplicationError::try_from(
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

    /// Set the error code of the application error
    #[setter]
    fn set_error_code(&self, error_code: u64) -> PyResult<()> {
        self.0
            .set_error_code(error_code)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the error code of the application error
    #[getter]
    fn error_code(&self) -> Option<u64> {
        self.0.error_code()
    }
}

//##################################################################

iterator_wrapper!(ApplicationErrorIterator, ApplicationError);

//##################################################################

/// A `ClientServerOperation` defines an operation in a `ClientServerInterface`
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct ClientServerOperation(
    pub(crate) autosar_data_abstraction::software_component::ClientServerOperation,
);

#[pymethods]
impl ClientServerOperation {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::ClientServerOperation::try_from(
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

    /// Add an argument to the operation
    #[pyo3(signature = (name, data_type, direction, /))]
    #[pyo3(
        text_signature = "(self, name: str, data_type: DataType, direction: ArgumentDirection, /)"
    )]
    fn create_argument(
        &self,
        name: &str,
        data_type: &Bound<'_, PyAny>,
        direction: ArgumentDirection,
    ) -> PyResult<ArgumentDataPrototype> {
        let data_type = pyany_to_autosar_data_type(data_type)?;
        match self.0.create_argument(name, &data_type, direction.into()) {
            Ok(value) => Ok(ArgumentDataPrototype(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all arguments
    fn arguments(&self) -> ArgumentDataPrototypeIterator {
        ArgumentDataPrototypeIterator::new(self.0.arguments().map(ArgumentDataPrototype))
    }

    /// add a reference to possible error to the operation
    #[pyo3(signature = (error, /))]
    #[pyo3(text_signature = "(self, error: ApplicationError, /)")]
    fn add_possible_error(&self, error: &ApplicationError) -> PyResult<()> {
        self.0
            .add_possible_error(&error.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the possible errors of the operation
    fn possible_errors(&self) -> ApplicationErrorIterator {
        ApplicationErrorIterator::new(self.0.possible_errors().map(ApplicationError))
    }
}

//##################################################################

iterator_wrapper!(ClientServerOperationIterator, ClientServerOperation);

//##################################################################

/// The `ArgumentDirection` defines the direction of an argument in a `ClientServerOperation`
///
/// Input arguments are used to pass data from the client to the server and are usualy passed by value.
/// Output arguments are used to pass data from the server to the client and are usually passed by reference.
/// In/Out arguments are used to pass data in both directions and are usually passed by reference.
#[pyclass(
    frozen,
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArgumentDirection {
    /// The argument is an input argument
    In,
    /// The argument is an output argument
    Out,
    /// The argument is an in/out argument
    InOut,
}

impl From<autosar_data_abstraction::software_component::ArgumentDirection> for ArgumentDirection {
    fn from(value: autosar_data_abstraction::software_component::ArgumentDirection) -> Self {
        match value {
            autosar_data_abstraction::software_component::ArgumentDirection::In => Self::In,
            autosar_data_abstraction::software_component::ArgumentDirection::Out => Self::Out,
            autosar_data_abstraction::software_component::ArgumentDirection::InOut => Self::InOut,
        }
    }
}

impl From<ArgumentDirection> for autosar_data_abstraction::software_component::ArgumentDirection {
    fn from(value: ArgumentDirection) -> Self {
        match value {
            ArgumentDirection::In => Self::In,
            ArgumentDirection::Out => Self::Out,
            ArgumentDirection::InOut => Self::InOut,
        }
    }
}

//##################################################################

/// An `ArgumentDataPrototype` represents an argument in a `ClientServerOperation`
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct ArgumentDataPrototype(
    pub(crate) autosar_data_abstraction::software_component::ArgumentDataPrototype,
);

#[pymethods]
impl ArgumentDataPrototype {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::ArgumentDataPrototype::try_from(
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

    /// Set the data type of the argument
    #[setter]
    fn set_data_type(&self, data_type: &Bound<'_, PyAny>) -> PyResult<()> {
        let data_type = pyany_to_autosar_data_type(data_type)?;
        self.0
            .set_data_type(&data_type)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the data type of the argument
    #[getter]
    fn data_type(&self) -> Option<Py<PyAny>> {
        self.0
            .data_type()
            .and_then(|data_type| autosar_data_type_to_pyany(data_type).ok())
    }

    /// Set the direction of the argument
    #[setter]
    fn set_direction(&self, direction: ArgumentDirection) -> PyResult<()> {
        self.0
            .set_direction(direction.into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the direction of the argument
    #[getter]
    fn direction(&self) -> Option<ArgumentDirection> {
        self.0.direction().map(Into::into)
    }
}

//##################################################################

iterator_wrapper!(ArgumentDataPrototypeIterator, ArgumentDataPrototype);
