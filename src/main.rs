#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_rp::{bind_interrupts, peripherals::USB};
use embassy_usb::class::hid::{Config as HidConfig, HidWriter, State};
use embassy_usb::class::hid::{HidBootProtocol, HidSubclass};
use embassy_usb::Config;
use static_cell::StaticCell;
use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    info!("RustyDucky Initialized!");

    let driver = Driver::new(p.USB, Irqs);

    let mut config = Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some("Logitech");
    config.product = Some("USB Keyboard");
    config.serial_number = Some("87654321");

    static DEVICE_DESC: StaticCell<[u8; 256]> = StaticCell::new();
    static CONFIG_DESC: StaticCell<[u8; 256]> = StaticCell::new();
    static BOS_DESC: StaticCell<[u8; 256]> = StaticCell::new();
    static CONTROL_BUF: StaticCell<[u8; 64]> = StaticCell::new();

    let mut builder = embassy_usb::Builder::new(
        driver,
        config,
        DEVICE_DESC.init([0; 256]),
        CONFIG_DESC.init([0; 256]),
        BOS_DESC.init([0; 256]),
        CONTROL_BUF.init([0; 64]),
    );

    static HID_STATE: StaticCell<State> = StaticCell::new();
    let hid_state = HID_STATE.init(State::new());

    // FIXED: Using the exact variants 'No' and 'Keyboard' from your library
    let hid_config = HidConfig {
        report_descriptor: KeyboardReport::desc(),
        request_handler: None,
        poll_ms: 1,
        max_packet_size: 8,
        hid_subclass: HidSubclass::No,
        hid_boot_protocol: HidBootProtocol::Keyboard,
    };

    let mut writer = HidWriter::<_, 8>::new(&mut builder, hid_state, hid_config);

    let mut usb = builder.build();
    let usb_fut = usb.run();

    let attack_fut = async {
        embassy_time::Timer::after_secs(5).await;
        info!("Sending Windows+R...");

        let report = KeyboardReport {
            modifier: 0x08, // Windows key
            reserved: 0,
            leds: 0,
            keycodes: [0x15, 0, 0, 0, 0, 0], // 'R' key
        };

        let _ = writer.write_serialize(&report).await;
        embassy_time::Timer::after_millis(100).await;
        let _ = writer.write_serialize(&KeyboardReport::default()).await;

        info!("Payload finished!");
    };

    embassy_futures::join::join(usb_fut, attack_fut).await;
}
