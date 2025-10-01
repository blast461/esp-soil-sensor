# ESP32 Soil Moisture Sensor

Rust-based soil moisture sensor firmware for ESP32 using capacitive moisture sensor.

## Hardware Setup

- **ESP32 DevKit** (tested on ESP32 with Silicon Labs CP210x USB bridge)
- **Capacitive Soil Moisture Sensor** connected to GPIO36 (ADC1_CH0)
- **Status LED** on GPIO2 (built-in LED on most ESP32 boards)

### Wiring

```
Soil Sensor → ESP32
VCC         → 3.3V
GND         → GND
AOUT        → GPIO36
```

## LED Behavior

The built-in LED indicates soil moisture status:

- **LED ON (solid)**: Dry soil - needs water
- **LED OFF**: Optimal/wet soil - no action needed
- **LED RAPID BLINK** (5Hz): Sensor error/disconnected

## Moisture Thresholds

Adjust these constants in `src/main.rs` based on your sensor calibration:

```rust
const DRY_THRESHOLD: u16 = 2800;      // Above this = dry
const OPTIMAL_THRESHOLD: u16 = 1500;  // Below this = optimal
const SENSOR_MAX: u16 = 3500;         // Above this = disconnected
```

## Building and Flashing

### Prerequisites

```bash
# Install Rust ESP toolchain (if not already installed)
rustup toolchain install esp

# Install espflash
cargo install espflash
```

### Build

```bash
cargo build --release
```

### Flash to ESP32

**Important**: Your ESP32 board requires manual bootloader entry:

1. Hold down the **BOOT** button
2. Press and release the **RESET** button
3. Release the **BOOT** button
4. Within 5 seconds, run:

```bash
cargo run --release
```

The firmware will flash and automatically open a serial monitor showing real-time sensor readings.

### Monitor Only (after flashing)

```bash
espflash monitor
```

## Serial Output

The firmware outputs readings every 2 seconds:

```
ESP32 Soil Moisture Sensor Starting...
Sensor initialized. GPIO36=ADC, GPIO2=LED
Reading soil moisture...
Soil: DRY (needs water) - ADC=3021
Soil: ACCEPTABLE - ADC=2456
Soil: OPTIMAL/WET - ADC=1234
WARNING: Sensor disconnected! ADC=4095
```

## Calibration

To calibrate for your specific sensor and soil:

1. Test sensor in **dry air**: Note ADC value → Use as DRY_THRESHOLD upper bound
2. Test sensor in **water**: Note ADC value → Use as OPTIMAL_THRESHOLD lower bound
3. Adjust thresholds in `src/main.rs` based on your readings

Typical ranges:
- Air: 3000-4000
- Dry soil: 2500-3000
- Moist soil: 1500-2500
- Water: 500-1500

## Project Structure

```
E:\esp\tb\
├── .cargo/
│   └── config.toml          # espflash runner configuration
├── src/
│   └── main.rs              # Soil sensor firmware
├── Cargo.toml               # Project dependencies
└── README.md                # This file
```

## Reproducible Build & Flash (Windows)

This project is configured for reliable, repeatable builds on Windows with:
- Toolchain pinned to the Espressif Rust toolchain via `rust-toolchain.toml` (channel = "esp").
- ESP‑IDF pinned to v5.3.3 via `.cargo/config.toml`.
- Project-local ESP‑IDF tools cache at `.embuild` via `.cargo/config.toml`.
- Dependencies locked via `Cargo.lock` (tracked in git).

Steps (PowerShell):

1) Install prerequisites (one time):
```
# Install Espressif Rust toolchain and source component
rustup toolchain install esp
rustup component add rust-src --toolchain esp

# espflash for flashing and monitor
cargo install espflash
```

2) Build (release):
```
cargo build --release
```

3) Flash + monitor (auto-detects COM port, defaults to 115200 baud):
```
cargo run --release
```

- To specify a port explicitly, run espflash directly:
```
espflash flash --baud 115200 --monitor -p COM3 "E:\\esp\\tb\\target\\xtensa-esp32-espidf\\release\\esp32-soil-sensor"
```

4) Clean-room verification (optional):
```
# Remove build artifacts and local tool cache, then rebuild
Remove-Item -Recurse -Force .\target, .\.embuild -ErrorAction SilentlyContinue
cargo build --release
```

Notes:
- The runner in `.cargo/config.toml` is set to `espflash flash --monitor --baud 115200`, so `cargo run --release` will flash and open a serial monitor automatically.
- If COM auto-detection fails, pass `-p COMx` to espflash, or temporarily export `ESPFLASH_PORT`.
- To record the exact toolchain used, you can capture:
```
rustc -Vv --toolchain esp
```
Include this output in your issue reports for full reproducibility.

## Troubleshooting

### "Error while connecting to device"

- Verify ESP32 is plugged into correct COM port
- Try manual bootloader entry sequence (BOOT + RESET)
- Check USB cable supports data transfer (not charge-only)

### Sensor always shows disconnected

- Check sensor wiring, especially AOUT → GPIO36
- Verify sensor has power (VCC + GND connected)
- Lower SENSOR_MAX threshold if needed

### LED behavior doesn't match soil

- Sensor needs calibration for your soil type
- Test sensor in air and water to find actual ADC ranges
- Adjust DRY_THRESHOLD and OPTIMAL_THRESHOLD accordingly

## License

MIT
