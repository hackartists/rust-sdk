# Package Structure

This document outlines the standardized directory structure for the **Common Package** within Biyard Rust projects. It provides shared components such as Data Transfer Objects (DTOs), database models, utilities, and error definitions used consistently throughout the workspace.

## Directory Structure

The `common` package structure is as follows:

```plaintext
common
├── lib.rs
├── error.rs
├── dto
│   ├── mod.rs
│   └── v1
│       ├── users.rs
│       └── users
│           └── codes.rs
├── tables
│   ├── mod.rs
│   ├── users
│   │   ├── mod.rs
│   │   └── user.rs
│   └── products
│       ├── mod.rs
│       └── product.rs
└── utils
```

## Explanation of Structure

### Root Files

- **`lib.rs`**  
  Serves as the entry point, defining module exports and public interfaces for the common crate.

- **`error.rs`**  
  Defines common error types and shared error-handling utilities.

### `dto` (Data Transfer Objects)

- DTO definitions are organized according to their corresponding **API paths**, reflecting API endpoints clearly.
- The directory hierarchy directly matches the API structure to maintain consistency and clarity.

**Example:**

If the API endpoint is defined as:
```
GET /v1/users/codes
```

Then the DTO path should be:
```plaintext
dto/v1/users/codes.rs
```

### `tables` (Database Models)

- Structured by database tables, each directory containing a `mod.rs` to export models clearly.
- Individual database models are organized by table name.

**Example structure:**
```plaintext
tables/users/mod.rs     // Module exports for user models
tables/users/user.rs    // Definition of the User model
```

### `utils` (Utility Functions)

- Contains common helper functions such as data validation, parsing, date/time utilities, and general-purpose tools.

**Example structure:**
```plaintext
utils/time.rs           // Utilities for time-related functions
utils/validation.rs     // Common validation functions
```
