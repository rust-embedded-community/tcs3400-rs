pub(crate) const DEVICE_ADDRESS: u8 = 0x39;

pub(crate) struct Register;

impl Register {
    pub(crate) const ENABLE: u8 = 0x80;
    pub(crate) const ATIME: u8 = 0x81;
    pub(crate) const WTIME: u8 = 0x83;
    pub(crate) const AILTL: u8 = 0x84;
    pub(crate) const AILTH: u8 = 0x85;
    pub(crate) const AIHTL: u8 = 0x86;
    pub(crate) const AIHTH: u8 = 0x87;
    pub(crate) const APERS: u8 = 0x8C;
    pub(crate) const CONFIG: u8 = 0x8D;
    pub(crate) const CONTROL: u8 = 0x8F;
    pub(crate) const ID: u8 = 0x92;
    pub(crate) const STATUS: u8 = 0x93;
    pub(crate) const CDATAL: u8 = 0x94;
    pub(crate) const RDATAL: u8 = 0x96;
    pub(crate) const GDATAL: u8 = 0x98;
    pub(crate) const BDATAL: u8 = 0x9A;
}

pub(crate) struct BitFlags;

impl BitFlags {
    pub(crate) const POWER_ON: u8 = 0b0000_0001; // PON
    pub(crate) const RGBC_EN: u8 = 0b0000_0010; // AEN
    pub(crate) const WAIT_EN: u8 = 0b0000_1000; // WEN
    pub(crate) const RGBC_INT_EN: u8 = 0b0001_0000; // AIEN
    pub(crate) const RGBC_VALID: u8 = 0b0000_0001; // AVALID
    pub(crate) const WLONG: u8 = 0b0000_0010;
}
