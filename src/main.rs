use clap::{App, Arg};
use std::io::Read;
use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() -> io::Result<()> {
    let app = App::new("Network Interaction Netcat Clone")
        .version("0.1.0")
        .author("akash2061")
        .about("Performs network operations similar to Netcat")
        .arg(
            Arg::new("iphost")
                .short('i')
                .long("host")
                .takes_value(true)
                .required(true)
                .help("The host address to connect to or listen on"),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .takes_value(true)
                .required(true)
                .help("The port number to connect to or listen on"),
        )
        .arg(
            Arg::new("mode")
                .short('m')
                .long("mode")
                .takes_value(true)
                .required(true)
                .help("Mode of operation: -c (connect) or -l (listen)"),
        );

    let matches = app.get_matches();

    let host = matches
        .value_of("host")
        .expect("Host is a required argument");
    let port_str = matches
        .value_of("port")
        .expect("Port is a required argument");
    let port = port_str.parse::<u16>().expect("Invalid port number");
    let mode = matches
        .value_of("mode")
        .expect("Mode is a required argument");

    match mode {
        "-c" => client(host, port)?,
        "-l" => server(port)?,
        _ => eprintln!("Invalid mode: {}", mode), // Clap should handle this, but just in case
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
