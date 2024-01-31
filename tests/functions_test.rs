
#[cfg(test)]
mod tests{
    use std::panic;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_fails() {
        let result = panic::catch_unwind(|| {
            panic!("oh no!");
        });
        assert!(result.is_err());
    }
}