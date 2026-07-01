use std::{thread, time::Duration};
use uinput::event::controller::{Controller, Mouse};
use uinput::event::keyboard::Key;
use uinput::event::relative::Position as RelativePosition;

pub fn init_virtual_device() -> Result<uinput::Device, uinput::Error> {
    let device = uinput::default()?
        .name("Virtual keyboard")?
        .event(uinput::event::Keyboard::All)?
        .event(uinput::event::Controller::All)?
        .event(uinput::event::Relative::Position(
            uinput::event::relative::Position::X,
        ))?
        .event(uinput::event::Relative::Position(
            uinput::event::relative::Position::Y,
        ))?
        .create()?;

    thread::sleep(Duration::from_millis(50));

    Ok(device)
}

pub fn send_command(device: &mut uinput::Device, action: &str) -> Result<(), uinput::Error> {
    if let Some(key) = key_for_action(action) {
        tap_key(device, key)
    } else {
        Ok(())
    }
}

pub fn key_down(device: &mut uinput::Device, action: &str) -> Result<(), uinput::Error> {
    if let Some(key) = key_for_action(action) {
        device.press(&key)?;
        device.synchronize()?;
    }
    Ok(())
}

pub fn key_up(device: &mut uinput::Device, action: &str) -> Result<(), uinput::Error> {
    if let Some(key) = key_for_action(action) {
        device.release(&key)?;
        device.synchronize()?;
    }
    Ok(())
}

pub fn send_click(device: &mut uinput::Device, button: &str) -> Result<(), uinput::Error> {
    if let Some(mouse_btn) = mouse_button_for(button) {
        tap_mouse_button(device, mouse_btn)
    } else {
        Ok(())
    }
}

pub fn mouse_down(device: &mut uinput::Device, button: &str) -> Result<(), uinput::Error> {
    if let Some(mouse_btn) = mouse_button_for(button) {
        device.press(&Controller::Mouse(mouse_btn))?;
        device.synchronize()?;
    }
    Ok(())
}

pub fn mouse_up(device: &mut uinput::Device, button: &str) -> Result<(), uinput::Error> {
    if let Some(mouse_btn) = mouse_button_for(button) {
        device.release(&Controller::Mouse(mouse_btn))?;
        device.synchronize()?;
    }
    Ok(())
}

pub fn send_mouse_move(device: &mut uinput::Device, x: i32, y: i32) -> Result<(), uinput::Error> {
    device.position(&RelativePosition::X, x)?;
    device.position(&RelativePosition::Y, y)?;
    device.synchronize()?;
    Ok(())
}

fn key_for_action(action: &str) -> Option<Key> {
    match action {
        "up" | "w" => Some(Key::W),
        "down" | "s" => Some(Key::S),
        "left" | "a" => Some(Key::A),
        "right" | "d" => Some(Key::D),
        "jump" | "space" => Some(Key::Space),
        "sneak" | "shift" => Some(Key::LeftShift),
        "sprint" => Some(Key::LeftControl),
        "inventory" | "e" => Some(Key::E),
        "chat" | "t" => Some(Key::T),
        "drop" | "q" => Some(Key::Q),
        "swap_hands" | "f" => Some(Key::F),
        _ => None,
    }
}

fn mouse_button_for(button: &str) -> Option<Mouse> {
    match button {
        "left" => Some(Mouse::Left),
        "right" => Some(Mouse::Right),
        _ => None,
    }
}

fn tap_key(device: &mut uinput::Device, key: Key) -> Result<(), uinput::Error> {
    device.press(&key)?;
    device.synchronize()?;

    thread::sleep(Duration::from_millis(50));

    device.release(&key)?;
    device.synchronize()?;
    Ok(())
}

fn tap_mouse_button(device: &mut uinput::Device, mouse_btn: Mouse) -> Result<(), uinput::Error> {
    device.press(&Controller::Mouse(mouse_btn))?;
    device.synchronize()?;

    thread::sleep(Duration::from_millis(50));

    device.release(&Controller::Mouse(mouse_btn))?;
    device.synchronize()?;
    Ok(())
}
