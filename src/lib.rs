use ::autosar_data as autosar_data_rs;
use pyo3::create_exception;
use pyo3::prelude::*;
use pyo3::types::*;

mod arxmlfile;
mod element;
mod model;

create_exception!(module, AutosarDataError, pyo3::exceptions::PyException);

#[pyclass(frozen)]
#[derive(Debug, Clone)]
struct AutosarModel(autosar_data_rs::AutosarModel);

#[pyclass(frozen)]
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
struct IncompatibleElementError {
    element: Element,
    version_mask: u32,
    target_version: autosar_data_rs::AutosarVersion,
}

#[pyclass(frozen)]
#[derive(Debug, Clone)]
struct IncompatibleAttributeError {
    element: Element,
    attribute: autosar_data_rs::AttributeName,
    version_mask: u32,
    target_version: autosar_data_rs::AutosarVersion,
}

#[pyclass(frozen)]
#[derive(Debug, Clone)]
struct IncompatibleAttributeValueError {
    element: Element,
    attribute: autosar_data_rs::AttributeName,
    attribute_value: String,
    version_mask: u32,
    target_version: autosar_data_rs::AutosarVersion,
}

#[pyclass(frozen)]
#[derive(Clone)]
struct ElementType(autosar_data_specification::ElementType);

#[pyclass(frozen)]
struct Attribute {
    pub attrname: autosar_data_rs::AttributeName,
    pub content: PyObject,
}

#[pyclass(frozen)]
#[derive(Debug, Clone)]
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
        format!(
            "Attribute {} in <{}> is incompatible with version {}",
            self.attribute,
            self.element.0.xml_path(),
            self.target_version
        )
    }

    #[getter]
    fn get_attribute(&self) -> autosar_data_rs::AttributeName {
        self.attribute
    }

    #[getter]
    fn get_version_mask(&self) -> u32 {
        self.version_mask
    }

    #[getter]
    fn get_element(&self) -> Element {
        self.element.clone()
    }
}

#[pymethods]
impl IncompatibleAttributeValueError {
    fn __repr__(&self) -> String {
        format!("{:#?}", self)
    }

    fn __str__(&self) -> String {
        format!(
            "Attribute value {} in attribue {} of element <{}> is incompatible with version {}",
            self.attribute_value,
            self.attribute,
            self.element.0.xml_path(),
            self.target_version
        )
    }

    #[getter]
    fn get_attribute(&self) -> autosar_data_rs::AttributeName {
        self.attribute
    }

    #[getter]
    fn get_attribute_value(&self) -> String {
        self.attribute_value.clone()
    }

    #[getter]
    fn get_version_mask(&self) -> u32 {
        self.version_mask
    }

    #[getter]
    fn get_element(&self) -> Element {
        self.element.clone()
    }
}

#[pymethods]
impl IncompatibleElementError {
    fn __repr__(&self) -> String {
        format!("{:#?}", self)
    }

    fn __str__(&self) -> String {
        format!(
            "Element <{}> is incompatible with version {}",
            self.element.0.xml_path(),
            self.target_version
        )
    }

    #[getter]
    fn get_version_mask(&self) -> u32 {
        self.version_mask
    }

    #[getter]
    fn get_element(&self) -> Element {
        self.element.clone()
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

    fn splittable_in(&self, version: autosar_data_rs::AutosarVersion) -> bool {
        self.0.splittable_in(version)
    }

    fn reference_dest_value(&self, target: &ElementType) -> Option<autosar_data_rs::EnumItem> {
        self.0.reference_dest_value(&target.0)
    }

    fn find_sub_element(
        &self,
        target_name: autosar_data_rs::ElementName,
        version: u32,
    ) -> Option<ElementType> {
        self.0
            .find_sub_element(target_name, version)
            .map(|(etype, _)| ElementType(etype))
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
            attrname,
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
    fn attrname(&self) -> autosar_data_rs::AttributeName {
        self.attrname
    }

    #[getter]
    fn content(&self) -> &PyObject {
        &self.content
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn autosar_data(py: Python, m: &PyModule) -> PyResult<()> {
    let submodule = PyModule::new(py, "specification")?;
    submodule.add_class::<autosar_data_rs::ElementName>()?;
    submodule.add_class::<autosar_data_rs::AttributeName>()?;
    submodule.add_class::<autosar_data_rs::AutosarVersion>()?;
    submodule.add_class::<autosar_data_rs::EnumItem>()?;
    submodule.add_class::<ElementType>()?;
    m.add_submodule(submodule)?;
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
    m.add("AutosarDataError", py.get_type::<AutosarDataError>())?;
    Ok(())
}

fn extract_character_data(any: PyObject) -> PyResult<autosar_data_rs::CharacterData> {
    Python::with_gil(|py| {
        if let Ok(text) = any.extract::<String>(py) {
            Ok(autosar_data_rs::CharacterData::String(text))
        } else if let Ok(val) = any.extract::<u64>(py) {
            Ok(autosar_data_rs::CharacterData::UnsignedInteger(val))
        } else if let Ok(val) = any.extract::<f64>(py) {
            Ok(autosar_data_rs::CharacterData::Double(val))
        } else if let Ok(enumitem) = any.extract::<autosar_data_rs::EnumItem>(py) {
            Ok(autosar_data_rs::CharacterData::Enum(enumitem))
        } else {
            Err(AutosarDataError::new_err(
                autosar_data_rs::AutosarDataError::IncorrectContentType.to_string(),
            ))
        }
    })
}

fn character_data_to_object(cdata: &autosar_data_rs::CharacterData) -> PyObject {
    Python::with_gil(|py| match cdata {
        autosar_data_rs::CharacterData::Enum(enumitem) => {
            Py::new(py, *enumitem).unwrap().into_py(py)
        }
        autosar_data_rs::CharacterData::String(s) => PyString::new(py, s).into_py(py),
        autosar_data_rs::CharacterData::UnsignedInteger(val) => val.to_object(py),
        autosar_data_rs::CharacterData::Double(val) => val.to_object(py),
    })
}
