use ctrlc;
use launchpad_mini_control::{Color, DeviceInfo, MatPos, MidiImpl, MidiInterface};
use std::process::exit;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Release};
use std::sync::Arc;
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
    let term = Arc::new(AtomicBool::new(false));
    let termsend: Arc<AtomicBool> = Arc::clone(&term);
    let _ = ctrlc::set_handler(move || {
        termsend.store(true, Release);
    });
    // init midi lib and some constants
    let midi: MidiImpl = MidiImpl::new("Launchpad Control");
    let mut lpad = launchpad_mini_control::new_launch_device_from_midi_interface(&midi);

    // DEVICE OVERVIEW
    print_devices(&midi);
    println!("Press Ctrl-C or Button (3,5) on Launchpad to stop execution\n");

    lpad.reset().unwrap();

    lpad.set_position(4_u8, 5_u8, Color::MedYellow).unwrap();
    loop {
        std::thread::sleep(Duration::from_millis(700));
        if let Some(buf) = lpad.read_single_msg().expect("valid read") {
            println!("{:?}", buf);
            let tappos = MatPos::from(buf);
            println!("{:?}", tappos);
            println!();

            if tappos.get_as_tuple() == (3_u8, 5_u8) {
                lpad.blackout().unwrap();
                lpad.set_all(Color::Green).unwrap();
                exit(0);
            }
        }
        if term.load(Acquire) {
            let _ = lpad.reset();
            exit(0);
        }
    }
}
