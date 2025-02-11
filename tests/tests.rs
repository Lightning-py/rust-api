use rust_api::*;

fn books_equal(first: &Book, second: &Book) -> bool {
    return first.title == second.title
        && first.author == second.author
        && first.year == second.year;
}

#[test]
fn test_add_book() {
    let store = BookStore::new();
    let book = Book {
        title: "The Great Gatsby".to_string(),
        author: "F. Scott Fitzgerald".to_string(),
        year: 1925,
    };

    store.add_book(book.clone());
    let books = store.get_books();
    assert_eq!(books.len(), 1);
    assert!(books_equal(&books[0], &book))
}

#[test]
fn test_get_book() {
    let store = BookStore::new();
    let book = Book {
        title: "1984".to_string(),
        author: "George Orwell".to_string(),
        year: 1949,
    };

    store.add_book(book.clone());
    let retrieved_book = store.get_book("1").unwrap();
    assert!(books_equal(&retrieved_book, &book))
}

#[test]
fn test_update_book() {
    let store = BookStore::new();
    let book = Book {
        title: "To Kill a Mockingbird".to_string(),
        author: "Harper Lee".to_string(),
        year: 1960,
    };

    store.add_book(book.clone());
    let updated_book = Book {
        title: "To Kill a Mockingbird (Updated)".to_string(),
        author: "Harper Lee".to_string(),
        year: 1961,
    };

    assert!(store.update_book("1", updated_book.clone()));
    let retrieved_book = store.get_book("1").unwrap();
    assert!(books_equal(&retrieved_book, &updated_book))
}

#[test]
fn test_delete_book() {
    let store = BookStore::new();
    let book = Book {
        title: "Brave New World".to_string(),
        author: "Aldous Huxley".to_string(),
        year: 1932,
    };

    store.add_book(book.clone());
    assert!(store.delete_book("1"));
    assert!(store.get_book("1").is_none());
}
