use crate::{
    Element,
    abstraction::{
        AutosarAbstractionError, ByteOrder, abstraction_err_to_pyerr,
        communication::{
            ContainedIPduProps, ISignal, ISignalGroup, PduTriggering, TransferProperty,
        },
    },
    iterator_wrapper,
};
use autosar_data_abstraction::{
    self, AbstractionElement, IdentifiableAbstractionElement,
    communication::{AbstractIpdu, AbstractPdu, SignalPdu},
};
use pyo3::prelude::*;

//##################################################################

/// Represents the `IPdus` handled by Com
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct ISignalIPdu(pub(crate) autosar_data_abstraction::communication::ISignalIPdu);

#[pymethods]
impl ISignalIPdu {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::ISignalIPdu::try_from(element.0.clone()) {
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

    /// set the transmission timing of the PDU
    #[pyo3(signature = (timing_spec, /))]
    #[pyo3(text_signature = "(self, timing_spec: IpduTiming, /)")]
    fn set_timing(&self, timing_spec: &IpduTiming) -> PyResult<()> {
        self.0
            .set_timing(&timing_spec.into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// Helper function to set the transmission mode timing, used by `ISignalIPdu::set_timing` for both true and false timing
    /// get the transmission timing of the PDU
    fn timing(&self) -> Option<IpduTiming> {
        self.0.timing().map(std::convert::Into::into)
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

    /// List all `PduTriggerings` that trigger this PDU
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

/// `ISignalToIPduMapping` connects an `ISignal` or `ISignalGroup` to an `ISignalToIPdu`
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct ISignalToIPduMapping(
    pub(crate) autosar_data_abstraction::communication::ISignalToIPduMapping,
);

#[pymethods]
impl ISignalToIPduMapping {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::ISignalToIPduMapping::try_from(
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

    /// Reference to the signal that is mapped to the PDU.
    /// Every mapping contains either a signal or a signal group.
    #[getter]
    fn signal(&self) -> Option<ISignal> {
        self.0.signal().map(ISignal)
    }

    /// Set the byte order of the data in the mapped signal.
    #[setter]
    fn set_byte_order(&self, byte_order: ByteOrder) -> PyResult<()> {
        self.0
            .set_byte_order(byte_order.into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// Byte order of the data in the signal.
    #[getter]
    fn byte_order(&self) -> Option<ByteOrder> {
        self.0.byte_order().map(std::convert::Into::into)
    }

    /// Start position of the signal data within the PDU (bit position).
    /// The start position is mandatory if the mapping describes a signal.
    #[getter]
    fn start_position(&self) -> Option<u32> {
        self.0.start_position()
    }

    /// Bit position of the update bit for the mapped signal. Not all signals use an update bit.
    /// This is never used for signal groups
    #[getter]
    fn update_bit(&self) -> Option<u32> {
        self.0.update_bit()
    }

    /// Set the transfer property of the mapped signal
    #[setter]
    fn set_transfer_property(&self, transfer_property: TransferProperty) -> PyResult<()> {
        self.0
            .set_transfer_property(transfer_property.into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the transfer property of the mapped signal
    #[getter]
    fn transfer_property(&self) -> Option<TransferProperty> {
        self.0.transfer_property().map(std::convert::Into::into)
    }

    /// Reference to the signal group that is mapped to the PDU.
    /// Every mapping contains either a signal or a signal group.
    #[getter]
    fn signal_group(&self) -> Option<ISignalGroup> {
        self.0.signal_group().map(ISignalGroup)
    }
}

//##################################################################

iterator_wrapper!(ISignalToIPduMappingIterator, ISignalToIPduMapping);

//##################################################################

/// Timing specification for an IPDU
#[pyclass(
    eq,
    module = "autosar_data._autosar_data._abstraction._communication",
    get_all,
    set_all
)]
pub(crate) struct IpduTiming {
    /// minimum delay in seconds between two transmissions of the PDU
    pub minimum_delay: Option<f64>,
    /// timing specification if the COM transmission mode is true
    pub transmission_mode_true_timing: Option<Py<TransmissionModeTiming>>,
    /// timing specification if the COM transmission mode is false
    pub transmission_mode_false_timing: Option<Py<TransmissionModeTiming>>,
}

impl From<autosar_data_abstraction::communication::IpduTiming> for IpduTiming {
    fn from(timing: autosar_data_abstraction::communication::IpduTiming) -> Self {
        Python::attach(|py| Self {
            minimum_delay: timing.minimum_delay,
            transmission_mode_true_timing: timing
                .transmission_mode_true_timing
                .map(|value| Py::new(py, TransmissionModeTiming::from(value)).unwrap()),
            transmission_mode_false_timing: timing
                .transmission_mode_false_timing
                .map(|value| Py::new(py, TransmissionModeTiming::from(value)).unwrap()),
        })
    }
}

impl From<&IpduTiming> for autosar_data_abstraction::communication::IpduTiming {
    fn from(timing: &IpduTiming) -> Self {
        Python::attach(|py| Self {
            minimum_delay: timing.minimum_delay,
            transmission_mode_true_timing: timing
                .transmission_mode_true_timing
                .as_ref()
                .map(|value| (&*value.borrow(py)).into()),
            transmission_mode_false_timing: timing
                .transmission_mode_false_timing
                .as_ref()
                .map(|value| (&*value.borrow(py)).into()),
        })
    }
}

#[pymethods]
impl IpduTiming {
    #[pyo3(signature = (*, minimum_delay=None, transmission_mode_true_timing=None, transmission_mode_false_timing=None))]
    #[pyo3(
        text_signature = "(self, *, minimum_delay: Optional[float] = None, transmission_mode_true_timing: Optional[TransmissionModeTiming] = None, transmission_mode_false_timing: Optional[TransmissionModeTiming] = None)"
    )]
    #[new]
    fn new(
        minimum_delay: Option<f64>,
        transmission_mode_true_timing: Option<Py<TransmissionModeTiming>>,
        transmission_mode_false_timing: Option<Py<TransmissionModeTiming>>,
    ) -> Self {
        Self {
            minimum_delay,
            transmission_mode_true_timing,
            transmission_mode_false_timing,
        }
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}

impl std::fmt::Debug for IpduTiming {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Python::attach(|py| {
            let mut ds = f.debug_struct("IpduTiming");
            match &self.minimum_delay {
                Some(value) => {
                    ds.field("minimum_delay", value);
                }
                None => {
                    ds.field("minimum_delay", &None::<&f64>);
                }
            }
            match &self.transmission_mode_true_timing {
                Some(value) => {
                    ds.field("transmission_mode_true_timing", &value.borrow(py));
                }
                None => {
                    ds.field("transmission_mode_true_timing", &None::<&str>);
                }
            }
            match &self.transmission_mode_false_timing {
                Some(value) => {
                    ds.field("transmission_mode_false_timing", &value.borrow(py));
                }
                None => {
                    ds.field("transmission_mode_false_timing", &None::<&str>);
                }
            }
            ds.finish()
        })
    }
}

impl PartialEq for IpduTiming {
    fn eq(&self, other: &Self) -> bool {
        Python::attach(|py|
        match (&self.transmission_mode_true_timing, &other.transmission_mode_true_timing) {
            (Some(a), Some(b)) => *a.borrow(py) == *b.borrow(py),
            (None, None) => true,
            _ => false,
        } && match (&self.transmission_mode_false_timing, &other.transmission_mode_false_timing) {
            (Some(a), Some(b)) => *a.borrow(py) == *b.borrow(py),
            (None, None) => true,
            _ => false,
        }
    )
    }
}

//##################################################################

/// Cyclic and event controlled timing parameters for an IPDU
#[pyclass(
    eq,
    module = "autosar_data._autosar_data._abstraction._communication",
    get_all,
    set_all
)]
pub(crate) struct TransmissionModeTiming {
    /// cyclic timing parameters
    pub cyclic_timing: Option<Py<CyclicTiming>>,
    /// event controlled timing parameters
    pub event_controlled_timing: Option<Py<EventControlledTiming>>,
}

impl From<autosar_data_abstraction::communication::TransmissionModeTiming>
    for TransmissionModeTiming
{
    fn from(timing: autosar_data_abstraction::communication::TransmissionModeTiming) -> Self {
        Python::attach(|py| Self {
            cyclic_timing: timing
                .cyclic_timing
                .map(|value| Py::new(py, CyclicTiming::from(value)).unwrap()),
            event_controlled_timing: timing
                .event_controlled_timing
                .map(|value| Py::new(py, EventControlledTiming::from(value)).unwrap()),
        })
    }
}

impl From<&TransmissionModeTiming>
    for autosar_data_abstraction::communication::TransmissionModeTiming
{
    fn from(timing: &TransmissionModeTiming) -> Self {
        Python::attach(|py| Self {
            cyclic_timing: timing
                .cyclic_timing
                .as_ref()
                .map(|value| (&*value.borrow(py)).into()),
            event_controlled_timing: timing
                .event_controlled_timing
                .as_ref()
                .map(|value| (&*value.borrow(py)).into()),
        })
    }
}

#[pymethods]
impl TransmissionModeTiming {
    #[pyo3(signature = (*, cyclic_timing=None, event_controlled_timing=None))]
    #[pyo3(
        text_signature = "(*, cyclic_timing: Optional[CyclicTiming] = None, event_controlled_timing: Optional[EventControlledTiming] = None)"
    )]
    #[new]
    fn new(
        cyclic_timing: Option<Py<CyclicTiming>>,
        event_controlled_timing: Option<Py<EventControlledTiming>>,
    ) -> Self {
        Self {
            cyclic_timing,
            event_controlled_timing,
        }
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}

impl std::fmt::Debug for TransmissionModeTiming {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Python::attach(|py| {
            let mut ds = f.debug_struct("TransmissionModeTiming");
            match &self.cyclic_timing {
                Some(value) => {
                    ds.field("cyclic_timing", &value.borrow(py));
                }
                None => {
                    ds.field("cyclic_timing", &None::<&str>);
                }
            }
            match &self.event_controlled_timing {
                Some(value) => {
                    ds.field("event_controlled_timing", &value.borrow(py));
                }
                None => {
                    ds.field("event_controlled_timing", &None::<&str>);
                }
            }
            ds.finish()
        })
    }
}

impl PartialEq for TransmissionModeTiming {
    fn eq(&self, other: &Self) -> bool {
        Python::attach(|py|
        match (&self.cyclic_timing, &other.cyclic_timing) {
            (Some(a), Some(b)) => *a.borrow(py) == *b.borrow(py),
            (None, None) => true,
            _ => false,
        } && match (&self.event_controlled_timing, &other.event_controlled_timing) {
            (Some(a), Some(b)) => *a.borrow(py) == *b.borrow(py),
            (None, None) => true,
            _ => false,
        }
    )
    }
}

//##################################################################

/// Cyclic timing parameters for an IPDU
#[pyclass(
    eq,
    module = "autosar_data._autosar_data._abstraction._communication",
    get_all,
    set_all
)]
#[derive(Clone, PartialEq)]
pub(crate) struct CyclicTiming {
    /// period of repetition in seconds
    pub time_period: f64,
    /// delay until the first transmission of the PDU in seconds
    pub time_offset: Option<f64>,
}

