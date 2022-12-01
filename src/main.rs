use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::collections::HashMap;

use crate::metrics::metric::Metric;
use crate::utils::utils::*;
use crate::http_response::http_response::HttpResponse;
use crate::cors::cors::add_cors_to_headers;
use crate::content_type::content_type::add_content_type_to_headers;

pub mod cors;
pub mod content_type;
pub mod http_constants;
pub mod http_response;
pub mod metrics;
pub mod utils;
pub mod url;

/**
* Main Entry Point of the Microservice;
*/
fn main() {
    // creates an instance of a TCPListener
    // This code will listen at the address ::7878 for incoming TCP streams,
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

    // https://blog.logrocket.com/packaging-a-rust-web-service-using-docker/#:~:text=The%20code%20for%20the%20basic%20web%20app%20isn%E2%80%99t%20particularly%20exciting.%20However%2C%20it%E2%80%99s%20important%20to%20note%20the%20criticality%20of%20the%200.0.0.0%20when%20binding%20the%20server%20to%20an%20IP%20and%20port.%20Using%20127.0.0.1%20or%20localhost%20here%20won%E2%80%99t%20work%20from%20inside%20docker. 
    let listener: TcpListener = TcpListener::bind("0.0.0.0:7878").unwrap();

    // The incoming method on TcpListener returns an iterator that gives us a sequence of streams
    // (more specifically, streams of type TcpStream).
    // A single stream represents an open connection between the client and the server.
    // A connection is the name for the full request and response process in which a client connects to the server,
    // the server generates a response, and the server closes the connection.
    // As such, TcpStream will read from itself to see what the client sends (Request),
    // and then allow us to write our response to the stream (Response).
    // Overall, this for loop will process each connection in turn and produce a series of streams for us to handle.

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {

    // WORKING WITH THE REQUEST 
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // store the request in a Clone-on-write<_, String> (smart pointer type)
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Request: {} \n", request);

    let mut errors: Vec<(String, String)> = Vec::new();

    let mut method = String::new();
    get_http_method(&request.clone().to_string(), &mut method);

    let req_url = get_url_from_req(&request.clone().to_string());
    let path = get_path(&req_url);

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
    
    // WORKING WITH THE RESPONSE    
    let mut headers_hashmap: HashMap<String, String> = HashMap::new();
    add_cors_to_headers(&mut headers_hashmap);
    add_content_type_to_headers(&mut headers_hashmap);

    let mut status_code = 200;
    let mut body = String::new();

    // if we do have errors, reassign status to 500, update body
    if errors.len() > 0 {
        let mut error_hashmap: HashMap<String, Vec<(String, String)>> = HashMap::new();
        error_hashmap.insert(
            String::from("errors"),
            errors
        );
        status_code = 500;
        body = serde_json::to_string(&error_hashmap).unwrap_or(String::new())
    } else {
        let metric_type = Metric::get_metric_type_off_query_param(&req_url);
        let metric_subfield = Metric::get_metric_subfield_off_query_params(&req_url);
        let metric_value = Metric::get_val_off_query_params(&req_url);
        let metric_target = Metric::get_target_string_off_query_params(&req_url);
        let metric = Metric::get_metric(metric_type, metric_subfield, metric_target, metric_value);
        let mut metric_hashmap: HashMap<String, Metric> = HashMap::new();
        metric_hashmap.insert(
            String::from("Metric"),
            metric
        );
        body = serde_json::to_string(&metric_hashmap).unwrap_or(String::new());
    }

    let http_response = HttpResponse {
        body,
        headers: headers_hashmap,
        status: status_code
    };

    let response = http_response.build();

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
