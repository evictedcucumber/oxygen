# How to Contribute

## Styling

### Names

| Style | Applied To |
| :---: | :---: |
| `snake_case` | Variables ; Members ; Functions |
| `ALL_CAPS` | Constants |
| `PascalCase` | Structs ; Traits |

### Documentation

Ensure functions have atleast one `# example` section and atleast a single description line.
Add a `# Errors` section to indicate that a function may return an error

```rust
/// Adds two [`i32`] integers together and return an [`i32`] result.
///
/// # Examples
/// ```
/// let result: i32 = add(1, 2);
/// assert_eq(result, 3);
/// ```
fn add (x: i32, y: i32) -> i32 {
    x + y
}
```

```rust
/// Accepts two [`String`] strings, converts them to an [`i32`] and returns the result of adding them as an [`i32`].
///
/// # Examples
/// ```
/// let result: Result<i32, i32::Err> = add("1".to_string(), "2".to_string());
/// assert_eq(result.unwrap(), 3);
///
/// let result2: Result<i32, i32::Err> = add("a".to_string(), "2".to_string());
/// assert(result2.is_err());
/// ```
fn add (x: String, y: String) -> Result<i32, i32::Err> {
    let x = x.parse::<i32>()?;
    let y = y.parse::<i32>()?;

    x + y
}
