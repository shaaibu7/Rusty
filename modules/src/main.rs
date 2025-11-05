mod custom;

use custom::types::types::Book;

fn main() {
    let book = Book {
        name: String::from("Journey to the east"),
        isbn: String::from("123-332-987"),
        author: String::from("suleiman"),
        year: 2018,
    };


    println!("The imported book from another module is: {:?}", book)
}
