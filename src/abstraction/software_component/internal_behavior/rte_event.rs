use crate::{
    Element,
    abstraction::{
        AutosarAbstractionError, abstraction_err_to_pyerr,
        software_component::{
            ClientServerOperation, ModeDeclaration, PPortPrototype, RunnableEntity,
            SwcInternalBehavior, VariableDataPrototype, port_prototype_to_pyany,
            pyany_to_port_prototype,
        },
    },
    iterator_wrapper,
};
use autosar_data_abstraction::{
    self, AbstractionElement, IdentifiableAbstractionElement, software_component::AbstractRTEEvent,
};
use pyo3::{IntoPyObjectExt, prelude::*};

//##################################################################

/// A `TimingEvent` is a subclass of `RTEEvent` which triggers a `RunnableEntity` periodically
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct TimingEvent(pub(crate) autosar_data_abstraction::software_component::TimingEvent);

#[pymethods]
impl TimingEvent {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::TimingEvent::try_from(element.0.clone())
        {
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

    /// Set the `RunnableEntity` that is triggered by the `TimingEvent`
    #[setter]
    fn set_runnable_entity(&self, runnable_entity: &RunnableEntity) -> PyResult<()> {
        self.0
            .set_runnable_entity(&runnable_entity.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the `RunnableEntity` that is triggered by the `TimingEvent`
    #[getter]
    fn runnable_entity(&self) -> Option<RunnableEntity> {
        self.0.runnable_entity().map(RunnableEntity)
    }

    /// Get the `SwcInternalBehavior` that contains the event
    #[getter]
    fn swc_internal_behavior(&self) -> Option<SwcInternalBehavior> {
        self.0.swc_internal_behavior().map(SwcInternalBehavior)
    }

    /// Set the period of the `TimingEvent`
    #[setter]
    fn set_period(&self, period: f64) -> PyResult<()> {
        self.0.set_period(period).map_err(abstraction_err_to_pyerr)
    }

    /// Get the period of the `TimingEvent`
    #[getter]
    fn period(&self) -> Option<f64> {
        self.0.period()
    }
}

//##################################################################

/// an asynchronous server call completed
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct AsynchronousServerCallReturnsEvent(
    pub(crate) autosar_data_abstraction::software_component::AsynchronousServerCallReturnsEvent,
);

#[pymethods]
impl AsynchronousServerCallReturnsEvent {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::AsynchronousServerCallReturnsEvent::try_from(
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

    /// Get the `RunnableEntity` that is triggered by the `AsynchronousServerCallCompleted`
    #[getter]
    fn runnable_entity(&self) -> Option<RunnableEntity> {
        self.0.runnable_entity().map(RunnableEntity)
    }

    /// Set the `RunnableEntity` that is triggered by the `AsynchronousServerCallCompleted`
    #[setter]
    fn set_runnable_entity(&self, runnable_entity: &RunnableEntity) -> PyResult<()> {
        self.0
            .set_runnable_entity(&runnable_entity.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the `SwcInternalBehavior` that contains the event
    #[getter]
    fn swc_internal_behavior(&self) -> Option<SwcInternalBehavior> {
        self.0.swc_internal_behavior().map(SwcInternalBehavior)
    }
}

//##################################################################

/// starts a runnable for background processing at low priority
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct BackgroundEvent(
    pub(crate) autosar_data_abstraction::software_component::BackgroundEvent,
);

#[pymethods]
impl BackgroundEvent {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::BackgroundEvent::try_from(
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

    /// Get the `RunnableEntity` that is triggered by the `BackgroundEvent`
    #[getter]
    fn runnable_entity(&self) -> Option<RunnableEntity> {
        self.0.runnable_entity().map(RunnableEntity)
    }

    /// Set the `RunnableEntity` that is triggered by the `BackgroundEvent`
    #[setter]
    fn set_runnable_entity(&self, runnable_entity: &RunnableEntity) -> PyResult<()> {
        self.0
            .set_runnable_entity(&runnable_entity.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the `SwcInternalBehavior` that contains the event
    #[getter]
    fn swc_internal_behavior(&self) -> Option<SwcInternalBehavior> {
        self.0.swc_internal_behavior().map(SwcInternalBehavior)
    }
}

//##################################################################

/// A `DataReceivedEvent` is a subclass of `RTEEvent` which triggers a `RunnableEntity` when data is received
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct DataReceivedEvent(
    pub(crate) autosar_data_abstraction::software_component::DataReceivedEvent,
);

#[pymethods]
impl DataReceivedEvent {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::DataReceivedEvent::try_from(
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

    /// Set the `RunnableEntity` that is triggered by the `DataReceivedEvent`
    #[setter]
    fn set_runnable_entity(&self, runnable_entity: &RunnableEntity) -> PyResult<()> {
        self.0
            .set_runnable_entity(&runnable_entity.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the `RunnableEntity` that is triggered by the `DataReceivedEvent`
    #[getter]
    fn runnable_entity(&self) -> Option<RunnableEntity> {
        self.0.runnable_entity().map(RunnableEntity)
    }

    /// Get the `SwcInternalBehavior` that contains the event
    #[getter]
    fn swc_internal_behavior(&self) -> Option<SwcInternalBehavior> {
        self.0.swc_internal_behavior().map(SwcInternalBehavior)
    }

    /// Set the `VariableDataPrototype` that triggers the `DataReceivedEvent`
    #[pyo3(signature = (variable_data_prototype, context_port, /))]
    #[pyo3(
        text_signature = "(self, variable_data_prototype: VariableDataPrototype, context_port: PPortPrototype, /)"
    )]
    fn set_variable_data_prototype(
        &self,
        variable_data_prototype: &VariableDataPrototype,
        context_port: &Bound<'_, PyAny>,
    ) -> PyResult<()> {
        let context_port = pyany_to_port_prototype(context_port)?;
        self.0
            .set_variable_data_prototype(&variable_data_prototype.0, &context_port)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the `VariableDataPrototype` and the associated context port that triggers the `DataReceivedEvent`
    #[getter]
    fn variable_data_prototype(&self) -> Option<(VariableDataPrototype, Py<PyAny>)> {
        let (variable_data_prototype, context_port) = self.0.variable_data_prototype()?;
        let context_port = port_prototype_to_pyany(context_port).ok()?;
        Some((VariableDataPrototype(variable_data_prototype), context_port))
    }
}

//##################################################################

/// A `DataSendCompletedEvent` is a subclass of `RTEEvent` which triggers a `RunnableEntity` when data is sent
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct DataSendCompletedEvent(
    pub(crate) autosar_data_abstraction::software_component::DataSendCompletedEvent,
);

#[pymethods]
impl DataSendCompletedEvent {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::DataSendCompletedEvent::try_from(
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

    /// Set the `RunnableEntity` that is triggered by the `DataSendCompletedEvent`
    #[setter]
    fn set_runnable_entity(&self, runnable_entity: &RunnableEntity) -> PyResult<()> {
        self.0
            .set_runnable_entity(&runnable_entity.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the `RunnableEntity` that is triggered by the `DataSendCompletedEvent`
    #[getter]
    fn runnable_entity(&self) -> Option<RunnableEntity> {
        self.0.runnable_entity().map(RunnableEntity)
    }

    /// Get the `SwcInternalBehavior` that contains the event
    #[getter]
    fn swc_internal_behavior(&self) -> Option<SwcInternalBehavior> {
        self.0.swc_internal_behavior().map(SwcInternalBehavior)
    }
}

//##################################################################

/// A `DataReceiveErrorEvent` is a subclass of `RTEEvent` which triggers a `RunnableEntity` when a data receive error occurs
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct DataReceiveErrorEvent(
    pub(crate) autosar_data_abstraction::software_component::DataReceiveErrorEvent,
);

#[pymethods]
impl DataReceiveErrorEvent {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::DataReceiveErrorEvent::try_from(
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

    /// Set the `RunnableEntity` that is triggered by the `DataReceiveErrorEvent`
    #[setter]
    fn set_runnable_entity(&self, runnable_entity: &RunnableEntity) -> PyResult<()> {
        self.0
            .set_runnable_entity(&runnable_entity.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the `RunnableEntity` that is triggered by the `DataReceiveErrorEvent`
    #[getter]
    fn runnable_entity(&self) -> Option<RunnableEntity> {
        self.0.runnable_entity().map(RunnableEntity)
    }

    /// Get the `SwcInternalBehavior` that contains the event
    #[getter]
    fn swc_internal_behavior(&self) -> Option<SwcInternalBehavior> {
        self.0.swc_internal_behavior().map(SwcInternalBehavior)
    }
}

//##################################################################

/// A `DataWriteCompletedEvent` is a subclass of `RTEEvent` which triggers a `RunnableEntity` when data is written
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct DataWriteCompletedEvent(
    pub(crate) autosar_data_abstraction::software_component::DataWriteCompletedEvent,
);

#[pymethods]
impl DataWriteCompletedEvent {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::DataWriteCompletedEvent::try_from(
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

    /// Set the `RunnableEntity` that is triggered by the `DataWriteCompletedEvent`
    #[setter]
    fn set_runnable_entity(&self, runnable_entity: &RunnableEntity) -> PyResult<()> {
        self.0
            .set_runnable_entity(&runnable_entity.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the `RunnableEntity` that is triggered by the `DataWriteCompletedEvent`
    #[getter]
    fn runnable_entity(&self) -> Option<RunnableEntity> {
        self.0.runnable_entity().map(RunnableEntity)
    }

    /// Get the `SwcInternalBehavior` that contains the event
    #[getter]
    fn swc_internal_behavior(&self) -> Option<SwcInternalBehavior> {
        self.0.swc_internal_behavior().map(SwcInternalBehavior)
    }
}

//##################################################################

/// A `ExternalTriggerOccurredEvent` is a subclass of `RTEEvent` which triggers a `RunnableEntity` when an external trigger occurs
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct ExternalTriggerOccurredEvent(
    pub(crate) autosar_data_abstraction::software_component::ExternalTriggerOccurredEvent,
);

#[pymethods]
impl ExternalTriggerOccurredEvent {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::ExternalTriggerOccurredEvent::try_from(
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

    /// Set the `RunnableEntity` that is triggered by the `ExternalTriggerOccurredEvent`
    #[setter]
    fn set_runnable_entity(&self, runnable_entity: &RunnableEntity) -> PyResult<()> {
        self.0
            .set_runnable_entity(&runnable_entity.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the `RunnableEntity` that is triggered by the `ExternalTriggerOccurredEvent`
    #[getter]
    fn runnable_entity(&self) -> Option<RunnableEntity> {
        self.0.runnable_entity().map(RunnableEntity)
    }

    /// Get the `SwcInternalBehavior` that contains the event
    #[getter]
    fn swc_internal_behavior(&self) -> Option<SwcInternalBehavior> {
        self.0.swc_internal_behavior().map(SwcInternalBehavior)
    }
}

//##################################################################

/// A `InitEvent` is a subclass of `RTEEvent` which triggers a `RunnableEntity` when the software component is initialized
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct InitEvent(pub(crate) autosar_data_abstraction::software_component::InitEvent);

#[pymethods]
impl InitEvent {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::InitEvent::try_from(element.0.clone()) {
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

    /// Set the `RunnableEntity` that is triggered by the `InitEvent`
    #[setter]
    fn set_runnable_entity(&self, runnable_entity: &RunnableEntity) -> PyResult<()> {
        self.0
            .set_runnable_entity(&runnable_entity.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the `RunnableEntity` that is triggered by the `InitEvent`
    #[getter]
    fn runnable_entity(&self) -> Option<RunnableEntity> {
        self.0.runnable_entity().map(RunnableEntity)
    }

    /// Get the `SwcInternalBehavior` that contains the event
    #[getter]
    fn swc_internal_behavior(&self) -> Option<SwcInternalBehavior> {
        self.0.swc_internal_behavior().map(SwcInternalBehavior)
    }
}

//##################################################################

/// A `InternalTriggerOccurredEvent` is a subclass of `RTEEvent` which triggers a `RunnableEntity` when an internal trigger occurs
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct InternalTriggerOccurredEvent(
    pub(crate) autosar_data_abstraction::software_component::InternalTriggerOccurredEvent,
);

#[pymethods]
impl InternalTriggerOccurredEvent {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::InternalTriggerOccurredEvent::try_from(
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

    /// Set the `RunnableEntity` that is triggered by the `InternalTriggerOccurredEvent`
    #[setter]
    fn set_runnable_entity(&self, runnable_entity: &RunnableEntity) -> PyResult<()> {
        self.0
            .set_runnable_entity(&runnable_entity.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the `RunnableEntity` that is triggered by the `InternalTriggerOccurredEvent`
    #[getter]
    fn runnable_entity(&self) -> Option<RunnableEntity> {
        self.0.runnable_entity().map(RunnableEntity)
    }

    /// Get the `SwcInternalBehavior` that contains the event
    #[getter]
    fn swc_internal_behavior(&self) -> Option<SwcInternalBehavior> {
        self.0.swc_internal_behavior().map(SwcInternalBehavior)
    }
}

//##################################################################

/// A `ModeSwitchedAckEvent` is a subclass of `RTEEvent` which triggers a `RunnableEntity` when a mode switch is acknowledged
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct ModeSwitchedAckEvent(
    pub(crate) autosar_data_abstraction::software_component::ModeSwitchedAckEvent,
);

#[pymethods]
impl ModeSwitchedAckEvent {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::ModeSwitchedAckEvent::try_from(
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

    /// Set the `RunnableEntity` that is triggered by the `ModeSwitchedAckEvent`
    #[setter]
    fn set_runnable_entity(&self, runnable_entity: &RunnableEntity) -> PyResult<()> {
        self.0
            .set_runnable_entity(&runnable_entity.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the `RunnableEntity` that is triggered by the `ModeSwitchedAckEvent`
    #[getter]
    fn runnable_entity(&self) -> Option<RunnableEntity> {
        self.0.runnable_entity().map(RunnableEntity)
    }

    /// Get the `SwcInternalBehavior` that contains the event
    #[getter]
    fn swc_internal_behavior(&self) -> Option<SwcInternalBehavior> {
        self.0.swc_internal_behavior().map(SwcInternalBehavior)
    }
}

//##################################################################

/// A `OperationInvokedEvent` is a subclass of `RTEEvent` which triggers a `RunnableEntity` when an operation is invoked
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct OperationInvokedEvent(
    pub(crate) autosar_data_abstraction::software_component::OperationInvokedEvent,
);

#[pymethods]
impl OperationInvokedEvent {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::OperationInvokedEvent::try_from(
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

    /// Set the `ClientServerOperation` that is triggers the `OperationInvokedEvent`
    #[pyo3(signature = (client_server_operation, context_p_port, /))]
    #[pyo3(
        text_signature = "(self, client_server_operation: ClientServerOperation, context_p_port: PPortPrototype, /)"
    )]
    fn set_client_server_operation(
        &self,
        client_server_operation: &ClientServerOperation,
        context_p_port: &PPortPrototype,
    ) -> PyResult<()> {
        self.0
            .set_client_server_operation(&client_server_operation.0, &context_p_port.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the `ClientServerOperation` that triggers the `OperationInvokedEvent`
    #[getter]
    fn client_server_operation(&self) -> Option<(ClientServerOperation, PPortPrototype)> {
        self.0
            .client_server_operation()
            .map(|(operation, context_p_port)| {
                (
                    ClientServerOperation(operation),
                    PPortPrototype(context_p_port),
                )
            })
    }

    /// Set the `RunnableEntity` that is triggered by the `OperationInvokedEvent`
    #[setter]
    fn set_runnable_entity(&self, runnable_entity: &RunnableEntity) -> PyResult<()> {
        self.0
            .set_runnable_entity(&runnable_entity.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the `RunnableEntity` that is triggered by the `OperationInvokedEvent`
    #[getter]
    fn runnable_entity(&self) -> Option<RunnableEntity> {
        self.0.runnable_entity().map(RunnableEntity)
    }

    /// Get the `SwcInternalBehavior` that contains the event
    #[getter]
    fn swc_internal_behavior(&self) -> Option<SwcInternalBehavior> {
        self.0.swc_internal_behavior().map(SwcInternalBehavior)
    }
}

//##################################################################

/// A `OsTaskExecutionEvent` is a subclass of `RTEEvent` which triggers a `RunnableEntity` when an OS task is executed
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct OsTaskExecutionEvent(
    pub(crate) autosar_data_abstraction::software_component::OsTaskExecutionEvent,
);

#[pymethods]
impl OsTaskExecutionEvent {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::OsTaskExecutionEvent::try_from(
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

    /// Set the `RunnableEntity` that is triggered by the `OsTaskExecutionEvent`
    #[setter]
    fn set_runnable_entity(&self, runnable_entity: &RunnableEntity) -> PyResult<()> {
        self.0
            .set_runnable_entity(&runnable_entity.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the `RunnableEntity` that is triggered by the `OsTaskExecutionEvent`
    #[getter]
    fn runnable_entity(&self) -> Option<RunnableEntity> {
        self.0.runnable_entity().map(RunnableEntity)
    }

    /// Get the `SwcInternalBehavior` that contains the event
    #[getter]
    fn swc_internal_behavior(&self) -> Option<SwcInternalBehavior> {
        self.0.swc_internal_behavior().map(SwcInternalBehavior)
    }
}

//##################################################################

/// A `SwcModeManagerErrorEvent` is a subclass of `RTEEvent` which triggers a `RunnableEntity` when a mode manager error occurs
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct SwcModeManagerErrorEvent(
    pub(crate) autosar_data_abstraction::software_component::SwcModeManagerErrorEvent,
);

#[pymethods]
impl SwcModeManagerErrorEvent {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::SwcModeManagerErrorEvent::try_from(
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

    /// Set the `RunnableEntity` that is triggered by the `SwcModeManagerErrorEvent`
    #[setter]
    fn set_runnable_entity(&self, runnable_entity: &RunnableEntity) -> PyResult<()> {
        self.0
            .set_runnable_entity(&runnable_entity.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the `RunnableEntity` that is triggered by the `SwcModeManagerErrorEvent`
    #[getter]
    fn runnable_entity(&self) -> Option<RunnableEntity> {
        self.0.runnable_entity().map(RunnableEntity)
    }

    /// Get the `SwcInternalBehavior` that contains the event
    #[getter]
    fn swc_internal_behavior(&self) -> Option<SwcInternalBehavior> {
        self.0.swc_internal_behavior().map(SwcInternalBehavior)
    }
}

//##################################################################

/// A `SwcModeSwitchEvent` is a subclass of `RTEEvent` which triggers a `RunnableEntity` when a mode switch occurs
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct SwcModeSwitchEvent(
    pub(crate) autosar_data_abstraction::software_component::SwcModeSwitchEvent,
);

#[pymethods]
impl SwcModeSwitchEvent {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::SwcModeSwitchEvent::try_from(
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

    /// Set the `RunnableEntity` that is triggered by the `SwcModeSwitchEvent`
    #[setter]
    fn set_runnable_entity(&self, runnable_entity: &RunnableEntity) -> PyResult<()> {
        self.0
            .set_runnable_entity(&runnable_entity.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the `RunnableEntity` that is triggered by the `SwcModeSwitchEvent`
    #[getter]
    fn runnable_entity(&self) -> Option<RunnableEntity> {
        self.0.runnable_entity().map(RunnableEntity)
    }

    /// Get the `SwcInternalBehavior` that contains the event
    #[getter]
    fn swc_internal_behavior(&self) -> Option<SwcInternalBehavior> {
        self.0.swc_internal_behavior().map(SwcInternalBehavior)
    }

    /// Set the `ModeActivationKind` for the `SwcModeSwitchEvent`
    #[setter]
    fn set_mode_activation_kind(&self, kind: ModeActivationKind) -> PyResult<()> {
        self.0
            .set_mode_activation_kind(kind.into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the `ModeActivationKind` for the `SwcModeSwitchEvent`
    #[getter]
    fn mode_activation_kind(&self) -> Option<ModeActivationKind> {
        self.0.mode_activation_kind().map(Into::into)
    }

    /// Set the `ModeDeclaration` that triggers the `SwcModeSwitchEvent`
    ///
    /// The second mode must be provided if the activation kind `OnTransition` is configured.
    /// In that case only transitions between the two modes trigger the event.
    #[pyo3(signature = (context_port, mode_declaration, /, second_mode_declaration=None))]
    #[pyo3(
        text_signature = "(self, ontext_port: PortPrototype, mode_declaration: ModeDeclaration, /, second_mode_declaration: Optional[ModeDeclaration] = None)"
    )]
    fn set_mode_declaration(
        &self,
        context_port: &Bound<'_, PyAny>,
        mode_declaration: &ModeDeclaration,
        second_mode_declaration: Option<&ModeDeclaration>,
    ) -> PyResult<()> {
        let context_port = pyany_to_port_prototype(context_port)?;
        self.0
            .set_mode_declaration(
                &context_port,
                &mode_declaration.0,
                second_mode_declaration.map(|m| &m.0),
            )
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the `ModeDeclaration`s that trigger the `SwcModeSwitchEvent`
    ///
    /// The list contains either one or two `ModeDeclaration`s depending on the `ModeActivationKind`.
    fn mode_declarations(&self) -> Option<(Vec<ModeDeclaration>, Py<PyAny>)> {
        let (modes, context_port) = self.0.mode_declarations()?;
        let mode_declarations = modes.into_iter().map(ModeDeclaration).collect::<Vec<_>>();
        let context_port = port_prototype_to_pyany(context_port).ok()?;
        Some((mode_declarations, context_port))
    }
}

//##################################################################

/// Kind of mode switch condition used for activation of an event
#[pyclass(
    frozen,
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(clippy::enum_variant_names)] // named as per AUTOSAR standard
pub(crate) enum ModeActivationKind {
    /// On entering the mode
    OnEntry,
    /// On leaving the mode
    OnExit,
    /// on transition from the first mode to the second mode
    OnTransition,
}

impl From<autosar_data_abstraction::software_component::ModeActivationKind> for ModeActivationKind {
    fn from(value: autosar_data_abstraction::software_component::ModeActivationKind) -> Self {
        match value {
            autosar_data_abstraction::software_component::ModeActivationKind::OnEntry => {
                ModeActivationKind::OnEntry
            }
            autosar_data_abstraction::software_component::ModeActivationKind::OnExit => {
                ModeActivationKind::OnExit
            }
            autosar_data_abstraction::software_component::ModeActivationKind::OnTransition => {
                ModeActivationKind::OnTransition
            }
        }
    }
}

impl From<ModeActivationKind> for autosar_data_abstraction::software_component::ModeActivationKind {
    fn from(value: ModeActivationKind) -> Self {
        match value {
            ModeActivationKind::OnEntry => {
                autosar_data_abstraction::software_component::ModeActivationKind::OnEntry
            }
            ModeActivationKind::OnExit => {
                autosar_data_abstraction::software_component::ModeActivationKind::OnExit
            }
            ModeActivationKind::OnTransition => {
                autosar_data_abstraction::software_component::ModeActivationKind::OnTransition
            }
        }
    }
}

//##################################################################

/// A `TransformerHardErrorEvent` is a subclass of `RTEEvent` which triggers a `RunnableEntity` when a transformer hard error occurs
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct TransformerHardErrorEvent(
    pub(crate) autosar_data_abstraction::software_component::TransformerHardErrorEvent,
);

#[pymethods]
impl TransformerHardErrorEvent {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::TransformerHardErrorEvent::try_from(
            element.0.clone(),
        ) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// Set the name of the `TransformerHardErrorEvent`
    #[setter]
    fn set_name(&self, name: &str) -> PyResult<()> {
        self.0.set_name(name).map_err(abstraction_err_to_pyerr)
    }

    /// Get the name of the `TransformerHardErrorEvent`
    #[getter]
    fn name(&self) -> Option<String> {
        self.0.name()
    }

    /// Get the element of the `TransformerHardErrorEvent`
    #[getter]
    fn element(&self) -> Element {
        Element(self.0.element().clone())
    }

    /// Get a string representation of the `TransformerHardErrorEvent`
    fn __repr__(&self) -> String {
        format!("{:#?}", self.0)
    }

    /// Set the `RunnableEntity` that is triggered by the `TransformerHardErrorEvent`
    #[setter]
    fn set_runnable_entity(&self, runnable_entity: &RunnableEntity) -> PyResult<()> {
        self.0
            .set_runnable_entity(&runnable_entity.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the `RunnableEntity` that is triggered by the `TransformerHardErrorEvent`
    #[getter]
    fn runnable_entity(&self) -> Option<RunnableEntity> {
        self.0.runnable_entity().map(RunnableEntity)
    }

    /// Get the `SwcInternalBehavior` that contains the event
    #[getter]
    fn swc_internal_behavior(&self) -> Option<SwcInternalBehavior> {
        self.0.swc_internal_behavior().map(SwcInternalBehavior)
    }
}

//##################################################################

iterator_wrapper!(RteEventIterator, Py<PyAny>, "RTEEvent");

//##################################################################

pub(crate) fn rte_event_to_pyany(
    event: autosar_data_abstraction::software_component::RTEEvent,
) -> PyResult<Py<PyAny>> {
    use autosar_data_abstraction::software_component::RTEEvent;
    Python::attach(|py| match event {
        RTEEvent::TimingEvent(event) => TimingEvent(event).into_py_any(py),
        RTEEvent::AsynchronousServerCallReturnsEvent(event) => {
            AsynchronousServerCallReturnsEvent(event).into_py_any(py)
        }
        RTEEvent::BackgroundEvent(event) => BackgroundEvent(event).into_py_any(py),
        RTEEvent::DataReceivedEvent(event) => DataReceivedEvent(event).into_py_any(py),
        RTEEvent::DataSendCompletedEvent(event) => DataSendCompletedEvent(event).into_py_any(py),
        RTEEvent::DataReceiveErrorEvent(event) => DataReceiveErrorEvent(event).into_py_any(py),
        RTEEvent::DataWriteCompletedEvent(event) => DataWriteCompletedEvent(event).into_py_any(py),
        RTEEvent::ExternalTriggerOccurredEvent(event) => {
            ExternalTriggerOccurredEvent(event).into_py_any(py)
        }
        RTEEvent::InitEvent(event) => InitEvent(event).into_py_any(py),
        RTEEvent::InternalTriggerOccurredEvent(event) => {
            InternalTriggerOccurredEvent(event).into_py_any(py)
        }
        RTEEvent::ModeSwitchedAckEvent(event) => ModeSwitchedAckEvent(event).into_py_any(py),
        RTEEvent::OperationInvokedEvent(event) => OperationInvokedEvent(event).into_py_any(py),
        RTEEvent::OsTaskExecutionEvent(event) => OsTaskExecutionEvent(event).into_py_any(py),
        RTEEvent::SwcModeManagerErrorEvent(event) => {
            SwcModeManagerErrorEvent(event).into_py_any(py)
        }
        RTEEvent::SwcModeSwitchEvent(event) => SwcModeSwitchEvent(event).into_py_any(py),
        RTEEvent::TransformerHardErrorEvent(event) => {
            TransformerHardErrorEvent(event).into_py_any(py)
        }
    })
}
