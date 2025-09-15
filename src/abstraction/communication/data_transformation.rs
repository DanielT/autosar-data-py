use crate::abstraction::AutosarAbstractionError;
use crate::{abstraction::*, *};
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};

//##################################################################

/// A [`DataTransformationSet`] contains `DataTransformation`s and `TransformationTechnology`s used in communication
///
/// Use [`ArPackage::create_data_transformation_set`] to create a new `DataTransformationSet`
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct DataTransformationSet(
    pub(crate) autosar_data_abstraction::communication::DataTransformationSet,
);

#[pymethods]
impl DataTransformationSet {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::DataTransformationSet::try_from(
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

    /// Create a new `DataTransformation` in the `DataTransformationSet`
    #[pyo3(signature = (name, transformations, execute_despite_data_unavailability, /))]
    #[pyo3(
        text_signature = "(self, name: str, transformations: List[TransformationTechnology], execute_despite_data_unavailability: bool, /)"
    )]
    fn create_data_transformation(
        &self,
        name: &str,
        transformations: Vec<TransformationTechnology>,
        execute_despite_data_unavailability: bool,
    ) -> PyResult<DataTransformation> {
        let transformations = transformations.iter().map(|t| &t.0).collect::<Vec<_>>();
        match self.0.create_data_transformation(
            name,
            &transformations,
            execute_despite_data_unavailability,
        ) {
            Ok(value) => Ok(DataTransformation(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// Iterate over all `DataTransformation`s in the `DataTransformationSet`
    fn data_transformations(&self) -> DataTransformationIterator {
        DataTransformationIterator::new(self.0.data_transformations().map(DataTransformation))
    }

    /// Create a new `TransformationTechnology` in the `DataTransformationSet`
    #[pyo3(signature = (name, config, /))]
    #[pyo3(text_signature = "(self, name: str, config: TransformationTechnologyConfig, /)")]
    fn create_transformation_technology(
        &self,
        name: &str,
        config: &Bound<'_, PyAny>, // some variant of TransformationTechnologyConfig
    ) -> PyResult<TransformationTechnology> {
        let config = transformation_technology_config_from_pyany(config)?;
        match self.0.create_transformation_technology(name, &config) {
            Ok(value) => Ok(TransformationTechnology(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// Iterate over all `TransformationTechnology`s in the `DataTransformationSet`
    fn transformation_technologies(&self) -> TransformationTechnologyIterator {
        TransformationTechnologyIterator::new(
            self.0
                .transformation_technologies()
                .map(TransformationTechnology),
        )
    }
}

//##################################################################

/// A `DataTransformation` is a chain of `TransformationTechnology`s that are used to transform data
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct DataTransformation(
    pub(crate) autosar_data_abstraction::communication::DataTransformation,
);

#[pymethods]
impl DataTransformation {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::DataTransformation::try_from(
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

    /// get the `DataTransformationSet` that contains this `DataTransformation`
    #[getter]
    fn data_transformation_set(&self) -> Option<DataTransformationSet> {
        self.0.data_transformation_set().map(DataTransformationSet)
    }

    /// Create an iterator over the `TransformationTechnologies` in the `DataTransformation`
    fn transformation_technologies(&self) -> TransformationTechnologyIterator {
        TransformationTechnologyIterator::new(
            self.0
                .transformation_technologies()
                .map(TransformationTechnology),
        )
    }
}

//##################################################################

iterator_wrapper!(DataTransformationIterator, DataTransformation);

//##################################################################

/// A `TransformationTechnology` describes how to transform signal or PDU data
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct TransformationTechnology(
    pub(crate) autosar_data_abstraction::communication::TransformationTechnology,
);

#[pymethods]
impl TransformationTechnology {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::TransformationTechnology::try_from(
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

    /// Get the protocol of the `TransformationTechnology`
    #[getter]
    fn protocol(&self) -> Option<String> {
        self.0.protocol()
    }

    /// Get the transformer class of the `TransformationTechnology`
    #[getter]
    fn transformer_class(&self) -> Option<&str> {
        self.0.transformer_class().map(|s| s.to_str())
    }

    /// get the `DataTransformationSet` that contains this `TransformationTechnology`
    #[getter]
    fn data_transformation_set(&self) -> Option<DataTransformationSet> {
        self.0.data_transformation_set().map(DataTransformationSet)
    }

    /// Set the configuration of the `TransformationTechnology`
    #[pyo3(signature = (config, /))]
    #[pyo3(text_signature = "(self, config: TransformationTechnologyConfig, /)")]
    fn set_config(&self, config: &Bound<'_, PyAny>) -> PyResult<()> {
        let config = transformation_technology_config_from_pyany(config)?;
        self.0.set_config(&config).map_err(abstraction_err_to_pyerr)
    }

    /// Get the configuration of the `TransformationTechnology`
    fn config(&self, py: Python) -> Option<Py<PyAny>> {
        self.0
            .config()
            .and_then(|config| transformation_technology_config_to_pyany(py, &config).ok())
    }
}

//##################################################################

iterator_wrapper!(TransformationTechnologyIterator, TransformationTechnology);

//##################################################################

// when we receive a generic Py<PyAny>, we need to determine the actual type of the config and wrap it appropriately
fn transformation_technology_config_from_pyany(
    config: &Bound<'_, PyAny>,
) -> PyResult<autosar_data_abstraction::communication::TransformationTechnologyConfig> {
    if let Ok(config) = config.extract::<GenericTransformationTechnologyConfig>() {
        Ok(
            autosar_data_abstraction::communication::TransformationTechnologyConfig::Generic(
                config.into(),
            ),
        )
    } else if let Ok(config) = config.extract::<ComTransformationTechnologyConfig>() {
        Ok(
            autosar_data_abstraction::communication::TransformationTechnologyConfig::Com(
                config.into(),
            ),
        )
    } else if let Ok(config) = config.extract::<E2ETransformationTechnologyConfig>() {
        Ok(
            autosar_data_abstraction::communication::TransformationTechnologyConfig::E2E(
                config.into(),
            ),
        )
    } else if let Ok(config) = config.extract::<SomeIpTransformationTechnologyConfig>() {
        Ok(
            autosar_data_abstraction::communication::TransformationTechnologyConfig::SomeIp(
                config.into(),
            ),
        )
    } else {
        Err(AutosarAbstractionError::new_err(
            "Invalid TransformationTechnologyConfig".to_string(),
        ))
    }
}

// instead of representing TransformationTechnologyConfig with a matching enum in python, we can simply return generic Py<PyAny>s
fn transformation_technology_config_to_pyany(
    py: Python,
    config: &autosar_data_abstraction::communication::TransformationTechnologyConfig,
) -> PyResult<Py<PyAny>> {
    match config {
        autosar_data_abstraction::communication::TransformationTechnologyConfig::Generic(
            config,
        ) => GenericTransformationTechnologyConfig::from(config).into_py_any(py),
        autosar_data_abstraction::communication::TransformationTechnologyConfig::Com(config) => {
            ComTransformationTechnologyConfig::from(config).into_py_any(py)
        }
        autosar_data_abstraction::communication::TransformationTechnologyConfig::E2E(config) => {
            E2ETransformationTechnologyConfig::from(config).into_py_any(py)
        }
        autosar_data_abstraction::communication::TransformationTechnologyConfig::SomeIp(config) => {
            SomeIpTransformationTechnologyConfig::from(config).into_py_any(py)
        }
    }
}

//##################################################################

/// Configuration for a generic transformation technology
/// For a generic trasformation, the mandatory values must be chosen by the user
#[pyclass(
    get_all,
    set_all,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GenericTransformationTechnologyConfig {
    /// The name of the custom protocol
    pub protocol_name: String,
    /// The version of the custom protocol
    pub protocol_version: String,
    /// The length of the header in bits
    pub header_length: u32,
    /// Should the transformation take place in the existing buffer or in a separate buffer?
    pub in_place: bool,
}

impl From<GenericTransformationTechnologyConfig>
    for autosar_data_abstraction::communication::GenericTransformationTechnologyConfig
{
    fn from(config: GenericTransformationTechnologyConfig) -> Self {
        autosar_data_abstraction::communication::GenericTransformationTechnologyConfig {
            protocol_name: config.protocol_name,
            protocol_version: config.protocol_version,
            header_length: config.header_length,
            in_place: config.in_place,
        }
    }
}

impl From<&autosar_data_abstraction::communication::GenericTransformationTechnologyConfig>
    for GenericTransformationTechnologyConfig
{
    fn from(
        config: &autosar_data_abstraction::communication::GenericTransformationTechnologyConfig,
    ) -> Self {
        GenericTransformationTechnologyConfig {
            protocol_name: config.protocol_name.clone(),
            protocol_version: config.protocol_version.clone(),
            header_length: config.header_length,
            in_place: config.in_place,
        }
    }
}

#[pymethods]
impl GenericTransformationTechnologyConfig {
    #[new]
    #[pyo3(signature = (*, protocol_name, protocol_version, header_length, in_place))]
    #[pyo3(
        text_signature = "(*, protocol_name: str, protocol_version: str, header_length: int, in_place: bool)"
    )]
    fn new(
        protocol_name: String,
        protocol_version: String,
        header_length: u32,
        in_place: bool,
    ) -> Self {
        GenericTransformationTechnologyConfig {
            protocol_name,
            protocol_version,
            header_length,
            in_place,
        }
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}

//#########################################################

/// Configuration for a COM transformation
#[pyclass(
    get_all,
    set_all,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ComTransformationTechnologyConfig {
    /// The length of the `ISignalIpdu` tha will be transformed by this Com transformer.
    /// The value is only used up to AUTOSAR R20-11 (`AUTOSAR_00049`), where it is needed to calculate the buffer size.
    pub isignal_ipdu_length: u32,
}

impl From<ComTransformationTechnologyConfig>
    for autosar_data_abstraction::communication::ComTransformationTechnologyConfig
{
    fn from(config: ComTransformationTechnologyConfig) -> Self {
        autosar_data_abstraction::communication::ComTransformationTechnologyConfig {
            isignal_ipdu_length: config.isignal_ipdu_length,
        }
    }
}

impl From<&autosar_data_abstraction::communication::ComTransformationTechnologyConfig>
    for ComTransformationTechnologyConfig
{
    fn from(
        config: &autosar_data_abstraction::communication::ComTransformationTechnologyConfig,
    ) -> Self {
        ComTransformationTechnologyConfig {
            isignal_ipdu_length: config.isignal_ipdu_length,
        }
    }
}

#[pymethods]
impl ComTransformationTechnologyConfig {
    #[new]
    #[pyo3(signature = (*, isignal_ipdu_length))]
    #[pyo3(text_signature = "(*, isignal_ipdu_length: int)")]
    fn new(isignal_ipdu_length: u32) -> Self {
        ComTransformationTechnologyConfig {
            isignal_ipdu_length,
        }
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}

//#########################################################

/// Configuration for an E2E transformation
#[pyclass(
    get_all,
    set_all,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct E2ETransformationTechnologyConfig {
    /// E2E profile to use
    pub profile: E2EProfile,
    /// When E2E is used in a transformer chain after COM, the header length must be zero.
    /// In this configuration you are expected to provide space for the E2E data inside the signal group layout, and `zero_header_length` should be set to true.
    /// If `zero_header_length` is set to false, the appropriate header length for the chosen E2E profile will be used (e.g. 24 bits for `PROFILE_05`)
    pub zero_header_length: bool,
    /// Should the E2E transformation take place in the existing buffer or in a separate buffer?
    pub transform_in_place: bool,
    /// The offset in bits from the start of the buffer where the E2E data should be placed
    /// If E2E is used after COM, the offset should be 0; if E2E is used after SOMEIP, the offset should be 64
    pub offset: u32,
    /// Maximum jump in the counter value between two consecutive messages
    pub max_delta_counter: u32,
    /// The maximum allowed number of consecutive failed counter checks in the init state
    pub max_error_state_init: u32,
    /// The maximum allowed number of consecutive failed counter checks in the invalid state
    pub max_error_state_invalid: u32,
    /// The maximum allowed number of consecutive failed counter checks in the valid state
    pub max_error_state_valid: u32,
    /// The maximum allowed number of consecutive failed counter checks
    pub max_no_new_or_repeated_data: u32,
    /// The minimum allowed number of consecutive successful counter checks in the init state
    pub min_ok_state_init: u32,
    /// The minimum allowed number of consecutive successful counter checks in the invalid state
    pub min_ok_state_invalid: u32,
    /// The minimum allowed number of consecutive successful counter checks in the valid state
    pub min_ok_state_valid: u32,
    /// window size: Size of the monitoring window for the E2E state machine.
    /// This can be directly set up to AUTOSAR 4.4.0 (`AUTOSAR_00047`).
    /// For newer files this only provides the default if `window_size_init`, `window_size_invalid` and `window_size_valid` are not set
    pub window_size: u32,
    /// window size in the init state - only valid in AUTOSAR 4.5.0 (`AUTOSAR_00048`) and newer. if it is not set, this will default to `window_size`
    pub window_size_init: Option<u32>,
    /// window size in the invalid state - only valid in AUTOSAR 4.5.0 (`AUTOSAR_00048`) and newer. if it is not set, this will default to `window_size`
    pub window_size_invalid: Option<u32>,
    /// window size in the valid state - only valid in AUTOSAR 4.5.0 (`AUTOSAR_00048`) and newer. if it is not set, this will default to `window_size`
    pub window_size_valid: Option<u32>,
    /// Behavior of the check functionality
    pub profile_behavior: Option<E2EProfileBehavior>,
    /// Number of successful checks required for validating the consistency of the counter
    pub sync_counter_init: Option<u32>,
    /// The data ID mode to use; required for E2E profiles 01 and 11, unused otherwise
    pub data_id_mode: Option<DataIdMode>,
    /// Offset of the data ID in the Data[] array in bits. Required for E2E profiles 01 and 11 when `data_id_mode` is `Lower12Bit`, unused otherwise
    pub data_id_nibble_offset: Option<u32>,
    /// Offset of the crc in the Data[] array in bits. Required for E2E profiles 01 and 11, unused otherwise
    pub crc_offset: Option<u32>,
    /// Offset of the counter in the Data[] array in bits. Required for E2E profiles 01 and 11, unused otherwise
    pub counter_offset: Option<u32>,
}

impl From<E2ETransformationTechnologyConfig>
    for autosar_data_abstraction::communication::E2ETransformationTechnologyConfig
{
    fn from(config: E2ETransformationTechnologyConfig) -> Self {
        autosar_data_abstraction::communication::E2ETransformationTechnologyConfig {
            profile: config.profile.into(),
            zero_header_length: config.zero_header_length,
            transform_in_place: config.transform_in_place,
            offset: config.offset,
            max_delta_counter: config.max_delta_counter,
            max_error_state_init: config.max_error_state_init,
            max_error_state_invalid: config.max_error_state_invalid,
            max_error_state_valid: config.max_error_state_valid,
            max_no_new_or_repeated_data: config.max_no_new_or_repeated_data,
            min_ok_state_init: config.min_ok_state_init,
            min_ok_state_invalid: config.min_ok_state_invalid,
            min_ok_state_valid: config.min_ok_state_valid,
            window_size: config.window_size,
            window_size_init: config.window_size_init,
            window_size_invalid: config.window_size_invalid,
            window_size_valid: config.window_size_valid,
            profile_behavior: config.profile_behavior.map(std::convert::Into::into),
            sync_counter_init: config.sync_counter_init,
            data_id_mode: config.data_id_mode.map(std::convert::Into::into),
            data_id_nibble_offset: config.data_id_nibble_offset,
            crc_offset: config.crc_offset,
            counter_offset: config.counter_offset,
        }
    }
}

impl From<&autosar_data_abstraction::communication::E2ETransformationTechnologyConfig>
    for E2ETransformationTechnologyConfig
{
    fn from(
        config: &autosar_data_abstraction::communication::E2ETransformationTechnologyConfig,
    ) -> Self {
        E2ETransformationTechnologyConfig {
            profile: config.profile.into(),
            zero_header_length: config.zero_header_length,
            transform_in_place: config.transform_in_place,
            offset: config.offset,
            max_delta_counter: config.max_delta_counter,
            max_error_state_init: config.max_error_state_init,
            max_error_state_invalid: config.max_error_state_invalid,
            max_error_state_valid: config.max_error_state_valid,
            max_no_new_or_repeated_data: config.max_no_new_or_repeated_data,
            min_ok_state_init: config.min_ok_state_init,
            min_ok_state_invalid: config.min_ok_state_invalid,
            min_ok_state_valid: config.min_ok_state_valid,
            window_size: config.window_size,
            window_size_init: config.window_size_init,
            window_size_invalid: config.window_size_invalid,
            window_size_valid: config.window_size_valid,
            profile_behavior: config.profile_behavior.map(E2EProfileBehavior::from),
            sync_counter_init: config.sync_counter_init,
            data_id_mode: config.data_id_mode.map(DataIdMode::from),
            data_id_nibble_offset: config.data_id_nibble_offset,
            crc_offset: config.crc_offset,
            counter_offset: config.counter_offset,
        }
    }
}

#[pymethods]
impl E2ETransformationTechnologyConfig {
    #[pyo3(signature = (*, profile, zero_header_length, transform_in_place, offset, max_delta_counter, max_error_state_init, max_error_state_invalid,
                        max_error_state_valid, max_no_new_or_repeated_data, min_ok_state_init, min_ok_state_invalid, min_ok_state_valid, window_size,
                        window_size_init=None, window_size_invalid=None, window_size_valid=None, profile_behavior=None, sync_counter_init=None,
                        data_id_mode=None, data_id_nibble_offset=None, crc_offset=None, counter_offset=None))]
    #[pyo3(
        text_signature = "(*, profile: E2EProfile, zero_header_length: bool, transform_in_place: bool, offset: int, max_delta_counter: int, max_error_state_init: int,
                        max_error_state_invalid: int, max_error_state_valid: int, max_no_new_or_repeated_data: int, min_ok_state_init: int, min_ok_state_invalid: int,
                        min_ok_state_valid: int, window_size: int, window_size_init:Optional[int]=None, window_size_invalid:Optional[int]=None,
                        window_size_valid:Optional[int]=None, profile_behavior:Optional[E2EProfileBehavior]=None, sync_counter_init:Optional[int]=None,
                        data_id_mode:Optional[DataIdMode]=None, data_id_nibble_offset:Optional[int]=None, crc_offset:Optional[int]=None, counter_offset:Optional[int]=None)"
    )]
    #[new]
    #[allow(clippy::too_many_arguments)] // having lots of parameters is desirable here, since they are all named parameters in python
    fn new(
        profile: E2EProfile,
        zero_header_length: bool,
        transform_in_place: bool,
        offset: u32,
        max_delta_counter: u32,
        max_error_state_init: u32,
        max_error_state_invalid: u32,
        max_error_state_valid: u32,
        max_no_new_or_repeated_data: u32,
        min_ok_state_init: u32,
        min_ok_state_invalid: u32,
        min_ok_state_valid: u32,
        window_size: u32,
        window_size_init: Option<u32>,
        window_size_invalid: Option<u32>,
        window_size_valid: Option<u32>,
        profile_behavior: Option<E2EProfileBehavior>,
        sync_counter_init: Option<u32>,
        data_id_mode: Option<DataIdMode>,
        data_id_nibble_offset: Option<u32>,
        crc_offset: Option<u32>,
        counter_offset: Option<u32>,
    ) -> Self {
        E2ETransformationTechnologyConfig {
            profile,
            zero_header_length,
            transform_in_place,
            offset,
            max_delta_counter,
            max_error_state_init,
            max_error_state_invalid,
            max_error_state_valid,
            max_no_new_or_repeated_data,
            min_ok_state_init,
            min_ok_state_invalid,
            min_ok_state_valid,
            window_size,
            window_size_init,
            window_size_invalid,
            window_size_valid,
            profile_behavior,
            sync_counter_init,
            data_id_mode,
            data_id_nibble_offset,
            crc_offset,
            counter_offset,
        }
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}

//#########################################################

/// Configuration for a SOMEIP transformation
#[pyclass(
    get_all,
    set_all,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SomeIpTransformationTechnologyConfig {
    /// The alignment of the data in bits
    pub alignment: u32,
    /// The byte order of the data
    pub byte_order: ByteOrder,
    /// The interface version the SOME/IP transformer shall use.
    pub interface_version: u32,
}

impl From<SomeIpTransformationTechnologyConfig>
    for autosar_data_abstraction::communication::SomeIpTransformationTechnologyConfig
{
    fn from(config: SomeIpTransformationTechnologyConfig) -> Self {
        autosar_data_abstraction::communication::SomeIpTransformationTechnologyConfig {
            alignment: config.alignment,
            byte_order: config.byte_order.into(),
            interface_version: config.interface_version,
        }
    }
}

impl From<&autosar_data_abstraction::communication::SomeIpTransformationTechnologyConfig>
    for SomeIpTransformationTechnologyConfig
{
    fn from(
        config: &autosar_data_abstraction::communication::SomeIpTransformationTechnologyConfig,
    ) -> Self {
        SomeIpTransformationTechnologyConfig {
            alignment: config.alignment,
            byte_order: config.byte_order.into(),
            interface_version: config.interface_version,
        }
    }
}

#[pymethods]
impl SomeIpTransformationTechnologyConfig {
    #[pyo3(signature = (*, alignment, byte_order, interface_version))]
    #[pyo3(text_signature = "(*, alignment: int, byte_order: ByteOrder, interface_version: int)")]
    #[new]
    fn new(alignment: u32, byte_order: ByteOrder, interface_version: u32) -> Self {
        SomeIpTransformationTechnologyConfig {
            alignment,
            byte_order,
            interface_version,
        }
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}

//#########################################################

/// enumeration of the possible E2E profiles
#[pyclass(
    frozen,
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum E2EProfile {
    /// E2E Profile 01: Legacy profile, uses a 4-bit counter, 16-bit data id and an 8-bit CRC. New projects should use P11 instead.
    P01,
    /// E2E Profile 02: Legacy profile, uses a 8-bit counter, 8-bit data id and a 8-bit CRC. New projects should use P22 instead.
    P02,
    /// E2E Profile 04: Uses an 16-bit length, 16-bit counter, 32-bit data id and a 32-bit CRC
    P04,
    /// E2E Profile 04m: Uses an 16-bit length, 16-bit counter, 32-bit data id and a 32-bit CRC, as well as source ID, message type and message result
    P04m,
    /// E2E Profile 05: Uses an 8-bit counter, 16-bit data id and a 16-bit CRC
    P05,
    /// E2E Profile 06: Uses a 16-bit length, 8-bit counter, 16-bit data id and a 16-bit CRC
    P06,
    /// E2E Profile 07: Uses an 32-bit length, 32-bit counter, 32-bit data id and a 64-bit CRC
    P07,
    /// E2E Profile 07: Uses an 32-bit length, 32-bit counter, 32-bit data id and a 64-bit CRC, as well as source ID, message type and message result
    P07m,
    /// E2E Profile 08: Uses an 32-bit length, 32-bit counter, 32-bit data id and a 32-bit CRC
    P08,
    /// E2E Profile 08m: Uses an 32-bit length, 32-bit counter, 32-bit data id and a 32-bit CRC, as well as source ID, message type and message result
    P08m,
    /// E2E Profile 11: Uses an 4-bit counter, 16-bit or 12-bit data id and a 8-bit CRC
    P11,
    /// E2E Profile 22: Uses a 4-bit counter, 8-bit data id and a 8-bit CRC
    P22,
    /// E2E Profile 44: Uses a 16-bit length, 16-bit counter, 32-bit data id and a 32-bit CRC
    P44,
    /// E2E Profile 44m: Uses a 16-bit length, 16-bit counter, 32-bit data id and a 32-bit CRC, as well as source ID, message type and message result
    P44m,
}

impl From<E2EProfile> for autosar_data_abstraction::communication::E2EProfile {
    fn from(profile: E2EProfile) -> Self {
        match profile {
            E2EProfile::P01 => autosar_data_abstraction::communication::E2EProfile::P01,
            E2EProfile::P02 => autosar_data_abstraction::communication::E2EProfile::P02,
            E2EProfile::P04 => autosar_data_abstraction::communication::E2EProfile::P04,
            E2EProfile::P04m => autosar_data_abstraction::communication::E2EProfile::P04m,
            E2EProfile::P05 => autosar_data_abstraction::communication::E2EProfile::P05,
            E2EProfile::P06 => autosar_data_abstraction::communication::E2EProfile::P06,
            E2EProfile::P07 => autosar_data_abstraction::communication::E2EProfile::P07,
            E2EProfile::P07m => autosar_data_abstraction::communication::E2EProfile::P07m,
            E2EProfile::P08 => autosar_data_abstraction::communication::E2EProfile::P08,
            E2EProfile::P08m => autosar_data_abstraction::communication::E2EProfile::P08m,
            E2EProfile::P11 => autosar_data_abstraction::communication::E2EProfile::P11,
            E2EProfile::P22 => autosar_data_abstraction::communication::E2EProfile::P22,
            E2EProfile::P44 => autosar_data_abstraction::communication::E2EProfile::P44,
            E2EProfile::P44m => autosar_data_abstraction::communication::E2EProfile::P44m,
        }
    }
}

impl From<autosar_data_abstraction::communication::E2EProfile> for E2EProfile {
    fn from(profile: autosar_data_abstraction::communication::E2EProfile) -> Self {
        match profile {
            autosar_data_abstraction::communication::E2EProfile::P01 => E2EProfile::P01,
            autosar_data_abstraction::communication::E2EProfile::P02 => E2EProfile::P02,
            autosar_data_abstraction::communication::E2EProfile::P04 => E2EProfile::P04,
            autosar_data_abstraction::communication::E2EProfile::P04m => E2EProfile::P04m,
            autosar_data_abstraction::communication::E2EProfile::P05 => E2EProfile::P05,
            autosar_data_abstraction::communication::E2EProfile::P06 => E2EProfile::P06,
            autosar_data_abstraction::communication::E2EProfile::P07 => E2EProfile::P07,
            autosar_data_abstraction::communication::E2EProfile::P07m => E2EProfile::P07m,
            autosar_data_abstraction::communication::E2EProfile::P08 => E2EProfile::P08,
            autosar_data_abstraction::communication::E2EProfile::P08m => E2EProfile::P08m,
            autosar_data_abstraction::communication::E2EProfile::P11 => E2EProfile::P11,
            autosar_data_abstraction::communication::E2EProfile::P22 => E2EProfile::P22,
            autosar_data_abstraction::communication::E2EProfile::P44 => E2EProfile::P44,
            autosar_data_abstraction::communication::E2EProfile::P44m => E2EProfile::P44m,
        }
    }
}

//#########################################################

/// there are two standardized behaviors for E2E profiles, which can be selected for each E2E transformation
#[pyclass(
    frozen,
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum E2EProfileBehavior {
    /// Pre Autosar-R4.2 behavior
    PreR4_2,
    /// behavior according to Autosar-R4.2 and newer
    R4_2,
}

impl From<E2EProfileBehavior> for autosar_data_abstraction::communication::E2EProfileBehavior {
    fn from(behavior: E2EProfileBehavior) -> Self {
        match behavior {
            E2EProfileBehavior::PreR4_2 => {
                autosar_data_abstraction::communication::E2EProfileBehavior::PreR4_2
            }
            E2EProfileBehavior::R4_2 => {
                autosar_data_abstraction::communication::E2EProfileBehavior::R4_2
            }
        }
    }
}

impl From<autosar_data_abstraction::communication::E2EProfileBehavior> for E2EProfileBehavior {
    fn from(behavior: autosar_data_abstraction::communication::E2EProfileBehavior) -> Self {
        match behavior {
            autosar_data_abstraction::communication::E2EProfileBehavior::PreR4_2 => {
                E2EProfileBehavior::PreR4_2
            }
            autosar_data_abstraction::communication::E2EProfileBehavior::R4_2 => {
                E2EProfileBehavior::R4_2
            }
        }
    }
}

//#########################################################

/// data ID modes for E2E profiles 01 and 11
#[allow(clippy::enum_variant_names)] // naming is consistent with the AUTOSAR standard
#[pyclass(
    frozen,
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DataIdMode {
    /// Two bytes of the data id are included in the CRC (double ID configuration).
    All16Bit,
    /// The data id is split into two 8-bit parts, which are included in the CRC in an alternating manner.
    Alternating8Bit,
    /// The low byte is included in the implicit CRC calculation, the low nibble of the high byte is transmitted along with the data
    Lower12Bit,
    /// Only the low byte is included, the high byte is never used
    Lower8Bit,
}

impl From<DataIdMode> for autosar_data_abstraction::communication::DataIdMode {
    fn from(mode: DataIdMode) -> Self {
        match mode {
            DataIdMode::All16Bit => autosar_data_abstraction::communication::DataIdMode::All16Bit,
            DataIdMode::Alternating8Bit => {
                autosar_data_abstraction::communication::DataIdMode::Alternating8Bit
            }
            DataIdMode::Lower12Bit => {
                autosar_data_abstraction::communication::DataIdMode::Lower12Bit
            }
            DataIdMode::Lower8Bit => autosar_data_abstraction::communication::DataIdMode::Lower8Bit,
        }
    }
}

impl From<autosar_data_abstraction::communication::DataIdMode> for DataIdMode {
    fn from(mode: autosar_data_abstraction::communication::DataIdMode) -> Self {
        match mode {
            autosar_data_abstraction::communication::DataIdMode::All16Bit => DataIdMode::All16Bit,
            autosar_data_abstraction::communication::DataIdMode::Alternating8Bit => {
                DataIdMode::Alternating8Bit
            }
            autosar_data_abstraction::communication::DataIdMode::Lower12Bit => {
                DataIdMode::Lower12Bit
            }
            autosar_data_abstraction::communication::DataIdMode::Lower8Bit => DataIdMode::Lower8Bit,
        }
    }
}

//#########################################################

/// Properties for the End to End transformation of an ISignal(Group)
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct EndToEndTransformationISignalProps(
    pub(crate) autosar_data_abstraction::communication::EndToEndTransformationISignalProps,
);

#[pymethods]
impl EndToEndTransformationISignalProps {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::EndToEndTransformationISignalProps::try_from(
            element.0.clone(),
        ) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    #[getter]
    fn element(&self) -> Element {
        Element(self.0.element().clone())
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self.0)
    }

    /// set the transformer reference of the E2E transformation properties
    #[setter]
    fn set_transformer(&self, transformer: &TransformationTechnology) -> PyResult<()> {
        self.0
            .set_transformer(&transformer.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the transformer reference of the E2E transformation properties
    #[getter]
    fn transformer(&self) -> Option<TransformationTechnology> {
        self.0.transformer().map(TransformationTechnology)
    }

    /// set the data IDs that are used for the E2E transformation
    #[setter]
    fn set_data_ids(&self, data_ids: Vec<u32>) -> PyResult<()> {
        self.0
            .set_data_ids(&data_ids)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the data IDs that are used for the E2E transformation
    #[getter]
    fn data_ids(&self) -> Vec<u32> {
        self.0.data_ids()
    }

    /// set the length of payload and E2E header in bits
    #[setter]
    fn set_data_length(&self, data_length: Option<u32>) -> PyResult<()> {
        self.0
            .set_data_length(data_length)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the length of payload and E2E header in bits
    #[getter]
    fn data_length(&self) -> Option<u32> {
        self.0.data_length()
    }

    /// set the maximum data length
    #[setter]
    fn set_max_data_length(&self, max_data_length: Option<u32>) -> PyResult<()> {
        self.0
            .set_max_data_length(max_data_length)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the maximum data length
    #[getter]
    fn max_data_length(&self) -> Option<u32> {
        self.0.max_data_length()
    }

    /// set the minimum data length
    #[setter]
    fn set_min_data_length(&self, min_data_length: Option<u32>) -> PyResult<()> {
        self.0
            .set_min_data_length(min_data_length)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the minimum data length
    #[getter]
    fn min_data_length(&self) -> Option<u32> {
        self.0.min_data_length()
    }

    /// set the source ID
    #[setter]
    fn set_source_id(&self, source_id: Option<u32>) -> PyResult<()> {
        self.0
            .set_source_id(source_id)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the source ID
    #[getter]
    fn source_id(&self) -> Option<u32> {
        self.0.source_id()
    }
}

//#########################################################

/// Properties for the SOMEIP transformation of an ISignal(Group)
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct SomeIpTransformationISignalProps(
    pub(crate) autosar_data_abstraction::communication::SomeIpTransformationISignalProps,
);

#[pymethods]
impl SomeIpTransformationISignalProps {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::SomeIpTransformationISignalProps::try_from(
            element.0.clone(),
        ) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    #[getter]
    fn element(&self) -> Element {
        Element(self.0.element().clone())
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self.0)
    }

    /// set the transformer reference of the E2E transformation properties
    #[setter]
    fn set_transformer(&self, transformer: &TransformationTechnology) -> PyResult<()> {
        self.0
            .set_transformer(&transformer.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the transformer reference of the E2E transformation properties
    #[getter]
    fn transformer(&self) -> Option<TransformationTechnology> {
        self.0.transformer().map(TransformationTechnology)
    }

    /// set the legacy strings property
    #[setter]
    fn set_legacy_strings(&self, legacy_strings: Option<bool>) -> PyResult<()> {
        self.0
            .set_legacy_strings(legacy_strings)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the legacy strings property
    #[getter]
    fn legacy_strings(&self) -> Option<bool> {
        self.0.legacy_strings()
    }

    /// set the interface version property
    #[setter]
    fn set_interface_version(&self, interface_version: Option<u32>) -> PyResult<()> {
        self.0
            .set_interface_version(interface_version)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the interface version property
    #[getter]
    fn interface_version(&self) -> Option<u32> {
        self.0.interface_version()
    }

    /// set the dynamic length property
    #[setter]
    fn set_dynamic_length(&self, dynamic_length: Option<bool>) -> PyResult<()> {
        self.0
            .set_dynamic_length(dynamic_length)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the dynamic length property
    #[getter]
    fn dynamic_length(&self) -> Option<bool> {
        self.0.dynamic_length()
    }

    /// set the message type property
    #[setter]
    fn set_message_type(&self, message_type: Option<SomeIpMessageType>) -> PyResult<()> {
        self.0
            .set_message_type(message_type.map(std::convert::Into::into))
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the message type property
    #[getter]
    fn message_type(&self) -> Option<SomeIpMessageType> {
        self.0.message_type().map(SomeIpMessageType::from)
    }

    /// set the size of array length property
    #[setter]
    fn set_size_of_array_length(&self, size_of_array_length: Option<u32>) -> PyResult<()> {
        self.0
            .set_size_of_array_length(size_of_array_length)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the size of array length property
    #[getter]
    fn size_of_array_length(&self) -> Option<u32> {
        self.0.size_of_array_length()
    }

    /// set the size of string length property
    #[setter]
    fn set_size_of_string_length(&self, size_of_string_length: Option<u32>) -> PyResult<()> {
        self.0
            .set_size_of_string_length(size_of_string_length)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the size of string length property
    #[getter]
    fn size_of_string_length(&self) -> Option<u32> {
        self.0.size_of_string_length()
    }

    /// set the size of struct length property
    #[setter]
    fn set_size_of_struct_length(&self, size_of_struct_length: Option<u32>) -> PyResult<()> {
        self.0
            .set_size_of_struct_length(size_of_struct_length)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the size of struct length property
    #[getter]
    fn size_of_struct_length(&self) -> Option<u32> {
        self.0.size_of_struct_length()
    }

    /// set the size of union length property
    #[setter]
    fn set_size_of_union_length(&self, size_of_union_length: Option<u32>) -> PyResult<()> {
        self.0
            .set_size_of_union_length(size_of_union_length)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the size of union length property
    #[getter]
    fn size_of_union_length(&self) -> Option<u32> {
        self.0.size_of_union_length()
    }
}

//#########################################################

/// message types that can be used in a SOME/IP message header, depending on the type of communication
#[pyclass(
    frozen,
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SomeIpMessageType {
    /// Notification message
    Notification,
    /// Request message, which expects a response
    Request,
    /// Request without return message - a fire and forget message
    RequestNoReturn,
    /// Response message
    Response,
}

impl From<SomeIpMessageType> for autosar_data_abstraction::communication::SomeIpMessageType {
    fn from(message_type: SomeIpMessageType) -> Self {
        match message_type {
            SomeIpMessageType::Notification => {
                autosar_data_abstraction::communication::SomeIpMessageType::Notification
            }
            SomeIpMessageType::Request => {
                autosar_data_abstraction::communication::SomeIpMessageType::Request
            }
            SomeIpMessageType::RequestNoReturn => {
                autosar_data_abstraction::communication::SomeIpMessageType::RequestNoReturn
            }
            SomeIpMessageType::Response => {
                autosar_data_abstraction::communication::SomeIpMessageType::Response
            }
        }
    }
}

impl From<autosar_data_abstraction::communication::SomeIpMessageType> for SomeIpMessageType {
    fn from(message_type: autosar_data_abstraction::communication::SomeIpMessageType) -> Self {
        match message_type {
            autosar_data_abstraction::communication::SomeIpMessageType::Notification => {
                SomeIpMessageType::Notification
            }
            autosar_data_abstraction::communication::SomeIpMessageType::Request => {
                SomeIpMessageType::Request
            }
            autosar_data_abstraction::communication::SomeIpMessageType::RequestNoReturn => {
                SomeIpMessageType::RequestNoReturn
            }
            autosar_data_abstraction::communication::SomeIpMessageType::Response => {
                SomeIpMessageType::Response
            }
        }
    }
}
