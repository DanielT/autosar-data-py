# `autosar-data-py`

[![PyPI version](https://badge.fury.io/py/autosar-data.svg)](https://badge.fury.io/py/autosar-data)
[![Github Actions](https://github.com/DanielT/autosar-data-py/workflows/Test/badge.svg)](https://github.com/DanielT/autosar-data-py/actions)
[![codecov](https://codecov.io/gh/DanielT/autosar-data-py/branch/main/graph/badge.svg?token=RGKUUJTWZ5)](https://codecov.io/gh/DanielT/autosar-data-py)

Python bindings for [autosar-data](https://github.com/DanielT/autosar-data)

## Features

This crate implements Python bindings for autosar-data using [PyO3](https://pyo3.rs). This allows all the features of `autosar-data` to be used from python code:

- read and write arxml files
- fully validate all data when it is loaded in strict mode
- non-strict mode so that invalid but structurally sound data can be loaded
- various element operations to modify and create sub-elements, data and attributes
- support for Autosar paths and cross references
- supports Autosar version 4.0.1 and up.

## API documentation

API documentation is located here: [https://danielt.github.io/autosar-data-py/](https://danielt.github.io/autosar-data-py/)

## Example

### Load data from a file

```python
from autosar_data import *

# load a file
model = AutosarModel()
(arxmlfile, warnings) = model.load_file("filename.arxml", False)
```

### Load data from text

```python
from autosar_data import *
# alternatively: load a buffer
model = AutosarModel()
filebuf = """<?xml version="1.0" encoding="utf-8"?>
    <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
    <AR-PACKAGES><AR-PACKAGE><SHORT-NAME>Pkg</SHORT-NAME></AR-PACKAGE></AR-PACKAGES></AUTOSAR>"""
(arxmlfile, warnings) = model.load_buffer(filebuf, "filename.arxml", False)
```

### Create data from scratch

```python
from autosar_data import *
# alternatively: create a new data model from scratch
model = AutosarModel()

# create a file in the model
file1 = model.create_file("filename.arxml", AutosarVersion.Autosar_4_3_0)
# a model can consist of multiple files - elements appear in all of them by default, unless restrictions are set
file2 = model.create_file("filename2.arxml", AutosarVersion.Autosar_00051)

# initially the model only has its root element, <AUTOSAR>. Create some elements
el_elements = model.root_element \
    .create_sub_element("AR-PACKAGES") \
    .create_named_sub_element("AR-PACKAGE", "Pkg") \
    .create_sub_element("ELEMENTS")

# create some more elements
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

# set a cross reference
el_fibex_element_ref.reference_target = el_can_cluster

# check the cross reference
el_fibex_element_ref.character_data
# '/Pkg2/CanCluster'
el_fibex_element_ref.reference_target == el_can_cluster
# True

# get an attribute
el_fibex_element_ref.attribute_value("DEST")
# EnumItem.CanCluster
model.root_element.attribute_value("xmlns")
# 'http://autosar.org/schema/r4.0'

# set an attribute value
el_fibex_element_ref.set_attribute("DEST", "I-SIGNAL")
# setting the DEST of the reference to an invalid value has invalidated the
# reference, so accessing el_fibex_element_ref.reference_target will now cause an exception

el_can_cluster.set_attribute("UUID", "1234567890abcdefg")

# get the current xml text of the model:
print(file1.serialize())
# this prints "<?xml version="1.0" encoding="utf-8"?>\n<AUTOSAR ..."

# write all the files in the model - this will create filename.arxml and filename2.arxml with identical content
model.write()

# get the autosar paths of all elements in the model
paths = model.identifiable_elements
# paths = ['/Pkg', '/Pkg/System', '/Pkg2', '/Pkg2/CanCluster']

# get an element by its path
el_ar_package1 = model.get_element_by_path("/Pkg")
el_ar_package2 = model.get_element_by_path("/Pkg2")
el_system = model.get_element_by_path("/Pkg/System")

# restrict the packages to only appear in one file each
el_ar_package1.remove_from_file(file2)
el_ar_package2.remove_from_file(file1)

# write all the files in the model - now the content is different
model.write()
```

### Working with data

```python
from autosar_data import *

model = AutosarModel()
(arxmlfile, warnings) = model.load_file("somefile.arxml", False)

# display all the triggered PDUs in the file
for (depth, element) in model.elements_dfs:
    if element.element_name == "PDU-TRIGGERING":
        pdu = element.get_sub_element("I-PDU-REF").reference_target
        print(str.format("PDU: <{}> = {}", pdu.element_name, pdu.item_name))

```

## Development

- maturin must be installed: `pip install maturin` if it isn't
- create a venv in the cloned source: `python -m venv .venv`
- build the wheel and directly install it in the venv: `maturin develop`
- activate the venv in a shell: `source .venv/bin/activate` or `.venv\Scripts\Activate.ps1`
- run python in the shell with the venv
