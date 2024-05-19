use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

use crate::*;
use ::autosar_data as autosar_data_rs;

#[pymethods]
impl Element {
    fn __repr__(&self) -> String {
        format!("{:#?}", self.0)
    }

    fn __str__(&self) -> String {
        self.0.serialize()
    }

    fn __richcmp__(&self, other: &Element, op: pyo3::basic::CompareOp) -> PyResult<bool> {
        match op {
            pyo3::pyclass::CompareOp::Eq => Ok(self.0 == other.0),
            pyo3::pyclass::CompareOp::Ne => Ok(self.0 != other.0),
            pyo3::pyclass::CompareOp::Lt => Err(pyo3::exceptions::PyTypeError::new_err("'<' is not supported between instances of 'builtins.Element' and 'builtins.Element'")),
            pyo3::pyclass::CompareOp::Le => Err(pyo3::exceptions::PyTypeError::new_err("'<=' is not supported between instances of 'builtins.Element' and 'builtins.Element'")),
            pyo3::pyclass::CompareOp::Gt => Err(pyo3::exceptions::PyTypeError::new_err("'>' is not supported between instances of 'builtins.Element' and 'builtins.Element'")),
            pyo3::pyclass::CompareOp::Ge => Err(pyo3::exceptions::PyTypeError::new_err("'>=' is not supported between instances of 'builtins.Element' and 'builtins.Element'")),
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
    fn named_parent(&self) -> PyResult<Option<Element>> {
        match self.0.named_parent() {
            Ok(Some(parent)) => Ok(Some(Element(parent))),
            Ok(None) => Ok(None),
            Err(error) => Err(AutosarDataError::new_err(error.to_string())),
        }
    }

    #[getter]
    fn element_name(&self) -> String {
        self.0.element_name().to_string()
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

    fn create_sub_element(&self, name_str: &str, position: Option<usize>) -> PyResult<Element> {
        let element_name = get_element_name(name_str)?;
        if let Some(position) = position {
            match self.0.create_sub_element_at(element_name, position) {
                Ok(element) => Ok(Element(element)),
                Err(error) => Err(AutosarDataError::new_err(error.to_string())),
            }
        } else {
            match self.0.create_sub_element(element_name) {
                Ok(element) => Ok(Element(element)),
                Err(error) => Err(AutosarDataError::new_err(error.to_string())),
            }
        }
    }

    fn create_named_sub_element(
        &self,
        name_str: &str,
        item_name: &str,
        position: Option<usize>,
    ) -> PyResult<Element> {
        let element_name = get_element_name(name_str)?;
        if let Some(position) = position {
            match self
                .0
                .create_named_sub_element_at(element_name, item_name, position)
            {
                Ok(element) => Ok(Element(element)),
                Err(error) => Err(AutosarDataError::new_err(error.to_string())),
            }
        } else {
            match self.0.create_named_sub_element(element_name, item_name) {
                Ok(element) => Ok(Element(element)),
                Err(error) => Err(AutosarDataError::new_err(error.to_string())),
            }
        }
    }

    fn get_or_create_sub_element(&self, name_str: &str) -> PyResult<Element> {
        let element_name = get_element_name(name_str)?;
        match self.0.get_or_create_sub_element(element_name) {
            Ok(element) => Ok(Element(element)),
            Err(error) => Err(AutosarDataError::new_err(error.to_string())),
        }
    }

    fn get_or_create_named_sub_element(
        &self,
        name_str: &str,
        item_name: &str,
    ) -> PyResult<Element> {
        let element_name = get_element_name(name_str)?;
        match self
            .0
            .get_or_create_named_sub_element(element_name, item_name)
        {
            Ok(element) => Ok(Element(element)),
            Err(error) => Err(AutosarDataError::new_err(error.to_string())),
        }
    }

    fn create_copied_sub_element(
        &self,
        other: &Element,
        position: Option<usize>,
    ) -> PyResult<Element> {
        if let Some(position) = position {
            match self.0.create_copied_sub_element_at(&other.0, position) {
                Ok(element) => Ok(Element(element)),
                Err(error) => Err(AutosarDataError::new_err(error.to_string())),
            }
        } else {
            match self.0.create_copied_sub_element(&other.0) {
                Ok(element) => Ok(Element(element)),
                Err(error) => Err(AutosarDataError::new_err(error.to_string())),
            }
        }
    }

    fn move_element_here(
        &self,
        move_element: &Element,
        position: Option<usize>,
    ) -> PyResult<Element> {
        if let Some(position) = position {
            match self.0.move_element_here_at(&move_element.0, position) {
                Ok(element) => Ok(Element(element)),
                Err(error) => Err(AutosarDataError::new_err(error.to_string())),
            }
        } else {
            match self.0.move_element_here(&move_element.0) {
                Ok(element) => Ok(Element(element)),
                Err(error) => Err(AutosarDataError::new_err(error.to_string())),
            }
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

    fn get_sub_element(&self, name_str: &str) -> PyResult<Option<Element>> {
        let element_name = get_element_name(name_str)?;
        Ok(self.0.get_sub_element(element_name).map(Element))
    }

    fn get_sub_element_at(&self, position: usize) -> Option<Element> {
        self.0.get_sub_element_at(position).map(Element)
    }

    fn get_named_sub_element(&self, item_name: String) -> Option<Element> {
        let item_name = Some(item_name);
        self.0
            .sub_elements()
            .find(|se| se.item_name() == item_name)
            .map(Element)
    }

    fn get_bsw_sub_element(&self, definition_ref: String) -> Option<Element> {
        self.0
            .sub_elements()
            .find(|se| {
                se.get_sub_element(autosar_data_rs::ElementName::DefinitionRef)
                    .and_then(|defref| defref.character_data())
                    .and_then(|cdata| cdata.string_value())
                    .map(|strval| {
                        strval == definition_ref
                            || strval.split('/').last().unwrap_or("") == definition_ref
                    })
                    .unwrap_or(false)
            })
            .map(Element)
    }

    #[getter]
    fn position(&self) -> Option<usize> {
        self.0.position()
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
        let spec = self
            .0
            .element_type()
            .chardata_spec()
            .ok_or(AutosarDataError::new_err(
                autosar_data_rs::AutosarDataError::IncorrectContentType.to_string(),
            ))?;
        let cdata = extract_character_data(spec, &chardata)?;
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
        self.0
            .remove_character_content_item(position)
            .map_err(|error| AutosarDataError::new_err(error.to_string()))
    }

    #[getter]
    fn content_item_count(&self) -> usize {
        self.0.content_item_count()
    }

    #[getter]
    fn content(&self) -> ElementContentIterator {
        ElementContentIterator(self.0.content())
    }

    #[getter]
    fn attributes(&self) -> AttributeIterator {
        AttributeIterator(self.0.attributes())
    }

    pub(crate) fn attribute_value(&self, attrname_str: &str) -> PyResult<Option<PyObject>> {
        let attrname = get_attribute_name(attrname_str)?;
        Ok(self
            .0
            .attribute_value(attrname)
            .map(|cdata| character_data_to_object(&cdata)))
    }

    pub(crate) fn set_attribute(&self, attrname_str: &str, value: PyObject) -> PyResult<()> {
        let attrname = get_attribute_name(attrname_str)?;
        let attrspec = self.0.element_type().find_attribute_spec(attrname).ok_or(
            AutosarDataError::new_err(
                autosar_data_rs::AutosarDataError::InvalidAttribute.to_string(),
            ),
        )?;
        let cdata = extract_character_data(attrspec.spec, &value)?;
        self.0
            .set_attribute(attrname, cdata)
            .map_err(|error| AutosarDataError::new_err(error.to_string()))
    }

    fn remove_attribute(&self, attrname_str: &str) -> PyResult<bool> {
        let attrname = get_attribute_name(attrname_str)?;
        Ok(self.0.remove_attribute(attrname))
    }

    fn sort(&self) {
        self.0.sort()
    }

    fn list_valid_sub_elements(&self) -> Vec<ValidSubElementInfo> {
        self.0
            .list_valid_sub_elements()
            .iter()
            .map(
                |autosar_data_rs::ValidSubElementInfo {
                     element_name,
                     is_named,
                     is_allowed,
                 }| ValidSubElementInfo {
                    element_name: element_name.to_string(),
                    is_named: *is_named,
                    is_allowed: *is_allowed,
                },
            )
            .collect()
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
                let frozenset = PyFrozenSet::new_bound(py, file_set.iter()).unwrap();
                let pytuple =
                    PyTuple::new_bound(py, [local.to_object(py), frozenset.to_object(py)].iter());
                Ok(pytuple.to_object(py))
            }
            Err(error) => Err(AutosarDataError::new_err(error.to_string())),
        })
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

    #[getter]
    fn min_version(&self) -> PyResult<AutosarVersion> {
        match self.0.min_version() {
            Ok(ver) => Ok(ver.into()),
            Err(error) => Err(AutosarDataError::new_err(error.to_string())),
        }
    }

    #[getter]
    fn comment(&self) -> Option<String> {
        self.0.comment()
    }

    #[setter]
    fn set_comment(&self, opt_comment: Option<String>) {
        self.0.set_comment(opt_comment);
    }
}
