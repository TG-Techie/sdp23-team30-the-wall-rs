//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m as _;

/* ========== [ BSP and related Items ] ========== */
// The related items are for setting cpu speed, peripharls, etc. Here we rename the adafruit
// BSP to "bsp"  so it is easy to switch BSPs if need be later.
use adafruit_feather_rp2040 as bsp;
use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

/* ========== [ eeprom driver imports ] ========== */
use pio_proc;
// use rp2040_hal::pio;

use rp2040_hal::gpio::{self, bank0::Gpio10, Pin, PinId, PullDown};
use rp2040_hal::pio::{PIOExt, PinDir, PinState, ShiftDirection};

/* ========== [ Main story / entry code ] ========== */

mod there_be_dragons;

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    type PIN = Gpio10;
    let pin = pins.d10.into_pull_up_input();

    // configure SCL pin as inverted
    pin.set_output_enable_override(rp2040_hal::gpio::OutputEnableOverride::Invert);

    // the PIO now keeps the pin as Input, we can set the pin state to Low.
    let pin = pin.into_readable_output();
    // turn on the pullup
    let _pulls = pin.as_pulls().unwrap().set_pull_up(true);

    loop {}
}

// End of file
