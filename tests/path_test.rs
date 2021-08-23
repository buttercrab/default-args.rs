#[cfg(test)]
mod path {
    use default_args::default_args;

    #[macro_use]
    pub mod foo {
        use super::*;

        default_args! {
            pub fn crate::path::foo::bar() -> usize {
                1
            }
        }
    }

    #[test]
    fn path_test() {
        assert_eq!(bar!(), 1);
    }
}
