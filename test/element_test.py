from autosar_data import *
import pytest

def test_element_basic_1() -> None:
    model = AutosarModel()
    arxmlfile = model.create_file("file")
    el_ar_packages = model.root_element.create_sub_element("AR-PACKAGES")
    el_ar_package = el_ar_packages.create_named_sub_element("AR-PACKAGE", "Pkg1")
    assert isinstance(el_ar_packages, Element)
    assert isinstance(el_ar_package, Element)

    # each element has a parent, only the root_element does not
    assert el_ar_package.parent == el_ar_packages
    assert model.root_element.parent is None

    # each element is part of the model
    assert el_ar_packages.model == model
    assert el_ar_package.model == model

    # identifiable elements always have a ShortName sub element
    assert el_ar_package.is_identifiable
    el_short_name = el_ar_package.get_sub_element("SHORT-NAME")
    assert isinstance(el_short_name, Element)

    with pytest.raises(AutosarDataError):
        el_ar_package.get_sub_element("not an element")

    # properties of named elements
    assert el_ar_package.item_name == "Pkg1"
    assert el_ar_package.path == "/Pkg1"

    # the name can be changed
    el_ar_package.item_name = "NewName"
    assert el_ar_package.item_name == "NewName"
    assert el_ar_package.path == "/NewName"

    # not every string is a valid name, for example the name cannot contain spaces or start with a digit
    with pytest.raises(AutosarDataError):
        el_ar_package.item_name = "text text"

    # these properies are not valid for elements that are not identifiable
    assert el_ar_packages.is_identifiable == False
    assert el_ar_packages.item_name is None
    with pytest.raises(AutosarDataError):
        print(el_ar_packages.path)
    
    # a removed element still exists until there are no more references to it, but it can no longer be used
    el_ar_packages.remove_sub_element(el_ar_package)
    with pytest.raises(AutosarDataError):
        el_ar_package.parent
    with pytest.raises(AutosarDataError):
        el_ar_package.path
    with pytest.raises(AutosarDataError):
        el_ar_package.add_to_file(arxmlfile)
    with pytest.raises(AutosarDataError):
        el_ar_package.file_membership


def test_element_basic_2() -> None:
    model = AutosarModel()
    model.create_file("file")
    el_ar_package = model.root_element.create_sub_element("AR-PACKAGES").create_named_sub_element("AR-PACKAGE", "Pkg1")
    el_root = model.root_element
    # the behavior of the element is determined by its element_type
    assert el_root.is_identifiable == el_root.element_type.is_named
    assert el_root.is_reference == el_root.element_type.is_ref
    assert not el_root.element_type.is_ordered
    assert el_root.element_type.splittable != 0
    assert not el_root.element_type.std_restriction is None

    # Element has __str__ and __repr__
    el_ar_packages_repr = el_root.__repr__()
    assert not el_ar_packages_repr is None
    el_ar_packages_str = el_root.__str__()
    assert not el_ar_packages_str is None
    assert el_ar_packages_repr != el_ar_packages_str

    # elements can be serialized
    el_ar_packages_text = el_root.serialize()
    # this is currently the same as __str__()
    assert el_ar_packages_text == el_ar_packages_str

    # Element has __hash__
    elementset = set([el_root, el_ar_package])
    assert len(elementset) == 2


def test_element_basic_3() -> None:
    model = AutosarModel()
    model.create_file("file")
    el_ar_packages = model.root_element.create_sub_element("AR-PACKAGES")
    el_ar_package = el_ar_packages.create_named_sub_element("AR-PACKAGE", "Pkg1")

    # Element comparison
    assert model.get_element_by_path("/Pkg1") == el_ar_package
    assert el_ar_packages != el_ar_package

    # Elements can be sorted
    el_ar_packages.create_named_sub_element("AR-PACKAGE", "AAA")
    sub_elements = [e for e in el_ar_packages.sub_elements]
    assert sub_elements[0].item_name == "Pkg1"
    assert sub_elements[1].item_name == "AAA"
    # sort all sub elements recursively
    el_ar_packages.sort()
    sub_elements = [e for e in el_ar_packages.sub_elements]
    assert sub_elements[0].item_name == "AAA"
    assert sub_elements[1].item_name == "Pkg1"

    # every alement has an "xml path" this path includes the names and item names of all parent elements
    assert el_ar_package.xml_path == "/<AUTOSAR>/<AR-PACKAGES>/Pkg1"


