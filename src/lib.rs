use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::hash::Hasher;

use ::autosar_data as autosar_data_rs;
use autosar_data_rs::CompatibilityError;
use pyo3::create_exception;
use pyo3::prelude::*;
use pyo3::types::*;

create_exception!(module, AutosarDataError, pyo3::exceptions::PyException);

#[pyclass(frozen)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct AutosarModel(autosar_data_rs::AutosarModel);

#[pyclass(frozen)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ArxmlFile(autosar_data_rs::ArxmlFile);

#[pyclass(frozen)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
struct IncompatibleElementError {
    element: Element,
    version_mask: u32,
    target_version: autosar_data_rs::AutosarVersion,
}

#[pyclass(frozen)]
#[derive(Debug, Clone, PartialEq, Eq)]
struct IncompatibleAttributeError {
    element: Element,
    attribute: autosar_data_rs::AttributeName,
    version_mask: u32,
    target_version: autosar_data_rs::AutosarVersion,
}

#[pyclass(frozen)]
#[derive(Debug, Clone, PartialEq, Eq)]
struct IncompatibleAttributeValueError {
    element: Element,
    attribute: autosar_data_rs::AttributeName,
    attribute_value: String,
    version_mask: u32,
    target_version: autosar_data_rs::AutosarVersion,
}

#[pyclass(frozen)]
#[derive(Clone, PartialEq, Eq)]
struct ElementType(autosar_data_specification::ElementType);

#[pyclass(frozen)]
struct Attribute {
    pub attrname: autosar_data_rs::AttributeName,
    pub content: PyObject,
}

#[pyclass(frozen)]
#[derive(Debug, Clone, PartialEq, Eq)]
enum ContentType {
    /// The element only contains other elements
    Elements,
    /// The element only contains character data
    CharacterData,
    /// The element contains both character data and sub elements
    Mixed,
}

#[pymethods]
impl AutosarModel {
    #[new]
    fn new() -> Self {
        Self(autosar_data_rs::AutosarModel::new())
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self.0)
    }

    fn __str__(&self) -> String {
        self.0.root_element().serialize()
    }

    fn __richcmp__(&self, other: &AutosarModel, op: pyo3::basic::CompareOp) -> bool {
        match op {
            pyo3::pyclass::CompareOp::Eq => self.0 == other.0,
            pyo3::pyclass::CompareOp::Ne => self.0 != other.0,
            pyo3::pyclass::CompareOp::Lt
            | pyo3::pyclass::CompareOp::Le
            | pyo3::pyclass::CompareOp::Gt
            | pyo3::pyclass::CompareOp::Ge => false,
        }
    }

    fn __hash__(&self) -> isize {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish() as isize
    }

    fn create_file(
        &self,
        filename: &str,
        version: autosar_data_rs::AutosarVersion,
    ) -> PyResult<ArxmlFile> {
        match self.0.create_file(filename, version) {
            Ok(file) => Ok(ArxmlFile(file)),
            Err(error) => PyResult::Err(AutosarDataError::new_err(error.to_string())),
        }
    }

    fn load_buffer(
        &self,
        buffer: &str,
        filename: &str,
        strict: bool,
    ) -> PyResult<(ArxmlFile, Vec<String>)> {
        match self
            .0
            .load_named_arxml_buffer(buffer.as_bytes(), filename, strict)
        {
            Ok((file, warn)) => {
                let warnstrings: Vec<String> = warn.iter().map(|w| w.to_string()).collect();
                Ok((ArxmlFile(file), warnstrings))
            }
            Err(error) => PyResult::Err(AutosarDataError::new_err(error.to_string())),
        }
    }

    fn load_file(&self, filename: &str, strict: bool) -> PyResult<(ArxmlFile, Vec<String>)> {
        match self.0.load_arxml_file(filename, strict) {
            Ok((file, warn)) => {
                let warnstrings: Vec<String> = warn.iter().map(|w| w.to_string()).collect();
                Ok((ArxmlFile(file), warnstrings))
            }
            Err(error) => PyResult::Err(AutosarDataError::new_err(error.to_string())),
        }
    }

    fn remove_file(&self, file: &ArxmlFile) {
        self.0.remove_file(&file.0);
    }

    fn serialize_files(&self) -> HashMap<String, String> {
        let hm_orig: HashMap<std::path::PathBuf, String> = self.0.serialize_files();
        let mut hm_out = HashMap::<String, String>::new();
        for (k, v) in hm_orig {
            hm_out.insert(String::from(k.to_string_lossy()), v);
        }
        hm_out
    }

    fn write(&self) -> PyResult<()> {
        self.0
            .write()
            .map_err(|error| AutosarDataError::new_err(error.to_string()))
    }

    #[getter]
    fn files(&self) -> Vec<ArxmlFile> {
        self.0.files().map(ArxmlFile).collect()
    }

    #[getter]
    fn root_element(&self) -> Element {
        Element(self.0.root_element())
    }

    fn get_element_by_path(&self, path: &str) -> Option<Element> {
        self.0.get_element_by_path(path).map(Element)
    }

    #[getter]
    fn elements_dfs(&self) -> ElementsDfsIterator {
        ElementsDfsIterator(self.0.elements_dfs())
    }

    fn sort(&self) {
        self.0.sort()
    }

    #[getter]
    fn identifiable_elements(&self) -> Vec<String> {
        self.0.identifiable_elements()
    }

    fn get_references_to(&self, target_path: &str) -> Vec<Element> {
        self.0
            .get_references_to(target_path)
            .iter()
            .filter_map(|weak| weak.upgrade().map(Element))
            .collect()
    }

    fn check_references(&self) -> Vec<Element> {
        self.0
            .check_references()
            .iter()
            .filter_map(|weak| weak.upgrade().map(Element))
            .collect()
    }
}

