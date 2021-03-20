//! Initialization code

#![no_std]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust53964
extern crate panic_itm; // panic handler

pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
pub use cortex_m_rt::entry;

//pub use stm32f3_discovery::stm32f3xx_hal;
use stm32f3xx_hal::prelude::*;
pub use stm32f3xx_hal::{
    delay::Delay,
    hal::blocking::delay::DelayMs,
    serial::Serial,
    stm32::{self, USART1, usart1 },
};

//pub fn init() -> (&'static mut usart1::RegisterBlock, MonoTimer, ITM) {
pub fn init() -> (&'static mut usart1::RegisterBlock, Delay, ITM) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let delay = Delay::new(cp.SYST, clocks);

    let mut itm = cp.ITM;

    let (tx, rx) = match () {
        #[cfg(feature = "adapter")]
        () => {
            iprintln!(&mut itm.stim[0], "Old school adapter");
            let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

            let tx = gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
            let rx = gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh);

            (tx, rx)
        }
        #[cfg(not(feature = "adapter"))]
        () => {
            iprintln!(&mut itm.stim[0], "No adapter");
            let mut gpioc = dp.GPIOC.split(&mut rcc.ahb);

            let tx = gpioc.pc4.into_af7(&mut gpioc.moder, &mut gpioc.afrl);
            let rx = gpioc.pc5.into_af7(&mut gpioc.moder, &mut gpioc.afrl);

            (tx, rx)
        }
    };

    Serial::usart1(dp.USART1, (tx, rx), 115_200.bps(), clocks, &mut rcc.apb2);
    // If you are having trouble sending/receiving data to/from the
    // HC-05 bluetooth module, try this configuration instead:
    // Serial::usart1(dp.USART1, (tx, rx), 9600.bps(), clocks, &mut rcc.apb2);

    (
        unsafe { &mut *(stm32::USART1::ptr() as *mut _) },
        delay,
        itm,
    )
}
