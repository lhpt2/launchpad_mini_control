use crate::mat_pos::MatPos;
use crate::LaunchMessage;
use crate::help_types::MessageType;

pub struct PadIdentifier {
    pub(crate) status: MessageType,
    pub(crate) key: u8,
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
