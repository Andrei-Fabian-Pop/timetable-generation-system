use tokio_postgres::{Client, NoTls, Error};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct UserCredentials {
    pub username: String,
    pub password: String,
}

pub struct Database {
    client: Client,
}

impl Database {
    // Function to create a new database connection
    pub async fn new(conn_str: &str) -> Result<Self, Error> {
        let (client, connection) = tokio_postgres::connect(conn_str, NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        Ok(Database { client })
    }

    // Function to add a new professor
    pub async fn add_professor(&self, first_name: &str, last_name: &str, email: &str, department: &str, username: &str, password: &str) -> Result<(), Error> {
        // Insert professor details
        self.client.execute(
            "INSERT INTO professors (first_name, last_name, email, department) VALUES ($1, $2, $3, $4)",
            &[&first_name, &last_name, &email, &department],
        ).await?;

        // Get the professor_id of the newly added professor
        let row = self.client.query_one(
            "SELECT professor_id FROM professors WHERE email = $1",
            &[&email],
        ).await?;
        let professor_id: i32 = row.get(0);

        // Hash the password using PostgreSQL crypt function
        let password_hash: String = self.client.query_one(
            "SELECT crypt($1, gen_salt('bf'))",
            &[&password],
        ).await?.get(0);

        // Insert login information
        self.client.execute(
            "INSERT INTO professor_logins (professor_id, username, password_hash) VALUES ($1, $2, $3)",
            &[&professor_id, &username, &password_hash],
        ).await?;

        Ok(())
    }

    // Function to check user credentials
    pub async fn check_credentials(&self, username: &str, password: &str) -> Result<bool, Error> {
        let row = self.client.query_opt(
            "SELECT password_hash FROM professor_logins WHERE username = $1",
            &[&username],
        ).await?;

        if let Some(row) = row {
            let password_hash: String = row.get("password_hash");
            let valid: bool = self.client.query_one(
                "SELECT crypt($1, $2) = $2",
                &[&password, &password_hash],
            ).await?.get(0);
            Ok(valid)
        } else {
            Ok(false)
        }
    }
}
