/* Copyright (C) 2023 Lucas Haupt

This program is distributed under the terms of the
GNU Lesser General Public License v3.0,
see COPYING.LESSER file for license information
*/

//! # Midirs Implementation
//!
//! Reference implementation for a midi backend
//! Implementation of the midilib traits for Portmidi library

use crate::midilib::MidiInterfaceError;
use crate::midilib::{DeviceInfo, Direction, Identifier};
use crate::utils::LaunchMessage;
use crate::{midilib as midi, BUFFER_SIZE};
use midir as md;
use midir::{MidiInputPort, MidiOutputConnection, MidiOutputPort};
use std::fmt::format;
use std::panic;
use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;

/// Type aliases to be implemented for abstraction of midi backend
pub struct InputPort {
    port: Option<md::MidiInputConnection<mpsc::SyncSender<LaunchMessage>>>,
    queue: mpsc::Receiver<LaunchMessage>,
}
impl InputPort {
    fn callback(bytes: &[u8], sender: &mut mpsc::SyncSender<LaunchMessage>) {
        match sender.send(LaunchMessage {
            status: bytes[0],
            data1: bytes[1],
            data2: bytes[2],
        }) {
            Ok(_) => {}
            Err(e) => eprintln!("Receiver not able to receive data: {}", e),
        };
    }
    fn get_port_by_name(port_name: &str) -> Result<md::MidiInputPort, MidiInterfaceError> {
        let midi = md::MidiInput::new("info").expect("Input init");
        let port: Vec<_> = midi
            .ports()
            .into_iter()
            .filter(|p| midi.port_name(p).expect("Portname") == port_name)
            .collect();
        let port = match port.first() {
            None => {
                return Err(MidiInterfaceError::NotAnInputDevice(format!(
                    "{port_name} not found"
                )))
            }
            Some(p) => p.to_owned(),
        };
        Ok(port)
    }

    pub fn reconnect(
        &mut self,
        client_name: &str,
        port_name: &str,
        conn_name: &str,
        buffer_size: usize,
    ) -> Result<(), MidiInterfaceError> {
        let port = match Self::get_port_by_name(port_name) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };
        if let Some(p) = self.port.take() {
            p.close();
        }
        let (sender, receiver) = mpsc::sync_channel::<LaunchMessage>(buffer_size);
        self.queue = receiver;
        self.port = Some(
            md::MidiInput::new(client_name)
                .expect("Input init")
                .connect(
                    &port,
                    conn_name,
                    |_, bytes, sender| Self::callback(bytes, sender),
                    sender,
                )
                .unwrap(),
        );
        Ok(())
    }

    pub fn new_by_name(
        client_name: &str,
        port_name: &str,
        conn_name: &str,
        buffer_size: usize,
    ) -> Self {
        let port = match Self::get_port_by_name(port_name) {
            Ok(p) => p,
            Err(e) => panic!("{}: {port_name} not found", e),
        };
        Self::new(client_name, &port, conn_name, buffer_size)
    }

    pub fn new(
        client_name: &str,
        port: &md::MidiInputPort,
        conn_name: &str,
        buffer_size: usize,
    ) -> Self {
        let (sender, receiver) = mpsc::sync_channel::<LaunchMessage>(buffer_size);
        let midi_in = md::MidiInput::new(client_name)
            .expect("Input Initialization")
            .connect(
                port,
                conn_name,
                |_, bytes, sender| Self::callback(bytes, sender),
                sender,
            )
            .expect("Input connection");
        println!("Connected to input {}", DeviceInfo::from(port));
        InputPort {
            port: Some(midi_in),
            queue: receiver,
        }
    }
}

impl Drop for InputPort {
    fn drop(&mut self) {
        if let Some(p) = self.port.take() {
            p.close();
        }
    }
}

pub struct OutputPort {
    port: Option<md::MidiOutputConnection>,
}
impl OutputPort {
    pub fn new(conn: md::MidiOutputConnection) -> Self {
        OutputPort { port: Some(conn) }
    }
}

