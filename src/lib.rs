use std::str::FromStr;

use ::autosar_data as autosar_data_rs;
use autosar_data_rs::CharacterData;
use autosar_data_specification::expand_version_mask;
use autosar_data_specification::CharacterDataSpec;
use pyo3::create_exception;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::intern;
use pyo3::prelude::*;
use pyo3::types::*;

mod arxmlfile;
mod element;
mod model;
mod specification;
mod version;

use version::AutosarVersion;

create_exception!(module, AutosarDataError, pyo3::exceptions::PyException);

#[pyclass(frozen)]
/// Autosar data model. It contains all elements.
struct AutosarModel(autosar_data_rs::AutosarModel);

#[pyclass(frozen)]
/// Represents a file that is part of an AutosarModel
struct ArxmlFile(autosar_data_rs::ArxmlFile);

#[pyclass(frozen)]
#[derive(Debug, Clone)]
/// An element in the Autosar data model
struct Element(autosar_data_rs::Element);

#[pyclass]
struct ElementsDfsIterator(autosar_data_rs::ElementsDfsIterator);

#[pyclass]
struct IdentifiablesIterator(autosar_data_rs::IdentifiablesIterator);

#[pyclass]
struct ArxmlFileElementsDfsIterator(autosar_data_rs::ArxmlFileElementsDfsIterator);

#[pyclass]
struct ElementContentIterator(autosar_data_rs::ElementContentIterator);

#[pyclass]
struct ElementsIterator(autosar_data_rs::ElementsIterator);

#[pyclass]
struct AttributeIterator(autosar_data_rs::AttributeIterator);

#[pyclass(frozen)]
#[derive(Debug)]
/// Information about an element that is incompatible with a given target version
struct IncompatibleElementError {
    #[pyo3(get)]
    element: Element,
    #[pyo3(get)]
    allowed_versions: Vec<AutosarVersion>,
    target_version: AutosarVersion,
}

#[pyclass(frozen)]
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

#[pyclass(frozen)]
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

#[pyclass(frozen)]
/// Type of an Element in the specification
struct ElementType(autosar_data_specification::ElementType);

#[pyclass(frozen)]
/// An attribute on an element
struct Attribute {
    #[pyo3(get)]
    pub attrname: String,
    #[pyo3(get)]
    pub content: PyObject,
}

#[pyclass(frozen)]
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

#[pyclass(frozen)]
#[derive(Debug)]
/// The content type of an element
enum ContentType {
    /// The element only contains other elements
    Elements,
    /// The element only contains character data
    CharacterData,
    /// The element contains both character data and sub elements
    Mixed,
}

#[pyclass]
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

#[pyclass]
#[derive(Debug)]
/// The character data in an element or attribute is an enum value
struct CharacterDataTypeEnum {
    #[pyo3(get)]
    /// list of permitted enum values
    values: Vec<String>,
}

#[pyclass]
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

#[pyclass]
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

#[pyclass]
#[derive(Debug)]
/// The character data in an element or attribute is an unsigned integer
struct CharacterDataTypeUnsignedInt(());

#[pyclass]
#[derive(Debug)]
/// The character data in an element or attribute is a float
struct CharacterDataTypeFloat(());

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
impl ElementsDfsIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&mut self) -> Option<PyObject> {
        Python::with_gil(|py| {
            self.0.next().map(|(depth, elem)| {
                PyTuple::new_bound(
                    py,
                    [
                        depth.to_object(py),
                        Py::new(py, Element(elem)).unwrap().into_py(py),
                    ]
                    .iter(),
                )
                .to_object(py)
            })
        })
    }
}

#[pymethods]
impl IdentifiablesIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&mut self) -> Option<PyObject> {
        Python::with_gil(|py| {
            for (path, weak) in &mut self.0 {
                if let Some(elem) = weak.upgrade() {
                    return Some(
                        PyTuple::new_bound(
                            py,
                            [
                                path.to_object(py),
                                Py::new(py, Element(elem)).unwrap().into_py(py),
                            ]
                            .iter(),
                        )
                        .to_object(py),
                    );
                }
            }

            None
        })
    }
}

#[pymethods]
impl ArxmlFileElementsDfsIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&mut self) -> Option<PyObject> {
        Python::with_gil(|py| {
            self.0.next().map(|(depth, elem)| {
                PyTuple::new_bound(
                    py,
                    [
                        depth.to_object(py),
                        Py::new(py, Element(elem)).unwrap().into_py(py),
                    ]
                    .iter(),
                )
                .to_object(py)
            })
        })
    }
}

