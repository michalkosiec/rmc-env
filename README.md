# rmc-env

Python bindings for controlling Minecraft with a virtual keyboard/mouse and reading the game window as screenshots.

## Features

- Capture a window by ID as an RGBA NumPy array
- Send keyboard commands
- Tap and hold keyboard keys
- Click mouse buttons
- Move the mouse relatively

## Requirements

- Linux
- Access to `/dev/uinput`
- A Python environment with `pyo3`/extension support

## Build

```bash
maturin develop
# or
maturin build --release
```

## Python API

```python
from rmc_env import MinecraftBot

bot = MinecraftBot(window_id)
screen = bot.get_screen()

bot.send_command("up")
bot.key_down("left")
bot.key_up("left")

bot.send_click("left")
bot.mouse_down("right")
bot.mouse_up("right")

bot.send_mouse_move(10, -5)
```

## Supported inputs

### Keyboard

- `up`, `down`, `left`, `right`
- `jump`
- `sneak`, `shift`
- `sprint`
- `inventory`, `e`
- `chat`, `t`
- `drop`, `q`
- `swap_hands`, `f`

### Mouse

- `left`
- `right`
