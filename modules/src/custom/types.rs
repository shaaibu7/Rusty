pub mod types {
    #[derive(Debug)]
    pub struct Book {
        pub name: String,
        pub isbn: String,
        pub author: String,
        pub year: u16,
    }
}
