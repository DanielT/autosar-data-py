use crate::{
    Element,
    abstraction::{
        AutosarAbstractionError, EcuInstance, abstraction_err_to_pyerr,
        communication::{
            CanCluster, CanCommunicationConnector, NPdu, ipdu_to_pyany, pyany_to_ipdu,
        },
    },
    iterator_wrapper,
};
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};
use pyo3::prelude::*;

//#########################################################

/// Container for `CanTp` configuration
///
/// There should be one `CanTpConfig` for each CAN network in the system
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct CanTpConfig(pub(crate) autosar_data_abstraction::communication::CanTpConfig);

#[pymethods]
impl CanTpConfig {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::CanTpConfig::try_from(element.0.clone()) {
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

    /// set the `CanCluster` associated with this configuration
    #[setter]
    fn set_cluster(&self, can_cluster: &CanCluster) -> PyResult<()> {
        self.0
            .set_cluster(&can_cluster.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the `CanCluster` associated with this configuration
    #[getter]
    fn cluster(&self) -> Option<CanCluster> {
        self.0.cluster().map(CanCluster)
    }

    /// create a `CanTp` ECU in the configuration
    #[pyo3(signature = (ecu_instance, /, *, cycle_time_main_function=None))]
    #[pyo3(
        text_signature = "(self, ecu_instance: EcuInstance, /, *, cycle_time_main_function: Optional[float])"
    )]
    fn create_can_tp_ecu(
        &self,
        ecu_instance: &EcuInstance,
        cycle_time_main_function: Option<f64>,
    ) -> PyResult<CanTpEcu> {
        match self
            .0
            .create_can_tp_ecu(&ecu_instance.0, cycle_time_main_function)
        {
            Ok(ecu) => Ok(CanTpEcu(ecu)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// get an iterator over all ECUs in the configuration
    fn can_tp_ecus(&self) -> CanTpEcuIterator {
        CanTpEcuIterator::new(self.0.can_tp_ecus().map(CanTpEcu))
    }

    /// create a new `CanTpAddress` in the configuration
    #[pyo3(signature = (name, address, /))]
    #[pyo3(text_signature = "(self, name: str, address: int)")]
    fn create_can_tp_address(&self, name: &str, address: u32) -> PyResult<CanTpAddress> {
        match self.0.create_can_tp_address(name, address) {
            Ok(address) => Ok(CanTpAddress(address)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// get all of the Can Tp addresses in the configuration
    fn can_tp_addresses(&self) -> CanTpAddressIterator {
        CanTpAddressIterator::new(self.0.can_tp_addresses().map(CanTpAddress))
    }

    /// create a new `CanTpChannel` in the configuration
    #[pyo3(signature = (name, channel_id, mode, /))]
    #[pyo3(text_signature = "(self, name: str, channel_id: int, mode: CanTpChannelMode, /)")]
    fn create_can_tp_channel(
        &self,
        name: &str,
        channel_id: u32,
        mode: CanTpChannelMode,
    ) -> PyResult<CanTpChannel> {
        match self.0.create_can_tp_channel(name, channel_id, mode.into()) {
            Ok(channel) => Ok(CanTpChannel(channel)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// iterate over all `CanTpChannel`s in the configuration
    fn can_tp_channels(&self) -> CanTpChannelIterator {
        CanTpChannelIterator::new(self.0.can_tp_channels().map(CanTpChannel))
    }

    /// create a new `CanTpConnection` in the configuration
    #[pyo3(signature = (name, addressing_format, can_tp_channel, data_pdu, tp_sdu, padding_activation, /))]
    #[pyo3(
        text_signature = "(self, name: Optional[str], addressing_format: CanTpAddressingFormat, can_tp_channel: CanTpChannel, data_pdu: NPdu, tp_sdu: IPdu, padding_activation: bool, /)"
    )]
    fn create_can_tp_connection(
        &self,
        name: Option<&str>,
        addressing_format: CanTpAddressingFormat,
        can_tp_channel: &CanTpChannel,
        data_pdu: &NPdu,
        tp_sdu: &Bound<'_, PyAny>,
        padding_activation: bool,
    ) -> PyResult<CanTpConnection> {
        let tp_sdu = pyany_to_ipdu(tp_sdu)?;

        match self.0.create_can_tp_connection(
            name,
            addressing_format.into(),
            &can_tp_channel.0,
            &data_pdu.0,
            &tp_sdu,
            padding_activation,
        ) {
            Ok(connection) => Ok(CanTpConnection(connection)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// get all of the `CanTpConnections` in the configuration
    fn can_tp_connections(&self) -> CanTpConnectionIterator {
        CanTpConnectionIterator::new(self.0.can_tp_connections().map(CanTpConnection))
    }

    /// create a new `CanTpNode` in the configuration
    #[pyo3(signature = (name, /))]
    #[pyo3(text_signature = "(self, name: str, /)")]
    fn create_can_tp_node(&self, name: &str) -> PyResult<CanTpNode> {
        match self.0.create_can_tp_node(name) {
            Ok(node) => Ok(CanTpNode(node)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// get all of the `CanTpNodes` in the configuration
    fn can_tp_nodes(&self) -> CanTpNodeIterator {
        CanTpNodeIterator::new(self.0.can_tp_nodes().map(CanTpNode))
    }
}

//#########################################################

/// A `CanTpEcu` represents an ECU that is using the `CanTp` module
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct CanTpEcu(pub(crate) autosar_data_abstraction::communication::CanTpEcu);

#[pymethods]
impl CanTpEcu {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::CanTpEcu::try_from(element.0.clone()) {
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

    /// set the ECU instance of the `CanTpEcu`
    #[setter]
    fn set_ecu_instance(&self, ecu_instance: &EcuInstance) -> PyResult<()> {
        self.0
            .set_ecu_instance(&ecu_instance.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the ECU instance of the `CanTpEcu`
    #[getter]
    fn ecu_instance(&self) -> Option<EcuInstance> {
        self.0.ecu_instance().map(EcuInstance)
    }

    /// set the cycle time of the `CanTp` main function of the ECU
    #[setter]
    fn set_cycle_time_main_function(&self, cycle_time: Option<f64>) -> PyResult<()> {
        self.0
            .set_cycle_time_main_function(cycle_time)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the cycle time of the `CanTp` main function of the ECU
    #[getter]
    fn cycle_time_main_function(&self) -> Option<f64> {
        self.0.cycle_time_main_function()
    }
}

//#########################################################

iterator_wrapper!(CanTpEcuIterator, CanTpEcu);

//#########################################################

/// A `CanTpAddress` represents a logical address in the `CanTp` module
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct CanTpAddress(pub(crate) autosar_data_abstraction::communication::CanTpAddress);

#[pymethods]
impl CanTpAddress {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::CanTpAddress::try_from(element.0.clone()) {
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

    /// set the address value of the `CanTpAddress`
    #[setter]
    fn set_tp_address(&self, address: u32) -> PyResult<()> {
        self.0
            .set_tp_address(address)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the address value of the `CanTpAddress`
    #[getter]
    fn tp_address(&self) -> Option<u32> {
        self.0.tp_address()
    }
}

//#########################################################

iterator_wrapper!(CanTpAddressIterator, CanTpAddress);

//#########################################################

/// A `CanTpChannel` represents a channel in the `CanTp` module
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct CanTpChannel(pub(crate) autosar_data_abstraction::communication::CanTpChannel);

#[pymethods]
impl CanTpChannel {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::CanTpChannel::try_from(element.0.clone()) {
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

    /// set the channel id of the channel
    #[setter]
    fn set_channel_id(&self, channel_id: u32) -> PyResult<()> {
        self.0
            .set_channel_id(channel_id)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the channel id of the channel
    #[getter]
    fn channel_id(&self) -> Option<u32> {
        self.0.channel_id()
    }

    /// set the channel mode of the channel
    #[setter]
    fn set_channel_mode(&self, mode: CanTpChannelMode) -> PyResult<()> {
        self.0
            .set_channel_mode(mode.into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the channel mode of the channel
    #[getter]
    fn channel_mode(&self) -> Option<CanTpChannelMode> {
        self.0.channel_mode().map(std::convert::Into::into)
    }
}

//#########################################################

iterator_wrapper!(CanTpChannelIterator, CanTpChannel);

//#########################################################

/// The mode of a `CanTpChannel`
#[pyclass(
    frozen,
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanTpChannelMode {
    /// Full duplex mode
    FullDuplex,
    /// Half duplex mode
    HalfDuplex,
}

impl From<autosar_data_abstraction::communication::CanTpChannelMode> for CanTpChannelMode {
    fn from(mode: autosar_data_abstraction::communication::CanTpChannelMode) -> Self {
        match mode {
            autosar_data_abstraction::communication::CanTpChannelMode::FullDuplex => {
                CanTpChannelMode::FullDuplex
            }
            autosar_data_abstraction::communication::CanTpChannelMode::HalfDuplex => {
                CanTpChannelMode::HalfDuplex
            }
        }
    }
}

impl From<CanTpChannelMode> for autosar_data_abstraction::communication::CanTpChannelMode {
    fn from(mode: CanTpChannelMode) -> Self {
        match mode {
            CanTpChannelMode::FullDuplex => {
                autosar_data_abstraction::communication::CanTpChannelMode::FullDuplex
            }
            CanTpChannelMode::HalfDuplex => {
                autosar_data_abstraction::communication::CanTpChannelMode::HalfDuplex
            }
        }
    }
}

//#########################################################

/// A connection identifies the sender and the receiver of this particular communication.
/// The `CanTp` module routes a Pdu through this connection.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct CanTpConnection(
    pub(crate) autosar_data_abstraction::communication::CanTpConnection,
);

#[pymethods]
impl CanTpConnection {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::CanTpConnection::try_from(element.0.clone())
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

    /// set the `CanTpChannel` associated with this connection
    #[setter]
    fn set_channel(&self, channel: &CanTpChannel) -> PyResult<()> {
        self.0
            .set_channel(&channel.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the `CanTpChannel` associated with this connection
    #[getter]
    fn channel(&self) -> Option<CanTpChannel> {
        self.0.channel().map(CanTpChannel)
    }

    /// set the `NPdu` associated with this connection
    #[setter]
    fn set_data_pdu(&self, data_pdu: &NPdu) -> PyResult<()> {
        self.0
            .set_data_pdu(&data_pdu.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the `NPdu` associated with this connection
    ///
    /// This is the Pdu that is sent over the CAN network
    #[getter]
    fn data_pdu(&self) -> Option<NPdu> {
        self.0.data_pdu().map(NPdu)
    }

    /// set the `IPdu` associated with this connection
    #[setter]
    fn set_tp_sdu(&self, tp_sdu: &Bound<'_, PyAny>) -> PyResult<()> {
        let tp_sdu = pyany_to_ipdu(tp_sdu)?;
        self.0.set_tp_sdu(&tp_sdu).map_err(abstraction_err_to_pyerr)
    }

    /// get the `IPdu` associated with this connection
    ///
    /// This is the Pdu that is sent over the transport protocol
    #[getter]
    fn tp_sdu(&self) -> Option<Py<PyAny>> {
        self.0.tp_sdu().and_then(|ipdu| ipdu_to_pyany(&ipdu).ok())
    }

    /// set the addressing format of the connection
    #[setter]
    fn set_addressing_format(&self, addressing_format: CanTpAddressingFormat) -> PyResult<()> {
        self.0
            .set_addressing_format(addressing_format.into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the addressing format of the connection
    #[getter]
    fn addressing_format(&self) -> Option<CanTpAddressingFormat> {
        self.0.addressing_format().map(CanTpAddressingFormat::from)
    }

    /// set the padding activation of the connection
    #[setter]
    fn set_padding_activation(&self, padding_activation: bool) -> PyResult<()> {
        self.0
            .set_padding_activation(padding_activation)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the padding activation of the connection
    #[getter]
    fn padding_activation(&self) -> Option<bool> {
        self.0.padding_activation()
    }

    /// set the transmitter of the connection
    ///
    /// This is a `CanTpNode` representing an ECU that will send the data
    #[setter]
    fn set_transmitter(&self, transmitter: &CanTpNode) -> PyResult<()> {
        self.0
            .set_transmitter(&transmitter.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the transmitter of the connection
    #[getter]
    fn transmitter(&self) -> Option<CanTpNode> {
        self.0.transmitter().map(CanTpNode)
    }

    /// add a receiver to the connection
    ///
    /// This is a `CanTpNode` representing an ECU that will receive the data
    #[pyo3(signature = (receiver, /))]
    #[pyo3(text_signature = "(self, receiver: CanTpNode, /)")]
    fn add_receiver(&self, receiver: &CanTpNode) -> PyResult<()> {
        self.0
            .add_receiver(&receiver.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get all of the receivers of the connection
    fn receivers(&self) -> CanTpNodeIterator {
        CanTpNodeIterator::new(self.0.receivers().map(CanTpNode))
    }
}

//#########################################################

iterator_wrapper!(CanTpConnectionIterator, CanTpConnection);

//#########################################################

/// The addressing format of a `CanTpConnection`
#[pyclass(
    frozen,
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanTpAddressingFormat {
    /// Extended addressing format
    Extended,
    /// Mixed 11-bit addressing format
    Mixed,
    /// Mixed 29-bit addressing format
    Mixed29Bit,
    /// Normal fixed addressing format
    NormalFixed,
    /// Standard addressing format
    Standard,
}

impl From<autosar_data_abstraction::communication::CanTpAddressingFormat>
    for CanTpAddressingFormat
{
    fn from(format: autosar_data_abstraction::communication::CanTpAddressingFormat) -> Self {
        match format {
            autosar_data_abstraction::communication::CanTpAddressingFormat::Extended => {
                CanTpAddressingFormat::Extended
            }
            autosar_data_abstraction::communication::CanTpAddressingFormat::Mixed => {
                CanTpAddressingFormat::Mixed
            }
            autosar_data_abstraction::communication::CanTpAddressingFormat::Mixed29Bit => {
                CanTpAddressingFormat::Mixed29Bit
            }
            autosar_data_abstraction::communication::CanTpAddressingFormat::NormalFixed => {
                CanTpAddressingFormat::NormalFixed
            }
            autosar_data_abstraction::communication::CanTpAddressingFormat::Standard => {
                CanTpAddressingFormat::Standard
            }
        }
    }
}

impl From<CanTpAddressingFormat>
    for autosar_data_abstraction::communication::CanTpAddressingFormat
{
    fn from(format: CanTpAddressingFormat) -> Self {
        match format {
            CanTpAddressingFormat::Extended => {
                autosar_data_abstraction::communication::CanTpAddressingFormat::Extended
            }
            CanTpAddressingFormat::Mixed => {
                autosar_data_abstraction::communication::CanTpAddressingFormat::Mixed
            }
            CanTpAddressingFormat::Mixed29Bit => {
                autosar_data_abstraction::communication::CanTpAddressingFormat::Mixed29Bit
            }
            CanTpAddressingFormat::NormalFixed => {
                autosar_data_abstraction::communication::CanTpAddressingFormat::NormalFixed
            }
            CanTpAddressingFormat::Standard => {
                autosar_data_abstraction::communication::CanTpAddressingFormat::Standard
            }
        }
    }
}

//#########################################################

/// A `CanTpNode` provides the TP address and the connection to the topology description in a `CanTpConfig`
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct CanTpNode(pub(crate) autosar_data_abstraction::communication::CanTpNode);

#[pymethods]
impl CanTpNode {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::CanTpNode::try_from(element.0.clone()) {
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

    /// set the `CanTpAddress` of this Node
    #[setter]
    fn set_address(&self, address: &CanTpAddress) -> PyResult<()> {
        self.0
            .set_address(&address.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the `CanTpAddress` of this Node
    #[getter]
    fn address(&self) -> Option<CanTpAddress> {
        self.0.address().map(CanTpAddress)
    }

    /// set the reference to a `CanCommunicationConnector` between an `EcuInstance` and a `CanPhysicalChannel`
    ///
    /// The connector connects the ECU to the physical channel, so by setting this reference, the
    /// ECU is also connected to the `CanTpNode`
    #[setter]
    fn set_connector(&self, connector: &CanCommunicationConnector) -> PyResult<()> {
        self.0
            .set_connector(&connector.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the `CanCommunicationConnector` of this Node
    #[getter]
    fn connector(&self) -> Option<CanCommunicationConnector> {
        self.0.connector().map(CanCommunicationConnector)
    }
}

//#########################################################

iterator_wrapper!(CanTpNodeIterator, CanTpNode);
