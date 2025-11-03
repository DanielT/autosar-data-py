use crate::abstraction::AutosarAbstractionError;
use crate::pyutils::compare_pylist;
use crate::{abstraction::*, *};
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};

//##################################################################

/// A `CompuMethod` describes the conversion between physical and internal values
///
/// Use [`ArPackage::create_compu_method`] to create a new `CompuMethod`
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._datatype"
)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CompuMethod(pub(crate) autosar_data_abstraction::datatype::CompuMethod);

#[pymethods]
impl CompuMethod {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::datatype::CompuMethod::try_from(element.0.clone()) {
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

    /// Get the category of the `CompuMethod`
    #[getter]
    fn category(&self) -> Option<CompuMethodCategory> {
        self.0.category().map(std::convert::Into::into)
    }

    /// Apply `CompumethodContent` to the `CompuMethod`
    ///
    /// This will remove any existing content
    #[pyo3(signature = (content, /))]
    #[pyo3(text_signature = "(self, content: CompuMethodContent, /)")]
    fn set_content(&self, content: &Bound<'_, PyAny>) -> PyResult<()> {
        let content = pyany_to_compu_method_content(content)?;
        self.0
            .set_content(content)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the content of the `CompuMethod`
    fn content(&self) -> Option<Py<PyAny>> {
        self.0
            .content()
            .map(|cmc| compu_method_content_to_pyany(&cmc))
    }

    /// create a `CompuScale` in the `CompuMethod`
    #[pyo3(signature = (direction, /, *, lower_limit=None, upper_limit=None))]
    #[pyo3(
        text_signature = "(self, direction: CompuScaleDirection, /, *, lower_limit: Optional[float] = None, upper_limit: Optional[float] = None)"
    )]
    fn create_compu_scale(
        &self,
        direction: CompuScaleDirection,
        lower_limit: Option<f64>,
        upper_limit: Option<f64>,
    ) -> PyResult<CompuScale> {
        match self
            .0
            .create_compu_scale(direction.into(), lower_limit, upper_limit)
        {
            Ok(value) => Ok(CompuScale(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    /// Create an iterator over the internal-to-physical `CompuScales`
    fn int_to_phys_compu_scales(&self) -> IntToPhysCompuScaleIterator {
        IntToPhysCompuScaleIterator::new(self.0.int_to_phys_compu_scales().map(CompuScale))
    }

    /// Create an iterator over the physical-to-internal `CompuScales`
    fn phys_to_int_compu_scales(&self) -> PhysToIntCompuScaleIterator {
        PhysToIntCompuScaleIterator::new(self.0.phys_to_int_compu_scales().map(CompuScale))
    }
}

//#########################################################

iterator_wrapper!(IntToPhysCompuScaleIterator, CompuScale);
iterator_wrapper!(PhysToIntCompuScaleIterator, CompuScale);

//#########################################################

/// Category of a `CompuMethod`
#[pyclass(
    frozen,
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._datatype"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CompuMethodCategory {
    /// Identical conversion: internal and physical values are the same
    Identical,
    /// Linear conversion: `y = offset + factor * x / divisor`
    Linear,
    /// Linear conversion with multiple scales, each with its own limits
    ScaleLinear,
    /// Rational function conversion: `y = (n0 + n1 * x + n2 * x^2 + ...) / (d0 + d1 * x + d2 * x^2 + ...)`
    Rational,
    /// Rational function conversion with multiple scales, each with its own limits
    ScaleRational,
    /// Text table conversion
    TextTable,
    /// Bitfield text table conversion
    BitfieldTextTable,
    /// Linear conversion with multiple scales and a text table
    ScaleLinearAndTextTable,
    /// Rational function conversion with multiple scales and a text table
    ScaleRationalAndTextTable,
    /// Value table with no interpretation
    TabNoInterpretation,
}

impl From<autosar_data_abstraction::datatype::CompuMethodCategory> for CompuMethodCategory {
    fn from(value: autosar_data_abstraction::datatype::CompuMethodCategory) -> Self {
        match value {
            autosar_data_abstraction::datatype::CompuMethodCategory::Identical => Self::Identical,
            autosar_data_abstraction::datatype::CompuMethodCategory::Linear => Self::Linear,
            autosar_data_abstraction::datatype::CompuMethodCategory::ScaleLinear => {
                Self::ScaleLinear
            }
            autosar_data_abstraction::datatype::CompuMethodCategory::Rational => Self::Rational,
            autosar_data_abstraction::datatype::CompuMethodCategory::ScaleRational => {
                Self::ScaleRational
            }
            autosar_data_abstraction::datatype::CompuMethodCategory::TextTable => Self::TextTable,
            autosar_data_abstraction::datatype::CompuMethodCategory::BitfieldTextTable => {
                Self::BitfieldTextTable
            }
            autosar_data_abstraction::datatype::CompuMethodCategory::ScaleLinearAndTextTable => {
                Self::ScaleLinearAndTextTable
            }
            autosar_data_abstraction::datatype::CompuMethodCategory::ScaleRationalAndTextTable => {
                Self::ScaleRationalAndTextTable
            }
            autosar_data_abstraction::datatype::CompuMethodCategory::TabNoInterpretation => {
                Self::TabNoInterpretation
            }
        }
    }
}

//#########################################################

/// A `CompuScale` describes the conversion between physical and internal values, as well as the limits of the scale
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._datatype"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct CompuScale(autosar_data_abstraction::datatype::CompuScale);

#[pymethods]
impl CompuScale {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::datatype::CompuScale::try_from(element.0.clone()) {
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

    /// get the lower limit of the `CompuScale`
    #[getter]
    fn lower_limit(&self) -> Option<f64> {
        self.0.lower_limit()
    }

    /// get the upper limit of the `CompuScale`
    #[getter]
    fn upper_limit(&self) -> Option<f64> {
        self.0.upper_limit()
    }

    /// Set the mask of the `CompuScale`, applicable for `BitfieldTextTable`
    #[setter]
    fn set_mask(&self, mask: u64) -> PyResult<()> {
        self.0.set_mask(mask).map_err(abstraction_err_to_pyerr)
    }

    /// Get the mask of the `CompuScale`, applicable for `BitfieldTextTable`
    #[getter]
    fn mask(&self) -> Option<u64> {
        self.0.mask()
    }

    /// Set the content of the `CompuScale`
    #[setter]
    fn set_content(&self, content: &Bound<'_, PyAny>) -> PyResult<()> {
        let content = if let Ok(content) = content.extract::<String>() {
            autosar_data_abstraction::datatype::CompuScaleContent::TextConstant(content)
        } else if let Ok(content) = content.extract::<f64>() {
            autosar_data_abstraction::datatype::CompuScaleContent::NumericConstant(content)
        } else if let Ok(content) = content.cast_exact::<CompuScaleRationalCoefficients>() {
            Python::attach(|py| {
                let content = content.borrow();
                autosar_data_abstraction::datatype::CompuScaleContent::RationalCoeffs {
                    numerator: content.numerator.extract(py).unwrap_or_default(),
                    denominator: content.denominator.extract(py).unwrap_or_default(),
                }
            })
        } else {
            return Err(AutosarAbstractionError::new_err(
                "Invalid content for CompuScale".to_string(),
            ));
        };

        self.0
            .set_content(content)
            .map_err(abstraction_err_to_pyerr)
    }

    /// Get the content of the `CompuScale`
    #[getter]
    fn content(&self, py: Python) -> Option<Py<PyAny>> {
        let content = self.0.content()?;

        match content {
            autosar_data_abstraction::datatype::CompuScaleContent::TextConstant(text) => {
                text.into_py_any(py).ok()
            }
            autosar_data_abstraction::datatype::CompuScaleContent::NumericConstant(value) => {
                value.into_py_any(py).ok()
            }
            autosar_data_abstraction::datatype::CompuScaleContent::RationalCoeffs {
                numerator,
                denominator,
            } => {
                let content = CompuScaleRationalCoefficients {
                    numerator: PyList::new(py, &numerator).unwrap().unbind(),
                    denominator: PyList::new(py, &denominator).unwrap().unbind(),
                };
                content.into_py_any(py).ok()
            }
        }
    }
}

//#########################################################

/// Direction of a `CompuScale`
#[pyclass(
    frozen,
    eq,
    eq_int,
    module = "autosar_data._autosar_data._abstraction._datatype"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CompuScaleDirection {
    /// Internal to physical conversion
    IntToPhys,
    /// Physical to internal conversion
    PhysToInt,
}

impl From<CompuScaleDirection> for autosar_data_abstraction::datatype::CompuScaleDirection {
    fn from(value: CompuScaleDirection) -> Self {
        match value {
            CompuScaleDirection::IntToPhys => Self::IntToPhys,
            CompuScaleDirection::PhysToInt => Self::PhysToInt,
        }
    }
}

impl From<autosar_data_abstraction::datatype::CompuScaleDirection> for CompuScaleDirection {
    fn from(value: autosar_data_abstraction::datatype::CompuScaleDirection) -> Self {
        match value {
            autosar_data_abstraction::datatype::CompuScaleDirection::IntToPhys => Self::IntToPhys,
            autosar_data_abstraction::datatype::CompuScaleDirection::PhysToInt => Self::PhysToInt,
        }
    }
}

//#########################################################

/// Rational coefficients of a CompuScale
#[pyclass(
    eq,
    module = "autosar_data._autosar_data._abstraction._datatype",
    get_all,
    set_all
)]
#[derive(Debug)]
pub(crate) struct CompuScaleRationalCoefficients {
    /// list of numerator coefficients
    numerator: Py<PyList>, // = Vec<f64>
    /// list of denominator coefficients
    denominator: Py<PyList>, // = Vec<f64>
}

#[pymethods]
impl CompuScaleRationalCoefficients {
    #[pyo3(signature = (*, numerator, denominator))]
    #[pyo3(text_signature = "(self, *, numerator: List[float], denominator: List[float])")]
    #[new]
    fn new(numerator: Vec<f64>, denominator: Vec<f64>) -> Self {
        Python::attach(|py| Self {
            numerator: PyList::new(py, numerator).unwrap().unbind(),
            denominator: PyList::new(py, denominator).unwrap().unbind(),
        })
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}

impl PartialEq for CompuScaleRationalCoefficients {
    fn eq(&self, other: &Self) -> bool {
        Python::attach(|py| {
            let self_numerator = self.numerator.extract::<Vec<f64>>(py).unwrap_or_default();
            let other_numerator = other.numerator.extract::<Vec<f64>>(py).unwrap_or_default();
            let self_denominator = self.denominator.extract::<Vec<f64>>(py).unwrap_or_default();
            let other_denominator = other
                .denominator
                .extract::<Vec<f64>>(py)
                .unwrap_or_default();
            self_numerator == other_numerator && self_denominator == other_denominator
        })
    }
}

//#########################################################

/// Content of a `CompuMethod`
#[pyclass(
    frozen,
    module = "autosar_data._autosar_data._abstraction._datatype",
    subclass
)]
pub(crate) struct CompuMethodContent();

//#########################################################

/// Identical conversion: internal and physical values are the same
#[allow(non_camel_case_types)]
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._datatype",
    extends = CompuMethodContent
)]
#[derive(PartialEq)]
pub(crate) struct CompuMethodContent_Identical();

