# autosar-data API Documentation

API documenation for the autosar-data Python library

## AutosarModel

The class AutosarModel represents the Autosar data model. It contains all files and elements.

### Constructor

```python
AutosarModel()
```

Create a new AutosarModel

### Functions

#### create_file

```python
def create_file(
    self,
    filename: str,
    version: AutosarVersion = AutosarVersion.LATEST
) -> ArxmlFile
```

Create a new file in the model

#### load_buffer

```python
def load_buffer(
    self,
    buffer: str,
    filename: str,
    strict: bool = False
) -> Tuple[ArxmlFile, List[str]]
```

Load a buffer (string) as arxml

#### load_file

```python
def load_file(self, filename: str, strict: bool = False) -> Tuple[ArxmlFile, List[str]]
```

Load an arxml file

#### remove_file

```python
def remove_file(self, arxmlfile: ArxmlFile) -> None
```

Remove a file from the model. Any elements belonging exclusively to that file will also be removed.

#### serialize_files

```python
def serialize_files(self) -> Dict[str, str]
```

Serialize all files individually, to generate a dict(filename, serialized content),

#### write

```python
def write(self) -> None
```

Write all files in the model to disk

#### get_element_by_path

```python
def get_element_by_path(self, autosar_path: str) -> Element
```

Get an identifiable element in the model by its Autosar path

#### sort

```python
def sort(self) -> None
```

Sort the entire model in place. Takes all ordering constraints into account.

#### get_references_to

```python
def get_references_to(self, target_path: str) -> List[Element]
```

Get all reference elements which refer to the given Autosar path

#### check_references

```python
def check_references(self) -> List[Element]
```

Check all references in the model and return a list of elements containing invalid references

### Class members

#### files

```python
files: List[ArxmlFile]
```

A list of ArxmlFile objects containing all files in the model

#### root_element

```python
root_element: Element
```

The root element of the model, `<AUTOSAR>`

#### elements_dfs

```python
elements_dfs: ElementsDfsIterator
```

Depth first dearch iterator over all elements in the model, regardless of their association with a file

#### identifiable_elements

```python
identifiable_elements: List[str]
```

List of all paths of identifiable elements in the model

## ArxmlFile

Represents a file that is part of an AutosarModel

### Functions

#### serialize

```python
def serialize(self) -> str
```

Serialize the the file to a string. This string can be loaded as valid arxml if is written to disk.

#### check_version_compatibility

```python
def check_version_compatibility(self, version: AutosarVersion) -> List[IncompatibleItemError]
```

check if the elements in the file would be compatible with the given target version.

Returns a list of compatibility errors, each of which is an IncompatibleElementError, IncompatibleAttributeError or IncompatibleAttributeValueError

### Class members

#### filename

```python
filename: str
```

filename of the arxml file. Must be unique within the model.

#### version

```python
version: AutosarVersion
```

Autosar version of the file

#### model

```python
model: AutosarModel
```

the autosar data model which this file is part of

#### elements_dfs

```python
elements_dfs: ArxmlFileElementsDfsIterator
```

dfs iterator over all elements in this file

#### xml_standalone

```python
xml_standalone: bool
```

contains the xml standalone attribute (if any) in the xml file header

## Element

An element in the Autosar data model

### Functions

#### create_sub_element

```python
def create_sub_element(
    self,
    element_name: ElementName,
    position: int = None
) -> Element
```

create a sub element under this element with the given ElementName (optionally at a specific position)

#### create_named_sub_element

```python
def create_named_sub_element(
    self,
    element_name: ElementName,
    item_name: str,
    position: int = None
) -> Element
```

create a named sub element under this element with the given ElementName (optionally at a specific position)

#### create_copied_sub_element

```python
def create_copied_sub_element(
    self,
    other: Element,
    position: int = None
) -> Element
```

create a copy of some other element (with all of its children) as a child of this element (optionally at a specific position)

#### move_element_here

```python
def move_element_here(
    self,
    move_element: Element,
    position: int = None
) -> Element
```

move an element from somewhere else in this model or from another model to become a child element (optionally at a specific position)

#### remove_sub_element

```python
def remove_sub_element(self, element: Element) -> None
```

remove a sub element and all of its content

#### get_sub_element

```python
def get_sub_element(self, name_str: str) -> Element
```

get a sub element by its element name. If there are several then this returns the first of them

#### get_sub_element_at

```python
def get_sub_element_at(self, position: int) -> Element
```

get an element by its position among the content of this element

