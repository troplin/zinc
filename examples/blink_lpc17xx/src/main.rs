#![feature(plugin)]
#![no_std]
#![plugin(macro_zinc)]

extern crate zinc;

use core::option::Option::Some;

use zinc::hal::lpc17xx::{pin, timer};
use zinc::hal::pin::Gpio;
use zinc::hal::pin::GpioDirection;
use zinc::hal::timer::Timer;

#[zinc_main]
pub fn main() {
  zinc::hal::mem_init::init_stack();
  zinc::hal::mem_init::init_data();

  // P1.20 => LED-2 (mbed LPC1768)
  let led2 = pin::Pin::new(
    pin::Port::Port1, 21,
    pin::Function::Gpio,
    Some(GpioDirection::Out));

  let timer = timer::Timer::new(timer::TimerPeripheral::Timer0, 25, 4);

  loop  {
    led2.set_high();
    timer.wait_ms(10);
    led2.set_low();
    timer.wait_ms(10);
  }
}
