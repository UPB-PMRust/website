// +---------------------------------------------------------------------------+
// |                   PM/MA lab embassy utilities                             |
// +---------------------------------------------------------------------------+

//! This crate contains functions and types that act as wrappers over embassy's
//! and embedded-hal more complex routines.

#![no_std]

use cyw43::{Control, NetDriver, PowerManagementMode};
use cyw43_pio::PioSpi;
use defmt::*;
use embassy_net::StackResources;
use embassy_rp::clocks::RoscRng;
use embassy_rp::{
    gpio::Output,
    peripherals::{DMA_CH2, PIO0},
};
use rand::RngCore as _;
use static_cell::StaticCell;

pub use cyw43;
pub use cyw43_pio;
pub use embassy_rp;

#[macro_export]
macro_rules! init_wifi {
    ($spawner_ref:expr, $p:expr) => {
        async {
            $crate::embassy_rp::bind_interrupts!(struct PioIrq {
                PIO0_IRQ_0 => $crate::embassy_rp::pio::InterruptHandler<$crate::embassy_rp::peripherals::PIO0>;
            });

            // Move the function directly here
            let pwr = $crate::embassy_rp::gpio::Output::new($p.PIN_23, $crate::embassy_rp::gpio::Level::Low);
            let cs = $crate::embassy_rp::gpio::Output::new($p.PIN_25, $crate::embassy_rp::gpio::Level::High);
            let mut pio = $crate::embassy_rp::pio::Pio::new($p.PIO0, PioIrq);

            let spi = $crate::cyw43_pio::PioSpi::new(
                &mut pio.common,
                pio.sm0,
                $crate::cyw43_pio::RM2_CLOCK_DIVIDER,
                pio.irq0,
                cs,
                $p.PIN_24,
                $p.PIN_29,
                $p.DMA_CH2,
            );

            let (net_driver, mut ctrl) = $crate::init_cy43w(pwr, spi, $spawner_ref).await;
            $crate::init_controller(&mut ctrl, $crate::cyw43::PowerManagementMode::None).await;
            (net_driver, ctrl)
        }
    };
}

/// This task runs the wifi chip driver. This will need to run in an infinite loop.
#[embassy_executor::task]
async fn cyw43_task(
    runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH2>>,
) -> ! {
    runner.run().await
}

/// This task runs the network stack, used for processing network events.
#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, cyw43::NetDriver<'static>>) -> ! {
    runner.run().await
}

/// Initialize the CYW43 Wifi chip driver and spawn the task that runs the driver.
///
/// Returns a handle to the network device, control handle and a runner for driving the low level
/// stack.
pub async fn init_cy43w(
    pwr: Output<'static>,
    spi: PioSpi<'static, PIO0, 0, DMA_CH2>,
    spawner: &embassy_executor::Spawner,
) -> (NetDriver<'static>, Control<'static>) {
    let fw = include_bytes!("../../cyw43-firmware/43439A0.bin");

    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    let (net_device, control, runner) = cyw43::new(state, pwr, spi, fw).await;
    unwrap!(spawner.spawn(cyw43_task(runner)));

    (net_device, control)
}

/// Initialize the WiFi controller with the given power management mode.
pub async fn init_controller(controller: &mut Control<'static>, mode: PowerManagementMode) {
    let clm = include_bytes!("../../cyw43-firmware/43439A0_clm.bin");
    controller.init(clm).await;
    controller.set_power_management(mode).await;
}

/// Initialize the network stack.
pub fn init_network_stack<const SOCK: usize>(
    spawner: &embassy_executor::Spawner,
    net_device: NetDriver<'static>,
    resources: &'static StaticCell<StackResources<SOCK>>,
    config: embassy_net::Config,
) -> embassy_net::Stack<'static> {
    // Generate random seed
    let seed = RoscRng.next_u64();

    let (stack, runner) = embassy_net::new(
        net_device,
        config,
        resources.init(StackResources::new()),
        seed,
    );

    unwrap!(spawner.spawn(net_task(runner)));

    stack
}
