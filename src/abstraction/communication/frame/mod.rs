use crate::abstraction::AutosarAbstractionError;
use crate::abstraction::communication::{CommunicationDirection, pdu_to_pyany};
use crate::{abstraction::*, *};
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};

mod can;
mod flexray;
mod lin;

pub(crate) use can::*;
pub(crate) use flexray::*;
pub(crate) use lin::*;

//##################################################################

/// `PduToFrameMapping` connects a PDU to a frame
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct PduToFrameMapping(
    pub(crate) autosar_data_abstraction::communication::PduToFrameMapping,
);

#[pymethods]
impl PduToFrameMapping {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::PduToFrameMapping::try_from(
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

    /// Reference to the PDU that is mapped into the frame. The PDU reference is mandatory.
    #[getter]
    fn pdu(&self) -> Option<Py<PyAny>> {
        self.0.pdu().and_then(|pdu| pdu_to_pyany(&pdu).ok())
    }

    /// set the byte order of the data in the PDU.
    ///
    /// All `PduToFrameMappings` within a frame must have the same byte order.
    /// PDUs may not use the byte order value `Opaque`.
    ///
    /// Note: If the byte order is swapped, then the start position must be adjusted accordingly.
    #[setter]
    fn set_byte_order(&self, byte_order: ByteOrder) -> PyResult<()> {
        self.0
            .set_byte_order(byte_order.into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the byte order of the data in the PDU.
    ///
    /// All `PduToFrameMappings` within a frame must have the same byte order.
    /// PDUs my not use the byte order value `Opaque`.
    #[getter]
    fn byte_order(&self) -> Option<ByteOrder> {
        self.0.byte_order().map(std::convert::Into::into)
    }

    /// set the start position of the PDU data within the frame (bit position).
    ///
    /// PDUs are byte aligned.
    /// For little-endian data the values 0, 8, 16, ... are allowed;
    /// for big-endian data the values 7, 15, 23, ... are allowed.
    ///
    /// Note: if you intend to change both the byte order and the start position, then you should change the byte order first.
    /// New values set here must match the configured byte order.
    #[setter]
    fn set_start_position(&self, start_position: u32) -> PyResult<()> {
        self.0
            .set_start_position(start_position)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Start position of the PDU data within the frame (bit position). The start position is mandatory.
    ///
    /// PDUs are byte aligned.
    /// For little-endian data the values 0, 8, 16, ... are allowed;
    /// for big-endian data the value 7, 15, 23, ... are allowed.
    #[getter]
    fn start_position(&self) -> Option<u32> {
        self.0.start_position()
    }

    /// set or clear the bit position of the update bit for the mapped PDU.
    #[setter]
    fn set_update_bit(&self, update_bit: Option<u32>) -> PyResult<()> {
        self.0
            .set_update_bit(update_bit)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Bit position of the update bit for the mapped PDU. Not all PDUs use an update bit.
    #[getter]
    fn update_bit(&self) -> Option<u32> {
        self.0.update_bit()
    }
}

//##################################################################

iterator_wrapper!(PduToFrameMappingIterator, PduToFrameMapping);

//##################################################################

/// The `FramePort` allows an ECU to send or receive a frame
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct FramePort(pub(crate) autosar_data_abstraction::communication::FramePort);

#[pymethods]
impl FramePort {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::FramePort::try_from(element.0.clone()) {
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

    /// get the ECU instance that contains this frame port
    #[getter]
    fn ecu(&self) -> PyResult<EcuInstance> {
        match self.0.ecu() {
            Ok(ecu) => Ok(EcuInstance(ecu)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// set the communication direction of the frame port
    #[setter]
    fn set_communication_direction(&self, direction: CommunicationDirection) -> PyResult<()> {
        self.0
            .set_communication_direction(direction.into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the communication direction of the frame port
    #[getter]
    fn communication_direction(&self) -> Option<CommunicationDirection> {
        self.0
            .communication_direction()
            .map(std::convert::Into::into)
    }
}

//##################################################################

iterator_wrapper!(FramePortIterator, FramePort);
