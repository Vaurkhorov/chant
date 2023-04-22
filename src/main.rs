use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 3 {
        println!("More than 2 arguments provided, considering only the first 2.");
    }

    if args[1] == "send".to_string() {
        return send(&args[2]);
    }
    else if args[1] == "receive".to_string() {
        return receive();
    }
    else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid argument, needs to be either 'send' or 'receive'.".to_string(),
        ));
    }

}

// fn handle()

fn send(message: &str) -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7000")?;

    println!("sending message");
    stream.write_all(message.as_bytes())?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    println!("server responded: {}", response);

    Ok(())
}

fn receive() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7000")?;

    println!("listening");
    let (mut stream, _) = listener.accept()?;

    let mut buffer = [0; 512];

    stream.read(&mut buffer)?;

    let message = String::from_utf8_lossy(&buffer[..]);
    println!("Received message {}", message);

    stream.write_all(b"message received")?;

    Ok(())
}
