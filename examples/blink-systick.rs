#![no_std]
#![no_main]

use core::sync::atomic::{AtomicUsize, Ordering};
use cortex_m_rt::entry;
use cortex_m_rt::exception;
use nrf52833_pac as pac;
use panic_halt as _;

static TICK_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn clock_config(p: &pac::Peripherals) {
    let clock = &p.CLOCK;

    if clock.hfclkstat.read().src().is_xtal()
        && clock.hfclkstat.read().state().is_running()
    {
        return;
    }

    clock
        .tasks_hfclkstart
        .write(|w| w.tasks_hfclkstart().set_bit());

    while !clock.hfclkstat.read().src().is_xtal() {}
    while !clock.hfclkstat.read().state().is_running() {}
}

fn systick_config(p: &mut cortex_m::Peripherals) {
    use cortex_m::peripheral::syst::SystClkSource;

    let syst = &mut p.SYST;

    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload((64_000_000 / 1000) - 1); // 1kHz
    syst.clear_current();
    syst.enable_counter();
    syst.enable_interrupt();
}

#[exception]
fn SysTick() {
    TICK_COUNTER.fetch_add(1, Ordering::Relaxed);
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut cp = cortex_m::Peripherals::take().unwrap();

    clock_config(&dp);
    systick_config(&mut cp);

    let p0 = &dp.P0;

    // ROW1(source): P0.21, COL1(sink): P0.28
    p0.dirset.write(|w| w.pin21().set_bit().pin28().set_bit());

    let mut curr_tick = TICK_COUNTER.load(Ordering::Relaxed);

    loop {
        // 10ms ON
        p0.outset.write(|w| w.pin21().set_bit());
        while TICK_COUNTER.load(Ordering::Relaxed).wrapping_sub(curr_tick) < 10
        {
        }

        // 990ms OFF
        p0.outclr.write(|w| w.pin21().clear_bit_by_one());
        while TICK_COUNTER.load(Ordering::Relaxed).wrapping_sub(curr_tick)
            < 1000
        {}

        curr_tick = curr_tick.wrapping_add(1000);
    }
}
