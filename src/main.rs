//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
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
use rp2040_hal::gpio::{FunctionPio0, Pin};
use rp2040_hal::pio;
use rp2040_hal::pio::PIOExt;

/* ========== [ Main story / entry code ] ========== */

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

    // pin re-assignments (to handle switching between bsp)
    let led_pin = pins.d13;

    // setup the blink led
    let mut led_pin = led_pin.into_push_pull_output();

    // setup the rp2040's pio
    let _eeprom_pin: Pin<_, FunctionPio0> = pins.a0.into_mode();
    let eeprom_pin_id: u8 = 26;

    let program = pio_proc::pio_asm!(
        ".wrap_target",
        "set pins 0b1",
        "set pins 0b0",
        ".wrap",
        // select_program("eeprom_11xx"), // Optional if only one program in the file
        // options(max_program_size = 32)
    );

    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);
    let installed = pio.install(&program.program).unwrap();
    let (mut sm, _, _) = rp2040_hal::pio::PIOBuilder::from_program(installed)
        .set_pins(eeprom_pin_id, 1)
        .clock_divisor(65536.0)
        .build(sm0);
    // The GPIO pin needs to be configured as an output.
    sm.set_pindirs([(eeprom_pin_id, pio::PinDir::Output)]);
    sm.start();

    // run

    loop {
        info!("on!");
        led_pin.set_high().unwrap();
        delay.delay_ms(500);
        info!("off!");
        led_pin.set_low().unwrap();
        delay.delay_ms(500);
    }
}

// End of file
