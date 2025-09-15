use crate::{
    abstraction::{
        AutosarAbstractionError, Element, abstraction_err_to_pyerr,
        software_component::{port_interface_to_pyany, sw_component_type_to_pyany},
    },
    iterator_wrapper,
};
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};
use pyo3::{IntoPyObjectExt, prelude::*};

//##################################################################

/// `RPortPrototype` represents a required port prototype
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct RPortPrototype(
    pub(crate) autosar_data_abstraction::software_component::RPortPrototype,
);

#[pymethods]
impl RPortPrototype {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::RPortPrototype::try_from(
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

    /// Get the port interface of the port prototype
    #[getter]
    fn port_interface(&self) -> PyResult<Py<PyAny>> {
        match self.0.port_interface() {
            Ok(value) => port_interface_to_pyany(value),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// Get the component type containing the port prototype
    #[getter]
    fn component_type(&self) -> PyResult<Py<PyAny>> {
        match self.0.component_type() {
            Ok(value) => sw_component_type_to_pyany(value),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }
}

//##################################################################

/// `PPortPrototype` represents a provided port prototype
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct PPortPrototype(
    pub(crate) autosar_data_abstraction::software_component::PPortPrototype,
);

#[pymethods]
impl PPortPrototype {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::PPortPrototype::try_from(
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

    /// Get the port interface of the port prototype
    #[getter]
    fn port_interface(&self) -> PyResult<Py<PyAny>> {
        match self.0.port_interface() {
            Ok(value) => port_interface_to_pyany(value),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// Get the component type containing the port prototype
    #[getter]
    fn component_type(&self) -> PyResult<Py<PyAny>> {
        match self.0.component_type() {
            Ok(value) => sw_component_type_to_pyany(value),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }
}

//##################################################################

/// `PRPortPrototype` represents a provided and required port prototype
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct PRPortPrototype(
    pub(crate) autosar_data_abstraction::software_component::PRPortPrototype,
);

#[pymethods]
impl PRPortPrototype {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::PRPortPrototype::try_from(
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

    /// Get the port interface of the port prototype
    #[getter]
    fn port_interface(&self) -> PyResult<Py<PyAny>> {
        match self.0.port_interface() {
            Ok(value) => port_interface_to_pyany(value),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// Get the component type containing the port prototype
    #[getter]
    fn component_type(&self) -> PyResult<Py<PyAny>> {
        match self.0.component_type() {
            Ok(value) => sw_component_type_to_pyany(value),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }
}

//##################################################################

iterator_wrapper!(PortPrototypeIterator, Py<PyAny>, "PortPrototype");

//##################################################################

pub(crate) fn pyany_to_port_prototype(
    pyobject: &Bound<'_, PyAny>,
) -> PyResult<autosar_data_abstraction::software_component::PortPrototype> {
    if let Ok(rport_proto) = pyobject.extract::<RPortPrototype>() {
        Ok(autosar_data_abstraction::software_component::PortPrototype::R(rport_proto.0))
    } else if let Ok(pport_proto) = pyobject.extract::<PPortPrototype>() {
        Ok(autosar_data_abstraction::software_component::PortPrototype::P(pport_proto.0))
    } else if let Ok(prport_proto) = pyobject.extract::<PRPortPrototype>() {
        Ok(autosar_data_abstraction::software_component::PortPrototype::PR(prport_proto.0))
    } else {
        Err(AutosarAbstractionError::new_err(
            "Invalid port prototype type".to_string(),
        ))
    }
}

pub(crate) fn port_prototype_to_pyany(
    port_proto: autosar_data_abstraction::software_component::PortPrototype,
) -> PyResult<Py<PyAny>> {
    use autosar_data_abstraction::software_component::PortPrototype;
    Python::attach(|py| match port_proto {
        PortPrototype::R(value) => RPortPrototype(value).into_py_any(py),
        PortPrototype::P(value) => PPortPrototype(value).into_py_any(py),
        PortPrototype::PR(value) => PRPortPrototype(value).into_py_any(py),
    })
}

//##################################################################

/// `PortGroup` represents a group of ports
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct PortGroup(pub(crate) autosar_data_abstraction::software_component::PortGroup);

#[pymethods]
impl PortGroup {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::PortGroup::try_from(element.0.clone()) {
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