#[pymethods]
impl CompuMethodContent_Identical {
    #[new]
    #[pyo3(text_signature = "(self, /)")]
    fn new(py: Python) -> PyResult<Py<CompuMethodContent_Identical>> {
        Py::new(py, (CompuMethodContent_Identical(), CompuMethodContent()))
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}

impl std::fmt::Debug for CompuMethodContent_Identical {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CompuMethodContent.Identical()")
    }
}

//#########################################################

#[allow(non_camel_case_types)]
#[pyclass(
    get_all, set_all, eq,
    module = "autosar_data._autosar_data._abstraction._datatype",
    extends = CompuMethodContent
)]
#[derive(PartialEq)]
pub(crate) struct CompuMethodContent_Linear {
    /// direction of the conversion
    direction: CompuScaleDirection,
    /// offset
    offset: f64,
    /// factor
    factor: f64,
    /// divisor
    divisor: f64,
    /// optional: lower limit of the scale
    lower_limit: Option<f64>,
    /// optional: upper limit of the scale
    upper_limit: Option<f64>,
}

#[pymethods]
impl CompuMethodContent_Linear {
    #[new]
    #[pyo3(signature = (*, direction, offset, factor, divisor, lower_limit=None, upper_limit=None))]
    #[pyo3(
        text_signature = "(self, *, direction: CompuScaleDirection, offset: float, factor: float, divisor: float, lower_limit: Optional[float] = None, upper_limit: Optional[float] = None)"
    )]
    fn new(
        py: Python,
        direction: CompuScaleDirection,
        offset: f64,
        factor: f64,
        divisor: f64,
        lower_limit: Option<f64>,
        upper_limit: Option<f64>,
    ) -> PyResult<Py<CompuMethodContent_Linear>> {
        Py::new(
            py,
            (
                CompuMethodContent_Linear {
                    direction,
                    offset,
                    factor,
                    divisor,
                    lower_limit,
                    upper_limit,
                },
                CompuMethodContent(),
            ),
        )
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}

