use postgres::{Client, NoTls};

pub fn connect() -> Option<Client> {
    let server_url = std::env::var("PG_URL").expect("PG_URL must be set");

    let mut client = match Client::connect(&server_url, NoTls) {
        Ok(c) => c,
        Err(_) => return None,
    };

    println!("Database connection success");

    let query = "
        CREATE TABLE IF NOT EXISTS users (
            id              SERIAL PRIMARY KEY,
            username        VARCHAR UNIQUE NOT NULL,
            password        VARCHAR NOT NULL,
            email           VARCHAR UNIQUE NOT NULL
            )
        ";

    if let Err(e) = client.batch_execute(query) {
        println!("{e}");
    }

    Some(client)
}
