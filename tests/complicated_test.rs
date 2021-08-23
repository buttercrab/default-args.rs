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
}