impl Drop for OutputPort {
    fn drop(&mut self) {
        if let Some(x) = self.port.take() {
            x.close();
        }
    }
}

pub struct MidiImpl {
    client_name: String,
}
impl MidiImpl {
    pub fn input(&self) -> md::MidiInput {
        md::MidiInput::new(&self.client_name).expect("Input Initialization")
    }
    pub fn output(&self) -> md::MidiOutput {
        md::MidiOutput::new(&self.client_name).expect("Output Initialization")
    }

    fn find_input_port(&self, name: &str) -> Result<MidiInputPort, MidiInterfaceError> {
        let inp: Vec<_> = self
            .input()
            .ports()
            .into_iter()
            .filter(|p| self.input().port_name(p).unwrap().contains(name))
            .collect();
        match inp.first() {
            None => Err(MidiInterfaceError::NotAnInputDevice(format!(
                "{name} not found"
            ))),
            Some(p) => Ok(p.to_owned()),
        }
    }

    fn find_output_port(&self, name: &str) -> Result<MidiOutputPort, MidiInterfaceError> {
        let out: Vec<_> = self
            .output()
            .ports()
            .into_iter()
            .filter(|p| self.output().port_name(p).unwrap().contains(name))
            .collect();
        match out.first() {
            None => Err(MidiInterfaceError::NotAnOutputDevice(format!(
                "{name} not found"
            ))),
            Some(p) => Ok(p.to_owned()),
        }
    }
}

/// Implementation of the Input trait (required for LaunchDevice)
impl midi::Input for InputPort {
    fn poll(&self) -> Result<(), MidiInterfaceError> {
        Ok(())
    }
    fn read_n(&self, count: usize) -> Result<Option<Vec<LaunchMessage>>, MidiInterfaceError> {
        let mut res = Vec::<LaunchMessage>::new();
        for _ in 0..count {
            match self.queue.try_recv() {
                Ok(m) => {
                    res.push(m);
                }
                Err(e) => match e {
                    TryRecvError::Empty => {
                        return Ok(None);
                    }
                    TryRecvError::Disconnected => {
                        return Err(MidiInterfaceError::Invalid(
                            "Channel disconnected".to_string(),
                        ));
                    }
                },
            }
        }
        Ok(Some(res))
    }
}

/// Implementation of the Output trait (required for LaunchDevice)
impl midi::Output for OutputPort {
    fn write_message(&mut self, msg: LaunchMessage) -> Result<(), MidiInterfaceError> {
        match self
            .port
            .as_mut()
            .unwrap()
            .send(&[msg.status, msg.data1, msg.data2])
        {
            Ok(_) => Ok(()),
            Err(e) => Err(MidiInterfaceError::from(e)),
        }
    }

    fn write_messages(&mut self, msg: Vec<LaunchMessage>) -> Result<(), MidiInterfaceError> {
        msg.into_iter().for_each(|m| {
            if let Err(e) = self
                .port
                .as_mut()
                .unwrap()
                .send(&[m.status, m.data1, m.data2])
            {
                eprintln!("Send Error: {e}")
            }
        });
        Ok(())
    }
}

/// Implementation of MidiInterface trait for PortMidi
impl<'a> midi::MidiInterface<'a> for MidiImpl {
    type MidiInput = InputPort;
    type MidiOutput = OutputPort;

    fn new(client_name: &str) -> Self {
        MidiImpl {
            client_name: client_name.to_string(),
        }
    }

    fn get_devices(&self) -> Result<Vec<DeviceInfo>, MidiInterfaceError> {
        let ins: md::MidiInputPorts = md::MidiInput::new(&self.client_name)?.ports();
        let outs: md::MidiOutputPorts = md::MidiOutput::new(&self.client_name)?.ports();

        let mut ins: Vec<DeviceInfo> = ins.into_iter().map(DeviceInfo::from).collect();
        let outs: Vec<DeviceInfo> = outs.into_iter().map(DeviceInfo::from).collect();

        ins.extend(outs);
        Ok(ins)
    }

