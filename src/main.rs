use rocket::routes;
use rust_api::{ws, BookStore};

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = BookStore::new();
    let _ = rocket::build()
        .manage(store)
        .mount("/", routes![ws])
        .launch()
        .await?;
    Ok(())
}
