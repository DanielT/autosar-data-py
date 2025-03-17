use crate::abstraction::AutosarAbstractionError;
use crate::abstraction::communication::{EthernetPhysicalChannel, EthernetVlanInfo};
use crate::{abstraction::*, *};
use autosar_data_abstraction::communication::AbstractCluster;
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};

//##################################################################

/// An `EthernetCluster` contains all configuration items associated with an ethernet network.
/// The cluster connects multiple ECUs.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct EthernetCluster(
    pub(crate) autosar_data_abstraction::communication::EthernetCluster,
);

#[pymethods]
impl EthernetCluster {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::EthernetCluster::try_from(element.0.clone())
        {
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

    /// Create a new physical channel for the cluster
    ///
    /// The supplied VLAN info must be unique - there cannot be two VLANs with the same vlan identifier.
    /// One channel may be created without VLAN information; it carries untagged traffic.
    #[pyo3(signature = (channel_name, /, *, vlan_info=None))]
    #[pyo3(
        text_signature = "(self, channel_name: str, /, *, vlan_info: Optional[EthernetVlanInfo])"
    )]
    fn create_physical_channel(
        &self,
        channel_name: &str,
        vlan_info: Option<EthernetVlanInfo>,
    ) -> PyResult<EthernetPhysicalChannel> {
        match self
            .0
            .create_physical_channel(channel_name, vlan_info.as_ref().map(|v| &v.0))
        {
            Ok(channel) => Ok(EthernetPhysicalChannel(channel)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// returns an iterator over all [`EthernetPhysicalChannel`]s in the cluster
    fn physical_channels(&self) -> EthernetPhysicalChannelsIterator {
        EthernetPhysicalChannelsIterator::new(
            self.0.physical_channels().map(EthernetPhysicalChannel),
        )
    }

    /// get the `System` this `CanCluster` is part of
    #[getter]
    fn system(&self) -> Option<System> {
        self.0.system().map(System)
    }
}

//##################################################################

iterator_wrapper!(EthernetPhysicalChannelsIterator, EthernetPhysicalChannel);
