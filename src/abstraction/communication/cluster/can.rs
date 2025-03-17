use crate::{
    Element,
    abstraction::{
        AutosarAbstractionError, System, abstraction_err_to_pyerr,
        communication::CanPhysicalChannel,
    },
};
use autosar_data_abstraction::{
    self, AbstractionElement, IdentifiableAbstractionElement, communication::AbstractCluster,
};
use pyo3::prelude::*;

//##################################################################

/// A `CanCluster` contains all configuration items associated with a CAN network.
/// The cluster connects multiple ECUs.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct CanCluster(pub(crate) autosar_data_abstraction::communication::CanCluster);

#[pymethods]
impl CanCluster {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::CanCluster::try_from(element.0.clone()) {
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

    /// set the CAN baudrate for the cluster
    #[setter]
    fn set_baudrate(&self, baudrate: u32) -> PyResult<()> {
        self.0
            .set_baudrate(baudrate)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the CAN baudrate for the cluster
    #[getter]
    fn baudrate(&self) -> Option<u32> {
        self.0.baudrate()
    }

    /// set the CAN FD baudrate for the cluster
    #[setter]
    fn set_can_fd_baudrate(&self, baudrate: Option<u32>) -> PyResult<()> {
        self.0
            .set_can_fd_baudrate(baudrate)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the CAN FD baudrate for the cluster
    #[getter]
    fn can_fd_baudrate(&self) -> Option<u32> {
        self.0.can_fd_baudrate()
    }

    /// set the CAN XL baudrate for the cluster
    #[setter]
    fn set_can_xl_baudrate(&self, baudrate: Option<u32>) -> PyResult<()> {
        self.0
            .set_can_xl_baudrate(baudrate)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the CAN XL baudrate for the cluster
    #[getter]
    fn can_xl_baudrate(&self) -> Option<u32> {
        self.0.can_xl_baudrate()
    }

    /// Create a new physical channel for the cluster
    ///
    /// A can cluster must contain exactly one physical channel; trying to add a second one triggers an error.
    #[pyo3(signature = (channel_name, /))]
    #[pyo3(text_signature = "(self, channel_name: str, /)")]
    fn create_physical_channel(&self, channel_name: &str) -> PyResult<CanPhysicalChannel> {
        match self.0.create_physical_channel(channel_name) {
            Ok(channel) => Ok(CanPhysicalChannel(channel)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// get the `CanPhysicalChannel` of the Cluster, if it has been created
    #[getter]
    fn physical_channel(&self) -> Option<CanPhysicalChannel> {
        self.0.physical_channel().map(CanPhysicalChannel)
    }

    /// get the `System` this `CanCluster` is part of
    #[getter]
    fn system(&self) -> Option<System> {
        self.0.system().map(System)
    }
}
