use crate::device::Device;
use crate::integrations::HueBridge;

pub struct Home {
  pub devices: Vec<Device>,
}

pub struct Action<Type> {
  device_name: String,
  action: Box<dyn FnMut(Box<Type>)>,
}

impl Home {
  pub fn initialize(&mut self) {
    println!("Bringing {:?} devices online...", self.devices.len());

    self
      .devices
      .iter_mut()
      .for_each(|device| match device.initialize() {
        Ok(_) => {
          println!("Initialized {:?}", device.name());
        }
        Err(e) => println!("Failed to initialize {:?}: {:?}", device.name(), e),
      });

    // let mut action = Action::<HueBridge> {
    //   device_name: String::from("Hue Bridge"),
    //   action: Box::new(|device| print!("in action {:?}", device.get_lights())),
    // };

    // self.devices.iter().for_each(|device| {
    //   if device.name() == action.device_name {
    //     (action.action)(device.device as Box<HueBridge>);
    //   }
    // });
  }
}
