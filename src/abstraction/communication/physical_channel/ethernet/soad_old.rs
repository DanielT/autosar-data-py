use crate::{
    Element,
    abstraction::{
        AutosarAbstractionError, abstraction_err_to_pyerr,
        communication::{
            EthernetPhysicalChannel, EventGroupControlType, PduCollectionTrigger, PduTriggering,
            PduTriggeringIterator, SocketAddress, pyany_to_pdu,
        },
    },
    iterator_wrapper,
};
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};
use pyo3::prelude::*;

/// A `SocketConnectionBundle` describes a connection between a server port and multiple client ports.
/// It contains multiple bundled connections, each transporting one or more PDUs.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct SocketConnectionBundle(
    pub(crate) autosar_data_abstraction::communication::SocketConnectionBundle,
);

#[pymethods]
impl SocketConnectionBundle {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::SocketConnectionBundle::try_from(
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

    /// get the physical channel containing this socket connection bundle
    #[getter]
    fn physical_channel(&self) -> PyResult<EthernetPhysicalChannel> {
        match self.0.physical_channel() {
            Ok(value) => Ok(EthernetPhysicalChannel(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    ///set the server port of this socket connection bundle
    #[setter]
    fn set_server_port(&self, server_port: &SocketAddress) -> PyResult<()> {
        self.0
            .set_server_port(&server_port.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the server port of this socket connection bundle
    #[getter]
    fn server_port(&self) -> Option<SocketAddress> {
        self.0.server_port().map(SocketAddress)
    }

    /// create a bundled `SocketConnection` between the server port and a client port
    #[pyo3(signature = (client_port, /))]
    #[pyo3(text_signature = "(self, client_port: SocketAddress, /)")]
    fn create_bundled_connection(&self, client_port: &SocketAddress) -> PyResult<SocketConnection> {
        match self.0.create_bundled_connection(&client_port.0) {
            Ok(value) => Ok(SocketConnection(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create an iterator over all bundled connections in this socket connection bundle
    fn bundled_connections(&self) -> SocketConnectionIterator {
        SocketConnectionIterator::new(self.0.bundled_connections().map(SocketConnection))
    }
}

//##################################################################

iterator_wrapper!(SocketConnectionBundleIterator, SocketConnectionBundle);

//##################################################################

/// A socketConnection inside a `SocketConnectionBundle` describes a single connection to a specific client port.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct SocketConnection(
    pub(crate) autosar_data_abstraction::communication::SocketConnection,
);

#[pymethods]
impl SocketConnection {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::SocketConnection::try_from(element.0.clone())
        {
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

    /// get the socket connection bundle containing this socket connection
    #[getter]
    fn socket_connection_bundle(&self) -> PyResult<SocketConnectionBundle> {
        match self.0.socket_connection_bundle() {
            Ok(value) => Ok(SocketConnectionBundle(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// set the client port of this socket connection
    #[setter]
    fn set_client_port(&self, client_port: &SocketAddress) -> PyResult<()> {
        self.0
            .set_client_port(&client_port.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the client port of this socket connection
    #[getter]
    fn client_port(&self) -> Option<SocketAddress> {
        self.0.client_port().map(SocketAddress)
    }

    /// add a PDU to the socket connection, returning a `PduTriggering`
    #[pyo3(signature = (pdu, header_id, /, *, timeout=None, collection_trigger=None))]
    #[pyo3(
        text_signature = "(self, pdu: Pdu, header_id: int, /, *, timeout: Optional[float] = None, collection_trigger: Optional[PduCollectionTrigger] = None)"
    )]
    fn create_socket_connection_ipdu_identifier(
        &self,
        pdu: &Bound<'_, PyAny>,
        header_id: u32,
        timeout: Option<f64>,
        collection_trigger: Option<PduCollectionTrigger>,
    ) -> PyResult<(SocketConnectionIpduIdentifier, PduTriggering)> {
        let pdu = pyany_to_pdu(pdu)?;
        match self.0.create_socket_connection_ipdu_identifier(
            &pdu,
            header_id,
            timeout,
            collection_trigger.map(Into::into),
        ) {
            Ok((identifier, triggering)) => Ok((
                SocketConnectionIpduIdentifier(identifier),
                PduTriggering(triggering),
            )),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create an iterator over all `SocketConnectionIpduIdentifiers` in this socket connection
    fn socket_connection_ipdu_identifiers(&self) -> SocketConnectionIpduIdentifierIterator {
        SocketConnectionIpduIdentifierIterator::new(
            self.0
                .socket_connection_ipdu_identifiers()
                .map(SocketConnectionIpduIdentifier),
        )
    }

    /// create an iterator over all PDU triggerings in this socket connection
    fn pdu_triggerings(&self) -> PduTriggeringIterator {
        PduTriggeringIterator::new(self.0.pdu_triggerings().map(PduTriggering))
    }

    /// set or remove the `client_ip_addr_from_connection_request` attribute for this socket connection
    ///
    /// if the value is Some(true), the attribute is set to "true"
    /// if the value is Some(false), the attribute is set to "false"
    /// if the value is None, the attribute is removed
    #[setter]
    fn set_client_ip_addr_from_connection_request(&self, value: Option<bool>) -> PyResult<()> {
        self.0
            .set_client_ip_addr_from_connection_request(value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the value of the `client_ip_addr_from_connection_request` attribute for this socket connection
    #[getter]
    fn client_ip_addr_from_connection_request(&self) -> Option<bool> {
        self.0.client_ip_addr_from_connection_request()
    }

    /// set or remove the `client_port_from_connection_request` attribute for this socket connection
    ///
    /// if the value is Some(true), the attribute is set to "true"
    /// if the value is Some(false), the attribute is set to "false"
    /// if the value is None, the attribute is removed
    #[setter]
    fn set_client_port_from_connection_request(&self, value: Option<bool>) -> PyResult<()> {
        self.0
            .set_client_port_from_connection_request(value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the value of the `client_port_from_connection_request` attribute for this socket connection
    #[getter]
    fn client_port_from_connection_request(&self) -> Option<bool> {
        self.0.client_port_from_connection_request()
    }

    /// set the value of the `runtime_ip_address_configuration` attribute for this socket connection
    #[setter]
    fn set_runtime_ip_address_configuration(&self, value: bool) -> PyResult<()> {
        self.0
            .set_runtime_ip_address_configuration(value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the value of the RuntimeIpAddressConfiguration attribute for this socket connection
    #[getter]
    fn runtime_ip_address_configuration(&self) -> bool {
        self.0.runtime_ip_address_configuration()
    }

    /// set the value of the `runtime_port_configuration` attribute for this socket connection
    #[setter]
    fn set_runtime_port_configuration(&self, value: bool) -> PyResult<()> {
        self.0
            .set_runtime_port_configuration(value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the value of the RuntimePortConfiguration attribute for this socket connection
    #[getter]
    fn runtime_port_configuration(&self) -> bool {
        self.0.runtime_port_configuration()
    }
}

//##################################################################

iterator_wrapper!(SocketConnectionIterator, SocketConnection);

//##################################################################

/// A `SocketConnectionIpduIdentifier` is used to trigger a PDU in a `SocketConnection`.
///
/// In addition to the Pdu Triggering, it also contains associated settings like the
/// header id, timeout and collection trigger.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SocketConnectionIpduIdentifier(
    pub(crate) autosar_data_abstraction::communication::SocketConnectionIpduIdentifier,
);

#[pymethods]
impl SocketConnectionIpduIdentifier {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::SocketConnectionIpduIdentifier::try_from(
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

    /// get the SocketConnection containing this `SocketConnectionIpduIdentifier`
    #[getter]
    fn socket_connection(&self) -> PyResult<SocketConnection> {
        match self.0.socket_connection() {
            Ok(value) => Ok(SocketConnection(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// trigger a PDU in this `SocketConnectionIpduIdentifier`, creating a `PduTriggering`
    #[pyo3(signature = (pdu, /))]
    #[pyo3(text_signature = "(self, pdu: Pdu, /)")]
    fn trigger_pdu(&self, pdu: &Bound<'_, PyAny>) -> PyResult<PduTriggering> {
        let pdu = pyany_to_pdu(pdu)?;
        match self.0.trigger_pdu(&pdu) {
            Ok(value) => Ok(PduTriggering(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// get the `PduTriggering` associated with this `SocketConnectionIpduIdentifier`
    #[getter]
    fn pdu_triggering(&self) -> Option<PduTriggering> {
        self.0.pdu_triggering().map(PduTriggering)
    }

    /// set the header id for this `SocketConnectionIpduIdentifier`
    #[setter]
    fn set_header_id(&self, header_id: u32) -> PyResult<()> {
        self.0
            .set_header_id(header_id)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the header id for this `SocketConnectionIpduIdentifier`
    #[getter]
    fn header_id(&self) -> Option<u64> {
        self.0.header_id()
    }

    /// set the timeout for this `SocketConnectionIpduIdentifier`
    #[setter]
    fn set_timeout(&self, timeout: Option<f64>) -> PyResult<()> {
        self.0
            .set_timeout(timeout)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the timeout for this `SocketConnectionIpduIdentifier`
    #[getter]
    fn timeout(&self) -> Option<f64> {
        self.0.timeout()
    }

    /// set the collection trigger for this `SocketConnectionIpduIdentifier`
    #[setter]
    fn set_collection_trigger(&self, trigger: Option<PduCollectionTrigger>) -> PyResult<()> {
        self.0
            .set_collection_trigger(trigger.map(Into::into))
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the collection trigger for this `SocketConnectionIpduIdentifier`
    #[getter]
    fn collection_trigger(&self) -> Option<PduCollectionTrigger> {
        self.0.collection_trigger().map(PduCollectionTrigger::from)
    }

    /// add a reference to a `SoAdRoutingGroup` to this `SocketConnectionIpduIdentifier`
    #[pyo3(signature = (routing_group, /))]
    #[pyo3(text_signature = "(self, routing_group: SoAdRoutingGroup, /)")]
    fn add_routing_group(&self, routing_group: &SoAdRoutingGroup) -> PyResult<()> {
        self.0
            .add_routing_group(&routing_group.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// create an iterator over all `SoAdRoutingGroups` referenced by this `SocketConnectionIpduIdentifier`
    fn routing_groups(&self) -> SoAdRoutingGroupIterator {
        SoAdRoutingGroupIterator::new(self.0.routing_groups().map(SoAdRoutingGroup))
    }
}

//##################################################################

iterator_wrapper!(
    SocketConnectionIpduIdentifierIterator,
    SocketConnectionIpduIdentifier
);

//##################################################################

/// A `SoAdRoutingGroup` is used to link `SomeIp` settings in Consumed/ProvidedServiceInstances
/// to the `SocketConnectionBundles` used for transmission.
/// `SoAdRoutingGroups` are part of the old way of configuring Ethernet communication in AUTOSAR.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct SoAdRoutingGroup(
    pub(crate) autosar_data_abstraction::communication::SoAdRoutingGroup,
);

#[pymethods]
impl SoAdRoutingGroup {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::SoAdRoutingGroup::try_from(element.0.clone())
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

    /// set the `EventGroupControlType` of this `SoAdRoutingGroup`
    #[setter]
    fn set_control_type(&self, control_type: EventGroupControlType) -> PyResult<()> {
        self.0
            .set_control_type(control_type.into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the `EventGroupControlType` of this `SoAdRoutingGroup`
    #[getter]
    fn control_type(&self) -> Option<EventGroupControlType> {
        self.0.control_type().map(Into::into)
    }
}

//##################################################################

iterator_wrapper!(SoAdRoutingGroupIterator, SoAdRoutingGroup);
