use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

use crate::*;
use ::autosar_data as autosar_data_rs;
use autosar_data_rs::CompatibilityError;
use autosar_data_specification::expand_version_mask;

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

    fn check_version_compatibility(&self, target_version: AutosarVersion) -> Vec<PyObject> {
        Python::with_gil(|py| {
            self.0
                .check_version_compatibility(target_version.into())
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
                                attribute: attribute.to_string(),
                                allowed_versions: expand_version_mask(*version_mask)
                                    .iter()
                                    .map(|&v| v.into())
                                    .collect(),
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
                                attribute: attribute.to_string(),
                                attribute_value: attribute_value.to_owned(),
                                allowed_versions: expand_version_mask(*version_mask)
                                    .iter()
                                    .map(|&v| v.into())
                                    .collect(),
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
                                allowed_versions: expand_version_mask(*version_mask)
                                    .iter()
                                    .map(|&v| v.into())
                                    .collect(),
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
