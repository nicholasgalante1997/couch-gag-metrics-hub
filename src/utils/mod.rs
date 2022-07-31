pub mod utils {
    use std::fs::File;

    pub fn file_reader(path: &str) -> File {
        let f = File::open(path);
        let f = match f {
            Ok(file) => file,
            Err(error) => {
                panic!("Problem opening file {:?}, with error {:?}", path, error)
            }
        };
        f
    }

    pub struct ReqUrl {
        pub path: String,
        pub query_parameters: Vec<(String, String)>,
    }
}
