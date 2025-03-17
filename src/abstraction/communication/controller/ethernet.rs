use crate::abstraction::AutosarAbstractionError;
use crate::abstraction::communication::EthernetPhysicalChannel;
use crate::{abstraction::*, *};
use autosar_data_abstraction::communication::{
    AbstractCommunicationConnector, AbstractCommunicationController,
};
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};

//##################################################################

/// An `EcuInstance` needs an `EthernetCommunicationController` in order to connect to an ethernet cluster.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct EthernetCommunicationController(
    pub(crate) autosar_data_abstraction::communication::EthernetCommunicationController,
);

#[pymethods]
impl EthernetCommunicationController {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::EthernetCommunicationController::try_from(
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

    /// return an iterator over the [`EthernetPhysicalChannel`]s connected to this controller
    fn connected_channels(&self) -> EthernetCCPhysicalChannelIterator {
        EthernetCCPhysicalChannelIterator::new(
            self.0.connected_channels().map(EthernetPhysicalChannel),
        )
    }

    /// Connect this [`EthernetCommunicationController`] inside an [`EcuInstance`] to an [`EthernetPhysicalChannel`] in the [`crate::System`]
    ///
    /// Creates an `EthernetCommunicationConnector` in the [`EcuInstance`] that contains this [`EthernetCommunicationController`].
    ///
    /// This function establishes the relationships:
    ///  - [`EthernetPhysicalChannel`] -> `EthernetCommunicationConnector`
    ///  - `EthernetCommunicationConnector` -> [`EthernetCommunicationController`]
    #[pyo3(signature = (connection_name, eth_channel, /))]
    #[pyo3(
        text_signature = "(self, connection_name: str, eth_channel: EthernetPhysicalChannel, /)"
    )]
    fn connect_physical_channel(
        &self,
        connection_name: &str,
        eth_channel: &EthernetPhysicalChannel,
    ) -> PyResult<EthernetCommunicationConnector> {
        match self
            .0
            .connect_physical_channel(connection_name, &eth_channel.0)
        {
            Ok(value) => Ok(EthernetCommunicationConnector(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// Get the `EcuInstance` that contains this `FlexrayCommunicationController`
    #[getter]
    fn ecu_instance(&self) -> PyResult<EcuInstance> {
        match self.0.ecu_instance() {
            Ok(value) => Ok(EcuInstance(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }
}

//##################################################################

iterator_wrapper!(EthernetCCPhysicalChannelIterator, EthernetPhysicalChannel);

//##################################################################

/// A connector between an [`EthernetCommunicationController`] in an ECU and an [`EthernetPhysicalChannel`]
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct EthernetCommunicationConnector(
    pub(crate) autosar_data_abstraction::communication::EthernetCommunicationConnector,
);

#[pymethods]
impl EthernetCommunicationConnector {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::EthernetCommunicationConnector::try_from(
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
    fn controller(&self) -> PyResult<EthernetCommunicationController> {
        match self.0.controller() {
            Ok(value) => Ok(EthernetCommunicationController(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }
}