#### get_named_sub_element

```python
def get_named_sub_element(self, item_name: str) -> Element:
```

Get the sub element with the given item name, if any

#### get_bsw_sub_element

```python
def get_bsw_sub_element(self, definition_ref: str) -> Element:
```

get the sub element with the given definition ref. It is possible to specify either the full definition ref, or only the last part after the final '/'

#### serialize

```python
def serialize(self) -> str
```

serialize this element and its sub elements int oa string. This string is valid xml, but it is not a vaild arxml file

#### remove_character_data

```python
def remove_character_data(self) -> None
```

remove the character data

#### insert_character_content_item

```python
def insert_character_content_item(self, chardata: str, position: int) -> None
```

for elements with ElementType mixed, this allows character data to be inserted at any point in the content of this element

#### remove_character_content_item

```python
def remove_character_content_item(self, position: int) -> None
```

remove one character content item from the given position

#### attribute_value

```python
def attribute_value(self, attrname: AttributeName) -> CharacterData
```

get the attribute value of a specific attribute. Returns None if that attribute is not set

#### set_attribute

```python
def set_attribute(self, attrname: AttributeName, chardata: CharacterData) -> None
```

set the given attribute to the provided value. If the attribute is valid for this element it will be created or modified as needed.

#### remove_attribute

```python
def remove_attribute(self, attrname: AttributeName) -> None
```

remove an attribute from the element

#### sort

```python
def sort(self) -> None
```

sort this element and all of its sub elements

#### list_valid_sub_elements

```python
def list_valid_sub_elements(self) -> List[ValidSubElementInfo]
```

provide information about valid sub elements as a list of ValidSubElementInfo

#### add_to_file

```python
def add_to_file(self, file: ArxmlFile) -> None
```

add the element to a file. if necessary all parent elements of this element also become part of the file

#### remove_from_file

```python
def remove_from_file(self, file: ArxmlFile) -> None
```

remove this element from a file. Does not affect parent elements. When an element is no longer part of any file it is deleted.

### Class members

#### parent

```python
parent: Element
```

reference to the parent of this element

#### element_name

```python
element_name: ElementName
```

ElementName of this element, e.g. AUTOSAR or AR-PACKAGE

#### element_type

```python
element_type: ElementType
```

Reference to the element type of the element in the specification

#### item_name

```python
item_name: str
```

item name of an identifiable element, or None for elements which are not identifiable.

Setting this value renames the element and updates all references to it.

#### is_identifiable

```python
is_identifiable: bool
```

true if the element is identifiable, false otherwise

#### is_reference

```python
is_reference: bool
```

true if the element can contain a reference to another element

#### path

```python
path: str
```

the full autosar path of an identifiable element

#### model

```python
model: AutosarModel
```

reference to the model containing this element

#### content_type

```python
content_type: ContentType
```

content type of the element: character data (`<X>some text</X>`), elements (`<X><Y></Y></X>`), or mixed

#### reference_target

```python
reference_target: Element
```

returns the target of the reference, if the element contains a reference

#### position

```python
position: int
```

the position of this element in the content of its parent

#### sub_elements

```python
sub_elements: ElementsIterator
```

an iterator over all sub elements in the content of this element. It skips character data content items

#### elements_dfs

```python
elements_dfs: ElementsDfsIterator
```

depth first search iterator for this element and all of its sub elements

#### character_data

```python
character_data: CharacterData
```

character content of this element, if any. For elements with ContentType=Element, or empty elements this is None

#### content_item_count

```python
content_item_count: int
```

number of content items (character data and/or sub elements)

#### content

```python
content: ElementContentIterator
```

iterator over all content of this element

#### attributes

```python
attributes: AttributeIterator
```

iterator over all attributes of this element

#### file_membership

```python
file_membership: Tuple[bool, FrozenSet[ArxmlFile]]
```

file membership information: the tuple (is_local, set(ArxmlFile)) tells if there is a restriction to file membership attached to this element, and which files the element is part of

#### xml_path

```python
xml_path: str
```

a path listing all xml elements from the root of the model to the element. This is intended for display. e.g. in error messages

## AutosarVersion

A version of the Autosar standard

- Autosar_4_0_1
- Autosar_4_0_2
- Autosar_4_0_3
- Autosar_4_1_1
- Autosar_4_1_2
- Autosar_4_1_3
- Autosar_4_2_1
- Autosar_4_2_2
- Autosar_4_3_0
- Autosar_00042
- Autosar_00043
- Autosar_00044
- Autosar_00045
- Autosar_00046
- Autosar_00047
- Autosar_00048
- Autosar_00049
- Autosar_00050
- Autosar_00051

