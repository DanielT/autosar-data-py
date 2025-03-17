use crate::abstraction::AutosarAbstractionError;
use crate::{abstraction::*, *};
use autosar_data_abstraction::{self, AbstractionElement, IdentifiableAbstractionElement};

/// A network endpoint contains address information for a connection
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Clone, PartialEq)]
pub(crate) struct NetworkEndpoint(
    pub(crate) autosar_data_abstraction::communication::NetworkEndpoint,
);

#[pymethods]
impl NetworkEndpoint {
    #[new]
    fn new(element: &Element) -> PyResult<Self> {
        match autosar_data_abstraction::communication::NetworkEndpoint::try_from(element.0.clone())
        {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(AutosarAbstractionError::new_err(e.to_string())),
        }
    }

    #[setter]
    fn set_name(&self, name: &str) -> PyResult<()> {
        self.0.set_name(name).map_err(abstraction_err_to_pyerr)
    }

    #[getter]
    fn name(&self) -> Option<String> {
        self.0.name()
    }

    #[getter]
    fn element(&self) -> Element {
        Element(self.0.element().clone())
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self.0)
    }

    /// add a network endpoint address to this `NetworkEndpoint`
    ///
    /// A `NetworkEndpoint` may have multiple sets of address information. The following restrictions apply:
    ///
    /// - all addresses must have the same type, i.e. all IPv4 or all IPv6
    /// - only one of them may be a `Fixed` address, all others must be dynamic (DHCP, automatic link local, etc.)
    #[pyo3(signature = (address, /))]
    #[pyo3(text_signature = "(self, address: NetworkEndpointAddress, /)")]
    fn add_network_endpoint_address(&self, address: NetworkEndpointAddress) -> PyResult<()> {
        self.0
            .add_network_endpoint_address(address.into())
            .map_err(abstraction_err_to_pyerr)
    }

    /// iterator over all addresses in the `NetworkEndpoint`
    fn addresses(&self) -> NetworkEndpointAddressIterator {
        NetworkEndpointAddressIterator::new(self.0.addresses().map(NetworkEndpointAddress::from))
    }
}

//##################################################################

iterator_wrapper!(NetworkEndpointIterator, NetworkEndpoint);

//##################################################################

/// address information for a network endpoint
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkEndpointAddress {
    /// IPv4 addressing information
    #[pyo3(constructor = (*, address=None, address_source=None, default_gateway=None, network_mask=None))]
    IPv4 {
        /// IPv4 address in the form "a.b.c.d". This is used if the address source is FIXED
        address: Option<String>,
        /// defines how the address is obtained
        address_source: Option<IPv4AddressSource>,
        /// ip address of the default gateway
        default_gateway: Option<String>,
        /// Network mask in the form "a.b.c.d"
        network_mask: Option<String>,
    },
    /// IPv6 addressing information
    #[pyo3(constructor = (*, address=None, address_source=None, default_router=None))]
    IPv6 {
        /// IPv6 address, without abbreviation
        address: Option<String>,
        /// defines how the address is obtained
        address_source: Option<IPv6AddressSource>,
        /// IP address of the default router
        default_router: Option<String>,
    },
}

impl From<NetworkEndpointAddress>
    for autosar_data_abstraction::communication::NetworkEndpointAddress
{
    fn from(value: NetworkEndpointAddress) -> Self {
        match value {
            NetworkEndpointAddress::IPv4 {
                address,
                address_source,
                default_gateway,
                network_mask,
            } => Self::IPv4 {
                address: address.clone(),
                address_source: address_source.map(std::convert::Into::into),
                default_gateway: default_gateway.clone(),
                network_mask: network_mask.clone(),
            },
            NetworkEndpointAddress::IPv6 {
                address,
                address_source,
                default_router,
            } => Self::IPv6 {
                address: address.clone(),
                address_source: address_source.map(std::convert::Into::into),
                default_router: default_router.clone(),
            },
        }
    }
}

impl From<autosar_data_abstraction::communication::NetworkEndpointAddress>
    for NetworkEndpointAddress
{
    fn from(value: autosar_data_abstraction::communication::NetworkEndpointAddress) -> Self {
        match value {
            autosar_data_abstraction::communication::NetworkEndpointAddress::IPv4 {
                address,
                address_source,
                default_gateway,
                network_mask,
            } => Self::IPv4 {
                address: address.clone(),
                address_source: address_source.map(std::convert::Into::into),
                default_gateway: default_gateway.clone(),
                network_mask: network_mask.clone(),
            },
            autosar_data_abstraction::communication::NetworkEndpointAddress::IPv6 {
                address,
                address_source,
                default_router,
            } => Self::IPv6 {
                address: address.clone(),
                address_source: address_source.map(std::convert::Into::into),
                default_router: default_router.clone(),
            },
        }
    }
}

