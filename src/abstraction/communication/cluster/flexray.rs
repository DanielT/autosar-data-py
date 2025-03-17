use crate::{
    Element,
    abstraction::{
        AutosarAbstractionError, System, abstraction_err_to_pyerr,
        communication::{FlexrayChannelName, FlexrayPhysicalChannel},
    },
};
use autosar_data_abstraction::{
    self, AbstractionElement, IdentifiableAbstractionElement, communication::AbstractCluster,
};
use pyo3::prelude::*;

//##################################################################

/// A `FlexrayCluster` contains all configuration items associated with a Flexray network.
/// The cluster connects multiple ECUs.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct FlexrayCluster(
    pub(crate) autosar_data_abstraction::communication::FlexrayCluster,
);

#[pymethods]
impl FlexrayCluster {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::FlexrayCluster::try_from(element.0.clone()) {
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

    /// update the cluster settings
    ///
    /// The settings of a flexray cluster determine all the details of timing and slot layout.
    /// These settings are subject to multiple cross dependencies and constraints.
    ///
    /// You may check the validity of the settings by calling [`FlexrayClusterSettings::verify`].
    ///
    /// However, the update function does not require that the settings are valid, and will
    /// also update the model with invalid settings if desired.
    fn set_settings(&self, settings: &FlexrayClusterSettings) {
        self.0.update_settings(&settings.0);
    }

    /// retrieve the current flexray cluster settings from a [`FlexrayCluster`]
    fn settings(&self) -> FlexrayClusterSettings {
        FlexrayClusterSettings(self.0.settings())
    }

    /// Create a new physical channel for the cluster
    ///
    /// A cluster may contain channel A, channel B, or both A and B.
    #[pyo3(signature = (name, channel_name, /))]
    #[pyo3(text_signature = "(self, name: str, channel_name: FlexrayChannelName, /)")]
    fn create_physical_channel(
        &self,
        name: &str,
        channel_name: FlexrayChannelName,
    ) -> PyResult<FlexrayPhysicalChannel> {
        match self.0.create_physical_channel(name, channel_name.into()) {
            Ok(channel) => Ok(FlexrayPhysicalChannel(channel)),
            Err(error) => Err(AutosarAbstractionError::new_err(error.to_string())),
        }
    }

    /// get the physical channels of this cluster
    #[getter]
    fn physical_channels(&self) -> FlexrayPhysicalChannelsInfo {
        FlexrayPhysicalChannelsInfo(self.0.physical_channels())
    }

    /// get the `System` this `CanCluster` is part of
    #[getter]
    fn system(&self) -> Option<System> {
        self.0.system().map(System)
    }
}

//##################################################################

/// Information about the flexray physical channels present inside a cluster
#[pyclass(eq, module = "autosar_data._autosar_data._abstraction._communication")]
#[derive(Clone, PartialEq)]
pub(crate) struct FlexrayPhysicalChannelsInfo(
    pub(crate) autosar_data_abstraction::communication::FlexrayPhysicalChannelsInfo,
);

#[pymethods]
impl FlexrayPhysicalChannelsInfo {
    fn __repr__(&self) -> String {
        format!("{:#?}", self.0)
    }

    /// get the channel A of the cluster
    #[getter]
    fn channel_a(&self) -> Option<FlexrayPhysicalChannel> {
        Some(FlexrayPhysicalChannel(self.0.channel_a.as_ref()?.clone()))
    }

    /// get the channel B of the cluster
    #[getter]
    fn channel_b(&self) -> Option<FlexrayPhysicalChannel> {
        Some(FlexrayPhysicalChannel(self.0.channel_b.as_ref()?.clone()))
    }
}

//##################################################################

#[pyclass(eq, module = "autosar_data._autosar_data._abstraction._communication")]
#[derive(Clone, PartialEq)]
pub(crate) struct FlexrayClusterSettings(
    pub(crate) autosar_data_abstraction::communication::FlexrayClusterSettings,
);

#[pymethods]
impl FlexrayClusterSettings {
    /// Create a new `FlexrayClusterSettings`
    #[new]
    pub(crate) fn new() -> Self {
        Self(autosar_data_abstraction::communication::FlexrayClusterSettings::default())
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self.0)
    }

    /// verify the settings of a flexray cluster
    #[pyo3(text_signature = "(self, /)")]
    pub(crate) fn verify(&self) -> bool {
        self.0.verify()
    }

    /// get the baudrate of the cluster
    #[getter]
    pub(crate) fn baudrate(&self) -> u32 {
        self.0.baudrate
    }

    /// set the baudrate of the cluster
    #[setter]
    pub(crate) fn set_baudrate(&mut self, baudrate: u32) {
        self.0.baudrate = baudrate;
    }

