#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay, IO};
use esp_println::println;

mod modules;
use modules::{buttons, led, microphone};

#[entry]
fn main() -> ! {
    // setting up the peripherals
    let peripherals: Peripherals = Peripherals::take();
    let system: esp_hal::system::SystemParts<'static> = peripherals.SYSTEM.split();
    let clocks: esp_hal::clock::Clocks<'static> = ClockControl::max(system.clock_control).freeze();
    let mut delay: Delay = Delay::new(&clocks);
    let mut io: IO = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    // abstractions :
    let mut led: led::Led = led::Led::new(io.pins.gpio10.into_push_pull_output());
    let mut microphone: microphone::Microphone =
        microphone::Microphone::new(io.pins.gpio0.enable_input(true));
    let buttons: buttons::Buttons = buttons::Buttons::new(
        io.pins.gpio37.into_floating_input(),
        io.pins.gpio39.into_floating_input(),
    );

    println!("Setting up the configuration...");
    loop {
        // println!("Loop...");
        if buttons.is_pressed_a() {
            println!("Button pressed");
            led.toggle();
            if microphone.is_listening() {
                println!("Microphone is listening.");
            } else {
                println!("Microphone is not listening");
                microphone.listen();
            }
        } else if !led.is_set_high() {
            println!("Button not pressed and led is on");
            led.toggle();
        }
        if microphone.is_listening() && buttons.is_released_a() {
            println!("Microphone is listening and button not pressed");
            // interrupt listening
            microphone.unlisten();
            microphone.enable_input(false);
        }
        if buttons.is_pressed_b() {
            println!("Button B pressed");
        }
        delay.delay_ms(500u32);
    }
}
