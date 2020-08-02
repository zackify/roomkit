mod room;
use crate::action_types::Action;
use crate::device::GenericDevice;
use crate::integrations::Integration;
pub use room::Room;

mod test_action;
use test_action::test_action;

#[derive(Debug)]
pub struct Home {
  pub rooms: Vec<Room>,
  pub integrations: Vec<Integration>,
}

impl Home {
  pub fn initialize(&mut self) {
    println!("Bringing {:?} devices online...", self.integrations.len());

    self
      .integrations
      .iter_mut()
      .for_each(|integration| match integration.initialize() {
        Ok(_) => {
          println!("Initialized {:?}", integration.name());
        }
        Err(e) => println!("Failed to initialize {:?}: {:?}", integration.name(), e),
      });

    test_action(self);
  }
}
