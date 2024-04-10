#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{clock::ClockControl, gpio, peripherals::Peripherals, prelude::*, Delay, IO};
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals: Peripherals = Peripherals::take();
    let system: esp_hal::system::SystemParts<'static> = peripherals.SYSTEM.split();

    let clocks: esp_hal::clock::Clocks<'static> = ClockControl::max(system.clock_control).freeze();
    let mut delay: Delay = Delay::new(&clocks);

    let mut io: IO = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut gpio10: esp_hal::gpio::GpioPin<esp_hal::gpio::Output<esp_hal::gpio::PushPull>, 10> =
        io.pins.gpio10.into_push_pull_output();
    let gpio37: gpio::GpioPin<gpio::Input<gpio::Floating>, 37> =
        io.pins.gpio37.into_floating_input();
    let gpio0: &mut gpio::GpioPin<gpio::Unknown, 0> = io.pins.gpio0.enable_input(true); // microphone pin (SPM1423)
    println!("Setting up the configuration...");
    loop {
        // println!("Loop...");
        if gpio37.is_low().unwrap() {
            println!("Button pressed");
            gpio10.toggle().unwrap();
            if gpio0.is_listening() {
                println!("Microphone is listening.");
            } else {
                println!("Microphone is not listening");
                gpio0.listen(gpio::Event::AnyEdge)
            }
        } else if !gpio10.is_set_high().unwrap() {
            println!("Button not pressed and led is on");
            gpio10.toggle().unwrap();
        }
        if gpio0.is_listening() && gpio37.is_high().unwrap() {
            println!("Microphone is listening and button not pressed");
            // interrupt listening
            gpio0.unlisten();
            gpio0.enable_input(false);
        }
        delay.delay_ms(500u32);
    }
}
