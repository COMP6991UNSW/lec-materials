use std::error::Error;
use std::str::FromStr;

use sqlx::ConnectOptions;
use sqlx::sqlite::SqliteConnectOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut conn = SqliteConnectOptions::from_str("sqlite://test.db")?
        .connect()
        .await?;

    let account = sqlx::query!("select first_name, last_name, email, phone from contacts")
        .fetch_one(&mut conn)
        .await?;
    
    println!("{account:?}");

    Ok(())
}
