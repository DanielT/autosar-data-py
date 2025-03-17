use crate::abstraction::AutosarAbstractionError;
use crate::abstraction::communication::{
    EthernetCluster, EthernetCommunicationController, EthernetPhysicalChannel, NmEcu, NmPdu,
    NmPduIterator,
};
use crate::{abstraction::*, *};
use autosar_data_abstraction::communication::{
    AbstractNmCluster, AbstractNmClusterCoupling, AbstractNmNode,
};
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};

//##################################################################

/// Udp / Ethernet specific `NmCluster`
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct UdpNmCluster(pub(crate) autosar_data_abstraction::communication::UdpNmCluster);

#[pymethods]
impl UdpNmCluster {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::UdpNmCluster::try_from(element.0.clone()) {
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

    /// set the nmMessageTimeoutTime
    #[setter]
    fn set_nm_message_timeout_time(&self, timeout_time: f64) -> PyResult<()> {
        self.0
            .set_nm_message_timeout_time(timeout_time)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the nmMessageTimeoutTime
    #[getter]
    fn nm_message_timeout_time(&self) -> Option<f64> {
        self.0.nm_message_timeout_time()
    }

    /// set the `NmNetworkTimeout`
    #[setter]
    fn set_nm_network_timeout(&self, timeout: f64) -> PyResult<()> {
        self.0
            .set_nm_network_timeout(timeout)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the `NmNetworkTimeout`
    #[getter]
    fn nm_network_timeout(&self) -> Option<f64> {
        self.0.nm_network_timeout()
    }

    /// set the `NmRemoteSleepIndicationTime`
    #[setter]
    fn set_nm_remote_sleep_indication_time(&self, time: f64) -> PyResult<()> {
        self.0
            .set_nm_remote_sleep_indication_time(time)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the `NmRemoteSleepIndicationTime`
    #[getter]
    fn nm_remote_sleep_indication_time(&self) -> Option<f64> {
        self.0.nm_remote_sleep_indication_time()
    }

    /// set the `NmRepeatMessageTime`
    #[setter]
    fn set_nm_repeat_message_time(&self, time: f64) -> PyResult<()> {
        self.0
            .set_nm_repeat_message_time(time)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the `NmRepeatMessageTime`
    #[getter]
    fn nm_repeat_message_time(&self) -> Option<f64> {
        self.0.nm_repeat_message_time()
    }

    /// set the `NmWaitBusSleepTime`
    #[setter]
    fn set_nm_wait_bus_sleep_time(&self, time: f64) -> PyResult<()> {
        self.0
            .set_nm_wait_bus_sleep_time(time)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the `NmWaitBusSleepTime`
    #[getter]
    fn nm_wait_bus_sleep_time(&self) -> Option<f64> {
        self.0.nm_wait_bus_sleep_time()
    }

    /// add a `UdpNmNode` to the cluster
    #[pyo3(signature = (name, controller, nm_ecu, nm_msg_cycle_offset, /))]
    #[pyo3(
        text_signature = "(self, name: str, controller: EthernetCommunicationController, nm_ecu: NmEcu, nm_msg_cycle_offset: float, /)"
    )]
    fn create_udp_nm_node(
        &self,
        name: &str,
        controller: &EthernetCommunicationController,
        nm_ecu: &NmEcu,
        nm_msg_cycle_offset: f64,
    ) -> PyResult<UdpNmNode> {
        match self
            .0
            .create_udp_nm_node(name, &controller.0, &nm_ecu.0, nm_msg_cycle_offset)
        {
            Ok(value) => Ok(UdpNmNode(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// set or delete the Vlan associated with the cluster through an `EthernetPhysicalChannel` reference.
    #[setter]
    fn set_vlan(&self, vlan: Option<&EthernetPhysicalChannel>) -> PyResult<()> {
        self.0
            .set_vlan(vlan.map(|v| &v.0))
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the Vlan associated with the cluster
    #[getter]
    fn vlan(&self) -> Option<EthernetPhysicalChannel> {
        self.0.vlan().map(EthernetPhysicalChannel)
    }

    /// set or delete the value nmImmediateNmTransmissions
    #[setter]
    fn set_nm_immediate_nm_transmissions(&self, value: Option<u32>) -> PyResult<()> {
        self.0
            .set_nm_immediate_nm_transmissions(value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the value of nmImmediateNmTransmissions
    #[getter]
    fn nm_immediate_nm_transmissions(&self) -> Option<u32> {
        self.0.nm_immediate_nm_transmissions()
    }

    /// set or delete the value nmCbvPosition
    #[setter]
    fn set_nm_cbv_position(&self, value: Option<u32>) -> PyResult<()> {
        self.0
            .set_nm_cbv_position(value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the value of nmCbvPosition
    #[getter]
    fn nm_cbv_position(&self) -> Option<u32> {
        self.0.nm_cbv_position()
    }

    /// set or delete the value nmNidPosition
    #[setter]
    fn set_nm_nid_position(&self, value: Option<u32>) -> PyResult<()> {
        self.0
            .set_nm_nid_position(value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the value of nmNidPosition
    #[getter]
    fn nm_nid_position(&self) -> Option<u32> {
        self.0.nm_nid_position()
    }

    // ----- items from the AbstractNmCluster trait -----

    /// set the referenced `EthernetCluster`
    #[setter]
    fn set_communication_cluster(&self, communication_cluster: &EthernetCluster) -> PyResult<()> {
        self.0
            .set_communication_cluster(&communication_cluster.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the referenced `EthernetCluster`
    #[getter]
    fn communication_cluster(&self) -> Option<EthernetCluster> {
        self.0.communication_cluster().map(EthernetCluster)
    }

    /// iterate over all `NmNodes` in this cluster
    fn nm_nodes(&self) -> UdpNmNodeIterator {
        UdpNmNodeIterator::new(self.0.nm_nodes().map(UdpNmNode))
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

iterator_wrapper!(UdpNmClusterIterator, UdpNmCluster);

//##################################################################

/// `UdpNmClusterSettings` encapsulates the mandatory settings for a `UdpNmCluster`
#[pyclass(
    eq,
    get_all,
    set_all,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, PartialEq)]
pub struct UdpNmClusterSettings {
    /// Period of an `NmPdu` in seconds
    pub nm_msg_cycle_time: f64,
    /// Timeout of a `NmPdu` in seconds
    pub nm_msg_timeout_time: f64,
    /// Network Timeout for `NmPdus` in seconds
    pub nm_network_timeout: f64,
    /// Timeout for Remote Sleep Indication in seconds
    pub nm_remote_sleep_indication_time: f64,
    /// Timeout for Repeat Message State in seconds
    pub nm_repeat_message_time: f64,
    /// Timeout for bus calm down phase in seconds
    pub nm_wait_bus_sleep_time: f64,
}

impl From<&UdpNmClusterSettings> for autosar_data_abstraction::communication::UdpNmClusterSettings {
    fn from(settings: &UdpNmClusterSettings) -> Self {
        Self {
            nm_msg_cycle_time: settings.nm_msg_cycle_time,
            nm_msg_timeout_time: settings.nm_msg_timeout_time,
            nm_network_timeout: settings.nm_network_timeout,
            nm_remote_sleep_indication_time: settings.nm_remote_sleep_indication_time,
            nm_repeat_message_time: settings.nm_repeat_message_time,
            nm_wait_bus_sleep_time: settings.nm_wait_bus_sleep_time,
        }
    }
}

#[pymethods]
impl UdpNmClusterSettings {
    #[pyo3(signature = (*, nm_msg_cycle_time, nm_msg_timeout_time, nm_network_timeout, nm_remote_sleep_indication_time, nm_repeat_message_time, nm_wait_bus_sleep_time))]
    #[pyo3(
        text_signature = "(self, *, nm_msg_cycle_time: float, nm_msg_timeout_time: float, nm_network_timeout: float,
                          nm_remote_sleep_indication_time: float, nm_repeat_message_time: float, nm_wait_bus_sleep_time: float)"
    )]
    #[new]
    fn new(
        nm_msg_cycle_time: f64,
        nm_msg_timeout_time: f64,
        nm_network_timeout: f64,
        nm_remote_sleep_indication_time: f64,
        nm_repeat_message_time: f64,
        nm_wait_bus_sleep_time: f64,
    ) -> Self {
        Self {
            nm_msg_cycle_time,
            nm_msg_timeout_time,
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

/// Udp / Ethernet specific `NmClusterCoupling`
///
/// It couples multiple `UdpNmCluster`s and provides UdpNm-specific settings
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct UdpNmClusterCoupling(
    pub(crate) autosar_data_abstraction::communication::UdpNmClusterCoupling,
);

#[pymethods]
impl UdpNmClusterCoupling {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::UdpNmClusterCoupling::try_from(
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

    /// set or remove the nmImmediateRestartEnabled flag
    #[setter]
    fn set_nm_immediate_restart_enabled(&self, enabled: Option<bool>) -> PyResult<()> {
        self.0
            .set_nm_immediate_restart_enabled(enabled)
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
    #[pyo3(text_signature = "(self, cluster: UdpNmCluster, /)")]
    fn add_coupled_cluster(&self, cluster: &UdpNmCluster) -> PyResult<()> {
        self.0
            .add_coupled_cluster(&cluster.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// iterate over all coupled `NmClusters`
    fn coupled_clusters(&self) -> UdpNmClusterIterator {
        UdpNmClusterIterator::new(self.0.coupled_clusters().map(UdpNmCluster))
    }
}

//##################################################################

/// Udp / Ethernet specific `NmNode`
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct UdpNmNode(pub(crate) autosar_data_abstraction::communication::UdpNmNode);

#[pymethods]
impl UdpNmNode {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::UdpNmNode::try_from(element.0.clone()) {
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

    /// set the `NmMsgCycleOffset`
    #[setter]
    fn set_nm_msg_cycle_offset(&self, offset: f64) -> PyResult<()> {
        self.0
            .set_nm_msg_cycle_offset(offset)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the `NmMsgCycleOffset`
    #[getter]
    fn nm_msg_cycle_offset(&self) -> Option<f64> {
        self.0.nm_msg_cycle_offset()
    }

    /// set ot remove the allNmMessagesKeepAwake flag
    #[setter]
    fn set_all_nm_messages_keep_awake(&self, enabled: Option<bool>) -> PyResult<()> {
        self.0
            .set_all_nm_messages_keep_awake(enabled)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the allNmMessagesKeepAwake flag
    #[getter]
    fn all_nm_messages_keep_awake(&self) -> Option<bool> {
        self.0.all_nm_messages_keep_awake()
    }

    // ----- items from the AbstractNmNode trait -----

    /// set the referenced `EthernetCommunicationController`
    #[setter]
    fn set_communication_controller(
        &self,
        controller: &EthernetCommunicationController,
    ) -> PyResult<()> {
        self.0
            .set_communication_controller(&controller.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the referenced `EthernetCommunicationController`
    #[getter]
    fn communication_controller(&self) -> Option<EthernetCommunicationController> {
        self.0
            .communication_controller()
            .map(EthernetCommunicationController)
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

iterator_wrapper!(UdpNmNodeIterator, UdpNmNode);
