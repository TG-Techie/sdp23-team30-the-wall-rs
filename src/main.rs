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

use rp2040_hal::gpio::{self, PullDown, PullUp};
use rp2040_hal::gpio::{bank0::Gpio10, Pin, PinId};

use rp2040_hal::pio::{
    PIOExt, PinDir, PinState, Rx, ShiftDirection, StateMachine, StateMachineIndex, Tx,
    UninitStateMachine, PIO,
};

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

    let mut _delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pad10 = pac.PADS_BANK0.gpio[10].as_ptr();

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // setup the rp2040's pio
    // https://learn.adafruit.com/assets/100337
    type EEPROMPinId = Gpio10;
    type SCL = EEPROMPinId;
    type P = pac::PIO0;

    let eeprom_pin_id = 10; //EEPROMPinId::DYN.num;
    let eeprom_pin: Pin<EEPROMPinId, gpio::Disabled<PullDown>> = pins.d10;
    let scl = eeprom_pin;

    let program = pio_proc::pio_file!("src/eeprom_11lcxx.pio");

    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);
    let installed = pio.install(&program.program).unwrap();
    let (mut sm, _, mut tx) = rp2040_hal::pio::PIOBuilder::from_program(installed)
        // .buffers(rp2040_hal::pio::Buffers::RxTx)
        .set_pins(eeprom_pin_id, 1)
        .out_pins(eeprom_pin_id, 1)
        .in_pin_base(eeprom_pin_id)
        .side_set_pin_base(eeprom_pin_id)
        .in_shift_direction(ShiftDirection::Left)
        .clock_divisor(65536.0)
        .build(sm0);

    // enable pull up on SDA & SCL: idle bus
    let scl = scl.into_pull_up_input();

    // This will pull the bus high for a little bit of time
    sm.set_pins([(SCL::DYN.num, PinState::High)]);
    sm.set_pindirs([(SCL::DYN.num, PinDir::Output)]);

    // attach SCL pin to pio
    let mut scl: Pin<SCL, gpio::Function<P>> = scl.into_mode();
    // configure SCL pin as inverted
    scl.set_output_enable_override(rp2040_hal::gpio::OutputEnableOverride::Invert);

    // the PIO now keeps the pin as Input, we can set the pin state to Low.
    sm.set_pins([(SCL::DYN.num, PinState::Low)]);

    unsafe {
        *pad10 |= 0b1000;
    }

    // run
    let _sm = sm.start();

    loop {
        tx.write(0);
        // info!("on!");
        // led_pin.set_high().unwrap();
        // delay.delay_ms(500);
        // info!("off!");
        // led_pin.set_low().unwrap();
        // delay.delay_ms(500);
    }
}

// End of file