#[pymethods]
impl ArxmlFile {
    fn __repr__(&self) -> String {
        format!("{:#?}", self.0)
    }

    fn __str__(&self) -> PyResult<String> {
        self.serialize()
    }

    fn __richcmp__(&self, other: &ArxmlFile, op: pyo3::basic::CompareOp) -> bool {
        match op {
            pyo3::pyclass::CompareOp::Eq => self.0 == other.0,
            pyo3::pyclass::CompareOp::Ne => self.0 != other.0,
            pyo3::pyclass::CompareOp::Lt
            | pyo3::pyclass::CompareOp::Le
            | pyo3::pyclass::CompareOp::Gt
            | pyo3::pyclass::CompareOp::Ge => false,
        }
    }

    fn __hash__(&self) -> isize {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish() as isize
    }

    #[getter]
    fn filename(&self) -> String {
        self.0.filename().to_string_lossy().into_owned()
    }

    #[setter]
    fn set_filename(&self, filename: &str) -> PyResult<()> {
        self.0
            .set_filename(filename)
            .map_err(|error| AutosarDataError::new_err(error.to_string()))
    }

    #[getter]
    fn version(&self) -> autosar_data_rs::AutosarVersion {
        self.0.version()
    }

    #[setter]
    fn set_version(&self, version: autosar_data_rs::AutosarVersion) -> PyResult<()> {
        self.0
            .set_version(version)
            .map_err(|error| AutosarDataError::new_err(error.to_string()))
    }

    fn check_version_compatibility(
        &self,
        target_version: autosar_data_rs::AutosarVersion,
    ) -> Vec<PyObject> {
        Python::with_gil(|py| {
            self.0
                .check_version_compatibility(target_version)
                .0
                .iter()
                .map(|cerr| -> PyObject {
                    match cerr {
                        CompatibilityError::IncompatibleAttribute {
                            element,
                            attribute,
                            version_mask,
                        } => Py::new(
                            py,
                            IncompatibleAttributeError {
                                element: Element(element.to_owned()),
                                attribute: *attribute,
                                version_mask: *version_mask,
                                target_version,
                            },
                        )
                        .unwrap()
                        .into_py(py),
                        CompatibilityError::IncompatibleAttributeValue {
                            element,
                            attribute,
                            attribute_value,
                            version_mask,
                        } => Py::new(
                            py,
                            IncompatibleAttributeValueError {
                                element: Element(element.to_owned()),
                                attribute: *attribute,
                                attribute_value: attribute_value.to_owned(),
                                version_mask: *version_mask,
                                target_version,
                            },
                        )
                        .unwrap()
                        .into_py(py),
                        CompatibilityError::IncompatibleElement {
                            element,
                            version_mask,
                        } => Py::new(
                            py,
                            IncompatibleElementError {
                                element: Element(element.to_owned()),
                                version_mask: *version_mask,
                                target_version,
                            },
                        )
                        .unwrap()
                        .into_py(py),
                    }
                })
                .collect()
        })
    }

