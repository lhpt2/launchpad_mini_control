use crate::utils::{conversions, LaunchMessage};
use crate::{MatPos, MessageType};

/// Struct for a typical MidiMessage
#[derive(PartialEq, Debug)]
pub struct MidiMessage {
    // status byte (either control message 0xb0, or note on/off msg 0x80/0x90)
    pub(crate) status: u8,
    // byte containing pitch information (here grid position info)
    pub(crate) pitch: u8,
    // byte containing velocity information (here color info)
    pub(crate) velocity: u8,
}
impl MidiMessage {
    pub fn into_u8(self) -> [u8; 3] {
        [self.status, self.pitch, self.velocity]
    }

    pub fn get_position(&self) -> MatPos {
        MatPos::from(self)
    }

    pub fn get_launchmsg(&self) -> LaunchMessage {
        LaunchMessage::from(self)
    }
}

impl From<LaunchMessage> for MidiMessage {
    fn from(value: LaunchMessage) -> Self {
        if value.status == MessageType::Ctl {
            MidiMessage {
                status: value.status as u8,
                pitch: 0x68 + value.col,
                velocity: value.color as u8,
            }
        } else {
            MidiMessage {
                status: value.status as u8,
                pitch: (0x10 * value.row) + value.col,
                velocity: value.color as u8,
            }
        }
    }
}

impl From<&MidiEvent> for MidiMessage {
    fn from(event: &MidiEvent) -> Self {
        MidiMessage {
            status: event.status,
            pitch: event.pitch,
            velocity: event.velocity,
        }
    }
}

impl From<&[u8]> for MidiMessage {
    fn from(value: &[u8]) -> Self {
        MidiMessage {
            status: value[0],
            pitch: value[1],
            velocity: value[2],
        }
    }
}

impl From<[u8; 3]> for MidiMessage {
    fn from(value: [u8; 3]) -> Self {
        MidiMessage {
            status: value[0],
            pitch: value[1],
            velocity: value[2],
        }
    }
}

/// Struct for a typical MidiEvent
#[derive(PartialEq, Debug, Clone)]
pub struct MidiEvent {
    // time information as u64 timestamp
    pub(crate) timestamp: u64,
    // status byte (NoteOn, NoteOff, CC, SysEx)
    pub(crate) status: u8,
    // pitch byte indicating pitch or similar
    pub(crate) pitch: u8,
    // velocity byte indicating intensity or similar
    pub(crate) velocity: u8,
}
impl MidiEvent {
    pub fn new(timestamp: u64, status: u8, pitch: u8, velocity: u8) -> MidiEvent {
        MidiEvent {
            timestamp,
            status,
            pitch,
            velocity,
        }
    }

    fn get_midi_message(&self) -> MidiMessage {
        MidiMessage::from(self)
    }

    fn get_launch_message(&self) -> LaunchMessage {
        LaunchMessage::from(self)
    }
}

impl From<&[u8]> for MidiEvent {
    fn from(data: &[u8]) -> Self {
        let mut res: [u8; 3] = [0, 0, 0];
        res[..data.len()].copy_from_slice(data);

        MidiEvent {
            timestamp: 0,
            status: res[0],
            pitch: res[1],
            velocity: res[2],
        }
    }
}

impl From<MidiMessage> for MidiEvent {
    fn from(msg: MidiMessage) -> Self {
        MidiEvent::from(&msg)
    }
}

impl From<&MidiMessage> for MidiEvent {
    fn from(msg: &MidiMessage) -> Self {
        MidiEvent {
            timestamp: 0,
            status: msg.status,
            pitch: msg.pitch,
            velocity: msg.velocity,
        }
    }
}

impl From<&LaunchMessage> for MidiEvent {
    fn from(lmsg: &LaunchMessage) -> Self {
        MidiEvent {
           timestamp: 0,
           status: lmsg.status.clone() as u8,
           pitch: conversions::to_pitch_byte(lmsg.col, lmsg.row),
           velocity: lmsg.color.into(),
        }
    }
}
