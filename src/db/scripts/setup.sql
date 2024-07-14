-- Create the database
CREATE DATABASE timetable_generator_db;

-- Connect to the database
\c timetable_generator_db;

-- Ensure the PostgreSQL pgcrypto extension is installed and enabled to use the crypt function:
CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- Create the users (professors) table
CREATE TABLE professors (
    professor_id SERIAL PRIMARY KEY,
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    department VARCHAR(100),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create the login information table
CREATE TABLE professor_logins (
    login_id SERIAL PRIMARY KEY,
    professor_id INT NOT NULL,
    username VARCHAR(50) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (professor_id) REFERENCES professors(professor_id) ON DELETE CASCADE
);

-- Create the availability table
CREATE TABLE availability (
    availability_id SERIAL PRIMARY KEY,
    professor_id INT NOT NULL,
    day_of_week VARCHAR(10) NOT NULL,
    start_time TIME NOT NULL,
    end_time TIME NOT NULL,
    FOREIGN KEY (professor_id) REFERENCES professors(professor_id) ON DELETE CASCADE
);

-- Create the timetable table
CREATE TABLE timetables (
    timetable_id SERIAL PRIMARY KEY,
    professor_id INT NOT NULL,
    day_of_week VARCHAR(10) NOT NULL,
    start_time TIME NOT NULL,
    end_time TIME NOT NULL,
    room VARCHAR(50),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (professor_id) REFERENCES professors(professor_id) ON DELETE CASCADE
);

-- Optional: Insert sample data
--INSERT INTO professors (first_name, last_name, email, department)
--VALUES ('John', 'Doe', 'john.doe@example.com', 'Computer Science'),
--       ('Jane', 'Smith', 'jane.smith@example.com', 'Mathematics');
--
--INSERT INTO availability (professor_id, day_of_week, start_time, end_time)
--VALUES (1, 'Monday', '09:00:00', '11:00:00'),
--       (1, 'Wednesday', '10:00:00', '12:00:00'),
--       (2, 'Tuesday', '13:00:00', '15:00:00');

-- Optional: Insert sample data into professor_logins table
-- Note: Passwords should be hashed using a secure hashing algorithm (e.g. bcrypt or blowfish)
INSERT INTO professor_logins (professor_id, username, password_hash)
VALUES (1, 'jdoe', crypt('hashed_password_for_jdoe', gen_salt('bf'))),
       (2, 'jsmith', crypt('hashed_password_for_jsmith', gen_salt('bf')));

-- Create index for faster queries
CREATE INDEX idx_professors_email ON professors(email);
CREATE INDEX idx_professor_logins_username ON professor_logins(username);
CREATE INDEX idx_availability_professor_id ON availability(professor_id);
CREATE INDEX idx_timetables_professor_id ON timetables(professor_id);
