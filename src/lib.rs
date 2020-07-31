mod home;
pub use home::Home;

pub mod device;
pub mod integrations;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