#[pymethods]
impl NetworkEndpointAddress {
    fn __repr__(&self) -> String {
        match self {
            NetworkEndpointAddress::IPv4 {
                address,
                address_source,
                default_gateway,
                network_mask,
            } => format!(
                "NetworkEndpointAddress.IPv4(address={}, address_source={:?}, default_gateway={}, network_mask={})",
                address.as_deref().unwrap_or("None"),
                address_source
                    .as_ref()
                    .map_or("None".to_string(), |v| format!("{v:?}")),
                default_gateway.as_deref().unwrap_or("None"),
                network_mask.as_deref().unwrap_or("None")
            ),
            NetworkEndpointAddress::IPv6 {
                address,
                address_source,
                default_router,
            } => format!(
                "NetworkEndpointAddress.IPv6(address={}, address_source={:?}, default_router={})",
                address.as_deref().unwrap_or("None"),
                address_source
                    .as_ref()
                    .map_or("None".to_string(), |v| format!("{v:?}")),
                default_router.as_deref().unwrap_or("None")
            ),
        }
    }
}

//##################################################################

iterator_wrapper!(NetworkEndpointAddressIterator, NetworkEndpointAddress);

//##################################################################

/// `IPv4AddressSource` defines how the address of an IPv4 `NetworkEndpoint` is obtained
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IPv4AddressSource {
    /// use `AutoIp` (aka APIPA) to assign a link-local address
    AutoIp,
    /// use `AutoIp` with `DoIp` settings to assign a link-local address
    AutoIpDoIp,
    /// dynamic assignment using DHCP
    DHCPv4,
    /// static IP address configuration - the address must be specified in `NetworkEndpointAddress`
    Fixed,
}

impl From<IPv4AddressSource> for autosar_data_abstraction::communication::IPv4AddressSource {
    fn from(value: IPv4AddressSource) -> Self {
        match value {
            IPv4AddressSource::AutoIp => Self::AutoIp,
            IPv4AddressSource::AutoIpDoIp => Self::AutoIpDoIp,
            IPv4AddressSource::DHCPv4 => Self::DHCPv4,
            IPv4AddressSource::Fixed => Self::Fixed,
        }
    }
}

impl From<autosar_data_abstraction::communication::IPv4AddressSource> for IPv4AddressSource {
    fn from(value: autosar_data_abstraction::communication::IPv4AddressSource) -> Self {
        match value {
            autosar_data_abstraction::communication::IPv4AddressSource::AutoIp => Self::AutoIp,
            autosar_data_abstraction::communication::IPv4AddressSource::AutoIpDoIp => {
                Self::AutoIpDoIp
            }
            autosar_data_abstraction::communication::IPv4AddressSource::DHCPv4 => Self::DHCPv4,
            autosar_data_abstraction::communication::IPv4AddressSource::Fixed => Self::Fixed,
        }
    }
}

//##################################################################

/// `IPv6AddressSource` defines how the address of an IPv6 `NetworkEndpoint` is obtained
#[pyclass(
    frozen,
    eq,
    module = "autosar_data._autosar_data._abstraction._communication"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IPv6AddressSource {
    /// dynamic assignment using DHCP
    DHCPv6,
    /// static IP address configuration - the address must be specified in `NetworkEndpointAddress`
    Fixed,
    /// automatic link local address assignment
    LinkLocal,
    /// automatic link local address assignment using doip parameters
    LinkLocalDoIp,
    /// IPv6 stateless autoconfiguration
    RouterAdvertisement,
}

impl From<IPv6AddressSource> for autosar_data_abstraction::communication::IPv6AddressSource {
    fn from(value: IPv6AddressSource) -> Self {
        match value {
            IPv6AddressSource::DHCPv6 => Self::DHCPv6,
            IPv6AddressSource::Fixed => Self::Fixed,
            IPv6AddressSource::LinkLocal => Self::LinkLocal,
            IPv6AddressSource::LinkLocalDoIp => Self::LinkLocalDoIp,
            IPv6AddressSource::RouterAdvertisement => Self::RouterAdvertisement,
        }
    }
}

impl From<autosar_data_abstraction::communication::IPv6AddressSource> for IPv6AddressSource {
    fn from(value: autosar_data_abstraction::communication::IPv6AddressSource) -> Self {
        match value {
            autosar_data_abstraction::communication::IPv6AddressSource::DHCPv6 => Self::DHCPv6,
            autosar_data_abstraction::communication::IPv6AddressSource::Fixed => Self::Fixed,
            autosar_data_abstraction::communication::IPv6AddressSource::LinkLocal => {
                Self::LinkLocal
            }
            autosar_data_abstraction::communication::IPv6AddressSource::LinkLocalDoIp => {
                Self::LinkLocalDoIp
            }
            autosar_data_abstraction::communication::IPv6AddressSource::RouterAdvertisement => {
                Self::RouterAdvertisement
            }
        }
    }
}
