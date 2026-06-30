use xcap::Window;

pub fn capture_window(window_id: u32) -> Result<(usize, usize, Vec<u8>), String> {
    let windows =
        Window::all().map_err(|e| format!("Error occured while fetching windows: {}", e))?;

    let target_window = windows
        .into_iter()
        .find(|w| w.id() == window_id)
        .ok_or_else(|| format!("Window with the id: {} not found", window_id))?;

    let image = target_window
        .capture_image()
        .map_err(|e| format!("Error occured while taking a screenshot: {}", e))?;

    let width = image.width() as usize;
    let height = image.height() as usize;

    let raw_pixels = image.into_raw();

    Ok((width, height, raw_pixels))
}
