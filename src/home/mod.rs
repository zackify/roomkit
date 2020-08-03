mod room;
pub use room::Room;
use std::any::Any;

mod test_action;
use test_action::test_action;

pub struct Home {
  pub rooms: Vec<Room>,
  pub integrations: Vec<Box<dyn Any>>,
}

impl Home {
  pub fn initialize(&mut self) {
    println!("Bringing {:?} devices online...", self.integrations.len());

    test_action(self);
  }

  pub fn integration<Integration>(&mut self) -> Option<&Integration>
  where
    Integration: 'static,
  {
    let found = self.integrations.iter().find(|integration| {
      match integration.downcast_ref::<Integration>() {
        Some(_) => true,
        None => false,
      }
    });
    if let Some(integration) = found {
      if let Some(real_ref) = integration.downcast_ref::<Integration>() {
        return Some(real_ref);
      } else {
        None
      }
    } else {
      return None;
    }
  }
}
