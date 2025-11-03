use crate::{
    Element,
    abstraction::{AutosarAbstractionError, EcuInstance, abstraction_err_to_pyerr},
};
use autosar_data_abstraction::{
    self, AbstractionElement, IdentifiableAbstractionElement,
    communication::AbstractCommunicationController,
};
use pyo3::prelude::*;

//##################################################################

/// An `EcuInstance` needs a `LinMaster` or `LinSlave` in order to connect to a LIN cluster.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct LinMaster(pub(crate) autosar_data_abstraction::communication::LinMaster);

#[pymethods]
impl LinMaster {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::LinMaster::try_from(element.0.clone()) {
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

    /// Get the `EcuInstance` that contains this `CommunicationController`
    #[getter]
    fn ecu_instance(&self) -> PyResult<EcuInstance> {
        match self.0.ecu_instance() {
            Ok(value) => Ok(EcuInstance(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }
}

//##################################################################

/// An `EcuInstance` needs a `LinMaster` or `LinSlave` in order to connect to a LIN cluster.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct LinSlave(pub(crate) autosar_data_abstraction::communication::LinSlave);

#[pymethods]
impl LinSlave {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::LinSlave::try_from(element.0.clone()) {
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

    /// Get the `EcuInstance` that contains this `CommunicationController`
    #[getter]
    fn ecu_instance(&self) -> PyResult<EcuInstance> {
        match self.0.ecu_instance() {
            Ok(value) => Ok(EcuInstance(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }
}
