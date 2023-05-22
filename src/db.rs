use postgres::{Client, NoTls};

pub fn connect() -> Client {
    let server_url = std::env::var("PG_URL").expect("PG_URL must be set");
    
    let mut client =
        Client::connect(&server_url, NoTls).unwrap();
    let result = client.batch_execute(
        "
          CREATE TABLE IF NOT EXISTS users (
              id              SERIAL PRIMARY KEY,
              username        VARCHAR UNIQUE NOT NULL,
              password        VARCHAR NOT NULL,
              email           VARCHAR UNIQUE NOT NULL
              )
      ",
    );

    match result {
        Ok(r) => println!("Query is success {:?}", r),
        Err(e) => {
            println!("Error");
            println!("{e}");
        }
    }

    client
}