    fn get_input_devices(&self) -> Result<Vec<DeviceInfo>, MidiInterfaceError> {
        let ins = md::MidiInput::new(&self.client_name)?.ports();
        Ok(ins.into_iter().map(DeviceInfo::from).collect())
    }

    fn get_output_devices(&self) -> Result<Vec<DeviceInfo>, MidiInterfaceError> {
        let outs = md::MidiOutput::new(&self.client_name)?.ports();
        Ok(outs.into_iter().map(DeviceInfo::from).collect())
    }

    fn get_input(&'a self, identifier: Identifier) -> Result<InputPort, MidiInterfaceError> {
        let in_ports = self.input().ports();
        let input: md::MidiInputPort = match identifier {
            Identifier::String(name) => match self.find_input_port(&name) {
                Ok(x) => x,
                Err(e) => return Err(e),
            },
            Identifier::Number(id) => match in_ports.get(id as usize) {
                None => {
                    return Err(MidiInterfaceError::NotAnInputDevice(format!(
                        "Input Port with ID {id}"
                    )))
                }
                Some(x) => x.to_owned(),
            },
        };
        Ok(Self::MidiInput::new(
            &self.client_name,
            &input,
            "Launchpad Mini Input",
            BUFFER_SIZE,
        ))
    }

    fn get_output(&'a self, identifier: Identifier) -> Result<OutputPort, MidiInterfaceError> {
        let outs = self.output().ports();

        let output: md::MidiOutputPort = match identifier {
            Identifier::String(name) => match self.find_output_port(&name) {
                Ok(x) => x.to_owned(),
                Err(e) => return Err(e),
            },
            Identifier::Number(id) => match outs.get(id as usize) {
                Some(x) => x.to_owned(),
                None => {
                    return Err(MidiInterfaceError::NotAnOutputDevice(format!(
                        "Output Port with ID {id}"
                    )))
                }
            },
        };

        match self.output().connect(&output, "Launchpad Mini Output") {
            Ok(con) => {
                println!("Connected to {}", DeviceInfo::from(output));
                Ok(OutputPort::new(con))
            }
            Err(e) => Err(MidiInterfaceError::from(e)),
        }
    }

    fn get_in_out(
        &'a self,
        name: &str,
    ) -> Result<(Self::MidiInput, Self::MidiOutput), MidiInterfaceError> {
        let input = self.get_input(Identifier::from(name))?;
        let output = self.get_output(Identifier::from(name))?;
        Ok((input, output))
    }

    fn get_default_input(&'a self) -> Result<Self::MidiInput, MidiInterfaceError> {
        let port = self.input().ports();
        let default = match port.first() {
            None => {
                return Err(MidiInterfaceError::NoDefaultDevice(
                    "Input Device".to_string(),
                ))
            }
            Some(p) => p,
        };

        Ok(Self::MidiInput::new(
            &self.client_name,
            default,
            "Launchpad Mini Input",
            BUFFER_SIZE,
        ))
    }

    fn get_default_output(&'a self) -> Result<Self::MidiOutput, MidiInterfaceError> {
        let ports = self.output().ports();
        let default = match ports.first() {
            None => {
                return Err(MidiInterfaceError::NoDefaultDevice(
                    "Output Device".to_string(),
                ))
            }
            Some(p) => p,
        };

        match self.output().connect(default, "Launchpad Mini Output") {
            Ok(con) => Ok(OutputPort::new(con)),
            Err(err) => Err(MidiInterfaceError::from(err)),
        }
    }
}
/// Implementation of the Error type MidiInterfaceError
impl From<md::SendError> for MidiInterfaceError {
    fn from(value: md::SendError) -> Self {
        match value {
            md::SendError::InvalidData(msg) => MidiInterfaceError::Invalid(msg.to_string()),
            md::SendError::Other(msg) => MidiInterfaceError::GenericBackendErr(msg.to_string()),
        }
    }
}

