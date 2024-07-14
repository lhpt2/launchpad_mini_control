/* Copyright (C) 2023 Lucas Haupt

This program is distributed under the terms of the
GNU Lesser General Public License v3.0,
see COPYING.LESSER file for license information
*/

use cartesian::*;

use crate::midilib::{Input, MidiInterfaceError, Output};
use crate::utils::conversions;
use crate::utils::launchpad::{BufferSetting, Color, GridMode, LaunchMessage, MessageType};
use crate::utils::midi::MidiEvent;

/// Number of Scene Launch button column
const SCENE_LAUNCH_COL: usize = 8;

/// Index for Control button row
const AUTOMAP_ROW: usize = 8;

/// maximum length for staying on the notegrid (only square buttons)
const NOTEGRID_MAX_LEN: usize = 8;

/// Maximum number of columns and rows on launchpad matrix
const MAX_PAD_COLSROWS: usize = 9;

/// Array of key (data1) bytes for indexing scene button column (on the right side)
const SCENE_BUTTON_COL: [u8; 8] = [0x08, 0x18, 0x28, 0x38, 0x48, 0x58, 0x68, 0x78];

/// Array of key (data1) bytes for indexing control button row (first row with round buttons)
/// Caution: status byte in message must be set to 0xB0
const STATUS_CONTROL_BUTTON_ROW: [u8; 8] = [0x68, 0x69, 0x6A, 0x6B, 0x6C, 0x6D, 0x6E, 0x6F];

