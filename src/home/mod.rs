mod room;
use crate::action_types::Action;
use crate::device::GenericDevice;
use crate::integrations::Integration;
pub use room::Room;

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

    // find a way to write one that checks if room empty then hue light, etc use match
    // to run methods from multiple devices
    let mut action = Action {
      callback: Box::new(|integration| match integration {
        Integration::HueBridge(hue) => print!("in action {:?}", hue.get_lights()),
        // => (),
      }),
    };

    self
      .integrations
      .iter_mut()
      .for_each(|integration| (action.callback)(integration));
  }
}
