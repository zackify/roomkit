//use crate::integrations::hue_bridge::toggle_light;
pub use crate::room::Room;
use std::any::Any;

pub struct Home {
  pub rooms: Vec<Room>,
  pub integrations: Vec<Box<dyn Any>>,
}

impl Home {
  pub fn initialize(&mut self) {
    println!("Bringing {:?} devices online...", self.integrations.len());

    //toggle_light(self);
  }

  /*
    pull out a single integration from our vector
    use downcasting to check for the correct integration!!!
    this took me a long time to figure out, I wanted to make
    integrations dynamic, and follow a trait, and not have to use
    an enum internally, this way other people can make an integration
    without this internal library knowing about it
  */
  pub fn integration<Integration>(&mut self) -> Option<&Integration>
  where
    Integration: 'static,
  {
    //Find the integration from our list of integrations
    let found = self.integrations.iter().find(|integration| {
      match integration.downcast_ref::<Integration>() {
        Some(_) => true,
        None => false,
      }
    });
    /* unwrap our integration finding Optional, if we did find the integration
       then we need to downcast_ref again so that we can return it
       i wish i didnt have to do this twice, if you know of a way to only
       do it one time, send a PR :)
    */
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
