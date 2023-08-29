from autosar_data import *
import pytest

def test_arxlfile_basic() -> None:
    model = AutosarModel()

    file1 = model.create_file("filename1.arxml", AutosarVersion.AUTOSAR_00051)
    file2 = model.create_file("filename2.arxml", AutosarVersion.AUTOSAR_00051)
    assert isinstance(file1, ArxmlFile)
    assert isinstance(file2, ArxmlFile)

    # the ArxmlFile object in Python is a reference, and references are only equal if they refer to the same file
    assert file1 != file2
    assert file1 == model.files[0]
    assert file2 == model.files[1]

    # each ArxmlFile is part of a model
    assert file1.model == model
    assert file2.model == model

    # change the filename
    file1.filename = "foo"
    assert model.files[0].filename == "foo"

    # can't have two files with the same name in one model
    with pytest.raises(AutosarDataError):
        file1.filename = file2.filename
    
    # each file has a version
    assert file1.version == AutosarVersion.AUTOSAR_00051
    file1.version = AutosarVersion.AUTOSAR_4_3_0
    assert file1.version == AutosarVersion.AUTOSAR_4_3_0

    # ArxmlFile has __str__ and __repr__
    arxmlfile_repr = file1.__repr__()
    assert not arxmlfile_repr is None
    arxmlfile_str = file1.__str__()
    assert not arxmlfile_str is None
    assert arxmlfile_repr != arxmlfile_str

    # ArxmlFile has __hash__
    fileset = set([file1, file2])
    assert len(fileset) == 2

    # a file can be removed from the model
    model.remove_file(file2)
    assert not file2 in set(model.files)

    # files that were loaded from disk might have an xml_standalone attribute
    # this is not used while processing the file and is only preserved
    assert file1.xml_standalone is None

    # if the model is dropped, then the file's reference to the model becomes invalid
    model = None
    with pytest.raises(AutosarDataError):
        print(file1.model)
    # other operations also require a valid model
    with pytest.raises(AutosarDataError):
        print(file1.serialize())


def test_check_version_compatibility() -> None:
    model = AutosarModel()

    file1 = model.create_file("filename", AutosarVersion.AUTOSAR_00050)
    el_elements = model.root_element \
        .create_sub_element("AR-PACKAGES") \
        .create_named_sub_element("AR-PACKAGE", "Pkg") \
        .create_sub_element("ELEMENTS")
    el_acl_object_set = el_elements.create_named_sub_element("ACL-OBJECT-SET", "AclObjectSet")
    el_short_name = el_acl_object_set.get_sub_element("SHORT-NAME")
    el_short_name.set_attribute("BLUEPRINT-VALUE", "xyz")
    el_blueprint_ref = el_acl_object_set \
        .create_sub_element("DERIVED-FROM-BLUEPRINT-REFS") \
        .create_sub_element("DERIVED-FROM-BLUEPRINT-REF")
    el_blueprint_ref.set_attribute("DEST", "ABSTRACT-IMPLEMENTATION-DATA-TYPE")
    el_adaptive_sw_component_type = el_elements \
        .create_named_sub_element("ADAPTIVE-APPLICATION-SW-COMPONENT-TYPE", "AdaptiveApplicationSwComponentType")
    
    with pytest.raises(AutosarDataError):
        file1.version = AutosarVersion.AUTOSAR_4_3_0 # fails because there are compatibility problems

    compat_problems = file1.check_version_compatibility(AutosarVersion.AUTOSAR_4_3_0)
    assert len(compat_problems) == 3
    assert isinstance(compat_problems[0], IncompatibleAttributeError)
    assert isinstance(compat_problems[1], IncompatibleAttributeValueError)
    assert isinstance(compat_problems[2], IncompatibleElementError)

    # IncompatibleAttributeError
    assert compat_problems[0].element == el_short_name
    assert compat_problems[0].attribute == "BLUEPRINT-VALUE"
    error_str = compat_problems[0].__str__()
    error_repr = compat_problems[0].__repr__()
    assert not error_str is None
    assert not error_repr is None

    # IncompatibleAttributeValueError
    assert compat_problems[1].element == el_blueprint_ref
    assert compat_problems[1].attribute == "DEST"
    assert compat_problems[1].attribute_value == "ABSTRACT-IMPLEMENTATION-DATA-TYPE"
    error_str = compat_problems[1].__str__()
    error_repr = compat_problems[1].__repr__()
    assert not error_str is None
    assert not error_repr is None

    # IncompatibleElementError
    assert compat_problems[2].element == el_adaptive_sw_component_type
    error_str = compat_problems[2].__str__()
    error_repr = compat_problems[2].__repr__()
    assert not error_str is None
    assert not error_repr is None


def test_file_misc() -> None:
    model = AutosarModel()

    file1 = model.create_file("filename1.arxml", AutosarVersion.AUTOSAR_00051)
    file2 = model.create_file("filename2.arxml", AutosarVersion.AUTOSAR_00051)

    with pytest.raises(TypeError):
        file1 < file2
    with pytest.raises(TypeError):
        file1 > file2
    with pytest.raises(TypeError):
        file1 <= file2
    with pytest.raises(TypeError):
        file1 >= file2
