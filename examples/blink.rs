#![no_std]
#![no_main]

use cortex_m_rt::entry;
use nrf52833_pac as pac;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let p0 = &p.P0;

    // ROW1(source): P0.21, COL1(sink): P0.28
    p0.dirset.write(|w| w.pin21().set_bit().pin28().set_bit());

    loop {
        p0.outset.write(|w| w.pin21().set_bit());
        for _ in 0..100 {
            cortex_m::asm::nop();
        }
        p0.outclr.write(|w| w.pin21().clear_bit_by_one());
        for _ in 0..200000 {
            cortex_m::asm::nop();
        }
    }
}
