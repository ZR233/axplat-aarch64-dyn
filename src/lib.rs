#![no_std]

use pie_boot::BootArgs;

#[macro_use]
extern crate axplat;

mod console;
mod init;
mod irq;
mod mem;
mod power;
mod time;

mod config {
    axconfig_gen_macros::include_configs!("axconfig.toml");
}

#[pie_boot::entry]
fn main(_args: &BootArgs) -> ! {
    // TODO: Implement actual bootstrap logic
    axplat::call_main(0, 0);
}
