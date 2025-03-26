# Query Builder

The Query Builder is a powerful tool for constructing database queries in a type-safe and fluent manner. It's automatically generated for models marked with the `#[api_model]` attribute when the `server` feature is enabled. The builder provides a chainable interface for constructing complex queries with conditions, ordering, and other database operations.

## Getting Started

To use the Query Builder for a model, first ensure your model is properly annotated:

```rust
#[api_model(base = "/v1/endpoint", table = my_models)]
struct MyModel {
    id: u64,
    name: String,
    is_active: bool,
    created_at: DateTime<Utc>,
}
```

Then you can access the query builder through the model's query_builder() method:

```rust
let query = MyModel::query_builder();
```

## Building Queries

### Basic Query Construction

The Query Builder supports a fluent interface for constructing queries:

```rust
let query = MyModel::query_builder()
    .name_equals("John Doe".to_string())
    .is_active_is_true()
    .order_by_created_at_desc();
```

### Condition Types

The builder generates different condition methods based on the field types:

### String Fields

- For string fields (like `name` in our example), the following methods are generated:

- `{field}_equals(value: String)` - Exact match

- `{field}_not_equals(value: String)` - Not equal

- `{field}_contains(value: String)` - Contains substring

- `{field}_not_contains(value: String)` - Does not contain substring

- `{field}_starts_with(value: String)` - Starts with

- `{field}_not_starts_with(value: String)` - Does not start with

- `{field}_ends_with(value: String)` - Ends with

- `{field}_not_ends_with(value: String)` - Does not end with

### Example: 
```rust
query.name_contains("John") // Finds records where name contains "John"
    .name_not_equals("Admin"); // And name is not exactly "Admin"
```

### Boolean Fields

For boolean fields (like `is_active`):

- `{field}_is_true()` - Field is true

- `{field}_is_false()` - Field is false

### Example: 
```rust
query.is_active_is_true(); // Only active records
```

### Ordering Results

You can order results using the generated ordering methods:

- `order_by_{field}_asc()` - Ascending order

- `order_by_{field}_desc()` - Descending order

Multiple ordering conditions can be chained:
```rust
query.order_by_created_at_desc()
    .order_by_name_asc(); // Primary sort by created_at (newest first), secondary by name
```

## Executing Queries

After building a query, you typically execute it through the repository:
```rust
let results = MyModel::get_repository()
    .find_by_query(query)
    .await?;
```

## Advanced Usage

### Combining Conditions

Conditions are combined with AND logic by default. For more complex queries, you can build subqueries:

```rust
let active_users = MyModel::query_builder()
    .is_active_is_true();
    
let recent_active_users = active_users
    .created_at_greater_than(Utc::now() - Duration::days(30));
```

## Best Practices

1. Chain methods fluently for readability

2. Reuse query builders for similar queries

3. Use specific conditions rather than overly broad ones

4. Consider indexing for frequently queried fields

5. Log queries during development for debugging

The Query Builder provides a balance between type safety and flexibility, making it easier to construct database queries while reducing the risk of SQL injection and runtime errors.
