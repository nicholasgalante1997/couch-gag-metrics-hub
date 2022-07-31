use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use crate::metrics::metric::Metric;
use crate::metrics::metric::MetricName;
use crate::utils::utils::file_reader;
use crate::utils::utils::ReqUrl;

pub mod metrics;
pub mod utils;

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
    headers.push((
        String::from("x-ulysses-metr-id"),
        String::from("ikebroflovski"),
    ));

    // cors headers
    headers.push((
        String::from("Access-Control-Allow-Origin"),
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

    // get the path off of the request
    let req_url = get_url_from_req(&request.clone().to_string());
    let path = get_path(&req_url);

    println!("{}", path);

    // handle an attempt to access a path we havent defined
    if !is_valid_path(&path) {
        let error = String::from("[Error] Attempt to access an inaccessible path.");
        errors.push((String::from("PathError"), error));
    }

    // handle an unauthorized attempt to hit the service
    if !has_valid_ulysses_key(&request.clone().to_string()) {
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
            response.push_str(&* format!("\"{}\":\"{}\"", error.0, error.1));
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
        value: metric_value
      };
    }

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn get_val_off_query_params(req_url: &ReqUrl) -> u8 {
  let mut val = 0;
  for req_param in req_url.query_parameters.clone() {
    if req_param.0.contains("value") {
      val = req_param.1.parse::<u8>().unwrap();
    }
  }
  val
}

fn get_metric_subfield_off_query_params(req_url: &ReqUrl) -> String {
  let mut msg = String::new();
  for req_param in req_url.query_parameters.clone() {
    if req_param.0.contains("subfield") {
      msg = String::from(req_param.1);
    }
  }
  msg
}

fn get_metric_type_off_query_param(req_url: &ReqUrl) -> MetricName {
  let mut metric_type: MetricName = MetricName::Base;

  for req_param in req_url.query_parameters.clone() {
    if req_param.0.contains("metric") {
      metric_type = {
        if req_param.1.eq("story-view") {
          MetricName::StoryView
        } else if req_param.1.eq("page-view") {
          MetricName::PageView
        } else if req_param.1.eq("share") {
          MetricName::Share
        } else if req_param.1.eq("button-click") {
          MetricName::ButtonClick
        } else {
          MetricName::Error
        }
      }
    }
  }

  metric_type
}

fn get_url_from_req(req: &String) -> ReqUrl {
  let mut query_parameters: Vec<(String, String)> = Vec::new();
  let request_vec: Vec<&str> = req.split("\r\n").collect();
  let request_line = request_vec[0];
  let split_request_line: Vec<&str> = request_line.split(" ").collect();
  let path_unsanitized = split_request_line[1];
  let path_vec: Vec<&str> = path_unsanitized.split("?").collect();
  let sanitized_path = path_vec[0];

  if path_vec.len() > 1 {
    let query_param_string = path_vec[1];

    let query_param_vec: Vec<&str> = query_param_string.split("&").collect();

    for kv in query_param_vec {
      let key_value_vec: Vec<&str> = kv.split("=").collect();
      query_parameters.push((String::from(key_value_vec[0]), String::from(key_value_vec[1])))
    }
  }

  ReqUrl {
    path: String::from(sanitized_path),
    query_parameters: query_parameters
  }
}

fn get_path(req_url: &ReqUrl) -> String {
    req_url.path.clone()
}

fn is_valid_path(path: &String) -> bool {
    if path.eq("/") {
        true
    } else if path.eq("/ping") {
        true
    } else if path.eq("/metric") {
        true
    } else {
        false
    }
}

fn get_env_file() -> String {
    // load .env variables
    let mut env_file: File = file_reader(".env");
    // create an empty string to load the file contents into
    let mut contents = String::new();
    // pass the mut string reference to read_to_string(...) which copies file contents to string
    env_file.read_to_string(&mut contents).unwrap();

    contents
}

fn get_key_value_pair_from_env(file_contents: String, key: &str) -> String {
    // split the string on newlines (kv pairs)
    let contents_vec: Vec<&str> = file_contents.split("\n").collect();
    // create a string to load the ulysses kv pair into
    let mut key_pair_load_string = String::new();

    // iterate through env key value pairs
    for key_pair in contents_vec.iter() {
        // if we find the right key, load it to ulysses_key_pair (empty string)
        if key_pair.contains(&key) {
            key_pair_load_string = String::from(*key_pair)
        }
    }

    // split the key-value pair on "="
    let selected_key_value_collection: Vec<&str> = key_pair_load_string.split("=").collect();
    let value = String::from(selected_key_value_collection[1]);
    value
}

fn get_headers_off_req(request: &String) -> Vec<(&str, &str)> {
  let mut headers: Vec<(&str, &str)> = Vec::new();
  let request_vec: Vec<&str> = request.split("\r\n").collect();
    for req_piece in request_vec.iter() {
      if req_piece.contains(": ") {
        let split_req_piece: Vec<&str> = req_piece.split(": ").collect();
        headers.push((split_req_piece[0], split_req_piece[1]));
      }
    }
  headers
}

fn has_valid_ulysses_key(request: &String) -> bool {
    let file_contents = get_env_file();
    // this is the flag that gets passed back as the return value

    let ulysses_key = get_key_value_pair_from_env(file_contents, "ULYSSES_HASHED_KEY");
    // get headers off the request
    
    let req_headers: Vec<(&str, &str)> = get_headers_off_req(request);
    let mut req_ulysses_key = String::new();
    for header in req_headers.iter() {
        if header.0.contains("x-ulysses-key") {
            req_ulysses_key = String::from(header.1);
        }
    }
    ulysses_key == req_ulysses_key
}

fn get_http_method(request: &String, method: &mut String) {
    if request.contains("GET") {
        *method = String::from("GET");
    } else if request.contains("POST") {
        *method = String::from("POST");
    } else if request.contains("OPTIONS") {
        *method = String::from("OPTIONS");
    } else {
        *method = String::from("OTHER");
    }
}

fn add_headers_to_response(response: &mut String, headers: &Vec<(String, String)>) {
    let final_index = headers.len() - 1;
    for header in headers.iter() {
        response.push_str(&* format!("{}: {}", header.0, header.1));
        if header != headers.get(final_index).unwrap() {
            response.push_str("\n");
        }
    }
}
