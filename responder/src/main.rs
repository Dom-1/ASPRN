use std::io;

fn main() -> io::Result<()> {
    let input = io::read_to_string(io::stdin())?;
    println!("{input}");
    io::Result::Ok(())
}
