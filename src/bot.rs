use crate::capture;
use numpy::{IntoPyArray, PyArray3};
use pyo3::{exceptions::PyValueError, prelude::*};

#[pyclass]
pub struct MinecraftBot {
    window_id: u32,
}

#[pymethods]
impl MinecraftBot {
    #[new]
    fn new(window_id: u32) -> Self {
        MinecraftBot { window_id }
    }

    fn get_screen<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyArray3<u8>>> {
        let (width, height, pixels) =
            capture::capture_window(self.window_id).map_err(|e| PyValueError::new_err(e))?;

        let np_array = numpy::ndarray::Array3::from_shape_vec((height, width, 4), pixels)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;

        Ok(np_array.into_pyarray(py))
    }
}
