use crate::{
    Element,
    abstraction::{
        AutosarAbstractionError, abstraction_err_to_pyerr,
        communication::{
            ContainedIPduProps, PduTriggering, pyany_to_ipdu, pyany_to_physical_channel,
        },
    },
};
use autosar_data_abstraction::{
    self, AbstractionElement, IdentifiableAbstractionElement,
    communication::{AbstractIpdu, AbstractPdu},
};
use pyo3::prelude::*;

//##################################################################

/// Wraps an `IPdu` to protect it from unauthorized manipulation
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct SecuredIPdu(pub(crate) autosar_data_abstraction::communication::SecuredIPdu);

#[pymethods]
impl SecuredIPdu {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::SecuredIPdu::try_from(element.0.clone()) {
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

    /// set the properties of the secured communication
    #[setter]
    fn set_secure_communication_props(&self, props: &SecureCommunicationProps) -> PyResult<()> {
        self.0
            .set_secure_communication_props(&props.into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the properties of the secured communication
    #[getter]
    fn secure_communication_props(&self) -> Option<SecureCommunicationProps> {
        self.0.secure_communication_props().map(Into::into)
    }

    /// set or remove the useAsCryptographicIPdu flag
    #[setter]
    fn set_use_as_cryptographic_ipdu(&self, value: Option<bool>) -> PyResult<()> {
        self.0
            .set_use_as_cryptographic_ipdu(value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the useAsCryptographicIPdu flag
    #[getter]
    fn use_as_cryptographic_ipdu(&self) -> Option<bool> {
        self.0.use_as_cryptographic_ipdu()
    }

    /// set the payload PduTriggering based on an IPdu
    ///
    /// This function should be used when useAsCryptographicIPdu is false or not set.
    /// A PduTriggering is created for the Pdu
    #[pyo3(signature = (ipdu, physical_channel, /))]
    #[pyo3(text_signature = "(self, ipdu: IPdu, physical_channel: PhysicalChannel, /)")]
    fn set_payload_ipdu(
        &self,
        ipdu: &Bound<'_, PyAny>,
        physical_channel: &Bound<'_, PyAny>,
    ) -> PyResult<PduTriggering> {
        let ipdu = pyany_to_ipdu(ipdu)?;
        let physical_channel = pyany_to_physical_channel(physical_channel)?;
        match self.0.set_payload_ipdu(&ipdu, &physical_channel) {
            Ok(value) => Ok(PduTriggering(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// set the payload PduTriggering with an existing PduTriggering
    ///
    /// This function should be used when useAsCryptographicIPdu is true.
    /// In this case the payload is transmitted separately from the
    /// cryptographic data, so the PduTriggering already exists.
    #[setter]
    fn set_payload_pdu_triggering(&self, pdu_triggering: &PduTriggering) -> PyResult<()> {
        self.0
            .set_payload_pdu_triggering(&pdu_triggering.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the payload PduTriggering
    #[getter]
    fn payload_pdu_triggering(&self) -> Option<PduTriggering> {
        self.0.payload_pdu_triggering().map(PduTriggering)
    }

    // --------- AbstractPdu methods ---------

    /// set the length of this PDU
    #[setter]
    fn set_length(&self, length: u32) -> PyResult<()> {
        self.0.set_length(length).map_err(abstraction_err_to_pyerr)
    }

    /// get the length of this PDU
    #[getter]
    fn length(&self) -> Option<u32> {
        self.0.length()
    }

    /// iterate over the `PduTriggerings` that trigger this PDU
    fn pdu_triggerings(&self) -> Vec<PduTriggering> {
        self.0
            .pdu_triggerings()
            .into_iter()
            .map(PduTriggering)
            .collect()
    }

    // --------- AbstractIPdu methods ---------

    /// set the ContainedIPduProps for this `IPdu`
    ///
    /// This is only relevant for IPdus that will be transmitted in `ContainerIPdus`
    #[setter]
    fn set_contained_ipdu_props(&self, props: Option<&ContainedIPduProps>) -> PyResult<()> {
        self.0
            .set_contained_ipdu_props(props.map(Into::into).as_ref())
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the ContainedIPduProps for this `IPdu`
    #[getter]
    fn contained_ipdu_props(&self) -> Option<ContainedIPduProps> {
        self.0.contained_ipdu_props().map(Into::into)
    }
}

//##################################################################

/// The properties of a `SecuredIPdu`
#[pyclass(
    get_all,
    set_all,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct SecureCommunicationProps {
    /// length in bits of the authentic PDU data
    pub auth_data_freshness_length: Option<u32>,
    /// start position in bits of the authentic PDU data
    pub auth_data_freshness_start_position: Option<u32>,
    /// number of authentication build attempts
    pub authentication_build_attempts: Option<u32>,
    /// number of additional authentication attempts. If this value is zero, the authentication is not repeated
    pub authentication_retries: Option<u32>,
    /// numerical identifier of the secured IPdu
    pub data_id: Option<u32>,
    /// id of the freshness value
    pub freshness_value_id: Option<u32>,
    /// message link length in bits
    pub message_link_length: Option<u32>,
    /// message link start position in bits
    pub message_link_position: Option<u32>,
    /// seconday freshness value id
    pub secondary_freshness_value_id: Option<u32>,
    /// length in bytes of the secure area inside the payload pdu
    pub secured_area_length: Option<u32>,
    /// start position in bytes of the secure area inside the payload pdu
    pub secured_area_offset: Option<u32>,
}

impl From<&SecureCommunicationProps>
    for autosar_data_abstraction::communication::SecureCommunicationProps
{
    fn from(props: &SecureCommunicationProps) -> Self {
        autosar_data_abstraction::communication::SecureCommunicationProps {
            auth_data_freshness_length: props.auth_data_freshness_length,
            auth_data_freshness_start_position: props.auth_data_freshness_start_position,
            authentication_build_attempts: props.authentication_build_attempts,
            authentication_retries: props.authentication_retries,
            data_id: props.data_id,
            freshness_value_id: props.freshness_value_id,
            message_link_length: props.message_link_length,
            message_link_position: props.message_link_position,
            secondary_freshness_value_id: props.secondary_freshness_value_id,
            secured_area_length: props.secured_area_length,
            secured_area_offset: props.secured_area_offset,
        }
    }
}

impl From<autosar_data_abstraction::communication::SecureCommunicationProps>
    for SecureCommunicationProps
{
    fn from(props: autosar_data_abstraction::communication::SecureCommunicationProps) -> Self {
        SecureCommunicationProps {
            auth_data_freshness_length: props.auth_data_freshness_length,
            auth_data_freshness_start_position: props.auth_data_freshness_start_position,
            authentication_build_attempts: props.authentication_build_attempts,
            authentication_retries: props.authentication_retries,
            data_id: props.data_id,
            freshness_value_id: props.freshness_value_id,
            message_link_length: props.message_link_length,
            message_link_position: props.message_link_position,
            secondary_freshness_value_id: props.secondary_freshness_value_id,
            secured_area_length: props.secured_area_length,
            secured_area_offset: props.secured_area_offset,
        }
    }
}

#[pymethods]
impl SecureCommunicationProps {
    /// Create a new `SecureCommunicationProps` with the given properties
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (*, auth_data_freshness_length=None, auth_data_freshness_start_position=None, authentication_build_attempts=None,
                        authentication_retries=None, data_id=None, freshness_value_id=None, message_link_length=None, message_link_position=None,
                        secondary_freshness_value_id=None, secured_area_length=None, secured_area_offset=None))]
    #[pyo3(text_signature = "(self, /, *,
        auth_data_freshness_length: Optional[int]=None,
        auth_data_freshness_start_position: Optional[int]=None,
        authentication_build_attempts: Optional[int]=None,
        authentication_retries: Optional[int]=None,
        data_id: Optional[int]=None,
        freshness_value_id: Optional[int]=None,
        message_link_length: Optional[int]=None,
        message_link_position: Optional[int]=None,
        secondary_freshness_value_id: Optional[int]=None,
        secured_area_length: Optional[int]=None,
        secured_area_offset: Optional[int]=None)")]
    #[new]
    pub fn new(
        auth_data_freshness_length: Option<u32>,
        auth_data_freshness_start_position: Option<u32>,
        authentication_build_attempts: Option<u32>,
        authentication_retries: Option<u32>,
        data_id: Option<u32>,
        freshness_value_id: Option<u32>,
        message_link_length: Option<u32>,
        message_link_position: Option<u32>,
        secondary_freshness_value_id: Option<u32>,
        secured_area_length: Option<u32>,
        secured_area_offset: Option<u32>,
    ) -> Self {
        SecureCommunicationProps {
            auth_data_freshness_length,
            auth_data_freshness_start_position,
            authentication_build_attempts,
            authentication_retries,
            data_id,
            freshness_value_id,
            message_link_length,
            message_link_position,
            secondary_freshness_value_id,
            secured_area_length,
            secured_area_offset,
        }
    }
}
