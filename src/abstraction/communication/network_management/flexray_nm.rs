use crate::abstraction::AutosarAbstractionError;
use crate::abstraction::communication::{
    FlexrayCluster, FlexrayCommunicationController, NmEcu, NmPdu, NmPduIterator,
};
use crate::{abstraction::*, *};
use autosar_data_abstraction::communication::{
    AbstractNmCluster, AbstractNmClusterCoupling, AbstractNmNode,
};
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};

//##################################################################

/// Flexray specific `NmCluster`
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct FlexrayNmCluster(
    pub(crate) autosar_data_abstraction::communication::FlexrayNmCluster,
);

#[pymethods]
impl FlexrayNmCluster {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::FlexrayNmCluster::try_from(element.0.clone())
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

    /// set the nmDataCycle
    ///
    /// Number of Flexray Communication Cycles needed to transmit the Nm Data PDUs of all Flexray Nm Ecus of this `FlexrayNmCluster`.
    #[setter]
    fn set_nm_data_cycle(&self, nm_data_cycle: u32) -> PyResult<()> {
        self.0
            .set_nm_data_cycle(nm_data_cycle)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the nmDataCycle
    ///
    /// Number of Flexray Communication Cycles needed to transmit the Nm Data PDUs of all Flexray Nm Ecus of this `FlexrayNmCluster`.
    #[getter]
    fn nm_data_cycle(&self) -> Option<u32> {
        self.0.nm_data_cycle()
    }

    /// set the nmRemoteSleepIndicationTime
    ///
    /// Timeout for Remote Sleep Indication in seconds.
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
    ///
    /// Timeout for Remote Sleep Indication in seconds.
    #[getter]
    fn nm_remote_sleep_indication_time(&self) -> Option<f64> {
        self.0.nm_remote_sleep_indication_time()
    }

    /// set the nmRepeatMessageTime
    ///
    /// Timeout for Repeat Message State in seconds.
    #[setter]
    fn set_nm_repeat_message_time(&self, nm_repeat_message_time: f64) -> PyResult<()> {
        self.0
            .set_nm_repeat_message_time(nm_repeat_message_time)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the nmRepeatMessageTime
    ///
    /// Timeout for Repeat Message State in seconds.
    #[getter]
    fn nm_repeat_message_time(&self) -> Option<f64> {
        self.0.nm_repeat_message_time()
    }

    /// set the nmRepetitionCycle
    ///
    /// Number of Flexray Communication Cycles used to repeat the transmission of the Nm vote Pdus of all
    /// Flexray `NmEcus` of this `FlexrayNmCluster`. This value shall be an integral multiple of nmVotingCycle.
    #[setter]
    fn set_nm_repetition_cycle(&self, nm_repetition_cycle: u32) -> PyResult<()> {
        self.0
            .set_nm_repetition_cycle(nm_repetition_cycle)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the nmRepetitionCycle
    ///
    /// Number of Flexray Communication Cycles used to repeat the transmission of the Nm vote Pdus of all
    /// Flexray `NmEcus` of this `FlexrayNmCluster`. This value shall be an integral multiple of nmVotingCycle.
    #[getter]
    fn nm_repetition_cycle(&self) -> Option<u32> {
        self.0.nm_repetition_cycle()
    }

    /// set the nmVotingCycle
    ///
    /// The number of Fexray Communication Cycles used to transmit the Nm Vote PDUs of all Fexray Nm Ecus of this `FlexrayNmCluster`.
    #[setter]
    fn set_nm_voting_cycle(&self, nm_voting_cycle: u32) -> PyResult<()> {
        self.0
            .set_nm_voting_cycle(nm_voting_cycle)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the nmVotingCycle
    ///
    /// The number of Fexray Communication Cycles used to transmit the Nm Vote PDUs of all Fexray Nm Ecus of this `FlexrayNmCluster`.
    #[getter]
    fn nm_voting_cycle(&self) -> Option<u32> {
        self.0.nm_voting_cycle()
    }

    /// add a `FlexrayNmNode` to the cluster
    #[pyo3(signature = (name, controller, nm_ecu, /))]
    #[pyo3(
        text_signature = "(self, name: str, controller: FlexrayCommunicationController, nm_ecu: NmEcu, /)"
    )]
    fn create_flexray_nm_node(
        &self,
        name: &str,
        controller: &FlexrayCommunicationController,
        nm_ecu: &NmEcu,
    ) -> PyResult<FlexrayNmNode> {
        match self
            .0
            .create_flexray_nm_node(name, &controller.0, &nm_ecu.0)
        {
            Ok(value) => Ok(FlexrayNmNode(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    // ----- items from the AbstractNmCluster trait -----

    /// set the referenced `FlexrayCluster`
    #[setter]
    fn set_communication_cluster(&self, communication_cluster: &FlexrayCluster) -> PyResult<()> {
        self.0
            .set_communication_cluster(&communication_cluster.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the referenced `FlexrayCluster`
    #[getter]
    fn communication_cluster(&self) -> Option<FlexrayCluster> {
        self.0.communication_cluster().map(FlexrayCluster)
    }

    /// iterate over all `NmNodes` in this cluster
    fn nm_nodes(&self) -> FlexrayNmNodeIterator {
        FlexrayNmNodeIterator::new(self.0.nm_nodes().map(FlexrayNmNode))
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

iterator_wrapper!(FlexrayNmClusterIterator, FlexrayNmCluster);

//##################################################################

/// Mandatory settings for a `FlexrayNmCluster`
///
/// These settings must be provided when creating a new `FlexrayNmCluster`.
/// Additional optional settings can be set using `FlexrayNmCluster` methods.
#[pyclass(
    eq,
    get_all,
    set_all,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, PartialEq)]
pub struct FlexrayNmClusterSettings {
    /// nmDataCycle: Number of Fexray Communication Cycles needed to transmit the Nm Data PDUs of all Fexray Nm Ecus of this `FlexrayNmCluster`.
    pub nm_data_cycle: u32,
    /// nmRemoteSleepIndicationTime: Timeout for Remote Sleep Indication in seconds.
    pub nm_remote_sleep_indication_time: f64,
    /// nmRepeatMessageTime: Timeout for Repeat Message State in seconds.
    pub nm_repeat_message_time: f64,
    /// nmRepetitionCycle: Number of Fexray Communication Cycles used to repeat the transmission of the Nm vote Pdus of all
    /// Fexray `NmEcus` of this `FlexrayNmCluster`. This value shall be an integral multiple of nmVotingCycle.
    pub nm_repetition_cycle: u32,
    /// nmVotingCycle: The number of Fexray Communication Cycles used to transmit the Nm Vote PDUs of all Fexray Nm Ecus of this `FlexrayNmCluster`.
    pub nm_voting_cycle: u32,
}

impl From<&FlexrayNmClusterSettings>
    for autosar_data_abstraction::communication::FlexrayNmClusterSettings
{
    fn from(settings: &FlexrayNmClusterSettings) -> Self {
        Self {
            nm_data_cycle: settings.nm_data_cycle,
            nm_remote_sleep_indication_time: settings.nm_remote_sleep_indication_time,
            nm_repeat_message_time: settings.nm_repeat_message_time,
            nm_repetition_cycle: settings.nm_repetition_cycle,
            nm_voting_cycle: settings.nm_voting_cycle,
        }
    }
}

#[pymethods]
impl FlexrayNmClusterSettings {
    #[pyo3(signature = (*, nm_data_cycle, nm_remote_sleep_indication_time, nm_repeat_message_time, nm_repetition_cycle, nm_voting_cycle))]
    #[pyo3(
        text_signature = "(self, *, nm_data_cycle: int, nm_remote_sleep_indication_time: float, nm_repeat_message_time: float,
                           nm_repetition_cycle: int, nm_voting_cycle: int)"
    )]
    #[new]
    fn new(
        nm_data_cycle: u32,
        nm_remote_sleep_indication_time: f64,
        nm_repeat_message_time: f64,
        nm_repetition_cycle: u32,
        nm_voting_cycle: u32,
    ) -> Self {
        Self {
            nm_data_cycle,
            nm_remote_sleep_indication_time,
            nm_repeat_message_time,
            nm_repetition_cycle,
            nm_voting_cycle,
        }
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}

//##################################################################

/// A `FlexrayNmClusterCoupling` `couples multiple `FlexrayNmCluster`s.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct FlexrayNmClusterCoupling(
    pub(crate) autosar_data_abstraction::communication::FlexrayNmClusterCoupling,
);

#[pymethods]
impl FlexrayNmClusterCoupling {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::FlexrayNmClusterCoupling::try_from(
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

    /// set the nmScheduleVariant
    #[setter]
    fn set_nm_schedule_variant(
        &self,
        nm_schedule_variant: FlexrayNmScheduleVariant,
    ) -> PyResult<()> {
        self.0
            .set_nm_schedule_variant(nm_schedule_variant.into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the nmScheduleVariant
    #[getter]
    fn nm_schedule_variant(&self) -> Option<FlexrayNmScheduleVariant> {
        self.0
            .nm_schedule_variant()
            .map(FlexrayNmScheduleVariant::from)
    }

    // ----- items from the AbstractNmClusterCoupling trait -----

    /// add a reference to a coupled `NmCluster`
    #[pyo3(signature = (cluster, /))]
    #[pyo3(text_signature = "(self, cluster: FlexrayNmCluster, /)")]
    fn add_coupled_cluster(&self, cluster: &FlexrayNmCluster) -> PyResult<()> {
        self.0
            .add_coupled_cluster(&cluster.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// iterate over all coupled `NmClusters`
    fn coupled_clusters(&self) -> FlexrayNmClusterIterator {
        FlexrayNmClusterIterator::new(self.0.coupled_clusters().map(FlexrayNmCluster))
    }
}

//##################################################################

/// The `FlexrayNmScheduleVariant` defines the way the NM-Vote and NM-Data are transmitted within the Fexray network.
#[pyclass(
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexrayNmScheduleVariant {
    /// NM-Vote and NM Data transmitted within one PDU in static segment. The NM-Vote has to be realized as separate bit within the PDU.
    ScheduleVariant1,
    /// NM-Vote and NM-Data transmitted within one PDU in dynamic segment. The presence (or non-presence) of the PDU corresponds to the NM-Vote
    ScheduleVariant2,
    /// NM-Vote and NM-Data are transmitted in the static segment in separate PDUs. This alternative is not recommended => Alternative 1 should be used instead.
    ScheduleVariant3,
    /// NM-Vote transmitted in static and NM-Data transmitted in dynamic segment.
    ScheduleVariant4,
    /// NM-Vote is transmitted in dynamic and NM-Data is transmitted in static segment. This alternative is not recommended => Variants 2 or 6 should be used instead.
    ScheduleVariant5,
    /// NM-Vote and NM-Data are transmitted in the dynamic segment in separate PDUs.
    ScheduleVariant6,
    /// NM-Vote and a copy of the CBV are transmitted in the static segment (using the Fexray NM Vector support) and NM-Data is transmitted in the dynamic segment
    ScheduleVariant7,
}

impl From<FlexrayNmScheduleVariant>
    for autosar_data_abstraction::communication::FlexrayNmScheduleVariant
{
    fn from(variant: FlexrayNmScheduleVariant) -> Self {
        match variant {
            FlexrayNmScheduleVariant::ScheduleVariant1 => {
                autosar_data_abstraction::communication::FlexrayNmScheduleVariant::ScheduleVariant1
            }
            FlexrayNmScheduleVariant::ScheduleVariant2 => {
                autosar_data_abstraction::communication::FlexrayNmScheduleVariant::ScheduleVariant2
            }
            FlexrayNmScheduleVariant::ScheduleVariant3 => {
                autosar_data_abstraction::communication::FlexrayNmScheduleVariant::ScheduleVariant3
            }
            FlexrayNmScheduleVariant::ScheduleVariant4 => {
                autosar_data_abstraction::communication::FlexrayNmScheduleVariant::ScheduleVariant4
            }
            FlexrayNmScheduleVariant::ScheduleVariant5 => {
                autosar_data_abstraction::communication::FlexrayNmScheduleVariant::ScheduleVariant5
            }
            FlexrayNmScheduleVariant::ScheduleVariant6 => {
                autosar_data_abstraction::communication::FlexrayNmScheduleVariant::ScheduleVariant6
            }
            FlexrayNmScheduleVariant::ScheduleVariant7 => {
                autosar_data_abstraction::communication::FlexrayNmScheduleVariant::ScheduleVariant7
            }
        }
    }
}

impl From<autosar_data_abstraction::communication::FlexrayNmScheduleVariant>
    for FlexrayNmScheduleVariant
{
    fn from(variant: autosar_data_abstraction::communication::FlexrayNmScheduleVariant) -> Self {
        match variant {
            autosar_data_abstraction::communication::FlexrayNmScheduleVariant::ScheduleVariant1 => {
                FlexrayNmScheduleVariant::ScheduleVariant1
            }
            autosar_data_abstraction::communication::FlexrayNmScheduleVariant::ScheduleVariant2 => {
                FlexrayNmScheduleVariant::ScheduleVariant2
            }
            autosar_data_abstraction::communication::FlexrayNmScheduleVariant::ScheduleVariant3 => {
                FlexrayNmScheduleVariant::ScheduleVariant3
            }
            autosar_data_abstraction::communication::FlexrayNmScheduleVariant::ScheduleVariant4 => {
                FlexrayNmScheduleVariant::ScheduleVariant4
            }
            autosar_data_abstraction::communication::FlexrayNmScheduleVariant::ScheduleVariant5 => {
                FlexrayNmScheduleVariant::ScheduleVariant5
            }
            autosar_data_abstraction::communication::FlexrayNmScheduleVariant::ScheduleVariant6 => {
                FlexrayNmScheduleVariant::ScheduleVariant6
            }
            autosar_data_abstraction::communication::FlexrayNmScheduleVariant::ScheduleVariant7 => {
                FlexrayNmScheduleVariant::ScheduleVariant7
            }
        }
    }
}

//##################################################################

/// A `FlexrayNmNode` represents a Fexray specific `NmNode`.
///
/// It connects a `FlexrayCommunicationController` with a `NmEcu`.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct FlexrayNmNode(pub(crate) autosar_data_abstraction::communication::FlexrayNmNode);

#[pymethods]
impl FlexrayNmNode {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::FlexrayNmNode::try_from(element.0.clone()) {
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

    /// set the referenced `FlexrayCommunicationController`
    #[setter]
    fn set_communication_controller(
        &self,
        controller: &FlexrayCommunicationController,
    ) -> PyResult<()> {
        self.0
            .set_communication_controller(&controller.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the referenced `FlexrayCommunicationController`
    #[getter]
    fn communication_controller(&self) -> Option<FlexrayCommunicationController> {
        self.0
            .communication_controller()
            .map(FlexrayCommunicationController)
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

iterator_wrapper!(FlexrayNmNodeIterator, FlexrayNmNode);
