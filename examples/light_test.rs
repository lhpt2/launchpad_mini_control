use launchpad_mini_control::{Color, DeviceInfo, MatPos, MidiImpl, MidiInterface};
use std::process::exit;
use std::time::Duration;

fn print_devices<'a>(ctx: &impl MidiInterface<'a>) {
    let devs = ctx.get_devices().expect("device list might be empty");
    let inputs: Vec<&DeviceInfo> = devs.iter().filter(|d| d.is_input()).collect();
    let outputs: Vec<&DeviceInfo> = devs.iter().filter(|d| d.is_output()).collect();
    println!("Available input devices:");
    for i in inputs {
        println!("{:?}", *i);
    }

    println!();

    println!("Available output devices:");
    for o in outputs {
        println!("{:?}", *o);
    }
}

fn main() {
    // init midi lib and some constants
    let midi: MidiImpl = MidiImpl::new("Launchpad Control");
    let mut lpad = launchpad_mini_control::new_launch_device_from_midi_interface(&midi);

    // DEVICE OVERVIEW
    print_devices(&midi);

    lpad.reset().unwrap();

    for row in 0..8 {
        for col in 0..9 {
            lpad.set_position(row, col, Color::Red).unwrap();
            std::thread::sleep(Duration::from_millis(700));
        }
    }
    lpad.reset().unwrap();
}
