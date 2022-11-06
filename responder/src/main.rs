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
                if !String::from_utf8(parsed.data.clone())
                    .unwrap()
                    .contains("got it")
                {
                    println!(
                        "\"{}\" from {}",
                        String::from_utf8(parsed.data.clone()).unwrap(),
                        from_node
                    );
                    send_response(
                        String::from(from_node),
                        ("ExampleProtocol", "EXMPL"),
                        String::from("got it"),
                    )
                    .unwrap();
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
    T: Protocol + for<'a> serde::Deserialize<'a> + std::fmt::Debug,
{
    match header {
        ProtoName::Example => {
            let res = ron::from_str(&String::from_utf8(body.to_vec()).unwrap()).unwrap();
            println!("res: {:?}", res);
            res
        }
        _ => Err(()),
    }
}

fn send_response(
    node: String,
    (packet_type, packet_header): (&str, &str),
    packet: String,
) -> Result<()> {
    let mut packet_data = String::new();
    packet_data.push_str(packet_header);
    packet_data.push_str("Ok(");
    packet_data.push_str(packet_type);
    packet_data.push('(');
    packet_data.push_str(&chr_array_from_string(packet));
    packet_data.push_str("))");
    println!("Generated packet: {}", packet_data);
    /*
    let echo = Command::new("echo")
        .arg("-n")
        .arg(&packet_data)
        .stdout(Stdio::piped())
        .spawn()?
        .wait_with_output()
        .expect("failed to spawn parent process");
    */

    let mut child = Command::new("dtnsend")
        .arg("-r")
        .arg(format!("dtn://{}/incoming", node))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn child process");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin
            .write_all(&packet_data.as_bytes())
            .expect("Failed to write to stdin");
    });

    let output = child.wait_with_output().expect("Failed to read stdout");
    println!("{output:?}");

    Ok(())
}

fn chr_array_from_string(str: String) -> String {
    let mut res = String::from("data:[");
    for chr in str.chars() {
        let nchr: u32 = chr.into();
        res.push_str(&nchr.to_string());
        res.push(',');
    }
    res.push(']');
    res
}
