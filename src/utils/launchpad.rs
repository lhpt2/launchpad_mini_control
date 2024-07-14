/* Copyright (C) 2023 Lucas Haupt

This program is distributed under the terms of the
GNU Lesser General Public License v3.0,
see COPYING.LESSER file for license information
*/
use crate::MidiInterfaceError;

#[derive(Debug, Clone)]
pub struct LaunchMessage {
    pub status: MessageType,
    pub col: u8,
    pub row: u8,
    pub color: Color,
}

impl From<MidiMessage> for LaunchMessage {
    fn from(value: MidiMessage) -> Self {
        let pos = MatPos::from(&value);
        let mtype = MessageType::try_from(&value.status).expect("no valid status byte");
        LaunchMessage {
            status: mtype,
            col: pos.col,
            row: pos.row,
            color: Color::from(value.data2),
        }
    }
}

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

/// Message type of message for the Launchpad, either
/// On (Light On, Button pressed), Off (Light Off, Button released), or
/// Ctl (Control msg, one of the round buttons in first row has been pressed)
#[derive(PartialEq, Clone, Debug)]
pub enum MessageType {
    Off = 0x80,
    On = 0x90,
    Ctl = 0xb0,
}

impl TryFrom<u8> for MessageType {
    type Error = MidiInterfaceError;

    fn try_from(value: u8) -> Result<Self, MidiInterfaceError> {
        match value {
            0x80 => Ok(MessageType::Off),
            0x90 => Ok(MessageType::On),
            0xb0 => Ok(MessageType::Ctl),
            _ => Err(MidiInterfaceError::Invalid("Invalid".to_string())),
        }
    }
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

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        match value {
            x if x == Color::DimGreen as u8 => Color::DimGreen,
            x if x == Color::MedGreen as u8 => Color::MedGreen,
            x if x == Color::Green as u8 => Color::Green,
            x if x == Color::Grellow as u8 => Color::Grellow,
            x if x == Color::DimGrellow as u8 => Color::DimGrellow,
            x if x == Color::Yellow as u8 => Color::Yellow,
            x if x == Color::MedYellow as u8 => Color::MedYellow,
            x if x == Color::DimYellow as u8 => Color::DimYellow,
            x if x == Color::YellOrange as u8 => Color::YellOrange,
            x if x == Color::Orange as u8 => Color::Orange,
            x if x == Color::DimORedange as u8 => Color::DimORedange,
            x if x == Color::ORedange as u8 => Color::ORedange,
            x if x == Color::Red as u8 => Color::Red,
            x if x == Color::MedRed as u8 => Color::MedRed,
            x if x == Color::DimRed as u8 => Color::DimRed,
            _ => Color::Black,
        }
    }
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
