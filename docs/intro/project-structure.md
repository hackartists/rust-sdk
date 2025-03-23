# Project Structure

This section describes the standard package structure for Biyard Rust projects, providing clarity on the organization and purpose of each directory and file.

## Basic Project Structure

The basic structure of a Biyard Rust project is organized as follows:

```plaintext
.
├── Cargo.toml
├── CHANGELOG.md
├── CODE_OF_CONDUCT.md
├── CODEOWNERS
├── CONTRIBUTING.md
├── Makefile
├── packages
│   ├── common
│   ├── api
│   └── www
├── README.md
```

Explanation of Directories

Root Directory
- Cargo.toml: Defines Rust project dependencies and workspace configuration.
- CHANGELOG.md: Documents changes and updates to the project.
- CODE_OF_CONDUCT.md, CODEOWNERS, CONTRIBUTING.md: Guidelines and instructions for contributing and managing community interactions.
- Makefile: Contains build tasks and command shortcuts to streamline development processes.

packages
- common: Shared utilities, models, data transfer objects (DTOs), validation logic, and common infrastructure used across various subdomains.
- api: Contains backend API implementations and related server-side logic specific to the API subdomain.
- www: Frontend web interface, UI components, and related assets specific to the web subdomain.

This standardized structure ensures clear separation of concerns, maintainability, and ease of collaboration across Biyard Rust projects.
- For example, if the project is desired for `example.com`, `packages/api` should be implemented for `api.example.com`.
- Similarly, `packages/www` indicates implementation of `www.example.com` and `example.com`.

## Cargo.toml
The root `Cargo.toml` file defines the Rust workspace configuration and global dependencies used throughout the project. Below is a typical example:

```toml
[workspace]
members = ["packages/*"]
resolver = "2"
exclude = ["deps"]

[workspace.package]
authors = ["Biyard"]
description = "Ratel"
edition = "2024"
repository = "https://github.com/biyard/service-repo"
license = "MIT"
description = "Project descrption"

[workspace.dependencies]
bdk = { path = "deps/rust-sdk/packages/bdk" }

sqlx = { version = "0.8.3", features = [
    "sqlite",
    "postgres",
    "runtime-tokio",
    "time",
    "bigdecimal",
] }

```

### Key Dependencies
- **bdk**: A crucial crate within the Biyard ecosystem, which encapsulates essential tools and libraries such as `dioxus`, `axum`, and other common utilities. Leveraging bdk promotes consistency, simplifies maintenance, and accelerates development by providing a pre-configured stack of widely-used Rust crates tailored specifically for Biyard projects.
- **sqlx**: Provides robust, type-safe interactions with databases, configured to support both SQLite and PostgreSQL with asynchronous runtime provided by Tokio.

This standardized structure ensures clear separation of concerns, maintainability, and ease of collaboration across Biyard Rust projects.