    /// get the action point offset of the cluster
    #[getter]
    pub(crate) fn action_point_offset(&self) -> u8 {
        self.0.action_point_offset
    }

    /// set the action point offset of the cluster
    #[setter]
    pub(crate) fn set_action_point_offset(&mut self, action_point_offset: u8) {
        self.0.action_point_offset = action_point_offset;
    }

    /// get the bit time of the cluster
    #[getter]
    pub(crate) fn bit(&self) -> f64 {
        self.0.bit
    }

    /// set the bit time of the cluster
    #[setter]
    pub(crate) fn set_bit(&mut self, bit: f64) {
        self.0.bit = bit;
    }

    /// get the cas rx low max of the cluster
    #[getter]
    pub(crate) fn cas_rx_low_max(&self) -> u8 {
        self.0.cas_rx_low_max
    }

    /// set the cas rx low max of the cluster
    #[setter]
    pub(crate) fn set_cas_rx_low_max(&mut self, cas_rx_low_max: u8) {
        self.0.cas_rx_low_max = cas_rx_low_max;
    }

    /// get the cold start attempts of the cluster
    #[getter]
    pub(crate) fn cold_start_attempts(&self) -> u8 {
        self.0.cold_start_attempts
    }

    /// set the cold start attempts of the cluster
    #[setter]
    pub(crate) fn set_cold_start_attempts(&mut self, cold_start_attempts: u8) {
        self.0.cold_start_attempts = cold_start_attempts;
    }

    /// get the cycle time of the cluster (in seconds)
    #[getter]
    pub(crate) fn cycle(&self) -> f64 {
        self.0.cycle
    }

    /// set the cycle time of the cluster (in seconds)
    #[setter]
    pub(crate) fn set_cycle(&mut self, cycle: f64) {
        self.0.cycle = cycle;
    }

    /// get the cycle count max of the cluster
    #[getter]
    pub(crate) fn cycle_count_max(&self) -> u8 {
        self.0.cycle_count_max
    }

    /// set the cycle count max of the cluster
    #[setter]
    pub(crate) fn set_cycle_count_max(&mut self, cycle_count_max: u8) {
        self.0.cycle_count_max = cycle_count_max;
    }

    /// get the detect nit error status of the cluster
    #[getter]
    pub(crate) fn detect_nit_error(&self) -> bool {
        self.0.detect_nit_error
    }

    /// set the detect nit error status of the cluster
    #[setter]
    pub(crate) fn set_detect_nit_error(&mut self, detect_nit_error: bool) {
        self.0.detect_nit_error = detect_nit_error;
    }

    /// get the dynamic slot idle phase of the cluster
    #[getter]
    pub(crate) fn dynamic_slot_idle_phase(&self) -> u8 {
        self.0.dynamic_slot_idle_phase
    }

    /// set the dynamic slot idle phase of the cluster
    #[setter]
    pub(crate) fn set_dynamic_slot_idle_phase(&mut self, dynamic_slot_idle_phase: u8) {
        self.0.dynamic_slot_idle_phase = dynamic_slot_idle_phase;
    }

    /// get the ignore after tx duration of the cluster
    #[getter]
    pub(crate) fn ignore_after_tx(&self) -> u16 {
        self.0.ignore_after_tx
    }

    /// set the ignore after tx duration of the cluster
    #[setter]
    pub(crate) fn set_ignore_after_tx(&mut self, ignore_after_tx: u16) {
        self.0.ignore_after_tx = ignore_after_tx;
    }

    /// get the listen noise of the cluster
    #[getter]
    pub(crate) fn listen_noise(&self) -> u8 {
        self.0.listen_noise
    }

    /// set the listen noise of the cluster
    #[setter]
    pub(crate) fn set_listen_noise(&mut self, listen_noise: u8) {
        self.0.listen_noise = listen_noise;
    }

    /// get the macro per cycle of the cluster
    #[getter]
    pub(crate) fn macro_per_cycle(&self) -> u16 {
        self.0.macro_per_cycle
    }

    /// set the macro per cycle of the cluster
    #[setter]
    pub(crate) fn set_macro_per_cycle(&mut self, macro_per_cycle: u16) {
        self.0.macro_per_cycle = macro_per_cycle;
    }

    /// get the macrotick duration of the cluster
    #[getter]
    pub(crate) fn macrotick_duration(&self) -> f64 {
        self.0.macrotick_duration
    }

    /// set the macrotick duration of the cluster
    #[setter]
    pub(crate) fn set_macrotick_duration(&mut self, macrotick_duration: f64) {
        self.0.macrotick_duration = macrotick_duration;
    }

    /// get the max without clock correction fatal of the cluster
    #[getter]
    pub(crate) fn max_without_clock_correction_fatal(&self) -> u8 {
        self.0.max_without_clock_correction_fatal
    }