impl std::fmt::Debug for CompuMethodContent_Linear {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CompuMethodContent.Linear( direction: {:?}, offset: {}, factor: {}, divisor: {}, lower_limit: {:?}, upper_limit: {:?})",
            self.direction,
            self.offset,
            self.factor,
            self.divisor,
            self.lower_limit,
            self.upper_limit
        )
    }
}

//#########################################################

/// Linear conversion with multiple scales, each with its own limits
#[allow(non_camel_case_types)]
#[pyclass(
        get_all, set_all, eq,
        module = "autosar_data._autosar_data._abstraction._datatype",
        extends = CompuMethodContent
    )]
pub(crate) struct CompuMethodContent_ScaleLinear {
    scales: Py<PyList>, // = Vec<LinearConversionParameters>
}

#[pymethods]
impl CompuMethodContent_ScaleLinear {
    #[new]
    #[pyo3(signature = (*, scales))]
    #[pyo3(text_signature = "(self, *, scales: List[LinearConversionParameters])")]
    fn new(py: Python, scales: Py<PyList>) -> PyResult<Py<CompuMethodContent_ScaleLinear>> {
        Py::new(
            py,
            (
                CompuMethodContent_ScaleLinear { scales },
                CompuMethodContent(),
            ),
        )
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}

impl PartialEq for CompuMethodContent_ScaleLinear {
    fn eq(&self, other: &Self) -> bool {
        Python::attach(|py| compare_pylist(py, &self.scales, &other.scales))
    }
}

impl std::fmt::Debug for CompuMethodContent_ScaleLinear {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Python::attach(|py| {
            let scales = self.scales.bind_borrowed(py);
            write!(f, "CompuMethodContent.ScaleLinear( scales: {scales:?} )")
        })
    }
}

//#########################################################

#[allow(non_camel_case_types)]
#[pyclass(
    get_all, set_all, eq,
    module = "autosar_data._autosar_data._abstraction._datatype",
    extends = CompuMethodContent
)]
#[derive(PartialEq)]
pub(crate) struct CompuMethodContent_Rational {
    /// direction of the conversion
    direction: CompuScaleDirection,
    /// list of numerator coefficients
    denominator: Vec<f64>,
    /// list of denominator coefficients
    numerator: Vec<f64>,
    /// lower limit of the scale
    lower_limit: f64,
    /// upper limit of the scale
    upper_limit: f64,
}

#[pymethods]
impl CompuMethodContent_Rational {
    #[new]
    #[pyo3(signature = (*, direction, denominator, numerator, lower_limit, upper_limit))]
    #[pyo3(
        text_signature = "(self, *, direction: CompuScaleDirection, denominator: List[float], numerator: List[float], lower_limit: float, upper_limit: float)"
    )]
    fn new(
        py: Python,
        direction: CompuScaleDirection,
        denominator: Vec<f64>,
        numerator: Vec<f64>,
        lower_limit: f64,
        upper_limit: f64,
    ) -> PyResult<Py<CompuMethodContent_Rational>> {
        Py::new(
            py,
            (
                CompuMethodContent_Rational {
                    direction,
                    denominator,
                    numerator,
                    lower_limit,
                    upper_limit,
                },
                CompuMethodContent(),
            ),
        )
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}

impl std::fmt::Debug for CompuMethodContent_Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CompuMethodContent.Rational( direction: {:?}, denominator: {:?}, numerator: {:?}, lower_limit: {}, upper_limit: {})",
            self.direction, self.denominator, self.numerator, self.lower_limit, self.upper_limit
        )
    }
}

//#########################################################

/// Rational function conversion with multiple scales, each with its own limits
#[allow(non_camel_case_types)]
#[pyclass(
    get_all, set_all, eq,
    module = "autosar_data._autosar_data._abstraction._datatype",
    extends = CompuMethodContent
)]
pub(crate) struct CompuMethodContent_ScaleRational {
    scales: Py<PyList>, // = Vec<CompuMethodRationalContent>
}

