# Axum + Postgres Application Boilerplate

This repository contains an application template built using [Axum](https://github.com/tokio-rs/axum) and [PostgreSQL](https://www.postgresql.org/). It serves as a starting point for creating a new Axum server.

<center>GraphQL</center>

![graphql](./docs/images/graphql.png)

<center>Swagger</center>

![swagger](./docs/images/swagger.png)

![wrk](./docs/images/wrk.png)

The full list of crates used can be found in the [Cargo.toml](./Cargo.toml) file. However, here are some key ones:

- [Axum](https://github.com/tokio-rs/axum) - A user-friendly, modular web framework built with Tokio, Tower, and Hyper.
- [Sea-ORM](https://github.com/SeaQL/sea-orm) - An async & dynamic ORM for Rust, supporting PostgreSQL, MySQL, SQLite, and MSSQL.
- [Tracing](https://github.com/tokio-rs/tracing) - A framework for instrumenting Rust programs to collect structured, event-based diagnostic information.
- [Chrono](https://github.com/chronotope/chrono) - A comprehensive Date and Time library for Rust.
- [Serde](https://serde.rs/) - A framework for efficiently and generically serializing and deserializing Rust data structures.
- [Uuid](https://github.com/uuid-rs/uuid) - A library for generating and parsing UUIDs.

## Table of Contents

- [Features](#features)
- [Project Structure](#project-structure)
  - [Source Code](#source-code-src)
    - [Common Utilities](#common-utilities-srccommon)
    - [Database](#database-srcdatabase)
    - [Modules](#modules-srcmodules)
    - [Core Files](#core-files)
  - [Configuration and Build Files](#configuration-and-build-files)
  - [Documentation](#documentation)
- [Getting Started](#getting-started)
  - [Clone this Repository](#clone-this-repository)
  - [Run Postgres](#run-postgres)
  - [Configure the Application](#configure-the-application)
  - [Set Up the Application Database](#set-up-the-application-database)
  - [Starting the Application](#starting-the-application)
  - [Autoreloading](#autoreloading)
- [Running with Docker Compose](#running-with-docker-compose)
- [Production Build](#production-build)

## Features

- **Project Structure**

  - [x] Modular Axum + PostgreSQL setup
  - [x] Environment-based configuration
  - [x] Docker support

- **Database**

  - [x] Sea-ORM integration
  - [x] Database migrations

- **API**

  - [x] REST endpoints
  - [x] GraphQL support
  - [x] API versioning
  - [x] OpenAPI/Swagger documentation

- **Authentication & Authorization**

  - [x] JWT authentication
  - [x] Role-based access control (RBAC)

- **Error Handling & Logging**

  - [x] Centralized error handling
  - [x] Logging with Tracing

- **Testing**

  - [x] Unit tests
  - [ ] Integration tests
  - [ ] API tests

- **Security**

  - [ ] Rate limiting
  - [ ] CORS configuration
  - [ ] Input validation

- **Monitoring & Observability**

  - [ ] Metrics collection
  - [ ] Health checks
  - [ ] Performance monitoring

- **Developer Experience**
  - [ ] Code generation tools
  - [ ] Development scripts
  - [ ] CI/CD pipeline configuration

## Project Structure

```sh
.
├── src/                  # Source code
│   ├── common/           # Common utilities and shared code
│   │   ├── utils/        # Utility functions and helpers
│   │   ├── cfg.rs        # Configuration management
│   │   ├── middleware.rs # Custom middleware implementations
│   │   ├── api_error.rs  # Error handling and custom error types
│   │   └── telemetry.rs  # Logging and observability setup
│   │
│   ├── database/         # Database configuration and migrations
│   │   ├── migrations/   # Database migration files
│   │   └── mod.rs        # Database connection and setup
│   │
│   ├── modules/          # Application modules and features
│   │   ├── auth/         # Authentication and authorization
│   │   ├── health/       # Health check endpoints
│   │   ├── users/        # User management
│   │   └── mod.rs        # Module registration and exports
│   │
│   ├── app.rs            # Application setup and configuration
│   ├── doc.rs            # API documentation setup
│   ├── lib.rs            # Library entry point
│   ├── main.rs           # Application entry point
│   └── query_root.rs     # GraphQL query root definitions
│
├── docs/                 # Documentation files
│   └── images/           # Documentation images and diagrams
│
├── Cargo.toml            # Project dependencies and metadata
├── docker-compose.yml    # Docker Compose configuration
├── Dockerfile            # Docker build instructions
└── .env.sample           # Sample environment variables
```

Each directory and file serves a specific purpose:

### Source Code (`src/`)

#### Common Utilities (`src/common/`)

- `utils/`: Reusable helper functions and utilities
- `cfg.rs`: Environment configuration and settings management
- `middleware.rs`: Custom middleware for request processing
- `api_error.rs`: Centralized error handling and custom error types
- `telemetry.rs`: Logging, tracing, and observability setup

#### Database (`src/database/`)

- `migrations/`: Database schema migration files
- `mod.rs`: Database connection pool and configuration

#### Modules (`src/modules/`)

- `auth/`: Authentication and authorization logic
  ```sh
  auth/
  ├── controller.rs         # Authentication endpoints and handlers
  ├── service.rs            # Authentication business logic
  ├── mod.rs                # Module exports and route registration
  ├── dto/                  # Data Transfer Objects
  │   └── mod.rs            # Auth request/response structures
  └── guards/               # Authentication guards
      ├── auth_guard.rs     # JWT authentication guard
      ├── admin_guard.rs    # Admin role guard
      ├── graphql_guards.rs # GraphQL-specific guards
      └── mod.rs            # Guard exports
  ```
- `health/`: Health check endpoints and monitoring
- `users/`: User management and related functionality
  ```sh
  users/
  ├── controller.rs      # HTTP request handlers and route definitions
  ├── service.rs         # Business logic and data operations
  ├── mod.rs             # Module exports and route registration
  ├── dto/               # Data Transfer Objects
  │   └── mod.rs         # Request/Response data structures
  ├── entities/          # Database entity definitions
  │   └── mod.rs         # User entity and related models
  └── enums/             # User-related enumerations
      ├── mod.rs         # Enum exports
      ├── user_role.rs   # User role definitions
      └── user_status.rs # User status definitions
  ```
- `mod.rs`: Module registration and exports

#### Core Files

- `app.rs`: Main application setup, middleware, and route configuration
- `doc.rs`: OpenAPI/Swagger documentation setup
- `lib.rs`: Library code and public API
- `main.rs`: Application entry point and server startup
- `query_root.rs`: GraphQL schema and resolver definitions

### Configuration and Build Files

- `Cargo.toml`: Project dependencies and metadata
- `docker-compose.yml`: Docker Compose configuration for development
- `Dockerfile`: Docker build instructions
- `.env.sample`: Sample environment variables template

### Documentation

- `docs/`: Project documentation
  - `images/`: Documentation images and diagrams

## Getting Started

To begin with this project:

### Clone this Repository

```shell
$ git clone https://github.com/nakamuraos/axum-postgres-boilerplate
$ cd axum-postgres-boilerplate
```

### Run Postgres

The most straightforward way to run Postgres is by using a container with a pre-built image. The command below will start latest version of Postgres using [Docker](https://www.docker.com/):

```shell
$ docker run -d -p 5432:5432 -e POSTGRES_PASSWORD=password postgres
```

### Configure the Application

The backend application is preferably configured via environment variables. To simplify the process during development, we can use `.env` files to avoid defining the variables each time. As a starting point, you can simply copy the sample `.env` file in this repo and modify the `.env` file as per the comments therein.

```shell
$ cp .env.sample .env
```

### Starting the Application

With everything else set up, all you need to do now is:

```shell
$ cargo run
```

- The application will be available at http://localhost:8080
  - Swagger: http://localhost:8080/docs
  - GraphQL: http://localhost:8080/graphql

### Autoreloading

To start the server and autoreload on code changes:

```shell
$ cargo install cargo-watch
$ cargo watch -q -x run
```

To format `.json` logs using [`jq`](https://github.com/jqlang/jq):

```shell
$ cargo watch -q -x run | jq .
```

## Running with Docker Compose

This project includes Docker Compose configuration for easy development and deployment. To run the application using Docker Compose:

1. Make sure you have Docker and Docker Compose installed on your system.

2. Copy the sample environment file:

```shell
$ cp .env.sample .env
```

3. Start the application and its dependencies (PostgreSQL):

```shell
$ docker-compose up
```

To run in detached mode (in the background):

```shell
$ docker-compose up -d
```

To stop the application:

```shell
$ docker-compose down
```

The application will be available at `http://localhost:8080`, and PostgreSQL will be accessible on port 5432.

## Production Build

### Building for Production

To create an optimized production build:

```shell
$ cargo build --release
```

The optimized binary will be available at `target/release/server`.

### Production Deployment

#### Using Docker

1. Build the production Docker image:

```shell
$ docker build -t axum-postgres-boilerplate:prod .
```

2. Run the container:

```shell
$ docker run -d \
  --name axum-app \
  -p 8080:8080 \
  -v $(pwd)/.env:/app/.env \
  axum-postgres-boilerplate:prod
```

#### Manual Deployment

1. Copy the release binary to your production server:

```shell
$ scp target/release/server user@your-server:/path/to/deployment
```

2. Copy your production `.env` file:

```shell
$ scp .env user@your-server:/path/to/deployment
```

3. Run the application:

```shell
$ ./server
```

### Production Considerations

- Use a reverse proxy (like Nginx) in front of your application
- Set up proper SSL/TLS certificates
- Use a production-grade PostgreSQL setup

## Contributing

Contributions are always welcome! Feel free to check the current issues in this repository for tasks that need attention. If you find something missing or that could be improved, please open a new issue.
