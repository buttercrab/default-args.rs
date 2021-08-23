#[cfg(test)]
mod complicated {
    use default_args::default_args;

    #[test]
    fn complicated_test() {
        default_args! {
            fn foo(a: u32, b: u32, c: u32 = 10, d: u32 = 11) -> u32 {
                a + b + c + d
            }
        }

        assert_eq!(foo!(1, 2), 24);
        assert_eq!(foo!(1, 2, d = 0), 13);
        assert_eq!(foo!(1, 2, 3), 17);
        assert_eq!(foo!(1, 2, 3, 4), 10);
        assert_eq!(foo!(1, 2, d = 3, c = 4), 10);
        assert_eq!(foo!(1, 2, 3, d = 4), 10);
    }

    #[test]
    fn all_optional() {
        default_args! {
            fn foo(a: u32 = 10, b: u32 = 20, c: u32 = 30, d: u32 = 40) -> u32 {
                a + b + c + d
            }
        }

        assert_eq!(foo!(), 100);
        assert_eq!(foo!(1, c = 10, b = 10), 61);
        assert_eq!(foo!(1, 2, 3, 4), 10);
        assert_eq!(foo!(d = 10), 70);
    }

    #[test]
    fn generics_test() {
        default_args! {
            fn foo<T: AsRef<str>>(a: T = "hello") -> String {
                a.as_ref().to_string()
            }
        }

        assert_eq!(foo!(), "hello");
        assert_eq!(foo!("world"), "world");
        assert_eq!(foo!(a = "a"), "a");
        assert_eq!(foo!(a = String::from("abcd")), "abcd");
    }

    #[test]
    fn const_test() {
        default_args! {
            const fn foo(a: u32 = 0) -> u32 {
                a
            }
        }

        const A: u32 = foo!();
        const B: u32 = foo!(1);
        assert_eq!(A, 0);
        assert_eq!(B, 1);
    }

    #[test]
    fn unsafe_test() {
        default_args! {
            unsafe fn foo(a: u32 = 0) -> u32 {
                a
            }
        }

        assert_eq!(unsafe { foo!() }, 0);
        assert_eq!(unsafe { foo!(1) }, 1);
    }

    #[test]
    fn async_test() {
        default_args! {
            async fn foo(a: u32 = 0) -> u32 {
                a
            }
        }

        tokio_test::block_on(async {
            assert_eq!(foo!().await, 0);
            assert_eq!(foo!(1).await, 1);
        });
    }

    #[test]
    fn extern_test() {
        default_args! {
            pub extern "C" fn foo(a: u32 = 0) -> u32 {
                a
            }
        }

        assert_eq!(foo!(), 0);
        assert_eq!(foo!(1), 1);
    }
}
