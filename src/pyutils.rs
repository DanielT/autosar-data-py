use pyo3::{prelude::*, types::PyList};

/// convert any slice of items to a Py<PyList>
/// There must be some function that converts a single item to a Py<PyAny>
pub(crate) fn slice_to_pylist<T>(
    py: Python<'_>,
    slice: &[T],
    convert: impl Fn(&T) -> PyResult<Py<PyAny>>,
) -> PyResult<Py<PyList>> {
    let object_vec: Vec<_> = slice
        .iter()
        .map(|item| -> PyResult<Py<PyAny>> { convert(item) })
        .collect::<PyResult<Vec<_>>>()?;
    PyList::new(py, object_vec).map(|x| x.unbind())
}

/// convert a Py<PyList> to a vector of items
/// There must be some function that converts a Py<PyAny> to an item
pub(crate) fn pylist_to_vec<T>(
    py: Python<'_>,
    seq: &Py<PyList>,
    convert: impl Fn(&Bound<'_, PyAny>) -> PyResult<T>,
) -> PyResult<Vec<T>> {
    seq.bind(py)
        .as_sequence()
        .try_iter()?
        .map(|elem| convert(&elem?))
        .collect::<PyResult<Vec<_>>>()
}

/// compare two PyLists of items that are pyclasses defined in Rust code
pub(crate) fn compare_pylist(py: Python, seq1: &Py<PyList>, seq2: &Py<PyList>) -> bool {
    if let (Ok(seq1_len), Ok(seq2_len)) = (
        seq1.bind_borrowed(py).as_sequence().len(),
        seq2.bind_borrowed(py).as_sequence().len(),
    ) {
        // first, make sure the lengths are the same, since iter.zip stops when the shorter iterator is exhausted
        if seq1_len != seq2_len {
            // lengths are not equal
            return false;
        }

        let mut seq1_try_iter = seq1.bind_borrowed(py).as_sequence().try_iter();
        let mut seq2_try_iter = seq2.bind_borrowed(py).as_sequence().try_iter();
        if let (Ok(seq1_iter), Ok(seq2_iter)) = (&mut seq1_try_iter, &mut seq2_try_iter) {
            seq1_iter.zip(seq2_iter).all(|(item1, item2)| {
                if let (Ok(item1), Ok(item2)) = (item1, item2) {
                    // both items are valid, we can compare them
                    item1.compare(&item2).unwrap_or(std::cmp::Ordering::Less)
                        == std::cmp::Ordering::Equal
                } else {
                    // at least one of the items is not valid
                    false
                }
            })
        } else {
            // could not get iterators for the sequences - it's not clear that this case is reachable, since we're able to get lengths
            false
        }
    } else {
        // could not get lengths for the sequences. At least one of them is not a sequence.
        false
    }
}
