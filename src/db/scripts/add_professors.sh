#!/bin/bash

# Function to add a single professor
add_professor() {
    local current_user
    current_user=$(whoami)

    local first_name=$1
    local last_name=$2
    local email=$3
    local department=$4
    local username=$5
    local password=$6

    # Insert professor into the database
    psql -U "$current_user" -d timetable_generator_db -c "INSERT INTO professors (first_name, last_name, email, department) VALUES ('$first_name', '$last_name', '$email', '$department');"

    # Get the professor_id of the newly added professor
    professor_id=$(psql -U "$current_user" -d timetable_generator_db -t -c "SELECT professor_id FROM professors WHERE email = '$email';")

    # Insert login information with temporary password
    psql -U "$current_user" -d timetable_generator_db -c "INSERT INTO professor_logins (professor_id, username, password_hash) VALUES ($professor_id, '$username', crypt('$password', gen_salt('bf')));"
}

# Function to add professors from a CSV file
add_professors_from_csv() {
    local csv_file=$1
    while IFS=, read -r first_name last_name email department username password; do
        add_professor "$first_name" "$last_name" "$email" "$department" "$username" "$password"
    done < "$csv_file"
}

# Main script logic
if [ "$1" == "-f" ]; then
    if [ -z "$2" ]; then
        echo "Please provide the path to the CSV file."
        exit 1
    fi
    add_professors_from_csv "$2"
else
    if [ $# -ne 6 ]; then
        echo "Usage: $0 first_name last_name email department username password"
        echo "       $0 -f path/to/professors.csv"
        exit 1
    fi
    add_professor "$1" "$2" "$3" "$4" "$5" "$6"
fi

echo "Professors added successfully."
