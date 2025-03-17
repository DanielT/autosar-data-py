from . import abstraction
from . import _autosar_data
from ._autosar_data import *

__doc__ = _autosar_data.__doc__

# explicitly set __all__ to avoid re-exports
__all__ = [
    "abstraction",
    "ElementType",
    "AutosarVersion",
    "AutosarModel",
    "ArxmlFile",
    "Element",
    "IncompatibleAttributeError",
    "IncompatibleAttributeValueError",
    "IncompatibleElementError",
    "ContentType",
    "ElementsDfsIterator",
    "ArxmlFileElementsDfsIterator",
    "ElementContentIterator",
    "ElementsIterator",
    "IdentifiablesIterator",
    "AttributeIterator",
    "Attribute",
    "AttributeSpec",
    "SubElementSpec",
    "ContentMode",
    "ValidSubElementInfo",
    "CharacterDataTypeEnum",
    "CharacterDataTypeFloat",
    "CharacterDataTypeRestrictedString",
    "CharacterDataTypeString",
    "CharacterDataTypeUnsignedInt",
    "check_file",
    "check_buffer",
    "AutosarDataError",
    "__version__",
]
