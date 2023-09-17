/* Copyright (C) 2023 Lucas Haupt

This program is distributed under the terms of the 
GNU Lesser General Public License v3.0, 
see COPYING.LESSER file for license information
*/

//! # Midilib
//! This is the trait to be implemented by the Midi backend
//! to be supported by the library (adaptor pattern)

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};


/// MidiInterface adapter for different midi backends,
/// has to comply with the Input and Output trait
pub trait MidiInterface<'a> {
    type MidiInput: Input + 'a;
    type MidiOutput: Output + 'a;

    /// Request vector of all midi devices (returns an Error if it fails)
    fn get_devices(&self) -> Result<Vec<DeviceInfo>, MidiInterfaceError>;

    /// Get the input with the supplied identifier (id or name) or return Error,
    /// if no input with id/name existent or on other error
    fn get_input(&'a self, identifier: Identifier) -> Result<Self::MidiInput, MidiInterfaceError>;

    /// Get the output with the supplied identifier (id or name) or return Error,
    /// if no output with id/name existent or on other error
    fn get_output(&'a self, identifier: Identifier)
        -> Result<Self::MidiOutput, MidiInterfaceError>;

    /// Get the input and output with the supplied name or return error
    /// if no output with id/name existent or on other error
    fn get_in_out(
        &'a self,
        name: &str,
    ) -> Result<(Self::MidiInput, Self::MidiOutput), MidiInterfaceError>;

    /// Return default input
    fn get_default_input(&'a self) -> Result<Self::MidiInput, MidiInterfaceError>;

    /// Return default output
    fn get_default_output(&'a self) -> Result<Self::MidiOutput, MidiInterfaceError>;
}

/// Trait representing an Output compatible with LaunchDevice and MidiInterface
pub trait Output {
    /// Write one messages to output port
    fn write_message(&mut self, msg: LaunchMessage) -> Result<(), MidiInterfaceError>;

    /// Write multiple messages to output port
    fn write_messages(&mut self, msg: Vec<LaunchMessage>) -> Result<(), MidiInterfaceError>;
}

/// Trait representing an Input compatible with LaunchDevice and MidiInterface
pub trait Input {
    /// Poll for new messages (true if messages available)
    fn poll(&self) -> Result<bool, MidiInterfaceError>;

    /// Read n messages from input port
    fn read_n(&self, count: usize) -> Result<Option<Vec<LaunchMessage>>, MidiInterfaceError>;
}

/// Direction being either input or output device type
#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    /// incoming midi messages
    Input,
    /// outgoing midi messages
    Output,
}

/// Contains info about a midi device
#[derive(Debug)]
pub struct DeviceInfo {
    /// unique identifier
    pub id: i32,
    /// name of device as string
    pub name: String,
    /// direction denoting input/output
    pub dir: Direction,
}
impl DeviceInfo {
    pub fn is_input(&self) -> bool {
        self.dir == Direction::Input
    }
    pub fn is_output(&self) -> bool {
        self.dir == Direction::Output
    }
}

/// Struct for a MidiMessage for communication with Launchpad
#[derive(Debug, Clone)]
pub struct LaunchMessage {
    /// status byte (either control message 0x, )
    pub status: u8,
    pub data1: u8,
    pub data2: u8,
}

/// device identifier, either being a name (string) or a id (number)
pub enum Identifier {
    String(String),
    Number(i32),
}

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        let name = value.to_string();
        Identifier::String(name)
    }
}

impl From<i32> for Identifier {
    fn from(value: i32) -> Self {
        Identifier::Number(value)
    }
}

/// Error for midi interface to be implemented for midi backend
#[derive(Debug)]
pub enum MidiInterfaceError {
    Unknown(String),
    Unimplemented(String),
    NoDefaultDevice(String),
    NotAnInputDevice(String),
    NotAnOutputDevice(String),
    Invalid(String),
    GenericBackendErr(String),
}

impl Error for MidiInterfaceError {}

impl Display for MidiInterfaceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            MidiInterfaceError::Unknown(msg) => msg.to_string(),
            MidiInterfaceError::Unimplemented(msg) => msg.to_string(),
            MidiInterfaceError::NoDefaultDevice(msg) => msg.to_string(),
            MidiInterfaceError::NotAnInputDevice(msg) => msg.to_string(),
            MidiInterfaceError::NotAnOutputDevice(msg) => msg.to_string(),
            MidiInterfaceError::Invalid(msg) => msg.to_string(),
            MidiInterfaceError::GenericBackendErr(msg) => msg.to_string(),
        };
        write!(f, "E midi backend: {}", message)
    }
}