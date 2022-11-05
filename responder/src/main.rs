use std::env;
use std::fs;
use std::io::*;
use std::process::{Command, Stdio};

fn main() -> std::io::Result<()> {
    let input: Vec<String> = env::args().collect();
    let node = &input[1];
    let message = fs::read(&input[2]);
    let messages_db: Vec<String> = Vec::new();
    /*
    if message doesn't already exist in db
        load message into db
        if message not meant for this node, then send to peers
        else process message data and respond correctly
    else disregard message
    */
    Ok(())
}

fn get_peers() -> Vec<String> {
    let mut child = Command::new("dtnquery peers");
    Vec::new()
}

fn send_packet(node: String, packet: String) -> Result<()> {
    let mut child = Command::new(format!("dtnsend -r dtn://{node}/incoming"))
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
