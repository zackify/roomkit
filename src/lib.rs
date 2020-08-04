mod home;
mod room;
pub use home::Home;
pub use room::Room;
pub mod integrations;

#[cfg(test)]
mod tests {
    use crate::home::Home;
    use crate::integrations::GenericIntegration;
    use crate::integrations::HueBridge;
    use crate::room::Room;
    use std::any::Any;

    #[test]
    fn find_integration_in_vector() {
        let hue = HueBridge {
            light_ids: None,
            bridge_url: "http://192.168.86.30/api/test".into(),
        };
        let name = hue.name();
        let integrations: Vec<Box<dyn Any>> = vec![Box::new(hue)];
        let mut home = Home {
            rooms: vec![Room {
                name: "Master".into(),
                people_inside: 0,
            }],
            integrations: integrations,
        };

        if let Some(found_hue) = home.integration::<HueBridge>() {
            assert_eq!(found_hue.name(), name);
        }
    }
}
