use crate::{
    Element,
    abstraction::{
        AutosarAbstractionError, abstraction_err_to_pyerr,
        datatype::{DataTypeMappingSet, DataTypeMappingSetIterator},
        software_component::{
            ClientServerOperation, ModeDeclaration, ModeGroup, PPortPrototype, RPortPrototype,
            VariableDataPrototype, port_prototype_to_pyany, pyany_to_port_prototype,
            sw_component_type_to_pyany,
        },
    },
    iterator_wrapper,
};
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};
use pyo3::prelude::*;

mod rte_event;

pub(crate) use rte_event::*;

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
    fn sw_component_type(&self) -> Option<Py<PyAny>> {
        self.0
            .sw_component_type()
            .and_then(|value| sw_component_type_to_pyany(value).ok())
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

    /// create a data received event that triggers a runnable in the `SwcInternalBehavior` when data is received
    #[pyo3(signature = (name, runnable, variable_data_prototype, context_port, /))]
    #[pyo3(
        text_signature = "(self, name: str, runnable: RunnableEntity, variable_data_prototype: VariableDataPrototype, context_port: PortPrototype, /)"
    )]
    fn create_data_received_event(
        &self,
        name: &str,
        runnable: &RunnableEntity,
        variable_data_prototype: &VariableDataPrototype,
        context_port: &Bound<'_, PyAny>,
    ) -> PyResult<DataReceivedEvent> {
        let context_port = pyany_to_port_prototype(context_port)?;
        match self.0.create_data_received_event(
            name,
            &runnable.0,
            &variable_data_prototype.0,
            &context_port,
        ) {
            Ok(value) => Ok(DataReceivedEvent(value)),
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

    /// create a mode switch event that triggers a runnable in the `SwcInternalBehavior` when the mode is switched
    #[pyo3(signature = (name, runnable, activation, context_port, mode_declaration, /, second_mode_declaration=None))]
    #[pyo3(
        text_signature = "(self, name: str, runnable: RunnableEntity, activation: ModeActivationKind, context_port: PortPrototype, mode_declaration: ModeDeclaration, /, second_mode_declaration: Optional[ModeDeclaration] = None)"
    )]
    fn create_mode_switch_event(
        &self,
        name: &str,
        runnable: &RunnableEntity,
        activation: ModeActivationKind,
        context_port: &Bound<'_, PyAny>,
        mode_declaration: &ModeDeclaration,
        second_mode_declaration: Option<&ModeDeclaration>,
    ) -> PyResult<SwcModeSwitchEvent> {
        let context_port = pyany_to_port_prototype(context_port)?;
        match self.0.create_mode_switch_event(
            name,
            &runnable.0,
            activation.into(),
            &context_port,
            &mode_declaration.0,
            second_mode_declaration.map(|m| &m.0),
        ) {
            Ok(value) => Ok(SwcModeSwitchEvent(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create an iterator over all events in the `SwcInternalBehavior`
    fn events(&self) -> RteEventIterator {
        RteEventIterator::new(
            self.0
                .events()
                .filter_map(|event| rte_event_to_pyany(event).ok()),
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
    fn events(&self) -> Vec<Py<PyAny>> {
        self.0
            .events()
            .into_iter()
            .filter_map(|event| rte_event_to_pyany(event).ok())
            .collect()
    }

    /// add implicit read access to a data element of a sender-receiver `PortPrototype`
    ///
    /// this results in `Rte_IRead_<port>_<data_element>` being generated
    #[pyo3(signature = (name, data_element, context_port, /))]
    #[pyo3(
        text_signature = "(self, name: str, data_element: VariableDataPrototype, port: PortPrototype, /)"
    )]
    fn create_data_read_access(
        &self,
        name: &str,
        data_element: &VariableDataPrototype,
        context_port: &Bound<'_, PyAny>,
    ) -> PyResult<VariableAccess> {
        let context_port = pyany_to_port_prototype(context_port)?;
        match self
            .0
            .create_data_read_access(name, &data_element.0, &context_port)
        {
            Ok(value) => Ok(VariableAccess(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all data read accesses
    fn data_read_accesses(&self) -> VariableAccessIterator {
        VariableAccessIterator::new(self.0.data_read_accesses().map(VariableAccess))
    }

    /// add implicit write access to a data element of a sender-receiver `PortPrototype`
    ///
    /// this results in `Rte_IWrite_<port>_<data_element>` being generated
    #[pyo3(signature = (name, data_element, context_port, /))]
    #[pyo3(
        text_signature = "(self, name: str, data_element: VariableDataPrototype, port: PortPrototype, /)"
    )]
    fn create_data_write_access(
        &self,
        name: &str,
        data_element: &VariableDataPrototype,
        context_port: &Bound<'_, PyAny>,
    ) -> PyResult<VariableAccess> {
        let context_port = pyany_to_port_prototype(context_port)?;
        match self
            .0
            .create_data_write_access(name, &data_element.0, &context_port)
        {
            Ok(value) => Ok(VariableAccess(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all data write accesses
    fn data_write_accesses(&self) -> VariableAccessIterator {
        VariableAccessIterator::new(self.0.data_write_accesses().map(VariableAccess))
    }

    /// add a data send point to a data element of a sender-receiver `PortPrototype`
    #[pyo3(signature = (name, data_element, context_port, /))]
    #[pyo3(
        text_signature = "(self, name: str, data_element: VariableDataPrototype, port: PortPrototype, /)"
    )]
    fn create_data_send_point(
        &self,
        name: &str,
        data_element: &VariableDataPrototype,
        context_port: &Bound<'_, PyAny>,
    ) -> PyResult<VariableAccess> {
        let context_port = pyany_to_port_prototype(context_port)?;
        match self
            .0
            .create_data_send_point(name, &data_element.0, &context_port)
        {
            Ok(value) => Ok(VariableAccess(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all data send points
    fn data_send_points(&self) -> VariableAccessIterator {
        VariableAccessIterator::new(self.0.data_send_points().map(VariableAccess))
    }

    /// add explicit read access by argument to a data element of a sender-receiver `PortPrototype`
    ///
    /// this results in `Rte_Read_<port>_<data_element>(DataType* data)` being generated
    #[pyo3(signature = (name, data_element, context_port, /))]
    #[pyo3(
        text_signature = "(self, name: str, data_element: VariableDataPrototype, port: PortPrototype, /)"
    )]
    fn create_data_receive_point_by_argument(
        &self,
        name: &str,
        data_element: &VariableDataPrototype,
        context_port: &Bound<'_, PyAny>,
    ) -> PyResult<VariableAccess> {
        let context_port = pyany_to_port_prototype(context_port)?;
        match self
            .0
            .create_data_receive_point_by_argument(name, &data_element.0, &context_port)
        {
            Ok(value) => Ok(VariableAccess(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all data receive points by argument
    fn data_receive_points_by_argument(&self) -> VariableAccessIterator {
        VariableAccessIterator::new(self.0.data_receive_points_by_argument().map(VariableAccess))
    }

    /// add explicit read access by value to a data element of a sender-receiver `PortPrototype`
    #[pyo3(signature = (name, data_element, context_port, /))]
    #[pyo3(
        text_signature = "(self, name: str, data_element: VariableDataPrototype, port: PortPrototype, /)"
    )]
    fn create_data_receive_point_by_value(
        &self,
        name: &str,
        data_element: &VariableDataPrototype,
        context_port: &Bound<'_, PyAny>,
    ) -> PyResult<VariableAccess> {
        let context_port = pyany_to_port_prototype(context_port)?;
        match self
            .0
            .create_data_receive_point_by_value(name, &data_element.0, &context_port)
        {
            Ok(value) => Ok(VariableAccess(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all data receive points by value
    fn data_receive_points_by_value(&self) -> VariableAccessIterator {
        VariableAccessIterator::new(self.0.data_receive_points_by_value().map(VariableAccess))
    }

    /// create a synchronous server call point that allows the runnable to call a server operation
    fn create_synchronous_server_call_point(
        &self,
        name: &str,
        client_server_operation: &ClientServerOperation,
        context_r_port: &RPortPrototype,
    ) -> PyResult<SynchronousServerCallPoint> {
        match self.0.create_synchronous_server_call_point(
            name,
            &client_server_operation.0,
            &context_r_port.0,
        ) {
            Ok(value) => Ok(SynchronousServerCallPoint(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all synchronous server call points
    fn synchronous_server_call_points(&self) -> SynchronousServerCallPointIterator {
        SynchronousServerCallPointIterator::new(
            self.0
                .synchronous_server_call_points()
                .map(SynchronousServerCallPoint),
        )
    }

    /// create a mode access point that allows the runnable to access the current mode of a ModeDeclarationGroup
    fn create_mode_access_point(
        &self,
        name: &str,
        mode_group: &ModeGroup,
        context_port: &Bound<'_, PyAny>,
    ) -> PyResult<ModeAccessPoint> {
        let context_port = pyany_to_port_prototype(context_port)?;
        match self
            .0
            .create_mode_access_point(name, &mode_group.0, &context_port)
        {
            Ok(value) => Ok(ModeAccessPoint(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all mode access points
    fn mode_access_points(&self) -> ModeAccessPointIterator {
        ModeAccessPointIterator::new(self.0.mode_access_points().map(ModeAccessPoint))
    }

    /// create a mode switch point that allows the runnable to switch modes in a ModeDeclarationGroup
    fn create_mode_switch_point(
        &self,
        name: &str,
        mode_group: &ModeGroup,
        context_port: &Bound<'_, PyAny>,
    ) -> PyResult<ModeSwitchPoint> {
        let context_port = pyany_to_port_prototype(context_port)?;
        match self
            .0
            .create_mode_switch_point(name, &mode_group.0, &context_port)
        {
            Ok(value) => Ok(ModeSwitchPoint(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all mode switch points
    fn mode_switch_points(&self) -> ModeSwitchPointIterator {
        ModeSwitchPointIterator::new(self.0.mode_switch_points().map(ModeSwitchPoint))
    }
}

//##################################################################

iterator_wrapper!(RunnableEntityIterator, RunnableEntity);

//##################################################################

/// A `VariableAccess` allows a `RunnableEntity` to access a variable in various contexts
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct VariableAccess(
    pub(crate) autosar_data_abstraction::software_component::VariableAccess,
);

#[pymethods]
impl VariableAccess {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::VariableAccess::try_from(
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

    /// Set the accessed variable
    #[pyo3(signature = (variable, context_port, /))]
    #[pyo3(
        text_signature = "(self, variable: VariableDataPrototype, context_port: PortPrototype, /)"
    )]
    fn set_accessed_variable(
        &self,
        variable: &VariableDataPrototype,
        context_port: &Bound<'_, PyAny>,
    ) -> PyResult<()> {
        let context_port = pyany_to_port_prototype(context_port)?;
        self.0
            .set_accessed_variable(&variable.0, &context_port)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the accessed variable
    #[getter]
    fn accessed_variable(&self) -> Option<(VariableDataPrototype, Py<PyAny>)> {
        let (variable_data_prototype, context_port) = self.0.accessed_variable()?;
        let variable = VariableDataPrototype(variable_data_prototype);
        let context_port = port_prototype_to_pyany(context_port).ok()?;
        Some((variable, context_port))
    }

    /// Get the `RunnableEntity` that contains the `VariableAccess`
    #[getter]
    fn runnable_entity(&self) -> Option<RunnableEntity> {
        self.0.runnable_entity().map(RunnableEntity)
    }
}

//##################################################################

iterator_wrapper!(VariableAccessIterator, VariableAccess);

//##################################################################

/// A `SynchronousServerCallPoint` allows a `RunnableEntity` to call a server operation synchronously
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct SynchronousServerCallPoint(
    pub(crate) autosar_data_abstraction::software_component::SynchronousServerCallPoint,
);

#[pymethods]
impl SynchronousServerCallPoint {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::SynchronousServerCallPoint::try_from(
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

    /// Set the client server operation
    #[pyo3(signature = (client_server_operation, context_r_port, /))]
    #[pyo3(
        text_signature = "(self, client_server_operation: ClientServerOperation, context_r_port: RPortPrototype, /)"
    )]
    fn set_client_server_operation(
        &self,
        client_server_operation: &ClientServerOperation,
        context_r_port: &RPortPrototype,
    ) -> PyResult<()> {
        self.0
            .set_client_server_operation(&client_server_operation.0, &context_r_port.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the client server operation
    #[getter]
    fn client_server_operation(&self) -> Option<(ClientServerOperation, RPortPrototype)> {
        let (client_server_operation, context_r_port) = self.0.client_server_operation()?;
        let client_server_operation = ClientServerOperation(client_server_operation);
        let context_r_port = RPortPrototype(context_r_port);
        Some((client_server_operation, context_r_port))
    }

    /// Get the `RunnableEntity` that contains the `SynchronousServerCallPoint`
    #[getter]
    fn runnable_entity(&self) -> Option<RunnableEntity> {
        self.0.runnable_entity().map(RunnableEntity)
    }
}

//##################################################################

iterator_wrapper!(
    SynchronousServerCallPointIterator,
    SynchronousServerCallPoint
);

//##################################################################

/// A `ModeAccessPoint`provides the ability to access the current mode of a ModeDeclarationGroup
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct ModeAccessPoint(
    pub(crate) autosar_data_abstraction::software_component::ModeAccessPoint,
);

#[pymethods]
impl ModeAccessPoint {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::ModeAccessPoint::try_from(
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

    /// Set the mode group and context port of the `ModeAccessPoint`
    #[pyo3(signature = (mode_group, context_port, /))]
    #[pyo3(text_signature = "(self, mode_group: ModeGroup, context_port: PortPrototype, /)")]
    fn set_mode_group(
        &self,
        mode_group: &ModeGroup,
        context_port: &Bound<'_, PyAny>,
    ) -> PyResult<()> {
        let context_port = pyany_to_port_prototype(context_port)?;
        self.0
            .set_mode_group(&mode_group.0, &context_port)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the mode group and context port of the `ModeAccessPoint`
    #[getter]
    fn mode_group(&self) -> Option<(ModeGroup, Py<PyAny>)> {
        let (mode_group, context_port) = self.0.mode_group()?;
        let mode_group = ModeGroup(mode_group);
        let context_port = port_prototype_to_pyany(context_port).ok()?;
        Some((mode_group, context_port))
    }

    /// Get the `RunnableEntity` that contains the `ModeAccessPoint`
    #[getter]
    fn runnable_entity(&self) -> Option<RunnableEntity> {
        self.0.runnable_entity().map(RunnableEntity)
    }
}

//##################################################################

iterator_wrapper!(ModeAccessPointIterator, ModeAccessPoint);

//##################################################################

/// A `ModeSwitchPoint` allows a `RunnableEntity` to switch modes in a ModeDeclarationGroup
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct ModeSwitchPoint(
    pub(crate) autosar_data_abstraction::software_component::ModeSwitchPoint,
);

#[pymethods]
impl ModeSwitchPoint {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::ModeSwitchPoint::try_from(
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

    /// Set the mode group and context port of the `ModeSwitchPoint`
    #[pyo3(signature = (mode_group, context_port, /))]
    #[pyo3(text_signature = "(self, mode_group: ModeGroup, context_port: PortPrototype, /)")]
    fn set_mode_group(
        &self,
        mode_group: &ModeGroup,
        context_port: &Bound<'_, PyAny>,
    ) -> PyResult<()> {
        let context_port = pyany_to_port_prototype(context_port)?;
        self.0
            .set_mode_group(&mode_group.0, &context_port)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the mode group and context port of the `ModeAccessPoint`
    #[getter]
    fn mode_group(&self) -> Option<(ModeGroup, Py<PyAny>)> {
        let (mode_group, context_port) = self.0.mode_group()?;
        let mode_group = ModeGroup(mode_group);
        let context_port = port_prototype_to_pyany(context_port).ok()?;
        Some((mode_group, context_port))
    }

    /// Get the `RunnableEntity` that contains the `ModeAccessPoint`
    #[getter]
    fn runnable_entity(&self) -> Option<RunnableEntity> {
        self.0.runnable_entity().map(RunnableEntity)
    }
}

//##################################################################

iterator_wrapper!(ModeSwitchPointIterator, ModeSwitchPoint);
