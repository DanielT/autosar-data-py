use crate::{
    abstraction::{AutosarAbstractionError, Element, abstraction_err_to_pyerr},
    iterator_wrapper,
};
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};
use pyo3::prelude::*;

//##################################################################

/// A `ModeDeclarationGroup` is a collection of mode declarations.
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct ModeDeclarationGroup(
    pub(crate) autosar_data_abstraction::software_component::ModeDeclarationGroup,
);

#[pymethods]
impl ModeDeclarationGroup {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::ModeDeclarationGroup::try_from(
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

    /// set the category of the mode declaration group
    #[setter]
    fn set_category(&self, category: Option<ModeDeclarationGroupCategory>) -> PyResult<()> {
        self.0
            .set_category(category.map(Into::into))
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the category of the mode declaration group
    #[getter]
    fn category(&self) -> Option<ModeDeclarationGroupCategory> {
        self.0.category().map(Into::into)
    }

    /// Create a new mode declaration in the mode declaration group
    fn create_mode_declaration(&self, name: &str) -> PyResult<ModeDeclaration> {
        match self.0.create_mode_declaration(name) {
            Ok(value) => Ok(ModeDeclaration(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// Iterate over all mode declarations in the mode declaration group
    fn mode_declarations(&self) -> ModeDeclarationIterator {
        ModeDeclarationIterator::new(self.0.mode_declarations().map(ModeDeclaration))
    }

    /// Set the initial mode of the mode declaration group
    ///
    /// The initial mode is active before any mode is set.
    /// This setting is required to be present and the referenced mode must be part of the mode declaration group.
    #[setter]
    fn set_initial_mode(&self, mode: &ModeDeclaration) -> PyResult<()> {
        self.0
            .set_initial_mode(&mode.0)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the initial mode of the mode declaration group
    #[getter]
    fn initial_mode(&self) -> Option<ModeDeclaration> {
        self.0.initial_mode().map(ModeDeclaration)
    }

    /// set the onTransitionValue attribute of the mode declaration group
    #[setter]
    fn set_on_transition_value(&self, value: Option<u64>) -> PyResult<()> {
        self.0
            .set_on_transition_value(value)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the onTransitionValue attribute of the mode declaration group
    #[getter]
    fn on_transition_value(&self) -> Option<u64> {
        self.0.on_transition_value()
    }
}

//##################################################################

/// Category of mode declaration groupy, which defines the ordering of the modes in the group
#[pyclass(
    frozen,
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModeDeclarationGroupCategory {
    /// Ordering of the modes in the mode declaration group is alphabetic, and the modes may not set a value
    AlphabeticOrder,
    /// Ordering of modes in the mode declaration group is made explicit by the value, which must be set for each mode.
    /// Additonally, the on_transition_value attribute must be set in this case.
    ExplicitOrder,
}

impl From<autosar_data_abstraction::software_component::ModeDeclarationGroupCategory>
    for ModeDeclarationGroupCategory
{
    fn from(
        value: autosar_data_abstraction::software_component::ModeDeclarationGroupCategory,
    ) -> Self {
        match value {
            autosar_data_abstraction::software_component::ModeDeclarationGroupCategory::AlphabeticOrder => {
                ModeDeclarationGroupCategory::AlphabeticOrder
            }
            autosar_data_abstraction::software_component::ModeDeclarationGroupCategory::ExplicitOrder => {
                ModeDeclarationGroupCategory::ExplicitOrder
            }
        }
    }
}

impl From<ModeDeclarationGroupCategory>
    for autosar_data_abstraction::software_component::ModeDeclarationGroupCategory
{
    fn from(value: ModeDeclarationGroupCategory) -> Self {
        match value {
            ModeDeclarationGroupCategory::AlphabeticOrder => {
                autosar_data_abstraction::software_component::ModeDeclarationGroupCategory::AlphabeticOrder
            }
            ModeDeclarationGroupCategory::ExplicitOrder => {
                autosar_data_abstraction::software_component::ModeDeclarationGroupCategory::ExplicitOrder
            }
        }
    }
}

//##################################################################

/// A `ModeDeclaration` represents a mode declaration in a `ModeDeclarationGroup`
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._software_component"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct ModeDeclaration(
    pub(crate) autosar_data_abstraction::software_component::ModeDeclaration,
);

#[pymethods]
impl ModeDeclaration {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::software_component::ModeDeclaration::try_from(
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

    /// set the value of the mode declaration
    #[setter]
    fn set_value(&self, value: Option<u64>) -> PyResult<()> {
        self.0.set_value(value).map_err(abstraction_err_to_pyerr)
    }

    /// get the value of the mode declaration
    #[getter]
    fn value(&self) -> Option<u64> {
        self.0.value()
    }
}

//##################################################################

iterator_wrapper!(ModeDeclarationIterator, ModeDeclaration);
