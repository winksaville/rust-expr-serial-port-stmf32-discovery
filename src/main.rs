//#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux::{entry, iprint, iprintln, usart1, DelayMs};

/// Write a character to usart
fn write_ch(usart: &mut usart1::RegisterBlock, c: u8) {
    while usart.isr.read().txe().bit_is_clear() {}
    unsafe {
        usart.tdr.write(|w| w.tdr().bits(u16::from(c)));
    }
}

#[entry]
fn main() -> ! {
    let (usart1, mut delay, mut itm) = aux::init();
    iprintln!(&mut itm.stim[0], "main: initialized");

    //const LF: u8 = b'\n';
    const CR: u8 = b'\r';

    loop {
        // Send characters
        write_ch(usart1,b'1');
        write_ch(usart1,b'2');
        write_ch(usart1,b'3');
        write_ch(usart1,b'4');
        write_ch(usart1, CR);
        //write_ch(usart1,LF);
        delay.delay_ms(1000u32);
    }
}
