//! Serial interface echo server
//!
//! In this example every received byte will be sent back to the sender. You can test this example
//! with serial terminal emulator like `minicom`.
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m_rt::entry;
use f3_r6::hal::{prelude::*, serial::Serial, pac};
use nb::block;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    let mut gpioc = p.GPIOC.split(&mut rcc.ahb);

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let tx = gpioc.pc4.into_af7(&mut gpioc.moder, &mut gpioc.afrl);
    let rx = gpioc.pc5.into_af7(&mut gpioc.moder, &mut gpioc.afrl);

    let serial = Serial::usart1(p.USART1, (tx, rx), 115_200.bps(), clocks, &mut rcc.apb2);
    let (mut tx, mut rx) = serial.split();

    loop {
        let byte = block!(rx.read()).unwrap();
        block!(tx.write(byte)).ok();
    }
}
