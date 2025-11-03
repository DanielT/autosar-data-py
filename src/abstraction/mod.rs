use crate::{ArxmlFile, AutosarModel, AutosarVersion, Element, iterator_wrapper};
use pyo3::PyTypeInfo;
use pyo3::create_exception;
use pyo3::prelude::*;

mod communication;
mod datatype;
mod ecu_configuration;
mod software_component;

mod arpackage;
mod ecuinstance;
mod system;

pub(crate) use arpackage::ArPackage;
pub(crate) use ecuinstance::EcuInstance;
pub(crate) use system::System;

create_exception!(
    module.abstraction,
    AutosarAbstractionError,
    pyo3::exceptions::PyException
);

//##################################################################

#[pyclass(frozen, eq, module = "autosar_data._autosar_data._abstraction")]
#[derive(Clone, PartialEq)]
pub(crate) struct AutosarModelAbstraction(
    pub(crate) autosar_data_abstraction::AutosarModelAbstraction,
);

#[pymethods]
impl AutosarModelAbstraction {
    /// Create a new `AutosarModelAbstraction` from an existing `AutosarModel`
    #[new]
    fn new(model: &AutosarModel) -> Self {
        Self(autosar_data_abstraction::AutosarModelAbstraction::new(
            model.0.clone(),
        ))
    }

    /// create a new `AutosarModelAbstraction` with an empty `AutosarModel`
    #[staticmethod]
    #[pyo3(signature = (filename, /, *, version=None))]
    #[pyo3(text_signature = "(cls, filename: str, /, *, version: Optional[AutosarVersion] = None)")]
    fn create(filename: &str, version: Option<AutosarVersion>) -> Self {
        let version = version.unwrap_or(AutosarVersion::Latest);
        let model_abstraction =
            autosar_data_abstraction::AutosarModelAbstraction::create(filename, version.into());
        Self(model_abstraction)
    }

