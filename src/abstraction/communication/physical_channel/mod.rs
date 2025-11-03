use crate::abstraction::AutosarAbstractionError;
use pyo3::prelude::*;

pub(crate) mod can;
pub(crate) mod ethernet;
pub(crate) mod flexray;
pub(crate) mod lin;

pub(crate) use can::*;
pub(crate) use ethernet::*;
pub(crate) use flexray::*;
pub(crate) use lin::*;

pub(crate) fn pyany_to_physical_channel(
    pyany: &Bound<'_, PyAny>,
) -> PyResult<autosar_data_abstraction::communication::PhysicalChannel> {
    if let Ok(can) = pyany.extract::<CanPhysicalChannel>() {
        Ok(autosar_data_abstraction::communication::PhysicalChannel::Can(can.0))
    } else if let Ok(ethernet) = pyany.extract::<EthernetPhysicalChannel>() {
        Ok(autosar_data_abstraction::communication::PhysicalChannel::Ethernet(ethernet.0))
    } else if let Ok(flexray) = pyany.extract::<FlexrayPhysicalChannel>() {
        Ok(autosar_data_abstraction::communication::PhysicalChannel::Flexray(flexray.0))
    } else if let Ok(lin) = pyany.extract::<LinPhysicalChannel>() {
        Ok(autosar_data_abstraction::communication::PhysicalChannel::Lin(lin.0))
    } else {
        Err(AutosarAbstractionError::new_err(
            "Expected a CanChannel, EthernetChannel, FlexRayChannel, or LinChannel",
        ))
    }
}
