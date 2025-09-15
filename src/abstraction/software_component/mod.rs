use crate::{abstraction::*, *};
use autosar_data_abstraction::{
    self, AbstractionElement, IdentifiableAbstractionElement,
    software_component::{AbstractSwComponentType, AtomicSwComponentType},
};

mod connector;
mod interface;
mod internal_behavior;
mod mode;
mod port;

pub(crate) use connector::*;
pub(crate) use interface::*;
pub(crate) use internal_behavior::*;
pub(crate) use mode::*;
pub(crate) use port::*;

//##################################################################

/// A `CompositionSwComponentType` is a software component that contains other software components
///
/// Use [`ArPackage::create_composition_sw_component_type`] to create a new composition sw component type.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct CompositionSwComponentType(
    pub(crate) autosar_data_abstraction::software_component::CompositionSwComponentType,
);

#[pymethods]
impl CompositionSwComponentType {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::CompositionSwComponentType::try_from(
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

    /// check if the composition is a parent (or grand-parent, etc.) of the component
    #[pyo3(signature = (other, /))]
    #[pyo3(text_signature = "(self, other: SwComponentType, /)")]
    fn is_parent_of(&self, other: &Bound<'_, PyAny>) -> bool {
        let other = pyany_to_sw_component_type(other).unwrap();
        self.0.is_parent_of(&other)
    }

    /// create a component of type `component_type` in the composition
    ///
    /// It is not allowed to form cycles in the composition hierarchy, and this will return an error
    #[pyo3(signature = (name, component_type, /))]
    #[pyo3(text_signature = "(self, name: str, component_type: SwComponentType, /)")]
    fn create_component(
        &self,
        name: &str,
        component_type: &Bound<'_, PyAny>,
    ) -> PyResult<SwComponentPrototype> {
        let component_type = pyany_to_sw_component_type(component_type)?;
        match self.0.create_component(name, &component_type) {
            Ok(value) => Ok(SwComponentPrototype(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// get an iterator over the components of the composition
    fn components(&self) -> SwComponentPrototypeIterator {
        SwComponentPrototypeIterator::new(self.0.components().map(SwComponentPrototype))
    }

    /// create a new delegation connector between an inner port and an outer port
    ///
    /// The two ports must be compatible.
    #[pyo3(signature = (name, inner_port, inner_sw_prototype, outer_port, /))]
    #[pyo3(
        text_signature = "(self, name: str, inner_port: PortPrototype, inner_sw_prototype: SwComponentPrototype, outer_port: PortPrototype, /)"
    )]
    fn create_delegation_connector(
        &self,
        name: &str,
        inner_port: &Bound<'_, PyAny>,
        inner_sw_prototype: &SwComponentPrototype,
        outer_port: &Bound<'_, PyAny>,
    ) -> PyResult<DelegationSwConnector> {
        let inner_port = pyany_to_port_prototype(inner_port)?;
        let outer_port = pyany_to_port_prototype(outer_port)?;
        match self.0.create_delegation_connector(
            name,
            &inner_port,
            &inner_sw_prototype.0,
            &outer_port,
        ) {
            Ok(value) => Ok(DelegationSwConnector(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new delegation connector between an inner port and an outer port
    /// this is the actual implementation of the public method, but without the generic parameters
    /// create a new assembly connector between two ports of contained software components
    ///
    /// The two ports must be compatible.
    #[pyo3(signature = (name, port_1, sw_prototype_1, port_2, sw_prototype_2, /))]
    #[pyo3(
        text_signature = "(self, name: str, port_1: PortPrototype, sw_prototype_1: SwComponentPrototype, port_2: PortPrototype, sw_prototype_2: SwComponentPrototype, /)"
    )]
    fn create_assembly_connector(
        &self,
        name: &str,
        port_1: &Bound<'_, PyAny>,
        sw_prototype_1: &SwComponentPrototype,
        port_2: &Bound<'_, PyAny>,
        sw_prototype_2: &SwComponentPrototype,
    ) -> PyResult<AssemblySwConnector> {
        let port_1 = pyany_to_port_prototype(port_1)?;
        let port_2 = pyany_to_port_prototype(port_2)?;
        match self.0.create_assembly_connector(
            name,
            &port_1,
            &sw_prototype_1.0,
            &port_2,
            &sw_prototype_2.0,
        ) {
            Ok(value) => Ok(AssemblySwConnector(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new passthrough connector between two outer ports of the composition
    ///
    /// The two ports must be compatible.
    #[pyo3(signature = (name, port_1, port_2, /))]
    #[pyo3(text_signature = "(self, name: str, port_1: PortPrototype, port_2: PortPrototype, /)")]
    fn create_pass_through_connector(
        &self,
        name: &str,
        port_1: &Bound<'_, PyAny>,
        port_2: &Bound<'_, PyAny>,
    ) -> PyResult<PassThroughSwConnector> {
        let port_1 = pyany_to_port_prototype(port_1)?;
        let port_2 = pyany_to_port_prototype(port_2)?;
        match self.0.create_pass_through_connector(name, &port_1, &port_2) {
            Ok(value) => Ok(PassThroughSwConnector(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all connectors
    fn connectors(&self) -> SwConnectorIterator {
        SwConnectorIterator::new(
            self.0
                .connectors()
                .filter_map(|connector| sw_connector_to_pyany(connector).ok()),
        )
    }

    // ------ AbstractSwComponentType ------

    /// list of all instances of the component type
    fn instances(&self) -> Vec<Py<PyAny>> {
        self.0
            .instances()
            .into_iter()
            .filter_map(|instance| component_prototype_to_pyany(instance).ok())
            .collect()
    }

    /// list all compositions containing instances of the component type
    fn parent_compositions(&self) -> Vec<CompositionSwComponentType> {
        self.0
            .parent_compositions()
            .into_iter()
            .map(CompositionSwComponentType)
            .collect()
    }

    /// create a new required port with the given name and port interface
    #[pyo3(signature = (name, port_interface, /))]
    #[pyo3(text_signature = "(self, name: str, port_interface: PortInterface, /)")]
    fn create_r_port(
        &self,
        name: &str,
        port_interface: &Bound<'_, PyAny>,
    ) -> PyResult<RPortPrototype> {
        let port_interface = pyany_to_port_interface(port_interface)?;
        match self.0.create_r_port(name, &port_interface) {
            Ok(value) => Ok(RPortPrototype(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new provided port with the given name and port interface
    #[pyo3(signature = (name, port_interface, /))]
    #[pyo3(text_signature = "(self, name: str, port_interface: PortInterface, /)")]
    fn create_p_port(
        &self,
        name: &str,
        port_interface: &Bound<'_, PyAny>,
    ) -> PyResult<PPortPrototype> {
        let port_interface = pyany_to_port_interface(port_interface)?;
        match self.0.create_p_port(name, &port_interface) {
            Ok(value) => Ok(PPortPrototype(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new provided required port with the given name and port interface
    #[pyo3(signature = (name, port_interface, /))]
    #[pyo3(text_signature = "(self, name: str, port_interface: PortInterface, /)")]
    fn create_pr_port(
        &self,
        name: &str,
        port_interface: &Bound<'_, PyAny>,
    ) -> PyResult<PRPortPrototype> {
        let port_interface = pyany_to_port_interface(port_interface)?;
        match self.0.create_pr_port(name, &port_interface) {
            Ok(value) => Ok(PRPortPrototype(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// get an iterator over the ports of the component
    fn ports(&self) -> PortPrototypeIterator {
        PortPrototypeIterator::new(
            self.0
                .ports()
                .filter_map(|port| port_prototype_to_pyany(port).ok()),
        )
    }

    /// create a new port group
    #[pyo3(signature = (name, /))]
    #[pyo3(text_signature = "(self, name: str, /)")]
    fn create_port_group(&self, name: &str) -> PyResult<PortGroup> {
        match self.0.create_port_group(name) {
            Ok(value) => Ok(PortGroup(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }
}

//##################################################################

/// An `ApplicationSwComponentType` is a software component that provides application functionality
///
/// Use [`ArPackage::create_application_sw_component_type`] to create a new application sw component type.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct ApplicationSwComponentType(
    pub(crate) autosar_data_abstraction::software_component::ApplicationSwComponentType,
);

#[pymethods]
impl ApplicationSwComponentType {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::ApplicationSwComponentType::try_from(
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

    // ------ AbstractSwComponentType ------

    /// list all instances of the component type
    fn instances(&self) -> Vec<Py<PyAny>> {
        self.0
            .instances()
            .into_iter()
            .filter_map(|instance| component_prototype_to_pyany(instance).ok())
            .collect()
    }

    /// list all compositions containing instances of the component type
    fn parent_compositions(&self) -> Vec<CompositionSwComponentType> {
        self.0
            .parent_compositions()
            .into_iter()
            .map(CompositionSwComponentType)
            .collect()
    }

    /// create a new required port with the given name and port interface
    #[pyo3(signature = (name, port_interface, /))]
    #[pyo3(text_signature = "(self, name: str, port_interface: PortInterface, /)")]
    fn create_r_port(
        &self,
        name: &str,
        port_interface: &Bound<'_, PyAny>,
    ) -> PyResult<RPortPrototype> {
        let port_interface = pyany_to_port_interface(port_interface)?;
        match self.0.create_r_port(name, &port_interface) {
            Ok(value) => Ok(RPortPrototype(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new provided port with the given name and port interface
    #[pyo3(signature = (name, port_interface, /))]
    #[pyo3(text_signature = "(self, name: str, port_interface: PortInterface, /)")]
    fn create_p_port(
        &self,
        name: &str,
        port_interface: &Bound<'_, PyAny>,
    ) -> PyResult<PPortPrototype> {
        let port_interface = pyany_to_port_interface(port_interface)?;
        match self.0.create_p_port(name, &port_interface) {
            Ok(value) => Ok(PPortPrototype(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new provided required port with the given name and port interface
    #[pyo3(signature = (name, port_interface, /))]
    #[pyo3(text_signature = "(self, name: str, port_interface: PortInterface, /)")]
    fn create_pr_port(
        &self,
        name: &str,
        port_interface: &Bound<'_, PyAny>,
    ) -> PyResult<PRPortPrototype> {
        let port_interface = pyany_to_port_interface(port_interface)?;
        match self.0.create_pr_port(name, &port_interface) {
            Ok(value) => Ok(PRPortPrototype(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// get an iterator over the ports of the component
    fn ports(&self) -> PortPrototypeIterator {
        PortPrototypeIterator::new(
            self.0
                .ports()
                .filter_map(|port| port_prototype_to_pyany(port).ok()),
        )
    }

    /// create a new port group
    #[pyo3(signature = (name, /))]
    #[pyo3(text_signature = "(self, name: str, /)")]
    fn create_port_group(&self, name: &str) -> PyResult<PortGroup> {
        match self.0.create_port_group(name) {
            Ok(value) => Ok(PortGroup(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create an SwcInternalBehavior for the component
    ///
    /// A component can have only one internal behavior, but since the internal behavior is a variation point,
    /// more than one internal behavior can be created. In this case the variation point settings must ensure that only one
    /// internal behavior is active.
    #[pyo3(signature = (name, /))]
    #[pyo3(text_signature = "(self, name: str, /)")]
    fn create_swc_internal_behavior(&self, name: &str) -> PyResult<SwcInternalBehavior> {
        match self.0.create_swc_internal_behavior(name) {
            Ok(value) => Ok(SwcInternalBehavior(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all swc internal behaviors - typically zero or one
    fn swc_internal_behaviors(&self) -> SwcInternalBehaviorIterator {
        SwcInternalBehaviorIterator::new(self.0.swc_internal_behaviors().map(SwcInternalBehavior))
    }
}

//##################################################################

/// A `ComplexDeviceDriverSwComponentType` is a software component that provides complex device driver functionality
///
/// Use [`ArPackage::create_complex_device_driver_sw_component_type`] to create a new complex device driver sw component type.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct ComplexDeviceDriverSwComponentType(
    pub(crate) autosar_data_abstraction::software_component::ComplexDeviceDriverSwComponentType,
);

#[pymethods]
impl ComplexDeviceDriverSwComponentType {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::ComplexDeviceDriverSwComponentType::try_from(
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

    // ------ AbstractSwComponentType ------

    /// list of all instances of the component type
    fn instances(&self) -> Vec<Py<PyAny>> {
        self.0
            .instances()
            .into_iter()
            .filter_map(|instance| component_prototype_to_pyany(instance).ok())
            .collect()
    }

    /// iterator over all compositions containing instances of the component type
    fn parent_compositions(&self) -> Vec<CompositionSwComponentType> {
        self.0
            .parent_compositions()
            .into_iter()
            .map(CompositionSwComponentType)
            .collect()
    }

    /// create a new required port with the given name and port interface
    #[pyo3(signature = (name, port_interface, /))]
    #[pyo3(text_signature = "(self, name: str, port_interface: PortInterface, /)")]
    fn create_r_port(
        &self,
        name: &str,
        port_interface: &Bound<'_, PyAny>,
    ) -> PyResult<RPortPrototype> {
        let port_interface = pyany_to_port_interface(port_interface)?;
        match self.0.create_r_port(name, &port_interface) {
            Ok(value) => Ok(RPortPrototype(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new provided port with the given name and port interface
    #[pyo3(signature = (name, port_interface, /))]
    #[pyo3(text_signature = "(self, name: str, port_interface: PortInterface, /)")]
    fn create_p_port(
        &self,
        name: &str,
        port_interface: &Bound<'_, PyAny>,
    ) -> PyResult<PPortPrototype> {
        let port_interface = pyany_to_port_interface(port_interface)?;
        match self.0.create_p_port(name, &port_interface) {
            Ok(value) => Ok(PPortPrototype(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new provided required port with the given name and port interface
    #[pyo3(signature = (name, port_interface, /))]
    #[pyo3(text_signature = "(self, name: str, port_interface: PortInterface, /)")]
    fn create_pr_port(
        &self,
        name: &str,
        port_interface: &Bound<'_, PyAny>,
    ) -> PyResult<PRPortPrototype> {
        let port_interface = pyany_to_port_interface(port_interface)?;
        match self.0.create_pr_port(name, &port_interface) {
            Ok(value) => Ok(PRPortPrototype(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// get an iterator over the ports of the component
    fn ports(&self) -> PortPrototypeIterator {
        PortPrototypeIterator::new(
            self.0
                .ports()
                .filter_map(|port| port_prototype_to_pyany(port).ok()),
        )
    }

    /// create a new port group
    #[pyo3(signature = (name, /))]
    #[pyo3(text_signature = "(self, name: str, /)")]
    fn create_port_group(&self, name: &str) -> PyResult<PortGroup> {
        match self.0.create_port_group(name) {
            Ok(value) => Ok(PortGroup(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create an SwcInternalBehavior for the component
    ///
    /// A component can have only one internal behavior, but since the internal behavior is a variation point,
    /// more than one internal behavior can be created. In this case the variation point settings must ensure that only one
    /// internal behavior is active.
    #[pyo3(signature = (name, /))]
    #[pyo3(text_signature = "(self, name: str, /)")]
    fn create_swc_internal_behavior(&self, name: &str) -> PyResult<SwcInternalBehavior> {
        match self.0.create_swc_internal_behavior(name) {
            Ok(value) => Ok(SwcInternalBehavior(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all swc internal behaviors - typically zero or one
    fn swc_internal_behaviors(&self) -> SwcInternalBehaviorIterator {
        SwcInternalBehaviorIterator::new(self.0.swc_internal_behaviors().map(SwcInternalBehavior))
    }
}

//##################################################################

/// `ServiceSwComponentType` is used for configuring services for a given ECU. Instances of this class should only
/// be created in ECU Configuration phase for the specific purpose of the service configuration.
///
/// Use [`ArPackage::create_service_sw_component_type`] to create a new service sw component type.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct ServiceSwComponentType(
    pub(crate) autosar_data_abstraction::software_component::ServiceSwComponentType,
);

#[pymethods]
impl ServiceSwComponentType {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::ServiceSwComponentType::try_from(
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

    // ------ AbstractSwComponentType ------

    /// list all the instances of the component type
    fn instances(&self) -> Vec<Py<PyAny>> {
        self.0
            .instances()
            .into_iter()
            .filter_map(|instance| component_prototype_to_pyany(instance).ok())
            .collect()
    }

    /// list all compositions containing instances of the component type
    fn parent_compositions(&self) -> Vec<CompositionSwComponentType> {
        self.0
            .parent_compositions()
            .into_iter()
            .map(CompositionSwComponentType)
            .collect()
    }

    /// create a new required port with the given name and port interface
    #[pyo3(signature = (name, port_interface, /))]
    #[pyo3(text_signature = "(self, name: str, port_interface: PortInterface, /)")]
    fn create_r_port(
        &self,
        name: &str,
        port_interface: &Bound<'_, PyAny>,
    ) -> PyResult<RPortPrototype> {
        let port_interface = pyany_to_port_interface(port_interface)?;
        match self.0.create_r_port(name, &port_interface) {
            Ok(value) => Ok(RPortPrototype(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new provided port with the given name and port interface
    #[pyo3(signature = (name, port_interface, /))]
    #[pyo3(text_signature = "(self, name: str, port_interface: PortInterface, /)")]
    fn create_p_port(
        &self,
        name: &str,
        port_interface: &Bound<'_, PyAny>,
    ) -> PyResult<PPortPrototype> {
        let port_interface = pyany_to_port_interface(port_interface)?;
        match self.0.create_p_port(name, &port_interface) {
            Ok(value) => Ok(PPortPrototype(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new provided required port with the given name and port interface
    #[pyo3(signature = (name, port_interface, /))]
    #[pyo3(text_signature = "(self, name: str, port_interface: PortInterface, /)")]
    fn create_pr_port(
        &self,
        name: &str,
        port_interface: &Bound<'_, PyAny>,
    ) -> PyResult<PRPortPrototype> {
        let port_interface = pyany_to_port_interface(port_interface)?;
        match self.0.create_pr_port(name, &port_interface) {
            Ok(value) => Ok(PRPortPrototype(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// get an iterator over the ports of the component
    fn ports(&self) -> PortPrototypeIterator {
        PortPrototypeIterator::new(
            self.0
                .ports()
                .filter_map(|port| port_prototype_to_pyany(port).ok()),
        )
    }

    /// create a new port group
    #[pyo3(signature = (name, /))]
    #[pyo3(text_signature = "(self, name: str, /)")]
    fn create_port_group(&self, name: &str) -> PyResult<PortGroup> {
        match self.0.create_port_group(name) {
            Ok(value) => Ok(PortGroup(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create an SwcInternalBehavior for the component
    ///
    /// A component can have only one internal behavior, but since the internal behavior is a variation point,
    /// more than one internal behavior can be created. In this case the variation point settings must ensure that only one
    /// internal behavior is active.
    #[pyo3(signature = (name, /))]
    #[pyo3(text_signature = "(self, name: str, /)")]
    fn create_swc_internal_behavior(&self, name: &str) -> PyResult<SwcInternalBehavior> {
        match self.0.create_swc_internal_behavior(name) {
            Ok(value) => Ok(SwcInternalBehavior(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all swc internal behaviors - typically zero or one
    fn swc_internal_behaviors(&self) -> SwcInternalBehaviorIterator {
        SwcInternalBehaviorIterator::new(self.0.swc_internal_behaviors().map(SwcInternalBehavior))
    }
}

//##################################################################

/// `SensorActuatorSwComponentType` is used to connect sensor/acutator devices to the ECU configuration
///
/// Use [`ArPackage::create_sensor_actuator_sw_component_type`] to create a new sensor/actuator sw component type.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct SensorActuatorSwComponentType(
    pub(crate) autosar_data_abstraction::software_component::SensorActuatorSwComponentType,
);

#[pymethods]
impl SensorActuatorSwComponentType {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::SensorActuatorSwComponentType::try_from(
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

    // ------ AbstractSwComponentType ------

    /// list all instances of the component type
    fn instances(&self) -> Vec<Py<PyAny>> {
        self.0
            .instances()
            .into_iter()
            .filter_map(|instance| component_prototype_to_pyany(instance).ok())
            .collect()
    }

    /// list all compositions containing instances of the component type
    fn parent_compositions(&self) -> Vec<CompositionSwComponentType> {
        self.0
            .parent_compositions()
            .into_iter()
            .map(CompositionSwComponentType)
            .collect()
    }

    /// create a new required port with the given name and port interface
    #[pyo3(signature = (name, port_interface, /))]
    #[pyo3(text_signature = "(self, name: str, port_interface: PortInterface, /)")]
    fn create_r_port(
        &self,
        name: &str,
        port_interface: &Bound<'_, PyAny>,
    ) -> PyResult<RPortPrototype> {
        let port_interface = pyany_to_port_interface(port_interface)?;
        match self.0.create_r_port(name, &port_interface) {
            Ok(value) => Ok(RPortPrototype(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new provided port with the given name and port interface
    #[pyo3(signature = (name, port_interface, /))]
    #[pyo3(text_signature = "(self, name: str, port_interface: PortInterface, /)")]
    fn create_p_port(
        &self,
        name: &str,
        port_interface: &Bound<'_, PyAny>,
    ) -> PyResult<PPortPrototype> {
        let port_interface = pyany_to_port_interface(port_interface)?;
        match self.0.create_p_port(name, &port_interface) {
            Ok(value) => Ok(PPortPrototype(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new provided required port with the given name and port interface
    #[pyo3(signature = (name, port_interface, /))]
    #[pyo3(text_signature = "(self, name: str, port_interface: PortInterface, /)")]
    fn create_pr_port(
        &self,
        name: &str,
        port_interface: &Bound<'_, PyAny>,
    ) -> PyResult<PRPortPrototype> {
        let port_interface = pyany_to_port_interface(port_interface)?;
        match self.0.create_pr_port(name, &port_interface) {
            Ok(value) => Ok(PRPortPrototype(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// get an iterator over the ports of the component
    fn ports(&self) -> PortPrototypeIterator {
        PortPrototypeIterator::new(
            self.0
                .ports()
                .filter_map(|port| port_prototype_to_pyany(port).ok()),
        )
    }

    /// create a new port group
    #[pyo3(signature = (name, /))]
    #[pyo3(text_signature = "(self, name: str, /)")]
    fn create_port_group(&self, name: &str) -> PyResult<PortGroup> {
        match self.0.create_port_group(name) {
            Ok(value) => Ok(PortGroup(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create an SwcInternalBehavior for the component
    ///
    /// A component can have only one internal behavior, but since the internal behavior is a variation point,
    /// more than one internal behavior can be created. In this case the variation point settings must ensure that only one
    /// internal behavior is active.
    #[pyo3(signature = (name, /))]
    #[pyo3(text_signature = "(self, name: str, /)")]
    fn create_swc_internal_behavior(&self, name: &str) -> PyResult<SwcInternalBehavior> {
        match self.0.create_swc_internal_behavior(name) {
            Ok(value) => Ok(SwcInternalBehavior(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all swc internal behaviors - typically zero or one
    fn swc_internal_behaviors(&self) -> SwcInternalBehaviorIterator {
        SwcInternalBehaviorIterator::new(self.0.swc_internal_behaviors().map(SwcInternalBehavior))
    }
}

//##################################################################

/// The `ECUAbstraction` is a special `AtomicSwComponentType` that resides between a software-component
/// that wants to access ECU periphery and the Microcontroller Abstraction
///
/// Use [`ArPackage::create_ecu_abstraction_sw_component_type`] to create a new ECU abstraction sw component type.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct EcuAbstractionSwComponentType(
    pub(crate) autosar_data_abstraction::software_component::EcuAbstractionSwComponentType,
);

#[pymethods]
impl EcuAbstractionSwComponentType {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::EcuAbstractionSwComponentType::try_from(
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

    // ------ AbstractSwComponentType ------

    /// iterator over the instances of the component type
    fn instances(&self) -> Vec<Py<PyAny>> {
        self.0
            .instances()
            .into_iter()
            .filter_map(|instance| component_prototype_to_pyany(instance).ok())
            .collect()
    }

    /// iterator over all compositions containing instances of the component type
    fn parent_compositions(&self) -> Vec<CompositionSwComponentType> {
        self.0
            .parent_compositions()
            .into_iter()
            .map(CompositionSwComponentType)
            .collect()
    }

    /// create a new required port with the given name and port interface
    #[pyo3(signature = (name, port_interface, /))]
    #[pyo3(text_signature = "(self, name: str, port_interface: PortInterface, /)")]
    fn create_r_port(
        &self,
        name: &str,
        port_interface: &Bound<'_, PyAny>,
    ) -> PyResult<RPortPrototype> {
        let port_interface = pyany_to_port_interface(port_interface)?;
        match self.0.create_r_port(name, &port_interface) {
            Ok(value) => Ok(RPortPrototype(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new provided port with the given name and port interface
    #[pyo3(signature = (name, port_interface, /))]
    #[pyo3(text_signature = "(self, name: str, port_interface: PortInterface, /)")]
    fn create_p_port(
        &self,
        name: &str,
        port_interface: &Bound<'_, PyAny>,
    ) -> PyResult<PPortPrototype> {
        let port_interface = pyany_to_port_interface(port_interface)?;
        match self.0.create_p_port(name, &port_interface) {
            Ok(value) => Ok(PPortPrototype(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new provided required port with the given name and port interface
    #[pyo3(signature = (name, port_interface, /))]
    #[pyo3(text_signature = "(self, name: str, port_interface: PortInterface, /)")]
    fn create_pr_port(
        &self,
        name: &str,
        port_interface: &Bound<'_, PyAny>,
    ) -> PyResult<PRPortPrototype> {
        let port_interface = pyany_to_port_interface(port_interface)?;
        match self.0.create_pr_port(name, &port_interface) {
            Ok(value) => Ok(PRPortPrototype(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// get an iterator over the ports of the component
    fn ports(&self) -> PortPrototypeIterator {
        PortPrototypeIterator::new(
            self.0
                .ports()
                .filter_map(|port| port_prototype_to_pyany(port).ok()),
        )
    }

    /// create a new port group
    #[pyo3(signature = (name, /))]
    #[pyo3(text_signature = "(self, name: str, /)")]
    fn create_port_group(&self, name: &str) -> PyResult<PortGroup> {
        match self.0.create_port_group(name) {
            Ok(value) => Ok(PortGroup(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create an SwcInternalBehavior for the component
    ///
    /// A component can have only one internal behavior, but since the internal behavior is a variation point,
    /// more than one internal behavior can be created. In this case the variation point settings must ensure that only one
    /// internal behavior is active.
    #[pyo3(signature = (name, /))]
    #[pyo3(text_signature = "(self, name: str, /)")]
    fn create_swc_internal_behavior(&self, name: &str) -> PyResult<SwcInternalBehavior> {
        match self.0.create_swc_internal_behavior(name) {
            Ok(value) => Ok(SwcInternalBehavior(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all swc internal behaviors - typically zero or one
    fn swc_internal_behaviors(&self) -> SwcInternalBehaviorIterator {
        SwcInternalBehaviorIterator::new(self.0.swc_internal_behaviors().map(SwcInternalBehavior))
    }
}

//##################################################################

pub(crate) fn pyany_to_sw_component_type(
    pyobject: &Bound<'_, PyAny>,
) -> PyResult<autosar_data_abstraction::software_component::SwComponentType> {
    if let Ok(value) = pyobject.extract::<CompositionSwComponentType>() {
        Ok(autosar_data_abstraction::software_component::SwComponentType::Composition(value.0))
    } else if let Ok(value) = pyobject.extract::<ApplicationSwComponentType>() {
        Ok(autosar_data_abstraction::software_component::SwComponentType::Application(value.0))
    } else if let Ok(value) = pyobject.extract::<ComplexDeviceDriverSwComponentType>() {
        Ok(
            autosar_data_abstraction::software_component::SwComponentType::ComplexDeviceDriver(
                value.0,
            ),
        )
    } else if let Ok(value) = pyobject.extract::<ServiceSwComponentType>() {
        Ok(autosar_data_abstraction::software_component::SwComponentType::Service(value.0))
    } else if let Ok(value) = pyobject.extract::<SensorActuatorSwComponentType>() {
        Ok(autosar_data_abstraction::software_component::SwComponentType::SensorActuator(value.0))
    } else if let Ok(value) = pyobject.extract::<EcuAbstractionSwComponentType>() {
        Ok(autosar_data_abstraction::software_component::SwComponentType::EcuAbstraction(value.0))
    } else {
        Err(AutosarAbstractionError::new_err(
            "Could not convert to SwComponentType".to_string(),
        ))
    }
}

pub(crate) fn sw_component_type_to_pyany(
    component_type: autosar_data_abstraction::software_component::SwComponentType,
) -> PyResult<Py<PyAny>> {
    Python::attach(|py| match component_type {
        autosar_data_abstraction::software_component::SwComponentType::Composition(component) => {
            CompositionSwComponentType(component).into_py_any(py)
        }
        autosar_data_abstraction::software_component::SwComponentType::Application(component) => {
            ApplicationSwComponentType(component).into_py_any(py)
        }
        autosar_data_abstraction::software_component::SwComponentType::ComplexDeviceDriver(
            component,
        ) => ComplexDeviceDriverSwComponentType(component).into_py_any(py),
        autosar_data_abstraction::software_component::SwComponentType::Service(component) => {
            ServiceSwComponentType(component).into_py_any(py)
        }
        autosar_data_abstraction::software_component::SwComponentType::SensorActuator(
            component,
        ) => SensorActuatorSwComponentType(component).into_py_any(py),
        autosar_data_abstraction::software_component::SwComponentType::EcuAbstraction(
            component,
        ) => EcuAbstractionSwComponentType(component).into_py_any(py),
    })
}

//##################################################################

/// A `SwComponentPrototype` is an instance of a software component type
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct SwComponentPrototype(
    pub(crate) autosar_data_abstraction::software_component::SwComponentPrototype,
);

#[pymethods]
impl SwComponentPrototype {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::SwComponentPrototype::try_from(
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

iterator_wrapper!(SwComponentPrototypeIterator, SwComponentPrototype);

//##################################################################

pub(crate) fn component_prototype_to_pyany(
    component_prototype: autosar_data_abstraction::software_component::ComponentPrototype,
) -> PyResult<Py<PyAny>> {
    Python::attach(|py| match component_prototype {
        autosar_data_abstraction::software_component::ComponentPrototype::SwComponent(
            component,
        ) => SwComponentPrototype(component).into_py_any(py),
        autosar_data_abstraction::software_component::ComponentPrototype::RootComposition(
            component,
        ) => RootSwCompositionPrototype(component).into_py_any(py),
    })
}

//##################################################################

/// The `RootSwCompositionPrototype` is a special kind of `SwComponentPrototype` that represents the root of the composition hierarchy
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct RootSwCompositionPrototype(
    pub(crate) autosar_data_abstraction::software_component::RootSwCompositionPrototype,
);

#[pymethods]
impl RootSwCompositionPrototype {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::RootSwCompositionPrototype::try_from(
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

    /// get the composition that this root component is based on
    #[getter]
    fn composition(&self) -> Option<CompositionSwComponentType> {
        self.0.composition().map(CompositionSwComponentType)
    }
}
