# keyboard_backlight

Small tool to change your keyboard backlight color.

## What it does

- Set a solid color (red, green, blue, etc)
- Turn the backlight on or off
- Adjust brightness (Linux: 0-255 or percentages, Windows: 1-4)
- Run rainbow animation
- Run breathing (color fades in and out)
- Stop any running effects

## Linux (Tuxedo)

Works on Tuxedo computers with the `tuxedo_keyboard` driver.

The program writes directly to `/sys/devices/platform/tuxedo_keyboard/leds/rgb:kbd_backlight/`.

## Windows

Works on Clevo laptops through WMI. You need to run the program as Administrator.

## Usage

```bash
# Set color by RGB values
keyboard_backlight --rgb 255 100 50

# Set color by name
keyboard_backlight --color red

# Turn off
keyboard_backlight --off

# Turn on
keyboard_backlight --on

# Brightness (Linux: 0-255 or 0-100%, Windows: 1-4)
keyboard_backlight --brightness 128
keyboard_backlight --brightness 50%

# Rainbow effect
keyboard_backlight --back-rainbow

# Breathing effect
keyboard_backlight --back-breath red

# Stop effects
keyboard_backlight --stop
```

## Driver info

This tool talks to your keyboard's firmware. For Linux it uses the `tuxedo_keyboard` driver.
For Windows it uses the `CLEVO_GET` WMI interface.

The Linux driver source is available here:
https://github.com/wessel-novacustom/clevo-keyboard

## Build

```bash
cargo build --release
```

The binary will be in `target/keyboard_backlight`.

## Tested

- 12 unit tests pass
- Code compiles without warnings
