//! # Utils
//!
//! modul with helper types to facilitate ease of use of library
//!
pub mod launchpad;
pub mod mat_pos;
pub mod midi;
pub use self::launchpad::*;
pub use self::mat_pos::*;

pub use crate::MatPos;

pub mod conversions {
    use crate::MatPos;
    pub fn to_pitch_byte(col: u8, row: u8) -> u8 {
        if row > 7 {
            0x68 + col
        } else {
            (0x10 * row) + col
        }
    }
    pub fn to_matpos(pitch: u8) -> MatPos {
        MatPos::from(pitch)
    }
}