def test_element_parent() -> None:
    model = AutosarModel()
    model.create_file("file")
    el_ar_packages = model.root_element.create_sub_element("AR-PACKAGES")
    el_ar_package = el_ar_packages.create_named_sub_element("AR-PACKAGE", "Pkg1")
    el_elements = el_ar_package.create_sub_element("ELEMENTS")
    el_system = el_elements.create_named_sub_element("SYSTEM", "System")
    
    assert el_system.parent == el_elements
    assert el_elements.parent == el_ar_package
    assert el_ar_package.parent == el_ar_packages
    assert el_ar_packages.parent == model.root_element
    assert model.root_element.parent is None

    assert el_system.named_parent == el_ar_package
    assert el_elements.named_parent == el_ar_package
    assert el_ar_package.named_parent is None


def test_element_content() -> None:
    model = AutosarModel()
    model.create_file("file")
    el_ar_packages = model.root_element.create_sub_element("AR-PACKAGES")
    el_pkg1 = el_ar_packages.create_named_sub_element("AR-PACKAGE", "Pkg1")
    el_short_name = el_pkg1.get_sub_element("SHORT-NAME")
    el_l2 = el_pkg1 \
        .create_sub_element("DESC") \
        .create_sub_element("L-2")

    # different elements have different content types
    assert el_pkg1.content_type == ContentType.Elements
    assert el_short_name.content_type == ContentType.CharacterData
    assert el_l2.content_type == ContentType.Mixed

    # create some items for the content of el_l2
    el_l2.insert_character_content_item("text", 0)
    assert len([c for c in el_l2.content]) == 1
    el_l2.create_sub_element("BR")
    assert len([c for c in el_l2.content]) == 2

    assert el_l2.content_item_count == 2

    # check the content
    content = [c for c in el_l2.content]
    assert isinstance(content[0], str)
    assert isinstance(content[1], Element)

    # remove an item
    el_l2.remove_character_content_item(0)
    assert len([c for c in el_l2.content]) == 1

    # *_character_content_item is not valid for elements with ContentType.CharacterData
    with pytest.raises(AutosarDataError):
        el_short_name.insert_character_content_item("text", 0)


def test_element_references() -> None:
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
    el_can_cluster = model.root_element \
        .get_sub_element("AR-PACKAGES") \
        .create_named_sub_element("AR-PACKAGE", "CanPkg") \
        .create_sub_element("ELEMENTS") \
        .create_named_sub_element("CAN-CLUSTER", "CanCluster")

    # various character data elements have constraints, e.g the reference element can only contain an autosar path
    # integers, or strings that do not look like paths cause an exception
    with pytest.raises(AutosarDataError):
        el_fibex_element_ref.character_data = 1
    with pytest.raises(AutosarDataError):
        el_fibex_element_ref.character_data = "? what ?"
    with pytest.raises(TypeError):
        el_fibex_element_ref.character_data = AutosarVersion.AUTOSAR_00042 # wrong datatype
    # "looks like" an autosar path
    el_fibex_element_ref.character_data = "/something/else"

    # the DEST attribute of el_fibex_element_ref takes an enum value, all other values cause an error
    with pytest.raises(ValueError):
        el_fibex_element_ref.set_attribute("DEST", "bla")
    with pytest.raises(AutosarDataError):
        el_fibex_element_ref.set_attribute("DEST", "default")

    # in cases where the element name of the target is NOT a valid value in the "DEST" attribute
    # the function reference_dest_value() can be used instead
    destval = el_fibex_element_ref.element_type.reference_dest_value(el_can_cluster.element_type)
    el_fibex_element_ref.set_attribute("DEST", destval)

    # there is special handling for references
    with pytest.raises(AutosarDataError):
        invalid = el_fibex_element_ref.reference_target
    # set the reference to a valid element
    el_fibex_element_ref.reference_target = el_can_cluster
    assert el_fibex_element_ref.character_data == "/CanPkg/CanCluster"

    # AR-PACKAGES is not a reference element, so setting a reference target isn't possible
    with pytest.raises(AutosarDataError):
        el_ar_packages.reference_target = el_can_cluster

    # remove the character data
    el_fibex_element_ref.remove_character_data()
    assert el_fibex_element_ref.character_data is None


