pub mod http_base_kit {

    const STATUS_200_RESPONSE_PREFIX: &str = "HTTP/1.1 200 OK";
    const STATUS_500_RESPONSE_PREFIX: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR";

    pub mod http_constants {
        use super::{STATUS_200_RESPONSE_PREFIX, STATUS_500_RESPONSE_PREFIX};

        pub fn get_crlf() -> &'static str {
            "\r\n"
        }

        pub fn get_success_get_protocol_http_prefix() -> String {
            let mut http_head = String::from(STATUS_200_RESPONSE_PREFIX);
            let crlf = get_crlf();
            http_head.push_str(crlf);
            http_head
        }

        pub fn get_error_server_internal_protocol_http_prefix() -> String {
            let mut http_head = String::from(STATUS_500_RESPONSE_PREFIX);
            let crlf = get_crlf();
            http_head.push_str(crlf);
            http_head
        }

    }
}
