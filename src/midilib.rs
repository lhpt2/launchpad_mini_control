use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub trait MidiInterface<'a> {
	type MidiInput: Input + 'a;
	type MidiOutput: Output + 'a;
	fn get_devices(&self) -> Result<Vec<DeviceInfo>, MidiInterfaceError>;
	fn get_input(&'a self, identifier: Identifier) -> Result<Self::MidiInput, MidiInterfaceError>;
	fn get_output(&'a self, identifier: Identifier) -> Result<Self::MidiOutput, MidiInterfaceError>;
	fn get_in_out(&'a self, name: &str) -> Result<(Self::MidiInput, Self::MidiOutput), MidiInterfaceError>;
	fn get_default_input(&'a self) -> Result<Self::MidiInput, MidiInterfaceError>;
	fn get_default_output(&'a self) -> Result<Self::MidiOutput, MidiInterfaceError>;
}

pub trait Output {
	fn write_message(&mut self, msg: LaunchMessage) -> Result<(), MidiInterfaceError>;
	fn write_messages(&mut self, msg: Vec<LaunchMessage>) -> Result<(), MidiInterfaceError>;
}

pub trait Input {
	fn poll(&self) -> Result<bool, MidiInterfaceError>;
	fn read_n(&self, count: usize) -> Result<Option<Vec<LaunchMessage>>, MidiInterfaceError>;
}

/// Direction being either input or output holding device type
#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
	Input,
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

#[derive(Debug, Clone)]
pub struct LaunchMessage {
	pub status: u8,
	pub data1: u8,
	pub data2: u8,
}

/// OR type of a device identifier, either being a name (string) or a id (number)
pub enum Identifier {
	String(String),
	Number(i32),
}

#[derive(Debug)]
pub enum MidiInterfaceError {
	Unknown(String),
	Unimplemented(String),
	NoDefaultDevice(String),
	NotAnInputDevice(String),
	NotAnOutputDevice(String),
	Invalid(String),
	GenericBackendErr(String)
}

impl Error for MidiInterfaceError { }

impl Display for MidiInterfaceError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let message = match self {
			MidiInterfaceError::Unknown(msg) => msg.to_string(),
			MidiInterfaceError::Unimplemented(msg) =>  msg.to_string(),
			MidiInterfaceError::NoDefaultDevice(msg) => msg.to_string(),
			MidiInterfaceError::NotAnInputDevice(msg) => msg.to_string(),
			MidiInterfaceError::NotAnOutputDevice(msg) => msg.to_string(),
			MidiInterfaceError::Invalid(msg) => msg.to_string(),
			MidiInterfaceError::GenericBackendErr(msg) => msg.to_string(),
		};
		write!(f, "E midi backend: {}", message)
	}
}
