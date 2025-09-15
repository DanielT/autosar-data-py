use crate::abstraction::AutosarAbstractionError;
use crate::abstraction::communication::{CanCluster, EthernetCluster, FlexrayCluster, NmPdu};
use crate::{abstraction::*, *};
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};

pub(crate) mod can_nm;
pub(crate) mod flexray_nm;
pub(crate) mod udp_nm;

pub(crate) use can_nm::*;
pub(crate) use flexray_nm::*;
pub(crate) use udp_nm::*;

//##################################################################

/// The `NmConfig` is the root element for the network management configuration.
///
/// Only one config may exist per `System`, and this configuration may contain multiple `NmClusters` for different bus types.
///
/// Use [`System::create_nm_config`](crate::System::create_nm_config) to create a new `NmConfig` in a `System`.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct NmConfig(pub(crate) autosar_data_abstraction::communication::NmConfig);

#[pymethods]
impl NmConfig {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::NmConfig::try_from(element.0.clone()) {
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

    /// create a new `CanNmCluster`
    #[pyo3(signature = (name, settings, can_cluster, /))]
    #[pyo3(
        text_signature = "(self, name: str, settings: CanNmClusterSettings, can_cluster: CanCluster)"
    )]
    fn create_can_nm_cluster(
        &self,
        name: &str,
        settings: &CanNmClusterSettings,
        can_cluster: &CanCluster,
    ) -> PyResult<CanNmCluster> {
        match self
            .0
            .create_can_nm_cluster(name, &settings.into(), &can_cluster.0)
        {
            Ok(value) => Ok(CanNmCluster(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `FlexrayNmCluster`
    #[pyo3(signature = (name, settings, flexray_cluster, /))]
    #[pyo3(
        text_signature = "(self, name: str, settings: FlexrayNmClusterSettings, flexray_cluster: FlexrayCluster)"
    )]
    fn create_flexray_nm_cluster(
        &self,
        name: &str,
        settings: &FlexrayNmClusterSettings,
        flexray_cluster: &FlexrayCluster,
    ) -> PyResult<FlexrayNmCluster> {
        match self
            .0
            .create_flexray_nm_cluster(name, &settings.into(), &flexray_cluster.0)
        {
            Ok(value) => Ok(FlexrayNmCluster(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `UdpNmCluster`
    #[pyo3(signature = (name, settings, ethernet_cluster, /))]
    #[pyo3(
        text_signature = "(self, name: str, settings: UdpNmClusterSettings, ethernet_cluster: EthernetCluster)"
    )]
    fn create_udp_nm_cluster(
        &self,
        name: &str,
        settings: &UdpNmClusterSettings,
        ethernet_cluster: &EthernetCluster,
    ) -> PyResult<UdpNmCluster> {
        match self
            .0
            .create_udp_nm_cluster(name, &settings.into(), &ethernet_cluster.0)
        {
            Ok(value) => Ok(UdpNmCluster(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// get all `NmClusters`
    fn nm_clusters(&self) -> NmClusterIterator {
        NmClusterIterator::new(self.0.nm_clusters().filter_map(|cluster| match cluster {
            autosar_data_abstraction::communication::NmCluster::CanNm(cluster) => {
                Python::attach(|py| CanNmCluster(cluster).into_py_any(py).ok())
            }
            autosar_data_abstraction::communication::NmCluster::FlexrayNm(cluster) => {
                Python::attach(|py| FlexrayNmCluster(cluster).into_py_any(py).ok())
            }
            autosar_data_abstraction::communication::NmCluster::UdpNm(cluster) => {
                Python::attach(|py| UdpNmCluster(cluster).into_py_any(py).ok())
            }
        }))
    }

    /// create a new `CanNmClusterCoupling`
    #[pyo3(signature = (nm_busload_reduction_enabled, nm_immediate_restart_enabled, /))]
    #[pyo3(
        text_signature = "(self, nm_busload_reduction_enabled: bool, nm_immediate_restart_enabled: bool, /)"
    )]
    fn create_can_nm_cluster_coupling(
        &self,
        nm_busload_reduction_enabled: bool,
        nm_immediate_restart_enabled: bool,
    ) -> PyResult<CanNmClusterCoupling> {
        match self.0.create_can_nm_cluster_coupling(
            nm_busload_reduction_enabled,
            nm_immediate_restart_enabled,
        ) {
            Ok(value) => Ok(CanNmClusterCoupling(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `FlexrayNmClusterCoupling`
    #[pyo3(signature = (nm_schedule_variant, /))]
    #[pyo3(text_signature = "(self, nm_schedule_variant: FlexrayNmScheduleVariant, /)")]
    fn create_flexray_nm_cluster_coupling(
        &self,
        nm_schedule_variant: FlexrayNmScheduleVariant,
    ) -> PyResult<FlexrayNmClusterCoupling> {
        match self
            .0
            .create_flexray_nm_cluster_coupling(nm_schedule_variant.into())
        {
            Ok(value) => Ok(FlexrayNmClusterCoupling(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// create a new `UdpNmClusterCoupling`
    fn create_udp_nm_cluster_coupling(&self) -> PyResult<UdpNmClusterCoupling> {
        match self.0.create_udp_nm_cluster_coupling() {
            Ok(value) => Ok(UdpNmClusterCoupling(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all `NmClusterCouplings`
    fn nm_cluster_couplings(&self) -> NmClusterCouplingIterator {
        NmClusterCouplingIterator::new(self.0.nm_cluster_couplings().filter_map(|coupling| match coupling {
            autosar_data_abstraction::communication::NmClusterCoupling::CanNmClusterCoupling(coupling) => {
                Python::attach(|py| CanNmClusterCoupling(coupling).into_py_any(py).ok())
            }
            autosar_data_abstraction::communication::NmClusterCoupling::FlexrayNmClusterCoupling(coupling) => {
                Python::attach(|py| FlexrayNmClusterCoupling(coupling).into_py_any(py).ok())
            }
            autosar_data_abstraction::communication::NmClusterCoupling::UdpNmClusterCoupling(coupling) => {
                Python::attach(|py| UdpNmClusterCoupling(coupling).into_py_any(py).ok())
            }
        }))
    }

    /// create a new `NmEcu`
    #[pyo3(signature = (name, ecu_instance, /))]
    #[pyo3(text_signature = "(self, name: str, ecu_instance: EcuInstance, /)")]
    fn create_nm_ecu(&self, name: &str, ecu_instance: &EcuInstance) -> PyResult<NmEcu> {
        match self.0.create_nm_ecu(name, &ecu_instance.0) {
            Ok(value) => Ok(NmEcu(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// iterate over all `NmEcus`
    fn nm_ecus(&self) -> NmEcuIterator {
        NmEcuIterator::new(self.0.nm_ecus().map(NmEcu))
    }
}

//##################################################################

iterator_wrapper!(
    NmClusterIterator,
    Py<PyAny>,
    "Union[CanNmCluster, FlexrayNmCluster, UdpNmCluster]"
);
iterator_wrapper!(
    NmClusterCouplingIterator,
    Py<PyAny>,
    "Union[CanNmClusterCoupling, FlexrayNmClusterCoupling, UdpNmClusterCoupling]"
);
iterator_wrapper!(NmEcuIterator, NmEcu);

//##################################################################

/// The `NmEcu` represents an `EcuInstance` wich participates in network management.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct NmEcu(pub(crate) autosar_data_abstraction::communication::NmEcu);

#[pymethods]
impl NmEcu {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::NmEcu::try_from(element.0.clone()) {
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

    /// set the referenced `EcuInstance`
    #[setter]
    fn set_ecu_instance(&self, ecu_instance: &EcuInstance) -> PyResult<()> {
        self.0
            .set_ecu_instance(&ecu_instance.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the referenced `EcuInstance`
    #[getter]
    fn ecu_instance(&self) -> Option<EcuInstance> {
        self.0.ecu_instance().map(EcuInstance)
    }

    /// set the nmBusSynchronizationEnabled flag
    #[setter]
    fn set_nm_bus_synchronization_enabled(&self, value: Option<bool>) -> PyResult<()> {
        self.0
            .set_nm_bus_synchronization_enabled(value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the nmBusSynchronizationEnabled flag
    #[getter]
    fn nm_bus_synchronization_enabled(&self) -> Option<bool> {
        self.0.nm_bus_synchronization_enabled()
    }

    /// set the nmComControlEnabled flag
    #[setter]
    fn set_nm_com_control_enabled(&self, value: Option<bool>) -> PyResult<()> {
        self.0
            .set_nm_com_control_enabled(value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the nmComControlEnabled flag
    #[getter]
    fn nm_com_control_enabled(&self) -> Option<bool> {
        self.0.nm_com_control_enabled()
    }

    /// set or remove the nmCycletimeMainFunction value
    #[setter]
    fn set_cycle_time_main_function(&self, value: Option<f64>) -> PyResult<()> {
        self.0
            .set_cycle_time_main_function(value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the nmCycletimeMainFunction value
    #[getter]
    fn cycle_time_main_function(&self) -> Option<f64> {
        self.0.cycle_time_main_function()
    }
}

//##################################################################

iterator_wrapper!(NmPduIterator, NmPdu);
