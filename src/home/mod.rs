use crate::device::GenericDevice;
use crate::integrations::Integration;

pub struct Home {
  pub integrations: Vec<Integration>,
}

pub struct Action {
  callback: Box<dyn FnMut(&mut Integration)>,
}

impl Home {
  pub fn initialize(&mut self) {
    println!("Bringing {:?} devices online...", self.integrations.len());

    self
      .integrations
      .iter_mut()
      .for_each(|integration| match integration {
        Integration::Hue(hue) => match hue.initialize() {
          Ok(_) => {
            println!("Initialized {:?}", hue.name());
          }
          Err(e) => println!("Failed to initialize {:?}: {:?}", hue.name(), e),
        },
        _ => (),
      });

    // find a way to write one that checks if room empty then hue light, etc use match
    // to run methods from multiple devices
    let mut action = Action {
      callback: Box::new(|integration| match integration {
        Integration::Hue(hue) => print!("in action {:?}", hue.get_lights()),
        _ => (),
      }),
    };

    self
      .integrations
      .iter_mut()
      .for_each(|integration| (action.callback)(integration));
  }
}
