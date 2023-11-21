// coding playground for Rust
struct Book {
    pages: i32,
    rating: i32,
}

fn display_page_count(book: &Book) {
    println!("This book has {:?} pages", book.pages);
}

fn display_rating(book: Book) {
    println!("This book is rated {:?}", book.rating);
}

fn main() {
    let book = Book {
        pages: 100,
        rating: 5,
    };

    display_page_count(&book);
    display_rating(book);
}
