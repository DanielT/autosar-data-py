from . import abstraction
from . import _autosar_data
from ._autosar_data import *

__doc__ = _autosar_data.__doc__
if hasattr(_autosar_data, "__all__"):
    __all__ = ["abstraction"] + _autosar_data.__all__
    __all__.remove("_abstraction")