def test_element_character_data_1() -> None:
    # some elements have the data type double / f64 for their character content
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
    el_macrotick.character_data = 2.71828
    el_macrotick.character_data = 3 # automatic conversion to double
    el_macrotick.character_data = "3.1415" # automatic conversion to double
    assert el_macrotick.character_data == 3.1415
    with pytest.raises(TypeError):
        el_macrotick.character_data = model
    with pytest.raises(ValueError):
        el_macrotick.character_data = "not numeric"


def test_element_character_data_2() -> None:
    # it seems there is only one element with datatype unsigned integer, and only in Autosar 4.0.1
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
    el_cse_code.character_data = 42
    el_cse_code.character_data = "123" # automatic conversion
    assert el_cse_code.character_data == 123
    with pytest.raises(TypeError):
        el_cse_code.character_data = model
    with pytest.raises(ValueError):
        el_cse_code.character_data = "text"
    with pytest.raises(TypeError):
        el_cse_code.character_data = 3.1 # no automatic conversion from float, truncation should be explicit


def test_element_character_data_3() -> None:
    model = AutosarModel()
    model.create_file("file")
    el_fibex_element_ref = model.root_element.create_sub_element("AR-PACKAGES") \
        .create_named_sub_element("AR-PACKAGE", "Pkg") \
        .create_sub_element("ELEMENTS") \
        .create_named_sub_element("SYSTEM", "System") \
        .create_sub_element("FIBEX-ELEMENTS") \
        .create_sub_element("FIBEX-ELEMENT-REF-CONDITIONAL") \
        .create_sub_element("FIBEX-ELEMENT-REF")
    el_fibex_element_ref.set_attribute("DEST", "I-SIGNAL")
    assert el_fibex_element_ref.attribute_value("DEST") == "I-SIGNAL"
    with pytest.raises(ValueError):
        el_fibex_element_ref.set_attribute("DEST", "not an enum item")
    with pytest.raises(TypeError):
        el_fibex_element_ref.set_attribute("DEST", 42)
    with pytest.raises(TypeError):
        el_fibex_element_ref.set_attribute("DEST", model)


def test_element_character_data_4() -> None:
    model = AutosarModel()
    model.create_file("file")
    model.root_element.set_attribute("S", "text")
    model.root_element.set_attribute("S", 42)
    model.root_element.set_attribute("S", 3.1415)
    assert model.root_element.attribute_value("S") == "3.1415"
    with pytest.raises(TypeError):
        model.root_element.set_attribute("S", model)


def test_character_data_5() -> None:
    model = AutosarModel()
    model.create_file("file")
    el_ar_packages = model.root_element.create_sub_element("AR-PACKAGES")
    # can't set character data on elements that have ContentType: Elements
    with pytest.raises(AutosarDataError):
        el_ar_packages.character_data = "abc"

    # can't remove character data from elements that have ContentType: Elements
    with pytest.raises(AutosarDataError):
        el_ar_packages.remove_character_data()

    # can't remove character data from elements that have ContentType: Elements
    with pytest.raises(AutosarDataError):
        el_ar_packages.remove_character_content_item(0)


def test_character_data_6() -> None:
    # reading and writing character data on an element with content_type == Mixed
    # this should work as long as there are 0 or 1 content items in the mixed content
    model = AutosarModel()
    model.create_file("file")
    el_l2 = model.root_element.create_sub_element("AR-PACKAGES") \
        .create_named_sub_element("AR-PACKAGE", "Pkg1") \
        .create_sub_element("DESC") \
        .create_sub_element("L-2")
    el_l2.character_data = "text"
    assert el_l2.character_data == "text"

