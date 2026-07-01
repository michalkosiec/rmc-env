use crate::{
    capture,
    input::{
        init_virtual_device, key_down as key_down_input, key_up as key_up_input,
        mouse_down as mouse_down_input, mouse_up as mouse_up_input, send_click as send_click_input,
        send_command, send_mouse_move as send_mouse_move_input,
    },
};
use numpy::{IntoPyArray, PyArray3};
use pyo3::{
    exceptions::{PyRuntimeError, PyValueError},
    prelude::*,
};

#[pyclass(unsendable)]
pub struct MinecraftBot {
    window_id: u32,
    device: uinput::Device,
}

#[pymethods]
impl MinecraftBot {
    #[new]
    fn new(window_id: u32) -> PyResult<Self> {
        let device = init_virtual_device().map_err(|e| {
            PyRuntimeError::new_err(format!("Failed to initialize virtual device: {}", e))
        })?;
        Ok(MinecraftBot { window_id, device })
    }

    fn get_screen<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyArray3<u8>>> {
        let (width, height, pixels) =
            capture::capture_window(self.window_id).map_err(|e| PyValueError::new_err(e))?;

        let np_array = numpy::ndarray::Array3::from_shape_vec((height, width, 4), pixels)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;

        Ok(np_array.into_pyarray(py))
    }

    fn send_command(&mut self, action: &str) -> PyResult<()> {
        send_command(&mut self.device, action)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        Ok(())
    }

    fn send_click(&mut self, button: &str) -> PyResult<()> {
        send_click_input(&mut self.device, button)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        Ok(())
    }

    fn key_down(&mut self, action: &str) -> PyResult<()> {
        key_down_input(&mut self.device, action)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        Ok(())
    }

    fn key_up(&mut self, action: &str) -> PyResult<()> {
        key_up_input(&mut self.device, action)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        Ok(())
    }

    fn mouse_down(&mut self, button: &str) -> PyResult<()> {
        mouse_down_input(&mut self.device, button)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        Ok(())
    }

    fn mouse_up(&mut self, button: &str) -> PyResult<()> {
        mouse_up_input(&mut self.device, button)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        Ok(())
    }

    fn send_mouse_move(&mut self, x: i32, y: i32) -> PyResult<()> {
        send_mouse_move_input(&mut self.device, x, y)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        Ok(())
    }
}
