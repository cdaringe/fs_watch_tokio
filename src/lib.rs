pub mod builder;
pub(crate) mod error;
pub(crate) mod interfaces;
pub mod watcher;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
