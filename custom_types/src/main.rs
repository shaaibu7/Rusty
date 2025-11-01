use core::num;

#[derive(Debug)]
struct Book {
    name: String,
    isbn: String,
    author: String,
    year: u16,
}

// Tuple struct

#[derive(Debug)]
struct Config(u8, String, u8);

impl Book {
    // Constructor function to create new book
    fn new_book(name: String, isbn: String, author: String, year: u16) -> Self {
        Book {
            name,
            isbn,
            author,
            year,
        }
    }
    fn get_book_name(&self) -> &String {
        let name = &self.name;
        name
    }

    fn get_book_author(&self) -> &String {
        let author = &self.author;
        author
    }
}

fn check_even_number(number: u16) -> Option<u16> {
    if number % 2 == 0 {
        Some(number)
    } else {
        None
    }
}

fn main() {
    let book: Book = Book {
        name: String::from("The journey of a thousand miles"),
        isbn: String::from("1232-4356-7787"),
        author: String::from("Shana Williams"),
        year: 2016,
    };

    // Using the constructor associated function to create a new book
    let book1 = Book::new_book(
        String::from("Chike and the river"),
        String::from("1234-5487-9088"),
        String::from("Wole Soyinka"),
        1999,
    );

    println!("The book details is {book:?}");
    println!("The book details created with a contructor function is {book1:?}");

    let even_check = check_even_number(8);
    println!("The result from the even number check is: {:?}", even_check.unwrap());

    let even_check1 = check_even_number(3);
    println!("The result from the even number check is: {:?}", even_check1);

    // Tuple struct
    let config = Config(89, String::from("data_cat"), 77);

    println!("Here is an example of a tuple struct {config:?}");
    println!("Data in a tuple struct can be accessed like this: config_no: {} config_name: {} config_version {}", config.0, config.1, config.2);
}
