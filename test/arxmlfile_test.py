from autosar_data import *
import pytest

def test_arxlfile_basic():
    model = AutosarModel()

    file1 = model.create_file("filename1.arxml", specification.AutosarVersion.Autosar_00051)
    file2 = model.create_file("filename2.arxml", specification.AutosarVersion.Autosar_00051)
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
    assert file1.version == specification.AutosarVersion.Autosar_00051
    file1.version = specification.AutosarVersion.Autosar_4_3_0
    assert file1.version == specification.AutosarVersion.Autosar_4_3_0

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


def test_check_version_compatibility():
    model = AutosarModel()

    file1 = model.create_file("filename", specification.AutosarVersion.Autosar_00050)
    el_elements = model.root_element \
        .create_sub_element(specification.ElementName.ArPackages) \
        .create_named_sub_element(specification.ElementName.ArPackage, "Pkg") \
        .create_sub_element(specification.ElementName.Elements)
    el_acl_object_set = el_elements.create_named_sub_element(specification.ElementName.AclObjectSet, "AclObjectSet")
    el_short_name = el_acl_object_set.get_sub_element(specification.ElementName.ShortName)
    el_short_name.set_attribute(specification.AttributeName.BlueprintValue, "xyz")
    el_blueprint_ref = el_acl_object_set \
        .create_sub_element(specification.ElementName.DerivedFromBlueprintRefs) \
        .create_sub_element(specification.ElementName.DerivedFromBlueprintRef)
    el_blueprint_ref.set_attribute(specification.AttributeName.Dest, specification.EnumItem.AbstractImplementationDataType)
    el_adaptive_sw_component_type = el_elements \
        .create_named_sub_element(specification.ElementName.AdaptiveApplicationSwComponentType, "AdaptiveApplicationSwComponentType")

    compat_problems = file1.check_version_compatibility(specification.AutosarVersion.Autosar_4_3_0)
    assert len(compat_problems) == 3
    assert isinstance(compat_problems[0], IncompatibleAttributeError)
    assert isinstance(compat_problems[1], IncompatibleAttributeValueError)
    assert isinstance(compat_problems[2], IncompatibleElementError)

    # IncompatibleAttributeError
    assert compat_problems[0].element == el_short_name
    assert compat_problems[0].attribute == specification.AttributeName.BlueprintValue
    error_str = compat_problems[0].__str__()
    error_repr = compat_problems[0].__repr__()
    assert not error_str is None
    assert not error_repr is None

    # IncompatibleAttributeValueError
    assert compat_problems[1].element == el_blueprint_ref
    assert compat_problems[1].attribute == specification.AttributeName.Dest
    assert compat_problems[1].attribute_value == "ABSTRACT-IMPLEMENTATION-DATA-TYPE" # todo - type conversion of AttributeValue to string?
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