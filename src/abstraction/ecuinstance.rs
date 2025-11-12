use crate::abstraction::{
    AutosarAbstractionError, abstraction_err_to_pyerr,
    communication::{
        CanCommunicationController, EthernetCommunicationController,
        FlexrayCommunicationController, ISignalIPduGroup,
    },
};
use crate::{Element, iterator_wrapper};
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};
use pyo3::{IntoPyObjectExt, prelude::*};

//##################################################################

/// The `EcuInstance` represents one ECU in a `System`
#[pyclass(frozen, eq, module = "autosar_data._autosar_data._abstraction")]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct EcuInstance(pub(crate) autosar_data_abstraction::EcuInstance);

#[pymethods]
impl EcuInstance {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::EcuInstance::try_from(element.0.clone()) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    #[pyo3(signature = (/, *, deep = false))]
    #[pyo3(text_signature = "(self, /, *, deep: bool = false)")]
    fn remove(&self, deep: bool) -> PyResult<()> {
        self.clone()
            .0
            .remove(deep)
            .map_err(abstraction_err_to_pyerr)
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

    /// Create a CAN-COMMUNICATION-CONTROLLER for this ECU-INSTANCE
    ///
    /// The ECU must have one controller per bus it communicates on.
    /// For example, if it communicates on two CAN buses, then two CAN-COMMUNICATION-CONTROLLERs are needed.
    //#[pyo3(signature = (name, /))]
    #[pyo3(text_signature = "(self, name: str, /)")]
    fn create_can_communication_controller(
        &self,
        name: &str,
    ) -> PyResult<CanCommunicationController> {
        match self.0.create_can_communication_controller(name) {
            Ok(value) => Ok(CanCommunicationController(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// Create an ETHERNET-COMMUNICATION-CONTROLLER for this ECU-INSTANCE
    ///
    /// The ECU must have one controller per bus it communicates on.
    /// For example, if it communicates on two CAN buses, then two CAN-COMMUNICATION-CONTROLLERs are needed.
    #[pyo3(signature = (name, /, *, mac_address=None))]
    #[pyo3(text_signature = "(self, name: str, /, *, mac_address: Optional[str] = None)")]
    fn create_ethernet_communication_controller(
        &self,
        name: &str,
        mac_address: Option<String>,
    ) -> PyResult<EthernetCommunicationController> {
        match self
            .0
            .create_ethernet_communication_controller(name, mac_address)
        {
            Ok(value) => Ok(EthernetCommunicationController(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// Create a FLEXRAY-COMMUNICATION-CONTROLLER for this ECU-INSTANCE
    ///
    /// The ECU must have one controller per bus it communicates on.
    /// For example, if it communicates on two CAN buses, then two CAN-COMMUNICATION-CONTROLLERs are needed.
    #[pyo3(signature = (name, /))]
    #[pyo3(text_signature = "(self, name: str, /)")]
    fn create_flexray_communication_controller(
        &self,
        name: &str,
    ) -> PyResult<FlexrayCommunicationController> {
        match self.0.create_flexray_communication_controller(name) {
            Ok(value) => Ok(FlexrayCommunicationController(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// return an interator over all communication controllers in this `EcuInstance`
    fn communication_controllers(&self) -> CommunicationControllersIterator {
        CommunicationControllersIterator::new(self.0.communication_controllers().filter_map(
            |comm_controller| match comm_controller {
                autosar_data_abstraction::communication::CommunicationController::Can(
                    controller,
                ) => {
                    Python::attach(|py| CanCommunicationController(controller).into_py_any(py).ok())
                }
                autosar_data_abstraction::communication::CommunicationController::Ethernet(
                    controller,
                ) => Python::attach(|py| {
                    EthernetCommunicationController(controller)
                        .into_py_any(py)
                        .ok()
                }),
                autosar_data_abstraction::communication::CommunicationController::Flexray(
                    controller,
                ) => Python::attach(|py| {
                    FlexrayCommunicationController(controller)
                        .into_py_any(py)
                        .ok()
                }),
                _ => None,
            },
        ))
    }

    /// Add a reference to an associated COM IPdu group
    #[pyo3(signature = (group, /))]
    #[pyo3(text_signature = "(self, group: ISignalIPduGroup, /)")]
    fn add_associated_com_ipdu_group(&self, group: &ISignalIPduGroup) -> PyResult<()> {
        self.0
            .add_associated_com_ipdu_group(&group.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Iterate over all associated COM IPdu groups
    fn associated_com_ipdu_groups(&self) -> ISignalIPduGroupIterator {
        ISignalIPduGroupIterator::new(self.0.associated_com_ipdu_groups().map(ISignalIPduGroup))
    }
}

//##################################################################

iterator_wrapper!(
    CommunicationControllersIterator,
    Py<PyAny>,
    "CommunicationController"
);
iterator_wrapper!(ISignalIPduGroupIterator, ISignalIPduGroup);

//##################################################################
