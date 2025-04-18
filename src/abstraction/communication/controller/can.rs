use crate::{
    Element,
    abstraction::{
        AutosarAbstractionError, EcuInstance, abstraction_err_to_pyerr,
        communication::CanPhysicalChannel,
    },
    iterator_wrapper,
};
use autosar_data_abstraction::{
    self, AbstractionElement, IdentifiableAbstractionElement,
    communication::{AbstractCommunicationConnector, AbstractCommunicationController},
};
use pyo3::prelude::*;

//##################################################################

/// An `EcuInstance` needs a `CanCommunicationController` in order to connect to a CAN cluster.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct CanCommunicationController(
    pub(crate) autosar_data_abstraction::communication::CanCommunicationController,
);

#[pymethods]
impl CanCommunicationController {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::CanCommunicationController::try_from(
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

    /// return an iterator over the [`CanPhysicalChannel`]s connected to this controller
    fn connected_channels(&self) -> CanCCPhysicalChannelIterator {
        CanCCPhysicalChannelIterator::new(self.0.connected_channels().map(CanPhysicalChannel))
    }

    /// Connect this [`CanCommunicationController`] inside an [`EcuInstance`] to a [`CanPhysicalChannel`] in the [`crate::System`]
    ///
    /// Creates a [`CanCommunicationConnector`] in the [`EcuInstance`] that contains this [`CanCommunicationController`].
    ///
    /// This function establishes the relationships:
    ///  - [`CanPhysicalChannel`] -> [`CanCommunicationConnector`]
    ///  - [`CanCommunicationConnector`] -> [`CanCommunicationController`]
    #[pyo3(signature = (connection_name, can_channel, /))]
    #[pyo3(text_signature = "(self, connection_name: str, can_channel: CanPhysicalChannel, /)")]
    fn connect_physical_channel(
        &self,
        connection_name: &str,
        can_channel: &CanPhysicalChannel,
    ) -> PyResult<CanCommunicationConnector> {
        match self
            .0
            .connect_physical_channel(connection_name, &can_channel.0)
        {
            Ok(value) => Ok(CanCommunicationConnector(value)),
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

iterator_wrapper!(CanCCPhysicalChannelIterator, CanPhysicalChannel);

//##################################################################

/// A connector between a [`CanCommunicationController`] in an ECU and a [`CanPhysicalChannel`]
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct CanCommunicationConnector(
    pub(crate) autosar_data_abstraction::communication::CanCommunicationConnector,
);

#[pymethods]
impl CanCommunicationConnector {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::CanCommunicationConnector::try_from(
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
    fn controller(&self) -> PyResult<CanCommunicationController> {
        match self.0.controller() {
            Ok(value) => Ok(CanCommunicationController(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }
}