    #[getter]
    fn model(&self) -> PyResult<AutosarModel> {
        match self.0.model() {
            Ok(model) => Ok(AutosarModel(model)),
            Err(error) => PyResult::Err(AutosarDataError::new_err(error.to_string())),
        }
    }

    #[getter]
    fn elements_dfs(&self) -> ArxmlFileElementsDfsIterator {
        ArxmlFileElementsDfsIterator(self.0.elements_dfs())
    }

    fn serialize(&self) -> PyResult<String> {
        match self.0.serialize() {
            Ok(text) => Ok(text),
            Err(error) => Err(AutosarDataError::new_err(error.to_string())),
        }
    }

    #[getter]
    fn xml_standalone(&self) -> Option<bool> {
        self.0.xml_standalone()
    }
}

#[pymethods]
impl Element {
    fn __repr__(&self) -> String {
        format!("{:#?}", self.0)
    }

    fn __str__(&self) -> String {
        self.0.serialize()
    }

    fn __richcmp__(&self, other: &Element, op: pyo3::basic::CompareOp) -> bool {
        match op {
            pyo3::pyclass::CompareOp::Eq => self.0 == other.0,
            pyo3::pyclass::CompareOp::Ne => self.0 != other.0,
            pyo3::pyclass::CompareOp::Lt
            | pyo3::pyclass::CompareOp::Le
            | pyo3::pyclass::CompareOp::Gt
            | pyo3::pyclass::CompareOp::Ge => false,
        }
    }

    fn __hash__(&self) -> isize {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish() as isize
    }

    fn serialize(&self) -> String {
        self.0.serialize()
    }

    #[getter]
    fn parent(&self) -> PyResult<Option<Element>> {
        match self.0.parent() {
            Ok(Some(parent)) => Ok(Some(Element(parent))),
            Ok(None) => Ok(None),
            Err(error) => Err(AutosarDataError::new_err(error.to_string())),
        }
    }

    #[getter]
    fn element_name(&self) -> autosar_data_rs::ElementName {
        self.0.element_name()
    }

    #[getter]
    fn element_type(&self) -> ElementType {
        ElementType(self.0.element_type())
    }

    #[getter]
    fn item_name(&self) -> Option<String> {
        self.0.item_name()
    }

    #[setter]
    fn set_item_name(&self, new_name: &str) -> PyResult<()> {
        match self.0.set_item_name(new_name) {
            Ok(()) => Ok(()),
            Err(error) => Err(AutosarDataError::new_err(error.to_string())),
        }
    }

    #[getter]
    fn is_identifiable(&self) -> bool {
        self.0.is_identifiable()
    }

    #[getter]
    fn is_reference(&self) -> bool {
        self.0.element_type().is_ref()
    }

    #[getter]
    fn path(&self) -> PyResult<String> {
        match self.0.path() {
            Ok(path) => Ok(path),
            Err(error) => Err(AutosarDataError::new_err(error.to_string())),
        }
    }

    #[getter]
    fn model(&self) -> PyResult<AutosarModel> {
        match self.0.model() {
            Ok(model) => Ok(AutosarModel(model)),
            Err(error) => Err(AutosarDataError::new_err(error.to_string())),
        }
    }

