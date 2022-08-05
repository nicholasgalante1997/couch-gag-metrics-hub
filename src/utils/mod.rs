pub mod utils {

    use crate::metrics::metric::MetricName;
    use crate::url::url::ReqUrl;
    use std::result::Result;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::Error;

    // File utils

    // https://doc.rust-lang.org/std/result/
    pub fn file_reader(path: &str) -> Result<String, Error> {

        // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator

        let mut f = File::open(path)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;

        // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#propagating-errors

        // let f = match f { 
        //     Ok(file) => file,
        //     Err(error) => {
        //         return Err(error)
        //     }
        // };

        // let f = match f.read_to_string(&mut s) {
        //     Ok(_) => Ok(s),
        //     Err(e) => Err(e)
        // };

        Ok(s)
    }

    pub fn get_key_value_pair_from_env(file_contents: String, key: &str) -> String {
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

    pub fn get_env_file() -> String {
        // load .env variables
        let env_file: Result<String, Error> = file_reader(".env");

        // create an empty string to load the file contents into
        let mut contents = String::new();

        if env_file.is_err() {
            let safe_error: Result<String, Error> = env_file.or_else(|err| Ok(format!("{}::{}", err.kind(), err.to_string())));
            contents = safe_error.and_then(|s| Ok(s)).unwrap();
        } else {
            contents = env_file.unwrap();
        }


        contents
    }

    // Query param utils

    pub fn get_val_off_query_params(req_url: &ReqUrl) -> u8 {
        let mut val = 0;
        for req_param in req_url.query_parameters.clone() {
            if req_param.0.contains("value") {
                val = req_param.1.parse::<u8>().unwrap();
            }
        }
        val
    }

    pub fn get_metric_subfield_off_query_params(req_url: &ReqUrl) -> String {
        let mut msg = String::new();
        for req_param in req_url.query_parameters.clone() {
            if req_param.0.contains("subfield") {
                msg = String::from(req_param.1);
            }
        }
        msg
    }

    pub fn get_metric_type_off_query_param(req_url: &ReqUrl) -> MetricName {
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

    // request utils

    pub fn get_url_from_req(req: &String) -> ReqUrl {
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
                query_parameters.push((
                    String::from(key_value_vec[0]),
                    String::from(key_value_vec[1]),
                ))
            }
        }

        ReqUrl {
            path: String::from(sanitized_path),
            query_parameters: query_parameters,
        }
    }

    pub fn get_path(req_url: &ReqUrl) -> String {
        req_url.path.clone()
    }

    pub fn get_headers_off_req(request: &String) -> Vec<(&str, &str)> {
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

    pub fn get_http_method(request: &String, method: &mut String) {
        let safe_req_split: Vec<&str> = request.split("\r\n").collect();
        let req_line = safe_req_split[0];
        if req_line.contains("GET") {
            *method = String::from("GET");
        } else if req_line.contains("POST") {
            *method = String::from("POST");
        } else if req_line.contains("OPTIONS") {
            *method = String::from("OPTIONS");
        } else {
            *method = String::from("OTHER");
        }
    }

    // Validity check

    pub fn has_valid_ulysses_key(request: &String) -> bool {
        let file_contents = get_env_file();
        // this is the flag that gets passed back as the return value

        let ulysses_key = get_key_value_pair_from_env(file_contents, "ULYSSES_HASHED_KEY");
        // get headers off the request

        let req_headers: Vec<(&str, &str)> = get_headers_off_req(request);
        let mut req_ulysses_key = String::new();
        for header in req_headers.iter() {
            if header.0.contains("x-ulysses-key") {
                req_ulysses_key = String::from(header.1);
                println!("req uly key: {}", req_ulysses_key)
            }
        }
        ulysses_key == req_ulysses_key
    }


    pub fn is_valid_path(path: &String) -> bool {
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

    // response utils

    pub fn add_headers_to_response(response: &mut String, headers: &Vec<(String, String)>) {
        let final_index = headers.len() - 1;
        for header in headers.iter() {
            response.push_str(&*format!("{}: {}", header.0, header.1));
            if header != headers.get(final_index).unwrap() {
                response.push_str("\n");
            }
        }
    }
    
}
