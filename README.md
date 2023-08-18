# `autosar-data-py`

Python bindings for [autosar-data](https://github.com/DanielT/autosar-data)

## Features

This crate implements Python bindings for autosar-data using [PyO3](https://pyo3.rs). This allows all the features of `autosar-data` to be used from python code:

- read and write arxml files
- fully validate all data when it is loaded in strict mode
- non-strict mode so that invalid but structurally sound data can be loaded
- various element operations to modify and create sub-elements, data and attributes
- support for Autosar paths and cross references
- supports Autosar version 4.0.1 and up.

## Example

```python
from autosar_data import *

# create a new data model
model = AutosarModel()

```

## Development

- maturin must be installed: `pip install maturin` if it isn't
- create a venv in the cloned source: `python -m venv .venv`
- build the wheel and directly install it in the venv: `maturin develop`
- activate the venv in a shell: `.venv/Scripts/activate` or `.venv\Scripts\Activate.ps1`
- run python in the shell with the venv
