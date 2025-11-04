use crate::{
    Element,
    abstraction::{
        AutosarAbstractionError, ByteOrder, EcuInstance, abstraction_err_to_pyerr,
        communication::{
            CanPhysicalChannel, CommunicationDirection, EthernetPhysicalChannel,
            FlexrayPhysicalChannel, ISignal, ISignalGroup, ISignalTriggering, LinPhysicalChannel,
            TransferProperty,
        },
    },
    iterator_wrapper,
};
use autosar_data_abstraction::{
    self, AbstractionElement, IdentifiableAbstractionElement,
    communication::{AbstractIpdu, AbstractPdu, SignalPdu},
};
use pyo3::{IntoPyObjectExt, prelude::*};

pub(crate) mod container_ipdu;
pub(crate) mod isignal_ipdu;
pub(crate) mod secured_ipdu;

pub(crate) use container_ipdu::*;
pub(crate) use isignal_ipdu::*;
pub(crate) use secured_ipdu::*;

//##################################################################

/// Network Management Pdu
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct NmPdu(pub(crate) autosar_data_abstraction::communication::NmPdu);

#[pymethods]
impl NmPdu {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::NmPdu::try_from(element.0.clone()) {
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

    /// returns an iterator over all signals and signal groups mapped to the PDU
    fn mapped_signals(&self) -> ISignalToIPduMappingIterator {
        ISignalToIPduMappingIterator::new(self.0.mapped_signals().map(ISignalToIPduMapping))
    }

