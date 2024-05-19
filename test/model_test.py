from autosar_data import *
import pytest
import os

def test_model_basic() -> None:
    model = AutosarModel()
    # check that the object was created - model is not None
    assert isinstance(model, AutosarModel)
    assert isinstance(model.root_element, Element)
    assert model.root_element.element_name == "AUTOSAR"
    assert len(model.files) == 0
    identifiables = [i for i in model.identifiable_elements]
    assert len(identifiables) == 0


def test_model_files(tmp_path: str) -> None:
    model = AutosarModel()

    # create a file
    filename1 = os.path.join(tmp_path, "test.arxml")
    file1 = model.create_file(filename1, AutosarVersion.AUTOSAR_00051)
    assert isinstance(file1, ArxmlFile)
    assert file1.filename == os.path.join(tmp_path, "test.arxml")

    # create another file
    file2 = model.create_file("test2.arxml")
    assert isinstance(file2, ArxmlFile)
    assert len(model.files) == 2

    # create a file with the same name as file1
    with pytest.raises(AutosarDataError):
        # a file called "$tmp_path/test.arxml" already exists in the model
        file3 = model.create_file(filename1)
    
    # remove file2 from the model again
    model.remove_file(file2)
    assert len(model.files) == 1

    # the file filename1 does not exist on disk yet
    assert os.path.isfile(filename1) == False
    # write all files in the model (i.e. file1)
    model.write()
    # now the file has been written
    assert os.path.isfile(filename1)

    # load the newly created file in a new model
    model2 = AutosarModel()
    (m2_file, warnings) = model2.load_file(filename1, True)
    assert isinstance(m2_file, ArxmlFile)
    assert len(warnings) == 0
        
    # can't load a nonexistent file
    with pytest.raises(AutosarDataError):
        model2.load_file("nonexistent_nothing")
    
    # create a string of arxml data from file1
    all_files_text = model.serialize_files()
    file1_text = all_files_text[filename1]

    # load the string in a new model
    model3 = AutosarModel()
    (m3_file, warnings) = model3.load_buffer(file1_text, "m3_file.arxml")
    assert isinstance(m3_file, ArxmlFile)
    assert len(warnings) == 0

    # can't load nonsense data as arxml
    with pytest.raises(AutosarDataError):
        model3.load_buffer("hello, world!", "m3_file2.arxml")


def test_strict_parsing(tmp_path: str) -> None:
    # the following arxml contains an element from the adaptive platform, but specifies version 4.3.0
    buffer = """<?xml version="1.0" encoding="utf-8"?>
<AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_4-3-0.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
  <AR-PACKAGES>
    <AR-PACKAGE>
      <SHORT-NAME>Pkg</SHORT-NAME>
      <ELEMENTS>
        <ADAPTIVE-APPLICATION-SW-COMPONENT-TYPE>
          <SHORT-NAME>AdaptiveApplicationSwComponentType</SHORT-NAME>
        </ADAPTIVE-APPLICATION-SW-COMPONENT-TYPE>
      </ELEMENTS>
    </AR-PACKAGE>
  </AR-PACKAGES>
</AUTOSAR>"""
    model = AutosarModel()
    filename = os.path.join(tmp_path, "test.arxml")
    # loading the buffer with strict = False succeeds but produces warnings
    (arxmlfile, warnings) = model.load_buffer(buffer, filename, False)
    assert isinstance(arxmlfile, ArxmlFile)
    assert len(warnings) != 0
    # loading with strict = True triggers an exception
    with pytest.raises(AutosarDataError):
        model.load_buffer(buffer, filename, True)
    
    model.write()
    # erase the model, so that loading the file with the existing filename works
    model = AutosarModel()
    # loading the file with strict = False succeeds but produces warnings
    (arxmlfile, warnings) = model.load_file(filename, False)
    assert isinstance(arxmlfile, ArxmlFile)
    assert len(warnings) != 0
    # loading with strict = True triggers an exception
    with pytest.raises(AutosarDataError):
        model.load_file(filename, False)


def test_model_write() -> None:
    model = AutosarModel()
    model.create_file("/this/path/does/not/exist.arxml")
    with pytest.raises(AutosarDataError):
        model.write()


def test_model_identifiables() -> None:
    model = AutosarModel()
    model.create_file("file")
    # create some elements
    el_elements = model.root_element \
        .create_sub_element("AR-PACKAGES") \
        .create_named_sub_element("AR-PACKAGE", "Pkg") \
        .create_sub_element("ELEMENTS")
    el_fibex_element_ref = el_elements \
        .create_named_sub_element("SYSTEM", "System") \
        .create_sub_element("FIBEX-ELEMENTS") \
        .create_sub_element("FIBEX-ELEMENT-REF-CONDITIONAL") \
        .create_sub_element("FIBEX-ELEMENT-REF")
    el_can_cluster = model.root_element \
        .get_sub_element("AR-PACKAGES") \
        .create_named_sub_element("AR-PACKAGE", "Pkg2") \
        .create_sub_element("ELEMENTS") \
        .create_named_sub_element("CAN-CLUSTER", "CanCluster")
    assert isinstance(el_elements, Element)
    assert isinstance(el_fibex_element_ref, Element)
    assert isinstance(el_can_cluster, Element)

    # create across reference between two elements
    el_fibex_element_ref.reference_target = el_can_cluster
    assert el_fibex_element_ref.reference_target == el_can_cluster

    # check that all the expected identifiable elements exist in the model
    identifiable_elements = [p for (p, _) in model.identifiable_elements]
    assert len(identifiable_elements) == 4
    idents = set(identifiable_elements)
    assert "/Pkg" in idents
    assert "/Pkg2" in idents
    assert "/Pkg/System" in idents
    assert "/Pkg2/CanCluster" in idents

    # follow a reference backward to the referrer
    assert model.get_element_by_path("/Pkg2/CanCluster") == el_can_cluster
    el_can_cluster_referrers = model.get_references_to("/Pkg2/CanCluster")
    assert len(el_can_cluster_referrers) == 1
    assert el_can_cluster_referrers[0] == el_fibex_element_ref


