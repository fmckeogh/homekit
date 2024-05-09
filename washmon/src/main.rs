#![no_std]
#![no_main]

use {
    core::mem,
    defmt::info,
    defmt_rtt as _,
    embassy_executor::Spawner,
    embassy_nrf::{
        gpio::{Level, Output, OutputDrive},
        interrupt::Priority,
        peripherals::P0_14,
    },
    embassy_time::Timer,
    homekit::advertise,
    nrf_softdevice::{
        ble::{
            advertisement_builder::{LegacyAdvertisementBuilder, LegacyAdvertisementPayload},
            peripheral,
        },
        raw, Softdevice,
    },
    panic_probe as _,
};

#[embassy_executor::task]
async fn softdevice_task(sd: &'static Softdevice) -> ! {
    sd.run().await
}

#[embassy_executor::task]
async fn advertiser_task(sd: &'static Softdevice) -> ! {
    let mut config = peripheral::Config::default();
    config.interval = 50;

    let adv_data = homekit::advertise::AdvertiseData::new(
        advertise::Interval::_501_1250MS,
        advertise::PairingStatus::NotPaired,
        1234,
        advertise::AccessoryCategory::Sensor,
        1,
        1,
    )
    .as_payload();

    // but we can put it in the scan data
    // so the full name is visible once connected
    let scan_data: LegacyAdvertisementPayload = LegacyAdvertisementBuilder::new()
        .full_name("Hello, Rust!")
        .build();

    let adv = peripheral::NonconnectableAdvertisement::ScannableUndirected {
        adv_data: &adv_data,
        scan_data: &scan_data,
    };

    loop {
        peripheral::advertise(sd, adv, &config).await.unwrap();
    }
}

#[embassy_executor::task]
async fn blinker(mut led: Output<'static, P0_14>) -> ! {
    loop {
        led.set_high();
        Timer::after_millis(500).await;

        led.set_low();
        Timer::after_millis(500).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("starting");

    // 0 is Highest. Lower prio number can preempt higher prio number
    // Softdevice has reserved priorities 0, 1 and 4
    let mut config = embassy_nrf::config::Config::default();
    config.gpiote_interrupt_priority = Priority::P2;
    config.time_interrupt_priority = Priority::P2;
    let peripherals = embassy_nrf::init(config);

    let led = Output::new(peripherals.P0_14, Level::High, OutputDrive::Standard);
    spawner.spawn(blinker(led)).unwrap();

    let sd = init_softdevice();
    spawner.spawn(softdevice_task(sd)).unwrap();
    spawner.spawn(advertiser_task(sd)).unwrap();
}

fn init_softdevice() -> &'static mut Softdevice {
    let config = nrf_softdevice::Config {
        clock: Some(raw::nrf_clock_lf_cfg_t {
            source: raw::NRF_CLOCK_LF_SRC_RC as u8,
            rc_ctiv: 16,
            rc_temp_ctiv: 2,
            accuracy: raw::NRF_CLOCK_LF_ACCURACY_500_PPM as u8,
        }),
        conn_gap: Some(raw::ble_gap_conn_cfg_t {
            conn_count: 6,
            event_length: 24,
        }),
        conn_gatt: Some(raw::ble_gatt_conn_cfg_t { att_mtu: 256 }),
        gatts_attr_tab_size: Some(raw::ble_gatts_cfg_attr_tab_size_t {
            attr_tab_size: raw::BLE_GATTS_ATTR_TAB_SIZE_DEFAULT,
        }),
        gap_role_count: Some(raw::ble_gap_cfg_role_count_t {
            adv_set_count: 1,
            periph_role_count: 3,
        }),
        gap_device_name: Some(raw::ble_gap_cfg_device_name_t {
            p_value: b"HelloRust" as *const u8 as _,
            current_len: 9,
            max_len: 9,
            write_perm: unsafe { mem::zeroed() },
            _bitfield_1: raw::ble_gap_cfg_device_name_t::new_bitfield_1(
                raw::BLE_GATTS_VLOC_STACK as u8,
            ),
        }),
        ..Default::default()
    };

    Softdevice::enable(&config)
}
