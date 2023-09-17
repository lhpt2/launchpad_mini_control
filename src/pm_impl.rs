/* Copyright (C) 2023 Lucas Haupt

This program is distributed under the terms of the 
GNU Lesser General Public License v3.0, 
see COPYING.LESSER file for license information
*/

//! # PM_IMPL
//!
//! Reference implementation for a midi backend
//! Implementation of the midilib traits for Portmidi library

use crate::midilib::MidiInterfaceError;
use crate::midilib::{DeviceInfo, Direction, Identifier, LaunchMessage};
use crate::{midilib as midi, BUFFER_SIZE};
use portmidi as pm;
use portmidi::{MidiEvent, MidiMessage};

/// Type aliases to be implemented for abstraction of midi backend
pub type InputPort<'a> = portmidi::InputPort<'a>;
pub type OutputPort<'a> = portmidi::OutputPort<'a>;
pub type MidiImpl = portmidi::PortMidi;

/// Implementation of the Error type MidiInterfaceError
impl From<pm::types::Error> for MidiInterfaceError {
    fn from(value: pm::Error) -> Self {
        match value {
            pm::Error::Invalid => MidiInterfaceError::Invalid(value.to_string()),
            pm::Error::Unknown => MidiInterfaceError::Unknown(value.to_string()),
            pm::Error::Unimplemented => MidiInterfaceError::Unimplemented(value.to_string()),
            pm::Error::NoDefaultDevice => MidiInterfaceError::NoDefaultDevice(value.to_string()),
            pm::Error::NotAnOutputDevice => {
                MidiInterfaceError::NotAnOutputDevice(value.to_string())
            }
            pm::Error::NotAnInputDevice => MidiInterfaceError::NotAnInputDevice(value.to_string()),
            pm::Error::PortMidi(err) => MidiInterfaceError::GenericBackendErr(err.to_string()),
        }
    }
}

/// Conversion traits for types in the midilib module
impl TryFrom<DeviceInfo> for pm::DeviceInfo {
    type Error = MidiInterfaceError;
    fn try_from(value: DeviceInfo) -> Result<Self, Self::Error> {
        let res = pm::DeviceInfo::new(value.id)?;
        Ok(res)
    }
}

impl From<pm::DeviceInfo> for DeviceInfo {
    fn from(value: portmidi::DeviceInfo) -> Self {
        DeviceInfo {
            id: value.id(),
            name: value.name().to_string(),
            dir: Direction::from(value.direction()),
        }
    }
}

impl From<Direction> for pm::Direction {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Input => pm::Direction::Input,
            Direction::Output => pm::Direction::Output,
        }
    }
}

impl From<pm::Direction> for Direction {
    fn from(value: portmidi::Direction) -> Self {
        match value {
            pm::Direction::Input => Direction::Input,
            pm::Direction::Output => Direction::Output,
        }
    }
}
impl From<MidiMessage> for LaunchMessage {
    fn from(msg: MidiMessage) -> Self {
        LaunchMessage {
            status: msg.status,
            data1: msg.data1,
            data2: msg.data2,
        }
    }
}

impl From<LaunchMessage> for MidiMessage {
    fn from(msg: LaunchMessage) -> Self {
        MidiMessage {
            status: msg.status,
            data1: msg.data1,
            data2: msg.data2,
            data3: 0x00,
        }
    }
}

impl From<LaunchMessage> for MidiEvent {
    fn from(value: LaunchMessage) -> Self {
        MidiEvent {
            message: MidiMessage::from(value),
            timestamp: 0,
        }
    }
}

/// Implementation of the Input trait (required for LaunchDevice)
impl midi::Input for InputPort<'_> {
    fn poll(&self) -> Result<bool, MidiInterfaceError> {
        Ok(self.poll()?)
    }
    fn read_n(&self, count: usize) -> Result<Option<Vec<LaunchMessage>>, MidiInterfaceError> {
        let res = self.read_n(count)?;
        let res = match res {
            None => None,
            Some(events) => {
                let events: Vec<LaunchMessage> = events
                    .into_iter()
                    .map(|ev| LaunchMessage::from(ev.message))
                    .collect();
                Some(events)
            }
        };
        Ok(res)
    }
}

/// Implementation of the Output trait (required for LaunchDevice)
impl midi::Output for OutputPort<'_> {
    fn write_message(&mut self, msg: LaunchMessage) -> Result<(), MidiInterfaceError> {
        Ok(self.write_message(MidiMessage::from(msg))?)
    }

    fn write_messages(&mut self, msgs: Vec<LaunchMessage>) -> Result<(), MidiInterfaceError> {
        Ok(self.write_events(msgs)?)
    }
}