#[pymethods]
impl CompuMethodContent_ScaleRational {
    #[new]
    #[pyo3(signature = (*, scales))]
    #[pyo3(text_signature = "(self, *, scales: List[RationalConversionParameters])")]
    fn new(py: Python, scales: Py<PyList>) -> PyResult<Py<CompuMethodContent_ScaleRational>> {
        Py::new(
            py,
            (
                CompuMethodContent_ScaleRational { scales },
                CompuMethodContent(),
            ),
        )
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}

impl PartialEq for CompuMethodContent_ScaleRational {
    fn eq(&self, other: &Self) -> bool {
        Python::attach(|py| compare_pylist(py, &self.scales, &other.scales))
    }
}

impl std::fmt::Debug for CompuMethodContent_ScaleRational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Python::attach(|py| {
            let scales = self.scales.bind_borrowed(py);
            write!(f, "CompuMethodContent.ScaleRational( scales: {scales:?} )")
        })
    }
}

//#########################################################

/// Text table conversion
#[allow(non_camel_case_types)]
#[pyclass(
    get_all, set_all, eq,
    module = "autosar_data._autosar_data._abstraction._datatype",
    extends = CompuMethodContent
)]
pub(crate) struct CompuMethodContent_TextTable {
    /// list of text table entries
    texts: Py<PyList>, // = Vec<TextTableEntry>,
}

#[pymethods]
impl CompuMethodContent_TextTable {
    #[new]
    #[pyo3(signature = (*, texts))]
    #[pyo3(text_signature = "(self, *, texts: List[TextTableEntry])")]
    fn new(py: Python, texts: Py<PyList>) -> PyResult<Py<CompuMethodContent_TextTable>> {
        Py::new(
            py,
            (CompuMethodContent_TextTable { texts }, CompuMethodContent()),
        )
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}

impl PartialEq for CompuMethodContent_TextTable {
    fn eq(&self, other: &Self) -> bool {
        Python::attach(|py| compare_pylist(py, &self.texts, &other.texts))
    }
}

impl std::fmt::Debug for CompuMethodContent_TextTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Python::attach(|py| {
            let texts = self.texts.bind_borrowed(py);
            write!(f, "CompuMethodContent.TextTable( texts: {texts:?} )")
        })
    }
}

//#########################################################

/// Bitfield text table conversion
#[allow(non_camel_case_types)]
#[pyclass(
    get_all, set_all, eq,
    module = "autosar_data._autosar_data._abstraction._datatype",
    extends = CompuMethodContent
)]
pub(crate) struct CompuMethodContent_BitfieldTextTable {
    /// list of bitfield description etnries in the table
    entries: Py<PyList>, // = Vec<BitfieldEntry>
}

#[pymethods]
impl CompuMethodContent_BitfieldTextTable {
    #[new]
    #[pyo3(signature = (*, entries))]
    #[pyo3(text_signature = "(self, *, entries: List[BitfieldEntry])")]
    fn new(py: Python, entries: Py<PyList>) -> PyResult<Py<CompuMethodContent_BitfieldTextTable>> {
        Py::new(
            py,
            (
                CompuMethodContent_BitfieldTextTable { entries },
                CompuMethodContent(),
            ),
        )
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}

impl PartialEq for CompuMethodContent_BitfieldTextTable {
    fn eq(&self, other: &Self) -> bool {
        Python::attach(|py| compare_pylist(py, &self.entries, &other.entries))
    }
}

impl std::fmt::Debug for CompuMethodContent_BitfieldTextTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Python::attach(|py| {
            let entries = self.entries.bind_borrowed(py);
            write!(
                f,
                "CompuMethodContent.BitfieldTextTable( entries: {entries:?} )"
            )
        })
    }
}

//#########################################################

/// Linear conversion with multiple scales and a text table
#[allow(non_camel_case_types)]
#[pyclass(
    get_all, set_all, eq,
    module = "autosar_data._autosar_data._abstraction._datatype",
    extends = CompuMethodContent
)]
pub(crate) struct CompuMethodContent_ScaleLinearAndTextTable {
    /// list of linear conversion parameters
    scales: Py<PyList>, // = Vec<LinearConversionParameters>
    /// list of text table entries
    texts: Py<PyList>, // = Vec<TextTableEntry>
}

#[pymethods]
impl CompuMethodContent_ScaleLinearAndTextTable {
    #[new]
    #[pyo3(signature = (*, scales, texts))]
    #[pyo3(
        text_signature = "(self, *, scales: List[LinearConversionParameters], texts: List[TextTableEntry])"
    )]
    fn new(
        py: Python,
        scales: Py<PyList>,
        texts: Py<PyList>,
    ) -> PyResult<Py<CompuMethodContent_ScaleLinearAndTextTable>> {
        Py::new(
            py,
            (
                CompuMethodContent_ScaleLinearAndTextTable { scales, texts },
                CompuMethodContent(),
            ),
        )
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}

impl PartialEq for CompuMethodContent_ScaleLinearAndTextTable {
    fn eq(&self, other: &Self) -> bool {
        Python::attach(|py| {
            compare_pylist(py, &self.scales, &other.scales)
                && compare_pylist(py, &self.texts, &other.texts)
        })
    }
}

impl std::fmt::Debug for CompuMethodContent_ScaleLinearAndTextTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Python::attach(|py| {
            let scales = self.scales.bind_borrowed(py);
            let texts = self.texts.bind_borrowed(py);
            write!(
                f,
                "CompuMethodContent.ScaleLinearAndTextTable( scales: {scales:?}, texts: {texts:?} )"
            )
        })
    }
}

//#########################################################

