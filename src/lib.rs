use std::str::FromStr;

use ::autosar_data as autosar_data_rs;
use autosar_data_specification::CharacterDataSpec;
use pyo3::create_exception;
use pyo3::intern;
use pyo3::prelude::*;
use pyo3::types::*;

mod arxmlfile;
mod element;
mod model;
mod version;

use version::*;

create_exception!(module, AutosarDataError, pyo3::exceptions::PyException);

#[pyclass(frozen)]
struct AutosarModel(autosar_data_rs::AutosarModel);

#[pyclass(frozen)]
struct ArxmlFile(autosar_data_rs::ArxmlFile);

#[pyclass(frozen)]
#[derive(Debug, Clone)]
struct Element(autosar_data_rs::Element);

#[pyclass]
struct ElementsDfsIterator(autosar_data_rs::ElementsDfsIterator);

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
struct IncompatibleElementError {
    #[pyo3(get)]
    element: Element,
    #[pyo3(get)]
    version_mask: u32,
    target_version: AutosarVersion,
}

#[pyclass(frozen)]
#[derive(Debug)]
struct IncompatibleAttributeError {
    #[pyo3(get)]
    element: Element,
    #[pyo3(get)]
    attribute: String,
    #[pyo3(get)]
    version_mask: u32,
    target_version: AutosarVersion,
}

#[pyclass(frozen)]
#[derive(Debug)]
struct IncompatibleAttributeValueError {
    #[pyo3(get)]
    element: Element,
    #[pyo3(get)]
    attribute: String,
    #[pyo3(get)]
    attribute_value: String,
    #[pyo3(get)]
    version_mask: u32,
    target_version: AutosarVersion,
}

#[pyclass(frozen)]
struct ElementType(autosar_data_specification::ElementType);

#[pyclass(frozen)]
struct Attribute {
    pub attrname: String,
    pub content: PyObject,
}

#[pyclass(frozen)]
#[derive(Debug)]
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
enum ContentType {
    /// The element only contains other elements
    Elements,
    /// The element only contains character data
    CharacterData,
    /// The element contains both character data and sub elements
    Mixed,
}

#[pymethods]
impl IncompatibleAttributeError {
    fn __repr__(&self) -> String {
        format!("{:#?}", self)
    }

    fn __str__(&self) -> String {
        let allowed_versions = autosar_data_specification::expand_version_mask(self.version_mask)
            .iter()
            .map(|ver| ver.describe())
            .collect::<Vec<&'static str>>()
            .join(", ");
        format!(
            "Attribute {} in <{}> is incompatible with version {:?}. It is allowed in \"{allowed_versions}\"",
            self.attribute,
            self.element.0.xml_path(),
            self.target_version
        )
    }
}

#[pymethods]
impl IncompatibleAttributeValueError {
    fn __repr__(&self) -> String {
        format!("{:#?}", self)
    }

    fn __str__(&self) -> String {
        let allowed_versions = autosar_data_specification::expand_version_mask(self.version_mask)
            .iter()
            .map(|ver| ver.describe())
            .collect::<Vec<&'static str>>()
            .join(", ");
        format!(
            "Attribute value {} in attribue {} of element <{}> is incompatible with version {:?}. It is allowed in \"{allowed_versions}\"",
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
        format!("{:#?}", self)
    }

    fn __str__(&self) -> String {
        let allowed_versions = autosar_data_specification::expand_version_mask(self.version_mask)
            .iter()
            .map(|ver| ver.describe())
            .collect::<Vec<&'static str>>()
            .join(", ");
        format!(
            "Element <{}> is incompatible with version {:?}. It is allowed in \"{allowed_versions}\"",
            self.element.0.xml_path(),
            self.target_version
        )
    }
}

#[pymethods]
impl ElementType {
    fn __repr__(&self) -> String {
        format!("{:#?}", self.0)
    }

    #[getter]
    fn is_named(&self) -> bool {
        self.0.is_named()
    }

    #[getter]
    fn is_ref(&self) -> bool {
        self.0.is_ref()
    }

    #[getter]
    fn is_ordered(&self) -> bool {
        self.0.is_ordered()
    }

    #[getter]
    fn splittable(&self) -> u32 {
        self.0.splittable()
    }

    fn splittable_in(&self, version: AutosarVersion) -> bool {
        self.0.splittable_in(version.into())
    }

    fn reference_dest_value(&self, target: &ElementType) -> Option<String> {
        self.0
            .reference_dest_value(&target.0)
            .map(|enumitem| enumitem.to_string())
    }

