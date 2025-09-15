use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

use crate::*;
use ::autosar_data as autosar_data_rs;
use autosar_data_rs::CompatibilityError;
use autosar_data_specification::expand_version_mask;
use pyo3::IntoPyObjectExt;

#[pymethods]
impl ArxmlFile {
    fn __repr__(&self) -> String {
        format!("{:#?}", self.0)
    }

    fn __str__(&self) -> PyResult<String> {
        self.serialize()
    }

    fn __richcmp__(&self, other: &ArxmlFile, op: pyo3::basic::CompareOp) -> PyResult<bool> {
        match op {
            pyo3::pyclass::CompareOp::Eq => Ok(self.0 == other.0),
            pyo3::pyclass::CompareOp::Ne => Ok(self.0 != other.0),
            pyo3::pyclass::CompareOp::Lt => Err(pyo3::exceptions::PyTypeError::new_err(
                "'<' is not supported between instances of 'builtins.Element' and 'builtins.Element'",
            )),
            pyo3::pyclass::CompareOp::Le => Err(pyo3::exceptions::PyTypeError::new_err(
                "'<=' is not supported between instances of 'builtins.Element' and 'builtins.Element'",
            )),
            pyo3::pyclass::CompareOp::Gt => Err(pyo3::exceptions::PyTypeError::new_err(
                "'>' is not supported between instances of 'builtins.Element' and 'builtins.Element'",
            )),
            pyo3::pyclass::CompareOp::Ge => Err(pyo3::exceptions::PyTypeError::new_err(
                "'>=' is not supported between instances of 'builtins.Element' and 'builtins.Element'",
            )),
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
    fn version(&self) -> AutosarVersion {
        self.0.version().into()
    }

    #[setter]
    fn set_version(&self, version: AutosarVersion) -> PyResult<()> {
        self.0
            .set_version(version.into())
            .map_err(|error| AutosarDataError::new_err(error.to_string()))
    }

    /// Check if the data in the ARXML file is compatible with the given target version
    #[pyo3(signature = (target_version, /))]
    #[pyo3(text_signature = "(self, target_version: AutosarVersion, /)")]
    fn check_version_compatibility(
        &self,
        target_version: AutosarVersion,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let (error_list, _) = self.0.check_version_compatibility(target_version.into());
        let mut out_list = Vec::with_capacity(error_list.len());
        for compat_err in error_list {
            let pyobj = match compat_err {
                CompatibilityError::IncompatibleAttribute {
                    element,
                    attribute,
                    version_mask,
                } => {
                    let errobj = IncompatibleAttributeError {
                        element: Element(element.clone()),
                        attribute: attribute.to_string(),
                        allowed_versions: expand_version_mask(version_mask)
                            .iter()
                            .map(|&v| v.into())
                            .collect(),
                        target_version,
                    };
                    Python::attach(|py| errobj.into_py_any(py))?
                }
                CompatibilityError::IncompatibleAttributeValue {
                    element,
                    attribute,
                    attribute_value,
                    version_mask,
                } => {
                    let errobj = IncompatibleAttributeValueError {
                        element: Element(element.clone()),
                        attribute: attribute.to_string(),
                        attribute_value: attribute_value.clone(),
                        allowed_versions: expand_version_mask(version_mask)
                            .iter()
                            .map(|&v| v.into())
                            .collect(),
                        target_version,
                    };
                    Python::attach(|py| errobj.into_py_any(py))?
                }
                CompatibilityError::IncompatibleElement {
                    element,
                    version_mask,
                } => {
                    let errobj = IncompatibleElementError {
                        element: Element(element.clone()),
                        allowed_versions: expand_version_mask(version_mask)
                            .iter()
                            .map(|&v| v.into())
                            .collect(),
                        target_version,
                    };
                    Python::attach(|py| errobj.into_py_any(py))?
                }
            };
            out_list.push(pyobj);
        }
        Ok(out_list)
    }

    /// Get the autosar model that is built from the ARXML files
    #[getter]
    fn model(&self) -> PyResult<AutosarModel> {
        match self.0.model() {
            Ok(model) => Ok(AutosarModel(model)),
            Err(error) => PyResult::Err(AutosarDataError::new_err(error.to_string())),
        }
    }

    #[getter]
    fn elements_dfs(&self) -> ElementsDfsIterator {
        ElementsDfsIterator::new(self.0.elements_dfs().filter_map(|(depth, elem)| {
            Python::attach(|py| (depth, Element(elem)).into_py_any(py).ok())
        }))
    }

    fn elements_dfs_with_max_depth(&self, max_depth: usize) -> ElementsDfsIterator {
        ElementsDfsIterator::new(self.0.elements_dfs_with_max_depth(max_depth).filter_map(
            |(depth, elem)| Python::attach(|py| (depth, Element(elem)).into_py_any(py).ok()),
        ))
    }

    /// Serialize the ARXML file to a string
    fn serialize(&self) -> PyResult<String> {
        match self.0.serialize() {
            Ok(text) => Ok(text),
            Err(error) => Err(AutosarDataError::new_err(error.to_string())),
        }
    }

    /// get the "xml_standalone" attribute from the header of the ARXML file
    #[getter]
    fn xml_standalone(&self) -> Option<bool> {
        self.0.xml_standalone()
    }
}
