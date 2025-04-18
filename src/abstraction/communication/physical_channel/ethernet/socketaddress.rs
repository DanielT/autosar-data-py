use crate::{
    Element,
    abstraction::{
        AutosarAbstractionError, EcuInstance, abstraction_err_to_pyerr,
        communication::{
            ConsumedServiceInstanceV1, ConsumedServiceInstanceV1Iterator, EthernetPhysicalChannel,
            NetworkEndpoint, ProvidedServiceInstanceV1, ProvidedServiceInstanceV1Iterator,
            StaticSocketConnection, StaticSocketConnectionIterator, TcpRole,
        },
    },
    iterator_wrapper,
};
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};
use pyo3::prelude::*;

/// A socket address establishes the link between one or more ECUs and a `NetworkEndpoint`.
/// It contains all settings that are relevant for this combination.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SocketAddress(pub(crate) autosar_data_abstraction::communication::SocketAddress);

#[pymethods]
impl SocketAddress {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::SocketAddress::try_from(element.0.clone()) {
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

    /// get the network endpoint of this `SocketAddress`
    #[getter]
    fn network_endpoint(&self) -> Option<NetworkEndpoint> {
        self.0.network_endpoint().map(NetworkEndpoint)
    }

    /// get the socket address type: unicast / multicast, as well as the connected ecus
    #[getter]
    fn socket_address_type(&self) -> Option<SocketAddressType> {
        self.0.socket_address_type().map(SocketAddressType::from)
    }

