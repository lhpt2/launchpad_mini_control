use crate::mat_pos::MatPos;
use crate::midilib::{LaunchMessage, MidiInterfaceError, Output, Input};
use crate::help_types::{Color, MessageType, Key, BufferSetting, GridMode};
use cartesian::*;

const SCENE_LAUNCH_COL: usize = 8;
const AUTOMAP_ROW: usize = 8;
const NOTEGRID_MAX_LEN: usize = 8;
const MAX_PAD_COLSROWS: usize = 9;

pub struct LaunchDevice<'a, I: Input, O: Output> {
    in_port: &'a I,
    out_port: &'a mut O,
    buffer_setting: u8,
}
impl<'a, I, O> LaunchDevice<'a, I, O>
where
    I: Input + 'a,
    O: Output + 'a,
{
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

    pub fn poll(&self) -> Result<bool, MidiInterfaceError> {
        self.in_port.poll()
    }

    pub fn read_single_msg(&self) -> Result<Option<LaunchMessage>, MidiInterfaceError> {
        let opt = self.in_port.read_n(1)?;
        match opt {
            None => Ok(None),
            Some(msg) => match msg.first() {
                None => Ok(None),
                Some(m) => {
                    let res: LaunchMessage = (*m).clone();
                    Ok(Some(res))
                }
            },
        }
    }

    pub fn read_n_msgs(&self, n: usize) -> Result<Option<Vec<LaunchMessage>>, MidiInterfaceError> {
        self.in_port.read_n(n)
    }

    pub fn send_note_msg(&mut self, on: bool, key: u8, vel: u8) -> Result<(), MidiInterfaceError> {
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

    pub fn send_messages(&mut self, msgs: Vec<LaunchMessage>) -> Result<(), MidiInterfaceError> {
        self.out_port.write_messages(msgs)?;
        Ok(())
    }

    pub fn send_ctl_msg(&mut self, data1: u8, data2: u8) -> Result<(), MidiInterfaceError> {
        self.out_port.write_message(LaunchMessage {
            status: 0xb0,
            data1,
            data2,
        })?;
        Ok(())
    }

    pub fn blackout(&mut self) -> Result<(), MidiInterfaceError> {
        self.set_all(Color::Black)?;
        Ok(())
    }

    pub fn full_blackout(&mut self) -> Result<(), MidiInterfaceError> {
        self.set_all(Color::Black)?;
        for i in 0..8 {
            self.send_ctl_msg(0x68 + i, Color::Black as u8)?;
        }
        Ok(())
    }

    pub fn set_position(
        &mut self,
        row: u8,
        col: u8,
        color: Color,
    ) -> Result<(), MidiInterfaceError> {
        self.out_port.write_message(LaunchMessage {
            status: 0x90,
            data1: Key::from(MatPos::new(row, col)),
            data2: color as u8,
        })?;
        Ok(())
    }

    pub fn set_all(&mut self, color: Color) -> Result<(), MidiInterfaceError> {
        let mut msg: Vec<LaunchMessage> = Vec::with_capacity(MAX_PAD_COLSROWS * MAX_PAD_COLSROWS);
        for (x, y) in cartesian!(0..8, 0..9) {
            //self.send_note_msg(true, Key::from(MatPos::new(x, y)), color.into());
            msg.push(LaunchMessage {
                status: MessageType::On as u8,
                data1: Key::from(MatPos::new(x, y)),
                data2: color as u8,
            });
        }

        self.out_port.write_messages(msg)?;
        Ok(())
    }

    pub fn select_mode(&mut self, mode: GridMode) -> Result<(), MidiInterfaceError> {
        self.send_ctl_msg(0x00, mode as u8)?;
        Ok(())
    }

    pub fn is_double_buffered(&self) -> bool {
        let buffered = 0x0F & self.buffer_setting;
        buffered == BufferSetting::OneActive as u8 || buffered == BufferSetting::ZeroActive as u8
    }

    pub fn set_matrix(&mut self, mat: &[[Color; 9]; 8]) -> Result<(), MidiInterfaceError> {
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

        self.out_port.write_messages(res)?;
        Ok(())
    }

    pub fn set_first_row(&mut self, color: Color) -> Result<(), MidiInterfaceError> {
        let mut msg: Vec<LaunchMessage> = Vec::with_capacity(8);
        for i in 0..8 {
            msg.push(LaunchMessage {
                status: 0xb0,
                data1: 0x68 + i,
                data2: color as u8,
            });
        }

        self.out_port.write_messages(msg)?;
        Ok(())
    }

    pub fn reset(&mut self) -> Result<(), MidiInterfaceError> {
        self.send_ctl_msg(0x00, 0x00)?;
        Ok(())
    }

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

    pub fn disable_double_buffering(&mut self) -> Result<(), MidiInterfaceError> {
        self.set_buffer_mode(BufferSetting::ZeroOnly, false)?;
        Ok(())
    }

    pub fn swap_buffers(&mut self, copy: bool) -> Result<(), MidiInterfaceError> {
        let setting = self.buffer_setting & 0x0F;

        if setting == BufferSetting::OneActive as u8 {
            self.set_buffer_mode(BufferSetting::ZeroActive, copy)?;
        } else {
            self.set_buffer_mode(BufferSetting::OneActive, copy)?;
        }

        Ok(())
    }

    pub fn hard_swap(&mut self) -> Result<(), MidiInterfaceError> {
        self.swap_buffers(false)?;
        Ok(())
    }

    pub fn set_duty_cycle(
        &mut self,
        numerator: u8,
        denominator: u8,
    ) -> Result<(), MidiInterfaceError> {
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