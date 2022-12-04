pub mod content_type_base_kit {
    use std::collections::HashMap;
    
    pub struct ContentHeaders {}

    impl ContentHeaders {
        pub fn add_content_type_to_headers(headers: &mut HashMap<String, String>){
            headers.insert(
                String::from("Content-Type"),
                String::from("application/json"),
            );
        }  
    }
   
}