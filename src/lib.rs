use std::str::FromStr;

use ::autosar_data as autosar_data_rs;
use autosar_data_rs::CharacterData;
use autosar_data_specification::CharacterDataSpec;
use autosar_data_specification::expand_version_mask;
use pyo3::create_exception;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::intern;
use pyo3::prelude::*;
use pyo3::types::*;

// abstraction appears in the api
mod abstraction;

// These modules are not part of the api
mod arxmlfile;
mod element;
mod model;
mod pyutils;
mod specification;
mod version;

use pyo3::IntoPyObjectExt;
use version::AutosarVersion;

create_exception!(
    autosar_data.autosar_data,
    AutosarDataError,
    pyo3::exceptions::PyException
);

#[pyclass(frozen, module = "autosar_data._autosar_data")]
/// Autosar data model. It contains all elements.
struct AutosarModel(autosar_data_rs::AutosarModel);

#[pyclass(frozen, module = "autosar_data._autosar_data")]
/// Represents a file that is part of an AutosarModel
struct ArxmlFile(autosar_data_rs::ArxmlFile);

#[pyclass(frozen, module = "autosar_data._autosar_data")]
#[derive(Debug, Clone)]
/// An element in the Autosar data model
struct Element(autosar_data_rs::Element);

#[pyclass(frozen, module = "autosar_data._autosar_data")]
#[derive(Debug)]
/// Information about an element that is incompatible with a given target version
struct IncompatibleElementError {
    #[pyo3(get)]
    element: Element,
    #[pyo3(get)]
    allowed_versions: Vec<AutosarVersion>,
    target_version: AutosarVersion,
}

#[pyclass(frozen, module = "autosar_data._autosar_data")]
#[derive(Debug)]
/// Information about an attribute that is incompatible with a given target version
struct IncompatibleAttributeError {
    #[pyo3(get)]
    element: Element,
    #[pyo3(get)]
    attribute: String,
    #[pyo3(get)]
    allowed_versions: Vec<AutosarVersion>,
    target_version: AutosarVersion,
}

#[pyclass(frozen, module = "autosar_data._autosar_data")]
#[derive(Debug)]
/// Information about an attribute value that is incompatible with a given target version
struct IncompatibleAttributeValueError {
    #[pyo3(get)]
    element: Element,
    #[pyo3(get)]
    attribute: String,
    #[pyo3(get)]
    attribute_value: String,
    #[pyo3(get)]
    allowed_versions: Vec<AutosarVersion>,
    target_version: AutosarVersion,
}

#[pyclass(eq, frozen, module = "autosar_data._autosar_data")]
#[derive(Clone, PartialEq, Eq)]
/// Type of an Element in the specification
struct ElementType(autosar_data_specification::ElementType);

#[pyclass(frozen, module = "autosar_data._autosar_data")]
/// An attribute on an element
struct Attribute {
    #[pyo3(get)]
    pub attrname: String,
    #[pyo3(get)]
    pub content: Py<PyAny>,
}

#[pyclass(frozen, module = "autosar_data._autosar_data")]
#[derive(Debug)]
/// Details about a particular sub element
struct ValidSubElementInfo {
    #[pyo3(get)]
    element_name: String,
    #[pyo3(get)]
    is_named: bool,
    #[pyo3(get)]
    is_allowed: bool,
}

#[pyclass(eq, eq_int, module = "autosar_data._autosar_data")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The content type of an element
enum ContentType {
    /// The element only contains other elements
    Elements,
    /// The element only contains character data
    CharacterData,
    /// The element contains both character data and sub elements
    Mixed,
}

#[pyclass(module = "autosar_data._autosar_data")]
#[derive(Debug)]
/// Specification of an attribute
struct AttributeSpec {
    #[pyo3(get)]
    /// name of the attribute
    attribute_name: String,
    /// specification of the attribute value
    value_spec: &'static CharacterDataSpec,
    #[pyo3(get)]
    /// is the attribute required or optional
    required: bool,
}

#[pyclass(frozen, module = "autosar_data._autosar_data")]
#[derive(Debug)]
/// Specification of a sub element
struct SubElementSpec {
    #[pyo3(get)]
    /// name of the sub element
    element_name: String,
    #[pyo3(get)]
    /// element type of the sub element
    element_type: ElementType,
    #[pyo3(get)]
    /// list of versions in which this sub element is compatible
    allowed_versions: Vec<AutosarVersion>,
}

#[pyclass(eq, eq_int, module = "autosar_data._autosar_data")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The content mode of an element type
enum ContentMode {
    /// Elements of this type contain an ordered sequence of elements
    Sequence,
    /// Elements of this type contain elements of a single element type chosen from multiple options
    Choice,
    /// Elements of this type contain a variable amount of unordered elements with different element types chosen from multiple options
    Bag,
    /// Elements of this type only contain character data
    Characters,
    /// Elements of this type contain both character data and sub elements
    Mixed,
}