    fn find_sub_element(&self, target_name: String, version: u32) -> PyResult<Option<ElementType>> {
        let elem_name = get_element_name(target_name)?;
        Ok(self
            .0
            .find_sub_element(elem_name, version)
            .map(|(etype, _)| ElementType(etype)))
    }
}

#[pymethods]
impl ContentType {
    fn __repr__(&self) -> String {
        format!("{:#?}", self)
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
                PyTuple::new(
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
impl ArxmlFileElementsDfsIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&mut self) -> Option<PyObject> {
        Python::with_gil(|py| {
            self.0.next().map(|(depth, elem)| {
                PyTuple::new(
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

    #[getter]
    fn attrname(&self) -> String {
        self.attrname.to_string()
    }

    #[getter]
    fn content(&self) -> &PyObject {
        &self.content
    }
}

#[pymethods]
impl ValidSubElementInfo {
    fn __repr__(&self) -> String {
        format!("{:#?}", self)
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn autosar_data(py: Python, m: &PyModule) -> PyResult<()> {
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
    m.add_class::<AttributeIterator>()?;
    m.add_class::<Attribute>()?;
    m.add_class::<ValidSubElementInfo>()?;
    m.add("AutosarDataError", py.get_type::<AutosarDataError>())?;
    m.add("version", intern!(m.py(), env!("CARGO_PKG_VERSION")))?;
    Ok(())
}

fn extract_character_data(
    spec: &CharacterDataSpec,
    any: PyObject,
) -> PyResult<autosar_data_rs::CharacterData> {
    Python::with_gil(|py| {
        if let Ok(text) = any.extract::<String>(py) {
            parse_cdata_string(spec, text).map_err(|_| {
                AutosarDataError::new_err(
                    autosar_data_rs::AutosarDataError::IncorrectContentType.to_string(),
                )
            })
        } else if let Ok(val) = any.extract::<u64>(py) {
            Ok(autosar_data_rs::CharacterData::UnsignedInteger(val))
        } else if let Ok(val) = any.extract::<f64>(py) {
            Ok(autosar_data_rs::CharacterData::Double(val))
        } else {
            Err(AutosarDataError::new_err(
                autosar_data_rs::AutosarDataError::IncorrectContentType.to_string(),
            ))
        }
    })
}

fn parse_cdata_string(
    spec: &CharacterDataSpec,
    text: String,
) -> Result<autosar_data_rs::CharacterData, ()> {
    match spec {
        CharacterDataSpec::Enum { items } => {
            let enumitem = autosar_data_rs::EnumItem::from_str(&text).map_err(|_| ())?;
            if items.iter().any(|(spec_item, _)| *spec_item == enumitem) {
                Ok(autosar_data_rs::CharacterData::Enum(enumitem))
            } else {
                Err(())
            }
        }
        CharacterDataSpec::Pattern {
            check_fn,
            max_length,
            ..
        } => {
            if text.len() < max_length.unwrap_or(usize::MAX) && check_fn(text.as_bytes()) {
                Ok(autosar_data_rs::CharacterData::String(text))
            } else {
                Err(())
            }
        }
        CharacterDataSpec::String { max_length, .. } => {
            if text.len() < max_length.unwrap_or(usize::MAX) {
                Ok(autosar_data_rs::CharacterData::String(text))
            } else {
                Err(())
            }
        }
        CharacterDataSpec::UnsignedInteger => {
            if let Ok(val) = text.parse() {
                Ok(autosar_data_rs::CharacterData::UnsignedInteger(val))
            } else {
                Err(())
            }
        }
        CharacterDataSpec::Double => {
            if let Ok(val) = text.parse() {
                Ok(autosar_data_rs::CharacterData::Double(val))
            } else {
                Err(())
            }
        }
    }
}

fn character_data_to_object(cdata: &autosar_data_rs::CharacterData) -> PyObject {
    Python::with_gil(|py| match cdata {
        autosar_data_rs::CharacterData::Enum(enumitem) => {
            PyString::new(py, enumitem.to_str()).into_py(py)
        }
        autosar_data_rs::CharacterData::String(s) => PyString::new(py, s).into_py(py),
        autosar_data_rs::CharacterData::UnsignedInteger(val) => val.to_object(py),
        autosar_data_rs::CharacterData::Double(val) => val.to_object(py),
    })
}

fn get_element_name(name_str: String) -> PyResult<autosar_data_rs::ElementName> {
    autosar_data_rs::ElementName::from_str(&name_str).or_else(|_| {
        PyResult::Err(AutosarDataError::new_err(format!(
            "Cannot convert \"{name_str}\" to ElementName"
        )))
    })
}

fn get_attribute_name(name_str: String) -> PyResult<autosar_data_rs::AttributeName> {
    autosar_data_rs::AttributeName::from_str(&name_str).or_else(|_| {
        PyResult::Err(AutosarDataError::new_err(format!(
            "Cannot convert \"{name_str}\" to AttributeName"
        )))
    })
}
