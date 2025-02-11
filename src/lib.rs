use rocket::futures::SinkExt;
use rocket::futures::StreamExt;
use rocket::get;
use rocket_ws::{Channel, Message, WebSocket};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Clone)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub year: i32,
}

pub struct BookStore {
    books: Arc<Mutex<HashMap<String, Book>>>,
}

impl BookStore {
    pub fn new() -> Self {
        BookStore {
            books: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get_books(&self) -> Vec<Book> {
        self.books.lock().unwrap().values().cloned().collect()
    }

    pub fn get_book(&self, id: &str) -> Option<Book> {
        self.books.lock().unwrap().get(id).cloned()
    }

    pub fn add_book(&self, book: Book) {
        let id = self.books.lock().unwrap().len() + 1;
        self.books.lock().unwrap().insert(id.to_string(), book);
    }

    pub fn update_book(&self, id: &str, book: Book) -> bool {
        let mut map = self.books.lock().unwrap();
        if let Some(_) = map.get(id) {
            map.insert(id.to_string(), book);
            true
        } else {
            false
        }
    }

    pub fn delete_book(&self, id: &str) -> bool {
        self.books.lock().unwrap().remove(id).is_some()
    }
}

// WebSocket route handler
#[get("/")]
pub fn ws<'r>(ws: WebSocket, store: &'r rocket::State<BookStore>) -> Channel<'r> {
    ws.channel(move |mut stream| {
        Box::pin(async move {
            while let Some(message) = stream.next().await {
                match message {
                    Ok(msg) => {
                        if let Message::Text(text) = msg {
                            println!("Received message: {}", text);
                            let request: serde_json::Value = match serde_json::from_str(&text) {
                                Ok(req) => req,
                                Err(err) => {
                                    println!("Error parsing JSON: {}", err);
                                    let _ = stream
                                        .send(Message::Text(format!("Error parsing JSON: {}", err)))
                                        .await;
                                    continue;
                                }
                            };

                            let action =
                                request.get("action").and_then(|a| a.as_str()).unwrap_or("");
                            match action {
                                "get_books" => {
                                    let books = store.get_books();
                                    let json = serde_json::to_string(&books).unwrap();
                                    let _ = stream.send(Message::Text(json)).await;
                                }
                                "get_book" => {
                                    let id =
                                        request.get("id").and_then(|i| i.as_str()).unwrap_or("");
                                    if let Some(book) = store.get_book(id) {
                                        let json = serde_json::to_string(&book).unwrap();
                                        let _ = stream.send(Message::Text(json)).await;
                                    } else {
                                        let _ = stream
                                            .send(Message::Text(format!("Book not found: {}", id)))
                                            .await;
                                    }
                                }
                                "add_book" => {
                                    let book: Book = match serde_json::from_value(
                                        request.get("book").unwrap().clone(),
                                    ) {
                                        Ok(b) => b,
                                        Err(err) => {
                                            println!("Error parsing book: {}", err);
                                            let _ = stream
                                                .send(Message::Text(format!(
                                                    "Error parsing book: {}",
                                                    err
                                                )))
                                                .await;
                                            continue;
                                        }
                                    };
                                    store.add_book(book);
                                    let _ = stream.send(Message::Text(format!("Book added"))).await;
                                }
                                "update_book" => {
                                    let id =
                                        request.get("id").and_then(|i| i.as_str()).unwrap_or("");
                                    let book: Book = match serde_json::from_value(
                                        request.get("book").unwrap().clone(),
                                    ) {
                                        Ok(b) => b,
                                        Err(err) => {
                                            println!("Error parsing book: {}", err);
                                            let _ = stream
                                                .send(Message::Text(format!(
                                                    "Error parsing book: {}",
                                                    err
                                                )))
                                                .await;
                                            continue;
                                        }
                                    };
                                    if store.update_book(id, book) {
                                        let _ = stream
                                            .send(Message::Text(format!("Book updated")))
                                            .await;
                                    } else {
                                        let _ = stream
                                            .send(Message::Text(format!("Book not found: {}", id)))
                                            .await;
                                    }
                                }
                                "delete_book" => {
                                    let id =
                                        request.get("id").and_then(|i| i.as_str()).unwrap_or("");
                                    if store.delete_book(id) {
                                        let _ = stream
                                            .send(Message::Text(format!("Book deleted: {}", id)))
                                            .await;
                                    } else {
                                        let _ = stream
                                            .send(Message::Text(format!("Book not found: {}", id)))
                                            .await;
                                    }
                                }
                                _ => {
                                    let str_ = "Unknown action".to_string();
                                    let _ = stream.send(Message::Text(str_)).await;
                                }
                            }
                        }
                    }
                    Err(err) => {
                        println!("Error: {}", err);
                        break;
                    }
                }
            }
            Ok(())
        })
    })
}
