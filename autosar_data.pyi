"""
Provides functionality to read, modify and write Autosar arxml files,
both separately and in projects consisting of multiple files.

Classes:

- ArxmlFile
- AutosarModel
- AutosarVersion
- Element
- ElementType
- ValidSubElementInfo

Variables:

- __version__

"""

from .autosar_data import *
from typing import Dict, List, FrozenSet, Literal, TypeAlias, Tuple, Union

IncompatibleItemError: TypeAlias = Union[IncompatibleAttributeError, IncompatibleAttributeValueError, IncompatibleElementError]
ElementName: TypeAlias = str # ~5900 variants is too many to list here
AttributeName: TypeAlias = Literal["ACCESSKEY", "ALIGN", "ALLOW-BREAK", "ALT", "BASE", "BGCOLOR", "BINDING-TIME", "BLUEPRINT-VALUE", "BREAK", "CLASS", "COLNAME", "COLNUM", "COLOR", "COLS", "COLSEP", "COLWIDTH", "COORDS", "DEST", "EDIT-HEIGHT", "EDIT-WIDTH", "EDITFIT", "EDITSCALE", "ENUM-TABLE", "FILENAME", "FIT", "FLOAT", "FONT", "FRAME", "GENERATOR", "GID", "HEIGHT", "HELP-ENTRY", "HREF", "HTML-FIT", "HTML-HEIGHT", "HTML-SCALE", "HTML-WIDTH", "INDEX", "INTERVAL-TYPE", "ITEM-LABEL-POS", "KEEP-WITH-PREVIOUS", "L", "LEVEL", "MIME-TYPE", "MOREROWS", "NAME", "NAME-PATTERN", "NAMEEND", "NAMEST", "NOHREF", "NOTATION", "NOTE-TYPE", "ONBLUR", "ONCLICK", "ONDBLCLICK", "ONFOCUS", "ONKEYDOWN", "ONKEYPRESS", "ONKEYUP", "ONMOUSEDOWN", "ONMOUSEMOVE", "ONMOUSEOUT", "ONMOUSEOVER", "ONMOUSEUP", "ORIENT", "PGWIDE", "RESOLUTION-POLICY", "ROTATE", "ROWSEP", "S", "SCALE", "SD", "SHAPE", "SHORT-LABEL", "SHOW-CONTENT", "SHOW-RESOURCE-ALIAS-NAME", "SHOW-RESOURCE-CATEGORY", "SHOW-RESOURCE-LONG-NAME", "SHOW-RESOURCE-NUMBER", "SHOW-RESOURCE-PAGE", "SHOW-RESOURCE-SHORT-NAME", "SHOW-RESOURCE-TYPE", "SHOW-SEE", "SI", "SPANNAME", "STYLE", "T", "TABINDEX", "TABSTYLE", "TEX-RENDER", "TITLE", "TYPE", "UUID", "VALIDITY", "VALIGN", "VIEW", "WIDTH", "xml:space", "xmlns", "xmlns:xsi", "xsi:schemaLocation"]
EnumItem: TypeAlias = str # ~2500 variants is too many to list here
CharacterData: TypeAlias = Union[EnumItem, str, int, float]
ElementContent: TypeAlias = Union[Element, CharacterData]
VersionSpecification: TypeAlias = Union[AutosarVersion, List[AutosarVersion]]
CharacterDataType: TypeAlias = Union[CharacterDataTypeEnum, CharacterDataTypeFloat, CharacterDataTypeRestrictedString, CharacterDataTypeString, CharacterDataTypeUnsignedInt]

class ArxmlFile:
    """
    Represents a file that is part of an AutosarModel
    """
    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...
    filename: str
    """filename of the arxml file. Must be unique within the model."""
    version: AutosarVersion
    """Autosar version of the file"""
    def check_version_compatibility(self, version: AutosarVersion) -> List[IncompatibleItemError]:
        """
        check if the elements in the file would be compatible with the given target version.
        
        returns a list of compatibility errors, each of which is an IncompatibleElementError, IncompatibleAttributeError or IncompatibleAttributeValueError
        """
        ...
    model: AutosarModel
    """the autosar data model which this file is part of"""
    elements_dfs: ArxmlFileElementsDfsIterator
    """dfs iterator over all elements in this file"""
    def serialize(self) -> str:
        """serialize the the file to a string. This string can be loaded as valid arxml if is written to disk."""
        ...
    xml_standalone: bool
    """contains the xml standalone attribute (if any) in the xml file header"""

class ArxmlFileElementsDfsIterator:
    """
    A depth first search iterator over all elements contained in the file that created this iterator
    """
    def __iter__(self) -> ArxmlFileElementsDfsIterator: ...
    def __next__(self) -> Tuple[int, Element]: ...

