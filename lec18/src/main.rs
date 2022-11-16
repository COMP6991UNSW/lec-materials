use std::{path::PathBuf, fmt::Display, str::FromStr};

use rocket::{get, routes, post, Data, data::ToByteUnit, tokio::fs::File, serde::json::Json, request::FromParam};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

struct PasteId(Uuid);

impl PasteId {
    pub fn new_random() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn paste_to_path(&self) -> PathBuf {
        PathBuf::from(".").join("pastes").join(self.0.to_string())
    }
}

impl Display for PasteId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<'a> FromParam<'a> for PasteId {
    type Error = String;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        Uuid::from_str(param)
            .map(|uuid| PasteId(uuid))
            .map_err(|_| String::from("Invalid paste id (not a uuidv4)"))
    }
}

#[get("/")]
fn hello() -> &'static str {
    "Welcome to cs6991-pasteRS!"
}

#[derive(Serialize, Deserialize)]
struct CreatePasteResponse {
    paste_id: String,
}

#[post("/create", data = "<paste>")]
async fn create_paste(paste: Data<'_>) -> Json<CreatePasteResponse> {
    let paste_id = PasteId::new_random();
    let path = paste_id.paste_to_path();
    let mut file = File::create(path)
        .await
        .expect("The path should be fine to create a new file in");

    paste.open(512.kilobytes())
        .stream_to(&mut file)
        .await
        .expect("The file should be fine to write to (with the data)");
    
    Json(
        CreatePasteResponse { paste_id: paste_id.to_string() }
    )
}

#[get("/get/<paste_id>")]
async fn retrieve_paste(paste_id: PasteId) -> Option<File> {
    let path = paste_id.paste_to_path();

    File::open(path)
        .await
        .ok()
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    println!("The website is starting...");

    let rocket = rocket::build()
        .mount("/", routes![hello, create_paste, retrieve_paste]);
    
    let _ = rocket.launch().await?;

    Ok(())
}