/// Rational function conversion with multiple scales and a text table
#[allow(non_camel_case_types)]
#[pyclass(
    get_all, set_all, eq,
    module = "autosar_data._autosar_data._abstraction._datatype",
    extends = CompuMethodContent
)]
pub(crate) struct CompuMethodContent_ScaleRationalAndTextTable {
    /// list of rational conversion parameters
    scales: Py<PyList>, // = Vec<RationalConversionParameters>
    /// list of text table entries
    texts: Py<PyList>, // = Vec<TextTableEntry>
}

#[pymethods]
impl CompuMethodContent_ScaleRationalAndTextTable {
    #[new]
    #[pyo3(signature = (*, scales, texts))]
    #[pyo3(
        text_signature = "(self, *, scales: List[RationalConversionParameters], texts: List[TextTableEntry])"
    )]
    fn new(
        py: Python,
        scales: Py<PyList>,
        texts: Py<PyList>,
    ) -> PyResult<Py<CompuMethodContent_ScaleRationalAndTextTable>> {
        Py::new(
            py,
            (
                CompuMethodContent_ScaleRationalAndTextTable { scales, texts },
                CompuMethodContent(),
            ),
        )
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}

impl PartialEq for CompuMethodContent_ScaleRationalAndTextTable {
    fn eq(&self, other: &Self) -> bool {
        Python::attach(|py| {
            compare_pylist(py, &self.scales, &other.scales)
                && compare_pylist(py, &self.texts, &other.texts)
        })
    }
}

impl std::fmt::Debug for CompuMethodContent_ScaleRationalAndTextTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Python::attach(|py| {
            let scales = self.scales.bind_borrowed(py);
            let texts = self.texts.bind_borrowed(py);
            write!(
                f,
                "CompuMethodContent.ScaleRationalAndTextTable( scales: {scales:?}, texts: {texts:?} )"
            )
        })
    }
}

//#########################################################

/// Value table with no interpretation
#[allow(non_camel_case_types)]
#[pyclass(
    get_all, set_all, eq,
    module = "autosar_data._autosar_data._abstraction._datatype",
    extends = CompuMethodContent
)]
pub(crate) struct CompuMethodContent_TabNoInterpretation {
    /// list of table entries, each mapping one input value to one output value
    entries: Py<PyList>, // = Vec<TabNoIntpEntry>
}

#[pymethods]
impl CompuMethodContent_TabNoInterpretation {
    #[new]
    #[pyo3(signature = (*, entries))]
    #[pyo3(text_signature = "(self, *, entries: List[TabNoIntpEntry])")]
    fn new(
        py: Python,
        entries: Py<PyList>,
    ) -> PyResult<Py<CompuMethodContent_TabNoInterpretation>> {
        Py::new(
            py,
            (
                CompuMethodContent_TabNoInterpretation { entries },
                CompuMethodContent(),
            ),
        )
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}

impl PartialEq for CompuMethodContent_TabNoInterpretation {
    fn eq(&self, other: &Self) -> bool {
        Python::attach(|py| compare_pylist(py, &self.entries, &other.entries))
    }
}

impl std::fmt::Debug for CompuMethodContent_TabNoInterpretation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Python::attach(|py| {
            let entries = self.entries.bind_borrowed(py);
            write!(
                f,
                "CompuMethodContent.TabNoInterpretation( entries: {entries:?} )"
            )
        })
    }
}

//#########################################################

fn compu_method_content_to_pyany(
    content: &autosar_data_abstraction::datatype::CompuMethodContent,
) -> Py<PyAny> {
    Python::attach(|py| match content {
        autosar_data_abstraction::datatype::CompuMethodContent::Identical => {
            CompuMethodContent_Identical::new(py)
                .unwrap()
                .into_py_any(py)
                .unwrap()
        }
        autosar_data_abstraction::datatype::CompuMethodContent::Linear(content) => {
            CompuMethodContent_Linear::new(
                py,
                content.direction.into(),
                content.offset,
                content.factor,
                content.divisor,
                content.lower_limit,
                content.upper_limit,
            )
            .unwrap()
            .into_py_any(py)
            .unwrap()
        }
        autosar_data_abstraction::datatype::CompuMethodContent::ScaleLinear(scales) => {
            CompuMethodContent_ScaleLinear::new(py, linear_scales_to_pylist(py, scales))
                .unwrap()
                .into_py_any(py)
                .unwrap()
        }
        autosar_data_abstraction::datatype::CompuMethodContent::Rational(content) => {
            CompuMethodContent_Rational::new(
                py,
                content.direction.into(),
                content.denominator.clone(),
                content.numerator.clone(),
                content.lower_limit,
                content.upper_limit,
            )
            .unwrap()
            .into_py_any(py)
            .unwrap()
        }
        autosar_data_abstraction::datatype::CompuMethodContent::ScaleRational(scales) => {
            CompuMethodContent_ScaleRational::new(py, rational_scales_to_pylist(py, scales))
                .unwrap()
                .into_py_any(py)
                .unwrap()
        }
        autosar_data_abstraction::datatype::CompuMethodContent::TextTable(texts) => {
            CompuMethodContent_TextTable::new(py, text_table_to_pylist(py, texts))
                .unwrap()
                .into_py_any(py)
                .unwrap()
        }
        autosar_data_abstraction::datatype::CompuMethodContent::BitfieldTextTable(entries) => {
            CompuMethodContent_BitfieldTextTable::new(py, bitfield_to_pylist(py, entries))
                .unwrap()
                .into_py_any(py)
                .unwrap()
        }
        autosar_data_abstraction::datatype::CompuMethodContent::ScaleLinearAndTextTable(
            scales,
            texts,
        ) => {
            let scales = linear_scales_to_pylist(py, scales);
            let texts = text_table_to_pylist(py, texts);
            CompuMethodContent_ScaleLinearAndTextTable::new(py, scales, texts)
                .unwrap()
                .into_py_any(py)
                .unwrap()
        }
        autosar_data_abstraction::datatype::CompuMethodContent::ScaleRationalAndTextTable(
            scales,
            texts,
        ) => {
            let scales = rational_scales_to_pylist(py, scales);
            let texts = text_table_to_pylist(py, texts);
            CompuMethodContent_ScaleRationalAndTextTable::new(py, scales, texts)
                .unwrap()
                .into_py_any(py)
                .unwrap()
        }
        autosar_data_abstraction::datatype::CompuMethodContent::TabNoInterpretation(entries) => {
            CompuMethodContent_TabNoInterpretation::new(py, tab_no_intp_to_pylist(py, entries))
                .unwrap()
                .into_py_any(py)
                .unwrap()
        }
    })
}

