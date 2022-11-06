#![no_std]
#![no_main]
#![feature(core_ffi_c)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[allow(dead_code)]
mod arduino;

use arduino::{LiquidCrystal_I2C, LiquidCrystal_I2C_write};

use arduino_hal::prelude::*;
use panic_halt as _;

extern "C" {
    fn init();
}

#[arduino_hal::entry]
unsafe fn main() -> ! {
    init();

    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut led = pins.d13.into_output();

    ufmt::uwriteln!(&mut serial, "starting on {}\r", 0x27).void_unwrap();

    let mut lcd = LiquidCrystal_I2C::new(0x27, 16, 2);

    let ferris = &[
        0b01010u8, 0b01010u8, 0b00000u8, 0b00100u8, 0b10101u8, 0b10101u8, 0b11111u8, 0b10101u8,
    ];

    lcd.begin(16, 2, 0);
    lcd.init();
    lcd.backlight();

    lcd.clear();
    lcd.printstr("Good morning\0".as_ptr().cast());
    lcd.setCursor(0, 1);
    lcd.printstr("from Rust!!\0".as_ptr().cast());

    lcd.createChar(0, ferris.as_ptr() as *mut _);
    lcd.setCursor(12, 1);

    LiquidCrystal_I2C_write((&mut lcd as *mut LiquidCrystal_I2C).cast(), 0);

    loop {
        led.toggle();
        arduino_hal::delay_ms(1000);
    }
}
