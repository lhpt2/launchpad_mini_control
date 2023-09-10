use cartesian::*;
use crate::midilib::{LaunchMessage, MidiInterfaceError};
use crate::midilib::Output;
use crate::midilib::Input;

const SCENE_LAUNCH_COL: usize = 8;
const AUTOMAP_ROW: usize = 8;
const NOTEGRID_MAX_LEN: usize = 8;
const MAX_PAD_COLSROWS: usize = 9;
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

#[derive(PartialEq)]
enum MessageType {
    Off = 0x80,
    On = 0x90,
    Ctl = 0xb0,
}

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

pub enum BufferSetting {
    ZeroOnly = 0x00,
    OneActive = 0x01,
    ZeroActive = 0x04,
    OneOnly = 0x05,
}

pub enum GridMode {
    XY = 0x01,
    DrumRack = 0x02,
}

type Key = u8;

impl From<MatPos> for Key {
    fn from(pos: MatPos) -> Self {
        if pos.row > 7 {
            121 as Key
        } else {
            PadIdentifier::from(pos).key as Key
        }
    }
}

struct PadIdentifier {
    status: MessageType,
    key: u8,
}

impl From<MatPos> for PadIdentifier {
    fn from(pos: MatPos) -> Self {
        if pos.row > 7 {
            PadIdentifier {
                status: MessageType::Ctl,
                key: 0x68 + pos.col,
            }
        } else {
            PadIdentifier {
                status: MessageType::On,
                key: (0x10 * pos.row) + pos.col,
            }
        }
    }
}

impl From<LaunchMessage> for PadIdentifier {
    fn from(msg: LaunchMessage) -> Self {
        if msg.status == MessageType::Ctl as u8 {
            PadIdentifier {
                status: MessageType::Ctl,
                key: msg.data1,
            }
        } else if msg.status == MessageType::On as u8 {
            PadIdentifier {
                status: MessageType::On,
                key: msg.data1,
            }
        } else {
            PadIdentifier {
                status: MessageType::Off,
                key: msg.data1,
            }
        }
    }
}

pub struct MatPos {
    pub row: u8,
    pub col: u8,
}

impl MatPos {
    fn new(row: u8, col: u8) -> MatPos {
        MatPos { row, col }
    }
    fn get_as_tuple(self) -> (u8, u8) {
        (self.row, self.col)
    }
}

impl From<LaunchMessage> for MatPos {
    fn from(msg: LaunchMessage) -> Self {
        MatPos::from(PadIdentifier::from(msg))
    }
}

impl From<PadIdentifier> for MatPos {
    fn from(padid: PadIdentifier) -> Self {
        if padid.status == MessageType::Ctl {
            MatPos {
                row: 8,
                col: padid.key % 0x68,
            }
        } else {
            MatPos {
                row: padid.key / 0x10,
                col: padid.key % 0x10,
            }
        }
    }
}

pub struct LaunchDevice<'a, I: Input, O: Output> {
    in_port: &'a I,
    out_port: &'a mut O,
    buffer_setting: u8,
}

