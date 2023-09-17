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

mod utils;
mod launch_device;
mod midilib;
mod pm_impl;

pub use launch_device::*;
pub use midilib::*;
pub use pm_impl::{MidiImpl, InputPort, OutputPort};
pub use pm_impl::*;

pub use utils::Color;
pub use utils::MatPos;

pub const BUFFER_SIZE: usize = 1024;
pub fn new_launch_device_from_midi_interface<'a>(ctx: &'a impl MidiInterface<'a, MidiInput = InputPort<'a>, MidiOutput = OutputPort<'a>>) -> LaunchDevice<InputPort, OutputPort> {
    let (in_p, out_p) = match ctx.get_in_out("Launchpad Mini MIDI 1") {
        Ok(res) => (res.0, res.1),
        Err(e) => match e {
            MidiInterfaceError::NotAnOutputDevice(_) | MidiInterfaceError::NotAnInputDevice(_) => {
                println!("Using default device");
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

    LaunchDevice::new(in_p, out_p)
}
