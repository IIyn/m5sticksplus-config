use esp_backtrace as _;
use esp_hal::{gpio, prelude::*};
pub struct Microphone<'a> {
    pin: &'a mut gpio::GpioPin<gpio::Unknown, 0>,
}

impl<'a> Microphone<'a> {
    pub fn new(pin: &'a mut gpio::GpioPin<gpio::Unknown, 0>) -> Self {
        Self { pin }
    }

    pub fn enable_input(&mut self, on: bool) {
        self.pin.enable_input(on);
    }

    pub fn listen(&mut self) {
        self.pin.listen(gpio::Event::AnyEdge);
    }

    pub fn unlisten(&mut self) {
        self.pin.unlisten();
    }

    pub fn is_listening(&self) -> bool {
        self.pin.is_listening()
    }
}