/// Implementation of MidiInterface trait for PortMidi
impl<'a> midi::MidiInterface<'a> for MidiImpl {
    type MidiInput = InputPort<'a>;
    type MidiOutput = OutputPort<'a>;

    fn get_devices(&self) -> Result<Vec<DeviceInfo>, MidiInterfaceError> {
        let res = self.devices()?;
        Ok(res.into_iter().map(DeviceInfo::from).collect())
    }

    fn get_input(&'a self, identifier: Identifier) -> Result<InputPort<'a>, MidiInterfaceError> {
        let devs: Vec<pm::DeviceInfo> = self.devices()?;
        let input: pm::DeviceInfo = match identifier {
            Identifier::String(name) => {
                let filt_devs = devs
                    .into_iter()
                    .filter(|i| i.is_input() && i.name().eq(&name))
                    .collect::<Vec<_>>();
                match filt_devs.first() {
                    Some(d) => d.to_owned(),
                    None => {
                        return Err(MidiInterfaceError::NotAnInputDevice(format!(
                            "device with name {} not found",
                            name
                        )));
                    }
                }
            }
            Identifier::Number(id) => {
                let filt_devs = devs
                    .into_iter()
                    .filter(|i| i.is_input() && i.id() == id)
                    .collect::<Vec<_>>();
                match filt_devs.first() {
                    Some(d) => d.to_owned(),
                    None => {
                        return Err(MidiInterfaceError::NotAnInputDevice(format!(
                            "device with id {}",
                            id
                        )));
                    }
                }
            }
        };

        Ok(self.input_port(input, BUFFER_SIZE)?)
    }

    fn get_output(&'a self, identifier: Identifier) -> Result<OutputPort<'a>, MidiInterfaceError> {
        let devs: Vec<pm::DeviceInfo> = self.devices()?;
        let output: pm::DeviceInfo = match identifier {
            Identifier::String(name) => {
                let filt_devs = devs
                    .into_iter()
                    .filter(|i| i.is_output() && i.name().eq(&name))
                    .collect::<Vec<_>>();
                match filt_devs.first() {
                    None => {
                        return Err(MidiInterfaceError::NotAnOutputDevice(format!(
                            "output device with name {} not found",
                            name
                        )));
                    }
                    Some(d) => d.to_owned(),
                }
            }
            Identifier::Number(id) => {
                let filt_devs = devs
                    .into_iter()
                    .filter(|i| i.is_output() && i.id() == id)
                    .collect::<Vec<_>>();
                match filt_devs.first() {
                    None => {
                        return Err(MidiInterfaceError::NotAnOutputDevice(format!(
                            "output device with id {} not found",
                            id
                        )));
                    }
                    Some(d) => d.to_owned(),
                }
            }
        };

        Ok(self.output_port(output, BUFFER_SIZE)?)
    }

    fn get_in_out(
        &'a self,
        name: &str,
    ) -> Result<(Self::MidiInput, Self::MidiOutput), MidiInterfaceError> {
        let devs: Vec<pm::DeviceInfo> = self.devices()?;
        let devs: Vec<pm::DeviceInfo> = devs.into_iter().filter(|d| d.name() == name).collect();

        let in_p: Vec<&pm::DeviceInfo> = devs.iter().filter(|d| d.is_input()).collect();
        let in_p = match in_p.first() {
            None => {
                return Err(MidiInterfaceError::NotAnInputDevice(format!(
                    "no input with name {} found",
                    name
                )));
            }
            Some(d) => d.to_owned().to_owned(),
        };

        let out_p: Vec<&pm::DeviceInfo> = devs.iter().filter(|d| d.is_output()).collect();
        let out_p = match out_p.first() {
            None => {
                return Err(MidiInterfaceError::NotAnOutputDevice(format!(
                    "no output with name {} found",
                    name
                )));
            }
            Some(d) => d.to_owned().to_owned(),
        };

        let in_p = self.input_port(in_p, BUFFER_SIZE)?;
        let out_p = self.output_port(out_p, BUFFER_SIZE)?;

        Ok((in_p, out_p))
    }

    fn get_default_input(&'a self) -> Result<Self::MidiInput, MidiInterfaceError> {
        let id = self.default_input_device_id()?;
        let dev = self.device(id)?;
        let dev = self.input_port(dev, BUFFER_SIZE)?;
        Ok(dev)
    }

    fn get_default_output(&'a self) -> Result<Self::MidiOutput, MidiInterfaceError> {
        let id = self.default_output_device_id()?;
        let dev = self.device(id)?;
        let dev = self.output_port(dev, BUFFER_SIZE)?;
        Ok(dev)
    }
}