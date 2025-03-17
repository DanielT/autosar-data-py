use crate::abstraction::AutosarAbstractionError;
use crate::abstraction::communication::FlexrayPhysicalChannel;
use crate::{abstraction::*, *};
use autosar_data_abstraction::communication::{
    AbstractCommunicationConnector, AbstractCommunicationController,
};
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};

//##################################################################

/// An `EcuInstance` needs a `FlexrayCommunicationController` in order to connect to a Flexray cluster.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct FlexrayCommunicationController(
    pub(crate) autosar_data_abstraction::communication::FlexrayCommunicationController,
);

#[pymethods]
impl FlexrayCommunicationController {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::FlexrayCommunicationController::try_from(
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

    /// return an iterator over the [`FlexrayPhysicalChannel`]s connected to this controller
    fn connected_channels(&self) -> FlexrayPhysicalChannelIterator {
        FlexrayPhysicalChannelIterator::new(self.0.connected_channels().map(FlexrayPhysicalChannel))
    }

    /// Connect this [`FlexrayCommunicationController`] inside an [`EcuInstance`] to a [`FlexrayPhysicalChannel`] in the [`crate::System`]
    ///
    /// Creates a `FlexrayCommunicationConnector` in the [`EcuInstance`] that contains this [`FlexrayCommunicationController`].
    ///
    /// This function establishes the relationships:
    ///  - [`FlexrayPhysicalChannel`] -> `FlexrayCommunicationConnector`
    ///  - `FlexrayCommunicationConnector` -> [`FlexrayCommunicationController`]
    #[pyo3(signature = (connection_name, flx_channel, /))]
    #[pyo3(text_signature = "(self, connection_name: str, flx_channel: FlexrayPhysicalChannel)")]
    fn connect_physical_channel(
        &self,
        connection_name: &str,
        flx_channel: &FlexrayPhysicalChannel,
    ) -> PyResult<FlexrayCommunicationConnector> {
        match self
            .0
            .connect_physical_channel(connection_name, &flx_channel.0)
        {
            Ok(value) => Ok(FlexrayCommunicationConnector(value)),
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

iterator_wrapper!(FlexrayPhysicalChannelIterator, FlexrayPhysicalChannel);

//##################################################################

/// A connector between a [`FlexrayCommunicationController`] in an ECU and a [`FlexrayPhysicalChannel`]
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct FlexrayCommunicationConnector(
    pub(crate) autosar_data_abstraction::communication::FlexrayCommunicationConnector,
);

#[pymethods]
impl FlexrayCommunicationConnector {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::FlexrayCommunicationConnector::try_from(
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
    fn controller(&self) -> PyResult<FlexrayCommunicationController> {
        match self.0.controller() {
            Ok(value) => Ok(FlexrayCommunicationController(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }
}

//##################################################################

iterator_wrapper!(
    FlexrayCommunicationConnectorIterator,
    FlexrayCommunicationConnector
);