    /// set the max without clock correction fatal of the cluster
    #[setter]
    pub(crate) fn set_max_without_clock_correction_fatal(
        &mut self,
        max_without_clock_correction_fatal: u8,
    ) {
        self.0.max_without_clock_correction_fatal = max_without_clock_correction_fatal;
    }

    /// get the max without clock correction passive of the cluster
    #[getter]
    pub(crate) fn max_without_clock_correction_passive(&self) -> u8 {
        self.0.max_without_clock_correction_passive
    }

    /// set the max without clock correction passive of the cluster
    #[setter]
    pub(crate) fn set_max_without_clock_correction_passive(
        &mut self,
        max_without_clock_correction_passive: u8,
    ) {
        self.0.max_without_clock_correction_passive = max_without_clock_correction_passive;
    }

    /// get the minislot action point offset of the cluster
    #[getter]
    pub(crate) fn minislot_action_point_offset(&self) -> u8 {
        self.0.minislot_action_point_offset
    }

    /// set the minislot action point offset of the cluster
    #[setter]
    pub(crate) fn set_minislot_action_point_offset(&mut self, minislot_action_point_offset: u8) {
        self.0.minislot_action_point_offset = minislot_action_point_offset;
    }

    /// get the minislot duration of the cluster
    #[getter]
    pub(crate) fn minislot_duration(&self) -> u8 {
        self.0.minislot_duration
    }

    /// set the minislot duration of the cluster
    #[setter]
    pub(crate) fn set_minislot_duration(&mut self, minislot_duration: u8) {
        self.0.minislot_duration = minislot_duration;
    }

    /// get the network idle time of the cluster
    #[getter]
    pub(crate) fn network_idle_time(&self) -> u16 {
        self.0.network_idle_time
    }

    /// set the network idle time of the cluster
    #[setter]
    pub(crate) fn set_network_idle_time(&mut self, network_idle_time: u16) {
        self.0.network_idle_time = network_idle_time;
    }

    /// get the network management vector length of the cluster
    #[getter]
    pub(crate) fn network_management_vector_length(&self) -> u8 {
        self.0.network_management_vector_length
    }

    /// set the network management vector length of the cluster
    #[setter]
    pub(crate) fn set_network_management_vector_length(
        &mut self,
        network_management_vector_length: u8,
    ) {
        self.0.network_management_vector_length = network_management_vector_length;
    }

    /// get the number of minislots of the cluster
    #[getter]
    pub(crate) fn number_of_minislots(&self) -> u16 {
        self.0.number_of_minislots
    }

    /// set the number of minislots of the cluster
    #[setter]
    pub(crate) fn set_number_of_minislots(&mut self, number_of_minislots: u16) {
        self.0.number_of_minislots = number_of_minislots;
    }

    /// get the number of static slots of the cluster
    #[getter]
    pub(crate) fn number_of_static_slots(&self) -> u16 {
        self.0.number_of_static_slots
    }

    /// set the number of static slots of the cluster
    #[setter]
    pub(crate) fn set_number_of_static_slots(&mut self, number_of_static_slots: u16) {
        self.0.number_of_static_slots = number_of_static_slots;
    }

    /// get the offset correction start of the cluster
    #[getter]
    pub(crate) fn offset_correction_start(&self) -> u16 {
        self.0.offset_correction_start
    }

    /// set the offset correction start of the cluster
    #[setter]
    pub(crate) fn set_offset_correction_start(&mut self, offset_correction_start: u16) {
        self.0.offset_correction_start = offset_correction_start;
    }

    /// get the payload length static of the cluster
    #[getter]
    pub(crate) fn payload_length_static(&self) -> u16 {
        self.0.payload_length_static
    }

    /// set the payload length static of the cluster
    #[setter]
    pub(crate) fn set_payload_length_static(&mut self, payload_length_static: u16) {
        self.0.payload_length_static = payload_length_static;
    }

    /// get the safety margin of the cluster
    #[getter]
    pub(crate) fn safety_margin(&self) -> u16 {
        self.0.safety_margin
    }

    /// set the safety margin of the cluster
    #[setter]
    pub(crate) fn set_safety_margin(&mut self, safety_margin: u16) {
        self.0.safety_margin = safety_margin;
    }

    /// get the sample clock period of the cluster
    #[getter]
    pub(crate) fn sample_clock_period(&self) -> Option<f64> {
        self.0.sample_clock_period
    }

    /// set the sample clock period of the cluster
    #[setter]
    pub(crate) fn set_sample_clock_period(&mut self, sample_clock_period: Option<f64>) {
        self.0.sample_clock_period = sample_clock_period;
    }

