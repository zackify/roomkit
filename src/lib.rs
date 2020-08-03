mod home;
pub use home::Home;
pub use home::Room;
pub mod integrations;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
