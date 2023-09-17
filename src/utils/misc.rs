/* Copyright (C) 2023 Lucas Haupt

This program is distributed under the terms of the 
GNU Lesser General Public License v3.0, 
see COPYING.LESSER file for license information
*/
use crate::utils::PadIdentifier;
use crate::MatPos;

/// Color gradient array, trying to sort all colors on a spectrum
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

/// Message type of a message for the Launchpad, either
/// On (Light On, Button pressed), Off (Light Off, Button released), or
/// Ctl (Control msg, one of the round buttons in first row has been pressed)
#[derive(PartialEq)]
pub enum MessageType {
    Off = 0x80,
    On = 0x90,
    Ctl = 0xb0,
}

/// All colors the Launchpad is able to display
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

/// Buffer modes for the Launchpad.
/// The Launchpad has two internal buffers, enabling it to make use of double buffering
/// There are four possible modes:
/// ZeroOnly: Using only buffer 0 (single buffering)
/// OneOnly: Using only buffer 1  (single buffering)
/// OneActive: Both buffers with buffer 1 being displayed  (double buffering)
/// ZeroActive: Both buffers with buffer 2 being displayed  (double buffering)
pub enum BufferSetting {
    ZeroOnly = 0x00,
    OneActive = 0x01,
    ZeroActive = 0x04,
    OneOnly = 0x05,
}

/// The Launchpad supports two grid modes, meaning the layout of the midi notes on the Launchpad
/// - The XY mode maps the midi notes from left two right and top (first square button),
/// to bottom starting from 0xR0 to 0xR8 (R being the row number starting from 0)
/// - The Drum Rack mode has a more complicated mapping pattern (see document)
/// See page 6 of doc/doc_launchpad-programmers-reference.pdf document
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
