/* Copyright (C) 2023 Lucas Haupt

This program is distributed under the terms of the
GNU Lesser General Public License v3.0,
see COPYING.LESSER file for license information
*/

use crate::utils::MessageType;
use crate::utils::midi::MidiMessage;

/// Struct representing a position on the Launchpad matrix with various type conversions
#[derive(Debug)]
pub struct MatPos {
    pub col: u8,
    pub row: u8,
}
impl MatPos {
    pub fn new(col: u8, row: u8) -> MatPos {
        MatPos { row, col }
    }
    pub fn get_as_tuple(self) -> (u8, u8) {
        (self.row, self.col)
    }

    pub fn get_msg_type(&self) -> MessageType {
       return if self.row > 7 {
          MessageType::Ctl
       } else {
          MessageType::Off
       }
    }

    pub fn to_pitch(&self) -> u8 {
        return if self.row > 7 {
            0x68 + self.col
        } else {
            (0x10 * self.row) + self.col
        }
    }

    pub fn get_midi_msg(self) -> MidiMessage {
        MidiMessage {
            status: self.get_msg_type() as u8,
            pitch: self.to_pitch(),
            velocity: 0,
        }
    }
}

impl From<MidiMessage> for MatPos {
    fn from(msg: MidiMessage) -> Self {
        if msg.status == MessageType::Ctl as u8 {
            MatPos {
                row: 8,
                col: msg.pitch % 0x68,
            }
        } else {
            MatPos {
                row: msg.pitch / 0x10,
                col: msg.pitch % 0x10,
            }
        }
    }
}

impl From<u8> for MatPos {
    fn from(pitch: u8) -> Self {
        if pitch >= 0x68 && pitch <= 0x6F {
            MatPos {
                row: 8,
                col: pitch % 0x68,
            }
        } else {
            MatPos {
                row: pitch / 0x10,
                col: pitch % 0x10,
            }
        }
    }
}
