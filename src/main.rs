//! This example circulary lights the LEDs on the board.
#![no_main]
#![no_std]

// RTT and defmt logger setup
use defmt_rtt as _;

// Setup panic behaviour
use panic_probe as _;

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

// add rust collections with custom allocator
extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

use embedded_alloc::Heap;

// Setup heap allocator for rust collections
#[global_allocator]
static HEAP: Heap = Heap::empty();

// Use crate for stm32f407 discovery board
use stm32f407g_disc as board;

use crate::board::{
    hal::stm32,
    hal::{delay::Delay, prelude::*},
    led::{LedColor, Leds},
};

use cortex_m::peripheral::Peripherals;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
        // !! RTT + defmt logger check
        defmt::debug!("Hello World");

        // ! Collections example
        // Initialize the allocator
        {
            use core::mem::MaybeUninit;
            // 192-Kbyte RAM available, check how much can be allocated for HEAP
            const HEAP_SIZE: usize = 1024;
            static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
            unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
        }

        // Test vector
        let c: u8 = 4;
        let mut a = vec![1, 2, 3];

        a.push(c);

        for &item in a.iter() {
            defmt::debug!("{}", item);
        }

        let b: Vec<u8> = a.iter().map(|x| x + 1).collect();

        assert_eq!(b, vec![2, 3, 4, 5]);

        for &item in b.iter() {
            defmt::debug!("{}", item);
        }

        let s: u8 = b.iter().sum();

        defmt::info!("Sum of vector: {}", s);

        // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        // ! Blinking example
        let gpiod = p.GPIOD.split();

        // Initialize on-board LEDs
        let mut leds = Leds::new(gpiod);

        // Constrain clock registers
        let rcc = p.RCC.constrain();

        // Configure clock to 168 MHz (i.e. the maximum) and freeze it
        let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();

        // Get delay provider
        let mut delay = Delay::new(cp.SYST, clocks);

        loop {
            // Turn LEDs on one after the other with 500ms delay between them
            leds[LedColor::Orange].on();
            delay.delay_ms(500_u16);
            leds[LedColor::Red].on();
            delay.delay_ms(500_u16);
            leds[LedColor::Blue].on();
            delay.delay_ms(500_u16);
            leds[LedColor::Green].on();
            delay.delay_ms(500_u16);

            // Delay twice for half a second due to limited timer resolution
            delay.delay_ms(500_u16);
            delay.delay_ms(500_u16);

            // Turn LEDs off one after the other with 500ms delay between them
            leds[LedColor::Orange].off();
            delay.delay_ms(500_u16);
            leds[LedColor::Red].off();
            delay.delay_ms(500_u16);
            leds[LedColor::Blue].off();
            delay.delay_ms(500_u16);
            leds[LedColor::Green].off();
            delay.delay_ms(500_u16);

            // Delay twice for half a second due to limited timer resolution
            delay.delay_ms(500_u16);
            delay.delay_ms(500_u16);
        }
    }

    loop {
        continue;
    }
}
