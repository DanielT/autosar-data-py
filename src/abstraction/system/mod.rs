use crate::{
    Element,
    abstraction::{
        ArPackage, AutosarAbstractionError, EcuInstance, abstraction_err_to_pyerr,
        communication::{
            CanCluster, CanFrame, CanTpConfig, ContainerIPdu, ContainerIPduHeaderType, DcmIPdu,
            DiagPduType, DoIpTpConfig, EthernetCluster, EventGroupControlType, FlexrayArTpConfig,
            FlexrayCluster, FlexrayClusterSettings, FlexrayFrame, FlexrayTpConfig,
            GeneralPurposeIPdu, GeneralPurposeIPduCategory, GeneralPurposePdu,
            GeneralPurposePduCategory, ISignal, ISignalGroup, ISignalIPdu, LinCluster,
            LinEventTriggeredFrame, LinSporadicFrame, LinUnconditionalFrame, MultiplexedIPdu, NPdu,
            NmConfig, NmPdu, RxAcceptContainedIPdu, SecureCommunicationProps, SecuredIPdu,
            ServiceInstanceCollectionSet, SoAdRoutingGroup, SocketConnectionIpduIdentifierSet,
            SomeipTpConfig, SystemSignal, SystemSignalGroup,
        },
        datatype::SwBaseType,
        software_component::{CompositionSwComponentType, RootSwCompositionPrototype},
    },
    iterator_wrapper,
};
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};
use pyo3::{IntoPyObjectExt, exceptions::PyTypeError, prelude::*};

mod mapping;

pub(crate) use mapping::*;

//##################################################################

/// The System is the top level of a system template
///
/// It defines how ECUs communicate with each other over various networks.
/// It also contains the mapping of software components to ECUs.
#[pyclass(frozen, eq, module = "autosar_data._autosar_data._abstraction")]
#[derive(Clone, PartialEq)]
pub(crate) struct System(pub(crate) autosar_data_abstraction::System);

#[pymethods]
impl System {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::System::try_from(element.0.clone()) {
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