fn linear_scales_to_pylist(
    py: Python<'_>,
    scales: &[autosar_data_abstraction::datatype::CompuMethodScaleLinearContent],
) -> Py<PyList> {
    let scales_vec: Vec<_> = scales
        .iter()
        .map(|scale| {
            LinearConversionParameters {
                direction: scale.direction.into(),
                offset: scale.offset,
                factor: scale.factor,
                divisor: scale.divisor,
                lower_limit: scale.lower_limit,
                upper_limit: scale.upper_limit,
            }
            .into_py_any(py)
            .unwrap()
        })
        .collect::<Vec<_>>();
    PyList::new(py, scales_vec).unwrap().unbind()
}

fn rational_scales_to_pylist(
    py: Python<'_>,
    scales: &[autosar_data_abstraction::datatype::CompuMethodRationalContent],
) -> Py<PyList> {
    let scales_vec = scales
        .iter()
        .map(|scale| {
            RationalConversionParameters {
                direction: scale.direction.into(),
                denominator: PyList::new(py, &scale.denominator).unwrap().unbind(),
                numerator: PyList::new(py, &scale.numerator).unwrap().unbind(),
                lower_limit: scale.lower_limit,
                upper_limit: scale.upper_limit,
            }
            .into_py_any(py)
            .unwrap()
        })
        .collect::<Vec<_>>();
    PyList::new(py, scales_vec).unwrap().unbind()
}

fn text_table_to_pylist(
    py: Python<'_>,
    entries: &[autosar_data_abstraction::datatype::CompuMethodTextTableContent],
) -> Py<PyList> {
    let entries_vec = entries
        .iter()
        .map(|entry| {
            TextTableEntry {
                value: entry.value,
                text: entry.text.clone(),
            }
            .into_py_any(py)
            .unwrap()
        })
        .collect::<Vec<_>>();
    PyList::new(py, entries_vec).unwrap().unbind()
}

fn tab_no_intp_to_pylist(
    py: Python<'_>,
    entries: &[autosar_data_abstraction::datatype::CompuMethodTabNoIntpContent],
) -> Py<PyList> {
    let entries_vec = entries
        .iter()
        .map(|entry| {
            TabNoIntpEntry {
                value_in: entry.value_in,
                value_out: entry.value_out,
            }
            .into_py_any(py)
            .unwrap()
        })
        .collect::<Vec<_>>();
    PyList::new(py, entries_vec).unwrap().unbind()
}

fn bitfield_to_pylist(
    py: Python<'_>,
    entries: &[autosar_data_abstraction::datatype::CompuMethodBitfieldTextTableContent],
) -> Py<PyList> {
    let entries_vec = entries
        .iter()
        .map(|entry| {
            BitfieldEntry {
                text: entry.text.clone(),
                value: entry.value,
                mask: entry.mask,
            }
            .into_py_any(py)
            .unwrap()
        })
        .collect::<Vec<_>>();
    PyList::new(py, entries_vec).unwrap().unbind()
}

pub(crate) fn pyany_to_compu_method_content(
    pyobject: &Bound<'_, PyAny>,
) -> PyResult<autosar_data_abstraction::datatype::CompuMethodContent> {
    use autosar_data_abstraction::datatype as dt;
    Python::attach(|py| {
        if pyobject
            .cast_exact::<CompuMethodContent_Identical>()
            .is_ok()
        {
            Ok(dt::CompuMethodContent::Identical)
        } else if let Ok(linear) = pyobject.cast_exact::<CompuMethodContent_Linear>() {
            let lb: PyRef<'_, CompuMethodContent_Linear> = linear.borrow();
            Ok(dt::CompuMethodContent::Linear(
                dt::CompuMethodLinearContent {
                    direction: lb.direction.into(),
                    offset: lb.offset,
                    factor: lb.factor,
                    divisor: lb.divisor,
                    lower_limit: lb.lower_limit,
                    upper_limit: lb.upper_limit,
                },
            ))
        } else if let Ok(scale_linear) = pyobject.cast_exact::<CompuMethodContent_ScaleLinear>() {
            let out_scales_vec = pylist_to_linear_scales(py, &scale_linear.borrow().scales);
            Ok(dt::CompuMethodContent::ScaleLinear(out_scales_vec))
        } else if let Ok(rational) = pyobject.cast_exact::<CompuMethodContent_Rational>() {
            let rb = rational.borrow();
            Ok(dt::CompuMethodContent::Rational(
                dt::CompuMethodRationalContent {
                    direction: rb.direction.into(),
                    denominator: rb.denominator.clone(),
                    numerator: rb.numerator.clone(),
                    lower_limit: rb.lower_limit,
                    upper_limit: rb.upper_limit,
                },
            ))
        } else if let Ok(scale_rational) = pyobject.cast_exact::<CompuMethodContent_ScaleRational>()
        {
            let scales = pylist_to_rational_scales(py, &scale_rational.borrow().scales);
            Ok(dt::CompuMethodContent::ScaleRational(scales))
        } else if let Ok(text_table) = pyobject.cast_exact::<CompuMethodContent_TextTable>() {
            let texts = pylist_to_text_table(py, &text_table.borrow().texts);
            Ok(dt::CompuMethodContent::TextTable(texts))
        } else if let Ok(bitfield) = pyobject.cast_exact::<CompuMethodContent_BitfieldTextTable>() {
            let entries = pylist_to_bitfield(py, &bitfield.borrow().entries);
            Ok(dt::CompuMethodContent::BitfieldTextTable(entries))
        } else if let Ok(scale_linear_text_table) =
            pyobject.cast_exact::<CompuMethodContent_ScaleLinearAndTextTable>()
        {
            let borrowed = scale_linear_text_table.borrow();
            let scales = pylist_to_linear_scales(py, &borrowed.scales);
            let texts = pylist_to_text_table(py, &borrowed.texts);
            Ok(dt::CompuMethodContent::ScaleLinearAndTextTable(
                scales, texts,
            ))
        } else if let Ok(scale_rational_text_table) =
            pyobject.cast_exact::<CompuMethodContent_ScaleRationalAndTextTable>()
        {
            let borrowed = scale_rational_text_table.borrow();
            let scales = pylist_to_rational_scales(py, &borrowed.scales);
            let texts = pylist_to_text_table(py, &borrowed.texts);
            Ok(dt::CompuMethodContent::ScaleRationalAndTextTable(
                scales, texts,
            ))
        } else if let Ok(tab_no_intp) =
            pyobject.cast_exact::<CompuMethodContent_TabNoInterpretation>()
        {
            let entries = pylist_to_tab_no_intp(py, &tab_no_intp.borrow().entries);
            Ok(dt::CompuMethodContent::TabNoInterpretation(entries))
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(format!(
                "'{:?}' cannot be converted to 'CompuMethodContent'",
                pyobject.get_type().name()
            )))
        }
    })
}

