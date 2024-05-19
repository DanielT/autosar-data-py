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

    fn __richcmp__(&self, other: &AutosarModel, op: pyo3::basic::CompareOp) -> PyResult<bool> {
        match op {
            pyo3::pyclass::CompareOp::Eq => Ok(self.0 == other.0),
            pyo3::pyclass::CompareOp::Ne => Ok(self.0 != other.0),
            pyo3::pyclass::CompareOp::Lt => Err(pyo3::exceptions::PyTypeError::new_err("'<' is not supported between instances of 'builtins.AutosarModel' and 'builtins.AutosarModel'")),
            pyo3::pyclass::CompareOp::Le => Err(pyo3::exceptions::PyTypeError::new_err("'<=' is not supported between instances of 'builtins.AutosarModel' and 'builtins.AutosarModel'")),
            pyo3::pyclass::CompareOp::Gt => Err(pyo3::exceptions::PyTypeError::new_err("'>' is not supported between instances of 'builtins.AutosarModel' and 'builtins.AutosarModel'")),
            pyo3::pyclass::CompareOp::Ge => Err(pyo3::exceptions::PyTypeError::new_err("'>=' is not supported between instances of 'builtins.AutosarModel' and 'builtins.AutosarModel'")),
        }
    }

    fn __hash__(&self) -> isize {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish() as isize
    }

    /// create a new file in the model
    #[pyo3(signature = (filename, version=AutosarVersion::Latest))]
    fn create_file(&self, filename: &str, version: AutosarVersion) -> PyResult<ArxmlFile> {
        match self.0.create_file(filename, version.into()) {
            Ok(file) => Ok(ArxmlFile(file)),
            Err(error) => PyResult::Err(AutosarDataError::new_err(error.to_string())),
        }
    }

    /// load a buffer (string) as arxml
    #[pyo3(signature = (buffer, filename, strict=false))]
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

    /// load a file as arxml
    #[pyo3(signature = (filename, strict=false))]
    fn load_file(&self, filename: &str, strict: bool) -> PyResult<(ArxmlFile, Vec<String>)> {
        match self.0.load_file(filename, strict) {
            Ok((file, warn)) => {
                let warnstrings: Vec<String> = warn.iter().map(|w| w.to_string()).collect();
                Ok((ArxmlFile(file), warnstrings))
            }
            Err(error) => PyResult::Err(AutosarDataError::new_err(error.to_string())),
        }
    }

    /// remove a file from the model. Any elements belonging exclusively to that file will also be removed.
    fn remove_file(&self, file: &ArxmlFile) {
        self.0.remove_file(&file.0);
    }

    /// serialize all files individually, to generate a dict(filename, serialized content),
    fn serialize_files(&self) -> HashMap<String, String> {
        let hm_orig: HashMap<std::path::PathBuf, String> = self.0.serialize_files();
        let mut hm_out = HashMap::<String, String>::new();
        for (k, v) in hm_orig {
            hm_out.insert(String::from(k.to_string_lossy()), v);
        }
        hm_out
    }

    /// write all files in the model to disk
    fn write(&self) -> PyResult<()> {
        self.0
            .write()
            .map_err(|error| AutosarDataError::new_err(error.to_string()))
    }

    #[getter]
    /// a list of ArxmlFile objects containing all files in the model
    fn files(&self) -> Vec<ArxmlFile> {
        self.0.files().map(ArxmlFile).collect()
    }

    #[getter]
    /// The root element of the model, <AUTOSAR>
    fn root_element(&self) -> Element {
        Element(self.0.root_element())
    }

    /// get an identifiable element in the model by its Autosar path
    fn get_element_by_path(&self, path: &str) -> Option<Element> {
        self.0.get_element_by_path(path).map(Element)
    }

    #[getter]
    /// depth first dearch iterator over all elements in the model, regardless of their association with a file
    fn elements_dfs(&self) -> ElementsDfsIterator {
        ElementsDfsIterator(self.0.elements_dfs())
    }

    ///sort the entire model in place. Takes all ordering constraints into account.
    fn sort(&self) {
        self.0.sort();
    }

    #[getter]
    /// List of all paths of identifiable elements in the model
    fn identifiable_elements(&self) -> IdentifiablesIterator {
        IdentifiablesIterator(self.0.identifiable_elements())
    }

    /// get all reference elements which refer to the given Autosar path
    fn get_references_to(&self, target_path: &str) -> Vec<Element> {
        self.0
            .get_references_to(target_path)
            .iter()
            .filter_map(|weak| weak.upgrade().map(Element))
            .collect()
    }

    /// check all references in the model and return a list of elements containing invalid references
    fn check_references(&self) -> Vec<Element> {
        self.0
            .check_references()
            .iter()
            .filter_map(|weak| weak.upgrade().map(Element))
            .collect()
    }

    /// duplicate the model, creating a new independent copy
    fn duplicate(&self) -> PyResult<AutosarModel> {
        match self.0.duplicate() {
            Ok(model) => Ok(AutosarModel(model)),
            Err(error) => PyResult::Err(AutosarDataError::new_err(error.to_string())),
        }
    }
}
