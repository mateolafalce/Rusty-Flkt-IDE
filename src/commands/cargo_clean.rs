use crate::functions::{
    write_terminal,
    root
};
use fltk::text::{
    TextDisplay,
    TextBuffer
};
use std::{
    thread,
    process::Command
};

pub fn cargo_clean(
    input: String,
    text: TextBuffer,
    terminal: TextDisplay
){
    thread::spawn(move || {
        let output = Command::new("cargo")
            .args(&["update", "--manifest-path", &((root.clone())().unwrap() + "\\Cargo.toml")])
            .output()
            .expect("Error");
        let result: String = format!("{}", String::from_utf8_lossy(&output.stdout));
            write_terminal(
                &(input + "\n" + &result),
                text.clone(),
                terminal.clone()
            ).expect("Error");
        }
    );
}
