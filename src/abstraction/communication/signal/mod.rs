use crate::{
    Element,
    abstraction::{
        AutosarAbstractionError, EcuInstance, abstraction_err_to_pyerr,
        communication::{
            CanPhysicalChannel, CommunicationDirection, DataTransformation,
            EndToEndTransformationISignalProps, EthernetPhysicalChannel, FlexrayPhysicalChannel,
            ISignalToIPduMapping, LinPhysicalChannel, SomeIpTransformationISignalProps,
            TransformationTechnology,
        },
        datatype::{
            CompuMethod, DataConstr, SwBaseType, Unit, pyany_to_value_specification,
            value_specification_to_pyany,
        },
    },
    iterator_wrapper,
};
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};
use pyo3::{IntoPyObjectExt, prelude::*};

//##################################################################

/// Signal of the Interaction Layer
#[pyclass(frozen, eq, module = "autosar.abstraction.communication")]
#[derive(Clone, PartialEq)]
pub(crate) struct ISignal(pub(crate) autosar_data_abstraction::communication::ISignal);

#[pymethods]
impl ISignal {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::ISignal::try_from(element.0.clone()) {
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

    /// set the data type for this signal
    #[setter]
    fn set_datatype(&self, datatype: &SwBaseType) -> PyResult<()> {
        self.0
            .set_datatype(&datatype.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the data type of this signal
    #[getter]
    fn datatype(&self) -> Option<SwBaseType> {
        self.0.datatype().map(SwBaseType)
    }

    /// set the length of this signal in bits
    #[setter]
    fn set_length(&self, bit_length: u64) -> PyResult<()> {
        self.0
            .set_length(bit_length)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the length of this signal in bits
    #[getter]
    fn length(&self) -> Option<u64> {
        self.0.length()
    }

    /// set the init value for this signal
    #[setter]
    fn set_init_value(&self, init_value: Option<&Bound<'_, PyAny>>) -> PyResult<()> {
        let init_value = init_value
            .map(|val| pyany_to_value_specification(val))
            .transpose()?;
        self.0
            .set_init_value(init_value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the init value for this signal
    #[getter]
    fn init_value(&self) -> Option<Py<PyAny>> {
        self.0
            .init_value()
            .and_then(|value_spec| value_specification_to_pyany(&value_spec).ok())
    }

    /// set the system signal that corresponds to this isignal
    #[setter]
    fn set_system_signal(&self, system_signal: &SystemSignal) -> PyResult<()> {
        self.0
            .set_system_signal(&system_signal.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the system signal that corresponds to this isignal
    #[getter]
    fn system_signal(&self) -> Option<SystemSignal> {
        self.0.system_signal().map(SystemSignal)
    }

    /// List all `ISignalToIPduMapping` for this signal
    ///
    /// Usually a signal should only be mapped to a single PDU,
    /// so this list is expected to contain either zero or one items in ordinary cases.
    fn mappings(&self) -> Vec<ISignalToIPduMapping> {
        self.0
            .mappings()
            .into_iter()
            .map(ISignalToIPduMapping)
            .collect()
    }

    /// list all `ISignalTriggering`s that trigger this signal
    fn signal_triggerings(&self) -> Vec<ISignalTriggering> {
        self.0
            .signal_triggerings()
            .into_iter()
            .map(ISignalTriggering)
            .collect()
    }

    /// get the signal group that contains this signal, if any
    #[getter]
    fn signal_group(&self) -> Option<ISignalGroup> {
        self.0.signal_group().map(ISignalGroup)
    }

    /// add a data transformation to this signal
    #[pyo3(signature = (data_transformation, /))]
    #[pyo3(text_signature = "(self, data_transformation: DataTransformation, /)")]
    fn add_data_transformation(&self, data_transformation: &DataTransformation) -> PyResult<()> {
        self.0
            .add_data_transformation(&data_transformation.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get all data transformations that are applied to this signal
    fn data_transformations(&self) -> DataTransformationIterator {
        DataTransformationIterator::new(self.0.data_transformations().map(DataTransformation))
    }

    /// create E2E transformation properties for this signal
    #[pyo3(signature = (transformer, /))]
    #[pyo3(text_signature = "(self, transformer: TransformationTechnology, /)")]
    fn create_e2e_transformation_isignal_props(
        &self,
        transformer: &TransformationTechnology,
    ) -> PyResult<EndToEndTransformationISignalProps> {
        match self
            .0
            .create_e2e_transformation_isignal_props(&transformer.0)
        {
            Ok(props) => Ok(EndToEndTransformationISignalProps(props)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// create SomeIp transformation properties for this signal
    #[pyo3(signature = (transformer, /))]
    #[pyo3(text_signature = "(self, transformer: TransformationTechnology, /)")]
    fn create_someip_transformation_isignal_props(
        &self,
        transformer: &TransformationTechnology,
    ) -> PyResult<SomeIpTransformationISignalProps> {
        match self
            .0
            .create_someip_transformation_isignal_props(&transformer.0)
        {
            Ok(props) => Ok(SomeIpTransformationISignalProps(props)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// get all transformation properties that are applied to this signal
    fn transformation_isignal_props(&self) -> TransformationISignalPropsIterator {
        TransformationISignalPropsIterator::new(self.0.transformation_isignal_props().filter_map(
            |props| match props {
                autosar_data_abstraction::communication::TransformationISignalProps::E2E(
                    end_to_end_transformation_isignal_props,
                ) => Python::attach(|py| {
                    EndToEndTransformationISignalProps(end_to_end_transformation_isignal_props)
                        .into_py_any(py)
                        .ok()
                }),
                autosar_data_abstraction::communication::TransformationISignalProps::SomeIp(
                    some_ip_transformation_isignal_props,
                ) => Python::attach(|py| {
                    SomeIpTransformationISignalProps(some_ip_transformation_isignal_props)
                        .into_py_any(py)
                        .ok()
                }),
            },
        ))
    }
}

//##################################################################

iterator_wrapper!(ISignalIterator, ISignal);
iterator_wrapper!(DataTransformationIterator, DataTransformation);

//##################################################################

/// The system signal represents the communication system's view of data exchanged between SW components which reside on different ECUs
///
/// Use [`ArPackage::create_system_signal`] to create a new system signal
#[pyclass(frozen, eq, module = "autosar.abstraction.communication")]
#[derive(Clone, PartialEq)]
pub(crate) struct SystemSignal(pub(crate) autosar_data_abstraction::communication::SystemSignal);

#[pymethods]
impl SystemSignal {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::SystemSignal::try_from(element.0.clone()) {
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

    /// get the signal group that contains this signal
    #[getter]
    fn signal_group(&self) -> Option<SystemSignalGroup> {
        self.0.signal_group().map(SystemSignalGroup)
    }

    /// set the unit for this signal
    #[setter]
    fn set_unit(&self, unit: &Unit) -> PyResult<()> {
        self.0.set_unit(&unit.0).map_err(abstraction_err_to_pyerr)
    }

    /// get the unit for this signal
    #[getter]
    fn unit(&self) -> Option<Unit> {
        self.0.unit().map(Unit)
    }

    /// set the compu method for this signal
    #[setter]
    fn set_compu_method(&self, compu_method: &CompuMethod) -> PyResult<()> {
        self.0
            .set_compu_method(&compu_method.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the compu method for this signal
    #[getter]
    fn compu_method(&self) -> Option<CompuMethod> {
        self.0.compu_method().map(CompuMethod)
    }

    /// set the data constraint for this signal
    #[setter]
    fn set_data_constr(&self, data_constr: &DataConstr) -> PyResult<()> {
        self.0
            .set_data_constr(&data_constr.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the data constraint for this signal
    #[getter]
    fn data_constr(&self) -> Option<DataConstr> {
        self.0.data_constr().map(DataConstr)
    }
}

//##################################################################

iterator_wrapper!(SystemSignalIterator, SystemSignal);

//##################################################################

/// An `ISignalGroup` groups signals that should always be kept together
#[pyclass(frozen, eq, module = "autosar.abstraction.communication")]
#[derive(Clone, PartialEq)]
pub(crate) struct ISignalGroup(pub(crate) autosar_data_abstraction::communication::ISignalGroup);

#[pymethods]
impl ISignalGroup {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::ISignalGroup::try_from(element.0.clone()) {
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

    /// Add a signal to the signal group
    fn add_signal(&self, signal: &ISignal) -> PyResult<()> {
        self.0
            .add_signal(&signal.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the system signal group that is associated with this signal group
    #[getter]
    fn system_signal_group(&self) -> Option<SystemSignalGroup> {
        self.0.system_signal_group().map(SystemSignalGroup)
    }

    /// Iterator over all [`ISignal`]s in this group
    ///
    /// # Example
    fn signals(&self) -> ISignalIterator {
        ISignalIterator::new(self.0.signals().map(ISignal))
    }

    /// add a data transformation to this signal group
    #[pyo3(signature = (data_transformation, /))]
    #[pyo3(text_signature = "(self, data_transformation: DataTransformation, /)")]
    fn add_data_transformation(&self, data_transformation: &DataTransformation) -> PyResult<()> {
        self.0
            .add_data_transformation(&data_transformation.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// iterate over all data transformations that are applied to this signal group
    fn data_transformations(&self) -> DataTransformationIterator {
        DataTransformationIterator::new(self.0.data_transformations().map(DataTransformation))
    }

    /// create E2E transformation properties for this signal group
    #[pyo3(signature = (transformer, /))]
    #[pyo3(text_signature = "(self, transformer: TransformationTechnology, /)")]
    fn create_e2e_transformation_isignal_props(
        &self,
        transformer: &TransformationTechnology,
    ) -> PyResult<EndToEndTransformationISignalProps> {
        match self
            .0
            .create_e2e_transformation_isignal_props(&transformer.0)
        {
            Ok(props) => Ok(EndToEndTransformationISignalProps(props)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// create SomeIp transformation properties for this signal group
    #[pyo3(signature = (transformer, /))]
    #[pyo3(text_signature = "(self, transformer: TransformationTechnology, /)")]
    fn create_someip_transformation_isignal_props(
        &self,
        transformer: &TransformationTechnology,
    ) -> PyResult<SomeIpTransformationISignalProps> {
        match self
            .0
            .create_someip_transformation_isignal_props(&transformer.0)
        {
            Ok(props) => Ok(SomeIpTransformationISignalProps(props)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// get all transformation properties that are applied to this signal group
    fn transformation_isignal_props(&self) -> TransformationISignalPropsIterator {
        TransformationISignalPropsIterator::new(self.0.transformation_isignal_props().filter_map(
            |props| match props {
                autosar_data_abstraction::communication::TransformationISignalProps::E2E(
                    end_to_end_transformation_isignal_props,
                ) => Python::attach(|py| {
                    EndToEndTransformationISignalProps(end_to_end_transformation_isignal_props)
                        .into_py_any(py)
                        .ok()
                }),
                autosar_data_abstraction::communication::TransformationISignalProps::SomeIp(
                    some_ip_transformation_isignal_props,
                ) => Python::attach(|py| {
                    SomeIpTransformationISignalProps(some_ip_transformation_isignal_props)
                        .into_py_any(py)
                        .ok()
                }),
            },
        ))
    }
}

//##################################################################

/// A signal group refers to a set of signals that shall always be kept together. A signal group is used to
/// guarantee the atomic transfer of AUTOSAR composite data types.
///
/// Use [`ArPackage::create_system_signal_group`] to create a new system signal group
#[pyclass(frozen, eq, module = "autosar.abstraction.communication")]
#[derive(Clone, PartialEq)]
pub(crate) struct SystemSignalGroup(
    pub(crate) autosar_data_abstraction::communication::SystemSignalGroup,
);

#[pymethods]
impl SystemSignalGroup {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::SystemSignalGroup::try_from(
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

    /// Add a signal to the signal group
    #[pyo3(signature = (signal, /))]
    #[pyo3(text_signature = "(self, signal: ISignal, /)")]
    fn add_signal(&self, signal: &SystemSignal) -> PyResult<()> {
        self.0
            .add_signal(&signal.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Iterator over all [`SystemSignal`]s in this group
    fn signals(&self) -> SystemSignalIterator {
        SystemSignalIterator::new(self.0.signals().map(SystemSignal))
    }
}

/// an `ISignalTriggering` triggers a signal in a PDU
#[pyclass(frozen, eq, module = "autosar.abstraction.communication")]
#[derive(Clone, PartialEq)]
pub(crate) struct ISignalTriggering(
    pub(crate) autosar_data_abstraction::communication::ISignalTriggering,
);

#[pymethods]
impl ISignalTriggering {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::ISignalTriggering::try_from(
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

    /// get the physical channel that contains this signal triggering
    #[getter]
    fn physical_channel(&self, py: Python) -> PyResult<Py<PyAny>> {
        match self.0.physical_channel() {
            Ok(physical_channel) => match physical_channel {
                autosar_data_abstraction::communication::PhysicalChannel::Can(
                    can_physical_channel,
                ) => CanPhysicalChannel(can_physical_channel).into_py_any(py),
                autosar_data_abstraction::communication::PhysicalChannel::Ethernet(
                    ethernet_physical_channel,
                ) => EthernetPhysicalChannel(ethernet_physical_channel).into_py_any(py),
                autosar_data_abstraction::communication::PhysicalChannel::Flexray(
                    flexray_physical_channel,
                ) => FlexrayPhysicalChannel(flexray_physical_channel).into_py_any(py),
                autosar_data_abstraction::communication::PhysicalChannel::Lin(
                    lin_physical_channel,
                ) => LinPhysicalChannel(lin_physical_channel).into_py_any(py),
            },
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// connect this signal triggering to an ECU
    #[pyo3(signature = (ecu, direction, /))]
    #[pyo3(text_signature = "(self, ecu: EcuInstance, direction: CommunicationDirection, /)")]
    fn connect_to_ecu(
        &self,
        ecu: &EcuInstance,
        direction: CommunicationDirection,
    ) -> PyResult<ISignalPort> {
        match self.0.connect_to_ecu(&ecu.0, direction.into()) {
            Ok(signal_port) => Ok(ISignalPort(signal_port)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// create an iterator over all signal ports that are connected to this signal triggering
    fn signal_ports(&self) -> ISignalPortIterator {
        ISignalPortIterator::new(self.0.signal_ports().map(ISignalPort))
    }
}

//##################################################################

iterator_wrapper!(ISignalPortIterator, ISignalPort);

//##################################################################

/// The `ISignalPort` allows an ECU to send or receive a Signal
#[pyclass(frozen, eq, module = "autosar.abstraction.communication")]
#[derive(Clone, PartialEq)]
pub(crate) struct ISignalPort(pub(crate) autosar_data_abstraction::communication::ISignalPort);

#[pymethods]
impl ISignalPort {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::ISignalPort::try_from(element.0.clone()) {
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

    /// get the ECU that is connected to this signal port
    #[getter]
    fn ecu(&self) -> PyResult<EcuInstance> {
        match self.0.ecu() {
            Ok(ecu) => Ok(EcuInstance(ecu)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// set the communication direction of this port
    #[setter]
    fn set_communication_direction(&self, direction: CommunicationDirection) -> PyResult<()> {
        self.0
            .set_communication_direction(direction.into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the communication direction of this port
    #[getter]
    fn communication_direction(&self) -> Option<CommunicationDirection> {
        self.0
            .communication_direction()
            .map(std::convert::Into::into)
    }
}

//##################################################################

/// The `TransferProperty` defines if or how the signal influences the transfer of the PDU
#[pyclass(frozen, eq, eq_int, module = "autosar.abstraction.communication")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferProperty {
    /// The signal is pending; it does not trigger the transfer of the PDU
    Pending,
    /// The signal triggers the transfer of the PDU
    Triggered,
    /// The signal triggers the transfer of the PDU if the value changes
    TriggeredOnChange,
    /// The signal triggers the transfer of the PDU if the value changes without repetition
    TriggeredOnChangeWithoutRepetition,
    /// The signal triggers the transfer of the PDU without repetition
    TriggeredWithoutRepetition,
}

impl From<autosar_data_abstraction::communication::TransferProperty> for TransferProperty {
    fn from(prop: autosar_data_abstraction::communication::TransferProperty) -> Self {
        match prop {
            autosar_data_abstraction::communication::TransferProperty::Pending => Self::Pending,
            autosar_data_abstraction::communication::TransferProperty::Triggered => Self::Triggered,
            autosar_data_abstraction::communication::TransferProperty::TriggeredOnChange => {
                Self::TriggeredOnChange
            }
            autosar_data_abstraction::communication::TransferProperty::TriggeredOnChangeWithoutRepetition => {
                Self::TriggeredOnChangeWithoutRepetition
            }
            autosar_data_abstraction::communication::TransferProperty::TriggeredWithoutRepetition => {
                Self::TriggeredWithoutRepetition
            }
        }
    }
}

impl From<TransferProperty> for autosar_data_abstraction::communication::TransferProperty {
    fn from(prop: TransferProperty) -> Self {
        match prop {
            TransferProperty::Pending => autosar_data_abstraction::communication::TransferProperty::Pending,
            TransferProperty::Triggered => autosar_data_abstraction::communication::TransferProperty::Triggered,
            TransferProperty::TriggeredOnChange => autosar_data_abstraction::communication::TransferProperty::TriggeredOnChange,
            TransferProperty::TriggeredOnChangeWithoutRepetition => autosar_data_abstraction::communication::TransferProperty::TriggeredOnChangeWithoutRepetition,
            TransferProperty::TriggeredWithoutRepetition => autosar_data_abstraction::communication::TransferProperty::TriggeredWithoutRepetition,
        }
    }
}

//##################################################################

iterator_wrapper!(
    TransformationISignalPropsIterator,
    Py<PyAny>,
    "Union[EndToEndTransformationISignalProps, SomeIpTransformationISignalProps]"
);
