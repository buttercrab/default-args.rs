#[cfg(test)]
mod basic_test {
    use default_args::default_args;

    #[test]
    fn basic_test() {
        default_args! {
            fn foo() -> u32 {
                1
            }
        }

        assert_eq!(foo!(), 1);
    }

    #[test]
    fn basic_test2() {
        default_args! {
            fn foo(a: u32 = 0) -> u32 {
                a
            }
        }

        assert_eq!(foo!(), 0);
        assert_eq!(foo!(1), 1);
        assert_eq!(foo!(a = 1), 1);
    }

    #[test]
    fn expression_test() {
        const DEFAULT_A: u32 = 10;
        fn default_b() -> u32 {
            20
        }
        #[derive(PartialEq, Debug)]
        struct S {
            c: u32,
        }

        default_args! {
            fn foo(a: u32 = DEFAULT_A, b: u32 = default_b(), c: S = S { c: 30 }) -> (u32, u32, S) {
                (a, b, c)
            }
        }

        assert_eq!(foo!(), (10, 20, S { c: 30 }));
        assert_eq!(foo!(1), (1, 20, S { c: 30 }));
        assert_eq!(foo!(1, 2), (1, 2, S { c: 30 }));
        assert_eq!(foo!(1, 2, S {c: 3}), (1, 2, S { c: 3 }));
        assert_eq!(foo!(c = S {c: 1}), (10, 20, S { c: 1 }));
    }
}
