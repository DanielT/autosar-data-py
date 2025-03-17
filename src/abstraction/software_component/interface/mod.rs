use crate::{abstraction::*, *};
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};

mod clientserver;
mod senderreceiver;

pub(crate) use clientserver::*;
pub(crate) use senderreceiver::*;

//###################################################################

/// A `ModeSwitchInterface` defines a set of modes that can be switched
///
/// Use [`ArPackage::create_mode_switch_interface`] to create a new mode switch interface
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct ModeSwitchInterface(
    pub(crate) autosar_data_abstraction::software_component::ModeSwitchInterface,
);

#[pymethods]
impl ModeSwitchInterface {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::ModeSwitchInterface::try_from(
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
}

//###################################################################

/// A `ParameterInterface` defines a set of parameters that can be accessed
///
/// Use [`ArPackage::create_parameter_interface`] to create a new parameter interface
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct ParameterInterface(
    pub(crate) autosar_data_abstraction::software_component::ParameterInterface,
);

#[pymethods]
impl ParameterInterface {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::ParameterInterface::try_from(
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
}

//##################################################################

/// An `NvDataInterface` defines non-volatile data that can be accessed through the interface
///
/// Use [`ArPackage::create_nv_data_interface`] to create a new non-volatile data interface
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct NvDataInterface(
    pub(crate) autosar_data_abstraction::software_component::NvDataInterface,
);

#[pymethods]
impl NvDataInterface {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::NvDataInterface::try_from(
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
}

//##################################################################

/// A `TriggerInterface` declares a number of triggers that can be sent by an trigger source
///
/// Use [`ArPackage::create_trigger_interface`] to create a new trigger interface
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct TriggerInterface(
    pub(crate) autosar_data_abstraction::software_component::TriggerInterface,
);

#[pymethods]
impl TriggerInterface {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::TriggerInterface::try_from(
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
}

//##################################################################

pub(crate) fn port_interface_to_pyobject(
    port_interface: autosar_data_abstraction::software_component::PortInterface,
) -> PyResult<PyObject> {
    use autosar_data_abstraction::software_component::PortInterface;
    Python::with_gil(|py| match port_interface {
        PortInterface::SenderReceiverInterface(value) => {
            SenderReceiverInterface(value).into_py_any(py)
        }
        PortInterface::ClientServerInterface(value) => ClientServerInterface(value).into_py_any(py),
        PortInterface::ModeSwitchInterface(value) => ModeSwitchInterface(value).into_py_any(py),
        PortInterface::ParameterInterface(value) => ParameterInterface(value).into_py_any(py),
        PortInterface::NvDataInterface(value) => NvDataInterface(value).into_py_any(py),
        PortInterface::TriggerInterface(value) => TriggerInterface(value).into_py_any(py),
    })
}

pub(crate) fn pyobject_to_port_interface(
    pyobject: &Bound<'_, PyAny>,
) -> PyResult<autosar_data_abstraction::software_component::PortInterface> {
    use autosar_data_abstraction::software_component::PortInterface;
    if let Ok(sender_receiver) = pyobject.extract::<SenderReceiverInterface>() {
        Ok(PortInterface::SenderReceiverInterface(sender_receiver.0))
    } else if let Ok(client_server) = pyobject.extract::<ClientServerInterface>() {
        Ok(PortInterface::ClientServerInterface(client_server.0))
    } else if let Ok(mode_switch) = pyobject.extract::<ModeSwitchInterface>() {
        Ok(PortInterface::ModeSwitchInterface(mode_switch.0))
    } else if let Ok(parameter) = pyobject.extract::<ParameterInterface>() {
        Ok(PortInterface::ParameterInterface(parameter.0))
    } else if let Ok(nv_data) = pyobject.extract::<NvDataInterface>() {
        Ok(PortInterface::NvDataInterface(nv_data.0))
    } else if let Ok(trigger) = pyobject.extract::<TriggerInterface>() {
        Ok(PortInterface::TriggerInterface(trigger.0))
    } else {
        Err(AutosarAbstractionError::new_err(
            "invalid port interface type".to_string(),
        ))
    }
}
