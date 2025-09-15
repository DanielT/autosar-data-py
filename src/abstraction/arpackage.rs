use crate::abstraction::{
    AutosarAbstractionError, ByteOrder, System, abstraction_err_to_pyerr,
    communication::{
        DataTransformationSet, RequestResponseDelay, SomeipSdClientEventGroupTimingConfig,
        SomeipSdClientServiceInstanceConfig, SomeipSdServerEventGroupTimingConfig,
        SomeipSdServerServiceInstanceConfig, SystemSignal, SystemSignalGroup,
    },
    datatype::{
        ApplicationArrayDataType, ApplicationArraySize, ApplicationPrimitiveCategory,
        ApplicationPrimitiveDataType, ApplicationRecordDataType, BaseTypeEncoding, CompuMethod,
        ConstantSpecification, DataConstr, DataTypeMappingSet, ImplementationDataType, SwBaseType,
        Unit, pyany_to_compu_method_content, pyany_to_implmentation_settings,
        pyany_to_value_specification,
    },
    ecu_configuration::{
        EcucDefinitionCollection, EcucDestinationUriDefSet, EcucModuleConfigurationValues,
        EcucModuleDef, EcucValueCollection,
    },
    software_component::{
        ApplicationSwComponentType, ClientServerInterface, ComplexDeviceDriverSwComponentType,
        CompositionSwComponentType, EcuAbstractionSwComponentType, ModeDeclarationGroup,
        ModeDeclarationGroupCategory, ModeSwitchInterface, NvDataInterface, ParameterInterface,
        SenderReceiverInterface, SensorActuatorSwComponentType, ServiceSwComponentType,
        TriggerInterface,
    },
    system::SystemCategory,
};
use crate::{Element, iterator_wrapper};
use autosar_data_abstraction::AbstractionElement;
use autosar_data_abstraction::{self, IdentifiableAbstractionElement};
use pyo3::prelude::*;

//##################################################################

/// An `ArPackage` is an Autosar package, which can contain other packages or elements
#[pyclass(frozen, eq, module = "autosar_data._autosar_data._abstraction")]
#[derive(Clone, PartialEq)]
pub(crate) struct ArPackage(pub(crate) autosar_data_abstraction::ArPackage);

#[pymethods]
impl ArPackage {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::ArPackage::try_from(element.0.clone()) {
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

