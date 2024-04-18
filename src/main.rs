use std::io::Read;
use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: {} <host> <port> <mode>", args[0]);
        eprintln!("Modes: -c (connect), -l (listen)");
        return Ok(());
    }

    let host = &args[1];
    let port = &args[2].parse::<u16>().expect("Invalid port number");
    let mode = &args[3];

    match mode.as_str() {
        "-c" => client(&host, *port)?,
        "-l" => server(*port)?,
        _ => eprintln!("Invalid mode: {}", mode),
    }

    Ok(())
}

fn client(host: &str, port: u16) -> io::Result<()> {
    let mut client_socket = TcpStream::connect((host, port))?;
    println!("Connected to {} on port {}", host, port);

    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin);
    let mut input = String::new();
    loop {
        input.clear();
        reader.read_line(&mut input)?;
        client_socket.write_all(input.as_bytes())?;

        let mut response = String::new();
        client_socket.read_to_string(&mut response)?;
        print!("Received: {}", response);
    }
}

fn server(port: u16) -> io::Result<()> {
    let listener = TcpListener::bind(("0.0.0.0", port))?;
    println!("Listening on port {}", port);

    for stream in listener.incoming() {
        let client_socket = stream?;
        let client_address = client_socket.peer_addr()?;
        println!("Connection from: {}", client_address);

        let child_socket = client_socket.try_clone()?;
        thread::spawn(move || handle_client(child_socket));
    }

    Ok(())
}

fn handle_client(client_socket: TcpStream) {
    let mut reader = BufReader::new(&client_socket);
    let mut input = String::new();
    loop {
        input.clear();
        reader.read_line(&mut input).unwrap();
        print!("Received: {}", input);

        let mut response = String::new();
        io::stdin().read_line(&mut response).unwrap();
        {
            let mut socket_writer = &client_socket;
            socket_writer.write_all(response.as_bytes()).unwrap();
        }
    }
}
