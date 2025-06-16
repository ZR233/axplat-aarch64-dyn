#![cfg(all(target_arch = "aarch64", target_os = "none"))]
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
    axconfig_macros::include_configs!(path_env = "AX_CONFIG_PATH", fallback = "axconfig.toml");
    assert_str_eq!(
        PACKAGE,
        env!("CARGO_PKG_NAME"),
        "`PACKAGE` field in the configuration does not match the Package name. Please check your configuration file."
    );
}
#[pie_boot::entry]
fn main(_args: &BootArgs) -> ! {
    // TODO: Implement actual bootstrap logic
    axplat::call_main(0, 0);
}
