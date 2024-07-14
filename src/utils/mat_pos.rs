/* Copyright (C) 2023 Lucas Haupt

This program is distributed under the terms of the
GNU Lesser General Public License v3.0,
see COPYING.LESSER file for license information
*/

use crate::utils::midi::{MidiEvent, MidiMessage};
use crate::utils::{conversions, MessageType};

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
        if self.row > 7 {
            MessageType::Ctl
        } else {
            MessageType::Off
        }
    }

    pub fn to_pitch(&self) -> u8 {
        conversions::to_pitch_byte(self.col, self.row)
    }

    pub fn get_midi_msg(self) -> MidiMessage {
        MidiMessage {
            status: self.get_msg_type() as u8,
            pitch: self.to_pitch(),
            velocity: 0,
        }
    }
}

impl From<&MidiMessage> for MatPos {
    fn from(msg: &MidiMessage) -> Self {
        MatPos::from(MidiEvent::from(msg))
    }
}

impl From<&MidiEvent> for MatPos {
    fn from(ev: &MidiEvent) -> Self {
        if ev.status == MessageType::Ctl as u8 {
            MatPos {
                row: 8,
                col: ev.pitch % 0x68,
            }
        } else {
            MatPos {
                row: ev.pitch / 0x10,
                col: ev.pitch % 0x10,
            }
        }
    }
}

impl From<MidiEvent> for MatPos {
    fn from(ev: MidiEvent) -> Self {
        MatPos::from(&ev)
    }
}

impl From<u8> for MatPos {
    fn from(pitch: u8) -> Self {
        if (0x68..=0x6F).contains(&pitch){
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
