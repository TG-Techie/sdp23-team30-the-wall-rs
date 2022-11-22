use rp2040_hal::gpio::{Input, Output, Pin, PinId, PullUp, PushPull};

enum PinWrapper<P: PinId> {
    open { p: Pin<P, Input<PullUp>> },
    down { p: Pin<P, Output<PushPull>> },
}