def test_element_creation() -> None:
    model = AutosarModel()
    model.create_file("file")

    # create an unnamed element
    el_ar_packages = model.root_element.create_sub_element("AR-PACKAGES")

    # get or create
    el_ar_packages_cpy = model.root_element.get_or_create_sub_element("AR-PACKAGES")
    assert el_ar_packages == el_ar_packages_cpy

    # create a named element
    el_pkg1 = el_ar_packages.create_named_sub_element("AR-PACKAGE", "Pkg1")

    # get or create named
    el_pkg1_cpy = el_ar_packages.get_or_create_named_sub_element("AR-PACKAGE", "Pkg1")
    assert el_pkg1 == el_pkg1_cpy

    # create an element at a given position
    # not every position is allowed
    with pytest.raises(AutosarDataError):
        el_elements = el_pkg1.create_sub_element("ELEMENTS", 0)
    # position 1 (after ShortName) is allowed
    assert len([e for e in el_pkg1.sub_elements]) == 1
    el_elements = el_pkg1.create_sub_element("ELEMENTS", 1)
    assert el_elements.position == 1

    # create a named sub element at a given position
    el_elements.create_named_sub_element("SYSTEM", "System", 0)
    el_system = el_elements.get_sub_element_at(0)
    assert el_system.element_name == "SYSTEM"

    # create an element by copying another element and all of its sub elements
    el_pkg2 = el_ar_packages.create_copied_sub_element(el_pkg1)
    # because the item_name must be unique among the siblings, the element might be renamed while copying
    assert el_pkg2.item_name == "Pkg1_1"
    el_pkg2.item_name = "Pkg2"
    copied_system = el_pkg2.get_sub_element("ELEMENTS").get_sub_element("SYSTEM")
    assert copied_system.item_name == "System"
    assert copied_system.path == "/Pkg2/System"

    # create a copied elelemt at a given position
    el_pkg3 = el_ar_packages.create_copied_sub_element(el_pkg1, 0)
    assert el_pkg3.position == 0
    el_pkg3.item_name = "Pkg3"

    # elements can be moved to a different parent, as long as this results in a valid hierarchy
    # not valid: ArPackage inside Arpackage
    with pytest.raises(AutosarDataError):
        el_pkg1.move_element_here(el_pkg2)
    # valid: AR-PACKAGE inside AR-PACKAGES
    el_pkg1.create_sub_element("AR-PACKAGES").move_element_here(el_pkg2)
    assert el_pkg2.path == "/Pkg1/Pkg2"
    assert copied_system.path == "/Pkg1/Pkg2/System"

    # move_element_here can move elements to a specified position within a target element
    # it can also be used to re-oder elements inside the current element
    sub_elements = [e for e in el_ar_packages.sub_elements]
    assert sub_elements[0] == el_pkg3
    assert sub_elements[1] == el_pkg1
    el_ar_packages.move_element_here(el_pkg3, 1)
    sub_elements = [e for e in el_ar_packages.sub_elements]
    assert sub_elements[0] == el_pkg1
    assert sub_elements[1] == el_pkg3

    # in all cases only valid sub elements can be created
    # el_pkg1 already has a ShortName, and only one of these is allowed
    with pytest.raises(AutosarDataError):
        el_pkg1.create_sub_element("SHORT-NAME")
    # the element Autosar is not a valid sub element of ArPackage
    with pytest.raises(AutosarDataError):
         el_pkg1.create_sub_element("AUTOSAR", 1)

    # it is possible to check which sub elements would be valid
    # returns a list of tuples: (ElementName, is_named, currently_allowed)
    vsi_list = el_pkg1.list_valid_sub_elements()
    vsi_dbg = vsi_list[0].__repr__()
    assert not vsi_dbg is None
    allowed_elements = [vsi.element_name if vsi.is_allowed else None for vsi in vsi_list]
    assert not "AUTOSAR" in allowed_elements
    assert "CATEGORY" in allowed_elements

    # remove an element
    el_ar_packages.remove_sub_element(el_pkg3)
    with pytest.raises(AutosarDataError):
        invalid = el_pkg3.path
    with pytest.raises(AutosarDataError):
        invalid2 = el_pkg3.model

    # validate the resulting model
    element_info = [x for x in model.root_element.elements_dfs]
    # element info is a list of tuple(depth, element)
    assert element_info[0][1].element_name == "AUTOSAR"
    assert element_info[1][1].element_name == "AR-PACKAGES"
    assert element_info[2][1].element_name == "AR-PACKAGE"
    assert element_info[2][1].item_name == "Pkg1"
    assert element_info[3][1].element_name == "SHORT-NAME"
    assert element_info[4][1].element_name == "ELEMENTS"
    assert element_info[5][1].element_name == "SYSTEM"
    assert element_info[5][1].item_name == "System"
    assert element_info[6][1].element_name == "SHORT-NAME"
    assert element_info[7][1].element_name == "AR-PACKAGES"
    assert element_info[8][1].element_name == "AR-PACKAGE"
    assert element_info[8][1].item_name == "Pkg2"
    assert element_info[9][1].element_name == "SHORT-NAME"
    assert element_info[10][1].element_name == "ELEMENTS"
    assert element_info[11][1].element_name == "SYSTEM"
    assert element_info[11][1].item_name == "System"
    assert element_info[12][1].element_name == "SHORT-NAME"
    assert len(element_info) == 13

