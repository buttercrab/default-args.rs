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

        assert_eq!(foo_(), 1);
    }
}
