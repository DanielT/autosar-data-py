use crate::abstraction::AutosarAbstractionError;
use crate::abstraction::communication::{
    FramePort, FramePortIterator, LinPhysicalChannel, PduToFrameMapping, PduToFrameMappingIterator,
    PduTriggering, PduTriggeringIterator, pyany_to_pdu,
};
use crate::{abstraction::*, *};
use autosar_data_abstraction::communication::{AbstractFrame, AbstractFrameTriggering};
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};

//##################################################################

/// An event-triggered frame on a LIN bus
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct LinEventTriggeredFrame(
    pub(crate) autosar_data_abstraction::communication::LinEventTriggeredFrame,
);

#[pymethods]
impl LinEventTriggeredFrame {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::LinEventTriggeredFrame::try_from(
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

    /// returns an iterator over all PDUs in the frame
    fn mapped_pdus(&self) -> PduToFrameMappingIterator {
        PduToFrameMappingIterator::new(self.0.mapped_pdus().map(PduToFrameMapping))
    }

    /// Iterator over all [`FrameTriggering`]s using this frame
    fn frame_triggerings(&self) -> Vec<LinFrameTriggering> {
        self.0
            .frame_triggerings()
            .into_iter()
            .map(LinFrameTriggering)
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

/// A sporadic frame on a LIN bus
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct LinSporadicFrame(
    pub(crate) autosar_data_abstraction::communication::LinSporadicFrame,
);

//##################################################################

/// An unconditional frame on a LIN bus
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct LinUnconditionalFrame(
    pub(crate) autosar_data_abstraction::communication::LinUnconditionalFrame,
);

//##################################################################

/// The frame triggering connects a frame to a physical channel
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct LinFrameTriggering(
    pub(crate) autosar_data_abstraction::communication::LinFrameTriggering,
);

#[pymethods]
impl LinFrameTriggering {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::LinFrameTriggering::try_from(
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
    fn frame(&self) -> Option<Py<PyAny>> {
        match self.0.frame()? {
            autosar_data_abstraction::communication::LinFrame::EventTriggered(
                lin_event_triggered_frame,
            ) => Python::attach(|py| {
                LinEventTriggeredFrame(lin_event_triggered_frame).into_py_any(py)
            })
            .ok(),
            autosar_data_abstraction::communication::LinFrame::Sporadic(lin_sporadic_frame) => {
                Python::attach(|py| LinSporadicFrame(lin_sporadic_frame).into_py_any(py)).ok()
            }
            autosar_data_abstraction::communication::LinFrame::Unconditional(
                lin_unconditional_frame,
            ) => {
                Python::attach(|py| LinUnconditionalFrame(lin_unconditional_frame).into_py_any(py))
                    .ok()
            }
        }
    }

    /// get the physical channel that contains this frame triggering
    #[getter]
    fn physical_channel(&self) -> PyResult<LinPhysicalChannel> {
        match self.0.physical_channel() {
            Ok(autosar_data_abstraction::communication::PhysicalChannel::Lin(channel)) => {
                Ok(LinPhysicalChannel(channel))
            }
            Ok(_) => unreachable!(), // LinFrameTriggering can only be on LinPhysicalChannel
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