    /// create a new `ApplicationArrayDataType` in the package
    #[pyo3(
        text_signature = "(self, name: str, element_type: ApplicationDataType, size: ApplicationArraySize, /)"
    )]
    fn create_application_array_data_type(
        &self,
        name: &str,
        element_type: &Bound<'_, PyAny>,
        size: ApplicationArraySize,
    ) -> PyResult<ApplicationArrayDataType> {
        let element_type = if let Ok(app_array) = element_type.extract::<ApplicationArrayDataType>()
        {
            autosar_data_abstraction::datatype::ApplicationDataType::Array(app_array.0)
        } else if let Ok(app_record) = element_type.extract::<ApplicationRecordDataType>() {
            autosar_data_abstraction::datatype::ApplicationDataType::Record(app_record.0)
        } else if let Ok(app_primitive) = element_type.extract::<ApplicationPrimitiveDataType>() {
            autosar_data_abstraction::datatype::ApplicationDataType::Primitive(app_primitive.0)
        } else {
            return Err(AutosarAbstractionError::new_err(
                "element_type must be an ApplicationArrayDataType, ApplicationRecordDataType, or ApplicationPrimitiveDataType",
            ));
        };
        match self
            .0
            .create_application_array_data_type(name, &element_type, size.into())
        {
            Ok(value) => Ok(ApplicationArrayDataType(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `ApplicationPrimitiveDataType` in the package
    #[pyo3(signature = (name, category, /, *, compu_method=None, unit=None, data_constraint=None))]
    #[pyo3(
        text_signature = "(self, name: str, category: ApplicationPrimitiveCategory, /, *, compu_method: CompuMethod = None, unit: Unit = None, data_constraint: DataConstr = None)"
    )]
    fn create_application_primitive_data_type(
        &self,
        name: &str,
        category: ApplicationPrimitiveCategory,
        compu_method: Option<&CompuMethod>,
        unit: Option<&Unit>,
        data_constraint: Option<&DataConstr>,
    ) -> PyResult<ApplicationPrimitiveDataType> {
        let compu_method = compu_method.map(|cm| cm.0.clone());
        let unit = unit.map(|u| u.0.clone());
        let data_constraint = data_constraint.map(|dc| dc.0.clone());
        match self.0.create_application_primitive_data_type(
            name,
            category.into(),
            compu_method.as_ref(),
            unit.as_ref(),
            data_constraint.as_ref(),
        ) {
            Ok(value) => Ok(ApplicationPrimitiveDataType(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `ApplicationRecordDataType` in the package
    #[pyo3(text_signature = "(self, name: str)")]
    fn create_application_record_data_type(
        &self,
        name: &str,
    ) -> PyResult<ApplicationRecordDataType> {
        match self.0.create_application_record_data_type(name) {
            Ok(value) => Ok(ApplicationRecordDataType(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `ApplicationSwComponentType` in the package
    #[pyo3(text_signature = "(self, name: str)")]
    fn create_application_sw_component_type(
        &self,
        name: &str,
    ) -> PyResult<ApplicationSwComponentType> {
        match self.0.create_application_sw_component_type(name) {
            Ok(value) => Ok(ApplicationSwComponentType(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `ClientServerInterface` in the package
    #[pyo3(text_signature = "(self, name: str)")]
    fn create_client_server_interface(&self, name: &str) -> PyResult<ClientServerInterface> {
        match self.0.create_client_server_interface(name) {
            Ok(value) => Ok(ClientServerInterface(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `ComplexDeviceDriverSwComponentType` in the package
    #[pyo3(text_signature = "(self, name: str)")]
    fn create_complex_device_driver_sw_component_type(
        &self,
        name: &str,
    ) -> PyResult<ComplexDeviceDriverSwComponentType> {
        match self.0.create_complex_device_driver_sw_component_type(name) {
            Ok(value) => Ok(ComplexDeviceDriverSwComponentType(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `CompositionSwComponentType` in the package
    #[pyo3(text_signature = "(self, name: str)")]
    fn create_composition_sw_component_type(
        &self,
        name: &str,
    ) -> PyResult<CompositionSwComponentType> {
        match self.0.create_composition_sw_component_type(name) {
            Ok(value) => Ok(CompositionSwComponentType(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `CompuMethod` in the package
    #[pyo3(text_signature = "(self, name: str, content: CompuMethodContent)")]
    fn create_compu_method(&self, name: &str, content: &Bound<'_, PyAny>) -> PyResult<CompuMethod> {
        let content = pyany_to_compu_method_content(content)?;
        match self.0.create_compu_method(name, content) {
            Ok(value) => Ok(CompuMethod(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `ConstantSpecification` in the package
    #[pyo3(text_signature = "(self, name: str, value: ValueSpecification)")]
    pub fn create_constant_specification(
        &self,
        name: &str,
        value: &Bound<'_, PyAny>,
    ) -> PyResult<ConstantSpecification> {
        let value = pyany_to_value_specification(value)?;
        match self.0.create_constant_specification(name, value) {
            Ok(value) => Ok(ConstantSpecification(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `DataConstr` in the package
    #[pyo3(text_signature = "(self, name: str)")]
    fn create_data_constr(&self, name: &str) -> PyResult<DataConstr> {
        match self.0.create_data_constr(name) {
            Ok(value) => Ok(DataConstr(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `DataTransformationSet` in the package
    #[pyo3(text_signature = "(self, name: str)")]
    fn create_data_transformation_set(&self, name: &str) -> PyResult<DataTransformationSet> {
        match self.0.create_data_transformation_set(name) {
            Ok(value) => Ok(DataTransformationSet(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `DataTypeMappingSet` in the package
    #[pyo3(text_signature = "(self, name: str)")]
    fn create_data_type_mapping_set(&self, name: &str) -> PyResult<DataTypeMappingSet> {
        match self.0.create_data_type_mapping_set(name) {
            Ok(value) => Ok(DataTypeMappingSet(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `EcuAbstractionSwComponentType` in the package
    #[pyo3(text_signature = "(self, name: str)")]
    fn create_ecu_abstraction_sw_component_type(
        &self,
        name: &str,
    ) -> PyResult<EcuAbstractionSwComponentType> {
        match self.0.create_ecu_abstraction_sw_component_type(name) {
            Ok(value) => Ok(EcuAbstractionSwComponentType(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `EcucDefinitionCollection` in the package
    #[pyo3(text_signature = "(self, name: str)")]
    fn create_ecuc_definition_collection(&self, name: &str) -> PyResult<EcucDefinitionCollection> {
        match self.0.create_ecuc_definition_collection(name) {
            Ok(value) => Ok(EcucDefinitionCollection(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `EcucDestinationUriDefSet` in the package
    #[pyo3(text_signature = "(self, name: str)")]
    fn create_ecuc_destination_uri_def_set(
        &self,
        name: &str,
    ) -> PyResult<EcucDestinationUriDefSet> {
        match self.0.create_ecuc_destination_uri_def_set(name) {
            Ok(value) => Ok(EcucDestinationUriDefSet(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `EcucModuleConfigurationValues` in the package
    #[pyo3(text_signature = "(self, name: str, definition: EcucModuleDef)")]
    fn create_ecuc_module_configuration_values(
        &self,
        name: &str,
        definition: &EcucModuleDef,
    ) -> PyResult<EcucModuleConfigurationValues> {
        match self
            .0
            .create_ecuc_module_configuration_values(name, &definition.0)
        {
            Ok(value) => Ok(EcucModuleConfigurationValues(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `EcucModuleDef` in the package
    #[pyo3(text_signature = "(self, name: str)")]
    fn create_ecuc_module_def(&self, name: &str) -> PyResult<EcucModuleDef> {
        match self.0.create_ecuc_module_def(name) {
            Ok(value) => Ok(EcucModuleDef(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `EcucValueCollection` in the package
    #[pyo3(text_signature = "(self, name: str)")]
    fn create_ecuc_value_collection(&self, name: &str) -> PyResult<EcucValueCollection> {
        match self.0.create_ecuc_value_collection(name) {
            Ok(value) => Ok(EcucValueCollection(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `ImplementationDataType` in the package
    #[pyo3(text_signature = "(self, settings: ImplementationDataTypeSettings)")]
    fn create_implementation_data_type(
        &self,
        settings: &Bound<'_, PyAny>,
    ) -> PyResult<ImplementationDataType> {
        let settings = pyany_to_implmentation_settings(settings)?;
        match self.0.create_implementation_data_type(&settings) {
            Ok(value) => Ok(ImplementationDataType(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `ModeDeclarationGroup` in the package
    #[pyo3(signature = (name, *, category=None))]
    #[pyo3(
        text_signature = "(self, name: str, *, category: Optional[ModeDeclarationGroupCategory] = None)"
    )]
    fn create_mode_declaration_group(
        &self,
        name: &str,
        category: Option<ModeDeclarationGroupCategory>,
    ) -> PyResult<ModeDeclarationGroup> {
        match self
            .0
            .create_mode_declaration_group(name, category.map(Into::into))
        {
            Ok(value) => Ok(ModeDeclarationGroup(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `ModeSwitchInterface` in the package
    #[pyo3(text_signature = "(self, name: str)")]
    fn create_mode_switch_interface(&self, name: &str) -> PyResult<ModeSwitchInterface> {
        match self.0.create_mode_switch_interface(name) {
            Ok(value) => Ok(ModeSwitchInterface(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `NvDataInterface` in the package
    #[pyo3(text_signature = "(self, name: str)")]
    fn create_nv_data_interface(&self, name: &str) -> PyResult<NvDataInterface> {
        match self.0.create_nv_data_interface(name) {
            Ok(value) => Ok(NvDataInterface(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `ParameterInterface` in the package
    #[pyo3(text_signature = "(self, name: str)")]
    fn create_parameter_interface(&self, name: &str) -> PyResult<ParameterInterface> {
        match self.0.create_parameter_interface(name) {
            Ok(value) => Ok(ParameterInterface(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `SenderReceiverInterface` in the package
    #[pyo3(text_signature = "(self, name: str)")]
    fn create_sender_receiver_interface(&self, name: &str) -> PyResult<SenderReceiverInterface> {
        match self.0.create_sender_receiver_interface(name) {
            Ok(value) => Ok(SenderReceiverInterface(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `SensorActuatorSwComponentType` in the package
    #[pyo3(text_signature = "(self, name: str)")]
    fn create_sensor_actuator_sw_component_type(
        &self,
        name: &str,
    ) -> PyResult<SensorActuatorSwComponentType> {
        match self.0.create_sensor_actuator_sw_component_type(name) {
            Ok(value) => Ok(SensorActuatorSwComponentType(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `ServiceSwComponentType` in the package
    #[pyo3(text_signature = "(self, name: str)")]
    fn create_service_sw_component_type(&self, name: &str) -> PyResult<ServiceSwComponentType> {
        match self.0.create_service_sw_component_type(name) {
            Ok(value) => Ok(ServiceSwComponentType(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `SomeipSdClientEventGroupTimingConfig` in the package
    #[pyo3(text_signature = "(self, name: str, time_to_live: int)")]
    fn create_someip_sd_client_event_group_timing_config(
        &self,
        name: &str,
        time_to_live: u32,
    ) -> PyResult<SomeipSdClientEventGroupTimingConfig> {
        match self
            .0
            .create_someip_sd_client_event_group_timing_config(name, time_to_live)
        {
            Ok(value) => Ok(SomeipSdClientEventGroupTimingConfig(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `SomeipSdClientServiceInstanceConfig` in the package
    #[pyo3(text_signature = "(self, name: str)")]
    fn create_someip_sd_client_service_instance_config(
        &self,
        name: &str,
    ) -> PyResult<SomeipSdClientServiceInstanceConfig> {
        match self.0.create_someip_sd_client_service_instance_config(name) {
            Ok(value) => Ok(SomeipSdClientServiceInstanceConfig(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `SomeipSdServerEventGroupTimingConfig` in the package
    #[pyo3(text_signature = "(self, name: str, request_response_delay: RequestResponseDelay)")]
    fn create_someip_sd_server_event_group_timing_config(
        &self,
        name: &str,
        request_response_delay: &RequestResponseDelay,
    ) -> PyResult<SomeipSdServerEventGroupTimingConfig> {
        match self
            .0
            .create_someip_sd_server_event_group_timing_config(name, &request_response_delay.0)
        {
            Ok(value) => Ok(SomeipSdServerEventGroupTimingConfig(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `SomeipSdServerServiceInstanceConfig` in the package
    #[pyo3(text_signature = "(self, name: str, ttl: int)")]
    fn create_someip_sd_server_service_instance_config(
        &self,
        name: &str,
        ttl: u32,
    ) -> PyResult<SomeipSdServerServiceInstanceConfig> {
        match self
            .0
            .create_someip_sd_server_service_instance_config(name, ttl)
        {
            Ok(value) => Ok(SomeipSdServerServiceInstanceConfig(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `SwBaseType` in the package
    #[pyo3(signature = (name, bit_length, base_type_encoding, /, *, byte_order=None, mem_alignment=None, native_declaration=None))]
    #[pyo3(
        text_signature = "(self, name: str, bit_length: int, base_type_encoding: BaseTypeEncoding, /, *, byte_order: Optional[ByteOrder] = None, mem_alignment: Optional[int] = None, native_declaration: Optional[str] = None)"
    )]
    fn create_sw_base_type(
        &self,
        name: &str,
        bit_length: u32,
        base_type_encoding: BaseTypeEncoding,
        byte_order: Option<ByteOrder>,
        mem_alignment: Option<u32>,
        native_declaration: Option<&str>,
    ) -> PyResult<SwBaseType> {
        match self.0.create_sw_base_type(
            name,
            bit_length,
            base_type_encoding.into(),
            byte_order.map(std::convert::Into::into),
            mem_alignment,
            native_declaration,
        ) {
            Ok(value) => Ok(SwBaseType(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new System in the package
    ///
    /// Note that an Autosar model should ony contain one SYSTEM. This is not checked here.
    #[pyo3(signature = (name, category, /))]
    #[pyo3(text_signature = "(self, name: str, category: SystemCategory, /)")]
    fn create_system(&self, name: &str, category: SystemCategory) -> PyResult<System> {
        match self.0.create_system(name, category.into()) {
            Ok(system) => Ok(System(system)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `SystemSignal` in the package
    #[pyo3(text_signature = "(self, name: str)")]
    fn create_system_signal(&self, name: &str) -> PyResult<SystemSignal> {
        match self.0.create_system_signal(name) {
            Ok(value) => Ok(SystemSignal(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `SystemSignalGroup` in the package
    #[pyo3(text_signature = "(self, name: str)")]
    fn create_system_signal_group(&self, name: &str) -> PyResult<SystemSignalGroup> {
        match self.0.create_system_signal_group(name) {
            Ok(value) => Ok(SystemSignalGroup(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `TriggerInterface` in the package
    #[pyo3(text_signature = "(self, name: str)")]
    fn create_trigger_interface(&self, name: &str) -> PyResult<TriggerInterface> {
        match self.0.create_trigger_interface(name) {
            Ok(value) => Ok(TriggerInterface(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `Unit` in the package
    #[pyo3(signature = (name, /, *, display_name=None))]
    #[pyo3(text_signature = "(self, name: str, /, *, display_name: Optional[str] = None)")]
    fn create_unit(&self, name: &str, display_name: Option<&str>) -> PyResult<Unit> {
        match self.0.create_unit(name, display_name) {
            Ok(value) => Ok(Unit(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a sub-package in the package
    #[pyo3(signature = (name, /))]
    #[pyo3(text_signature = "(self, name: str, /)")]
    fn create_sub_package(&self, name: &str) -> PyResult<ArPackage> {
        match self.0.create_sub_package(name) {
            Ok(value) => Ok(ArPackage(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all sub-packages in the package
    #[pyo3(text_signature = "(self, /)")]
    fn sub_packages(&self) -> ArPackagesIterator {
        ArPackagesIterator::new(self.0.sub_packages().map(ArPackage))
    }

    /// iterate over all elements in the package
    #[pyo3(text_signature = "(self)")]
    fn elements(&self) -> ElementsIterator {
        ElementsIterator::new(self.0.elements().map(Element))
    }
}

iterator_wrapper!(ElementsIterator, Element);
iterator_wrapper!(ArPackagesIterator, ArPackage);
