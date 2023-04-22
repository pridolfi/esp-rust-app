use anyhow::{bail, Result};
use esp_idf_hal::prelude::Peripherals;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_sys::{nvs_flash_init};
use log::info;
use rgb_led::{RGB8, WS2812RMT};
use wifi::wifi;
use std::net::UdpSocket;

/// This configuration is picked up at compile time by `build.rs` from the
/// file `cfg.toml`.
#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

fn main() -> Result<()> {
    unsafe {
        nvs_flash_init();
    }

    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let sysloop = EspSystemEventLoop::take()?;

    info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();
    let mut led = WS2812RMT::new(peripherals.pins.gpio4, peripherals.rmt.channel0)?;
    led.set_pixel(RGB8::new(50, 50, 0))?;

    // The constant `CONFIG` is auto-generated by `toml_config`.
    let app_config = CONFIG;

    // Connect to the Wi-Fi network
    let _wifi = match wifi(
        app_config.wifi_ssid,
        app_config.wifi_psk,
        peripherals.modem,
        sysloop,
    ) {
        Ok(inner) => inner,
        Err(err) => {
            // Red!
            led.set_pixel(RGB8::new(50, 0, 0))?;
            bail!("Could not connect to Wi-Fi network: {:?}", err)
        }
    };

    socket_test();

    loop {
        // Blue!
        led.set_pixel(RGB8::new(0, 0, 50))?;
        std::thread::sleep(std::time::Duration::from_millis(500));

        info!("blink!");

        // Green!
        led.set_pixel(RGB8::new(0, 50, 0))?;
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}

fn socket_test() {
    let socket = UdpSocket::bind("0.0.0.0:8001").expect("couldn't create socket");
    socket.send_to(b"pepe", "192.168.223.23:8000").expect("couldn't send_to");
}