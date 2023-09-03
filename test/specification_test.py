from autosar_data import *
import pytest

def test_specification_basic() -> None:
    model = AutosarModel()
    model.create_file("file")
    assert model.root_element.element_type.chardata_spec is None

    el_ar_packages = model.root_element.create_sub_element("AR-PACKAGES")
    el_ar_package = el_ar_packages.create_named_sub_element("AR-PACKAGE", "Pkg1")

    # get a character data specification
    el_short_name = el_ar_package.get_sub_element("SHORT-NAME")
    # SHORT-NAME contains a restricted string
    assert isinstance(el_short_name.element_type.chardata_spec, CharacterDataTypeRestrictedString)

    attribute_spec = model.root_element.element_type.attributes_spec
    assert len(attribute_spec) > 3
    assert not attribute_spec[0].__str__() is None
    assert not attribute_spec[0].__repr__() is None

    with pytest.raises(ValueError):
        model.root_element.element_type.find_attribute_spec("DEST")
    with pytest.raises(TypeError):
        model.root_element.element_type.find_attribute_spec("xyz")


def test_specification_enum() -> None:
    model = AutosarModel()
    model.create_file("file")
    el_ar_packages = model.root_element.create_sub_element("AR-PACKAGES")
    el_elements = el_ar_packages \
        .create_named_sub_element("AR-PACKAGE", "SysPkg") \
        .create_sub_element("ELEMENTS")
    el_fibex_element_ref = el_elements \
        .create_named_sub_element("SYSTEM", "System") \
        .create_sub_element("FIBEX-ELEMENTS") \
        .create_sub_element("FIBEX-ELEMENT-REF-CONDITIONAL") \
        .create_sub_element("FIBEX-ELEMENT-REF")
    dest_attr_spec = el_fibex_element_ref.element_type.find_attribute_spec("DEST")
    print(dest_attr_spec.__repr__())
    assert dest_attr_spec.attribute_name == "DEST"
    assert isinstance(dest_attr_spec, AttributeSpec)
    assert isinstance(dest_attr_spec.value_spec, CharacterDataTypeEnum)
    assert len(dest_attr_spec.value_spec.values) > 1
    assert not dest_attr_spec.__str__() is None
    assert not dest_attr_spec.__repr__() is None
    assert not dest_attr_spec.value_spec.__str__() is None
    assert not dest_attr_spec.value_spec.__repr__() is None


def test_specification_float() -> None:
    model = AutosarModel()
    model.create_file("file")
    el_macrotick = model.root_element \
        .create_sub_element("AR-PACKAGES") \
        .create_named_sub_element("AR-PACKAGE", "pkg") \
        .create_sub_element("ELEMENTS") \
        .create_named_sub_element("FLEXRAY-CLUSTER", "fc") \
        .create_sub_element("FLEXRAY-CLUSTER-VARIANTS") \
        .create_sub_element("FLEXRAY-CLUSTER-CONDITIONAL") \
        .create_sub_element("MACROTICK-DURATION")
    mtd_spec = el_macrotick.element_type.chardata_spec
    assert isinstance(mtd_spec, CharacterDataTypeFloat)
    assert not mtd_spec.__str__() is None
    assert not mtd_spec.__repr__() is None


def test_specification_restricted_string() -> None:
    model = AutosarModel()
    model.create_file("file")
    el_ar_packages = model.root_element.create_sub_element("AR-PACKAGES")
    el_elements = el_ar_packages \
        .create_named_sub_element("AR-PACKAGE", "SysPkg") \
        .create_sub_element("ELEMENTS")
    el_fibex_element_ref = el_elements \
        .create_named_sub_element("SYSTEM", "System") \
        .create_sub_element("FIBEX-ELEMENTS") \
        .create_sub_element("FIBEX-ELEMENT-REF-CONDITIONAL") \
        .create_sub_element("FIBEX-ELEMENT-REF")
    fibex_el_ref_spec = el_fibex_element_ref.element_type.chardata_spec
    assert isinstance(fibex_el_ref_spec, CharacterDataTypeRestrictedString)
    assert not fibex_el_ref_spec.regex is None
    assert not fibex_el_ref_spec.__str__() is None
    assert not fibex_el_ref_spec.__repr__() is None


def test_specification_string() -> None:
    model = AutosarModel()
    model.create_file("file")
    attr_s_spec = model.root_element.element_type.find_attribute_spec("S")
    print(attr_s_spec)
    assert isinstance(attr_s_spec, AttributeSpec)
    assert isinstance(attr_s_spec.value_spec, CharacterDataTypeString)
    assert not attr_s_spec.__str__() is None
    assert not attr_s_spec.__repr__() is None
    assert not attr_s_spec.value_spec.__str__() is None
    assert not attr_s_spec.value_spec.__repr__() is None


def test_specification_uint() -> None:
    model = AutosarModel()
    model.create_file("file")
    el_cse_code = model.root_element \
        .create_sub_element("AR-PACKAGES") \
        .create_named_sub_element("AR-PACKAGE", "pkg") \
        .create_sub_element("ELEMENTS") \
        .create_named_sub_element("BSW-MODULE-TIMING", "bmt") \
        .create_sub_element("TIMING-GUARANTEES") \
        .create_named_sub_element("SYNCHRONIZATION-TIMING-CONSTRAINT", "stc") \
        .create_sub_element("TOLERANCE") \
        .create_sub_element("CSE-CODE")
    cse_spec = el_cse_code.element_type.chardata_spec
    assert isinstance(cse_spec, CharacterDataTypeUnsignedInt)
    assert not cse_spec.__str__() is None
    assert not cse_spec.__repr__() is None