fn pylist_to_linear_scales(
    py: Python<'_>,
    scales: &Py<PyList>,
) -> Vec<autosar_data_abstraction::datatype::CompuMethodScaleLinearContent> {
    if let Ok(scales_iter) = scales.bind_borrowed(py).as_sequence().try_iter() {
        let mut out_scales_vec = vec![];
        for params in scales_iter
            .filter_map(Result::ok)
            .filter_map(|pyany| pyany.cast_into_exact::<LinearConversionParameters>().ok())
            .map(|py_lcp| py_lcp.borrow())
        {
            out_scales_vec.push(
                autosar_data_abstraction::datatype::CompuMethodScaleLinearContent {
                    direction: params.direction.into(),
                    offset: params.offset,
                    factor: params.factor,
                    divisor: params.divisor,
                    lower_limit: params.lower_limit,
                    upper_limit: params.upper_limit,
                },
            );
        }
        out_scales_vec
    } else {
        vec![]
    }
}

fn pylist_to_rational_scales(
    py: Python<'_>,
    scales: &Py<PyList>,
) -> Vec<autosar_data_abstraction::datatype::CompuMethodRationalContent> {
    if let Ok(scales_iter) = scales.bind_borrowed(py).as_sequence().try_iter() {
        let mut out_scales_vec = vec![];
        for params in scales_iter
            .filter_map(Result::ok)
            .filter_map(|pyany| pyany.cast_into_exact::<RationalConversionParameters>().ok())
            .map(|py_rcp| py_rcp.borrow())
        {
            out_scales_vec.push(
                autosar_data_abstraction::datatype::CompuMethodRationalContent {
                    direction: params.direction.into(),
                    denominator: params.denominator.extract(py).unwrap_or_default(),
                    numerator: params.numerator.extract(py).unwrap_or_default(),
                    lower_limit: params.lower_limit,
                    upper_limit: params.upper_limit,
                },
            );
        }
        out_scales_vec
    } else {
        vec![]
    }
}

fn pylist_to_text_table(
    py: Python<'_>,
    texts: &Py<PyList>,
) -> Vec<autosar_data_abstraction::datatype::CompuMethodTextTableContent> {
    if let Ok(texts_iter) = texts.bind_borrowed(py).as_sequence().try_iter() {
        let mut out_texts_vec = vec![];
        for ttentry in texts_iter
            .filter_map(Result::ok)
            .filter_map(|pyany| pyany.cast_into_exact::<TextTableEntry>().ok())
            .map(|py_tte| py_tte.borrow())
        {
            out_texts_vec.push(
                autosar_data_abstraction::datatype::CompuMethodTextTableContent {
                    value: ttentry.value,
                    text: ttentry.text.clone(),
                },
            );
        }
        out_texts_vec
    } else {
        vec![]
    }
}

fn pylist_to_bitfield(
    py: Python<'_>,
    texts: &Py<PyList>,
) -> Vec<autosar_data_abstraction::datatype::CompuMethodBitfieldTextTableContent> {
    if let Ok(texts_iter) = texts.bind_borrowed(py).as_sequence().try_iter() {
        let mut out_texts_vec = vec![];
        for bfentry in texts_iter
            .filter_map(Result::ok)
            .filter_map(|pyany| pyany.cast_into_exact::<BitfieldEntry>().ok())
            .map(|py_tte| py_tte.borrow())
        {
            out_texts_vec.push(
                autosar_data_abstraction::datatype::CompuMethodBitfieldTextTableContent {
                    text: bfentry.text.clone(),
                    value: bfentry.value,
                    mask: bfentry.mask,
                },
            );
        }
        out_texts_vec
    } else {
        vec![]
    }
}

