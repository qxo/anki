// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use std::process::Command;

use camino::Utf8Path;
use clap::Args;

use crate::paths::absolute_msys_path;
use crate::run::run_command;

#[derive(Args)]
pub struct RsyncArgs {
    #[arg(long, value_delimiter(','), allow_hyphen_values(true))]
    extra_args: Vec<String>,
    #[arg(long)]
    prefix: String,
    #[arg(long, required(true), num_args(..))]
    inputs: Vec<String>,
    #[arg(long)]
    output_dir: String,
}

pub fn rsync_files(args: RsyncArgs) {
    let output_dir = absolute_msys_path(Utf8Path::new(&args.output_dir));
    run_command(
        Command::new("echo").arg(output_dir.clone()),
    );
    //let mut cmd1: String = String::from("mkdir -p ");
    //cmd1.push_str(output_dir.clone().as_str());
    run_command(
        Command::new("mkdir")
        .arg("-p")
        .arg(output_dir.clone()),
    );
    run_command(
        Command::new("sh").arg("."),
    );
    run_command(
        Command::new("rsync")
            .current_dir(&args.prefix)
            .arg("--relative")
            .args(args.extra_args)
            .args(args.inputs.iter().map(|i| {
                if cfg!(windows) {
                    i.replace('\\', "/")
                } else {
                    i.clone()
                }
            }))
            .arg(output_dir),
    );
}
