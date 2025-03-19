use crate::{
    Element,
    abstraction::{
        AutosarAbstractionError, abstraction_err_to_pyerr,
        communication::{
            PduCollectionTrigger, PduTriggering, PduTriggeringIterator, pyany_to_ipdu,
            pyany_to_physical_channel,
        },
    },
};
use autosar_data_abstraction::{
    self, AbstractionElement, IdentifiableAbstractionElement,
    communication::{AbstractIpdu, AbstractPdu},
};
use pyo3::prelude::*;

//##################################################################

/// Several `IPdus` can be collected in one `ContainerIPdu` based on the headerType
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct ContainerIPdu(pub(crate) autosar_data_abstraction::communication::ContainerIPdu);

#[pymethods]
impl ContainerIPdu {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::ContainerIPdu::try_from(element.0.clone()) {
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

    /// set the header type of this `ContainerIPdu`
    #[setter]
    fn set_header_type(&self, header_type: ContainerIPduHeaderType) -> PyResult<()> {
        self.0
            .set_header_type(header_type.into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the header type of this `ContainerIPdu`
    #[getter]
    fn header_type(&self) -> Option<ContainerIPduHeaderType> {
        self.0.header_type().map(Into::into)
    }

    /// set the rx accept of this `ContainerIPdu`
    #[setter]
    fn set_rx_accept_contained_ipdu(&self, rx_accept: RxAcceptContainedIPdu) -> PyResult<()> {
        self.0
            .set_rx_accept_contained_ipdu(rx_accept.into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the rx accept of this `ContainerIPdu`
    #[getter]
    fn rx_accept_contained_ipdu(&self) -> Option<RxAcceptContainedIPdu> {
        self.0.rx_accept_contained_ipdu().map(Into::into)
    }

    /// set the container timeout of this `ContainerIPdu`
    #[setter]
    fn set_container_timeout(&self, timeout: Option<f64>) -> PyResult<()> {
        self.0
            .set_container_timeout(timeout)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the container timeout of this `ContainerIPdu`
    #[getter]
    fn container_timeout(&self) -> Option<f64> {
        self.0.container_timeout()
    }

    /// set the container trigger of this `ContainerIPdu`
    #[setter]
    fn set_container_trigger(&self, trigger: Option<ContainerIPduTrigger>) -> PyResult<()> {
        self.0
            .set_container_trigger(trigger.map(Into::into))
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the container trigger of this `ContainerIPdu`
    #[getter]
    fn container_trigger(&self) -> Option<ContainerIPduTrigger> {
        self.0.container_trigger().map(Into::into)
    }

    /// map an IPdu to this `ContainerIPdu`, and create a PduTriggering for it in the physical channel
    #[pyo3(signature = (ipdu, physical_channel, /))]
    #[pyo3(text_signature = "(self, ipdu: AbstractIPdu, physical_channel: PhysicalChannel, /)")]
    fn map_ipdu(
        &self,
        ipdu: &Bound<'_, PyAny>,
        physical_channel: &Bound<'_, PyAny>,
    ) -> PyResult<PduTriggering> {
        let ipdu = pyany_to_ipdu(ipdu)?;
        let physical_channel = pyany_to_physical_channel(physical_channel)?;
        match self.0.map_ipdu(&ipdu, &physical_channel) {
            Ok(value) => Ok(PduTriggering(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all contained IPdu triggerings
    fn contained_ipdu_triggerings(&self) -> PduTriggeringIterator {
        PduTriggeringIterator::new(self.0.contained_ipdu_triggerings().map(PduTriggering))
    }

    // --------- AbstractPdu methods ---------

    /// set the length of this PDU
    #[setter]
    fn set_length(&self, length: u32) -> PyResult<()> {
        self.0.set_length(length).map_err(abstraction_err_to_pyerr)
    }

    /// get the length of this PDU
    #[getter]
    fn length(&self) -> Option<u32> {
        self.0.length()
    }

    /// iterate over the `PduTriggerings` that trigger this PDU
    fn pdu_triggerings(&self) -> Vec<PduTriggering> {
        self.0
            .pdu_triggerings()
            .into_iter()
            .map(PduTriggering)
            .collect()
    }

    // --------- AbstractIPdu methods ---------

    /// set the ContainedIPduProps for this `IPdu`
    ///
    /// This is only relevant for IPdus that will be transmitted in `ContainerIPdus`
    #[setter]
    fn set_contained_ipdu_props(&self, props: Option<&ContainedIPduProps>) -> PyResult<()> {
        self.0
            .set_contained_ipdu_props(props.map(Into::into).as_ref())
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the ContainedIPduProps for this `IPdu`
    #[getter]
    fn contained_ipdu_props(&self) -> Option<ContainedIPduProps> {
        self.0.contained_ipdu_props().map(Into::into)
    }
}

//##################################################################

/// The header type of a `ContainerIPdu`
#[allow(clippy::enum_variant_names)] // item names are taken from the AUTOSAR standard
#[pyclass(
    frozen,
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContainerIPduHeaderType {
    /// Header size is 64 bit: Header id is 32 bit, dlc is 32 bit
    LongHeader,
    /// no header is used, the locations of the contained PDUs are fixed
    NoHeader,
    /// Header size is 32 bit: Header id is 24 bit, dlc is 8 bit
    ShortHeader,
}

impl From<autosar_data_abstraction::communication::ContainerIPduHeaderType>
    for ContainerIPduHeaderType
{
    fn from(header_type: autosar_data_abstraction::communication::ContainerIPduHeaderType) -> Self {
        match header_type {
            autosar_data_abstraction::communication::ContainerIPduHeaderType::LongHeader => {
                Self::LongHeader
            }
            autosar_data_abstraction::communication::ContainerIPduHeaderType::NoHeader => {
                Self::NoHeader
            }
            autosar_data_abstraction::communication::ContainerIPduHeaderType::ShortHeader => {
                Self::ShortHeader
            }
        }
    }
}

impl From<ContainerIPduHeaderType>
    for autosar_data_abstraction::communication::ContainerIPduHeaderType
{
    fn from(header_type: ContainerIPduHeaderType) -> Self {
        match header_type {
            ContainerIPduHeaderType::LongHeader => {
                autosar_data_abstraction::communication::ContainerIPduHeaderType::LongHeader
            }
            ContainerIPduHeaderType::NoHeader => {
                autosar_data_abstraction::communication::ContainerIPduHeaderType::NoHeader
            }
            ContainerIPduHeaderType::ShortHeader => {
                autosar_data_abstraction::communication::ContainerIPduHeaderType::ShortHeader
            }
        }
    }
}

//##################################################################

/// The `RxAcceptContainedIPdu` enum defines whether a fixed set of contained IPdus is accepted or all contained IPdus
#[pyclass(
    frozen,
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RxAcceptContainedIPdu {
    /// All contained IPdus are accepted
    AcceptAll,
    /// Only the configured contained IPdus are accepted
    AcceptConfigured,
}

impl From<autosar_data_abstraction::communication::RxAcceptContainedIPdu>
    for RxAcceptContainedIPdu
{
    fn from(
        accept_contained_ipdu: autosar_data_abstraction::communication::RxAcceptContainedIPdu,
    ) -> Self {
        match accept_contained_ipdu {
            autosar_data_abstraction::communication::RxAcceptContainedIPdu::AcceptAll => {
                Self::AcceptAll
            }
            autosar_data_abstraction::communication::RxAcceptContainedIPdu::AcceptConfigured => {
                Self::AcceptConfigured
            }
        }
    }
}

impl From<RxAcceptContainedIPdu>
    for autosar_data_abstraction::communication::RxAcceptContainedIPdu
{
    fn from(accept_contained_ipdu: RxAcceptContainedIPdu) -> Self {
        match accept_contained_ipdu {
            RxAcceptContainedIPdu::AcceptAll => {
                autosar_data_abstraction::communication::RxAcceptContainedIPdu::AcceptAll
            }
            RxAcceptContainedIPdu::AcceptConfigured => {
                autosar_data_abstraction::communication::RxAcceptContainedIPdu::AcceptConfigured
            }
        }
    }
}

//##################################################################

/// collection semantics for the ContainedIPdu
#[pyclass(
    frozen,
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContainedIPduCollectionSemantics {
    /// The ContainedIPdu data will be fetched via TriggerTransmit just before the transmission executes.
    LastIsBest,
    /// The ContainedIPdu data will instantly be stored to the ContainerIPdu in the context of the Transmit call
    Queued,
}

impl From<autosar_data_abstraction::communication::ContainedIPduCollectionSemantics>
    for ContainedIPduCollectionSemantics
{
    fn from(
        collection_semantics: autosar_data_abstraction::communication::ContainedIPduCollectionSemantics,
    ) -> Self {
        match collection_semantics {
            autosar_data_abstraction::communication::ContainedIPduCollectionSemantics::LastIsBest => {
                Self::LastIsBest
            }
            autosar_data_abstraction::communication::ContainedIPduCollectionSemantics::Queued => {
                Self::Queued
            }
        }
    }
}

impl From<ContainedIPduCollectionSemantics>
    for autosar_data_abstraction::communication::ContainedIPduCollectionSemantics
{
    fn from(collection_semantics: ContainedIPduCollectionSemantics) -> Self {
        match collection_semantics {
            ContainedIPduCollectionSemantics::LastIsBest => {
                autosar_data_abstraction::communication::ContainedIPduCollectionSemantics::LastIsBest
            }
            ContainedIPduCollectionSemantics::Queued => {
                autosar_data_abstraction::communication::ContainedIPduCollectionSemantics::Queued
            }
        }
    }
}

//##################################################################

/// Defines when the transmission of the ContainerIPdu shall be requested
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[pyclass(
    frozen,
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
pub enum ContainerIPduTrigger {
    /// transmission of the ContainerIPdu shall be requested when the default trigger conditions apply
    DefaultTrigger,
    /// transmission of the ContainerIPdu shall be requested right after the first Contained
    /// IPdu was put into the ContainerIPdu
    FirstContainedTrigger,
}

impl From<autosar_data_abstraction::communication::ContainerIPduTrigger> for ContainerIPduTrigger {
    fn from(trigger: autosar_data_abstraction::communication::ContainerIPduTrigger) -> Self {
        match trigger {
            autosar_data_abstraction::communication::ContainerIPduTrigger::DefaultTrigger => {
                Self::DefaultTrigger
            }
            autosar_data_abstraction::communication::ContainerIPduTrigger::FirstContainedTrigger => {
                Self::FirstContainedTrigger
            }
        }
    }
}

impl From<ContainerIPduTrigger> for autosar_data_abstraction::communication::ContainerIPduTrigger {
    fn from(trigger: ContainerIPduTrigger) -> Self {
        match trigger {
            ContainerIPduTrigger::DefaultTrigger => {
                autosar_data_abstraction::communication::ContainerIPduTrigger::DefaultTrigger
            }
            ContainerIPduTrigger::FirstContainedTrigger => {
                autosar_data_abstraction::communication::ContainerIPduTrigger::FirstContainedTrigger
            }
        }
    }
}

//##################################################################

/// Properties for an IPdu that is transmitted in a container IPdu
#[pyclass(
    get_all,
    set_all,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, PartialEq)]
pub struct ContainedIPduProps {
    /// collection semantics: LastIsBest or Queued
    pub collection_semantics: Option<ContainedIPduCollectionSemantics>,
    /// header id of the contained IPdu, used when the header type is LongHeader
    pub header_id_long: Option<u32>,
    /// header id of the contained IPdu, used when the header type is ShortHeader
    pub header_id_short: Option<u32>, // 24 bit
    /// offset of the contained IPdu in the container IPdu, used when the header type is NoHeader
    pub offset: Option<u32>,
    /// priority of the contained IPdu. 255: lowest, 0: highest
    pub priority: Option<u8>,
    /// sender timeout. Ignored on the receiver side
    pub timeout: Option<f64>,
    /// defines whether the contained IPdu triggers transmission of the container IPdu
    pub trigger: Option<PduCollectionTrigger>,
    /// update indication bit position of the contained IPdu
    pub update_indication_bit_position: Option<u32>,
}

impl From<autosar_data_abstraction::communication::ContainedIPduProps> for ContainedIPduProps {
    fn from(props: autosar_data_abstraction::communication::ContainedIPduProps) -> Self {
        Self {
            collection_semantics: props.collection_semantics.map(Into::into),
            header_id_long: props.header_id_long,
            header_id_short: props.header_id_short,
            offset: props.offset,
            priority: props.priority,
            timeout: props.timeout,
            trigger: props.trigger.map(Into::into),
            update_indication_bit_position: props.update_indication_bit_position,
        }
    }
}

impl From<&ContainedIPduProps> for autosar_data_abstraction::communication::ContainedIPduProps {
    fn from(props: &ContainedIPduProps) -> Self {
        Self {
            collection_semantics: props.collection_semantics.map(Into::into),
            header_id_long: props.header_id_long,
            header_id_short: props.header_id_short,
            offset: props.offset,
            priority: props.priority,
            timeout: props.timeout,
            trigger: props.trigger.map(Into::into),
            update_indication_bit_position: props.update_indication_bit_position,
        }
    }
}

#[pymethods]
impl ContainedIPduProps {
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (*, collection_semantics=None, header_id_long=None, header_id_short=None, offset=None, priority=None,
                        timeout=None, trigger=None, update_indication_bit_position=None))]
    #[pyo3(text_signature = "(self, *,
                              collection_semantics: Optional[ContainedIPduCollectionSemantics] = None,
                              header_id_long: Optional[int] = None,
                              header_id_short: Optional[int] = None,
                              offset: Optional[int] = None,
                              priority: Optional[int] = None,
                              timeout: Optional[float] = None,
                              trigger: Optional[PduCollectionTrigger] = None,
                              update_indication_bit_position: Optional[int] = None)")]
    #[new]
    fn new(
        collection_semantics: Option<ContainedIPduCollectionSemantics>,
        header_id_long: Option<u32>,
        header_id_short: Option<u32>,
        offset: Option<u32>,
        priority: Option<u8>,
        timeout: Option<f64>,
        trigger: Option<PduCollectionTrigger>,
        update_indication_bit_position: Option<u32>,
    ) -> Self {
        Self {
            collection_semantics,
            header_id_long,
            header_id_short,
            offset,
            priority,
            timeout,
            trigger,
            update_indication_bit_position,
        }
    }
}
