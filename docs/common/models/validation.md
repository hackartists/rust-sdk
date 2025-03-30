# Validation

This document explains how to define and implement validation for fields in Biyard Rust projects using the `api_model` macro in combination with the `validator` crate.

## Overview

Field validation ensures data integrity by verifying that data meets certain criteria before being processed or stored. Validation attributes defined using the `validator` crate automatically integrate with generated models.

## Defining Validation

Validation is defined directly within the data model structs using attributes provided by the [validator](https://docs.rs/validator/latest/validator/).

### Example

Here's a practical example demonstrating field validation on a `User` model:

```rust
use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v1/", table = users)]
pub struct User {
    #[api_model(primary_key)]
    pub id: i64,

    #[validate(custom(function = "validate_nickname"))]
    pub nickname: String,

    #[validate(email)]
    pub email: String,

    #[validate(url)]
    pub profile_url: String,
}
```

## Supported Validation Attributes

Use validation attributes provided by the `validator` crate:

| Validator Type            | Description                           | Example                                 |
|---------------------------|---------------------------------------|-----------------------------------------|
| `email`                   | Validates that a field is a valid email | `#[validate(email)]`                    |
| `url`                     | Validates that a field is a valid URL  | `#[validate(url)]`                      |
| `length`                  | Validates field length                 | `#[validate(length(min = 1, max = 20))]`|
| `range`                   | Validates numeric ranges               | `#[validate(range(min = 1, max = 100))]`|
| `custom`                  | Validates using a custom function      | `#[validate(custom(function="validate_nickname"))]`|

### Custom Validation Example

Custom validation allows you to write specific validation logic:

```rust
use validator::ValidationError;

fn validate_nickname(nickname: &str) -> Result<(), ValidationError> {
    if nickname.len() < 3 {
        return Err(ValidationError::new("nickname too short"));
    }
    Ok(())
}
```

## Automatic Inclusion in Generated Models

When defining validations with `validator`, generated request and response structures automatically include validation checks:

```rust
#[tokio::main]
async fn signup_user(user: User) -> Result<()> {
    user.validate()?;
    // Proceed if validation succeeds
    Ok(())
}
```

## Recommendations and Best Practices

- Define clear and concise validation rules.
- Always use specific validators rather than overly generic ones for clearer error messages.
- Leverage custom validations for complex validation logic.

Following these guidelines ensures robust and maintainable data validation in your Biyard Rust projects.
