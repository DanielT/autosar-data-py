use crate::{
    abstraction::{
        datatype::{
            pyobject_to_autosar_data_type, pyobject_to_value_specification,
            value_specification_to_pyobject,
        },
        software_component::ModeDeclarationGroup,
        *,
    },
    *,
};
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

    /// Create a new `ModeGroup` in this `ModeSwitchInterface`
    ///
    /// The `ModeSwitchInterface` can only contain one mode group
    fn create_mode_group(
        &self,
        name: &str,
        mode_declaration_group: &ModeDeclarationGroup,
    ) -> PyResult<ModeGroup> {
        let mode_group = self
            .0
            .create_mode_group(name, &mode_declaration_group.0)
            .map_err(abstraction_err_to_pyerr)?;
        Ok(ModeGroup(mode_group))
    }

    /// Get the mode group for this `ModeSwitchInterface`
    #[getter]
    fn mode_group(&self) -> Option<ModeGroup> {
        self.0.mode_group().map(ModeGroup)
    }
}

//###################################################################

/// A `ModeGroup` represents a mode group in a `ModeSwitchInterface`
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct ModeGroup(pub(crate) autosar_data_abstraction::software_component::ModeGroup);

#[pymethods]
impl ModeGroup {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::ModeGroup::try_from(element.0.clone()) {
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

    /// Set the mode declaration group for this `ModeGroup`
    #[setter]
    fn set_mode_declaration_group(
        &self,
        mode_declaration_group: &ModeDeclarationGroup,
    ) -> PyResult<()> {
        self.0
            .set_mode_declaration_group(&mode_declaration_group.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the mode declaration group for this `ModeGroup`
    #[getter]
    fn mode_declaration_group(&self) -> Option<ModeDeclarationGroup> {
        self.0.mode_declaration_group().map(ModeDeclarationGroup)
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

    /// Create a new `ParameterDataPrototype` in this `ParameterInterface`
    fn create_parameter(
        &self,
        name: &str,
        data_type: &Bound<'_, PyAny>,
    ) -> PyResult<ParameterDataPrototype> {
        let data_type = pyobject_to_autosar_data_type(data_type)?;
        let parameter = self
            .0
            .create_parameter(name, &data_type)
            .map_err(abstraction_err_to_pyerr)?;
        Ok(ParameterDataPrototype(parameter))
    }

    /// Iterate over all parameters in this `ParameterInterface`
    fn parameters(&self) -> ParameterDataPrototypeIterator {
        ParameterDataPrototypeIterator::new(self.0.parameters().map(ParameterDataPrototype))
    }
}

//##################################################################

/// A `ParameterDataPrototype` defines a read-only parameter.
///
/// Typically such a parameter can be calibrated, but this is not required.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct ParameterDataPrototype(
    pub(crate) autosar_data_abstraction::software_component::ParameterDataPrototype,
);

#[pymethods]
impl ParameterDataPrototype {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::ParameterDataPrototype::try_from(
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

    /// set the init value for the data element
    #[setter]
    fn set_init_value(&self, init_value: &Bound<'_, PyAny>) -> PyResult<()> {
        let init_value = pyobject_to_value_specification(init_value)?;
        self.0
            .set_init_value(init_value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the init value for the data element
    #[getter]
    fn init_value(&self) -> Option<PyObject> {
        self.0
            .init_value()
            .and_then(|value_spec| value_specification_to_pyobject(&value_spec).ok())
    }
}

//##################################################################

iterator_wrapper!(ParameterDataPrototypeIterator, ParameterDataPrototype);

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
