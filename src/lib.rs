use numpy::{IntoPyArray, PyArray3};
use pyo3::prelude::*;

#[pyclass]
struct MinecraftBot {
    window_id: u32,
}

#[pymethods]
impl MinecraftBot {
    #[new]
    fn new(window_id: u32) -> Self {
        MinecraftBot { window_id }
    }

    fn say_hello(&self) -> String {
        format!("Hello, I am {}", self.window_id)
    }

    fn get_screen<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyArray3<u8>>> {
        let pixels = vec![0u8; 120 * 160 * 3];

        let np_array = numpy::ndarray::Array3::from_shape_vec((120, 160, 3), pixels).unwrap();

        Ok(np_array.into_pyarray(py))
    }
}

#[pymodule]
fn rmc_env(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MinecraftBot>()?;
    Ok(())
}
