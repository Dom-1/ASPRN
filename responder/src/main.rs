use std::env;
use std::io::*;
use std::process::{Command, Stdio};

fn main() -> std::io::Result<()> {
    let input: Vec<String> = env::args().collect();
    dbg!(input);
    Ok(())
}

fn send_packet(node: String, packet: String) -> Result<()> {
    let mut child = Command::new(format!("dtnsend -r dtn://{node}"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn child process");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin
            .write_all(packet.as_bytes())
            .expect("Failed to write to stdin");
    });

    let output = child.wait_with_output().expect("Failed to read stdout");
    println!("This is the output of the child process: {output:?}");

    Ok(())
}