/// This is the main struct for communicating with a LaunchpadMini
pub struct LaunchDeviceTemplate<I: Input, O: Output> {
    in_port: I,
    out_port: O,
    buffer_setting: u8,
}
impl<'a, I, O> LaunchDeviceTemplate<I, O>
where
    I: Input + 'a,
    O: Output + 'a,
{
    /// Create a new Connection to a Launchpad Mini Device.
    /// It takes an input and output port from a compatible midi backend (see midilib.rs),
    /// which are already the input and output port pointing to the Launchpad Mini device
    pub fn new(in_port: I, out_port: O) -> LaunchDeviceTemplate<I, O> {
        LaunchDeviceTemplate {
            in_port,
            out_port,
            buffer_setting: 0,
        }
    }

    /// Returns if messages from Launchpad are available or
    /// an MidiInterfaceError, if polling fails
    pub fn poll(&self) -> Result<(), MidiInterfaceError> {
        self.in_port.poll()
    }

    /// Read a single midi message, gives an Error if action fails
    pub fn read_single_msg(&self) -> Result<Option<MidiEvent>, MidiInterfaceError> {
        let opt = self.in_port.read_n(1)?;
        match opt {
            None => Ok(None),
            Some(msg) => match msg.first() {
                None => Ok(None),
                Some(m) => {
                    let res: MidiEvent = (*m).clone();
                    Ok(Some(res))
                }
            },
        }
    }

    /// Read a number of midi messages, return Error if action fails
    pub fn read_n_msgs(&self, n: usize) -> Result<Option<Vec<MidiEvent>>, MidiInterfaceError> {
        self.in_port.read_n(n)
    }

    /// Send a note msg to the Launchpad, turning lights on and of (and return Error, if action fails)
    pub fn send_note_msg(&mut self, on: bool, key: u8, vel: u8) -> Result<(), MidiInterfaceError> {
        let mut vel = vel;

        if !self.is_double_buffered() {
            vel |= 0x0C;
        }

        let mut mtype = MessageType::Off as u8;
        if on {
            mtype = MessageType::On as u8;
        }

        self.out_port.write_message(MidiEvent {
            timestamp: 0,
            status: mtype,
            pitch: key,
            velocity: vel,
        })?;

        Ok(())
    }

    /// Send multiple messages to the Launchpad (and return an Error, if action fails)
    pub fn send_messages(&mut self, msgs: Vec<LaunchMessage>) -> Result<(), MidiInterfaceError> {
        let msgs: Vec<MidiEvent> = msgs.iter().map(MidiEvent::from).collect();
        self.out_port.write_messages(msgs)?;
        Ok(())
    }

    /// Send a control message to the Launchpad (and return an Error, if action fails)
    pub fn send_ctl_msg(&mut self, data1: u8, data2: u8) -> Result<(), MidiInterfaceError> {
        self.out_port.write_message(MidiEvent {
            timestamp: 0,
            status: 0xb0,
            pitch: data1,
            velocity: data2,
        })?;
        Ok(())
    }

    /// Turn (almost) all lights on the Launchpad off, leave out control button row (first row with round buttons)
    /// (return an Error, if action fails)
    pub fn blackout(&mut self) -> Result<(), MidiInterfaceError> {
        self.set_all(Color::Black)?;
        Ok(())
    }

    /// Turn all lights on the Launchpad off
    /// Returns Error, if action fails
    pub fn full_blackout(&mut self) -> Result<(), MidiInterfaceError> {
        self.set_all(Color::Black)?;
        for i in 0..8 {
            self.send_ctl_msg(0x68 + i, Color::Black as u8)?;
        }
        Ok(())
    }

    /// Set the color/light at a position on the Launchpad Matrix
    /// Returns Error, if action fails
    pub fn set_position(
        &mut self,
        row: u8,
        col: u8,
        color: Color,
    ) -> Result<(), MidiInterfaceError> {
        let mut status = MessageType::On;
        if row > 7 {
            status = MessageType::Ctl;
        }
        self.out_port.write_message(MidiEvent::from(LaunchMessage {
            status,
            col,
            row,
            color,
        }))?;
        Ok(())
    }

    /// Set all buttons to one color
    /// Returns Error, if action fails
    pub fn set_all(&mut self, color: Color) -> Result<(), MidiInterfaceError> {
        for (x, y) in cartesian!(0..8, 0..9) {
            //self.send_note_msg(true, Key::from(MatPos::new(x, y)), color.into());
            self.out_port.write_message(MidiEvent {
                timestamp: 0,
                status: MessageType::On as u8,
                pitch: conversions::to_pitch_byte(x, y),
                velocity: color as u8,
            })?;
        }

        Ok(())
    }

    /// Select the GridMode of the Launchpad (XY-Mode or DrumRack-Mode)
    /// Returns Error, if action fails
    pub fn select_mode(&mut self, mode: GridMode) -> Result<(), MidiInterfaceError> {
        self.send_ctl_msg(0x00, mode as u8)?;
        Ok(())
    }

    /// Return if Launchpad is double buffered
    pub fn is_double_buffered(&self) -> bool {
        let buffered = 0x0F & self.buffer_setting;
        buffered == BufferSetting::OneActive as u8 || buffered == BufferSetting::ZeroActive as u8
    }

    /// Takes a 8x9 (row, col) matrix of Colors and sets the lights according to the matrix
    /// Returns Error, if action fails
    pub fn set_matrix(&mut self, mat: &[[Color; 9]; 8]) -> Result<(), MidiInterfaceError> {
        for (i, parent) in mat.iter().enumerate() {
            for (j, elem) in parent.iter().enumerate() {
                self.out_port.write_message(MidiEvent {
                    timestamp: 0,
                    status: 0x90,
                    pitch: conversions::to_pitch_byte(i as u8, j as u8),
                    velocity: *elem as u8,
                })?;
            }
        }

        Ok(())
    }

    /// Set lights of the first row on the Launchpad (round control buttons)
    /// Returns Error, if action fails
    pub fn set_first_row(&mut self, color: Color) -> Result<(), MidiInterfaceError> {
        for i in 0..8 {
            self.out_port.write_message(MidiEvent {
                timestamp: 0,
                status: 0xb0,
                pitch: 0x68 + i,
                velocity: color.into(),
            })?;
        }

        Ok(())
    }

    /// Reset the state of the Launchpad
    /// Returns Error, if action fails
    pub fn reset(&mut self) -> Result<(), MidiInterfaceError> {
        self.send_ctl_msg(0x00, 0x00)?;
        Ok(())
    }

    /// Set the buffer mode of the Launchpad (double buffering possible)
    /// The buffer modes are described in the BufferSetting struct
    /// Returns Error, if action fails
    pub fn set_buffer_mode(
        &mut self,
        setting: BufferSetting,
        copy: bool,
    ) -> Result<(), MidiInterfaceError> {
        if copy {
            self.buffer_setting = 0x30;
        } else {
            self.buffer_setting = 0x20;
        }

        self.buffer_setting |= setting as u8;
        self.send_ctl_msg(0x00, self.buffer_setting)?;
        Ok(())
    }

    /// Disable double buffering if activated, do nothing if not activated
    /// Returns Error, if action fails
    pub fn disable_double_buffering(&mut self) -> Result<(), MidiInterfaceError> {
        self.set_buffer_mode(BufferSetting::ZeroOnly, false)?;
        Ok(())
    }

    /// Swaps the active buffer and copies the current state to the new buffer,
    /// if copy equals true
    /// Returns Error, if action fails
    pub fn swap_buffers(&mut self, copy: bool) -> Result<(), MidiInterfaceError> {
        let setting = self.buffer_setting & 0x0F;

        if setting == BufferSetting::OneActive as u8 {
            self.set_buffer_mode(BufferSetting::ZeroActive, copy)?;
        } else {
            self.set_buffer_mode(BufferSetting::OneActive, copy)?;
        }

        Ok(())
    }

    /// Swap the active buffer without copying
    /// Returns Error, if action fails
    pub fn hard_swap(&mut self) -> Result<(), MidiInterfaceError> {
        self.swap_buffers(false)?;
        Ok(())
    }

    /// Set the refresh cycle of the Launchpad LEDs
    /// numerator is supposed to be in \[1; 16\]
    /// denominator is supposed to be in \[3; 18\]
    /// Returns Error, if action fails
    pub fn set_duty_cycle(
        &mut self,
        numerator: u8,
        denominator: u8,
    ) -> Result<(), MidiInterfaceError> {
        let mut numerator = numerator;
        let mut denominator = denominator;

        numerator = numerator.clamp(1, 16);
        denominator = denominator.clamp(3, 18);

        let mut data1: u8 = 0x1f;
        let mut data2: u8 = 0x10 * (numerator - 9) + (denominator - 3);

        if numerator < 9 {
            data1 = 0x1e;
            data2 = 0x10 * (numerator - 1) + (denominator - 3);
        }

        self.out_port.write_message(MidiEvent {
            timestamp: 0,
            status: 0xb0,
            pitch: data1,
            velocity: data2,
        })?;

        Ok(())
    }
}

impl<I: Input, O: Output> Drop for LaunchDeviceTemplate<I, O> {
    fn drop(&mut self) {
        let _ = self.reset();
    }
}
