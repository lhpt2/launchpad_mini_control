use crate::pad_identifier::PadIdentifier;
use crate::MatPos;

const COLOR_GRADIENT: [Color; 16] = [
    Color::Black,
    Color::DimGreen,
    Color::MedGreen,
    Color::Green,
    Color::Grellow,
    Color::DimGrellow,
    Color::Yellow,
    Color::MedYellow,
    Color::DimYellow,
    Color::YellOrange,
    Color::Orange,
    Color::DimORedange,
    Color::ORedange,
    Color::Red,
    Color::MedRed,
    Color::DimRed,
];

#[derive(PartialEq)]
pub enum MessageType {
    Off = 0x80,
    On = 0x90,
    Ctl = 0xb0,
}

#[derive(Copy, Clone, Debug)]
pub enum Color {
    Black = 0x00,
    DimGreen = 0x10,
    MedGreen = 0x20,
    Green = 0x30,
    Grellow = 0x31,
    DimGrellow = 0x21,
    Yellow = 0x32,
    MedYellow = 0x22,
    DimYellow = 0x11,
    YellOrange = 0x33,
    Orange = 0x23,
    DimORedange = 0x12,
    ORedange = 0x13,
    Red = 0x03,
    MedRed = 0x02,
    DimRed = 0x01,
}

pub enum BufferSetting {
    ZeroOnly = 0x00,
    OneActive = 0x01,
    ZeroActive = 0x04,
    OneOnly = 0x05,
}

pub enum GridMode {
    XY = 0x01,
    DrumRack = 0x02,
}

pub(crate) type Key = u8;
impl From<MatPos> for Key {
    fn from(pos: MatPos) -> Self {
        if pos.row > 7 {
            121 as Key
        } else {
            PadIdentifier::from(pos).key as Key
        }
    }
}
