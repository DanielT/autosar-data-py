from autosar_data import *
import pytest

def test_others():
    model = AutosarModel()

    # content type - __str__ / __repr__
    ct_str = model.root_element.content_type.__str__()
    ct_repr = model.root_element.content_type.__repr__()
    assert not ct_str is None
    assert not ct_repr is None

    # ElementType
    assert model.root_element.element_type.splittable_in(AutosarVersion.Autosar_00042) == False
    ar_pkg_type = model.root_element.element_type.find_sub_element("AR-PACKAGES", 1)
    assert ar_pkg_type.splittable_in(AutosarVersion.Autosar_00042) == True
    et_str = ar_pkg_type.__str__()
    et_repr = ar_pkg_type.__repr__()
    assert not et_str is None
    assert not et_repr is None

    # invalid items
    with pytest.raises(AutosarDataError):
        model.root_element.create_sub_element("bla")
    
    with pytest.raises(AutosarDataError):
        model.root_element.set_attribute("bla", 0)
