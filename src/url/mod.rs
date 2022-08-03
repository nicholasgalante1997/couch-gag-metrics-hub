pub mod url {
    pub struct ReqUrl {
        pub path: String,
        pub query_parameters: Vec<(String, String)>,
    }
}