impl From<autosar_data_abstraction::communication::CyclicTiming> for CyclicTiming {
    fn from(timing: autosar_data_abstraction::communication::CyclicTiming) -> Self {
        Self {
            time_period: timing.time_period,
            time_offset: timing.time_offset,
        }
    }
}

impl From<&CyclicTiming> for autosar_data_abstraction::communication::CyclicTiming {
    fn from(timing: &CyclicTiming) -> Self {
        Self {
            time_period: timing.time_period,
            time_offset: timing.time_offset,
        }
    }
}

#[pymethods]
impl CyclicTiming {
    #[pyo3(signature = (time_period, /, *, time_offset=None))]
    #[pyo3(
        text_signature = "(self, time_period: float, /, *, time_offset: Optional[float] = None)"
    )]
    #[new]
    fn new(time_period: f64, time_offset: Option<f64>) -> Self {
        Self {
            time_period,
            time_offset,
        }
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}

impl std::fmt::Debug for CyclicTiming {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("CyclicTiming");
        ds.field("time_period", &self.time_period);
        match &self.time_offset {
            Some(value) => {
                ds.field("time_offset", value);
            }
            None => {
                ds.field("time_offset", &None::<&f64>);
            }
        }
        ds.finish()
    }
}

//##################################################################

/// Event controlled timing parameters for an IPDU
#[pyclass(
    eq,
    module = "autosar_data._autosar_data._abstraction._communication",
    get_all,
    set_all
)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct EventControlledTiming {
    /// The PDU will be sent (number of repetitions + 1) times. If number of repetitions is 0, then the PDU is sent exactly once.
    pub number_of_repetitions: u32,
    /// time in seconds between two transmissions of the PDU
    pub repetition_period: Option<f64>,
}

impl From<autosar_data_abstraction::communication::EventControlledTiming>
    for EventControlledTiming
{
    fn from(timing: autosar_data_abstraction::communication::EventControlledTiming) -> Self {
        Self {
            number_of_repetitions: timing.number_of_repetitions,
            repetition_period: timing.repetition_period,
        }
    }
}

impl From<&EventControlledTiming>
    for autosar_data_abstraction::communication::EventControlledTiming
{
    fn from(timing: &EventControlledTiming) -> Self {
        Self {
            number_of_repetitions: timing.number_of_repetitions,
            repetition_period: timing.repetition_period,
        }
    }
}

#[pymethods]
impl EventControlledTiming {
    #[pyo3(signature = (number_of_repetitions, /, *, repetition_period=None))]
    #[new]
    fn new(number_of_repetitions: u32, repetition_period: Option<f64>) -> Self {
        Self {
            number_of_repetitions,
            repetition_period,
        }
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}
