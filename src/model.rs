use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;

use crate::*;
use ::autosar_data as autosar_data_rs;

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

    fn create_file(&self, filename: &str, version: AutosarVersion) -> PyResult<ArxmlFile> {
        match self.0.create_file(filename, version.into()) {
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
        match self.0.load_buffer(buffer.as_bytes(), filename, strict) {
            Ok((file, warn)) => {
                let warnstrings: Vec<String> = warn.iter().map(|w| w.to_string()).collect();
                Ok((ArxmlFile(file), warnstrings))
            }
            Err(error) => PyResult::Err(AutosarDataError::new_err(error.to_string())),
        }
    }

    fn load_file(&self, filename: &str, strict: bool) -> PyResult<(ArxmlFile, Vec<String>)> {
        match self.0.load_file(filename, strict) {
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
