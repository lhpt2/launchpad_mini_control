# Launchpad_Mini_Control

## Overview

This is a low level library for communicating with the Launchpad Mini.
It provides functionality to set the LED lights, read button presses and make use of features like double buffering.

### Devices known to work

So far testing has only be done with the Launchpad Mini MK 1.

## Example

```toml
[dependencies]
launchpad_mini_control = "0.1.0"
```
Then, on your main.rs:

```rust,no_run
use launchpad_mini_control::{MidiImpl, MatPos};

fn main() {
    let midi: MidiImpl = MidiImpl::new().expect("initialization");
    let mut lpad = launchpad_mini_control::new_launch_device_from_midi_interface(&midi);
    
    // reset Launchpad
    lpad.reset().unwrap();
    
    // set row 3, column 5 to MedYellow
    lpad.set_position(3_u8, 5_u8, Color::MedYellow).unwrap();
    
    // wait for button presses and print to terminal 
    loop {
        std::thread::sleep(Duration::from_millis(700));
        if lpad.poll().is_ok() {
            if let Some(buf) = lpad.read_single_msg().expect("valid read") {
                println!("{:?}", buf);
                println!();
                let tappos = MatPos::from(buf);
                println!("{:?}", tappos);
                if tappos.get_as_tuple() == (3_u8, 5_u8) {
                    lpad.blackout().unwrap();
                    lpad.set_all(Color::Green).unwrap();
                    lpad.set_position(2, 5, Color::DimGreen).unwrap();
                }
            }
        }
    }
}
```

## Contributing

:balloon: Thanks for your help improving the project! Just fork the project and create a pull request
with an appropriate description, we will find a way to include the suggested changes into the project.

## License

This project is licensed under the [LGPL license].

[LGPL license]: https://www.gnu.org/licenses/lgpl-3.0.en.html#license-text

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Launchpad_Mini_Control by you, shall be licensed as LGPLv3+, without any additional
terms or conditions.
