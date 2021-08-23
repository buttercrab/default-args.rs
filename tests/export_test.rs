#[cfg(test)]
mod export {
    use default_args::default_args;

    #[macro_use]
    pub mod foo {
        use super::*;

        default_args! {
            pub fn crate::export::foo::bar() -> usize {
                1
            }
        }
    }

    #[test]
    fn export_test() {
        assert_eq!(bar!(), 1);
    }
}