fn pylist_to_tab_no_intp(
    py: Python<'_>,
    texts: &Py<PyList>,
) -> Vec<autosar_data_abstraction::datatype::CompuMethodTabNoIntpContent> {
    if let Ok(texts_iter) = texts.bind_borrowed(py).as_sequence().try_iter() {
        let mut out_texts_vec = vec![];
        for tnientry in texts_iter
            .filter_map(Result::ok)
            .filter_map(|pyany| pyany.cast_into_exact::<TabNoIntpEntry>().ok())
            .map(|py_tte| py_tte.borrow())
        {
            out_texts_vec.push(
                autosar_data_abstraction::datatype::CompuMethodTabNoIntpContent {
                    value_in: tnientry.value_in,
                    value_out: tnientry.value_out,
                },
            );
        }
        out_texts_vec
    } else {
        vec![]
    }
}

//##################################################################

/// Linear conversion parameters for CompuMethodScaleLinearContent and CompuMethodScaleLinearAndTextTable
#[pyclass(
    eq,
    module = "autosar_data._autosar_data._abstraction._datatype",
    get_all,
    set_all
)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct LinearConversionParameters {
    /// direction of the conversion
    direction: CompuScaleDirection,
    /// offset
    offset: f64,
    /// factor
    factor: f64,
    /// divisor
    divisor: f64,
    /// lower limit of the scale
    lower_limit: f64,
    /// upper limit of the scale
    upper_limit: f64,
}

#[pymethods]
impl LinearConversionParameters {
    #[pyo3(signature = (*, direction, offset, factor, divisor, lower_limit, upper_limit))]
    #[pyo3(
        text_signature = "(self, *, direction: CompuScaleDirection, offset: float, factor: float, divisor: float, lower_limit: float, upper_limit: float)"
    )]
    #[new]
    fn new(
        direction: CompuScaleDirection,
        offset: f64,
        factor: f64,
        divisor: f64,
        lower_limit: f64,
        upper_limit: f64,
    ) -> Self {
        Self {
            direction,
            offset,
            factor,
            divisor,
            lower_limit,
            upper_limit,
        }
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}

/// Description of the content of a `CompuMethod` whose category is `Rational`
#[pyclass(
    eq,
    module = "autosar_data._autosar_data._abstraction._datatype",
    get_all,
    set_all
)]
#[derive(Debug)]
pub(crate) struct RationalConversionParameters {
    /// direction of the conversion
    direction: CompuScaleDirection,
    /// list of numerator coefficients
    denominator: Py<PyList>,
    /// list of denominator coefficients
    numerator: Py<PyList>,
    /// lower limit of the scale
    lower_limit: f64,
    /// upper limit of the scale
    upper_limit: f64,
}

#[pymethods]
impl RationalConversionParameters {
    #[pyo3(signature = (*, direction, denominator, numerator, lower_limit, upper_limit))]
    #[pyo3(
        text_signature = "(self, *, direction: CompuScaleDirection, denominator: List[float], numerator: List[float], lower_limit: float, upper_limit: float)"
    )]
    #[new]
    fn new(
        direction: CompuScaleDirection,
        denominator: Py<PyList>,
        numerator: Py<PyList>,
        lower_limit: f64,
        upper_limit: f64,
    ) -> Self {
        Self {
            direction,
            denominator,
            numerator,
            lower_limit,
            upper_limit,
        }
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}

impl PartialEq for RationalConversionParameters {
    fn eq(&self, other: &Self) -> bool {
        Python::attach(|py| {
            self.direction == other.direction
                && self.lower_limit == other.lower_limit
                && self.upper_limit == other.upper_limit
                && self.numerator.extract::<Vec<f64>>(py).unwrap_or_default()
                    == other.numerator.extract::<Vec<f64>>(py).unwrap_or_default()
                && self.denominator.extract::<Vec<f64>>(py).unwrap_or_default()
                    == other
                        .denominator
                        .extract::<Vec<f64>>(py)
                        .unwrap_or_default()
        })
    }
}

//##################################################################

/// A single entry of a text table conversion
#[pyclass(
    eq,
    module = "autosar_data._autosar_data._abstraction._datatype",
    get_all,
    set_all
)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TextTableEntry {
    /// value
    value: f64,
    /// text
    text: String,
}

#[pymethods]
impl TextTableEntry {
    #[pyo3(signature = (*, value, text))]
    #[pyo3(text_signature = "(self, *, value: float, text: str)")]
    #[new]
    fn new(value: f64, text: &str) -> Self {
        Self {
            value,
            text: text.to_string(),
        }
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}

/// A single entry of a bitfield text table conversion
#[pyclass(
    eq,
    module = "autosar_data._autosar_data._abstraction._datatype",
    get_all,
    set_all
)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BitfieldEntry {
    /// text of this entry
    text: String,
    /// numeric value of this entry
    value: f64,
    /// bit mask of this entry
    mask: u64,
}

#[pymethods]
impl BitfieldEntry {
    #[pyo3(signature = (*, text, value, mask))]
    #[pyo3(text_signature = "(self, *, text: str, value: float, mask: int)")]
    #[new]
    fn new(text: &str, value: f64, mask: u64) -> Self {
        Self {
            text: text.to_string(),
            value,
            mask,
        }
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}

/// a single entry of a `CompuMethod` whose category is `TabNoInterpretation`
#[pyclass(
    eq,
    module = "autosar_data._autosar_data._abstraction._datatype",
    get_all,
    set_all
)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TabNoIntpEntry {
    /// input value
    value_in: f64,
    /// output value
    value_out: f64,
}

#[pymethods]
impl TabNoIntpEntry {
    #[pyo3(signature = (*, value_in, value_out))]
    #[pyo3(text_signature = "(self, *, value_in: float, value_out: float)")]
    #[new]
    fn new(value_in: f64, value_out: f64) -> Self {
        Self {
            value_in,
            value_out,
        }
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}