class Attribute:
    """
    An attribute on an element
    """
    attrname: AttributeName
    """name of this attribute"""
    content: CharacterData
    """content of the attribute - this data can be free-form text, a pre-defined enum value (str), or very rarely a float or int"""

class AttributeIterator:
    """
    Iterates over all attributes on an element
    """
    def __iter__(self) -> AttributeIterator: ...
    def __next__(self) -> Attribute : ...

class AutosarDataError(Exception):
    pass

class AutosarModel:
    """
    Autosar data model. It contains all elements.
    """
    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...
    def create_file(self, filename: str, version: AutosarVersion = AutosarVersion.LATEST) -> ArxmlFile:
        """create a new file in the model"""
        ...
    def load_buffer(self, buffer: str, filename: str, strict: bool = False) -> Tuple[ArxmlFile, List[str]]:
        """load a buffer (string) as arxml"""
        ...
    def load_file(self, filename: str, strict: bool = False) -> Tuple[ArxmlFile, List[str]]:
        """load a file as arxml"""
        ...
    def remove_file(self, arxmlfile: ArxmlFile) -> None:
        """remove a file from the model. Any elements belonging exclusively to that file will also be removed."""
        ...
    def serialize_files(self) -> Dict[str, str]:
        """serialize all files individually, to generate a dict(filename, serialized content),"""
        ...
    def write(self) -> None:
        """write all files in the model to disk"""
        ...
    files: List[ArxmlFile]
    """a list of ArxmlFile objects containing all files in the model"""
    root_element: Element
    """The root element of the model, <AUTOSAR>"""
    def get_element_by_path(self, autosar_path: str) -> Element:
        """get an identifiable element in the model by its Autosar path"""
        ...
    elements_dfs: ElementsDfsIterator
    """depth first dearch iterator over all elements in the model, regardless of their association with a file"""
    def sort(self) -> None:
        """sort the entire model in place. Takes all ordering constraints into account."""
        ...
    identifiable_elements: IdentifiablesIterator
    """iterator over all identifiable elements in the model"""
    def get_references_to(self, target_path: str) -> List[Element]:
        """get all reference elements which refer to the given Autosar path"""
        ...
    def check_references(self) -> List[Element]:
        """check all references in the model and return a list of elements containing invalid references"""
        ...
    def duplicate(self) -> AutosarModel:
        """create a fully independent copy of the model"""
        ...

class AutosarVersion:
    """
    A version of the Autosar standard
    """
    def __new__(cls, verstring: str) -> AutosarVersion: ...
    # this is the stupid result of method used by PyO3 to translate Rust enums
    Autosar_4_0_1: AutosarVersion
    Autosar_4_0_2: AutosarVersion
    Autosar_4_0_3: AutosarVersion
    Autosar_4_1_1: AutosarVersion
    Autosar_4_1_2: AutosarVersion
    Autosar_4_1_3: AutosarVersion
    Autosar_4_2_1: AutosarVersion
    Autosar_4_2_2: AutosarVersion
    Autosar_4_3_0: AutosarVersion
    Autosar_00042: AutosarVersion
    Autosar_00043: AutosarVersion
    Autosar_00044: AutosarVersion
    Autosar_00045: AutosarVersion
    Autosar_00046: AutosarVersion
    Autosar_00047: AutosarVersion
    Autosar_00048: AutosarVersion
    Autosar_00049: AutosarVersion
    Autosar_00050: AutosarVersion
    Autosar_00051: AutosarVersion
    Autosar_00052: AutosarVersion

class ContentType:
    """
    The content type of an element
    """
    # this is the stupid result of method used by PyO3 to translate Rust enums
    Elements: ContentType
    CharacterData: ContentType
    Mixed: ContentType

