use rp2040_hal::gpio::{
    Input, Output, Pin, PinId, PinMode, PullUp, PullUpInput, PushPull, ValidPinMode,
};

enum OpenDrainPin<P: PinId> {
    PulledUp(Pin<P, PullUpInput>),
    Low(Pin<P, Output<PushPull>>),
}

pub struct UNIOBang<P: PinId> {
    pin: OpenDrainPin<P>,
}

impl<P: PinId> OpenDrainPin<P> {
    fn new_init<S: Into<Pin<P, Input<PullUp>>>>(some_pin: S) -> Self {
        let mut pin: Pin<P, PullUpInput> = some_pin.into();

        //
        pin.set_output_enable_override(rp2040_hal::gpio::OutputEnableOverride::Invert);

        // Turn on the pullup
        use crate::there_be_dragons::SetPulls;
        let _pulls = pin.as_pulls().unwrap().set_pull_up(true);

        Self::PulledUp(pin)
    }

    fn set_pulled_up(&mut self) -> Result<(), ()> {
        Err(())
    }

    fn set_low(&mut self) -> Result<(), ()> {
        Err(())
    }
}


impl<P: PinId> UNIOBang<P> {
    pub fn new<S: Into<Pin<P, Input<PullUp>>>>(some_pin: S) -> Self {
        return Self {
            pin: OpenDrainPin::new_init(some_pin),
        };
    }

    pub write_word(&mut self, word: u8) -> Result<(), ()> {
        self.pin.set_open();
        Err(())
    }
}
