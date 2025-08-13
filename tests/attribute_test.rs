#[cfg(test)]
mod attribute_test {
    use default_args::default_args_attribute;

    #[test]
    fn basic_test() {
        #[default_args_attribute]
        fn foo() -> u32 {
            1
        }

        assert_eq!(foo!(), 1);
    }

    #[test]
    fn basic_test2() {
        #[default_args_attribute]
        fn foo(#[default(0)] a: u32) -> u32 {
            a
        }

        assert_eq!(foo!(), 0);
        assert_eq!(foo!(1), 1);
        assert_eq!(foo!(a = 1), 1);
    }

    #[test]
    fn basic_test3() {
        #[default_args_attribute]
        fn foo(a: u32, #[default(0)] b: u32) -> u32 {
            a + b
        }

        assert_eq!(foo!(1), 1);
        assert_eq!(foo!(1, 2), 3);
        assert_eq!(foo!(1, b = 2), 3);
    }

    #[test]
    fn complicated_test() {
        #[default_args_attribute]
        fn foo(a: u32, b: u32, #[default(10)] c: u32, #[default(11)] d: u32) -> u32 {
            a + b + c + d
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
        #[default_args_attribute]
        fn foo(#[default(10)] a: u32, #[default(20)] b: u32, #[default(30)] c: u32, #[default(40)] d: u32) -> u32 {
            a + b + c + d
        }

        assert_eq!(foo!(), 100);
        assert_eq!(foo!(1, c = 10, b = 10), 61);
        assert_eq!(foo!(1, 2, 3, 4), 10);
        assert_eq!(foo!(d = 10), 70);
    }

    #[test]
    fn generics_test() {
        #[default_args_attribute]
        fn foo<T: AsRef<str>>(#[default("hello")] a: T) -> String {
            a.as_ref().to_string()
        }

        assert_eq!(foo!(), "hello");
        assert_eq!(foo!("world"), "world");
        assert_eq!(foo!(a = "a"), "a");
        assert_eq!(foo!(a = String::from("abcd")), "abcd");
    }

    #[test]
    fn const_test() {
        #[default_args_attribute]
        const fn foo(#[default(0)] a: u32) -> u32 {
            a
        }

        const A: u32 = foo!();
        const B: u32 = foo!(1);
        assert_eq!(A, 0);
        assert_eq!(B, 1);
    }

    #[test]
    fn unsafe_test() {
        #[default_args_attribute]
        unsafe fn foo(#[default(0)] a: u32) -> u32 {
            a
        }

        assert_eq!(unsafe { foo!() }, 0);
        assert_eq!(unsafe { foo!(1) }, 1);
    }

    #[test]
    fn async_test() {
        #[default_args_attribute]
        async fn foo(#[default(0)] a: u32) -> u32 {
            a
        }

        tokio_test::block_on(async {
            assert_eq!(foo!().await, 0);
            assert_eq!(foo!(1).await, 1);
        });
    }

    #[test]
    fn extern_test() {
        #[default_args_attribute]
        pub extern "C" fn foo(#[default(0)] a: u32) -> u32 {
            a
        }

        assert_eq!(foo!(), 0);
        assert_eq!(foo!(1), 1);
    }

    #[test]
    fn expression_test() {
        const DEFAULT_A: u32 = 10;
        fn default_b() -> u32 {
            20
        }
        #[derive(PartialEq, Debug, Clone, Copy)]
        struct S {
            c: u32,
        }
        const DEFAULT_C: S = S { c: 30 };

        #[default_args_attribute]
        fn foo(#[default(DEFAULT_A)] a: u32, #[default(default_b())] b: u32, #[default(DEFAULT_C)] c: S) -> (u32, u32, S) {
            (a, b, c)
        }

        assert_eq!(foo!(), (10, 20, S { c: 30 }));
        assert_eq!(foo!(1), (1, 20, S { c: 30 }));
        assert_eq!(foo!(1, 2), (1, 2, S { c: 30 }));
        assert_eq!(foo!(1, 2, S {c: 3}), (1, 2, S { c: 3 }));
        assert_eq!(foo!(c = S {c: 1}), (10, 20, S { c: 1 }));
    }
}