    /// create an `AutosarModelAbstraction` from a file on disk
    #[staticmethod]
    #[pyo3(signature = (filename, /))]
    #[pyo3(text_signature = "(cls, filename: str, /)")]
    fn from_file(filename: &str) -> PyResult<Self> {
        match autosar_data_abstraction::AutosarModelAbstraction::from_file(filename) {
            Ok(model_abstraction) => Ok(Self(model_abstraction)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// Get the underlying `AutosarModel` from the abstraction model
    #[getter]
    fn model(&self) -> AutosarModel {
        AutosarModel(self.0.model().clone())
    }

    /// Get the root element of the model
    #[getter]
    fn root_element(&self) -> Element {
        Element(self.0.root_element().clone())
    }

    /// iterate over all top-level packages
    fn packages(&self) -> ArPackageIterator {
        ArPackageIterator::new(self.0.packages().map(ArPackage))
    }

    /// Get a package by its path or create it if it does not exist
    #[pyo3(signature = (path, /))]
    #[pyo3(text_signature = "(path: str, /)")]
    fn get_or_create_package(&self, path: &str) -> PyResult<ArPackage> {
        match self.0.get_or_create_package(path) {
            Ok(package) => Ok(ArPackage(package)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// Create a new file in the model
    #[pyo3(signature = (filename, /, *, version=None))]
    #[pyo3(text_signature = "(filename: str, /, *, version: Optional[AutosarVersion] = None)")]
    fn create_file(&self, filename: &str, version: Option<AutosarVersion>) -> PyResult<ArxmlFile> {
        let version = version.unwrap_or(AutosarVersion::Latest);
        match self.0.create_file(filename, version.into()) {
            Ok(file) => Ok(ArxmlFile(file)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// Load a file into the model
    #[pyo3(signature = (filename, /, *, strict=false))]
    #[pyo3(text_signature = "(filename: str, /, * strict: bool = False)")]
    fn load_file(&self, filename: &str, strict: bool) -> PyResult<(ArxmlFile, Vec<String>)> {
        match self.0.load_file(filename, strict) {
            Ok((file, warn)) => {
                let warnstrings: Vec<String> =
                    warn.iter().map(std::string::ToString::to_string).collect();
                Ok((ArxmlFile(file), warnstrings))
            }
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// iterate over all files in the model
    fn files(&self) -> Vec<ArxmlFile> {
        self.0.files().map(ArxmlFile).collect()
    }

    /// write the model to disk, creating or updating all files in the model
    fn write(&self) -> PyResult<()> {
        self.0.write().map_err(abstraction_err_to_pyerr)
    }

    /// Get an element by its path
    #[pyo3(signature = (path, /))]
    #[pyo3(text_signature = "(path: str, /)")]
    fn get_element_by_path(&self, path: &str) -> Option<Element> {
        self.0.get_element_by_path(path).map(Element)
    }

    /// find an existing SYSTEM in the model, if it exists
    #[pyo3(text_signature = "(self, /)")]
    fn find_system(&self) -> Option<System> {
        self.0.find_system().map(System)
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self.0)
    }
}

//##################################################################

iterator_wrapper!(ArPackageIterator, ArPackage);

//##################################################################

/// The `ByteOrder` is used to define the order of bytes in a multi-byte value
#[pyclass(frozen, eq, eq_int, module = "autosar_data._autosar_data._abstraction")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ByteOrder {
    /// Most significant byte at the lowest address = big endian
    MostSignificantByteFirst,
    /// Most significant byte at the highest address = little endian
    MostSignificantByteLast,
    /// The byte order is not defined / not relevant
    Opaque,
}

impl From<autosar_data_abstraction::ByteOrder> for ByteOrder {
    fn from(byte_order: autosar_data_abstraction::ByteOrder) -> Self {
        match byte_order {
            autosar_data_abstraction::ByteOrder::MostSignificantByteFirst => {
                ByteOrder::MostSignificantByteFirst
            }
            autosar_data_abstraction::ByteOrder::MostSignificantByteLast => {
                ByteOrder::MostSignificantByteLast
            }
            autosar_data_abstraction::ByteOrder::Opaque => ByteOrder::Opaque,
        }
    }
}

impl From<ByteOrder> for autosar_data_abstraction::ByteOrder {
    fn from(byte_order: ByteOrder) -> Self {
        match byte_order {
            ByteOrder::MostSignificantByteFirst => {
                autosar_data_abstraction::ByteOrder::MostSignificantByteFirst
            }
            ByteOrder::MostSignificantByteLast => {
                autosar_data_abstraction::ByteOrder::MostSignificantByteLast
            }
            ByteOrder::Opaque => autosar_data_abstraction::ByteOrder::Opaque,
        }
    }
}

//##################################################################

pub(crate) fn add_submodules(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let abstraction = PyModule::new(py, "_abstraction")?;
    parent.add_submodule(&abstraction)?;
    abstraction.add_class::<AutosarModelAbstraction>()?;
    abstraction.add_class::<ByteOrder>()?;
    abstraction.add_class::<arpackage::ArPackage>()?;
    abstraction.add_class::<ecuinstance::EcuInstance>()?;
    abstraction.add_class::<system::SwcToEcuMapping>()?;
    abstraction.add_class::<system::System>()?;
    abstraction.add_class::<system::SystemCategory>()?;
    abstraction.add_class::<system::SystemMapping>()?;

    let communication = PyModule::new(py, "_communication")?;
    abstraction.add_submodule(&communication)?;
    communication.add_class::<communication::CanAddressingMode>()?;
    communication.add_class::<communication::CanCluster>()?;
    communication.add_class::<communication::CanCommunicationConnector>()?;
    communication.add_class::<communication::CanCommunicationController>()?;
    communication.add_class::<communication::CanFrame>()?;
    communication.add_class::<communication::CanFrameTriggering>()?;
    communication.add_class::<communication::CanFrameType>()?;
    communication.add_class::<communication::CanNmCluster>()?;
    communication.add_class::<communication::CanNmClusterCoupling>()?;
    communication.add_class::<communication::CanNmClusterSettings>()?;
    communication.add_class::<communication::CanNmNode>()?;
    communication.add_class::<communication::CanPhysicalChannel>()?;
    communication.add_class::<communication::CanTpAddress>()?;
    communication.add_class::<communication::CanTpAddressingFormat>()?;
    communication.add_class::<communication::CanTpChannel>()?;
    communication.add_class::<communication::CanTpChannelMode>()?;
    communication.add_class::<communication::CanTpConfig>()?;
    communication.add_class::<communication::CanTpConnection>()?;
    communication.add_class::<communication::CanTpEcu>()?;
    communication.add_class::<communication::CanTpNode>()?;
    communication.add_class::<communication::ComTransformationTechnologyConfig>()?;
    communication.add_class::<communication::CommonServiceDiscoveryConfig>()?;
    communication.add_class::<communication::CommunicationDirection>()?;
    communication.add_class::<communication::ConsumedEventGroup>()?;
    communication.add_class::<communication::ConsumedEventGroupV1>()?;
    communication.add_class::<communication::ConsumedServiceInstance>()?;
    communication.add_class::<communication::ConsumedServiceInstanceV1>()?;
    communication.add_class::<communication::ContainedIPduCollectionSemantics>()?;
    communication.add_class::<communication::ContainedIPduProps>()?;
    communication.add_class::<communication::ContainerIPdu>()?;
    communication.add_class::<communication::ContainerIPduHeaderType>()?;
    communication.add_class::<communication::ContainerIPduTrigger>()?;
    communication.add_class::<communication::CycleRepetition>()?;
    communication.add_class::<communication::CyclicTiming>()?;
    communication.add_class::<communication::DataIdMode>()?;
    communication.add_class::<communication::DataTransformation>()?;
    communication.add_class::<communication::DataTransformationSet>()?;
    communication.add_class::<communication::DcmIPdu>()?;
    communication.add_class::<communication::DiagPduType>()?;
    communication.add_class::<communication::DoIpLogicAddress>()?;
    communication.add_class::<communication::DoIpTpConfig>()?;
    communication.add_class::<communication::DoIpTpConnection>()?;
    communication.add_class::<communication::E2EProfile>()?;
    communication.add_class::<communication::E2EProfileBehavior>()?;
    communication.add_class::<communication::E2ETransformationTechnologyConfig>()?;
    communication.add_class::<communication::EndToEndTransformationISignalProps>()?;
    communication.add_class::<communication::EthernetCluster>()?;
    communication.add_class::<communication::EthernetCommunicationConnector>()?;
    communication.add_class::<communication::EthernetCommunicationController>()?;
    communication.add_class::<communication::EthernetPhysicalChannel>()?;
    communication.add_class::<communication::EthernetVlanInfo>()?;
    communication.add_class::<communication::EventControlledTiming>()?;
    communication.add_class::<communication::EventGroupControlType>()?;
    communication.add_class::<communication::EventHandler>()?;
    communication.add_class::<communication::EventHandlerV1>()?;
    communication.add_class::<communication::FlexrayArTpChannel>()?;
    communication.add_class::<communication::FlexrayArTpConfig>()?;
    communication.add_class::<communication::FlexrayArTpConnection>()?;
    communication.add_class::<communication::FlexrayArTpNode>()?;
    communication.add_class::<communication::FlexrayChannelName>()?;
    communication.add_class::<communication::FlexrayCluster>()?;
    communication.add_class::<communication::FlexrayClusterSettings>()?;
    communication.add_class::<communication::FlexrayCommunicationConnector>()?;
    communication.add_class::<communication::FlexrayCommunicationController>()?;
    communication.add_class::<communication::FlexrayCommunicationCycle>()?;
    communication.add_class::<communication::FlexrayFrame>()?;
    communication.add_class::<communication::FlexrayFrameTriggering>()?;
    communication.add_class::<communication::FlexrayNmCluster>()?;
    communication.add_class::<communication::FlexrayNmClusterCoupling>()?;
    communication.add_class::<communication::FlexrayNmClusterSettings>()?;
    communication.add_class::<communication::FlexrayNmNode>()?;
    communication.add_class::<communication::FlexrayNmScheduleVariant>()?;
    communication.add_class::<communication::FlexrayPhysicalChannel>()?;
    communication.add_class::<communication::FlexrayPhysicalChannelsInfo>()?;
    communication.add_class::<communication::FlexrayTpConfig>()?;
    communication.add_class::<communication::FlexrayTpConnection>()?;
    communication.add_class::<communication::FlexrayTpConnectionControl>()?;
    communication.add_class::<communication::FlexrayTpEcu>()?;
    communication.add_class::<communication::FlexrayTpNode>()?;
    communication.add_class::<communication::FlexrayTpPduPool>()?;
    communication.add_class::<communication::FrArTpAckType>()?;
    communication.add_class::<communication::FramePort>()?;
    communication.add_class::<communication::GeneralPurposeIPdu>()?;
    communication.add_class::<communication::GeneralPurposeIPduCategory>()?;
    communication.add_class::<communication::GeneralPurposePdu>()?;
    communication.add_class::<communication::GeneralPurposePduCategory>()?;
    communication.add_class::<communication::GenericTransformationTechnologyConfig>()?;
    communication.add_class::<communication::IPduPort>()?;
    communication.add_class::<communication::IPv4AddressSource>()?;
    communication.add_class::<communication::IPv6AddressSource>()?;
    communication.add_class::<communication::ISignal>()?;
    communication.add_class::<communication::ISignalGroup>()?;
    communication.add_class::<communication::ISignalIPdu>()?;
    communication.add_class::<communication::ISignalPort>()?;
    communication.add_class::<communication::ISignalToIPduMapping>()?;
    communication.add_class::<communication::ISignalTriggering>()?;
    communication.add_class::<communication::InitialSdDelayConfig>()?;
    communication.add_class::<communication::IpduTiming>()?;
    communication.add_class::<communication::LinCluster>()?;
    communication.add_class::<communication::LinEventTriggeredFrame>()?;
    communication.add_class::<communication::LinFrameTriggering>()?;
    communication.add_class::<communication::LinFrameTriggeringIterator>()?;
    communication.add_class::<communication::LinMaster>()?;
    communication.add_class::<communication::LinPhysicalChannel>()?;
    communication.add_class::<communication::LinSlave>()?;
    communication.add_class::<communication::LinSporadicFrame>()?;
    communication.add_class::<communication::LinUnconditionalFrame>()?;
    communication.add_class::<communication::LocalUnicastAddress>()?;
    communication.add_class::<communication::MaximumMessageLengthType>()?;
    communication.add_class::<communication::MultiplexedIPdu>()?;
    communication.add_class::<communication::NPdu>()?;
    communication.add_class::<communication::NetworkEndpoint>()?;
    communication.add_class::<communication::NetworkEndpointAddress>()?;
    communication.add_class::<communication::NmConfig>()?;
    communication.add_class::<communication::NmEcu>()?;
    communication.add_class::<communication::NmPdu>()?;
    communication.add_class::<communication::PduActivationRoutingGroup>()?;
    communication.add_class::<communication::PduCollectionTrigger>()?;
    communication.add_class::<communication::PduToFrameMapping>()?;
    communication.add_class::<communication::PduTriggering>()?;
    communication.add_class::<communication::ProvidedServiceInstance>()?;
    communication.add_class::<communication::ProvidedServiceInstanceV1>()?;
    communication.add_class::<communication::RequestResponseDelay>()?;
    communication.add_class::<communication::RxAcceptContainedIPdu>()?;
    communication.add_class::<communication::SdConfig>()?;
    communication.add_class::<communication::SdEventConfig>()?;
    communication.add_class::<communication::SecureCommunicationProps>()?;
    communication.add_class::<communication::SecuredIPdu>()?;
    communication.add_class::<communication::ServiceInstanceCollectionSet>()?;
    communication.add_class::<communication::SoAdRoutingGroup>()?;
    communication.add_class::<communication::SoConIPduIdentifier>()?;
    communication.add_class::<communication::SocketAddress>()?;
    communication.add_class::<communication::SocketAddressType>()?;
    communication.add_class::<communication::SocketConnection>()?;
    communication.add_class::<communication::SocketConnectionBundle>()?;
    communication.add_class::<communication::SocketConnectionIpduIdentifier>()?;
    communication.add_class::<communication::SocketConnectionIpduIdentifierSet>()?;
    communication.add_class::<communication::SomeIpMessageType>()?;
    communication.add_class::<communication::SomeIpTransformationISignalProps>()?;
    communication.add_class::<communication::SomeIpTransformationTechnologyConfig>()?;
    communication.add_class::<communication::SomeipSdClientEventGroupTimingConfig>()?;
    communication.add_class::<communication::SomeipSdClientServiceInstanceConfig>()?;
    communication.add_class::<communication::SomeipSdServerEventGroupTimingConfig>()?;
    communication.add_class::<communication::SomeipSdServerServiceInstanceConfig>()?;
    communication.add_class::<communication::SomeipTpChannel>()?;
    communication.add_class::<communication::SomeipTpConfig>()?;
    communication.add_class::<communication::SomeipTpConnection>()?;
    communication.add_class::<communication::StaticSocketConnection>()?;
    communication.add_class::<communication::SystemSignal>()?;
    communication.add_class::<communication::SystemSignalGroup>()?;
    communication.add_class::<communication::TcpRole>()?;
    communication.add_class::<communication::TpAddress>()?;
    communication.add_class::<communication::TpConfig>()?;
    communication.add_class::<communication::TransferProperty>()?;
    communication.add_class::<communication::TransformationTechnology>()?;
    communication.add_class::<communication::TransmissionModeTiming>()?;
    communication.add_class::<communication::UdpNmCluster>()?;
    communication.add_class::<communication::UdpNmClusterCoupling>()?;
    communication.add_class::<communication::UdpNmClusterSettings>()?;
    communication.add_class::<communication::UdpNmNode>()?;

    let datatype = PyModule::new(py, "_datatype")?;
    abstraction.add_submodule(&datatype)?;
    datatype.add_class::<datatype::ApplicationArrayDataType>()?;
    datatype.add_class::<datatype::ApplicationArrayElement>()?;
    datatype.add_class::<datatype::ApplicationArraySize>()?;
    datatype.add_class::<datatype::ApplicationPrimitiveCategory>()?;
    datatype.add_class::<datatype::ApplicationPrimitiveDataType>()?;
    datatype.add_class::<datatype::ApplicationRecordDataType>()?;
    datatype.add_class::<datatype::ApplicationRecordElement>()?;
    datatype.add_class::<datatype::ApplicationRuleBasedValueSpecification>()?;
    datatype.add_class::<datatype::ApplicationValueSpecification>()?;
    datatype.add_class::<datatype::ArrayValueSpecification>()?;
    datatype.add_class::<datatype::BaseTypeEncoding>()?;
    datatype.add_class::<datatype::BitfieldEntry>()?;
    datatype.add_class::<datatype::CompositeRuleBasedValueSpecification>()?;
    datatype.add_class::<datatype::CompuMethod>()?;
    datatype.add_class::<datatype::CompuMethodCategory>()?;
    datatype.add_class::<datatype::CompuMethodContent>()?;
    datatype.add_class::<datatype::CompuMethodContent_BitfieldTextTable>()?;
    datatype.add_class::<datatype::CompuMethodContent_Linear>()?;
    datatype.add_class::<datatype::CompuMethodContent_Rational>()?;
    datatype.add_class::<datatype::CompuMethodContent_ScaleLinear>()?;
    datatype.add_class::<datatype::CompuMethodContent_ScaleLinearAndTextTable>()?;
    datatype.add_class::<datatype::CompuMethodContent_ScaleRational>()?;
    datatype.add_class::<datatype::CompuMethodContent_ScaleRationalAndTextTable>()?;
    datatype.add_class::<datatype::CompuMethodContent_TabNoInterpretation>()?;
    datatype.add_class::<datatype::CompuMethodContent_TextTable>()?;
    datatype.add_class::<datatype::CompuScale>()?;
    datatype.add_class::<datatype::CompuScaleDirection>()?;
    datatype.add_class::<datatype::CompuScaleRationalCoefficients>()?;
    datatype.add_class::<datatype::ConstantReference>()?;
    datatype.add_class::<datatype::ConstantSpecification>()?;
    datatype.add_class::<datatype::DataConstr>()?;
    datatype.add_class::<datatype::DataConstrRule>()?;
    datatype.add_class::<datatype::DataConstrType>()?;
    datatype.add_class::<datatype::DataTypeMap>()?;
    datatype.add_class::<datatype::DataTypeMappingSet>()?;
    datatype.add_class::<datatype::ImplementationDataCategory>()?;
    datatype.add_class::<datatype::ImplementationDataType>()?;
    datatype.add_class::<datatype::ImplementationDataTypeElement>()?;
    datatype.add_class::<datatype::ImplementationDataTypeSettings>()?;
    datatype.add_class::<datatype::ImplementationDataTypeSettings_Array>()?;
    datatype.add_class::<datatype::ImplementationDataTypeSettings_DataReference>()?;
    datatype.add_class::<datatype::ImplementationDataTypeSettings_FunctionReference>()?;
    datatype.add_class::<datatype::ImplementationDataTypeSettings_Structure>()?;
    datatype.add_class::<datatype::ImplementationDataTypeSettings_TypeReference>()?;
    datatype.add_class::<datatype::ImplementationDataTypeSettings_Union>()?;
    datatype.add_class::<datatype::ImplementationDataTypeSettings_Value>()?;
    datatype.add_class::<datatype::LinearConversionParameters>()?;
    datatype.add_class::<datatype::NotAvailableValueSpecification>()?;
    datatype.add_class::<datatype::NumericalRuleBasedValueSpecification>()?;
    datatype.add_class::<datatype::NumericalValueSpecification>()?;
    datatype.add_class::<datatype::RationalConversionParameters>()?;
    datatype.add_class::<datatype::RecordValueSpecification>()?;
    datatype.add_class::<datatype::ReferenceValueSpecification>()?;
    datatype.add_class::<datatype::RuleArgument>()?;
    datatype.add_class::<datatype::RuleBasedAxisCont>()?;
    datatype.add_class::<datatype::RuleBasedFillUntil>()?;
    datatype.add_class::<datatype::RuleBasedValueCont>()?;
    datatype.add_class::<datatype::RuleBasedValueSpecification>()?;
    datatype.add_class::<datatype::SwAxisCont>()?;
    datatype.add_class::<datatype::SwAxisContCategory>()?;
    datatype.add_class::<datatype::SwBaseType>()?;
    datatype.add_class::<datatype::SwValue>()?;
    datatype.add_class::<datatype::SwValueCont>()?;
    datatype.add_class::<datatype::TabNoIntpEntry>()?;
    datatype.add_class::<datatype::TextTableEntry>()?;
    datatype.add_class::<datatype::TextValueSpecification>()?;
    datatype.add_class::<datatype::Unit>()?;

    // workaround - pyo3 complex enums do not support setters
    // The "mechanism" of the pyo3 complex enum is reconstructed here, but with subclasses that have setters
    let compu_method_content_type = datatype::CompuMethodContent::type_object(py);
    compu_method_content_type.setattr(
        "Identical",
        datatype::CompuMethodContent_Identical::type_object(py),
    )?;
    compu_method_content_type.setattr(
        "Linear",
        datatype::CompuMethodContent_Linear::type_object(py),
    )?;
    compu_method_content_type.setattr(
        "Rational",
        datatype::CompuMethodContent_Rational::type_object(py),
    )?;
    compu_method_content_type.setattr(
        "TextTable",
        datatype::CompuMethodContent_TextTable::type_object(py),
    )?;
    compu_method_content_type.setattr(
        "ScaleLinear",
        datatype::CompuMethodContent_ScaleLinear::type_object(py),
    )?;
    compu_method_content_type.setattr(
        "ScaleRational",
        datatype::CompuMethodContent_ScaleRational::type_object(py),
    )?;
    compu_method_content_type.setattr(
        "ScaleLinearAndTextTable",
        datatype::CompuMethodContent_ScaleLinearAndTextTable::type_object(py),
    )?;
    compu_method_content_type.setattr(
        "ScaleRationalAndTextTable",
        datatype::CompuMethodContent_ScaleRationalAndTextTable::type_object(py),
    )?;
    compu_method_content_type.setattr(
        "BitfieldTextTable",
        datatype::CompuMethodContent_BitfieldTextTable::type_object(py),
    )?;
    compu_method_content_type.setattr(
        "TabNoInterpretation",
        datatype::CompuMethodContent_TabNoInterpretation::type_object(py),
    )?;

    let implementation_settings_type = datatype::ImplementationDataTypeSettings::type_object(py);
    implementation_settings_type.setattr(
        "Array",
        datatype::ImplementationDataTypeSettings_Array::type_object(py),
    )?;
    implementation_settings_type.setattr(
        "DataReference",
        datatype::ImplementationDataTypeSettings_DataReference::type_object(py),
    )?;
    implementation_settings_type.setattr(
        "FunctionReference",
        datatype::ImplementationDataTypeSettings_FunctionReference::type_object(py),
    )?;
    implementation_settings_type.setattr(
        "Structure",
        datatype::ImplementationDataTypeSettings_Structure::type_object(py),
    )?;
    implementation_settings_type.setattr(
        "TypeReference",
        datatype::ImplementationDataTypeSettings_TypeReference::type_object(py),
    )?;
    implementation_settings_type.setattr(
        "Union",
        datatype::ImplementationDataTypeSettings_Union::type_object(py),
    )?;
    implementation_settings_type.setattr(
        "Value",
        datatype::ImplementationDataTypeSettings_Value::type_object(py),
    )?;

    let ecu_configuration = PyModule::new(py, "_ecu_configuration")?;
    abstraction.add_submodule(&ecu_configuration)?;
    ecu_configuration.add_class::<ecu_configuration::EcucAddInfoParamDef>()?;
    ecu_configuration.add_class::<ecu_configuration::EcucAddInfoParamValue>()?;
    ecu_configuration.add_class::<ecu_configuration::EcucBooleanParamDef>()?;
    ecu_configuration.add_class::<ecu_configuration::EcucChoiceContainerDef>()?;
    ecu_configuration.add_class::<ecu_configuration::EcucChoiceReferenceDef>()?;
    ecu_configuration.add_class::<ecu_configuration::EcucConfigurationClass>()?;
    ecu_configuration.add_class::<ecu_configuration::EcucConfigurationVariant>()?;
    ecu_configuration.add_class::<ecu_configuration::EcucContainerValue>()?;
    ecu_configuration.add_class::<ecu_configuration::EcucDefinitionCollection>()?;
    ecu_configuration.add_class::<ecu_configuration::EcucDestinationUriDef>()?;
    ecu_configuration.add_class::<ecu_configuration::EcucDestinationUriDefSet>()?;
    ecu_configuration.add_class::<ecu_configuration::EcucDestinationUriNestingContract>()?;
    ecu_configuration.add_class::<ecu_configuration::EcucEnumerationLiteralDef>()?;
    ecu_configuration.add_class::<ecu_configuration::EcucEnumerationParamDef>()?;
    ecu_configuration.add_class::<ecu_configuration::EcucFloatParamDef>()?;
    ecu_configuration.add_class::<ecu_configuration::EcucForeignReferenceDef>()?;
    ecu_configuration.add_class::<ecu_configuration::EcucFunctionNameDef>()?;
    ecu_configuration.add_class::<ecu_configuration::EcucInstanceReferenceDef>()?;
    ecu_configuration.add_class::<ecu_configuration::EcucInstanceReferenceValue>()?;
    ecu_configuration.add_class::<ecu_configuration::EcucIntegerParamDef>()?;
    ecu_configuration.add_class::<ecu_configuration::EcucLinkerSymbolDef>()?;
    ecu_configuration.add_class::<ecu_configuration::EcucModuleConfigurationValues>()?;
    ecu_configuration.add_class::<ecu_configuration::EcucModuleDef>()?;
    ecu_configuration.add_class::<ecu_configuration::EcucModuleDefCategory>()?;
    ecu_configuration.add_class::<ecu_configuration::EcucMultilineStringParamDef>()?;
    ecu_configuration.add_class::<ecu_configuration::EcucNumericalParamValue>()?;
    ecu_configuration.add_class::<ecu_configuration::EcucParamConfContainerDef>()?;
    ecu_configuration.add_class::<ecu_configuration::EcucReferenceDef>()?;
    ecu_configuration.add_class::<ecu_configuration::EcucReferenceValue>()?;
    ecu_configuration.add_class::<ecu_configuration::EcucStringParamDef>()?;
    ecu_configuration.add_class::<ecu_configuration::EcucTextualParamValue>()?;
    ecu_configuration.add_class::<ecu_configuration::EcucUriReferenceDef>()?;
    ecu_configuration.add_class::<ecu_configuration::EcucValueCollection>()?;

    let software_component = PyModule::new(py, "_software_component")?;
    abstraction.add_submodule(&software_component)?;
    software_component.add_class::<software_component::ApplicationError>()?;
    software_component.add_class::<software_component::ApplicationSwComponentType>()?;
    software_component.add_class::<software_component::ArgumentDataPrototype>()?;
    software_component.add_class::<software_component::ArgumentDirection>()?;
    software_component.add_class::<software_component::AssemblySwConnector>()?;
    software_component.add_class::<software_component::AsynchronousServerCallReturnsEvent>()?;
    software_component.add_class::<software_component::BackgroundEvent>()?;
    software_component.add_class::<software_component::ClientServerInterface>()?;
    software_component.add_class::<software_component::ClientServerOperation>()?;
    software_component.add_class::<software_component::ComplexDeviceDriverSwComponentType>()?;
    software_component.add_class::<software_component::CompositionSwComponentType>()?;
    software_component.add_class::<software_component::DataReceiveErrorEvent>()?;
    software_component.add_class::<software_component::DataReceivedEvent>()?;
    software_component.add_class::<software_component::DataSendCompletedEvent>()?;
    software_component.add_class::<software_component::DataWriteCompletedEvent>()?;
    software_component.add_class::<software_component::DelegationSwConnector>()?;
    software_component.add_class::<software_component::EcuAbstractionSwComponentType>()?;
    software_component.add_class::<software_component::ExternalTriggerOccurredEvent>()?;
    software_component.add_class::<software_component::InitEvent>()?;
    software_component.add_class::<software_component::InternalTriggerOccurredEvent>()?;
    software_component.add_class::<software_component::ModeAccessPoint>()?;
    software_component.add_class::<software_component::ModeActivationKind>()?;
    software_component.add_class::<software_component::ModeDeclaration>()?;
    software_component.add_class::<software_component::ModeDeclarationGroup>()?;
    software_component.add_class::<software_component::ModeDeclarationGroupCategory>()?;
    software_component.add_class::<software_component::ModeGroup>()?;
    software_component.add_class::<software_component::ModeSwitchInterface>()?;
    software_component.add_class::<software_component::ModeSwitchedAckEvent>()?;
    software_component.add_class::<software_component::ModeSwitchPoint>()?;
    software_component.add_class::<software_component::NvDataInterface>()?;
    software_component.add_class::<software_component::OperationInvokedEvent>()?;
    software_component.add_class::<software_component::OsTaskExecutionEvent>()?;
    software_component.add_class::<software_component::PPortPrototype>()?;
    software_component.add_class::<software_component::PRPortPrototype>()?;
    software_component.add_class::<software_component::ParameterDataPrototype>()?;
    software_component.add_class::<software_component::ParameterInterface>()?;
    software_component.add_class::<software_component::PassThroughSwConnector>()?;
    software_component.add_class::<software_component::PortGroup>()?;
    software_component.add_class::<software_component::RPortPrototype>()?;
    software_component.add_class::<software_component::RootSwCompositionPrototype>()?;
    software_component.add_class::<software_component::RunnableEntity>()?;
    software_component.add_class::<software_component::SenderReceiverInterface>()?;
    software_component.add_class::<software_component::SensorActuatorSwComponentType>()?;
    software_component.add_class::<software_component::ServiceSwComponentType>()?;
    software_component.add_class::<software_component::SwComponentPrototype>()?;
    software_component.add_class::<software_component::SwcInternalBehavior>()?;
    software_component.add_class::<software_component::SwcModeManagerErrorEvent>()?;
    software_component.add_class::<software_component::SwcModeSwitchEvent>()?;
    software_component.add_class::<software_component::SynchronousServerCallPoint>()?;
    software_component.add_class::<software_component::TimingEvent>()?;
    software_component.add_class::<software_component::TransformerHardErrorEvent>()?;
    software_component.add_class::<software_component::TriggerInterface>()?;
    software_component.add_class::<software_component::VariableAccess>()?;
    software_component.add_class::<software_component::VariableDataPrototype>()?;

    // Workaround for Pyo3 issue #759 (https://github.com/PyO3/pyo3/issues/759)
    // See also https://github.com/PyO3/pyo3/issues/1517#issuecomment-808664021
    let sys_modules = py.import("sys")?.getattr("modules")?;

    sys_modules.set_item("autosar_data._autosar_data._abstraction", &abstraction)?;
    sys_modules.set_item(
        "autosar_data._autosar_data._abstraction._communication",
        &communication,
    )?;
    sys_modules.set_item(
        "autosar_data._autosar_data._abstraction._datatype",
        &datatype,
    )?;
    sys_modules.set_item(
        "autosar_data._autosar_data._abstraction._ecu_configuration",
        &ecu_configuration,
    )?;
    sys_modules.set_item(
        "autosar_data._autosar_data._abstraction._software_component",
        &software_component,
    )?;

    // Workaround proposed by amorenoz in Pyo3 issue #4870
    // This allows griffe to find the submodules; griffe is used by mkdocs
    abstraction.setattr("__module__", "autosar_data._autosar_data._abstraction")?;
    abstraction.setattr("__name__", "autosar_data._autosar_data._abstraction")?;

    communication.setattr(
        "__module__",
        "autosar_data._autosar_data._abstraction._communication",
    )?;
    communication.setattr(
        "__name__",
        "autosar_data._autosar_data._abstraction._communication",
    )?;

    datatype.setattr(
        "__module__",
        "autosar_data._autosar_data._abstraction._datatype",
    )?;
    datatype.setattr(
        "__name__",
        "autosar_data._autosar_data._abstraction._datatype",
    )?;

    ecu_configuration.setattr(
        "__module__",
        "autosar_data._autosar_data._abstraction._ecu_configuration",
    )?;
    ecu_configuration.setattr(
        "__name__",
        "autosar_data._autosar_data._abstraction._ecu_configuration",
    )?;

    software_component.setattr(
        "__module__",
        "autosar_data._autosar_data._abstraction._software_component",
    )?;
    software_component.setattr(
        "__name__",
        "autosar_data._autosar_data._abstraction._software_component",
    )?;

    Ok(())
}

/// Convert an `AutosarAbstractionError` to a `PyErr`
///
/// This function can't be a From/Into implementation, because both types are defined in different crates
pub(crate) fn abstraction_err_to_pyerr(
    err: autosar_data_abstraction::AutosarAbstractionError,
) -> PyErr {
    AutosarAbstractionError::new_err(err.to_string())
}
