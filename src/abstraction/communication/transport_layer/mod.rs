use crate::abstraction::AutosarAbstractionError;
use crate::{abstraction::*, *};
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};

pub(crate) mod can_tp;
pub(crate) mod doip_tp;
pub(crate) mod flexray_ar_tp;
pub(crate) mod flexray_tp;

pub(crate) use can_tp::*;
pub(crate) use doip_tp::*;
pub(crate) use flexray_ar_tp::*;
pub(crate) use flexray_tp::*;

//#########################################################

/// Represents an ECUs transport layer address on the referenced channel
///
/// The `TpAddress` element is used by `FlexrayArTpConfig` and `FlexrayTpConfig`
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct TpAddress(pub(crate) autosar_data_abstraction::communication::TpAddress);

#[pymethods]
impl TpAddress {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::TpAddress::try_from(element.0.clone()) {
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

    /// set the value of the address
    #[setter]
    fn set_address(&self, address: u32) -> PyResult<()> {
        self.0
            .set_address(address)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the value of the address
    #[getter]
    fn address(&self) -> Option<u32> {
        self.0.address()
    }
}

//#########################################################

iterator_wrapper!(TpAddressIterator, TpAddress);
