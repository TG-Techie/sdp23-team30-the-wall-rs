#![no_std]

// use rp2040_hal::gpio::reg::RegisterInterface;

use core::marker::PhantomData;

use rp2040_hal::gpio::{Pin, PinId, PinMode, ValidPinMode};

pub struct PinPulls<I: PinId> {
    _ptr: &'static mut u32,
    _marker: PhantomData<I>,
}

impl<I: PinId> PinPulls<I> {
    const PULL_UP_ENABLE_MASK: u32 = (1 << 3);
    const PULL_DOWN_ENABLE_MASK: u32 = (1 << 2);

    pub fn get_pull_up(&mut self) -> bool {
        (*self._ptr & Self::PULL_UP_ENABLE_MASK) == Self::PULL_UP_ENABLE_MASK
    }

    pub fn set_pull_up(mut self, enable: bool) {
        if enable {
            *self._ptr |= Self::PULL_UP_ENABLE_MASK;
        } else {
            *self._ptr &= Self::PULL_UP_ENABLE_MASK;
        }
    }

    pub fn get_pull_down(&mut self) -> bool {
        (*self._ptr & Self::PULL_DOWN_ENABLE_MASK) == Self::PULL_DOWN_ENABLE_MASK
    }

    pub fn set_pull_down(&mut self, enable: bool) {
        if enable {
            *self._ptr |= Self::PULL_DOWN_ENABLE_MASK;
        } else {
            *self._ptr &= Self::PULL_DOWN_ENABLE_MASK;
        }
    }
}

pub trait SetPulls<I: PinId> {
    unsafe fn into_pointer() -> Option<*const u32>;

    fn as_pulls(&self) -> Option<PinPulls<I>> {
        Some(PinPulls {
            _ptr: unsafe { core::mem::transmute(Self::into_pointer()?) },
            _marker: PhantomData,
        })
    }
}

impl<I, M> SetPulls<I> for Pin<I, M>
where
    I: PinId,
    M: PinMode + ValidPinMode<I>,
{
    unsafe fn into_pointer() -> Option<*const u32> {
        use rp2040_hal::gpio::dynpin::DynGroup;
        use rp2040_pac::PADS_BANK0;
        match I::DYN.group {
            DynGroup::Bank0 => {}
            DynGroup::Qspi => return None,
        };

        Some((*PADS_BANK0::ptr()).gpio[I::DYN.num as usize].as_ptr())
    }
}