#[pyclass(module = "autosar_data._autosar_data")]
#[derive(Debug)]
/// The character data in an element or attribute is an enum value
struct CharacterDataTypeEnum {
    #[pyo3(get)]
    /// list of permitted enum values
    values: Vec<String>,
}

#[pyclass(module = "autosar_data._autosar_data")]
#[derive(Debug)]
/// The character data in an element or attribute is a string that must match a regex
struct CharacterDataTypeRestrictedString {
    #[pyo3(get)]
    /// validation regex
    regex: String,
    #[pyo3(get)]
    /// max length (if any)
    max_length: Option<usize>,
}

#[pyclass(module = "autosar_data._autosar_data")]
#[derive(Debug)]
/// The character data in an element or attribute is a string
struct CharacterDataTypeString {
    #[pyo3(get)]
    /// does this element preserve whitespace in its character data
    preserve_whitespace: bool,
    #[pyo3(get)]
    /// max length (if any)
    max_length: Option<usize>,
}

#[pyclass(module = "autosar_data._autosar_data")]
#[derive(Debug)]
/// The character data in an element or attribute is an unsigned integer
struct CharacterDataTypeUnsignedInt(());

#[pyclass(module = "autosar_data._autosar_data")]
#[derive(Debug)]
/// The character data in an element or attribute is a float
struct CharacterDataTypeFloat(());

//##################################################################

// The autosar_data_abstraction crate returns iterators that are not directly usable in Python.
// Every one of these iterators follows the same pattern, returning "impl Iterator<Item = T>", so
// they can all be wrapped using the same method.
macro_rules! iterator_wrapper {
    ($iter_name:ident, $item_name:ty) => {
        iterator_wrapper!($iter_name, $item_name, stringify!($item_name));
    };
    ($iter_name:ident, $item_name:ty, $desc:expr) => {
        #[pyclass(module = "autosar_data._autosar_data._iterators")]
        pub(crate) struct $iter_name {
            iter: Box<dyn Iterator<Item = $item_name> + Sync + Send + 'static>,
        }

        impl $iter_name {
            pub(crate) fn new(
                iter: impl Iterator<Item = $item_name> + Sync + Send + 'static,
            ) -> Self {
                Self {
                    iter: Box::new(iter),
                }
            }
        }

        #[pymethods]
        impl $iter_name {
            fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
                slf
            }

            fn __next__(&mut self) -> Option<$item_name> {
                self.iter.next()
            }

            fn __repr__(&self) -> String {
                concat!("Iterator[", $desc, "]").to_string()
            }
        }
    };
}

pub(crate) use iterator_wrapper;

//##################################################################

iterator_wrapper!(ElementsDfsIterator, Py<PyAny>, "Tuple[int, Element]");
iterator_wrapper!(IdentifiablesIterator, Py<PyAny>, "Tuple[str, Element]");
iterator_wrapper!(AttributeIterator, Attribute);
iterator_wrapper!(ElementsIterator, Element);
iterator_wrapper!(
    ElementContentIterator,
    Py<PyAny>,
    "Union[Element, CharacterData]"
);

//##################################################################

#[pymethods]
impl IncompatibleAttributeError {
    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }

    fn __str__(&self) -> String {
        let ver_first: autosar_data_rs::AutosarVersion = self.allowed_versions[0].into();
        let ver_last: autosar_data_rs::AutosarVersion =
            self.allowed_versions[self.allowed_versions.len() - 1].into();
        let allowed_versions_str = if ver_first == ver_last {
            format!("{ver_first:?}")
        } else {
            format!("{ver_first:?} - {ver_last:?}")
        };
        format!(
            "Attribute {} in <{}> is incompatible with version {:?}. It is allowed in {allowed_versions_str}",
            self.attribute,
            self.element.0.xml_path(),
            self.target_version
        )
    }
}

#[pymethods]
impl IncompatibleAttributeValueError {
    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }

    fn __str__(&self) -> String {
        let ver_first: autosar_data_rs::AutosarVersion = self.allowed_versions[0].into();
        let ver_last: autosar_data_rs::AutosarVersion =
            self.allowed_versions[self.allowed_versions.len() - 1].into();
        let allowed_versions_str = if ver_first == ver_last {
            format!("{ver_first:?}")
        } else {
            format!("{ver_first:?} - {ver_last:?}")
        };
        format!(
            "Attribute value {} in attribue {} of element <{}> is incompatible with version {:?}. It is allowed in {allowed_versions_str}",
            self.attribute_value,
            self.attribute,
            self.element.0.xml_path(),
            self.target_version
        )
    }
}

