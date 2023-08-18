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

// Use HAL crate for stm32f407
use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
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

        // Initialize on-board LEDs pd12-15
        let mut orange_led = gpiod.pd12.into_push_pull_output();
        let mut red_led = gpiod.pd13.into_push_pull_output();
        let mut blue_led = gpiod.pd14.into_push_pull_output();
        let mut green_led = gpiod.pd15.into_push_pull_output();

        // Constrain clock registers
        let rcc = p.RCC.constrain();

        // Configure clock to 168 MHz (i.e. the maximum) and freeze it
        let clocks = rcc.cfgr.sysclk(168.MHz()).freeze();

        // Get delay provider
        let mut delay = cp.SYST.delay(&clocks);

        loop {
            // Turn LEDs on one after the other with 500ms delay between them
            orange_led.set_low();
            delay.delay_ms(500_u16);
            red_led.set_low();
            delay.delay_ms(500_u16);
            blue_led.set_low();
            delay.delay_ms(500_u16);
            green_led.set_low();
            delay.delay_ms(500_u16);

            // Delay twice for half a second due to limited timer resolution
            delay.delay_ms(500_u16);
            delay.delay_ms(500_u16);

            // Turn LEDs off one after the other with 500ms delay between them
            orange_led.set_high();
            delay.delay_ms(500_u16);
            red_led.set_high();
            delay.delay_ms(500_u16);
            blue_led.set_high();
            delay.delay_ms(500_u16);
            green_led.set_high();
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
