extern crate clap;

use clap::{App, Arg};

use std::io;
use std::process::Command;

fn main() {
    let matches = App::new("stamp")
        .version("0.1.0")
        .about("Prepend the output of a command run for each line of stdin")
        .author("Richard Lupton 2017")
        .arg(
            Arg::with_name("command")
                .short("c")
                .takes_value(true)
                .required(true)
                .number_of_values(1),
        )
        .get_matches();

    let command: &str = matches.value_of("command").unwrap();

    let mut buffer = String::new();
    let stdin: io::Stdin = io::stdin();

    loop {
        buffer.clear();
        if stdin.read_line(&mut buffer).expect(
            "Failed to read from stdin",
        ) == 0
        {
            break;
        }

        let output = Command::new("sh").arg("-c").arg(command).output().expect(
            "Failed to launch subprocess",
        );

        // TODO: use eprintln to print error output of subprocess
        print!(
            "{} {}",
            String::from_utf8_lossy(&output.stdout).trim(),
            buffer
        );
    }
}
