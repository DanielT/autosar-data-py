use crate::{
    Element,
    abstraction::{
        AutosarAbstractionError, EcuInstance, abstraction_err_to_pyerr,
        communication::LinPhysicalChannel,
    },
    iterator_wrapper,
};
use autosar_data_abstraction::{
    self, AbstractionElement, IdentifiableAbstractionElement,
    communication::{
        AbstractCommunicationConnector, AbstractCommunicationController,
        AbstractLinCommunicationController,
    },
};
use pyo3::{IntoPyObjectExt, prelude::*};

//##################################################################

/// An `EcuInstance` needs a `LinMaster` or `LinSlave` in order to connect to a LIN cluster.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct LinMaster(pub(crate) autosar_data_abstraction::communication::LinMaster);

#[pymethods]
impl LinMaster {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::LinMaster::try_from(element.0.clone()) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    #[pyo3(signature = (/, *, deep = false))]
    #[pyo3(text_signature = "(self, /, *, deep: bool = false)")]
    fn remove(&self, deep: bool) -> PyResult<()> {
        self.clone()
            .0
            .remove(deep)
            .map_err(abstraction_err_to_pyerr)
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

    /// return an iterator over the [`LinPhysicalChannel`]s connected to this controller
    fn connected_channels(&self) -> LinCCPhysicalChannelIterator {
        LinCCPhysicalChannelIterator::new(self.0.connected_channels().map(LinPhysicalChannel))
    }

    /// Connect this [`LinCommunicationController`] inside an [`EcuInstance`] to a [`LinPhysicalChannel`] in the [`crate::System`]
    ///
    /// Creates a [`LinCommunicationConnector`] in the [`EcuInstance`] that contains this [`LinMaster`].
    ///
    /// This function establishes the relationships:
    ///  - [`LinPhysicalChannel`] -> [`LinCommunicationConnector`]
    ///  - [`LinCommunicationConnector`] -> [`LinMaster`]
    #[pyo3(signature = (connection_name, lin_channel, /))]
    #[pyo3(text_signature = "(self, connection_name: str, lin_channel: LinPhysicalChannel, /)")]
    fn connect_physical_channel(
        &self,
        connection_name: &str,
        lin_channel: &LinPhysicalChannel,
    ) -> PyResult<LinCommunicationConnector> {
        match self
            .0
            .connect_physical_channel(connection_name, &lin_channel.0)
        {
            Ok(value) => Ok(LinCommunicationConnector(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// Get the `EcuInstance` that contains this `CommunicationController`
    #[getter]
    fn ecu_instance(&self) -> PyResult<EcuInstance> {
        match self.0.ecu_instance() {
            Ok(value) => Ok(EcuInstance(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }
}

//##################################################################

/// An `EcuInstance` needs a `LinMaster` or `LinSlave` in order to connect to a LIN cluster.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct LinSlave(pub(crate) autosar_data_abstraction::communication::LinSlave);

#[pymethods]
impl LinSlave {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::LinSlave::try_from(element.0.clone()) {
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

    /// return an iterator over the [`LinPhysicalChannel`]s connected to this controller
    fn connected_channels(&self) -> LinCCPhysicalChannelIterator {
        LinCCPhysicalChannelIterator::new(self.0.connected_channels().map(LinPhysicalChannel))
    }

    /// Connect this [`LinCommunicationController`] inside an [`EcuInstance`] to a [`LinPhysicalChannel`] in the [`crate::System`]
    ///
    /// Creates a [`LinCommunicationConnector`] in the [`EcuInstance`] that contains this [`LinSlave`].
    ///
    /// This function establishes the relationships:
    ///  - [`LinPhysicalChannel`] -> [`LinCommunicationConnector`]
    ///  - [`LinCommunicationConnector`] -> [`LinSlave`]
    #[pyo3(signature = (connection_name, lin_channel, /))]
    #[pyo3(text_signature = "(self, connection_name: str, lin_channel: LinPhysicalChannel, /)")]
    fn connect_physical_channel(
        &self,
        connection_name: &str,
        lin_channel: &LinPhysicalChannel,
    ) -> PyResult<LinCommunicationConnector> {
        match self
            .0
            .connect_physical_channel(connection_name, &lin_channel.0)
        {
            Ok(value) => Ok(LinCommunicationConnector(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// Get the `EcuInstance` that contains this `CommunicationController`
    #[getter]
    fn ecu_instance(&self) -> PyResult<EcuInstance> {
        match self.0.ecu_instance() {
            Ok(value) => Ok(EcuInstance(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }
}

//##################################################################

iterator_wrapper!(LinCCPhysicalChannelIterator, LinPhysicalChannel);

//##################################################################

/// A `LinCommunicationConnector` connects an `EcuInstance` to a `LinCluster` via a `LinMaster` or `LinSlave`.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct LinCommunicationConnector(
    pub(crate) autosar_data_abstraction::communication::LinCommunicationConnector,
);

#[pymethods]
impl LinCommunicationConnector {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::LinCommunicationConnector::try_from(
            element.0.clone(),
        ) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    #[pyo3(signature = (/, *, deep = false))]
    #[pyo3(text_signature = "(self, /, *, deep: bool = false)")]
    fn remove(&self, deep: bool) -> PyResult<()> {
        self.clone()
            .0
            .remove(deep)
            .map_err(abstraction_err_to_pyerr)
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

    /// Get the `EcuInstance` that contains this `CommunicationConnector`
    #[getter]
    fn ecu_instance(&self) -> PyResult<EcuInstance> {
        match self.0.ecu_instance() {
            Ok(value) => Ok(EcuInstance(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// Get the controller of the `CommunicationConnector`
    #[getter]
    fn controller(&self) -> PyResult<Py<PyAny>> {
        match self.0.controller() {
            Ok(autosar_data_abstraction::communication::LinCommunicationController::Master(
                master,
            )) => Python::attach(|py| LinMaster(master).into_py_any(py)),
            Ok(autosar_data_abstraction::communication::LinCommunicationController::Slave(
                slave,
            )) => Python::attach(|py| LinSlave(slave).into_py_any(py)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }
}