    /// add an `EcuInstance` to this multicast `SocketAddress`
    #[pyo3(signature = (ecu, /))]
    #[pyo3(text_signature = "(self, ecu: EcuInstance, /)")]
    fn add_multicast_ecu(&self, ecu: &EcuInstance) -> PyResult<()> {
        self.0
            .add_multicast_ecu(&ecu.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// set the `EcuInstance` for this unicast `SocketAddress`
    #[pyo3(signature = (ecu, /))]
    #[pyo3(text_signature = "(self, ecu: EcuInstance, /)")]
    fn set_unicast_ecu(&self, ecu: &EcuInstance) -> PyResult<()> {
        self.0
            .set_unicast_ecu(&ecu.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the transport protocol settings for this `SocketAddress`
    #[getter]
    fn tp_config(&self) -> Option<TpConfig> {
        self.0.tp_config().map(TpConfig::from)
    }

    /// get the `EthernetPhysicalChannel` containing this `SocketAddress`
    #[getter]
    fn physical_channel(&self) -> PyResult<EthernetPhysicalChannel> {
        match self.0.physical_channel() {
            Ok(value) => Ok(EthernetPhysicalChannel(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `StaticSocketConnection` from this `SocketAddress` to a remote `SocketAddress`
    #[pyo3(signature = (name, remote_address, /, *, tcp_role=None, tcp_connect_timeout=None))]
    #[pyo3(
        text_signature = "(self, name: str, remote_address: SocketAddress, /, *, tcp_role: Optional[TcpRole] = None, tcp_connect_timeout: Optional[float] = None)"
    )]
    fn create_static_socket_connection(
        &self,
        name: &str,
        remote_address: &SocketAddress,
        tcp_role: Option<TcpRole>,
        tcp_connect_timeout: Option<f64>,
    ) -> PyResult<StaticSocketConnection> {
        match self.0.create_static_socket_connection(
            name,
            &remote_address.0,
            tcp_role.map(Into::into),
            tcp_connect_timeout,
        ) {
            Ok(value) => Ok(StaticSocketConnection(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all `StaticSocketConnection`s in this `SocketAddress`
    fn static_socket_connections(&self) -> StaticSocketConnectionIterator {
        StaticSocketConnectionIterator::new(
            self.0
                .static_socket_connections()
                .map(StaticSocketConnection),
        )
    }

    /// create a `ProvidedServiceInstanceV1` in this `SocketAddress`
    ///
    /// Creating a `ProvidedServiceInstanceV1` in a `SocketAddress` is part of the old way of defining services (<= Autosar 4.5.0).
    /// It is obsolete in newer versions of the standard.
    ///
    /// When using the new way of defining services, a `ProvidedServiceInstance` should be created in a `ServiceInstanceCollectionSet` instead.
    #[pyo3(signature = (name, service_identifier, instance_identifier, /))]
    #[pyo3(
        text_signature = "(self, name: str, service_identifier: int, instance_identifier: int, /)"
    )]
    fn create_provided_service_instance(
        &self,
        name: &str,
        service_identifier: u16,
        instance_identifier: u16,
    ) -> PyResult<ProvidedServiceInstanceV1> {
        match self
            .0
            .create_provided_service_instance(name, service_identifier, instance_identifier)
        {
            Ok(value) => Ok(ProvidedServiceInstanceV1(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// get the `ProvidedServiceInstanceV1`s in this `SocketAddress`
    fn provided_service_instances(&self) -> ProvidedServiceInstanceV1Iterator {
        ProvidedServiceInstanceV1Iterator::new(
            self.0
                .provided_service_instances()
                .map(ProvidedServiceInstanceV1),
        )
    }

    /// create a `ConsumedServiceInstanceV1` in this `SocketAddress`
    ///
    /// Creating a `ConsumedServiceInstanceV1` in a `SocketAddress` is part of the old way of defining services (<= Autosar 4.5.0).
    /// It is obsolete in newer versions of the standard.
    ///
    /// When using the new way of defining services, a `ConsumedServiceInstance` should be created in a `ServiceInstanceCollectionSet` instead.
    #[pyo3(signature = (name, provided_service_instance, /))]
    #[pyo3(
        text_signature = "(self, name: str, provided_service_instance: ProvidedServiceInstanceV1, /)"
    )]
    fn create_consumed_service_instance(
        &self,
        name: &str,
        provided_service_instance: &ProvidedServiceInstanceV1,
    ) -> PyResult<ConsumedServiceInstanceV1> {
        match self
            .0
            .create_consumed_service_instance(name, &provided_service_instance.0)
        {
            Ok(value) => Ok(ConsumedServiceInstanceV1(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// get the `ConsumedServiceInstance`s in this `SocketAddress`
    fn consumed_service_instances(&self) -> ConsumedServiceInstanceV1Iterator {
        ConsumedServiceInstanceV1Iterator::new(
            self.0
                .consumed_service_instances()
                .map(ConsumedServiceInstanceV1),
        )
    }
}

//##################################################################

iterator_wrapper!(SocketAddressIterator, SocketAddress);

//##################################################################

/// transport protocol settings of a [`SocketAddress`]
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TpConfig {
    /// The socket uses TCP
    #[pyo3(constructor = (*, port_number = None, port_dynamically_assigned = None))]
    TcpTp {
        /// The port number used by the socket
        port_number: Option<u16>,
        /// If the port number is dynamically assigned. Obsolete; set the port number to None instead
        port_dynamically_assigned: Option<bool>,
        // additional TCP options: currently not supported
    },
    /// The socket uses UDP
    #[pyo3(constructor = (*, port_number = None, port_dynamically_assigned = None))]
    UdpTp {
        /// The port number used by the socket
        port_number: Option<u16>,
        /// If the port number is dynamically assigned. Obsolete; set the port number to None instead
        port_dynamically_assigned: Option<bool>,
    },
    // RtpTp, Ieee1722Tp, HttpTp: currently not supported
}

impl From<autosar_data_abstraction::communication::TpConfig> for TpConfig {
    fn from(tp_config: autosar_data_abstraction::communication::TpConfig) -> Self {
        match tp_config {
            autosar_data_abstraction::communication::TpConfig::TcpTp {
                port_number,
                port_dynamically_assigned,
            } => TpConfig::TcpTp {
                port_number,
                port_dynamically_assigned,
            },
            autosar_data_abstraction::communication::TpConfig::UdpTp {
                port_number,
                port_dynamically_assigned,
            } => TpConfig::UdpTp {
                port_number,
                port_dynamically_assigned,
            },
        }
    }
}

impl From<TpConfig> for autosar_data_abstraction::communication::TpConfig {
    fn from(tp_config: TpConfig) -> Self {
        match tp_config {
            TpConfig::TcpTp {
                port_number,
                port_dynamically_assigned,
            } => autosar_data_abstraction::communication::TpConfig::TcpTp {
                port_number,
                port_dynamically_assigned,
            },
            TpConfig::UdpTp {
                port_number,
                port_dynamically_assigned,
            } => autosar_data_abstraction::communication::TpConfig::UdpTp {
                port_number,
                port_dynamically_assigned,
            },
        }
    }
}

#[pymethods]
impl TpConfig {
    fn __repr__(&self) -> String {
        match self {
            TpConfig::TcpTp {
                port_number,
                port_dynamically_assigned,
            } => format!(
                "TpConfig.TcpTp(port_number: {port_number:?}, port_dynamically_assigned: {port_dynamically_assigned:?})"
            ),
            TpConfig::UdpTp {
                port_number,
                port_dynamically_assigned,
            } => format!(
                "TpConfig.UdpTp(port_number: {port_number:?}, port_dynamically_assigned: {port_dynamically_assigned:?})"
            ),
        }
    }
}

//##################################################################

/// Describes if a [`SocketAddress`] is used for unicast or multicast
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) enum SocketAddressType {
    /// The socket is used for unicast communication with a single ECU
    #[pyo3(constructor = (ecu))]
    Unicast { ecu: Option<EcuInstance> },
    /// The socket is used for multicast communication with multiple ECUs
    #[pyo3(constructor = (ecus))]
    Multicast { ecus: Vec<EcuInstance> },
}

impl From<autosar_data_abstraction::communication::SocketAddressType> for SocketAddressType {
    fn from(
        socket_address_type: autosar_data_abstraction::communication::SocketAddressType,
    ) -> Self {
        match socket_address_type {
            autosar_data_abstraction::communication::SocketAddressType::Unicast(ecu) => {
                SocketAddressType::Unicast {
                    ecu: ecu.map(EcuInstance),
                }
            }
            autosar_data_abstraction::communication::SocketAddressType::Multicast(ecus) => {
                SocketAddressType::Multicast {
                    ecus: ecus.into_iter().map(EcuInstance).collect(),
                }
            }
        }
    }
}

impl From<SocketAddressType> for autosar_data_abstraction::communication::SocketAddressType {
    fn from(socket_address_type: SocketAddressType) -> Self {
        match socket_address_type {
            SocketAddressType::Unicast { ecu } => {
                autosar_data_abstraction::communication::SocketAddressType::Unicast(
                    ecu.map(|ecu| ecu.0),
                )
            }
            SocketAddressType::Multicast { ecus } => {
                autosar_data_abstraction::communication::SocketAddressType::Multicast(
                    ecus.into_iter().map(|ecu| ecu.0).collect(),
                )
            }
        }
    }
}

#[pymethods]
impl SocketAddressType {
    fn __repr__(&self) -> String {
        match self {
            SocketAddressType::Unicast { ecu } => {
                format!(
                    "SocketAddressType.Unicast({:?})",
                    ecu.as_ref().map(|ecu| &ecu.0)
                )
            }
            SocketAddressType::Multicast { ecus } => {
                format!("SocketAddressType.Multicast({ecus:?})")
            }
        }
    }
}
