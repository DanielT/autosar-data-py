use crate::abstraction::communication::{
    CanCluster, EthernetCluster, EventGroupControlType, FlexrayCluster, ISignalIPdu, LinCluster, PduTriggering, SoConIPduIdentifier, SoConIPduIdentifierIterator, SocketAddress, SocketAddressIterator
};
use crate::abstraction::{AutosarAbstractionError, abstraction_err_to_pyerr};
use crate::{Element, iterator_wrapper};
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};
use pyo3::exceptions::PyTypeError;
use pyo3::{IntoPyObjectExt, prelude::*};

//##################################################################

/// A `ServiceInstanceCollectionSet` contains `ServiceInstance`s that are provided or consumed by an ECU
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct ServiceInstanceCollectionSet(
    pub(crate) autosar_data_abstraction::communication::ServiceInstanceCollectionSet,
);

#[pymethods]
impl ServiceInstanceCollectionSet {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::ServiceInstanceCollectionSet::try_from(
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

    /// create a new `ProvidedServiceInstance` in this `ServiceInstanceCollectionSet`
    #[pyo3(signature = (name, service_identifier, instance_identifier, major_version, minor_version, /))]
    #[pyo3(
        text_signature = "(self, name: str, service_identifier: int, instance_identifier: int, major_version: int, minor_version: int, /)"
    )]
    fn create_provided_service_instance(
        &self,
        name: &str,
        service_identifier: u16,
        instance_identifier: u16,
        major_version: u32,
        minor_version: u32,
    ) -> PyResult<ProvidedServiceInstance> {
        match self.0.create_provided_service_instance(
            name,
            service_identifier,
            instance_identifier,
            major_version,
            minor_version,
        ) {
            Ok(value) => Ok(ProvidedServiceInstance(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `ConsumedServiceInstance` in this `ServiceInstanceCollectionSet`
    #[pyo3(signature = (name, service_identifier, instance_identifier, major_version, minor_version, /))]
    #[pyo3(
        text_signature = "(self, name: str, service_identifier: int, instance_identifier: int, major_version: int, minor_version: str, /)"
    )]
    fn create_consumed_service_instance(
        &self,
        name: &str,
        service_identifier: u16,
        instance_identifier: u16,
        major_version: u32,
        minor_version: &str,
    ) -> PyResult<ConsumedServiceInstance> {
        match self.0.create_consumed_service_instance(
            name,
            service_identifier,
            instance_identifier,
            major_version,
            minor_version,
        ) {
            Ok(value) => Ok(ConsumedServiceInstance(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create an iterator over all `ServiceInstances` in this set
    fn service_instances(&self) -> ServiceInstanceIterator {
        ServiceInstanceIterator::new(self.0.service_instances().filter_map(|service_instance| {
            match service_instance {
                autosar_data_abstraction::communication::ServiceInstance::Provided(psi) => {
                    Python::attach(|py| ProvidedServiceInstance(psi).into_py_any(py).ok())
                }
                autosar_data_abstraction::communication::ServiceInstance::Consumed(csi) => {
                    Python::attach(|py| ConsumedServiceInstance(csi).into_py_any(py).ok())
                }
            }
        }))
    }
}

//##################################################################

iterator_wrapper!(
    ServiceInstanceIterator,
    Py<PyAny>,
    "Union[ProvidedServiceInstance, ConsumedServiceInstance]"
);

//##################################################################

/// A `ProvidedServiceInstance` is a service that is provided by an ECU
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct ProvidedServiceInstance(
    pub(crate) autosar_data_abstraction::communication::ProvidedServiceInstance,
);

#[pymethods]
impl ProvidedServiceInstance {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::ProvidedServiceInstance::try_from(
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

    /// set the service identifier of this `ProvidedServiceInstance`
    #[setter]
    fn set_service_identifier(&self, identifier: u16) -> PyResult<()> {
        self.0
            .set_service_identifier(identifier)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the service identifier of this `ProvidedServiceInstance`
    #[getter]
    fn service_identifier(&self) -> Option<u16> {
        self.0.service_identifier()
    }

    /// set the instance identifier of this `ProvidedServiceInstance`
    #[setter]
    fn set_instance_identifier(&self, identifier: u16) -> PyResult<()> {
        self.0
            .set_instance_identifier(identifier)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the instance identifier of this `ProvidedServiceInstance`
    #[getter]
    fn instance_identifier(&self) -> Option<u16> {
        self.0.instance_identifier()
    }

    /// set the major version of this `ProvidedServiceInstance`
    #[setter]
    fn set_major_version(&self, version: u32) -> PyResult<()> {
        self.0
            .set_major_version(version)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the major version of this `ProvidedServiceInstance`
    #[getter]
    fn major_version(&self) -> Option<u32> {
        self.0.major_version()
    }

    /// set the minor version of this `ProvidedServiceInstance`
    #[setter]
    fn set_minor_version(&self, version: u32) -> PyResult<()> {
        self.0
            .set_minor_version(version)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the minor version of this `ProvidedServiceInstance`
    #[getter]
    fn minor_version(&self) -> Option<u32> {
        self.0.minor_version()
    }

    /// create a new `EventHandler` in this `ProvidedServiceInstance`
    #[pyo3(signature = (name, event_group_identifier, /))]
    #[pyo3(text_signature = "(self, name: str, event_group_identifier: int, /)")]
    fn create_event_handler(
        &self,
        name: &str,
        event_group_identifier: u32,
    ) -> PyResult<EventHandler> {
        match self.0.create_event_handler(name, event_group_identifier) {
            Ok(value) => Ok(EventHandler(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// get the `EventHandler`s in this `ProvidedServiceInstance`
    fn event_handlers(&self) -> EventHandlerIterator {
        EventHandlerIterator::new(self.0.event_handlers().map(EventHandler))
    }

    /// set a local unicast address for this `ProvidedServiceInstance`
    ///
    /// The PSI may use two local unicast addresses, one each for UDP and TCP.
    /// The unicast address is used to assign the service to a specific ECU, and may not be empty.
    #[pyo3(signature = (address, /))]
    #[pyo3(text_signature = "(self, address: SocketAddress, /)")]
    fn set_local_unicast_address(&self, address: &SocketAddress) -> PyResult<()> {
        self.0
            .set_local_unicast_address(&address.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// iterate over the local unicast addresses
    fn local_unicast_addresses(&self) -> LocalUnicastAddressIterator {
        LocalUnicastAddressIterator::new(
            self.0
                .local_unicast_addresses()
                .map(LocalUnicastAddress::from),
        )
    }

    /// set the SD server instance configuration for this `ProvidedServiceInstance`
    #[setter]
    fn set_sd_server_instance_config(
        &self,
        config: &SomeipSdServerServiceInstanceConfig,
    ) -> PyResult<()> {
        self.0
            .set_sd_server_instance_config(&config.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the SD server instance configuration for this `ProvidedServiceInstance`
    #[getter]
    fn sd_server_instance_config(&self) -> Option<SomeipSdServerServiceInstanceConfig> {
        self.0
            .sd_server_instance_config()
            .map(SomeipSdServerServiceInstanceConfig)
    }
}

//##################################################################

/// An `EventHandler` describes the handling of a single event in a `ProvidedServiceInstance`
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct EventHandler(pub(crate) autosar_data_abstraction::communication::EventHandler);

#[pymethods]
impl EventHandler {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::EventHandler::try_from(element.0.clone()) {
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

    /// set the event group identifier of this `EventHandler`
    #[setter]
    fn set_event_group_identifier(&self, identifier: u32) -> PyResult<()> {
        self.0
            .set_event_group_identifier(identifier)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the event group identifier of this `EventHandler`
    #[getter]
    fn event_group_identifier(&self) -> Option<u32> {
        self.0.event_group_identifier()
    }

    /// create a new `PduActivationRoutingGroup` in this `EventHandler`
    #[pyo3(signature = (name, event_group_control_type, /))]
    #[pyo3(
        text_signature = "(self, name: str, event_group_control_type: EventGroupControlType, /)"
    )]
    fn create_pdu_activation_routing_group(
        &self,
        name: &str,
        event_group_control_type: EventGroupControlType,
    ) -> PyResult<PduActivationRoutingGroup> {
        match self
            .0
            .create_pdu_activation_routing_group(name, event_group_control_type.into())
        {
            Ok(value) => Ok(PduActivationRoutingGroup(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// get the `PduActivationRoutingGroup`s in this `EventHandler`
    fn pdu_activation_routing_groups(&self) -> PduActivationRoutingGroupIterator {
        PduActivationRoutingGroupIterator::new(
            self.0
                .pdu_activation_routing_groups()
                .map(PduActivationRoutingGroup),
        )
    }

    /// set the SD server event group timing configuration for this `EventHandler`
    #[setter]
    fn set_sd_server_event_group_timing_config(
        &self,
        config: &SomeipSdServerEventGroupTimingConfig,
    ) -> PyResult<()> {
        self.0
            .set_sd_server_event_group_timing_config(&config.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the SD server event group timing configuration for this `EventHandler`
    #[getter]
    fn sd_server_event_group_timing_config(&self) -> Option<SomeipSdServerEventGroupTimingConfig> {
        self.0
            .sd_server_event_group_timing_config()
            .map(SomeipSdServerEventGroupTimingConfig)
    }
}

//##################################################################

iterator_wrapper!(EventHandlerIterator, EventHandler);

//##################################################################

/// A `ConsumedServiceInstance` is a service that is consumed by an ECU
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct ConsumedServiceInstance(
    pub(crate) autosar_data_abstraction::communication::ConsumedServiceInstance,
);

#[pymethods]
impl ConsumedServiceInstance {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::ConsumedServiceInstance::try_from(
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

    /// set the service identifier of this `ConsumedServiceInstance`
    #[setter]
    fn set_service_identifier(&self, identifier: u16) -> PyResult<()> {
        self.0
            .set_service_identifier(identifier)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the service identifier of this `ConsumedServiceInstance`
    #[getter]
    fn service_identifier(&self) -> Option<u16> {
        self.0.service_identifier()
    }

    /// set the instance identifier of this `ConsumedServiceInstance`
    #[setter]
    fn set_instance_identifier(&self, identifier: u16) -> PyResult<()> {
        self.0
            .set_instance_identifier(identifier)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the instance identifier of this `ConsumedServiceInstance`
    #[getter]
    fn instance_identifier(&self) -> Option<u16> {
        self.0.instance_identifier()
    }

    /// set the major version of this `ConsumedServiceInstance`
    #[setter]
    fn set_major_version(&self, version: u32) -> PyResult<()> {
        self.0
            .set_major_version(version)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the major version of this `ConsumedServiceInstance`
    #[getter]
    fn major_version(&self) -> Option<u32> {
        self.0.major_version()
    }

    /// set the minor version of this `ConsumedServiceInstance`
    ///
    /// The minor version can be a number or the String "ANY".
    #[setter]
    fn set_minor_version(&self, any: &Bound<'_, PyAny>) -> PyResult<()> {
        let version = if let Ok(version_numeric) = any.extract::<u32>() {
            version_numeric.to_string()
        } else if let Ok(version) = any.extract::<String>() {
            version
        } else {
            return Err(PyTypeError::new_err(format!(
                "'{}' cannot be converted to 'str' or 'int'",
                any.get_type()
            )));
        };
        self.0
            .set_minor_version(&version)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the minor version of this `ConsumedServiceInstance`
    ///
    /// The minor version can be a number or the String "ANY".
    #[getter]
    fn minor_version(&self) -> Option<String> {
        self.0.minor_version()
    }

    /// create a new `ConsumedEventGrup` in this `ConsumedServiceInstance`
    #[pyo3(signature = (name, event_group_identifier, /))]
    #[pyo3(text_signature = "(self, name: str, event_group_identifier: int, /)")]
    fn create_consumed_event_group(
        &self,
        name: &str,
        event_group_identifier: u32,
    ) -> PyResult<ConsumedEventGroup> {
        match self
            .0
            .create_consumed_event_group(name, event_group_identifier)
        {
            Ok(value) => Ok(ConsumedEventGroup(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// get the `ConsumedEventGroup`s in this `ConsumedServiceInstance`
    fn consumed_event_groups(&self) -> ConsumedEventGroupIterator {
        ConsumedEventGroupIterator::new(self.0.consumed_event_groups().map(ConsumedEventGroup))
    }

    /// set a local unicast address for this `ConsumedServiceInstance`
    ///
    /// The CSI may use two local unicast addresses, one each for UDP and TCP.
    /// If the consumed service instance does not specify a local unicast address
    /// because it only receives multicast messages, then the `ConsumedEventGroup`
    /// must have an eventMulticastAddress.
    #[pyo3(signature = (address, /))]
    #[pyo3(text_signature = "(self, address: SocketAddress, /)")]
    fn set_local_unicast_address(&self, address: &SocketAddress) -> PyResult<()> {
        self.0
            .set_local_unicast_address(&address.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// iterate over the local unicast addresses
    fn local_unicast_addresses(&self) -> LocalUnicastAddressIterator {
        LocalUnicastAddressIterator::new(
            self.0
                .local_unicast_addresses()
                .map(LocalUnicastAddress::from),
        )
    }

    /// set the SD client instance configuration for this `ConsumedServiceInstance`
    #[setter]
    fn set_sd_client_instance_config(
        &self,
        config: &SomeipSdClientServiceInstanceConfig,
    ) -> PyResult<()> {
        self.0
            .set_sd_client_instance_config(&config.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the SD client instance configuration for this `ConsumedServiceInstance`
    #[getter]
    fn sd_client_instance_config(&self) -> Option<SomeipSdClientServiceInstanceConfig> {
        self.0
            .sd_client_instance_config()
            .map(SomeipSdClientServiceInstanceConfig)
    }
}

//##################################################################

/// A `ConsumedEventGroup` is a group of events in a `ConsumedServiceInstance` that are consumed by an ECU
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct ConsumedEventGroup(
    pub(crate) autosar_data_abstraction::communication::ConsumedEventGroup,
);

#[pymethods]
impl ConsumedEventGroup {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::ConsumedEventGroup::try_from(
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

    /// set the event group identifier of this `ConsumedEventGroup`
    #[setter]
    fn set_event_group_identifier(&self, identifier: u32) -> PyResult<()> {
        self.0
            .set_event_group_identifier(identifier)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the event group identifier of this `ConsumedEventGroup`
    #[getter]
    fn event_group_identifier(&self) -> Option<u32> {
        self.0.event_group_identifier()
    }

    /// create a new `PduActivationRoutingGroup` in this `ConsumedEventGroup`
    #[pyo3(signature = (name, event_group_control_type, /))]
    #[pyo3(
        text_signature = "(self, name: str, event_group_control_type: EventGroupControlType, /)"
    )]
    fn create_pdu_activation_routing_group(
        &self,
        name: &str,
        event_group_control_type: EventGroupControlType,
    ) -> PyResult<PduActivationRoutingGroup> {
        match self
            .0
            .create_pdu_activation_routing_group(name, event_group_control_type.into())
        {
            Ok(value) => Ok(PduActivationRoutingGroup(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over the `PduActivationRoutingGroup`s in this `ConsumedEventGroup`
    fn pdu_activation_routing_groups(&self) -> PduActivationRoutingGroupIterator {
        PduActivationRoutingGroupIterator::new(
            self.0
                .pdu_activation_routing_groups()
                .map(PduActivationRoutingGroup),
        )
    }

    /// add an event multicast address to this `ConsumedEventGroup`
    #[pyo3(signature = (address, /))]
    #[pyo3(text_signature = "(self, address: SocketAddress, /)")]
    fn add_event_multicast_address(&self, address: &SocketAddress) -> PyResult<()> {
        self.0
            .add_event_multicast_address(&address.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the event multicast addresses
    fn event_multicast_addresses(&self) -> SocketAddressIterator {
        SocketAddressIterator::new(self.0.event_multicast_addresses().map(SocketAddress))
    }

    /// set the SD client timer configuration for this `ConsumedEventGroup`
    #[setter]
    fn set_sd_client_timer_config(
        &self,
        config: &SomeipSdClientEventGroupTimingConfig,
    ) -> PyResult<()> {
        self.0
            .set_sd_client_timer_config(&config.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the SD client timer configuration for this `ConsumedEventGroup`
    #[getter]
    fn sd_client_timer_config(&self) -> Option<SomeipSdClientEventGroupTimingConfig> {
        self.0
            .sd_client_timer_config()
            .map(SomeipSdClientEventGroupTimingConfig)
    }
}

//##################################################################

iterator_wrapper!(ConsumedEventGroupIterator, ConsumedEventGroup);

//##################################################################

/// A `LocalUnicastAddress` is a local address (TCP or UDP) that can be used for a `ProvidedServiceInstance` or `ConsumedServiceInstance`
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication",
    get_all
)]
#[derive(Clone, PartialEq)]
pub(crate) enum LocalUnicastAddress {
    /// A UDP address
    #[pyo3(constructor=(address, /))]
    Udp { address: SocketAddress },
    /// A TCP address
    #[pyo3(constructor=(address, /))]
    Tcp { address: SocketAddress },
}

impl From<autosar_data_abstraction::communication::LocalUnicastAddress> for LocalUnicastAddress {
    fn from(value: autosar_data_abstraction::communication::LocalUnicastAddress) -> Self {
        match value {
            autosar_data_abstraction::communication::LocalUnicastAddress::Udp(address) => {
                LocalUnicastAddress::Udp {
                    address: SocketAddress(address),
                }
            }
            autosar_data_abstraction::communication::LocalUnicastAddress::Tcp(address) => {
                LocalUnicastAddress::Tcp {
                    address: SocketAddress(address),
                }
            }
        }
    }
}

#[pymethods]
impl LocalUnicastAddress {
    fn __repr__(&self) -> String {
        match self {
            LocalUnicastAddress::Udp { address } => {
                format!("LocalUnicastAddress.Udp({:#?})", address.0)
            }
            LocalUnicastAddress::Tcp { address } => {
                format!("LocalUnicastAddress.Tcp({:#?})", address.0)
            }
        }
    }
}

//##################################################################

iterator_wrapper!(LocalUnicastAddressIterator, LocalUnicastAddress);

//##################################################################

/// A group of Pdus that can be activated or deactivated for transmission over a socket connection.
/// It is used by `EventHandler`s in `ProvidedServiceInstance`s and `ConsumedServiceInstance`s.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct PduActivationRoutingGroup(
    pub(crate) autosar_data_abstraction::communication::PduActivationRoutingGroup,
);

#[pymethods]
impl PduActivationRoutingGroup {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::PduActivationRoutingGroup::try_from(
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

    /// set the event group control type of this `PduActivationRoutingGroup`
    #[setter]
    fn set_event_group_control_type(&self, control_type: EventGroupControlType) -> PyResult<()> {
        self.0
            .set_event_group_control_type(control_type.into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the event group control type of this `PduActivationRoutingGroup`
    #[getter]
    fn event_group_control_type(&self) -> Option<EventGroupControlType> {
        self.0
            .event_group_control_type()
            .map(EventGroupControlType::from)
    }

    /// add a reference to a `SoConIPduIdentifier` for UDP communication to this `PduActivationRoutingGroup`
    #[pyo3(signature = (ipdu_identifier, /))]
    #[pyo3(text_signature = "(self, ipdu_identifier: SoConIPduIdentifier, /)")]
    fn add_ipdu_identifier_udp(&self, ipdu_identifier: &SoConIPduIdentifier) -> PyResult<()> {
        self.0
            .add_ipdu_identifier_udp(&ipdu_identifier.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get all `SoConIPduIdentifier`s for UDP communication in this `PduActivationRoutingGroup`
    fn ipdu_identifiers_udp(&self) -> SoConIPduIdentifierIterator {
        SoConIPduIdentifierIterator::new(self.0.ipdu_identifiers_udp().map(SoConIPduIdentifier))
    }

    /// add a reference to a `SoConIPduIdentifier` for TCP communication to this `PduActivationRoutingGroup`
    #[pyo3(signature = (ipdu_identifier, /))]
    #[pyo3(text_signature = "(self, ipdu_identifier: SoConIPduIdentifier, /)")]
    fn add_ipdu_identifier_tcp(&self, ipdu_identifier: &SoConIPduIdentifier) -> PyResult<()> {
        self.0
            .add_ipdu_identifier_tcp(&ipdu_identifier.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get all `SoConIPduIdentifier`s for TCP communication in this `PduActivationRoutingGroup`
    fn ipdu_identifiers_tcp(&self) -> SoConIPduIdentifierIterator {
        SoConIPduIdentifierIterator::new(self.0.ipdu_identifiers_tcp().map(SoConIPduIdentifier))
    }
}

//##################################################################

iterator_wrapper!(PduActivationRoutingGroupIterator, PduActivationRoutingGroup);

//##################################################################

/// A `SomeipSdServerServiceInstanceConfig` is a configuration for a `ProvidedServiceInstance`
///
/// This configuration is a named element that is created separately and can be used by multiple `ProvidedServiceInstance`s.
///
/// Use [`ArPackage::create_someip_sd_server_service_instance_config`] to create a new `SomeipSdServerServiceInstanceConfig`.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct SomeipSdServerServiceInstanceConfig(
    pub(crate) autosar_data_abstraction::communication::SomeipSdServerServiceInstanceConfig,
);

#[pymethods]
impl SomeipSdServerServiceInstanceConfig {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::SomeipSdServerServiceInstanceConfig::try_from(
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

    /// set the service offer time to live of this `SomeipSdServerServiceInstanceConfig`
    #[setter]
    fn set_service_offer_time_to_live(&self, ttl: u32) -> PyResult<()> {
        self.0
            .set_service_offer_time_to_live(ttl)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the service offer time to live of this `SomeipSdServerServiceInstanceConfig`
    #[getter]
    fn service_offer_time_to_live(&self) -> Option<u32> {
        self.0.service_offer_time_to_live()
    }

    /// set the offer cyclic delay of this `SomeipSdServerServiceInstanceConfig`
    #[setter]
    fn set_offer_cyclic_delay(&self, delay: f64) -> PyResult<()> {
        self.0
            .set_offer_cyclic_delay(delay)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the offer cyclic delay of this `SomeipSdServerServiceInstanceConfig`
    #[getter]
    fn offer_cyclic_delay(&self) -> Option<f64> {
        self.0.offer_cyclic_delay()
    }

    /// set the priority of this `SomeipSdServerServiceInstanceConfig`
    ///
    /// Available since R21-11 (`AUTOSAR_00050`)
    #[setter]
    fn set_priority(&self, priority: u8) -> PyResult<()> {
        self.0
            .set_priority(priority)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the priority of this `SomeipSdServerServiceInstanceConfig`
    #[getter]
    fn priority(&self) -> Option<u8> {
        self.0.priority()
    }

    /// set the initial offer behavior of this `SomeipSdServerServiceInstanceConfig`
    fn set_initial_offer_behavior(
        &self,
        initial_offer_behavior: &InitialSdDelayConfig,
    ) -> PyResult<()> {
        self.0
            .set_initial_offer_behavior(&initial_offer_behavior.into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the initial offer behavior of this `SomeipSdServerServiceInstanceConfig`
    fn initial_offer_behavior(&self) -> Option<InitialSdDelayConfig> {
        self.0
            .initial_offer_behavior()
            .map(InitialSdDelayConfig::from)
    }

    /// set the request response delay of this `SomeipSdServerServiceInstanceConfig`
    fn set_request_response_delay(
        &self,
        request_response_delay: &RequestResponseDelay,
    ) -> PyResult<()> {
        self.0
            .set_request_response_delay(&request_response_delay.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the request response delay of this `SomeipSdServerServiceInstanceConfig`
    fn request_response_delay(&self) -> Option<RequestResponseDelay> {
        self.0.request_response_delay().map(RequestResponseDelay)
    }
}

//##################################################################

/// A `SomeipSdServerEventGroupTimingConfig` contains the configuration for the timing of an `EventHandler`
///
/// This configuration is a named element that is created separately and can be used by multiple `EventHandler`s.
///
/// Use [`ArPackage::create_someip_sd_server_event_group_timing_config`] to create a new `SomeipSdServerEventGroupTimingConfig`.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct SomeipSdServerEventGroupTimingConfig(
    pub(crate) autosar_data_abstraction::communication::SomeipSdServerEventGroupTimingConfig,
);

#[pymethods]
impl SomeipSdServerEventGroupTimingConfig {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::SomeipSdServerEventGroupTimingConfig::try_from(
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

    /// set the request response delay of this `SomeipSdServerEventGroupTimingConfig`
    fn set_request_response_delay(
        &self,
        request_response_delay: &RequestResponseDelay,
    ) -> PyResult<()> {
        self.0
            .set_request_response_delay(&request_response_delay.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the request response delay of this `SomeipSdServerEventGroupTimingConfig`
    fn request_response_delay(&self) -> Option<RequestResponseDelay> {
        self.0.request_response_delay().map(RequestResponseDelay)
    }
}

//##################################################################

/// A `SomeipSdClientServiceInstanceConfig` is a configuration for a `ConsumedServiceInstance`
///
/// This configuration is a named element that is created separately and can be used by multiple `ConsumedServiceInstance`s.
///
/// Use [`ArPackage::create_someip_sd_client_service_instance_config`] to create a new `SomeipSdClientServiceInstanceConfig`.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct SomeipSdClientServiceInstanceConfig(
    pub(crate) autosar_data_abstraction::communication::SomeipSdClientServiceInstanceConfig,
);

#[pymethods]
impl SomeipSdClientServiceInstanceConfig {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::SomeipSdClientServiceInstanceConfig::try_from(
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

    /// set the initial find behavior of this `SomeipSdClientServiceInstanceConfig`
    #[pyo3(signature = (initial_find_behavior, /))]
    #[pyo3(text_signature = "(self, initial_find_behavior: InitialSdDelayConfig, /)")]
    fn set_initial_find_behavior(
        &self,
        initial_find_behavior: &InitialSdDelayConfig,
    ) -> PyResult<()> {
        self.0
            .set_initial_find_behavior(&initial_find_behavior.into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the initial find behavior of this `SomeipSdClientServiceInstanceConfig`
    fn initial_find_behavior(&self) -> Option<InitialSdDelayConfig> {
        self.0
            .initial_find_behavior()
            .map(InitialSdDelayConfig::from)
    }

    /// set the priority of this `SomeipSdClientServiceInstanceConfig`
    ///
    /// Available since R21-11 (`AUTOSAR_00050`)
    #[setter]
    fn set_priority(&self, priority: u8) -> PyResult<()> {
        self.0
            .set_priority(priority)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the priority of this `SomeipSdClientServiceInstanceConfig`
    #[getter]
    fn priority(&self) -> Option<u8> {
        self.0.priority()
    }
}

//##################################################################

/// A `SomeipSdClientEventGroupTimingConfig` contains the configuration for the timing of a `ConsumedEventGroup`
///
/// This configuration is a named element that is created separately and can be used by multiple `ConsumedEventGroup`s.
///
/// Use [`ArPackage::create_someip_sd_client_event_group_timing_config`] to create a new `SomeipSdClientEventGroupTimingConfig`.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct SomeipSdClientEventGroupTimingConfig(
    pub(crate) autosar_data_abstraction::communication::SomeipSdClientEventGroupTimingConfig,
);

#[pymethods]
impl SomeipSdClientEventGroupTimingConfig {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::SomeipSdClientEventGroupTimingConfig::try_from(
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

    /// set the time to live of this `SomeipSdClientEventGroupTimingConfig`
    #[setter]
    fn set_time_to_live(&self, time_to_live: u32) -> PyResult<()> {
        self.0
            .set_time_to_live(time_to_live)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the time to live of this `SomeipSdClientEventGroupTimingConfig`
    #[getter]
    fn time_to_live(&self) -> Option<u32> {
        self.0.time_to_live()
    }

    /// set the request response delay of this `SomeipSdClientEventGroupTimingConfig`
    fn set_request_response_delay(
        &self,
        request_response_delay: &RequestResponseDelay,
    ) -> PyResult<()> {
        self.0
            .set_request_response_delay(&request_response_delay.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the request response delay of this `SomeipSdClientEventGroupTimingConfig`
    fn request_response_delay(&self) -> Option<RequestResponseDelay> {
        self.0.request_response_delay().map(RequestResponseDelay)
    }

    /// set the subscribe eventgroup retry delay of this `SomeipSdClientEventGroupTimingConfig`
    #[setter]
    fn set_subscribe_eventgroup_retry_delay(
        &self,
        subscribe_eventgroup_retry_delay: f64,
    ) -> PyResult<()> {
        self.0
            .set_subscribe_eventgroup_retry_delay(subscribe_eventgroup_retry_delay)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the subscribe eventgroup retry delay of this `SomeipSdClientEventGroupTimingConfig`
    #[getter]
    fn subscribe_eventgroup_retry_delay(&self) -> Option<f64> {
        self.0.subscribe_eventgroup_retry_delay()
    }

    /// set subscribe eventgroup retry max of this `SomeipSdClientEventGroupTimingConfig`
    #[setter]
    fn set_subscribe_eventgroup_retry_max(
        &self,
        subscribe_eventgroup_retry_max: u32,
    ) -> PyResult<()> {
        self.0
            .set_subscribe_eventgroup_retry_max(subscribe_eventgroup_retry_max)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the value of subscribe eventgroup retry max of this `SomeipSdClientEventGroupTimingConfig`
    #[getter]
    fn subscribe_eventgroup_retry_max(&self) -> Option<u32> {
        self.0.subscribe_eventgroup_retry_max()
    }
}

//##################################################################

/// A `RequestResponseDelay` contains the minimum and maximum delay for a request-response communication
#[pyclass(eq, module = "autosar_data._autosar_data._abstraction._communication")]
#[derive(Clone, PartialEq)]
pub(crate) struct RequestResponseDelay(
    pub(crate) autosar_data_abstraction::communication::RequestResponseDelay,
);

#[pymethods]
impl RequestResponseDelay {
    #[pyo3(signature = (*, min_value, max_value))]
    #[pyo3(text_signature = "(*, min_value: float, max_value: float)")]
    #[new]
    fn new(min_value: f64, max_value: f64) -> Self {
        let rrd = autosar_data_abstraction::communication::RequestResponseDelay {
            min_value,
            max_value,
        };
        Self(rrd)
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self.0)
    }

    /// set the minimum value of this `RequestResponseDelay`
    #[setter]
    fn set_min_value(&mut self, min_value: f64) {
        self.0.min_value = min_value;
    }

    /// get the minimum value of this `RequestResponseDelay`
    #[getter]
    fn min_value(&self) -> f64 {
        self.0.min_value
    }

    /// set the maximum value of this `RequestResponseDelay`
    #[setter]
    fn set_max_value(&mut self, max_value: f64) {
        self.0.max_value = max_value;
    }

    /// get the maximum value of this `RequestResponseDelay`
    #[getter]
    fn max_value(&self) -> f64 {
        self.0.max_value
    }
}

//##################################################################

/// A `InitialSdDelayConfig` contains the configuration for the initial delay of an SD client or server
#[pyclass(
    eq,
    get_all,
    set_all,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, PartialEq)]
pub struct InitialSdDelayConfig {
    /// maximum value of the randomized delay in seconds
    pub initial_delay_max_value: f64,
    /// minimum value of the randomized delay in seconds
    pub initial_delay_min_value: f64,
    /// base delay for repetitions in seconds
    pub initial_repetitions_base_delay: Option<f64>,
    /// maximum number of repetitions
    pub initial_repetitions_max: Option<u32>,
}

#[pymethods]
impl InitialSdDelayConfig {
    #[pyo3(signature = (*, initial_delay_max_value, initial_delay_min_value, initial_repetitions_base_delay=None, initial_repetitions_max=None))]
    #[pyo3(
        text_signature = "(*, initial_delay_max_value: float, initial_delay_min_value: float, initial_repetitions_base_delay: Optional[float] = None, initial_repetitions_max: Optional[int] = None)"
    )]
    #[new]
    fn new(
        initial_delay_max_value: f64,
        initial_delay_min_value: f64,
        initial_repetitions_base_delay: Option<f64>,
        initial_repetitions_max: Option<u32>,
    ) -> Self {
        Self {
            initial_delay_max_value,
            initial_delay_min_value,
            initial_repetitions_base_delay,
            initial_repetitions_max,
        }
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}

impl From<&InitialSdDelayConfig> for autosar_data_abstraction::communication::InitialSdDelayConfig {
    fn from(config: &InitialSdDelayConfig) -> Self {
        autosar_data_abstraction::communication::InitialSdDelayConfig {
            initial_delay_max_value: config.initial_delay_max_value,
            initial_delay_min_value: config.initial_delay_min_value,
            initial_repetitions_base_delay: config.initial_repetitions_base_delay,
            initial_repetitions_max: config.initial_repetitions_max,
        }
    }
}

impl From<autosar_data_abstraction::communication::InitialSdDelayConfig> for InitialSdDelayConfig {
    fn from(config: autosar_data_abstraction::communication::InitialSdDelayConfig) -> Self {
        Self {
            initial_delay_max_value: config.initial_delay_max_value,
            initial_delay_min_value: config.initial_delay_min_value,
            initial_repetitions_base_delay: config.initial_repetitions_base_delay,
            initial_repetitions_max: config.initial_repetitions_max,
        }
    }
}

//##################################################################

/// A `SomipTpConfig` contains the configuration of individual `SomeIp` TP connections
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct SomeipTpConfig(
    pub(crate) autosar_data_abstraction::communication::SomeipTpConfig,
);

#[pymethods]
impl SomeipTpConfig {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::SomeipTpConfig::try_from(element.0.clone()) {
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

    /// get the communication cluster of this `SomeipTpConfig`
    #[getter]
    fn cluster(&self) -> Option<Py<PyAny>> {
        let cluster = self.0.cluster()?;
        Python::attach(|py| match cluster {
            autosar_data_abstraction::communication::Cluster::Can(can_cluster) => {
                CanCluster(can_cluster).into_py_any(py).ok()
            }
            autosar_data_abstraction::communication::Cluster::Ethernet(ethernet_cluster) => {
                EthernetCluster(ethernet_cluster).into_py_any(py).ok()
            }
            autosar_data_abstraction::communication::Cluster::FlexRay(flexray_cluster) => {
                FlexrayCluster(flexray_cluster).into_py_any(py).ok()
            }
            autosar_data_abstraction::communication::Cluster::Lin(lin_cluster) => {
                LinCluster(lin_cluster).into_py_any(py).ok()
            }
            _ => None,
        })
    }

    /// create a new `SomeipTpChannel` in this `SomeipTpConfig`
    ///
    /// version >= `AUTOSAR_00046`
    #[pyo3(signature = (name, /))]
    #[pyo3(text_signature = "(self, name: str, /)")]
    fn create_someip_tp_channel(&self, name: &str) -> PyResult<SomeipTpChannel> {
        match self.0.create_someip_tp_channel(name) {
            Ok(value) => Ok(SomeipTpChannel(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all `SomeipTpChannel`s in this `SomeipTpConfig`
    fn someip_tp_channels(&self) -> SomeipTpChannelIterator {
        SomeipTpChannelIterator::new(self.0.someip_tp_channels().map(SomeipTpChannel))
    }

    /// create a new SomeIp TP connection in this `SomeipTpConfig`
    #[pyo3(signature = (tp_sdu, transport_pdu_triggering, /, *, tp_channel=None))]
    #[pyo3(
        text_signature = "(self, tp_sdu: ISignalIPdu, transport_pdu_triggering: PduTriggering, /, *, tp_channel: Optional[SomeIpTpChannel] = None)"
    )]
    fn create_someip_tp_connection(
        &self,
        tp_sdu: &ISignalIPdu,
        transport_pdu_triggering: &PduTriggering,
        tp_channel: Option<SomeipTpChannel>,
    ) -> PyResult<SomeipTpConnection> {
        match self.0.create_someip_tp_connection(
            &tp_sdu.0,
            &transport_pdu_triggering.0,
            tp_channel.map(|c| c.0),
        ) {
            Ok(value) => Ok(SomeipTpConnection(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// get all `SomeipTpConnection`s in this `SomeipTpConfig`
    fn someip_tp_connections(&self) -> SomeipTpConnectionIterator {
        SomeipTpConnectionIterator::new(self.0.someip_tp_connections().map(SomeipTpConnection))
    }
}

//##################################################################

/// A `SomeipTpConnection` contains the configuration of a single `SomeIp` TP connection
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct SomeipTpConnection(
    pub(crate) autosar_data_abstraction::communication::SomeipTpConnection,
);

#[pymethods]
impl SomeipTpConnection {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::SomeipTpConnection::try_from(
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

    /// get the `SomeipTpConfig` that contains this `SomeipTpConnection`
    #[getter]
    fn someip_tp_config(&self) -> PyResult<SomeipTpConfig> {
        match self.0.someip_tp_config() {
            Ok(value) => Ok(SomeipTpConfig(value)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// set the `PduTriggering` for the transport PDU of this `SomeipTpConnection`
    #[setter]
    fn set_transport_pdu_triggering(
        &self,
        transport_pdu_triggering: &PduTriggering,
    ) -> PyResult<()> {
        self.0
            .set_transport_pdu_triggering(&transport_pdu_triggering.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the `PduTriggering` for the transport PDU of this `SomeipTpConnection`
    #[getter]
    fn transport_pdu_triggering(&self) -> Option<PduTriggering> {
        self.0.transport_pdu_triggering().map(PduTriggering)
    }

    /// set the `TpSdu` of this `SomeipTpConnection`
    #[setter]
    fn set_tp_sdu(&self, tp_sdu: &ISignalIPdu) -> PyResult<()> {
        self.0
            .set_tp_sdu(&tp_sdu.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the `TpSdu` of this `SomeipTpConnection`
    #[getter]
    fn tp_sdu(&self) -> Option<ISignalIPdu> {
        self.0.tp_sdu().map(ISignalIPdu)
    }

    /// set the `TpChannel` of this `SomeipTpConnection`
    #[setter]
    fn set_tp_channel(&self, tp_channel: Option<SomeipTpChannel>) -> PyResult<()> {
        self.0
            .set_tp_channel(tp_channel.map(|c| c.0))
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the `TpChannel` of this `SomeipTpConnection`
    #[getter]
    fn tp_channel(&self) -> Option<SomeipTpChannel> {
        self.0.tp_channel().map(SomeipTpChannel)
    }
}

//##################################################################

iterator_wrapper!(SomeipTpConnectionIterator, SomeipTpConnection);

//##################################################################

/// General settings for a `SomeIp` TP channel
///
/// version >= `AUTOSAR_00046`
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct SomeipTpChannel(
    pub(crate) autosar_data_abstraction::communication::SomeipTpChannel,
);

#[pymethods]
impl SomeipTpChannel {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::SomeipTpChannel::try_from(element.0.clone())
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

    /// set the rxTimeoutTime for the `SomeIpTpChannel`
    #[setter]
    fn set_rx_timeout_time(&self, rx_timeout_time: f64) -> PyResult<()> {
        self.0
            .set_rx_timeout_time(rx_timeout_time)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the rxTimeoutTime for the `SomeIpTpChannel`
    #[getter]
    fn rx_timeout_time(&self) -> Option<f64> {
        self.0.rx_timeout_time()
    }

    /// set the separationTime for the `SomeIpTpChannel`
    #[setter]
    fn set_separation_time(&self, separation_time: f64) -> PyResult<()> {
        self.0
            .set_separation_time(separation_time)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the separationTime for the `SomeIpTpChannel`
    #[getter]
    fn separation_time(&self) -> Option<f64> {
        self.0.separation_time()
    }
}

//##################################################################

iterator_wrapper!(SomeipTpChannelIterator, SomeipTpChannel);
