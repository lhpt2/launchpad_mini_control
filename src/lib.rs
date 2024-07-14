/* Copyright (C) 2023 Lucas Haupt

This program is distributed under the terms of the
GNU Lesser General Public License v3.0,
see COPYING.LESSER file for license information
*/

//! # Launchpad_Mini_Control
//!
//! This library contains various functions and a struct to handle
//! communication with a Launchpad Mini device over its Midi interface.
//!
//!

mod launch_device;
mod midilib;
mod utils;
pub use utils::MessageType;

pub use launch_device::*;
pub use midilib::*;

#[cfg(all(feature = "midir", not(feature = "pm")))]
mod midirs_impl;
#[cfg(all(feature = "midir", not(feature = "pm")))]
pub use midirs_impl::*;
#[cfg(all(feature = "midir", not(feature = "pm")))]
pub type LaunchDevice = LaunchDeviceTemplate<midirs_impl::InputPort, midirs_impl::OutputPort>;

/// The types and implementations in this module do have to be implemented
#[cfg(all(feature = "pm", not(feature = "midir")))]
mod pm_impl;
#[cfg(all(feature = "pm", not(feature = "midir")))]
pub use pm_impl::*;
#[cfg(all(feature = "pm", not(feature = "midir")))]
pub use pm_impl::{InputPort, MidiImpl, OutputPort};
#[cfg(all(feature = "pm", not(feature = "midir")))]
pub type LaunchDevice = LaunchDeviceTemplate<pm_impl::InputPort, pm_impl::OutputPort>;

pub use utils::Color;
pub use utils::MatPos;

pub const BUFFER_SIZE: usize = 1024;

/// construct a new LaunchDevice from a midi backend context
pub fn new_launch_device_from_midi_interface<'a>(
    #[cfg(all(feature = "pm", not(feature = "midir")))] ctx: &'a impl MidiInterface<
        'a,
        MidiInput = InputPort<'a>,
        MidiOutput = OutputPort<'a>,
    >,
    #[cfg(all(feature = "midir", not(feature = "pm")))] ctx: &'a impl MidiInterface<
        'a,
        MidiInput = InputPort,
        MidiOutput = OutputPort,
    >,
) -> LaunchDevice {
    let (in_p, out_p) = match ctx.get_in_out("Launchpad Mini MIDI 1") {
        Ok(res) => (res.0, res.1),
        Err(e) => match e {
            MidiInterfaceError::NotAnOutputDevice(_) | MidiInterfaceError::NotAnInputDevice(_) => {
                eprintln!("{}: Using default device", e);
                (
                    ctx.get_default_input().expect("default in"),
                    ctx.get_default_output().expect("default out"),
                )
            }
            _ => {
                panic!("{}", e);
            }
        },
    };

    LaunchDeviceTemplate::new(in_p, out_p)
}

pub fn new_launch_device_with_midi_init() -> (MidiImpl, LaunchDevice) {
    let midi = MidiImpl::new("LaunchControl");
    let lpad = new_launch_device_from_midi_interface(&midi);
    (midi, lpad)
}