    #[getter]
    fn content_type(&self) -> ContentType {
        match self.0.content_type() {
            autosar_data_rs::ContentType::Elements => ContentType::Elements,
            autosar_data_rs::ContentType::CharacterData => ContentType::CharacterData,
            autosar_data_rs::ContentType::Mixed => ContentType::Mixed,
        }
    }

    fn create_sub_element(&self, element_name: autosar_data_rs::ElementName) -> PyResult<Element> {
        match self.0.create_sub_element(element_name) {
            Ok(element) => Ok(Element(element)),
            Err(error) => Err(AutosarDataError::new_err(error.to_string())),
        }
    }

    fn create_sub_element_at(
        &self,
        element_name: autosar_data_rs::ElementName,
        position: usize,
    ) -> PyResult<Element> {
        match self.0.create_sub_element_at(element_name, position) {
            Ok(element) => Ok(Element(element)),
            Err(error) => Err(AutosarDataError::new_err(error.to_string())),
        }
    }

    fn create_named_sub_element(
        &self,
        element_name: autosar_data_rs::ElementName,
        item_name: &str,
    ) -> PyResult<Element> {
        match self.0.create_named_sub_element(element_name, item_name) {
            Ok(element) => Ok(Element(element)),
            Err(error) => Err(AutosarDataError::new_err(error.to_string())),
        }
    }

    fn create_named_sub_element_at(
        &self,
        element_name: autosar_data_rs::ElementName,
        item_name: &str,
        position: usize,
    ) -> PyResult<Element> {
        match self
            .0
            .create_named_sub_element_at(element_name, item_name, position)
        {
            Ok(element) => Ok(Element(element)),
            Err(error) => Err(AutosarDataError::new_err(error.to_string())),
        }
    }

    fn create_copied_sub_element(&self, other: &Element) -> PyResult<Element> {
        match self.0.create_copied_sub_element(&other.0) {
            Ok(element) => Ok(Element(element)),
            Err(error) => Err(AutosarDataError::new_err(error.to_string())),
        }
    }

    fn create_copied_sub_element_at(&self, other: &Element, position: usize) -> PyResult<Element> {
        match self.0.create_copied_sub_element_at(&other.0, position) {
            Ok(element) => Ok(Element(element)),
            Err(error) => Err(AutosarDataError::new_err(error.to_string())),
        }
    }

    fn move_element_here(&self, move_element: &Element) -> PyResult<Element> {
        match self.0.move_element_here(&move_element.0) {
            Ok(element) => Ok(Element(element)),
            Err(error) => Err(AutosarDataError::new_err(error.to_string())),
        }
    }

    fn move_element_here_at(&self, move_element: &Element, position: usize) -> PyResult<Element> {
        match self.0.move_element_here_at(&move_element.0, position) {
            Ok(element) => Ok(Element(element)),
            Err(error) => Err(AutosarDataError::new_err(error.to_string())),
        }
    }

    fn remove_sub_element(&self, sub_element: Element) -> PyResult<()> {
        self.0
            .remove_sub_element(sub_element.0)
            .map_err(|error| AutosarDataError::new_err(error.to_string()))
    }

    #[setter]
    fn set_reference_target(&self, target: Element) -> PyResult<()> {
        self.0
            .set_reference_target(&target.0)
            .map_err(|error| AutosarDataError::new_err(error.to_string()))
    }

    #[getter]
    fn get_reference_target(&self) -> PyResult<Element> {
        match self.0.get_reference_target() {
            Ok(target) => Ok(Element(target)),
            Err(error) => Err(AutosarDataError::new_err(error.to_string())),
        }
    }

    fn get_sub_element(&self, name: autosar_data_rs::ElementName) -> Option<Element> {
        self.0.get_sub_element(name).map(Element)
    }

    #[getter]
    fn sub_elements(&self) -> ElementsIterator {
        ElementsIterator(self.0.sub_elements())
    }

    #[getter]
    fn elements_dfs(&self) -> ElementsDfsIterator {
        ElementsDfsIterator(self.0.elements_dfs())
    }

