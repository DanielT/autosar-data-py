use crate::{
    Element,
    abstraction::{
        AutosarAbstractionError, abstraction_err_to_pyerr,
        communication::{
            FlexrayCluster, FlexrayCommunicationConnector, FlexrayCommunicationConnectorIterator,
            NPdu, NPduIterator, TpAddress, TpAddressIterator, ipdu_to_pyany, pyany_to_ipdu,
        },
    },
    iterator_wrapper,
};
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};
use pyo3::prelude::*;

//#########################################################

/// The `FlexrayArTpConfig` represents the configuration of the Flexray Autosar Transport Protocol
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct FlexrayArTpConfig(
    pub(crate) autosar_data_abstraction::communication::FlexrayArTpConfig,
);

#[pymethods]
impl FlexrayArTpConfig {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::FlexrayArTpConfig::try_from(
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

    /// set the Flexray cluster for the configuration
    #[setter]
    fn set_cluster(&self, cluster: &FlexrayCluster) -> PyResult<()> {
        self.0
            .set_cluster(&cluster.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the Flexray cluster for the configuration
    #[getter]
    fn cluster(&self) -> Option<FlexrayCluster> {
        self.0.cluster().map(FlexrayCluster)
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

    /// create a new `FlexrayArTpChannel`
    #[pyo3(signature = (ack_type, extended_addressing, maximum_message_length, minimum_separation_time, multicast_segmentation, /))]
    #[pyo3(
        text_signature = "(self, ack_type: FrArTpAckType, extended_addressing: bool, maximum_message_length: MaximumMessageLengthType, minimum_separation_time: float, multicast_segmentation: bool, /)"
    )]
    fn create_flexray_ar_tp_channel(
        &self,
        ack_type: FrArTpAckType,
        extended_addressing: bool,
        maximum_message_length: MaximumMessageLengthType,
        minimum_separation_time: f32,
        multicast_segmentation: bool,
    ) -> PyResult<FlexrayArTpChannel> {
        match self.0.create_flexray_ar_tp_channel(
            ack_type.into(),
            extended_addressing,
            maximum_message_length.into(),
            minimum_separation_time,
            multicast_segmentation,
        ) {
            Ok(value) => Ok(FlexrayArTpChannel(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// get an iterator over the channels in the configuration
    fn flexray_ar_tp_channels(&self) -> FlexrayArTpChannelIterator {
        FlexrayArTpChannelIterator::new(self.0.flexray_ar_tp_channels().map(FlexrayArTpChannel))
    }

    /// create a new `FlexrayArTpNode`
    #[pyo3(signature = (name, /))]
    #[pyo3(text_signature = "(self, name: str, /)")]
    fn create_flexray_ar_tp_node(&self, name: &str) -> PyResult<FlexrayArTpNode> {
        match self.0.create_flexray_ar_tp_node(name) {
            Ok(value) => Ok(FlexrayArTpNode(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// get an iterator over the nodes
    fn flexray_ar_tp_nodes(&self) -> FlexrayArTpNodeIterator {
        FlexrayArTpNodeIterator::new(self.0.flexray_ar_tp_nodes().map(FlexrayArTpNode))
    }
}

//##################################################################

/// The `FlexrayArTpChannel` represents a channel in the Flexray Autosar Transport Protocol
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct FlexrayArTpChannel(
    pub(crate) autosar_data_abstraction::communication::FlexrayArTpChannel,
);

#[pymethods]
impl FlexrayArTpChannel {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::FlexrayArTpChannel::try_from(
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

    /// set the ack type of the channel
    #[setter]
    fn set_ack_type(&self, ack_type: FrArTpAckType) -> PyResult<()> {
        self.0
            .set_ack_type(ack_type.into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the ack type of the channel
    #[getter]
    fn ack_type(&self) -> Option<FrArTpAckType> {
        self.0.ack_type().map(FrArTpAckType::from)
    }

    /// set the extended addressing attribute
    #[setter]
    fn set_extended_addressing(&self, extended_addressing: bool) -> PyResult<()> {
        self.0
            .set_extended_addressing(extended_addressing)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the extended addressing attribute
    #[getter]
    fn extended_addressing(&self) -> Option<bool> {
        self.0.extended_addressing()
    }

    /// set the maximum message length type
    #[setter]
    fn set_maximum_message_length(
        &self,
        maximum_message_length: MaximumMessageLengthType,
    ) -> PyResult<()> {
        self.0
            .set_maximum_message_length(maximum_message_length.into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the maximum message length type
    #[getter]
    fn maximum_message_length(&self) -> Option<MaximumMessageLengthType> {
        self.0
            .maximum_message_length()
            .map(MaximumMessageLengthType::from)
    }

    /// set the minimum separation time
    #[setter]
    fn set_minimum_separation_time(&self, minimum_separation_time: f32) -> PyResult<()> {
        self.0
            .set_minimum_separation_time(minimum_separation_time)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the minimum separation time
    #[getter]
    fn minimum_separation_time(&self) -> Option<f32> {
        self.0.minimum_separation_time()
    }

    /// set the multicast segmentation
    #[setter]
    fn set_multicast_segmentation(&self, multicast_segmentation: bool) -> PyResult<()> {
        self.0
            .set_multicast_segmentation(multicast_segmentation)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the multicast segmentation
    #[getter]
    fn multicast_segmentation(&self) -> Option<bool> {
        self.0.multicast_segmentation()
    }

    /// create a new `FlexrayArTpConnection` for this channel
    #[pyo3(signature = (name, direct_tp_sdu, source, target, /))]
    #[pyo3(
        text_signature = "(self, name: Optional[str], direct_tp_sdu: IPdu, source: FlexrayArTpNode, target: FlexrayArTpNode, /)"
    )]
    fn create_flexray_ar_tp_connection(
        &self,
        name: Option<&str>,
        direct_tp_sdu: &Bound<'_, PyAny>,
        source: &FlexrayArTpNode,
        target: &FlexrayArTpNode,
    ) -> PyResult<FlexrayArTpConnection> {
        let direct_tp_sdu = pyany_to_ipdu(direct_tp_sdu)?;
        match self
            .0
            .create_flexray_ar_tp_connection(name, &direct_tp_sdu, &source.0, &target.0)
        {
            Ok(value) => Ok(FlexrayArTpConnection(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// get an iterator over the connections of the channel
    fn flexray_ar_tp_connections(&self) -> FlexrayArTpConnectionIterator {
        FlexrayArTpConnectionIterator::new(
            self.0
                .flexray_ar_tp_connections()
                .map(FlexrayArTpConnection),
        )
    }

    /// add an N-PDU to the channel
    ///
    /// The `NPdus` are logically assembled into a pool of Rx `NPdus` and another pool of Tx `NPdus`.
    /// This function is supported on autosar 4.1 and later, while Autosar 4.0 uses a different approach.
    #[pyo3(signature = (n_pdu, /))]
    #[pyo3(text_signature = "(self, n_pdu: NPdu, /)")]
    fn add_n_pdu(&self, n_pdu: &NPdu) -> PyResult<()> {
        self.0.add_n_pdu(&n_pdu.0).map_err(abstraction_err_to_pyerr)
    }

    /// iterate over the `NPdus` of the channel
    fn n_pdus(&self) -> NPduIterator {
        NPduIterator::new(self.0.n_pdus().map(NPdu))
    }
}

//#########################################################

iterator_wrapper!(FlexrayArTpChannelIterator, FlexrayArTpChannel);

//#########################################################

/// Types of Acknowledgement that can be used in an `FlexrayArTpChannel`
#[pyclass(
    frozen,
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FrArTpAckType {
    /// Acknowledgement without retry
    AckWithoutRt,
    /// Acknowledgement with retry
    AckWithRt,
    /// No acknowledgement
    NoAck,
}

impl From<FrArTpAckType> for autosar_data_abstraction::communication::FrArTpAckType {
    fn from(value: FrArTpAckType) -> Self {
        match value {
            FrArTpAckType::AckWithoutRt => {
                autosar_data_abstraction::communication::FrArTpAckType::AckWithoutRt
            }
            FrArTpAckType::AckWithRt => {
                autosar_data_abstraction::communication::FrArTpAckType::AckWithRt
            }
            FrArTpAckType::NoAck => autosar_data_abstraction::communication::FrArTpAckType::NoAck,
        }
    }
}

impl From<autosar_data_abstraction::communication::FrArTpAckType> for FrArTpAckType {
    fn from(value: autosar_data_abstraction::communication::FrArTpAckType) -> Self {
        match value {
            autosar_data_abstraction::communication::FrArTpAckType::AckWithoutRt => {
                FrArTpAckType::AckWithoutRt
            }
            autosar_data_abstraction::communication::FrArTpAckType::AckWithRt => {
                FrArTpAckType::AckWithRt
            }
            autosar_data_abstraction::communication::FrArTpAckType::NoAck => FrArTpAckType::NoAck,
        }
    }
}

//#########################################################

/// Types of Maximum Message Length that can be used in an `FlexrayArTpChannel`
#[pyclass(
    frozen,
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MaximumMessageLengthType {
    /// I4G: up to (2**32)-1 byte message length
    I4g,
    /// ISO: up to 4095 byte message length
    Iso,
    /// ISO6: payload length is limited to 6 byte (SF-I, FF-I, CF). This is necessary to route TP on CAN
    Iso6,
}

impl From<MaximumMessageLengthType>
    for autosar_data_abstraction::communication::MaximumMessageLengthType
{
    fn from(value: MaximumMessageLengthType) -> Self {
        match value {
            MaximumMessageLengthType::I4g => {
                autosar_data_abstraction::communication::MaximumMessageLengthType::I4g
            }
            MaximumMessageLengthType::Iso => {
                autosar_data_abstraction::communication::MaximumMessageLengthType::Iso
            }
            MaximumMessageLengthType::Iso6 => {
                autosar_data_abstraction::communication::MaximumMessageLengthType::Iso6
            }
        }
    }
}

impl From<autosar_data_abstraction::communication::MaximumMessageLengthType>
    for MaximumMessageLengthType
{
    fn from(value: autosar_data_abstraction::communication::MaximumMessageLengthType) -> Self {
        match value {
            autosar_data_abstraction::communication::MaximumMessageLengthType::I4g => {
                MaximumMessageLengthType::I4g
            }
            autosar_data_abstraction::communication::MaximumMessageLengthType::Iso => {
                MaximumMessageLengthType::Iso
            }
            autosar_data_abstraction::communication::MaximumMessageLengthType::Iso6 => {
                MaximumMessageLengthType::Iso6
            }
        }
    }
}

//##################################################################

/// `FlexrayArTpConnection` represents a connection within a `FlexrayArTpChannel`
///
/// The connection identifies the sender and the receiver of this particular communication.
/// The Flexray Autosar Tp module routes a Pdu through this connection.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct FlexrayArTpConnection(
    pub(crate) autosar_data_abstraction::communication::FlexrayArTpConnection,
);

#[pymethods]
impl FlexrayArTpConnection {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::FlexrayArTpConnection::try_from(
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

    /// set the direct TP SDU
    #[setter]
    fn set_direct_tp_sdu(&self, direct_tp_sdu: &Bound<'_, PyAny>) -> PyResult<()> {
        let direct_tp_sdu = pyany_to_ipdu(direct_tp_sdu)?;
        self.0
            .set_direct_tp_sdu(&direct_tp_sdu)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the direct tp sdu
    #[getter]
    fn direct_tp_sdu(&self) -> Option<Py<PyAny>> {
        self.0
            .direct_tp_sdu()
            .and_then(|ipdu| ipdu_to_pyany(&ipdu).ok())
    }

    /// set the source of the connection
    #[setter]
    fn set_source(&self, source: &FlexrayArTpNode) -> PyResult<()> {
        self.0
            .set_source(&source.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the source of the connection
    #[getter]
    fn source(&self) -> Option<FlexrayArTpNode> {
        self.0.source().map(FlexrayArTpNode)
    }

    /// add a target to the connection
    ///
    /// The connection can have multiple targets, but at least one target is required.
    #[pyo3(signature = (target, /))]
    #[pyo3(text_signature = "(self, target: FlexrayArTpNode, /)")]
    fn add_target(&self, target: &FlexrayArTpNode) -> PyResult<()> {
        self.0
            .add_target(&target.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the targets
    fn targets(&self) -> FlexrayArTpNodeIterator {
        FlexrayArTpNodeIterator::new(self.0.targets().map(FlexrayArTpNode))
    }

    /// set or remove the reversed TP SDU
    ///
    /// If the connection supports both directions, then the reversed TP SDU is required.
    /// if Some(value) is passed, the reversed TP SDU is set to the given value, otherwise it is removed.
    #[setter]
    fn set_reversed_tp_sdu(&self, reversed_tp_sdu: Option<&Bound<'_, PyAny>>) -> PyResult<()> {
        let reversed_tp_sdu = reversed_tp_sdu.map(pyany_to_ipdu).transpose()?;
        self.0
            .set_reversed_tp_sdu(reversed_tp_sdu.as_ref())
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the reversed tp sdu
    #[getter]
    fn reversed_tp_sdu(&self) -> Option<Py<PyAny>> {
        self.0
            .reversed_tp_sdu()
            .and_then(|ipdu| ipdu_to_pyany(&ipdu).ok())
    }
}

//##################################################################

iterator_wrapper!(FlexrayArTpConnectionIterator, FlexrayArTpConnection);

//##################################################################

/// `FlexrayArTpNode` represents a node in the Flexray Autosar Transport Protocol
///
/// A TP node (sender or receiver) provides the TP address and the connection to the topology description
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct FlexrayArTpNode(
    pub(crate) autosar_data_abstraction::communication::FlexrayArTpNode,
);

#[pymethods]
impl FlexrayArTpNode {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::FlexrayArTpNode::try_from(element.0.clone())
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

    /// set or remove the TP address
    ///
    /// if Some(value) is passed, the TP address is set to the given value, otherwise it is removed.
    #[setter]
    fn set_tp_address(&self, tp_address: Option<&TpAddress>) -> PyResult<()> {
        self.0
            .set_tp_address(tp_address.map(|tp_address| &tp_address.0))
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the TP address
    #[getter]
    fn tp_address(&self) -> Option<TpAddress> {
        self.0.tp_address().map(TpAddress)
    }

    /// add a reference to a `FlexrayCommunicationConnector`
    ///
    /// The connectors define the association with a `PhysicalChannel` and an ECU.
    /// In a `SystemDescription`, this reference is mandatory, but in an `ECUExtract` it is optional.
    /// Up to 2 connectors can be added to a node.
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

    /// get the connectors
    fn communication_connectors(&self) -> FlexrayCommunicationConnectorIterator {
        FlexrayCommunicationConnectorIterator::new(
            self.0
                .communication_connectors()
                .map(FlexrayCommunicationConnector),
        )
    }
}

//##################################################################

iterator_wrapper!(FlexrayArTpNodeIterator, FlexrayArTpNode);
