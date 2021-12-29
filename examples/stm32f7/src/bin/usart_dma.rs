#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

#[path = "../example_common.rs"]
mod example_common;
use core::fmt::Write;
use embassy::executor::Spawner;
use embassy_stm32::dma::NoDma;
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::Peripherals;
use embassy_traits::uart::Write as _Write;
use example_common::*;

use heapless::String;

#[embassy::main]
async fn main(_spawner: Spawner, p: Peripherals) {
    let config = Config::default();
    let mut usart = Uart::new(p.UART7, p.PA8, p.PA15, p.DMA1_CH1, NoDma, config);

    for n in 0u32.. {
        let mut s: String<128> = String::new();
        core::write!(&mut s, "Hello DMA World {}!\r\n", n).unwrap();

        unwrap!(usart.write(s.as_bytes()).await);

        info!("wrote DMA");
    }
}