    /// map a signal to the `ISignalIPdu`
    ///
    /// If this signal is part of a signal group, then the group must be mapped first
    #[pyo3(signature = (signal, start_position, byte_order, /, *, update_bit=None, transfer_property=TransferProperty::Pending))]
    #[pyo3(
        text_signature = "(self, signal: ISignal, start_position: int, byte_order: ByteOrder, /, *, update_bit: Optional[int] = None, transfer_property: TransferProperty = TransferProperty.Pending)"
    )]
    fn map_signal(
        &self,
        signal: &ISignal,
        start_position: u32,
        byte_order: ByteOrder,
        update_bit: Option<u32>,
        transfer_property: TransferProperty,
    ) -> PyResult<ISignalToIPduMapping> {
        match self.0.map_signal(
            &signal.0,
            start_position,
            byte_order.into(),
            update_bit,
            transfer_property.into(),
        ) {
            Ok(value) => Ok(ISignalToIPduMapping(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// map a signal group to the PDU
    #[pyo3(signature = (signal_group, /))]
    #[pyo3(text_signature = "(self, signal_group: ISignalGroup, /)")]
    fn map_signal_group(&self, signal_group: &ISignalGroup) -> PyResult<ISignalToIPduMapping> {
        match self.0.map_signal_group(&signal_group.0) {
            Ok(value) => Ok(ISignalToIPduMapping(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// set the unused bit pattern for this PDU
    #[setter]
    fn set_unused_bit_pattern(&self, pattern: u8) -> PyResult<()> {
        self.0
            .set_unused_bit_pattern(pattern)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the unused bit pattern for this PDU
    #[getter]
    fn unused_bit_pattern(&self) -> Option<u8> {
        self.0.unused_bit_pattern()
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
}

//##################################################################

/// This is a Pdu of the transport layer. The main purpose of the TP layer is to segment and reassemble `IPdus`.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct NPdu(pub(crate) autosar_data_abstraction::communication::NPdu);

#[pymethods]
impl NPdu {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::NPdu::try_from(element.0.clone()) {
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

iterator_wrapper!(NPduIterator, NPdu);

//##################################################################

/// Represents the `IPdus` handled by Dcm
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct DcmIPdu(pub(crate) autosar_data_abstraction::communication::DcmIPdu);

#[pymethods]
impl DcmIPdu {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::DcmIPdu::try_from(element.0.clone()) {
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

    #[getter]
    fn diag_pdu_type(&self) -> Option<DiagPduType> {
        self.0.diag_pdu_type().map(Into::into)
    }

    #[setter]
    fn set_diag_pdu_type(&self, diag_pdu_type: DiagPduType) -> PyResult<()> {
        self.0
            .set_diag_pdu_type(diag_pdu_type.into())
            .map_err(abstraction_err_to_pyerr)
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

/// The category of a `GeneralPurposePdu`
///
/// The Autosar standard defines the following categories:
/// - `SD`
/// - `GLOBAL_TIME`
/// - `DOIP`
#[pyclass(
    frozen,
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DiagPduType {
    /// diagnostic request
    DiagRequest,
    /// diagnostic response
    DiagResponse,
}

impl From<autosar_data_abstraction::communication::DiagPduType> for DiagPduType {
    fn from(category: autosar_data_abstraction::communication::DiagPduType) -> Self {
        match category {
            autosar_data_abstraction::communication::DiagPduType::DiagRequest => {
                DiagPduType::DiagRequest
            }
            autosar_data_abstraction::communication::DiagPduType::DiagResponse => {
                DiagPduType::DiagResponse
            }
        }
    }
}

impl From<DiagPduType> for autosar_data_abstraction::communication::DiagPduType {
    fn from(category: DiagPduType) -> Self {
        match category {
            DiagPduType::DiagRequest => {
                autosar_data_abstraction::communication::DiagPduType::DiagRequest
            }
            DiagPduType::DiagResponse => {
                autosar_data_abstraction::communication::DiagPduType::DiagResponse
            }
        }
    }
}

//##################################################################

/// This element is used for AUTOSAR Pdus without additional attributes that are routed by a bus interface
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct GeneralPurposePdu(
    pub(crate) autosar_data_abstraction::communication::GeneralPurposePdu,
);

#[pymethods]
impl GeneralPurposePdu {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::GeneralPurposePdu::try_from(
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

    /// set the category of this PDU
    #[setter]
    fn set_category(&self, category: GeneralPurposePduCategory) -> PyResult<()> {
        self.0
            .set_category(category.into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the category of this PDU
    #[getter]
    fn category(&self) -> Option<GeneralPurposePduCategory> {
        self.0.category().map(std::convert::Into::into)
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
}

//##################################################################

/// The category of a `GeneralPurposePdu`
///
/// The Autosar standard defines the following categories:
/// - `SD`
/// - `GLOBAL_TIME`
/// - `DOIP`
#[pyclass(
    frozen,
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GeneralPurposePduCategory {
    /// Service Discovery
    Sd,
    /// Global Time Synchronization
    GlobalTime,
    /// Diagnostic over IP
    DoIp,
}

impl From<autosar_data_abstraction::communication::GeneralPurposePduCategory>
    for GeneralPurposePduCategory
{
    fn from(category: autosar_data_abstraction::communication::GeneralPurposePduCategory) -> Self {
        match category {
            autosar_data_abstraction::communication::GeneralPurposePduCategory::Sd => {
                GeneralPurposePduCategory::Sd
            }
            autosar_data_abstraction::communication::GeneralPurposePduCategory::GlobalTime => {
                GeneralPurposePduCategory::GlobalTime
            }
            autosar_data_abstraction::communication::GeneralPurposePduCategory::DoIp => {
                GeneralPurposePduCategory::DoIp
            }
        }
    }
}

impl From<GeneralPurposePduCategory>
    for autosar_data_abstraction::communication::GeneralPurposePduCategory
{
    fn from(category: GeneralPurposePduCategory) -> Self {
        match category {
            GeneralPurposePduCategory::Sd => {
                autosar_data_abstraction::communication::GeneralPurposePduCategory::Sd
            }
            GeneralPurposePduCategory::GlobalTime => {
                autosar_data_abstraction::communication::GeneralPurposePduCategory::GlobalTime
            }
            GeneralPurposePduCategory::DoIp => {
                autosar_data_abstraction::communication::GeneralPurposePduCategory::DoIp
            }
        }
    }
}

//##################################################################

/// This element is used for AUTOSAR Pdus without attributes that are routed by the `PduR`
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct GeneralPurposeIPdu(
    pub(crate) autosar_data_abstraction::communication::GeneralPurposeIPdu,
);

#[pymethods]
impl GeneralPurposeIPdu {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::GeneralPurposeIPdu::try_from(
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

    /// set the category of this PDU
    #[setter]
    fn set_category(&self, category: GeneralPurposeIPduCategory) -> PyResult<()> {
        self.0
            .set_category(category.into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the category of this PDU
    #[getter]
    fn category(&self) -> Option<GeneralPurposeIPduCategory> {
        self.0.category().map(std::convert::Into::into)
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

/// The category of a `GeneralPurposeIPdu`
///
/// The Autosar standard defines the following categories:
/// - XCP
/// - SOMEIP_SEGMENTED_IPDU
/// - DLT
#[pyclass(
    frozen,
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GeneralPurposeIPduCategory {
    /// XCP
    Xcp,
    /// SOME/IP Segmented `IPdu`
    SomeipSegmentedIpdu,
    /// Diagnostic Log and Trace
    Dlt,
}

impl From<autosar_data_abstraction::communication::GeneralPurposeIPduCategory>
    for GeneralPurposeIPduCategory
{
    fn from(category: autosar_data_abstraction::communication::GeneralPurposeIPduCategory) -> Self {
        match category {
            autosar_data_abstraction::communication::GeneralPurposeIPduCategory::Xcp => GeneralPurposeIPduCategory::Xcp,
            autosar_data_abstraction::communication::GeneralPurposeIPduCategory::SomeipSegmentedIpdu => GeneralPurposeIPduCategory::SomeipSegmentedIpdu,
            autosar_data_abstraction::communication::GeneralPurposeIPduCategory::Dlt => GeneralPurposeIPduCategory::Dlt,
        }
    }
}

impl From<GeneralPurposeIPduCategory>
    for autosar_data_abstraction::communication::GeneralPurposeIPduCategory
{
    fn from(category: GeneralPurposeIPduCategory) -> Self {
        match category {
            GeneralPurposeIPduCategory::Xcp => autosar_data_abstraction::communication::GeneralPurposeIPduCategory::Xcp,
            GeneralPurposeIPduCategory::SomeipSegmentedIpdu => autosar_data_abstraction::communication::GeneralPurposeIPduCategory::SomeipSegmentedIpdu,
            GeneralPurposeIPduCategory::Dlt => autosar_data_abstraction::communication::GeneralPurposeIPduCategory::Dlt,
        }
    }
}

//##################################################################

/// The multiplexed pdu contains one of serveral signal pdus
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct MultiplexedIPdu(
    pub(crate) autosar_data_abstraction::communication::MultiplexedIPdu,
);

#[pymethods]
impl MultiplexedIPdu {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::MultiplexedIPdu::try_from(element.0.clone())
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

    #[setter]
    fn set_static_part(&self, static_part: &ISignalIPdu) -> PyResult<()> {
        self.0
            .set_static_part(&static_part.0)
            .map_err(abstraction_err_to_pyerr)
    }

    #[getter]
    fn static_part(&self) -> Option<ISignalIPdu> {
        self.0.static_part().map(|pdu| ISignalIPdu(pdu.clone()))
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

    /// addd a dynamic part alternative to this `MultiplexedIPdu`
    #[pyo3(signature = (dynamic_ipdu, selector_code, /, *, initial_dynamic_part))]
    #[pyo3(
        text_signature = "(self, dynamic_ipdu: ISignalIPdu, selector_code: int, /, *, initial_dynamic_part: bool = false)"
    )]
    fn add_dynamic_part(
        &self,
        dynamic_ipdu: &ISignalIPdu,
        selector_code: u16,
        initial_dynamic_part: bool,
    ) -> PyResult<DynamicPartAlternative> {
        match self
            .0
            .add_dynamic_part(&dynamic_ipdu.0, selector_code, initial_dynamic_part)
        {
            Ok(value) => Ok(DynamicPartAlternative(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    // TODO: enable iterator once the fixed version of autosar-data-abstraction is published
    // fn dynamic_part_alternatives(&self) -> DynamicPartAlternativesIterator {
    //     DynamicPartAlternativesIterator::new(
    //         self.0
    //             .dynamic_part_alternatives()
    //             .map(DynamicPartAlternative),
    //     )
    // }

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

// iterator_wrapper!(DynamicPartAlternativesIterator, DynamicPartAlternative);

//##################################################################

/// An alternative for the dynamic part of a `MultiplexedIPdu`
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct DynamicPartAlternative(
    pub(crate) autosar_data_abstraction::communication::DynamicPartAlternative,
);

#[pymethods]
impl DynamicPartAlternative {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::DynamicPartAlternative::try_from(
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
}

//##################################################################

/// a `PduTriggering` triggers a PDU in a frame or ethernet connection
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct PduTriggering(pub(crate) autosar_data_abstraction::communication::PduTriggering);

#[pymethods]
impl PduTriggering {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::PduTriggering::try_from(element.0.clone()) {
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

    /// get the Pdu that is triggered by this pdu triggering
    #[getter]
    fn pdu(&self) -> Option<Py<PyAny>> {
        self.0.pdu().and_then(|pdu| pdu_to_pyany(&pdu).ok())
    }

    /// get the physical channel that contains this pdu triggering
    #[getter]
    fn physical_channel(&self, py: Python) -> PyResult<Py<PyAny>> {
        match self.0.physical_channel() {
            Ok(physical_channel) => match physical_channel {
                autosar_data_abstraction::communication::PhysicalChannel::Can(
                    can_physical_channel,
                ) => CanPhysicalChannel(can_physical_channel).into_py_any(py),
                autosar_data_abstraction::communication::PhysicalChannel::Ethernet(
                    ethernet_physical_channel,
                ) => EthernetPhysicalChannel(ethernet_physical_channel).into_py_any(py),
                autosar_data_abstraction::communication::PhysicalChannel::Flexray(
                    flexray_physical_channel,
                ) => FlexrayPhysicalChannel(flexray_physical_channel).into_py_any(py),
                autosar_data_abstraction::communication::PhysicalChannel::Lin(
                    lin_physical_channel,
                ) => LinPhysicalChannel(lin_physical_channel).into_py_any(py),
            },
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// create an `IPduPort` to connect a `PduTriggering` to an `EcuInstance`
    #[pyo3(signature = (ecu, direction, /))]
    #[pyo3(text_signature = "(self, ecu: EcuInstance, direction: CommunicationDirection, /)")]
    fn create_pdu_port(
        &self,
        ecu: &EcuInstance,
        direction: CommunicationDirection,
    ) -> PyResult<IPduPort> {
        match self.0.create_pdu_port(&ecu.0, direction.into()) {
            Ok(value) => Ok(IPduPort(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create an iterator over the `IPduPorts` that are connected to this `PduTriggering`
    fn pdu_ports(&self) -> PduPortsIterator {
        PduPortsIterator::new(self.0.pdu_ports().map(IPduPort))
    }

    /// create an iterator over the `ISignalTriggerings` that are triggered by this `PduTriggering`
    fn signal_triggerings(&self) -> SignalTriggeringsIterator {
        SignalTriggeringsIterator::new(self.0.signal_triggerings().map(ISignalTriggering))
    }
}

//##################################################################

iterator_wrapper!(PduTriggeringIterator, PduTriggering);
iterator_wrapper!(PduPortsIterator, IPduPort);
iterator_wrapper!(SignalTriggeringsIterator, ISignalTriggering);

//##################################################################

/// The `IPduPort` allows an ECU to send or receive a PDU
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct IPduPort(pub(crate) autosar_data_abstraction::communication::IPduPort);

#[pymethods]
impl IPduPort {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::IPduPort::try_from(element.0.clone()) {
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

    /// get the ECU instance that contains this `IPduPort`
    #[getter]
    fn ecu(&self) -> PyResult<EcuInstance> {
        match self.0.ecu() {
            Ok(ecu) => Ok(EcuInstance(ecu)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// set the communication direction of this `IPduPort`
    #[setter]
    fn set_communication_direction(&self, direction: CommunicationDirection) -> PyResult<()> {
        self.0
            .set_communication_direction(direction.into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the communication direction of this `IPduPort`
    #[getter]
    fn communication_direction(&self) -> Option<CommunicationDirection> {
        self.0
            .communication_direction()
            .map(std::convert::Into::into)
    }
}

//##################################################################

/// The collction trigger defines whether a Pdu contributes to the triggering
/// of the data transmission if Pdu collection is enabled
#[pyclass(
    frozen,
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PduCollectionTrigger {
    /// Pdu will trigger the transmission of the data.
    Always,
    /// Pdu will be buffered and will not trigger the transmission of the data
    Never,
}

impl From<autosar_data_abstraction::communication::PduCollectionTrigger> for PduCollectionTrigger {
    fn from(trigger: autosar_data_abstraction::communication::PduCollectionTrigger) -> Self {
        match trigger {
            autosar_data_abstraction::communication::PduCollectionTrigger::Always => {
                PduCollectionTrigger::Always
            }
            autosar_data_abstraction::communication::PduCollectionTrigger::Never => {
                PduCollectionTrigger::Never
            }
        }
    }
}

impl From<PduCollectionTrigger> for autosar_data_abstraction::communication::PduCollectionTrigger {
    fn from(trigger: PduCollectionTrigger) -> Self {
        match trigger {
            PduCollectionTrigger::Always => {
                autosar_data_abstraction::communication::PduCollectionTrigger::Always
            }
            PduCollectionTrigger::Never => {
                autosar_data_abstraction::communication::PduCollectionTrigger::Never
            }
        }
    }
}

//##################################################################

pub(crate) fn pdu_to_pyany(
    pdu: &autosar_data_abstraction::communication::Pdu,
) -> PyResult<Py<PyAny>> {
    Python::attach(|py| match pdu {
        autosar_data_abstraction::communication::Pdu::ISignalIPdu(isignal_ipdu) => {
            ISignalIPdu(isignal_ipdu.clone()).into_py_any(py)
        }
        autosar_data_abstraction::communication::Pdu::NmPdu(nm_pdu) => {
            NmPdu(nm_pdu.clone()).into_py_any(py)
        }
        autosar_data_abstraction::communication::Pdu::NPdu(npdu) => {
            NPdu(npdu.clone()).into_py_any(py)
        }
        autosar_data_abstraction::communication::Pdu::DcmIPdu(dcm_ipdu) => {
            DcmIPdu(dcm_ipdu.clone()).into_py_any(py)
        }
        autosar_data_abstraction::communication::Pdu::GeneralPurposePdu(general_purpose_pdu) => {
            GeneralPurposePdu(general_purpose_pdu.clone()).into_py_any(py)
        }
        autosar_data_abstraction::communication::Pdu::GeneralPurposeIPdu(general_purpose_ipdu) => {
            GeneralPurposeIPdu(general_purpose_ipdu.clone()).into_py_any(py)
        }
        autosar_data_abstraction::communication::Pdu::ContainerIPdu(container_ipdu) => {
            ContainerIPdu(container_ipdu.clone()).into_py_any(py)
        }
        autosar_data_abstraction::communication::Pdu::SecuredIPdu(secured_ipdu) => {
            SecuredIPdu(secured_ipdu.clone()).into_py_any(py)
        }
        autosar_data_abstraction::communication::Pdu::MultiplexedIPdu(multiplexed_ipdu) => {
            MultiplexedIPdu(multiplexed_ipdu.clone()).into_py_any(py)
        }
    })
}

pub(crate) fn pyany_to_pdu(
    py_any: &Bound<'_, PyAny>,
) -> PyResult<autosar_data_abstraction::communication::Pdu> {
    if let Ok(isignal_ipdu) = py_any.extract::<ISignalIPdu>() {
        Ok(autosar_data_abstraction::communication::Pdu::ISignalIPdu(
            isignal_ipdu.0,
        ))
    } else if let Ok(nm_pdu) = py_any.extract::<NmPdu>() {
        Ok(autosar_data_abstraction::communication::Pdu::NmPdu(
            nm_pdu.0,
        ))
    } else if let Ok(npdu) = py_any.extract::<NPdu>() {
        Ok(autosar_data_abstraction::communication::Pdu::NPdu(npdu.0))
    } else if let Ok(dcm_ipdu) = py_any.extract::<DcmIPdu>() {
        Ok(autosar_data_abstraction::communication::Pdu::DcmIPdu(
            dcm_ipdu.0,
        ))
    } else if let Ok(general_purpose_pdu) = py_any.extract::<GeneralPurposePdu>() {
        Ok(autosar_data_abstraction::communication::Pdu::GeneralPurposePdu(general_purpose_pdu.0))
    } else if let Ok(general_purpose_ipdu) = py_any.extract::<GeneralPurposeIPdu>() {
        Ok(
            autosar_data_abstraction::communication::Pdu::GeneralPurposeIPdu(
                general_purpose_ipdu.0,
            ),
        )
    } else if let Ok(container_ipdu) = py_any.extract::<ContainerIPdu>() {
        Ok(autosar_data_abstraction::communication::Pdu::ContainerIPdu(
            container_ipdu.0,
        ))
    } else if let Ok(secured_ipdu) = py_any.extract::<SecuredIPdu>() {
        Ok(autosar_data_abstraction::communication::Pdu::SecuredIPdu(
            secured_ipdu.0,
        ))
    } else if let Ok(multiplexed_ipdu) = py_any.extract::<MultiplexedIPdu>() {
        Ok(autosar_data_abstraction::communication::Pdu::MultiplexedIPdu(multiplexed_ipdu.0))
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(format!(
            "'{:?}' cannot be converted to 'AbstractPdu'",
            py_any.get_type().name()
        )))
    }
}

pub(crate) fn ipdu_to_pyany(
    ipdu: &autosar_data_abstraction::communication::IPdu,
) -> PyResult<Py<PyAny>> {
    Python::attach(|py| match ipdu {
        autosar_data_abstraction::communication::IPdu::ISignalIPdu(isignal_ipdu) => {
            ISignalIPdu(isignal_ipdu.clone()).into_py_any(py)
        }
        autosar_data_abstraction::communication::IPdu::NPdu(npdu) => {
            NPdu(npdu.clone()).into_py_any(py)
        }
        autosar_data_abstraction::communication::IPdu::DcmIPdu(dcm_ipdu) => {
            DcmIPdu(dcm_ipdu.clone()).into_py_any(py)
        }
        autosar_data_abstraction::communication::IPdu::GeneralPurposeIPdu(general_purpose_ipdu) => {
            GeneralPurposeIPdu(general_purpose_ipdu.clone()).into_py_any(py)
        }
        autosar_data_abstraction::communication::IPdu::ContainerIPdu(container_ipdu) => {
            ContainerIPdu(container_ipdu.clone()).into_py_any(py)
        }
        autosar_data_abstraction::communication::IPdu::SecuredIPdu(secured_ipdu) => {
            SecuredIPdu(secured_ipdu.clone()).into_py_any(py)
        }
        autosar_data_abstraction::communication::IPdu::MultiplexedIPdu(multiplexed_ipdu) => {
            MultiplexedIPdu(multiplexed_ipdu.clone()).into_py_any(py)
        }
    })
}

pub(crate) fn pyany_to_ipdu(
    py_any: &Bound<'_, PyAny>,
) -> PyResult<autosar_data_abstraction::communication::IPdu> {
    if let Ok(isignal_ipdu) = py_any.extract::<ISignalIPdu>() {
        Ok(autosar_data_abstraction::communication::IPdu::ISignalIPdu(
            isignal_ipdu.0,
        ))
    } else if let Ok(npdu) = py_any.extract::<NPdu>() {
        Ok(autosar_data_abstraction::communication::IPdu::NPdu(npdu.0))
    } else if let Ok(dcm_ipdu) = py_any.extract::<DcmIPdu>() {
        Ok(autosar_data_abstraction::communication::IPdu::DcmIPdu(
            dcm_ipdu.0,
        ))
    } else if let Ok(general_purpose_ipdu) = py_any.extract::<GeneralPurposeIPdu>() {
        Ok(
            autosar_data_abstraction::communication::IPdu::GeneralPurposeIPdu(
                general_purpose_ipdu.0,
            ),
        )
    } else if let Ok(container_ipdu) = py_any.extract::<ContainerIPdu>() {
        Ok(autosar_data_abstraction::communication::IPdu::ContainerIPdu(container_ipdu.0))
    } else if let Ok(secured_ipdu) = py_any.extract::<SecuredIPdu>() {
        Ok(autosar_data_abstraction::communication::IPdu::SecuredIPdu(
            secured_ipdu.0,
        ))
    } else if let Ok(multiplexed_ipdu) = py_any.extract::<MultiplexedIPdu>() {
        Ok(autosar_data_abstraction::communication::IPdu::MultiplexedIPdu(multiplexed_ipdu.0))
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(format!(
            "'{:?}' cannot be converted to 'AbstractPdu'",
            py_any.get_type().name()
        )))
    }
}
