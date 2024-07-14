#!/bin/bash

# Update package lists and install PostgreSQL
sudo apt update
sudo apt install -y postgresql postgresql-contrib

# Start PostgreSQL service
sudo systemctl start postgresql
sudo systemctl enable postgresql

# Get the current username
CURRENT_USER=$(whoami)

# Create a PostgreSQL role for the current user with superuser privileges
sudo -u postgres psql -c "CREATE ROLE $CURRENT_USER WITH SUPERUSER LOGIN;"

# Create a database with the same name as the current user
sudo -u postgres createdb "$CURRENT_USER"

# Optionally, set a password for the PostgreSQL role (replace 'your_password' with your desired password)
# Uncomment the following line if you want to set a password
#sudo -u postgres psql -c "ALTER ROLE $CURRENT_USER WITH PASSWORD 'test_password';"

# Print success message
echo "PostgreSQL installed and configured. The current user ($CURRENT_USER) has been granted superuser privileges."

# Optionally, you can test the setup by connecting to the PostgreSQL server
# Uncomment the following line to connect to PostgreSQL
# psql

# Setup the database (see setup.sql to modify the db)
echo "Setting up the PostgreSQL database according to the setup.sql file."
psql -f setup.sql

# Finish
echo "Database configured successfully."
