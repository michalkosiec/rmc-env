use pyo3::prelude::*;

mod bot;
use bot::MinecraftBot;

mod capture;

mod input;

#[pymodule]
fn rmc_env(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MinecraftBot>()?;
    Ok(())
}
