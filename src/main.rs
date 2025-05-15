#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use rp235x_hal::block::ImageDef;

#[used]
#[link_section = ".start_block"]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

#[rp235x_hal::entry]
fn main() -> ! {
    info!("Hello from Pico 2!");
    loop {}
}