def test_get_sub_element_additional() -> None:
    model = AutosarModel()
    model.create_file("file")
    el_ar_packages = model.root_element.create_sub_element("AR-PACKAGES")
    el_ar_package = el_ar_packages.create_named_sub_element("AR-PACKAGE", "Pkg")
    el_elements = el_ar_package.create_sub_element("ELEMENTS")
    el_elements.create_named_sub_element("ECUC-MODULE-CONFIGURATION-VALUES", "BswConfig") \
        .create_sub_element("DEFINITION-REF") \
        .character_data = "/Bsw/Definition/Container"

    el_bsw = el_elements.get_bsw_sub_element("/Bsw/Definition/Container")
    assert isinstance(el_bsw, Element)
    assert el_bsw.item_name == "BswConfig"

    el_bsw = el_elements.get_bsw_sub_element("Container")
    assert isinstance(el_bsw, Element)
    assert el_bsw.item_name == "BswConfig"

    el_bsw = el_elements.get_named_sub_element("BswConfig")
    assert isinstance(el_bsw, Element)
    assert el_bsw.element_name == "ECUC-MODULE-CONFIGURATION-VALUES"

def test_element_action_errors() -> None:
    model = AutosarModel()
    model.create_file("file")
    el_ar_packages = model.root_element.create_sub_element("AR-PACKAGES")
    el_ar_package = el_ar_packages.create_named_sub_element("AR-PACKAGE", "Pkg")
    el_ar_package2 = el_ar_packages.create_named_sub_element("AR-PACKAGE", "Pkg2")

    # cannot create unknown elements, or elements that are not valid sub elements
    with pytest.raises(AutosarDataError):
        el_ar_package.create_sub_element("not an element")
    with pytest.raises(AutosarDataError):
        el_ar_package.create_sub_element("not an element", 0)
    with pytest.raises(AutosarDataError):
        el_ar_package.create_sub_element("AUTOSAR")
    with pytest.raises(AutosarDataError):
        el_ar_package.create_sub_element("AUTOSAR", 0)

    with pytest.raises(AutosarDataError):
        el_ar_package.create_named_sub_element("not an element", "name")
    with pytest.raises(AutosarDataError):
        el_ar_package.create_named_sub_element("not an element", "name", 0)
    with pytest.raises(AutosarDataError):
        el_ar_package.create_named_sub_element("AUTOSAR", "name")
    with pytest.raises(AutosarDataError):
        el_ar_package.create_named_sub_element("AUTOSAR", "name", 0)

    # cannot create invalid an structure by copying
    with pytest.raises(AutosarDataError):
        el_ar_package.create_copied_sub_element(el_ar_package2)
    with pytest.raises(AutosarDataError):
        el_ar_package.create_copied_sub_element(el_ar_package2, 0)

    # cannot create an invalid stucture by moving elements
    with pytest.raises(AutosarDataError):
        el_ar_package.move_element_here(el_ar_package2)
    with pytest.raises(AutosarDataError):
        el_ar_package.move_element_here(el_ar_package2, 0)
    
    # can't remove an element that is not a sub element
    with pytest.raises(AutosarDataError):
        el_ar_package.remove_sub_element(el_ar_package2)