#[pymethods]
impl IncompatibleElementError {
    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }

    fn __str__(&self) -> String {
        let ver_first: autosar_data_rs::AutosarVersion = self.allowed_versions[0].into();
        let ver_last: autosar_data_rs::AutosarVersion =
            self.allowed_versions[self.allowed_versions.len() - 1].into();
        let allowed_versions_str = if ver_first == ver_last {
            format!("{ver_first:?}")
        } else {
            format!("{ver_first:?} - {ver_last:?}")
        };
        format!(
            "Element <{}> is incompatible with version {:?}. It is allowed in {allowed_versions_str}",
            self.element.0.xml_path(),
            self.target_version
        )
    }
}

#[pymethods]
impl ContentType {
    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}

#[pymethods]
impl Attribute {
    fn __repr__(&self) -> String {
        format!(
            "Attribute {{attrname={:?}, content=\"{}\" }}",
            self.attrname, self.content
        )
    }

    fn __str__(&self) -> String {
        format!("Attribute({}=\"{}\")", self.attrname, self.content)
    }
}

#[pymethods]
impl ValidSubElementInfo {
    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}

#[pyfunction]
fn check_file(filename: &str) -> bool {
    autosar_data_rs::check_file(filename)
}

/// Check if the given buffer contains valid Autosar data
///
/// The function returns true if the buffer starts with a valid arxml header (after
/// skipping whitespace and comments). This function does not check anything after the header.
#[pyfunction]
#[pyo3(signature = (input, /))]
#[pyo3(text_signature = "(input: Union[bytes, str], /)")]
fn check_buffer(input: Py<PyAny>) -> PyResult<bool> {
    Python::attach(|py| {
        if let Ok(bytebuffer) = input.extract::<&[u8]>(py) {
            Ok(autosar_data_rs::check_buffer(bytebuffer))
        } else if let Ok(stringbuffer) = input.extract::<&str>(py) {
            Ok(autosar_data_rs::check_buffer(stringbuffer.as_bytes()))
        } else {
            let any = input.bind(py);
            Err(PyTypeError::new_err(format!(
                "'{}' cannot be converted to 'bytes'",
                any.get_type()
            )))
        }
    })
}

/// Provides functionality to read, modify and write Autosar arxml files,
/// both separately and in projects consisting of multiple files.
///
/// Submodules:
///
/// - abstraction
///
/// Classes:
///
/// - Attribute
/// - ArxmlFile
/// - AutosarModel
/// - AutosarVersion
/// - ContentMode
/// - Element
/// - ElementType
/// - SubElementSpec
/// - ValidSubElementInfo
///
/// Functions:
///
/// - check_file
/// - check_buffer
///
/// Variables:
///
/// - __version__
#[pymodule(gil_used = false)]
fn _autosar_data(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ElementType>()?;
    m.add_class::<AutosarVersion>()?;
    m.add_class::<AutosarModel>()?;
    m.add_class::<ArxmlFile>()?;
    m.add_class::<Element>()?;
    m.add_class::<IncompatibleAttributeError>()?;
    m.add_class::<IncompatibleAttributeValueError>()?;
    m.add_class::<IncompatibleElementError>()?;
    m.add_class::<ContentType>()?;
    m.add_class::<ElementsDfsIterator>()?;
    m.add_class::<ElementContentIterator>()?;
    m.add_class::<ElementsIterator>()?;
    m.add_class::<IdentifiablesIterator>()?;
    m.add_class::<AttributeIterator>()?;
    m.add_class::<Attribute>()?;
    m.add_class::<AttributeSpec>()?;
    m.add_class::<SubElementSpec>()?;
    m.add_class::<ContentMode>()?;
    m.add_class::<ValidSubElementInfo>()?;
    m.add_class::<CharacterDataTypeEnum>()?;
    m.add_class::<CharacterDataTypeFloat>()?;
    m.add_class::<CharacterDataTypeRestrictedString>()?;
    m.add_class::<CharacterDataTypeString>()?;
    m.add_class::<CharacterDataTypeUnsignedInt>()?;
    m.add_function(wrap_pyfunction!(check_file, m)?)?;
    m.add_function(wrap_pyfunction!(check_buffer, m)?)?;
    m.add("AutosarDataError", py.get_type::<AutosarDataError>())?;
    m.add("__version__", intern!(m.py(), env!("CARGO_PKG_VERSION")))?;

    abstraction::add_submodules(py, m)?;

    Ok(())
}