```python
LATEST
```

Alias for the latest version of the standard

## CharacterData

```python
CharacterData: TypeAlias = Union[EnumItem, str, int, float]
```

## ElementType

### Functions

Type of an Element in the specification

#### splittable_in

```python
def splittable_in(self, version: AutosarVersion) -> bool
```

is this element splittable in a particular AutosarVersion

#### reference_dest_value

```python
def reference_dest_value(self, target: ElementType) -> EnumItem
```

helper to determine the correct value for the `DEST` attribute when setting a reference

#### find_sub_element

```python
def find_sub_element(
    self,
    target_name: ElementName,
    version: VersionSpecification
) -> ElementType
```

Find the ElementType of the named sub element in the specification of this ElementType

#### find_attribute_spec

```python
def find_attribute_spec(self, attribute_name: AttributeName) -> AttributeSpec
```

Find the specification for the given attribute name

### Class members

#### is_named

```python
is_named: bool
```

Elements of this type must have a `SHORT-NAME`

#### is_ref

```python
is_ref: bool
```

Elements of this type must contain an autosar path in their character data, and have a `DEST` attribute

#### is_ordered

```python
is_ordered: bool
```

Ordered elements may not be sorted, since the sub element order is semantically meaningful

#### splittable

```python
splittable: List[AutosarVersion]
```

A list of AutosarVersions in which this element is splittable

#### std_restriction

```python
std_restriction: str
```

Indicates if the element type is restricted to a particular version.
std_restriction contains a string, one of 'NotRestricted', 'ClassicPlatform' or 'AdaptivePlatform'.

#### chardata_spec

```python
chardata_spec: CharacterDataType
```

The specification of the character data content of elements of this type

#### attributes_spec

```python
attributes_spec: List[AttributeSpec]
```

A list of the specifications of all attributes allowed on elements of this type

## IncompatibleItemError

```python
IncompatibleItemError: TypeAlias = Union[
    IncompatibleAttributeError,
    IncompatibleAttributeValueError,
    IncompatibleElementError
]
```

## IncompatibleAttributeError

Information about an attribute that is incompatible with a given target version

### Class members

#### element

```python
element: Element
```

Element which contains the incompatible attribute

#### attribute

```python
attribute: AttributeName
```

Incompatible attribute

#### allowed_versions

```python
allowed_versions: List[AutosarVersion]
```

list of versions in which the attribute is permitted on this element

## IncompatibleAttributeValueError

Information about an attribute value that is incompatible with a given target version

### Class members

#### element

```python
element: Element
```

Element which contains the incompatible attribute value

#### attribute

```python
attribute: AttributeName
```

Attribute which contains the invalid value

#### attribute_value

```python
attribute_value: str
```

The incompatible attribute value

#### allowed_versions

```python
allowed_versions: List[AutosarVersion]
```

list of versions in which the attribute value is permitted on this attribute

## IncompatibleElementError

Information about an element that is incompatible with a given target version

### Class members

#### element

```python
element: Element
```

incompatible element

#### allowed_versions

```python
allowed_versions: List[AutosarVersion]
```

list of versions in which this element is compatible

## ValidSubElementInfo

Details about a particular sub element

### Class members

#### element_name

```python
element_name: str
```

name of the potential sub element

#### is_named

```python
is_named: bool
```

is the sub element named, i.e. does it need to be created with create_named_sub_element

#### is_allowed

```python
is_allowed: bool
```

is the sub element currently allowed, given the existing content of the element. Note that some sub elements are mutually exclusive

## AttributeSpec

The specification of an attribute

### Class members

#### attribute_name

```python
attribute_name: str
```

name of the attribute

#### value_spec

```python
value_spec: CharacterDataType
```

specification of the attribute value

#### required

```python
required: bool
```

is the attribute required or optional

## Globals

### Functions

#### check_file

```python
def check_file(filename: str)
```

Check if the file contains arxml data. Returns true if an arxml file header is found and does not parse anything after it.

#### check_buffer

```python
def check_buffer(filename: bytes)
```

Check if the buffer contains arxml data. Returns true if an arxml file header is found and does not parse anything after it.

### Constants

#### \_\_version\_\_

```python
__version__: str
```

Version of the running autosar_data module.
It contains a semver string of the form 'x.y.z'