impl<'a, I, O> LaunchDevice<'a, I, O>
    where I: Input + 'a, O: Output + 'a {
    pub fn new(
        in_port: &'a I,
        out_port: &'a mut O,
        //in_port: pm::InputPort<'a>,
        //out_port: pm::OutputPort<'a>,
    ) -> LaunchDevice<'a, I, O> {
        LaunchDevice {
            in_port,
            out_port,
            buffer_setting: 0,
        }
    }

    pub fn send_note_msg(&mut self, on: bool, key: u8, vel: u8) -> Result<(), MidiInterfaceError>{
        let mut vel = vel;

        if !self.is_double_buffered() {
            vel |= 0x0C;
        }

        let mut mtype = MessageType::Off as u8;
        if on {
            mtype = MessageType::On as u8;
        }

        self.out_port.write_message(LaunchMessage {
            status: mtype,
            data1: key,
            data2: vel,
        })?;

        Ok(())
    }

    pub fn send_messages(&mut self, msgs: Vec<LaunchMessage>) {
       let _ = self.out_port.write_messages(msgs);
    }

    pub fn send_ctl_msg(&mut self, data1: u8, data2: u8) {
        let _ = self.out_port.write_message(LaunchMessage {
            status: 0xb0,
            data1,
            data2,
        });
    }

    pub fn blackout(&mut self) {
        self.set_all(Color::Black);
    }

    pub fn full_blackout(&mut self) {
        self.set_all(Color::Black);
        for i in 0..8 {
            self.send_ctl_msg(0x68 + i, Color::Black as u8);
        }
    }

    pub fn set_position(&mut self, row: u8, col: u8, color: Color) {
        let _ = self.out_port.write_message(LaunchMessage {
            status: 0x90,
            data1: Key::from(MatPos::new(row, col)),
            data2: color as u8,
        });
    }

    pub fn set_all(&mut self, color: Color) {
        let mut msg: Vec<LaunchMessage> = Vec::with_capacity(MAX_PAD_COLSROWS * MAX_PAD_COLSROWS);
        for (x, y) in cartesian!(0..8, 0..9) {
            //self.send_note_msg(true, Key::from(MatPos::new(x, y)), color.into());
            msg.push(LaunchMessage {
                status: MessageType::On as u8,
                data1: Key::from(MatPos::new(x, y)),
                data2: color as u8,
            });
        }

        let _ = self.out_port.write_messages(msg);
    }

    pub fn select_mode(&mut self, mode: GridMode) {
        self.send_ctl_msg(0x00, mode as u8);
    }

    pub fn is_double_buffered(&self) -> bool {
        let buffered = 0x0F & self.buffer_setting;
        buffered == BufferSetting::OneActive as u8 || buffered == BufferSetting::ZeroActive as u8
    }

    pub fn set_matrix(&mut self, mat: &[[Color; 9]; 8]) {
        let mut res: Vec<LaunchMessage> = Vec::with_capacity(mat.len());

        for (i, parent) in mat.iter().enumerate() {
            for (j, elem) in parent.iter().enumerate() {
                res.push(LaunchMessage {
                    status: 0x90,
                    data1: Key::from(MatPos::new(i as u8, j as u8)),
                    data2: *elem as u8,
                });
            }
        }

        let _ = self.out_port.write_messages(res);
    }

    pub fn set_first_row(&mut self, color: Color) {
        let mut msg: Vec<LaunchMessage> = Vec::with_capacity(8);
        for i in 0..8 {
            msg.push(LaunchMessage {
                status: 0xb0,
                data1: 0x68 + i,
                data2: color as u8,
            });
        }

        let _ = self.out_port.write_messages(msg);
    }

    pub fn reset(&mut self) {
        self.send_ctl_msg(0x00, 0x00);
    }

    pub fn set_buffer_mode(&mut self, setting: BufferSetting, copy: bool) {
        if copy {
            self.buffer_setting = 0x30;
        } else {
            self.buffer_setting = 0x20;
        }

        self.buffer_setting |= setting as u8;
        self.send_ctl_msg(0x00, self.buffer_setting);
    }

    pub fn disable_double_buffering(&mut self) {
        self.set_buffer_mode(BufferSetting::ZeroOnly, false);
    }

    pub fn swap_buffers(&mut self, copy: bool) {
        let setting = self.buffer_setting & 0x0F;

        if setting == BufferSetting::OneActive as u8 {
            self.set_buffer_mode(BufferSetting::ZeroActive, copy);
        } else {
            self.set_buffer_mode(BufferSetting::OneActive, copy);
        }
    }

    pub fn hard_swap(&mut self) {
        self.swap_buffers(false);
    }

    pub fn set_duty_cycle(&mut self, numerator: u8, denominator: u8) -> Result<(), MidiInterfaceError>{
        let mut numerator = numerator;
        let mut denominator = denominator;

        if numerator > 16 {
            numerator = 16;
        } else if numerator < 1 {
            numerator = 1;
        }

        if denominator > 18 {
            denominator = 18;
        } else if denominator < 3 {
            denominator = 3;
        }

        let mut data1: u8 = 0x1f;
        let mut data2: u8 = 0x10 * (numerator - 9) + (denominator - 3);

        if numerator < 9 {
            data1 = 0x1e;
            data2 = 0x10 * (numerator - 1) + (denominator - 3);
        }

        self.out_port.write_message(LaunchMessage {
            status: 0xb0,
            data1,
            data2,
        })?;

        Ok(())
    }
}