    /// set the category of the system
    #[setter]
    fn set_category(&self, category: SystemCategory) -> PyResult<()> {
        self.0
            .set_category(category.into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the category of the system
    #[getter]
    fn category(&self) -> Option<SystemCategory> {
        self.0.category().map(std::convert::Into::into)
    }

    /// set the pncVectorLength of the system
    #[setter]
    fn set_pnc_vector_length(&self, length: Option<u32>) -> PyResult<()> {
        self.0
            .set_pnc_vector_length(length)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the pncVectorLength of the system
    #[getter]
    fn pnc_vector_length(&self) -> Option<u32> {
        self.0.pnc_vector_length()
    }

    /// set the pncVectorOffset of the system
    #[setter]
    fn set_pnc_vector_offset(&self, offset: Option<u32>) -> PyResult<()> {
        self.0
            .set_pnc_vector_offset(offset)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the pncVectorOffset of the system
    #[getter]
    fn pnc_vector_offset(&self) -> Option<u32> {
        self.0.pnc_vector_offset()
    }

    /// create an `EcuInstance` that is connected to this System
    #[pyo3(signature = (name, package, /))]
    #[pyo3(text_signature = "(self, name: str, package: ArPackage, /)")]
    fn create_ecu_instance(&self, name: &str, package: &ArPackage) -> PyResult<EcuInstance> {
        match self.0.create_ecu_instance(name, &package.0) {
            Ok(ecu) => Ok(EcuInstance(ecu)),
            Err(error) => PyResult::Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// get an iterator over all ECU-INSTANCEs in this SYSTEM
    fn ecu_instances(&self) -> EcuInstanceIterator {
        EcuInstanceIterator::new(self.0.ecu_instances().map(EcuInstance))
    }

    /// create a new CAN-CLUSTER
    ///
    /// The cluster must have a channel to be valid, but this channel is not created automatically.
    /// Call [`CanCluster::create_physical_channel`] to create it.
    #[pyo3(signature = (cluster_name, package, /, *, can_baudrate = Some(500_000)))]
    #[pyo3(
        text_signature = "(self, cluster_name: str, package: ArPackage, /, *, can_baudrate: Optional[int] = 500000)"
    )]
    fn create_can_cluster(
        &self,
        cluster_name: &str,
        package: &ArPackage,
        can_baudrate: Option<u32>,
    ) -> PyResult<CanCluster> {
        match self
            .0
            .create_can_cluster(cluster_name, &package.0, can_baudrate)
        {
            Ok(cluster) => Ok(CanCluster(cluster)),
            Err(error) => PyResult::Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// create a new ETHERNET-CLUSTER and connect it to the SYSTEM
    ///
    /// The cluster must have at least one channel to be valid.
    /// Call [`EthernetCluster.create_physical_channel`] to create it.
    #[pyo3(signature = (cluster_name, package, /))]
    #[pyo3(text_signature = "(self, cluster_name: str, package: ArPackage, /)")]
    fn create_ethernet_cluster(
        &self,
        cluster_name: &str,
        package: &ArPackage,
    ) -> PyResult<EthernetCluster> {
        match self.0.create_ethernet_cluster(cluster_name, &package.0) {
            Ok(cluster) => Ok(EthernetCluster(cluster)),
            Err(error) => PyResult::Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// create a new FLEXRAY-CLUSTER and connect it to the SYSTEM
    ///
    /// A `FlexrayClusterSettings` structure containing the timings and parameters for the Flexray cluster must be provided.
    ///
    /// The cluster must have at least one channel to be valid.
    /// Call [`FlexrayCluster.create_physical_channel`] to create it.
    #[pyo3(signature = (cluster_name, package, settings, /))]
    #[pyo3(
        text_signature = "(self, cluster_name: str, package: ArPackage, settings: FlexrayClusterSettings, /)"
    )]
    fn create_flexray_cluster(
        &self,
        cluster_name: &str,
        package: &ArPackage,
        settings: &FlexrayClusterSettings,
    ) -> PyResult<FlexrayCluster> {
        match self
            .0
            .create_flexray_cluster(cluster_name, &package.0, &settings.0)
        {
            Ok(cluster) => Ok(FlexrayCluster(cluster)),
            Err(error) => PyResult::Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// create a new LIN-CLUSTER and connect it to the SYSTEM
    ///
    /// The cluster must have a channel to be valid, but this channel is not created automatically.
    /// Call [`LinCluster::create_physical_channel`] to create it.
    #[pyo3(signature = (cluster_name, package, /))]
    #[pyo3(text_signature = "(self, cluster_name: str, package: ArPackage, /)")]
    fn create_lin_cluster(&self, cluster_name: &str, package: &ArPackage) -> PyResult<LinCluster> {
        match self.0.create_lin_cluster(cluster_name, &package.0) {
            Ok(cluster) => Ok(LinCluster(cluster)),
            Err(error) => PyResult::Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// Create an iterator over all clusters connected to the SYSTEM
    fn clusters(&self) -> ClusterIterator {
        ClusterIterator::new(self.0.clusters().filter_map(|cluster| match cluster {
            autosar_data_abstraction::communication::Cluster::Can(cluster) => {
                Python::attach(|py| CanCluster(cluster).into_py_any(py).ok())
            }
            autosar_data_abstraction::communication::Cluster::Ethernet(cluster) => {
                Python::attach(|py| EthernetCluster(cluster).into_py_any(py).ok())
            }
            autosar_data_abstraction::communication::Cluster::FlexRay(cluster) => {
                Python::attach(|py| FlexrayCluster(cluster).into_py_any(py).ok())
            }
            autosar_data_abstraction::communication::Cluster::Lin(cluster) => {
                Python::attach(|py| LinCluster(cluster).into_py_any(py).ok())
            }
            _ => None,
        }))
    }

    /// create a new [`CanFrame`]
    ///
    /// This new frame needs to be linked to a `CanPhysicalChannel`
    #[pyo3(signature = (name, package, byte_length, /))]
    #[pyo3(text_signature = "(self, name: str, package: ArPackage, byte_length: int, /)")]
    fn create_can_frame(
        &self,
        name: &str,
        package: &ArPackage,
        byte_length: u64,
    ) -> PyResult<CanFrame> {
        match self.0.create_can_frame(name, &package.0, byte_length) {
            Ok(frame) => Ok(CanFrame(frame)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// create a new [`FlexrayFrame`]
    ///
    /// This new frame needs to be linked to a `FlexrayPhysicalChannel`
    #[pyo3(signature = (name, package, byte_length, /))]
    #[pyo3(text_signature = "(self, name: str, package: ArPackage, byte_length: int, /)")]
    fn create_flexray_frame(
        &self,
        name: &str,
        package: &ArPackage,
        byte_length: u64,
    ) -> PyResult<FlexrayFrame> {
        match self.0.create_flexray_frame(name, &package.0, byte_length) {
            Ok(frame) => Ok(FlexrayFrame(frame)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// iterate over all Frames in the System
    fn frames(&self) -> FrameIterator {
        use autosar_data_abstraction::communication as comm;

        FrameIterator::new(self.0.frames().filter_map(|frame| match frame {
            comm::Frame::Can(frame) => Python::attach(|py| CanFrame(frame).into_py_any(py).ok()),
            comm::Frame::Flexray(frame) => {
                Python::attach(|py| FlexrayFrame(frame).into_py_any(py).ok())
            }
            comm::Frame::Lin(comm::LinFrame::EventTriggered(lin_event_triggered_frame)) => {
                Python::attach(|py| {
                    LinEventTriggeredFrame(lin_event_triggered_frame)
                        .into_py_any(py)
                        .ok()
                })
            }
            comm::Frame::Lin(comm::LinFrame::Sporadic(lin_sporadic_frame)) => {
                Python::attach(|py| LinSporadicFrame(lin_sporadic_frame).into_py_any(py).ok())
            }
            comm::Frame::Lin(comm::LinFrame::Unconditional(lin_unconditional_frame)) => {
                Python::attach(|py| {
                    LinUnconditionalFrame(lin_unconditional_frame)
                        .into_py_any(py)
                        .ok()
                })
            }
            _ => None,
        }))
    }

    /// create a new isignal in the [`System`]
    #[pyo3(signature = (name, package, bit_length, syssignal, /, *, datatype=None))]
    #[pyo3(
        text_signature = "(self, name: str, package: ArPackage, bit_length: int, syssignal: SystemSignal, /, *, datatype: Optional[SwBaseType] = None)"
    )]
    fn create_isignal(
        &self,
        name: &str,
        package: &ArPackage,
        bit_length: u64,
        syssignal: &SystemSignal,
        datatype: Option<&SwBaseType>,
    ) -> PyResult<ISignal> {
        match self.0.create_isignal(
            name,
            &package.0,
            bit_length,
            &syssignal.0,
            datatype.map(|datatype| &datatype.0),
        ) {
            Ok(signal) => Ok(ISignal(signal)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// iterate over all ISignals in the System
    ///
    /// This iterator returns all ISignals that are connected to the System using a FibexElementRef.
    fn isignals(&self) -> ISignalIterator {
        ISignalIterator::new(self.0.isignals().map(ISignal))
    }

    /// create a new signal group in the [`System`]
    ///
    /// `I-SIGNAL-GROUP` and `SYSTEM-SIGNAL-GROUP` are created using the same name; therefore they must be placed in
    /// different packages: `sig_package` and `sys_package` may not be identical.
    #[pyo3(signature = (name, package, system_signal_group, /))]
    #[pyo3(
        text_signature = "(self, name: str, package: ArPackage, system_signal_group: SystemSignalGroup, /)"
    )]
    fn create_isignal_group(
        &self,
        name: &str,
        package: &ArPackage,
        system_signal_group: &SystemSignalGroup,
    ) -> PyResult<ISignalGroup> {
        match self
            .0
            .create_isignal_group(name, &package.0, &system_signal_group.0)
        {
            Ok(group) => Ok(ISignalGroup(group)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// iterate over all ISignalGroups in the System
    fn isignal_groups(&self) -> ISignalGroupIterator {
        ISignalGroupIterator::new(self.0.isignal_groups().map(ISignalGroup))
    }

    /// create an [`ISignalIPdu`] in the [`System`]
    #[pyo3(signature = (name, package, length, /))]
    #[pyo3(text_signature = "(self, name: str, package: ArPackage, length: int, /)")]
    fn create_isignal_ipdu(
        &self,
        name: &str,
        package: &ArPackage,
        length: u32,
    ) -> PyResult<ISignalIPdu> {
        match self.0.create_isignal_ipdu(name, &package.0, length) {
            Ok(ipdu) => Ok(ISignalIPdu(ipdu)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// create an [`NmPdu`] in the [`System`]
    #[pyo3(signature = (name, package, length, /))]
    #[pyo3(text_signature = "(self, name: str, package: ArPackage, length: int, /)")]
    fn create_nm_pdu(&self, name: &str, package: &ArPackage, length: u32) -> PyResult<NmPdu> {
        match self.0.create_nm_pdu(name, &package.0, length) {
            Ok(ipdu) => Ok(NmPdu(ipdu)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// create an [`NPdu`] in the [`System`]
    #[pyo3(signature = (name, package, length, /))]
    #[pyo3(text_signature = "(self, name: str, package: ArPackage, length: int, /)")]
    fn create_n_pdu(&self, name: &str, package: &ArPackage, length: u32) -> PyResult<NPdu> {
        match self.0.create_n_pdu(name, &package.0, length) {
            Ok(ipdu) => Ok(NPdu(ipdu)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// create a [`DcmIPdu`] in the [`System`]
    #[pyo3(signature = (name, package, length, diag_pdu_type, /))]
    #[pyo3(
        text_signature = "(self, name: str, package: ArPackage, length: int, diag_pdu_type: DiagPduType, /)"
    )]
    fn create_dcm_ipdu(
        &self,
        name: &str,
        package: &ArPackage,
        length: u32,
        diag_pdu_type: DiagPduType,
    ) -> PyResult<DcmIPdu> {
        match self
            .0
            .create_dcm_ipdu(name, &package.0, length, diag_pdu_type.into())
        {
            Ok(ipdu) => Ok(DcmIPdu(ipdu)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// create a [`GeneralPurposePdu`] in the [`System`]
    #[pyo3(signature = (name, package, length, category, /))]
    #[pyo3(
        text_signature = "(self, name: str, package: ArPackage, length: int, category: GeneralPurposePduCategory, /)"
    )]
    fn create_general_purpose_pdu(
        &self,
        name: &str,
        package: &ArPackage,
        length: u32,
        category: GeneralPurposePduCategory,
    ) -> PyResult<GeneralPurposePdu> {
        match self
            .0
            .create_general_purpose_pdu(name, &package.0, length, category.into())
        {
            Ok(ipdu) => Ok(GeneralPurposePdu(ipdu)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// create a [`GeneralPurposeIPdu`] in the [`System`]
    #[pyo3(signature = (name, package, length, category, /))]
    #[pyo3(
        text_signature = "(self, name: str, package: ArPackage, length: int, category: GeneralPurposeIPduCategory, /)"
    )]
    fn create_general_purpose_ipdu(
        &self,
        name: &str,
        package: &ArPackage,
        length: u32,
        category: GeneralPurposeIPduCategory,
    ) -> PyResult<GeneralPurposeIPdu> {
        match self
            .0
            .create_general_purpose_ipdu(name, &package.0, length, category.into())
        {
            Ok(ipdu) => Ok(GeneralPurposeIPdu(ipdu)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// create a [`ContainerIPdu`] in the [`System`]
    #[pyo3(signature = (name, package, length, header_type, rx_accept, /))]
    #[pyo3(
        text_signature = "(self, name: str, package: ArPackage, length: int, header_type: ContainerIPduHeaderType, rx_accept: RxAcceptContainedIPdu, /)"
    )]
    fn create_container_ipdu(
        &self,
        name: &str,
        package: &ArPackage,
        length: u32,
        header_type: ContainerIPduHeaderType,
        rx_accept: RxAcceptContainedIPdu,
    ) -> PyResult<ContainerIPdu> {
        match self.0.create_container_ipdu(
            name,
            &package.0,
            length,
            header_type.into(),
            rx_accept.into(),
        ) {
            Ok(ipdu) => Ok(ContainerIPdu(ipdu)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// create a [`SecuredIPdu`] in the [`System`]
    #[pyo3(signature = (name, package, length, secure_props, /))]
    #[pyo3(
        text_signature = "(self, name: str, package: ArPackage, length: int, secure_props: SecureCommunicationProps /)"
    )]
    fn create_secured_ipdu(
        &self,
        name: &str,
        package: &ArPackage,
        length: u32,
        secure_props: &SecureCommunicationProps,
    ) -> PyResult<SecuredIPdu> {
        match self
            .0
            .create_secured_ipdu(name, &package.0, length, &secure_props.into())
        {
            Ok(ipdu) => Ok(SecuredIPdu(ipdu)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// create a [`MultiplexedIPdu`] in the [`System`]
    #[pyo3(signature = (name, package, length, /))]
    #[pyo3(text_signature = "(self, name: str, package: ArPackage, length: int, /)")]
    fn create_multiplexed_ipdu(
        &self,
        name: &str,
        package: &ArPackage,
        length: u32,
    ) -> PyResult<MultiplexedIPdu> {
        match self.0.create_multiplexed_ipdu(name, &package.0, length) {
            Ok(ipdu) => Ok(MultiplexedIPdu(ipdu)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// iterate over all PDUs in the System
    ///
    /// This iterator returns all PDUs that are connected to the System using a FibexElementRef.
    fn pdus(&self) -> PduIterator {
        PduIterator::new(self.0.pdus().filter_map(|pdu| match pdu {
            autosar_data_abstraction::communication::Pdu::ISignalIPdu(pdu) => {
                Python::attach(|py| ISignalIPdu(pdu).into_py_any(py).ok())
            }
            autosar_data_abstraction::communication::Pdu::NmPdu(pdu) => {
                Python::attach(|py| NmPdu(pdu).into_py_any(py).ok())
            }
            autosar_data_abstraction::communication::Pdu::NPdu(pdu) => {
                Python::attach(|py| NPdu(pdu).into_py_any(py).ok())
            }
            autosar_data_abstraction::communication::Pdu::DcmIPdu(pdu) => {
                Python::attach(|py| DcmIPdu(pdu).into_py_any(py).ok())
            }
            autosar_data_abstraction::communication::Pdu::GeneralPurposePdu(pdu) => {
                Python::attach(|py| GeneralPurposePdu(pdu).into_py_any(py).ok())
            }
            autosar_data_abstraction::communication::Pdu::GeneralPurposeIPdu(pdu) => {
                Python::attach(|py| GeneralPurposeIPdu(pdu).into_py_any(py).ok())
            }
            autosar_data_abstraction::communication::Pdu::ContainerIPdu(pdu) => {
                Python::attach(|py| ContainerIPdu(pdu).into_py_any(py).ok())
            }
            autosar_data_abstraction::communication::Pdu::SecuredIPdu(pdu) => {
                Python::attach(|py| SecuredIPdu(pdu).into_py_any(py).ok())
            }
            autosar_data_abstraction::communication::Pdu::MultiplexedIPdu(pdu) => {
                Python::attach(|py| MultiplexedIPdu(pdu).into_py_any(py).ok())
            } //_ => None,
        }))
    }

    /// Create a `SocketConnectionIpduIdentifierSet` in the SYSTEM
    ///
    /// `SocketConnectionIpduIdentifierSet` are part of the new ethernet modeling that was introduced in Autosar 4.5.0 (`AUTOSAR_00048`).
    #[pyo3(signature = (name, package, /))]
    #[pyo3(text_signature = "(self, name: str, package: ArPackage, /)")]
    fn create_socket_connection_ipdu_identifier_set(
        &self,
        name: &str,
        package: &ArPackage,
    ) -> PyResult<SocketConnectionIpduIdentifierSet> {
        match self
            .0
            .create_socket_connection_ipdu_identifier_set(name, &package.0)
        {
            Ok(set) => Ok(SocketConnectionIpduIdentifierSet(set)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// Create a `SoAdRoutingGroup` in the SYSTEM
    ///
    /// `SoAdRoutingGroup` are part of the old ethernet modeling that was used prior to Autosar 4.5.0 (`AUTOSAR_00048`).
    /// The elements are still present (but obsolete) in newer versions of the standard.
    /// Old and new elements may not be mixed in the same model.
    #[pyo3(signature = (name, package, /, *, control_type=None))]
    #[pyo3(
        text_signature = "(self, name: str, package: ArPackage, /, *, control_type: Optional[EventGroupControlType] = None)"
    )]
    fn create_so_ad_routing_group(
        &self,
        name: &str,
        package: &ArPackage,
        control_type: Option<EventGroupControlType>,
    ) -> PyResult<SoAdRoutingGroup> {
        match self.0.create_so_ad_routing_group(
            name,
            &package.0,
            control_type.map(std::convert::Into::into),
        ) {
            Ok(group) => Ok(SoAdRoutingGroup(group)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// Create a `ServiceInstanceCollectionSet` in the SYSTEM
    ///
    /// `ServiceInstanceCollectionSet`s are part of the new ethernet modeling that was introduced in Autosar 4.5.0 (`AUTOSAR_00048`).
    #[pyo3(signature = (name, package, /))]
    #[pyo3(text_signature = "(self, name: str, package: ArPackage, /)")]
    fn create_service_instance_collection_set(
        &self,
        name: &str,
        package: &ArPackage,
    ) -> PyResult<ServiceInstanceCollectionSet> {
        match self
            .0
            .create_service_instance_collection_set(name, &package.0)
        {
            Ok(set) => Ok(ServiceInstanceCollectionSet(set)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// Create a `SomeipTpConfig` in the SYSTEM
    ///
    /// `SomeipTpConfig`s contain the configuration how to segment or reassemble large `SomeIpTp` PDUs.
    #[pyo3(signature = (name, package, cluster, /))]
    #[pyo3(text_signature = "(self, name: str, package: ArPackage, cluster: Cluster, /)")]
    fn create_someip_tp_config(
        &self,
        name: &str,
        package: &ArPackage,
        cluster: &Bound<'_, PyAny>,
    ) -> PyResult<SomeipTpConfig> {
        let cluster = if let Ok(can_cluster) = cluster.extract::<CanCluster>() {
            autosar_data_abstraction::communication::Cluster::Can(can_cluster.0)
        } else if let Ok(ethernet_cluster) = cluster.extract::<EthernetCluster>() {
            autosar_data_abstraction::communication::Cluster::Ethernet(ethernet_cluster.0)
        } else if let Ok(flexray_cluster) = cluster.extract::<FlexrayCluster>() {
            autosar_data_abstraction::communication::Cluster::FlexRay(flexray_cluster.0)
        } else if let Ok(lin_cluster) = cluster.extract::<LinCluster>() {
            autosar_data_abstraction::communication::Cluster::Lin(lin_cluster.0)
        } else {
            return Err(PyTypeError::new_err(format!(
                "'{}' cannot be converted to 'Cluster'",
                cluster.get_type()
            )));
        };

        match self.0.create_someip_tp_config(name, &package.0, &cluster) {
            Ok(config) => Ok(SomeipTpConfig(config)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// Create a `CanTpConfig` in the SYSTEM
    ///
    /// `CanTpConfig`s contain the configuration how to segment or reassemble diagnostic messages on a CAN bus.
    #[pyo3(signature = (name, package, can_cluster, /))]
    #[pyo3(text_signature = "(self, name: str, package: ArPackage, can_cluster: CanCluster, /)")]
    fn create_can_tp_config(
        &self,
        name: &str,
        package: &ArPackage,
        can_cluster: &CanCluster,
    ) -> PyResult<CanTpConfig> {
        match self
            .0
            .create_can_tp_config(name, &package.0, &can_cluster.0)
        {
            Ok(config) => Ok(CanTpConfig(config)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// Create a `DoIpTpConfig` in the SYSTEM
    ///
    /// `DoIpTpConfig`s contain the configuration how to transmit diagnostic messages over IP networks.
    #[pyo3(signature = (name, package, eth_cluster, /))]
    #[pyo3(
        text_signature = "(self, name: str, package: ArPackage, eth_cluster: EthernetCluster, /)"
    )]
    fn create_doip_tp_config(
        &self,
        name: &str,
        package: &ArPackage,
        eth_cluster: &EthernetCluster,
    ) -> PyResult<DoIpTpConfig> {
        match self
            .0
            .create_doip_tp_config(name, &package.0, &eth_cluster.0)
        {
            Ok(config) => Ok(DoIpTpConfig(config)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// Create a `FlexrayTpConfig` in the SYSTEM
    ///
    /// `FlexrayTpConfig`s describe how to segment or reassemble diagnostic messages on a Flexray bus.
    /// This configuration type is used for Flexray ISO TP communication.
    #[pyo3(signature = (name, package, flexray_cluster, /))]
    #[pyo3(
        text_signature = "(self, name: str, package: ArPackage, flexray_cluster: FlexrayCluster, /)"
    )]
    fn create_flexray_tp_config(
        &self,
        name: &str,
        package: &ArPackage,
        flexray_cluster: &FlexrayCluster,
    ) -> PyResult<FlexrayTpConfig> {
        match self
            .0
            .create_flexray_tp_config(name, &package.0, &flexray_cluster.0)
        {
            Ok(config) => Ok(FlexrayTpConfig(config)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// Create a `FlexrayArTpConfig` in the SYSTEM
    ///
    /// `FlexrayArTpConfig`s describe how to segment or reassemble diagnostic messages on a Flexray bus.
    /// This configuration type is used for Flexray AUTOSAR TP communication.
    #[pyo3(signature = (name, package, flexray_cluster, /))]
    #[pyo3(
        text_signature = "(self, name: str, package: ArPackage, flexray_cluster: FlexrayCluster, /)"
    )]
    fn create_flexray_ar_tp_config(
        &self,
        name: &str,
        package: &ArPackage,
        flexray_cluster: &FlexrayCluster,
    ) -> PyResult<FlexrayArTpConfig> {
        match self
            .0
            .create_flexray_ar_tp_config(name, &package.0, &flexray_cluster.0)
        {
            Ok(config) => Ok(FlexrayArTpConfig(config)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// Create a new `NmConfig` in the SYSTEM
    ///
    /// `NmConfig`s contain the configuration for network management.
    /// The System may contain zero or one `NmConfig`s.
    #[pyo3(signature = (name, package, /))]
    #[pyo3(text_signature = "(self, name: str, package: ArPackage, /)")]
    fn create_nm_config(&self, name: &str, package: &ArPackage) -> PyResult<NmConfig> {
        match self.0.create_nm_config(name, &package.0) {
            Ok(config) => Ok(NmConfig(config)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// Get the `NmConfig` of the SYSTEM, if any
    ///
    /// The System may contain zero or one `NmConfig`s.
    fn nm_config(&self) -> Option<NmConfig> {
        self.0.nm_config().map(NmConfig)
    }

    /// connect an element to the SYSTEM by creating a FIBEX-ELEMENT-REF
    ///
    /// If there is already a FIBEX-ELEMENT-REF, this function does nothing, successfully.
    #[pyo3(signature = (elem, /))]
    #[pyo3(text_signature = "(self, elem: Element, /)")]
    fn create_fibex_element_ref(&self, elem: &Element) -> PyResult<()> {
        self.0
            .create_fibex_element_ref(&elem.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// set the root software composition of the system
    ///
    /// This function will remove any existing root software composition
    #[pyo3(signature = (name, composition_type, /))]
    #[pyo3(text_signature = "(self, name: str, composition_type: CompositionSwComponentType, /)")]
    fn set_root_sw_composition(
        &self,
        name: &str,
        composition_type: &CompositionSwComponentType,
    ) -> PyResult<RootSwCompositionPrototype> {
        match self.0.set_root_sw_composition(name, &composition_type.0) {
            Ok(composition) => Ok(RootSwCompositionPrototype(composition)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// get the root software composition of the system
    #[getter]
    fn root_sw_composition(&self) -> Option<RootSwCompositionPrototype> {
        self.0.root_sw_composition().map(RootSwCompositionPrototype)
    }

    /// get or create a mapping for this system
    ///
    /// There does not seem to be any benefit to having multiple mappings for a single system, so this function
    /// will return the first mapping if it exists. Otherwise a new mapping will be created with the provided name.
    #[pyo3(signature = (name, /))]
    #[pyo3(text_signature = "(self, name: str, /)")]
    fn get_or_create_mapping(&self, name: &str) -> PyResult<SystemMapping> {
        match self.0.get_or_create_mapping(name) {
            Ok(mapping) => Ok(SystemMapping(mapping)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }
}

iterator_wrapper!(EcuInstanceIterator, EcuInstance);
iterator_wrapper!(
    ClusterIterator,
    Py<PyAny>,
    "Union[CanCluster, EthernetCluster, FlexrayCluster]"
);
iterator_wrapper!(FrameIterator, Py<PyAny>, "Union[CanFrame, FlexrayFrame]");
iterator_wrapper!(ISignalIterator, ISignal);
iterator_wrapper!(ISignalGroupIterator, ISignalGroup);
iterator_wrapper!(PduIterator, Py<PyAny>, "Pdu");

//#########################################################

/// The category of a System
#[pyclass(frozen, eq, eq_int, module = "autosar_data._autosar_data._abstraction")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemCategory {
    /// The `System` is used to describe system constraints
    SystemConstraints,
    /// The `System` is used to describe the system configuration of a complete AUTOSAR system
    SystemDescription,
    /// The `System` is used to describe a subsystem specific view on the complete system description
    SystemExtract,
    /// The `System` is used to describe the ECU specific view on the complete system description
    EcuExtract,
    /// The `System` is used to describe a functional (solution-independent/abstract) system design
    AbstractSystemDescription,
    /// The `System` is used to describe the closed view on one ECU
    EcuSystemDescription,
    /// The `System` describes the content of one `CpSoftwareCluster`
    SwClusterSystemDescription,
    /// `System` which describes the rapid prototyping algorithm in the format of AUTOSAR Software Components
    RptSystem,
}

impl From<SystemCategory> for autosar_data_abstraction::SystemCategory {
    fn from(cat: SystemCategory) -> autosar_data_abstraction::SystemCategory {
        match cat {
            SystemCategory::SystemConstraints => {
                autosar_data_abstraction::SystemCategory::SystemConstraints
            }
            SystemCategory::SystemDescription => {
                autosar_data_abstraction::SystemCategory::SystemDescription
            }
            SystemCategory::SystemExtract => {
                autosar_data_abstraction::SystemCategory::SystemExtract
            }
            SystemCategory::EcuExtract => autosar_data_abstraction::SystemCategory::EcuExtract,
            SystemCategory::AbstractSystemDescription => {
                autosar_data_abstraction::SystemCategory::AbstractSystemDescription
            }
            SystemCategory::EcuSystemDescription => {
                autosar_data_abstraction::SystemCategory::EcuSystemDescription
            }
            SystemCategory::SwClusterSystemDescription => {
                autosar_data_abstraction::SystemCategory::SwClusterSystemDescription
            }
            SystemCategory::RptSystem => autosar_data_abstraction::SystemCategory::RptSystem,
        }
    }
}

impl From<autosar_data_abstraction::SystemCategory> for SystemCategory {
    fn from(cat: autosar_data_abstraction::SystemCategory) -> SystemCategory {
        match cat {
            autosar_data_abstraction::SystemCategory::SystemConstraints => {
                SystemCategory::SystemConstraints
            }
            autosar_data_abstraction::SystemCategory::SystemDescription => {
                SystemCategory::SystemDescription
            }
            autosar_data_abstraction::SystemCategory::SystemExtract => {
                SystemCategory::SystemExtract
            }
            autosar_data_abstraction::SystemCategory::EcuExtract => SystemCategory::EcuExtract,
            autosar_data_abstraction::SystemCategory::AbstractSystemDescription => {
                SystemCategory::AbstractSystemDescription
            }
            autosar_data_abstraction::SystemCategory::EcuSystemDescription => {
                SystemCategory::EcuSystemDescription
            }
            autosar_data_abstraction::SystemCategory::SwClusterSystemDescription => {
                SystemCategory::SwClusterSystemDescription
            }
            autosar_data_abstraction::SystemCategory::RptSystem => SystemCategory::RptSystem,
        }
    }
}