impl From<md::InitError> for MidiInterfaceError {
    fn from(value: md::InitError) -> Self {
        MidiInterfaceError::GenericBackendErr(value.to_string())
    }
}

impl<T> From<md::ConnectError<T>> for MidiInterfaceError {
    fn from(value: md::ConnectError<T>) -> Self {
        MidiInterfaceError::GenericBackendErr(value.to_string())
    }
}

impl From<md::PortInfoError> for MidiInterfaceError {
    fn from(value: md::PortInfoError) -> Self {
        match value {
            md::PortInfoError::PortNumberOutOfRange => MidiInterfaceError::GenericBackendErr(
                "PortInfoError: Portnumber out of range".to_string(),
            ),
            md::PortInfoError::InvalidPort => {
                MidiInterfaceError::Invalid("PortInfoError: Invalid Port".to_string())
            }
            md::PortInfoError::CannotRetrievePortName => MidiInterfaceError::GenericBackendErr(
                "PortInfoError: Cannot Retrieve Port Name".to_string(),
            ),
        }
    }
}

impl From<md::MidiInputPort> for DeviceInfo {
    fn from(value: md::MidiInputPort) -> Self {
        let name = md::MidiInput::new("info")
            .unwrap()
            .port_name(&value)
            .unwrap();
        let mut name = name.split(' ');
        let id: String = name.next_back().unwrap_or("").to_string();
        let name = name.collect::<Vec<_>>().join(" ");
        let mut name = name.split(':');
        let client_name = name.next().unwrap_or("").to_string();
        let name: Vec<&str> = name.collect();
        let name = name.join("");
        DeviceInfo {
            id,
            client_name,
            name,
            dir: Direction::Input,
        }
    }
}

impl From<md::MidiOutputPort> for DeviceInfo {
    fn from(value: md::MidiOutputPort) -> Self {
        let name = md::MidiOutput::new("info")
            .unwrap()
            .port_name(&value)
            .unwrap();
        let mut name = name.split(' ');
        let id: String = name.next_back().unwrap_or("").to_string();
        let name = name.collect::<Vec<_>>().join(" ");
        let mut name = name.split(':');
        let client_name = name.next().unwrap_or("").to_string();
        let name: Vec<&str> = name.collect();
        let name = name.join("");
        DeviceInfo {
            id,
            client_name,
            name,
            dir: Direction::Output,
        }
    }
}

impl From<&md::MidiInputPort> for DeviceInfo {
    fn from(value: &md::MidiInputPort) -> Self {
        let name = md::MidiInput::new("info")
            .unwrap()
            .port_name(value)
            .unwrap();
        let mut name = name.split(' ');
        let id: String = name.next_back().unwrap_or("").to_string();
        let name = name.collect::<Vec<_>>().join(" ");
        let mut name = name.split(':');
        let client_name = name.next().unwrap_or("").to_string();
        let name: Vec<&str> = name.collect();
        let name = name.join("");
        DeviceInfo {
            id,
            client_name,
            name,
            dir: Direction::Input,
        }
    }
}
impl From<&md::MidiOutputPort> for DeviceInfo {
    fn from(value: &md::MidiOutputPort) -> Self {
        let name = md::MidiOutput::new("info")
            .unwrap()
            .port_name(value)
            .unwrap();
        let mut name = name.split(' ');
        let id: String = name.next_back().unwrap_or("").to_string();
        let name = name.collect::<Vec<_>>().join(" ");
        let mut name = name.split(':');
        let client_name = name.next().unwrap_or("").to_string();
        let name: Vec<&str> = name.collect();
        let name = name.join("");
        DeviceInfo {
            id,
            client_name,
            name,
            dir: Direction::Output,
        }
    }
}
