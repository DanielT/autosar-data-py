use crate::{
    Element,
    abstraction::{
        AutosarAbstractionError, EcuInstance, abstraction_err_to_pyerr,
        communication::{
            FlexrayCluster, FlexrayCommunicationConnector, FlexrayCommunicationConnectorIterator,
            NPdu, NPduIterator, TpAddress, TpAddressIterator, ipdu_to_pyany, pyany_to_ipdu,
        },
    },
    iterator_wrapper,
};
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};
use pyo3::prelude::*;

//##################################################################

/// `FlexrayTpConfig` defines exactly one Flexray ISO TP Configuration
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct FlexrayTpConfig(
    pub(crate) autosar_data_abstraction::communication::FlexrayTpConfig,
);

#[pymethods]
impl FlexrayTpConfig {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::FlexrayTpConfig::try_from(element.0.clone())
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

    /// set the `FlexrayCluster` of the `FlexrayTpConfig`
    #[setter]
    fn set_cluster(&self, cluster: &FlexrayCluster) -> PyResult<()> {
        self.0
            .set_cluster(&cluster.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the `FlexrayCluster` of the `FlexrayTpConfig`
    #[getter]
    fn cluster(&self) -> Option<FlexrayCluster> {
        self.0.cluster().map(FlexrayCluster)
    }

    /// create a new `FlexrayTpPduPool`
    #[pyo3(signature = (name, /))]
    #[pyo3(text_signature = "(self, name: str, /)")]
    fn create_flexray_tp_pdu_pool(&self, name: &str) -> PyResult<FlexrayTpPduPool> {
        match self.0.create_flexray_tp_pdu_pool(name) {
            Ok(value) => Ok(FlexrayTpPduPool(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all `FlexrayTpPduPools`
    fn flexray_tp_pdu_pools(&self) -> FlexrayTpPduPoolIterator {
        FlexrayTpPduPoolIterator::new(self.0.flexray_tp_pdu_pools().map(FlexrayTpPduPool))
    }

    /// create a new `TpAddress`
    #[pyo3(signature = (name, address, /))]
    #[pyo3(text_signature = "(self, name: str, address: int, /)")]
    fn create_tp_address(&self, name: &str, address: u32) -> PyResult<TpAddress> {
        match self.0.create_tp_address(name, address) {
            Ok(value) => Ok(TpAddress(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all `TpAddresses`
    fn tp_addresses(&self) -> TpAddressIterator {
        TpAddressIterator::new(self.0.tp_addresses().map(TpAddress))
    }

    /// create a new `FlexrayTpConnection`
    #[pyo3(signature = (name, transmitter, direct_tp_sdu, connection_control, /))]
    #[pyo3(
        text_signature = "(self, name: Optional[str], transmitter: FlexrayTpNode, direct_tp_sdu: IPdu, connection_control: FlexrayTpConnectionControl, /)"
    )]
    fn create_flexray_tp_connection(
        &self,
        name: Option<&str>,
        transmitter: &FlexrayTpNode,
        direct_tp_sdu: &Bound<'_, PyAny>,
        connection_control: &FlexrayTpConnectionControl,
    ) -> PyResult<FlexrayTpConnection> {
        let direct_tp_sdu = pyany_to_ipdu(direct_tp_sdu)?;

        match self.0.create_flexray_tp_connection(
            name,
            &transmitter.0,
            &direct_tp_sdu,
            &connection_control.0,
        ) {
            Ok(value) => Ok(FlexrayTpConnection(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all `FlexrayTpConnections`
    fn flexray_tp_connections(&self) -> FlexrayTpConnectionIterator {
        FlexrayTpConnectionIterator::new(self.0.flexray_tp_connections().map(FlexrayTpConnection))
    }

    /// create a new `FlexrayTpConnectionControl`
    #[pyo3(signature = (name, /))]
    #[pyo3(text_signature = "(self, name: str, /)")]
    fn create_flexray_tp_connection_control(
        &self,
        name: &str,
    ) -> PyResult<FlexrayTpConnectionControl> {
        match self.0.create_flexray_tp_connection_control(name) {
            Ok(value) => Ok(FlexrayTpConnectionControl(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all `FlexrayTpConnectionControls`
    fn flexray_tp_connection_controls(&self) -> FlexrayTpConnectionControlIterator {
        FlexrayTpConnectionControlIterator::new(
            self.0
                .flexray_tp_connection_controls()
                .map(FlexrayTpConnectionControl),
        )
    }

    /// add a `FlexrayTpEcu` to the `FlexrayTpConfig`
    #[pyo3(signature = (ecu_instance, full_duplex_enabled, /))]
    #[pyo3(text_signature = "(self, ecu_instance: EcuInstance, full_duplex_enabled: bool, /)")]
    fn create_flexray_tp_ecu(
        &self,
        ecu_instance: &EcuInstance,
        full_duplex_enabled: bool,
    ) -> PyResult<FlexrayTpEcu> {
        match self
            .0
            .create_flexray_tp_ecu(&ecu_instance.0, full_duplex_enabled)
        {
            Ok(value) => Ok(FlexrayTpEcu(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all `FlexrayTpEcus`
    fn flexray_tp_ecus(&self) -> FlexrayTpEcuIterator {
        FlexrayTpEcuIterator::new(self.0.flexray_tp_ecus().map(FlexrayTpEcu))
    }

    /// create a new `FlexrayTpNode`
    #[pyo3(signature = (name, /))]
    #[pyo3(text_signature = "(self, name: str, /)")]
    fn create_flexray_tp_node(&self, name: &str) -> PyResult<FlexrayTpNode> {
        match self.0.create_flexray_tp_node(name) {
            Ok(value) => Ok(FlexrayTpNode(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all `FlexrayTpNodes`
    fn flexray_tp_nodes(&self) -> FlexrayTpNodeIterator {
        FlexrayTpNodeIterator::new(self.0.flexray_tp_nodes().map(FlexrayTpNode))
    }
}

//##################################################################

/// A `FlexrayTpPduPool` contains a set of `NPdus` that can be used for sending and receiving
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct FlexrayTpPduPool(
    pub(crate) autosar_data_abstraction::communication::FlexrayTpPduPool,
);

#[pymethods]
impl FlexrayTpPduPool {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::FlexrayTpPduPool::try_from(element.0.clone())
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

    /// add an `NPdu` to the `PduPool`
    #[pyo3(signature = (n_pdu, /))]
    #[pyo3(text_signature = "(self, n_pdu: NPdu, /)")]
    fn add_n_pdu(&self, n_pdu: &NPdu) -> PyResult<()> {
        self.0.add_n_pdu(&n_pdu.0).map_err(abstraction_err_to_pyerr)
    }

    /// iterate over all referenced `NPdus`
    fn n_pdus(&self) -> NPduIterator {
        NPduIterator::new(self.0.n_pdus().map(NPdu))
    }
}

//##################################################################

iterator_wrapper!(FlexrayTpPduPoolIterator, FlexrayTpPduPool);

//##################################################################

/// A `FlexrayTpConnection` defines a connection between `FlexrayTpNodes`
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct FlexrayTpConnection(
    pub(crate) autosar_data_abstraction::communication::FlexrayTpConnection,
);

#[pymethods]
impl FlexrayTpConnection {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::FlexrayTpConnection::try_from(
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

    /// set the transmitter of the connection
    #[setter]
    fn set_transmitter(&self, transmitter: &FlexrayTpNode) -> PyResult<()> {
        self.0
            .set_transmitter(&transmitter.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the transmitter of the connection
    #[getter]
    fn transmitter(&self) -> Option<FlexrayTpNode> {
        self.0.transmitter().map(FlexrayTpNode)
    }

    /// set the direct TP SDU of the connection
    #[setter]
    fn set_direct_tp_sdu(&self, direct_tp_sdu: &Bound<'_, PyAny>) -> PyResult<()> {
        let direct_tp_sdu = pyany_to_ipdu(direct_tp_sdu)?;

        self.0
            .set_direct_tp_sdu(&direct_tp_sdu)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the direct TP SDU of the connection
    #[getter]
    fn direct_tp_sdu(&self) -> Option<Py<PyAny>> {
        self.0
            .direct_tp_sdu()
            .and_then(|ipdu| ipdu_to_pyany(&ipdu).ok())
    }

    /// set the connection control of the connection
    #[setter]
    fn set_connection_control(
        &self,
        connection_control: &FlexrayTpConnectionControl,
    ) -> PyResult<()> {
        self.0
            .set_connection_control(&connection_control.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the connection control of the connection
    #[getter]
    fn connection_control(&self) -> Option<FlexrayTpConnectionControl> {
        self.0.connection_control().map(FlexrayTpConnectionControl)
    }

    /// add a receiver to the connection
    #[pyo3(signature = (receiver, /))]
    #[pyo3(text_signature = "(self, receiver: FlexrayTpNode, /)")]
    fn add_receiver(&self, receiver: &FlexrayTpNode) -> PyResult<()> {
        self.0
            .add_receiver(&receiver.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// iterate over all receivers of the connection
    fn receivers(&self) -> FlexrayTpNodeIterator {
        FlexrayTpNodeIterator::new(self.0.receivers().map(FlexrayTpNode))
    }

    /// set the reversed TP SDU of the connection
    /// This is used if the connection supports both sending and receiving
    #[setter]
    fn set_reversed_tp_sdu(&self, reversed_tp_sdu: &Bound<'_, PyAny>) -> PyResult<()> {
        let reversed_tp_sdu = pyany_to_ipdu(reversed_tp_sdu)?;

        self.0
            .set_reversed_tp_sdu(&reversed_tp_sdu)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the reversed TP SDU of the connection
    #[getter]
    fn reversed_tp_sdu(&self) -> Option<Py<PyAny>> {
        self.0
            .reversed_tp_sdu()
            .and_then(|ipdu| ipdu_to_pyany(&ipdu).ok())
    }

    /// set the TX `FlexrayTpPduPool` of the connection
    #[setter]
    fn set_tx_pdu_pool(&self, tx_pdu_pool: &FlexrayTpPduPool) -> PyResult<()> {
        self.0
            .set_tx_pdu_pool(&tx_pdu_pool.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the TX `FlexrayTpPduPool` of the connection
    #[getter]
    fn tx_pdu_pool(&self) -> Option<FlexrayTpPduPool> {
        self.0.tx_pdu_pool().map(FlexrayTpPduPool)
    }

    /// set the RX `FlexrayTpPduPool` of the connection
    #[setter]
    fn set_rx_pdu_pool(&self, rx_pdu_pool: &FlexrayTpPduPool) -> PyResult<()> {
        self.0
            .set_rx_pdu_pool(&rx_pdu_pool.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the RX `FlexrayTpPduPool` of the connection
    #[getter]
    fn rx_pdu_pool(&self) -> Option<FlexrayTpPduPool> {
        self.0.rx_pdu_pool().map(FlexrayTpPduPool)
    }

    /// set the multicast `TpAddress` of the connection
    #[setter]
    fn set_multicast_address(&self, multicast_address: Option<&TpAddress>) -> PyResult<()> {
        self.0
            .set_multicast_address(multicast_address.map(|tp_address| &tp_address.0))
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the multicast `TpAddress` of the connection
    #[getter]
    fn multicast_address(&self) -> Option<TpAddress> {
        self.0.multicast_address().map(TpAddress)
    }
}

//##################################################################

iterator_wrapper!(FlexrayTpConnectionIterator, FlexrayTpConnection);

//##################################################################

/// A `FlexrayTpConnectionControl` defines the connection control parameters for a `FlexrayTpConnection`
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct FlexrayTpConnectionControl(
    pub(crate) autosar_data_abstraction::communication::FlexrayTpConnectionControl,
);

#[pymethods]
impl FlexrayTpConnectionControl {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::FlexrayTpConnectionControl::try_from(
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

    /// set the maxFcWait value
    #[setter]
    fn set_max_fc_wait(&self, max_fc_wait: u32) -> PyResult<()> {
        self.0
            .set_max_fc_wait(max_fc_wait)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the maxFcWait value
    #[getter]
    fn max_fc_wait(&self) -> Option<u32> {
        self.0.max_fc_wait()
    }

    /// set the maxNumberOfNpduPerCycle value
    #[setter]
    fn set_max_number_of_npdu_per_cycle(&self, max_number_of_npdu_per_cycle: u32) -> PyResult<()> {
        self.0
            .set_max_number_of_npdu_per_cycle(max_number_of_npdu_per_cycle)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the maxNumberOfNpduPerCycle value
    #[getter]
    fn max_number_of_npdu_per_cycle(&self) -> Option<u32> {
        self.0.max_number_of_npdu_per_cycle()
    }

    /// set the maxRetries value
    #[setter]
    fn set_max_retries(&self, max_retries: u32) -> PyResult<()> {
        self.0
            .set_max_retries(max_retries)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the maxRetries value
    #[getter]
    fn max_retries(&self) -> Option<u32> {
        self.0.max_retries()
    }

    /// set the separationCycleExponent value
    #[setter]
    fn set_separation_cycle_exponent(&self, separation_cycle_exponent: u32) -> PyResult<()> {
        self.0
            .set_separation_cycle_exponent(separation_cycle_exponent)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the separationCycleExponent value
    #[getter]
    fn separation_cycle_exponent(&self) -> Option<u32> {
        self.0.separation_cycle_exponent()
    }
}

//##################################################################

iterator_wrapper!(
    FlexrayTpConnectionControlIterator,
    FlexrayTpConnectionControl
);

//##################################################################

/// A `FlexrayTpEcu` represents an ECU within the `FlexrayTpConfig`
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct FlexrayTpEcu(pub(crate) autosar_data_abstraction::communication::FlexrayTpEcu);

#[pymethods]
impl FlexrayTpEcu {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::FlexrayTpEcu::try_from(element.0.clone()) {
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

    /// set the ECU instance of the `FlexrayTpEcu`
    #[setter]
    fn set_ecu_instance(&self, ecu_instance: &EcuInstance) -> PyResult<()> {
        self.0
            .set_ecu_instance(&ecu_instance.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the ECU instance of the `FlexrayTpEcu`
    #[getter]
    fn ecu_instance(&self) -> Option<EcuInstance> {
        self.0.ecu_instance().map(EcuInstance)
    }

    /// set the full duplex enabled flag of the `FlexrayTpEcu`
    #[setter]
    fn set_full_duplex_enabled(&self, full_duplex_enabled: bool) -> PyResult<()> {
        self.0
            .set_full_duplex_enabled(full_duplex_enabled)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the full duplex enabled flag of the `FlexrayTpEcu`
    #[getter]
    fn full_duplex_enabled(&self) -> Option<bool> {
        self.0.full_duplex_enabled()
    }

    /// set the cycle time of the TP main function in seconds
    #[setter]
    fn set_cycle_time_main_function(&self, cycle_time_main_function: Option<f64>) -> PyResult<()> {
        self.0
            .set_cycle_time_main_function(cycle_time_main_function)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the cycle time of the TP main function in seconds
    #[getter]
    fn cycle_time_main_function(&self) -> Option<f64> {
        self.0.cycle_time_main_function()
    }

    /// set the cancellation status of the `FlexrayTpEcu`
    #[setter]
    fn set_cancellation(&self, cancellation: Option<bool>) -> PyResult<()> {
        self.0
            .set_cancellation(cancellation)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the cancellation status of the `FlexrayTpEcu`
    #[getter]
    fn cancellation(&self) -> Option<bool> {
        self.0.cancellation()
    }
}

//##################################################################

iterator_wrapper!(FlexrayTpEcuIterator, FlexrayTpEcu);

//##################################################################

/// A `FlexrayTpNode` provides the TP address and the connection to the topology description in a `FlexrayTpConfig`
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct FlexrayTpNode(pub(crate) autosar_data_abstraction::communication::FlexrayTpNode);

#[pymethods]
impl FlexrayTpNode {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::FlexrayTpNode::try_from(element.0.clone()) {
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

    /// set or remove `FlexrayTpAddress` of the node
    /// A TP address is mandatory for unicast nodes, but optional for multicast nodes
    /// Setting None will remove the element
    #[setter]
    fn set_tp_address(&self, tp_address: Option<&TpAddress>) -> PyResult<()> {
        self.0
            .set_tp_address(tp_address.map(|tp_address| &tp_address.0))
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the `FlexrayTpAddress` of the node
    #[getter]
    fn tp_address(&self) -> Option<TpAddress> {
        self.0.tp_address().map(TpAddress)
    }

    /// add a `FlexrayCommunicationConnector` to the node
    /// The node can be associated with up to 2 connectors.
    /// In a system description this reference is mandatory.
    #[pyo3(signature = (connector, /))]
    #[pyo3(text_signature = "(self, connector: FlexrayCommunicationConnector, /)")]
    fn add_communication_connector(
        &self,
        connector: &FlexrayCommunicationConnector,
    ) -> PyResult<()> {
        self.0
            .add_communication_connector(&connector.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// iterate over all `FlexrayCommunicationConnectors` of the node
    fn communication_connectors(&self) -> FlexrayCommunicationConnectorIterator {
        FlexrayCommunicationConnectorIterator::new(
            self.0
                .communication_connectors()
                .map(FlexrayCommunicationConnector),
        )
    }
}

//##################################################################

iterator_wrapper!(FlexrayTpNodeIterator, FlexrayTpNode);
