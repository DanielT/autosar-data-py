# API Documentation: autosar_data

## Example

```python
from autosar_data import *
# alternatively: create a new data model from scratch
model = AutosarModel()

# create a file in the model
file1 = model.create_file("filename.arxml", AutosarVersion.AUTOSAR_4_3_0)
# a model can consist of multiple files - elements appear in all of them by default, unless restrictions are set
file2 = model.create_file("filename2.arxml", AutosarVersion.AUTOSAR_00051)

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

## API

::: autosar_data