#[pymethods]
impl ElementContentIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&mut self) -> Option<PyObject> {
        let ec = self.0.next()?;
        Python::with_gil(|py| match ec {
            autosar_data_rs::ElementContent::Element(elem) => {
                Some(Py::new(py, Element(elem)).unwrap().into_py(py))
            }
            autosar_data_rs::ElementContent::CharacterData(cdata) => {
                Some(character_data_to_object(&cdata))
            }
        })
    }
}

#[pymethods]
impl ElementsIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&mut self) -> Option<Element> {
        self.0.next().map(Element)
    }
}

#[pymethods]
impl AttributeIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&mut self) -> Option<Attribute> {
        let autosar_data_rs::Attribute { attrname, content } = self.0.next()?;
        Some(Attribute {
            attrname: attrname.to_string(),
            content: character_data_to_object(&content),
        })
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

#[pyfunction]
fn check_buffer(object: PyObject) -> PyResult<bool> {
    Python::with_gil(|py| {
        if let Ok(bytebuffer) = object.extract::<&[u8]>(py) {
            Ok(autosar_data_rs::check_buffer(bytebuffer))
        } else if let Ok(stringbuffer) = object.extract::<&str>(py) {
            Ok(autosar_data_rs::check_buffer(stringbuffer.as_bytes()))
        } else {
            let any = object.bind(py);
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
/// Classes:
///
/// - ArxmlFile
/// - AutosarModel
/// - AutosarVersion
/// - Element
/// - ElementType
/// - ValidSubElementInfo
///
/// Functions:
///
/// - check_file
/// - check_buffwe
///
/// Variables:
///
/// - __version__
#[pymodule]
fn autosar_data(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
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
    m.add_class::<ArxmlFileElementsDfsIterator>()?;
    m.add_class::<ElementContentIterator>()?;
    m.add_class::<ElementsIterator>()?;
    m.add_class::<IdentifiablesIterator>()?;
    m.add_class::<AttributeIterator>()?;
    m.add_class::<Attribute>()?;
    m.add_class::<AttributeSpec>()?;
    m.add_class::<ValidSubElementInfo>()?;
    m.add_class::<CharacterDataTypeEnum>()?;
    m.add_class::<CharacterDataTypeFloat>()?;
    m.add_class::<CharacterDataTypeRestrictedString>()?;
    m.add_class::<CharacterDataTypeString>()?;
    m.add_class::<CharacterDataTypeUnsignedInt>()?;
    m.add_function(wrap_pyfunction!(check_file, m)?)?;
    m.add_function(wrap_pyfunction!(check_buffer, m)?)?;
    m.add("AutosarDataError", py.get_type_bound::<AutosarDataError>())?;
    m.add("__version__", intern!(m.py(), env!("CARGO_PKG_VERSION")))?;
    Ok(())
}

fn extract_character_data(
    spec: &CharacterDataSpec,
    object: &PyObject,
) -> PyResult<autosar_data_rs::CharacterData> {
    Python::with_gil(|py| {
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
            CharacterDataSpec::Double => {
                if let Ok(strval) = any.extract::<String>() {
                    if let Ok(floatval) = strval.parse() {
                        Ok(CharacterData::Double(floatval))
                    } else {
                        Err(PyValueError::new_err(format!(
                            "invalid literal '{strval}' for conversion to float"
                        )))
                    }
                } else if let Ok(intval) = any.extract::<u64>() {
                    Ok(CharacterData::Double(intval as f64))
                } else if let Ok(floatval) = any.extract::<f64>() {
                    Ok(CharacterData::Double(floatval))
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

fn character_data_to_object(cdata: &autosar_data_rs::CharacterData) -> PyObject {
    Python::with_gil(|py| match cdata {
        autosar_data_rs::CharacterData::Enum(enumitem) => {
            PyString::new_bound(py, enumitem.to_str()).into_py(py)
        }
        autosar_data_rs::CharacterData::String(s) => {
            if let Some(val) = cdata.decode_integer::<i64>() {
                val.to_object(py)
            } else {
                PyString::new_bound(py, s).into_py(py)
            }
        }
        autosar_data_rs::CharacterData::UnsignedInteger(val) => val.to_object(py),
        autosar_data_rs::CharacterData::Double(val) => val.to_object(py),
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

fn version_mask_from_any(version_obj: &PyObject) -> PyResult<u32> {
    Python::with_gil(|py| {
        if let Ok(list) = version_obj.extract::<&PyList>(py) {
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
