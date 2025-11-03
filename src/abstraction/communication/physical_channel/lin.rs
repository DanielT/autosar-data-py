use crate::{
    Element,
    abstraction::{
        AutosarAbstractionError, abstraction_err_to_pyerr,
        communication::{
            ISignalTriggering, LinCluster, LinFrameTriggering, PduTriggering,
            PduTriggeringIterator, SignalTriggeringsIterator,
        },
    },
    iterator_wrapper,
};
use autosar_data_abstraction::{
    self, AbstractionElement, IdentifiableAbstractionElement,
    communication::AbstractPhysicalChannel,
};
use pyo3::prelude::*;

//##################################################################

/// The `LinPhysicalChannel` contains all of the communication on a LIN network
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct LinPhysicalChannel(
    pub(crate) autosar_data_abstraction::communication::LinPhysicalChannel,
);

#[pymethods]
impl LinPhysicalChannel {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::LinPhysicalChannel::try_from(
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
    fn cluster(&self) -> PyResult<LinCluster> {
        match self.0.cluster() {
            Ok(cluster) => Ok(LinCluster(cluster)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// iterate over all frame triggerings of this physical channel
    fn frame_triggerings(&self) -> LinFrameTriggeringIterator {
        LinFrameTriggeringIterator::new(self.0.frame_triggerings().map(LinFrameTriggering))
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

iterator_wrapper!(LinFrameTriggeringIterator, LinFrameTriggering);
