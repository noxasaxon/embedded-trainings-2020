#![deny(unused_must_use)]
#![no_main]
#![no_std]

use core::str;

use cortex_m_rt::entry;
use heapless::{consts, Vec};
// this imports `beginner/apps/lib.rs` to retrieve our global logger + panicking-behavior
use apps as _;

#[entry]
fn main() -> ! {
    dk::init().unwrap();

    // a buffer with capacity for 2 bytes
    let mut buffer = Vec::<u8, consts::U2>::new();
    //                                 ^^ capacity; this is a type not a value

    // do some insertions
    buffer.push(b'H').expect("buffer full");
    buffer.push(b'i').expect("buffer full");

    // look into the contents so far
    defmt::info!("{:?}", defmt::Debug2Format(&buffer));
    //                   ^^^^^^^^^^^^^^^^^^^ this adapter iscurrently needed to log `heapless`
    //                                       data structures (like `Vec` here) with `defmt`

    // or more readable
    // NOTE utf-8 conversion works as long as you only push bytes in the ASCII range (0..=127)
    defmt::info!(
        "{}",
        str::from_utf8(&buffer).expect("content was not UTF-8")
    );

    dk::exit()
}
