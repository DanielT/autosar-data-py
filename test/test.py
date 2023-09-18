from autosar_data import *
import pytest
import os

def test_others() -> None:
    model = AutosarModel()
    model.create_file("file")

    # content type - __str__ / __repr__
    ct_str = model.root_element.content_type.__str__()
    ct_repr = model.root_element.content_type.__repr__()
    assert not ct_str is None
    assert not ct_repr is None

    # ElementType
    assert model.root_element.element_type.splittable_in(AutosarVersion.AUTOSAR_00042) == True
    # find a sub element for a particular version
    ar_pkg_type = model.root_element.element_type.find_sub_element("AR-PACKAGES", AutosarVersion.AUTOSAR_4_0_1)
    assert ar_pkg_type.splittable_in(AutosarVersion.AUTOSAR_00042) == True
    # find a sub element for multiple versions
    ar_pkg_type = model.root_element.element_type.find_sub_element("AR-PACKAGES", [AutosarVersion.AUTOSAR_4_0_1, AutosarVersion.AUTOSAR_4_0_2])
    assert ar_pkg_type.splittable_in(AutosarVersion.AUTOSAR_00042) == True
    with pytest.raises(TypeError):
        model.root_element.element_type.find_sub_element("AR-PACKAGES", "wrong type")
    with pytest.raises(TypeError):
        model.root_element.element_type.find_sub_element("AR-PACKAGES", ["wrong type"])
    with pytest.raises(AutosarDataError):
        model.root_element.element_type.find_sub_element("nonexistent", AutosarVersion.AUTOSAR_4_0_1)
    
    assert AutosarVersion.AUTOSAR_4_0_1 in ar_pkg_type.splittable

    et_str = ar_pkg_type.__str__()
    et_repr = ar_pkg_type.__repr__()
    assert not et_str is None
    assert not et_repr is None

    assert isinstance(__version__, str)

    # invalid items
    with pytest.raises(AutosarDataError):
        model.root_element.create_sub_element("bla")
    
    with pytest.raises(AutosarDataError):
        model.root_element.set_attribute("bla", 0)


def test_check_arxml(tmp_path: str) -> None:
    model = AutosarModel()
    filename1 = os.path.join(tmp_path, "test.arxml")
    file = model.create_file(filename1)
    model.write()
    assert check_file(filename1) == True
    assert check_file("no_such_file") == False

    text = file.serialize()
    assert check_buffer(text) == True
    assert check_buffer(text.encode('utf-8')) == True
    assert check_buffer(b'abcdef') == False
    with pytest.raises(TypeError):
        check_buffer(file)