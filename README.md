# Default Arguments in Rust

[![Github Actions](https://img.shields.io/github/workflow/status/buttercrab/default-args.rs/build?style=flat-square)](https://github.com/buttercrab/default-args.rs/actions/workflows/build.yml)
[![Github Releases](https://img.shields.io/github/v/release/buttercrab/default-args.rs?include_prereleases&style=flat-square)](https://github.com/buttercrab/default-args.rs/releases)
[![crate.io](https://img.shields.io/crates/v/default-args?style=flat-square)](https://crates.io/crates/default-args)
[![MIT License](https://img.shields.io/github/license/buttercrab/default-args.rs?style=flat-square)](https://github.com/buttercrab/default-args.rs/blob/master/LICENSE)

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

// in other codes ...
assert_eq!(foo!(1), "1, 100"); // foo(1, 100)
assert_eq!(foo!(1, 3), "1, 3"); // foo(1, 3)
assert_eq!(foo!(1, optional = 10), "1, 10"); // foo(1, 10)

// let's make another one
default_args! {
    #[inline]
    pub async unsafe extern "C" fn bar<S1, S2, S3>(a: S1, b: S2 = "b", c: S3 = "c") -> String
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
        S3: AsRef<str>,
    {
        format!("{}, {}, {}", a.as_ref(), b.as_ref(), c.as_ref())
    }
    // that was long signature!
}

// in other codes ...
assert_eq!(unsafe { bar!("a") }.await, "a, b, c");
assert_eq!(unsafe { bar!("a", "d") }.await, "a, d, c");
// you can even mix named & unnamed argument in optional arguments
assert_eq!(unsafe { bar!("a", "d", c = "e") }.await, "a, d, e");
assert_eq!(unsafe { bar!("a", c = "e") }.await, "a, b, e");
```

See [examples](https://github.com/buttercrab/default-args.rs/tree/master/examples) for more information.

## More Features

### Export

Add export in the front of the function and the macro would be exported.
*(add pub to export function with macro)*

```rust
default_args! {
    export pub fn foo() {}
}
```

Above macro will expand as below

```rust
pub fn foo_() {}

#[macro_export]
macro_rules! foo { () => {}; }
```

### Path of function

Macro just call the function in name, so you should import both macro and the function to use it. By writing the path of
this function, you can just only import the macro.
*(path should start with `crate`)*

```rust
#[macro_use]
pub mod foo {
    default_args! {
        pub fn crate::foo::bar() {}
    }
}

// then it would create `bar!()`
bar!();
```

Above macro would expand as below

```rust
pub mod foo {
    pub fn bar_() {}

    macro_rules! bar {
        () => {
            $crate::foo::bar_()
        };
    }
}
```

#### *Why do we have to write module?*

> `std::module_path!` can resolve the module path of the function where it is declared.
> However, it can be resolved in runtime, not compile-time.
> I couldn't find a way to get module path in compile-time.

## License

[MIT License](https://github.com/buttercrab/default-args.rs/blob/master/LICENSE)