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
pub use pm_impl::*;

pub use utils::Color;
pub use utils::MatPos;

pub const BUFFER_SIZE: usize = 1024;
