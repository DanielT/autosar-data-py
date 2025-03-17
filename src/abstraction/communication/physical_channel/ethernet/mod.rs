use crate::abstraction::AutosarAbstractionError;
use crate::abstraction::communication::{
    EthernetCluster, GeneralPurposePdu, ISignalTriggering, PduCollectionTrigger, PduTriggering,
    PduTriggeringIterator, SignalTriggeringsIterator, pyany_to_pdu,
};
use crate::{abstraction::*, *};
use autosar_data_abstraction::communication::AbstractPhysicalChannel;
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};

pub(crate) mod networkendpoint;
pub(crate) mod soad_old;
pub(crate) mod socketaddress;
pub(crate) mod someip;
pub(crate) mod someip_old;

pub(crate) use networkendpoint::*;
pub(crate) use soad_old::*;
pub(crate) use socketaddress::*;
pub(crate) use someip::*;
pub(crate) use someip_old::*;

//##################################################################

/// Provides information about the VLAN of an [`EthernetPhysicalChannel`]
#[pyclass(eq, module = "autosar_data._autosar_data._abstraction._communication")]
#[derive(Clone, PartialEq)]
pub(crate) struct EthernetVlanInfo(
    pub(crate) autosar_data_abstraction::communication::EthernetVlanInfo,
);

#[pymethods]
impl EthernetVlanInfo {
    #[pyo3(signature = (*, vlan_name, vlan_id))]
    #[pyo3(text_signature = "(*, vlan_name: str, vlan_id: int)")]
    #[new]
    fn new(vlan_name: &str, vlan_id: u16) -> Self {
        Self(autosar_data_abstraction::communication::EthernetVlanInfo {
            vlan_name: vlan_name.to_string(),
            vlan_id,
        })
    }

    #[setter]
    fn set_vlan_name(&mut self, vlan_name: &str) {
        self.0.vlan_name = vlan_name.to_string();
    }

    #[getter]
    fn vlan_name(&self) -> &str {
        &self.0.vlan_name
    }

    #[setter]
    fn set_vlan_id(&mut self, vlan_id: u16) {
        self.0.vlan_id = vlan_id;
    }

    #[getter]
    fn vlan_id(&self) -> u16 {
        self.0.vlan_id
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self.0)
    }
}

//##################################################################

/// The `EthernetPhysicalChannel` represents a VLAN or untagged traffic
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub struct EthernetPhysicalChannel(
    pub(crate) autosar_data_abstraction::communication::EthernetPhysicalChannel,
);

