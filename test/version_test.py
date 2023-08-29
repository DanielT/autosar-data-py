from autosar_data import *
import pytest

def test_version() -> None:
    model = AutosarModel()

    ver = AutosarVersion("AUTOSAR_4-0-1.xsd")
    assert ver == AutosarVersion.AUTOSAR_4_0_1
    arxmlfile = model.create_file("AUTOSAR_4-0-1.arxml", ver)
    assert arxmlfile.version == ver
    assert ver.__str__() == "AUTOSAR 4.0.1"

    ver = AutosarVersion("AUTOSAR_4-0-2.xsd")
    assert ver == AutosarVersion.AUTOSAR_4_0_2
    arxmlfile = model.create_file("AUTOSAR_4-0-2.arxml", ver)
    assert arxmlfile.version == ver

    ver = AutosarVersion("AUTOSAR_4-0-3.xsd")
    assert ver == AutosarVersion.AUTOSAR_4_0_3
    arxmlfile = model.create_file("AUTOSAR_4-0-3.arxml", ver)
    assert arxmlfile.version == ver

    ver = AutosarVersion("AUTOSAR_4-1-1.xsd")
    assert ver == AutosarVersion.AUTOSAR_4_1_1
    arxmlfile = model.create_file("AUTOSAR_4-1-1.arxml", ver)
    assert arxmlfile.version == ver

    ver = AutosarVersion("AUTOSAR_4-1-2.xsd")
    assert ver == AutosarVersion.AUTOSAR_4_1_2
    arxmlfile = model.create_file("AUTOSAR_4-1-2.arxml", ver)
    assert arxmlfile.version == ver

    ver = AutosarVersion("AUTOSAR_4-1-3.xsd")
    assert ver == AutosarVersion.AUTOSAR_4_1_3
    arxmlfile = model.create_file("AUTOSAR_4-1-3.arxml", ver)
    assert arxmlfile.version == ver

    ver = AutosarVersion("AUTOSAR_4-2-1.xsd")
    assert ver == AutosarVersion.AUTOSAR_4_2_1
    arxmlfile = model.create_file("AUTOSAR_4-2-1.arxml", ver)
    assert arxmlfile.version == ver

    ver = AutosarVersion("AUTOSAR_4-2-2.xsd")
    assert ver == AutosarVersion.AUTOSAR_4_2_2
    arxmlfile = model.create_file("AUTOSAR_4-2-2.arxml", ver)
    assert arxmlfile.version == ver

    ver = AutosarVersion("AUTOSAR_4-3-0.xsd")
    assert ver == AutosarVersion.AUTOSAR_4_3_0
    arxmlfile = model.create_file("AUTOSAR_4-3-0.arxml", ver)
    assert arxmlfile.version == ver

    ver = AutosarVersion("AUTOSAR_00042.xsd")
    assert ver == AutosarVersion.AUTOSAR_00042
    arxmlfile = model.create_file("AUTOSAR_00042.arxml", ver)
    assert arxmlfile.version == ver

    ver = AutosarVersion("AUTOSAR_00043.xsd")
    assert ver == AutosarVersion.AUTOSAR_00043
    arxmlfile = model.create_file("AUTOSAR_00043.arxml", ver)
    assert arxmlfile.version == ver

    ver = AutosarVersion("AUTOSAR_00044.xsd")
    assert ver == AutosarVersion.AUTOSAR_00044
    arxmlfile = model.create_file("AUTOSAR_00044.arxml", ver)
    assert arxmlfile.version == ver

    ver = AutosarVersion("AUTOSAR_00045.xsd")
    assert ver == AutosarVersion.AUTOSAR_00045
    arxmlfile = model.create_file("AUTOSAR_00045.arxml", ver)
    assert arxmlfile.version == ver

    ver = AutosarVersion("AUTOSAR_00046.xsd")
    assert ver == AutosarVersion.AUTOSAR_00046
    arxmlfile = model.create_file("AUTOSAR_00046.arxml", ver)
    assert arxmlfile.version == ver

    ver = AutosarVersion("AUTOSAR_00047.xsd")
    assert ver == AutosarVersion.AUTOSAR_00047
    arxmlfile = model.create_file("AUTOSAR_00047.arxml", ver)
    assert arxmlfile.version == ver

    ver = AutosarVersion("AUTOSAR_00048.xsd")
    assert ver == AutosarVersion.AUTOSAR_00048
    arxmlfile = model.create_file("AUTOSAR_00048.arxml", ver)
    assert arxmlfile.version == ver

    ver = AutosarVersion("AUTOSAR_00049.xsd")
    assert ver == AutosarVersion.AUTOSAR_00049
    arxmlfile = model.create_file("AUTOSAR_00049.arxml", ver)
    assert arxmlfile.version == ver

    ver = AutosarVersion("AUTOSAR_00050.xsd")
    assert ver == AutosarVersion.AUTOSAR_00050
    arxmlfile = model.create_file("AUTOSAR_00050.arxml", ver)
    assert arxmlfile.version == ver

    ver = AutosarVersion("AUTOSAR_00051.xsd")
    assert ver == AutosarVersion.AUTOSAR_00051
    arxmlfile = model.create_file("AUTOSAR_00051.arxml", ver)
    assert arxmlfile.version == ver

    with pytest.raises(AutosarDataError):
        ver = AutosarVersion("bad.xsd")
    
    # test (in)equalities
    assert AutosarVersion.AUTOSAR_00051 == AutosarVersion.AUTOSAR_00051
    assert AutosarVersion.AUTOSAR_4_0_1 != AutosarVersion.AUTOSAR_00051
    assert AutosarVersion.AUTOSAR_4_0_1 < AutosarVersion.AUTOSAR_00051
    assert AutosarVersion.AUTOSAR_4_0_1 <= AutosarVersion.AUTOSAR_00051
    assert AutosarVersion.AUTOSAR_00051 > AutosarVersion.AUTOSAR_4_0_1
    assert AutosarVersion.AUTOSAR_00051 >= AutosarVersion.AUTOSAR_4_0_1
