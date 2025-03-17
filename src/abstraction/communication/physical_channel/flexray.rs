use crate::abstraction::AutosarAbstractionError;
use crate::abstraction::communication::{
    FlexrayCluster, FlexrayCommunicationCycle, FlexrayFrame, FlexrayFrameTriggering,
    ISignalTriggering, PduTriggering, PduTriggeringIterator, SignalTriggeringsIterator,
};
use crate::{abstraction::*, *};
use autosar_data_abstraction::communication::AbstractPhysicalChannel;
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};

/// the `FlexrayPhysicalChannel` represents either channel A or B of Flexray cluster
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct FlexrayPhysicalChannel(
    pub(crate) autosar_data_abstraction::communication::FlexrayPhysicalChannel,
);

#[pymethods]
impl FlexrayPhysicalChannel {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::FlexrayPhysicalChannel::try_from(
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

    /// get the channel name of a `FlexrayPhysicalChannel`
    #[getter]
    fn channel_name(&self) -> Option<FlexrayChannelName> {
        self.0.channel_name().map(std::convert::Into::into)
    }

    /// get the cluster containing this physical channel
    #[getter]
    fn cluster(&self) -> PyResult<FlexrayCluster> {
        match self.0.cluster() {
            Ok(cluster) => Ok(FlexrayCluster(cluster)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// add a trigger for a flexray frame in this physical channel
    #[pyo3(signature = (frame, slot_id, timing, /))]
    #[pyo3(
        text_signature = "(self, frame: FlexrayFrame, slot_id: int, timing: FlexrayCommunicationCycle, /)"
    )]
    fn trigger_frame(
        &self,
        frame: &FlexrayFrame,
        slot_id: u16,
        timing: &FlexrayCommunicationCycle,
    ) -> PyResult<FlexrayFrameTriggering> {
        match self.0.trigger_frame(&frame.0, slot_id, &(*timing).into()) {
            Ok(value) => Ok(FlexrayFrameTriggering(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all frame triggerings of this physical channel
    fn frame_triggerings(&self) -> FlexrayFrameTriggeringsIterator {
        FlexrayFrameTriggeringsIterator::new(self.0.frame_triggerings().map(FlexrayFrameTriggering))
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

iterator_wrapper!(FlexrayFrameTriggeringsIterator, FlexrayFrameTriggering);

//##################################################################

/// A flexray cluster may contain the channels A and/or B.
///
/// This enum is an abstraction over the \<CHANNEL-NAME\> element.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexrayChannelName {
    /// Channel A
    A,
    /// Channel B
    B,
}

impl From<FlexrayChannelName> for autosar_data_abstraction::communication::FlexrayChannelName {
    fn from(channel_name: FlexrayChannelName) -> Self {
        match channel_name {
            FlexrayChannelName::A => autosar_data_abstraction::communication::FlexrayChannelName::A,
            FlexrayChannelName::B => autosar_data_abstraction::communication::FlexrayChannelName::B,
        }
    }
}

impl From<autosar_data_abstraction::communication::FlexrayChannelName> for FlexrayChannelName {
    fn from(channel_name: autosar_data_abstraction::communication::FlexrayChannelName) -> Self {
        match channel_name {
            autosar_data_abstraction::communication::FlexrayChannelName::A => FlexrayChannelName::A,
            autosar_data_abstraction::communication::FlexrayChannelName::B => FlexrayChannelName::B,
        }
    }
}
