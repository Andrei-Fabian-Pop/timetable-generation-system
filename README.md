# Project Overview
Welcome to the Timetable Generator! This project was born out of a need to simplify the complex task of creating timetables for educational institutions. Scheduling classes, managing professor availability, and ensuring optimal time slots can be a daunting task. To address this, I decided to build a robust and efficient timetable generator that leverages modern technologies and algorithms.

## Why I Built This
Creating a timetable manually is a tedious and error-prone process. Here’s why I thought a timetable generator with a server component would be a great solution:

- Ease of Use for Professors: Professors can easily input their available times, reducing the back-and-forth communication usually required to gather this information.
- Efficient Scheduling: By using a genetic algorithm, the system can quickly generate optimized timetables, saving a lot of manual effort and reducing the likelihood of conflicts.
- Scalability and Performance: The use of threading allows the algorithm to run multiple calculations simultaneously, speeding up the timetable generation process.
- Reliable Data Storage: PostgreSQL serves as the database, ensuring that all data related to professor availability and timetables is stored securely and can be accessed efficiently.

This project combines these elements to create a powerful tool that streamlines the process of generating timetables, ultimately saving time and improving accuracy for educational institutions.

## How to Use
To get started with the Timetable Generator, follow these steps:

- `Configuration`
In the root of the project, you'll find a file named config.toml. This file contains important settings for the database and server. Open this file and edit it with your specific information.
- `Setting Up the Server`
Navigate to the server folder. This folder contains the asynchronous server implementation along with controllers for handling the web pages.
- `Configuring the Timetable Generator`
The timetable generator is located in the generator folder. It uses a threaded genetic algorithm to generate the timetable. You can configure the generator by editing the `configuration.rs` file in the generator folder. This file allows you to tweak various parameters of the algorithm.
- `Database Setup`
The database folder contains a README file with detailed instructions on how to install and configure PostgreSQL for this project. Make sure to follow these instructions carefully to set up your database correctly.

## Contributing
Contributions are welcomed to the Timetable Generator project! Whether you're fixing bugs, adding new features, or improving documentation, your help is appreciated.