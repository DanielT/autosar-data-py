use pyo3::prelude::*;

mod cluster;
mod controller;
mod data_transformation;
mod frame;
mod network_management;
mod pdu;
mod physical_channel;
mod signal;
mod transport_layer;

pub(crate) use cluster::*;
pub(crate) use controller::*;
pub(crate) use data_transformation::*;
pub(crate) use frame::*;
pub(crate) use network_management::*;
pub(crate) use pdu::*;
pub(crate) use physical_channel::*;
pub(crate) use signal::*;
pub(crate) use transport_layer::*;

//#########################################################

/// The [`CommunicationDirection`] is used by the communication ports for frames, PDUs and signals
#[pyclass(
    frozen,
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommunicationDirection {
    /// The communication is incoming
    In,
    /// The communication is outgoing
    Out,
}

impl From<autosar_data_abstraction::communication::CommunicationDirection>
    for CommunicationDirection
{
    fn from(direction: autosar_data_abstraction::communication::CommunicationDirection) -> Self {
        match direction {
            autosar_data_abstraction::communication::CommunicationDirection::In => {
                CommunicationDirection::In
            }
            autosar_data_abstraction::communication::CommunicationDirection::Out => {
                CommunicationDirection::Out
            }
        }
    }
}

impl From<CommunicationDirection>
    for autosar_data_abstraction::communication::CommunicationDirection
{
    fn from(direction: CommunicationDirection) -> Self {
        match direction {
            CommunicationDirection::In => {
                autosar_data_abstraction::communication::CommunicationDirection::In
            }
            CommunicationDirection::Out => {
                autosar_data_abstraction::communication::CommunicationDirection::Out
            }
        }
    }
}
