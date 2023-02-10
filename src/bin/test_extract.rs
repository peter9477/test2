#![no_main]
#![no_std]

use test2 as _; // global logger + panicking-behavior + memory layout


#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("running");

    let res = test2::extract::test_function();
    assert_eq!(res, (1, 99, [3, 99]));

    defmt::info!("it worked!");

    test2::exit();
}
