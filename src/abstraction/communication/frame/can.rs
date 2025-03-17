use crate::abstraction::AutosarAbstractionError;
use crate::abstraction::communication::{
    CanPhysicalChannel, CommunicationDirection, FramePort, FramePortIterator, PduToFrameMapping,
    PduToFrameMappingIterator, PduTriggering, PduTriggeringIterator, pyany_to_pdu,
};
use crate::{abstraction::*, *};
use autosar_data_abstraction::communication::{AbstractFrame, AbstractFrameTriggering};
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};

//##################################################################

/// A frame on a CAN bus
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct CanFrame(pub(crate) autosar_data_abstraction::communication::CanFrame);

#[pymethods]
impl CanFrame {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::CanFrame::try_from(element.0.clone()) {
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

    /// Iterator over all [`FrameTriggering`]s using this frame
    fn frame_triggerings(&self) -> Vec<CanFrameTriggering> {
        self.0
            .frame_triggerings()
            .into_iter()
            .map(CanFrameTriggering)
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

/// The frame triggering connects a frame to a physical channel
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct CanFrameTriggering(
    pub(crate) autosar_data_abstraction::communication::CanFrameTriggering,
);

#[pymethods]
impl CanFrameTriggering {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::CanFrameTriggering::try_from(
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
    fn frame(&self) -> Option<CanFrame> {
        self.0.frame().map(CanFrame)
    }

    /// set the can id associated with this frame
    #[setter]
    fn set_identifier(&self, identifier: u32) -> PyResult<()> {
        self.0
            .set_identifier(identifier)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the can id associated with this frame triggering
    #[getter]
    fn identifier(&self) -> Option<u32> {
        self.0.identifier()
    }

    /// set the addressing mode for this frame triggering
    #[setter]
    fn set_addressing_mode(&self, addressing_mode: CanAddressingMode) -> PyResult<()> {
        self.0
            .set_addressing_mode(addressing_mode.into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the addressing mode for this frame triggering
    #[getter]
    fn addressing_mode(&self) -> Option<CanAddressingMode> {
        self.0.addressing_mode().map(CanAddressingMode::from)
    }

    /// set the frame type for this frame triggering
    #[setter]
    fn set_frame_type(&self, frame_type: CanFrameType) -> PyResult<()> {
        self.0
            .set_frame_type(frame_type.into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the frame type for this frame triggering
    #[getter]
    fn frame_type(&self) -> Option<CanFrameType> {
        self.0.frame_type().map(CanFrameType::from)
    }

    /// get the physical channel that contains this frame triggering
    #[getter]
    fn physical_channel(&self) -> PyResult<CanPhysicalChannel> {
        match self.0.physical_channel() {
            Ok(channel) => Ok(CanPhysicalChannel(channel)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// connect this frame triggering to an ECU
    ///
    /// The direction parameter specifies if the communication is incoming or outgoing
    #[pyo3(signature = (ecu, direction, /))]
    #[pyo3(text_signature = "(self, ecu: EcuInstance, direction: CommunicationDirection, /)")]
    fn connect_to_ecu(
        &self,
        ecu: &EcuInstance,
        direction: CommunicationDirection,
    ) -> PyResult<FramePort> {
        match self.0.connect_to_ecu(&ecu.0, direction.into()) {
            Ok(port) => Ok(FramePort(port)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
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

/// The addressing mode for a CAN frame
#[pyclass(
    frozen,
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanAddressingMode {
    /// Standard addressing mode: 11-bit identifier
    Standard,
    /// Extended addressing mode: 29-bit identifier
    Extended,
}

impl From<CanAddressingMode> for autosar_data_abstraction::communication::CanAddressingMode {
    fn from(mode: CanAddressingMode) -> Self {
        match mode {
            CanAddressingMode::Standard => {
                autosar_data_abstraction::communication::CanAddressingMode::Standard
            }
            CanAddressingMode::Extended => {
                autosar_data_abstraction::communication::CanAddressingMode::Extended
            }
        }
    }
}

impl From<autosar_data_abstraction::communication::CanAddressingMode> for CanAddressingMode {
    fn from(mode: autosar_data_abstraction::communication::CanAddressingMode) -> Self {
        match mode {
            autosar_data_abstraction::communication::CanAddressingMode::Standard => {
                CanAddressingMode::Standard
            }
            autosar_data_abstraction::communication::CanAddressingMode::Extended => {
                CanAddressingMode::Extended
            }
        }
    }
}

//##################################################################

/// The type of a CAN frame
#[pyclass(
    frozen,
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanFrameType {
    /// CAN 2.0 frame (max 8 bytes)
    Can20,
    /// CAN FD frame (max 64 bytes, transmitted at the `CanFD` baud rate)
    CanFd,
    /// Any CAN frame
    Any,
}

impl From<CanFrameType> for autosar_data_abstraction::communication::CanFrameType {
    fn from(frame_type: CanFrameType) -> Self {
        match frame_type {
            CanFrameType::Can20 => autosar_data_abstraction::communication::CanFrameType::Can20,
            CanFrameType::CanFd => autosar_data_abstraction::communication::CanFrameType::CanFd,
            CanFrameType::Any => autosar_data_abstraction::communication::CanFrameType::Any,
        }
    }
}

impl From<autosar_data_abstraction::communication::CanFrameType> for CanFrameType {
    fn from(frame_type: autosar_data_abstraction::communication::CanFrameType) -> Self {
        match frame_type {
            autosar_data_abstraction::communication::CanFrameType::Can20 => CanFrameType::Can20,
            autosar_data_abstraction::communication::CanFrameType::CanFd => CanFrameType::CanFd,
            autosar_data_abstraction::communication::CanFrameType::Any => CanFrameType::Any,
        }
    }
}
