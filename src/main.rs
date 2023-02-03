// Copyright 2025 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

mod cmd;
mod config;
use clap::Command;
use cmd::version::get_version;

fn main() {
    let matches = Command::new("Pentheus")
        .about("Your Database Guardian, Set up in Minutes.")
        .version(get_version())
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Clivern")
        .subcommand(
            Command::new("version").about("Displays the current version pentheus."),
        )
        .get_matches();

    match matches.subcommand() {
        // version command
        Some(("version", _)) => {
            println!("Pentheus {}", get_version());
        }
        _ => unreachable!(),
    }
}
