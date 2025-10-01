use esp_idf_svc::hal::{
    adc::{
        attenuation,
        oneshot::{config::AdcChannelConfig, AdcChannelDriver, AdcDriver},
    },
    delay::FreeRtos,
    gpio::PinDriver,
    peripherals::Peripherals,
};

// Soil moisture thresholds (ADC values, 0-4095 range)
const DRY_THRESHOLD: u16 = 2800; // Above this = dry soil
const OPTIMAL_THRESHOLD: u16 = 1500; // Below this = optimal/wet soil
const SENSOR_MAX: u16 = 3500; // Above this = sensor disconnected
const SENSOR_MIN: u16 = 10; // Persistently below this = sensor likely not connected
const DISCONNECT_STREAK: u8 = 5; // Number of consecutive low readings to consider disconnected

fn main() {
    esp_idf_svc::sys::link_patches();

    println!("ESP32 Soil Moisture Sensor Starting...");

    let peripherals = Peripherals::take().unwrap();

    // Setup LED on GPIO2
    let mut led = PinDriver::output(peripherals.pins.gpio2).unwrap();

    // Setup ADC on GPIO36 with oneshot driver
    let mut adc = AdcDriver::new(peripherals.adc1).unwrap();

    // Configure ADC channel with 11dB attenuation for full 0-3.3V range
    let adc_config = AdcChannelConfig {
        attenuation: attenuation::DB_11,
        ..Default::default()
    };
    let mut adc_pin =
        AdcChannelDriver::new(&mut adc, peripherals.pins.gpio36, &adc_config).unwrap();

    println!("Sensor initialized. GPIO36=ADC, GPIO2=LED");
    println!("Reading soil moisture...");

    // Track consecutive near-zero readings to detect disconnected sensor
    let mut low_disconnect_streak: u8 = 0;

    loop {
        // Read soil moisture sensor
        let raw_value: u16 = adc_pin.read().unwrap();

        // Low-value disconnect heuristic: treat persistent near-zero readings as disconnected
        if raw_value <= SENSOR_MIN {
            low_disconnect_streak = low_disconnect_streak.saturating_add(1);
        } else {
            low_disconnect_streak = 0;
        }
        if low_disconnect_streak >= DISCONNECT_STREAK {
            println!(
                "WARNING: Sensor not connected (low ADC) - ADC={}",
                raw_value
            );
            for _ in 0..10 {
                led.set_high().unwrap();
                FreeRtos::delay_ms(100);
                led.set_low().unwrap();
                FreeRtos::delay_ms(100);
            }
            continue; // Skip normal processing
        }

        // Check for sensor disconnection first
        if raw_value > SENSOR_MAX {
            println!("WARNING: Sensor disconnected! ADC={}", raw_value);

            // Rapid blink pattern (5 blinks per second)
            for _ in 0..10 {
                led.set_high().unwrap();
                FreeRtos::delay_ms(100);
                led.set_low().unwrap();
                FreeRtos::delay_ms(100);
            }
            continue; // Skip normal processing
        }

        // Determine soil condition and LED state
        if raw_value > DRY_THRESHOLD {
            // Dry soil - LED ON
            println!("Soil: DRY (needs water) - ADC={}", raw_value);
            led.set_high().unwrap();
        } else if raw_value < OPTIMAL_THRESHOLD {
            // Optimal/wet soil - LED OFF
            println!("Soil: OPTIMAL/WET - ADC={}", raw_value);
            led.set_low().unwrap();
        } else {
            // In-between - LED OFF (acceptable moisture)
            println!("Soil: ACCEPTABLE - ADC={}", raw_value);
            led.set_low().unwrap();
        }

        // Wait 2 seconds before next reading
        FreeRtos::delay_ms(2000);
    }
}
