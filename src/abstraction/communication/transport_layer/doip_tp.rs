use crate::{
    Element,
    abstraction::{
        AutosarAbstractionError, abstraction_err_to_pyerr,
        communication::{EthernetCluster, PduTriggering},
    },
    iterator_wrapper,
};
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};
use pyo3::prelude::*;

/// Container for `DoIp` TP configuration
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct DoIpTpConfig(pub(crate) autosar_data_abstraction::communication::DoIpTpConfig);

#[pymethods]
impl DoIpTpConfig {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::DoIpTpConfig::try_from(element.0.clone()) {
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

    /// set the reference to the `EthernetCluster` for this `DoIpTpConfig`
    #[setter]
    fn set_cluster(&self, cluster: &EthernetCluster) -> PyResult<()> {
        self.0
            .set_cluster(&cluster.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the reference to the `EthernetCluster` for this `DoIpTpConfig`
    #[getter]
    fn cluster(&self) -> Option<EthernetCluster> {
        self.0.cluster().map(EthernetCluster)
    }

    /// create a new `DoIpLogicAddress`
    #[pyo3(signature = (name, address, /))]
    #[pyo3(text_signature = "(self, name: str, address: int, /)")]
    fn create_doip_logic_address(&self, name: &str, address: u32) -> PyResult<DoIpLogicAddress> {
        match self.0.create_doip_logic_address(name, address) {
            Ok(value) => Ok(DoIpLogicAddress(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all `DoIpLogicAddresss`
    fn doip_logic_addresses(&self) -> DoIpLogicAddressIterator {
        DoIpLogicAddressIterator::new(self.0.doip_logic_addresses().map(DoIpLogicAddress))
    }

    /// create a new `DoIpTpConnection`
    #[pyo3(signature = (name, source, target, tp_sdu_triggering, /))]
    #[pyo3(
        text_signature = "(self, name: Optional[str], source: DoIpLogicAddress, target: DoIpLogicAddress, tp_sdu_triggering: PduTriggering, /)"
    )]
    fn create_doip_tp_connection(
        &self,
        name: Option<&str>,
        source: &DoIpLogicAddress,
        target: &DoIpLogicAddress,
        tp_sdu_triggering: &PduTriggering,
    ) -> PyResult<DoIpTpConnection> {
        match self
            .0
            .create_doip_tp_connection(name, &source.0, &target.0, &tp_sdu_triggering.0)
        {
            Ok(value) => Ok(DoIpTpConnection(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all `DoIpTpConnections`
    fn doip_tp_connections(&self) -> DoIpTpConnectionIterator {
        DoIpTpConnectionIterator::new(self.0.doip_tp_connections().map(DoIpTpConnection))
    }
}

//##################################################################

/// This element defines the logical address of a `DoIp` connection
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct DoIpLogicAddress(
    pub(crate) autosar_data_abstraction::communication::DoIpLogicAddress,
);

#[pymethods]
impl DoIpLogicAddress {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::DoIpLogicAddress::try_from(element.0.clone())
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

    /// set the address of this `DoIpLogicAddress`
    #[setter]
    fn set_address(&self, address: u32) -> PyResult<()> {
        self.0
            .set_address(address)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the address of this `DoIpLogicAddress`
    #[getter]
    fn address(&self) -> Option<u32> {
        self.0.address()
    }
}

//##################################################################

iterator_wrapper!(DoIpLogicAddressIterator, DoIpLogicAddress);

//##################################################################

/// The `DoIpTpConnection` defines a `DoIp` transport protocol connection
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct DoIpTpConnection(
    pub(crate) autosar_data_abstraction::communication::DoIpTpConnection,
);

#[pymethods]
impl DoIpTpConnection {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::DoIpTpConnection::try_from(element.0.clone())
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

    /// set the source `DoIpLogicAddress`
    #[setter]
    fn set_source(&self, source: &DoIpLogicAddress) -> PyResult<()> {
        self.0
            .set_source(&source.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the source `DoIpLogicAddress`
    #[getter]
    fn source(&self) -> Option<DoIpLogicAddress> {
        self.0.source().map(DoIpLogicAddress)
    }

    /// set the target `DoIpLogicAddress`
    #[setter]
    fn set_target(&self, target: &DoIpLogicAddress) -> PyResult<()> {
        self.0
            .set_target(&target.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the target `DoIpLogicAddress`
    #[getter]
    fn target(&self) -> Option<DoIpLogicAddress> {
        self.0.target().map(DoIpLogicAddress)
    }

    /// set the `PduTriggering` for this connection
    #[setter]
    fn set_tp_sdu_triggering(&self, tp_sdu_triggering: &PduTriggering) -> PyResult<()> {
        self.0
            .set_tp_sdu_triggering(&tp_sdu_triggering.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the `PduTriggering` for this connection
    #[getter]
    fn tp_sdu_triggering(&self) -> Option<PduTriggering> {
        self.0.tp_sdu_triggering().map(PduTriggering)
    }
}

//##################################################################

iterator_wrapper!(DoIpTpConnectionIterator, DoIpTpConnection);