#[pymethods]
impl EthernetPhysicalChannel {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::EthernetPhysicalChannel::try_from(
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

    /// set the VLAN information for this channel
    ///
    /// In an EthernetCluster, each physical channel must have unique VLAN settings; only
    /// one channel can omit VLAN information - it carries untagged traffic.
    /// Setting duplicate VLAN information will result in an error.
    #[pyo3(signature = (vlan_info = None, /))]
    #[pyo3(text_signature = "(self, vlan_info: Optional[EthernetVlanInfo], /)")]
    fn set_vlan_info(&self, vlan_info: Option<EthernetVlanInfo>) -> PyResult<()> {
        self.0
            .set_vlan_info(vlan_info.as_ref().map(|info| &info.0))
            .map_err(abstraction_err_to_pyerr)
    }

    /// Retrieves the VLAN information from a channel
    ///
    /// An ethernet physical channel that represents untagged traffic has no VLAN information and returns None.
    fn vlan_info(&self) -> Option<EthernetVlanInfo> {
        self.0.vlan_info().map(EthernetVlanInfo)
    }

    /// get the cluster containing this physical channel
    #[getter]
    fn cluster(&self) -> PyResult<EthernetCluster> {
        match self.0.cluster() {
            Ok(cluster) => Ok(EthernetCluster(cluster)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// create a network endpoint - IPv4 or IPv6 address - for this channel
    ///
    /// In older versions of the Autosar standard, up to version 4.4.0, the `NetworkEndpoint` could be linked to an Ecu.
    /// The parameter `ecu` specifies the target.
    /// The link is obsoleted in newer versions, and will only be created if the file version allows it.
    #[pyo3(signature = (name, address, /, *, ecu=None))]
    #[pyo3(
        text_signature = "(self, name: str, address: NetworkEndpointAddress, /, *, ecu: Optional[EcuInstance] = None)"
    )]
    fn create_network_endpoint(
        &self,
        name: &str,
        address: NetworkEndpointAddress,
        ecu: Option<&EcuInstance>,
    ) -> PyResult<NetworkEndpoint> {
        match self
            .0
            .create_network_endpoint(name, address.into(), ecu.map(|ecu| &ecu.0))
        {
            Ok(endpoint) => Ok(NetworkEndpoint(endpoint)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// create an iterator over all [`NetworkEndpoint`]s in this channel
    fn network_endpoints(&self) -> NetworkEndpointIterator {
        NetworkEndpointIterator::new(self.0.network_endpoints().map(NetworkEndpoint))
    }

    /// create a socket address in the ethernet channel
    ///
    /// It contains the settings of the TCP/UDP port and links to a [`NetworkEndpoint`] which contains the IP address.
    /// The socket address can either be a unicast adress which is associated with a single ECU, or a multicast address
    #[pyo3(signature = (name, network_endpoint, tp_config, sa_type, /))]
    #[pyo3(
        text_signature = "(self, name: str, network_endpoint: NetworkEndpoint, tp_config: TpConfig, sa_type: SocketAddressType, /)"
    )]
    fn create_socket_address(
        &self,
        name: &str,
        network_endpoint: &NetworkEndpoint,
        tp_config: &TpConfig,
        sa_type: SocketAddressType,
    ) -> PyResult<SocketAddress> {
        match self.0.create_socket_address(
            name,
            &network_endpoint.0,
            &tp_config.clone().into(),
            sa_type.into(),
        ) {
            Ok(socket_address) => Ok(SocketAddress(socket_address)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// create an iterator over all [`SocketAddress`]es in this channel
    fn socket_addresses(&self) -> SocketAddressIterator {
        SocketAddressIterator::new(self.0.socket_addresses().map(SocketAddress))
    }

    /// create a socket connection bundle
    ///
    /// The `SocketConnectionBundle` is the "old" way to establish a connection between two sockets.
    /// It is deprecated in newer versions of the Autosar standard, but remains available for compatibility.
    #[pyo3(signature = (name, server_port, /))]
    #[pyo3(text_signature = "(self, name: str, server_port: SocketAddress, /)")]
    fn create_socket_connection_bundle(
        &self,
        name: &str,
        server_port: &SocketAddress,
    ) -> PyResult<SocketConnectionBundle> {
        match self.0.create_socket_connection_bundle(name, &server_port.0) {
            Ok(bundle) => Ok(SocketConnectionBundle(bundle)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// iterate over all socket connection bundles in this channel
    ///
    /// The `SocketConnectionBundle` is the "old" way to establish a connection between two sockets.
    /// It is deprecated in newer versions of the Autosar standard, but remains available for compatibility.
    fn socket_connection_bundles(&self) -> SocketConnectionBundleIterator {
        SocketConnectionBundleIterator::new(
            self.0
                .socket_connection_bundles()
                .map(SocketConnectionBundle),
        )
    }

    /// create a pair of static socket connections
    ///
    /// Static socket connections are usually created as a pair, one on each socket involved on the connection.
    /// This helper function creates both at once. To create a single connection, use [`SocketAddress::create_static_socket_connection`].
    ///
    /// If the connection is a TCP connection, the first port connects to the second port, and the second port listens for incoming connection.
    /// The ordering of `port_1` and `port_2` has no impact on the direction of the transported PDUs. This is defined in the `PduTriggering`.
    ///
    /// `StaticSocketConnection`s are the "new" way to establish a connection between two sockets.
    /// It was introduced in Autosar 4.5.0 (`AUTOSAR_00048`) and is the recommended way to create connections.
    ///
    /// `SocketConnectionBundles` (old) and `StaticSocketConnections` (new) may never be used in the same file.
    #[pyo3(signature = (name, port_1, port_2, /, *, tcp_connect_timeout=None))]
    #[pyo3(
        text_signature = "(self, name: str, port_1: SocketAddress, port_2: SocketAddress, /, *, tcp_connect_timeout: Optional[float] = None)"
    )]
    fn create_static_socket_connection_pair(
        &self,
        name: &str,
        port_1: &SocketAddress,
        port_2: &SocketAddress,
        tcp_connect_timeout: Option<f64>,
    ) -> PyResult<(StaticSocketConnection, StaticSocketConnection)> {
        match self.0.create_static_socket_connection_pair(
            name,
            &port_1.0,
            &port_2.0,
            tcp_connect_timeout,
        ) {
            Ok((conn_1, conn_2)) => Ok((
                StaticSocketConnection(conn_1),
                StaticSocketConnection(conn_2),
            )),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// configure SOME/IP service discovery (SD) for an ECU connected to this channel
    ///
    /// SD is used to broadcast service offers on the network and subscribe to services offered by other ECUs.
    /// This function configures the ECU to use the SOME/IP SD protocol.
    ///
    /// SD uses either socket connection bundles or static socket connections to communicate.
    ///
    /// `ecu` is the ECU that should be configured for SD.
    /// `unicast_socket` is the socket address used for unicast rx/tx communication by the ECU.
    /// `unicast_rx_pdu` and `unicast_tx_pdu` are the `GeneralPurposePdus` used for the unicast communication.
    /// `common_config` contains common configuration settings that can be used for all SD ECUs.
    ///  - `multicast_rx_socket` is the socket address used for multicast communication by all SD ECUs.
    ///  - `remote_socket` is a socket whose IP is set to ANY with UDP port 0, acting as the remote address in the SD communication.
    ///  - `name_prefix` is an optional prefix for the names of the created elements.
    ///  - `prefer_static_socket_connections` is a flag that determines if `SocketConnectionBundles` should be used instead of `StaticSocketConnections`.
    ///     This is only relevant if the type can't be detected automatically.
    ///  - `ipdu_identifier_set` is contains the `IPduIdentifiers` that are used in `StaticSocketConnections`.
    ///
    /// Note:
    /// Usually `SomeIP` SD is expected to use port 30490, but this is not mandatory.
    /// The port number is set in the sockets, and must be the same for all SD sockets.
    #[pyo3(signature = (ecu, unicast_socket, unicast_rx_pdu, unicast_tx_pdu, common_config, /))]
    #[pyo3(
        text_signature = "(self, ecu: EcuInstance, unicast_socket: SocketAddress, unicast_rx_pdu: GeneralPurposePdu, unicast_tx_pdu: GeneralPurposePdu, common_config: CommonServiceDiscoveryConfig, /)"
    )]
    fn configure_service_discovery_for_ecu(
        &self,
        ecu: &EcuInstance,
        unicast_socket: &SocketAddress,
        unicast_rx_pdu: &GeneralPurposePdu,
        unicast_tx_pdu: &GeneralPurposePdu,
        common_config: &CommonServiceDiscoveryConfig,
    ) -> PyResult<()> {
        self.0
            .configure_service_discovery_for_ecu(
                &ecu.0,
                &unicast_socket.0,
                &unicast_rx_pdu.0,
                &unicast_tx_pdu.0,
                &common_config.into(),
            )
            .map_err(abstraction_err_to_pyerr)
    }

    /// check if the channel contains any `SocketConnectionBundles` (old) or `SocketConnections` (very old)
    fn has_socket_connections(&self) -> bool {
        self.0.has_socket_connections()
    }

    /// iterate over all pdu triggerings of this physical channel
    fn pdu_triggerings(&self) -> PduTriggeringIterator {
        PduTriggeringIterator::new(self.0.pdu_triggerings().map(PduTriggering))
    }

    /// iterate over all ISignalTriggerings of this physical channel
    fn signal_triggerings(&self) -> SignalTriggeringsIterator {
        SignalTriggeringsIterator::new(self.0.signal_triggerings().map(ISignalTriggering))
    }
}

//##################################################################

/// A `CommonServiceDiscoveryConfig` contains common configuration settings for `System::configure_service_discovery_for_ecu`.
///
/// This struct contains ECU-independent settings that should be re-used for all ECUs that are configured for SD.
#[pyclass(
    eq,
    set_all,
    get_all,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CommonServiceDiscoveryConfig {
    /// the socket address used for multicast rx by all SD ECUs
    pub multicast_rx_socket: SocketAddress,
    /// the multicast rx PDU used by all SD ECUs
    pub multicast_rx_pdu: GeneralPurposePdu,
    /// the remote socket used for SD communication. This socket must have an IP address (v4 or v6) set to ANY.
    pub remote_socket: SocketAddress,
    /// `configure_service_discovery_for_ecu` checks if any `SocketConnectionBundles` exist. If so, the old configuration method must be used.
    /// If none are found and the version is new enough, both methods are possible, and this flag determines which one to use.
    pub prefer_static_socket_connections: bool,
    /// an ipdu identifier set in which `PduTriggerings` are created. Only needed for `StaticSocketConnections`.
    pub ipdu_identifier_set: Option<SocketConnectionIpduIdentifierSet>,
    /// an optional prefix for the names of the created elements
    pub name_prefix: Option<String>,
}

impl<'a> From<&'a CommonServiceDiscoveryConfig>
    for autosar_data_abstraction::communication::CommonServiceDiscoveryConfig<'a>
{
    fn from(config: &'a CommonServiceDiscoveryConfig) -> Self {
        autosar_data_abstraction::communication::CommonServiceDiscoveryConfig {
            multicast_rx_socket: &config.multicast_rx_socket.0,
            multicast_rx_pdu: &config.multicast_rx_pdu.0,
            remote_socket: &config.remote_socket.0,
            prefer_static_socket_connections: config.prefer_static_socket_connections,
            ipdu_identifier_set: config.ipdu_identifier_set.as_ref().map(|set| &set.0),
            name_prefix: config.name_prefix.as_deref(),
        }
    }
}

#[pymethods]
impl CommonServiceDiscoveryConfig {
    #[pyo3(signature = (*, multicast_rx_socket, multicast_rx_pdu, remote_socket, prefer_static_socket_connections, ipdu_identifier_set=None, name_prefix=None))]
    #[pyo3(
        text_signature = "(*, multicast_rx_socket: SocketAddress, multicast_rx_pdu: GeneralPurposePdu, remote_socket: SocketAddress, prefer_static_socket_connections: bool, ipdu_identifier_set: Optional[SocketConnectionIpduIdentifierSet] = None, name_prefix: Optional[str] = None)"
    )]
    #[new]
    fn new(
        multicast_rx_socket: &SocketAddress,
        multicast_rx_pdu: &GeneralPurposePdu,
        remote_socket: &SocketAddress,
        prefer_static_socket_connections: bool,
        ipdu_identifier_set: Option<SocketConnectionIpduIdentifierSet>,
        name_prefix: Option<String>,
    ) -> Self {
        Self {
            multicast_rx_socket: multicast_rx_socket.clone(),
            multicast_rx_pdu: multicast_rx_pdu.clone(),
            remote_socket: remote_socket.clone(),
            prefer_static_socket_connections,
            ipdu_identifier_set,
            name_prefix,
        }
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}

//#########################################################

/// A static socket connection is a connection between two sockets.
///
/// This is the new way to establish a connection. It was introduced in Autosar 4.5.0 (`AUTOSAR_00048`).
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StaticSocketConnection(
    pub(crate) autosar_data_abstraction::communication::StaticSocketConnection,
);

#[pymethods]
impl StaticSocketConnection {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::StaticSocketConnection::try_from(
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

    /// get the socket address containing this static socket connection
    #[getter]
    fn socket_address(&self) -> PyResult<SocketAddress> {
        match self.0.socket_address() {
            Ok(socket_address) => Ok(SocketAddress(socket_address)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// set the remote socket of this connection
    #[setter]
    fn set_remote_socket(&self, remote_socket: &SocketAddress) -> PyResult<()> {
        self.0
            .set_remote_socket(&remote_socket.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the remote socket of this connection
    #[getter]
    fn remote_socket(&self) -> Option<SocketAddress> {
        self.0.remote_socket().map(SocketAddress)
    }

    /// add a `SoConIPduIdentifier` to this static socket connection
    #[pyo3(signature = (identifier, /))]
    #[pyo3(text_signature = "(self, identifier: SoConIPduIdentifier, /)")]
    fn add_ipdu_identifier(&self, identifier: &SoConIPduIdentifier) -> PyResult<()> {
        self.0
            .add_ipdu_identifier(&identifier.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// create an iterator over all `SoConIPduIdentifiers` in this static socket connection
    fn ipdu_identifiers(&self) -> SoConIPduIdentifierIterator {
        SoConIPduIdentifierIterator::new(self.0.ipdu_identifiers().map(SoConIPduIdentifier))
    }

    /// set the TCP role of this static socket connection
    #[setter]
    fn set_tcp_role(&self, role: Option<TcpRole>) -> PyResult<()> {
        self.0
            .set_tcp_role(role.map(std::convert::Into::into))
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the TCP role of this static socket connection
    #[getter]
    fn tcp_role(&self) -> Option<TcpRole> {
        self.0.tcp_role().map(std::convert::Into::into)
    }

    /// set the TCP connect timeout of this static socket connection
    #[setter]
    fn set_tcp_connect_timeout(&self, timeout: Option<f64>) -> PyResult<()> {
        self.0
            .set_tcp_connect_timeout(timeout)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the TCP connect timeout of this static socket connection
    #[getter]
    fn tcp_connect_timeout(&self) -> Option<f64> {
        self.0.tcp_connect_timeout()
    }
}

//#########################################################

iterator_wrapper!(StaticSocketConnectionIterator, StaticSocketConnection);

//#########################################################

/// A `SocketConnectionIpduIdentifierSet` contains a set of `SoConIPduIdentifiers`, which are used in static socket connections and in `SomeIp` events.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SocketConnectionIpduIdentifierSet(
    pub(crate) autosar_data_abstraction::communication::SocketConnectionIpduIdentifierSet,
);

#[pymethods]
impl SocketConnectionIpduIdentifierSet {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::SocketConnectionIpduIdentifierSet::try_from(
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

    /// create a new `SoConIPduIdentifier` in this set
    #[pyo3(signature = (name, pdu, channel, /, *, header_id=None, timeout=None, collection_trigger=None))]
    #[pyo3(
        text_signature = "(self, name: str, pdu: Pdu, channel: EthernetPhysicalChannel, /, *, header_id: Optional[int] = None, timeout: Optional[float] = None, collection_trigger: Optional[PduCollectionTrigger] = None)"
    )]
    fn create_socon_ipdu_identifier(
        &self,
        name: &str,
        pdu: &Bound<'_, PyAny>,
        channel: &EthernetPhysicalChannel,
        header_id: Option<u64>,
        timeout: Option<f64>,
        collection_trigger: Option<PduCollectionTrigger>,
    ) -> PyResult<SoConIPduIdentifier> {
        let pdu = pyany_to_pdu(pdu)?;
        match self.0.create_socon_ipdu_identifier(
            name,
            &pdu,
            &channel.0,
            header_id,
            timeout,
            collection_trigger.map(std::convert::Into::into),
        ) {
            Ok(identifier) => Ok(SoConIPduIdentifier(identifier)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// create an iterator over all `SoConIPduIdentifiers` in this set
    fn socon_ipdu_identifiers(&self) -> SoConIPduIdentifierIterator {
        SoConIPduIdentifierIterator::new(self.0.socon_ipdu_identifiers().map(SoConIPduIdentifier))
    }
}

//#########################################################

/// A `SoConIPduIdentifier` describes a PDU that is transported over a static socket connection.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SoConIPduIdentifier(
    pub(crate) autosar_data_abstraction::communication::SoConIPduIdentifier,
);

#[pymethods]
impl SoConIPduIdentifier {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::SoConIPduIdentifier::try_from(
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

    /// create a new `PduTriggering` for the pdu and reference it in this `SoConIPduIdentifier`
    #[pyo3(signature = (pdu, channel, /))]
    #[pyo3(text_signature = "(self, pdu: Pdu, channel: EthernetPhysicalChannel, /)")]
    fn set_pdu(&self, pdu: &Bound<'_, PyAny>, channel: &EthernetPhysicalChannel) -> PyResult<()> {
        let pdu = pyany_to_pdu(pdu)?;
        self.0
            .set_pdu(&pdu, &channel.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the `PduTriggering` referenced by this `SoConIPduIdentifier`
    #[getter]
    fn pdu_triggering(&self) -> Option<PduTriggering> {
        self.0.pdu_triggering().map(PduTriggering)
    }

    /// set the header id for this `SoConIPduIdentifier`
    #[setter]
    fn set_header_id(&self, header_id: u64) -> PyResult<()> {
        self.0
            .set_header_id(header_id)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the header id for this `SoConIPduIdentifier`
    #[getter]
    fn header_id(&self) -> Option<u64> {
        self.0.header_id()
    }

    /// set the timeout for this `SoConIPduIdentifier`
    #[setter]
    fn set_timeout(&self, timeout: f64) -> PyResult<()> {
        self.0
            .set_timeout(timeout)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the timeout for this `SoConIPduIdentifier`
    #[getter]
    fn timeout(&self) -> Option<f64> {
        self.0.timeout()
    }

    /// set the collection trigger for this `SoConIPduIdentifier`
    #[setter]
    fn set_collection_trigger(&self, trigger: PduCollectionTrigger) -> PyResult<()> {
        self.0
            .set_collection_trigger(trigger.into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the collection trigger for this `SoConIPduIdentifier`
    #[getter]
    fn collection_trigger(&self) -> Option<PduCollectionTrigger> {
        self.0.collection_trigger().map(PduCollectionTrigger::from)
    }
}

//#########################################################

iterator_wrapper!(SoConIPduIdentifierIterator, SoConIPduIdentifier);

//##################################################################

/// The role of a TCP connection in a static socket connection can either be `Connect` (=client) or `Listen` (=server).
#[pyclass(
    frozen,
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TcpRole {
    /// The TCP socket is a client which connects to a server
    Connect,
    /// The TCP socket is a server which listens for incoming connections
    Listen,
}

impl From<autosar_data_abstraction::communication::TcpRole> for TcpRole {
    fn from(role: autosar_data_abstraction::communication::TcpRole) -> Self {
        match role {
            autosar_data_abstraction::communication::TcpRole::Connect => TcpRole::Connect,
            autosar_data_abstraction::communication::TcpRole::Listen => TcpRole::Listen,
        }
    }
}

impl From<TcpRole> for autosar_data_abstraction::communication::TcpRole {
    fn from(role: TcpRole) -> Self {
        match role {
            TcpRole::Connect => autosar_data_abstraction::communication::TcpRole::Connect,
            TcpRole::Listen => autosar_data_abstraction::communication::TcpRole::Listen,
        }
    }
}

//#########################################################

/// control types used in routing groups for SOME/IP events
#[pyclass(
    frozen,
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventGroupControlType {
    /// Activate the data path for unicast events and triggered unicast events that are sent out after a client got subscribed
    ActivationAndTriggerUnicast,
    /// Activate the data path for multicast events of an `EventGroup`
    ActivationMulticast,
    /// Activate the data path for unicast events
    ActivationUnicast,
    /// Activate the data path for triggered unicast events that are sent out after a client got subscribed
    TriggerUnicast,
}

impl From<autosar_data_abstraction::communication::EventGroupControlType>
    for EventGroupControlType
{
    fn from(control_type: autosar_data_abstraction::communication::EventGroupControlType) -> Self {
        match control_type {
            autosar_data_abstraction::communication::EventGroupControlType::ActivationAndTriggerUnicast => {
                EventGroupControlType::ActivationAndTriggerUnicast
            }
            autosar_data_abstraction::communication::EventGroupControlType::ActivationMulticast => {
                EventGroupControlType::ActivationMulticast
            }
            autosar_data_abstraction::communication::EventGroupControlType::ActivationUnicast => {
                EventGroupControlType::ActivationUnicast
            }
            autosar_data_abstraction::communication::EventGroupControlType::TriggerUnicast => {
                EventGroupControlType::TriggerUnicast
            }
        }
    }
}

impl From<EventGroupControlType>
    for autosar_data_abstraction::communication::EventGroupControlType
{
    fn from(control_type: EventGroupControlType) -> Self {
        match control_type {
            EventGroupControlType::ActivationAndTriggerUnicast => {
                autosar_data_abstraction::communication::EventGroupControlType::ActivationAndTriggerUnicast
            }
            EventGroupControlType::ActivationMulticast => {
                autosar_data_abstraction::communication::EventGroupControlType::ActivationMulticast
            }
            EventGroupControlType::ActivationUnicast => {
                autosar_data_abstraction::communication::EventGroupControlType::ActivationUnicast
            }
            EventGroupControlType::TriggerUnicast => {
                autosar_data_abstraction::communication::EventGroupControlType::TriggerUnicast
            }
        }
    }
}