fn extract_character_data(
    spec: &CharacterDataSpec,
    object: &Py<PyAny>,
) -> PyResult<autosar_data_rs::CharacterData> {
    Python::attach(|py| {
        let any = object.bind(py);
        match spec {
            CharacterDataSpec::Enum { .. } => {
                if let Ok(strval) = any.extract::<String>() {
                    if let Ok(enumitem) = autosar_data_rs::EnumItem::from_str(&strval) {
                        Ok(CharacterData::Enum(enumitem))
                    } else {
                        Err(PyValueError::new_err(format!(
                            "string value '{strval}' cannot be converted to 'EnumItem'"
                        )))
                    }
                } else {
                    Err(PyTypeError::new_err(format!(
                        "'{}' cannot be converted to 'EnumItem'",
                        any.get_type()
                    )))
                }
            }
            CharacterDataSpec::Pattern { .. } | CharacterDataSpec::String { .. } => {
                if let Ok(text) = any.extract::<String>() {
                    Ok(CharacterData::String(text))
                } else if let Ok(intval) = any.extract::<u64>() {
                    Ok(CharacterData::String(intval.to_string()))
                } else if let Ok(floatval) = any.extract::<f64>() {
                    Ok(CharacterData::String(floatval.to_string()))
                } else {
                    Err(PyTypeError::new_err(format!(
                        "'{}' cannot be converted to 'str'",
                        any.get_type()
                    )))
                }
            }
            CharacterDataSpec::UnsignedInteger => {
                if let Ok(strval) = any.extract::<String>() {
                    if let Ok(intval) = strval.parse() {
                        Ok(CharacterData::UnsignedInteger(intval))
                    } else {
                        Err(PyValueError::new_err(format!(
                            "invalid literal '{strval}' for conversion to int"
                        )))
                    }
                } else if let Ok(intval) = any.extract::<u64>() {
                    Ok(CharacterData::UnsignedInteger(intval))
                } else {
                    Err(PyTypeError::new_err(format!(
                        "'{}' cannot be converted to 'int'",
                        any.get_type()
                    )))
                }
            }
            CharacterDataSpec::Float => {
                if let Ok(strval) = any.extract::<String>() {
                    if let Ok(floatval) = strval.parse() {
                        Ok(CharacterData::Float(floatval))
                    } else {
                        Err(PyValueError::new_err(format!(
                            "invalid literal '{strval}' for conversion to float"
                        )))
                    }
                } else if let Ok(intval) = any.extract::<u64>() {
                    Ok(CharacterData::Float(intval as f64))
                } else if let Ok(floatval) = any.extract::<f64>() {
                    Ok(CharacterData::Float(floatval))
                } else {
                    Err(PyTypeError::new_err(format!(
                        "'{}' cannot be converted to 'float'",
                        any.get_type()
                    )))
                }
            }
        }
    })
}

fn character_data_to_object(cdata: &autosar_data_rs::CharacterData) -> PyResult<Py<PyAny>> {
    Python::attach(|py| match cdata {
        autosar_data_rs::CharacterData::Enum(enumitem) => enumitem.to_str().into_py_any(py),
        autosar_data_rs::CharacterData::String(s) => {
            if let Some(val) = cdata.parse_integer::<i64>() {
                val.into_py_any(py)
            } else {
                s.into_py_any(py)
            }
        }
        autosar_data_rs::CharacterData::UnsignedInteger(val) => val.into_py_any(py),
        autosar_data_rs::CharacterData::Float(val) => val.into_py_any(py),
    })
}

fn get_element_name(name_str: &str) -> PyResult<autosar_data_rs::ElementName> {
    autosar_data_rs::ElementName::from_str(name_str).or_else(|_| {
        PyResult::Err(AutosarDataError::new_err(format!(
            "Cannot convert \"{name_str}\" to ElementName"
        )))
    })
}

fn get_attribute_name(name_str: &str) -> PyResult<autosar_data_rs::AttributeName> {
    autosar_data_rs::AttributeName::from_str(name_str).or_else(|_| {
        PyResult::Err(AutosarDataError::new_err(format!(
            "Cannot convert \"{name_str}\" to AttributeName"
        )))
    })
}

fn version_mask_from_any(version_obj: &Py<PyAny>) -> PyResult<u32> {
    Python::attach(|py| {
        if let Ok(list) = version_obj.cast_bound::<PyList>(py) {
            let mut mask = 0;
            for item in list {
                let ver: autosar_data_rs::AutosarVersion = item.extract::<AutosarVersion>()?.into();
                mask |= ver as u32;
            }
            Ok(mask)
        } else if let Ok(version_py) = version_obj.extract::<AutosarVersion>(py) {
            let ver: autosar_data_rs::AutosarVersion = version_py.into();
            Ok(ver as u32)
        } else {
            let any = version_obj.bind(py);
            Err(PyTypeError::new_err(format!(
                "'{}' cannot be converted to 'VersionSpecification'",
                any.get_type()
            )))
        }
    })
}