class Element:
    """
    An element in the Autosar data model
    """
    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...
    def serialize(self) -> str:
        """serialize this element and its sub elements int oa string. This string is valid xml, but it is not a vaild arxml file"""
        ...
    parent: Element
    """reference to the parent of this element"""
    named_parent: Element
    """reference to the next named (grand-)parent of this element"""
    element_name: ElementName
    """ElementName of this element, e.g. AUTOSAR or AR-PACKAGE"""
    element_type: ElementType
    """Reference to the element type of the element in the specification"""
    item_name: str
    """item name of an identifiable element, or None for elements which are not identifiable.

    Setting this value renames the element and updates all references to it.
    """
    is_identifiable: bool
    """true if the element is identifiable, false otherwise"""
    is_reference: bool
    """true if the element can contain a reference to another element"""
    path: str
    """the full autosar path of an identifiable element"""
    model: AutosarModel
    """reference to the model containing this element"""
    content_type: ContentType
    """content type of the element: character data (<X>some text</X>), elements (<X><Y></Y></X>), or Mixed"""
    comment: str
    """XML comment attached to this element"""
    def create_sub_element(self, element_name: ElementName, position: int = None) -> Element:
        """create a sub element under this element with the given ElementName (optionally at a specific position)"""
        ...
    def create_named_sub_element(self, element_name: ElementName, item_name: str, position: int = None) -> Element:
        """create a named sub element under this element with the given ElementName (optionally at a specific position)"""
        ...
    def create_copied_sub_element(self, other: Element, position: int = None) -> Element:
        """create a copy of some other element (with all of its children) as a child of this element (optionally at a specific position)"""
        ...
    def move_element_here(self, move_element: Element, position: int = None) -> Element:
        """move an element from somewhere else in this model or from another model to become a child element (optionally at a specific position)"""
        ...
    def remove_sub_element(self, element: Element) -> None:
        """remove a sub element and all of its content"""
        ...
    reference_target: Element
    """returns the target of the reference, if the element contains a reference"""
    def get_sub_element(self, name_str: str) -> Element:
        """get a sub element by its element name. If there are several then this returns the first of them"""
        ...
    def get_sub_element_at(self, position: int) -> Element:
        """get an element by its position among the content of this element"""
        ...
    def get_named_sub_element(self, item_name: str) -> Element:
        """get the sub element with the given item name, if any"""
        ...
    def get_bsw_sub_element(self, definition_ref: str) -> Element:
        """get the sub element with the given definition ref. It is possible to specify either the full definition ref, or only the last part after the final '/'"""
        ...
    def get_or_create_sub_element(self, name_str: str) -> Element:
        """get an existing sub element or create it if it does not exist"""
        ...
    def get_or_create_named_sub_element(self, name_str: str) -> Element:
        """get an existing named sub element or create it if it does not exist"""
        ...
    position: int
    """the position of this element in the content of its parent"""
    sub_elements: ElementsIterator
    """an iterator over all sub elements in the content of this element. It skips character data content items"""
    elements_dfs: ElementsDfsIterator
    """depth first search iterator for this element and all of its sub elements"""
    character_data: CharacterData
    """character content of this element, if any. For elements with ContentType=Element, or empty elements this is None"""
    def remove_character_data(self) -> None:
        """remove the character data"""
        ...
    def insert_character_content_item(self, chardata: str, position: int) -> None:
        """for elements with ElementType mixed, this allows character data to be inserted at any point in the content of this element"""
        ...
    def remove_character_content_item(self, position: int) -> None:
        """remove one character content item from the given position"""
        ...
    content_item_count: int
    """number of content items (character data and/or sub elements)"""
    content: ElementContentIterator
    """iterator over all content of this element"""
    attributes: AttributeIterator
    """iterator over all attributes of this element"""
    def attribute_value(self, attrname: AttributeName) -> CharacterData:
        """get the attribute value of a specific attribute. Returns None if that attribute is not set"""
        ...
    def set_attribute(self, attrname: AttributeName, chardata: CharacterData) -> None:
        """set the given attribute to the provided value. If the attribute is valid for this element it will be created or modified as needed."""
        ...
    def remove_attribute(self, attrname: AttributeName) -> None:
        """remove an attribute from the element"""
        ...
    def sort(self) -> None:
        """sort this element and all of its sub elements"""
        ...
    def list_valid_sub_elements(self) -> List[ValidSubElementInfo]:
        """provide information about valid sub elements as a list of ValidSubElementInfo"""
        ...
    file_membership: Tuple[bool, FrozenSet[ArxmlFile]]
    """file membership information: the tuple (is_local, set(ArxmlFile)) tells if there is a restriction to file membership attached to this element, and which files the element is part of"""
    def add_to_file(self, file: ArxmlFile) -> None:
        """add the element to a file. if necessary all parent elements of this element also become part of the file"""
        ...
    def remove_from_file(self, file: ArxmlFile) -> None:
        """remove this element from a file. Does not affect parent elements. When an element is no longer part of any file it is deleted."""
        ...
    xml_path: str
    """a path listing all xml elements from the root of the model to the element. This is intended for display. e.g. in error messages"""
    min_version: AutosarVersion
    """the autosar version of the file containing the element. If multiple files in a merged model contain the element, then this is the minimum of the file versions."""

