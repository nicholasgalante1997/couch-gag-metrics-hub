pub mod cors_base_kit {
    use std::collections::HashMap;

    pub struct CorsHeaders {}

    impl CorsHeaders {
        pub fn add_cors_to_headers(headers: & mut HashMap<String, String>) {
            headers.insert(
                String::from("Access-Control-Allow-Origin"), 
                String::from("*")
            );
            headers.insert(
                String::from("Access-Control-Request-Methods"),
                String::from("*"),
            );
            headers.insert(
                String::from("Access-Control-Allow-Methods"),
                String::from("OPTIONS, GET"),
            );
            headers.insert(
                String::from("Access-Control-Allow-Headers"),
                String::from("*"),
            );
        }
    }

    
}