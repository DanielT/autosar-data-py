use crate::{
    Element,
    abstraction::{AutosarAbstractionError, ByteOrder, abstraction_err_to_pyerr},
};
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};
use pyo3::prelude::*;

//##################################################################

/// `SwBaseType` is a basic data type.
///
/// It is used to define the data types of signals and variables.
#[pyclass(frozen, eq, module = "autosar.abstraction.datatype")]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SwBaseType(pub(crate) autosar_data_abstraction::datatype::SwBaseType);

#[pymethods]
impl SwBaseType {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::datatype::SwBaseType::try_from(element.0.clone()) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    #[setter]
    fn set_name(&self, name: &str) -> PyResult<()> {
        self.0.set_name(name).map_err(abstraction_err_to_pyerr)
    }

    #[getter]
    fn name(&self) -> Option<String> {
        self.0.name()
    }

    #[getter]
    fn element(&self) -> Element {
        Element(self.0.element().clone())
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self.0)
    }

    /// set the base type size (in bits) of the `SwBaseType`
    #[setter]
    fn set_bit_length(&self, bit_length: u32) -> PyResult<()> {
        self.0
            .set_bit_length(bit_length)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the bit length of the `SwBaseType`
    #[getter]
    fn bit_length(&self) -> Option<u32> {
        self.0.bit_length()
    }

    /// set the base type encoding of the `SwBaseType`
    #[setter]
    fn set_base_type_encoding(&self, base_type_encoding: BaseTypeEncoding) -> PyResult<()> {
        self.0
            .set_base_type_encoding(base_type_encoding.into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the base type encoding of the `SwBaseType`
    #[getter]
    fn base_type_encoding(&self) -> Option<BaseTypeEncoding> {
        self.0.base_type_encoding().map(std::convert::Into::into)
    }

    /// set the byte order of the `SwBaseType`
    ///
    /// The byte order is platform specific and should only be set when it is really needed.
    #[setter]
    fn set_byte_order(&self, byte_order: ByteOrder) -> PyResult<()> {
        self.0
            .set_byte_order(byte_order.into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the byte order of the `SwBaseType`
    #[getter]
    fn byte_order(&self) -> Option<ByteOrder> {
        self.0.byte_order().map(std::convert::Into::into)
    }

    /// set the memory alignment of the `SwBaseType`
    ///
    /// The memory alignment describes the slignement in bits. Example: 8 means that the type is aligned to a byte.
    /// Since the memory alignment is platform specific, it should only be set when it is really needed.
    #[setter]
    fn set_mem_alignment(&self, mem_alignment: u32) -> PyResult<()> {
        self.0
            .set_mem_alignment(mem_alignment)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the memory alignment of the `SwBaseType`
    #[getter]
    fn mem_alignment(&self) -> Option<u32> {
        self.0.mem_alignment()
    }

    /// set the native declaration of the `SwBaseType`
    ///
    /// The native declaration is a string that represents the type in the native programming language.
    #[setter]
    fn set_native_declaration(&self, native_declaration: &str) -> PyResult<()> {
        self.0
            .set_native_declaration(native_declaration)
            .map_err(abstraction_err_to_pyerr)
    }

    /// get the native declaration of the `SwBaseType`
    #[getter]
    fn native_declaration(&self) -> Option<String> {
        self.0.native_declaration()
    }
}

//##################################################################

/// `BaseTypeEncoding` describes the encoding of a basic data type.
#[pyclass(frozen, eq, eq_int, module = "autosar.abstraction.datatype")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BaseTypeEncoding {
    /// `OnesComplement` is used for signed integers
    OnesComplement,
    /// `TwosComplement` is used for signed integers
    TwosComplement,
    /// `SignMagnitude` is used for signed integers
    SignMagnitude,
    /// `BcdPacked` is used for packed binary coded decimals
    BcdPacked,
    /// `BcdUnpacked` is used for unpacked binary coded decimals
    BcdUnpacked,
    /// `DspFractional` is used for values in a digital signal processor
    DspFractional,
    /// Ieee754 is used for floating point numbers
    Ieee754,
    /// encoding: `Iso8859_1` is used for strings
    Iso8859_1,
    /// encoding: `Iso8859_2` is used for strings
    Iso8859_2,
    /// encoding: Windows1252 is used for strings
    Windows1252,
    /// encoding: Utf8 is used for strings
    Utf8,
    /// encoding: Utf16 is used for strings - byte order must be specified
    Utf16,
    /// encoding: Ucs2 is used for strings
    Ucs2,
    /// encoding: Boolean is used for boolean values
    Boolean,
    /// encoding: Void is used for C void types
    Void,
    /// encoding: NoEncoding is used for unsigned integers
    NoEncoding, // originally None, but None is a reserved keyword in Python
}

impl From<autosar_data_abstraction::datatype::BaseTypeEncoding> for BaseTypeEncoding {
    fn from(value: autosar_data_abstraction::datatype::BaseTypeEncoding) -> Self {
        match value {
            autosar_data_abstraction::datatype::BaseTypeEncoding::OnesComplement => {
                Self::OnesComplement
            }
            autosar_data_abstraction::datatype::BaseTypeEncoding::TwosComplement => {
                Self::TwosComplement
            }
            autosar_data_abstraction::datatype::BaseTypeEncoding::SignMagnitude => {
                Self::SignMagnitude
            }
            autosar_data_abstraction::datatype::BaseTypeEncoding::BcdPacked => Self::BcdPacked,
            autosar_data_abstraction::datatype::BaseTypeEncoding::BcdUnpacked => Self::BcdUnpacked,
            autosar_data_abstraction::datatype::BaseTypeEncoding::DspFractional => {
                Self::DspFractional
            }
            autosar_data_abstraction::datatype::BaseTypeEncoding::Ieee754 => Self::Ieee754,
            autosar_data_abstraction::datatype::BaseTypeEncoding::Iso8859_1 => Self::Iso8859_1,
            autosar_data_abstraction::datatype::BaseTypeEncoding::Iso8859_2 => Self::Iso8859_2,
            autosar_data_abstraction::datatype::BaseTypeEncoding::Windows1252 => Self::Windows1252,
            autosar_data_abstraction::datatype::BaseTypeEncoding::Utf8 => Self::Utf8,
            autosar_data_abstraction::datatype::BaseTypeEncoding::Utf16 => Self::Utf16,
            autosar_data_abstraction::datatype::BaseTypeEncoding::Ucs2 => Self::Ucs2,
            autosar_data_abstraction::datatype::BaseTypeEncoding::Boolean => Self::Boolean,
            autosar_data_abstraction::datatype::BaseTypeEncoding::Void => Self::Void,
            autosar_data_abstraction::datatype::BaseTypeEncoding::None => Self::NoEncoding,
        }
    }
}

impl From<BaseTypeEncoding> for autosar_data_abstraction::datatype::BaseTypeEncoding {
    fn from(value: BaseTypeEncoding) -> Self {
        match value {
            BaseTypeEncoding::OnesComplement => {
                autosar_data_abstraction::datatype::BaseTypeEncoding::OnesComplement
            }
            BaseTypeEncoding::TwosComplement => {
                autosar_data_abstraction::datatype::BaseTypeEncoding::TwosComplement
            }
            BaseTypeEncoding::SignMagnitude => {
                autosar_data_abstraction::datatype::BaseTypeEncoding::SignMagnitude
            }
            BaseTypeEncoding::BcdPacked => {
                autosar_data_abstraction::datatype::BaseTypeEncoding::BcdPacked
            }
            BaseTypeEncoding::BcdUnpacked => {
                autosar_data_abstraction::datatype::BaseTypeEncoding::BcdUnpacked
            }
            BaseTypeEncoding::DspFractional => {
                autosar_data_abstraction::datatype::BaseTypeEncoding::DspFractional
            }
            BaseTypeEncoding::Ieee754 => {
                autosar_data_abstraction::datatype::BaseTypeEncoding::Ieee754
            }
            BaseTypeEncoding::Iso8859_1 => {
                autosar_data_abstraction::datatype::BaseTypeEncoding::Iso8859_1
            }
            BaseTypeEncoding::Iso8859_2 => {
                autosar_data_abstraction::datatype::BaseTypeEncoding::Iso8859_2
            }
            BaseTypeEncoding::Windows1252 => {
                autosar_data_abstraction::datatype::BaseTypeEncoding::Windows1252
            }
            BaseTypeEncoding::Utf8 => autosar_data_abstraction::datatype::BaseTypeEncoding::Utf8,
            BaseTypeEncoding::Utf16 => autosar_data_abstraction::datatype::BaseTypeEncoding::Utf16,
            BaseTypeEncoding::Ucs2 => autosar_data_abstraction::datatype::BaseTypeEncoding::Ucs2,
            BaseTypeEncoding::Boolean => {
                autosar_data_abstraction::datatype::BaseTypeEncoding::Boolean
            }
            BaseTypeEncoding::Void => autosar_data_abstraction::datatype::BaseTypeEncoding::Void,
            BaseTypeEncoding::NoEncoding => {
                autosar_data_abstraction::datatype::BaseTypeEncoding::None
            }
        }
    }
}
