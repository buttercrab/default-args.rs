# Default Args

[![Github Actions](https://img.shields.io/github/workflow/status/buttercrab/default-args.rs/build?style=flat-square)](https://github.com/buttercrab/default-args.rs/actions/workflows/build.yml)
[![CodeCov Badge](https://img.shields.io/codecov/c/github/buttercrab/default-args.rs?style=flat-square)](https://app.codecov.io/gh/buttercrab/default-args.rs)

Enables default arguments in rust by macro in zero cost. Just wrap function with `default_args!` and macro with name of
function would be automatically generated to be used with default argument. See below for usage

```rust
use default_args::default_args;

// this would make a macro named `foo`
// and original function named `foo_`
default_args! {
    fn foo(important_arg: u32, optional: u32 = 100) -> String {
        format!("{}, {}", important_arg, optional)
    }
}

// in other codes...
assert_eq!(foo!(1), "1, 100"); // foo(1, 100)
assert_eq!(foo!(1, 3), "1, 3"); // foo(1, 3)
assert_eq!(foo!(1, optional=5), "1, 5"); // foo(1, 5)
assert_eq!(foo!(1, optional = 10), "1, 10"); // foo(1, 10)
```

## More Features

### Export

Add export in the front of the function and the macro would be exported.
*(add pub to export function with macro)*

```rust
default_args! {
    export pub fn foo() {}
}
```

### Path of function

Macro just call the function in name, so you should import both macro and the function to use it. By writing the path of
this function, you can just only import the macro.
*(path should start with `crate`)*

```rust
mod foo {
    default_args! {
        fn crate::foo::bar() {}
    }
}

// then it would create `bar!()`
use foo::bar;
bar!();
```

## License

[MIT License](https://github.com/buttercrab/default-args.rs/blob/master/LICENSE)