def test_model_duplicate() -> None:
    model = AutosarModel()
    file1 = model.create_file("file1")
    file2 = model.create_file("file2")
    # create some elements
    el_elements = model.root_element \
        .create_sub_element("AR-PACKAGES") \
        .create_named_sub_element("AR-PACKAGE", "Pkg") \
        .create_sub_element("ELEMENTS")
    el_elements \
        .create_named_sub_element("SYSTEM", "System") \
        .create_sub_element("FIBEX-ELEMENTS") \
        .create_sub_element("FIBEX-ELEMENT-REF-CONDITIONAL") \
        .create_sub_element("FIBEX-ELEMENT-REF")
    model.root_element \
        .get_sub_element("AR-PACKAGES") \
        .create_named_sub_element("AR-PACKAGE", "Pkg2") \
        .create_sub_element("ELEMENTS") \
        .create_named_sub_element("CAN-CLUSTER", "CanCluster")
    # make the scenario a bit mor complicated by setting file membershhip for some elements
    pkg1 = model.get_element_by_path("/Pkg")
    pkg2 = model.get_element_by_path("/Pkg2")
    pkg1.remove_from_file(file2)
    pkg2.remove_from_file(file1)

    # duplicate the model
    model2 = model.duplicate()

    # the two models are not the same object
    assert model != model2
    # the properties of the two models are identical
    assert len(model2.files) == 2
    m2_file1 = model2.files[0]
    m2_file2 = model2.files[1]
    assert file1.serialize() == m2_file1.serialize()
    assert file2.serialize() == m2_file2.serialize()

    m2_pkg1 = model2.get_element_by_path("/Pkg")
    assert pkg1 != m2_pkg1

def test_model_misc() -> None:
    model = AutosarModel()
    model2 = AutosarModel()

    # two different models are not equal, even if they have the same content
    assert model != model2
    # two references to the same model are equal
    assert model.root_element.model == model
    # inequalities do not exist
    with pytest.raises(TypeError):
        model < model2
    with pytest.raises(TypeError):
        model > model2
    with pytest.raises(TypeError):
        model <= model2
    with pytest.raises(TypeError):
        model >= model2

    # models can be hashed
    modelset = set([model, model2])
    assert len(modelset) == 2


def test_model_misc_2() -> None:
    model = AutosarModel()

    # the model can be displayed as a string
    model_str = str.format("{}", model)
    assert not model_str is None

    # the model can be displayed as a string
    model_str = str.format("{}", model.__repr__())
    assert not model_str is None


def test_model_misc_3() -> None:
    model = AutosarModel()
    model.create_file("file")

    # dfs iterator test: create some elements
    model.root_element \
        .create_sub_element("AR-PACKAGES") \
        .create_named_sub_element("AR-PACKAGE", "Pkg1") \
        .create_sub_element("ELEMENTS")
    elements = [{"depth":depth, "element":element} for (depth, element) in model.elements_dfs]
    assert len(elements) == 5
    assert elements[0]['depth'] == 0
    assert elements[0]['element'].element_name == "AUTOSAR"
    assert elements[0]['element'] == model.root_element
    assert elements[1]['depth'] == 1
    assert elements[1]['element'].element_name == "AR-PACKAGES"
    assert elements[2]['depth'] == 2
    assert elements[2]['element'].element_name == "AR-PACKAGE"
    assert elements[3]['depth'] == 3
    assert elements[3]['element'].element_name == "SHORT-NAME"
    assert elements[4]['depth'] == 3
    assert elements[4]['element'].element_name == "ELEMENTS"


def test_model_misc_4() -> None:
    model = AutosarModel()
    model.create_file("file")

    # dfs iterator test: create some elements
    el_elements = model.root_element \
        .create_sub_element("AR-PACKAGES") \
        .create_named_sub_element("AR-PACKAGE", "Pkg1") \
        .create_sub_element("ELEMENTS")
    # create a ref element for check_references()
    el_fibex_element_ref = el_elements \
        .create_named_sub_element("SYSTEM", "System") \
        .create_sub_element("FIBEX-ELEMENTS") \
        .create_sub_element("FIBEX-ELEMENT-REF-CONDITIONAL") \
        .create_sub_element("FIBEX-ELEMENT-REF")
    # set the referecne to a nonexistent path
    el_fibex_element_ref.character_data = "/Pkg"
    broken_refs = model.check_references()
    assert len(broken_refs) == 1
    assert broken_refs[0] == el_fibex_element_ref

    # create a second ArPackage "pkg2" and put it in front of the existing "Pkg1"
    el_ar_packages = model.root_element.get_sub_element("AR-PACKAGES")
    el_pkg2 = el_ar_packages.create_named_sub_element("AR-PACKAGE", "Pkg2", 0)
    el_pkg1 = model.get_element_by_path("/Pkg1")
    # verify the initial order
    subelements = [elem for elem in el_ar_packages.sub_elements]
    assert len(subelements) == 2
    assert subelements[0] == el_pkg2
    assert subelements[1] == el_pkg1
    # sort the elements in the model
    model.sort()
    # verify that sorting changed the order of elements
    subelements = [elem for elem in el_ar_packages.sub_elements]
    assert subelements[0] == el_pkg1
    assert subelements[1] == el_pkg2

    