    /// get the static slot duration of the cluster
    #[getter]
    pub(crate) fn static_slot_duration(&self) -> u16 {
        self.0.static_slot_duration
    }

    /// set the static slot duration of the cluster
    #[setter]
    pub(crate) fn set_static_slot_duration(&mut self, static_slot_duration: u16) {
        self.0.static_slot_duration = static_slot_duration;
    }

    /// get the symbol window of the cluster
    #[getter]
    pub(crate) fn symbol_window(&self) -> u8 {
        self.0.symbol_window
    }

    /// set the symbol window of the cluster
    #[setter]
    pub(crate) fn set_symbol_window(&mut self, symbol_window: u8) {
        self.0.symbol_window = symbol_window;
    }

    /// get the symbol window action point offset of the cluster
    #[getter]
    pub(crate) fn symbol_window_action_point_offset(&self) -> Option<u8> {
        self.0.symbol_window_action_point_offset
    }

    /// set the symbol window action point offset of the cluster
    #[setter]
    pub(crate) fn set_symbol_window_action_point_offset(
        &mut self,
        symbol_window_action_point_offset: Option<u8>,
    ) {
        self.0.symbol_window_action_point_offset = symbol_window_action_point_offset;
    }

    /// get the sync frame id count max of the cluster
    #[getter]
    pub(crate) fn sync_frame_id_count_max(&self) -> u8 {
        self.0.sync_frame_id_count_max
    }

    /// set the sync frame id count max of the cluster
    #[setter]
    pub(crate) fn set_sync_frame_id_count_max(&mut self, sync_frame_id_count_max: u8) {
        self.0.sync_frame_id_count_max = sync_frame_id_count_max;
    }

    /// get the transceiver standby delay of the cluster
    #[getter]
    pub(crate) fn transceiver_standby_delay(&self) -> Option<f64> {
        self.0.transceiver_standby_delay
    }

    /// set the transceiver standby delay of the cluster
    #[setter]
    pub(crate) fn set_transceiver_standby_delay(&mut self, transceiver_standby_delay: Option<f64>) {
        self.0.transceiver_standby_delay = transceiver_standby_delay;
    }

    /// get the transmission start sequence duration of the cluster
    #[getter]
    pub(crate) fn transmission_start_sequence_duration(&self) -> u8 {
        self.0.transmission_start_sequence_duration
    }

    /// set the transmission start sequence duration of the cluster
    #[setter]
    pub(crate) fn set_transmission_start_sequence_duration(
        &mut self,
        transmission_start_sequence_duration: u8,
    ) {
        self.0.transmission_start_sequence_duration = transmission_start_sequence_duration;
    }

    /// get the wakeup rx idle of the cluster
    #[getter]
    pub(crate) fn wakeup_rx_idle(&self) -> u16 {
        self.0.wakeup_rx_idle
    }

    /// set the wakeup rx idle of the cluster
    #[setter]
    pub(crate) fn set_wakeup_rx_idle(&mut self, wakeup_rx_idle: u16) {
        self.0.wakeup_rx_idle = wakeup_rx_idle;
    }

    /// get the wakeup rx low of the cluster
    #[getter]
    pub(crate) fn wakeup_rx_low(&self) -> u8 {
        self.0.wakeup_rx_low
    }

    /// set the wakeup rx low of the cluster
    #[setter]
    pub(crate) fn set_wakeup_rx_low(&mut self, wakeup_rx_low: u8) {
        self.0.wakeup_rx_low = wakeup_rx_low;
    }

    /// get the wakeup rx window of the cluster
    #[getter]
    pub(crate) fn wakeup_rx_window(&self) -> u16 {
        self.0.wakeup_rx_window
    }

    /// set the wakeup rx window of the cluster
    #[setter]
    pub(crate) fn set_wakeup_rx_window(&mut self, wakeup_rx_window: u16) {
        self.0.wakeup_rx_window = wakeup_rx_window;
    }

    /// get the wakeup tx active of the cluster
    #[getter]
    pub(crate) fn wakeup_tx_active(&self) -> u16 {
        self.0.wakeup_tx_active
    }

    /// set the wakeup tx active of the cluster
    #[setter]
    pub(crate) fn set_wakeup_tx_active(&mut self, wakeup_tx_active: u16) {
        self.0.wakeup_tx_active = wakeup_tx_active;
    }

    /// get the wakeup tx idle of the cluster
    #[getter]
    pub(crate) fn wakeup_tx_idle(&self) -> u16 {
        self.0.wakeup_tx_idle
    }

    /// set the wakeup tx idle of the cluster
    #[setter]
    pub(crate) fn set_wakeup_tx_idle(&mut self, wakeup_tx_idle: u16) {
        self.0.wakeup_tx_idle = wakeup_tx_idle;
    }
}
