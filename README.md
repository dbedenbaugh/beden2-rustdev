# Rust Web Example
Programmer: Devon Bedenbaugh, beden2@pdx.edu
Homework repository for CS410 rust web development. 
Assignments will be stored here for grading purposes.  

Recent notes:

Most folders are just for learning purposes and some don't even run properly.
The final_project folder contains my most up to date CRUD api. It's memory persists
past program termination and is stored in a postgres server. A proper custom error handler
is currently being implemented and the next step is a more proper frontend. 

The API for censoring bad words has been implemented and should censor 
bad words when adding question content to the database.  

### IMPORTANT
# The current implementation of final_project is having build issues last minute
To run this program, you will want to use 
   ```sh
   trunk serve
   ```
to run the frontend in addition to the instructions below. 
## Installation

1. **Install PostgreSQL Server:**
   Install a PostgreSQL server on your system. You can use your package manager to install PostgreSQL. For example, on Debian-based systems, you can run:
   ```sh
   sudo apt install postgresql
    ```
2. **Install Rust**
   You can install Rust using rustup. Follow the instructions on the Rust website or use the following command:
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
3. **Clone and Build the Program:**
  Clone the repository and build the program with Cargo
  ```sh
  git clone https://github.com/dbedenbaugh/beden2-rustdev/final_project
  cd /final_project
  cargo build --release
  ```
4. **Configure the Database URL:***
  Create a .env file in the directory containing Cargo.toml and set your database URL to your PostgreSQL server URL:
  ```sh
  echo "DATABASE_URL=postgres://postgres:Password@localhost/your_database_here" > .env
  ```
**Usage**
  Once the dependencies are installed and the program is built, you can run your server with the following command:
  ```sh
  cargo run
  ```
You should be greeted with a link to the local host port the program runs on by default. 
