#![allow(unused)]

use clap::Parser;
use std::process::{Command, Stdio};

#[derive(Parser)]
struct Cli {
    port: String,
}

fn main() {
    let args = Cli::parse();
    let port = format!(":{}", args.port);
    let output: std::process::Output = Command::new("lsof")
        .arg("-i")
        .arg(port)
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    let mut stdout = String::from_utf8(output.stdout).unwrap();

    let main_vec = stdout.lines()
                             .map(|s| s.trim().split(' ').map(String::from).collect::<Vec<String>>())
                             .collect::<Vec<Vec<String>>>();
    for arr in &main_vec{
        if &arr[&arr.len()-1] == "(LISTEN)"{
            Command::new("kill")
                .arg("-9")
                .arg(&arr[1])
                .spawn();
        }
    }

}

