# Postgres + Rust Tokio Postgres

```shell
apt install postgresql postgresql-contrib -y
```

### Notes
The crypt function is used to hash passwords with the Blowfish algorithm (bf).
Ensure the PostgreSQL pgcrypto extension is installed and enabled to use the crypt function:

```shell
psql -U postgres -d website_db -c "CREATE EXTENSION IF NOT EXISTS pgcrypto;"
```

### Run the Script:
#### To add a single professor:

```shell
./add_professors.sh first_name last_name email department username password
```

#### To add multiple professors from a CSV file:

```shell
./add_professors.sh -f path/to/professors.csv
```

CSV File Format
The CSV file should have the following format:

```csv
first_name,last_name,email,department,username,password
John,Doe,john.doe@example.com,Computer Science,jdoe,temp_password123
Jane,Smith,jane.smith@example.com,Mathematics,jsmith,temp_password456
Dependencies
```

This script uses the psql command-line utility to interact with the PostgreSQL database. Ensure you have the psql utility installed and configured to connect to your PostgreSQL instance.

## TODO: security

