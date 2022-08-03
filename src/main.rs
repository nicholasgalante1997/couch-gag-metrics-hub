use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use crate::metrics::metric::Metric;
use crate::utils::utils::*;

pub mod metrics;
pub mod utils;
pub mod url;

/** This does not make use of Rocket.
 * In fact, I'm ashamed to say I've never even looked at their docs/source code
 * We (I) may elect to lean on them for **loose** inspiration of structs */

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
    // creates a mutable Array of 1024 elements. Every element is 0;
    // we'll use this buffer to store the stream_read
    let mut buffer = [0; 1024];
    // pull bytes from the stream source and load them into the "buffer"
    stream.read(&mut buffer).unwrap();

    // store the request in a Clone-on-write<_, String> (smart pointer type)
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Request: {}", request);

    // create a vec of tuple<String, String> to load headers
    let mut headers: Vec<(String, String)> = Vec::new();

    // cors headers
    headers.push((
        String::from("Access-Control-Allow-Origin"),
        String::from("*"),
    ));
    headers.push((
        String::from("Access-Control-Request-Methods"),
        String::from("*"),
    ));
    headers.push((
        String::from("Access-Control-Allow-Methods"),
        String::from("OPTIONS, GET, POST"),
    ));
    headers.push((
        String::from("Access-Control-Allow-Headers"),
        String::from("*"),
    ));

    // content type header
    headers.push((
        String::from("Content-Type"),
        String::from("application/json"),
    ));

    // create a vec to store possible server errors to append to body
    let mut errors: Vec<(String, String)> = Vec::new();

    // create an empty string to hold the result of get_http_method
    let mut method = String::new();
    get_http_method(&request.clone().to_string(), &mut method);

    println!("{}", method);

    // get the path off of the request
    let req_url = get_url_from_req(&request.clone().to_string());
    let path = get_path(&req_url);

    // handle an attempt to access a path we havent defined
    if !is_valid_path(&path) {
        let error = String::from("[Error] Attempt to access an inaccessible path.");
        errors.push((String::from("PathError"), error));
    }

    // We hit a gnarly bug with preflight requests being for lack of a better word fucked
    // because the browser wasn't attaching x-ulysses-key to the preflight check
    // and I think it just wont (will investigate)
    // if we hit an options request, we bypass ulysses key, but to perform operations,
    // we'll run this has_valid_ulysses_key_check before pumping metrics
    // handle an unauthorized attempt to hit the service
    if method != "OPTIONS" && !has_valid_ulysses_key(&request.clone().to_string()) {
        let error = String::from("[Error]: Invalid ulysses key.");
        errors.push((String::from("CredentialsError"), error));
    }

    // create our ideal response, handle errors later
    let mut response = String::from("HTTP/1.1 200 OK\r\n");

    if errors.len() > 0 {
        response = String::from("HTTP/1.1 500 Internal Server Error\r\n");
    }

    add_headers_to_response(&mut response, &headers);

    if errors.len() > 0 {
        let final_index = errors.len() - 1;
        response.push_str("\r\n\r\n"); // append CRLF
        response.push_str("{"); // begin json
        for error in errors.iter() {
            response.push_str(&*format!("\"{}\":\"{}\"", error.0, error.1));
            if error != errors.get(final_index).unwrap() {
                response.push_str(",");
            }
        }
        response.push_str("}") // end json
    } else {
        let metric_type = get_metric_type_off_query_param(&req_url);
        let metric_subfield = get_metric_subfield_off_query_params(&req_url);
        let metric_value = get_val_off_query_params(&req_url);
        let _metric = Metric {
            metric_type: metric_type,
            subfield: metric_subfield,
            value: metric_value,
        };
    }

    println!("Response:\n{}", response);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
