use crate::abstraction::AutosarAbstractionError;
use crate::abstraction::communication::{
    CommunicationDirection, FlexrayPhysicalChannel, FramePort, FramePortIterator,
    PduToFrameMapping, PduToFrameMappingIterator, PduTriggering, PduTriggeringIterator,
    pyany_to_pdu,
};
use crate::{abstraction::*, *};
use autosar_data_abstraction::communication::{AbstractFrame, AbstractFrameTriggering};
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};

//##################################################################

/// a Flexray frame
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct FlexrayFrame(pub(crate) autosar_data_abstraction::communication::FlexrayFrame);

#[pymethods]
impl FlexrayFrame {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::FlexrayFrame::try_from(element.0.clone()) {
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

    /// returns an iterator over all PDUs in the frame
    fn mapped_pdus(&self) -> PduToFrameMappingIterator {
        PduToFrameMappingIterator::new(self.0.mapped_pdus().map(PduToFrameMapping))
    }

    /// List all `FlexrayFrameTriggering`s using this frame
    fn frame_triggerings(&self) -> Vec<FlexrayFrameTriggering> {
        self.0
            .frame_triggerings()
            .into_iter()
            .map(FlexrayFrameTriggering)
            .collect()
    }

    /// map a PDU to the frame
    #[pyo3(signature = (pdu, start_position, byte_order, /, *, update_bit=None))]
    #[pyo3(
        text_signature = "(self, pdu: Pdu, start_position: int, byte_order: ByteOrder, /, *, update_bit: Optional[int] = None)"
    )]
    fn map_pdu(
        &self,
        pdu: &Bound<'_, PyAny>,
        start_position: u32,
        byte_order: ByteOrder,
        update_bit: Option<u32>,
    ) -> PyResult<PduToFrameMapping> {
        let pdu_int = pyany_to_pdu(pdu)?;

        match self
            .0
            .map_pdu(&pdu_int, start_position, byte_order.into(), update_bit)
        {
            Ok(value) => Ok(PduToFrameMapping(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// set the length of the frame
    #[setter]
    fn set_length(&self, length: u32) -> PyResult<()> {
        self.0.set_length(length).map_err(abstraction_err_to_pyerr)
    }

    /// get the length of the frame
    #[getter]
    fn length(&self) -> Option<u32> {
        self.0.length()
    }
}

//##################################################################

/// Iterator over all [`FlexrayFrameTriggering`]s using this frame
/// map a PDU to the frame
/// The frame triggering connects a frame to a physical channel
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct FlexrayFrameTriggering(
    pub(crate) autosar_data_abstraction::communication::FlexrayFrameTriggering,
);

#[pymethods]
impl FlexrayFrameTriggering {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::FlexrayFrameTriggering::try_from(
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

    /// get the frame that is triggered
    #[getter]
    fn frame(&self) -> Option<FlexrayFrame> {
        self.0.frame().map(FlexrayFrame)
    }

    /// set the slot id for the flexray frame triggering
    #[setter]
    fn set_slot(&self, slot_id: u16) -> PyResult<()> {
        self.0.set_slot(slot_id).map_err(abstraction_err_to_pyerr)
    }

    /// get the slot id of the flexray frame triggering
    ///
    /// In a well-formed file this always returns Some(value); None should only be seen if the file is incomplete.
    #[getter]
    fn slot(&self) -> Option<u16> {
        self.0.slot()
    }

    /// set the timing of the flexray frame
    #[pyo3(signature = (timing, /))]
    #[pyo3(text_signature = "(self, timing: FlexrayCommunicationCycle, /)")]
    fn set_timing(&self, timing: &FlexrayCommunicationCycle) -> PyResult<()> {
        self.0
            .set_timing(&(*timing).into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the timing of the flexray frame
    ///
    /// In a well-formed file this should always return Some(...)
    fn timing(&self) -> Option<FlexrayCommunicationCycle> {
        self.0.timing().map(std::convert::Into::into)
    }

    /// get the FlexrayPhysicalChannel that contains this frame triggering
    #[getter]
    fn physical_channel(&self) -> PyResult<FlexrayPhysicalChannel> {
        match self.0.physical_channel() {
            Ok(value) => Ok(FlexrayPhysicalChannel(value)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// connect this frame triggering to an ECU
    ///
    /// The frame triggering may be connected to any number of ECUs.
    #[pyo3(signature = (ecu, direction, /))]
    #[pyo3(text_signature = "(self, ecu: EcuInstance, direction: CommunicationDirection, /)")]
    fn connect_to_ecu(
        &self,
        ecu: &EcuInstance,
        direction: CommunicationDirection,
    ) -> PyResult<FramePort> {
        match self.0.connect_to_ecu(&ecu.0, direction.into()) {
            Ok(value) => Ok(FramePort(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all frame ports referenced by this frame triggering
    fn frame_ports(&self) -> FramePortIterator {
        FramePortIterator::new(self.0.frame_ports().map(FramePort))
    }

    /// iterate over all PDU triggerings used by this frame triggering
    fn pdu_triggerings(&self) -> PduTriggeringIterator {
        PduTriggeringIterator::new(self.0.pdu_triggerings().map(PduTriggering))
    }
}

//##################################################################

/// The timing settings of a Flexray frame
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication",
    get_all,
    set_all
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexrayCommunicationCycle {
    /// The frame is sent every `cycle_counter` cycles
    #[pyo3(constructor = (cycle_counter, /))]
    Counter {
        /// the cycle counter
        cycle_counter: u8,
    },
    #[pyo3(constructor = (base_cycle, cycle_repetition, /))]
    /// The frame is sent every `base_cycle` cycles and repeated every `cycle_repetition` cycles
    Repetition {
        /// the base cycle - typically 64
        base_cycle: u8,
        /// the cycle repetition
        cycle_repetition: CycleRepetition,
    },
}

impl From<FlexrayCommunicationCycle>
    for autosar_data_abstraction::communication::FlexrayCommunicationCycle
{
    fn from(cycle: FlexrayCommunicationCycle) -> Self {
        match cycle {
            FlexrayCommunicationCycle::Counter { cycle_counter } => {
                autosar_data_abstraction::communication::FlexrayCommunicationCycle::Counter {
                    cycle_counter,
                }
            }
            FlexrayCommunicationCycle::Repetition {
                base_cycle,
                cycle_repetition,
            } => autosar_data_abstraction::communication::FlexrayCommunicationCycle::Repetition {
                base_cycle,
                cycle_repetition: cycle_repetition.into(),
            },
        }
    }
}

impl From<autosar_data_abstraction::communication::FlexrayCommunicationCycle>
    for FlexrayCommunicationCycle
{
    fn from(cycle: autosar_data_abstraction::communication::FlexrayCommunicationCycle) -> Self {
        match cycle {
            autosar_data_abstraction::communication::FlexrayCommunicationCycle::Counter {
                cycle_counter,
            } => FlexrayCommunicationCycle::Counter { cycle_counter },
            autosar_data_abstraction::communication::FlexrayCommunicationCycle::Repetition {
                base_cycle,
                cycle_repetition,
            } => FlexrayCommunicationCycle::Repetition {
                base_cycle,
                cycle_repetition: cycle_repetition.into(),
            },
        }
    }
}

#[pymethods]
impl FlexrayCommunicationCycle {
    fn __repr__(&self) -> String {
        match self {
            FlexrayCommunicationCycle::Counter { cycle_counter } => {
                format!("FlexrayCommunicationCycle.Counter( cycle_counter: {cycle_counter} )")
            }
            FlexrayCommunicationCycle::Repetition {
                base_cycle,
                cycle_repetition,
            } => format!(
                "FlexrayCommunicationCycle.Repetition( base_cycle: {base_cycle}, cycle_repetition: {cycle_repetition:?} )"
            ),
        }
    }
}

//##################################################################

/// The cycle repetition of a Flexray frame, from the Flexray standard
#[pyclass(
    frozen,
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CycleRepetition {
    /// 1 - sent every cycle
    C1,
    /// 2 - sent every second cycle
    C2,
    /// 4 - sent every fourth cycle
    C4,
    /// 5 - sent every fifth cycle (Flexray 3.0 only)
    C5,
    /// 8 - sent every eighth cycle
    C8,
    /// 10 - sent every tenth cycle (Flexray 3.0 only)
    C10,
    /// 16 - sent every sixteenth cycle
    C16,
    /// 20 - sent every twentieth cycle (Flexray 3.0 only)
    C20,
    /// 32 - sent every thirty-second cycle
    C32,
    /// 40 - sent every fortieth cycle (Flexray 3.0 only)
    C40,
    /// 50 - sent every fiftieth cycle (Flexray 3.0 only)
    C50,
    /// 64 - sent every sixty-fourth cycle
    C64,
}

impl From<CycleRepetition> for autosar_data_abstraction::communication::CycleRepetition {
    fn from(repetition: CycleRepetition) -> Self {
        match repetition {
            CycleRepetition::C1 => autosar_data_abstraction::communication::CycleRepetition::C1,
            CycleRepetition::C2 => autosar_data_abstraction::communication::CycleRepetition::C2,
            CycleRepetition::C4 => autosar_data_abstraction::communication::CycleRepetition::C4,
            CycleRepetition::C5 => autosar_data_abstraction::communication::CycleRepetition::C5,
            CycleRepetition::C8 => autosar_data_abstraction::communication::CycleRepetition::C8,
            CycleRepetition::C10 => autosar_data_abstraction::communication::CycleRepetition::C10,
            CycleRepetition::C16 => autosar_data_abstraction::communication::CycleRepetition::C16,
            CycleRepetition::C20 => autosar_data_abstraction::communication::CycleRepetition::C20,
            CycleRepetition::C32 => autosar_data_abstraction::communication::CycleRepetition::C32,
            CycleRepetition::C40 => autosar_data_abstraction::communication::CycleRepetition::C40,
            CycleRepetition::C50 => autosar_data_abstraction::communication::CycleRepetition::C50,
            CycleRepetition::C64 => autosar_data_abstraction::communication::CycleRepetition::C64,
        }
    }
}

impl From<autosar_data_abstraction::communication::CycleRepetition> for CycleRepetition {
    fn from(repetition: autosar_data_abstraction::communication::CycleRepetition) -> Self {
        match repetition {
            autosar_data_abstraction::communication::CycleRepetition::C1 => CycleRepetition::C1,
            autosar_data_abstraction::communication::CycleRepetition::C2 => CycleRepetition::C2,
            autosar_data_abstraction::communication::CycleRepetition::C4 => CycleRepetition::C4,
            autosar_data_abstraction::communication::CycleRepetition::C5 => CycleRepetition::C5,
            autosar_data_abstraction::communication::CycleRepetition::C8 => CycleRepetition::C8,
            autosar_data_abstraction::communication::CycleRepetition::C10 => CycleRepetition::C10,
            autosar_data_abstraction::communication::CycleRepetition::C16 => CycleRepetition::C16,
            autosar_data_abstraction::communication::CycleRepetition::C20 => CycleRepetition::C20,
            autosar_data_abstraction::communication::CycleRepetition::C32 => CycleRepetition::C32,
            autosar_data_abstraction::communication::CycleRepetition::C40 => CycleRepetition::C40,
            autosar_data_abstraction::communication::CycleRepetition::C50 => CycleRepetition::C50,
            autosar_data_abstraction::communication::CycleRepetition::C64 => CycleRepetition::C64,
        }
    }
}