    #[setter]
    fn set_character_data(&self, chardata: PyObject) -> PyResult<()> {
        let cdata = extract_character_data(chardata)?;
        self.0
            .set_character_data(cdata)
            .map_err(|error| AutosarDataError::new_err(error.to_string()))
    }

    fn remove_character_data(&self) -> PyResult<()> {
        self.0
            .remove_character_data()
            .map_err(|error| AutosarDataError::new_err(error.to_string()))
    }

    #[getter]
    fn character_data(&self) -> Option<PyObject> {
        self.0
            .character_data()
            .map(|cdata| character_data_to_object(&cdata))
    }

    fn insert_character_content_item(&self, chardata: &str, position: usize) -> PyResult<()> {
        self.0
            .insert_character_content_item(chardata, position)
            .map_err(|error| AutosarDataError::new_err(error.to_string()))
    }

    fn remove_character_content_item(&self, position: usize) -> PyResult<()> {
        match self.0.remove_character_content_item(position) {
            Ok(()) => Ok(()),
            Err(error) => Err(AutosarDataError::new_err(error.to_string())),
        }
    }

    #[getter]
    fn content(&self) -> ElementContentIterator {
        ElementContentIterator(self.0.content())
    }

    #[getter]
    fn attributes(&self) -> AttributeIterator {
        AttributeIterator(self.0.attributes())
    }

    fn attribute_value(&self, attrname: autosar_data_rs::AttributeName) -> Option<PyObject> {
        Some(character_data_to_object(&self.0.attribute_value(attrname)?))
    }

    fn set_attribute(
        &self,
        attrname: autosar_data_rs::AttributeName,
        value: PyObject,
    ) -> PyResult<()> {
        let cdata = extract_character_data(value)?;
        self.0
            .set_attribute(attrname, cdata)
            .map_err(|error| AutosarDataError::new_err(error.to_string()))
    }

    fn set_attribute_string(
        &self,
        attrname: autosar_data_rs::AttributeName,
        text: &str,
    ) -> PyResult<()> {
        self.0
            .set_attribute_string(attrname, text)
            .map_err(|error| AutosarDataError::new_err(error.to_string()))
    }

    fn remove_attribute(&self, attrname: autosar_data_rs::AttributeName) -> bool {
        self.0.remove_attribute(attrname)
    }

    fn sort(&self) {
        self.0.sort()
    }

    fn list_valid_sub_elements(&self) -> Vec<(autosar_data_rs::ElementName, bool, bool)> {
        self.0.list_valid_sub_elements()
    }

    #[getter]
    fn file_membership(&self) -> PyResult<PyObject> {
        Python::with_gil(|py| match self.0.file_membership() {
            Ok((local, weak_file_set)) => {
                let file_set: Vec<PyObject> = weak_file_set
                    .iter()
                    .filter_map(|weak| {
                        weak.upgrade()
                            .map(|raw| Py::new(py, ArxmlFile(raw)).unwrap().into_py(py))
                    })
                    .collect();
                let frozenset: &PyFrozenSet = PyFrozenSet::new(py, file_set.iter()).unwrap();
                let pytuple: &PyTuple =
                    PyTuple::new(py, [local.to_object(py), frozenset.to_object(py)].iter());
                Ok(pytuple.to_object(py))
            }
            Err(error) => Err(AutosarDataError::new_err(error.to_string())),
        })
    }

    fn set_file_membership(&self, file_membership: HashSet<ArxmlFile>) {
        self.0.set_file_membership(
            file_membership
                .iter()
                .map(|weak| weak.0.downgrade())
                .collect(),
        )
    }

    fn add_to_file(&self, file: &ArxmlFile) -> PyResult<()> {
        self.0
            .add_to_file(&file.0)
            .map_err(|error| AutosarDataError::new_err(error.to_string()))
    }

    fn remove_from_file(&self, file: &ArxmlFile) -> PyResult<()> {
        self.0
            .remove_from_file(&file.0)
            .map_err(|error| AutosarDataError::new_err(error.to_string()))
    }

    #[getter]
    fn xml_path(&self) -> String {
        self.0.xml_path()
    }
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