class ElementContentIterator:
    """
    Iterates over all content in an element

    Content items an be sub elements or character data
    """
    def __iter__(self) -> ElementContentIterator: ...
    def __next__(self) -> ElementContent: ...

class ElementType:
    """
    Type of an Element in the specification
    """
    is_named: bool
    """Elements of this type must have a SHORT-NAME"""
    is_ref: bool
    """elements of this type must contain an autosar path in their character data, and have a DEST attribute"""
    is_ordered: bool
    """ordered elements may not be sorted, since the sub element order is semantically meaningful"""
    splittable: List[AutosarVersion]
    """a list of AutosarVersions in which this element is splittable"""
    std_restriction: str
    """a string indication if the element is restricted to ClassicPlatform, AdaptivePlatform or NotRestricted"""
    def splittable_in(self, version: AutosarVersion) -> bool:
        """is this element splittable in a particular AutosarVersion"""
        ...
    def reference_dest_value(self, target: ElementType) -> EnumItem:
        """helper to determine the correct value for the DEST attribute when setting a reference"""
        ...
    def find_sub_element(self, target_name: ElementName, version: VersionSpecification) -> ElementType:
        """find the ElementType of the named sub element in the specification of this ElementType"""
        ...
    chardata_spec: CharacterDataType
    """the specification of the character data content of elements of this type"""
    attributes_spec: List[AttributeSpec]
    """a list of the specifications of all attributes allowed on elements of this type"""
    def find_attribute_spec(self, attribute_name: AttributeName) -> AttributeSpec:
        """find the specification for the given attribute name"""
        ...

class ElementsDfsIterator:
    """
    Dpeth first search iterator starting at the element which created the iterator
    """
    def __iter__(self) -> ElementsDfsIterator: ...
    def __next__(self) -> Tuple[int, Element]: ...

class ElementsIterator:
    """
    Iterator over all sub elements of an element
    """
    def __iter__(self) -> ElementsIterator: ...
    def __next__(self) -> Element: ...

class IdentifiablesIterator:
    """
    Iterator of all identifiable elements in the model. It provides the tuple (path, Element) for each entry.
    """
    def __iter__(self) -> IdentifiablesIterator: ...
    def __next__(self) -> Tuple[str, Element]: ...

class IncompatibleAttributeError:
    """
    Information about an attribute that is incompatible with a given target version
    """
    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...
    element: Element
    """Element which contains the incompatible attribute"""
    attribute: AttributeName
    """Incompatible attribute"""
    allowed_versions: List[AutosarVersion]
    """list of versions in which the attribute is permitted on this element"""

class IncompatibleAttributeValueError:
    """
    Information about an attribute value that is incompatible with a given target version
    """
    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...
    element: Element
    """Element which contains the incompatible attribute value"""
    attribute: AttributeName
    """Attribute which contains the invalid value"""
    attribute_value: str
    """The incompatible attribute value"""
    allowed_versions: List[AutosarVersion]
    """list of versions in which the attribute value is permitted on this attribute"""

class IncompatibleElementError:
    """
    Information about an element that is incompatible with a given target version
    """
    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...
    element: Element
    """incompatible element"""
    allowed_versions: List[AutosarVersion]
    """list of versions in which this element is compatible"""

class ValidSubElementInfo:
    """
    Details about a particular sub element
    """
    element_name: str
    """name of the potential sub element"""
    is_named: bool
    """is the sub element named, i.e. does it need to be created with create_named_sub_element"""
    is_allowed: bool
    """is the sub element currently allowed, given the existing content of the element. Note that some sub elements are mutually exclusive"""

class AttributeSpec:
    """The specification of an attribute"""
    attribute_name: str
    """name of the attribute"""
    value_spec: CharacterDataType
    """specification of the attribute value"""
    required: bool
    """is the attribute required or optional"""

class CharacterDataTypeEnum:
    """Character data type: enum"""
    values: List[str]
    """List of valid enum values"""

class CharacterDataTypeFloat:
    """Character data type: float"""
    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...

class CharacterDataTypeRestrictedString:
    """Character data type: restricted string"""
    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...
    regex: str
    """to be valid, a string must match this regex"""

class CharacterDataTypeString:
    """Character data type: string"""
    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...

class CharacterDataTypeUnsignedInt:
    """Character data type: unsigned int"""
    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...

def check_file(filename: str):
    """Check if the file contains arxml data. Returns true if an arxml file header is found and does not parse anything after it."""
    ...

def check_buffer(filename: bytes):
    """Check if the buffer contains arxml data. Returns true if an arxml file header is found and does not parse anything after it."""
    ...

__version__: str
"""
Version of the running autosar_data module.
It contains a semver string of the form 'x.y.z'
"""