def test_element_attributes() -> None:
    model = AutosarModel()
    model.create_file("file")
    el_autosar = model.root_element

    attributes = [attr for attr in el_autosar.attributes]
    assert isinstance(attributes[0], Attribute)
    assert attributes[0].attrname == "xsi:schemaLocation"
    assert attributes[1].attrname == "xmlns"
    assert attributes[1].content == "http://autosar.org/schema/r4.0"
    assert attributes[2].attrname == "xmlns:xsi"
    assert attributes[2].content == "http://www.w3.org/2001/XMLSchema-instance"
    assert len(attributes) == 3

    # __repr__ and __str__ exist for Attribute
    assert not attributes[0].__repr__() is None
    assert not attributes[0].__str__() is None

    el_autosar.set_attribute("S", "some text")
    # attribute values are checked - attribute T must contain a valid timestamp
    with pytest.raises(AutosarDataError):
        el_autosar.set_attribute("T", "some text")
    # the function set_attribute automatically converts an input string to enum or integer if the attribute requires this
    el_autosar.set_attribute("T", "2023-04-05T12:34:56Z")
    assert el_autosar.attribute_value("T") == "2023-04-05T12:34:56Z"

    with pytest.raises(AutosarDataError):
        el_autosar.set_attribute("not an attribute", "some text")
    with pytest.raises(AutosarDataError):
        el_autosar.attribute_value("not an attribute")
    with pytest.raises(AutosarDataError):
        el_autosar.set_attribute("DEST", 0)

    assert len([attr for attr in el_autosar.attributes]) == 5
    el_autosar.remove_attribute("T")
    assert len([attr for attr in el_autosar.attributes]) == 4
    with pytest.raises(AutosarDataError):
        el_autosar.remove_attribute("xyz")


def test_file_membership() -> None:
    model = AutosarModel()
    model.create_file("file")
    file1 = model.create_file("file1", AutosarVersion.AUTOSAR_00050)
    file2 = model.create_file("file2", AutosarVersion.AUTOSAR_00050)
    el_ar_packages = model.root_element.create_sub_element("AR-PACKAGES")
    el_pkg1 = el_ar_packages.create_named_sub_element("AR-PACKAGE", "Pkg1")
    el_elements = el_pkg1.create_sub_element("ELEMENTS")
    el_pkg2 = el_ar_packages.create_named_sub_element("AR-PACKAGE", "Pkg2")

    total_element_count = len([e for e in model.elements_dfs])
    # initially all elements are part of every file
    file1_element_count = len([e for e in file1.elements_dfs])
    assert total_element_count == file1_element_count
    assert file1 in el_pkg1.file_membership[1]
    assert file2 in el_pkg1.file_membership[1]

    # remove pkg1 from file2 and pkg2 from file1
    el_pkg1.remove_from_file(file2)
    el_pkg2.remove_from_file(file1)
    file1_element_count = len([e for e in file1.elements_dfs])
    file2_element_count = len([e for e in file2.elements_dfs])
    assert file1_element_count < total_element_count
    assert file2_element_count < total_element_count
    assert file1_element_count != file2_element_count

    el_pkg1.add_to_file(file2)
    assert file2 in el_pkg1.file_membership[1]

    el_pkg1.remove_sub_element(el_elements)
    with pytest.raises(AutosarDataError):
        el_elements.remove_from_file(file2)


def test_element_version() -> None:
    model = AutosarModel()
    file1 = model.create_file("file1", AutosarVersion.AUTOSAR_00050)
    file2 = model.create_file("file2", AutosarVersion.AUTOSAR_00051)
    el_ar_packages = model.root_element.create_sub_element("AR-PACKAGES")
    # el_ar_packages is present in both files
    assert len(el_ar_packages.file_membership) == 2
    assert el_ar_packages.min_version == AutosarVersion.AUTOSAR_00050


def test_element_misc() -> None:
    model = AutosarModel()
    element = model.root_element

    assert element == element
    assert not element != element

    with pytest.raises(TypeError):
        element < element
    with pytest.raises(TypeError):
        element > element
    with pytest.raises(TypeError):
        element <= element
    with pytest.raises(TypeError):
        element >= element

def test_element_coment() -> None:
    model = AutosarModel()
    model.create_file("test")
    assert model.root_element.comment is None
    model.root_element.comment = "text"
    assert model.root_element.comment == "text"
