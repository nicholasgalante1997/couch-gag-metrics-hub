use std::io::prelude::*;
// use std::fs;
// use std::fs::File;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    // creates an instance of a TCPListener
    // This code will listen at the address 127.0.0.1:7878 for incoming TCP streams, 
    // which we can access via invoking .incoming() on listener which returns an iterator (see below)

    // The bind function in this scenario works like the new function in that it will return a new TcpListener instance. 
    // The reason the function is called bind is that in networking, 
    // connecting to a port to listen to is known as “binding to a port.”

    // The bind function returns a Result<T, E>, which indicates that binding might fail. 
    // For example, connecting to port 80 requires administrator privileges (nonadministrators can listen only on ports higher than 1023), 
    // so if we tried to connect to port 80 without being an administrator, binding wouldn’t work. 
    // As another example, binding wouldn’t work if we ran two instances of our program and so had two programs listening to the same port. 
    // Because we’re writing a basic server just for learning purposes, we won’t worry about handling these kinds of errors; 
    // instead, we use .unwrap() to stop the program if errors happen.

    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // The incoming method on TcpListener returns an iterator that gives us a sequence of streams 
    // (more specifically, streams of type TcpStream). 
    // A single stream represents an open connection between the client and the server. 
    // A connection is the name for the full request and response process in which a client connects to the server, 
    // the server generates a response, and the server closes the connection. 
    // As such, TcpStream will read from itself to see what the client sends,
    // and then allow us to write our response to the stream. 
    // Overall, this for loop will process each connection in turn and produce a series of streams for us to handle.

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);

    let mut method = String::new();

    get_http_method(&request.to_string(), &mut method);

    println!("Method: {}", method);

    println!("Request: {}", request);

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn get_http_method(request: &String, method: &mut String) {
  if request.contains("GET") {
    * method = String::from("GET");
  } else if request.contains("POST") {
    * method = String::from("POST");
  } else if request.contains("OPTIONS") {
    * method = String::from("OPTIONS");
  } else {
    * method = String::from("OTHER");
  }
}