#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    // RGB traffic light example for Arduino Mega 2560. Connect the red LED to pin 10, the green LED to pin 11 and the blue LED to pin 12. The LEDs will alternate between red, green and blue every second.
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut red_led = pins.d10.into_output();
    let mut green_led = pins.d11.into_output();
    let mut blue_led = pins.d12.into_output();

    loop {
        red_led.set_high();
        green_led.set_low();
        blue_led.set_low();
        arduino_hal::delay_ms(1000);

        red_led.set_high();
        green_led.set_high();
        blue_led.set_low();
        arduino_hal::delay_ms(1000);

        red_led.set_low();
        green_led.set_high();
        blue_led.set_low();
        arduino_hal::delay_ms(1000);
    }
}
