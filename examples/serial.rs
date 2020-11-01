//! Serial interface loopback test
//!
//! You have to short the TX (pin 3) and RX (pin 4) of the UEXT connector

#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::rtt_init_default;

use nb::block;

use cortex_m_rt::entry;

use stm32f1xx_hal::{
    pac,
    prelude::*,
};

use olimex_stm32_p107::serial;

#[entry]
fn main() -> ! {
    rtt_init_default!();

    // Get access to the device specific peripherals from the peripheral access crate
    let p = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Prepare the alternate function I/O registers
    let mut afio = p.AFIO.constrain(&mut rcc.apb2);

    // Prepare the GPIOD peripheral
    let mut gpiod = p.GPIOD.split(&mut rcc.apb2);

    // Set up the usart device. Taks ownership over the USART register and tx/rx pins. The rest of
    // the registers are used to enable and configure the device.
    let mut serial = serial::uext(
        p.USART3, gpiod.pd8, gpiod.pd9, &mut gpiod.crh,
        &mut afio.mapr,
        serial::Config::default().baudrate(115200.bps()),
        clocks,
        &mut rcc.apb1,
    );

    // Loopback test. Write `X` and wait until the write is successful.
    let sent = b'X';
    block!(serial.write(sent)).ok();

    // Read the byte that was just sent. Blocks until the read is complete
    let received = block!(serial.read()).unwrap();

    // Since we have connected tx and rx, the byte we sent should be the one we received
    assert_eq!(received, sent);

    // You can also split the serial struct into a receiving and a transmitting part
    let (mut tx, mut rx) = serial.split();
    let sent = b'Y';
    block!(tx.write(sent)).ok();
    let received = block!(rx.read()).unwrap();
    assert_eq!(received, sent);

    loop {
        core::sync::atomic::spin_loop_hint();
    }
}
