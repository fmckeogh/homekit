use byteorder::{self, ByteOrder};

pub struct AdvertiseData([u8; 17]);

impl<'a, 'b> AdvertiseData
where
    'a: 'b,
{
    pub fn new(
        interval: Interval,
        pair_status: PairingStatus,
        device_id: u64,
        accessory_category: AccessoryCategory,
        global_state_number: u16,
        configuration_number: u8,
    ) -> Self {
        let mut inner = [0u8; 17];

        // CoID of Apple, Inc.
        inner[0] = 0x4C;
        inner[1] = 0x00;

        // Type (HomeKit)
        inner[2] = 0x06;

        // Advertising Interval and Length
        inner[3] = interval.value();

        // Status Flags
        inner[4] = pair_status.value();

        // 48-bit Device ID
        byteorder::BigEndian::write_u48(&mut inner[5..12], device_id);

        // Accessory Category Identifier
        byteorder::BigEndian::write_u16(&mut inner[11..13], accessory_category.value());

        // Global State Number
        byteorder::BigEndian::write_u16(&mut inner[13..15], global_state_number);

        // Configuration Number
        inner[15] = configuration_number;

        // Compatible Version
        inner[16] = 0x02;

        Self(inner)
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }
}

pub enum Interval {
    _10_25MS,
    _26_100MS,
    _101_300MS,
    _301_500MS,
    _501_1250MS,
    _1251_2500MS,
    _2500MS,
}

impl Interval {
    fn value(&self) -> u8 {
        match *self {
            Interval::_10_25MS => 0x2D,
            Interval::_26_100MS => 0x4D,
            Interval::_101_300MS => 0x6D,
            Interval::_301_500MS => 0x8D,
            Interval::_501_1250MS => 0xAD,
            Interval::_1251_2500MS => 0xCD,
            Interval::_2500MS => 0xED,
        }
    }
}

pub enum PairingStatus {
    Paired,
    NotPaired,
}

impl PairingStatus {
    fn value(&self) -> u8 {
        match *self {
            PairingStatus::Paired => 0b0,
            PairingStatus::NotPaired => 0b1,
        }
    }
}

pub enum AccessoryCategory {
    Other,
    Bridge,
    Fan,
    Garage,
    Lightbulb,
    DoorLock,
    Outlet,
    Switch,
    Thermostat,
    Sensor,
    SecuritySystem,
    Door,
    Window,
    WindowCovering,
    ProgrammableSwitch,
    RangeExtender,
    IPCamera,
    VideoDoorBell,
    AirPurifier,
    Reserved,
}

impl AccessoryCategory {
    fn value(&self) -> u16 {
        match *self {
            AccessoryCategory::Other => 1,
            AccessoryCategory::Bridge => 2,
            AccessoryCategory::Fan => 3,
            AccessoryCategory::Garage => 4,
            AccessoryCategory::Lightbulb => 5,
            AccessoryCategory::DoorLock => 6,
            AccessoryCategory::Outlet => 7,
            AccessoryCategory::Switch => 8,
            AccessoryCategory::Thermostat => 9,
            AccessoryCategory::Sensor => 10,
            AccessoryCategory::SecuritySystem => 11,
            AccessoryCategory::Door => 12,
            AccessoryCategory::Window => 13,
            AccessoryCategory::WindowCovering => 14,
            AccessoryCategory::ProgrammableSwitch => 15,
            AccessoryCategory::RangeExtender => 16,
            AccessoryCategory::IPCamera => 17,
            AccessoryCategory::VideoDoorBell => 18,
            AccessoryCategory::AirPurifier => 19,
            AccessoryCategory::Reserved => 65535,
        }
    }
}
