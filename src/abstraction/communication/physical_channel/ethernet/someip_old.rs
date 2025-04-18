use crate::{
    Element,
    abstraction::{
        AutosarAbstractionError, abstraction_err_to_pyerr,
        communication::{SoAdRoutingGroup, SoAdRoutingGroupIterator, SocketAddress},
    },
    iterator_wrapper,
};
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};
use pyo3::prelude::*;

/// A `ProvidedServiceInstanceV1` is a SD service instance that is provided by this ECU.
///
/// This is the old V1 version of the service definition.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct ProvidedServiceInstanceV1(
    pub(crate) autosar_data_abstraction::communication::ProvidedServiceInstanceV1,
);

#[pymethods]
impl ProvidedServiceInstanceV1 {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::ProvidedServiceInstanceV1::try_from(
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
    fn set_service_identifier(&self, service_identifier: u32) -> PyResult<()> {
        self.0
            .set_service_identifier(service_identifier)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the service identifier of this `ProvidedServiceInstance`
    #[getter]
    fn service_identifier(&self) -> Option<u32> {
        self.0.service_identifier()
    }

    /// set the instance identifier of this `ProvidedServiceInstance`
    #[setter]
    fn set_instance_identifier(&self, instance_identifier: u32) -> PyResult<()> {
        self.0
            .set_instance_identifier(instance_identifier)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the instance identifier of this `ProvidedServiceInstance`
    #[getter]
    fn instance_identifier(&self) -> Option<u32> {
        self.0.instance_identifier()
    }

    /// create a new `EventHandlerV1` in this `ProvidedServiceInstance`
    #[pyo3(signature = (name, /))]
    #[pyo3(text_signature = "(self, name: str, /)")]
    fn create_event_handler(&self, name: &str) -> PyResult<EventHandlerV1> {
        match self.0.create_event_handler(name) {
            Ok(value) => Ok(EventHandlerV1(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// get the `EventHandlerV1`s in this `ProvidedServiceInstance`
    fn event_handlers(&self) -> EventHandlerV1Iterator {
        EventHandlerV1Iterator::new(self.0.event_handlers().map(EventHandlerV1))
    }

    /// set the SD server configuration for this `ProvidedServiceInstance`
    #[pyo3(signature = (sd_server_config, /))]
    #[pyo3(text_signature = "(self, sd_server_config: SdConfig, /)")]
    fn set_sd_server_config(&self, sd_server_config: &SdConfig) -> PyResult<()> {
        self.0
            .set_sd_server_config(&sd_server_config.clone().into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the SD server configuration for this `ProvidedServiceInstance`
    fn sd_server_config(&self) -> Option<SdConfig> {
        self.0.sd_server_config().map(SdConfig::from)
    }
}

//##################################################################

iterator_wrapper!(ProvidedServiceInstanceV1Iterator, ProvidedServiceInstanceV1);

//##################################################################

/// An `EventHandlerV1` is a SD event handler that is used to receive events from other ECUs.
///
/// This is the old V1 version of the service definition.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct EventHandlerV1(
    pub(crate) autosar_data_abstraction::communication::EventHandlerV1,
);

#[pymethods]
impl EventHandlerV1 {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::EventHandlerV1::try_from(element.0.clone()) {
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

    /// add a reference to a `ConsumedEventGroupV1` to this `EventHandlerV1`
    #[pyo3(signature = (consumed_event_group, /))]
    #[pyo3(text_signature = "(self, consumed_event_group: ConsumedEventGroupV1, /)")]
    fn add_consumed_event_group(
        &self,
        consumed_event_group: &ConsumedEventGroupV1,
    ) -> PyResult<()> {
        self.0
            .add_consumed_event_group(&consumed_event_group.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// add a reference to a `SoAdRoutingGroup` to this `EventHandlerV1`
    #[pyo3(signature = (routing_group, /))]
    #[pyo3(text_signature = "(self, routing_group: SoAdRoutingGroup, /)")]
    fn add_routing_group(&self, routing_group: &SoAdRoutingGroup) -> PyResult<()> {
        self.0
            .add_routing_group(&routing_group.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the routing groups referenced by this `EventHandlerV1`
    fn routing_groups(&self) -> SoAdRoutingGroupIterator {
        SoAdRoutingGroupIterator::new(self.0.routing_groups().map(SoAdRoutingGroup))
    }

    /// set the SD server configuration for this `EventHandlerV1`
    #[pyo3(signature = (sd_event_config, /))]
    #[pyo3(text_signature = "(self, sd_event_config: SdEventConfig, /)")]
    fn set_sd_server_config(&self, sd_event_config: &SdEventConfig) -> PyResult<()> {
        self.0
            .set_sd_server_config(&sd_event_config.into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the SD server configuration for this `EventHandlerV1`
    fn sd_server_config(&self) -> Option<SdEventConfig> {
        self.0.sd_server_config().map(SdEventConfig::from)
    }

    /// get the consumed event groups referenced by this `EventHandlerV1`
    fn consumed_event_groups(&self) -> ConsumedEventGroupV1Iterator {
        ConsumedEventGroupV1Iterator::new(self.0.consumed_event_groups().map(ConsumedEventGroupV1))
    }
}

//##################################################################

iterator_wrapper!(EventHandlerV1Iterator, EventHandlerV1);

//##################################################################

/// A `ConsumedServiceInstanceV1` is a SD service instance that is consumed by this ECU.
///
/// This is the old V1 version of the service definition.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct ConsumedServiceInstanceV1(
    pub(crate) autosar_data_abstraction::communication::ConsumedServiceInstanceV1,
);

#[pymethods]
impl ConsumedServiceInstanceV1 {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::ConsumedServiceInstanceV1::try_from(
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

    /// get the `ProvidedServiceInstanceV1` referenced by this `ConsumedServiceInstanceV1`
    #[getter]
    fn provided_service_instance(&self) -> Option<ProvidedServiceInstanceV1> {
        self.0
            .provided_service_instance()
            .map(ProvidedServiceInstanceV1)
    }

    /// create a new `ConsumedEventGrup` in this `ConsumedServiceInstanceV1`
    #[pyo3(signature = (name, event_group_identifier, event_handler, /))]
    #[pyo3(
        text_signature = "(self, name: str, event_group_identifier: int, event_handler: EventHandlerV1, /)"
    )]
    fn create_consumed_event_group(
        &self,
        name: &str,
        event_group_identifier: u32,
        event_handler: &EventHandlerV1,
    ) -> PyResult<ConsumedEventGroupV1> {
        match self
            .0
            .create_consumed_event_group(name, event_group_identifier, &event_handler.0)
        {
            Ok(value) => Ok(ConsumedEventGroupV1(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// get the `ConsumedEventGroup`s in this `ConsumedServiceInstanceV1`
    fn consumed_event_groups(&self) -> ConsumedEventGroupV1Iterator {
        ConsumedEventGroupV1Iterator::new(self.0.consumed_event_groups().map(ConsumedEventGroupV1))
    }

    /// set the SD client configuration for this `ConsumedServiceInstanceV1`
    #[pyo3(signature = (sd_client_config, /))]
    #[pyo3(text_signature = "(self, sd_client_config: SdConfig, /)")]
    fn set_sd_client_config(&self, sd_client_config: &SdConfig) -> PyResult<()> {
        self.0
            .set_sd_client_config(&sd_client_config.clone().into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the SD client configuration for this `ConsumedServiceInstanceV1`
    fn sd_client_config(&self) -> Option<SdConfig> {
        self.0.sd_client_config().map(SdConfig::from)
    }
}

//##################################################################

iterator_wrapper!(ConsumedServiceInstanceV1Iterator, ConsumedServiceInstanceV1);

//##################################################################

/// A `ConsumedEventGroupV1` is a SD event group of a service instance that is consumed by this ECU.
///
/// This is the old V1 version of the service definition.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct ConsumedEventGroupV1(
    pub(crate) autosar_data_abstraction::communication::ConsumedEventGroupV1,
);

#[pymethods]
impl ConsumedEventGroupV1 {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::ConsumedEventGroupV1::try_from(
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

    /// list all `EventHandlerV1`s that reference this `ConsumedEventGroupV1`
    fn event_handlers(&self) -> Vec<EventHandlerV1> {
        self.0
            .event_handlers()
            .into_iter()
            .map(EventHandlerV1)
            .collect()
    }

    /// set the `SocketAddress` that receives events from this `ConsumedEventGroup`
    /// This may be a different `SocketAddress` than the one that is used to send requests.
    #[setter]
    fn set_application_endpoint(&self, socket_address: &SocketAddress) -> PyResult<()> {
        self.0
            .set_application_endpoint(&socket_address.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the Socket that receives events from this `ConsumedEventGroup`
    /// This may be a different Socket than the one that is used to send requests.
    #[getter]
    fn application_endpoint(&self) -> Option<SocketAddress> {
        self.0.application_endpoint().map(SocketAddress)
    }

    /// set the event group identifier of this `ConsumedEventGroup`
    #[setter]
    fn set_event_group_identifier(&self, event_group_identifier: u32) -> PyResult<()> {
        self.0
            .set_event_group_identifier(event_group_identifier)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the event group identifier of this `ConsumedEventGroup`
    #[getter]
    fn event_group_identifier(&self) -> Option<u32> {
        self.0.event_group_identifier()
    }

    /// add a reference to a `SoAdRoutingGroup` to this `ConsumedEventGroup`
    #[pyo3(signature = (routing_group, /))]
    #[pyo3(text_signature = "(self, routing_group: SoAdRoutingGroup, /)")]
    fn add_routing_group(&self, routing_group: &SoAdRoutingGroup) -> PyResult<()> {
        self.0
            .add_routing_group(&routing_group.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the routing groups referenced by this `ConsumedEventGroup`
    fn routing_groups(&self) -> SoAdRoutingGroupIterator {
        SoAdRoutingGroupIterator::new(self.0.routing_groups().map(SoAdRoutingGroup))
    }

    /// set the SD client configuration for this `ConsumedEventGroup`
    #[setter]
    fn set_sd_client_config(&self, sd_client_config: &SdEventConfig) -> PyResult<()> {
        self.0
            .set_sd_client_config(&sd_client_config.into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the SD client configuration for this `ConsumedEventGroup`
    #[getter]
    fn sd_client_config(&self) -> Option<SdEventConfig> {
        self.0.sd_client_config().map(SdEventConfig::from)
    }
}

//##################################################################

iterator_wrapper!(ConsumedEventGroupV1Iterator, ConsumedEventGroupV1);

//##################################################################

/// SD configuration for a service instance
///
/// This struct is used to configure the SD server and client behavior for a service instance.
/// it is used for the old V1 service definitions.
#[pyclass(
    eq,
    get_all,
    set_all,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SdConfig {
    /// The major version of the service
    pub service_major_version: u32,
    /// The minor version of the service
    pub service_minor_version: u32,
    /// The maximum delay for the initial offer
    pub initial_delay_max_value: f64,
    /// The minimum delay for the initial offer
    pub initial_delay_min_value: f64,
    /// The base delay for offer repetitions (if aggregated by `SdServerConfig`) or find repetitions (if aggregated by `SdClientConfig`)
    pub initial_repetitions_base_delay: Option<f64>,
    /// The maximum number of repetitions for the initial offer or find
    pub initial_repetitions_max: u32,
    /// The delay between two offers (if aggregated by `SdServerConfig`) or finds (if aggregated by `SdClientConfig`)
    pub offer_cyclic_delay: Option<f64>,
    /// The maximum delay for a request-response cycle
    pub request_response_delay_max_value: f64,
    /// The minimum delay for a request-response cycle
    pub request_response_delay_min_value: f64,
    /// The time-to-live for the service offer
    pub ttl: u32,
}

impl From<SdConfig> for autosar_data_abstraction::communication::SdConfig {
    fn from(config: SdConfig) -> Self {
        autosar_data_abstraction::communication::SdConfig {
            service_major_version: config.service_major_version,
            service_minor_version: config.service_minor_version,
            initial_delay_max_value: config.initial_delay_max_value,
            initial_delay_min_value: config.initial_delay_min_value,
            initial_repetitions_base_delay: config.initial_repetitions_base_delay,
            initial_repetitions_max: config.initial_repetitions_max,
            offer_cyclic_delay: config.offer_cyclic_delay,
            request_response_delay_max_value: config.request_response_delay_max_value,
            request_response_delay_min_value: config.request_response_delay_min_value,
            ttl: config.ttl,
        }
    }
}

impl From<autosar_data_abstraction::communication::SdConfig> for SdConfig {
    fn from(config: autosar_data_abstraction::communication::SdConfig) -> Self {
        SdConfig {
            service_major_version: config.service_major_version,
            service_minor_version: config.service_minor_version,
            initial_delay_max_value: config.initial_delay_max_value,
            initial_delay_min_value: config.initial_delay_min_value,
            initial_repetitions_base_delay: config.initial_repetitions_base_delay,
            initial_repetitions_max: config.initial_repetitions_max,
            offer_cyclic_delay: config.offer_cyclic_delay,
            request_response_delay_max_value: config.request_response_delay_max_value,
            request_response_delay_min_value: config.request_response_delay_min_value,
            ttl: config.ttl,
        }
    }
}

#[pymethods]
impl SdConfig {
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (*, service_major_version, service_minor_version, initial_delay_max_value, initial_delay_min_value,
        initial_repetitions_base_delay=None, initial_repetitions_max, offer_cyclic_delay=None, request_response_delay_max_value, request_response_delay_min_value, ttl))]
    #[new]
    fn new(
        service_major_version: u32,
        service_minor_version: u32,
        initial_delay_max_value: f64,
        initial_delay_min_value: f64,
        initial_repetitions_base_delay: Option<f64>,
        initial_repetitions_max: u32,
        offer_cyclic_delay: Option<f64>,
        request_response_delay_max_value: f64,
        request_response_delay_min_value: f64,
        ttl: u32,
    ) -> Self {
        Self {
            service_major_version,
            service_minor_version,
            initial_delay_max_value,
            initial_delay_min_value,
            initial_repetitions_base_delay,
            initial_repetitions_max,
            offer_cyclic_delay,
            request_response_delay_max_value,
            request_response_delay_min_value,
            ttl,
        }
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}

//##################################################################

/// Configuration for an SD event handler
#[pyclass(
    eq,
    get_all,
    set_all,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, PartialEq)]
pub struct SdEventConfig {
    /// The maximum delay for a request-response cycle
    pub request_response_delay_max_value: f64,
    /// The minimum delay for a request-response cycle
    pub request_response_delay_min_value: f64,
    /// The time-to-live for the service offer
    pub ttl: u32,
}

impl From<&SdEventConfig> for autosar_data_abstraction::communication::SdEventConfig {
    fn from(config: &SdEventConfig) -> Self {
        autosar_data_abstraction::communication::SdEventConfig {
            request_response_delay_max_value: config.request_response_delay_max_value,
            request_response_delay_min_value: config.request_response_delay_min_value,
            ttl: config.ttl,
        }
    }
}

impl From<autosar_data_abstraction::communication::SdEventConfig> for SdEventConfig {
    fn from(config: autosar_data_abstraction::communication::SdEventConfig) -> Self {
        SdEventConfig {
            request_response_delay_max_value: config.request_response_delay_max_value,
            request_response_delay_min_value: config.request_response_delay_min_value,
            ttl: config.ttl,
        }
    }
}

#[pymethods]
impl SdEventConfig {
    #[pyo3(signature = (*, request_response_delay_max_value, request_response_delay_min_value, ttl))]
    #[pyo3(
        text_signature = "(*, request_response_delay_max_value: float, request_response_delay_min_value: float, ttl: int)"
    )]
    #[new]
    fn new(
        request_response_delay_max_value: f64,
        request_response_delay_min_value: f64,
        ttl: u32,
    ) -> Self {
        Self {
            request_response_delay_max_value,
            request_response_delay_min_value,
            ttl,
        }
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}
