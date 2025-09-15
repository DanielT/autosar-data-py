use crate::{
    Element,
    abstraction::{
        AutosarAbstractionError, abstraction_err_to_pyerr,
        software_component::{SwComponentPrototype, port_prototype_to_pyany},
    },
    iterator_wrapper,
};
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};
use pyo3::{IntoPyObjectExt, prelude::*};

//##################################################################

/// A `DelegationSwConnector` connects a port of a software component that is contained inside a `SwCompositionType` with a port of the `SwCompositionType`.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct DelegationSwConnector(
    pub(crate) autosar_data_abstraction::software_component::DelegationSwConnector,
);

#[pymethods]
impl DelegationSwConnector {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::DelegationSwConnector::try_from(
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

    /// Get the inner port of the delegation connector
    #[getter]
    fn inner_port(&self) -> Option<Py<PyAny>> {
        self.0
            .inner_port()
            .and_then(|port| port_prototype_to_pyany(port).ok())
    }

    /// Get the component containing the inner port of the delegation connector
    #[getter]
    fn inner_sw_component(&self) -> Option<SwComponentPrototype> {
        self.0.inner_sw_component().map(SwComponentPrototype)
    }

    /// Get the outer port of the delegation connector
    #[getter]
    fn outer_port(&self) -> Option<Py<PyAny>> {
        self.0
            .outer_port()
            .and_then(|port| port_prototype_to_pyany(port).ok())
    }
}

//##################################################################

/// An `AssemblySwConnector` connects ports of two `SwCompositionType`s.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct AssemblySwConnector(
    pub(crate) autosar_data_abstraction::software_component::AssemblySwConnector,
);

#[pymethods]
impl AssemblySwConnector {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::AssemblySwConnector::try_from(
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

    /// Get the provider port of the assembly connector
    #[getter]
    fn p_port(&self) -> Option<Py<PyAny>> {
        self.0
            .p_port()
            .and_then(|port| port_prototype_to_pyany(port).ok())
    }

    /// get the component containing the p_port of the assembly connector
    #[getter]
    fn p_sw_component(&self) -> Option<SwComponentPrototype> {
        self.0.p_sw_component().map(SwComponentPrototype)
    }

    /// Get the requester port of the assembly connector
    #[getter]
    fn r_port(&self) -> Option<Py<PyAny>> {
        self.0
            .r_port()
            .and_then(|port| port_prototype_to_pyany(port).ok())
    }

    /// get the component containing the r_port of the assembly connector
    #[getter]
    fn r_sw_component(&self) -> Option<SwComponentPrototype> {
        self.0.r_sw_component().map(SwComponentPrototype)
    }
}

//##################################################################

/// A `PassThroughSwConnector` connects two ports of a `SwCompositionType`.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct PassThroughSwConnector(
    pub(crate) autosar_data_abstraction::software_component::PassThroughSwConnector,
);

#[pymethods]
impl PassThroughSwConnector {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::PassThroughSwConnector::try_from(
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

    /// Get the provided port of the pass-through connector
    #[getter]
    fn p_port(&self) -> Option<Py<PyAny>> {
        self.0
            .p_port()
            .and_then(|port| port_prototype_to_pyany(port).ok())
    }

    /// Get the required port of the pass-through connector
    #[getter]
    fn r_port(&self) -> Option<Py<PyAny>> {
        self.0
            .r_port()
            .and_then(|port| port_prototype_to_pyany(port).ok())
    }
}

//##################################################################

iterator_wrapper!(
    SwConnectorIterator,
    Py<PyAny>,
    "Union[DelegationSwConnector, AssemblySwConnector, PassThroughSwConnector]"
);

//##################################################################

pub(crate) fn sw_connector_to_pyany(
    connector: autosar_data_abstraction::software_component::SwConnector,
) -> PyResult<Py<PyAny>> {
    use autosar_data_abstraction::software_component::SwConnector;
    Python::attach(|py| match connector {
        SwConnector::Delegation(connector) => DelegationSwConnector(connector).into_py_any(py),
        SwConnector::Assembly(connector) => AssemblySwConnector(connector).into_py_any(py),
        SwConnector::PassThrough(connector) => PassThroughSwConnector(connector).into_py_any(py),
    })
}
