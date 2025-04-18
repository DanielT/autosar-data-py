use crate::{
    Element,
    abstraction::{
        AutosarAbstractionError, abstraction_err_to_pyerr,
        communication::{
            CanAddressingMode, CanCluster, CanFrame, CanFrameTriggering, CanFrameType,
            ISignalTriggering, PduTriggering, PduTriggeringIterator, SignalTriggeringsIterator,
        },
    },
    iterator_wrapper,
};
use autosar_data_abstraction::{
    self, AbstractionElement, IdentifiableAbstractionElement,
    communication::AbstractPhysicalChannel,
};
use pyo3::prelude::*;

/// The `CanPhysicalChannel contains all of the communication on a CAN network
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct CanPhysicalChannel(
    pub(crate) autosar_data_abstraction::communication::CanPhysicalChannel,
);

#[pymethods]
impl CanPhysicalChannel {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::CanPhysicalChannel::try_from(
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

    /// get the cluster containing this physical channel
    #[getter]
    fn cluster(&self) -> PyResult<CanCluster> {
        match self.0.cluster() {
            Ok(cluster) => Ok(CanCluster(cluster)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// add a trigger for a CAN frame in this physical channel
    #[pyo3(signature = (frame, identifier, addressing_mode, frame_type, /))]
    #[pyo3(
        text_signature = "(self, frame: CanFrame, identifier: int, addressing_mode: CanAddressingMode, frame_type: CanFrameType, /)"
    )]
    fn trigger_frame(
        &self,
        frame: &CanFrame,
        identifier: u32,
        addressing_mode: CanAddressingMode,
        frame_type: CanFrameType,
    ) -> PyResult<CanFrameTriggering> {
        match self.0.trigger_frame(
            &frame.0,
            identifier,
            addressing_mode.into(),
            frame_type.into(),
        ) {
            Ok(triggering) => Ok(CanFrameTriggering(triggering)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// iterate over all frame triggerings of this physical channel
    fn frame_triggerings(&self) -> CanFrameTriggeringIterator {
        CanFrameTriggeringIterator::new(self.0.frame_triggerings().map(CanFrameTriggering))
    }

    /// iterate over all pdu triggerings of this physical channel
    fn pdu_triggerings(&self) -> PduTriggeringIterator {
        PduTriggeringIterator::new(self.0.pdu_triggerings().map(PduTriggering))
    }

    /// iterate over all ISignalTriggerings of this physical channel
    fn signal_triggerings(&self) -> SignalTriggeringsIterator {
        SignalTriggeringsIterator::new(self.0.signal_triggerings().map(ISignalTriggering))
    }
}

//##################################################################

iterator_wrapper!(CanFrameTriggeringIterator, CanFrameTriggering);
