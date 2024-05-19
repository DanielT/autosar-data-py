use crate::*;

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
    fn splittable(&self) -> Vec<AutosarVersion> {
        let versions = expand_version_mask(self.0.splittable());
        versions
            .iter()
            .map(|&ver| AutosarVersion::from(ver))
            .collect()
    }

    #[getter]
    fn std_restriction(&self) -> String {
        format!("{:?}", self.0.std_restriction())
    }

    fn splittable_in(&self, version: AutosarVersion) -> bool {
        self.0.splittable_in(version.into())
    }

    fn reference_dest_value(&self, target: &ElementType) -> Option<String> {
        self.0
            .reference_dest_value(&target.0)
            .map(|enumitem| enumitem.to_string())
    }

    fn find_sub_element(
        &self,
        target_name: &str,
        version_obj: PyObject,
    ) -> PyResult<Option<ElementType>> {
        let version = version_mask_from_any(&version_obj)?;
        let elem_name = get_element_name(target_name)?;
        Ok(self
            .0
            .find_sub_element(elem_name, version)
            .map(|(etype, _)| ElementType(etype)))
    }

    #[getter]
    fn chardata_spec(&self) -> Option<PyObject> {
        self.0.chardata_spec().map(character_data_spec_to_object)
    }

    #[getter]
    fn attributes_spec(&self) -> Vec<AttributeSpec> {
        self.0
            .attribute_spec_iter()
            .map(|(attribute_name, value_spec, required)| AttributeSpec {
                attribute_name: attribute_name.to_string(),
                value_spec,
                required,
            })
            .collect()
    }

    fn find_attribute_spec(&self, attrname_str: &str) -> PyResult<AttributeSpec> {
        let attrname =
            autosar_data_specification::AttributeName::from_str(attrname_str).map_err(|_| {
                pyo3::exceptions::PyTypeError::new_err(format!(
                    "'{attrname_str}' cannot be converted to 'AttributeName'"
                ))
            })?;

        if let Some(attrspec) = self.0.find_attribute_spec(attrname) {
            Ok(AttributeSpec {
                attribute_name: attrname_str.to_owned(),
                value_spec: attrspec.spec,
                required: attrspec.required,
            })
        } else {
            Err(pyo3::exceptions::PyValueError::new_err(format!(
                "'{attrname_str}' is not a valid attribute for this ElementType"
            )))
        }
    }
}

#[pymethods]
impl AttributeSpec {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    #[getter]
    fn value_spec(&self) -> PyObject {
        character_data_spec_to_object(self.value_spec)
    }
}

#[pymethods]
impl CharacterDataTypeEnum {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    fn __str__(&self) -> String {
        format!("CharacterDataType: Enum of [{}]", self.values.join(", "))
    }
}

#[pymethods]
impl CharacterDataTypeFloat {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    fn __str__(&self) -> String {
        "CharacterDataType: Float".to_owned()
    }
}

#[pymethods]
impl CharacterDataTypeRestrictedString {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    fn __str__(&self) -> String {
        format!("CharacterDataType: String matching \"{}\"", self.regex)
    }
}

#[pymethods]
impl CharacterDataTypeString {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    fn __str__(&self) -> String {
        "CharacterDataType: String".to_owned()
    }
}

#[pymethods]
impl CharacterDataTypeUnsignedInt {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    fn __str__(&self) -> String {
        "CharacterDataType: Float".to_owned()
    }
}

fn character_data_spec_to_object(spec: &CharacterDataSpec) -> PyObject {
    Python::with_gil(|py| match spec {
        CharacterDataSpec::Enum { items } => {
            let pytype = Py::new(
                py,
                CharacterDataTypeEnum {
                    values: items.iter().map(|(item, _)| item.to_string()).collect(),
                },
            );
            pytype.unwrap().into_py(py)
        }
        CharacterDataSpec::Pattern {
            regex, max_length, ..
        } => {
            let pytype = Py::new(
                py,
                CharacterDataTypeRestrictedString {
                    regex: regex.to_string(),
                    max_length: *max_length,
                },
            );
            pytype.unwrap().into_py(py)
        }
        CharacterDataSpec::String {
            preserve_whitespace,
            max_length,
        } => {
            let pytype = Py::new(
                py,
                CharacterDataTypeString {
                    preserve_whitespace: *preserve_whitespace,
                    max_length: *max_length,
                },
            );
            pytype.unwrap().into_py(py)
        }
        CharacterDataSpec::UnsignedInteger => Py::new(py, CharacterDataTypeUnsignedInt(()))
            .unwrap()
            .into_py(py),
        CharacterDataSpec::Double => Py::new(py, CharacterDataTypeFloat(())).unwrap().into_py(py),
    })
}
