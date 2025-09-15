use crate::{
    Element,
    abstraction::{
        AutosarAbstractionError, EcuInstance, System, abstraction_err_to_pyerr,
        communication::SystemSignal,
        software_component::{
            RootSwCompositionPrototype, SwComponentPrototype, VariableDataPrototype,
            pyany_to_port_prototype,
        },
    },
};
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};
use pyo3::prelude::*;

//##################################################################

/// A `SystemMapping` contains mappings in the `System`
///
/// it contains mappings between SWCs and ECUs, as well as between ports and signals
#[pyclass(frozen, eq, module = "autosar_data._autosar_data._abstraction")]
#[derive(Clone, PartialEq)]
pub(crate) struct SystemMapping(pub(crate) autosar_data_abstraction::SystemMapping);

#[pymethods]
impl SystemMapping {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::SystemMapping::try_from(element.0.clone()) {
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

    /// get the system that contains this mapping
    #[getter]
    fn system(&self) -> PyResult<System> {
        match self.0.system() {
            Ok(value) => Ok(System(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new mapping between a SWC and an ECU
    #[pyo3(signature = (name, component_prototype, ecu, /))]
    #[pyo3(
        text_signature = "(self, name: str, component_prototype: SwComponentPrototype, ecu: EcuInstance, /)"
    )]
    fn map_swc_to_ecu(
        &self,
        name: &str,
        component_prototype: &SwComponentPrototype,
        ecu: &EcuInstance,
    ) -> PyResult<SwcToEcuMapping> {
        match self.0.map_swc_to_ecu(name, &component_prototype.0, &ecu.0) {
            Ok(value) => Ok(SwcToEcuMapping(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new mapping between a sender/receiver port and a signal
    ///
    /// `signal`: the system signal that the port is mapped to
    ///
    /// `data_element`: the data element that is mapped to the signal
    ///
    /// `port_prototype`: the port prototype that contains the data element
    ///
    /// `context_components`: a list of component prototypes from the root up to the component that directly contains the port.
    /// This list may be empty, or it could only contain the final application component prototype containing the port.
    ///
    /// `root_composition_prototype`: the root composition prototype that contains the `swc_prototype`.
    /// Rarely required, but may be needed if multiple root compositions use the same composition/component hierarchy.
    #[pyo3(signature = (signal, data_element, port_prototype, context_components, /, *, root_composition_prototype=None))]
    #[pyo3(
        text_signature = "(self, signal: SystemSignal, data_element: VariableDataPrototype, port_prototype: PortPrototype, context_components: List[SwComponentPrototype], /, *, root_composition_prototype: Optional[RootSwCompositionPrototype] = None)"
    )]
    fn map_sender_receiver_to_signal(
        &self,
        signal: &SystemSignal,
        data_element: &VariableDataPrototype,
        port_prototype: &Bound<'_, PyAny>,
        context_components: Vec<SwComponentPrototype>,
        root_composition_prototype: Option<&RootSwCompositionPrototype>,
    ) -> PyResult<()> {
        let port_prototype = pyany_to_port_prototype(port_prototype)?;
        let context_components: Vec<_> = context_components.iter().map(|c| &c.0).collect();
        self.0
            .map_sender_receiver_to_signal(
                &signal.0,
                &data_element.0,
                &port_prototype,
                &context_components,
                root_composition_prototype.map(|r| &r.0),
            )
            .map_err(abstraction_err_to_pyerr)
    }
}

//##################################################################

/// A `SwcToEcuMapping` contains a mapping between a `SwComponentPrototype` and an `EcuInstance`
#[pyclass(frozen, eq, module = "autosar_data._autosar_data._abstraction")]
#[derive(Clone, PartialEq)]
pub(crate) struct SwcToEcuMapping(pub(crate) autosar_data_abstraction::SwcToEcuMapping);

#[pymethods]
impl SwcToEcuMapping {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::SwcToEcuMapping::try_from(element.0.clone()) {
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

    /// get the component prototype that is mapped here
    #[getter]
    fn target_component(&self) -> Option<SwComponentPrototype> {
        self.0.target_component().map(SwComponentPrototype)
    }

    /// get the ECU instance which is the target of this mapping
    #[getter]
    fn ecu_instance(&self) -> Option<EcuInstance> {
        self.0.ecu_instance().map(EcuInstance)
    }
}
