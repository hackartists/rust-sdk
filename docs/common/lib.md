# `lib.rs` Structure

The `lib.rs` file serves as the entry point and central module definition for the **Common Package** within Biyard Rust projects. It organizes module visibility, re-exports commonly used types, and simplifies access to shared functionality throughout the workspace.

## Example Template

```rust
// packages/common/lib.rs
pub mod dto;
pub mod tables;
pub mod utils;
pub mod error;

pub mod prelude {
    pub use crate::dto::*;
    pub use crate::tables::*;
    pub use crate::utils::*;
    pub use crate::error::*;
}

pub type Result<T> = std::result::Result<T, crate::error::Error>;
```

## Explanation of Components

### Module Exports

The following modules are declared publicly to enable their usage across the workspace:

- **`dto`**: Contains Data Transfer Objects structured according to API paths.
- **`tables`**: Database models organized by table names.
- **`utils`**: Shared utility functions used throughout the workspace.
- **`error`**: Centralized error definitions and handling mechanisms.

### `prelude` Module

The `prelude` module simplifies imports for commonly used components from the common package. By re-exporting essential types and utilities, developers can streamline imports significantly.

**Usage Example**:

Instead of multiple imports:

```rust
use common::dto::*;
use common::tables::*;
use common::utils::*;
use common::error::*;
```

Simply use:

```rust
use common::prelude::*;
```

This improves readability and reduces boilerplate in application code.

### Type Alias (`Result<T>`)

The custom type alias:

```rust
pub type Result<T> = std::result::Result<T, crate::error::Error>;
```

provides a convenient and consistent way to handle results throughout the project. It ensures uniform error handling and clearly communicates potential errors from functions.

**Example Usage**:

```rust
use common::prelude::*;

pub async fn fetch_user(user_id: i32) -> Result<User> {
    // implementation logic...
}
```

This approach streamlines error handling by clearly defining the expected error type for all operations within the workspace.
