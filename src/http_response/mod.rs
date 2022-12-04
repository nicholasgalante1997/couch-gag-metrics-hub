pub mod http_response {
    use std::collections::HashMap;
    use crate::http_constants::http_base_kit::http_constants::HttpConstants;

    pub struct HttpResponse {
        pub status: usize,
        pub headers: HashMap<String, String>,
        pub body: String,
    }

    impl HttpResponse {
        pub fn build(&self) -> String {
            let crlf = HttpConstants::get_crlf();

            let mut response_string = String::new();
            let mut status_line = String::new();

            if self.status == 200 {
                status_line = HttpConstants::get_success_get_protocol_http_prefix();
            } else {
                status_line = HttpConstants::get_error_server_internal_protocol_http_prefix();
            };

            // scaffolding headers onto response
            let mut header_string = String::new();

            // we map over the hashmap to a vec for formatting ease bc we need to know when to add the crlf
            let mut header_vector_stash: Vec<(String, String)> = vec![];
            for (key, value) in self.headers.clone() {
                header_vector_stash.push((String::from(key), String::from(value)));
            }

            let header_vec_len: usize = header_vector_stash.len();
            let mut header_vec_index_count: usize = 0;
            while header_vec_index_count < header_vec_len {
                let header_kv_option = header_vector_stash.get(header_vec_index_count);
                let header_kv_tuple = header_kv_option.unwrap();
                let header_kv_string = format!("{}: {}", header_kv_tuple.0, header_kv_tuple.1);

                header_string.push_str(&header_kv_string);

                if header_vec_index_count == header_vec_len - 1 {
                    header_string.push_str(crlf);
                    header_string.push_str(crlf);
                } else {
                    header_string.push_str("\n");
                };

                header_vec_index_count += 1;
            }

            // scaffolding body onto response
            let mut body_string = String::new();
            if self.body != String::new() {
                body_string = self.body.clone();
            }

            response_string.push_str(&status_line);
            response_string.push_str(&header_string);

            if body_string != String::new() {
                response_string.push_str(&body_string);
            }

            response_string
        }
    }
}