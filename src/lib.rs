pub use karten::*;

mod fluent;
mod karten;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("{}", '♠');
        assert_eq!(2 + 2, 4);
    }
}
