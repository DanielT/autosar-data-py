use crate::abstraction::AutosarAbstractionError;
use crate::abstraction::communication::{
    CanCluster, CanCommunicationController, NmEcu, NmPdu, NmPduIterator,
};
use crate::{abstraction::*, *};
use autosar_data_abstraction::communication::{
    AbstractNmCluster, AbstractNmClusterCoupling, AbstractNmNode,
};
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};

//##################################################################

/// Can specific `NmCluster` attributes
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct CanNmCluster(pub(crate) autosar_data_abstraction::communication::CanNmCluster);

#[pymethods]
impl CanNmCluster {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::CanNmCluster::try_from(element.0.clone()) {
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

    /// set the nmBusloadReductionActive flag
    #[setter]
    fn set_nm_busload_reduction_active(&self, nm_busload_reduction_active: bool) -> PyResult<()> {
        self.0
            .set_nm_busload_reduction_active(nm_busload_reduction_active)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the nmBusloadReductionActive flag
    #[getter]
    fn nm_busload_reduction_active(&self) -> Option<bool> {
        self.0.nm_busload_reduction_active()
    }

    /// set the nmImmediateNmTransmissions value
    #[setter]
    fn set_nm_immediate_nm_transmissions(
        &self,
        nm_immediate_nm_transmissions: u32,
    ) -> PyResult<()> {
        self.0
            .set_nm_immediate_nm_transmissions(nm_immediate_nm_transmissions)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the nmImmediateNmTransmissions value
    #[getter]
    fn nm_immediate_nm_transmissions(&self) -> Option<u32> {
        self.0.nm_immediate_nm_transmissions()
    }

    /// set the nmMessageTimeoutTime
    #[setter]
    fn set_nm_message_timeout_time(&self, nm_message_timeout_time: f64) -> PyResult<()> {
        self.0
            .set_nm_message_timeout_time(nm_message_timeout_time)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the nmMessageTimeoutTime
    #[getter]
    fn nm_message_timeout_time(&self) -> Option<f64> {
        self.0.nm_message_timeout_time()
    }

    /// set the nmMsgCycleTime
    #[setter]
    fn set_nm_msg_cycle_time(&self, cycle_time: f64) -> PyResult<()> {
        self.0
            .set_nm_msg_cycle_time(cycle_time)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the nmMsgCycleTime
    #[getter]
    fn nm_msg_cycle_time(&self) -> Option<f64> {
        self.0.nm_msg_cycle_time()
    }

    /// set the nmNetworkTimeout
    #[setter]
    fn set_nm_network_timeout(&self, nm_network_timeout: f64) -> PyResult<()> {
        self.0
            .set_nm_network_timeout(nm_network_timeout)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the nmNetworkTimeout
    #[getter]
    fn nm_network_timeout(&self) -> Option<f64> {
        self.0.nm_network_timeout()
    }

    /// set the nmRemoteSleepIndicationTime
    #[setter]
    fn set_nm_remote_sleep_indication_time(
        &self,
        nm_remote_sleep_indication_time: f64,
    ) -> PyResult<()> {
        self.0
            .set_nm_remote_sleep_indication_time(nm_remote_sleep_indication_time)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the nmRemoteSleepIndicationTime
    #[getter]
    fn nm_remote_sleep_indication_time(&self) -> Option<f64> {
        self.0.nm_remote_sleep_indication_time()
    }

    /// set the nmRepeatMessageTime
    #[setter]
    fn set_nm_repeat_message_time(&self, nm_repeat_message_time: f64) -> PyResult<()> {
        self.0
            .set_nm_repeat_message_time(nm_repeat_message_time)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the nmRepeatMessageTime
    #[getter]
    fn nm_repeat_message_time(&self) -> Option<f64> {
        self.0.nm_repeat_message_time()
    }

    /// set the nmWaitBusSleepTime
    #[setter]
    fn set_nm_wait_bus_sleep_time(&self, nm_wait_bus_sleep_time: f64) -> PyResult<()> {
        self.0
            .set_nm_wait_bus_sleep_time(nm_wait_bus_sleep_time)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the nmWaitBusSleepTime
    #[getter]
    fn nm_wait_bus_sleep_time(&self) -> Option<f64> {
        self.0.nm_wait_bus_sleep_time()
    }

    /// add a `CanNmNode` to the cluster
    #[pyo3(signature = (name, controller, nm_ecu, /))]
    #[pyo3(
        text_signature = "(self, name: str, controller: CanCommunicationController, nm_ecu: NmEcu, /)"
    )]
    fn create_can_nm_node(
        &self,
        name: &str,
        controller: &CanCommunicationController,
        nm_ecu: &NmEcu,
    ) -> PyResult<CanNmNode> {
        match self.0.create_can_nm_node(name, &controller.0, &nm_ecu.0) {
            Ok(value) => Ok(CanNmNode(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    // ----- items from the AbstractNmCluster trait -----

    /// set the referenced `CommunicationCluster`
    #[setter]
    fn set_communication_cluster(&self, communication_cluster: &CanCluster) -> PyResult<()> {
        self.0
            .set_communication_cluster(&communication_cluster.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the referenced `CommunicationCluster`
    #[getter]
    fn communication_cluster(&self) -> Option<CanCluster> {
        self.0.communication_cluster().map(CanCluster)
    }

    /// iterate over all `NmNodes` in this cluster
    fn nm_nodes(&self) -> CanNmNodeIterator {
        CanNmNodeIterator::new(self.0.nm_nodes().map(CanNmNode))
    }

    /// set or remove the nmChannelSleepMaster flag
    #[setter]
    fn set_channel_sleep_master(&self, value: Option<bool>) -> PyResult<()> {
        self.0
            .set_channel_sleep_master(value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the nmChannelSleepMaster flag
    #[getter]
    fn channel_sleep_master(&self) -> Option<bool> {
        self.0.channel_sleep_master()
    }

    /// set the nmNodeDetectionEnabled flag
    #[setter]
    fn set_node_detection_enabled(&self, value: Option<bool>) -> PyResult<()> {
        self.0
            .set_node_detection_enabled(value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the nmNodeDetectionEnabled flag
    #[getter]
    fn node_detection_enabled(&self) -> Option<bool> {
        self.0.node_detection_enabled()
    }

    /// set the nmNodeIdEnabled flag
    #[setter]
    fn set_node_id_enabled(&self, value: Option<bool>) -> PyResult<()> {
        self.0
            .set_node_id_enabled(value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the nmNodeIdEnabled flag
    #[getter]
    fn node_id_enabled(&self) -> Option<bool> {
        self.0.node_id_enabled()
    }

    /// set the nmPncParticipation flag
    #[setter]
    fn set_pnc_participation(&self, value: Option<bool>) -> PyResult<()> {
        self.0
            .set_pnc_participation(value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the nmPncParticipation flag
    #[getter]
    fn pnc_participation(&self) -> Option<bool> {
        self.0.pnc_participation()
    }

    /// set the nmRepeatMsgIndEnabled flag
    #[setter]
    fn set_repeat_msg_ind_enabled(&self, value: Option<bool>) -> PyResult<()> {
        self.0
            .set_repeat_msg_ind_enabled(value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the nmRepeatMsgIndEnabled flag
    #[getter]
    fn repeat_msg_ind_enabled(&self) -> Option<bool> {
        self.0.repeat_msg_ind_enabled()
    }

    /// set the nmSynchronizingNetwork flag
    #[setter]
    fn set_synchronizing_network(&self, value: Option<bool>) -> PyResult<()> {
        self.0
            .set_synchronizing_network(value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the nmSynchronizingNetwork flag
    #[getter]
    fn synchronizing_network(&self) -> Option<bool> {
        self.0.synchronizing_network()
    }

    /// set the pncClusterVectorLength
    #[setter]
    fn set_pnc_cluster_vector_length(&self, value: Option<u8>) -> PyResult<()> {
        self.0
            .set_pnc_cluster_vector_length(value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the pncClusterVectorLength
    #[getter]
    fn pnc_cluster_vector_length(&self) -> Option<u8> {
        self.0.pnc_cluster_vector_length()
    }
}

//##################################################################

iterator_wrapper!(CanNmClusterIterator, CanNmCluster);

//##################################################################

/// Mandatory settings for a `CanNmCluster`
///
/// These settings are mandatory for a `CanNmCluster` and must be set during creation.
/// Additional optional settings can be set using the `CanNmCluster` methods.
#[pyclass(
    eq,
    get_all,
    set_all,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CanNmClusterSettings {
    /// nmBusloadReductionActive: Determines if bus load reduction for the respective `CanNm` channel is active.
    pub nm_busload_reduction_active: bool,
    /// nmImmediateNmTransmissions: Defines the number of immediate `NmPdus` which shall be transmitted.
    /// If the value is zero no immediate `NmPdus` are transmitted.
    pub nm_immediate_nm_transmissions: u32,
    /// nmMessageTimeoutTime: Timeout of an `NmPdu` in seconds.
    pub nm_message_timeout_time: f64,
    /// nmMsgCycleTime: Period of a `NmPdu` in seconds
    pub nm_msg_cycle_time: f64,
    /// nmNetworkTimeout: Network Timeout for `NmPdus` in seconds.
    pub nm_network_timeout: f64,
    /// nmRemoteSleepIndicationTime: Timeout for Remote Sleep Indication in seconds.
    pub nm_remote_sleep_indication_time: f64,
    /// nmRepeatMessageTime: Timeout for Repeat Message State in seconds.
    pub nm_repeat_message_time: f64,
    /// nmWaitBusSleepTime: Timeout for bus calm down phase in seconds.
    pub nm_wait_bus_sleep_time: f64,
}

impl From<&CanNmClusterSettings> for autosar_data_abstraction::communication::CanNmClusterSettings {
    fn from(settings: &CanNmClusterSettings) -> Self {
        Self {
            nm_busload_reduction_active: settings.nm_busload_reduction_active,
            nm_immediate_nm_transmissions: settings.nm_immediate_nm_transmissions,
            nm_message_timeout_time: settings.nm_message_timeout_time,
            nm_msg_cycle_time: settings.nm_msg_cycle_time,
            nm_network_timeout: settings.nm_network_timeout,
            nm_remote_sleep_indication_time: settings.nm_remote_sleep_indication_time,
            nm_repeat_message_time: settings.nm_repeat_message_time,
            nm_wait_bus_sleep_time: settings.nm_wait_bus_sleep_time,
        }
    }
}

#[pymethods]
impl CanNmClusterSettings {
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (*, nm_busload_reduction_active, nm_immediate_nm_transmissions, nm_message_timeout_time, nm_msg_cycle_time,
                        nm_network_timeout, nm_remote_sleep_indication_time, nm_repeat_message_time, nm_wait_bus_sleep_time))]
    #[pyo3(
        text_signature = "(self, *, nm_busload_reduction_active: bool, nm_immediate_nm_transmissions: int, nm_message_timeout_time: float, nm_msg_cycle_time: float,
                        nm_network_timeout: float, nm_remote_sleep_indication_time: float, nm_repeat_message_time: float, nm_wait_bus_sleep_time: float)"
    )]
    #[new]
    fn new(
        nm_busload_reduction_active: bool,
        nm_immediate_nm_transmissions: u32,
        nm_message_timeout_time: f64,
        nm_msg_cycle_time: f64,
        nm_network_timeout: f64,
        nm_remote_sleep_indication_time: f64,
        nm_repeat_message_time: f64,
        nm_wait_bus_sleep_time: f64,
    ) -> Self {
        Self {
            nm_busload_reduction_active,
            nm_immediate_nm_transmissions,
            nm_message_timeout_time,
            nm_msg_cycle_time,
            nm_network_timeout,
            nm_remote_sleep_indication_time,
            nm_repeat_message_time,
            nm_wait_bus_sleep_time,
        }
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}

//##################################################################

/// A `CanNmClusterCoupling` couples multiple `CanNmCluster`s, and contains CAN specific settings.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct CanNmClusterCoupling(
    pub(crate) autosar_data_abstraction::communication::CanNmClusterCoupling,
);

#[pymethods]
impl CanNmClusterCoupling {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::CanNmClusterCoupling::try_from(
            element.0.clone(),
        ) {
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

    /// set the nmBusloadReductionEnabled flag
    #[setter]
    fn set_nm_busload_reduction_enabled(&self, nm_busload_reduction_enabled: bool) -> PyResult<()> {
        self.0
            .set_nm_busload_reduction_enabled(nm_busload_reduction_enabled)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the nmBusloadReductionEnabled flag
    #[getter]
    fn nm_busload_reduction_enabled(&self) -> Option<bool> {
        self.0.nm_busload_reduction_enabled()
    }

    /// set the nmImmediateRestartEnabled flag
    #[setter]
    fn set_nm_immediate_restart_enabled(&self, nm_immediate_restart_enabled: bool) -> PyResult<()> {
        self.0
            .set_nm_immediate_restart_enabled(nm_immediate_restart_enabled)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the nmImmediateRestartEnabled flag
    #[getter]
    fn nm_immediate_restart_enabled(&self) -> Option<bool> {
        self.0.nm_immediate_restart_enabled()
    }

    // ----- items from the AbstractNmClusterCoupling trait -----

    /// add a reference to a coupled `NmCluster`
    #[pyo3(signature = (cluster, /))]
    #[pyo3(text_signature = "(self, cluster: CanNmCluster, /)")]
    fn add_coupled_cluster(&self, cluster: &CanNmCluster) -> PyResult<()> {
        self.0
            .add_coupled_cluster(&cluster.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// iterate over all coupled `NmClusters`
    fn coupled_clusters(&self) -> CanNmClusterIterator {
        CanNmClusterIterator::new(self.0.coupled_clusters().map(CanNmCluster))
    }
}

//##################################################################

/// A `CanNmNode` represents a node in a `CanNmCluster`.
///
/// The node connects to a `CanCommunicationController` and an `NmEcu`.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct CanNmNode(pub(crate) autosar_data_abstraction::communication::CanNmNode);

#[pymethods]
impl CanNmNode {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::CanNmNode::try_from(element.0.clone()) {
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

    // ----- items from the AbstractNmNode trait -----

    /// set the referenced `CommunicationController`
    #[setter]
    fn set_communication_controller(
        &self,
        controller: &CanCommunicationController,
    ) -> PyResult<()> {
        self.0
            .set_communication_controller(&controller.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the referenced `CommunicationController`
    #[getter]
    fn communication_controller(&self) -> Option<CanCommunicationController> {
        self.0
            .communication_controller()
            .map(CanCommunicationController)
    }

    /// set the referenced `NmEcu`
    #[setter]
    fn set_nm_ecu(&self, ecu: &NmEcu) -> PyResult<()> {
        self.0.set_nm_ecu(&ecu.0).map_err(abstraction_err_to_pyerr)
    }

    /// get the referenced `NmEcu`
    #[getter]
    fn nm_ecu(&self) -> Option<NmEcu> {
        self.0.nm_ecu().map(NmEcu)
    }

    /// set the nmNodeId
    /// This value is optional; if it is set to Some(x) the value is created, if it is set to None the value is removed.
    #[setter]
    fn set_node_id(&self, value: Option<u32>) -> PyResult<()> {
        self.0.set_node_id(value).map_err(abstraction_err_to_pyerr)
    }

    /// get the nmNodeId
    #[getter]
    fn node_id(&self) -> Option<u32> {
        self.0.node_id()
    }

    /// set ot remove the nmPassiveModeEnabled flag
    ///
    /// This flag is optional; if it is set to Some(x) the value is created, if it is set to None the value is removed.
    #[setter]
    fn set_passive_mode(&self, value: Option<bool>) -> PyResult<()> {
        self.0
            .set_passive_mode(value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the nmPassiveModeEnabled flag
    #[getter]
    fn passive_mode(&self) -> Option<bool> {
        self.0.passive_mode()
    }

    /// add an Rx `NmPdu`
    ///
    /// Every `NmNode` must have at least one Rx `NmPdu`
    #[pyo3(signature = (nm_pdu, /))]
    #[pyo3(text_signature = "(self, nm_pdu: NmPdu, /)")]
    fn add_rx_nm_pdu(&self, nm_pdu: &NmPdu) -> PyResult<()> {
        self.0
            .add_rx_nm_pdu(&nm_pdu.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// iterate over all RX `NmPdus`
    fn rx_nm_pdus(&self) -> NmPduIterator {
        NmPduIterator::new(self.0.rx_nm_pdus().map(NmPdu))
    }

    /// add a Tx `NmPdu`
    ///
    /// Active `NmNodes` must have at least one Tx `NmPdu`, while passive `NmNodes` may have none.
    #[pyo3(signature = (nm_pdu, /))]
    #[pyo3(text_signature = "(self, nm_pdu: NmPdu, /)")]
    fn add_tx_nm_pdu(&self, nm_pdu: &NmPdu) -> PyResult<()> {
        self.0
            .add_tx_nm_pdu(&nm_pdu.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// iterate over all TX `NmPdus`
    fn tx_nm_pdus(&self) -> NmPduIterator {
        NmPduIterator::new(self.0.tx_nm_pdus().map(NmPdu))
    }
}

//##################################################################

iterator_wrapper!(CanNmNodeIterator, CanNmNode);
