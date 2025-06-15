# Autosar Data for Python

The autosar_data package provides a way to create or modify autosar data (ARXML files) in Python.

There are two ways to interact with the autosar data model:

- The basic API allows access to any valid element in the autosar data model.
  This API guarantees that all modifications are valid: The element hierarchy and content data types are enforced.
  API description wioth usage examples: [Basic API](api.md)
- The abstraction API builds on the basic API to provide a high-level view many autosar concepts
  API description wioth usage examples: [Abstraction API](abstraction_api.md)

autosar_data for Python is a wrapper around the Rust crates [autosar-data](https://docs.rs/autosar-data/latest/autosar_data/) and [autosar-data-abstraction](https://docs.rs/autosar-data-abstraction/latest/autosar_data_abstraction/).
