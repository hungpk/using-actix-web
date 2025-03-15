# using-actix-web: A Rust Web Application Example

This project demonstrates a basic web application built using the Actix Web framework in Rust. It showcases key concepts in building RESTful APIs, including:

**Core Technologies:**

*   **Actix Web:** A powerful, performant, and ergonomic web framework for Rust.
*   **SQLx:** A pure Rust SQL toolkit that provides compile-time checked SQL queries.
*   **PostgreSQL:** A robust, open-source relational database used for data persistence.
*   **Bcrypt:** A secure password hashing algorithm for storing user credentials.
*   **JSON Web Tokens (JWT):** A standard for securely transmitting information between parties.
*   **Redis:** An in-memory data structure store, used as a cache or message broker (configured, but not yet implemented in the current code).
* **devenv**: Development environment configuration for the project

**Key Features and Concepts Demonstrated:**

1.  **RESTful API Design:**
    *   The project defines RESTful endpoints for user management:
        *   `POST /users`: Creates a new user.
        *   `GET /users/{id}`: Retrieves a user by ID.

2.  **Database Integration (SQLx):**
    *   **Database Migrations:** Uses SQLx migrations to manage the database schema (see `migrations` directory), defining a `users` table.
    *   **Data Modeling:** Defines the `User` entity (`src/user/entity.rs`) as a Rust struct that maps to the database table.
    *   **Data Access:** Implements data access logic in `src/user/repos.rs`, showing how to:
        *   Create a new user with password hashing.
        *   Find a user by ID.
        * Find a user by email.
    * Compile time sql checking

3.  **User Management:**
    *   **User Creation:**  Handles creating new users, including:
        *   Input validation (basic field existence, but could be expanded).
        *   Password hashing using `bcrypt`.
        *   Database insertion with SQLx.
        * Error Handling for user already exist.
    *   **User Retrieval:** Handles fetching users by ID.
    *   **User Data Formatting:**  Uses `UserResponse` to shape data returned to the client.

4.  **Password Hashing (Bcrypt):**
    *   Demonstrates secure password storage by hashing user passwords before saving them to the database.

5.  **JSON Web Tokens (JWT):**
    *   **Token Creation:** Shows how to create JWTs (in `src/auth/entity.rs`) to represent authenticated users.
    *   **Token Decoding:** Shows how to decode and validate JWTs.
    *   **Secret Management:**  Demonstrates fetching the JWT secret from environment variables (or a default value).
    *   **Error Handling**: Handles the case of token expired, invalid token or invalid signature.

6.  **Dependency Injection:**
    *   The Actix Web framework is used with `web::Data` to inject the database connection pool (`PgPool`) into request handlers.

7.  **Environment Variables:**
    *   Demonstrates the use of environment variables to configure:
        *   Database connection details (`DATABASE_URL`, `TEST_DATABASE_URL`).
        *   JWT Secret (`JWT_SECRET`).

8.  **Error Handling:**
    *   Uses `Result` to handle potential errors from:
        *   Database operations.
        *   JWT encoding/decoding.
        *   Password hashing.
    * Return error as http response.

9. **Testing**:
    * demonstrates how to create unit test for the project.
    * Demonstrate how to test method from the same struct.
    * demonstrate how to check for the right error.

10. **Concurrency**:
    * Demonstrates the concurrent access using the lock.

11. **Dev environment**:
    * The `devenv.nix` file provide an easy way to setup the development environment.
    * The database, redis are started when using `devenv shell`

**Project Structure:**

*   **`src/`:**
    *   **`main.rs`:** The entry point of the application.
    *   **`app/`**: module for app global data.
    *   **`user/`:**
        *   **`entity.rs`:** Defines the `User`, `CreateUserPayload` and `UserResponse` structs.
        *   **`repos.rs`:** Contains the database logic (create, find, find by email).
        *   **`handler.rs`:** Defines the Actix Web route handlers.
        *   **`service.rs`**: Contains the business logic for users.
    *   **`auth/`:**
        *   **`entity.rs`:** Defines JWT logic.
    *   **`lib.rs`**: contains reusable code, not used in this project yet.
*   **`migrations/`:** SQL files for managing the database schema.
*   **`Cargo.toml`:** Rust project configuration and dependencies.
*   **`devenv.nix`:** Development environment configuration for the project
*   **`README.md`**: this file.

**Getting Started (Development with devenv):**

1.  **Install devenv:** Follow the instructions at [https://devenv.sh/getting-started/](https://devenv.sh/getting-started/) to install `devenv`.
2.  **Start environment**: use the following command to start the dev environment
    ```bash
    devenv shell
    ```
    it will install all dependancies and start postgres, redis.
3.  **Run the Migrations:**
    ```bash
    sqlx migrate run
    ```
    This creates the `users` table in your database.
4.  **Run the Application:**
    ```bash
    cargo run
    ```
    This starts the Actix Web server.
5.  **Running Tests**:
    ```bash
    cargo test
    ```
    this run all the tests in the project.

**Next Steps (Potential Improvements):**

*   **Authentication Middleware:** Implement proper authentication middleware to protect routes.
*   **Authorization:** Add role-based access control (RBAC) for managing permissions.
*   **Redis Integration:** Use Redis for caching or session management.
*   **Input Validation:** Add more robust validation for user input.
*   **More API Endpoints:** Implement more endpoints (e.g., update user, delete user).
*   **More Comprehensive Testing:** Add integration tests.
* **Dockerisation**: dockerize the project.
* **Logging**: add logs.

**Conclusion:**

This project provides a solid foundation for building more complex web applications with Rust, Actix Web, and SQLx. It demonstrates many best practices and is a great starting point for learning these technologies.
PS: This document is generated by AI
