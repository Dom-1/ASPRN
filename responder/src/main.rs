use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::*;
use std::process::{Command, Stdio};
use std::str;

fn main() {
    let input: Vec<String> = env::args().collect();
    let from_node = &input[1];
    let message = fs::read(&input[2]).unwrap();
    let mut messages_db: Vec<String> = Vec::new();
    // if message doesn't already exist in db
    if !messages_db
        .iter()
        .any(|msg| msg == str::from_utf8(&message).unwrap())
    {
        // load message into db
        messages_db.push(String::from_utf8(message.clone()).unwrap());

        // process message data and respond correctly
        // first 5 bytes should be proto name
        let (head, tail) = message.split_at(5);
        let header = parse_message_header(head.to_vec());
        match header {
            ProtoName::Example => {
                let parsed: ExampleProtocol = parse_message(header, tail).unwrap();
                if !String::from_utf8(parsed.data).unwrap().contains("got it") {
                    send_response(String::from(from_node), String::from("got it")).unwrap();
                }
            }
            ProtoName::Uninmplemented => {
                println!("Packet arrived with unimplemented protocol!");
            }
        }
    }
}

// every entry in this enum should have a corresponding Protocol impl
enum ProtoName {
    Example,
    Uninmplemented,
}

trait Protocol {
    fn get_data(&self) -> Vec<u8>;
}

#[derive(Serialize, Deserialize, Debug)]
struct ExampleProtocol {
    data: Vec<u8>,
}

impl Protocol for ExampleProtocol {
    fn get_data(&self) -> Vec<u8> {
        self.data.clone()
    }
}

fn parse_message_header(message: Vec<u8>) -> ProtoName {
    match str::from_utf8(&message[..=4]).unwrap() {
        "EXMPL" => ProtoName::Example,
        _ => ProtoName::Uninmplemented,
    }
}

fn parse_message<T>(header: ProtoName, body: &[u8]) -> std::result::Result<T, ()>
where
    T: Protocol + for<'a> serde::Deserialize<'a>,
{
    match header {
        ProtoName::Example => ron::from_str(&String::from_utf8(body.to_vec()).unwrap()).unwrap(),
        _ => Err(()),
    }
}

fn send_response(node: String, packet: String) -> Result<()> {
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
