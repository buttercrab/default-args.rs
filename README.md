# Default Args

**work in progress**

## Proposed

```rust
// this would make a macro named `foo`
// and original function named `foo_`
default_args! {
    #[some_attribute]
    fn foo(important_arg: u32, optional: u32 = 100) -> String {
        ...
    }
}

// in other codes...
foo!(1); // foo(1, 100)
foo!(1, 3); // foo(1, 3)
foo!(1, optional=5); // foo(1, 5)
foo!(1, optional = 10); // foo(1, 10)
```