use sqlx::MySqlPool;

pub async fn create_tables(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    match sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTO_INCREMENT,
            email TEXT NOT NULL,
            username TEXT NOT NULL,
            password TEXT NOT NULL,
            created_at BIGINT
        )",
    )
    .execute(pool)
    .await
    {
        Ok(_) => println!("Users table created successfully"),
        Err(e) => {
            eprintln!("Error creating users table: {:?}", e);
            return Err(e);
        }
    };
    Ok(())
}
