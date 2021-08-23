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
}
