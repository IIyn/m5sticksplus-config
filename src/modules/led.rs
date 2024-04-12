use esp_backtrace as _;
use esp_hal::{gpio, prelude::*};

pub struct Led {
    pin: gpio::GpioPin<gpio::Output<gpio::PushPull>, 10>,
}

impl Led {
    pub fn new(
        pin: esp_hal::gpio::GpioPin<esp_hal::gpio::Output<esp_hal::gpio::PushPull>, 10>,
    ) -> Self {
        Self { pin }
    }

    pub fn toggle(&mut self) {
        self.pin.toggle().unwrap();
    }

    pub fn is_set_high(&self) -> bool {
        self.pin.is_set_high().unwrap()
    }
}
