/* Copyright (C) 2023 Lucas Haupt

This program is distributed under the terms of the 
GNU Lesser General Public License v3.0, 
see COPYING.LESSER file for license information
*/

use crate::help_types::MessageType;
use crate::pad_identifier::PadIdentifier;
use crate::LaunchMessage;

#[derive(Debug)]
pub struct MatPos {
    pub row: u8,
    pub col: u8,
}
impl MatPos {
    pub fn new(row: u8, col: u8) -> MatPos {
        MatPos { row, col }
    }
    pub fn get_as_tuple(self) -> (u8, u8) {
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
