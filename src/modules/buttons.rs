use esp_backtrace as _;
use esp_hal::{gpio, prelude::*};

pub struct Buttons {
    button_a: gpio::GpioPin<gpio::Input<gpio::Floating>, 37>,
    button_b: gpio::GpioPin<gpio::Input<gpio::Floating>, 39>,
}

impl Buttons {
    pub fn new(
        pin_a: gpio::GpioPin<gpio::Input<gpio::Floating>, 37>,
        pin_b: gpio::GpioPin<gpio::Input<gpio::Floating>, 39>,
    ) -> Self {
        Self {
            button_a: pin_a,
            button_b: pin_b,
        }
    }

    pub fn is_pressed_a(&self) -> bool {
        self.button_a.is_low().unwrap()
    }

    pub fn is_released_a(&self) -> bool {
        self.button_a.is_high().unwrap()
    }

    pub fn is_pressed_b(&self) -> bool {
        self.button_b.is_low().unwrap()
    }

    #[allow(dead_code)]
    pub fn is_released_b(&self) -> bool {
        self.button_b.is_high().unwrap()
    }
}
