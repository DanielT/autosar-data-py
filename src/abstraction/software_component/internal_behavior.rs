use crate::{
    Element,
    abstraction::{
        AutosarAbstractionError, abstraction_err_to_pyerr,
        datatype::{DataTypeMappingSet, DataTypeMappingSetIterator},
        iterator_wrapper,
        software_component::{
            ClientServerOperation, PPortPrototype, sw_component_type_to_pyobject,
        },
    },
};
use autosar_data_abstraction::{
    self, AbstractionElement, IdentifiableAbstractionElement, software_component::AbstractRTEEvent,
};
use pyo3::{IntoPyObjectExt, prelude::*};

//##################################################################

/// The `SwcInternalBehavior` of a software component type describes the
/// details that are needed to generate the RTE.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct SwcInternalBehavior(
    pub(crate) autosar_data_abstraction::software_component::SwcInternalBehavior,
);

#[pymethods]
impl SwcInternalBehavior {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::SwcInternalBehavior::try_from(
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

    /// Get the software component type that contains the `SwcInternalBehavior`
    #[getter]
    fn sw_component_type(&self) -> Option<PyObject> {
        self.0
            .sw_component_type()
            .and_then(|value| sw_component_type_to_pyobject(value).ok())
    }

    /// Create a new RunnableEntity in the SwcInternalBehavior
    #[pyo3(signature = (name, /))]
    #[pyo3(text_signature = "(self, name: str, /)")]
    fn create_runnable_entity(&self, name: &str) -> PyResult<RunnableEntity> {
        match self.0.create_runnable_entity(name) {
            Ok(value) => Ok(RunnableEntity(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// Get an iterator over all RunnableEntities in the SwcInternalBehavior
    fn runnable_entities(&self) -> RunnableEntityIterator {
        RunnableEntityIterator::new(self.0.runnable_entities().map(RunnableEntity))
    }

    /// Add a reference to a `DataTypeMappingSet` to the `SwcInternalBehavior`
    #[pyo3(signature = (data_type_mapping_set, /))]
    #[pyo3(text_signature = "(self, data_type_mapping_set: DataTypeMappingSet, /)")]
    fn add_data_type_mapping_set(
        &self,
        data_type_mapping_set: &DataTypeMappingSet,
    ) -> PyResult<()> {
        self.0
            .add_data_type_mapping_set(&data_type_mapping_set.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// iterator over all `DataTypeMappingSet` references in the `SwcInternalBehavior`
    fn data_type_mapping_sets(&self) -> DataTypeMappingSetIterator {
        DataTypeMappingSetIterator::new(self.0.data_type_mapping_sets().map(DataTypeMappingSet))
    }

    /// Create a new `InitEvent` in the `SwcInternalBehavior`
    #[pyo3(signature = (name, runnable, /))]
    #[pyo3(text_signature = "(self, name: str, runnable: RunnableEntity, /)")]
    fn create_init_event(&self, name: &str, runnable: &RunnableEntity) -> PyResult<InitEvent> {
        match self.0.create_init_event(name, &runnable.0) {
            Ok(value) => Ok(InitEvent(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// Create a new `OperationInvokedEvent` in the `SwcInternalBehavior`that triggers
    /// a `RunnableEntity` when a client-server operation is invoked
    #[pyo3(signature = (name, runnable, client_server_operation, context_p_port, /))]
    #[pyo3(
        text_signature = "(self, name: str, runnable: RunnableEntity, client_server_operation: ClientServerOperation, context_p_port: PPortPrototype, /)"
    )]
    fn create_operation_invoked_event(
        &self,
        name: &str,
        runnable: &RunnableEntity,
        client_server_operation: &ClientServerOperation,
        context_p_port: &PPortPrototype,
    ) -> PyResult<OperationInvokedEvent> {
        match self.0.create_operation_invoked_event(
            name,
            &runnable.0,
            &client_server_operation.0,
            &context_p_port.0,
        ) {
            Ok(value) => Ok(OperationInvokedEvent(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// Create a timing event that triggers a runnable in the `SwcInternalBehavior` based on a timer
    #[pyo3(signature = (name, runnable, period, /))]
    #[pyo3(text_signature = "(self, name: str, runnable: RunnableEntity, period: float, /)")]
    fn create_timing_event(
        &self,
        name: &str,
        runnable: &RunnableEntity,
        period: f64,
    ) -> PyResult<TimingEvent> {
        match self.0.create_timing_event(name, &runnable.0, period) {
            Ok(value) => Ok(TimingEvent(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a background event that triggers a runnable in the `SwcInternalBehavior` for background processing
    #[pyo3(signature = (name, runnable, /))]
    #[pyo3(text_signature = "(self, name: str, runnable: RunnableEntity, /)")]
    fn create_background_event(
        &self,
        name: &str,
        runnable: &RunnableEntity,
    ) -> PyResult<BackgroundEvent> {
        match self.0.create_background_event(name, &runnable.0) {
            Ok(value) => Ok(BackgroundEvent(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create an os task execution event that triggers a runnable in the `SwcInternalBehavior` every time the task is executed
    #[pyo3(signature = (name, runnable, /))]
    #[pyo3(text_signature = "(self, name: str, runnable: RunnableEntity, /)")]
    fn create_os_task_execution_event(
        &self,
        name: &str,
        runnable: &RunnableEntity,
    ) -> PyResult<OsTaskExecutionEvent> {
        match self.0.create_os_task_execution_event(name, &runnable.0) {
            Ok(value) => Ok(OsTaskExecutionEvent(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create an iterator over all events in the `SwcInternalBehavior`
    fn events(&self) -> RteEventIterator {
        RteEventIterator::new(
            self.0
                .events()
                .filter_map(|event| rte_event_to_pyobject(event).ok()),
        )
    }
}

//##################################################################

iterator_wrapper!(SwcInternalBehaviorIterator, SwcInternalBehavior);

//##################################################################

/// A `RunnableEntity` is a function that can be executed by the RTE
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct RunnableEntity(
    pub(crate) autosar_data_abstraction::software_component::RunnableEntity,
);

#[pymethods]
impl RunnableEntity {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::RunnableEntity::try_from(
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

    /// Get the `SwcInternalBehavior` that contains the `RunnableEntity`
    #[getter]
    fn swc_internal_behavior(&self) -> Option<SwcInternalBehavior> {
        self.0.swc_internal_behavior().map(SwcInternalBehavior)
    }

    /// Iterate over all events that can trigger the `RunnableEntity`
    fn events(&self) -> Vec<PyObject> {
        self.0
            .events()
            .into_iter()
            .filter_map(|event| rte_event_to_pyobject(event).ok())
            .collect()
    }
}

//##################################################################

iterator_wrapper!(RunnableEntityIterator, RunnableEntity);

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

iterator_wrapper!(RteEventIterator, PyObject);

//##################################################################

pub(crate) fn rte_event_to_pyobject(
    event: autosar_data_abstraction::software_component::RTEEvent,
) -> PyResult<PyObject> {
    use autosar_data_abstraction::software_component::RTEEvent;
    Python::with_gil(|py| match event {
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
